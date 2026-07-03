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
    v1: f64, v3: f64, vw: f64, vx: f64, v1c: f64, v4g: f64, 
    vbd: f64, vbh: f64, vbt: f64, vcj: f64, vjf: f64, vjj: f64, 
    vjl: f64, vjq: f64, vjt: f64, vjy: f64, vk6: f64, vk9: f64, 
    vkc: f64, vkg: f64, vlh: f64, vli: f64, vlk: f64, vln: bool, 
    vlo: f64, vo0: f64, vre: f64, vt2: f64, vtr: f64, vtu: f64, 
    vtx: f64, vuo: f64, vww: f64, vxw: f64, vxx: f64, vy2: f64, 
    vy3: f64, vym: f64, vyo: f64, vyr: bool, vys: f64, vz1: f64, 
    vzx: f64, vzz: f64, v101: f64, v106: bool, v107: f64, v10e: f64, 
    v10f: f64, v10h: f64, v10m: bool, v10o: f64, v124: f64, v126: f64, 
    v128: f64, v12d: bool, v12e: f64, v135: f64, v13i: f64, v13v: f64, 
    v148: f64, v14f: f64, v14g: f64, v14j: f64, v14l: f64, v14q: bool, 
    v14r: f64, v14x: f64, v151: f64, v154: f64, v15c: f64, v15d: f64, 
    v15e: f64, v15g: f64, v15i: f64, v15m: f64, v15n: f64, v15p: f64, 
    v15s: f64, v15u: f64, v15v: bool, v160: bool, v161: f64, v173: f64, 
    v175: f64, v177: f64, v178: f64, v17b: f64, v17d: f64, v17i: bool, 
    v17j: f64, v17o: f64, v17r: f64, v17t: f64, v181: f64, v182: f64, 
    v183: f64, v185: f64, v18a: f64, v18b: f64, v18d: f64, v18f: f64, 
    v18h: f64, v18i: bool, v18n: bool, v18o: f64, v1ak: f64, v1b1: f64, 
    v1bn: f64, v1dn: f64, v1dz: f64, v1ec: bool, v1ed: bool, v1ee: f64, 
    v1eh: bool, v1ei: f64, v1em: f64, v1en: f64, v1ep: f64, v1et: f64, 
    v1ev: f64, v1f0: bool, v1f1: f64, v1fg: bool, v1if: bool, v1ig: f64, 
    v1ii: f64, v1ik: f64, v1im: f64, v1io: f64, v1ip: bool, v1ir: bool, 
    v1iz: f64, v1j2: bool, v1j3: f64, v1j4: f64, v1ja: bool, v1jc: f64, 
    v1jd: f64, v1jh: f64, v1jj: f64, v1jm: f64, v1jo: f64, v1jt: bool, 
    v1ju: f64, v1ta: f64, v1u6: f64, v1vd: f64, v1vg: f64, v1vj: f64, 
    v1vm: f64, v1vq: f64, v1vu: f64, v1w2: f64, v1w8: f64, v1wj: f64, 
    v1xp: f64, v1xq: f64, v1xr: f64, v1xs: f64, v212: f64, v213: f64, 
    v214: f64, v293: f64, v294: f64, v295: f64, v2d7: f64, v2d8: f64, 
    v2d9: f64, v2ee: f64, v2ef: f64, v2eg: f64, v2en: f64, v2eo: f64, 
    v2ep: f64, v2ew: f64, v2ex: f64, v2ey: f64, v2fu: f64, v2fv: f64, 
    v2ku: f64, v2kv: f64, v2kw: f64, v2ne: f64, v2nf: f64, v2ng: f64, 
    v2nh: f64, v2nk: f64, v2nn: f64, v2nq: f64, v2nt: f64, v2nu: f64, 
    v2nv: f64, v2nw: f64, v2ny: f64, v2o2: f64, v2o5: f64, v2p3: f64, 
    v2p4: f64, v2qr: f64, v2qs: f64, v2uh: f64, v2ui: f64, v2uj: f64, 
    v2w2: f64, v2w3: f64, v2w4: f64, v2wh: f64, v2wi: f64, v2wj: f64, 
    v2x4: f64, v2x5: f64, v2x6: f64, v2x7: f64, v2x8: f64, v2xp: f64, 
    v2xq: f64, v2xr: f64, v2xs: f64, v2xt: f64, v3aj: f64, v3ak: f64, 
    v3al: f64, v3am: f64, v3az: f64, v3b0: f64, v3b1: f64, v3b2: f64, 
    v3b3: f64, v3b4: f64, v3b5: f64, v3b6: f64, v3en: f64, v3eo: f64, 
    v3ep: f64, v3eq: f64, v3er: f64, v3es: f64, v3et: f64, v3eu: f64, 
    v3nb: f64, v3nc: f64, v3nd: f64, v3ne: f64, v4yi: f64, v4yj: f64, 
    v4yk: f64, v4yl: f64, v4ym: f64, v4yn: f64, v539: f64, v53a: f64, 
    v53b: f64, v53c: f64, v53d: f64, v53e: f64, v53s: f64, v53t: f64, 
    v53y: f64, v53z: f64, v540: f64, v541: f64, v542: f64, v543: f64, 
    v54g: f64, v54h: f64, v54i: f64, v54j: f64, v54k: f64, v54l: f64, 
    v562: f64, v563: f64, v564: f64, v565: f64, v566: f64, v567: f64, 
    v568: f64, v569: f64, v57d: f64, v57e: f64, v57f: f64, v57g: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v1=1.0;let v3=0.0;let vw=0.001;let vx=2.0;let v1c=0.1;let v4g=3.0;let vbd=1e-6;let vbh=0.5;let vbt=4.0;let vcj=6.0;let vjc=ctx.node_voltage(nodes[5]);let vjd=ctx.node_voltage(nodes[6]);let vjf=(sf[0]*(vjc-vjd));let vjg=ctx.node_voltage(nodes[7]);let vji=(sf[0]*(vjc-vjg));let vjj=ctx.node_voltage(nodes[3]);let vjl=(sf[0]*(vjc-vjj));let vjm=ctx.node_voltage(nodes[4]);let vjo=(sf[0]*(vjm-vjj));let vjq=(sf[0]*(vjm-vjc));let vjs=(sf[0]*(vjd-vjg));let vjt=ctx.node_voltage(nodes[2]);
        let vjw=ctx.node_voltage(nodes[1]);let vjy=(sf[0]*(vjw-vjm));let vk3=(sf[0]*(vjw-ctx.node_voltage(nodes[0])));let vk4=ctx.node_voltage(nodes[9]);let vk6=(sf[0]*(vk4-vjd));let vk9=(sf[0]*(ctx.node_voltage(nodes[8])-vk4));let vkc=(((vji+vjq)-vjs)-vk6);let vkg=((vkc+(vjy+(-vk3)))-vk9);let vkh=(vk3+vkg);let vki=(sf[381]*vji);let vkl=(if (vki<sf[198]){v1}else{v3});let vkm=(vki).exp();let vko=(!(vkl!=0.0));let vkq=(if vko{sf[199]}else{v3});let vkv=(sf[381]*vjl);let vkw=(vkv/sf[588]);
        let vky=(if (vkw<sf[198]){v1}else{v3});let vkz=(vkw).exp();let vl1=(!(vky!=0.0));let vl2=(if vl1{sf[199]}else{vkq});let vl6=(if vl1{(vl2*(v1+(vkw-sf[198])))}else{(if (vky!=0.0){vkz}else{v3})});let vl7=(sf[381]*vkc);let vl9=(if (vl7<sf[198]){v1}else{v3});let vla=(vl7).exp();let vlc=(!(vl9!=0.0));let vld=(if vlc{sf[199]}else{vl2});let vlh=(if vlc{(vld*(v1+(vl7-sf[198])))}else{(if (vl9!=0.0){vla}else{v3})});let vli=(sf[381]*vjq);let vlk=(if (vli<sf[198]){v1}else{v3});let vln=(!(vlk!=0.0));
        let vlo=(if vln{sf[199]}else{vld});let vlt=(sf[381]*vkh);let vlv=(if (vlt<sf[198]){v1}else{v3});let vlw=(vlt).exp();let vly=(!(vlv!=0.0));let vlz=(if vly{sf[199]}else{vlo});let vm3=(if vly{(vlz*(v1+(vlt-sf[198])))}else{(if (vlv!=0.0){vlw}else{v3})});let vm5=(sf[381]*(vkh-sf[469]));let vm7=(if (vm5<sf[198]){v1}else{v3});let vm8=(vm5).exp();let vma=(!(vm7!=0.0));let vmb=(if vma{sf[199]}else{vlz});let vmh=(sf[381]*(vkc-sf[469]));let vmj=(if (vmh<sf[198]){v1}else{v3});let vmk=(vmh).exp();
        let vmm=(!(vmj!=0.0));let vmn=(if vmm{sf[199]}else{vmb});let vmt=(sf[381]*(vji-sf[469]));let vmv=(if (vmt<sf[198]){v1}else{v3});let vmw=(vmt).exp();let vmy=(!(vmv!=0.0));let vmz=(if vmy{sf[199]}else{vmn});let vn3=(if vmy{(vmz*(v1+(vmt-sf[198])))}else{(if (vmv!=0.0){vmw}else{v3})});let vn5=(sf[381]*(vjf-sf[469]));let vn7=(if (vn5<sf[198]){v1}else{v3});let vn8=(vn5).exp();let vna=(!(vn7!=0.0));let vnb=(if vna{sf[199]}else{vmz});
        let vnf=(if vna{(vnb*(v1+(vn5-sf[198])))}else{(if (vn7!=0.0){vn8}else{v3})});let vni=((v1+(vbt*vn3))).sqrt();let vnl=((v1+(vbt*vnf))).sqrt();let vnm=(vx*vnf);let vnn=(v1+vnl);let vno=(vnm/vnn);let vnr=(if (vno<sf[200]){v1}else{v3});let vns=(if (vnr!=0.0){sf[200]}else{vno});let vnu=(v1+vni);let vnv=(vnu/vnn);let vny=(sf[380]*((vni-vnl)-(vnv).ln()));let vo0=((vjs+vny)/sf[564]);let vo2=(if (vo0>v3){v1}else{v3});let vo3=100.0;let vo5=(if (vjf<vo3){v1}else{v3});let vo6=((vo2!=0.0)&&(vo5!=0.0));
        let vo9=((vo2!=0.0)&&(!(vo5!=0.0)));let vob=(v1+(vjf-vo3));let voh=(sf[564]*(vbh*vo0));let voj=(v1+(sf[381]*voh));let voo=(if (vo2!=0.0){((sf[469]+(sf[795]*(voj).ln()))-(if vo9{(vo3+(vob).ln())}else{(if vo6{vjf}else{v3})}))}else{v3});let vor=(if (vo2!=0.0){sf[796]}else{v3});let vot=(if (vo2!=0.0){(vor*vor)}else{vbd});let vox=(if (voo<v3){v1}else{v3});let voy=((vo2!=0.0)&&(vox!=0.0));let voz=(vbh*vot);let vp1=((vot+(if (vo2!=0.0){(voo*voo)}else{sf[616]}))).sqrt();let vp2=(vp1-voo);
        let vp6=((vo2!=0.0)&&(!(vox!=0.0)));let vp9=(if vp6{(vbh*(voo+vp1))}else{(if voy{(voz/vp2)}else{v3})});let vpd=(vp9+sf[203]);let vpe=(vp9*vpd);let vph=(sf[202]*(vp9+sf[797]));let vpj=(if (vo2!=0.0){(vpe/vph)}else{v3});let vpl=(if (vo2!=0.0){(vo0/vpj)}else{v3});let vpp=(if (vo2!=0.0){((vpl-v1)/sf[204])}else{sf[595]});let vpr=(if (vpl<v1){v1}else{v3});let vps=((vo2!=0.0)&&(vpr!=0.0));let vpt=(vpp).exp();let vpu=(v1+vpt);let vq0=((vo2!=0.0)&&(!(vpr!=0.0)));let vq2=((-vpp)).exp();let vq3=(v1+vq2);
        let vqg=(if (vo2!=0.0){((if vq0{(vpl+(sf[204]*(vq3).ln()))}else{(if vps{(v1+(sf[204]*(vpu).ln()))}else{v3})})/sf[210])}else{v3});let vqi=(if (vo2!=0.0){(vp9/sf[203])}else{v3});let vqj=(vbt*vqg);let vqk=(vqi*vqj);let vql=(v1+vqi);let vqo=((v1+(vqk*vql))).sqrt();let vqp=(v1+vqo);let vqq=(vx*vqg);let vqr=(vql*vqq);let vqt=(if (vo2!=0.0){(vqp/vqr)}else{v3});let vqv=(vns*vqt);let vqw=((v1-vqt)+vqv);let vqx=(v1+vqv);let vqz=(if (vo2!=0.0){(vqw/vqx)}else{v3});
        let vr2=(if (vo2!=0.0){(sf[381]*(voh*vqz))}else{v3});let vr5=(v1+(vns+vr2));let vr8=(if (vo2!=0.0){((vx*vr2)+(vns*vr5))}else{v3});let vrb=(if (vo2!=0.0){(vbh*(vr2-v1))}else{v3});let vre=(if (vo2!=0.0){(vr8+(vrb*vrb))}else{v3});let vrg=(if (vr2>=v1){v1}else{v3});let vrh=((vo2!=0.0)&&(vrg!=0.0));let vri=(vre).sqrt();let vrm=((vo2!=0.0)&&(!(vrg!=0.0)));let vrn=(vri-vrb);let vrp=(if vrm{(vr8/vrn)}else{(if vrh{(vrb+vri)}else{v3})});let vrt=((vo2!=0.0)&&((if (vrp<sf[211]){v1}else{v3})!=0.0));
        let vru=(if vrt{sf[211]}else{vrp});let vrv=(v1+vru);let vs4=(if (vo2!=0.0){(sf[212]*(vo0-sf[201]))}else{v3});let vsb=(((if (vo2!=0.0){(vo0*sf[801])}else{v3})+(vs4*vs4))).sqrt();let vsl=((vo2!=0.0)&&sb[20]);let vsm=(vx*vo0);let vsn=(vo0+vpj);let vss=(vo0*sf[201]);let vst=(vo0+sf[201]);let vsy=(!(vo2!=0.0));let vsz=(vx*vn3);let vt2=(if vsy{(if vko{(vkq*(v1+(vki-sf[198])))}else{(if (vkl!=0.0){vkm}else{v3})})}else{(if (vo2!=0.0){((vru*vrv)*sf[799])}else{v3})});
        let vte=(if (((vjs).abs()<sf[803])||((vny).abs()<(sf[804]*(vni+vnl)))){v1}else{v3});let vtf=(vsy&&(vte!=0.0));let vtg=(vns+(if vsy{(vsz/vnu)}else{vru}));let vti=(if vtf{(vbh*vtg)}else{v3});let vtj=(v1+vti);let vtn=(vsy&&(!(vte!=0.0)));let vtp=((vji+vny)-vjf);let vtr=(if vtn{(vny/vtp)}else{(if vtf{(vti/vtj)}else{vqz})});let vtt=(if vsy{sf[802]}else{(if vsl{(sf[507]*(v1c+(vsm/vsn)))}else{(if ((vo2!=0.0)&&(sf[214]!=0.0)){sf[802]}else{v3})})});
        let vtu=(if vsy{vo0}else{(if (vo2!=0.0){(vss/vst)}else{v3})});let vtx=(if vsy{(v1-(vtu/sf[201]))}else{(if (vo2!=0.0){(sf[201]/vst)}else{v3})});let vu4=((vjl-sf[805])/sf[806]);let vu6=(if (vjl<sf[805]){v1}else{v3});let vu7=(vu4).exp();let vu8=(v1+vu7);let vud=(!(vu6!=0.0));let vuf=((-vu4)).exp();let vug=(v1+vuf);let vuk=(if vud{(sf[805]-(sf[806]*(vug).ln()))}else{(if (vu6!=0.0){(vjl-(sf[806]*(vu8).ln()))}else{v3})});let vum=(v1-(sf[528]*vuk));let vuo=f64::powf(vum,sf[218]);
        let vuu=((sf[807]*(v1-vuo))+(v4g*(vjl-vuk)));let vv7=(if sb[26]{vji}else{(if sb[24]{(vjf+(if vsy{vjs}else{(if (vo2!=0.0){(vs4+vsb)}else{v3})}))}else{(if (sf[220]!=0.0){vjf}else{v3})})});let vvf=(vv7-sf[813]);let vvg=(vvf/vtt);let vvi=(if (vv7<sf[813]){v1}else{v3});let vvj=(vvg).exp();let vvk=(v1+vvj);let vvl=(vvk).ln();let vvp=(!(vvi!=0.0));let vvr=((-vvg)).exp();let vvs=(v1+vvr);let vvt=(vvs).ln();let vvw=(if vvp{(sf[813]-(vtt*vvt))}else{(if (vvi!=0.0){(vv7-(vtt*vvl))}else{v3})});
        let vvy=f64::powf(vtx,sf[223]);let vw2=(v1-(vvw/sf[507]));let vw3=f64::powf(vw2,sf[224]);let vw7=(sf[810]*vvy);let vw8=(vv7-vvw);let vwd=((sf[809]*((sf[814]*(v1-(vvy*vw3)))+(vw7*vw8)))+(sf[541]*vjf));let vwg=(vl6*sf[816]);let vwi=((v1+vwg)).sqrt();let vwj=(v1+vwi);let vwk=(vwg/vwj);let vwm=f64::powf(vt2,sf[817]);let vwn=(sf[816]*vwm);let vwp=((v1+vwn)).sqrt();let vwq=(v1+vwp);let vwr=(vwn/vwq);let vwv=(v1+(vuu/sf[750]));let vww=(vwd/sf[748]);let vwx=(vwv+vww);
        let vx8=((if sb[28]{(sf[381]*(sf[779]*vwv))}else{v3})).exp();let vx9=((if sb[28]{(sf[381]*(sf[779]*((-vwd)/sf[748])))}else{v3})).exp();let vxf=(if sb[28]{((vx8-vx9)/sf[820])}else{(if (sf[225]!=0.0){vwx}else{v3})});let vxg=0.010000000000000002;let vxh=(vxf*vxf);let vxj=(if (vxf<v3){v1}else{v3});let vxk=0.005000000000000001;let vxm=((vxg+vxh)).sqrt();let vxn=(vxm-vxf);let vxq=(!(vxj!=0.0));let vxt=(if vxq{(vbh*(vxf+vxm))}else{(if (vxj!=0.0){(vxk/vxn)}else{v3})});let vxw=(v1+(vbh*(vwk+vwr)));
        let vxx=(vxt*vxw);let vy0=(vwm*sf[821]);let vy1=(sf[633]*vl6);let vy2=(vy1-vy0);let vy3=(vy2/vxx);let vy4=0.0001;let vy5=(vjl/vy4);let vy6=(vjl<v3);let vy7=(if vy6{v1}else{v3});let vy8=(vy5).exp();let vy9=(v1+vy8);let vyd=(!(vy7!=0.0));let vyf=((-vy5)).exp();let vyg=(v1+vyf);let vyk=(if vyd{(vjl+(vy4*(vyg).ln()))}else{(if (vy7!=0.0){(vy4*(vy9).ln())}else{v3})});let vym=(vyk/sf[227]);let vyo=(if (vym<sf[198]){v1}else{v3});let vyr=(!(vyo!=0.0));let vys=(if vyr{sf[199]}else{vnb});
        let vz1=((vjl-sf[228])/vw);let vzn=(vkv/sf[143]);let vzp=(if (vzn<sf[198]){v1}else{v3});let vzq=(vzn).exp();let vzs=(!(vzp!=0.0));let vzt=(if vzs{sf[199]}else{vys});let vzx=(if vzs{(vzt*(v1+(vzn-sf[198])))}else{(if (vzp!=0.0){vzq}else{vyk})});let vzz=(sf[381]*(vjl-sf[527]));let v101=(if (vzz<sf[198]){v1}else{v3});let v106=((sf[149]!=0.0)&&(!(v101!=0.0)));let v107=(if v106{sf[199]}else{vzt});let v10e=((vy3/sf[633])-1000.0);let v10f=40.0;let v10h=(if (v10e<v10f){v1}else{v3});
        let v10m=((sf[149]!=0.0)&&(!(v10h!=0.0)));let v10o=(if v10m{2.3538526683702e17}else{v107});let v11t=(sf[381]*vjo);let v11u=(v11t/sf[147]);let v11w=(if (v11u<sf[198]){v1}else{v3});let v11x=(v11u).exp();let v11z=(!(v11w!=0.0));let v120=(if v11z{sf[199]}else{v10o});let v124=(if v11z{(v120*(v1+(v11u-sf[198])))}else{(if (v11w!=0.0){v11x}else{vzx})});let v126=(sf[381]*(vjo-sf[527]));let v128=(if (v126<sf[198]){v1}else{v3});let v12d=((sf[149]!=0.0)&&(!(v128!=0.0)));let v12e=(if v12d{sf[199]}else{v120});
        let v12v=(vkv/sf[130]);let v12x=(if (v12v<sf[198]){v1}else{v3});let v12y=(v12v).exp();let v130=(!(v12x!=0.0));let v131=(if v130{sf[199]}else{v12e});let v135=(if v130{(v131*(v1+(v12v-sf[198])))}else{(if (v12x!=0.0){v12y}else{v124})});let v138=(v11t/sf[165]);let v13a=(if (v138<sf[198]){v1}else{v3});let v13b=(v138).exp();let v13d=(!(v13a!=0.0));let v13e=(if v13d{sf[199]}else{v131});let v13i=(if v13d{(v13e*(v1+(v138-sf[198])))}else{(if (v13a!=0.0){v13b}else{v135})});let v13l=(vl7/sf[136]);
        let v13n=(if (v13l<sf[198]){v1}else{v3});let v13o=(v13l).exp();let v13q=(!(v13n!=0.0));let v13r=(if v13q{sf[199]}else{v13e});let v13v=(if v13q{(v13r*(v1+(v13l-sf[198])))}else{(if (v13n!=0.0){v13o}else{v13i})});let v13y=(v11t/sf[169]);let v140=(if (v13y<sf[198]){v1}else{v3});let v141=(v13y).exp();let v143=(!(v140!=0.0));let v144=(if v143{sf[199]}else{v13r});let v148=(if v143{(v144*(v1+(v13y-sf[198])))}else{(if (v140!=0.0){v141}else{v13v})});let v14f=(if (vy6&&sb[36]){v1}else{v3});let v14g=(vx*vuo);
        let v14j=(sf[715]*(v1-(sf[20]/v14g)));let v14l=(if (v14j<sf[198]){v1}else{v3});let v14q=((v14f!=0.0)&&(!(v14l!=0.0)));let v14r=(if v14q{sf[199]}else{v144});let v14x=(if (v14f!=0.0){(sf[528]*vjl)}else{sf[746]});let v14z=1e-30;let v151=(((v14x*v14x)+v14z)).sqrt();let v154=f64::powf(v151,sf[233]);let v15c=(vcj*v14x);let v15d=(v14x*v15c);let v15e=(v14x+sf[236]);let v15g=((sf[18]*(sf[235]-((v4g*v14x)*sf[236])))-(v15d*v15e));let v15i=0.16666666666666666;let v15m=(sf[715]*(sf[20]*vjl));
        let v15n=(sf[405]*(if (v14f!=0.0){((v154*v15g)*v15i)}else{v3}));let v15p=(if (v14f!=0.0){(v15m/v15n)}else{v14x});let v15q=-0.001;let v15s=(if (v15p<v15q){v1}else{v3});let v15u=(if (v15p<sf[198]){v1}else{v3});let v15v=((v14f!=0.0)&&(v15s!=0.0));let v160=(v15v&&(!(v15u!=0.0)));let v161=(if v160{sf[199]}else{v14r});let v173=(if (sb[39]&&(vjf<v3)){v1}else{v3});let v174=(sf[529]*vjf);let v175=(v1-v174);let v177=(if (v173!=0.0){f64::powf(v175,sf[224])}else{v3});let v178=(vx*v177);
        let v17b=(sf[735]*(v1-(sf[52]/v178)));let v17d=(if (v17b<sf[198]){v1}else{v3});let v17i=((v173!=0.0)&&(!(v17d!=0.0)));let v17j=(if v17i{sf[199]}else{v161});let v17o=(if (v173!=0.0){v174}else{sf[726]});let v17r=((v14z+(v17o*v17o))).sqrt();let v17t=f64::powf(v17r,sf[237]);let v181=(vcj*v17o);let v182=(v17o*v181);let v183=(v17o+sf[240]);let v185=((sf[50]*(sf[239]-((v4g*v17o)*sf[240])))-(v182*v183));let v18a=(sf[735]*(sf[52]*vjf));let v18b=(sf[426]*(if (v173!=0.0){(v15i*(v17t*v185))}else{v3}));
        let v18d=(if (v173!=0.0){(v18a/v18b)}else{v17o});let v18f=(if (v18d<v15q){v1}else{v3});let v18h=(if (v18d<sf[198]){v1}else{v3});let v18i=((v173!=0.0)&&(v18f!=0.0));let v18n=(v18i&&(!(v18h!=0.0)));let v18o=(if v18n{sf[199]}else{v17j});let v19j=(vlh*sf[816]);let v19k=(vbt*(if vmm{(vmn*(v1+(vmh-sf[198])))}else{(if (vmj!=0.0){vmk}else{v3})}));let v19l=(v19j-sf[816]);let v19n=((v1+v19j)).sqrt();let v19o=(v1+v19n);let v19r=((v1+v19k)).sqrt();let v19s=(v1+v19r);let v1ae=(sf[829]*(vm3-v1));
        let v1ah=((v1+(vm3*sf[828]))).sqrt();let v1ai=(v1+v1ah);let v1ak=(if (sf[242]!=0.0){(v1ae/v1ai)}else{v3});let v1ax=(if sb[44]{(vkh-sf[837])}else{v3});let v1b1=(if sb[44]{(v1ax*v1ax)}else{vxh});let v1b3=(if (v1ax<v3){v1}else{v3});let v1b4=(sb[44]&&(v1b3!=0.0));let v1b7=((sf[245]+v1b1)).sqrt();let v1b8=(v1b7-v1ax);let v1bc=(sb[44]&&(!(v1b3!=0.0)));let v1bf=(if v1bc{(vbh*(v1ax+v1b7))}else{(if v1b4{(sf[246]/v1b8)}else{v3})});let v1bi=(v1bf+(sf[832]+(sf[557]*v1ak)));
        let v1bn=(if sb[46]{v1}else{(if sb[44]{(v1bf/v1bi)}else{v1})});let v1de=(if (vwx<v3){v1}else{v3});let v1dg=((vxg+(vwx*vwx))).sqrt();let v1dh=(v1dg-vwx);let v1dk=(!(v1de!=0.0));let v1dn=(if v1dk{(vbh*(vwx+v1dg))}else{(if (v1de!=0.0){(vxk/v1dh)}else{v3})});let v1dz=(if (vy3>v3){v1}else{v3});let v1e5=(if (vjf<sf[268]){v1}else{v3});let v1e8=((-vy3)/sf[269]);let v1ea=(if (v1e8<sf[198]){v1}else{v3});let v1ec=((v1e5!=0.0)&&((v1dz!=0.0)&&(sf[267]!=0.0)));let v1ed=((v1ea!=0.0)&&v1ec);let v1ee=(v1e8).exp();
        let v1eh=(v1ec&&(!(v1ea!=0.0)));let v1ei=(if v1eh{sf[199]}else{v18o});let v1em=(if v1eh{(v1ei*(v1+(v1e8-sf[198])))}else{(if v1ed{v1ee}else{v3})});let v1en=(sf[268]-vjf);let v1ep=(if v1ec{(v1em*v1en)}else{v3});let v1et=(sf[838]*f64::powf(v1ep,sf[270]));let v1ev=(if (v1et<sf[198]){v1}else{v3});let v1f0=(v1ec&&(!(v1ev!=0.0)));let v1f1=(if v1f0{sf[199]}else{v1ei});let v1fg=((v1dz!=0.0)&&sb[51]);let v1if=((v1e5!=0.0)&&((sf[285]!=0.0)&&(v1fg&&sb[55])));let v1ig=f64::powf(v1en,sf[270]);
        let v1ii=(vy3+sf[286]);let v1ik=(v1-(vy3/v1ii));let v1im=f64::powf(v1ik,sf[287]);let v1io=(if v1if{(v1ig*v1im)}else{v3});let v1ip=((sf[279]!=0.0)&&v1if);let v1ir=(sb[53]&&v1if);let v1iv=(if v1ir{((vy3-sf[288])/sf[286])}else{v3});let v1iz=(if v1ir{((v1iv-v1)/sf[289])}else{vz1});let v1j1=(if (v1iv<v1){v1}else{v3});let v1j2=(v1ir&&(v1j1!=0.0));let v1j3=(v1iz).exp();let v1j4=(v1+v1j3);let v1ja=(v1ir&&(!(v1j1!=0.0)));let v1jc=((-v1iz)).exp();let v1jd=(v1+v1jc);
        let v1jh=(if v1ja{(v1iv+(sf[289]*(v1jd).ln()))}else{(if v1j2{(v1+(sf[289]*(v1j4).ln()))}else{v3})});let v1jj=f64::powf(v1jh,sf[290]);let v1jm=(sf[838]*(if v1ir{(v1io*v1jj)}else{(if v1ip{v1io}else{v3})}));let v1jo=(if (v1jm<sf[198]){v1}else{v3});let v1jt=(v1if&&(!(v1jo!=0.0)));let v1ju=(if v1jt{sf[199]}else{v1f1});let v1lk=((vjo-sf[805])/sf[806]);let v1lm=(if (vjo<sf[805]){v1}else{v3});let v1ln=(v1lk).exp();let v1lo=(v1+v1ln);let v1lt=(!(v1lm!=0.0));let v1lv=((-v1lk)).exp();let v1lw=(v1+v1lv);
        let v1m0=(if v1lt{(sf[805]-(sf[806]*(v1lw).ln()))}else{(if (v1lm!=0.0){(vjo-(sf[806]*(v1lo).ln()))}else{v3})});let v1m3=(v1-(sf[528]*v1m0));let v1mg=(vwk*sf[846]);let v1mh=(v1dn*v1mg);let v1mi=(vwr*sf[846]);let v1mj=(v1dn*v1mi);let v1ml=((vkc-sf[813])/sf[802]);let v1mn=(if (vkc<sf[813]){v1}else{v3});let v1mo=(v1ml).exp();let v1mp=(v1+v1mo);let v1mu=(!(v1mn!=0.0));let v1mw=((-v1ml)).exp();let v1mx=(v1+v1mw);
        let v1n1=(if v1mu{(sf[813]-(sf[802]*(v1mx).ln()))}else{(if (v1mn!=0.0){(vkc-(sf[802]*(v1mp).ln()))}else{v3})});let v1n3=(v1-(v1n1/sf[507]));let v1ni=((vkh-sf[813])/sf[802]);let v1nk=(if (vkh<sf[813]){v1}else{v3});let v1nl=(v1ni).exp();let v1nm=(v1+v1nl);let v1nr=(!(v1nk!=0.0));let v1nt=((-v1ni)).exp();let v1nu=(v1+v1nt);let v1ny=(if v1nr{(sf[813]-(sf[802]*(v1nu).ln()))}else{(if (v1nk!=0.0){(vkh-(sf[802]*(v1nm).ln()))}else{v3})});let v1o0=(v1-(v1ny/sf[507]));let v1ok=(vjl/sf[851]);
        let v1om=(if (v1ok<sf[198]){v1}else{v3});let v1on=(v1ok).exp();let v1op=(!(v1om!=0.0));let v1oq=(if v1op{sf[199]}else{v1ju});let v1ov=(sf[850]*(if v1op{(v1oq*(v1+(v1ok-sf[198])))}else{(if (v1om!=0.0){v1on}else{v148})}));let v1p0=(vtr*sf[855]);let v1p1=(vx+vtg);let v1pg=(sf[381]*((vkc-sf[488])/sf[301]));let v1pi=(if (v1pg<sf[198]){v1}else{v3});let v1pk=((v1pi!=0.0)&&sb[60]);let v1pl=(v1pg).exp();let v1po=(sb[60]&&(!(v1pi!=0.0)));let v1pp=(if v1po{sf[199]}else{v1oq});let v1pv=(vlh*sf[857]);
        let v1py=((v1+(vbt*(if v1po{(v1pp*(v1+(v1pg-sf[198])))}else{(if v1pk{v1pl}else{v3})})))).sqrt();let v1pz=(v1+v1py);let v1q1=(if sb[60]{(v1pv/v1pz)}else{(if (sf[300]!=0.0){((sf[856]*(((v19l/v19o)*sf[845])+((v19k/v19s)*sf[854])))/sf[763])}else{v3})});let v1qa=(if sb[64]{(vm3*sf[816])}else{v3});let v1qb=(v1qa-sf[816]);let v1qd=((v1+v1qa)).sqrt();let v1qe=(v1+v1qd);let v1qi=(if sb[64]{(vbt*(if vma{(vmb*(v1+(vm5-sf[198])))}else{(if (vm7!=0.0){vm8}else{v3})}))}else{v3});let v1qk=((v1+v1qi)).sqrt();
        let v1ql=(v1+v1qk);let v1qx=(sf[381]*(vkh-sf[488]));let v1qz=(if (v1qx<sf[198]){v1}else{v3});let v1r1=((v1qz!=0.0)&&sb[65]);let v1r2=(v1qx).exp();let v1r5=(sb[65]&&(!(v1qz!=0.0)));let v1r6=(if v1r5{sf[199]}else{v1pp});let v1rc=(vm3*sf[859]);let v1rf=((v1+(vbt*(if v1r5{(v1r6*(v1+(v1qx-sf[198])))}else{(if v1r1{v1r2}else{v3})})))).sqrt();let v1rg=(v1+v1rf);
        let v1ri=(if sb[65]{(v1rc/v1rg)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(v1qb/v1qe)}else{v3}))+(sf[854]*(if sb[64]{(v1qi/v1ql)}else{v3}))))/sf[763])}else{v3})});let v1rr=(if (sf[305]!=0.0){(f64::powf(vum,sf[306])-v4g)}else{v3});let v1rs=(if (sf[305]!=0.0){vu4}else{v3});let v1ru=(if (v1rs<v3){v1}else{v3});let v1rv=((sf[305]!=0.0)&&(v1ru!=0.0));let v1rw=(v1rs).exp();let v1rx=(v1+v1rw);let v1s1=((sf[305]!=0.0)&&(!(v1ru!=0.0)));let v1s3=((-v1rs)).exp();let v1s4=(v1+v1s3);
        let v1s6=(if v1s1{(v1s3/v1s4)}else{(if v1rv{(v1/v1rx)}else{v3})});let v1sd=((sf[381]*vwg)/sf[588]);let v1se=(vbh/vwi);let v1sg=(if (sf[305]!=0.0){(v1sd*v1se)}else{v3});let v1sh=(v1dn*sf[846]);let v1sm=(vjq*0.2);let v1so=((if (sf[305]!=0.0){(v1ov/sf[851])}else{v3})+((if (sf[305]!=0.0){(sf[842]*(if (sf[305]!=0.0){(v4g+(v1rr*v1s6))}else{v3}))}else{v3})+(if (sf[305]!=0.0){(v1sg*v1sh)}else{v3})));let v1sx=(if (sf[305]!=0.0){(v1mh+(v1ov*sf[307]))}else{v3});
        let v1t6=(if sb[67]{v1mh}else{(if (sf[305]!=0.0){(v1sx*sf[310])}else{v3})});let v1t7=(if sb[67]{v1mj}else{(if (sf[305]!=0.0){(v1mj+(v1sx*sf[309]))}else{v3})});let v1t9=(vy0+vy1);let v1ta=(v1t9/vxx);let v1tk=(if (v1ta>v3){v1}else{v3});let v1tl=(v1t6+v1t7);let v1to=(!(v1tk!=0.0));let v1tp=(sf[759]*v1dn);let v1tr=(if v1to{(vxx*v1tp)}else{(if (v1tk!=0.0){(v1tl/v1ta)}else{v3})});let v1u6=(if sb[75]{v3}else{(if sb[73]{(v1tr*sf[316])}else{(if (sf[314]!=0.0){(sf[309]*v1tr)}else{v3})})});
        let v1vd=(sf[0]*((if sb[67]{v1ov}else{(if (sf[305]!=0.0){(v1ov*sf[308])}else{v3})})+((vuu*sf[842])+v1t6)));let v1vg=(sf[0]*(sf[843]*((sf[807]*(v1-f64::powf(v1m3,sf[218])))+(v4g*(vjo-v1m0)))));let v1vj=(sf[0]*((v1p0*v1p1)+((vwd*sf[844])+v1t7)));let v1vm=(sf[0]*(if (sf[305]!=0.0){(v1sm*v1so)}else{v3}));let v1vq=((sf[0]*(vjw-vjt))*sf[319]);let v1vu=(vk3*sf[320]);
        let v1w2=(sf[0]*((sf[6]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(v1-f64::powf(v1o0,sf[224])))+(sf[810]*(vkh-v1ny))))+(sf[541]*vkh)))))+(if (sf[302]!=0.0){(v1bn*v1ri)}else{v3})));let v1w8=(sf[0]*((sf[7]*((sf[540]*((sf[809]*((sf[814]*(v1-f64::powf(v1n3,sf[224])))+(sf[810]*(vkc-v1n1))))+(sf[541]*vkc)))*sf[296]))+(if (sf[302]!=0.0){(sf[7]*v1q1)}else{v1q1})));let v1wj=ctx.node_voltage(nodes[10]);let v1x9=(if vl1{(vl2*sf[862])}else{(if (vky!=0.0){(vkz*sf[862])}else{v3})});
        let v1xa=(if vl1{(vl2*sf[863])}else{(if (vky!=0.0){(vkz*sf[863])}else{v3})});let v1xp=(if vlc{(vld*sf[860])}else{(if (vl9!=0.0){(vla*sf[860])}else{v3})});let v1xq=(if vlc{(vld*sf[864])}else{(if (vl9!=0.0){(vla*sf[864])}else{v3})});let v1xr=(if vlc{(vld*sf[865])}else{(if (vl9!=0.0){(vla*sf[865])}else{v3})});let v1xs=(if vlc{(vld*sf[861])}else{(if (vl9!=0.0){(vla*sf[861])}else{v3})});let v1ye=(if vly{(vlz*sf[864])}else{(if (vlv!=0.0){(vlw*sf[864])}else{v3})});
        let v1yf=(if vly{(vlz*sf[866])}else{(if (vlv!=0.0){(vlw*sf[866])}else{v3})});let v1yg=(if vly{(vlz*sf[865])}else{(if (vlv!=0.0){(vlw*sf[865])}else{v3})});let v1yh=(if vly{(vlz*sf[861])}else{(if (vlv!=0.0){(vlw*sf[861])}else{v3})});let v1zk=(if vmy{(vmz*sf[860])}else{(if (vmv!=0.0){(vmw*sf[860])}else{v3})});let v1zl=(if vmy{(vmz*sf[861])}else{(if (vmv!=0.0){(vmw*sf[861])}else{v3})});let v1zs=(if vna{(vnb*sf[860])}else{(if (vn7!=0.0){(vn8*sf[860])}else{v3})});
        let v1zt=(if vna{(vnb*sf[861])}else{(if (vn7!=0.0){(vn8*sf[861])}else{v3})});let v1zw=(vx*vni);let v1zx=((vbt*v1zk)/v1zw);let v1zy=((vbt*v1zl)/v1zw);let v201=(vx*vnl);let v202=((vbt*v1zs)/v201);let v203=((vbt*v1zt)/v201);let v209=(vnn*vnn);let v20f=(if (vnr!=0.0){v3}else{(((vnn*(vx*v1zs))-(vnm*v202))/v209)});let v20g=(if (vnr!=0.0){v3}else{(((vnn*(vx*v1zt))-(vnm*v203))/v209)});let v20x=(sf[380]*((v1zx-v202)-((((vnn*v1zx)-(vnu*v202))/v209)/vnv)));
        let v20y=(sf[380]*((-v203)-(((-(vnu*v203))/v209)/vnv)));let v20z=(sf[380]*(v1zy-((v1zy/vnn)/vnv)));let v211=(sf[321]+v20z);let v212=(v20x/sf[564]);let v213=((sf[0]+v20y)/sf[564]);let v214=(v211/sf[564]);let v21e=(sf[564]*(vbh*v212));let v21f=(sf[564]*(vbh*v213));let v21g=(sf[564]*(vbh*v214));let v21s=(if (vo2!=0.0){((sf[795]*((sf[381]*v21e)/voj))-(if vo9{(sf[0]/vob)}else{(if vo6{sf[0]}else{v3})}))}else{v3});
        let v21t=(if (vo2!=0.0){((sf[795]*((sf[381]*v21f)/voj))-(if vo9{(sf[321]/vob)}else{(if vo6{sf[321]}else{v3})}))}else{v3});let v21u=(if (vo2!=0.0){(sf[795]*((sf[381]*v21g)/voj))}else{v3});let v21v=(voo*v21s);let v21x=(voo*v21t);let v21z=(voo*v21u);let v224=(vx*vp1);let v225=((if (vo2!=0.0){(v21v+v21v)}else{v3})/v224);let v226=((if (vo2!=0.0){(v21x+v21x)}else{v3})/v224);let v227=((if (vo2!=0.0){(v21z+v21z)}else{v3})/v224);let v22d=(vp2*vp2);
        let v22u=(if vp6{(vbh*(v21s+v225))}else{(if voy{((-(voz*(v225-v21s)))/v22d)}else{v3})});let v22v=(if vp6{(vbh*(v21t+v226))}else{(if voy{((-(voz*(v226-v21t)))/v22d)}else{v3})});let v22w=(if vp6{(vbh*(v21u+v227))}else{(if voy{((-(voz*(v227-v21u)))/v22d)}else{v3})});let v23c=(vph*vph);let v23m=(if (vo2!=0.0){(((vph*((vpd*v22u)+(vp9*v22u)))-(vpe*(sf[202]*v22u)))/v23c)}else{v3});let v23n=(if (vo2!=0.0){(((vph*((vpd*v22v)+(vp9*v22v)))-(vpe*(sf[202]*v22v)))/v23c)}else{v3});
        let v23o=(if (vo2!=0.0){(((vph*((vpd*v22w)+(vp9*v22w)))-(vpe*(sf[202]*v22w)))/v23c)}else{v3});let v23s=(vpj*vpj);let v242=(if (vo2!=0.0){(((vpj*v212)-(vo0*v23m))/v23s)}else{v3});let v243=(if (vo2!=0.0){(((vpj*v213)-(vo0*v23n))/v23s)}else{v3});let v244=(if (vo2!=0.0){(((vpj*v214)-(vo0*v23o))/v23s)}else{v3});let v248=(if (vo2!=0.0){(v242/sf[204])}else{v3});let v249=(if (vo2!=0.0){(v243/sf[204])}else{v3});let v24a=(if (vo2!=0.0){(v244/sf[204])}else{v3});
        let v258=(if (vo2!=0.0){((if vq0{(v242+(sf[204]*((vq2*(-v248))/vq3)))}else{(if vps{(sf[204]*((vpt*v248)/vpu))}else{v3})})/sf[210])}else{v3});let v259=(if (vo2!=0.0){((if vq0{(v243+(sf[204]*((vq2*(-v249))/vq3)))}else{(if vps{(sf[204]*((vpt*v249)/vpu))}else{v3})})/sf[210])}else{v3});let v25a=(if (vo2!=0.0){((if vq0{(v244+(sf[204]*((vq2*(-v24a))/vq3)))}else{(if vps{(sf[204]*((vpt*v24a)/vpu))}else{v3})})/sf[210])}else{v3});let v25e=(if (vo2!=0.0){(v22u/sf[203])}else{v3});
        let v25f=(if (vo2!=0.0){(v22v/sf[203])}else{v3});let v25g=(if (vo2!=0.0){(v22w/sf[203])}else{v3});let v262=(vx*vqo);let v26l=(vqr*vqr);let v26v=(if (vo2!=0.0){(((vqr*(((vql*((vqj*v25e)+(vqi*(vbt*v258))))+(vqk*v25e))/v262))-(vqp*((vqq*v25e)+(vql*(vx*v258)))))/v26l)}else{v3});let v26w=(if (vo2!=0.0){(((vqr*(((vql*((vqj*v25f)+(vqi*(vbt*v259))))+(vqk*v25f))/v262))-(vqp*((vqq*v25f)+(vql*(vx*v259)))))/v26l)}else{v3});
        let v26x=(if (vo2!=0.0){(((vqr*(((vql*((vqj*v25g)+(vqi*(vbt*v25a))))+(vqk*v25g))/v262))-(vqp*((vqq*v25g)+(vql*(vx*v25a)))))/v26l)}else{v3});let v273=((vqt*v20f)+(vns*v26v));let v276=((vqt*v20g)+(vns*v26w));let v277=(vns*v26x);let v27e=(vqx*vqx);let v27o=(if (vo2!=0.0){(((vqx*((-v26v)+v273))-(vqw*v273))/v27e)}else{v3});let v27p=(if (vo2!=0.0){(((vqx*((-v26w)+v276))-(vqw*v276))/v27e)}else{v3});let v27q=(if (vo2!=0.0){(((vqx*((-v26x)+v277))-(vqw*v277))/v27e)}else{v3});
        let v283=(if (vo2!=0.0){(sf[381]*((vqz*v21e)+(voh*v27o)))}else{v3});let v284=(if (vo2!=0.0){(sf[381]*((vqz*v21f)+(voh*v27p)))}else{v3});let v285=(if (vo2!=0.0){(sf[381]*((vqz*v21g)+(voh*v27q)))}else{v3});let v28l=(if (vo2!=0.0){((vx*v283)+((vr5*v20f)+(vns*(v20f+v283))))}else{v3});let v28m=(if (vo2!=0.0){((vx*v284)+((vr5*v20g)+(vns*(v20g+v284))))}else{v3});let v28n=(if (vo2!=0.0){((vx*v285)+(vns*v285))}else{v3});let v28r=(if (vo2!=0.0){(vbh*v283)}else{v3});
        let v28s=(if (vo2!=0.0){(vbh*v284)}else{v3});let v28t=(if (vo2!=0.0){(vbh*v285)}else{v3});let v28u=(vrb*v28r);let v28w=(vrb*v28s);let v28y=(vrb*v28t);let v293=(if (vo2!=0.0){(v28l+(v28u+v28u))}else{v3});let v294=(if (vo2!=0.0){(v28m+(v28w+v28w))}else{v3});let v295=(if (vo2!=0.0){(v28n+(v28y+v28y))}else{v3});let v296=(vx*vri);let v297=(v293/v296);let v298=(v294/v296);let v299=(v295/v296);let v29m=(vrn*vrn);
        let v29z=(if vrt{v3}else{(if vrm{(((vrn*v28l)-(vr8*(v297-v28r)))/v29m)}else{(if vrh{(v28r+v297)}else{v3})})});let v2a0=(if vrt{v3}else{(if vrm{(((vrn*v28m)-(vr8*(v298-v28s)))/v29m)}else{(if vrh{(v28s+v298)}else{v3})})});let v2a1=(if vrt{v3}else{(if vrm{(((vrn*v28n)-(vr8*(v299-v28t)))/v29m)}else{(if vrh{(v28t+v299)}else{v3})})});let v2ak=(if (vo2!=0.0){(sf[212]*v212)}else{v3});let v2al=(if (vo2!=0.0){(sf[212]*v213)}else{v3});let v2am=(if (vo2!=0.0){(sf[212]*v214)}else{v3});let v2at=(vs4*v2ak);
        let v2av=(vs4*v2al);let v2ax=(vs4*v2am);let v2b2=(vx*vsb);let v2bl=(vsn*vsn);let v2c1=(sf[201]*v212);let v2c2=(sf[201]*v213);let v2c3=(sf[201]*v214);let v2c7=(vst*vst);let v2cy=(vnu*vnu);let v2d6=(if vsy{(((vnu*(vx*v1zl))-(vsz*v1zy))/v2cy)}else{v2a1});let v2d7=(if vsy{(if vko{(vkq*sf[860])}else{(if (vkl!=0.0){(vkm*sf[860])}else{v3})})}else{(if (vo2!=0.0){(sf[799]*((vrv*v29z)+(vru*v29z)))}else{v3})});let v2d8=(if vsy{v3}else{(if (vo2!=0.0){(sf[799]*((vrv*v2a0)+(vru*v2a0)))}else{v3})});
        let v2d9=(if vsy{(if vko{(vkq*sf[861])}else{(if (vkl!=0.0){(vkm*sf[861])}else{v3})})}else{(if (vo2!=0.0){(sf[799]*((vrv*v2a1)+(vru*v2a1)))}else{v3})});let v2da=(v20f+(if vsy{(((vnu*(vx*v1zk))-(vsz*v1zx))/v2cy)}else{v29z}));let v2db=(v20g+(if vsy{v3}else{v2a0}));let v2df=(if vtf{(vbh*v2da)}else{v3});let v2dg=(if vtf{(vbh*v2db)}else{v3});let v2dh=(if vtf{(vbh*v2d6)}else{v3});let v2dl=(vtj*vtj);let v2e4=(vtp*vtp);
        let v2ee=(if vtn{(((vtp*v20x)-(vny*((sf[0]+v20x)-sf[0])))/v2e4)}else{(if vtf{(((vtj*v2df)-(vti*v2df))/v2dl)}else{v27o})});let v2ef=(if vtn{(((vtp*v20y)-(vny*(v20y-sf[321])))/v2e4)}else{(if vtf{(((vtj*v2dg)-(vti*v2dg))/v2dl)}else{v27p})});let v2eg=(if vtn{(((vtp*v20z)-(vny*v211))/v2e4)}else{(if vtf{(((vtj*v2dh)-(vti*v2dh))/v2dl)}else{v27q})});let v2ek=(if vsy{v3}else{(if vsl{(sf[507]*(((vsn*(vx*v212))-(vsm*(v212+v23m)))/v2bl))}else{v3})});
        let v2el=(if vsy{v3}else{(if vsl{(sf[507]*(((vsn*(vx*v213))-(vsm*(v213+v23n)))/v2bl))}else{v3})});let v2em=(if vsy{v3}else{(if vsl{(sf[507]*(((vsn*(vx*v214))-(vsm*(v214+v23o)))/v2bl))}else{v3})});let v2en=(if vsy{v212}else{(if (vo2!=0.0){(((vst*v2c1)-(vss*v212))/v2c7)}else{v3})});let v2eo=(if vsy{v213}else{(if (vo2!=0.0){(((vst*v2c2)-(vss*v213))/v2c7)}else{v3})});let v2ep=(if vsy{v214}else{(if (vo2!=0.0){(((vst*v2c3)-(vss*v214))/v2c7)}else{v3})});
        let v2ew=(if vsy{(-(v2en/sf[201]))}else{(if (vo2!=0.0){((-v2c1)/v2c7)}else{v3})});let v2ex=(if vsy{(-(v2eo/sf[201]))}else{(if (vo2!=0.0){((-v2c2)/v2c7)}else{v3})});let v2ey=(if vsy{(-(v2ep/sf[201]))}else{(if (vo2!=0.0){((-v2c3)/v2c7)}else{v3})});let v2fl=(if vud{(-(sf[806]*((vuf*sf[869])/vug)))}else{(if (vu6!=0.0){(sf[321]-(sf[806]*((vu7*sf[867])/vu8)))}else{v3})});let v2fm=(if vud{(-(sf[806]*((vuf*sf[870])/vug)))}else{(if (vu6!=0.0){(sf[0]-(sf[806]*((vu7*sf[868])/vu8)))}else{v3})});
        let v2fp=(-(sf[528]*v2fl));let v2fq=(-(sf[528]*v2fm));let v2ft=(sf[218]*f64::powf(vum,sf[325]));let v2fu=(v2fp*v2ft);let v2fv=(v2fq*v2ft);let v2g4=((sf[807]*(-v2fu))+(v4g*(sf[321]-v2fl)));let v2g5=((sf[807]*(-v2fv))+(v4g*(sf[0]-v2fm)));let v2gd=(if sb[26]{sf[0]}else{(if sb[24]{(sf[0]+(if vsy{v3}else{(if (vo2!=0.0){(v2ak+(((if (vo2!=0.0){(sf[801]*v212)}else{v3})+(v2at+v2at))/v2b2))}else{v3})}))}else{sf[326]})});
        let v2ge=(if sb[26]{v3}else{(if sb[24]{(sf[321]+(if vsy{sf[0]}else{(if (vo2!=0.0){(v2al+(((if (vo2!=0.0){(sf[801]*v213)}else{v3})+(v2av+v2av))/v2b2))}else{v3})}))}else{sf[327]})});let v2gf=(if sb[26]{sf[321]}else{(if sb[24]{(if vsy{sf[321]}else{(if (vo2!=0.0){(v2am+(((if (vo2!=0.0){(sf[801]*v214)}else{v3})+(v2ax+v2ax))/v2b2))}else{v3})})}else{v3})});let v2gj=(vtt*vtt);let v2gk=(((vtt*v2gd)-(vvf*v2ek))/v2gj);let v2go=(((vtt*v2ge)-(vvf*v2el))/v2gj);let v2gs=(((vtt*v2gf)-(vvf*v2em))/v2gj);
        let v2hz=(if vvp{(-((vvt*v2ek)+(vtt*((vvr*(-v2gk))/vvs))))}else{(if (vvi!=0.0){(v2gd-((vvl*v2ek)+(vtt*((vvj*v2gk)/vvk))))}else{v3})});let v2i0=(if vvp{(-((vvt*v2el)+(vtt*((vvr*(-v2go))/vvs))))}else{(if (vvi!=0.0){(v2ge-((vvl*v2el)+(vtt*((vvj*v2go)/vvk))))}else{v3})});let v2i1=(if vvp{(-((vvt*v2em)+(vtt*((vvr*(-v2gs))/vvs))))}else{(if (vvi!=0.0){(v2gf-((vvl*v2em)+(vtt*((vvj*v2gs)/vvk))))}else{v3})});let v2i4=(sf[223]*f64::powf(vtx,sf[328]));let v2i5=(v2ew*v2i4);let v2i6=(v2ex*v2i4);
        let v2i7=(v2ey*v2i4);let v2ig=(sf[224]*f64::powf(vw2,sf[329]));let v2jj=(sf[809]*((sf[814]*(-((vw3*v2i7)+(vvy*((-(v2i1/sf[507]))*v2ig)))))+((vw8*(sf[810]*v2i7))+(vw7*(v2gf-v2i1)))));let v2jm=((sf[809]*((sf[814]*(-((vw3*v2i5)+(vvy*((-(v2hz/sf[507]))*v2ig)))))+((vw8*(sf[810]*v2i5))+(vw7*(v2gd-v2hz)))))+sf[871]);let v2jn=((sf[809]*((sf[814]*(-((vw3*v2i6)+(vvy*((-(v2i0/sf[507]))*v2ig)))))+((vw8*(sf[810]*v2i6))+(vw7*(v2ge-v2i0)))))+sf[872]);let v2jo=(sf[816]*v1x9);let v2jp=(sf[816]*v1xa);
        let v2jq=(vx*vwi);let v2jr=(v2jo/v2jq);let v2js=(v2jp/v2jq);let v2jw=(vwj*vwj);let v2jx=(((vwj*v2jo)-(vwg*v2jr))/v2jw);let v2k1=(((vwj*v2jp)-(vwg*v2js))/v2jw);let v2k4=(sf[817]*f64::powf(vt2,sf[873]));let v2k5=(v2d7*v2k4);let v2k6=(v2d8*v2k4);let v2k7=(v2d9*v2k4);let v2k8=(sf[816]*v2k5);let v2k9=(sf[816]*v2k6);let v2ka=(sf[816]*v2k7);let v2kb=(vx*vwp);let v2ki=(vwq*vwq);let v2kj=(((vwq*v2k8)-(vwn*(v2k8/v2kb)))/v2ki);let v2kn=(((vwq*v2k9)-(vwn*(v2k9/v2kb)))/v2ki);
        let v2kr=(((vwq*v2ka)-(vwn*(v2ka/v2kb)))/v2ki);let v2ks=(v2g4/sf[750]);let v2kt=(v2g5/sf[750]);let v2ku=(v2jm/sf[748]);let v2kv=(v2jn/sf[748]);let v2kw=(v2jj/sf[748]);let v2kx=(v2kt+v2ku);let v2lz=(if sb[28]{((vx8*(if sb[28]{(sf[381]*(sf[779]*v2ks))}else{v3}))/sf[820])}else{(if (sf[225]!=0.0){v2ks}else{v3})});let v2m0=(if sb[28]{(((vx8*(if sb[28]{(sf[381]*(sf[779]*v2kt))}else{v3}))-(vx9*(if sb[28]{(sf[381]*(sf[779]*((-v2jm)/sf[748])))}else{v3})))/sf[820])}else{(if (sf[225]!=0.0){v2kx}else{v3})});
        let v2m1=(if sb[28]{((-(vx9*(if sb[28]{(sf[381]*(sf[779]*((-v2jn)/sf[748])))}else{v3})))/sf[820])}else{(if (sf[225]!=0.0){v2kv}else{v3})});let v2m2=(if sb[28]{((-(vx9*(if sb[28]{(sf[381]*(sf[779]*((-v2jj)/sf[748])))}else{v3})))/sf[820])}else{(if (sf[225]!=0.0){v2kw}else{v3})});let v2m3=(vxf*v2lz);let v2m4=(v2m3+v2m3);let v2m5=(vxf*v2m0);let v2m6=(v2m5+v2m5);let v2m7=(vxf*v2m1);let v2m8=(v2m7+v2m7);let v2m9=(vxf*v2m2);let v2ma=(v2m9+v2m9);let v2mb=(vx*vxm);let v2mc=(v2m4/v2mb);let v2md=(v2m6/v2mb);
        let v2me=(v2m8/v2mb);let v2mf=(v2ma/v2mb);let v2mm=(vxn*vxn);let v2ne=(vbh*v2jx);let v2nf=(vbh*(v2k1+v2kj));let v2ng=(vbh*v2kn);let v2nh=(vbh*v2kr);let v2nk=((vxw*(if vxq{(vbh*(v2lz+v2mc))}else{(if (vxj!=0.0){((-(vxk*(v2mc-v2lz)))/v2mm)}else{v3})}))+(vxt*v2ne));let v2nn=((vxw*(if vxq{(vbh*(v2m0+v2md))}else{(if (vxj!=0.0){((-(vxk*(v2md-v2m0)))/v2mm)}else{v3})}))+(vxt*v2nf));let v2nq=((vxw*(if vxq{(vbh*(v2m1+v2me))}else{(if (vxj!=0.0){((-(vxk*(v2me-v2m1)))/v2mm)}else{v3})}))+(vxt*v2ng));
        let v2nt=((vxw*(if vxq{(vbh*(v2m2+v2mf))}else{(if (vxj!=0.0){((-(vxk*(v2mf-v2m2)))/v2mm)}else{v3})}))+(vxt*v2nh));let v2nu=(sf[821]*v2k5);let v2nv=(sf[821]*v2k6);let v2nw=(sf[821]*v2k7);let v2ny=(sf[633]*v1xa);let v2o2=(vxx*(sf[633]*v1x9));let v2o5=(vxx*vxx);let v2p3=(if vyd{(sf[321]+(vy4*((vyf*sf[332])/vyg)))}else{(if (vy7!=0.0){(vy4*((vy8*sf[330])/vy9))}else{v3})});let v2p4=(if vyd{(sf[0]+(vy4*((vyf*sf[333])/vyg)))}else{(if (vy7!=0.0){(vy4*((vy8*sf[331])/vy9))}else{v3})});
        let v2qr=(if vzs{(vzt*sf[874])}else{(if (vzp!=0.0){(vzq*sf[874])}else{v2p3})});let v2qs=(if vzs{(vzt*sf[875])}else{(if (vzp!=0.0){(vzq*sf[875])}else{v2p4})});let v2uh=(if v11z{(v120*sf[876])}else{(if (v11w!=0.0){(v11x*sf[876])}else{v2qr})});let v2ui=(if v11z{(v120*sf[877])}else{(if (v11w!=0.0){(v11x*sf[877])}else{v3})});let v2uj=(if v11z{v3}else{(if (v11w!=0.0){v3}else{v2qs})});let v2w2=(if v130{(v131*sf[878])}else{(if (v12x!=0.0){(v12y*sf[878])}else{v2uh})});
        let v2w3=(if v130{v3}else{(if (v12x!=0.0){v3}else{v2ui})});let v2w4=(if v130{(v131*sf[879])}else{(if (v12x!=0.0){(v12y*sf[879])}else{v2uj})});let v2wh=(if v13d{(v13e*sf[880])}else{(if (v13a!=0.0){(v13b*sf[880])}else{v2w2})});let v2wi=(if v13d{(v13e*sf[881])}else{(if (v13a!=0.0){(v13b*sf[881])}else{v2w3})});let v2wj=(if v13d{v3}else{(if (v13a!=0.0){v3}else{v2w4})});let v2x4=(if v13q{v3}else{(if (v13n!=0.0){v3}else{v2wh})});
        let v2x5=(if v13q{(v13r*sf[882])}else{(if (v13n!=0.0){(v13o*sf[882])}else{v2wi})});let v2x6=(if v13q{(v13r*sf[883])}else{(if (v13n!=0.0){(v13o*sf[883])}else{v2wj})});let v2x7=(if v13q{(v13r*sf[884])}else{(if (v13n!=0.0){(v13o*sf[884])}else{v3})});let v2x8=(if v13q{(v13r*sf[885])}else{(if (v13n!=0.0){(v13o*sf[885])}else{v3})});let v2xp=(if v143{(v144*sf[886])}else{(if (v140!=0.0){(v141*sf[886])}else{v2x4})});let v2xq=(if v143{(v144*sf[887])}else{(if (v140!=0.0){(v141*sf[887])}else{v2x5})});
        let v2xr=(if v143{v3}else{(if (v140!=0.0){v3}else{v2x6})});let v2xs=(if v143{v3}else{(if (v140!=0.0){v3}else{v2x7})});let v2xt=(if v143{v3}else{(if (v140!=0.0){v3}else{v2x8})});let v377=(sf[816]*v1xp);let v378=(sf[816]*v1xq);let v379=(sf[816]*v1xr);let v37a=(sf[816]*v1xs);let v37b=(vbt*(if vmm{(vmn*sf[860])}else{(if (vmj!=0.0){(vmk*sf[860])}else{v3})}));let v37c=(vbt*(if vmm{(vmn*sf[864])}else{(if (vmj!=0.0){(vmk*sf[864])}else{v3})}));
        let v37d=(vbt*(if vmm{(vmn*sf[865])}else{(if (vmj!=0.0){(vmk*sf[865])}else{v3})}));let v37e=(vbt*(if vmm{(vmn*sf[861])}else{(if (vmj!=0.0){(vmk*sf[861])}else{v3})}));let v37f=(vx*v19n);let v37n=(v19o*v19o);let v381=(vx*v19r);let v389=(v19s*v19s);let v39x=(vx*v1ah);let v3a5=(v1ai*v1ai);let v3aj=(if (sf[242]!=0.0){(((v1ai*(sf[829]*v1ye))-(v1ae*((sf[828]*v1ye)/v39x)))/v3a5)}else{v3});let v3ak=(if (sf[242]!=0.0){(((v1ai*(sf[829]*v1yf))-(v1ae*((sf[828]*v1yf)/v39x)))/v3a5)}else{v3});
        let v3al=(if (sf[242]!=0.0){(((v1ai*(sf[829]*v1yg))-(v1ae*((sf[828]*v1yg)/v39x)))/v3a5)}else{v3});let v3am=(if (sf[242]!=0.0){(((v1ai*(sf[829]*v1yh))-(v1ae*((sf[828]*v1yh)/v39x)))/v3a5)}else{v3});let v3ar=(v1ax*sf[346]);let v3as=(v3ar+v3ar);let v3at=(v1ax*sf[347]);let v3av=(v1ax*sf[348]);let v3aw=(v3av+v3av);let v3ax=(v1ax*sf[349]);let v3az=(if sb[44]{v3as}else{v3});let v3b0=(if sb[44]{(v3at+v3at)}else{v3});let v3b1=(if sb[44]{v3}else{v2m4});let v3b2=(if sb[44]{v3as}else{v2m6});
        let v3b3=(if sb[44]{v3aw}else{v2m8});let v3b4=(if sb[44]{v3aw}else{v2ma});let v3b5=(if sb[44]{(v3ax+v3ax)}else{v3});let v3b6=(if sb[44]{v3aw}else{v3});let v3b7=(vx*v1b7);let v3b8=(v3az/v3b7);let v3b9=(v3b0/v3b7);let v3ba=(v3b1/v3b7);let v3bb=(v3b2/v3b7);let v3bc=(v3b3/v3b7);let v3bd=(v3b4/v3b7);let v3be=(v3b5/v3b7);let v3bf=(v3b6/v3b7);let v3bp=(v1b8*v1b8);let v3cz=(if v1bc{(vbh*(sf[346]+v3b8))}else{(if v1b4{((-(sf[246]*(v3b8-sf[346])))/v3bp)}else{v3})});
        let v3d0=(if v1bc{(vbh*(sf[347]+v3b9))}else{(if v1b4{((-(sf[246]*(v3b9-sf[347])))/v3bp)}else{v3})});let v3d1=(if v1bc{(vbh*v3ba)}else{(if v1b4{((-(sf[246]*v3ba))/v3bp)}else{v3})});let v3d2=(if v1bc{(vbh*(sf[346]+v3bb))}else{(if v1b4{((-(sf[246]*(v3bb-sf[346])))/v3bp)}else{v3})});let v3d3=(if v1bc{(vbh*(sf[348]+v3bc))}else{(if v1b4{((-(sf[246]*(v3bc-sf[348])))/v3bp)}else{v3})});let v3d4=(if v1bc{(vbh*(sf[348]+v3bd))}else{(if v1b4{((-(sf[246]*(v3bd-sf[348])))/v3bp)}else{v3})});
        let v3d5=(if v1bc{(vbh*(sf[349]+v3be))}else{(if v1b4{((-(sf[246]*(v3be-sf[349])))/v3bp)}else{v3})});let v3d6=(if v1bc{(vbh*(sf[348]+v3bf))}else{(if v1b4{((-(sf[246]*(v3bf-sf[348])))/v3bp)}else{v3})});let v3d7=(sf[557]*v3aj);let v3d9=(sf[557]*v3al);let v3dl=(v1bi*v1bi);let v3en=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3cz)-(v1bf*(v3cz+v3d7)))/v3dl)}else{v3})});let v3eo=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d0)-(v1bf*(v3d0+(sf[557]*v3ak))))/v3dl)}else{v3})});
        let v3ep=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d1)-(v1bf*v3d1))/v3dl)}else{v3})});let v3eq=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d2)-(v1bf*(v3d2+v3d7)))/v3dl)}else{v3})});let v3er=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d3)-(v1bf*(v3d3+v3d9)))/v3dl)}else{v3})});let v3es=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d4)-(v1bf*(v3d4+v3d9)))/v3dl)}else{v3})});let v3et=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d5)-(v1bf*(v3d5+(sf[557]*v3am))))/v3dl)}else{v3})});
        let v3eu=(if sb[46]{v3}else{(if sb[44]{(((v1bi*v3d6)-(v1bf*(v3d6+v3d9)))/v3dl)}else{v3})});let v3m5=(vwx*v2ks);let v3m7=(vwx*v2kx);let v3m9=(vwx*v2kv);let v3mb=(vwx*v2kw);let v3md=(vx*v1dg);let v3me=((v3m5+v3m5)/v3md);let v3mf=((v3m7+v3m7)/v3md);let v3mg=((v3m9+v3m9)/v3md);let v3mh=((v3mb+v3mb)/v3md);let v3mo=(v1dh*v1dh);let v3nb=(if v1dk{(vbh*(v2ks+v3me))}else{(if (v1de!=0.0){((-(vxk*(v3me-v2ks)))/v3mo)}else{v3})});
        let v3nc=(if v1dk{(vbh*(v2kx+v3mf))}else{(if (v1de!=0.0){((-(vxk*(v3mf-v2kx)))/v3mo)}else{v3})});let v3nd=(if v1dk{(vbh*(v2kv+v3mg))}else{(if (v1de!=0.0){((-(vxk*(v3mg-v2kv)))/v3mo)}else{v3})});let v3ne=(if v1dk{(vbh*(v2kw+v3mh))}else{(if (v1de!=0.0){((-(vxk*(v3mh-v2kw)))/v3mo)}else{v3})});let v4bs=(if v1lt{(-(sf[806]*((v1lv*sf[869])/v1lw)))}else{(if (v1lm!=0.0){(sf[321]-(sf[806]*((v1ln*sf[867])/v1lo)))}else{v3})});
        let v4bt=(if v1lt{(-(sf[806]*((v1lv*sf[870])/v1lw)))}else{(if (v1lm!=0.0){(sf[0]-(sf[806]*((v1ln*sf[868])/v1lo)))}else{v3})});let v4bz=(sf[218]*f64::powf(v1m3,sf[325]));let v4cl=((v1mg*v3nb)+(v1dn*(sf[846]*v2jx)));let v4co=((v1mg*v3nc)+(v1dn*(sf[846]*v2k1)));let v4cp=(v1mg*v3nd);let v4cq=(v1mg*v3ne);let v4cu=(v1mi*v3nb);let v4cx=((v1mi*v3nc)+(v1dn*(sf[846]*v2kj)));let v4d0=((v1mi*v3nd)+(v1dn*(sf[846]*v2kn)));let v4d3=((v1mi*v3ne)+(v1dn*(sf[846]*v2kr)));
        let v4ec=(if v1mu{(-(sf[802]*((v1mw*sf[904])/v1mx)))}else{(if (v1mn!=0.0){(sf[0]-(sf[802]*((v1mo*sf[900])/v1mp)))}else{v3})});let v4ed=(if v1mu{(-(sf[802]*((v1mw*sf[905])/v1mx)))}else{(if (v1mn!=0.0){(sf[322]-(sf[802]*((v1mo*sf[901])/v1mp)))}else{v3})});let v4ee=(if v1mu{(-(sf[802]*((v1mw*sf[906])/v1mx)))}else{(if (v1mn!=0.0){(sf[323]-(sf[802]*((v1mo*sf[902])/v1mp)))}else{v3})});
        let v4ef=(if v1mu{(-(sf[802]*((v1mw*sf[907])/v1mx)))}else{(if (v1mn!=0.0){(sf[321]-(sf[802]*((v1mo*sf[903])/v1mp)))}else{v3})});let v4ep=(sf[224]*f64::powf(v1n3,sf[329]));let v4h2=(if v1nr{(-(sf[802]*((v1nt*sf[905])/v1nu)))}else{(if (v1nk!=0.0){(sf[322]-(sf[802]*((v1nl*sf[901])/v1nm)))}else{v3})});let v4h3=(if v1nr{(-(sf[802]*((v1nt*sf[911])/v1nu)))}else{(if (v1nk!=0.0){(sf[324]-(sf[802]*((v1nl*sf[910])/v1nm)))}else{v3})});
        let v4h4=(if v1nr{(-(sf[802]*((v1nt*sf[906])/v1nu)))}else{(if (v1nk!=0.0){(sf[323]-(sf[802]*((v1nl*sf[902])/v1nm)))}else{v3})});let v4h5=(if v1nr{(-(sf[802]*((v1nt*sf[907])/v1nu)))}else{(if (v1nk!=0.0){(sf[321]-(sf[802]*((v1nl*sf[903])/v1nm)))}else{v3})});let v4hf=(sf[224]*f64::powf(v1o0,sf[329]));let v4il=(sf[6]*(sf[296]*(sf[540]*(sf[908]+(sf[809]*((sf[814]*(-((-(v4h2/sf[507]))*v4hf)))+(sf[810]*(sf[322]-v4h2))))))));
        let v4in=(sf[6]*(sf[296]*(sf[540]*(sf[909]+(sf[809]*((sf[814]*(-((-(v4h4/sf[507]))*v4hf)))+(sf[810]*(sf[323]-v4h4))))))));let v4j5=(sf[850]*(if v1op{(v1oq*sf[913])}else{(if (v1om!=0.0){(v1on*sf[913])}else{v2xp})}));let v4j6=(sf[850]*(if v1op{v3}else{(if (v1om!=0.0){v3}else{v2xq})}));let v4j7=(sf[850]*(if v1op{(v1oq*sf[914])}else{(if (v1om!=0.0){(v1on*sf[914])}else{v2xr})}));let v4j8=(sf[850]*(if v1op{v3}else{(if (v1om!=0.0){v3}else{v2xs})}));
        let v4j9=(sf[850]*(if v1op{v3}else{(if (v1om!=0.0){v3}else{v2xt})}));let v4l6=(vx*v1py);let v4le=(v1pz*v1pz);let v4ls=(if sb[60]{(((v1pz*(sf[857]*v1xp))-(v1pv*((vbt*(if v1po{(v1pp*sf[915])}else{(if v1pk{(v1pl*sf[915])}else{v3})}))/v4l6)))/v4le)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((v19o*v377)-(v19l*(v377/v37f)))/v37n))+(sf[854]*(((v19s*v37b)-(v19k*(v37b/v381)))/v389))))/sf[763])}else{v3})});
        let v4lt=(if sb[60]{(((v1pz*(sf[857]*v1xq))-(v1pv*((vbt*(if v1po{(v1pp*sf[916])}else{(if v1pk{(v1pl*sf[916])}else{v3})}))/v4l6)))/v4le)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((v19o*v378)-(v19l*(v378/v37f)))/v37n))+(sf[854]*(((v19s*v37c)-(v19k*(v37c/v381)))/v389))))/sf[763])}else{v3})});
        let v4lu=(if sb[60]{(((v1pz*(sf[857]*v1xr))-(v1pv*((vbt*(if v1po{(v1pp*sf[917])}else{(if v1pk{(v1pl*sf[917])}else{v3})}))/v4l6)))/v4le)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((v19o*v379)-(v19l*(v379/v37f)))/v37n))+(sf[854]*(((v19s*v37d)-(v19k*(v37d/v381)))/v389))))/sf[763])}else{v3})});
        let v4lv=(if sb[60]{(((v1pz*(sf[857]*v1xs))-(v1pv*((vbt*(if v1po{(v1pp*sf[918])}else{(if v1pk{(v1pl*sf[918])}else{v3})}))/v4l6)))/v4le)}else{(if (sf[300]!=0.0){((sf[856]*((sf[845]*(((v19o*v37a)-(v19l*(v37a/v37f)))/v37n))+(sf[854]*(((v19s*v37e)-(v19k*(v37e/v381)))/v389))))/sf[763])}else{v3})});let v4m8=(if sb[64]{(sf[816]*v1ye)}else{v3});let v4m9=(if sb[64]{(sf[816]*v1yf)}else{v3});let v4ma=(if sb[64]{(sf[816]*v1yg)}else{v3});let v4mb=(if sb[64]{(sf[816]*v1yh)}else{v3});let v4mc=(vx*v1qd);
        let v4mk=(v1qe*v1qe);let v4n6=(if sb[64]{(vbt*(if vma{(vmb*sf[864])}else{(if (vm7!=0.0){(vm8*sf[864])}else{v3})}))}else{v3});let v4n7=(if sb[64]{(vbt*(if vma{(vmb*sf[866])}else{(if (vm7!=0.0){(vm8*sf[866])}else{v3})}))}else{v3});let v4n8=(if sb[64]{(vbt*(if vma{(vmb*sf[865])}else{(if (vm7!=0.0){(vm8*sf[865])}else{v3})}))}else{v3});let v4n9=(if sb[64]{(vbt*(if vma{(vmb*sf[861])}else{(if (vm7!=0.0){(vm8*sf[861])}else{v3})}))}else{v3});let v4na=(vx*v1qk);let v4ni=(v1ql*v1ql);let v4pc=(vx*v1rf);
        let v4pk=(v1rg*v1rg);let v4q3=(v1bn*(if sb[65]{(((v1rg*(sf[859]*v1ye))-(v1rc*((vbt*(if v1r5{(v1r6*sf[864])}else{(if v1r1{(v1r2*sf[864])}else{v3})}))/v4pc)))/v4pk)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((v1qe*v4m8)-(v1qb*(v4m8/v4mc)))/v4mk)}else{v3}))+(sf[854]*(if sb[64]{(((v1ql*v4n6)-(v1qi*(v4n6/v4na)))/v4ni)}else{v3}))))/sf[763])}else{v3})}));
        let v4qc=(v1bn*(if sb[65]{(((v1rg*(sf[859]*v1yg))-(v1rc*((vbt*(if v1r5{(v1r6*sf[865])}else{(if v1r1{(v1r2*sf[865])}else{v3})}))/v4pc)))/v4pk)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((v1qe*v4ma)-(v1qb*(v4ma/v4mc)))/v4mk)}else{v3}))+(sf[854]*(if sb[64]{(((v1ql*v4n8)-(v1qi*(v4n8/v4na)))/v4ni)}else{v3}))))/sf[763])}else{v3})}));let v4qv=(sf[306]*f64::powf(vum,sf[363]));let v4r5=(v1rx*v1rx);let v4rd=(v1s3*sf[921]);let v4re=(v1s3*sf[922]);let v4ri=(v1s4*v1s4);let v4s8=(vwi*vwi);
        let v4t9=(if (sf[305]!=0.0){(v4j8/sf[851])}else{v3});let v4uc=(sf[307]*v4j8);let v4ui=(if (sf[305]!=0.0){(v4cl+(sf[307]*v4j5))}else{v3});let v4uj=(if (sf[305]!=0.0){(sf[307]*v4j6)}else{v3});let v4uk=(if (sf[305]!=0.0){(v4co+(sf[307]*v4j7))}else{v3});let v4ul=(if (sf[305]!=0.0){(v4cp+v4uc)}else{v3});let v4um=(if (sf[305]!=0.0){(v4cq+v4uc)}else{v3});let v4un=(if (sf[305]!=0.0){(sf[307]*v4j9)}else{v3});let v4vg=(if sb[67]{v4cl}else{(if (sf[305]!=0.0){(sf[310]*v4ui)}else{v3})});
        let v4vh=(if sb[67]{v3}else{(if (sf[305]!=0.0){(sf[310]*v4uj)}else{v3})});let v4vi=(if sb[67]{v4co}else{(if (sf[305]!=0.0){(sf[310]*v4uk)}else{v3})});let v4vj=(if sb[67]{v4cp}else{(if (sf[305]!=0.0){(sf[310]*v4ul)}else{v3})});let v4vk=(if sb[67]{v4cq}else{(if (sf[305]!=0.0){(sf[310]*v4um)}else{v3})});let v4vl=(if sb[67]{v3}else{(if (sf[305]!=0.0){(sf[310]*v4un)}else{v3})});let v4vm=(if sb[67]{v4cu}else{(if (sf[305]!=0.0){(v4cu+(sf[309]*v4ui))}else{v3})});
        let v4vn=(if sb[67]{v3}else{(if (sf[305]!=0.0){(sf[309]*v4uj)}else{v3})});let v4vo=(if sb[67]{v4cx}else{(if (sf[305]!=0.0){(v4cx+(sf[309]*v4uk))}else{v3})});let v4vp=(if sb[67]{v4d0}else{(if (sf[305]!=0.0){(v4d0+(sf[309]*v4ul))}else{v3})});let v4vq=(if sb[67]{v4d3}else{(if (sf[305]!=0.0){(v4d3+(sf[309]*v4um))}else{v3})});let v4vr=(if sb[67]{v3}else{(if (sf[305]!=0.0){(sf[309]*v4un)}else{v3})});let v4vv=(if sb[67]{v4j8}else{(if (sf[305]!=0.0){(sf[308]*v4j8)}else{v3})});let v4wd=(v1ta*v1ta);
        let v4xo=(if v1to{((v1tp*v2nk)+(vxx*(sf[759]*v3nb)))}else{(if (v1tk!=0.0){(((v1ta*(v4vg+v4vm))-(v1tl*((v2o2-(v1t9*v2nk))/v2o5)))/v4wd)}else{v3})});let v4xp=(if v1to{v3}else{(if (v1tk!=0.0){((v4vh+v4vn)/v1ta)}else{v3})});let v4xq=(if v1to{((v1tp*v2nn)+(vxx*(sf[759]*v3nc)))}else{(if (v1tk!=0.0){(((v1ta*(v4vi+v4vo))-(v1tl*(((vxx*(v2nu+v2ny))-(v1t9*v2nn))/v2o5)))/v4wd)}else{v3})});
        let v4xr=(if v1to{((v1tp*v2nq)+(vxx*(sf[759]*v3nd)))}else{(if (v1tk!=0.0){(((v1ta*(v4vj+v4vp))-(v1tl*(((vxx*v2nv)-(v1t9*v2nq))/v2o5)))/v4wd)}else{v3})});let v4xs=(if v1to{((v1tp*v2nt)+(vxx*(sf[759]*v3ne)))}else{(if (v1tk!=0.0){(((v1ta*(v4vk+v4vq))-(v1tl*(((vxx*v2nw)-(v1t9*v2nt))/v2o5)))/v4wd)}else{v3})});let v4xt=(if v1to{v3}else{(if (v1tk!=0.0){((v4vl+v4vr)/v1ta)}else{v3})});let v4yi=(if sb[75]{v3}else{(if sb[73]{(sf[316]*v4xo)}else{(if (sf[314]!=0.0){(sf[309]*v4xo)}else{v3})})});
        let v4yj=(if sb[75]{v3}else{(if sb[73]{(sf[316]*v4xp)}else{(if (sf[314]!=0.0){(sf[309]*v4xp)}else{v3})})});let v4yk=(if sb[75]{v3}else{(if sb[73]{(sf[316]*v4xq)}else{(if (sf[314]!=0.0){(sf[309]*v4xq)}else{v3})})});let v4yl=(if sb[75]{v3}else{(if sb[73]{(sf[316]*v4xr)}else{(if (sf[314]!=0.0){(sf[309]*v4xr)}else{v3})})});let v4ym=(if sb[75]{v3}else{(if sb[73]{(sf[316]*v4xs)}else{(if (sf[314]!=0.0){(sf[309]*v4xs)}else{v3})})});
        let v4yn=(if sb[75]{v3}else{(if sb[73]{(sf[316]*v4xt)}else{(if (sf[314]!=0.0){(sf[309]*v4xt)}else{v3})})});let v539=(sf[0]*((if sb[67]{v4j5}else{(if (sf[305]!=0.0){(sf[308]*v4j5)}else{v3})})+((sf[842]*v2g4)+v4vg)));let v53a=(sf[0]*(v4vh+(if sb[67]{v4j6}else{(if (sf[305]!=0.0){(sf[308]*v4j6)}else{v3})})));let v53b=(sf[0]*((if sb[67]{v4j7}else{(if (sf[305]!=0.0){(sf[308]*v4j7)}else{v3})})+((sf[842]*v2g5)+v4vi)));let v53c=(sf[0]*(v4vj+v4vv));let v53d=(sf[0]*(v4vk+v4vv));
        let v53e=(sf[0]*(v4vl+(if sb[67]{v4j9}else{(if (sf[305]!=0.0){(sf[308]*v4j9)}else{v3})})));let v53s=(sf[0]*(sf[843]*((sf[807]*(-((-(sf[528]*v4bs))*v4bz)))+(v4g*(sf[321]-v4bs)))));let v53t=(sf[0]*(sf[843]*((sf[807]*(-((-(sf[528]*v4bt))*v4bz)))+(v4g*(sf[0]-v4bt)))));let v53y=(sf[0]*v4vm);let v53z=(sf[0]*v4vn);let v540=(sf[0]*(((v1p1*(sf[855]*v2ee))+(v1p0*v2da))+((sf[844]*v2jm)+v4vo)));let v541=(sf[0]*(((v1p1*(sf[855]*v2ef))+(v1p0*v2db))+((sf[844]*v2jn)+v4vp)));
        let v542=(sf[0]*(((v1p1*(sf[855]*v2eg))+(v1p0*v2d6))+((sf[844]*v2jj)+v4vq)));let v543=(sf[0]*v4vr);
        let v54g=(sf[0]*(if (sf[305]!=0.0){(v1sm*((if (sf[305]!=0.0){(v4j5/sf[851])}else{v3})+((if (sf[305]!=0.0){(sf[842]*(if (sf[305]!=0.0){((v1s6*(if (sf[305]!=0.0){(v2fp*v4qv)}else{v3}))+(v1rr*(if v1s1{(((v1s4*v4rd)-(v1s3*v4rd))/v4ri)}else{(if v1rv{((-(v1rw*sf[919]))/v4r5)}else{v3})})))}else{v3}))}else{v3})+(if (sf[305]!=0.0){((v1sh*(if (sf[305]!=0.0){((v1se*((sf[381]*v2jo)/sf[588]))+(v1sd*((-(vbh*v2jr))/v4s8)))}else{v3}))+(v1sg*(sf[846]*v3nb)))}else{v3}))))}else{v3}));
        let v54h=(sf[0]*(if (sf[305]!=0.0){((v1so*sf[364])+(v1sm*(if (sf[305]!=0.0){(v4j6/sf[851])}else{v3})))}else{v3}));
        let v54i=(sf[0]*(if (sf[305]!=0.0){((v1so*sf[365])+(v1sm*((if (sf[305]!=0.0){(v4j7/sf[851])}else{v3})+((if (sf[305]!=0.0){(sf[842]*(if (sf[305]!=0.0){((v1s6*(if (sf[305]!=0.0){(v2fq*v4qv)}else{v3}))+(v1rr*(if v1s1{(((v1s4*v4re)-(v1s3*v4re))/v4ri)}else{(if v1rv{((-(v1rw*sf[920]))/v4r5)}else{v3})})))}else{v3}))}else{v3})+(if (sf[305]!=0.0){((v1sh*(if (sf[305]!=0.0){((v1se*((sf[381]*v2jp)/sf[588]))+(v1sd*((-(vbh*v2js))/v4s8)))}else{v3}))+(v1sg*(sf[846]*v3nc)))}else{v3})))))}else{v3}));
        let v54j=(sf[0]*(if (sf[305]!=0.0){(v1sm*((if (sf[305]!=0.0){(v1sg*(sf[846]*v3nd))}else{v3})+v4t9))}else{v3}));let v54k=(sf[0]*(if (sf[305]!=0.0){(v1sm*((if (sf[305]!=0.0){(v1sg*(sf[846]*v3ne))}else{v3})+v4t9))}else{v3}));let v54l=(sf[0]*(if (sf[305]!=0.0){(v1sm*(if (sf[305]!=0.0){(v4j9/sf[851])}else{v3}))}else{v3}));let v562=(sf[0]*(v4il+(if (sf[302]!=0.0){((v1ri*v3en)+v4q3)}else{v3})));
        let v563=(sf[0]*((sf[6]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(-((-(v4h3/sf[507]))*v4hf)))+(sf[810]*(sf[324]-v4h3))))+sf[912]))))+(if (sf[302]!=0.0){((v1ri*v3eo)+(v1bn*(if sb[65]{(((v1rg*(sf[859]*v1yf))-(v1rc*((vbt*(if v1r5{(v1r6*sf[866])}else{(if v1r1{(v1r2*sf[866])}else{v3})}))/v4pc)))/v4pk)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((v1qe*v4m9)-(v1qb*(v4m9/v4mc)))/v4mk)}else{v3}))+(sf[854]*(if sb[64]{(((v1ql*v4n7)-(v1qi*(v4n7/v4na)))/v4ni)}else{v3}))))/sf[763])}else{v3})})))}else{v3})));
        let v564=(sf[0]*(if (sf[302]!=0.0){(v1ri*v3ep)}else{v3}));let v565=(sf[0]*(v4il+(if (sf[302]!=0.0){(v4q3+(v1ri*v3eq))}else{v3})));let v566=(sf[0]*(v4in+(if (sf[302]!=0.0){((v1ri*v3er)+v4qc)}else{v3})));let v567=(sf[0]*(v4in+(if (sf[302]!=0.0){(v4qc+(v1ri*v3es))}else{v3})));
        let v568=(sf[0]*((sf[6]*(sf[296]*(sf[540]*(sf[872]+(sf[809]*((sf[814]*(-((-(v4h5/sf[507]))*v4hf)))+(sf[810]*(sf[321]-v4h5))))))))+(if (sf[302]!=0.0){((v1ri*v3et)+(v1bn*(if sb[65]{(((v1rg*(sf[859]*v1yh))-(v1rc*((vbt*(if v1r5{(v1r6*sf[861])}else{(if v1r1{(v1r2*sf[861])}else{v3})}))/v4pc)))/v4pk)}else{(if sb[64]{((sf[858]*((sf[845]*(if sb[64]{(((v1qe*v4mb)-(v1qb*(v4mb/v4mc)))/v4mk)}else{v3}))+(sf[854]*(if sb[64]{(((v1ql*v4n9)-(v1qi*(v4n9/v4na)))/v4ni)}else{v3}))))/sf[763])}else{v3})})))}else{v3})));
        let v569=(sf[0]*(v4in+(if (sf[302]!=0.0){(v4qc+(v1ri*v3eu))}else{v3})));let v57d=(sf[0]*((sf[7]*(sf[296]*(sf[540]*(sf[871]+(sf[809]*((sf[814]*(-((-(v4ec/sf[507]))*v4ep)))+(sf[810]*(sf[0]-v4ec))))))))+(if (sf[302]!=0.0){(sf[7]*v4ls)}else{v4ls})));let v57e=(sf[0]*((sf[7]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(-((-(v4ed/sf[507]))*v4ep)))+(sf[810]*(sf[322]-v4ed))))+sf[908]))))+(if (sf[302]!=0.0){(sf[7]*v4lt)}else{v4lt})));
        let v57f=(sf[0]*((sf[7]*(sf[296]*(sf[540]*((sf[809]*((sf[814]*(-((-(v4ee/sf[507]))*v4ep)))+(sf[810]*(sf[323]-v4ee))))+sf[909]))))+(if (sf[302]!=0.0){(sf[7]*v4lu)}else{v4lu})));let v57g=(sf[0]*((sf[7]*(sf[296]*(sf[540]*(sf[872]+(sf[809]*((sf[814]*(-((-(v4ef/sf[507]))*v4ep)))+(sf[810]*(sf[321]-v4ef))))))))+(if (sf[302]!=0.0){(sf[7]*v4lv)}else{v4lv})));

        CommonStampValues {
            v1, v3, vw, vx, v1c, v4g, vbd, vbh, 
            vbt, vcj, vjf, vjj, vjl, vjq, vjt, vjy, 
            vk6, vk9, vkc, vkg, vlh, vli, vlk, vln, 
            vlo, vo0, vre, vt2, vtr, vtu, vtx, vuo, 
            vww, vxw, vxx, vy2, vy3, vym, vyo, vyr, 
            vys, vz1, vzx, vzz, v101, v106, v107, v10e, 
            v10f, v10h, v10m, v10o, v124, v126, v128, v12d, 
            v12e, v135, v13i, v13v, v148, v14f, v14g, v14j, 
            v14l, v14q, v14r, v14x, v151, v154, v15c, v15d, 
            v15e, v15g, v15i, v15m, v15n, v15p, v15s, v15u, 
            v15v, v160, v161, v173, v175, v177, v178, v17b, 
            v17d, v17i, v17j, v17o, v17r, v17t, v181, v182, 
            v183, v185, v18a, v18b, v18d, v18f, v18h, v18i, 
            v18n, v18o, v1ak, v1b1, v1bn, v1dn, v1dz, v1ec, 
            v1ed, v1ee, v1eh, v1ei, v1em, v1en, v1ep, v1et, 
            v1ev, v1f0, v1f1, v1fg, v1if, v1ig, v1ii, v1ik, 
            v1im, v1io, v1ip, v1ir, v1iz, v1j2, v1j3, v1j4, 
            v1ja, v1jc, v1jd, v1jh, v1jj, v1jm, v1jo, v1jt, 
            v1ju, v1ta, v1u6, v1vd, v1vg, v1vj, v1vm, v1vq, 
            v1vu, v1w2, v1w8, v1wj, v1xp, v1xq, v1xr, v1xs, 
            v212, v213, v214, v293, v294, v295, v2d7, v2d8, 
            v2d9, v2ee, v2ef, v2eg, v2en, v2eo, v2ep, v2ew, 
            v2ex, v2ey, v2fu, v2fv, v2ku, v2kv, v2kw, v2ne, 
            v2nf, v2ng, v2nh, v2nk, v2nn, v2nq, v2nt, v2nu, 
            v2nv, v2nw, v2ny, v2o2, v2o5, v2p3, v2p4, v2qr, 
            v2qs, v2uh, v2ui, v2uj, v2w2, v2w3, v2w4, v2wh, 
            v2wi, v2wj, v2x4, v2x5, v2x6, v2x7, v2x8, v2xp, 
            v2xq, v2xr, v2xs, v2xt, v3aj, v3ak, v3al, v3am, 
            v3az, v3b0, v3b1, v3b2, v3b3, v3b4, v3b5, v3b6, 
            v3en, v3eo, v3ep, v3eq, v3er, v3es, v3et, v3eu, 
            v3nb, v3nc, v3nd, v3ne, v4yi, v4yj, v4yk, v4yl, 
            v4ym, v4yn, v539, v53a, v53b, v53c, v53d, v53e, 
            v53s, v53t, v53y, v53z, v540, v541, v542, v543, 
            v54g, v54h, v54i, v54j, v54k, v54l, v562, v563, 
            v564, v565, v566, v567, v568, v569, v57d, v57e, 
            v57f, v57g, 
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
            v1, v3, vw, vx, v1c, v4g, vbd, vbh, 
            vbt, vcj, vjf, vjj, vjl, vjq, vjt, vjy, 
            vk6, vk9, vkc, vkg, vlh, vli, vlk, vln, 
            vlo, vo0, vre, vt2, vtr, vtu, vtx, vuo, 
            vww, vxw, vxx, vy2, vy3, vym, vyo, vyr, 
            vys, vz1, vzx, vzz, v101, v106, v107, v10e, 
            v10f, v10h, v10m, v10o, v124, v126, v128, v12d, 
            v12e, v135, v13i, v13v, v148, v14f, v14g, v14j, 
            v14l, v14q, v14r, v14x, v151, v154, v15c, v15d, 
            v15e, v15g, v15i, v15m, v15n, v15p, v15s, v15u, 
            v15v, v160, v161, v173, v175, v177, v178, v17b, 
            v17d, v17i, v17j, v17o, v17r, v17t, v181, v182, 
            v183, v185, v18a, v18b, v18d, v18f, v18h, v18i, 
            v18n, v18o, v1ak, v1b1, v1bn, v1dn, v1dz, v1ec, 
            v1ed, v1ee, v1eh, v1ei, v1em, v1en, v1ep, v1et, 
            v1ev, v1f0, v1f1, v1fg, v1if, v1ig, v1ii, v1ik, 
            v1im, v1io, v1ip, v1ir, v1iz, v1j2, v1j3, v1j4, 
            v1ja, v1jc, v1jd, v1jh, v1jj, v1jm, v1jo, v1jt, 
            v1ju, v1ta, v1u6, v1vd, v1vg, v1vj, v1vm, v1vq, 
            v1vu, v1w2, v1w8, v1wj, v1xp, v1xq, v1xr, v1xs, 
            v212, v213, v214, v293, v294, v295, v2d7, v2d8, 
            v2d9, v2ee, v2ef, v2eg, v2en, v2eo, v2ep, v2ew, 
            v2ex, v2ey, v2fu, v2fv, v2ku, v2kv, v2kw, v2ne, 
            v2nf, v2ng, v2nh, v2nk, v2nn, v2nq, v2nt, v2nu, 
            v2nv, v2nw, v2ny, v2o2, v2o5, v2p3, v2p4, v2qr, 
            v2qs, v2uh, v2ui, v2uj, v2w2, v2w3, v2w4, v2wh, 
            v2wi, v2wj, v2x4, v2x5, v2x6, v2x7, v2x8, v2xp, 
            v2xq, v2xr, v2xs, v2xt, v3aj, v3ak, v3al, v3am, 
            v3az, v3b0, v3b1, v3b2, v3b3, v3b4, v3b5, v3b6, 
            v3en, v3eo, v3ep, v3eq, v3er, v3es, v3et, v3eu, 
            v3nb, v3nc, v3nd, v3ne, v4yi, v4yj, v4yk, v4yl, 
            v4ym, v4yn, v539, v53a, v53b, v53c, v53d, v53e, 
            v53s, v53t, v53y, v53z, v540, v541, v542, v543, 
            v54g, v54h, v54i, v54j, v54k, v54l, v562, v563, 
            v564, v565, v566, v567, v568, v569, v57d, v57e, 
            v57f, v57g, 
        }=self.eval_common_stamp_values(ctx);
        let vll=(vli).exp();let vyp=(vym).exp();let vyw=(if vyr{(vys*(v1+(vym-sf[198])))}else{(if (vyo!=0.0){vyp}else{v3})});let vz3=(if (vjl<sf[228]){v1}else{v3});let vz4=(vz1).exp();let vz5=(v1+vz4);let vza=(!(vz3!=0.0));let vzc=((-vz1)).exp();let vzd=(v1+vzc);let vzh=(if vza{(sf[228]-(vw*(vzd).ln()))}else{(if (vz3!=0.0){(vjl-(vw*(vz5).ln()))}else{v3})});let vzj=(vzh*sf[229]);let vzk=(sf[228]-vzh);let vzl=f64::powf(vzk,vx);let v102=((sf[149]!=0.0)&&(v101!=0.0));let v103=(vzz).exp();
        let v10b=(if v106{(v107*(v1+(vzz-sf[198])))}else{(if v102{v103}else{vym})});let v10i=((sf[149]!=0.0)&&(v10h!=0.0));let v10j=(v10e).exp();let v10s=(if v10m{(v10o*(v1+(v10e-v10f)))}else{(if v10i{v10j}else{vyw})});let v10t=(vzx-v1);let v10u=(sf[661]*v10t);let v10w=(v10t*sf[822]);let v10z=((v1+(vbt*v10b))).sqrt();let v110=(v1+v10z);let v111=(v10w/v110);let v112=(v1+vww);let v116=(sf[676]*(vt2-v1));let v117=(v10s*v116);let v118=(v1+v10s);let v11o=(sf[230]*((vt2+vzx)-vx));
        let v129=((sf[149]!=0.0)&&(v128!=0.0));let v12a=(v126).exp();let v12j=(v124-v1);let v12k=(sf[667]*v12j);let v12m=(v12j*sf[823]);let v12p=((v1+(vbt*(if v12d{(v12e*(v1+(v126-sf[198])))}else{(if v129{v12a}else{v10b})})))).sqrt();let v12q=(v1+v12p);let v13x=(sf[653]*(v13v-v1));let v14m=((v14f!=0.0)&&(v14l!=0.0));let v14n=(v14j).exp();let v14v=(if v14q{(v14r*(v1+(v14j-sf[198])))}else{(if v14m{v14n}else{v3})});let v15w=((v15u!=0.0)&&v15v);let v15x=(v15p).exp();let v166=(-vjl);
        let v167=(v1-(if v160{(v161*(v1+(v15p-sf[198])))}else{(if v15w{v15x}else{v3})}));let v169=(v1+(v167/v15p));let v16d=((v14f!=0.0)&&(!(v15s!=0.0)));let v16e=(vbh*vjl);let v16f=(v15p*v16e);let v16g=0.3333333333333333;let v16h=(v15p*v16g);let v16i=0.25;let v16k=(v1+(v15p*v16i));let v16m=(v1+(v16h*v16k));let v16q=((if v16d{(v16f*v16m)}else{(if v15v{(v166*v169)}else{v3})})*sf[824]);let v16r=(vuo*v16q);let v16w=(!(v14f!=0.0));let v17e=((v173!=0.0)&&(v17d!=0.0));let v17f=(v17b).exp();
        let v17n=(if v17i{(v17j*(v1+(v17b-sf[198])))}else{(if v17e{v17f}else{v3})});let v18j=((v18h!=0.0)&&v18i);let v18k=(v18d).exp();let v18t=(-vjf);let v18u=(v1-(if v18n{(v18o*(v1+(v18d-sf[198])))}else{(if v18j{v18k}else{v3})}));let v18w=(v1+(v18u/v18d));let v190=((v173!=0.0)&&(!(v18f!=0.0)));let v191=(vbh*vjf);let v192=(v18d*v191);let v193=(v16g*v18d);let v195=(v1+(v16i*v18d));let v197=(v1+(v193*v195));let v19b=((if v190{(v192*v197)}else{(if v18i{(v18t*v18w)}else{v3})})*sf[825]);let v19c=(v177*v19b);
        let v19h=(!(v173!=0.0));let v19i=(if v19h{v3}else{(if (v173!=0.0){(sf[53]*(sf[529]*(v17n*v19c)))}else{v3})});let v19w=(sf[826]*(vlh-v1));let v1a1=((v1+(vlh*sf[828]))).sqrt();let v1a2=(v1+v1a1);let v1a3=(v19w/v1a2);let v1aa=(if (sf[242]!=0.0){(sf[7]*v1a3)}else{v1a3});let v1bp=(if (sf[242]!=0.0){(v1ak*v1bn)}else{v3});let v1bu=(if (sf[248]!=0.0){(vjf+vjq)}else{v3});let v1bw=(-v1bu);let v1c0=(if (v1bw<v3){v1}else{v3});let v1c1=((sf[248]!=0.0)&&(v1c0!=0.0));
        let v1c4=((sf[249]+(if (sf[248]!=0.0){(v1bu*v1bu)}else{v1b1}))).sqrt();let v1c5=(v1c4-v1bw);let v1c9=((sf[248]!=0.0)&&(!(v1c0!=0.0)));let v1cc=(if v1c9{(vbh*(v1bw+v1c4))}else{(if v1c1{(sf[250]/v1c5)}else{v3})});let v1ct=(if (v1cc<sf[258]){v1}else{v3});let v1cu=((sf[248]!=0.0)&&(v1ct!=0.0));let v1cv=(v1cc/sf[256]);let v1cx=(v1-f64::powf(v1cv,sf[251]));let v1d1=((sf[248]!=0.0)&&(!(v1ct!=0.0)));
        let v1d7=(if sb[48]{v1}else{(if v1d1{(sf[255]+(sf[265]*(v1cc-sf[258])))}else{(if v1cu{(v1/v1cx)}else{v3})})});let v1do=(vxw*v1dn);let v1dp=(sf[549]/v1do);let v1dr=(if (v1dp<sf[16]){v1}else{v3});let v1dt=(v4g*(if (v1dr!=0.0){sf[16]}else{v1dp}));let v1dw=(vjq+(sf[795]*((if vln{(vlo*(v1+(vli-sf[198])))}else{(if (vlk!=0.0){vll}else{v3})})-v1)));let v1ew=(v1ec&&(v1ev!=0.0));let v1ex=(v1et).exp();let v1f5=(if v1f0{(v1f1*(v1+(v1et-sf[198])))}else{(if v1ew{v1ex}else{v3})});let v1f8=(v1ep*sf[839]);
        let v1fi=(((if (vjf<sf[469]){v1}else{v3})!=0.0)&&((sf[272]!=0.0)&&v1fg));let v1fo=(if v1fi{sf[277]}else{v3});let v1fp=(sf[469]-vjf);let v1fr=(if v1fi{(v1fp/vtx)}else{vre});let v1fu=(((vx*v1fr)/v1fo)).sqrt();let v1fv=(if v1fi{v1fu}else{v3});let v1fz=(v1fi&&(sf[279]!=0.0));let v1g2=(v1fi&&sb[53]);let v1g5=(if v1g2{(v1-(vbh*vtr))}else{v3});let v1g6=(sf[275]*v1g5);let v1g8=(if v1g2{(v1g5*v1g6)}else{(if v1fz{sf[275]}else{v3})});let v1g9=(v1fv*v1g8);let v1gd=(((v1fv*v1fv)+(v1g8*v1g8))).sqrt();
        let v1gf=(if v1fi{(v1g9/v1gd)}else{v3});let v1gh=(if v1fi{(v1fp/v1gf)}else{v3});let v1gi=(vbh*v1gf);let v1gj=(v1fo*v1gi);let v1gm=(if v1fi{(v1gh+(vtx*v1gj))}else{v3});let v1gz=(sf[201]*(if v1g2{(v1+(sf[281]*(v1+(vx*vtr))))}else{v3}));let v1h1=((if v1g2{sf[284]}else{v3})-(vy3/v1gz));let v1h4=(if v1g2{(v1gh-(v1gj*v1h1))}else{v3});let v1h5=(v1h4-v1gm);let v1h7=(v1c*v1gh);let v1h8=(v1gh*v1h7);let v1he=((if v1g2{((v1h5*v1h5)+((vtu*v1h8)/sf[201]))}else{v1fr})).sqrt();
        let v1hh=(if v1g2{(vbh*((v1gm+v1h4)+v1he))}else{(if v1fz{v1gm}else{v3})});let v1hi=(v1hh-v1gh);let v1hk=(if v1fi{(v1hi/v1hh)}else{v3});let v1ho=(if ((v1hk).abs()>1e-7){v1}else{v3});let v1hp=(v1fi&&(v1ho!=0.0));let v1hr=(if v1hp{(v1gi/v1hk)}else{v3});let v1ht=(v1hh*sf[840]);let v1hu=(v1hr*v1ht);let v1hw=(sf[841]/v1hh);let v1hx=(v1hw).exp();let v1hz=(v1+(v1g8/v1hr));let v1i1=((v1hw*v1hz)).exp();let v1i2=(v1hx-v1i1);let v1i6=(v1fi&&(!(v1ho!=0.0)));let v1i7=(sf[4]*v1g8);let v1jp=(v1if&&(v1jo!=0.0));
        let v1jq=(v1jm).exp();let v1jy=(if v1jt{(v1ju*(v1+(v1jm-sf[198])))}else{(if v1jp{v1jq}else{v1f5})});let v1jz=(v1en*sf[839]);let v1k1=(if v1if{(v1jy*v1jz)}else{(if v1i6{(v1hx*v1i7)}else{(if v1hp{(v1hu*v1i2)}else{(if v1ec{(v1f5*v1f8)}else{v3})})})});let v1k7=((v1dz!=0.0)&&((if (v1k1>v3){v1}else{v3})!=0.0));let v1k8=((sf[292]!=0.0)&&v1k7);let v1k9=(sf[554]+v1dt);let v1ka=(vy3*v1k9);let v1kh=(if v1k8{(((sf[380]/v1ka)+(sf[661]*(vxx/sf[633])))+(sf[546]/v1k9))}else{v3});let v1ki=((sf[285]!=0.0)&&v1k8);
        let v1kl=(if v1ki{((v1k1-v1kh)/vbd)}else{v1iz});let v1kn=(if (v1k1<v1kh){v1}else{v3});let v1ko=(v1ki&&(v1kn!=0.0));let v1kp=(v1kl).exp();let v1kq=(v1+v1kp);let v1kw=(v1ki&&(!(v1kn!=0.0)));let v1ky=((-v1kl)).exp();let v1kz=(v1+v1ky);let v1l3=(if v1kw{(v1kh-(vbd*(v1kz).ln()))}else{(if v1ko{(v1k1-(vbd*(v1kq).ln()))}else{v1k1})});let v1l4=(vy3*v1l3);let v1l7=(v1k8&&sb[57]);let v1l8=(v1kh*v1l4);let v1l9=(v1kh+v1l3);let v1ld=(v1k7&&sb[58]);
        let v1le=(if v1ld{v1l4}else{(if v1l7{(v1l8/v1l9)}else{(if v1ki{v1l4}else{v3})})});let v1ti=(if sb[69]{v3}else{(if (sf[312]!=0.0){((v1le/v1ta)).abs()}else{v3})});let v1v0=(sf[15]*(sf[0]*(-(v19i*v1d7))));let v1ve=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v1vd);
        let v1vh=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v1vg);
        let v1vk=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v1vj);
        let v1vn=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v1vm);
        let v1vr=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v1vq);
        let v1vv=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v1vu);
        let v1w3=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v1w2);
        let v1w9=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v1w8);
        let v1wk=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v1wj);let v2o6=((v2o2-(vy2*v2nk))/v2o5);let v2oa=(((vxx*(v2ny-v2nu))-(vy2*v2nn))/v2o5);let v2oe=(((vxx*(-v2nv))-(vy2*v2nq))/v2o5);let v2oi=(((vxx*(-v2nw))-(vy2*v2nt))/v2o5);let v2p5=(v2p3/sf[227]);let v2p6=(v2p4/sf[227]);
        let v2pd=(if vyr{(vys*v2p5)}else{(if (vyo!=0.0){(vyp*v2p5)}else{v3})});let v2pe=(if vyr{(vys*v2p6)}else{(if (vyo!=0.0){(vyp*v2p6)}else{v3})});let v2q3=(if vza{(-(vw*((vzc*sf[336])/vzd)))}else{(if (vz3!=0.0){(sf[321]-(vw*((vz4*sf[334])/vz5)))}else{v3})});let v2q4=(if vza{(-(vw*((vzc*sf[337])/vzd)))}else{(if (vz3!=0.0){(sf[0]-(vw*((vz4*sf[335])/vz5)))}else{v3})});let v2qa=(vx*f64::powf(vzk,v1));let v2qz=(if v106{(v107*sf[861])}else{(if v102{(v103*sf[861])}else{v2p5})});
        let v2r0=(if v106{(v107*sf[860])}else{(if v102{(v103*sf[860])}else{v2p6})});let v2r1=(v2o6/sf[633]);let v2r2=(v2oa/sf[633]);let v2r3=(v2oe/sf[633]);let v2r4=(v2oi/sf[633]);let v2rh=(if v10m{(v10o*v2r1)}else{(if v10i{(v10j*v2r1)}else{v2pd})});let v2ri=(if v10m{(v10o*v2r2)}else{(if v10i{(v10j*v2r2)}else{v2pe})});let v2rj=(if v10m{(v10o*v2r3)}else{(if v10i{(v10j*v2r3)}else{v3})});let v2rk=(if v10m{(v10o*v2r4)}else{(if v10i{(v10j*v2r4)}else{v3})});let v2rl=(sf[661]*v2qr);let v2rm=(sf[661]*v2qs);
        let v2rr=(vx*v10z);let v2rx=(v110*v110);let v2sr=(v118*v118);let v2uu=(sf[667]*v2uh);let v2uv=(sf[667]*v2ui);let v2uw=(sf[667]*v2uj);let v2v3=(vx*v12p);let v2va=(v12q*v12q);let v2y3=(v14g*v14g);let v2ya=(sf[715]*(-((-(sf[20]*(vx*v2fu)))/v2y3)));let v2yb=(sf[715]*(-((-(sf[20]*(vx*v2fv)))/v2y3)));let v2ym=(if (v14f!=0.0){sf[888]}else{v3});let v2yn=(if (v14f!=0.0){sf[889]}else{v3});let v2yo=(v14x*v2ym);let v2yq=(v14x*v2yn);let v2ys=(vx*v151);let v2yx=(sf[233]*f64::powf(v151,sf[338]));
        let v307=(v15n*v15n);let v30d=(if (v14f!=0.0){(((v15n*sf[890])-(v15m*(sf[405]*(if (v14f!=0.0){(v15i*((v15g*(((v2yo+v2yo)/v2ys)*v2yx))+(v154*((sf[18]*(-(sf[236]*(v4g*v2ym))))-((v15e*((v15c*v2ym)+(v14x*(vcj*v2ym))))+(v15d*v2ym))))))}else{v3}))))/v307)}else{v2ym});let v30e=(if (v14f!=0.0){(((v15n*sf[891])-(v15m*(sf[405]*(if (v14f!=0.0){(v15i*((v15g*(((v2yq+v2yq)/v2ys)*v2yx))+(v154*((sf[18]*(-(sf[236]*(v4g*v2yn))))-((v15e*((v15c*v2yn)+(v14x*(vcj*v2yn))))+(v15d*v2yn))))))}else{v3}))))/v307)}else{v2yn});
        let v30s=(v15p*v15p);let v32n=(sf[224]*f64::powf(v175,sf[329]));let v32q=(if (v173!=0.0){(sf[894]*v32n)}else{v3});let v32r=(if (v173!=0.0){(sf[895]*v32n)}else{v3});let v32w=(v178*v178);let v333=(sf[735]*(-((-(sf[52]*(vx*v32q)))/v32w)));let v334=(sf[735]*(-((-(sf[52]*(vx*v32r)))/v32w)));let v33d=(if (v173!=0.0){sf[892]}else{v3});let v33e=(if (v173!=0.0){sf[893]}else{v3});let v33f=(v17o*v33d);let v33h=(v17o*v33e);let v33j=(vx*v17r);let v33o=(sf[237]*f64::powf(v17r,sf[343]));let v34y=(v18b*v18b);
        let v354=(if (v173!=0.0){(((v18b*sf[896])-(v18a*(sf[426]*(if (v173!=0.0){(v15i*((v185*(((v33f+v33f)/v33j)*v33o))+(v17t*((sf[50]*(-(sf[240]*(v4g*v33d))))-((v183*((v181*v33d)+(v17o*(vcj*v33d))))+(v182*v33d))))))}else{v3}))))/v34y)}else{v33d});let v355=(if (v173!=0.0){(((v18b*sf[897])-(v18a*(sf[426]*(if (v173!=0.0){(v15i*((v185*(((v33h+v33h)/v33j)*v33o))+(v17t*((sf[50]*(-(sf[240]*(v4g*v33e))))-((v183*((v181*v33e)+(v17o*(vcj*v33e))))+(v182*v33e))))))}else{v3}))))/v34y)}else{v33e});let v35j=(v18d*v18d);
        let v38v=(vx*v1a1);let v393=(v1a2*v1a2);let v394=(((v1a2*(sf[826]*v1xp))-(v19w*((sf[828]*v1xp)/v38v)))/v393);let v398=(((v1a2*(sf[826]*v1xq))-(v19w*((sf[828]*v1xq)/v38v)))/v393);let v39c=(((v1a2*(sf[826]*v1xr))-(v19w*((sf[828]*v1xr)/v38v)))/v393);let v39g=(((v1a2*(sf[826]*v1xs))-(v19w*((sf[828]*v1xs)/v38v)))/v393);let v3ev=(v1bn*v3aj);let v3f4=(v1bn*v3al);let v3fs=(v1bu*sf[350]);let v3fu=(v1bu*sf[351]);let v3fw=(v1bu*sf[352]);let v3g7=(vx*v1c4);let v3g8=((if (sf[248]!=0.0){v3}else{v3az})/v3g7);
        let v3g9=((if (sf[248]!=0.0){v3}else{v3b0})/v3g7);let v3ga=((if (sf[248]!=0.0){v3}else{v3b1})/v3g7);let v3gb=((if (sf[248]!=0.0){(v3fs+v3fs)}else{v3az})/v3g7);let v3gc=((if (sf[248]!=0.0){(v3fu+v3fu)}else{v3b2})/v3g7);let v3gd=((if (sf[248]!=0.0){(v3fw+v3fw)}else{v3b3})/v3g7);let v3ge=((if (sf[248]!=0.0){v3}else{v3b4})/v3g7);let v3gf=((if (sf[248]!=0.0){v3}else{v3b5})/v3g7);let v3gg=((if (sf[248]!=0.0){v3}else{v3b6})/v3g7);let v3gm=(v1c5*v1c5);
        let v3hx=(if v1c9{(vbh*v3g8)}else{(if v1c1{((-(sf[250]*v3g8))/v3gm)}else{v3})});let v3hy=(if v1c9{(vbh*v3g9)}else{(if v1c1{((-(sf[250]*v3g9))/v3gm)}else{v3})});let v3hz=(if v1c9{(vbh*v3ga)}else{(if v1c1{((-(sf[250]*v3ga))/v3gm)}else{v3})});let v3i0=(if v1c9{(vbh*(sf[353]+v3gb))}else{(if v1c1{((-(sf[250]*(v3gb-sf[353])))/v3gm)}else{v3})});let v3i1=(if v1c9{(vbh*(sf[354]+v3gc))}else{(if v1c1{((-(sf[250]*(v3gc-sf[354])))/v3gm)}else{v3})});
        let v3i2=(if v1c9{(vbh*(sf[355]+v3gd))}else{(if v1c1{((-(sf[250]*(v3gd-sf[355])))/v3gm)}else{v3})});let v3i3=(if v1c9{(vbh*v3ge)}else{(if v1c1{((-(sf[250]*v3ge))/v3gm)}else{v3})});let v3i4=(if v1c9{(vbh*v3gf)}else{(if v1c1{((-(sf[250]*v3gf))/v3gm)}else{v3})});let v3i5=(if v1c9{(vbh*v3gg)}else{(if v1c1{((-(sf[250]*v3gg))/v3gm)}else{v3})});let v3ig=(sf[251]*f64::powf(v1cv,sf[260]));let v3iq=(v1cx*v1cx);
        let v3jr=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3hx)}else{(if v1cu{(((v3hx/sf[256])*v3ig)/v3iq)}else{v3})})});let v3js=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3hy)}else{(if v1cu{(((v3hy/sf[256])*v3ig)/v3iq)}else{v3})})});let v3jt=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3hz)}else{(if v1cu{(((v3hz/sf[256])*v3ig)/v3iq)}else{v3})})});let v3ju=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3i0)}else{(if v1cu{(((v3i0/sf[256])*v3ig)/v3iq)}else{v3})})});
        let v3jv=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3i1)}else{(if v1cu{(((v3i1/sf[256])*v3ig)/v3iq)}else{v3})})});let v3jw=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3i2)}else{(if v1cu{(((v3i2/sf[256])*v3ig)/v3iq)}else{v3})})});let v3jx=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3i3)}else{(if v1cu{(((v3i3/sf[256])*v3ig)/v3iq)}else{v3})})});let v3jy=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3i4)}else{(if v1cu{(((v3i4/sf[256])*v3ig)/v3iq)}else{v3})})});
        let v3jz=(if sb[48]{v3}else{(if v1d1{(sf[265]*v3i5)}else{(if v1cu{(((v3i5/sf[256])*v3ig)/v3iq)}else{v3})})});let v3km=(v1d7*(if (sf[242]!=0.0){(sf[7]*v39c)}else{v39c}));let v3l6=(v1d7*(sf[653]*v2x7));let v3lf=(v1d7*(if (sf[242]!=0.0){(v3ev+(v1ak*v3en))}else{v3}));let v3nt=(v1do*v1do);let v3o8=(v4g*(if (v1dr!=0.0){v3}else{((-(sf[549]*((v1dn*v2ne)+(vxw*v3nb))))/v3nt)}));let v3o9=(v4g*(if (v1dr!=0.0){v3}else{((-(sf[549]*((v1dn*v2nf)+(vxw*v3nc))))/v3nt)}));
        let v3oa=(v4g*(if (v1dr!=0.0){v3}else{((-(sf[549]*((v1dn*v2ng)+(vxw*v3nd))))/v3nt)}));let v3ob=(v4g*(if (v1dr!=0.0){v3}else{((-(sf[549]*((v1dn*v2nh)+(vxw*v3ne))))/v3nt)}));let v3oi=(v1dt*v1dt);let v3oz=((-v2o6)/sf[269]);let v3p0=((-v2oa)/sf[269]);let v3p1=((-v2oe)/sf[269]);let v3p2=((-v2oi)/sf[269]);let v3pr=(if v1ec{(v1en*(if v1eh{(v1ei*v3oz)}else{(if v1ed{(v1ee*v3oz)}else{v3})}))}else{v3});
        let v3ps=(if v1ec{((v1en*(if v1eh{(v1ei*v3p0)}else{(if v1ed{(v1ee*v3p0)}else{v3})}))+(v1em*sf[321]))}else{v3});let v3pt=(if v1ec{((v1en*(if v1eh{(v1ei*v3p1)}else{(if v1ed{(v1ee*v3p1)}else{v3})}))+(sf[0]*v1em))}else{v3});let v3pu=(if v1ec{(v1en*(if v1eh{(v1ei*v3p2)}else{(if v1ed{(v1ee*v3p2)}else{v3})}))}else{v3});let v3px=(sf[270]*f64::powf(v1ep,sf[356]));let v3q2=(sf[838]*(v3pr*v3px));let v3q3=(sf[838]*(v3ps*v3px));let v3q4=(sf[838]*(v3pt*v3px));let v3q5=(sf[838]*(v3pu*v3px));
        let v3qi=(if v1f0{(v1f1*v3q2)}else{(if v1ew{(v1ex*v3q2)}else{v3})});let v3qj=(if v1f0{(v1f1*v3q3)}else{(if v1ew{(v1ex*v3q3)}else{v3})});let v3qk=(if v1f0{(v1f1*v3q4)}else{(if v1ew{(v1ex*v3q4)}else{v3})});let v3ql=(if v1f0{(v1f1*v3q5)}else{(if v1ew{(v1ex*v3q5)}else{v3})});let v3r9=(vtx*vtx);let v3ri=(if v1fi{(((vtx*sf[321])-(v1fp*v2ew))/v3r9)}else{v293});let v3rj=(if v1fi{(((sf[0]*vtx)-(v1fp*v2ex))/v3r9)}else{v294});let v3rk=(if v1fi{((-(v1fp*v2ey))/v3r9)}else{v295});let v3rr=(vx*v1fu);
        let v3rv=(if v1fi{(((vx*v3ri)/v1fo)/v3rr)}else{v3});let v3rw=(if v1fi{(((vx*v3rj)/v1fo)/v3rr)}else{v3});let v3rx=(if v1fi{(((vx*v3rk)/v1fo)/v3rr)}else{v3});let v3s4=(if v1g2{(-(vbh*v2ee))}else{v3});let v3s5=(if v1g2{(-(vbh*v2ef))}else{v3});let v3s6=(if v1g2{(-(vbh*v2eg))}else{v3});let v3sj=(if v1g2{((v1g6*v3s4)+(v1g5*(sf[275]*v3s4)))}else{v3});let v3sk=(if v1g2{((v1g6*v3s5)+(v1g5*(sf[275]*v3s5)))}else{v3});let v3sl=(if v1g2{((v1g6*v3s6)+(v1g5*(sf[275]*v3s6)))}else{v3});let v3sv=(v1fv*v3rv);
        let v3sx=(v1fv*v3rw);let v3sz=(v1fv*v3rx);let v3t1=(v1g8*v3sj);let v3t3=(v1g8*v3sk);let v3t5=(v1g8*v3sl);let v3ta=(vx*v1gd);let v3th=(v1gd*v1gd);let v3tr=(if v1fi{(((v1gd*((v1g8*v3rv)+(v1fv*v3sj)))-(v1g9*(((v3sv+v3sv)+(v3t1+v3t1))/v3ta)))/v3th)}else{v3});let v3ts=(if v1fi{(((v1gd*((v1g8*v3rw)+(v1fv*v3sk)))-(v1g9*(((v3sx+v3sx)+(v3t3+v3t3))/v3ta)))/v3th)}else{v3});let v3tt=(if v1fi{(((v1gd*((v1g8*v3rx)+(v1fv*v3sl)))-(v1g9*(((v3sz+v3sz)+(v3t5+v3t5))/v3ta)))/v3th)}else{v3});let v3tx=(v1gf*v1gf);
        let v3u6=(if v1fi{(((v1gf*sf[321])-(v1fp*v3tr))/v3tx)}else{v3});let v3u7=(if v1fi{(((sf[0]*v1gf)-(v1fp*v3ts))/v3tx)}else{v3});let v3u8=(if v1fi{((-(v1fp*v3tt))/v3tx)}else{v3});let v3u9=(vbh*v3tr);let v3ua=(vbh*v3ts);let v3ub=(vbh*v3tt);let v3uc=(v1fo*v3u9);let v3ud=(v1fo*v3ua);let v3ue=(v1fo*v3ub);let v3ur=(if v1fi{(v3u6+((v1gj*v2ew)+(vtx*v3uc)))}else{v3});let v3us=(if v1fi{(v3u7+((v1gj*v2ex)+(vtx*v3ud)))}else{v3});let v3ut=(if v1fi{(v3u8+((v1gj*v2ey)+(vtx*v3ue)))}else{v3});let v3vd=(v1gz*v1gz);
        let v3w5=(if v1g2{(-(v1gj*(-(v2o6/v1gz))))}else{v3});let v3w6=(if v1g2{(v3u6-((v1h1*v3uc)+(v1gj*(-(((v1gz*v2oa)-(vy3*(sf[201]*(if v1g2{(sf[281]*(vx*v2ee))}else{v3}))))/v3vd)))))}else{v3});let v3w7=(if v1g2{(v3u7-((v1h1*v3ud)+(v1gj*(-(((v1gz*v2oe)-(vy3*(sf[201]*(if v1g2{(sf[281]*(vx*v2ef))}else{v3}))))/v3vd)))))}else{v3});let v3w8=(if v1g2{(v3u8-((v1h1*v3ue)+(v1gj*(-(((v1gz*v2oi)-(vy3*(sf[201]*(if v1g2{(sf[281]*(vx*v2eg))}else{v3}))))/v3vd)))))}else{v3});let v3wc=(v1h5*v3w5);
        let v3we=(v1h5*(v3w6-v3ur));let v3wg=(v1h5*(v3w7-v3us));let v3wi=(v1h5*(v3w8-v3ut));let v3xi=(vx*v1he);let v3xv=(if v1g2{(vbh*(v3w5+((if v1g2{(v3wc+v3wc)}else{v3})/v3xi)))}else{v3});let v3xw=(if v1g2{(vbh*((v3ur+v3w6)+((if v1g2{((v3we+v3we)+(((v1h8*v2en)+(vtu*((v1h7*v3u6)+(v1gh*(v1c*v3u6)))))/sf[201]))}else{v3ri})/v3xi)))}else{(if v1fz{v3ur}else{v3})});
        let v3xx=(if v1g2{(vbh*((v3us+v3w7)+((if v1g2{((v3wg+v3wg)+(((v1h8*v2eo)+(vtu*((v1h7*v3u7)+(v1gh*(v1c*v3u7)))))/sf[201]))}else{v3rj})/v3xi)))}else{(if v1fz{v3us}else{v3})});let v3xy=(if v1g2{(vbh*((v3ut+v3w8)+((if v1g2{((v3wi+v3wi)+(((v1h8*v2ep)+(vtu*((v1h7*v3u8)+(v1gh*(v1c*v3u8)))))/sf[201]))}else{v3rk})/v3xi)))}else{(if v1fz{v3ut}else{v3})});let v3y5=(v1hh*v1hh);let v3yp=(v1hk*v1hk);let v3z3=(if v1hp{((-(v1gi*(if v1fi{(((v1hh*v3xv)-(v1hi*v3xv))/v3y5)}else{v3})))/v3yp)}else{v3});
        let v3z4=(if v1hp{(((v1hk*v3u9)-(v1gi*(if v1fi{(((v1hh*(v3xw-v3u6))-(v1hi*v3xw))/v3y5)}else{v3})))/v3yp)}else{v3});let v3z5=(if v1hp{(((v1hk*v3ua)-(v1gi*(if v1fi{(((v1hh*(v3xx-v3u7))-(v1hi*v3xx))/v3y5)}else{v3})))/v3yp)}else{v3});let v3z6=(if v1hp{(((v1hk*v3ub)-(v1gi*(if v1fi{(((v1hh*(v3xy-v3u8))-(v1hi*v3xy))/v3y5)}else{v3})))/v3yp)}else{v3});let v3zp=((-(sf[841]*v3xv))/v3y5);let v3zs=((-(sf[841]*v3xw))/v3y5);let v3zv=((-(sf[841]*v3xx))/v3y5);let v3zy=((-(sf[841]*v3xy))/v3y5);let v3zz=(v1hx*v3zp);
        let v400=(v1hx*v3zs);let v401=(v1hx*v3zv);let v402=(v1hx*v3zy);let v405=(v1hr*v1hr);let v421=(sf[270]*f64::powf(v1en,sf[356]));let v427=(v1ii*v1ii);let v42r=(sf[287]*f64::powf(v1ik,sf[357]));let v434=(if v1if{(v1ig*((-(((v1ii*v2o6)-(vy3*v2o6))/v427))*v42r))}else{v3});let v435=(if v1if{((v1im*(sf[321]*v421))+(v1ig*((-(((v1ii*v2oa)-(vy3*v2oa))/v427))*v42r)))}else{v3});let v436=(if v1if{((v1im*(sf[0]*v421))+(v1ig*((-(((v1ii*v2oe)-(vy3*v2oe))/v427))*v42r)))}else{v3});
        let v437=(if v1if{(v1ig*((-(((v1ii*v2oi)-(vy3*v2oi))/v427))*v42r))}else{v3});let v43g=(if v1ir{(v2o6/sf[286])}else{v3});let v43h=(if v1ir{(v2oa/sf[286])}else{v3});let v43i=(if v1ir{(v2oe/sf[286])}else{v3});let v43j=(if v1ir{(v2oi/sf[286])}else{v3});let v43o=(if v1ir{(v43g/sf[289])}else{sf[334]});let v43p=(if v1ir{(v43h/sf[289])}else{sf[335]});let v43q=(if v1ir{(v43i/sf[289])}else{v3});let v43r=(if v1ir{(v43j/sf[289])}else{v3});let v44y=(sf[290]*f64::powf(v1jh,sf[358]));
        let v45j=(sf[838]*(if v1ir{((v1jj*v434)+(v1io*((if v1ja{(v43g+(sf[289]*((v1jc*(-v43o))/v1jd)))}else{(if v1j2{(sf[289]*((v1j3*v43o)/v1j4))}else{v3})})*v44y)))}else{(if v1ip{v434}else{v3})}));let v45k=(sf[838]*(if v1ir{((v1jj*v435)+(v1io*((if v1ja{(v43h+(sf[289]*((v1jc*(-v43p))/v1jd)))}else{(if v1j2{(sf[289]*((v1j3*v43p)/v1j4))}else{v3})})*v44y)))}else{(if v1ip{v435}else{v3})}));
        let v45l=(sf[838]*(if v1ir{((v1jj*v436)+(v1io*((if v1ja{(v43i+(sf[289]*((v1jc*(-v43q))/v1jd)))}else{(if v1j2{(sf[289]*((v1j3*v43q)/v1j4))}else{v3})})*v44y)))}else{(if v1ip{v436}else{v3})}));let v45m=(sf[838]*(if v1ir{((v1jj*v437)+(v1io*((if v1ja{(v43j+(sf[289]*((v1jc*(-v43r))/v1jd)))}else{(if v1j2{(sf[289]*((v1j3*v43r)/v1j4))}else{v3})})*v44y)))}else{(if v1ip{v437}else{v3})}));
        let v46d=(if v1if{(v1jz*(if v1jt{(v1ju*v45j)}else{(if v1jp{(v1jq*v45j)}else{v3qi})}))}else{(if v1i6{(v1i7*v3zz)}else{(if v1hp{((v1i2*((v1ht*v3z3)+(v1hr*(sf[840]*v3xv))))+(v1hu*(v3zz-(v1i1*((v1hz*v3zp)+(v1hw*((-(v1g8*v3z3))/v405)))))))}else{(if v1ec{((v1f8*v3qi)+(v1f5*(sf[839]*v3pr)))}else{v3})})})});
        let v46e=(if v1if{((v1jz*(if v1jt{(v1ju*v45k)}else{(if v1jp{(v1jq*v45k)}else{v3qj})}))+(v1jy*sf[898]))}else{(if v1i6{((v1i7*v400)+(v1hx*(sf[4]*v3sj)))}else{(if v1hp{((v1i2*((v1ht*v3z4)+(v1hr*(sf[840]*v3xw))))+(v1hu*(v400-(v1i1*((v1hz*v3zs)+(v1hw*(((v1hr*v3sj)-(v1g8*v3z4))/v405)))))))}else{(if v1ec{((v1f8*v3qj)+(v1f5*(sf[839]*v3ps)))}else{v3})})})});
        let v46f=(if v1if{((v1jz*(if v1jt{(v1ju*v45l)}else{(if v1jp{(v1jq*v45l)}else{v3qk})}))+(v1jy*sf[899]))}else{(if v1i6{((v1i7*v401)+(v1hx*(sf[4]*v3sk)))}else{(if v1hp{((v1i2*((v1ht*v3z5)+(v1hr*(sf[840]*v3xx))))+(v1hu*(v401-(v1i1*((v1hz*v3zv)+(v1hw*(((v1hr*v3sk)-(v1g8*v3z5))/v405)))))))}else{(if v1ec{((v1f8*v3qk)+(v1f5*(sf[839]*v3pt)))}else{v3})})})});
        let v46g=(if v1if{(v1jz*(if v1jt{(v1ju*v45m)}else{(if v1jp{(v1jq*v45m)}else{v3ql})}))}else{(if v1i6{((v1i7*v402)+(v1hx*(sf[4]*v3sl)))}else{(if v1hp{((v1i2*((v1ht*v3z6)+(v1hr*(sf[840]*v3xy))))+(v1hu*(v402-(v1i1*((v1hz*v3zy)+(v1hw*(((v1hr*v3sl)-(v1g8*v3z6))/v405)))))))}else{(if v1ec{((v1f8*v3ql)+(v1f5*(sf[839]*v3pu)))}else{v3})})})});let v46v=(v1ka*v1ka);let v47k=(v1k9*v1k9);let v47z=(if v1k8{((((-(sf[380]*((v1k9*v2o6)+(vy3*v3o8))))/v46v)+(sf[661]*(v2nk/sf[633])))+((-(sf[546]*v3o8))/v47k))}else{v3});
        let v480=(if v1k8{((((-(sf[380]*((v1k9*v2oa)+(vy3*v3o9))))/v46v)+(sf[661]*(v2nn/sf[633])))+((-(sf[546]*v3o9))/v47k))}else{v3});let v481=(if v1k8{((((-(sf[380]*((v1k9*v2oe)+(vy3*v3oa))))/v46v)+(sf[661]*(v2nq/sf[633])))+((-(sf[546]*v3oa))/v47k))}else{v3});let v482=(if v1k8{((((-(sf[380]*((v1k9*v2oi)+(vy3*v3ob))))/v46v)+(sf[661]*(v2nt/sf[633])))+((-(sf[546]*v3ob))/v47k))}else{v3});let v48b=(if v1ki{((v46d-v47z)/vbd)}else{v43o});let v48c=(if v1ki{((v46e-v480)/vbd)}else{v43p});
        let v48d=(if v1ki{((v46f-v481)/vbd)}else{v43q});let v48e=(if v1ki{((v46g-v482)/vbd)}else{v43r});let v49j=(if v1kw{(v47z-(vbd*((v1ky*(-v48b))/v1kz)))}else{(if v1ko{(v46d-(vbd*((v1kp*v48b)/v1kq)))}else{v46d})});let v49k=(if v1kw{(v480-(vbd*((v1ky*(-v48c))/v1kz)))}else{(if v1ko{(v46e-(vbd*((v1kp*v48c)/v1kq)))}else{v46e})});let v49l=(if v1kw{(v481-(vbd*((v1ky*(-v48d))/v1kz)))}else{(if v1ko{(v46f-(vbd*((v1kp*v48d)/v1kq)))}else{v46f})});
        let v49m=(if v1kw{(v482-(vbd*((v1ky*(-v48e))/v1kz)))}else{(if v1ko{(v46g-(vbd*((v1kp*v48e)/v1kq)))}else{v46g})});let v49p=((v1l3*v2o6)+(vy3*v49j));let v49s=((v1l3*v2oa)+(vy3*v49k));let v49v=((v1l3*v2oe)+(vy3*v49l));let v49y=((v1l3*v2oi)+(vy3*v49m));let v4am=(v1l9*v1l9);let v50o=(sf[15]*(sf[0]*(sf[699]*v2xs)));
        let v50s=((((if sb[33]{(sf[661]*((sf[232]*v2qr)+(v112*(sf[230]*v2qr))))}else{(if sb[31]{v2rl}else{(if (sf[149]!=0.0){((v2rl+(v112*(((v110*(sf[822]*v2qr))-(v10w*((vbt*v2qz)/v2rr)))/v2rx)))+(((v118*(v116*v2rh))-(v117*v2rh))/v2sr))}else{v3})})})+(sf[646]*v2w2))+sf[366])-(if v16w{v3}else{(if (v14f!=0.0){(sf[21]*(sf[528]*((v16r*(if v14q{(v14r*v2ya)}else{(if v14m{(v14n*v2ya)}else{v3})}))+(v14v*((v16q*v2fu)+(vuo*(sf[824]*(if v16d{((v16m*((v16e*v30d)+(v15p*sf[341])))+(v16f*((v16k*(v16g*v30d))+(v16h*(v16i*v30d)))))}else{(if v15v{((sf[0]*v169)+(v166*(((v15p*(-(if v160{(v161*v30d)}else{(if v15w{(v15x*v30d)}else{v3})})))-(v167*v30d))/v30s)))}else{v3})}))))))))}else{v3})}));
        let v50t=((((if sb[33]{(sf[661]*((sf[232]*v2qs)+((v11o*v2ku)+(v112*(sf[230]*(v2d7+v2qs))))))}else{(if sb[31]{v2rm}else{(if (sf[149]!=0.0){((v2rm+((v112*(((v110*(sf[822]*v2qs))-(v10w*((vbt*v2r0)/v2rr)))/v2rx))+(v111*v2ku)))+(((v118*((v116*v2ri)+(v10s*(sf[676]*v2d7))))-(v117*v2ri))/v2sr))}else{v3})})})+(sf[646]*v2w4))+sf[367])-(if v16w{v3}else{(if (v14f!=0.0){(sf[21]*(sf[528]*((v16r*(if v14q{(v14r*v2yb)}else{(if v14m{(v14n*v2yb)}else{v3})}))+(v14v*((v16q*v2fv)+(vuo*(sf[824]*(if v16d{((v16m*((v16e*v30e)+(v15p*sf[342])))+(v16f*((v16k*(v16g*v30e))+(v16h*(v16i*v30e)))))}else{(if v15v{((v169*sf[321])+(v166*(((v15p*(-(if v160{(v161*v30e)}else{(if v15w{(v15x*v30e)}else{v3})})))-(v167*v30e))/v30s)))}else{v3})}))))))))}else{v3})}));
        let v51q=(sf[15]*(sf[0]*(-(v19i*v3jr))));let v51r=(sf[15]*(sf[0]*(-(v19i*v3js))));let v51s=(sf[15]*(sf[0]*(-(v19i*v3jt))));let v51t=(sf[15]*(sf[0]*(-(v19i*v3ju))));
        let v51u=(sf[15]*(sf[0]*(-((v1d7*(if v19h{v3}else{(if (v173!=0.0){(sf[53]*(sf[529]*((v19c*(if v17i{(v17j*v333)}else{(if v17e{(v17f*v333)}else{v3})}))+(v17n*((v19b*v32q)+(v177*(sf[825]*(if v190{((v197*((v191*v354)+(v18d*sf[342])))+(v192*((v195*(v16g*v354))+(v193*(v16i*v354)))))}else{(if v18i{((v18w*sf[321])+(v18t*(((v18d*(-(if v18n{(v18o*v354)}else{(if v18j{(v18k*v354)}else{v3})})))-(v18u*v354))/v35j)))}else{v3})}))))))))}else{v3})}))+(v19i*v3jv)))));
        let v51v=(sf[15]*(sf[0]*(-((v1d7*(if v19h{v3}else{(if (v173!=0.0){(sf[53]*(sf[529]*((v19c*(if v17i{(v17j*v334)}else{(if v17e{(v17f*v334)}else{v3})}))+(v17n*((v19b*v32r)+(v177*(sf[825]*(if v190{((v197*((v191*v355)+(v18d*sf[341])))+(v192*((v195*(v16g*v355))+(v193*(v16i*v355)))))}else{(if v18i{((sf[0]*v18w)+(v18t*(((v18d*(-(if v18n{(v18o*v355)}else{(if v18j{(v18k*v355)}else{v3})})))-(v18u*v355))/v35j)))}else{v3})}))))))))}else{v3})}))+(v19i*v3jw)))));let v51w=(sf[15]*(sf[0]*(-(v19i*v3jx))));
        let v51x=(sf[15]*(sf[0]*(-(v19i*v3jy))));let v51y=(sf[15]*(sf[0]*(-(v19i*v3jz))));let v53f=ddt_scale;let v56i=(sf[15]*(v53f*v562));let v57n=(sf[15]*(v53f*v57f));

        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*vo0))),
            5,
            multiplicity * ((sf[15]*(sf[0]*v212))),
            6,
            multiplicity * ((sf[15]*(sf[0]*v213))),
            7,
            multiplicity * ((sf[15]*(sf[0]*v214))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*vy3))),
            [3, 5, 6, 7],
            [(sf[15]*(sf[0]*v2o6)), (sf[15]*(sf[0]*v2oa)), (sf[15]*(sf[0]*v2oe)), (sf[15]*(sf[0]*v2oi))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*((sf[699]*(v148-v1))+((if sb[30]{v12k}else{(if (sf[149]!=0.0){(v12k+(v12m/v12q))}else{v3})})+(sf[693]*(v13i-v1))))))),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(sf[0]*((sf[699]*v2xp)+((if sb[30]{v2uu}else{(if (sf[149]!=0.0){(v2uu+(((v12q*(sf[823]*v2uh))-(v12m*((vbt*(if v12d{(v12e*sf[861])}else{(if v129{(v12a*sf[861])}else{v2qz})}))/v2v3)))/v2va))}else{v3})})+(sf[693]*v2wh))))), (sf[15]*(sf[0]*((sf[699]*v2xq)+((if sb[30]{v2uv}else{(if (sf[149]!=0.0){(v2uv+(((v12q*(sf[823]*v2ui))-(v12m*((vbt*(if v12d{(v12e*sf[860])}else{(if v129{(v12a*sf[860])}else{v3})}))/v2v3)))/v2va))}else{v3})})+(sf[693]*v2wi))))), (sf[15]*(sf[0]*((sf[699]*v2xr)+((if sb[30]{v2uw}else{(if (sf[149]!=0.0){(v2uw+(((v12q*(sf[823]*v2uj))-(v12m*((vbt*(if v12d{v3}else{(if v129{v3}else{v2r0})}))/v2v3)))/v2va))}else{v3})})+(sf[693]*v2wj))))), v50o, v50o, (sf[15]*(sf[0]*(sf[699]*v2xt)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*(sf[0]*((sf[704]*(vyw-v1))+((vzj*vzl)+((((if sb[33]{(sf[661]*((v10t*sf[232])+(v112*v11o)))}else{(if sb[31]{v10u}else{(if (sf[149]!=0.0){((v10u+(v111*v112))+(v117/v118))}else{v3})})})+(sf[646]*(v135-v1)))+(v3*vjl))-(if v16w{v3}else{(if (v14f!=0.0){(sf[21]*(sf[528]*(v14v*v16r)))}else{v3})}))))))),
            [3, 4, 5, 6, 7],
            [(sf[15]*(sf[0]*((sf[704]*v2pd)+(((vzl*(sf[229]*v2q3))+(vzj*((-v2q3)*v2qa)))+v50s)))), (sf[15]*(sf[0]*(sf[646]*v2w3))), (sf[15]*(sf[0]*((sf[704]*v2pe)+(((vzl*(sf[229]*v2q4))+(vzj*((-v2q4)*v2qa)))+v50t)))), (sf[15]*(sf[0]*(if sb[33]{(sf[661]*((v11o*v2kv)+(v112*(sf[230]*v2d8))))}else{(if sb[31]{v3}else{(if (sf[149]!=0.0){((v111*v2kv)+(((v118*((v116*v2rj)+(v10s*(sf[676]*v2d8))))-(v117*v2rj))/v2sr))}else{v3})})}))), (sf[15]*(sf[0]*(if sb[33]{(sf[661]*((v11o*v2kw)+(v112*(sf[230]*v2d9))))}else{(if sb[31]{v3}else{(if (sf[149]!=0.0){((v111*v2kw)+(((v118*((v116*v2rk)+(v10s*(sf[676]*v2d9))))-(v117*v2rk))/v2sr))}else{v3})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * ((if (sf[149]!=0.0){v1v0}else{v3})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if (sf[149]!=0.0){v51q}else{v3}), (if (sf[149]!=0.0){v51r}else{v3}), (if (sf[149]!=0.0){v51s}else{v3}), (if (sf[149]!=0.0){v51t}else{v3}), (if (sf[149]!=0.0){v51u}else{v3}), (if (sf[149]!=0.0){v51v}else{v3}), (if (sf[149]!=0.0){v51w}else{v3}), (if (sf[149]!=0.0){v51x}else{v3}), (if (sf[149]!=0.0){v51y}else{v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * ((if sb[30]{v1v0}else{v3})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if sb[30]{v51q}else{v3}), (if sb[30]{v51r}else{v3}), (if sb[30]{v51s}else{v3}), (if sb[30]{v51t}else{v3}), (if sb[30]{v51u}else{v3}), (if sb[30]{v51v}else{v3}), (if sb[30]{v51w}else{v3}), (if sb[30]{v51x}else{v3}), (if sb[30]{v51y}else{v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * ((sf[15]*(sf[0]*(v1dw/v1dt)))),
            [3, 4, 5, 6, 7],
            [(sf[15]*(sf[0]*((-(v1dw*v3o8))/v3oi))), (sf[15]*(sf[0]*((sf[0]+(sf[795]*(if vln{(vlo*sf[860])}else{(if (vlk!=0.0){(vll*sf[860])}else{v3})})))/v1dt))), (sf[15]*(sf[0]*(((v1dt*(sf[321]+(sf[795]*(if vln{(vlo*sf[861])}else{(if (vlk!=0.0){(vll*sf[861])}else{v3})}))))-(v1dw*v3o9))/v3oi))), (sf[15]*(sf[0]*((-(v1dw*v3oa))/v3oi))), (sf[15]*(sf[0]*((-(v1dw*v3ob))/v3oi)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * ((sf[15]*(sf[0]*(-v1le)))),
            [3, 5, 6, 7],
            [(sf[15]*(sf[0]*(-(if v1ld{v49p}else{(if v1l7{(((v1l9*((v1l4*v47z)+(v1kh*v49p)))-(v1l8*(v47z+v49j)))/v4am)}else{(if v1ki{v49p}else{v3})})})))), (sf[15]*(sf[0]*(-(if v1ld{v49s}else{(if v1l7{(((v1l9*((v1l4*v480)+(v1kh*v49s)))-(v1l8*(v480+v49k)))/v4am)}else{(if v1ki{v49s}else{v3})})})))), (sf[15]*(sf[0]*(-(if v1ld{v49v}else{(if v1l7{(((v1l9*((v1l4*v481)+(v1kh*v49v)))-(v1l8*(v481+v49l)))/v4am)}else{(if v1ki{v49v}else{v3})})})))), (sf[15]*(sf[0]*(-(if v1ld{v49y}else{(if v1l7{(((v1l9*((v1l4*v482)+(v1kh*v49y)))-(v1l8*(v482+v49m)))/v4am)}else{(if v1ki{v49y}else{v3})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((sf[15]*((sf[0]*(sf[0]*(vjt-vjj)))/sf[546]))),
            2,
            multiplicity * (sf[925]),
            3,
            multiplicity * (sf[926]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(4),
            multiplicity * ((sf[15]*((sf[0]*vjy)/sf[554]))),
            1,
            multiplicity * (sf[929]),
            4,
            multiplicity * (sf[930]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((sf[15]*v1ve)),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(v539*v53f)), (sf[15]*(v53a*v53f)), (sf[15]*(v53b*v53f)), (sf[15]*(v53c*v53f)), (sf[15]*(v53d*v53f)), (sf[15]*(v53e*v53f))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((sf[15]*v1vh)),
            3,
            multiplicity * ((sf[15]*(v53f*v53s))),
            4,
            multiplicity * ((sf[15]*(v53f*v53t))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((sf[15]*v1vk)),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(v53f*v53y)), (sf[15]*(v53f*v53z)), (sf[15]*(v53f*v540)), (sf[15]*(v53f*v541)), (sf[15]*(v53f*v542)), (sf[15]*(v53f*v543))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(5),
            multiplicity * ((sf[15]*v1vn)),
            [3, 4, 5, 6, 7, 9],
            [(sf[15]*(v53f*v54g)), (sf[15]*(v53f*v54h)), (sf[15]*(v53f*v54i)), (sf[15]*(v53f*v54j)), (sf[15]*(v53f*v54k)), (sf[15]*(v53f*v54l))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((sf[15]*v1vr)),
            1,
            multiplicity * ((sf[15]*(v53f*sf[372]))),
            2,
            multiplicity * ((sf[15]*(v53f*sf[373]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((sf[15]*v1vv)),
            0,
            multiplicity * ((sf[15]*(v53f*sf[374]))),
            1,
            multiplicity * ((sf[15]*(v53f*sf[375]))),
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((sf[15]*(sf[0]*(v1bp*v1d7)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(sf[15]*(sf[0]*(v3lf+(v1bp*v3jr)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){((v1bn*v3ak)+(v1ak*v3eo))}else{v3}))+(v1bp*v3js)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){(v1ak*v3ep)}else{v3}))+(v1bp*v3jt)))), (sf[15]*(sf[0]*(v3lf+(v1bp*v3ju)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){(v3ev+(v1ak*v3eq))}else{v3}))+(v1bp*v3jv)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){(v3f4+(v1ak*v3er))}else{v3}))+(v1bp*v3jw)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){(v3f4+(v1ak*v3es))}else{v3}))+(v1bp*v3jx)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){((v1bn*v3am)+(v1ak*v3et))}else{v3}))+(v1bp*v3jy)))), (sf[15]*(sf[0]*((v1d7*(if (sf[242]!=0.0){(v3f4+(v1ak*v3eu))}else{v3}))+(v1bp*v3jz))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * ((sf[15]*(sf[784]*(sf[0]*vkg)))),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [sf[935], sf[936], sf[936], sf[936], sf[937], sf[937], sf[938], sf[937]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((sf[15]*v1w3)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [v56i, (sf[15]*(v53f*v563)), (sf[15]*(v53f*v564)), v56i, (sf[15]*(v53f*v565)), (sf[15]*(v53f*v566)), (sf[15]*(v53f*v567)), (sf[15]*(v53f*v568)), (sf[15]*(v53f*v569))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * ((sf[15]*(sf[0]*((v1aa*v1d7)+((v13x*v1d7)+(v3*vkc)))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(sf[15]*(sf[0]*((v1aa*v3jr)+(v13x*v3jr)))), (sf[15]*(sf[0]*((v1aa*v3js)+(v13x*v3js)))), (sf[15]*(sf[0]*((v1aa*v3jt)+((v1d7*(sf[653]*v2x4))+(v13x*v3jt))))), (sf[15]*(sf[0]*(((v1d7*(if (sf[242]!=0.0){(sf[7]*v394)}else{v394}))+(v1aa*v3ju))+(((v1d7*(sf[653]*v2x5))+(v13x*v3ju))+sf[367])))), (sf[15]*(sf[0]*(((v1d7*(if (sf[242]!=0.0){(sf[7]*v398)}else{v398}))+(v1aa*v3jv))+(((v1d7*(sf[653]*v2x6))+(v13x*v3jv))+sf[368])))), (sf[15]*(sf[0]*((v3km+(v1aa*v3jw))+((v3l6+(v13x*v3jw))+sf[369])))), (sf[15]*(sf[0]*((v3km+(v1aa*v3jx))+((v3l6+(v13x*v3jx))+sf[369])))), (sf[15]*(sf[0]*((v1aa*v3jy)+(v13x*v3jy)))), (sf[15]*(sf[0]*(((v1d7*(if (sf[242]!=0.0){(sf[7]*v39g)}else{v39g}))+(v1aa*v3jz))+(((v1d7*(sf[653]*v2x8))+(v13x*v3jz))+sf[366]))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * ((sf[15]*v1w9)),
            [4, 5, 6, 7, 9],
            [(sf[15]*(v53f*v57d)), (sf[15]*(v53f*v57e)), v57n, v57n, (sf[15]*(v53f*v57g))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * ((if (sf[196]!=0.0){(sf[15]*(sf[789]*(sf[0]*vk9)))}else{v3})),
            8,
            multiplicity * (sf[943]),
            9,
            multiplicity * (sf[944]),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v3,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(6),
            multiplicity * ((if (sf[197]!=0.0){(sf[15]*(sf[794]*(sf[0]*vk6)))}else{v3})),
            6,
            multiplicity * (sf[949]),
            9,
            multiplicity * (sf[950]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v3,
        );
        stamper.stamp_current_const_local(
            Some(10),
            None,
            multiplicity * (v3),
        );
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v1wj),
            10,
            multiplicity * (v1),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(3),
            multiplicity * ((v1u6*v1wk)),
            [3, 4, 5, 6, 7, 9, 10],
            [(v1wk*v4yi), (v1wk*v4yj), (v1wk*v4yk), (v1wk*v4yl), (v1wk*v4ym), (v1wk*v4yn), (v1u6*v53f)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((v1ti*v1wj)),
            10,
            multiplicity * (v1ti),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (v1wj),
            10,
            multiplicity * (v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(3),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
            multiplicity * (v3),
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
            v1, v3, vw, vx, v1c, v4g, vbd, vbh, 
            vbt, vcj, vjf, vjj, vjl, vjq, vjt, vjy, 
            vk6, vk9, vkc, vkg, vlh, vli, vlk, vln, 
            vlo, vo0, vre, vt2, vtr, vtu, vtx, vuo, 
            vww, vxw, vxx, vy2, vy3, vym, vyo, vyr, 
            vys, vz1, vzx, vzz, v101, v106, v107, v10e, 
            v10f, v10h, v10m, v10o, v124, v126, v128, v12d, 
            v12e, v135, v13i, v13v, v148, v14f, v14g, v14j, 
            v14l, v14q, v14r, v14x, v151, v154, v15c, v15d, 
            v15e, v15g, v15i, v15m, v15n, v15p, v15s, v15u, 
            v15v, v160, v161, v173, v175, v177, v178, v17b, 
            v17d, v17i, v17j, v17o, v17r, v17t, v181, v182, 
            v183, v185, v18a, v18b, v18d, v18f, v18h, v18i, 
            v18n, v18o, v1ak, v1b1, v1bn, v1dn, v1dz, v1ec, 
            v1ed, v1ee, v1eh, v1ei, v1em, v1en, v1ep, v1et, 
            v1ev, v1f0, v1f1, v1fg, v1if, v1ig, v1ii, v1ik, 
            v1im, v1io, v1ip, v1ir, v1iz, v1j2, v1j3, v1j4, 
            v1ja, v1jc, v1jd, v1jh, v1jj, v1jm, v1jo, v1jt, 
            v1ju, v1ta, v1u6, v1vd, v1vg, v1vj, v1vm, v1vq, 
            v1vu, v1w2, v1w8, v1wj, v1xp, v1xq, v1xr, v1xs, 
            v212, v213, v214, v293, v294, v295, v2d7, v2d8, 
            v2d9, v2ee, v2ef, v2eg, v2en, v2eo, v2ep, v2ew, 
            v2ex, v2ey, v2fu, v2fv, v2ku, v2kv, v2kw, v2ne, 
            v2nf, v2ng, v2nh, v2nk, v2nn, v2nq, v2nt, v2nu, 
            v2nv, v2nw, v2ny, v2o2, v2o5, v2p3, v2p4, v2qr, 
            v2qs, v2uh, v2ui, v2uj, v2w2, v2w3, v2w4, v2wh, 
            v2wi, v2wj, v2x4, v2x5, v2x6, v2x7, v2x8, v2xp, 
            v2xq, v2xr, v2xs, v2xt, v3aj, v3ak, v3al, v3am, 
            v3az, v3b0, v3b1, v3b2, v3b3, v3b4, v3b5, v3b6, 
            v3en, v3eo, v3ep, v3eq, v3er, v3es, v3et, v3eu, 
            v3nb, v3nc, v3nd, v3ne, v4yi, v4yj, v4yk, v4yl, 
            v4ym, v4yn, v539, v53a, v53b, v53c, v53d, v53e, 
            v53s, v53t, v53y, v53z, v540, v541, v542, v543, 
            v54g, v54h, v54i, v54j, v54k, v54l, v562, v563, 
            v564, v565, v566, v567, v568, v569, v57d, v57e, 
            v57f, v57g, 
        }=self.eval_common_stamp_values(ctx);
        let v1ve=0.0;let v1vh=0.0;let v1vk=0.0;let v1vn=0.0;let v1vr=0.0;let v1vv=0.0;let v1w3=0.0;let v1w9=0.0;let v1wk=0.0;let v53f=1.0;let v56i=(sf[15]*(v53f*v562));let v57n=(sf[15]*(v53f*v57f));

        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(sf[15]*(v539*v53f)), (sf[15]*(v53a*v53f)), (sf[15]*(v53b*v53f)), (sf[15]*(v53c*v53f)), (sf[15]*(v53d*v53f)), (sf[15]*(v53e*v53f))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * ((sf[15]*(v53f*v53s))),
            nodes[4],
            multiplicity * ((sf[15]*(v53f*v53t))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(sf[15]*(v53f*v53y)), (sf[15]*(v53f*v53z)), (sf[15]*(v53f*v540)), (sf[15]*(v53f*v541)), (sf[15]*(v53f*v542)), (sf[15]*(v53f*v543))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(sf[15]*(v53f*v54g)), (sf[15]*(v53f*v54h)), (sf[15]*(v53f*v54i)), (sf[15]*(v53f*v54j)), (sf[15]*(v53f*v54k)), (sf[15]*(v53f*v54l))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((sf[15]*(v53f*sf[372]))),
            nodes[2],
            multiplicity * ((sf[15]*(v53f*sf[373]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((sf[15]*(v53f*sf[374]))),
            nodes[1],
            multiplicity * ((sf[15]*(v53f*sf[375]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[v56i, (sf[15]*(v53f*v563)), (sf[15]*(v53f*v564)), v56i, (sf[15]*(v53f*v565)), (sf[15]*(v53f*v566)), (sf[15]*(v53f*v567)), (sf[15]*(v53f*v568)), (sf[15]*(v53f*v569))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(sf[15]*(v53f*v57d)), (sf[15]*(v53f*v57e)), v57n, v57n, (sf[15]*(v53f*v57g))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9], nodes[10]],
            &[(v1wk*v4yi), (v1wk*v4yj), (v1wk*v4yk), (v1wk*v4yl), (v1wk*v4ym), (v1wk*v4yn), (v1u6*v53f)],
            &[],
            &[],
            multiplicity,
        );
    }
}
