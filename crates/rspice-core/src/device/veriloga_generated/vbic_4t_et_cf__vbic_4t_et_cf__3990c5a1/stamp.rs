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
    v6: f64, vb: f64, vc: f64, vd: f64, v1f: f64, v1g: f64, 
    v2c: f64, v52: f64, v55: f64, v5t: f64, v6v: f64, v8g: f64, 
    v9f: f64, v9g: f64, v9h: f64, v9i: f64, v9j: f64, v9k: f64, 
    v9l: f64, v9m: f64, v9o: f64, v9p: f64, va2: f64, vn7: f64, 
    vn8: f64, vow: f64, vph: f64, vpn: f64, vqn: f64, vqs: f64, 
    vqu: f64, vqw: f64, vqy: f64, vr0: f64, vr8: f64, vwl: f64, 
    vwr: f64, vws: f64, vwx: f64, vx0: f64, vxz: f64, vyf: f64, 
    v10e: f64, v10g: f64, v10n: f64, v10o: f64, v10r: f64, v10v: f64, 
    v10y: f64, v111: f64, v124: f64, v125: f64, v13k: f64, v14q: f64, 
    v19y: f64, v2ih: f64, v2ii: f64, v2ij: f64, v2ik: f64, v2il: f64, 
    v2im: f64, v2in: f64, v2lh: f64, v2li: f64, v2lj: f64, v2m2: f64, 
    v2m3: f64, v2m4: f64, v2p7: f64, v2p8: f64, v2p9: f64, v2pa: f64, 
    v2pb: f64, v2pf: f64, v2qg: f64, v2qj: f64, v2qk: f64, v2ql: f64, 
    v2qm: f64, v2qn: f64, v2qo: f64, v2qv: f64, v2qw: f64, v2qx: f64, 
    v2qy: f64, v2qz: f64, v2r3: f64, v2r8: f64, v2r9: f64, v2ra: f64, 
    v2rt: f64, v2ru: f64, v2rv: f64, v2rw: f64, v2rx: f64, v3dy: f64, 
    v3dz: f64, v3e0: f64, v3e2: f64, v3e3: f64, v3e4: f64, v3ei: f64, 
    v3ej: f64, v3ek: f64, v3er: f64, v3es: f64, v3et: f64, v3pn: f64, 
    v3pr: f64, v3q0: f64, v3q1: f64, v3q2: f64, v3q9: f64, v3qa: f64, 
    v3qb: f64, v3qc: f64, v3qh: f64, v3qj: f64, v3qt: f64, v3qu: f64, 
    v3qv: f64, v3qw: f64, v3qx: f64, v3qy: f64, v3r5: f64, v3rc: f64, 
    v3rd: f64, v3re: f64, v3rf: f64, v3rg: f64, v3rj: f64, v3rk: f64, 
    v3rl: f64, v3rm: f64, v3rn: f64, v3rr: f64, v3rs: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v6=ctx.node_voltage(nodes[4]);
        let v7=(sf[321]+v6);
        let vb=((v7*1.3806503e-23)/1.602176462e-19);
        let vc=(v7/sf[1]);
        let vd=(v7-sf[1]);
        let vh=(sf[3]*f64::powf(vc,sf[4]));
        let v1c=f64::powf(vc,sf[20]);
        let v1f=1.0;
        let v1g=(v1f-vc);
        let v1h=(sf[22]*v1g);
        let v1j=((v1h/vb)).exp();
        let v1k=(v1c*v1j);
        let v1o=(sf[19]*f64::powf(v1k,sf[24]));
        let v1r=f64::powf(vc,sf[26]);
        let v1u=(v1g*sf[28]);
        let v1w=((v1u/vb)).exp();
        let v1x=(v1r*v1w);
        let v21=(sf[25]*f64::powf(v1x,sf[30]));
        let v25=(v1g*sf[33]);
        let v27=((v25/vb)).exp();
        let v28=(v1c*v27);
        let v2c=(sf[31]*f64::powf(v28,sf[35]));
        let v4h=(v1f+(vd*sf[70]));
        let v4i=(sf[23]*v4h);
        let v4j=(sf[29]*v4h);
        let v52=2.0;
        let v54=(v52*(vb/vc));
        let v55=0.5;
        let v58=(vc*sf[79]);
        let v5a=((v58/vb)).exp();
        let v5b=-0.5;
        let v5d=(vc*sf[80]);
        let v5f=((v5d/vb)).exp();
        let v5g=(v5a-v5f);
        let v5h=(v5g).ln();
        let v5i=(v54*v5h);
        let v5l=(vb*3.0);
        let v5m=(vc).ln();
        let v5n=(v5l*v5m);
        let v5p=(vc-v1f);
        let v5r=(((vc*v5i)-v5n)-(sf[38]*v5p));
        let v5s=(vb*v52);
        let v5t=4.0;
        let v5u=(-v5r);
        let v5w=((v5u/vb)).exp();
        let v5z=((v1f+(v5t*v5w))).sqrt();
        let v61=(v55*(v1f+v5z));
        let v62=(v61).ln();
        let v64=(v5r+(v5s*v62));
        let v67=(vc*sf[82]);
        let v69=((v67/vb)).exp();
        let v6b=(vc*sf[83]);
        let v6d=((v6b/vb)).exp();
        let v6e=(v69-v6d);
        let v6f=(v6e).ln();
        let v6g=(v54*v6f);
        let v6k=(((vc*v6g)-v5n)-(sf[49]*v5p));
        let v6l=(-v6k);
        let v6n=((v6l/vb)).exp();
        let v6q=((v1f+(v5t*v6n))).sqrt();
        let v6s=(v55*(v1f+v6q));
        let v6t=(v6s).ln();
        let v6v=(v6k+(v5s*v6t));
        let v6y=(vc*sf[85]);
        let v70=((v6y/vb)).exp();
        let v72=(vc*sf[86]);
        let v74=((v72/vb)).exp();
        let v75=(v70-v74);
        let v76=(v75).ln();
        let v77=(v54*v76);
        let v7b=(((vc*v77)-v5n)-(sf[61]*v5p));
        let v7c=(-v7b);
        let v7e=((v7c/vb)).exp();
        let v7h=((v1f+(v5t*v7e))).sqrt();
        let v7j=(v55*(v1f+v7h));
        let v7k=(v7j).ln();
        let v7m=(v7b+(v5s*v7k));
        let v7o=(sf[78]/v64);
        let v7r=(sf[87]*f64::powf(v7o,sf[88]));
        let v7t=(sf[81]/v6v);
        let v7v=f64::powf(v7t,sf[90]);
        let v7w=(sf[89]*v7v);
        let v7y=(v7v*sf[91]);
        let v80=(sf[84]/v7m);
        let v83=(sf[92]*f64::powf(v80,sf[93]));
        let v85=(v1c*sf[94]);
        let v86=(v1j*v85);
        let v8g=0.0;
        let v8q=(if sb[2]{(v1f/vh)}else{v8g});
        let v9f=ctx.node_voltage(nodes[8]);
        let v9g=ctx.node_voltage(nodes[9]);
        let v9h=(v9f-v9g);
        let v9i=ctx.node_voltage(nodes[7]);
        let v9j=(v9i-v9g);
        let v9k=ctx.node_voltage(nodes[6]);
        let v9l=(v9f-v9k);
        let v9m=ctx.node_voltage(nodes[5]);
        let v9n=(v9f-v9m);
        let v9o=ctx.node_voltage(nodes[10]);
        let v9p=(v9i-v9o);
        let v9q=(-v64);
        let v9s=(v9q*sf[119]);
        let v9w=(v9h+v9s);
        let v9x=(if (sf[121]!=0.0){v9w}else{v8g});
        let v9z=(if (v9x>v8g){v1f}else{v8g});
        let va0=((sf[121]!=0.0)&&(v9z!=0.0));
        let va2=-1.0;
        let va5=(if va0{sf[124]}else{v8g});
        let va8=(v1f-(sf[122]*(sf[122]*va5)));
        let vae=(v9x*sf[126]);
        let vag=(sf[122]+(vae/v64));
        let val=((sf[121]!=0.0)&&(!(v9z!=0.0)));
        let van=(v1f-(v9h/v64));
        let vap=(v1f-f64::powf(van,sf[125]));
        let vas=(if val{((v64*vap)/sf[125])}else{(if va0{((v64*va8)/sf[125])}else{v8g})});
        let vat=(if val{v8g}else{(if va0{(va5*(v9x*vag))}else{v8g})});
        let vb1=(((v9s*v9s)+sf[128])).sqrt();
        let vb2=(if sb[10]{vb1}else{v8g});
        let vb5=(if sb[10]{(v5b*(v9s+vb2))}else{v8g});
        let vb7=(v1f-(vb5/v64));
        let vb8=f64::powf(vb7,sf[125]);
        let vbb=(if sb[10]{((v9q*vb8)/sf[125])}else{v8g});
        let vbc=(if sb[10]{v9w}else{v8g});
        let vbf=((sf[128]+(vbc*vbc))).sqrt();
        let vbg=(if sb[10]{vbf}else{v8g});
        let vbk=(if sb[10]{((v55*(vbc-vbg))-v9s)}else{v8g});
        let vbm=(v1f-(vbk/v64));
        let vbn=f64::powf(vbm,sf[125]);
        let vbq=(if sb[10]{((v9q*vbn)/sf[125])}else{vas});
        let vby=(if sb[10]{((vbq+(sf[130]*(vb5+(v9h-vbk))))-vbb)}else{(if (sf[121]!=0.0){(vas+vat)}else{v8g})});
        let vbz=(v9j+v9s);
        let vc0=(if (sf[121]!=0.0){vbz}else{v9x});
        let vc2=(if (vc0>v8g){v1f}else{v8g});
        let vc3=((sf[121]!=0.0)&&(vc2!=0.0));
        let vc4=(if vc3{sf[124]}else{va5});
        let vc7=(v1f-(sf[122]*(sf[122]*vc4)));
        let vcb=(sf[126]*vc0);
        let vcd=(sf[122]+(vcb/v64));
        let vci=((sf[121]!=0.0)&&(!(vc2!=0.0)));
        let vck=(v1f-(v9j/v64));
        let vcm=(v1f-f64::powf(vck,sf[125]));
        let vcp=(if vci{((v64*vcm)/sf[125])}else{(if vc3{((v64*vc7)/sf[125])}else{vbq})});
        let vcq=(if vci{v8g}else{(if vc3{(vc4*(vc0*vcd))}else{vat})});
        let vct=(if sb[10]{vb1}else{vb2});
        let vcw=(if sb[10]{(v5b*(v9s+vct))}else{vb5});
        let vcy=(v1f-(vcw/v64));
        let vcz=f64::powf(vcy,sf[125]);
        let vd2=(if sb[10]{((v9q*vcz)/sf[125])}else{vbb});
        let vd3=(if sb[10]{vbz}else{vbc});
        let vd6=((sf[128]+(vd3*vd3))).sqrt();
        let vd7=(if sb[10]{vd6}else{vbg});
        let vdb=(if sb[10]{((v55*(vd3-vd7))-v9s)}else{vbk});
        let vdd=(v1f-(vdb/v64));
        let vde=f64::powf(vdd,sf[125]);
        let vdh=(if sb[10]{((v9q*vde)/sf[125])}else{vcp});
        let vdn=(if sb[10]{((vdh+(sf[130]*(vcw+(v9j-vdb))))-vd2)}else{(if (sf[121]!=0.0){(vcp+vcq)}else{v8g})});
        let vdo=(-v6v);
        let vdp=(sf[119]*vdo);
        let vdt=(v9l+vdp);
        let vdu=(if (sf[132]!=0.0){vdt}else{vc0});
        let vdw=(if (vdu>v8g){v1f}else{v8g});
        let vdx=((sf[132]!=0.0)&&(vdw!=0.0));
        let ve0=(if vdx{sf[134]}else{vc4});
        let ve3=(v1f-(sf[122]*(sf[122]*ve0)));
        let ve9=(vdu*sf[136]);
        let veb=(sf[122]+(ve9/v6v));
        let vek=(if (sb[12]&&(v9l<sf[138])){v1f}else{v8g});
        let vem=((sf[132]!=0.0)&&(!(vdw!=0.0)));
        let ven=((vek!=0.0)&&vem);
        let vep=(v1f+(sf[137]/v6v));
        let veq=f64::powf(vep,sf[135]);
        let ves=(sf[135]*(v9l+sf[137]));
        let vet=(v6v+sf[137]);
        let vev=(v1f-(ves/vet));
        let vex=(v1f-(veq*vev));
        let vf2=(vem&&(!(vek!=0.0)));
        let vf4=(v1f-(v9l/v6v));
        let vf6=(v1f-f64::powf(vf4,sf[135]));
        let vf9=(if vf2{((v6v*vf6)/sf[135])}else{(if ven{((v6v*vex)/sf[135])}else{(if vdx{((v6v*ve3)/sf[135])}else{vdh})})});
        let vfa=(if vem{v8g}else{(if vdx{(ve0*(vdu*veb))}else{vcq})});
        let vfj=(vdp+sf[137]);
        let vfk=(sf[137]-vdp);
        let vfl=(vfj/vfk);
        let vfm=(if sb[16]{vfl}else{v8g});
        let vfn=(v52*vfm);
        let vfo=(vfm-v1f);
        let vft=(((vfo*vfo)+sf[142])).sqrt();
        let vfu=(v1f+vfm);
        let vfz=(((vfu*vfu)+sf[144])).sqrt();
        let vg0=(vft+vfz);
        let vg2=(if sb[16]{(vfn/vg0)}else{v8g});
        let vg7=(if sb[16]{(v55*(((vfk*vg2)-sf[137])-vdp))}else{vcw});
        let vg9=(v1f-(vg7/v6v));
        let vgb=(v1f-f64::powf(vg9,sf[135]));
        let vge=(if sb[16]{((v6v*vgb)/sf[135])}else{v8g});
        let vgh=(vdp+(sf[137]+(v52*v9l)));
        let vgj=(if sb[16]{(vgh/vfk)}else{v8g});
        let vgk=(v52*vgj);
        let vgl=(vgj-v1f);
        let vgo=((sf[142]+(vgl*vgl))).sqrt();
        let vgp=(v1f+vgj);
        let vgs=((sf[144]+(vgp*vgp))).sqrt();
        let vgt=(vgo+vgs);
        let vgv=(if sb[16]{(vgk/vgt)}else{v8g});
        let vh0=(if sb[16]{(v55*(((vfk*vgv)-sf[137])-vdp))}else{vdb});
        let vh2=(v1f-(vh0/v6v));
        let vh4=(v1f-f64::powf(vh2,sf[135]));
        let vh7=(if sb[16]{((v6v*vh4)/sf[135])}else{vf9});
        let vha=(if sb[16]{(v55*(v1f+vgv))}else{v8g});
        let vhc=f64::powf(vep,sf[145]);
        let vhd=(if sb[16]{vhc}else{v8g});
        let vhf=(v1f+(vdp/v6v));
        let vhg=f64::powf(vhf,sf[145]);
        let vhh=(if sb[16]{vhg}else{v8g});
        let vhi=(v1f-vha);
        let vhm=(if sb[16]{((vhd*vhi)+(vha*vhh))}else{v8g});
        let vho=(vg7+(v9l-vh0));
        let vhq=(if sb[16]{(vhm*vho)}else{v8g});
        let vhy=((sf[142]+(vdp*vdp))).sqrt();
        let vhz=(if sb[18]{vhy}else{vct});
        let vi2=(if sb[18]{(v5b*(vdp+vhz))}else{vg7});
        let vi4=(v1f-(vi2/v6v));
        let vi5=f64::powf(vi4,sf[135]);
        let vi8=(if sb[18]{((vdo*vi5)/sf[135])}else{vd2});
        let vi9=(if sb[18]{vdt}else{vd3});
        let vic=((sf[142]+(vi9*vi9))).sqrt();
        let vid=(if sb[18]{vic}else{vd7});
        let vih=(if sb[18]{((v55*(vi9-vid))-vdp)}else{vh0});
        let vij=(v1f-(vih/v6v));
        let vik=f64::powf(vij,sf[135]);
        let vin=(if sb[18]{((vdo*vik)/sf[135])}else{vh7});
        let viu=(if sb[18]{((vin+(sf[146]*(vi2+(v9l-vih))))-vi8)}else{(if sb[16]{((vh7+vhq)-vge)}else{(if (sf[132]!=0.0){(vf9+vfa)}else{v8g})})});
        let viv=(v9p+vdp);
        let viw=(if (sf[132]!=0.0){viv}else{vdu});
        let viy=(if (viw>v8g){v1f}else{v8g});
        let viz=((sf[132]!=0.0)&&(viy!=0.0));
        let vj0=(if viz{sf[134]}else{ve0});
        let vj3=(v1f-(sf[122]*(sf[122]*vj0)));
        let vj7=(sf[136]*viw);
        let vj9=(sf[122]+(vj7/v6v));
        let vjf=(if (sb[12]&&(v9p<sf[138])){v1f}else{v8g});
        let vjh=((sf[132]!=0.0)&&(!(viy!=0.0)));
        let vji=((vjf!=0.0)&&vjh);
        let vjk=(sf[135]*(v9p+sf[137]));
        let vjm=(v1f-(vjk/vet));
        let vjo=(v1f-(veq*vjm));
        let vjt=(vjh&&(!(vjf!=0.0)));
        let vjv=(v1f-(v9p/v6v));
        let vjx=(v1f-f64::powf(vjv,sf[135]));
        let vk0=(if vjt{((v6v*vjx)/sf[135])}else{(if vji{((v6v*vjo)/sf[135])}else{(if viz{((v6v*vj3)/sf[135])}else{vin})})});
        let vk1=(if vjh{v8g}else{(if viz{(vj0*(viw*vj9))}else{vfa})});
        let vk4=(if sb[16]{vfl}else{vfm});
        let vk5=(v52*vk4);
        let vk6=(vk4-v1f);
        let vk9=((sf[142]+(vk6*vk6))).sqrt();
        let vka=(v1f+vk4);
        let vkd=((sf[144]+(vka*vka))).sqrt();
        let vke=(vk9+vkd);
        let vkg=(if sb[16]{(vk5/vke)}else{vg2});
        let vkl=(if sb[16]{(v55*(((vfk*vkg)-sf[137])-vdp))}else{vi2});
        let vkn=(v1f-(vkl/v6v));
        let vkp=(v1f-f64::powf(vkn,sf[135]));
        let vkv=(vdp+(sf[137]+(v52*v9p)));
        let vkx=(if sb[16]{(vkv/vfk)}else{vgj});
        let vky=(v52*vkx);
        let vkz=(vkx-v1f);
        let vl2=((sf[142]+(vkz*vkz))).sqrt();
        let vl3=(v1f+vkx);
        let vl6=((sf[144]+(vl3*vl3))).sqrt();
        let vl7=(vl2+vl6);
        let vl9=(if sb[16]{(vky/vl7)}else{vgv});
        let vle=(if sb[16]{(v55*(((vfk*vl9)-sf[137])-vdp))}else{vih});
        let vlg=(v1f-(vle/v6v));
        let vli=(v1f-f64::powf(vlg,sf[135]));
        let vll=(if sb[16]{((v6v*vli)/sf[135])}else{vk0});
        let vlo=(if sb[16]{(v55*(v1f+vl9))}else{vha});
        let vlp=(if sb[16]{vhc}else{vhd});
        let vlq=(if sb[16]{vhg}else{vhh});
        let vlr=(v1f-vlo);
        let vlv=(if sb[16]{((vlp*vlr)+(vlo*vlq))}else{vhm});
        let vlx=(vkl+(v9p-vle));
        let vm3=(if sb[18]{vhy}else{vhz});
        let vm6=(if sb[18]{(v5b*(vdp+vm3))}else{vkl});
        let vm8=(v1f-(vm6/v6v));
        let vm9=f64::powf(vm8,sf[135]);
        let vmc=(if sb[18]{((vdo*vm9)/sf[135])}else{vi8});
        let vmd=(if sb[18]{viv}else{vi9});
        let vmg=((sf[142]+(vmd*vmd))).sqrt();
        let vmh=(if sb[18]{vmg}else{vid});
        let vml=(if sb[18]{((v55*(vmd-vmh))-vdp)}else{vle});
        let vmn=(v1f-(vml/v6v));
        let vmo=f64::powf(vmn,sf[135]);
        let vmr=(if sb[18]{((vdo*vmo)/sf[135])}else{vll});
        let vmx=(if sb[18]{((vmr+(sf[146]*(vm6+(v9p-vml))))-vmc)}else{(if sb[16]{((vll+(if sb[16]{(vlv*vlx)}else{vhq}))-(if sb[16]{((v6v*vkp)/sf[135])}else{vge}))}else{(if (sf[132]!=0.0){(vk0+vk1)}else{v8g})})});
        let vn0=(-v7m);
        let vn2=(if (sf[147]!=0.0){(sf[119]*vn0)}else{vdp});
        let vn7=ctx.node_voltage(nodes[11]);
        let vn8=(vn7-v9o);
        let vn9=(vn2+vn8);
        let vna=(if sb[21]{vn9}else{viw});
        let vnc=(if (vna>v8g){v1f}else{v8g});
        let vnd=(sb[21]&&(vnc!=0.0));
        let vng=(if vnd{sf[151]}else{vj0});
        let vnj=(v1f-(sf[122]*(sf[122]*vng)));
        let vnp=(vna*sf[153]);
        let vnr=(sf[122]+(vnp/v7m));
        let vnw=(sb[21]&&(!(vnc!=0.0)));
        let vny=(v1f-(vn8/v7m));
        let vo0=(v1f-f64::powf(vny,sf[152]));
        let vo3=(if vnw{((v7m*vo0)/sf[152])}else{(if vnd{((v7m*vnj)/sf[152])}else{vmr})});
        let vod=(((vn2*vn2)+sf[155])).sqrt();
        let voh=(if sb[23]{(v5b*(vn2+(if sb[23]{vod}else{vm3})))}else{vm6});
        let voj=(v1f-(voh/v7m));
        let vok=f64::powf(voj,sf[152]);
        let voo=(if sb[23]{vn9}else{vmd});
        let vor=((sf[155]+(voo*voo))).sqrt();
        let vow=(if sb[23]{((v55*(voo-(if sb[23]{vor}else{vmh})))-vn2)}else{vml});
        let voy=(v1f-(vow/v7m));
        let voz=f64::powf(voy,sf[152]);
        let vpc=(if sb[24]{v8g}else{(if sb[23]{(((if sb[23]{((vn0*voz)/sf[152])}else{vo3})+(sf[157]*(voh+(vn8-vow))))-(if sb[23]{((vn0*vok)/sf[152])}else{vmc}))}else{(if sb[21]{(vo3+(if vnw{v8g}else{(if vnd{(vng*(vna*vnr))}else{vk1})}))}else{v8g})})});
        let vpd=(vb*v4i);
        let vpe=(v9h/vpd);
        let vpg=(scalar_limexp(vpe)-v1f);
        let vph=(v1o*vpg);
        let vpi=(vb*v4j);
        let vpj=(v9l/vpi);
        let vpk=scalar_limexp(vpj);
        let vpl=(v1o*v21);
        let vpm=(vpk-v1f);
        let vpn=(vpl*vpm);
        let vpr=((v1f+(sf[102]*vby))+(sf[99]*viu));
        let vps=0.0001;
        let vpt=(vpr-vps);
        let vpx=(((vpt*vpt)+1e-8)).sqrt();
        let vq1=(vps+(v55*((vpr+vpx)-vps)));
        let vqb=(v5t*((v8q*vph)+(sf[105]*vpn)));
        let vqc=(f64::powf(vq1,sf[161])+vqb);
        let vqi=(v55*vq1);
        let vqj=(v1f+vqb);
        let vql=(v1f+f64::powf(vqj,sf[160]));
        let vqn=(if sb[26]{(vqi*vql)}else{(if (sf[159]!=0.0){(v55*(vq1+f64::powf(vqc,sf[160])))}else{v8g})});
        let vqs=(vb*sf[34]);
        let vqu=(if (sf[162]!=0.0){(v9p/vqs)}else{vpj});
        let vqw=(if (sf[162]!=0.0){scalar_limexp(vqu)}else{vpk});
        let vqy=(if (sf[162]!=0.0){(v9l/vqs)}else{v8g});
        let vr0=(if (sf[162]!=0.0){scalar_limexp(vqy)}else{v8g});
        let vr6=(((vqw*sf[163])+(vr0*sf[164]))-v1f);
        let vr8=(if (sf[162]!=0.0){(v2c*vr6)}else{v8g});
        let vwl=ctx.node_voltage(nodes[0]);
        let vwr=(v9l/vb);
        let vws=scalar_limexp(vwr);
        let vwt=(v9n/vb);
        let vwu=scalar_limexp(vwt);
        let vwx=((v1f+(v86*vws))).sqrt();
        let vx0=((v1f+(v86*vwu))).sqrt();
        let vxz=ctx.node_voltage(nodes[1]);
        let vyf=ctx.node_voltage(nodes[2]);
        let vzo=(if (vph>v8g){v1f}else{v8g});
        let vzq=(sf[117]*(vph*vzo));
        let vzr=(v1f+vzq);
        let vzs=(vzq/vzr);
        let vzx=(sf[183]*(v1f+(vq1*sf[184])));
        let v101=((sf[114]*v9l)/1.44);
        let v103=(sf[185]*scalar_limexp(v101));
        let v105=(sf[118]+(vzs*vzs));
        let v108=(v1f+(vzo*(v103*v105)));
        let v109=(vzx*v108);
        let v10c=(vph*v109);
        let v10e=((sf[165]*(v7r*vby))+(v10c/vqn));
        let v10g=(sf[170]*(v7r*vdn));
        let v10n=(((v7w*viu)+(vpn*sf[186]))+(vwx*sf[187]));
        let v10o=(vx0*sf[187]);
        let v10r=((v7y*vmx)+((if sb[28]{v8g}else{vr8})*sf[186]));
        let v10v=((v83*vpc)+(vn8*sf[188]));
        let v10y=((vxz-vyf)*sf[189]);
        let v111=((vxz-vwl)*sf[190]);
        let v124=(v6*sf[193]);
        let v125=8.617342301212761e-5;
        let v13e=(sf[194]*(sf[20]*f64::powf(vc,sf[203])));
        let v13k=(vb*vb);
        let v13m=(v1j*(((vb*sf[205])-(v1h*v125))/v13k));
        let v13u=(sf[19]*(((v1j*v13e)+(v1c*v13m))*(sf[24]*f64::powf(v1k,sf[206]))));
        let v14q=(sf[31]*(((v27*v13e)+(v1c*(v27*(((vb*sf[210])-(v25*v125))/v13k))))*(sf[35]*f64::powf(v28,sf[211]))));
        let v17o=(v52*(((vc*v125)-(vb*sf[194]))/(vc*vc)));
        let v18d=((v5m*0.00025852026903638284)+(v5l*(sf[194]/vc)));
        let v18g=((((v5i*sf[194])+(vc*((v5h*v17o)+(v54*(((v5a*(((vb*sf[230])-(v58*v125))/v13k))-(v5f*(((vb*sf[231])-(v5d*v125))/v13k)))/v5g)))))-v18d)-sf[232]);
        let v18h=0.00017234684602425522;
        let v18w=(v18g+((v62*v18h)+(v5s*((v55*((v5t*(v5w*(((vb*(-v18g))-(v5u*v125))/v13k)))/(v52*v5z)))/v61))));
        let v19j=((((v6g*sf[194])+(vc*((v6f*v17o)+(v54*(((v69*(((vb*sf[233])-(v67*v125))/v13k))-(v6d*(((vb*sf[234])-(v6b*v125))/v13k)))/v6e)))))-v18d)-sf[235]);
        let v19y=(v19j+((v6t*v18h)+(v5s*((v55*((v5t*(v6n*(((vb*(-v19j))-(v6l*v125))/v13k)))/(v52*v6q)))/v6s))));
        let v1al=((((v77*sf[194])+(vc*((v76*v17o)+(v54*(((v70*(((vb*sf[236])-(v6y*v125))/v13k))-(v74*(((vb*sf[237])-(v72*v125))/v13k)))/v75)))))-v18d)-sf[238]);
        let v1b0=(v1al+((v7k*v18h)+(v5s*((v55*((v5t*(v7e*(((vb*(-v1al))-(v7c*v125))/v13k)))/(v52*v7h)))/v7j))));
        let v1b3=(v64*v64);
        let v1b9=(sf[87]*(((-(sf[78]*v18w))/v1b3)*(sf[88]*f64::powf(v7o,sf[239]))));
        let v1bc=(v6v*v6v);
        let v1bg=(((-(sf[81]*v19y))/v1bc)*(sf[90]*f64::powf(v7t,sf[174])));
        let v1bl=(v7m*v7m);
        let v1bv=((v85*v13m)+(v1j*(sf[94]*v13e)));
        let v1cj=(-v18w);
        let v1ck=(sf[119]*v1cj);
        let v1cl=(if (sf[121]!=0.0){v1ck}else{v8g});
        let v1cy=(sf[244]/v64);
        let v1dl=(-(v1f/v64));
        let v1dm=(-(va2/v64));
        let v1dp=(sf[125]*f64::powf(van,sf[246]));
        let v1e4=(if val{(((vap*v18w)+(v64*(-((-((-(v9h*v18w))/v1b3))*v1dp))))/sf[125])}else{(if va0{((va8*v18w)/sf[125])}else{v8g})});
        let v1e5=(if val{((v64*(-(v1dl*v1dp)))/sf[125])}else{v8g});
        let v1e6=(if val{((v64*(-(v1dm*v1dp)))/sf[125])}else{v8g});
        let v1e7=(if val{v8g}else{(if va0{(va5*((vag*v1cl)+(v9x*(((v64*(sf[126]*v1cl))-(vae*v18w))/v1b3))))}else{v8g})});
        let v1e8=(if val{v8g}else{(if va0{(va5*((vag*sf[242])+(v9x*v1cy)))}else{v8g})});
        let v1e9=(if val{v8g}else{(if va0{(va5*((vag*sf[243])+(v9x*(sf[245]/v64))))}else{v8g})});
        let v1eg=(v9s*v1ck);
        let v1ej=((v1eg+v1eg)/(v52*vb1));
        let v1ek=(if sb[10]{v1ej}else{v8g});
        let v1en=(if sb[10]{(v5b*(v1ck+v1ek))}else{v8g});
        let v1f0=(if sb[10]{(((vb8*v1cj)+(v9q*((-(((v64*v1en)-(vb5*v18w))/v1b3))*(sf[125]*f64::powf(vb7,sf[246])))))/sf[125])}else{v8g});
        let v1f1=(if sb[10]{v1ck}else{v8g});
        let v1f4=(vbc*v1f1);
        let v1f6=(vbc*sf[247]);
        let v1f8=(vbc*sf[248]);
        let v1fa=(v52*vbf);
        let v1fe=(if sb[10]{((v1f4+v1f4)/v1fa)}else{v8g});
        let v1ff=(if sb[10]{((v1f6+v1f6)/v1fa)}else{v8g});
        let v1fg=(if sb[10]{((v1f8+v1f8)/v1fa)}else{v8g});
        let v1fo=(if sb[10]{((v55*(v1f1-v1fe))-v1ck)}else{v8g});
        let v1fp=(if sb[10]{(v55*(sf[247]-v1ff))}else{v8g});
        let v1fq=(if sb[10]{(v55*(sf[248]-v1fg))}else{v8g});
        let v1g1=(sf[125]*f64::powf(vbm,sf[246]));
        let v1gd=(if sb[10]{(((vbn*v1cj)+(v9q*((-(((v64*v1fo)-(vbk*v18w))/v1b3))*v1g1)))/sf[125])}else{v1e4});
        let v1ge=(if sb[10]{((v9q*((-(v1fp/v64))*v1g1))/sf[125])}else{v1e5});
        let v1gf=(if sb[10]{((v9q*((-(v1fq/v64))*v1g1))/sf[125])}else{v1e6});
        let v1gr=(if sb[10]{((v1gd+(sf[130]*(v1en+(-v1fo))))-v1f0)}else{(if (sf[121]!=0.0){(v1e4+v1e7)}else{v8g})});
        let v1gs=(if sb[10]{(v1ge+(sf[130]*(v1f-v1fp)))}else{(if (sf[121]!=0.0){(v1e5+v1e8)}else{v8g})});
        let v1gt=(if sb[10]{(v1gf+(sf[130]*(va2-v1fq)))}else{(if (sf[121]!=0.0){(v1e6+v1e9)}else{v8g})});
        let v1gu=(if (sf[121]!=0.0){v1ck}else{v1cl});
        let v1i0=(sf[125]*f64::powf(vck,sf[246]));
        let v1if=(if vci{(((vcm*v18w)+(v64*(-((-((-(v9j*v18w))/v1b3))*v1i0))))/sf[125])}else{(if vc3{((vc7*v18w)/sf[125])}else{v1gd})});
        let v1ig=(if vci{((v64*(-(v1dl*v1i0)))/sf[125])}else{v8g});
        let v1ih=(if vci{v8g}else{(if vc3{v8g}else{v1ge})});
        let v1ii=(if vci{((v64*(-(v1dm*v1i0)))/sf[125])}else{(if vc3{v8g}else{v1gf})});
        let v1ij=(if vci{v8g}else{(if vc3{(vc4*((vcd*v1gu)+(vc0*(((v64*(sf[126]*v1gu))-(vcb*v18w))/v1b3))))}else{v1e7})});
        let v1ik=(if vci{v8g}else{(if vc3{(vc4*((vcd*sf[242])+(vc0*v1cy)))}else{v8g})});
        let v1il=(if vci{v8g}else{(if vc3{(vc4*((vcd*sf[249])+(vc0*(sf[251]/v64))))}else{v1e8})});
        let v1im=(if vci{v8g}else{(if vc3{(vc4*((vcd*sf[250])+(vc0*(sf[252]/v64))))}else{v1e9})});
        let v1iv=(if sb[10]{v1ej}else{v1ek});
        let v1iy=(if sb[10]{(v5b*(v1ck+v1iv))}else{v1en});
        let v1jb=(if sb[10]{(((vcz*v1cj)+(v9q*((-(((v64*v1iy)-(vcw*v18w))/v1b3))*(sf[125]*f64::powf(vcy,sf[246])))))/sf[125])}else{v1f0});
        let v1jc=(if sb[10]{v1ck}else{v1f1});
        let v1jf=(vd3*v1jc);
        let v1jh=(vd3*sf[247]);
        let v1jj=(vd3*sf[253]);
        let v1jl=(vd3*sf[254]);
        let v1jn=(v52*vd6);
        let v1js=(if sb[10]{((v1jf+v1jf)/v1jn)}else{v1fe});
        let v1jt=(if sb[10]{((v1jh+v1jh)/v1jn)}else{v8g});
        let v1ju=(if sb[10]{((v1jj+v1jj)/v1jn)}else{v1ff});
        let v1jv=(if sb[10]{((v1jl+v1jl)/v1jn)}else{v1fg});
        let v1k5=(if sb[10]{((v55*(v1jc-v1js))-v1ck)}else{v1fo});
        let v1k6=(if sb[10]{(v55*(sf[247]-v1jt))}else{v8g});
        let v1k7=(if sb[10]{(v55*(sf[253]-v1ju))}else{v1fp});
        let v1k8=(if sb[10]{(v55*(sf[254]-v1jv))}else{v1fq});
        let v1kl=(sf[125]*f64::powf(vdd,sf[246]));
        let v1l0=(if sb[10]{(((vde*v1cj)+(v9q*((-(((v64*v1k5)-(vdb*v18w))/v1b3))*v1kl)))/sf[125])}else{v1if});
        let v1l1=(if sb[10]{((v9q*((-(v1k6/v64))*v1kl))/sf[125])}else{v1ig});
        let v1l2=(if sb[10]{((v9q*((-(v1k7/v64))*v1kl))/sf[125])}else{v1ih});
        let v1l3=(if sb[10]{((v9q*((-(v1k8/v64))*v1kl))/sf[125])}else{v1ii});
        let v1lm=(-v19y);
        let v1ln=(sf[119]*v1lm);
        let v1lo=(if (sf[132]!=0.0){v1ln}else{v1gu});
        let v1m8=(sf[259]/v6v);
        let v1n3=((-(sf[137]*v19y))/v1bc);
        let v1n7=(v1n3*(sf[135]*f64::powf(vep,sf[263])));
        let v1nb=(vet*vet);
        let v1nw=((v6v*(-(veq*(-(sf[264]/vet)))))/sf[135]);
        let v1nx=((v6v*(-(veq*(-(sf[135]/vet)))))/sf[135]);
        let v1o9=(-(va2/v6v));
        let v1oa=(-(v1f/v6v));
        let v1oc=(sf[135]*f64::powf(vf4,sf[263]));
        let v1or=(if vf2{(((vf6*v19y)+(v6v*(-((-((-(v9l*v19y))/v1bc))*v1oc))))/sf[135])}else{(if ven{(((vex*v19y)+(v6v*(-((vev*v1n7)+(veq*(-((-(ves*v19y))/v1nb)))))))/sf[135])}else{(if vdx{((ve3*v19y)/sf[135])}else{v1l0})})});
        let v1os=(if vf2{((v6v*(-(v1o9*v1oc)))/sf[135])}else{(if ven{v1nw}else{v8g})});
        let v1ot=(if vf2{v8g}else{(if ven{v8g}else{(if vdx{v8g}else{v1l1})})});
        let v1ou=(if vf2{((v6v*(-(v1oa*v1oc)))/sf[135])}else{(if ven{v1nx}else{(if vdx{v8g}else{v1l2})})});
        let v1ov=(if vf2{v8g}else{(if ven{v8g}else{(if vdx{v8g}else{v1l3})})});
        let v1ow=(if vem{v8g}else{(if vdx{(ve0*((veb*v1lo)+(vdu*(((v6v*(sf[136]*v1lo))-(ve9*v19y))/v1bc))))}else{v1ij})});
        let v1ox=(if vem{v8g}else{(if vdx{(ve0*((veb*sf[255])+(vdu*v1m8)))}else{v8g})});
        let v1oy=(if vem{v8g}else{(if vdx{(ve0*((veb*sf[256])+(vdu*(sf[260]/v6v))))}else{v1ik})});
        let v1oz=(if vem{v8g}else{(if vdx{(ve0*((veb*sf[257])+(vdu*(sf[261]/v6v))))}else{v1il})});
        let v1p0=(if vem{v8g}else{(if vdx{(ve0*((veb*sf[258])+(vdu*(sf[262]/v6v))))}else{v1im})});
        let v1pb=(-v1ln);
        let v1pc=(vfk*v1ln);
        let v1pf=(vfk*vfk);
        let v1pg=((v1pc-(vfj*v1pb))/v1pf);
        let v1ph=(if sb[16]{v1pg}else{v8g});
        let v1pj=(vfo*v1ph);
        let v1pn=(vfu*v1ph);
        let v1px=(if sb[16]{(((vg0*(v52*v1ph))-(vfn*(((v1pj+v1pj)/(v52*vft))+((v1pn+v1pn)/(v52*vfz)))))/(vg0*vg0))}else{v8g});
        let v1q3=(if sb[16]{(v55*(((vg2*v1pb)+(vfk*v1px))-v1ln))}else{v1iy});
        let v1qh=(if sb[16]{(((vgb*v19y)+(v6v*(-((-(((v6v*v1q3)-(vg7*v19y))/v1bc))*(sf[135]*f64::powf(vg9,sf[263]))))))/sf[135])}else{v8g});
        let v1qo=(if sb[16]{((v1pc-(vgh*v1pb))/v1pf)}else{v8g});
        let v1qp=(if sb[16]{(-2.0/vfk)}else{v8g});
        let v1qq=(if sb[16]{(v52/vfk)}else{v8g});
        let v1qs=(v52*v1qp);
        let v1qt=(v52*v1qq);
        let v1qu=(vgl*v1qo);
        let v1qw=(vgl*v1qp);
        let v1qy=(vgl*v1qq);
        let v1r0=(v52*vgo);
        let v1r4=(vgp*v1qo);
        let v1r6=(vgp*v1qp);
        let v1r8=(vgp*v1qq);
        let v1ra=(v52*vgs);
        let v1rk=(vgt*vgt);
        let v1ru=(if sb[16]{(((vgt*(v52*v1qo))-(vgk*(((v1qu+v1qu)/v1r0)+((v1r4+v1r4)/v1ra))))/v1rk)}else{v8g});
        let v1rv=(if sb[16]{(((vgt*v1qs)-(vgk*(((v1qw+v1qw)/v1r0)+((v1r6+v1r6)/v1ra))))/v1rk)}else{v8g});
        let v1rw=(if sb[16]{(((vgt*v1qt)-(vgk*(((v1qy+v1qy)/v1r0)+((v1r8+v1r8)/v1ra))))/v1rk)}else{v8g});
        let v1s6=(if sb[16]{(v55*(((vgv*v1pb)+(vfk*v1ru))-v1ln))}else{v1k5});
        let v1s7=(if sb[16]{(v55*(vfk*v1rv))}else{v8g});
        let v1s8=(if sb[16]{v8g}else{v1k6});
        let v1s9=(if sb[16]{(v55*(vfk*v1rw))}else{v1k7});
        let v1sa=(if sb[16]{v8g}else{v1k8});
        let v1sp=(sf[135]*f64::powf(vh2,sf[263]));
        let v1tc=(if sb[16]{(((vh4*v19y)+(v6v*(-((-(((v6v*v1s6)-(vh0*v19y))/v1bc))*v1sp))))/sf[135])}else{v1or});
        let v1td=(if sb[16]{((v6v*(-((-(v1s7/v6v))*v1sp)))/sf[135])}else{v1os});
        let v1te=(if sb[16]{((v6v*(-((-(v1s8/v6v))*v1sp)))/sf[135])}else{v1ot});
        let v1tf=(if sb[16]{((v6v*(-((-(v1s9/v6v))*v1sp)))/sf[135])}else{v1ou});
        let v1tg=(if sb[16]{((v6v*(-((-(v1sa/v6v))*v1sp)))/sf[135])}else{v1ov});
        let v1tk=(if sb[16]{(v55*v1ru)}else{v8g});
        let v1tl=(if sb[16]{(v55*v1rv)}else{v8g});
        let v1tm=(if sb[16]{(v55*v1rw)}else{v8g});
        let v1tq=(v1n3*(sf[145]*f64::powf(vep,sf[265])));
        let v1tr=(if sb[16]{v1tq}else{v8g});
        let v1ty=((((v6v*v1ln)-(vdp*v19y))/v1bc)*(sf[145]*f64::powf(vhf,sf[265])));
        let v1tz=(if sb[16]{v1ty}else{v8g});
        let v1ug=(if sb[16]{(((vhi*v1tr)+(vhd*(-v1tk)))+((vhh*v1tk)+(vha*v1tz)))}else{v8g});
        let v1uh=(if sb[16]{((vhd*(-v1tl))+(vhh*v1tl))}else{v8g});
        let v1ui=(if sb[16]{((vhd*(-v1tm))+(vhh*v1tm))}else{v8g});
        let v1v0=(if sb[16]{((vho*v1ug)+(vhm*(v1q3+(-v1s6))))}else{v8g});
        let v1v1=(if sb[16]{((vho*v1uh)+(vhm*(va2-v1s7)))}else{v8g});
        let v1v2=(if sb[16]{(vhm*(-v1s8))}else{v8g});
        let v1v3=(if sb[16]{((vho*v1ui)+(vhm*(v1f-v1s9)))}else{v8g});
        let v1v4=(if sb[16]{(vhm*(-v1sa))}else{v8g});
        let v1vg=(vdp*v1ln);
        let v1vj=((v1vg+v1vg)/(v52*vhy));
        let v1vk=(if sb[18]{v1vj}else{v1iv});
        let v1vn=(if sb[18]{(v5b*(v1ln+v1vk))}else{v1q3});
        let v1w0=(if sb[18]{(((vi5*v1lm)+(vdo*((-(((v6v*v1vn)-(vi2*v19y))/v1bc))*(sf[135]*f64::powf(vi4,sf[263])))))/sf[135])}else{v1jb});
        let v1w1=(if sb[18]{v1ln}else{v1jc});
        let v1w6=(vi9*v1w1);
        let v1w8=(vi9*sf[266]);
        let v1wa=(vi9*sf[267]);
        let v1wc=(vi9*sf[268]);
        let v1we=(vi9*sf[269]);
        let v1wg=(v52*vic);
        let v1wm=(if sb[18]{((v1w6+v1w6)/v1wg)}else{v1js});
        let v1wn=(if sb[18]{((v1w8+v1w8)/v1wg)}else{v8g});
        let v1wo=(if sb[18]{((v1wa+v1wa)/v1wg)}else{v1jt});
        let v1wp=(if sb[18]{((v1wc+v1wc)/v1wg)}else{v1ju});
        let v1wq=(if sb[18]{((v1we+v1we)/v1wg)}else{v1jv});
        let v1x2=(if sb[18]{((v55*(v1w1-v1wm))-v1ln)}else{v1s6});
        let v1x3=(if sb[18]{(v55*(sf[266]-v1wn))}else{v1s7});
        let v1x4=(if sb[18]{(v55*(sf[267]-v1wo))}else{v1s8});
        let v1x5=(if sb[18]{(v55*(sf[268]-v1wp))}else{v1s9});
        let v1x6=(if sb[18]{(v55*(sf[269]-v1wq))}else{v1sa});
        let v1xl=(sf[135]*f64::powf(vij,sf[263]));
        let v1y3=(if sb[18]{(((vik*v1lm)+(vdo*((-(((v6v*v1x2)-(vih*v19y))/v1bc))*v1xl)))/sf[135])}else{v1tc});
        let v1y4=(if sb[18]{((vdo*((-(v1x3/v6v))*v1xl))/sf[135])}else{v1td});
        let v1y5=(if sb[18]{((vdo*((-(v1x4/v6v))*v1xl))/sf[135])}else{v1te});
        let v1y6=(if sb[18]{((vdo*((-(v1x5/v6v))*v1xl))/sf[135])}else{v1tf});
        let v1y7=(if sb[18]{((vdo*((-(v1x6/v6v))*v1xl))/sf[135])}else{v1tg});
        let v1yp=(if sb[18]{((v1y3+(sf[146]*(v1vn+(-v1x2))))-v1w0)}else{(if sb[16]{((v1tc+v1v0)-v1qh)}else{(if (sf[132]!=0.0){(v1or+v1ow)}else{v8g})})});
        let v1yq=(if sb[18]{(v1y4+(sf[146]*(va2-v1x3)))}else{(if sb[16]{(v1td+v1v1)}else{(if (sf[132]!=0.0){(v1os+v1ox)}else{v8g})})});
        let v1yr=(if sb[18]{(v1y5+(sf[146]*(-v1x4)))}else{(if sb[16]{(v1te+v1v2)}else{(if (sf[132]!=0.0){(v1ot+v1oy)}else{v8g})})});
        let v1ys=(if sb[18]{(v1y6+(sf[146]*(v1f-v1x5)))}else{(if sb[16]{(v1tf+v1v3)}else{(if (sf[132]!=0.0){(v1ou+v1oz)}else{v8g})})});
        let v1yt=(if sb[18]{(v1y7+(sf[146]*(-v1x6)))}else{(if sb[16]{(v1tg+v1v4)}else{(if (sf[132]!=0.0){(v1ov+v1p0)}else{v8g})})});
        let v1yu=(if (sf[132]!=0.0){v1ln}else{v1lo});
        let v210=(sf[135]*f64::powf(vjv,sf[263]));
        let v21f=(if vjt{(((vjx*v19y)+(v6v*(-((-((-(v9p*v19y))/v1bc))*v210))))/sf[135])}else{(if vji{(((vjo*v19y)+(v6v*(-((vjm*v1n7)+(veq*(-((-(vjk*v19y))/v1nb)))))))/sf[135])}else{(if viz{((vj3*v19y)/sf[135])}else{v1y3})})});
        let v21g=(if vjt{v8g}else{(if vji{v8g}else{(if viz{v8g}else{v1y4})})});
        let v21h=(if vjt{((v6v*(-(v1oa*v210)))/sf[135])}else{(if vji{v1nx}else{(if viz{v8g}else{v1y5})})});
        let v21i=(if vjt{v8g}else{(if vji{v8g}else{(if viz{v8g}else{v1y6})})});
        let v21j=(if vjt{v8g}else{(if vji{v8g}else{(if viz{v8g}else{v1y7})})});
        let v21k=(if vjt{((v6v*(-(v1o9*v210)))/sf[135])}else{(if vji{v1nw}else{v8g})});
        let v21l=(if vjh{v8g}else{(if viz{(vj0*((vj9*v1yu)+(viw*(((v6v*(sf[136]*v1yu))-(vj7*v19y))/v1bc))))}else{v1ow})});
        let v21m=(if vjh{v8g}else{(if viz{(vj0*((vj9*sf[270])+(viw*(sf[274]/v6v))))}else{v1ox})});
        let v21n=(if vjh{v8g}else{(if viz{(vj0*((vj9*sf[271])+(viw*(sf[275]/v6v))))}else{v1oy})});
        let v21o=(if vjh{v8g}else{(if viz{(vj0*((vj9*sf[272])+(viw*(sf[276]/v6v))))}else{v1oz})});
        let v21p=(if vjh{v8g}else{(if viz{(vj0*((vj9*sf[273])+(viw*(sf[277]/v6v))))}else{v1p0})});
        let v21q=(if vjh{v8g}else{(if viz{(vj0*((vj9*sf[255])+(viw*v1m8)))}else{v8g})});
        let v223=(if sb[16]{v1pg}else{v1ph});
        let v225=(vk6*v223);
        let v229=(vka*v223);
        let v22p=(if sb[16]{(v55*(((vkg*v1pb)+(vfk*(if sb[16]{(((vke*(v52*v223))-(vk5*(((v225+v225)/(v52*vk9))+((v229+v229)/(v52*vkd)))))/(vke*vke))}else{v1px})))-v1ln))}else{v1vn});
        let v237=(if sb[16]{((v1pc-(vkv*v1pb))/v1pf)}else{v1qo});
        let v238=(if sb[16]{v8g}else{v1qp});
        let v239=(if sb[16]{v8g}else{v1qq});
        let v23d=(vkz*v237);
        let v23f=(vkz*v238);
        let v23h=(vkz*v1qq);
        let v23j=(vkz*v239);
        let v23l=(vkz*v1qp);
        let v23n=(v52*vl2);
        let v23t=(vl3*v237);
        let v23v=(vl3*v238);
        let v23x=(vl3*v1qq);
        let v23z=(vl3*v239);
        let v241=(vl3*v1qp);
        let v243=(v52*vl6);
        let v24h=(vl7*vl7);
        let v24z=(if sb[16]{(((vl7*(v52*v237))-(vky*(((v23d+v23d)/v23n)+((v23t+v23t)/v243))))/v24h)}else{v1ru});
        let v250=(if sb[16]{(((vl7*(v52*v238))-(vky*(((v23f+v23f)/v23n)+((v23v+v23v)/v243))))/v24h)}else{v1rv});
        let v251=(if sb[16]{(((vl7*v1qt)-(vky*(((v23h+v23h)/v23n)+((v23x+v23x)/v243))))/v24h)}else{v8g});
        let v252=(if sb[16]{(((vl7*(v52*v239))-(vky*(((v23j+v23j)/v23n)+((v23z+v23z)/v243))))/v24h)}else{v1rw});
        let v253=(if sb[16]{(((vl7*v1qs)-(vky*(((v23l+v23l)/v23n)+((v241+v241)/v243))))/v24h)}else{v8g});
        let v25h=(if sb[16]{(v55*(((vl9*v1pb)+(vfk*v24z))-v1ln))}else{v1x2});
        let v25i=(if sb[16]{(v55*(vfk*v250))}else{v1x3});
        let v25j=(if sb[16]{(v55*(vfk*v251))}else{v1x4});
        let v25k=(if sb[16]{(v55*(vfk*v252))}else{v1x5});
        let v25l=(if sb[16]{v8g}else{v1x6});
        let v25m=(if sb[16]{(v55*(vfk*v253))}else{v8g});
        let v263=(sf[135]*f64::powf(vlg,sf[263]));
        let v26u=(if sb[16]{(((vli*v19y)+(v6v*(-((-(((v6v*v25h)-(vle*v19y))/v1bc))*v263))))/sf[135])}else{v21f});
        let v26v=(if sb[16]{((v6v*(-((-(v25i/v6v))*v263)))/sf[135])}else{v21g});
        let v26w=(if sb[16]{((v6v*(-((-(v25j/v6v))*v263)))/sf[135])}else{v21h});
        let v26x=(if sb[16]{((v6v*(-((-(v25k/v6v))*v263)))/sf[135])}else{v21i});
        let v26y=(if sb[16]{((v6v*(-((-(v25l/v6v))*v263)))/sf[135])}else{v21j});
        let v26z=(if sb[16]{((v6v*(-((-(v25m/v6v))*v263)))/sf[135])}else{v21k});
        let v275=(if sb[16]{(v55*v24z)}else{v1tk});
        let v276=(if sb[16]{(v55*v250)}else{v1tl});
        let v277=(if sb[16]{(v55*v251)}else{v8g});
        let v278=(if sb[16]{(v55*v252)}else{v1tm});
        let v279=(if sb[16]{(v55*v253)}else{v8g});
        let v29b=(if sb[18]{v1vj}else{v1vk});
        let v29e=(if sb[18]{(v5b*(v1ln+v29b))}else{v22p});
        let v29r=(if sb[18]{(((vm9*v1lm)+(vdo*((-(((v6v*v29e)-(vm6*v19y))/v1bc))*(sf[135]*f64::powf(vm8,sf[263])))))/sf[135])}else{v1w0});
        let v29s=(if sb[18]{v1ln}else{v1w1});
        let v29x=(vmd*v29s);
        let v29z=(vmd*sf[278]);
        let v2a1=(vmd*sf[279]);
        let v2a3=(vmd*sf[280]);
        let v2a5=(vmd*sf[281]);
        let v2a7=(vmd*sf[266]);
        let v2a9=(v52*vmg);
        let v2ag=(if sb[18]{((v29x+v29x)/v2a9)}else{v1wm});
        let v2ah=(if sb[18]{((v29z+v29z)/v2a9)}else{v1wn});
        let v2ai=(if sb[18]{((v2a1+v2a1)/v2a9)}else{v1wo});
        let v2aj=(if sb[18]{((v2a3+v2a3)/v2a9)}else{v1wp});
        let v2ak=(if sb[18]{((v2a5+v2a5)/v2a9)}else{v1wq});
        let v2al=(if sb[18]{((v2a7+v2a7)/v2a9)}else{v8g});
        let v2az=(if sb[18]{((v55*(v29s-v2ag))-v1ln)}else{v25h});
        let v2b0=(if sb[18]{(v55*(sf[278]-v2ah))}else{v25i});
        let v2b1=(if sb[18]{(v55*(sf[279]-v2ai))}else{v25j});
        let v2b2=(if sb[18]{(v55*(sf[280]-v2aj))}else{v25k});
        let v2b3=(if sb[18]{(v55*(sf[281]-v2ak))}else{v25l});
        let v2b4=(if sb[18]{(v55*(sf[266]-v2al))}else{v25m});
        let v2bl=(sf[135]*f64::powf(vmn,sf[263]));
        let v2c6=(if sb[18]{(((vmo*v1lm)+(vdo*((-(((v6v*v2az)-(vml*v19y))/v1bc))*v2bl)))/sf[135])}else{v26u});
        let v2c7=(if sb[18]{((vdo*((-(v2b0/v6v))*v2bl))/sf[135])}else{v26v});
        let v2c8=(if sb[18]{((vdo*((-(v2b1/v6v))*v2bl))/sf[135])}else{v26w});
        let v2c9=(if sb[18]{((vdo*((-(v2b2/v6v))*v2bl))/sf[135])}else{v26x});
        let v2ca=(if sb[18]{((vdo*((-(v2b3/v6v))*v2bl))/sf[135])}else{v26y});
        let v2cb=(if sb[18]{((vdo*((-(v2b4/v6v))*v2bl))/sf[135])}else{v26z});
        let v2d2=(-v1b0);
        let v2d4=(if (sf[147]!=0.0){(sf[119]*v2d2)}else{v1ln});
        let v2d5=(if sb[21]{v2d4}else{v1yu});
        let v2fa=(sf[152]*f64::powf(vny,sf[294]));
        let v2fp=(if vnw{(((vo0*v1b0)+(v7m*(-((-((-(vn8*v1b0))/v1bl))*v2fa))))/sf[152])}else{(if vnd{((vnj*v1b0)/sf[152])}else{v2c6})});
        let v2fq=(if vnw{v8g}else{(if vnd{v8g}else{v2c7})});
        let v2fr=(if vnw{v8g}else{(if vnd{v8g}else{v2c8})});
        let v2fs=(if vnw{v8g}else{(if vnd{v8g}else{v2c9})});
        let v2ft=(if vnw{v8g}else{(if vnd{v8g}else{v2ca})});
        let v2fu=(if vnw{((v7m*(-((-(va2/v7m))*v2fa)))/sf[152])}else{(if vnd{v8g}else{v2cb})});
        let v2fv=(if vnw{((v7m*(-((-(v1f/v7m))*v2fa)))/sf[152])}else{v8g});
        let v2gh=(vn2*v2d4);
        let v2go=(if sb[23]{(v5b*(v2d4+(if sb[23]{((v2gh+v2gh)/(v52*vod))}else{v29b})))}else{v29e});
        let v2h2=(if sb[23]{v2d4}else{v29s});
        let v2h9=(voo*v2h2);
        let v2hb=(voo*sf[295]);
        let v2hd=(voo*sf[296]);
        let v2hf=(voo*sf[297]);
        let v2hh=(voo*sf[298]);
        let v2hj=(voo*sf[299]);
        let v2hl=(voo*sf[300]);
        let v2hn=(v52*vor);
        let v2ih=(if sb[23]{((v55*(v2h2-(if sb[23]{((v2h9+v2h9)/v2hn)}else{v2ag})))-v2d4)}else{v2az});
        let v2ii=(if sb[23]{(v55*(sf[295]-(if sb[23]{((v2hb+v2hb)/v2hn)}else{v2ah})))}else{v2b0});
        let v2ij=(if sb[23]{(v55*(sf[296]-(if sb[23]{((v2hd+v2hd)/v2hn)}else{v2ai})))}else{v2b1});
        let v2ik=(if sb[23]{(v55*(sf[297]-(if sb[23]{((v2hf+v2hf)/v2hn)}else{v2aj})))}else{v2b2});
        let v2il=(if sb[23]{(v55*(sf[298]-(if sb[23]{((v2hh+v2hh)/v2hn)}else{v2ak})))}else{v2b3});
        let v2im=(if sb[23]{(v55*(sf[299]-(if sb[23]{((v2hj+v2hj)/v2hn)}else{v2al})))}else{v2b4});
        let v2in=(if sb[23]{(v55*(sf[300]-(if sb[23]{((v2hl+v2hl)/v2hn)}else{v8g})))}else{v8g});
        let v2j6=(sf[152]*f64::powf(voy,sf[294]));
        let v2lb=scalar_limexp_derivative(vpe);
        let v2lh=((vpg*v13u)+(v1o*(((-(v9h*((v4i*v125)+(vb*sf[226]))))/(vpd*vpd))*v2lb)));
        let v2li=(v1o*((v1f/vpd)*v2lb));
        let v2lj=(v1o*((va2/vpd)*v2lb));
        let v2lq=((-(v9l*((v4j*v125)+(vb*sf[227]))))/(vpi*vpi));
        let v2lr=(va2/vpi);
        let v2ls=(v1f/vpi);
        let v2lt=scalar_limexp_derivative(vpj);
        let v2lu=(v2lq*v2lt);
        let v2lv=(v2lr*v2lt);
        let v2lw=(v2ls*v2lt);
        let v2m2=((vpm*((v21*v13u)+(v1o*(sf[25]*(((v1w*(sf[194]*(sf[26]*f64::powf(vc,sf[207]))))+(v1r*(v1w*(((vb*sf[208])-(v1u*v125))/v13k))))*(sf[30]*f64::powf(v1x,sf[209])))))))+(vpl*v2lu));
        let v2m3=(vpl*v2lv);
        let v2m4=(vpl*v2lw);
        let v2m9=(sf[99]*v1yq);
        let v2ma=(sf[99]*v1yr);
        let v2md=((sf[102]*v1gr)+(sf[99]*v1yp));
        let v2me=((sf[102]*v1gs)+(sf[99]*v1ys));
        let v2mf=((sf[102]*v1gt)+(sf[99]*v1yt));
        let v2mg=(vpt*v2md);
        let v2mi=(vpt*v2m9);
        let v2mk=(vpt*v2ma);
        let v2mm=(vpt*v2me);
        let v2mo=(vpt*v2mf);
        let v2mq=(v52*vpx);
        let v2n1=(v55*(v2md+((v2mg+v2mg)/v2mq)));
        let v2n2=(v55*(v2m9+((v2mi+v2mi)/v2mq)));
        let v2n3=(v55*(v2ma+((v2mk+v2mk)/v2mq)));
        let v2n4=(v55*(v2me+((v2mm+v2mm)/v2mq)));
        let v2n5=(v55*(v2mf+((v2mo+v2mo)/v2mq)));
        let v2ni=(sf[161]*f64::powf(vq1,sf[301]));
        let v2no=(v5t*(((vph*(if sb[2]{((-(sf[3]*(sf[194]*(sf[4]*f64::powf(vc,sf[195])))))/(vh*vh))}else{v8g}))+(v8q*v2lh))+(sf[105]*v2m2)));
        let v2np=(v5t*(sf[105]*v2m3));
        let v2nq=(v5t*((v8q*v2li)+(sf[105]*v2m4)));
        let v2nr=(v5t*(v8q*v2lj));
        let v2ny=(sf[160]*f64::powf(vqc,sf[302]));
        let v2op=(sf[160]*f64::powf(vqj,sf[302]));
        let v2p7=(if sb[26]{((vql*(v55*v2n1))+(vqi*(v2no*v2op)))}else{(if (sf[159]!=0.0){(v55*(v2n1+(((v2n1*v2ni)+v2no)*v2ny)))}else{v8g})});
        let v2p8=(if sb[26]{((vql*(v55*v2n2))+(vqi*(v2np*v2op)))}else{(if (sf[159]!=0.0){(v55*(v2n2+(((v2n2*v2ni)+v2np)*v2ny)))}else{v8g})});
        let v2p9=(if sb[26]{(vql*(v55*v2n3))}else{(if (sf[159]!=0.0){(v55*(v2n3+((v2n3*v2ni)*v2ny)))}else{v8g})});
        let v2pa=(if sb[26]{((vql*(v55*v2n4))+(vqi*(v2nq*v2op)))}else{(if (sf[159]!=0.0){(v55*(v2n4+(((v2n4*v2ni)+v2nq)*v2ny)))}else{v8g})});
        let v2pb=(if sb[26]{((vql*(v55*v2n5))+(vqi*(v2nr*v2op)))}else{(if (sf[159]!=0.0){(v55*(v2n5+(((v2n5*v2ni)+v2nr)*v2ny)))}else{v8g})});
        let v2pf=(vqn*vqn);
        let v2qg=(vqs*vqs);
        let v2qj=(va2/vqs);
        let v2qk=(if (sf[162]!=0.0){((-(v9p*sf[303]))/v2qg)}else{v2lq});
        let v2ql=(if (sf[162]!=0.0){v8g}else{v2lr});
        let v2qm=(if (sf[162]!=0.0){(v1f/vqs)}else{v8g});
        let v2qn=(if (sf[162]!=0.0){v8g}else{v2ls});
        let v2qo=(if (sf[162]!=0.0){v2qj}else{v8g});
        let v2qp=scalar_limexp_derivative(vqu);
        let v2qv=(if (sf[162]!=0.0){(v2qk*v2qp)}else{v2lu});
        let v2qw=(if (sf[162]!=0.0){(v2ql*v2qp)}else{v2lv});
        let v2qx=(if (sf[162]!=0.0){(v2qm*v2qp)}else{v8g});
        let v2qy=(if (sf[162]!=0.0){(v2qn*v2qp)}else{v2lw});
        let v2qz=(if (sf[162]!=0.0){(v2qo*v2qp)}else{v8g});
        let v2r3=(if (sf[162]!=0.0){((-(v9l*sf[303]))/v2qg)}else{v8g});
        let v2r4=scalar_limexp_derivative(vqy);
        let v2r8=(if (sf[162]!=0.0){(v2r3*v2r4)}else{v8g});
        let v2r9=(if (sf[162]!=0.0){(v2qo*v2r4)}else{v8g});
        let v2ra=(if (sf[162]!=0.0){(v2qm*v2r4)}else{v8g});
        let v2rt=(if (sf[162]!=0.0){((vr6*v14q)+(v2c*((sf[163]*v2qv)+(sf[164]*v2r8))))}else{v8g});
        let v2ru=(if (sf[162]!=0.0){(v2c*((sf[163]*v2qw)+(sf[164]*v2r9)))}else{v8g});
        let v2rv=(if (sf[162]!=0.0){(v2c*(sf[163]*v2qx))}else{v8g});
        let v2rw=(if (sf[162]!=0.0){(v2c*((sf[163]*v2qy)+(sf[164]*v2ra)))}else{v8g});
        let v2rx=(if (sf[162]!=0.0){(v2c*(sf[163]*v2qz))}else{v8g});
        let v3dy=((-(v9l*v125))/v13k);
        let v3dz=(va2/vb);
        let v3e0=(v1f/vb);
        let v3e1=scalar_limexp_derivative(vwr);
        let v3e2=(v3dy*v3e1);
        let v3e3=(v3dz*v3e1);
        let v3e4=(v3e0*v3e1);
        let v3e8=scalar_limexp_derivative(vwt);
        let v3eh=(v52*vwx);
        let v3ei=(((vws*v1bv)+(v86*v3e2))/v3eh);
        let v3ej=((v86*v3e3)/v3eh);
        let v3ek=((v86*v3e4)/v3eh);
        let v3eq=(v52*vx0);
        let v3er=(((vwu*v1bv)+(v86*(((-(v9n*v125))/v13k)*v3e8)))/v3eq);
        let v3es=((v86*(v3dz*v3e8))/v3eq);
        let v3et=((v86*(v3e0*v3e8))/v3eq);
        let v3n6=(sf[117]*(vzo*v2lh));
        let v3n7=(sf[117]*(vzo*v2li));
        let v3n8=(sf[117]*(vzo*v2lj));
        let v3nc=(vzr*vzr);
        let v3nz=scalar_limexp_derivative(v101);
        let v3o4=(vzs*(((vzr*v3n6)-(vzq*v3n6))/v3nc));
        let v3o6=(vzs*(((vzr*v3n7)-(vzq*v3n7))/v3nc));
        let v3o8=(vzs*(((vzr*v3n8)-(vzq*v3n8))/v3nc));
        let v3pn=(((vqn*(vph*((v108*(sf[183]*(sf[184]*v2n2)))+(vzx*(vzo*(v105*(sf[185]*(sf[313]*v3nz))))))))-(v10c*v2p8))/v2pf);
        let v3pr=(((vqn*(vph*(v108*(sf[183]*(sf[184]*v2n3)))))-(v10c*v2p9))/v2pf);
        let v3q0=((sf[165]*((vby*v1b9)+(v7r*v1gr)))+(((vqn*((v109*v2lh)+(vph*((v108*(sf[183]*(sf[184]*v2n1)))+(vzx*(vzo*(v103*(v3o4+v3o4))))))))-(v10c*v2p7))/v2pf));
        let v3q1=((sf[165]*(v7r*v1gs))+(((vqn*((v109*v2li)+(vph*((v108*(sf[183]*(sf[184]*v2n4)))+(vzx*(vzo*((v105*(sf[185]*(sf[314]*v3nz)))+(v103*(v3o6+v3o6)))))))))-(v10c*v2pa))/v2pf));
        let v3q2=((sf[165]*(v7r*v1gt))+(((vqn*((v109*v2lj)+(vph*((v108*(sf[183]*(sf[184]*v2n5)))+(vzx*(vzo*(v103*(v3o8+v3o8))))))))-(v10c*v2pb))/v2pf));
        let v3q9=(sf[170]*((vdn*v1b9)+(v7r*(if sb[10]{((v1l0+(sf[130]*(v1iy+(-v1k5))))-v1jb)}else{(if (sf[121]!=0.0){(v1if+v1ij)}else{v8g})}))));
        let v3qa=(sf[170]*(v7r*(if sb[10]{(v1l1+(sf[130]*(v1f-v1k6)))}else{(if (sf[121]!=0.0){(v1ig+v1ik)}else{v8g})})));
        let v3qb=(sf[170]*(v7r*(if sb[10]{(v1l2+(sf[130]*(-v1k7)))}else{(if (sf[121]!=0.0){(v1ih+v1il)}else{v8g})})));
        let v3qc=(sf[170]*(v7r*(if sb[10]{(v1l3+(sf[130]*(va2-v1k8)))}else{(if (sf[121]!=0.0){(v1ii+v1im)}else{v8g})})));
        let v3qh=(v7w*v1yr);
        let v3qj=(v7w*v1yt);
        let v3qt=((((viu*(sf[89]*v1bg))+(v7w*v1yp))+(sf[186]*v2m2))+(sf[187]*v3ei));
        let v3qu=(((v7w*v1yq)+(sf[186]*v2m3))+(sf[187]*v3ej));
        let v3qv=(((v7w*v1ys)+(sf[186]*v2m4))+(sf[187]*v3ek));
        let v3qw=(sf[187]*v3er);
        let v3qx=(sf[187]*v3es);
        let v3qy=(sf[187]*v3et);
        let v3r5=(v7y*(if sb[18]{(v2ca+(sf[146]*(-v2b3)))}else{(if sb[16]{(v26y+(if sb[16]{(vlv*(-v25l))}else{v1v4}))}else{(if (sf[132]!=0.0){(v21j+v21p)}else{v8g})})}));
        let v3rc=(((vmx*(sf[91]*v1bg))+(v7y*(if sb[18]{((v2c6+(sf[146]*(v29e+(-v2az))))-v29r)}else{(if sb[16]{((v26u+(if sb[16]{((vlx*(if sb[16]{(((vlr*(if sb[16]{v1tq}else{v1tr}))+(vlp*(-v275)))+((vlq*v275)+(vlo*(if sb[16]{v1ty}else{v1tz}))))}else{v1ug}))+(vlv*(v22p+(-v25h))))}else{v1v0}))-(if sb[16]{(((vkp*v19y)+(v6v*(-((-(((v6v*v22p)-(vkl*v19y))/v1bc))*(sf[135]*f64::powf(vkn,sf[263]))))))/sf[135])}else{v1qh}))}else{(if (sf[132]!=0.0){(v21f+v21l)}else{v8g})})})))+(sf[186]*(if sb[28]{v8g}else{v2rt})));
        let v3rd=((v7y*(if sb[18]{(v2c7+(sf[146]*(-v2b0)))}else{(if sb[16]{(v26v+(if sb[16]{((vlx*(if sb[16]{((vlp*(-v276))+(vlq*v276))}else{v1uh}))+(vlv*(-v25i)))}else{v1v1}))}else{(if (sf[132]!=0.0){(v21g+v21m)}else{v8g})})}))+(sf[186]*(if sb[28]{v8g}else{v2ru})));
        let v3re=((v7y*(if sb[18]{(v2c8+(sf[146]*(v1f-v2b1)))}else{(if sb[16]{(v26w+(if sb[16]{((vlx*(if sb[16]{((vlp*(-v277))+(vlq*v277))}else{v8g}))+(vlv*(v1f-v25j)))}else{v1v2}))}else{(if (sf[132]!=0.0){(v21h+v21n)}else{v8g})})}))+(sf[186]*(if sb[28]{v8g}else{v2rv})));
        let v3rf=((v7y*(if sb[18]{(v2c9+(sf[146]*(-v2b2)))}else{(if sb[16]{(v26x+(if sb[16]{((vlx*(if sb[16]{((vlp*(-v278))+(vlq*v278))}else{v1ui}))+(vlv*(-v25k)))}else{v1v3}))}else{(if (sf[132]!=0.0){(v21i+v21o)}else{v8g})})}))+(sf[186]*(if sb[28]{v8g}else{v2rw})));
        let v3rg=((v7y*(if sb[18]{(v2cb+(sf[146]*(va2-v2b4)))}else{(if sb[16]{(v26z+(if sb[16]{((vlx*(if sb[16]{((vlp*(-v279))+(vlq*v279))}else{v8g}))+(vlv*(va2-v25m)))}else{v8g}))}else{(if (sf[132]!=0.0){(v21k+v21q)}else{v8g})})}))+(sf[186]*(if sb[28]{v8g}else{v2rx})));
        let v3rj=((vpc*(sf[92]*(((-(sf[84]*v1b0))/v1bl)*(sf[93]*f64::powf(v80,sf[240])))))+(v83*(if sb[24]{v8g}else{(if sb[23]{(((if sb[23]{(((voz*v2d2)+(vn0*((-(((v7m*v2ih)-(vow*v1b0))/v1bl))*v2j6)))/sf[152])}else{v2fp})+(sf[157]*(v2go+(-v2ih))))-(if sb[23]{(((vok*v2d2)+(vn0*((-(((v7m*v2go)-(voh*v1b0))/v1bl))*(sf[152]*f64::powf(voj,sf[294])))))/sf[152])}else{v29r}))}else{(if sb[21]{(v2fp+(if vnw{v8g}else{(if vnd{(vng*((vnr*v2d5)+(vna*(((v7m*(sf[153]*v2d5))-(vnp*v1b0))/v1bl))))}else{v21l})}))}else{v8g})})})));
        let v3rk=(v83*(if sb[24]{v8g}else{(if sb[23]{((if sb[23]{((vn0*((-(v2ii/v7m))*v2j6))/sf[152])}else{v2fq})+(sf[157]*(-v2ii)))}else{(if sb[21]{(v2fq+(if vnw{v8g}else{(if vnd{(vng*((vnr*sf[282])+(vna*(sf[288]/v7m))))}else{v21m})}))}else{v8g})})}));
        let v3rl=(v83*(if sb[24]{v8g}else{(if sb[23]{((if sb[23]{((vn0*((-(v2ij/v7m))*v2j6))/sf[152])}else{v2fr})+(sf[157]*(-v2ij)))}else{(if sb[21]{(v2fr+(if vnw{v8g}else{(if vnd{(vng*((vnr*sf[283])+(vna*(sf[289]/v7m))))}else{v21n})}))}else{v8g})})}));
        let v3rm=(v83*(if sb[24]{v8g}else{(if sb[23]{((if sb[23]{((vn0*((-(v2ik/v7m))*v2j6))/sf[152])}else{v2fs})+(sf[157]*(-v2ik)))}else{(if sb[21]{(v2fs+(if vnw{v8g}else{(if vnd{(vng*((vnr*sf[284])+(vna*(sf[290]/v7m))))}else{v21o})}))}else{v8g})})}));
        let v3rn=(v83*(if sb[24]{v8g}else{(if sb[23]{((if sb[23]{((vn0*((-(v2il/v7m))*v2j6))/sf[152])}else{v2ft})+(sf[157]*(-v2il)))}else{(if sb[21]{(v2ft+(if vnw{v8g}else{(if vnd{(vng*((vnr*sf[285])+(vna*(sf[291]/v7m))))}else{v21p})}))}else{v8g})})}));
        let v3rr=((v83*(if sb[24]{v8g}else{(if sb[23]{((if sb[23]{((vn0*((-(v2im/v7m))*v2j6))/sf[152])}else{v2fu})+(sf[157]*(va2-v2im)))}else{(if sb[21]{(v2fu+(if vnw{v8g}else{(if vnd{(vng*((vnr*sf[286])+(vna*(sf[292]/v7m))))}else{v21q})}))}else{v8g})})}))+sf[315]);
        let v3rs=(sf[188]+(v83*(if sb[24]{v8g}else{(if sb[23]{((if sb[23]{((vn0*((-(v2in/v7m))*v2j6))/sf[152])}else{v2fv})+(sf[157]*(v1f-v2in)))}else{(if sb[21]{(v2fv+(if vnw{v8g}else{(if vnd{(vng*((vnr*sf[287])+(vna*(sf[293]/v7m))))}else{v8g})}))}else{v8g})})})));

        CommonStampValues {
            v6, vb, vc, vd, v1f, v1g, v2c, v52, 
            v55, v5t, v6v, v8g, v9f, v9g, v9h, v9i, 
            v9j, v9k, v9l, v9m, v9o, v9p, va2, vn7, 
            vn8, vow, vph, vpn, vqn, vqs, vqu, vqw, 
            vqy, vr0, vr8, vwl, vwr, vws, vwx, vx0, 
            vxz, vyf, v10e, v10g, v10n, v10o, v10r, v10v, 
            v10y, v111, v124, v125, v13k, v14q, v19y, v2ih, 
            v2ii, v2ij, v2ik, v2il, v2im, v2in, v2lh, v2li, 
            v2lj, v2m2, v2m3, v2m4, v2p7, v2p8, v2p9, v2pa, 
            v2pb, v2pf, v2qg, v2qj, v2qk, v2ql, v2qm, v2qn, 
            v2qo, v2qv, v2qw, v2qx, v2qy, v2qz, v2r3, v2r8, 
            v2r9, v2ra, v2rt, v2ru, v2rv, v2rw, v2rx, v3dy, 
            v3dz, v3e0, v3e2, v3e3, v3e4, v3ei, v3ej, v3ek, 
            v3er, v3es, v3et, v3pn, v3pr, v3q0, v3q1, v3q2, 
            v3q9, v3qa, v3qb, v3qc, v3qh, v3qj, v3qt, v3qu, 
            v3qv, v3qw, v3qx, v3qy, v3r5, v3rc, v3rd, v3re, 
            v3rf, v3rg, v3rj, v3rk, v3rl, v3rm, v3rn, v3rr, 
            v3rs, 
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
            v6, vb, vc, vd, v1f, v1g, v2c, v52, 
            v55, v5t, v6v, v8g, v9f, v9g, v9h, v9i, 
            v9j, v9k, v9l, v9m, v9o, v9p, va2, vn7, 
            vn8, vow, vph, vpn, vqn, vqs, vqu, vqw, 
            vqy, vr0, vr8, vwl, vwr, vws, vwx, vx0, 
            vxz, vyf, v10e, v10g, v10n, v10o, v10r, v10v, 
            v10y, v111, v124, v125, v13k, v14q, v19y, v2ih, 
            v2ii, v2ij, v2ik, v2il, v2im, v2in, v2lh, v2li, 
            v2lj, v2m2, v2m3, v2m4, v2p7, v2p8, v2p9, v2pa, 
            v2pb, v2pf, v2qg, v2qj, v2qk, v2ql, v2qm, v2qn, 
            v2qo, v2qv, v2qw, v2qx, v2qy, v2qz, v2r3, v2r8, 
            v2r9, v2ra, v2rt, v2ru, v2rv, v2rw, v2rx, v3dy, 
            v3dz, v3e0, v3e2, v3e3, v3e4, v3ei, v3ej, v3ek, 
            v3er, v3es, v3et, v3pn, v3pr, v3q0, v3q1, v3q2, 
            v3q9, v3qa, v3qb, v3qc, v3qh, v3qj, v3qt, v3qu, 
            v3qv, v3qw, v3qx, v3qy, v3r5, v3rc, v3rd, v3re, 
            v3rf, v3rg, v3rj, v3rk, v3rl, v3rm, v3rn, v3rr, 
            v3rs, 
        }=self.eval_common_stamp_values(ctx);
        let vl=(sf[5]*f64::powf(vc,sf[6]));
        let vp=(sf[7]*f64::powf(vc,sf[8]));
        let vt=(sf[9]*f64::powf(vc,sf[10]));
        let vx=(sf[11]*f64::powf(vc,sf[12]));
        let v11=(sf[13]*f64::powf(vc,sf[14]));
        let v15=(sf[15]*f64::powf(vc,sf[16]));
        let v19=(sf[17]*f64::powf(vc,sf[18]));
        let v2f=f64::powf(vc,sf[37]);
        let v2i=(v1g*sf[39]);
        let v2k=((v2i/vb)).exp();
        let v2l=(v2f*v2k);
        let v2p=(sf[36]*f64::powf(v2l,sf[41]));
        let v2s=f64::powf(vc,sf[43]);
        let v2v=(v1g*sf[45]);
        let v2x=((v2v/vb)).exp();
        let v2y=(v2s*v2x);
        let v32=(sf[42]*f64::powf(v2y,sf[47]));
        let v36=(v1g*sf[50]);
        let v38=((v36/vb)).exp();
        let v39=(v2f*v38);
        let v3c=f64::powf(v39,sf[52]);
        let v3d=(sf[48]*v3c);
        let v3h=(v1g*sf[55]);
        let v3j=((v3h/vb)).exp();
        let v3k=(v2s*v3j);
        let v3n=f64::powf(v3k,sf[57]);
        let v3o=(sf[53]*v3n);
        let v3q=(v3c*sf[58]);
        let v3s=(v3n*sf[59]);
        let v3w=(v1g*sf[62]);
        let v3y=((v3w/vb)).exp();
        let v3z=(v2f*v3y);
        let v43=(sf[60]*f64::powf(v3z,sf[64]));
        let v47=(v1g*sf[67]);
        let v49=((v47/vb)).exp();
        let v4a=(v2s*v49);
        let v4e=(sf[65]*f64::powf(v4a,sf[69]));
        let v4s=(vd*sf[75]);
        let v4t=(sf[74]+v4s);
        let v51=(sf[76]*(v1f+(vd*sf[77])));
        let v8a=(sf[95]*f64::powf(vc,sf[96]));
        let v8b=(-(sf[73]*(v1f+(vd*v4t))));
        let v8c=(vb*v51);
        let v8e=((v8b/v8c)).exp();
        let v91=(if sb[5]{(v1f/v8a)}else{v8g});
        let vqo=(vpn/vqn);
        let vqp=(vph/vqn);
        let vrd=((v1f+(v5t*(if (sf[162]!=0.0){(sf[108]*vr8)}else{v8g})))).sqrt();
        let vrg=(if (sf[162]!=0.0){(v55*(v1f+vrd))}else{v8g});
        let vri=(if (sf[162]!=0.0){(vn8/vqs)}else{vqu});
        let vrk=(if (sf[162]!=0.0){scalar_limexp(vri)}else{vqw});
        let vrl=(vrk-v1f);
        let vro=(vr8-(if (sf[162]!=0.0){(v2c*vrl)}else{v8g}));
        let vrt=(if sb[28]{v1f}else{vrg});
        let vru=(if sb[28]{v8g}else{(if (sf[162]!=0.0){(vro/vrg)}else{v8g})});
        let vry=(vb*sf[40]);
        let vrz=(v9h/vry);
        let vs0=(if (sf[166]!=0.0){vrz}else{vri});
        let vs2=(if (sf[166]!=0.0){scalar_limexp(vs0)}else{vrk});
        let vs3=(vb*sf[46]);
        let vs4=(v9h/vs3);
        let vs5=(if (sf[166]!=0.0){vs4}else{v8g});
        let vs7=(if (sf[166]!=0.0){scalar_limexp(vs5)}else{v8g});
        let vsb=(v8b-v9h);
        let vsc=(vsb/v8c);
        let vsd=(if sb[31]{vsc}else{vqy});
        let vsf=(if sb[31]{scalar_limexp(vsd)}else{vr0});
        let vsg=(vs2-v1f);
        let vsi=(vs7-v1f);
        let vsk=((v2p*vsg)+(v32*vsi));
        let vsy=(v9j/vry);
        let vsz=(if sb[36]{vsy}else{vs0});
        let vt1=(if sb[36]{scalar_limexp(vsz)}else{vs2});
        let vt2=(v9j/vs3);
        let vt3=(if sb[36]{vt2}else{vs5});
        let vt5=(if sb[36]{scalar_limexp(vt3)}else{vs7});
        let vt7=(v8b-v9j);
        let vt8=(vt7/v8c);
        let vt9=(if sb[37]{vt8}else{vsd});
        let vtb=(if sb[37]{scalar_limexp(vt9)}else{vsf});
        let vtc=(vt1-v1f);
        let vte=(vt5-v1f);
        let vtg=((v2p*vtc)+(v32*vte));
        let vtp=(if sb[40]{vrz}else{vsz});
        let vtr=(if sb[40]{scalar_limexp(vtp)}else{vt1});
        let vts=(if sb[40]{vs4}else{vt3});
        let vtu=(if sb[40]{scalar_limexp(vts)}else{vt5});
        let vtw=(if sb[41]{vsc}else{vt9});
        let vty=(if sb[41]{scalar_limexp(vtw)}else{vtb});
        let vtz=(vtr-v1f);
        let vu1=(vtu-v1f);
        let vu3=((v2p*vtz)+(v32*vu1));
        let vub=(if sb[42]{(sf[165]*vu3)}else{(if sb[41]{(sf[165]*(vu3-(sf[168]*(vty-v8e))))}else{(if sb[36]{v8g}else{(if sb[33]{vsk}else{(if sb[31]{(vsk-(sf[168]*(vsf-v8e)))}else{v8g})})})})});
        let vuc=(if sb[40]{vsy}else{vtp});
        let vuf=(if sb[40]{vt2}else{vts});
        let vui=(if sb[41]{vt8}else{vtw});
        let vum=((if sb[40]{scalar_limexp(vuc)}else{vtr})-v1f);
        let vuo=((if sb[40]{scalar_limexp(vuf)}else{vtu})-v1f);
        let vuq=((v2p*vum)+(v32*vuo));
        let vux=(if sb[42]{(sf[170]*vuq)}else{(if sb[41]{(sf[170]*(vuq-(sf[168]*((if sb[41]{scalar_limexp(vui)}else{vty})-v8e))))}else{(if sb[38]{vtg}else{(if sb[37]{(vtg-(sf[168]*(vtb-v8e)))}else{v8g})})})});
        let vuy=(vb*sf[51]);
        let vuz=(v9l/vuy);
        let vv0=scalar_limexp(vuz);
        let vv1=(vb*sf[56]);
        let vv2=(v9l/vv1);
        let vv3=scalar_limexp(vv2);
        let vv4=(vv0-v1f);
        let vv6=(vv3-v1f);
        let vv8=((v3d*vv4)+(v3o*vv6));
        let vve=(if (sf[171]!=0.0){(v9p/vuy)}else{vuz});
        let vvi=(if (sf[171]!=0.0){(v9p/vv1)}else{vv2});
        let vvk=(if (sf[171]!=0.0){scalar_limexp(vvi)}else{vv3});
        let vvl=((if (sf[171]!=0.0){scalar_limexp(vve)}else{vv0})-v1f);
        let vvn=(vvk-v1f);
        let vvs=(if sb[46]{v8g}else{(if (sf[171]!=0.0){((v3q*vvl)+(v3s*vvn))}else{v8g})});
        let vvw=(v6v-v9l);
        let vvy=0.01;
        let vw0=(((vvw*vvw)+vvy)).sqrt();
        let vw3=(if (sf[173]!=0.0){(v55*(vvw+vw0))}else{vow});
        let vw4=(sf[172]*vw3);
        let vw5=(-(sf[71]*(v1f+(vd*sf[72]))));
        let vw7=f64::powf(vw3,sf[174]);
        let vw8=(vw5*vw7);
        let vw9=scalar_limexp(vw8);
        let vwb=(if (sf[173]!=0.0){(vw4*vw9)}else{v8g});
        let vwc=(vqp-vqo);
        let vwd=(vwc-vv8);
        let vwi=(vv8-(if sb[48]{v8g}else{(if (sf[173]!=0.0){(vwb*vwd)}else{v8g})}));
        let vwm=(vwl-v9m);
        let vwq=(if sb[50]{v8g}else{(if (sf[175]!=0.0){(vwm/vl)}else{v8g})});
        let vx3=(v1f+vwx);
        let vx4=(v1f+vx0);
        let vx6=(if (sf[176]!=0.0){(vx3/vx4)}else{v8g});
        let vx7=(v9m-v9k);
        let vxa=((vwx-vx0)-(vx6).ln());
        let vxc=(vx7+(vb*vxa));
        let vxe=(if (sf[176]!=0.0){(vxc/vp)}else{v8g});
        let vxf=(vp*v91);
        let vxg=(vxe*vxf);
        let vxi=(sf[111]*(v55*v91));
        let vxl=((vvy+(vx7*vx7))).sqrt();
        let vxn=(v1f+(vxi*vxl));
        let vxp=(if (sf[176]!=0.0){(vxg/vxn)}else{v8g});
        let vxs=((v1f+(vxp*vxp))).sqrt();
        let vxw=(if sb[52]{v8g}else{(if (sf[176]!=0.0){(vxe/vxs)}else{v8g})});
        let vy0=(vxz-v9i);
        let vy4=(if sb[54]{v8g}else{(if (sf[177]!=0.0){(vy0/vt)}else{v8g})});
        let vy7=(v9i-v9f);
        let vy8=(vqn*vy7);
        let vyc=(if sb[56]{v8g}else{(if (sf[178]!=0.0){(vy8/vx)}else{v8g})});
        let vyg=(vyf-v9g);
        let vyk=(if sb[58]{v8g}else{(if (sf[179]!=0.0){(vyg/v11)}else{v8g})});
        let vyn=(v9o-v9m);
        let vyo=(vrt*vyn);
        let vys=(if sb[60]{v8g}else{(if (sf[180]!=0.0){(vyo/v19)}else{v8g})});
        let vyx=(vb*sf[63]);
        let vyz=(if (sf[181]!=0.0){(vn8/vyx)}else{vwr});
        let vz2=(vb*sf[68]);
        let vz4=(if (sf[181]!=0.0){(vn8/vz2)}else{vvi});
        let vz7=((if (sf[181]!=0.0){scalar_limexp(vyz)}else{vws})-v1f);
        let vz9=((if (sf[181]!=0.0){scalar_limexp(vz4)}else{vvk})-v1f);
        let vze=(if sb[64]{v8g}else{(if (sf[181]!=0.0){((v43*vz7)+(v4e*vz9))}else{v8g})});
        let vzi=(ctx.node_voltage(nodes[3])-vn7);
        let vzm=(if sb[66]{v8g}else{(if (sf[182]!=0.0){(vzi/v15)}else{v8g})});
        let v115=(v9k-v9g);
        let v11g=(v9i-vn7);
        let v12l=(sf[7]*(sf[194]*(sf[8]*f64::powf(vc,sf[197]))));
        let v14u=(sf[194]*(sf[37]*f64::powf(vc,sf[212])));
        let v158=(sf[36]*(((v2k*v14u)+(v2f*(v2k*(((vb*sf[213])-(v2i*v125))/v13k))))*(sf[41]*f64::powf(v2l,sf[214]))));
        let v15c=(sf[194]*(sf[43]*f64::powf(vc,sf[215])));
        let v15q=(sf[42]*(((v2x*v15c)+(v2s*(v2x*(((vb*sf[216])-(v2v*v125))/v13k))))*(sf[47]*f64::powf(v2y,sf[217]))));
        let v163=(((v38*v14u)+(v2f*(v38*(((vb*sf[218])-(v36*v125))/v13k))))*(sf[52]*f64::powf(v39,sf[219])));
        let v16h=(((v3j*v15c)+(v2s*(v3j*(((vb*sf[220])-(v3h*v125))/v13k))))*(sf[57]*f64::powf(v3k,sf[221])));
        let v1c4=((v51*v125)+(vb*sf[229]));
        let v1c5=(v8c*(-(sf[73]*(v4s+v4t))));
        let v1c8=(v8c*v8c);
        let v1ca=(v8e*((v1c5-(v8b*v1c4))/v1c8));
        let v1ci=(if sb[5]{((-(sf[95]*(sf[194]*(sf[96]*f64::powf(vc,sf[241])))))/(v8a*v8a))}else{v8g});
        let v2pg=(((vqn*v2m2)-(vpn*v2p7))/v2pf);
        let v2pk=(((vqn*v2m3)-(vpn*v2p8))/v2pf);
        let v2pn=((-(vpn*v2p9))/v2pf);
        let v2pr=(((vqn*v2m4)-(vpn*v2pa))/v2pf);
        let v2pu=((-(vpn*v2pb))/v2pf);
        let v2py=(((vqn*v2lh)-(vph*v2p7))/v2pf);
        let v2q1=((-(vph*v2p8))/v2pf);
        let v2q4=((-(vph*v2p9))/v2pf);
        let v2q8=(((vqn*v2li)-(vph*v2pa))/v2pf);
        let v2qc=(((vqn*v2lj)-(vph*v2pb))/v2pf);
        let v2sd=(v52*vrd);
        let v2so=(if (sf[162]!=0.0){(v55*((v5t*(if (sf[162]!=0.0){(sf[108]*v2rt)}else{v8g}))/v2sd))}else{v8g});
        let v2sp=(if (sf[162]!=0.0){(v55*((v5t*(if (sf[162]!=0.0){(sf[108]*v2ru)}else{v8g}))/v2sd))}else{v8g});
        let v2sq=(if (sf[162]!=0.0){(v55*((v5t*(if (sf[162]!=0.0){(sf[108]*v2rv)}else{v8g}))/v2sd))}else{v8g});
        let v2sr=(if (sf[162]!=0.0){(v55*((v5t*(if (sf[162]!=0.0){(sf[108]*v2rw)}else{v8g}))/v2sd))}else{v8g});
        let v2ss=(if (sf[162]!=0.0){(v55*((v5t*(if (sf[162]!=0.0){(sf[108]*v2rx)}else{v8g}))/v2sd))}else{v8g});
        let v2sw=(if (sf[162]!=0.0){((-(vn8*sf[303]))/v2qg)}else{v2qk});
        let v2sx=(if (sf[162]!=0.0){v8g}else{v2ql});
        let v2sy=(if (sf[162]!=0.0){v8g}else{v2qm});
        let v2sz=(if (sf[162]!=0.0){v8g}else{v2qn});
        let v2t0=(if (sf[162]!=0.0){v2qj}else{v2qo});
        let v2t1=scalar_limexp_derivative(vri);
        let v2t8=(if (sf[162]!=0.0){(v2sw*v2t1)}else{v2qv});
        let v2t9=(if (sf[162]!=0.0){(v2sx*v2t1)}else{v2qw});
        let v2ta=(if (sf[162]!=0.0){(v2sy*v2t1)}else{v2qx});
        let v2tb=(if (sf[162]!=0.0){(v2sz*v2t1)}else{v2qy});
        let v2tc=(if (sf[162]!=0.0){(v2t0*v2t1)}else{v2qz});
        let v2td=(if (sf[162]!=0.0){(v2qm*v2t1)}else{v8g});
        let v2u1=(vrg*vrg);
        let v2v0=(if sb[28]{v8g}else{(if (sf[162]!=0.0){(((vrg*(v2rt-(if (sf[162]!=0.0){((vrl*v14q)+(v2c*v2t8))}else{v8g})))-(vro*v2so))/v2u1)}else{v8g})});
        let v2v1=(if sb[28]{v8g}else{(if (sf[162]!=0.0){(((vrg*(v2ru-(if (sf[162]!=0.0){(v2c*v2t9)}else{v8g})))-(vro*v2sp))/v2u1)}else{v8g})});
        let v2v2=(if sb[28]{v8g}else{(if (sf[162]!=0.0){(((vrg*(v2rv-(if (sf[162]!=0.0){(v2c*v2ta)}else{v8g})))-(vro*v2sq))/v2u1)}else{v8g})});
        let v2v3=(if sb[28]{v8g}else{(if (sf[162]!=0.0){(((vrg*(v2rw-(if (sf[162]!=0.0){(v2c*v2tb)}else{v8g})))-(vro*v2sr))/v2u1)}else{v8g})});
        let v2v4=(if sb[28]{v8g}else{(if (sf[162]!=0.0){(((vrg*(v2rx-(if (sf[162]!=0.0){(v2c*v2tc)}else{v8g})))-(vro*v2ss))/v2u1)}else{v8g})});
        let v2v5=(if sb[28]{v8g}else{(if (sf[162]!=0.0){((-(if (sf[162]!=0.0){(v2c*v2td)}else{v8g}))/vrg)}else{v8g})});
        let v2v9=(vry*vry);
        let v2va=((-(v9h*sf[304]))/v2v9);
        let v2vb=(v1f/vry);
        let v2vc=(va2/vry);
        let v2vd=(if (sf[166]!=0.0){v2va}else{v2sw});
        let v2ve=(if (sf[166]!=0.0){v8g}else{v2sx});
        let v2vf=(if (sf[166]!=0.0){v8g}else{v2sy});
        let v2vg=(if (sf[166]!=0.0){v2vb}else{v2sz});
        let v2vh=(if (sf[166]!=0.0){v2vc}else{v8g});
        let v2vi=(if (sf[166]!=0.0){v8g}else{v2t0});
        let v2vj=(if (sf[166]!=0.0){v8g}else{v2qm});
        let v2vk=scalar_limexp_derivative(vs0);
        let v2vs=(if (sf[166]!=0.0){(v2vd*v2vk)}else{v2t8});
        let v2vt=(if (sf[166]!=0.0){(v2ve*v2vk)}else{v2t9});
        let v2vu=(if (sf[166]!=0.0){(v2vf*v2vk)}else{v2ta});
        let v2vv=(if (sf[166]!=0.0){(v2vg*v2vk)}else{v2tb});
        let v2vw=(if (sf[166]!=0.0){(v2vh*v2vk)}else{v8g});
        let v2vx=(if (sf[166]!=0.0){(v2vi*v2vk)}else{v2tc});
        let v2vy=(if (sf[166]!=0.0){(v2vj*v2vk)}else{v2td});
        let v2w2=(vs3*vs3);
        let v2w3=((-(v9h*sf[305]))/v2w2);
        let v2w4=(v1f/vs3);
        let v2w5=(va2/vs3);
        let v2w6=(if (sf[166]!=0.0){v2w3}else{v8g});
        let v2w7=(if (sf[166]!=0.0){v2w4}else{v8g});
        let v2w8=(if (sf[166]!=0.0){v2w5}else{v8g});
        let v2w9=scalar_limexp_derivative(vs5);
        let v2wd=(if (sf[166]!=0.0){(v2w6*v2w9)}else{v8g});
        let v2we=(if (sf[166]!=0.0){(v2w7*v2w9)}else{v8g});
        let v2wf=(if (sf[166]!=0.0){(v2w8*v2w9)}else{v8g});
        let v2wi=((v1c5-(vsb*v1c4))/v1c8);
        let v2wj=(va2/v8c);
        let v2wk=(v1f/v8c);
        let v2wl=(if sb[31]{v2wi}else{v2r3});
        let v2wm=(if sb[31]{v8g}else{v2qo});
        let v2wn=(if sb[31]{v2wj}else{v2qm});
        let v2wo=(if sb[31]{v2wk}else{v8g});
        let v2wp=scalar_limexp_derivative(vsd);
        let v2wu=(if sb[31]{(v2wl*v2wp)}else{v2r8});
        let v2wv=(if sb[31]{(v2wm*v2wp)}else{v2r9});
        let v2ww=(if sb[31]{(v2wn*v2wp)}else{v2ra});
        let v2wx=(if sb[31]{(v2wo*v2wp)}else{v8g});
        let v2x1=(v2p*v2vt);
        let v2x2=(v2p*v2vu);
        let v2x5=(v2p*v2vx);
        let v2x6=(v2p*v2vy);
        let v2xc=(((vsg*v158)+(v2p*v2vs))+((vsi*v15q)+(v32*v2wd)));
        let v2xd=((v2p*v2vv)+(v32*v2we));
        let v2xe=((v2p*v2vw)+(v32*v2wf));
        let v2yb=((-(v9j*sf[304]))/v2v9);
        let v2yc=(if sb[36]{v2yb}else{v2vd});
        let v2yd=(if sb[36]{v8g}else{v2ve});
        let v2ye=(if sb[36]{v2vb}else{v2vf});
        let v2yf=(if sb[36]{v8g}else{v2vg});
        let v2yg=(if sb[36]{v2vc}else{v2vh});
        let v2yh=(if sb[36]{v8g}else{v2vi});
        let v2yi=(if sb[36]{v8g}else{v2vj});
        let v2yj=scalar_limexp_derivative(vsz);
        let v2yr=(if sb[36]{(v2yc*v2yj)}else{v2vs});
        let v2ys=(if sb[36]{(v2yd*v2yj)}else{v2vt});
        let v2yt=(if sb[36]{(v2ye*v2yj)}else{v2vu});
        let v2yu=(if sb[36]{(v2yf*v2yj)}else{v2vv});
        let v2yv=(if sb[36]{(v2yg*v2yj)}else{v2vw});
        let v2yw=(if sb[36]{(v2yh*v2yj)}else{v2vx});
        let v2yx=(if sb[36]{(v2yi*v2yj)}else{v2vy});
        let v2z0=((-(v9j*sf[305]))/v2w2);
        let v2z1=(if sb[36]{v2z0}else{v2w6});
        let v2z2=(if sb[36]{v2w4}else{v8g});
        let v2z3=(if sb[36]{v8g}else{v2w7});
        let v2z4=(if sb[36]{v2w5}else{v2w8});
        let v2z5=scalar_limexp_derivative(vt3);
        let v2za=(if sb[36]{(v2z1*v2z5)}else{v2wd});
        let v2zb=(if sb[36]{(v2z2*v2z5)}else{v8g});
        let v2zc=(if sb[36]{(v2z3*v2z5)}else{v2we});
        let v2zd=(if sb[36]{(v2z4*v2z5)}else{v2wf});
        let v2zg=((v1c5-(vt7*v1c4))/v1c8);
        let v2zh=(if sb[37]{v2zg}else{v2wl});
        let v2zi=(if sb[37]{v8g}else{v2wm});
        let v2zj=(if sb[37]{v2wj}else{v8g});
        let v2zk=(if sb[37]{v8g}else{v2wn});
        let v2zl=(if sb[37]{v2wk}else{v2wo});
        let v2zm=scalar_limexp_derivative(vt9);
        let v2zs=(if sb[37]{(v2zh*v2zm)}else{v2wu});
        let v2zt=(if sb[37]{(v2zi*v2zm)}else{v2wv});
        let v2zu=(if sb[37]{(v2zj*v2zm)}else{v8g});
        let v2zv=(if sb[37]{(v2zk*v2zm)}else{v2ww});
        let v2zw=(if sb[37]{(v2zl*v2zm)}else{v2wx});
        let v300=(v2p*v2ys);
        let v304=(v2p*v2yw);
        let v305=(v2p*v2yx);
        let v30c=(((vtc*v158)+(v2p*v2yr))+((vte*v15q)+(v32*v2za)));
        let v30d=((v2p*v2yt)+(v32*v2zb));
        let v30e=((v2p*v2yu)+(v32*v2zc));
        let v30f=((v2p*v2yv)+(v32*v2zd));
        let v315=(if sb[40]{v2va}else{v2yc});
        let v316=(if sb[40]{v8g}else{v2yd});
        let v317=(if sb[40]{v8g}else{v2ye});
        let v318=(if sb[40]{v2vb}else{v2yf});
        let v319=(if sb[40]{v2vc}else{v2yg});
        let v31a=(if sb[40]{v8g}else{v2yh});
        let v31b=(if sb[40]{v8g}else{v2yi});
        let v31c=scalar_limexp_derivative(vtp);
        let v31k=(if sb[40]{(v315*v31c)}else{v2yr});
        let v31l=(if sb[40]{(v316*v31c)}else{v2ys});
        let v31m=(if sb[40]{(v317*v31c)}else{v2yt});
        let v31n=(if sb[40]{(v318*v31c)}else{v2yu});
        let v31o=(if sb[40]{(v319*v31c)}else{v2yv});
        let v31p=(if sb[40]{(v31a*v31c)}else{v2yw});
        let v31q=(if sb[40]{(v31b*v31c)}else{v2yx});
        let v31r=(if sb[40]{v2w3}else{v2z1});
        let v31s=(if sb[40]{v8g}else{v2z2});
        let v31t=(if sb[40]{v2w4}else{v2z3});
        let v31u=(if sb[40]{v2w5}else{v2z4});
        let v31v=scalar_limexp_derivative(vts);
        let v320=(if sb[40]{(v31r*v31v)}else{v2za});
        let v321=(if sb[40]{(v31s*v31v)}else{v2zb});
        let v322=(if sb[40]{(v31t*v31v)}else{v2zc});
        let v323=(if sb[40]{(v31u*v31v)}else{v2zd});
        let v324=(if sb[41]{v2wi}else{v2zh});
        let v325=(if sb[41]{v8g}else{v2zi});
        let v326=(if sb[41]{v8g}else{v2zj});
        let v327=(if sb[41]{v2wj}else{v2zk});
        let v328=(if sb[41]{v2wk}else{v2zl});
        let v329=scalar_limexp_derivative(vtw);
        let v32f=(if sb[41]{(v324*v329)}else{v2zs});
        let v32g=(if sb[41]{(v325*v329)}else{v2zt});
        let v32h=(if sb[41]{(v326*v329)}else{v2zu});
        let v32i=(if sb[41]{(v327*v329)}else{v2zv});
        let v32j=(if sb[41]{(v328*v329)}else{v2zw});
        let v32n=(v2p*v31l);
        let v32z=(((vtz*v158)+(v2p*v31k))+((vu1*v15q)+(v32*v320)));
        let v330=((v2p*v31m)+(v32*v321));
        let v331=((v2p*v31n)+(v32*v322));
        let v332=((v2p*v31o)+(v32*v323));
        let v33j=(sf[165]*(v2p*v31p));
        let v33k=(sf[165]*(v2p*v31q));
        let v33x=(if sb[42]{(sf[165]*v32z)}else{(if sb[41]{(sf[165]*(v32z-(sf[168]*(v32f-v1ca))))}else{(if sb[36]{v8g}else{(if sb[33]{v2xc}else{(if sb[31]{(v2xc-(sf[168]*(v2wu-v1ca)))}else{v8g})})})})});
        let v33y=(if sb[42]{(sf[165]*v32n)}else{(if sb[41]{(sf[165]*(v32n-(sf[168]*v32g)))}else{(if sb[36]{v8g}else{(if sb[33]{v2x1}else{(if sb[31]{(v2x1-(sf[168]*v2wv))}else{v8g})})})})});
        let v33z=(if sb[42]{(sf[165]*v330)}else{(if sb[41]{(sf[165]*(v330-(sf[168]*v32h)))}else{(if sb[36]{v8g}else{(if sb[33]{v2x2}else{(if sb[31]{v2x2}else{v8g})})})})});
        let v340=(if sb[42]{(sf[165]*v331)}else{(if sb[41]{(sf[165]*(v331-(sf[168]*v32i)))}else{(if sb[36]{v8g}else{(if sb[33]{v2xd}else{(if sb[31]{(v2xd-(sf[168]*v2ww))}else{v8g})})})})});
        let v341=(if sb[42]{(sf[165]*v332)}else{(if sb[41]{(sf[165]*(v332-(sf[168]*v32j)))}else{(if sb[36]{v8g}else{(if sb[33]{v2xe}else{(if sb[31]{(v2xe-(sf[168]*v2wx))}else{v8g})})})})});
        let v342=(if sb[42]{v33j}else{(if sb[41]{v33j}else{(if sb[36]{v8g}else{(if sb[33]{v2x5}else{(if sb[31]{v2x5}else{v8g})})})})});
        let v343=(if sb[42]{v33k}else{(if sb[41]{v33k}else{(if sb[36]{v8g}else{(if sb[33]{v2x6}else{(if sb[31]{v2x6}else{v8g})})})})});
        let v34b=scalar_limexp_derivative(vuc);
        let v34u=scalar_limexp_derivative(vuf);
        let v358=scalar_limexp_derivative(vui);
        let v35m=(v2p*(if sb[40]{((if sb[40]{v8g}else{v316})*v34b)}else{v31l}));
        let v35y=(((vum*v158)+(v2p*(if sb[40]{((if sb[40]{v2yb}else{v315})*v34b)}else{v31k})))+((vuo*v15q)+(v32*(if sb[40]{((if sb[40]{v2z0}else{v31r})*v34u)}else{v320}))));
        let v35z=((v2p*(if sb[40]{((if sb[40]{v2vb}else{v317})*v34b)}else{v31m}))+(v32*(if sb[40]{((if sb[40]{v2w4}else{v31s})*v34u)}else{v321})));
        let v360=((v2p*(if sb[40]{((if sb[40]{v8g}else{v318})*v34b)}else{v31n}))+(v32*(if sb[40]{((if sb[40]{v8g}else{v31t})*v34u)}else{v322})));
        let v361=((v2p*(if sb[40]{((if sb[40]{v2vc}else{v319})*v34b)}else{v31o}))+(v32*(if sb[40]{((if sb[40]{v2w5}else{v31u})*v34u)}else{v323})));
        let v36i=(sf[170]*(v2p*(if sb[40]{((if sb[40]{v8g}else{v31a})*v34b)}else{v31p})));
        let v36j=(sf[170]*(v2p*(if sb[40]{((if sb[40]{v8g}else{v31b})*v34b)}else{v31q})));
        let v36w=(if sb[42]{(sf[170]*v35y)}else{(if sb[41]{(sf[170]*(v35y-(sf[168]*((if sb[41]{((if sb[41]{v2zg}else{v324})*v358)}else{v32f})-v1ca))))}else{(if sb[38]{v30c}else{(if sb[37]{(v30c-(sf[168]*(v2zs-v1ca)))}else{v8g})})})});
        let v36x=(if sb[42]{(sf[170]*v35m)}else{(if sb[41]{(sf[170]*(v35m-(sf[168]*(if sb[41]{((if sb[41]{v8g}else{v325})*v358)}else{v32g}))))}else{(if sb[38]{v300}else{(if sb[37]{(v300-(sf[168]*v2zt))}else{v8g})})})});
        let v36y=(if sb[42]{(sf[170]*v35z)}else{(if sb[41]{(sf[170]*(v35z-(sf[168]*(if sb[41]{((if sb[41]{v2wj}else{v326})*v358)}else{v32h}))))}else{(if sb[38]{v30d}else{(if sb[37]{(v30d-(sf[168]*v2zu))}else{v8g})})})});
        let v36z=(if sb[42]{(sf[170]*v360)}else{(if sb[41]{(sf[170]*(v360-(sf[168]*(if sb[41]{((if sb[41]{v8g}else{v327})*v358)}else{v32i}))))}else{(if sb[38]{v30e}else{(if sb[37]{(v30e-(sf[168]*v2zv))}else{v8g})})})});
        let v370=(if sb[42]{(sf[170]*v361)}else{(if sb[41]{(sf[170]*(v361-(sf[168]*(if sb[41]{((if sb[41]{v2wk}else{v328})*v358)}else{v32j}))))}else{(if sb[38]{v30f}else{(if sb[37]{(v30f-(sf[168]*v2zw))}else{v8g})})})});
        let v371=(if sb[42]{v36i}else{(if sb[41]{v36i}else{(if sb[38]{v304}else{(if sb[37]{v304}else{v8g})})})});
        let v372=(if sb[42]{v36j}else{(if sb[41]{v36j}else{(if sb[38]{v305}else{(if sb[37]{v305}else{v8g})})})});
        let v376=(vuy*vuy);
        let v377=((-(v9l*sf[306]))/v376);
        let v378=(va2/vuy);
        let v379=(v1f/vuy);
        let v37a=scalar_limexp_derivative(vuz);
        let v37b=(v377*v37a);
        let v37c=(v378*v37a);
        let v37d=(v379*v37a);
        let v37h=(vv1*vv1);
        let v37i=((-(v9l*sf[307]))/v37h);
        let v37j=(va2/vv1);
        let v37k=(v1f/vv1);
        let v37l=scalar_limexp_derivative(vv2);
        let v37m=(v37i*v37l);
        let v37n=(v37j*v37l);
        let v37o=(v37k*v37l);
        let v37z=(((vv4*(sf[48]*v163))+(v3d*v37b))+((vv6*(sf[53]*v16h))+(v3o*v37m)));
        let v380=((v3d*v37c)+(v3o*v37n));
        let v381=((v3d*v37d)+(v3o*v37o));
        let v38a=scalar_limexp_derivative(vve);
        let v38o=(if (sf[171]!=0.0){((-(v9p*sf[307]))/v37h)}else{v37i});
        let v38p=(if (sf[171]!=0.0){v8g}else{v37j});
        let v38q=(if (sf[171]!=0.0){v37k}else{v8g});
        let v38r=(if (sf[171]!=0.0){v8g}else{v37k});
        let v38s=(if (sf[171]!=0.0){v37j}else{v8g});
        let v38t=scalar_limexp_derivative(vvi);
        let v38z=(if (sf[171]!=0.0){(v38o*v38t)}else{v37m});
        let v390=(if (sf[171]!=0.0){(v38p*v38t)}else{v37n});
        let v391=(if (sf[171]!=0.0){(v38q*v38t)}else{v8g});
        let v392=(if (sf[171]!=0.0){(v38r*v38t)}else{v37o});
        let v393=(if (sf[171]!=0.0){(v38s*v38t)}else{v8g});
        let v39s=(if sb[46]{v8g}else{(if (sf[171]!=0.0){(((vvl*(sf[58]*v163))+(v3q*(if (sf[171]!=0.0){((if (sf[171]!=0.0){((-(v9p*sf[306]))/v376)}else{v377})*v38a)}else{v37b})))+((vvn*(sf[59]*v16h))+(v3s*v38z)))}else{v8g})});
        let v39t=(if sb[46]{v8g}else{(if (sf[171]!=0.0){((v3q*(if (sf[171]!=0.0){((if (sf[171]!=0.0){v8g}else{v378})*v38a)}else{v37c}))+(v3s*v390))}else{v8g})});
        let v39u=(if sb[46]{v8g}else{(if (sf[171]!=0.0){((v3q*(if (sf[171]!=0.0){((if (sf[171]!=0.0){v379}else{v8g})*v38a)}else{v8g}))+(v3s*v391))}else{v8g})});
        let v39v=(if sb[46]{v8g}else{(if (sf[171]!=0.0){((v3q*(if (sf[171]!=0.0){((if (sf[171]!=0.0){v8g}else{v379})*v38a)}else{v37d}))+(v3s*v392))}else{v8g})});
        let v39w=(if sb[46]{v8g}else{(if (sf[171]!=0.0){((v3q*(if (sf[171]!=0.0){((if (sf[171]!=0.0){v378}else{v8g})*v38a)}else{v8g}))+(v3s*v393))}else{v8g})});
        let v39x=(vvw*v19y);
        let v3a0=(-vvw);
        let v3a2=(v52*vw0);
        let v3ac=(if (sf[173]!=0.0){(v55*(v19y+((v39x+v39x)/v3a2)))}else{v2ih});
        let v3ad=(if (sf[173]!=0.0){(v55*(v1f+((vvw+vvw)/v3a2)))}else{v2ii});
        let v3ae=(if (sf[173]!=0.0){v8g}else{v2ij});
        let v3af=(if (sf[173]!=0.0){(v55*(va2+((v3a0+v3a0)/v3a2)))}else{v2ik});
        let v3ag=(if (sf[173]!=0.0){v8g}else{v2il});
        let v3ah=(if (sf[173]!=0.0){v8g}else{v2im});
        let v3ai=(if (sf[173]!=0.0){v8g}else{v2in});
        let v3at=(sf[174]*f64::powf(vw3,sf[309]));
        let v3ba=scalar_limexp_derivative(vw8);
        let v3ca=(v2py-v2pg);
        let v3cb=(v2q1-v2pk);
        let v3cc=(v2q4-v2pn);
        let v3cd=(v2q8-v2pr);
        let v3ce=(v2qc-v2pu);
        let v3dd=(v37z-(if sb[48]{v8g}else{(if (sf[173]!=0.0){((vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3ac))+(vw4*(((vw7*sf[308])+(vw5*(v3ac*v3at)))*v3ba)))}else{v8g}))+(vwb*(v3ca-v37z)))}else{v8g})}));
        let v3de=(v380-(if sb[48]{v8g}else{(if (sf[173]!=0.0){((vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3ad))+(vw4*((vw5*(v3ad*v3at))*v3ba)))}else{v8g}))+(vwb*(v3cb-v380)))}else{v8g})}));
        let v3df=(-(if sb[48]{v8g}else{(if (sf[173]!=0.0){((vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3ae))+(vw4*((vw5*(v3ae*v3at))*v3ba)))}else{v8g}))+(vwb*v3cc))}else{v8g})}));
        let v3dg=(v381-(if sb[48]{v8g}else{(if (sf[173]!=0.0){((vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3af))+(vw4*((vw5*(v3af*v3at))*v3ba)))}else{v8g}))+(vwb*(v3cd-v381)))}else{v8g})}));
        let v3dh=(-(if sb[48]{v8g}else{(if (sf[173]!=0.0){((vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3ag))+(vw4*((vw5*(v3ag*v3at))*v3ba)))}else{v8g}))+(vwb*v3ce))}else{v8g})}));
        let v3di=(-(if sb[48]{v8g}else{(if (sf[173]!=0.0){(vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3ah))+(vw4*((vw5*(v3ah*v3at))*v3ba)))}else{v8g}))}else{v8g})}));
        let v3dj=(-(if sb[48]{v8g}else{(if (sf[173]!=0.0){(vwd*(if (sf[173]!=0.0){((vw9*(sf[172]*v3ai))+(vw4*((vw5*(v3ai*v3at))*v3ba)))}else{v8g}))}else{v8g})}));
        let v3dt=(if sb[50]{v8g}else{(if (sf[175]!=0.0){(v1f/vl)}else{v8g})});
        let v3du=(if sb[50]{v8g}else{(if (sf[175]!=0.0){((-(vwm*(sf[5]*(sf[194]*(sf[6]*f64::powf(vc,sf[196]))))))/(vl*vl))}else{v8g})});
        let v3dv=(if sb[50]{v8g}else{(if (sf[175]!=0.0){(va2/vl)}else{v8g})});
        let v3ex=(vx4*vx4);
        let v3g2=(if (sf[176]!=0.0){(((vp*((vxa*v125)+(vb*((v3ei-v3er)-((if (sf[176]!=0.0){(((vx4*v3ei)-(vx3*v3er))/v3ex)}else{v8g})/vx6)))))-(vxc*v12l))/(vp*vp))}else{v8g});
        let v3g3=(if (sf[176]!=0.0){((v1f+(vb*((-v3es)-((if (sf[176]!=0.0){((-(vx3*v3es))/v3ex)}else{v8g})/vx6))))/vp)}else{v8g});
        let v3g4=(if (sf[176]!=0.0){((va2+(vb*(v3ej-((if (sf[176]!=0.0){(v3ej/vx4)}else{v8g})/vx6))))/vp)}else{v8g});
        let v3g5=(if (sf[176]!=0.0){((vb*((v3ek-v3et)-((if (sf[176]!=0.0){(((vx4*v3ek)-(vx3*v3et))/v3ex)}else{v8g})/vx6)))/vp)}else{v8g});
        let v3gi=(-vx7);
        let v3gk=(v52*vxl);
        let v3gt=(vxn*vxn);
        let v3h8=(vxp*(if (sf[176]!=0.0){(((vxn*((vxf*v3g2)+(vxe*((v91*v12l)+(vp*v1ci)))))-(vxg*(vxl*(sf[111]*(v55*v1ci)))))/v3gt)}else{v8g}));
        let v3ha=(vxp*(if (sf[176]!=0.0){(((vxn*(vxf*v3g3))-(vxg*(vxi*((vx7+vx7)/v3gk))))/v3gt)}else{v8g}));
        let v3hc=(vxp*(if (sf[176]!=0.0){(((vxn*(vxf*v3g4))-(vxg*(vxi*((v3gi+v3gi)/v3gk))))/v3gt)}else{v8g}));
        let v3he=(vxp*(if (sf[176]!=0.0){((vxf*v3g5)/vxn)}else{v8g}));
        let v3hg=(v52*vxs);
        let v3ho=(vxs*vxs);
        let v3i6=(if sb[52]{v8g}else{(if (sf[176]!=0.0){(((vxs*v3g2)-(vxe*((v3h8+v3h8)/v3hg)))/v3ho)}else{v8g})});
        let v3i7=(if sb[52]{v8g}else{(if (sf[176]!=0.0){(((vxs*v3g3)-(vxe*((v3ha+v3ha)/v3hg)))/v3ho)}else{v8g})});
        let v3i8=(if sb[52]{v8g}else{(if (sf[176]!=0.0){(((vxs*v3g4)-(vxe*((v3hc+v3hc)/v3hg)))/v3ho)}else{v8g})});
        let v3i9=(if sb[52]{v8g}else{(if (sf[176]!=0.0){(((vxs*v3g5)-(vxe*((v3he+v3he)/v3hg)))/v3ho)}else{v8g})});
        let v3ij=(if sb[54]{v8g}else{(if (sf[177]!=0.0){(v1f/vt)}else{v8g})});
        let v3ik=(if sb[54]{v8g}else{(if (sf[177]!=0.0){((-(vy0*(sf[9]*(sf[194]*(sf[10]*f64::powf(vc,sf[198]))))))/(vt*vt))}else{v8g})});
        let v3il=(if sb[54]{v8g}else{(if (sf[177]!=0.0){(va2/vt)}else{v8g})});
        let v3j8=(if sb[56]{v8g}else{(if (sf[178]!=0.0){(((vx*(vy7*v2p7))-(vy8*(sf[11]*(sf[194]*(sf[12]*f64::powf(vc,sf[199]))))))/(vx*vx))}else{v8g})});
        let v3j9=(if sb[56]{v8g}else{(if (sf[178]!=0.0){((vy7*v2p8)/vx)}else{v8g})});
        let v3ja=(if sb[56]{v8g}else{(if (sf[178]!=0.0){((vqn+(vy7*v2p9))/vx)}else{v8g})});
        let v3jb=(if sb[56]{v8g}else{(if (sf[178]!=0.0){(((vy7*v2pa)+(-vqn))/vx)}else{v8g})});
        let v3jc=(if sb[56]{v8g}else{(if (sf[178]!=0.0){((vy7*v2pb)/vx)}else{v8g})});
        let v3jm=(if sb[58]{v8g}else{(if (sf[179]!=0.0){(v1f/v11)}else{v8g})});
        let v3jn=(if sb[58]{v8g}else{(if (sf[179]!=0.0){((-(vyg*(sf[13]*(sf[194]*(sf[14]*f64::powf(vc,sf[200]))))))/(v11*v11))}else{v8g})});
        let v3jo=(if sb[58]{v8g}else{(if (sf[179]!=0.0){(va2/v11)}else{v8g})});
        let v3kc=(if sb[60]{v8g}else{(if (sf[180]!=0.0){(((v19*(vyn*(if sb[28]{v8g}else{v2so})))-(vyo*(sf[17]*(sf[194]*(sf[18]*f64::powf(vc,sf[202]))))))/(v19*v19))}else{v8g})});
        let v3kd=(if sb[60]{v8g}else{(if (sf[180]!=0.0){((-vrt)/v19)}else{v8g})});
        let v3ke=(if sb[60]{v8g}else{(if (sf[180]!=0.0){((vyn*(if sb[28]{v8g}else{v2sp}))/v19)}else{v8g})});
        let v3kf=(if sb[60]{v8g}else{(if (sf[180]!=0.0){((vyn*(if sb[28]{v8g}else{v2sq}))/v19)}else{v8g})});
        let v3kg=(if sb[60]{v8g}else{(if (sf[180]!=0.0){((vyn*(if sb[28]{v8g}else{v2sr}))/v19)}else{v8g})});
        let v3kh=(if sb[60]{v8g}else{(if (sf[180]!=0.0){((vrt+(vyn*(if sb[28]{v8g}else{v2ss})))/v19)}else{v8g})});
        let v3ku=scalar_limexp_derivative(vyz);
        let v3li=scalar_limexp_derivative(vz4);
        let v3ml=(if sb[64]{v8g}else{(if (sf[181]!=0.0){(((vz7*(sf[60]*(((v3y*v14u)+(v2f*(v3y*(((vb*sf[222])-(v3w*v125))/v13k))))*(sf[64]*f64::powf(v3z,sf[223])))))+(v43*(if (sf[181]!=0.0){((if (sf[181]!=0.0){((-(vn8*sf[310]))/(vyx*vyx))}else{v3dy})*v3ku)}else{v3e2})))+((vz9*(sf[65]*(((v49*v15c)+(v2s*(v49*(((vb*sf[224])-(v47*v125))/v13k))))*(sf[69]*f64::powf(v4a,sf[225])))))+(v4e*(if (sf[181]!=0.0){((if (sf[181]!=0.0){((-(vn8*sf[311]))/(vz2*vz2))}else{v38o})*v3li)}else{v38z}))))}else{v8g})});
        let v3mm=(if sb[64]{v8g}else{(if (sf[181]!=0.0){((v43*(if (sf[181]!=0.0){((if (sf[181]!=0.0){v8g}else{v3dz})*v3ku)}else{v3e3}))+(v4e*(if (sf[181]!=0.0){((if (sf[181]!=0.0){v8g}else{v38p})*v3li)}else{v390})))}else{v8g})});
        let v3mn=(if sb[64]{v8g}else{(if (sf[181]!=0.0){(v4e*(if (sf[181]!=0.0){((if (sf[181]!=0.0){v8g}else{v38q})*v3li)}else{v391}))}else{v8g})});
        let v3mo=(if sb[64]{v8g}else{(if (sf[181]!=0.0){((v43*(if (sf[181]!=0.0){((if (sf[181]!=0.0){v8g}else{v3e0})*v3ku)}else{v3e4}))+(v4e*(if (sf[181]!=0.0){((if (sf[181]!=0.0){v8g}else{v38r})*v3li)}else{v392})))}else{v8g})});
        let v3mp=(if sb[64]{v8g}else{(if (sf[181]!=0.0){((v43*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(va2/vyx)}else{v8g})*v3ku)}else{v8g}))+(v4e*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(va2/vz2)}else{v38s})*v3li)}else{v393})))}else{v8g})});
        let v3mq=(if sb[64]{v8g}else{(if (sf[181]!=0.0){((v43*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(v1f/vyx)}else{v8g})*v3ku)}else{v8g}))+(v4e*(if (sf[181]!=0.0){((if (sf[181]!=0.0){(v1f/vz2)}else{v8g})*v3li)}else{v8g})))}else{v8g})});
        let v3n0=(if sb[66]{v8g}else{(if (sf[182]!=0.0){(v1f/v15)}else{v8g})});
        let v3n1=(if sb[66]{v8g}else{(if (sf[182]!=0.0){((-(vzi*(sf[15]*(sf[194]*(sf[16]*f64::powf(vc,sf[201]))))))/(v15*v15))}else{v8g})});
        let v3n2=(if sb[66]{v8g}else{(if (sf[182]!=0.0){(va2/v15)}else{v8g})});

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (vub),
            [4, 6, 7, 8, 9, 10, 11],
            [v33x, v33y, v33z, v340, v341, v342, v343],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (vux),
            [4, 6, 7, 8, 9, 10, 11],
            [v36w, v36x, v36y, v36z, v370, v371, v372],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (vqp),
            [4, 6, 7, 8, 9],
            [v2py, v2q1, v2q4, v2q8, v2qc],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(6),
            multiplicity * (vqo),
            [4, 6, 7, 8, 9],
            [v2pg, v2pk, v2pn, v2pr, v2pu],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (vwi),
            [4, 6, 7, 8, 9, 10, 11],
            [v3dd, v3de, v3df, v3dg, v3dh, v3di, v3dj],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (vvs),
            [4, 6, 7, 8, 10],
            [v39s, v39t, v39u, v39v, v39w],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (vwq),
            0,
            multiplicity * (v3dt),
            4,
            multiplicity * (v3du),
            5,
            multiplicity * (v3dv),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (vxw),
            [4, 5, 6, 8],
            [v3i6, v3i7, v3i8, v3i9],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (vy4),
            1,
            multiplicity * (v3ij),
            4,
            multiplicity * (v3ik),
            7,
            multiplicity * (v3il),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (vyc),
            [4, 6, 7, 8, 9],
            [v3j8, v3j9, v3ja, v3jb, v3jc],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (vyk),
            2,
            multiplicity * (v3jm),
            4,
            multiplicity * (v3jn),
            9,
            multiplicity * (v3jo),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (vys),
            [4, 5, 6, 7, 8, 10],
            [v3kc, v3kd, v3ke, v3kf, v3kg, v3kh],
            [],
            [],
            multiplicity,
        );
        let v10e_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v10e);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (v10e_ddt),
            [4, 6, 7, 8, 9],
            [((v3q0) * ddt_scale), ((v3pn) * ddt_scale), ((v3pr) * ddt_scale), ((v3q1) * ddt_scale), ((v3q2) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v10g_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v10g);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v10g_ddt),
            [4, 7, 8, 9],
            [((v3q9) * ddt_scale), ((v3qa) * ddt_scale), ((v3qb) * ddt_scale), ((v3qc) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v10n_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v10n);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v10n_ddt),
            [4, 6, 7, 8, 9],
            [((v3qt) * ddt_scale), ((v3qu) * ddt_scale), ((v3qh) * ddt_scale), ((v3qv) * ddt_scale), ((v3qj) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v10o_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v10o);
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (v10o_ddt),
            4,
            multiplicity * (((v3qw) * ddt_scale)),
            5,
            multiplicity * (((v3qx) * ddt_scale)),
            8,
            multiplicity * (((v3qy) * ddt_scale)),
        );
        let v10r_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v10r);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v10r_ddt),
            [4, 6, 7, 8, 9, 10],
            [((v3rc) * ddt_scale), ((v3rd) * ddt_scale), ((v3re) * ddt_scale), ((v3rf) * ddt_scale), ((v3r5) * ddt_scale), ((v3rg) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v10y_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v10y);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v10y_ddt),
            1,
            multiplicity * (((sf[189]) * ddt_scale)),
            2,
            multiplicity * (((sf[316]) * ddt_scale)),
        );
        let v111_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v111);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v111_ddt),
            0,
            multiplicity * (((sf[317]) * ddt_scale)),
            1,
            multiplicity * (((sf[190]) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(10),
            multiplicity * (vze),
            [4, 6, 7, 8, 10, 11],
            [v3ml, v3mm, v3mn, v3mo, v3mp, v3mq],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (vru),
            [4, 6, 7, 8, 10, 11],
            [v2v0, v2v1, v2v2, v2v3, v2v4, v2v5],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (vzm),
            3,
            multiplicity * (v3n0),
            4,
            multiplicity * (v3n1),
            11,
            multiplicity * (v3n2),
        );
        let v10v_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v10v);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * (v10v_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((v3rj) * ddt_scale), ((v3rk) * ddt_scale), ((v3rl) * ddt_scale), ((v3rm) * ddt_scale), ((v3rn) * ddt_scale), ((v3rr) * ddt_scale), ((v3rs) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if sb[68]{v8g}else{(if (sf[192]!=0.0){(v6/sf[191])}else{v8g})})),
            4,
            multiplicity * (sf[320]),
        );
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * ((-((((((((((((((v9h*vub)+(v9l*vwi))+(vwc*v115))+(v9j*vux))+(v9p*vvs))+(vzi*vzm))+(vn8*vze))+(vru*v11g))+(vwm*vwq))+(vx7*vxw))+(vy0*vy4))+(vy7*vyc))+(vyg*vyk))+(vyn*vys)))),
            &[(-(vwq+(vwm*v3dt))),(-(vy4+(vy0*v3ij))),(-(vyk+(vyg*v3jm))),(-(vzm+(vzi*v3n0))),(-((((((((((((((v9h*v33x)+(v9l*v3dd))+(v115*v3ca))+(v9j*v36w))+(v9p*v39s))+(vzi*v3n1))+(vn8*v3ml))+(v11g*v2v0))+(vwm*v3du))+(vx7*v3i6))+(vy0*v3ik))+(vy7*v3j8))+(vyg*v3jn))+(vyn*v3kc))),(-((((-vwq)+(vwm*v3dv))+(vxw+(vx7*v3i7)))+((-vys)+(vyn*v3kd)))),(-((((((((((v9h*v33y)+((-vwi)+(v9l*v3de)))+(vwc+(v115*v3cb)))+(v9j*v36x))+(v9p*v39t))+(vn8*v3mm))+(v11g*v2v1))+((-vxw)+(vx7*v3i8)))+(vy7*v3j9))+(vyn*v3ke))),(-((((((((((v9h*v33z)+(v9l*v3df))+(v115*v3cc))+(vux+(v9j*v36y)))+(vvs+(v9p*v39u)))+(vn8*v3mn))+(vru+(v11g*v2v2)))+((-vy4)+(vy0*v3il)))+(vyc+(vy7*v3ja)))+(vyn*v3kf))),(-((((((((((vub+(v9h*v340))+(vwi+(v9l*v3dg)))+(v115*v3cd))+(v9j*v36z))+(v9p*v39v))+(vn8*v3mo))+(v11g*v2v3))+(vx7*v3i9))+((-vyc)+(vy7*v3jb)))+(vyn*v3kg))),(-(((((((-vub)+(v9h*v341))+(v9l*v3dh))+((v115*v3ce)+(-vwc)))+((-vux)+(v9j*v370)))+(vy7*v3jc))+((-vyk)+(vyg*v3jo)))),(-(((((((v9h*v342)+(v9l*v3di))+(v9j*v371))+((-vvs)+(v9p*v39w)))+((-vze)+(vn8*v3mp)))+(v11g*v2v4))+(vys+(vyn*v3kh)))),(-((((((v9h*v343)+(v9l*v3dj))+(v9j*v372))+((-vzm)+(vzi*v3n2)))+(vze+(vn8*v3mq)))+((v11g*v2v5)+(-vru))))],
            &[],
            multiplicity,
        );
        let v124_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v124);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v124_ddt),
            4,
            multiplicity * (((sf[193]) * ddt_scale)),
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
            v6, vb, vc, vd, v1f, v1g, v2c, v52, 
            v55, v5t, v6v, v8g, v9f, v9g, v9h, v9i, 
            v9j, v9k, v9l, v9m, v9o, v9p, va2, vn7, 
            vn8, vow, vph, vpn, vqn, vqs, vqu, vqw, 
            vqy, vr0, vr8, vwl, vwr, vws, vwx, vx0, 
            vxz, vyf, v10e, v10g, v10n, v10o, v10r, v10v, 
            v10y, v111, v124, v125, v13k, v14q, v19y, v2ih, 
            v2ii, v2ij, v2ik, v2il, v2im, v2in, v2lh, v2li, 
            v2lj, v2m2, v2m3, v2m4, v2p7, v2p8, v2p9, v2pa, 
            v2pb, v2pf, v2qg, v2qj, v2qk, v2ql, v2qm, v2qn, 
            v2qo, v2qv, v2qw, v2qx, v2qy, v2qz, v2r3, v2r8, 
            v2r9, v2ra, v2rt, v2ru, v2rv, v2rw, v2rx, v3dy, 
            v3dz, v3e0, v3e2, v3e3, v3e4, v3ei, v3ej, v3ek, 
            v3er, v3es, v3et, v3pn, v3pr, v3q0, v3q1, v3q2, 
            v3q9, v3qa, v3qb, v3qc, v3qh, v3qj, v3qt, v3qu, 
            v3qv, v3qw, v3qx, v3qy, v3r5, v3rc, v3rd, v3re, 
            v3rf, v3rg, v3rj, v3rk, v3rl, v3rm, v3rn, v3rr, 
            v3rs, 
        }=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[v3q0, v3pn, v3pr, v3q1, v3q2],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            &[nodes[4], nodes[7], nodes[8], nodes[9]],
            &[v3q9, v3qa, v3qb, v3qc],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[v3qt, v3qu, v3qh, v3qv, v3qj],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[4],
            multiplicity * (v3qw),
            nodes[5],
            multiplicity * (v3qx),
            nodes[8],
            multiplicity * (v3qy),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[v3rc, v3rd, v3re, v3rf, v3r5, v3rg],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (sf[189]),
            nodes[2],
            multiplicity * (sf[316]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (sf[317]),
            nodes[1],
            multiplicity * (sf[190]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]],
            &[v3rj, v3rk, v3rl, v3rm, v3rn, v3rr, v3rs],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (sf[193]),
        );
    }
}
