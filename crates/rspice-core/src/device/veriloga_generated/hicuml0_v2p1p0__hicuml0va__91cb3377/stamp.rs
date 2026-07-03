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
    v1: f64, v2: f64, v4: f64, v5: f64, v6: f64, v7: f64, 
    v8: f64, v9: f64, va: f64, vb: f64, ve: f64, vf: f64, 
    vw: f64, v1c: f64, v1o: f64, v2m: f64, vc7: f64, vcm: f64, 
    vco: f64, vcq: f64, vcu: f64, vcx: f64, vfx: f64, vg3: f64, 
    vlr: f64, vly: bool, vm0: f64, vm3: f64, vmb: f64, vmi: f64, 
    vmm: f64, vmp: bool, vmr: f64, vms: f64, vn0: bool, vnd: bool, 
    vnf: f64, vng: f64, vnu: bool, vo2: f64, vo6: f64, vpl: bool, 
    vpu: f64, vpx: f64, vq6: f64, vqp: f64, vqr: f64, vqy: bool, 
    vr0: f64, vr9: f64, vrg: f64, vrm: bool, vro: f64, vrp: f64, 
    vrx: bool, vs8: bool, vsa: f64, vsb: f64, vsp: bool, vsx: f64, 
    vt1: f64, vug: bool, vuo: f64, vur: f64, vv0: f64, vvj: f64, 
    vvm: bool, vvn: f64, vvr: f64, vvw: f64, vw2: bool, vw4: f64, 
    vw5: f64, vwd: bool, vwo: bool, vwq: f64, vwr: f64, vx5: bool, 
    vxd: f64, vxh: f64, vys: bool, vz0: f64, vz3: f64, vzc: f64, 
    vzw: bool, vzx: f64, v101: f64, v106: f64, v10c: bool, v10e: f64, 
    v10f: f64, v10n: bool, v10y: bool, v110: f64, v111: f64, v11f: bool, 
    v11n: f64, v11r: f64, v132: bool, v13a: f64, v13d: f64, v13m: f64, 
    v165: f64, v167: f64, v16l: f64, v16o: f64, v16x: f64, v17h: f64, 
    v17k: bool, v17y: f64, v181: f64, v18a: f64, v1b6: f64, v1db: f64, 
    v1hu: f64, v1hx: f64, v1io: f64, v1kr: f64, v1ks: bool, v1kt: f64, 
    v1kx: f64, v1l2: f64, v1l8: bool, v1la: f64, v1lb: f64, v1lj: bool, 
    v1lu: bool, v1lw: f64, v1lx: f64, v1mb: bool, v1mj: f64, v1mn: f64, 
    v1nv: bool, v1o3: f64, v1o6: f64, v1of: f64, v1wi: f64, v1wj: f64, 
    v1wq: f64, v1wr: f64, v1x0: f64, v1x2: f64, v1xb: f64, v1xc: f64, 
    v1xd: f64, v1xe: f64, v1xg: f64, v1xi: f64, v1y1: f64, v1yt: f64, 
    v1yx: f64, v1yy: f64, v1z2: f64, v1z6: f64, v21z: f64, v228: f64, 
    v27o: f64, v27u: f64, v284: f64, v28g: f64, v28h: f64, v28i: f64, 
    v2a1: f64, v2a2: f64, v2a3: f64, v2bt: f64, v2bu: f64, v2bv: f64, 
    v2c8: f64, v2c9: f64, v2ca: f64, v2h2: f64, v2h3: f64, v2h4: f64, 
    v2hb: f64, v2hc: f64, v2hd: f64, v2ih: f64, v2ii: f64, v2ij: f64, 
    v2k6: f64, v2k8: f64, v2ke: f64, v2ko: f64, v2l0: f64, v2l1: f64, 
    v2l2: f64, v2l3: f64, v2mz: f64, v2n0: f64, v2n1: f64, v2n2: f64, 
    v2p7: f64, v2p8: f64, v2p9: f64, v2pa: f64, v2pq: f64, v2pr: f64, 
    v2ps: f64, v2pt: f64, v2vz: f64, v2w0: f64, v2w1: f64, v2w2: f64, 
    v2wb: f64, v2wc: f64, v2wd: f64, v2we: f64, v2xt: f64, v2xu: f64, 
    v2xv: f64, v2xw: f64, v302: f64, v306: f64, v30c: f64, v30o: f64, 
    v30p: f64, v30q: f64, v30r: f64, v32n: f64, v32o: f64, v32p: f64, 
    v32q: f64, v34v: f64, v34w: f64, v34x: f64, v34y: f64, v35e: f64, 
    v35f: f64, v35g: f64, v35h: f64, v3bn: f64, v3bo: f64, v3bp: f64, 
    v3bq: f64, v3bz: f64, v3c0: f64, v3c1: f64, v3c2: f64, v3dh: f64, 
    v3di: f64, v3dj: f64, v3dk: f64, v3fs: f64, v3fy: f64, v3ga: f64, 
    v3gb: f64, v3gc: f64, v3gd: f64, v3i9: f64, v3ia: f64, v3ib: f64, 
    v3ic: f64, v3kh: f64, v3ki: f64, v3kj: f64, v3kk: f64, v3l0: f64, 
    v3l1: f64, v3l2: f64, v3l3: f64, v3r9: f64, v3ra: f64, v3rb: f64, 
    v3rc: f64, v3rl: f64, v3rm: f64, v3rn: f64, v3ro: f64, v3t3: f64, 
    v3t4: f64, v3t5: f64, v3t6: f64, v42v: f64, v42w: f64, v42x: f64, 
    v42y: f64, v43w: f64, v43x: f64, v43y: f64, v43z: f64, v440: f64, 
    v44b: f64, v44c: f64, v44d: f64, v44e: f64, v44f: f64, v465: f64, 
    v466: f64, v467: f64, v468: f64, v469: f64, v48y: f64, v493: f64, 
    v494: f64, v495: f64, v496: f64, v4a4: f64, v4a5: f64, v4a6: f64, 
    v4a7: f64, v4a8: f64, v4aj: f64, v4ak: f64, v4al: f64, v4am: f64, 
    v4an: f64, v4cd: f64, v4ce: f64, v4cf: f64, v4cg: f64, v4ch: f64, 
    v4la: f64, v4lb: f64, v4lc: f64, v4nc: f64, v5de: f64, v5di: f64, 
    v5dm: f64, v5dq: f64, v5dt: f64, v5du: f64, v5dv: f64, v5dw: f64, 
    v5dx: f64, v5dy: f64, v5if: f64, v5ig: f64, v5ih: f64, v5ii: f64, 
    v5ij: f64, v5mu: f64, v5n0: f64, v5nc: f64, v5nd: f64, v5ne: f64, 
    v5nf: f64, v5pb: f64, v5pc: f64, v5pd: f64, v5pe: f64, v5rj: f64, 
    v5rk: f64, v5rl: f64, v5rm: f64, v5s2: f64, v5s3: f64, v5s4: f64, 
    v5s5: f64, v5xz: f64, v5y0: f64, v5y1: f64, v5y2: f64, v5y3: f64, 
    v5ye: f64, v5yf: f64, v5yg: f64, v5yh: f64, v5yi: f64, v608: f64, 
    v609: f64, v60a: f64, v60b: f64, v60c: f64, v6ux: f64, v6uy: f64, 
    v6uz: f64, v6v0: f64, v6v1: f64, v6vw: f64, v6vx: f64, v6vy: f64, 
    v6vz: f64, v6w0: f64, v6wz: f64, v6x0: f64, v6x1: f64, v6x2: f64, 
    v6x3: f64, v6xa: f64, v6xb: f64, v6xc: f64, v6xd: f64, v6xe: f64, 
    v6y3: f64, v6y4: f64, v6y5: f64, v6y6: f64, v6y7: f64, v6y8: f64, 
    v6y9: f64, v6ya: f64, v6yb: f64, v6yc: f64, v6ym: f64, v6yn: f64, 
    v6yo: f64, v6yp: f64, v6yq: f64, v6yu: f64, v6yv: f64, v6yw: f64, 
    v6yx: f64, v6yy: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v1=ctx.node_voltage(nodes[1]);
        let v2=ctx.node_voltage(nodes[5]);
        let v4=(sf[0]*(v1-v2));
        let v5=ctx.node_voltage(nodes[6]);
        let v6=(v5-v2);
        let v7=(sf[0]*v6);
        let v8=ctx.node_voltage(nodes[7]);
        let v9=(v5-v8);
        let va=(sf[0]*v9);
        let vb=(va-v7);
        let ve=(sf[0]*(ctx.node_voltage(nodes[3])-v2));
        let vf=ctx.node_voltage(nodes[2]);
        let vq=1.3806226e-23;
        let vs=1.602176462e-19;
        let vw=0.5;
        let v17=3.0;
        let v1c=1.0;
        let v1o=0.0;
        let v21=173.14999999999998;
        let v25=600.0;
        let v2m=2.0;
        let v32=4.0;
        let vc7=ctx.node_voltage(nodes[4]);
        let vc9=(if (sf[148]!=0.0){(sf[303]+vc7)}else{sf[307]});
        let vcb=(if (vc9<v21){v1c}else{v1o});
        let vcc=((sf[148]!=0.0)&&(vcb!=0.0));
        let vcd=(if vcc{v21}else{vc9});
        let vci=(((if (vcd>v25){v1c}else{v1o})!=0.0)&&((sf[148]!=0.0)&&(!(vcb!=0.0))));
        let vcj=(if vci{v25}else{vcd});
        let vcm=(if (sf[148]!=0.0){((vq*vcj)/vs)}else{sf[309]});
        let vco=(if (sf[148]!=0.0){(v1c/vcm)}else{sf[310]});
        let vcq=(if (sf[148]!=0.0){(vcj-sf[2])}else{sf[311]});
        let vcs=(if (sf[148]!=0.0){(vcj/sf[2])}else{sf[312]});
        let vcu=(if (sf[148]!=0.0){(vcs).ln()}else{sf[313]});
        let vcv=(vcs-v1c);
        let vcx=(if (sf[148]!=0.0){(vco*vcv)}else{sf[315]});
        let vd7=(v1c-vcs);
        let vd8=(sf[10]*vd7);
        let vda=(sf[20]*vcm);
        let vdb=(vcu*vda);
        let vdd=(if (sf[148]!=0.0){(((vcs*sf[156])+vd8)-vdb)}else{sf[476]});
        let vde=(v2m*vcm);
        let vdf=(-vdd);
        let vdh=((vco*vdf)).exp();
        let vdk=((v1c+(v32*vdh))).sqrt();
        let vdm=(vw*(v1c+vdk));
        let vdn=(vdm).ln();
        let vdq=(if (sf[148]!=0.0){(vdd+(vde*vdn))}else{sf[334]});
        let vdr=(sf[37]/vdq);
        let vdu=((sf[47]*(vdr).ln())).exp();
        let vdw=(if (sf[148]!=0.0){(sf[30]*vdu)}else{sf[339]});
        let vdz=(if (sf[148]!=0.0){((sf[48]*vdq)/sf[37])}else{sf[341]});
        let veb=(if (sf[148]!=0.0){((vd8+(vcs*sf[164]))-vdb)}else{vdd});
        let vec=(-veb);
        let vee=((vco*vec)).exp();
        let veh=((v1c+(v32*vee))).sqrt();
        let vej=(vw*(v1c+veh));
        let vek=(vej).ln();
        let ven=(if (sf[148]!=0.0){(veb+(vde*vek))}else{sf[355]});
        let veo=(sf[49]/ven);
        let ver=((sf[58]*(veo).ln())).exp();
        let vet=(if (sf[148]!=0.0){(sf[30]*ver)}else{sf[360]});
        let vew=(if (sf[148]!=0.0){((sf[59]*ven)/sf[49])}else{sf[362]});
        let vfi=(sf[13]*vd7);
        let vfl=(if (sf[148]!=0.0){(((vcs*sf[172])+vfi)-vdb)}else{veb});
        let vfm=(-vfl);
        let vfo=((vco*vfm)).exp();
        let vfr=((v1c+(v32*vfo))).sqrt();
        let vft=(vw*(v1c+vfr));
        let vfu=(vft).ln();
        let vfx=(if (sf[148]!=0.0){(vfl+(vde*vfu))}else{sf[387]});
        let vfy=(sf[64]/vfx);
        let vg1=((sf[73]*(vfy).ln())).exp();
        let vg3=(if (sf[148]!=0.0){(sf[32]*vg1)}else{sf[392]});
        let vgd=(((sf[26]*vcu)+(sf[7]*vcx))).exp();
        let vgf=(if (sf[148]!=0.0){(sf[75]*vgd)}else{sf[402]});
        let vgj=(((sf[77]*vcu)-(sf[78]*vcx))).exp();
        let vgl=(if (sf[148]!=0.0){(sf[76]*vgj)}else{sf[407]});
        let vgn=((sf[80]*vcu)).exp();
        let vgp=(if (sf[148]!=0.0){(sf[79]*vgn)}else{sf[410]});
        let vgr=((sf[22]*vcu)).exp();
        let vgt=(if (sf[148]!=0.0){(sf[81]*vgr)}else{sf[413]});
        let vgv=(if (sf[148]!=0.0){(v1c/vgt)}else{sf[414]});
        let vgy=(sf[82]*(v1c+(sf[83]*vcq)));
        let vhb=(sf[89]*vcq);
        let vhf=(if (sf[148]!=0.0){(sf[87]*((v1c+(sf[88]*vcq))+(vcq*vhb)))}else{sf[430]});
        let vhi=(sf[29]*vcx);
        let vhk=(((sf[28]*vcu)-vhi)).exp();
        let vho=(if sb[18]{sf[92]}else{(if sb[17]{(sf[92]*vhk)}else{sf[437]})});
        let vhq=((sf[94]*vcu)).exp();
        let vhs=(if (sf[148]!=0.0){(sf[93]*vhq)}else{sf[440]});
        let vik=(if (sf[148]!=0.0){((vfi+(vcs*sf[180]))-vdb)}else{vfl});
        let vil=(-vik);
        let vin=((vco*vil)).exp();
        let viq=((v1c+(v32*vin))).sqrt();
        let vis=(vw*(v1c+viq));
        let vit=(vis).ln();
        let viw=(if (sf[148]!=0.0){(vik+(vde*vit))}else{sf[467]});
        let vix=(sf[101]/viw);
        let vj0=((sf[111]*(vix).ln())).exp();
        let vj2=(if (sf[148]!=0.0){(sf[110]*vj0)}else{sf[472]});
        let vjf=(if (sf[148]!=0.0){(((vcs*sf[188])+(sf[16]*vd7))-vdb)}else{vik});
        let vjg=(-vjf);
        let vji=((vco*vjg)).exp();
        let vjl=((v1c+(v32*vji))).sqrt();
        let vjn=(vw*(v1c+vjl));
        let vjo=(vjn).ln();
        let vjr=(if (sf[148]!=0.0){(vjf+(vde*vjo))}else{sf[487]});
        let vjs=(sf[112]/vjr);
        let vjv=((sf[122]*(vjs).ln())).exp();
        let vjx=(if (sf[148]!=0.0){(sf[121]*vjv)}else{sf[492]});
        let vk9=((sf[126]*vcu)).exp();
        let vkb=(if (sf[148]!=0.0){(sf[125]*vk9)}else{sf[503]});
        let vkc=(sf[78]*vco);
        let vke=((sf[128]*vcu)).exp();
        let vkf=(vke-v1c);
        let vkh=((vkc*vkf)).exp();
        let vkj=(if (sf[148]!=0.0){(sf[127]/vkh)}else{sf[510]});
        let vkm=(sf[131]+(sf[132]*vcq));
        let vks=((sf[133]*vcu)).exp();
        let vkt=(if sb[22]{vks}else{(if sb[21]{(v1c+(vcq*vkm))}else{sf[518]})});
        let vkv=(if (sf[148]!=0.0){(sf[134]*vkt)}else{sf[519]});
        let vkw=(sf[135]*vkt);
        let vkx=(vhi).exp();
        let vkz=(if (sf[148]!=0.0){(vkw*vkx)}else{sf[522]});
        let vll=(if (vj2<=1e-30){v1c}else{v1o});
        let vlr=(if (vll!=0.0){(vg3*sf[190])}else{v1o});
        let vlw=(if (vlr>v1o){v1c}else{v1o});
        let vlx=((vll!=0.0)&&(sf[192]!=0.0));
        let vly=((vlw!=0.0)&&vlx);
        let vm0=(if vly{sf[193]}else{v1o});
        let vm1=(sf[191]-vfx);
        let vm2=(if vly{vm1}else{v1o});
        let vm3=2.4;
        let vm8=(vfx*sf[196]);
        let vm9=(if vly{vm8}else{v1o});
        let vmb=(if vly{(vlr*vm3)}else{v1o});
        let vmc=(vm0-sf[73]);
        let vmd=(sf[191]/vfx);
        let vme=(vmd).ln();
        let vmg=((vmc*vme)).exp();
        let vmi=(if vly{(vlr*vmg)}else{v1o});
        let vmj=(vm9-v4);
        let vml=(if vly{(vco*vmj)}else{v1o});
        let vmm=80.0;
        let vmo=(if (vml<vmm){v1c}else{v1o});
        let vmp=(vly&&(vmo!=0.0));
        let vmq=(vml).exp();
        let vmr=(if vmp{vmq}else{v1o});
        let vms=(v1c+vmr);
        let vmv=(vms).ln();
        let vn0=(vly&&(!(vmo!=0.0)));
        let vn2=(if vn0{v4}else{(if vmp{(vm9-(vcm*vmv))}else{v1o})});
        let vn3=0.1;
        let vn5=(v32*vcm);
        let vn7=(if vly{((vm2*vn3)+vn5)}else{v1o});
        let vn8=(vm2+vn2);
        let vna=(if vly{(vn8/vn7)}else{v1o});
        let vnc=(if (vna<vmm){v1c}else{v1o});
        let vnd=(vly&&(vnc!=0.0));
        let vne=(vna).exp();
        let vnf=(if vnd{vne}else{vmr});
        let vng=(v1c+vnf);
        let vnm=(-(vm2+vm9));
        let vno=((vnm/vn7)).exp();
        let vnp=((vng).ln()-vno);
        let vnu=(vly&&(!(vnc!=0.0)));
        let vnw=(if vnu{vn2}else{(if vnd{((-vm2)+(vn7*vnp))}else{v1o})});
        let vny=(if vly{(v4-vn2)}else{v1o});
        let vo0=(v1c-(vn2/vfx));
        let vo2=(if vly{(vo0).ln()}else{v1o});
        let vo4=(v1c-(vnw/vfx));
        let vo6=(if vly{(vo4).ln()}else{v1o});
        let vo8=(if vly{sf[197]}else{v1o});
        let voa=(if vly{(v1c-vm0)}else{v1o});
        let vot=((vo6*vo8)).exp();
        let vou=(v1c-vot);
        let vox=(if vly{((vlr*vou)/vo8)}else{v1o});
        let voz=((vo2*voa)).exp();
        let vp0=(v1c-voz);
        let vp3=(if vly{((vmi*vp0)/voa)}else{v1o});
        let vp5=((vo6*voa)).exp();
        let vp6=(v1c-vp5);
        let vp9=(if vly{((vmi*vp6)/voa)}else{v1o});
        let vpb=((vox+vp3)-vp9);
        let vpg=(!(vlw!=0.0));
        let vph=(vlx&&vpg);
        let vpk=((vll!=0.0)&&sb[24]);
        let vpl=((vlw!=0.0)&&vpk);
        let vpm=(if vpl{vm8}else{v1o});
        let vpn=(vpm-v4);
        let vpp=(if vpl{(vco*vpn)}else{v1o});
        let vpr=1.921812;
        let vpt=(((vpp*vpp)+vpr)).sqrt();
        let vpu=(if vpl{vpt}else{v1o});
        let vpx=(if vpl{(vw*(vpp+vpu))}else{v1o});
        let vq0=(if vpl{(vpm-(vcm*vpx))}else{v1o});
        let vq4=(v1c-(vq0/vfx));
        let vq6=(if vpl{(vq4).ln()}else{v1o});
        let vqc=((sf[197]*vq6)).exp();
        let vqd=(v1c-vqc);
        let vqg=(if vpl{((vfx*vqd)/sf[197])}else{v1o});
        let vqj=(vqg+(vm3*(v4-vq0)));
        let vqm=(vpg&&vpk);
        let vqo=(!(vll!=0.0));
        let vqp=(if vqo{vg3}else{(if (vll!=0.0){(vg3*sf[189])}else{v1o})});
        let vqr=(if vqo{(vj2*sf[189])}else{v1o});
        let vqw=(if (vqr>v1o){v1c}else{v1o});
        let vqx=(vqo&&(sf[200]!=0.0));
        let vqy=((vqw!=0.0)&&vqx);
        let vr0=(if vqy{sf[201]}else{vm0});
        let vr1=(sf[199]-viw);
        let vr2=(if vqy{vr1}else{vm2});
        let vr6=(viw*sf[204]);
        let vr7=(if vqy{vr6}else{vm9});
        let vr9=(if vqy{(vm3*vqr)}else{vmb});
        let vra=(vr0-sf[111]);
        let vrb=(sf[199]/viw);
        let vrc=(vrb).ln();
        let vre=((vra*vrc)).exp();
        let vrg=(if vqy{(vqr*vre)}else{vmi});
        let vrh=(vr7-v7);
        let vrj=(if vqy{(vco*vrh)}else{vml});
        let vrl=(if (vrj<vmm){v1c}else{v1o});
        let vrm=(vqy&&(vrl!=0.0));
        let vrn=(vrj).exp();
        let vro=(if vrm{vrn}else{vnf});
        let vrp=(v1c+vro);
        let vrs=(vrp).ln();
        let vrx=(vqy&&(!(vrl!=0.0)));
        let vrz=(if vrx{v7}else{(if vrm{(vr7-(vcm*vrs))}else{vn2})});
        let vs2=(if vqy{(vn5+(vn3*vr2))}else{vn7});
        let vs3=(vr2+vrz);
        let vs5=(if vqy{(vs3/vs2)}else{vna});
        let vs7=(if (vs5<vmm){v1c}else{v1o});
        let vs8=(vqy&&(vs7!=0.0));
        let vs9=(vs5).exp();
        let vsa=(if vs8{vs9}else{vro});
        let vsb=(v1c+vsa);
        let vsh=(-(vr2+vr7));
        let vsj=((vsh/vs2)).exp();
        let vsk=((vsb).ln()-vsj);
        let vsp=(vqy&&(!(vs7!=0.0)));
        let vsr=(if vsp{vrz}else{(if vs8{((-vr2)+(vs2*vsk))}else{vnw})});
        let vst=(if vqy{(v7-vrz)}else{vny});
        let vsv=(v1c-(vrz/viw));
        let vsx=(if vqy{(vsv).ln()}else{vo2});
        let vsz=(v1c-(vsr/viw));
        let vt1=(if vqy{(vsz).ln()}else{vo6});
        let vt3=(if vqy{sf[205]}else{vo8});
        let vt5=(if vqy{(v1c-vr0)}else{voa});
        let vto=((vt1*vt3)).exp();
        let vtp=(v1c-vto);
        let vts=(if vqy{((vqr*vtp)/vt3)}else{vox});
        let vtu=((vsx*vt5)).exp();
        let vtv=(v1c-vtu);
        let vty=(if vqy{((vrg*vtv)/vt5)}else{vp3});
        let vu0=((vt1*vt5)).exp();
        let vu1=(v1c-vu0);
        let vu4=(if vqy{((vrg*vu1)/vt5)}else{vp9});
        let vu6=((vts+vty)-vu4);
        let vub=(!(vqw!=0.0));
        let vuc=(vqx&&vub);
        let vuf=(vqo&&sb[26]);
        let vug=((vqw!=0.0)&&vuf);
        let vuh=(if vug{vr6}else{vpm});
        let vui=(vuh-v7);
        let vuk=(if vug{(vco*vui)}else{vpp});
        let vun=((vpr+(vuk*vuk))).sqrt();
        let vuo=(if vug{vun}else{vpu});
        let vur=(if vug{(vw*(vuk+vuo))}else{vpx});
        let vuu=(if vug{(vuh-(vcm*vur))}else{vq0});
        let vuy=(v1c-(vuu/viw));
        let vv0=(if vug{(vuy).ln()}else{vq6});
        let vv6=((sf[205]*vv0)).exp();
        let vv7=(v1c-vv6);
        let vva=(if vug{((viw*vv7)/sf[205])}else{vqg});
        let vvd=(vva+(vm3*(v7-vuu)));
        let vvg=(vub&&vuf);
        let vvj=(if vqo{(vj2*sf[190])}else{vlr});
        let vvl=(if (vvj>v1o){v1c}else{v1o});
        let vvm=(vqx&&(vvl!=0.0));
        let vvn=(if vvm{sf[201]}else{vr0});
        let vvo=(if vvm{vr1}else{vr2});
        let vvp=(if vvm{vr6}else{vr7});
        let vvr=(if vvm{(vm3*vvj)}else{vr9});
        let vvs=(vvn-sf[111]);
        let vvu=((vrc*vvs)).exp();
        let vvw=(if vvm{(vvj*vvu)}else{vrg});
        let vvx=(vvp-v4);
        let vvz=(if vvm{(vco*vvx)}else{vrj});
        let vw1=(if (vvz<vmm){v1c}else{v1o});
        let vw2=(vvm&&(vw1!=0.0));
        let vw3=(vvz).exp();
        let vw4=(if vw2{vw3}else{vsa});
        let vw5=(v1c+vw4);
        let vw8=(vw5).ln();
        let vwd=(vvm&&(!(vw1!=0.0)));
        let vwf=(if vwd{v4}else{(if vw2{(vvp-(vcm*vw8))}else{vrz})});
        let vwi=(if vvm{(vn5+(vn3*vvo))}else{vs2});
        let vwj=(vvo+vwf);
        let vwl=(if vvm{(vwj/vwi)}else{vs5});
        let vwn=(if (vwl<vmm){v1c}else{v1o});
        let vwo=(vvm&&(vwn!=0.0));
        let vwp=(vwl).exp();
        let vwq=(if vwo{vwp}else{vw4});
        let vwr=(v1c+vwq);
        let vwx=(-(vvo+vvp));
        let vwz=((vwx/vwi)).exp();
        let vx0=((vwr).ln()-vwz);
        let vx5=(vvm&&(!(vwn!=0.0)));
        let vx7=(if vx5{vwf}else{(if vwo{((-vvo)+(vwi*vx0))}else{vsr})});
        let vx9=(if vvm{(v4-vwf)}else{vst});
        let vxb=(v1c-(vwf/viw));
        let vxd=(if vvm{(vxb).ln()}else{vsx});
        let vxf=(v1c-(vx7/viw));
        let vxh=(if vvm{(vxf).ln()}else{vt1});
        let vxi=(if vvm{sf[205]}else{vt3});
        let vxk=(if vvm{(v1c-vvn)}else{vt5});
        let vy2=((vxh*vxi)).exp();
        let vy3=(v1c-vy2);
        let vy6=(if vvm{((vvj*vy3)/vxi)}else{vts});
        let vy8=((vxd*vxk)).exp();
        let vy9=(v1c-vy8);
        let vyc=(if vvm{((vvw*vy9)/vxk)}else{vty});
        let vye=((vxh*vxk)).exp();
        let vyf=(v1c-vye);
        let vyi=(if vvm{((vvw*vyf)/vxk)}else{vu4});
        let vyk=((vy6+vyc)-vyi);
        let vyp=(!(vvl!=0.0));
        let vyq=(vqx&&vyp);
        let vys=(vuf&&(vvl!=0.0));
        let vyt=(if vys{vr6}else{vuh});
        let vyu=(vyt-v4);
        let vyw=(if vys{(vco*vyu)}else{vuk});
        let vyz=((vpr+(vyw*vyw))).sqrt();
        let vz0=(if vys{vyz}else{vuo});
        let vz3=(if vys{(vw*(vyw+vz0))}else{vur});
        let vz6=(if vys{(vyt-(vcm*vz3))}else{vuu});
        let vza=(v1c-(vz6/viw));
        let vzc=(if vys{(vza).ln()}else{vv0});
        let vzi=((sf[205]*vzc)).exp();
        let vzj=(v1c-vzi);
        let vzm=(if vys{((viw*vzj)/sf[205])}else{vva});
        let vzp=(vzm+(vm3*(v4-vz6)));
        let vzs=(vuf&&vyp);
        let vzu=(vqp>v1o);
        let vzv=(if vzu{v1c}else{v1o});
        let vzw=((sf[192]!=0.0)&&(vzv!=0.0));
        let vzx=(if vzw{sf[193]}else{vvn});
        let vzy=(if vzw{vm1}else{vvo});
        let vzz=(if vzw{vm8}else{vvp});
        let v100=(vm3*vqp);
        let v101=(if vzw{v100}else{vvr});
        let v102=(vzx-sf[73]);
        let v104=((vme*v102)).exp();
        let v106=(if vzw{(vqp*v104)}else{vvw});
        let v107=(vzz-v7);
        let v109=(if vzw{(vco*v107)}else{vvz});
        let v10b=(if (v109<vmm){v1c}else{v1o});
        let v10c=(vzw&&(v10b!=0.0));
        let v10d=(v109).exp();
        let v10e=(if v10c{v10d}else{vwq});
        let v10f=(v1c+v10e);
        let v10i=(v10f).ln();
        let v10n=(vzw&&(!(v10b!=0.0)));
        let v10p=(if v10n{v7}else{(if v10c{(vzz-(vcm*v10i))}else{vwf})});
        let v10s=(if vzw{(vn5+(vn3*vzy))}else{vwi});
        let v10t=(vzy+v10p);
        let v10v=(if vzw{(v10t/v10s)}else{vwl});
        let v10x=(if (v10v<vmm){v1c}else{v1o});
        let v10y=(vzw&&(v10x!=0.0));
        let v10z=(v10v).exp();
        let v110=(if v10y{v10z}else{v10e});
        let v111=(v1c+v110);
        let v117=(-(vzy+vzz));
        let v119=((v117/v10s)).exp();
        let v11a=((v111).ln()-v119);
        let v11f=(vzw&&(!(v10x!=0.0)));
        let v11h=(if v11f{v10p}else{(if v10y{((-vzy)+(v10s*v11a))}else{vx7})});
        let v11j=(if vzw{(v7-v10p)}else{vx9});
        let v11l=(v1c-(v10p/vfx));
        let v11n=(if vzw{(v11l).ln()}else{vxd});
        let v11p=(v1c-(v11h/vfx));
        let v11r=(if vzw{(v11p).ln()}else{vxh});
        let v11s=(if vzw{sf[197]}else{vxi});
        let v11u=(if vzw{(v1c-vzx)}else{vxk});
        let v12c=((v11r*v11s)).exp();
        let v12d=(v1c-v12c);
        let v12g=(if vzw{((vqp*v12d)/v11s)}else{vy6});
        let v12i=((v11n*v11u)).exp();
        let v12j=(v1c-v12i);
        let v12m=(if vzw{((v106*v12j)/v11u)}else{vyc});
        let v12o=((v11r*v11u)).exp();
        let v12p=(v1c-v12o);
        let v12s=(if vzw{((v106*v12p)/v11u)}else{vyi});
        let v12u=((v12g+v12m)-v12s);
        let v12z=(!(vzv!=0.0));
        let v130=((sf[192]!=0.0)&&v12z);
        let v132=(sb[24]&&(vzv!=0.0));
        let v133=(if v132{vm8}else{vyt});
        let v134=(v133-v7);
        let v136=(if v132{(vco*v134)}else{vyw});
        let v139=((vpr+(v136*v136))).sqrt();
        let v13a=(if v132{v139}else{vz0});
        let v13d=(if v132{(vw*(v136+v13a))}else{vz3});
        let v13g=(if v132{(v133-(vcm*v13d))}else{vz6});
        let v13k=(v1c-(v13g/vfx));
        let v13m=(if v132{(v13k).ln()}else{vzc});
        let v13s=((sf[197]*v13m)).exp();
        let v13t=(v1c-v13s);
        let v13w=(if v132{((vfx*v13t)/sf[197])}else{vzm});
        let v13z=(v13w+(vm3*(v7-v13g)));
        let v142=(sb[24]&&v12z);
        let v143=(if v142{v1o}else{(if v132{(vqp*v13z)}else{(if v130{v1o}else{(if vzw{((vfx*v12u)+(v101*v11j))}else{v1o})})})});
        let v145=(if (vzv!=0.0){vm8}else{v1o});
        let v146=(v145-v7);
        let v148=(if (vzv!=0.0){(vco*v146)}else{v1o});
        let v14b=((vpr+(v148*v148))).sqrt();
        let v14c=(if (vzv!=0.0){v14b}else{v1o});
        let v14f=(if (vzv!=0.0){(vw*(v148+v14c))}else{v1o});
        let v14i=(if (vzv!=0.0){(v145-(vcm*v14f))}else{v1o});
        let v14k=(if (vzv!=0.0){(v14f/v14c)}else{v1o});
        let v14m=(v1c-(v14i/vfx));
        let v14p=((sf[198]*(v14m).ln())).exp();
        let v14q=(vqp*v14p);
        let v14s=(v1c-v14k);
        let v14w=(if v12z{v1o}else{(if (vzv!=0.0){((v14k*v14q)+(v100*v14s))}else{v1o})});
        let v150=(if sb[5]{(vb-(if sb[16]{vgy}else{(if sb[15]{sf[82]}else{(if (sf[148]!=0.0){vgy}else{sf[423]})})}))}else{(if (sf[85]!=0.0){((if sb[16]{sf[84]}else{(if sb[15]{(sf[84]*(v1c-(sf[86]*vcq)))}else{sf[424]})})-v7)}else{v1o})});
        let v152=((vco*v150)-v1c);
        let v155=((vpr+(v152*v152))).sqrt();
        let v158=(v1c+((v152+v155)/v2m));
        let v159=(vcm*v158);
        let v15a=(v159/vgp);
        let v15b=(vgv*v159);
        let v15f=((sf[207]*(v15a).ln())).exp();
        let v15g=(v1c+v15f);
        let v15j=(((v15g).ln()/sf[207])).exp();
        let v15k=(v15b/v15j);
        let v15n=((v159-vgp)/sf[208]);
        let v15r=(((v15n*v15n)+sf[209])).sqrt();
        let v15u=(v1c+(vw*(v15n+v15r)));
        let v15v=(v15k*v15u);
        let v15y=(if (vzu&&(v14w>v1o)){v1c}else{v1o});
        let v163=(!(v15y!=0.0));
        let v164=(if v163{v1c}else{(if (v15y!=0.0){(vqp/v14w)}else{v1o})});
        let v165=(if v163{v1o}else{(if (v15y!=0.0){(v143/vqp)}else{v143})});
        let v167=(if (vdw>v1o){v1c}else{v1o});
        let v16b=(((-(vdz).ln())/sf[47])).exp();
        let v16c=(v1c-v16b);
        let v16e=(if (v167!=0.0){(vdq*v16c)}else{v133});
        let v16f=(v16e-va);
        let v16h=(if (v167!=0.0){(vco*v16f)}else{v136});
        let v16k=((vpr+(v16h*v16h))).sqrt();
        let v16l=(if (v167!=0.0){v16k}else{v13a});
        let v16o=(if (v167!=0.0){(vw*(v16h+v16l))}else{v13d});
        let v16r=(if (v167!=0.0){(v16e-(vcm*v16o))}else{v13g});
        let v16v=(v1c-(v16r/vdq));
        let v16x=(if (v167!=0.0){(v16v).ln()}else{v13m});
        let v175=((v16x*sf[211])).exp();
        let v176=(v1c-v175);
        let v179=(if (v167!=0.0){((vdq*v176)/sf[211])}else{v13w});
        let v17a=(va-v16r);
        let v17c=(v179+(vdz*v17a));
        let v17f=(!(v167!=0.0));
        let v17g=(if v17f{v1o}else{(if (v167!=0.0){(vdw*v17c)}else{v1o})});
        let v17h=(v17g/vdw);
        let v17j=(if (vet>v1o){v1c}else{v1o});
        let v17k=((sf[130]!=0.0)&&(v17j!=0.0));
        let v17o=(((-(vew).ln())/sf[58])).exp();
        let v17p=(v1c-v17o);
        let v17r=(if v17k{(ven*v17p)}else{v16e});
        let v17s=(v17r-va);
        let v17u=(if v17k{(vco*v17s)}else{v16h});
        let v17x=((vpr+(v17u*v17u))).sqrt();
        let v17y=(if v17k{v17x}else{v16l});
        let v181=(if v17k{(vw*(v17u+v17y))}else{v16o});
        let v184=(if v17k{(v17r-(vcm*v181))}else{v16r});
        let v188=(v1c-(v184/ven));
        let v18a=(if v17k{(v188).ln()}else{v16x});
        let v18i=((v18a*sf[213])).exp();
        let v18j=(v1c-v18i);
        let v18m=(if v17k{((ven*v18j)/sf[213])}else{v179});
        let v18n=(va-v184);
        let v18p=(v18m+(vew*v18n));
        let v18t=((sf[130]!=0.0)&&(!(v17j!=0.0)));
        let v18u=(if v18t{v1o}else{(if v17k{(vet*v18p)}else{v1o})});
        let v18z=(if sb[11]{v17h}else{(if (sf[130]!=0.0){(v18u/vet)}else{v1o})});
        let v190=(if sb[11]{vdq}else{(if (sf[130]!=0.0){ven}else{v1o})});
        let v198=(if sb[28]{(vcm*sf[218])}else{v1o});
        let v199=(v190-va);
        let v19b=(if sb[28]{(v199/v198)}else{v1o});
        let v19e=((vpr+(v19b*v19b))).sqrt();
        let v19f=(v19b+v19e);
        let v19j=(if sb[28]{(v190-(vw*(v198*v19f)))}else{v1o});
        let v19l=(v1c-(v19j/v190));
        let v19o=((sf[215]*(v19l).ln())).exp();
        let v19p=(v1c-v19o);
        let v19r=(if sb[28]{(vkb*v19p)}else{v1o});
        let v19v=(if ((v19r).abs()>=0.001){v1c}else{v1o});
        let v19w=(sb[28]&&(v19v!=0.0));
        let v19x=(v19r).exp();
        let v19y=(v19x-v1c);
        let v1a2=(sb[28]&&(!(v19v!=0.0)));
        let v1a5=(if v1a2{(v1c+(vw*v19r))}else{(if v19w{(v19y/v19r)}else{sf[217]})});
        let v1a6=(v18z*v1a5);
        let v1ac=20.0;
        let v1ae=((((v1c+(v1a6/vkj))+(v165/sf[219]))*v1ac)-v1c);
        let v1af=0.025;
        let v1ai=((vpr+(v1ae*v1ae))).sqrt();
        let v1am=(v1af*(v1c+((v1ae+v1ai)/v2m)));
        let v1av=((vhf+(sf[220]*(v164-v1c)))+(sf[221]*((v1c/v164)-v1c)));
        let v1b2=(v1c+(if (sf[223]!=0.0){((v1av/vhf)-v1c)}else{v1o}));
        let v1b6=(if sb[30]{vgl}else{(if (sf[223]!=0.0){(vgl/v1b2)}else{v1o})});
        let v1b9=(vcm*sf[225]);
        let v1ba=(va/v1b9);
        let v1bc=(if (v1ba>vmm){v1c}else{v1o});
        let v1bg=(if (v1bc!=0.0){vmm}else{v1ba});
        let v1bh=(!(v1bc!=0.0));
        let v1bi=(if v1bh{v1c}else{(if (v1bc!=0.0){(v1c+(v1ba-vmm))}else{v1o})});
        let v1bj=scalar_limexp(v1bg);
        let v1bk=(v1bi*v1bj);
        let v1bl=(vgf*v1bk);
        let v1bn=(vcm*sf[226]);
        let v1bo=(v7/v1bn);
        let v1bq=(if (v1bo>vmm){v1c}else{v1o});
        let v1bu=(if (v1bq!=0.0){vmm}else{v1bo});
        let v1bv=(!(v1bq!=0.0));
        let v1bw=(if v1bv{v1c}else{(if (v1bq!=0.0){(v1c+(v1bo-vmm))}else{v1o})});
        let v1bx=scalar_limexp(v1bu);
        let v1by=(v1bw*v1bx);
        let v1bz=(vgf*v1by);
        let v1c4=((v1bl/v1b6)+(v1bz/sf[224]));
        let v1c5=0.6666;
        let v1c6=(v1bl/v15v);
        let v1c7=(v1bl*v1c6);
        let v1c8=(vkz/vkv);
        let v1c9=(v1c7*v1c8);
        let v1cc=((v1c5*(v1c9).ln())).exp();
        let v1cf=(v1bl/vkv);
        let v1cg=(v1c4+v1cf);
        let v1ck=(if sb[32]{v1c4}else{(if (sf[227]!=0.0){(v1c4+v1cc)}else{v1o})});
        let v1cl=(if sb[32]{v1cg}else{(if (sf[227]!=0.0){(v1cc+v1cg)}else{v1o})});
        let v1cm=(v1am*v1am);
        let v1co=((v1ck+v1cm)).sqrt();
        let v1cp=(v1am+v1co);
        let v1cr=((v1cl+v1cm)).sqrt();
        let v1cx=(if (((v1cl-v1ck)).abs()>1e-8){v1c}else{v1o});
        let v1cz=(v15v/sf[228]);
        let v1d0=(v1cz/v1bl);
        let v1d3=(if (v1cx!=0.0){(v1c-(v1cp*v1d0))}else{v1o});
        let v1d4=((v1am+v1cr)-v1cp);
        let v1d7=(if (v1cx!=0.0){(v1c+(v1d0*v1d4))}else{v1o});
        let v1d9=(if (v1cx!=0.0){(v1d3/v1d7)}else{v1o});
        let v1db=0.01;
        let v1dd=(((v1d9*v1d9)+v1db)).sqrt();
        let v1df=2.004987562112089;
        let v1di=(!(v1cx!=0.0));
        let v1dj=(if v1di{v1o}else{(if (v1cx!=0.0){((v1d9+v1dd)/v1df)}else{v1o})});
        let v1do=(v1cf*v1dj);
        let v1dq=(v1c4+(v1dj*v1do));
        let v1dw=((v1cm+(if sb[35]{v1dq}else{(if sb[34]{(v1cc+v1dq)}else{v1o})}))).sqrt();
        let v1e2=-2.0;
        let v1e4=(if sb[36]{(v1am*v1e2)}else{v1o});
        let v1ed=(if sb[41]{(-v1dq)}else{v1o});
        let v1ee=(-v1bl);
        let v1ef=(v1bl*v1ee);
        let v1eg=(v1ef/v15v);
        let v1eh=(vkz*v1eg);
        let v1el=(if sb[36]{(v1e4*v1e4)}else{v1o});
        let v1eo=(if sb[36]{(v1ed-(sf[231]*v1el))}else{v1o});
        let v1ep=(v2m*v1e4);
        let v1er=27.0;
        let v1ex=(if sb[36]{((if sb[36]{(v1eh/vkv)}else{v1o})+(((v1el*v1ep)/v1er)-(sf[231]*(v1e4*v1ed))))}else{v1o});
        let v1ez=0.25;
        let v1f1=(v1eo*v1eo);
        let v1f2=(v1eo*v1f1);
        let v1f5=(if sb[36]{(((v1ex*v1ex)*v1ez)+(v1f2/v1er))}else{v1o});
        let v1f9=(if ((v1f5).abs()<1e-10){v1c}else{v1o});
        let v1fa=(sb[36]&&(v1f9!=0.0));
        let v1fb=(v17*v1ex);
        let v1fd=(sf[231]*v1e4);
        let v1fh=(if (v1f5>v1o){v1c}else{v1o});
        let v1fj=(sb[36]&&(!(v1f9!=0.0)));
        let v1fk=((v1fh!=0.0)&&v1fj);
        let v1fm=(vw*(-v1ex));
        let v1fn=(if v1fk{v1fm}else{v1o});
        let v1fo=(v1f5).sqrt();
        let v1fp=(if v1fk{v1fo}else{v1o});
        let v1fr=(if v1fk{(v1fn+v1fp)}else{v1el});
        let v1ft=(if (v1fr>v1o){v1c}else{v1o});
        let v1fu=(v1fk&&(v1ft!=0.0));
        let v1fx=((sf[231]*(v1fr).ln())).exp();
        let v1g0=(v1fk&&(!(v1ft!=0.0)));
        let v1g1=(-v1fr);
        let v1g4=((sf[231]*(v1g1).ln())).exp();
        let v1g8=(if v1fk{(v1fn-v1fp)}else{v1fr});
        let v1ga=(if (v1g8>v1o){v1c}else{v1o});
        let v1gb=(v1fk&&(v1ga!=0.0));
        let v1ge=((sf[231]*(v1g8).ln())).exp();
        let v1gh=(v1fk&&(!(v1ga!=0.0)));
        let v1gi=(-v1g8);
        let v1gl=((sf[231]*(v1gi).ln())).exp();
        let v1gs=(v1fj&&(!(v1fh!=0.0)));
        let v1gt=-27.0;
        let v1gv=((v1gt/v1f2)).sqrt();
        let v1gx=(if v1gs{(v1fm*v1gv)}else{v1g8});
        let v1gz=(if v1gs{(v1gx*v1gx)}else{v1fn});
        let v1h1=(if (v1gx>=v1o){v1c}else{v1o});
        let v1h2=(v1gs&&(v1h1!=0.0));
        let v1h3=1.5707963267948966;
        let v1h4=(v1c-v1gz);
        let v1h6=((v1gz/v1h4)).sqrt();
        let v1h7=(v1h6).atan();
        let v1hb=(v1gs&&(!(v1h1!=0.0)));
        let v1hd=(if v1hb{(v1h3+v1h7)}else{(if v1h2{(v1h3-v1h7)}else{v1gx})});
        let v1he=-4.0;
        let v1hh=((sf[231]*(v1eo*v1he))).sqrt();
        let v1hi=(sf[231]*v1hd);
        let v1hj=(v1hi).cos();
        let v1ho=(if sb[36]{(if v1gs{(if v1gs{((v1hh*v1hj)-v1fd)}else{v1hd})}else{(if v1fk{(((if v1g0{(-v1g4)}else{(if v1fu{v1fx}else{v1o})})+(if v1gh{(-v1gl)}else{(if v1gb{v1ge}else{v1o})}))-v1fd)}else{(if v1fa{((v1fb/v1eo)-v1fd)}else{v1o})})})}else{(if (sf[230]!=0.0){(v1am+v1dw)}else{v1o})});
        let v1hp=1e-20;
        let v1hr=(if (v1ho<v1hp){v1c}else{v1o});
        let v1hs=(if (v1hr!=0.0){v1hp}else{v1ho});
        let v1ht=(v1bl/v1hs);
        let v1hu=(v1bz/v1hs);
        let v1hw=(if (v1ht<v1hp){v1c}else{v1o});
        let v1hx=(if (v1hw!=0.0){v1hp}else{v1ht});
        let v1i1=(v1c-(v15v/v1hx));
        let v1i5=(((v1i1*v1i1)+sf[233])).sqrt();
        let v1ia=((v1i1+v1i5)/sf[236]);
        let v1ib=(vhs*v1ia);
        let v1ic=(v1ia*v1ib);
        let v1if=(v1hx/v15v);
        let v1ii=((sf[237]*(v1if).ln())).exp();
        let v1ij=(vho*v1ii);
        let v1io=((v1hx*v1ic)+((v1av*v1hx)+((v1hx*v1ij)/sf[238])));
        let v1kr=(if (vg3>v1o){v1c}else{v1o});
        let v1ks=((sf[192]!=0.0)&&(v1kr!=0.0));
        let v1kt=(if v1ks{sf[193]}else{vzx});
        let v1ku=(if v1ks{vm1}else{vzy});
        let v1kv=(if v1ks{vm8}else{vzz});
        let v1kx=(if v1ks{(vg3*vm3)}else{v101});
        let v1ky=(v1kt-sf[73]);
        let v1l0=((vme*v1ky)).exp();
        let v1l2=(if v1ks{(vg3*v1l0)}else{v106});
        let v1l3=(v1kv-v7);
        let v1l5=(if v1ks{(vco*v1l3)}else{v109});
        let v1l7=(if (v1l5<vmm){v1c}else{v1o});
        let v1l8=(v1ks&&(v1l7!=0.0));
        let v1l9=(v1l5).exp();
        let v1la=(if v1l8{v1l9}else{v110});
        let v1lb=(v1c+v1la);
        let v1le=(v1lb).ln();
        let v1lj=(v1ks&&(!(v1l7!=0.0)));
        let v1ll=(if v1lj{v7}else{(if v1l8{(v1kv-(vcm*v1le))}else{v10p})});
        let v1lo=(if v1ks{(vn5+(vn3*v1ku))}else{v10s});
        let v1lp=(v1ku+v1ll);
        let v1lr=(if v1ks{(v1lp/v1lo)}else{v10v});
        let v1lt=(if (v1lr<vmm){v1c}else{v1o});
        let v1lu=(v1ks&&(v1lt!=0.0));
        let v1lv=(v1lr).exp();
        let v1lw=(if v1lu{v1lv}else{v1la});
        let v1lx=(v1c+v1lw);
        let v1m3=(-(v1ku+v1kv));
        let v1m5=((v1m3/v1lo)).exp();
        let v1m6=((v1lx).ln()-v1m5);
        let v1mb=(v1ks&&(!(v1lt!=0.0)));
        let v1md=(if v1mb{v1ll}else{(if v1lu{((-v1ku)+(v1lo*v1m6))}else{v11h})});
        let v1mh=(v1c-(v1ll/vfx));
        let v1mj=(if v1ks{(v1mh).ln()}else{v11n});
        let v1ml=(v1c-(v1md/vfx));
        let v1mn=(if v1ks{(v1ml).ln()}else{v11r});
        let v1mo=(if v1ks{sf[197]}else{v11s});
        let v1mq=(if v1ks{(v1c-v1kt)}else{v11u});
        let v1nb=((v1mn*v1mo)).exp();
        let v1nc=(v1c-v1nb);
        let v1nh=((v1mj*v1mq)).exp();
        let v1ni=(v1c-v1nh);
        let v1nn=((v1mn*v1mq)).exp();
        let v1no=(v1c-v1nn);
        let v1nv=(sb[24]&&(v1kr!=0.0));
        let v1nw=(if v1nv{vm8}else{v17r});
        let v1nx=(v1nw-v7);
        let v1nz=(if v1nv{(vco*v1nx)}else{v17u});
        let v1o2=((vpr+(v1nz*v1nz))).sqrt();
        let v1o3=(if v1nv{v1o2}else{v17y});
        let v1o6=(if v1nv{(vw*(v1nz+v1o3))}else{v181});
        let v1o9=(if v1nv{(v1nw-(vcm*v1o6))}else{v184});
        let v1od=(v1c-(v1o9/vfx));
        let v1of=(if v1nv{(v1od).ln()}else{v18a});
        let v1oq=((sf[197]*v1of)).exp();
        let v1or=(v1c-v1oq);
        let v1sf=(if (vjx>v1o){v1c}else{v1o});
        let v1sg=((sf[254]!=0.0)&&(v1sf!=0.0));
        let v1si=(if v1sg{sf[255]}else{v1kt});
        let v1sk=(if v1sg{(sf[253]-vjr)}else{v1ku});
        let v1so=(vjr*sf[258]);
        let v1sp=(if v1sg{v1so}else{v1kv});
        let v1sr=(if v1sg{(vjx*vm3)}else{v1kx});
        let v1ss=(v1si-sf[122]);
        let v1st=(sf[253]/vjr);
        let v1sw=((v1ss*(v1st).ln())).exp();
        let v1sy=(if v1sg{(vjx*v1sw)}else{v1l2});
        let v1sz=(v1sp-ve);
        let v1t1=(if v1sg{(vco*v1sz)}else{v1l5});
        let v1t3=(if (v1t1<vmm){v1c}else{v1o});
        let v1t4=(v1sg&&(v1t3!=0.0));
        let v1t5=(v1t1).exp();
        let v1t6=(if v1t4{v1t5}else{v1lw});
        let v1t7=(v1c+v1t6);
        let v1t8=(v1t7).ln();
        let v1td=(v1sg&&(!(v1t3!=0.0)));
        let v1te=(if v1td{ve}else{(if v1t4{(v1sp-(vcm*v1t8))}else{v1ll})});
        let v1th=(if v1sg{(vn5+(vn3*v1sk))}else{v1lo});
        let v1ti=(v1sk+v1te);
        let v1tk=(if v1sg{(v1ti/v1th)}else{v1lr});
        let v1tm=(if (v1tk<vmm){v1c}else{v1o});
        let v1tn=(v1sg&&(v1tm!=0.0));
        let v1to=(v1tk).exp();
        let v1tq=(v1c+(if v1tn{v1to}else{v1t6}));
        let v1tu=(-(v1sk+v1sp));
        let v1tw=((v1tu/v1th)).exp();
        let v1tx=((v1tq).ln()-v1tw);
        let v1u2=(v1sg&&(!(v1tm!=0.0)));
        let v1u3=(if v1u2{v1te}else{(if v1tn{((-v1sk)+(v1th*v1tx))}else{v1md})});
        let v1u5=(if v1sg{(ve-v1te)}else{(if v1ks{(v7-v1ll)}else{v11j})});
        let v1u7=(v1c-(v1te/vjr));
        let v1ub=(v1c-(v1u3/vjr));
        let v1ud=(if v1sg{(v1ub).ln()}else{v1mn});
        let v1uf=(if v1sg{sf[259]}else{v1mo});
        let v1uh=(if v1sg{(v1c-v1si)}else{v1mq});
        let v1uj=((v1ud*v1uf)).exp();
        let v1uk=(v1c-v1uj);
        let v1up=(((if v1sg{(v1u7).ln()}else{v1mj})*v1uh)).exp();
        let v1uq=(v1c-v1up);
        let v1uv=((v1ud*v1uh)).exp();
        let v1uw=(v1c-v1uv);
        let v1v1=(((if v1sg{((vjx*v1uk)/v1uf)}else{(if v1ks{((vg3*v1nc)/v1mo)}else{v12g})})+(if v1sg{((v1sy*v1uq)/v1uh)}else{(if v1ks{((v1l2*v1ni)/v1mq)}else{v12m})}))-(if v1sg{((v1sy*v1uw)/v1uh)}else{(if v1ks{((v1l2*v1no)/v1mq)}else{v12s})}));
        let v1v6=(!(v1sf!=0.0));
        let v1v7=((sf[254]!=0.0)&&v1v6);
        let v1va=((v1sf!=0.0)&&sb[53]);
        let v1vb=(if v1va{v1so}else{v1nw});
        let v1vc=(v1vb-ve);
        let v1ve=(if v1va{(vco*v1vc)}else{v1nz});
        let v1vh=((vpr+(v1ve*v1ve))).sqrt();
        let v1vl=(if v1va{(vw*(v1ve+(if v1va{v1vh}else{v1o3})))}else{v1o6});
        let v1vo=(if v1va{(v1vb-(vcm*v1vl))}else{v1o9});
        let v1vq=(v1c-(v1vo/vjr));
        let v1vu=((sf[259]*(if v1va{(v1vq).ln()}else{v1of}))).exp();
        let v1vv=(v1c-v1vu);
        let v1w1=((if v1va{((vjr*v1vv)/sf[259])}else{(if v1nv{((vfx*v1or)/sf[197])}else{v18m})})+(vm3*(ve-v1vo)));
        let v1w4=(v1v6&&sb[53]);
        let v1wi=ctx.node_voltage(nodes[8]);
        let v1wj=(if (sf[262]!=0.0){v1wi}else{v1io});
        let v1wq=ctx.node_voltage(nodes[9]);
        let v1wr=(if (sf[262]!=0.0){v1wq}else{v1hx});
        let v1x0=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(v1wj*sf[263]))}else{v1o})});
        let v1x2=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(v1wr*sf[264]))}else{v1o})});
        let v1xb=(sf[0]*(if v1w4{v1o}else{(if v1va{(vjx*v1w1)}else{(if v1v7{v1o}else{(if v1sg{((vjr*v1v1)+(v1sr*v1u5))}else{v1o})})})}));
        let v1xc=(sf[0]*(if vzs{v1o}else{(if vys{(vvj*vzp)}else{(if vyq{v1o}else{(if vvm{((viw*vyk)+(vvr*vx9))}else{(if vqm{v1o}else{(if vpl{(vlr*vqj)}else{(if vph{v1o}else{(if vly{((vfx*vpb)+(vmb*vny))}else{v1o})})})})})})})}));
        let v1xd=(sf[0]*(v4*sf[265]));
        let v1xe=(sf[0]*((sf[0]*(v1-vf))*sf[266]));
        let v1xg=(sf[0]*(((if vvg{v1o}else{(if vug{(vqr*vvd)}else{(if vuc{v1o}else{(if vqy{((viw*vu6)+(vr9*vst))}else{v1o})})})})+v143)+(v1hu*sf[239])));
        let v1xi=(sf[0]*(v17g+v1wj));
        let v1y1=(vc7*sf[270]);
        let v1yq=(if vci{v1o}else{(if vcc{v1o}else{sf[275]})});
        let v1yt=(if (sf[148]!=0.0){((vq*v1yq)/vs)}else{v1o});
        let v1yx=(if (sf[148]!=0.0){((-v1yt)/(vcm*vcm))}else{v1o});
        let v1yy=(if (sf[148]!=0.0){v1yq}else{v1o});
        let v1z0=(if (sf[148]!=0.0){(v1yq/sf[2])}else{v1o});
        let v1z2=(if (sf[148]!=0.0){(v1z0/vcs)}else{v1o});
        let v1z6=(if (sf[148]!=0.0){((vcv*v1yx)+(vco*v1z0))}else{v1o});
        let v1z8=(-v1z0);
        let v1z9=(sf[10]*v1z8);
        let v1ze=((vda*v1z2)+(vcu*(sf[20]*v1yt)));
        let v1zg=(if (sf[148]!=0.0){(((sf[156]*v1z0)+v1z9)-v1ze)}else{v1o});
        let v1zh=(v2m*v1yt);
        let v1zw=(if (sf[148]!=0.0){(v1zg+((vdn*v1zh)+(vde*((vw*((v32*(vdh*((vdf*v1yx)+(vco*(-v1zg)))))/(v2m*vdk)))/vdm))))}else{v1o});
        let v1zz=(vdq*vdq);
        let v205=(if (sf[148]!=0.0){(sf[30]*(vdu*(sf[47]*(((-(sf[37]*v1zw))/v1zz)/vdr))))}else{v1o});
        let v208=(if (sf[148]!=0.0){((sf[48]*v1zw)/sf[37])}else{v1o});
        let v20c=(if (sf[148]!=0.0){((v1z9+(sf[164]*v1z0))-v1ze)}else{v1zg});
        let v20r=(if (sf[148]!=0.0){(v20c+((vek*v1zh)+(vde*((vw*((v32*(vee*((vec*v1yx)+(vco*(-v20c)))))/(v2m*veh)))/vej))))}else{v1o});
        let v20u=(ven*ven);
        let v210=(if (sf[148]!=0.0){(sf[30]*(ver*(sf[58]*(((-(sf[49]*v20r))/v20u)/veo))))}else{v1o});
        let v213=(if (sf[148]!=0.0){((sf[59]*v20r)/sf[49])}else{v1o});
        let v21h=(sf[13]*v1z8);
        let v21k=(if (sf[148]!=0.0){(((sf[172]*v1z0)+v21h)-v1ze)}else{v20c});
        let v21z=(if (sf[148]!=0.0){(v21k+((vfu*v1zh)+(vde*((vw*((v32*(vfo*((vfm*v1yx)+(vco*(-v21k)))))/(v2m*vfr)))/vft))))}else{v1o});
        let v222=(vfx*vfx);
        let v228=(if (sf[148]!=0.0){(sf[32]*(vg1*(sf[73]*(((-(sf[64]*v21z))/v222)/vfy))))}else{v1o});
        let v22k=(if (sf[148]!=0.0){(sf[75]*(vgd*((sf[26]*v1z2)+(sf[7]*v1z6))))}else{v1o});
        let v22q=(if (sf[148]!=0.0){(sf[76]*(vgj*((sf[77]*v1z2)-(sf[78]*v1z6))))}else{v1o});
        let v22u=(if (sf[148]!=0.0){(sf[79]*(vgn*(sf[80]*v1z2)))}else{v1o});
        let v234=(sf[82]*(sf[83]*v1yy));
        let v23k=(if (sf[148]!=0.0){(sf[87]*((sf[88]*v1yy)+((vhb*v1yy)+(vcq*(sf[89]*v1yy)))))}else{v1o});
        let v23m=(sf[29]*v1z6);
        let v24d=(if (sf[148]!=0.0){((v21h+(sf[180]*v1z0))-v1ze)}else{v21k});
        let v24s=(if (sf[148]!=0.0){(v24d+((vit*v1zh)+(vde*((vw*((v32*(vin*((vil*v1yx)+(vco*(-v24d)))))/(v2m*viq)))/vis))))}else{v1o});
        let v24v=(viw*viw);
        let v251=(if (sf[148]!=0.0){(sf[110]*(vj0*(sf[111]*(((-(sf[101]*v24s))/v24v)/vix))))}else{v1o});
        let v256=(if (sf[148]!=0.0){(((sf[188]*v1z0)+(sf[16]*v1z8))-v1ze)}else{v24d});
        let v25l=(if (sf[148]!=0.0){(v256+((vjo*v1zh)+(vde*((vw*((v32*(vji*((vjg*v1yx)+(vco*(-v256)))))/(v2m*vjl)))/vjn))))}else{v1o});
        let v25o=(vjr*vjr);
        let v25u=(if (sf[148]!=0.0){(sf[121]*(vjv*(sf[122]*(((-(sf[112]*v25l))/v25o)/vjs))))}else{v1o});
        let v26s=(if sb[22]{(vks*(sf[133]*v1z2))}else{(if sb[21]{((vkm*v1yy)+(vcq*(sf[132]*v1yy)))}else{v1o})});
        let v26u=(if (sf[148]!=0.0){(sf[134]*v26s)}else{v1o});
        let v270=(if (sf[148]!=0.0){((vkx*(sf[135]*v26s))+(vkw*(vkx*v23m)))}else{v1o});
        let v27o=(if (vll!=0.0){(sf[190]*v228)}else{v1o});
        let v27p=(-v21z);
        let v27q=(if vly{v27p}else{v1o});
        let v27r=(sf[196]*v21z);
        let v27s=(if vly{v27r}else{v1o});
        let v27u=(if vly{(vm3*v27o)}else{v1o});
        let v27y=(((-(sf[191]*v21z))/v222)/vmd);
        let v284=(if vly{((vmg*v27o)+(vlr*(vmg*(vmc*v27y))))}else{v1o});
        let v285=(vco*sf[273]);
        let v289=(sf[0]*vco);
        let v28a=(if vly{v285}else{v1o});
        let v28b=(if vly{((vmj*v1yx)+(vco*v27s))}else{v1o});
        let v28c=(if vly{v289}else{v1o});
        let v28g=(if vmp{(vmq*v28a)}else{v1o});
        let v28h=(if vmp{(vmq*v28b)}else{v1o});
        let v28i=(if vmp{(vmq*v28c)}else{v1o});
        let v29g=(if vn0{sf[0]}else{(if vmp{(-(vcm*(v28g/vms)))}else{v1o})});
        let v29h=(if vn0{v1o}else{(if vmp{(v27s-((vmv*v1yt)+(vcm*(v28h/vms))))}else{v1o})});
        let v29i=(if vn0{sf[273]}else{(if vmp{(-(vcm*(v28i/vms)))}else{v1o})});
        let v29k=(v32*v1yt);
        let v29m=(if vly{((vn3*v27q)+v29k)}else{v1o});
        let v29s=(vn7*vn7);
        let v29v=(if vly{(v29g/vn7)}else{v1o});
        let v29w=(if vly{(((vn7*(v27q+v29h))-(vn8*v29m))/v29s)}else{v1o});
        let v29x=(if vly{(v29i/vn7)}else{v1o});
        let v2a1=(if vnd{(vne*v29v)}else{v28g});
        let v2a2=(if vnd{(vne*v29w)}else{v28h});
        let v2a3=(if vnd{(vne*v29x)}else{v28i});
        let v2b8=(if vnu{v29g}else{(if vnd{(vn7*(v2a1/vng))}else{v1o})});
        let v2b9=(if vnu{v29h}else{(if vnd{((-v27q)+((vnp*v29m)+(vn7*((v2a2/vng)-(vno*(((vn7*(-(v27q+v27s)))-(vnm*v29m))/v29s))))))}else{v1o})});
        let v2ba=(if vnu{v29i}else{(if vnd{(vn7*(v2a3/vng))}else{v1o})});
        let v2be=(if vly{(sf[0]-v29g)}else{v1o});
        let v2bf=(if vly{(-v29h)}else{v1o});
        let v2bg=(if vly{(sf[273]-v29i)}else{v1o});
        let v2bt=(if vly{((-(v29g/vfx))/vo0)}else{v1o});
        let v2bu=(if vly{((-(((vfx*v29h)-(vn2*v21z))/v222))/vo0)}else{v1o});
        let v2bv=(if vly{((-(v29i/vfx))/vo0)}else{v1o});
        let v2c8=(if vly{((-(v2b8/vfx))/vo4)}else{v1o});
        let v2c9=(if vly{((-(((vfx*v2b9)-(vnw*v21z))/v222))/vo4)}else{v1o});
        let v2ca=(if vly{((-(v2ba/vfx))/vo4)}else{v1o});
        let v2ep=(if vly{((vlr*(-(vot*(vo8*v2c8))))/vo8)}else{v1o});
        let v2eq=(if vly{(((vou*v27o)+(vlr*(-(vot*(vo8*v2c9)))))/vo8)}else{v1o});
        let v2er=(if vly{((vlr*(-(vot*(vo8*v2ca))))/vo8)}else{v1o});
        let v2f9=(if vly{((vmi*(-(voz*(voa*v2bt))))/voa)}else{v1o});
        let v2fa=(if vly{(((vp0*v284)+(vmi*(-(voz*(voa*v2bu)))))/voa)}else{v1o});
        let v2fb=(if vly{((vmi*(-(voz*(voa*v2bv))))/voa)}else{v1o});
        let v2ft=(if vly{((vmi*(-(vp5*(voa*v2c8))))/voa)}else{v1o});
        let v2fu=(if vly{(((vp6*v284)+(vmi*(-(vp5*(voa*v2c9)))))/voa)}else{v1o});
        let v2fv=(if vly{((vmi*(-(vp5*(voa*v2ca))))/voa)}else{v1o});
        let v2gl=(if vpl{v27r}else{v1o});
        let v2gp=(if vpl{v285}else{v1o});
        let v2gq=(if vpl{((vpn*v1yx)+(vco*v2gl))}else{v1o});
        let v2gr=(if vpl{v289}else{v1o});
        let v2gs=(vpp*v2gp);
        let v2gu=(vpp*v2gq);
        let v2gw=(vpp*v2gr);
        let v2gy=(v2m*vpt);
        let v2h2=(if vpl{((v2gs+v2gs)/v2gy)}else{v1o});
        let v2h3=(if vpl{((v2gu+v2gu)/v2gy)}else{v1o});
        let v2h4=(if vpl{((v2gw+v2gw)/v2gy)}else{v1o});
        let v2hb=(if vpl{(vw*(v2gp+v2h2))}else{v1o});
        let v2hc=(if vpl{(vw*(v2gq+v2h3))}else{v1o});
        let v2hd=(if vpl{(vw*(v2gr+v2h4))}else{v1o});
        let v2hm=(if vpl{(-(vcm*v2hb))}else{v1o});
        let v2hn=(if vpl{(v2gl-((vpx*v1yt)+(vcm*v2hc)))}else{v1o});
        let v2ho=(if vpl{(-(vcm*v2hd))}else{v1o});
        let v2ih=(if vpl{((-(v2hm/vfx))/vq4)}else{v1o});
        let v2ii=(if vpl{((-(((vfx*v2hn)-(vq0*v21z))/v222))/vq4)}else{v1o});
        let v2ij=(if vpl{((-(v2ho/vfx))/vq4)}else{v1o});
        let v2jj=(if vpl{((vfx*(-(vqc*(sf[197]*v2ih))))/sf[197])}else{v1o});
        let v2jk=(if vpl{(((vqd*v21z)+(vfx*(-(vqc*(sf[197]*v2ii)))))/sf[197])}else{v1o});
        let v2jl=(if vpl{((vfx*(-(vqc*(sf[197]*v2ij))))/sf[197])}else{v1o});
        let v2k6=(if vqo{v228}else{(if (vll!=0.0){(sf[189]*v228)}else{v1o})});
        let v2k8=(if vqo{(sf[189]*v251)}else{v1o});
        let v2k9=(-v24s);
        let v2ka=(if vqy{v2k9}else{v27q});
        let v2kb=(sf[204]*v24s);
        let v2kc=(if vqy{v2kb}else{v27s});
        let v2ke=(if vqy{(vm3*v2k8)}else{v27u});
        let v2ki=(((-(sf[199]*v24s))/v24v)/vrb);
        let v2ko=(if vqy{((vre*v2k8)+(vqr*(vre*(vra*v2ki))))}else{v284});
        let v2ks=(if vqy{v1o}else{v28a});
        let v2kt=(if vqy{((vrh*v1yx)+(vco*v2kc))}else{v28b});
        let v2ku=(if vqy{v289}else{v28c});
        let v2kv=(if vqy{v285}else{v1o});
        let v2l0=(if vrm{(vrn*v2ks)}else{v2a1});
        let v2l1=(if vrm{(vrn*v2kt)}else{v2a2});
        let v2l2=(if vrm{(vrn*v2ku)}else{v2a3});
        let v2l3=(if vrm{(vrn*v2kv)}else{v1o});
        let v2mb=(if vrx{v1o}else{(if vrm{(-(vcm*(v2l0/vrp)))}else{v29g})});
        let v2mc=(if vrx{v1o}else{(if vrm{(v2kc-((vrs*v1yt)+(vcm*(v2l1/vrp))))}else{v29h})});
        let v2md=(if vrx{sf[273]}else{(if vrm{(-(vcm*(v2l2/vrp)))}else{v29i})});
        let v2me=(if vrx{sf[0]}else{(if vrm{(-(vcm*(v2l3/vrp)))}else{v1o})});
        let v2mh=(if vqy{(v29k+(vn3*v2ka))}else{v29m});
        let v2mn=(vs2*vs2);
        let v2mr=(if vqy{(v2mb/vs2)}else{v29v});
        let v2ms=(if vqy{(((vs2*(v2ka+v2mc))-(vs3*v2mh))/v2mn)}else{v29w});
        let v2mt=(if vqy{(v2md/vs2)}else{v29x});
        let v2mu=(if vqy{(v2me/vs2)}else{v1o});
        let v2mz=(if vs8{(vs9*v2mr)}else{v2l0});
        let v2n0=(if vs8{(vs9*v2ms)}else{v2l1});
        let v2n1=(if vs8{(vs9*v2mt)}else{v2l2});
        let v2n2=(if vs8{(vs9*v2mu)}else{v2l3});
        let v2og=(if vsp{v2mb}else{(if vs8{(vs2*(v2mz/vsb))}else{v2b8})});
        let v2oh=(if vsp{v2mc}else{(if vs8{((-v2ka)+((vsk*v2mh)+(vs2*((v2n0/vsb)-(vsj*(((vs2*(-(v2ka+v2kc)))-(vsh*v2mh))/v2mn))))))}else{v2b9})});
        let v2oi=(if vsp{v2md}else{(if vs8{(vs2*(v2n1/vsb))}else{v2ba})});
        let v2oj=(if vsp{v2me}else{(if vs8{(vs2*(v2n2/vsb))}else{v1o})});
        let v2oo=(if vqy{(-v2mb)}else{v2be});
        let v2op=(if vqy{(-v2mc)}else{v2bf});
        let v2oq=(if vqy{(sf[273]-v2md)}else{v2bg});
        let v2or=(if vqy{(sf[0]-v2me)}else{v1o});
        let v2p7=(if vqy{((-(v2mb/viw))/vsv)}else{v2bt});
        let v2p8=(if vqy{((-(((viw*v2mc)-(vrz*v24s))/v24v))/vsv)}else{v2bu});
        let v2p9=(if vqy{((-(v2md/viw))/vsv)}else{v2bv});
        let v2pa=(if vqy{((-(v2me/viw))/vsv)}else{v1o});
        let v2pq=(if vqy{((-(v2og/viw))/vsz)}else{v2c8});
        let v2pr=(if vqy{((-(((viw*v2oh)-(vsr*v24s))/v24v))/vsz)}else{v2c9});
        let v2ps=(if vqy{((-(v2oi/viw))/vsz)}else{v2ca});
        let v2pt=(if vqy{((-(v2oj/viw))/vsz)}else{v1o});
        let v2sy=(if vqy{((vqr*(-(vto*(vt3*v2pq))))/vt3)}else{v2ep});
        let v2sz=(if vqy{(((vtp*v2k8)+(vqr*(-(vto*(vt3*v2pr)))))/vt3)}else{v2eq});
        let v2t0=(if vqy{((vqr*(-(vto*(vt3*v2ps))))/vt3)}else{v2er});
        let v2t1=(if vqy{((vqr*(-(vto*(vt3*v2pt))))/vt3)}else{v1o});
        let v2to=(if vqy{((vrg*(-(vtu*(vt5*v2p7))))/vt5)}else{v2f9});
        let v2tp=(if vqy{(((vtv*v2ko)+(vrg*(-(vtu*(vt5*v2p8)))))/vt5)}else{v2fa});
        let v2tq=(if vqy{((vrg*(-(vtu*(vt5*v2p9))))/vt5)}else{v2fb});
        let v2tr=(if vqy{((vrg*(-(vtu*(vt5*v2pa))))/vt5)}else{v1o});
        let v2ue=(if vqy{((vrg*(-(vu0*(vt5*v2pq))))/vt5)}else{v2ft});
        let v2uf=(if vqy{(((vu1*v2ko)+(vrg*(-(vu0*(vt5*v2pr)))))/vt5)}else{v2fu});
        let v2ug=(if vqy{((vrg*(-(vu0*(vt5*v2ps))))/vt5)}else{v2fv});
        let v2uh=(if vqy{((vrg*(-(vu0*(vt5*v2pt))))/vt5)}else{v1o});
        let v2ve=(if vug{v2kb}else{v2gl});
        let v2vi=(if vug{v1o}else{v2gp});
        let v2vj=(if vug{((vui*v1yx)+(vco*v2ve))}else{v2gq});
        let v2vk=(if vug{v289}else{v2gr});
        let v2vl=(if vug{v285}else{v1o});
        let v2vm=(vuk*v2vi);
        let v2vo=(vuk*v2vj);
        let v2vq=(vuk*v2vk);
        let v2vs=(vuk*v2vl);
        let v2vu=(v2m*vun);
        let v2vz=(if vug{((v2vm+v2vm)/v2vu)}else{v2h2});
        let v2w0=(if vug{((v2vo+v2vo)/v2vu)}else{v2h3});
        let v2w1=(if vug{((v2vq+v2vq)/v2vu)}else{v2h4});
        let v2w2=(if vug{((v2vs+v2vs)/v2vu)}else{v1o});
        let v2wb=(if vug{(vw*(v2vi+v2vz))}else{v2hb});
        let v2wc=(if vug{(vw*(v2vj+v2w0))}else{v2hc});
        let v2wd=(if vug{(vw*(v2vk+v2w1))}else{v2hd});
        let v2we=(if vug{(vw*(v2vl+v2w2))}else{v1o});
        let v2wp=(if vug{(-(vcm*v2wb))}else{v2hm});
        let v2wq=(if vug{(v2ve-((vur*v1yt)+(vcm*v2wc)))}else{v2hn});
        let v2wr=(if vug{(-(vcm*v2wd))}else{v2ho});
        let v2ws=(if vug{(-(vcm*v2we))}else{v1o});
        let v2xt=(if vug{((-(v2wp/viw))/vuy)}else{v2ih});
        let v2xu=(if vug{((-(((viw*v2wq)-(vuu*v24s))/v24v))/vuy)}else{v2ii});
        let v2xv=(if vug{((-(v2wr/viw))/vuy)}else{v2ij});
        let v2xw=(if vug{((-(v2ws/viw))/vuy)}else{v1o});
        let v2z7=(if vug{((viw*(-(vv6*(sf[205]*v2xt))))/sf[205])}else{v2jj});
        let v2z8=(if vug{(((vv7*v24s)+(viw*(-(vv6*(sf[205]*v2xu)))))/sf[205])}else{v2jk});
        let v2z9=(if vug{((viw*(-(vv6*(sf[205]*v2xv))))/sf[205])}else{v2jl});
        let v2za=(if vug{((viw*(-(vv6*(sf[205]*v2xw))))/sf[205])}else{v1o});
        let v302=(if vqo{(sf[190]*v251)}else{v27o});
        let v303=(if vvm{v2k9}else{v2ka});
        let v304=(if vvm{v2kb}else{v2kc});
        let v306=(if vvm{(vm3*v302)}else{v2ke});
        let v30c=(if vvm{((vvu*v302)+(vvj*(vvu*(vvs*v2ki))))}else{v2ko});
        let v30g=(if vvm{v285}else{v2ks});
        let v30h=(if vvm{((vvx*v1yx)+(vco*v304))}else{v2kt});
        let v30i=(if vvm{v289}else{v2ku});
        let v30j=(if vvm{v1o}else{v2kv});
        let v30o=(if vw2{(vw3*v30g)}else{v2mz});
        let v30p=(if vw2{(vw3*v30h)}else{v2n0});
        let v30q=(if vw2{(vw3*v30i)}else{v2n1});
        let v30r=(if vw2{(vw3*v30j)}else{v2n2});
        let v31z=(if vwd{sf[0]}else{(if vw2{(-(vcm*(v30o/vw5)))}else{v2mb})});
        let v320=(if vwd{v1o}else{(if vw2{(v304-((vw8*v1yt)+(vcm*(v30p/vw5))))}else{v2mc})});
        let v321=(if vwd{sf[273]}else{(if vw2{(-(vcm*(v30q/vw5)))}else{v2md})});
        let v322=(if vwd{v1o}else{(if vw2{(-(vcm*(v30r/vw5)))}else{v2me})});
        let v325=(if vvm{(v29k+(vn3*v303))}else{v2mh});
        let v32b=(vwi*vwi);
        let v32f=(if vvm{(v31z/vwi)}else{v2mr});
        let v32g=(if vvm{(((vwi*(v303+v320))-(vwj*v325))/v32b)}else{v2ms});
        let v32h=(if vvm{(v321/vwi)}else{v2mt});
        let v32i=(if vvm{(v322/vwi)}else{v2mu});
        let v32n=(if vwo{(vwp*v32f)}else{v30o});
        let v32o=(if vwo{(vwp*v32g)}else{v30p});
        let v32p=(if vwo{(vwp*v32h)}else{v30q});
        let v32q=(if vwo{(vwp*v32i)}else{v30r});
        let v344=(if vx5{v31z}else{(if vwo{(vwi*(v32n/vwr))}else{v2og})});
        let v345=(if vx5{v320}else{(if vwo{((-v303)+((vx0*v325)+(vwi*((v32o/vwr)-(vwz*(((vwi*(-(v303+v304)))-(vwx*v325))/v32b))))))}else{v2oh})});
        let v346=(if vx5{v321}else{(if vwo{(vwi*(v32p/vwr))}else{v2oi})});
        let v347=(if vx5{v322}else{(if vwo{(vwi*(v32q/vwr))}else{v2oj})});
        let v34c=(if vvm{(sf[0]-v31z)}else{v2oo});
        let v34d=(if vvm{(-v320)}else{v2op});
        let v34e=(if vvm{(sf[273]-v321)}else{v2oq});
        let v34f=(if vvm{(-v322)}else{v2or});
        let v34v=(if vvm{((-(v31z/viw))/vxb)}else{v2p7});
        let v34w=(if vvm{((-(((viw*v320)-(vwf*v24s))/v24v))/vxb)}else{v2p8});
        let v34x=(if vvm{((-(v321/viw))/vxb)}else{v2p9});
        let v34y=(if vvm{((-(v322/viw))/vxb)}else{v2pa});
        let v35e=(if vvm{((-(v344/viw))/vxf)}else{v2pq});
        let v35f=(if vvm{((-(((viw*v345)-(vx7*v24s))/v24v))/vxf)}else{v2pr});
        let v35g=(if vvm{((-(v346/viw))/vxf)}else{v2ps});
        let v35h=(if vvm{((-(v347/viw))/vxf)}else{v2pt});
        let v38m=(if vvm{((vvj*(-(vy2*(vxi*v35e))))/vxi)}else{v2sy});
        let v38n=(if vvm{(((vy3*v302)+(vvj*(-(vy2*(vxi*v35f)))))/vxi)}else{v2sz});
        let v38o=(if vvm{((vvj*(-(vy2*(vxi*v35g))))/vxi)}else{v2t0});
        let v38p=(if vvm{((vvj*(-(vy2*(vxi*v35h))))/vxi)}else{v2t1});
        let v39c=(if vvm{((vvw*(-(vy8*(vxk*v34v))))/vxk)}else{v2to});
        let v39d=(if vvm{(((vy9*v30c)+(vvw*(-(vy8*(vxk*v34w)))))/vxk)}else{v2tp});
        let v39e=(if vvm{((vvw*(-(vy8*(vxk*v34x))))/vxk)}else{v2tq});
        let v39f=(if vvm{((vvw*(-(vy8*(vxk*v34y))))/vxk)}else{v2tr});
        let v3a2=(if vvm{((vvw*(-(vye*(vxk*v35e))))/vxk)}else{v2ue});
        let v3a3=(if vvm{(((vyf*v30c)+(vvw*(-(vye*(vxk*v35f)))))/vxk)}else{v2uf});
        let v3a4=(if vvm{((vvw*(-(vye*(vxk*v35g))))/vxk)}else{v2ug});
        let v3a5=(if vvm{((vvw*(-(vye*(vxk*v35h))))/vxk)}else{v2uh});
        let v3b2=(if vys{v2kb}else{v2ve});
        let v3b6=(if vys{v285}else{v2vi});
        let v3b7=(if vys{((vyu*v1yx)+(vco*v3b2))}else{v2vj});
        let v3b8=(if vys{v289}else{v2vk});
        let v3b9=(if vys{v1o}else{v2vl});
        let v3ba=(vyw*v3b6);
        let v3bc=(vyw*v3b7);
        let v3be=(vyw*v3b8);
        let v3bg=(vyw*v3b9);
        let v3bi=(v2m*vyz);
        let v3bn=(if vys{((v3ba+v3ba)/v3bi)}else{v2vz});
        let v3bo=(if vys{((v3bc+v3bc)/v3bi)}else{v2w0});
        let v3bp=(if vys{((v3be+v3be)/v3bi)}else{v2w1});
        let v3bq=(if vys{((v3bg+v3bg)/v3bi)}else{v2w2});
        let v3bz=(if vys{(vw*(v3b6+v3bn))}else{v2wb});
        let v3c0=(if vys{(vw*(v3b7+v3bo))}else{v2wc});
        let v3c1=(if vys{(vw*(v3b8+v3bp))}else{v2wd});
        let v3c2=(if vys{(vw*(v3b9+v3bq))}else{v2we});
        let v3cd=(if vys{(-(vcm*v3bz))}else{v2wp});
        let v3ce=(if vys{(v3b2-((vz3*v1yt)+(vcm*v3c0)))}else{v2wq});
        let v3cf=(if vys{(-(vcm*v3c1))}else{v2wr});
        let v3cg=(if vys{(-(vcm*v3c2))}else{v2ws});
        let v3dh=(if vys{((-(v3cd/viw))/vza)}else{v2xt});
        let v3di=(if vys{((-(((viw*v3ce)-(vz6*v24s))/v24v))/vza)}else{v2xu});
        let v3dj=(if vys{((-(v3cf/viw))/vza)}else{v2xv});
        let v3dk=(if vys{((-(v3cg/viw))/vza)}else{v2xw});
        let v3ev=(if vys{((viw*(-(vzi*(sf[205]*v3dh))))/sf[205])}else{v2z7});
        let v3ew=(if vys{(((vzj*v24s)+(viw*(-(vzi*(sf[205]*v3di)))))/sf[205])}else{v2z8});
        let v3ex=(if vys{((viw*(-(vzi*(sf[205]*v3dj))))/sf[205])}else{v2z9});
        let v3ey=(if vys{((viw*(-(vzi*(sf[205]*v3dk))))/sf[205])}else{v2za});
        let v3fp=(if vzw{v27p}else{v303});
        let v3fq=(if vzw{v27r}else{v304});
        let v3fr=(vm3*v2k6);
        let v3fs=(if vzw{v3fr}else{v306});
        let v3fy=(if vzw{((v104*v2k6)+(vqp*(v104*(v102*v27y))))}else{v30c});
        let v3g2=(if vzw{v1o}else{v30g});
        let v3g3=(if vzw{((v107*v1yx)+(vco*v3fq))}else{v30h});
        let v3g4=(if vzw{v289}else{v30i});
        let v3g5=(if vzw{v285}else{v30j});
        let v3ga=(if v10c{(v10d*v3g2)}else{v32n});
        let v3gb=(if v10c{(v10d*v3g3)}else{v32o});
        let v3gc=(if v10c{(v10d*v3g4)}else{v32p});
        let v3gd=(if v10c{(v10d*v3g5)}else{v32q});
        let v3hl=(if v10n{v1o}else{(if v10c{(-(vcm*(v3ga/v10f)))}else{v31z})});
        let v3hm=(if v10n{v1o}else{(if v10c{(v3fq-((v10i*v1yt)+(vcm*(v3gb/v10f))))}else{v320})});
        let v3hn=(if v10n{sf[273]}else{(if v10c{(-(vcm*(v3gc/v10f)))}else{v321})});
        let v3ho=(if v10n{sf[0]}else{(if v10c{(-(vcm*(v3gd/v10f)))}else{v322})});
        let v3hr=(if vzw{(v29k+(vn3*v3fp))}else{v325});
        let v3hx=(v10s*v10s);
        let v3i1=(if vzw{(v3hl/v10s)}else{v32f});
        let v3i2=(if vzw{(((v10s*(v3fp+v3hm))-(v10t*v3hr))/v3hx)}else{v32g});
        let v3i3=(if vzw{(v3hn/v10s)}else{v32h});
        let v3i4=(if vzw{(v3ho/v10s)}else{v32i});
        let v3i9=(if v10y{(v10z*v3i1)}else{v3ga});
        let v3ia=(if v10y{(v10z*v3i2)}else{v3gb});
        let v3ib=(if v10y{(v10z*v3i3)}else{v3gc});
        let v3ic=(if v10y{(v10z*v3i4)}else{v3gd});
        let v3jq=(if v11f{v3hl}else{(if v10y{(v10s*(v3i9/v111))}else{v344})});
        let v3jr=(if v11f{v3hm}else{(if v10y{((-v3fp)+((v11a*v3hr)+(v10s*((v3ia/v111)-(v119*(((v10s*(-(v3fp+v3fq)))-(v117*v3hr))/v3hx))))))}else{v345})});
        let v3js=(if v11f{v3hn}else{(if v10y{(v10s*(v3ib/v111))}else{v346})});
        let v3jt=(if v11f{v3ho}else{(if v10y{(v10s*(v3ic/v111))}else{v347})});
        let v3jy=(if vzw{(-v3hl)}else{v34c});
        let v3jz=(if vzw{(-v3hm)}else{v34d});
        let v3k0=(if vzw{(sf[273]-v3hn)}else{v34e});
        let v3k1=(if vzw{(sf[0]-v3ho)}else{v34f});
        let v3kh=(if vzw{((-(v3hl/vfx))/v11l)}else{v34v});
        let v3ki=(if vzw{((-(((vfx*v3hm)-(v10p*v21z))/v222))/v11l)}else{v34w});
        let v3kj=(if vzw{((-(v3hn/vfx))/v11l)}else{v34x});
        let v3kk=(if vzw{((-(v3ho/vfx))/v11l)}else{v34y});
        let v3l0=(if vzw{((-(v3jq/vfx))/v11p)}else{v35e});
        let v3l1=(if vzw{((-(((vfx*v3jr)-(v11h*v21z))/v222))/v11p)}else{v35f});
        let v3l2=(if vzw{((-(v3js/vfx))/v11p)}else{v35g});
        let v3l3=(if vzw{((-(v3jt/vfx))/v11p)}else{v35h});
        let v3o8=(if vzw{((vqp*(-(v12c*(v11s*v3l0))))/v11s)}else{v38m});
        let v3o9=(if vzw{(((v12d*v2k6)+(vqp*(-(v12c*(v11s*v3l1)))))/v11s)}else{v38n});
        let v3oa=(if vzw{((vqp*(-(v12c*(v11s*v3l2))))/v11s)}else{v38o});
        let v3ob=(if vzw{((vqp*(-(v12c*(v11s*v3l3))))/v11s)}else{v38p});
        let v3oy=(if vzw{((v106*(-(v12i*(v11u*v3kh))))/v11u)}else{v39c});
        let v3oz=(if vzw{(((v12j*v3fy)+(v106*(-(v12i*(v11u*v3ki)))))/v11u)}else{v39d});
        let v3p0=(if vzw{((v106*(-(v12i*(v11u*v3kj))))/v11u)}else{v39e});
        let v3p1=(if vzw{((v106*(-(v12i*(v11u*v3kk))))/v11u)}else{v39f});
        let v3po=(if vzw{((v106*(-(v12o*(v11u*v3l0))))/v11u)}else{v3a2});
        let v3pp=(if vzw{(((v12p*v3fy)+(v106*(-(v12o*(v11u*v3l1)))))/v11u)}else{v3a3});
        let v3pq=(if vzw{((v106*(-(v12o*(v11u*v3l2))))/v11u)}else{v3a4});
        let v3pr=(if vzw{((v106*(-(v12o*(v11u*v3l3))))/v11u)}else{v3a5});
        let v3qo=(if v132{v27r}else{v3b2});
        let v3qs=(if v132{v1o}else{v3b6});
        let v3qt=(if v132{((v134*v1yx)+(vco*v3qo))}else{v3b7});
        let v3qu=(if v132{v289}else{v3b8});
        let v3qv=(if v132{v285}else{v3b9});
        let v3qw=(v136*v3qs);
        let v3qy=(v136*v3qt);
        let v3r0=(v136*v3qu);
        let v3r2=(v136*v3qv);
        let v3r4=(v2m*v139);
        let v3r9=(if v132{((v3qw+v3qw)/v3r4)}else{v3bn});
        let v3ra=(if v132{((v3qy+v3qy)/v3r4)}else{v3bo});
        let v3rb=(if v132{((v3r0+v3r0)/v3r4)}else{v3bp});
        let v3rc=(if v132{((v3r2+v3r2)/v3r4)}else{v3bq});
        let v3rl=(if v132{(vw*(v3qs+v3r9))}else{v3bz});
        let v3rm=(if v132{(vw*(v3qt+v3ra))}else{v3c0});
        let v3rn=(if v132{(vw*(v3qu+v3rb))}else{v3c1});
        let v3ro=(if v132{(vw*(v3qv+v3rc))}else{v3c2});
        let v3rz=(if v132{(-(vcm*v3rl))}else{v3cd});
        let v3s0=(if v132{(v3qo-((v13d*v1yt)+(vcm*v3rm)))}else{v3ce});
        let v3s1=(if v132{(-(vcm*v3rn))}else{v3cf});
        let v3s2=(if v132{(-(vcm*v3ro))}else{v3cg});
        let v3t3=(if v132{((-(v3rz/vfx))/v13k)}else{v3dh});
        let v3t4=(if v132{((-(((vfx*v3s0)-(v13g*v21z))/v222))/v13k)}else{v3di});
        let v3t5=(if v132{((-(v3s1/vfx))/v13k)}else{v3dj});
        let v3t6=(if v132{((-(v3s2/vfx))/v13k)}else{v3dk});
        let v3uh=(if v132{((vfx*(-(v13s*(sf[197]*v3t3))))/sf[197])}else{v3ev});
        let v3ui=(if v132{(((v13t*v21z)+(vfx*(-(v13s*(sf[197]*v3t4)))))/sf[197])}else{v3ew});
        let v3uj=(if v132{((vfx*(-(v13s*(sf[197]*v3t5))))/sf[197])}else{v3ex});
        let v3uk=(if v132{((vfx*(-(v13s*(sf[197]*v3t6))))/sf[197])}else{v3ey});
        let v3v7=(if v142{v1o}else{(if v132{(vqp*(v3uh+(vm3*(-v3rz))))}else{(if v130{v1o}else{(if vzw{((vfx*((v3o8+v3oy)-v3po))+(v101*v3jy))}else{v1o})})})});
        let v3v8=(if v142{v1o}else{(if v132{((v13z*v2k6)+(vqp*(v3ui+(vm3*(-v3s0)))))}else{(if v130{v1o}else{(if vzw{(((v12u*v21z)+(vfx*((v3o9+v3oz)-v3pp)))+((v11j*v3fs)+(v101*v3jz)))}else{v1o})})})});
        let v3v9=(if v142{v1o}else{(if v132{(vqp*(v3uj+(vm3*(sf[273]-v3s1))))}else{(if v130{v1o}else{(if vzw{((vfx*((v3oa+v3p0)-v3pq))+(v101*v3k0))}else{v1o})})})});
        let v3va=(if v142{v1o}else{(if v132{(vqp*(v3uk+(vm3*(sf[0]-v3s2))))}else{(if v130{v1o}else{(if vzw{((vfx*((v3ob+v3p1)-v3pr))+(v101*v3k1))}else{v1o})})})});
        let v3vf=(if (vzv!=0.0){v27r}else{v1o});
        let v3vj=(if (vzv!=0.0){((v146*v1yx)+(vco*v3vf))}else{v1o});
        let v3vk=(if (vzv!=0.0){v289}else{v1o});
        let v3vl=(if (vzv!=0.0){v285}else{v1o});
        let v3vm=(v148*v3vj);
        let v3vo=(v148*v3vk);
        let v3vq=(v148*v3vl);
        let v3vs=(v2m*v14b);
        let v3vw=(if (vzv!=0.0){((v3vm+v3vm)/v3vs)}else{v1o});
        let v3vx=(if (vzv!=0.0){((v3vo+v3vo)/v3vs)}else{v1o});
        let v3vy=(if (vzv!=0.0){((v3vq+v3vq)/v3vs)}else{v1o});
        let v3w5=(if (vzv!=0.0){(vw*(v3vj+v3vw))}else{v1o});
        let v3w6=(if (vzv!=0.0){(vw*(v3vk+v3vx))}else{v1o});
        let v3w7=(if (vzv!=0.0){(vw*(v3vl+v3vy))}else{v1o});
        let v3wm=(v14c*v14c);
        let v3ww=(if (vzv!=0.0){(((v14c*v3w5)-(v14f*v3vw))/v3wm)}else{v1o});
        let v3wx=(if (vzv!=0.0){(((v14c*v3w6)-(v14f*v3vx))/v3wm)}else{v1o});
        let v3wy=(if (vzv!=0.0){(((v14c*v3w7)-(v14f*v3vy))/v3wm)}else{v1o});
        let v3ym=((v150*v1yx)+(vco*(if sb[5]{(-(if sb[16]{v234}else{(if sb[15]{v1o}else{(if (sf[148]!=0.0){v234}else{v1o})})}))}else{(if (sf[85]!=0.0){(if sb[16]{v1o}else{(if sb[15]{(sf[84]*(-(sf[86]*v1yy)))}else{v1o})})}else{v1o})})));
        let v3yn=(vco*sf[278]);
        let v3yo=(vco*sf[279]);
        let v3yp=(vco*sf[280]);
        let v3yq=(v152*v3ym);
        let v3ys=(v152*v3yn);
        let v3yu=(v152*v3yo);
        let v3yw=(v152*v3yp);
        let v3yy=(v2m*v155);
        let v3zd=((v158*v1yt)+(vcm*((v3ym+((v3yq+v3yq)/v3yy))/v2m)));
        let v3ze=(vcm*((v3yn+((v3ys+v3ys)/v3yy))/v2m));
        let v3zf=(vcm*((v3yo+((v3yu+v3yu)/v3yy))/v2m));
        let v3zg=(vcm*((v3yp+((v3yw+v3yw)/v3yy))/v2m));
        let v40m=(v15j*v15j);
        let v411=((v3zd-v22u)/sf[208]);
        let v412=(v3ze/sf[208]);
        let v413=(v3zf/sf[208]);
        let v414=(v3zg/sf[208]);
        let v415=(v15n*v411);
        let v417=(v15n*v412);
        let v419=(v15n*v413);
        let v41b=(v15n*v414);
        let v41d=(v2m*v15r);
        let v41s=((v15u*(((v15j*((v159*(if (sf[148]!=0.0){((-(if (sf[148]!=0.0){(sf[81]*(vgr*(sf[22]*v1z2)))}else{v1o}))/(vgt*vgt))}else{v1o}))+(vgv*v3zd)))-(v15b*(v15j*(((v15f*(sf[207]*((((vgp*v3zd)-(v159*v22u))/(vgp*vgp))/v15a)))/v15g)/sf[207]))))/v40m))+(v15k*(vw*(v411+((v415+v415)/v41d)))));
        let v41v=((v15u*(((v15j*(vgv*v3ze))-(v15b*(v15j*(((v15f*(sf[207]*((v3ze/vgp)/v15a)))/v15g)/sf[207]))))/v40m))+(v15k*(vw*(v412+((v417+v417)/v41d)))));
        let v41y=((v15u*(((v15j*(vgv*v3zf))-(v15b*(v15j*(((v15f*(sf[207]*((v3zf/vgp)/v15a)))/v15g)/sf[207]))))/v40m))+(v15k*(vw*(v413+((v419+v419)/v41d)))));
        let v421=((v15u*(((v15j*(vgv*v3zg))-(v15b*(v15j*(((v15f*(sf[207]*((v3zg/vgp)/v15a)))/v15g)/sf[207]))))/v40m))+(v15k*(vw*(v414+((v41b+v41b)/v41d)))));
        let v425=(v14w*v14w);
        let v42s=(if v163{v1o}else{(if (v15y!=0.0){(((v14w*v2k6)-(vqp*(if v12z{v1o}else{(if (vzv!=0.0){(((v14q*v3ww)+(v14k*((v14p*v2k6)+(vqp*(v14p*(sf[198]*((-(((vfx*(if (vzv!=0.0){(v3vf-((v14f*v1yt)+(vcm*v3w5)))}else{v1o}))-(v14i*v21z))/v222))/v14m)))))))+((v14s*v3fr)+(v100*(-v3ww))))}else{v1o})})))/v425)}else{v1o})});
        let v42t=(if v163{v1o}else{(if (v15y!=0.0){((-(vqp*(if v12z{v1o}else{(if (vzv!=0.0){(((v14q*v3wx)+(v14k*(vqp*(v14p*(sf[198]*((-((if (vzv!=0.0){(-(vcm*v3w6))}else{v1o})/vfx))/v14m))))))+(v100*(-v3wx)))}else{v1o})})))/v425)}else{v1o})});
        let v42u=(if v163{v1o}else{(if (v15y!=0.0){((-(vqp*(if v12z{v1o}else{(if (vzv!=0.0){(((v14q*v3wy)+(v14k*(vqp*(v14p*(sf[198]*((-((if (vzv!=0.0){(-(vcm*v3w7))}else{v1o})/vfx))/v14m))))))+(v100*(-v3wy)))}else{v1o})})))/v425)}else{v1o})});
        let v42v=(if v163{v1o}else{(if (v15y!=0.0){(v3v7/vqp)}else{v3v7})});
        let v42w=(if v163{v1o}else{(if (v15y!=0.0){(((vqp*v3v8)-(v143*v2k6))/(vqp*vqp))}else{v3v8})});
        let v42x=(if v163{v1o}else{(if (v15y!=0.0){(v3v9/vqp)}else{v3v9})});
        let v42y=(if v163{v1o}else{(if (v15y!=0.0){(v3va/vqp)}else{v3va})});
        let v437=(if (v167!=0.0){((v16c*v1zw)+(vdq*(-(v16b*((-(v208/vdz))/sf[47])))))}else{v3qo});
        let v43b=(if (v167!=0.0){v1o}else{v3qs});
        let v43c=(if (v167!=0.0){((v16f*v1yx)+(vco*v437))}else{v3qt});
        let v43d=(if (v167!=0.0){v1o}else{v3qu});
        let v43e=(if (v167!=0.0){v285}else{v3qv});
        let v43f=(if (v167!=0.0){v289}else{v1o});
        let v43g=(v16h*v43b);
        let v43i=(v16h*v43c);
        let v43k=(v16h*v43d);
        let v43m=(v16h*v43e);
        let v43o=(v16h*v43f);
        let v43q=(v2m*v16k);
        let v43w=(if (v167!=0.0){((v43g+v43g)/v43q)}else{v3r9});
        let v43x=(if (v167!=0.0){((v43i+v43i)/v43q)}else{v3ra});
        let v43y=(if (v167!=0.0){((v43k+v43k)/v43q)}else{v3rb});
        let v43z=(if (v167!=0.0){((v43m+v43m)/v43q)}else{v3rc});
        let v440=(if (v167!=0.0){((v43o+v43o)/v43q)}else{v1o});
        let v44b=(if (v167!=0.0){(vw*(v43b+v43w))}else{v3rl});
        let v44c=(if (v167!=0.0){(vw*(v43c+v43x))}else{v3rm});
        let v44d=(if (v167!=0.0){(vw*(v43d+v43y))}else{v3rn});
        let v44e=(if (v167!=0.0){(vw*(v43e+v43z))}else{v3ro});
        let v44f=(if (v167!=0.0){(vw*(v43f+v440))}else{v1o});
        let v44s=(if (v167!=0.0){(-(vcm*v44b))}else{v3rz});
        let v44t=(if (v167!=0.0){(v437-((v16o*v1yt)+(vcm*v44c)))}else{v3s0});
        let v44u=(if (v167!=0.0){(-(vcm*v44d))}else{v3s1});
        let v44v=(if (v167!=0.0){(-(vcm*v44e))}else{v3s2});
        let v44w=(if (v167!=0.0){(-(vcm*v44f))}else{v1o});
        let v465=(if (v167!=0.0){((-(v44s/vdq))/v16v)}else{v3t3});
        let v466=(if (v167!=0.0){((-(((vdq*v44t)-(v16r*v1zw))/v1zz))/v16v)}else{v3t4});
        let v467=(if (v167!=0.0){((-(v44u/vdq))/v16v)}else{v3t5});
        let v468=(if (v167!=0.0){((-(v44v/vdq))/v16v)}else{v3t6});
        let v469=(if (v167!=0.0){((-(v44w/vdq))/v16v)}else{v1o});
        let v47v=(if (v167!=0.0){((vdq*(-(v175*(sf[211]*v465))))/sf[211])}else{v3uh});
        let v47w=(if (v167!=0.0){(((v176*v1zw)+(vdq*(-(v175*(sf[211]*v466)))))/sf[211])}else{v3ui});
        let v47x=(if (v167!=0.0){((vdq*(-(v175*(sf[211]*v467))))/sf[211])}else{v3uj});
        let v47y=(if (v167!=0.0){((vdq*(-(v175*(sf[211]*v468))))/sf[211])}else{v3uk});
        let v47z=(if (v167!=0.0){((vdq*(-(v175*(sf[211]*v469))))/sf[211])}else{v1o});
        let v48t=(if v17f{v1o}else{(if (v167!=0.0){(vdw*(v47v+(vdz*(-v44s))))}else{v1o})});
        let v48u=(if v17f{v1o}else{(if (v167!=0.0){((v17c*v205)+(vdw*(v47w+((v17a*v208)+(vdz*(-v44t))))))}else{v1o})});
        let v48v=(if v17f{v1o}else{(if (v167!=0.0){(vdw*(v47x+(vdz*(-v44u))))}else{v1o})});
        let v48w=(if v17f{v1o}else{(if (v167!=0.0){(vdw*(v47y+(vdz*(sf[0]-v44v))))}else{v1o})});
        let v48x=(if v17f{v1o}else{(if (v167!=0.0){(vdw*(v47z+(vdz*(sf[273]-v44w))))}else{v1o})});
        let v48y=(v48t/vdw);
        let v493=(((vdw*v48u)-(v17g*v205))/(vdw*vdw));
        let v494=(v48v/vdw);
        let v495=(v48w/vdw);
        let v496=(v48x/vdw);
        let v49f=(if v17k{((v17p*v20r)+(ven*(-(v17o*((-(v213/vew))/sf[58])))))}else{v437});
        let v49j=(if v17k{v1o}else{v43b});
        let v49k=(if v17k{((v17s*v1yx)+(vco*v49f))}else{v43c});
        let v49l=(if v17k{v1o}else{v43d});
        let v49m=(if v17k{v285}else{v43e});
        let v49n=(if v17k{v289}else{v43f});
        let v49o=(v17u*v49j);
        let v49q=(v17u*v49k);
        let v49s=(v17u*v49l);
        let v49u=(v17u*v49m);
        let v49w=(v17u*v49n);
        let v49y=(v2m*v17x);
        let v4a4=(if v17k{((v49o+v49o)/v49y)}else{v43w});
        let v4a5=(if v17k{((v49q+v49q)/v49y)}else{v43x});
        let v4a6=(if v17k{((v49s+v49s)/v49y)}else{v43y});
        let v4a7=(if v17k{((v49u+v49u)/v49y)}else{v43z});
        let v4a8=(if v17k{((v49w+v49w)/v49y)}else{v440});
        let v4aj=(if v17k{(vw*(v49j+v4a4))}else{v44b});
        let v4ak=(if v17k{(vw*(v49k+v4a5))}else{v44c});
        let v4al=(if v17k{(vw*(v49l+v4a6))}else{v44d});
        let v4am=(if v17k{(vw*(v49m+v4a7))}else{v44e});
        let v4an=(if v17k{(vw*(v49n+v4a8))}else{v44f});
        let v4b0=(if v17k{(-(vcm*v4aj))}else{v44s});
        let v4b1=(if v17k{(v49f-((v181*v1yt)+(vcm*v4ak)))}else{v44t});
        let v4b2=(if v17k{(-(vcm*v4al))}else{v44u});
        let v4b3=(if v17k{(-(vcm*v4am))}else{v44v});
        let v4b4=(if v17k{(-(vcm*v4an))}else{v44w});
        let v4cd=(if v17k{((-(v4b0/ven))/v188)}else{v465});
        let v4ce=(if v17k{((-(((ven*v4b1)-(v184*v20r))/v20u))/v188)}else{v466});
        let v4cf=(if v17k{((-(v4b2/ven))/v188)}else{v467});
        let v4cg=(if v17k{((-(v4b3/ven))/v188)}else{v468});
        let v4ch=(if v17k{((-(v4b4/ven))/v188)}else{v469});
        let v4e3=(if v17k{((ven*(-(v18i*(sf[213]*v4cd))))/sf[213])}else{v47v});
        let v4e4=(if v17k{(((v18j*v20r)+(ven*(-(v18i*(sf[213]*v4ce)))))/sf[213])}else{v47w});
        let v4e5=(if v17k{((ven*(-(v18i*(sf[213]*v4cf))))/sf[213])}else{v47x});
        let v4e6=(if v17k{((ven*(-(v18i*(sf[213]*v4cg))))/sf[213])}else{v47y});
        let v4e7=(if v17k{((ven*(-(v18i*(sf[213]*v4ch))))/sf[213])}else{v47z});
        let v4fq=(if sb[11]{v1zw}else{(if (sf[130]!=0.0){v20r}else{v1o})});
        let v4fs=(if sb[28]{(sf[218]*v1yt)}else{v1o});
        let v4g0=(if sb[28]{(((v198*v4fq)-(v199*v4fs))/(v198*v198))}else{v1o});
        let v4g1=(if sb[28]{(sf[273]/v198)}else{v1o});
        let v4g2=(if sb[28]{(sf[0]/v198)}else{v1o});
        let v4g3=(v19b*v4g0);
        let v4g5=(v19b*v4g1);
        let v4g7=(v19b*v4g2);
        let v4g9=(v2m*v19e);
        let v4hl=(if sb[28]{((v19p*(if (sf[148]!=0.0){(sf[125]*(vk9*(sf[126]*v1z2)))}else{v1o}))+(vkb*(-(v19o*(sf[215]*((-(((v190*(if sb[28]{(v4fq-(vw*((v19f*v4fs)+(v198*(v4g0+((v4g3+v4g3)/v4g9))))))}else{v1o}))-(v19j*v4fq))/(v190*v190)))/v19l))))))}else{v1o});
        let v4hm=(if sb[28]{(vkb*(-(v19o*(sf[215]*((-((if sb[28]{(-(vw*(v198*(v4g1+((v4g5+v4g5)/v4g9)))))}else{v1o})/v190))/v19l)))))}else{v1o});
        let v4hn=(if sb[28]{(vkb*(-(v19o*(sf[215]*((-((if sb[28]{(-(vw*(v198*(v4g2+((v4g7+v4g7)/v4g9)))))}else{v1o})/v190))/v19l)))))}else{v1o});
        let v4hu=(v19r*v19r);
        let v4j5=(v1ac*(((v1a5*(if sb[11]{v48y}else{(if (sf[130]!=0.0){((if v18t{v1o}else{(if v17k{(vet*(v4e3+(vew*(-v4b0))))}else{v1o})})/vet)}else{v1o})}))/vkj)+(v42v/sf[219])));
        let v4j6=(v1ac*((((vkj*((v1a5*(if sb[11]{v493}else{(if (sf[130]!=0.0){(((vet*(if v18t{v1o}else{(if v17k{((v18p*v210)+(vet*(v4e4+((v18n*v213)+(vew*(-v4b1))))))}else{v1o})}))-(v18u*v210))/(vet*vet))}else{v1o})}))+(v18z*(if v1a2{(vw*v4hl)}else{(if v19w{(((v19r*(v19x*v4hl))-(v19y*v4hl))/v4hu)}else{v1o})}))))-(v1a6*(if (sf[148]!=0.0){((-(sf[127]*(vkh*((vkf*(sf[78]*v1yx))+(vkc*(vke*(sf[128]*v1z2)))))))/(vkh*vkh))}else{v1o})))/(vkj*vkj))+(v42w/sf[219])));
        let v4j7=(v1ac*(((v1a5*(if sb[11]{v494}else{(if (sf[130]!=0.0){((if v18t{v1o}else{(if v17k{(vet*(v4e5+(vew*(-v4b2))))}else{v1o})})/vet)}else{v1o})}))/vkj)+(v42x/sf[219])));
        let v4j8=(v1ac*((((v1a5*(if sb[11]{v495}else{(if (sf[130]!=0.0){((if v18t{v1o}else{(if v17k{(vet*(v4e6+(vew*(sf[0]-v4b3))))}else{v1o})})/vet)}else{v1o})}))+(v18z*(if v1a2{(vw*v4hm)}else{(if v19w{(((v19r*(v19x*v4hm))-(v19y*v4hm))/v4hu)}else{v1o})})))/vkj)+(v42y/sf[219])));
        let v4j9=(v1ac*(((v1a5*(if sb[11]{v496}else{(if (sf[130]!=0.0){((if v18t{v1o}else{(if v17k{(vet*(v4e7+(vew*(sf[273]-v4b4))))}else{v1o})})/vet)}else{v1o})}))+(v18z*(if v1a2{(vw*v4hn)}else{(if v19w{(((v19r*(v19x*v4hn))-(v19y*v4hn))/v4hu)}else{v1o})})))/vkj));
        let v4ja=(v1ae*v4j5);
        let v4jc=(v1ae*v4j6);
        let v4je=(v1ae*v4j7);
        let v4jg=(v1ae*v4j8);
        let v4ji=(v1ae*v4j9);
        let v4jk=(v2m*v1ai);
        let v4k0=(v1af*((v4j5+((v4ja+v4ja)/v4jk))/v2m));
        let v4k1=(v1af*((v4j6+((v4jc+v4jc)/v4jk))/v2m));
        let v4k2=(v1af*((v4j7+((v4je+v4je)/v4jk))/v2m));
        let v4k3=(v1af*((v4j8+((v4jg+v4jg)/v4jk))/v2m));
        let v4k4=(v1af*((v4j9+((v4ji+v4ji)/v4jk))/v2m));
        let v4ka=(v164*v164);
        let v4kj=((v23k+(sf[220]*v42s))+(sf[221]*((-v42s)/v4ka)));
        let v4kk=((sf[220]*v42t)+(sf[221]*((-v42t)/v4ka)));
        let v4kl=((sf[220]*v42u)+(sf[221]*((-v42u)/v4ka)));
        let v4kz=(v1b2*v1b2);
        let v4la=(if sb[30]{v22q}else{(if (sf[223]!=0.0){(((v1b2*v22q)-(vgl*(if (sf[223]!=0.0){(((vhf*v4kj)-(v1av*v23k))/(vhf*vhf))}else{v1o})))/v4kz)}else{v1o})});
        let v4lb=(if sb[30]{v1o}else{(if (sf[223]!=0.0){((-(vgl*(if (sf[223]!=0.0){(v4kk/vhf)}else{v1o})))/v4kz)}else{v1o})});
        let v4lc=(if sb[30]{v1o}else{(if (sf[223]!=0.0){((-(vgl*(if (sf[223]!=0.0){(v4kl/vhf)}else{v1o})))/v4kz)}else{v1o})});
        let v4lh=((-(va*(sf[225]*v1yt)))/(v1b9*v1b9));
        let v4li=(sf[0]/v1b9);
        let v4lj=(sf[273]/v1b9);
        let v4lt=scalar_limexp_derivative(v1bg);
        let v4m8=((v1bk*v22k)+(vgf*((v1bj*(if v1bh{v1o}else{(if (v1bc!=0.0){v4lh}else{v1o})}))+(v1bi*((if (v1bc!=0.0){v1o}else{v4lh})*v4lt)))));
        let v4m9=(vgf*((v1bj*(if v1bh{v1o}else{(if (v1bc!=0.0){v4li}else{v1o})}))+(v1bi*((if (v1bc!=0.0){v1o}else{v4li})*v4lt))));
        let v4ma=(vgf*((v1bj*(if v1bh{v1o}else{(if (v1bc!=0.0){v4lj}else{v1o})}))+(v1bi*((if (v1bc!=0.0){v1o}else{v4lj})*v4lt))));
        let v4mf=((-(v7*(sf[226]*v1yt)))/(v1bn*v1bn));
        let v4mg=(sf[273]/v1bn);
        let v4mh=(sf[0]/v1bn);
        let v4mr=scalar_limexp_derivative(v1bu);
        let v4n6=((v1by*v22k)+(vgf*((v1bx*(if v1bv{v1o}else{(if (v1bq!=0.0){v4mf}else{v1o})}))+(v1bw*((if (v1bq!=0.0){v1o}else{v4mf})*v4mr)))));
        let v4n7=(vgf*((v1bx*(if v1bv{v1o}else{(if (v1bq!=0.0){v4mg}else{v1o})}))+(v1bw*((if (v1bq!=0.0){v1o}else{v4mg})*v4mr))));
        let v4n8=(vgf*((v1bx*(if v1bv{v1o}else{(if (v1bq!=0.0){v4mh}else{v1o})}))+(v1bw*((if (v1bq!=0.0){v1o}else{v4mh})*v4mr))));
        let v4nc=(v1b6*v1b6);
        let v4nl=(v4ma/v1b6);
        let v4np=((((v1b6*v4m8)-(v1bl*v4la))/v4nc)+(v4n6/sf[224]));
        let v4nq=(((-(v1bl*v4lb))/v4nc)+(v4n7/sf[224]));
        let v4nr=((((v1b6*v4m9)-(v1bl*v4lc))/v4nc)+(v4n8/sf[224]));
        let v4nv=(v15v*v15v);
        let v4ol=(vkv*vkv);
        let v4p1=(v1cc*(v1c5*(((v1c8*((v1c6*v4m8)+(v1bl*(((v15v*v4m8)-(v1bl*v41s))/v4nv))))+(v1c7*(((vkv*v270)-(vkz*v26u))/v4ol)))/v1c9)));
        let v4p2=(v1cc*(v1c5*((v1c8*(v1bl*((-(v1bl*v41v))/v4nv)))/v1c9)));
        let v4p3=(v1cc*(v1c5*((v1c8*((v1c6*v4m9)+(v1bl*(((v15v*v4m9)-(v1bl*v41y))/v4nv))))/v1c9)));
        let v4p4=(v1cc*(v1c5*((v1c8*((v1c6*v4ma)+(v1bl*(((v15v*v4ma)-(v1bl*v421))/v4nv))))/v1c9)));
        let v4pg=(((vkv*v4m8)-(v1bl*v26u))/v4ol);
        let v4ph=(v4m9/vkv);
        let v4pi=(v4ma/vkv);
        let v4pj=(v4np+v4pg);
        let v4pk=(v4nr+v4ph);
        let v4pl=(v4nl+v4pi);
        let v4pz=(v1am*v4k0);
        let v4q0=(v4pz+v4pz);
        let v4q1=(v1am*v4k1);
        let v4q2=(v4q1+v4q1);
        let v4q3=(v1am*v4k2);
        let v4q4=(v4q3+v4q3);
        let v4q5=(v1am*v4k3);
        let v4q6=(v4q5+v4q5);
        let v4q7=(v1am*v4k4);
        let v4q8=(v4q7+v4q7);
        let v4qa=((if sb[32]{v4nq}else{(if (sf[227]!=0.0){(v4nq+v4p2)}else{v1o})})+v4q4);
        let v4qd=(v2m*v1co);
        let v4qj=(v4k0+(v4q0/v4qd));
        let v4qk=(v4k1+(((if sb[32]{v4np}else{(if (sf[227]!=0.0){(v4np+v4p1)}else{v1o})})+v4q2)/v4qd));
        let v4ql=(v4k2+(v4qa/v4qd));
        let v4qm=(v4k3+(((if sb[32]{v4nr}else{(if (sf[227]!=0.0){(v4nr+v4p3)}else{v1o})})+v4q6)/v4qd));
        let v4qn=(v4k4+(((if sb[32]{v4nl}else{(if (sf[227]!=0.0){(v4nl+v4p4)}else{v1o})})+v4q8)/v4qd));
        let v4qr=(v2m*v1cr);
        let v4r9=(v1bl*v1bl);
        let v4ra=(((v1bl*(v41s/sf[228]))-(v1cz*v4m8))/v4r9);
        let v4rb=((v41v/sf[228])/v1bl);
        let v4rf=(((v1bl*(v41y/sf[228]))-(v1cz*v4m9))/v4r9);
        let v4rj=(((v1bl*(v421/sf[228]))-(v1cz*v4ma))/v4r9);
        let v4sx=(v1d7*v1d7);
        let v4tf=(if (v1cx!=0.0){(((v1d7*(if (v1cx!=0.0){(-(v1d0*v4qj))}else{v1o}))-(v1d3*(if (v1cx!=0.0){(v1d0*((v4k0+(v4q0/v4qr))-v4qj))}else{v1o})))/v4sx)}else{v1o});
        let v4tg=(if (v1cx!=0.0){(((v1d7*(if (v1cx!=0.0){(-((v1d0*v4qk)+(v1cp*v4ra)))}else{v1o}))-(v1d3*(if (v1cx!=0.0){((v1d4*v4ra)+(v1d0*((v4k1+(((if sb[32]{v4pj}else{(if (sf[227]!=0.0){(v4p1+v4pj)}else{v1o})})+v4q2)/v4qr))-v4qk)))}else{v1o})))/v4sx)}else{v1o});
        let v4th=(if (v1cx!=0.0){(((v1d7*(if (v1cx!=0.0){(-((v1d0*v4ql)+(v1cp*v4rb)))}else{v1o}))-(v1d3*(if (v1cx!=0.0){((v1d4*v4rb)+(v1d0*((v4k2+(v4qa/v4qr))-v4ql)))}else{v1o})))/v4sx)}else{v1o});
        let v4ti=(if (v1cx!=0.0){(((v1d7*(if (v1cx!=0.0){(-((v1d0*v4qm)+(v1cp*v4rf)))}else{v1o}))-(v1d3*(if (v1cx!=0.0){((v1d4*v4rf)+(v1d0*((v4k3+(((if sb[32]{v4pk}else{(if (sf[227]!=0.0){(v4p3+v4pk)}else{v1o})})+v4q6)/v4qr))-v4qm)))}else{v1o})))/v4sx)}else{v1o});
        let v4tj=(if (v1cx!=0.0){(((v1d7*(if (v1cx!=0.0){(-((v1d0*v4qn)+(v1cp*v4rj)))}else{v1o}))-(v1d3*(if (v1cx!=0.0){((v1d4*v4rj)+(v1d0*((v4k4+(((if sb[32]{v4pl}else{(if (sf[227]!=0.0){(v4p4+v4pl)}else{v1o})})+v4q8)/v4qr))-v4qn)))}else{v1o})))/v4sx)}else{v1o});
        let v4tk=(v1d9*v4tf);
        let v4tm=(v1d9*v4tg);
        let v4to=(v1d9*v4th);
        let v4tq=(v1d9*v4ti);
        let v4ts=(v1d9*v4tj);
        let v4tu=(v2m*v1dd);
        let v4uf=(if v1di{v1o}else{(if (v1cx!=0.0){((v4tf+((v4tk+v4tk)/v4tu))/v1df)}else{v1o})});
        let v4ug=(if v1di{v1o}else{(if (v1cx!=0.0){((v4tg+((v4tm+v4tm)/v4tu))/v1df)}else{v1o})});
        let v4uh=(if v1di{v1o}else{(if (v1cx!=0.0){((v4th+((v4to+v4to)/v4tu))/v1df)}else{v1o})});
        let v4ui=(if v1di{v1o}else{(if (v1cx!=0.0){((v4ti+((v4tq+v4tq)/v4tu))/v1df)}else{v1o})});
        let v4uj=(if v1di{v1o}else{(if (v1cx!=0.0){((v4tj+((v4ts+v4ts)/v4tu))/v1df)}else{v1o})});
        let v4ux=((v1do*v4uf)+(v1dj*(v1cf*v4uf)));
        let v4va=(v4np+((v1do*v4ug)+(v1dj*((v1dj*v4pg)+(v1cf*v4ug)))));
        let v4vb=(v4nq+((v1do*v4uh)+(v1dj*(v1cf*v4uh))));
        let v4vc=(v4nr+((v1do*v4ui)+(v1dj*((v1dj*v4ph)+(v1cf*v4ui)))));
        let v4vd=(v4nl+((v1do*v4uj)+(v1dj*((v1dj*v4pi)+(v1cf*v4uj)))));
        let v4vx=(v2m*v1dw);
        let v4wi=(if sb[36]{(v1e2*v4k0)}else{v1o});
        let v4wj=(if sb[36]{(v1e2*v4k1)}else{v1o});
        let v4wk=(if sb[36]{(v1e2*v4k2)}else{v1o});
        let v4wl=(if sb[36]{(v1e2*v4k3)}else{v1o});
        let v4wm=(if sb[36]{(v1e2*v4k4)}else{v1o});
        let v4ws=(if sb[41]{(-v4ux)}else{v1o});
        let v4wt=(if sb[41]{(-v4va)}else{v1o});
        let v4wu=(if sb[41]{(-v4vb)}else{v1o});
        let v4wv=(if sb[41]{(-v4vc)}else{v1o});
        let v4ww=(if sb[41]{(-v4vd)}else{v1o});
        let v4y5=(v1e4*v4wi);
        let v4y7=(v1e4*v4wj);
        let v4y9=(v1e4*v4wk);
        let v4yb=(v1e4*v4wl);
        let v4yd=(v1e4*v4wm);
        let v4yf=(if sb[36]{(v4y5+v4y5)}else{v1o});
        let v4yg=(if sb[36]{(v4y7+v4y7)}else{v1o});
        let v4yh=(if sb[36]{(v4y9+v4y9)}else{v1o});
        let v4yi=(if sb[36]{(v4yb+v4yb)}else{v1o});
        let v4yj=(if sb[36]{(v4yd+v4yd)}else{v1o});
        let v4yu=(if sb[36]{(v4ws-(sf[231]*v4yf))}else{v1o});
        let v4yv=(if sb[36]{(v4wt-(sf[231]*v4yg))}else{v1o});
        let v4yw=(if sb[36]{(v4wu-(sf[231]*v4yh))}else{v1o});
        let v4yx=(if sb[36]{(v4wv-(sf[231]*v4yi))}else{v1o});
        let v4yy=(if sb[36]{(v4ww-(sf[231]*v4yj))}else{v1o});
        let v50h=(if sb[36]{((((v1ep*v4yf)+(v1el*(v2m*v4wi)))/v1er)-(sf[231]*((v1ed*v4wi)+(v1e4*v4ws))))}else{v1o});
        let v50i=(if sb[36]{((if sb[36]{(((vkv*((v1eg*v270)+(vkz*(((v15v*((v1ee*v4m8)+(v1bl*(-v4m8))))-(v1ef*v41s))/v4nv))))-(v1eh*v26u))/v4ol)}else{v1o})+((((v1ep*v4yg)+(v1el*(v2m*v4wj)))/v1er)-(sf[231]*((v1ed*v4wj)+(v1e4*v4wt)))))}else{v1o});
        let v50j=(if sb[36]{((if sb[36]{((vkz*((-(v1ef*v41v))/v4nv))/vkv)}else{v1o})+((((v1ep*v4yh)+(v1el*(v2m*v4wk)))/v1er)-(sf[231]*((v1ed*v4wk)+(v1e4*v4wu)))))}else{v1o});
        let v50k=(if sb[36]{((if sb[36]{((vkz*(((v15v*((v1ee*v4m9)+(v1bl*(-v4m9))))-(v1ef*v41y))/v4nv))/vkv)}else{v1o})+((((v1ep*v4yi)+(v1el*(v2m*v4wl)))/v1er)-(sf[231]*((v1ed*v4wl)+(v1e4*v4wv)))))}else{v1o});
        let v50l=(if sb[36]{((if sb[36]{((vkz*(((v15v*((v1ee*v4ma)+(v1bl*(-v4ma))))-(v1ef*v421))/v4nv))/vkv)}else{v1o})+((((v1ep*v4yj)+(v1el*(v2m*v4wm)))/v1er)-(sf[231]*((v1ed*v4wm)+(v1e4*v4ww)))))}else{v1o});
        let v50m=(v1ex*v50h);
        let v50o=(v1ex*v50i);
        let v50q=(v1ex*v50j);
        let v50s=(v1ex*v50k);
        let v50u=(v1ex*v50l);
        let v511=(v1eo*v4yu);
        let v513=(v1eo*v4yv);
        let v515=(v1eo*v4yw);
        let v517=(v1eo*v4yx);
        let v519=(v1eo*v4yy);
        let v51d=((v1f1*v4yu)+(v1eo*(v511+v511)));
        let v51g=((v1f1*v4yv)+(v1eo*(v513+v513)));
        let v51j=((v1f1*v4yw)+(v1eo*(v515+v515)));
        let v51m=((v1f1*v4yx)+(v1eo*(v517+v517)));
        let v51p=((v1f1*v4yy)+(v1eo*(v519+v519)));
        let v52u=(sf[231]*v4wi);
        let v52v=(sf[231]*v4wj);
        let v52w=(sf[231]*v4wk);
        let v52x=(sf[231]*v4wl);
        let v52y=(sf[231]*v4wm);
        let v53e=(vw*(-v50h));
        let v53f=(vw*(-v50i));
        let v53g=(vw*(-v50j));
        let v53h=(vw*(-v50k));
        let v53i=(vw*(-v50l));
        let v53j=(if v1fk{v53e}else{v1o});
        let v53k=(if v1fk{v53f}else{v1o});
        let v53l=(if v1fk{v53g}else{v1o});
        let v53m=(if v1fk{v53h}else{v1o});
        let v53n=(if v1fk{v53i}else{v1o});
        let v53o=(v2m*v1fo);
        let v53u=(if v1fk{((if sb[36]{((v1ez*(v50m+v50m))+(v51d/v1er))}else{v1o})/v53o)}else{v1o});
        let v53v=(if v1fk{((if sb[36]{((v1ez*(v50o+v50o))+(v51g/v1er))}else{v1o})/v53o)}else{v1o});
        let v53w=(if v1fk{((if sb[36]{((v1ez*(v50q+v50q))+(v51j/v1er))}else{v1o})/v53o)}else{v1o});
        let v53x=(if v1fk{((if sb[36]{((v1ez*(v50s+v50s))+(v51m/v1er))}else{v1o})/v53o)}else{v1o});
        let v53y=(if v1fk{((if sb[36]{((v1ez*(v50u+v50u))+(v51p/v1er))}else{v1o})/v53o)}else{v1o});
        let v544=(if v1fk{(v53j+v53u)}else{v4yf});
        let v545=(if v1fk{(v53k+v53v)}else{v4yg});
        let v546=(if v1fk{(v53l+v53w)}else{v4yh});
        let v547=(if v1fk{(v53m+v53x)}else{v4yi});
        let v548=(if v1fk{(v53n+v53y)}else{v4yj});
        let v55s=(if v1fk{(v53j-v53u)}else{v544});
        let v55t=(if v1fk{(v53k-v53v)}else{v545});
        let v55u=(if v1fk{(v53l-v53w)}else{v546});
        let v55v=(if v1fk{(v53m-v53x)}else{v547});
        let v55w=(if v1fk{(v53n-v53y)}else{v548});
        let v57s=(v1f2*v1f2);
        let v586=(v2m*v1gv);
        let v58r=(if v1gs{((v1gv*v53e)+(v1fm*(((-(v1gt*v51d))/v57s)/v586)))}else{v55s});
        let v58s=(if v1gs{((v1gv*v53f)+(v1fm*(((-(v1gt*v51g))/v57s)/v586)))}else{v55t});
        let v58t=(if v1gs{((v1gv*v53g)+(v1fm*(((-(v1gt*v51j))/v57s)/v586)))}else{v55u});
        let v58u=(if v1gs{((v1gv*v53h)+(v1fm*(((-(v1gt*v51m))/v57s)/v586)))}else{v55v});
        let v58v=(if v1gs{((v1gv*v53i)+(v1fm*(((-(v1gt*v51p))/v57s)/v586)))}else{v55w});
        let v58w=(v1gx*v58r);
        let v58y=(v1gx*v58s);
        let v590=(v1gx*v58t);
        let v592=(v1gx*v58u);
        let v594=(v1gx*v58v);
        let v596=(if v1gs{(v58w+v58w)}else{v53j});
        let v597=(if v1gs{(v58y+v58y)}else{v53k});
        let v598=(if v1gs{(v590+v590)}else{v53l});
        let v599=(if v1gs{(v592+v592)}else{v53m});
        let v59a=(if v1gs{(v594+v594)}else{v53n});
        let v59j=(v1h4*v1h4);
        let v5a1=(v2m*v1h6);
        let v5a8=(v1c+(v1h6*v1h6));
        let v5a9=(((((v1h4*v596)-(v1gz*(-v596)))/v59j)/v5a1)/v5a8);
        let v5aa=(((((v1h4*v597)-(v1gz*(-v597)))/v59j)/v5a1)/v5a8);
        let v5ab=(((((v1h4*v598)-(v1gz*(-v598)))/v59j)/v5a1)/v5a8);
        let v5ac=(((((v1h4*v599)-(v1gz*(-v599)))/v59j)/v5a1)/v5a8);
        let v5ad=(((((v1h4*v59a)-(v1gz*(-v59a)))/v59j)/v5a1)/v5a8);
        let v5ao=(if v1hb{v5a9}else{(if v1h2{(-v5a9)}else{v58r})});
        let v5ap=(if v1hb{v5aa}else{(if v1h2{(-v5aa)}else{v58s})});
        let v5aq=(if v1hb{v5ab}else{(if v1h2{(-v5ab)}else{v58t})});
        let v5ar=(if v1hb{v5ac}else{(if v1h2{(-v5ac)}else{v58u})});
        let v5as=(if v1hb{v5ad}else{(if v1h2{(-v5ad)}else{v58v})});
        let v5b3=(v2m*v1hh);
        let v5be=(v1hi).sin();
        let v5co=(if (v1hr!=0.0){v1o}else{(if sb[36]{(if v1gs{(if v1gs{(((v1hj*((sf[231]*(v1he*v4yu))/v5b3))+(v1hh*(-((sf[231]*v5ao)*v5be))))-v52u)}else{v5ao})}else{(if v1fk{(((if v1g0{(-(v1g4*(sf[231]*((-v544)/v1g1))))}else{(if v1fu{(v1fx*(sf[231]*(v544/v1fr)))}else{v1o})})+(if v1gh{(-(v1gl*(sf[231]*((-v55s)/v1gi))))}else{(if v1gb{(v1ge*(sf[231]*(v55s/v1g8)))}else{v1o})}))-v52u)}else{(if v1fa{((((v1eo*(v17*v50h))-(v1fb*v4yu))/v1f1)-v52u)}else{v1o})})})}else{(if (sf[230]!=0.0){(v4k0+((v4q0+(if sb[35]{v4ux}else{(if sb[34]{v4ux}else{v1o})}))/v4vx))}else{v1o})})});
        let v5cp=(if (v1hr!=0.0){v1o}else{(if sb[36]{(if v1gs{(if v1gs{(((v1hj*((sf[231]*(v1he*v4yv))/v5b3))+(v1hh*(-((sf[231]*v5ap)*v5be))))-v52v)}else{v5ap})}else{(if v1fk{(((if v1g0{(-(v1g4*(sf[231]*((-v545)/v1g1))))}else{(if v1fu{(v1fx*(sf[231]*(v545/v1fr)))}else{v1o})})+(if v1gh{(-(v1gl*(sf[231]*((-v55t)/v1gi))))}else{(if v1gb{(v1ge*(sf[231]*(v55t/v1g8)))}else{v1o})}))-v52v)}else{(if v1fa{((((v1eo*(v17*v50i))-(v1fb*v4yv))/v1f1)-v52v)}else{v1o})})})}else{(if (sf[230]!=0.0){(v4k1+((v4q2+(if sb[35]{v4va}else{(if sb[34]{(v4p1+v4va)}else{v1o})}))/v4vx))}else{v1o})})});
        let v5cq=(if (v1hr!=0.0){v1o}else{(if sb[36]{(if v1gs{(if v1gs{(((v1hj*((sf[231]*(v1he*v4yw))/v5b3))+(v1hh*(-((sf[231]*v5aq)*v5be))))-v52w)}else{v5aq})}else{(if v1fk{(((if v1g0{(-(v1g4*(sf[231]*((-v546)/v1g1))))}else{(if v1fu{(v1fx*(sf[231]*(v546/v1fr)))}else{v1o})})+(if v1gh{(-(v1gl*(sf[231]*((-v55u)/v1gi))))}else{(if v1gb{(v1ge*(sf[231]*(v55u/v1g8)))}else{v1o})}))-v52w)}else{(if v1fa{((((v1eo*(v17*v50j))-(v1fb*v4yw))/v1f1)-v52w)}else{v1o})})})}else{(if (sf[230]!=0.0){(v4k2+((v4q4+(if sb[35]{v4vb}else{(if sb[34]{(v4p2+v4vb)}else{v1o})}))/v4vx))}else{v1o})})});
        let v5cr=(if (v1hr!=0.0){v1o}else{(if sb[36]{(if v1gs{(if v1gs{(((v1hj*((sf[231]*(v1he*v4yx))/v5b3))+(v1hh*(-((sf[231]*v5ar)*v5be))))-v52x)}else{v5ar})}else{(if v1fk{(((if v1g0{(-(v1g4*(sf[231]*((-v547)/v1g1))))}else{(if v1fu{(v1fx*(sf[231]*(v547/v1fr)))}else{v1o})})+(if v1gh{(-(v1gl*(sf[231]*((-v55v)/v1gi))))}else{(if v1gb{(v1ge*(sf[231]*(v55v/v1g8)))}else{v1o})}))-v52x)}else{(if v1fa{((((v1eo*(v17*v50k))-(v1fb*v4yx))/v1f1)-v52x)}else{v1o})})})}else{(if (sf[230]!=0.0){(v4k3+((v4q6+(if sb[35]{v4vc}else{(if sb[34]{(v4p3+v4vc)}else{v1o})}))/v4vx))}else{v1o})})});
        let v5cs=(if (v1hr!=0.0){v1o}else{(if sb[36]{(if v1gs{(if v1gs{(((v1hj*((sf[231]*(v1he*v4yy))/v5b3))+(v1hh*(-((sf[231]*v5as)*v5be))))-v52y)}else{v5as})}else{(if v1fk{(((if v1g0{(-(v1g4*(sf[231]*((-v548)/v1g1))))}else{(if v1fu{(v1fx*(sf[231]*(v548/v1fr)))}else{v1o})})+(if v1gh{(-(v1gl*(sf[231]*((-v55w)/v1gi))))}else{(if v1gb{(v1ge*(sf[231]*(v55w/v1g8)))}else{v1o})}))-v52y)}else{(if v1fa{((((v1eo*(v17*v50l))-(v1fb*v4yy))/v1f1)-v52y)}else{v1o})})})}else{(if (sf[230]!=0.0){(v4k4+((v4q8+(if sb[35]{v4vd}else{(if sb[34]{(v4p4+v4vd)}else{v1o})}))/v4vx))}else{v1o})})});
        let v5cv=(v1hs*v1hs);
        let v5de=((-(v1bz*v5co))/v5cv);
        let v5di=(((v1hs*v4n6)-(v1bz*v5cp))/v5cv);
        let v5dm=(((v1hs*v4n7)-(v1bz*v5cq))/v5cv);
        let v5dq=(((v1hs*v4n8)-(v1bz*v5cr))/v5cv);
        let v5dt=((-(v1bz*v5cs))/v5cv);
        let v5du=(if (v1hw!=0.0){v1o}else{((-(v1bl*v5co))/v5cv)});
        let v5dv=(if (v1hw!=0.0){v1o}else{(((v1hs*v4m8)-(v1bl*v5cp))/v5cv)});
        let v5dw=(if (v1hw!=0.0){v1o}else{((-(v1bl*v5cq))/v5cv)});
        let v5dx=(if (v1hw!=0.0){v1o}else{(((v1hs*v4m9)-(v1bl*v5cr))/v5cv)});
        let v5dy=(if (v1hw!=0.0){v1o}else{(((v1hs*v4ma)-(v1bl*v5cs))/v5cv)});
        let v5eh=(v1hx*v1hx);
        let v5ej=(v1hx*v41s);
        let v5ek=(v15v*v5dv);
        let v5en=(v1hx*v41v);
        let v5eo=(v15v*v5dw);
        let v5er=(v1hx*v41y);
        let v5es=(v15v*v5dx);
        let v5ev=(v1hx*v421);
        let v5ew=(v15v*v5dy);
        let v5ez=(-((-(v15v*v5du))/v5eh));
        let v5f0=(-((v5ej-v5ek)/v5eh));
        let v5f1=(-((v5en-v5eo)/v5eh));
        let v5f2=(-((v5er-v5es)/v5eh));
        let v5f3=(-((v5ev-v5ew)/v5eh));
        let v5f4=(v1i1*v5ez);
        let v5f6=(v1i1*v5f0);
        let v5f8=(v1i1*v5f1);
        let v5fa=(v1i1*v5f2);
        let v5fc=(v1i1*v5f3);
        let v5fe=(v2m*v1i5);
        let v5fp=((v5ez+((v5f4+v5f4)/v5fe))/sf[236]);
        let v5fq=((v5f0+((v5f6+v5f6)/v5fe))/sf[236]);
        let v5fr=((v5f1+((v5f8+v5f8)/v5fe))/sf[236]);
        let v5fs=((v5f2+((v5fa+v5fa)/v5fe))/sf[236]);
        let v5ft=((v5f3+((v5fc+v5fc)/v5fe))/sf[236]);
        let v5if=(((v1ic*v5du)+(v1hx*((v1ib*v5fp)+(v1ia*(vhs*v5fp)))))+((v1av*v5du)+(((v1ij*v5du)+(v1hx*(vho*(v1ii*(sf[237]*((v5du/v15v)/v1if))))))/sf[238])));
        let v5ig=(((v1ic*v5dv)+(v1hx*((v1ib*v5fq)+(v1ia*((v1ia*(if (sf[148]!=0.0){(sf[93]*(vhq*(sf[94]*v1z2)))}else{v1o}))+(vhs*v5fq))))))+(((v1hx*v4kj)+(v1av*v5dv))+(((v1ij*v5dv)+(v1hx*((v1ii*(if sb[18]{v1o}else{(if sb[17]{(sf[92]*(vhk*((sf[28]*v1z2)-v23m)))}else{v1o})}))+(vho*(v1ii*(sf[237]*(((v5ek-v5ej)/v4nv)/v1if)))))))/sf[238])));
        let v5ih=(((v1ic*v5dw)+(v1hx*((v1ib*v5fr)+(v1ia*(vhs*v5fr)))))+(((v1hx*v4kk)+(v1av*v5dw))+(((v1ij*v5dw)+(v1hx*(vho*(v1ii*(sf[237]*(((v5eo-v5en)/v4nv)/v1if))))))/sf[238])));
        let v5ii=(((v1ic*v5dx)+(v1hx*((v1ib*v5fs)+(v1ia*(vhs*v5fs)))))+(((v1hx*v4kl)+(v1av*v5dx))+(((v1ij*v5dx)+(v1hx*(vho*(v1ii*(sf[237]*(((v5es-v5er)/v4nv)/v1if))))))/sf[238])));
        let v5ij=(((v1ic*v5dy)+(v1hx*((v1ib*v5ft)+(v1ia*(vhs*v5ft)))))+((v1av*v5dy)+(((v1ij*v5dy)+(v1hx*(vho*(v1ii*(sf[237]*(((v5ew-v5ev)/v4nv)/v1if))))))/sf[238])));
        let v5mr=(if v1ks{v27p}else{v3fp});
        let v5ms=(if v1ks{v27r}else{v3fq});
        let v5mu=(if v1ks{(vm3*v228)}else{v3fs});
        let v5n0=(if v1ks{((v1l0*v228)+(vg3*(v1l0*(v1ky*v27y))))}else{v3fy});
        let v5n4=(if v1ks{v1o}else{v3g2});
        let v5n5=(if v1ks{((v1l3*v1yx)+(vco*v5ms))}else{v3g3});
        let v5n6=(if v1ks{v289}else{v3g4});
        let v5n7=(if v1ks{v285}else{v3g5});
        let v5nc=(if v1l8{(v1l9*v5n4)}else{v3i9});
        let v5nd=(if v1l8{(v1l9*v5n5)}else{v3ia});
        let v5ne=(if v1l8{(v1l9*v5n6)}else{v3ib});
        let v5nf=(if v1l8{(v1l9*v5n7)}else{v3ic});
        let v5on=(if v1lj{v1o}else{(if v1l8{(-(vcm*(v5nc/v1lb)))}else{v3hl})});
        let v5oo=(if v1lj{v1o}else{(if v1l8{(v5ms-((v1le*v1yt)+(vcm*(v5nd/v1lb))))}else{v3hm})});
        let v5op=(if v1lj{sf[273]}else{(if v1l8{(-(vcm*(v5ne/v1lb)))}else{v3hn})});
        let v5oq=(if v1lj{sf[0]}else{(if v1l8{(-(vcm*(v5nf/v1lb)))}else{v3ho})});
        let v5ot=(if v1ks{(v29k+(vn3*v5mr))}else{v3hr});
        let v5oz=(v1lo*v1lo);
        let v5p3=(if v1ks{(v5on/v1lo)}else{v3i1});
        let v5p4=(if v1ks{(((v1lo*(v5mr+v5oo))-(v1lp*v5ot))/v5oz)}else{v3i2});
        let v5p5=(if v1ks{(v5op/v1lo)}else{v3i3});
        let v5p6=(if v1ks{(v5oq/v1lo)}else{v3i4});
        let v5pb=(if v1lu{(v1lv*v5p3)}else{v5nc});
        let v5pc=(if v1lu{(v1lv*v5p4)}else{v5nd});
        let v5pd=(if v1lu{(v1lv*v5p5)}else{v5ne});
        let v5pe=(if v1lu{(v1lv*v5p6)}else{v5nf});
        let v5qs=(if v1mb{v5on}else{(if v1lu{(v1lo*(v5pb/v1lx))}else{v3jq})});
        let v5qt=(if v1mb{v5oo}else{(if v1lu{((-v5mr)+((v1m6*v5ot)+(v1lo*((v5pc/v1lx)-(v1m5*(((v1lo*(-(v5mr+v5ms)))-(v1m3*v5ot))/v5oz))))))}else{v3jr})});
        let v5qu=(if v1mb{v5op}else{(if v1lu{(v1lo*(v5pd/v1lx))}else{v3js})});
        let v5qv=(if v1mb{v5oq}else{(if v1lu{(v1lo*(v5pe/v1lx))}else{v3jt})});
        let v5rj=(if v1ks{((-(v5on/vfx))/v1mh)}else{v3kh});
        let v5rk=(if v1ks{((-(((vfx*v5oo)-(v1ll*v21z))/v222))/v1mh)}else{v3ki});
        let v5rl=(if v1ks{((-(v5op/vfx))/v1mh)}else{v3kj});
        let v5rm=(if v1ks{((-(v5oq/vfx))/v1mh)}else{v3kk});
        let v5s2=(if v1ks{((-(v5qs/vfx))/v1ml)}else{v3l0});
        let v5s3=(if v1ks{((-(((vfx*v5qt)-(v1md*v21z))/v222))/v1ml)}else{v3l1});
        let v5s4=(if v1ks{((-(v5qu/vfx))/v1ml)}else{v3l2});
        let v5s5=(if v1ks{((-(v5qv/vfx))/v1ml)}else{v3l3});
        let v5xa=(if v1nv{v27r}else{v49f});
        let v5xe=(if v1nv{v1o}else{v49j});
        let v5xf=(if v1nv{((v1nx*v1yx)+(vco*v5xa))}else{v49k});
        let v5xg=(if v1nv{v289}else{v49l});
        let v5xh=(if v1nv{v285}else{v49m});
        let v5xi=(if v1nv{v1o}else{v49n});
        let v5xj=(v1nz*v5xe);
        let v5xl=(v1nz*v5xf);
        let v5xn=(v1nz*v5xg);
        let v5xp=(v1nz*v5xh);
        let v5xr=(v1nz*v5xi);
        let v5xt=(v2m*v1o2);
        let v5xz=(if v1nv{((v5xj+v5xj)/v5xt)}else{v4a4});
        let v5y0=(if v1nv{((v5xl+v5xl)/v5xt)}else{v4a5});
        let v5y1=(if v1nv{((v5xn+v5xn)/v5xt)}else{v4a6});
        let v5y2=(if v1nv{((v5xp+v5xp)/v5xt)}else{v4a7});
        let v5y3=(if v1nv{((v5xr+v5xr)/v5xt)}else{v4a8});
        let v5ye=(if v1nv{(vw*(v5xe+v5xz))}else{v4aj});
        let v5yf=(if v1nv{(vw*(v5xf+v5y0))}else{v4ak});
        let v5yg=(if v1nv{(vw*(v5xg+v5y1))}else{v4al});
        let v5yh=(if v1nv{(vw*(v5xh+v5y2))}else{v4am});
        let v5yi=(if v1nv{(vw*(v5xi+v5y3))}else{v4an});
        let v5yv=(if v1nv{(-(vcm*v5ye))}else{v4b0});
        let v5yw=(if v1nv{(v5xa-((v1o6*v1yt)+(vcm*v5yf)))}else{v4b1});
        let v5yx=(if v1nv{(-(vcm*v5yg))}else{v4b2});
        let v5yy=(if v1nv{(-(vcm*v5yh))}else{v4b3});
        let v5yz=(if v1nv{(-(vcm*v5yi))}else{v4b4});
        let v608=(if v1nv{((-(v5yv/vfx))/v1od)}else{v4cd});
        let v609=(if v1nv{((-(((vfx*v5yw)-(v1o9*v21z))/v222))/v1od)}else{v4ce});
        let v60a=(if v1nv{((-(v5yx/vfx))/v1od)}else{v4cf});
        let v60b=(if v1nv{((-(v5yy/vfx))/v1od)}else{v4cg});
        let v60c=(if v1nv{((-(v5yz/vfx))/v1od)}else{v4ch});
        let v6gk=(if v1sg{(-v25l)}else{v5mr});
        let v6gl=(sf[258]*v25l);
        let v6gm=(if v1sg{v6gl}else{v5ms});
        let v6gy=(if v1sg{((v1sw*v25u)+(vjx*(v1sw*(v1ss*(((-(sf[253]*v25l))/v25o)/v1st)))))}else{v5n0});
        let v6hc=(if v1t4{(v1t5*(if v1sg{v1o}else{v5n4}))}else{v5pb});
        let v6hd=(if v1t4{(v1t5*(if v1sg{v285}else{v1o}))}else{v1o});
        let v6he=(if v1t4{(v1t5*(if v1sg{((v1sz*v1yx)+(vco*v6gm))}else{v5n5}))}else{v5pc});
        let v6hf=(if v1t4{(v1t5*(if v1sg{v289}else{v5n6}))}else{v5pd});
        let v6hg=(if v1t4{(v1t5*(if v1sg{v1o}else{v5n7}))}else{v5pe});
        let v6i3=(if v1td{v1o}else{(if v1t4{(-(vcm*(v6hc/v1t7)))}else{v5on})});
        let v6i4=(if v1td{sf[0]}else{(if v1t4{(-(vcm*(v6hd/v1t7)))}else{v1o})});
        let v6i5=(if v1td{v1o}else{(if v1t4{(v6gm-((v1t8*v1yt)+(vcm*(v6he/v1t7))))}else{v5oo})});
        let v6i6=(if v1td{sf[273]}else{(if v1t4{(-(vcm*(v6hf/v1t7)))}else{v5op})});
        let v6i7=(if v1td{v1o}else{(if v1t4{(-(vcm*(v6hg/v1t7)))}else{v5oq})});
        let v6ia=(if v1sg{(v29k+(vn3*v6gk))}else{v5ot});
        let v6ih=(v1th*v1th);
        let v6lb=(if v1sg{((-((if v1u2{v6i3}else{(if v1tn{(v1th*((if v1tn{(v1to*(if v1sg{(v6i3/v1th)}else{v5p3}))}else{v6hc})/v1tq))}else{v5qs})})/vjr))/v1ub)}else{v5s2});
        let v6lc=(if v1sg{((-((if v1u2{v6i4}else{(if v1tn{(v1th*((if v1tn{(v1to*(if v1sg{(v6i4/v1th)}else{v1o}))}else{v6hd})/v1tq))}else{v1o})})/vjr))/v1ub)}else{v1o});
        let v6ld=(if v1sg{((-(((vjr*(if v1u2{v6i5}else{(if v1tn{((-v6gk)+((v1tx*v6ia)+(v1th*(((if v1tn{(v1to*(if v1sg{(((v1th*(v6gk+v6i5))-(v1ti*v6ia))/v6ih)}else{v5p4}))}else{v6he})/v1tq)-(v1tw*(((v1th*(-(v6gk+v6gm)))-(v1tu*v6ia))/v6ih))))))}else{v5qt})}))-(v1u3*v25l))/v25o))/v1ub)}else{v5s3});
        let v6le=(if v1sg{((-((if v1u2{v6i6}else{(if v1tn{(v1th*((if v1tn{(v1to*(if v1sg{(v6i6/v1th)}else{v5p5}))}else{v6hf})/v1tq))}else{v5qu})})/vjr))/v1ub)}else{v5s4});
        let v6lf=(if v1sg{((-((if v1u2{v6i7}else{(if v1tn{(v1th*((if v1tn{(v1to*(if v1sg{(v6i7/v1th)}else{v5p6}))}else{v6hg})/v1tq))}else{v5qv})})/vjr))/v1ub)}else{v5s5});
        let v6ou=(((v1v1*v25l)+(vjr*(((if v1sg{(((v1uk*v25u)+(vjx*(-(v1uj*(v1uf*v6ld)))))/v1uf)}else{(if v1ks{(((v1nc*v228)+(vg3*(-(v1nb*(v1mo*v5s3)))))/v1mo)}else{v3o9})})+(if v1sg{(((v1uq*v6gy)+(v1sy*(-(v1up*(v1uh*(if v1sg{((-(((vjr*v6i5)-(v1te*v25l))/v25o))/v1u7)}else{v5rk}))))))/v1uh)}else{(if v1ks{(((v1ni*v5n0)+(v1l2*(-(v1nh*(v1mq*v5rk)))))/v1mq)}else{v3oz})}))-(if v1sg{(((v1uw*v6gy)+(v1sy*(-(v1uv*(v1uh*v6ld)))))/v1uh)}else{(if v1ks{(((v1no*v5n0)+(v1l2*(-(v1nn*(v1mq*v5s3)))))/v1mq)}else{v3pp})}))))+((v1u5*(if v1sg{(vm3*v25u)}else{v5mu}))+(v1sr*(if v1sg{(-v6i5)}else{(if v1ks{(-v5oo)}else{v3jz})}))));
        let v6p7=(if v1va{v6gl}else{v5xa});
        let v6pb=(if v1va{v1o}else{v5xe});
        let v6pc=(if v1va{v285}else{v1o});
        let v6pd=(if v1va{((v1vc*v1yx)+(vco*v6p7))}else{v5xf});
        let v6pe=(if v1va{v289}else{v5xg});
        let v6pf=(if v1va{v1o}else{v5xh});
        let v6pg=(if v1va{v1o}else{v5xi});
        let v6ph=(v1ve*v6pb);
        let v6pj=(v1ve*v6pc);
        let v6pl=(v1ve*v6pd);
        let v6pn=(v1ve*v6pe);
        let v6pp=(v1ve*v6pf);
        let v6pr=(v1ve*v6pg);
        let v6pt=(v2m*v1vh);
        let v6r2=(if v1va{(-(vcm*(if v1va{(vw*(v6pb+(if v1va{((v6ph+v6ph)/v6pt)}else{v5xz})))}else{v5ye})))}else{v5yv});
        let v6r3=(if v1va{(-(vcm*(if v1va{(vw*(v6pc+(if v1va{((v6pj+v6pj)/v6pt)}else{v1o})))}else{v1o})))}else{v1o});
        let v6r4=(if v1va{(v6p7-((v1vl*v1yt)+(vcm*(if v1va{(vw*(v6pd+(if v1va{((v6pl+v6pl)/v6pt)}else{v5y0})))}else{v5yf}))))}else{v5yw});
        let v6r5=(if v1va{(-(vcm*(if v1va{(vw*(v6pe+(if v1va{((v6pn+v6pn)/v6pt)}else{v5y1})))}else{v5yg})))}else{v5yx});
        let v6r6=(if v1va{(-(vcm*(if v1va{(vw*(v6pf+(if v1va{((v6pp+v6pp)/v6pt)}else{v5y2})))}else{v5yh})))}else{v5yy});
        let v6r7=(if v1va{(-(vcm*(if v1va{(vw*(v6pg+(if v1va{((v6pr+v6pr)/v6pt)}else{v5y3})))}else{v5yi})))}else{v5yz});
        let v6tr=(if v1va{(vjx*((if v1va{((vjr*(-(v1vu*(sf[259]*(if v1va{((-(v6r2/vjr))/v1vq)}else{v608})))))/sf[259])}else{(if v1nv{((vfx*(-(v1oq*(sf[197]*v608))))/sf[197])}else{v4e3})})+(vm3*(-v6r2))))}else{(if v1v7{v1o}else{(if v1sg{((vjr*(((if v1sg{((vjx*(-(v1uj*(v1uf*v6lb))))/v1uf)}else{(if v1ks{((vg3*(-(v1nb*(v1mo*v5s2))))/v1mo)}else{v3o8})})+(if v1sg{((v1sy*(-(v1up*(v1uh*(if v1sg{((-(v6i3/vjr))/v1u7)}else{v5rj})))))/v1uh)}else{(if v1ks{((v1l2*(-(v1nh*(v1mq*v5rj))))/v1mq)}else{v3oy})}))-(if v1sg{((v1sy*(-(v1uv*(v1uh*v6lb))))/v1uh)}else{(if v1ks{((v1l2*(-(v1nn*(v1mq*v5s2))))/v1mq)}else{v3po})})))+(v1sr*(if v1sg{(-v6i3)}else{(if v1ks{(-v5on)}else{v3jy})})))}else{v1o})})});
        let v6tu=(if v1va{(vjx*((if v1va{((vjr*(-(v1vu*(sf[259]*(if v1va{((-(v6r5/vjr))/v1vq)}else{v60a})))))/sf[259])}else{(if v1nv{((vfx*(-(v1oq*(sf[197]*v60a))))/sf[197])}else{v4e5})})+(vm3*(sf[273]-v6r5))))}else{(if v1v7{v1o}else{(if v1sg{((vjr*(((if v1sg{((vjx*(-(v1uj*(v1uf*v6le))))/v1uf)}else{(if v1ks{((vg3*(-(v1nb*(v1mo*v5s4))))/v1mo)}else{v3oa})})+(if v1sg{((v1sy*(-(v1up*(v1uh*(if v1sg{((-(v6i6/vjr))/v1u7)}else{v5rl})))))/v1uh)}else{(if v1ks{((v1l2*(-(v1nh*(v1mq*v5rl))))/v1mq)}else{v3p0})}))-(if v1sg{((v1sy*(-(v1uv*(v1uh*v6le))))/v1uh)}else{(if v1ks{((v1l2*(-(v1nn*(v1mq*v5s4))))/v1mq)}else{v3pq})})))+(v1sr*(if v1sg{(sf[273]-v6i6)}else{(if v1ks{(sf[273]-v5op)}else{v3k0})})))}else{v1o})})});
        let v6tv=(if v1va{(vjx*((if v1va{((vjr*(-(v1vu*(sf[259]*(if v1va{((-(v6r6/vjr))/v1vq)}else{v60b})))))/sf[259])}else{(if v1nv{((vfx*(-(v1oq*(sf[197]*v60b))))/sf[197])}else{v4e6})})+(vm3*(-v6r6))))}else{(if v1v7{v1o}else{(if v1sg{((vjr*(((if v1sg{((vjx*(-(v1uj*(v1uf*v6lf))))/v1uf)}else{(if v1ks{((vg3*(-(v1nb*(v1mo*v5s5))))/v1mo)}else{v3ob})})+(if v1sg{((v1sy*(-(v1up*(v1uh*(if v1sg{((-(v6i7/vjr))/v1u7)}else{v5rm})))))/v1uh)}else{(if v1ks{((v1l2*(-(v1nh*(v1mq*v5rm))))/v1mq)}else{v3p1})}))-(if v1sg{((v1sy*(-(v1uv*(v1uh*v6lf))))/v1uh)}else{(if v1ks{((v1l2*(-(v1nn*(v1mq*v5s5))))/v1mq)}else{v3pr})})))+(v1sr*(if v1sg{(-v6i7)}else{(if v1ks{(sf[0]-v5oq)}else{v3k1})})))}else{v1o})})});
        let v6ux=(if (sf[262]!=0.0){v1o}else{v5if});
        let v6uy=(if (sf[262]!=0.0){v1o}else{v5ig});
        let v6uz=(if (sf[262]!=0.0){v1o}else{v5ih});
        let v6v0=(if (sf[262]!=0.0){v1o}else{v5ii});
        let v6v1=(if (sf[262]!=0.0){v1o}else{v5ij});
        let v6vw=(if (sf[262]!=0.0){v1o}else{v5du});
        let v6vx=(if (sf[262]!=0.0){v1o}else{v5dv});
        let v6vy=(if (sf[262]!=0.0){v1o}else{v5dw});
        let v6vz=(if (sf[262]!=0.0){v1o}else{v5dx});
        let v6w0=(if (sf[262]!=0.0){v1o}else{v5dy});
        let v6wz=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[263]*v6ux))}else{v1o})});
        let v6x0=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[263]*v6uy))}else{v1o})});
        let v6x1=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[263]*v6uz))}else{v1o})});
        let v6x2=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[263]*v6v0))}else{v1o})});
        let v6x3=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[263]*v6v1))}else{v1o})});
        let v6xa=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[264]*v6vw))}else{v1o})});
        let v6xb=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[264]*v6vx))}else{v1o})});
        let v6xc=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[264]*v6vy))}else{v1o})});
        let v6xd=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[264]*v6vz))}else{v1o})});
        let v6xe=(if sb[59]{v1o}else{(if (sf[262]!=0.0){(sf[87]*(sf[264]*v6w0))}else{v1o})});
        let v6y3=(sf[0]*(if v1w4{v1o}else{v6tr}));
        let v6y4=(sf[0]*(if v1w4{v1o}else{(if v1va{(vjx*((if v1va{((vjr*(-(v1vu*(sf[259]*(if v1va{((-(v6r3/vjr))/v1vq)}else{v1o})))))/sf[259])}else{v1o})+(vm3*(sf[0]-v6r3))))}else{(if v1v7{v1o}else{(if v1sg{((vjr*(((if v1sg{((vjx*(-(v1uj*(v1uf*v6lc))))/v1uf)}else{v1o})+(if v1sg{((v1sy*(-(v1up*(v1uh*(if v1sg{((-(v6i4/vjr))/v1u7)}else{v1o})))))/v1uh)}else{v1o}))-(if v1sg{((v1sy*(-(v1uv*(v1uh*v6lc))))/v1uh)}else{v1o})))+(v1sr*(if v1sg{(sf[0]-v6i4)}else{v1o})))}else{v1o})})})}));
        let v6y5=(sf[0]*(if v1w4{v1o}else{(if v1va{((v1w1*v25u)+(vjx*((if v1va{(((v1vv*v25l)+(vjr*(-(v1vu*(sf[259]*(if v1va{((-(((vjr*v6r4)-(v1vo*v25l))/v25o))/v1vq)}else{v609}))))))/sf[259])}else{(if v1nv{(((v1or*v21z)+(vfx*(-(v1oq*(sf[197]*v609)))))/sf[197])}else{v4e4})})+(vm3*(-v6r4)))))}else{(if v1v7{v1o}else{(if v1sg{v6ou}else{v1o})})})}));
        let v6y6=(sf[0]*(if v1w4{v1o}else{v6tu}));
        let v6y7=(sf[0]*(if v1w4{v1o}else{v6tv}));
        let v6y8=(sf[0]*(if v1w4{v1o}else{(if v1va{(vjx*((if v1va{((vjr*(-(v1vu*(sf[259]*(if v1va{((-(v6r7/vjr))/v1vq)}else{v60c})))))/sf[259])}else{(if v1nv{((vfx*(-(v1oq*(sf[197]*v60c))))/sf[197])}else{v4e7})})+(vm3*(-v6r7))))}else{v1o})}));
        let v6y9=(sf[0]*(if vzs{v1o}else{(if vys{(vvj*(v3ev+(vm3*(sf[0]-v3cd))))}else{(if vyq{v1o}else{(if vvm{((viw*((v38m+v39c)-v3a2))+(vvr*v34c))}else{(if vqm{v1o}else{(if vpl{(vlr*(v2jj+(vm3*(sf[0]-v2hm))))}else{(if vph{v1o}else{(if vly{((vfx*((v2ep+v2f9)-v2ft))+(vmb*v2be))}else{v1o})})})})})})})}));
        let v6ya=(sf[0]*(if vzs{v1o}else{(if vys{((vzp*v302)+(vvj*(v3ew+(vm3*(-v3ce)))))}else{(if vyq{v1o}else{(if vvm{(((vyk*v24s)+(viw*((v38n+v39d)-v3a3)))+((vx9*v306)+(vvr*v34d)))}else{(if vqm{v1o}else{(if vpl{((vqj*v27o)+(vlr*(v2jk+(vm3*(-v2hn)))))}else{(if vph{v1o}else{(if vly{(((vpb*v21z)+(vfx*((v2eq+v2fa)-v2fu)))+((vny*v27u)+(vmb*v2bf)))}else{v1o})})})})})})})}));
        let v6yb=(sf[0]*(if vzs{v1o}else{(if vys{(vvj*(v3ex+(vm3*(sf[273]-v3cf))))}else{(if vyq{v1o}else{(if vvm{((viw*((v38o+v39e)-v3a4))+(vvr*v34e))}else{(if vqm{v1o}else{(if vpl{(vlr*(v2jl+(vm3*(sf[273]-v2ho))))}else{(if vph{v1o}else{(if vly{((vfx*((v2er+v2fb)-v2fv))+(vmb*v2bg))}else{v1o})})})})})})})}));
        let v6yc=(sf[0]*(if vzs{v1o}else{(if vys{(vvj*(v3ey+(vm3*(-v3cg))))}else{(if vyq{v1o}else{(if vvm{((viw*((v38p+v39f)-v3a5))+(vvr*v34f))}else{v1o})})})}));
        let v6ym=(sf[0]*(((if vvg{v1o}else{(if vug{(vqr*(v2z7+(vm3*(-v2wp))))}else{(if vuc{v1o}else{(if vqy{((viw*((v2sy+v2to)-v2ue))+(vr9*v2oo))}else{v1o})})})})+v3v7)+(sf[239]*v5de)));
        let v6yn=(sf[0]*(((if vvg{v1o}else{(if vug{((vvd*v2k8)+(vqr*(v2z8+(vm3*(-v2wq)))))}else{(if vuc{v1o}else{(if vqy{(((vu6*v24s)+(viw*((v2sz+v2tp)-v2uf)))+((vst*v2ke)+(vr9*v2op)))}else{v1o})})})})+v3v8)+(sf[239]*v5di)));
        let v6yo=(sf[0]*(((if vvg{v1o}else{(if vug{(vqr*(v2z9+(vm3*(sf[273]-v2wr))))}else{(if vuc{v1o}else{(if vqy{((viw*((v2t0+v2tq)-v2ug))+(vr9*v2oq))}else{v1o})})})})+v3v9)+(sf[239]*v5dm)));
        let v6yp=(sf[0]*(((if vvg{v1o}else{(if vug{(vqr*(v2za+(vm3*(sf[0]-v2ws))))}else{(if vuc{v1o}else{(if vqy{((viw*((v2t1+v2tr)-v2uh))+(vr9*v2or))}else{v1o})})})})+v3va)+(sf[239]*v5dq)));
        let v6yq=(sf[0]*(sf[239]*v5dt));
        let v6yu=(sf[0]*(v48t+v6ux));
        let v6yv=(sf[0]*(v48u+v6uy));
        let v6yw=(sf[0]*(v48v+v6uz));
        let v6yx=(sf[0]*(v48w+v6v0));
        let v6yy=(sf[0]*(v48x+v6v1));

        CommonStampValues {
            v1, v2, v4, v5, v6, v7, v8, v9, 
            va, vb, ve, vf, vw, v1c, v1o, v2m, 
            vc7, vcm, vco, vcq, vcu, vcx, vfx, vg3, 
            vlr, vly, vm0, vm3, vmb, vmi, vmm, vmp, 
            vmr, vms, vn0, vnd, vnf, vng, vnu, vo2, 
            vo6, vpl, vpu, vpx, vq6, vqp, vqr, vqy, 
            vr0, vr9, vrg, vrm, vro, vrp, vrx, vs8, 
            vsa, vsb, vsp, vsx, vt1, vug, vuo, vur, 
            vv0, vvj, vvm, vvn, vvr, vvw, vw2, vw4, 
            vw5, vwd, vwo, vwq, vwr, vx5, vxd, vxh, 
            vys, vz0, vz3, vzc, vzw, vzx, v101, v106, 
            v10c, v10e, v10f, v10n, v10y, v110, v111, v11f, 
            v11n, v11r, v132, v13a, v13d, v13m, v165, v167, 
            v16l, v16o, v16x, v17h, v17k, v17y, v181, v18a, 
            v1b6, v1db, v1hu, v1hx, v1io, v1kr, v1ks, v1kt, 
            v1kx, v1l2, v1l8, v1la, v1lb, v1lj, v1lu, v1lw, 
            v1lx, v1mb, v1mj, v1mn, v1nv, v1o3, v1o6, v1of, 
            v1wi, v1wj, v1wq, v1wr, v1x0, v1x2, v1xb, v1xc, 
            v1xd, v1xe, v1xg, v1xi, v1y1, v1yt, v1yx, v1yy, 
            v1z2, v1z6, v21z, v228, v27o, v27u, v284, v28g, 
            v28h, v28i, v2a1, v2a2, v2a3, v2bt, v2bu, v2bv, 
            v2c8, v2c9, v2ca, v2h2, v2h3, v2h4, v2hb, v2hc, 
            v2hd, v2ih, v2ii, v2ij, v2k6, v2k8, v2ke, v2ko, 
            v2l0, v2l1, v2l2, v2l3, v2mz, v2n0, v2n1, v2n2, 
            v2p7, v2p8, v2p9, v2pa, v2pq, v2pr, v2ps, v2pt, 
            v2vz, v2w0, v2w1, v2w2, v2wb, v2wc, v2wd, v2we, 
            v2xt, v2xu, v2xv, v2xw, v302, v306, v30c, v30o, 
            v30p, v30q, v30r, v32n, v32o, v32p, v32q, v34v, 
            v34w, v34x, v34y, v35e, v35f, v35g, v35h, v3bn, 
            v3bo, v3bp, v3bq, v3bz, v3c0, v3c1, v3c2, v3dh, 
            v3di, v3dj, v3dk, v3fs, v3fy, v3ga, v3gb, v3gc, 
            v3gd, v3i9, v3ia, v3ib, v3ic, v3kh, v3ki, v3kj, 
            v3kk, v3l0, v3l1, v3l2, v3l3, v3r9, v3ra, v3rb, 
            v3rc, v3rl, v3rm, v3rn, v3ro, v3t3, v3t4, v3t5, 
            v3t6, v42v, v42w, v42x, v42y, v43w, v43x, v43y, 
            v43z, v440, v44b, v44c, v44d, v44e, v44f, v465, 
            v466, v467, v468, v469, v48y, v493, v494, v495, 
            v496, v4a4, v4a5, v4a6, v4a7, v4a8, v4aj, v4ak, 
            v4al, v4am, v4an, v4cd, v4ce, v4cf, v4cg, v4ch, 
            v4la, v4lb, v4lc, v4nc, v5de, v5di, v5dm, v5dq, 
            v5dt, v5du, v5dv, v5dw, v5dx, v5dy, v5if, v5ig, 
            v5ih, v5ii, v5ij, v5mu, v5n0, v5nc, v5nd, v5ne, 
            v5nf, v5pb, v5pc, v5pd, v5pe, v5rj, v5rk, v5rl, 
            v5rm, v5s2, v5s3, v5s4, v5s5, v5xz, v5y0, v5y1, 
            v5y2, v5y3, v5ye, v5yf, v5yg, v5yh, v5yi, v608, 
            v609, v60a, v60b, v60c, v6ux, v6uy, v6uz, v6v0, 
            v6v1, v6vw, v6vx, v6vy, v6vz, v6w0, v6wz, v6x0, 
            v6x1, v6x2, v6x3, v6xa, v6xb, v6xc, v6xd, v6xe, 
            v6y3, v6y4, v6y5, v6y6, v6y7, v6y8, v6y9, v6ya, 
            v6yb, v6yc, v6ym, v6yn, v6yo, v6yp, v6yq, v6yu, 
            v6yv, v6yw, v6yx, v6yy, 
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
            v1, v2, v4, v5, v6, v7, v8, v9, 
            va, vb, ve, vf, vw, v1c, v1o, v2m, 
            vc7, vcm, vco, vcq, vcu, vcx, vfx, vg3, 
            vlr, vly, vm0, vm3, vmb, vmi, vmm, vmp, 
            vmr, vms, vn0, vnd, vnf, vng, vnu, vo2, 
            vo6, vpl, vpu, vpx, vq6, vqp, vqr, vqy, 
            vr0, vr9, vrg, vrm, vro, vrp, vrx, vs8, 
            vsa, vsb, vsp, vsx, vt1, vug, vuo, vur, 
            vv0, vvj, vvm, vvn, vvr, vvw, vw2, vw4, 
            vw5, vwd, vwo, vwq, vwr, vx5, vxd, vxh, 
            vys, vz0, vz3, vzc, vzw, vzx, v101, v106, 
            v10c, v10e, v10f, v10n, v10y, v110, v111, v11f, 
            v11n, v11r, v132, v13a, v13d, v13m, v165, v167, 
            v16l, v16o, v16x, v17h, v17k, v17y, v181, v18a, 
            v1b6, v1db, v1hu, v1hx, v1io, v1kr, v1ks, v1kt, 
            v1kx, v1l2, v1l8, v1la, v1lb, v1lj, v1lu, v1lw, 
            v1lx, v1mb, v1mj, v1mn, v1nv, v1o3, v1o6, v1of, 
            v1wi, v1wj, v1wq, v1wr, v1x0, v1x2, v1xb, v1xc, 
            v1xd, v1xe, v1xg, v1xi, v1y1, v1yt, v1yx, v1yy, 
            v1z2, v1z6, v21z, v228, v27o, v27u, v284, v28g, 
            v28h, v28i, v2a1, v2a2, v2a3, v2bt, v2bu, v2bv, 
            v2c8, v2c9, v2ca, v2h2, v2h3, v2h4, v2hb, v2hc, 
            v2hd, v2ih, v2ii, v2ij, v2k6, v2k8, v2ke, v2ko, 
            v2l0, v2l1, v2l2, v2l3, v2mz, v2n0, v2n1, v2n2, 
            v2p7, v2p8, v2p9, v2pa, v2pq, v2pr, v2ps, v2pt, 
            v2vz, v2w0, v2w1, v2w2, v2wb, v2wc, v2wd, v2we, 
            v2xt, v2xu, v2xv, v2xw, v302, v306, v30c, v30o, 
            v30p, v30q, v30r, v32n, v32o, v32p, v32q, v34v, 
            v34w, v34x, v34y, v35e, v35f, v35g, v35h, v3bn, 
            v3bo, v3bp, v3bq, v3bz, v3c0, v3c1, v3c2, v3dh, 
            v3di, v3dj, v3dk, v3fs, v3fy, v3ga, v3gb, v3gc, 
            v3gd, v3i9, v3ia, v3ib, v3ic, v3kh, v3ki, v3kj, 
            v3kk, v3l0, v3l1, v3l2, v3l3, v3r9, v3ra, v3rb, 
            v3rc, v3rl, v3rm, v3rn, v3ro, v3t3, v3t4, v3t5, 
            v3t6, v42v, v42w, v42x, v42y, v43w, v43x, v43y, 
            v43z, v440, v44b, v44c, v44d, v44e, v44f, v465, 
            v466, v467, v468, v469, v48y, v493, v494, v495, 
            v496, v4a4, v4a5, v4a6, v4a7, v4a8, v4aj, v4ak, 
            v4al, v4am, v4an, v4cd, v4ce, v4cf, v4cg, v4ch, 
            v4la, v4lb, v4lc, v4nc, v5de, v5di, v5dm, v5dq, 
            v5dt, v5du, v5dv, v5dw, v5dx, v5dy, v5if, v5ig, 
            v5ih, v5ii, v5ij, v5mu, v5n0, v5nc, v5nd, v5ne, 
            v5nf, v5pb, v5pc, v5pd, v5pe, v5rj, v5rk, v5rl, 
            v5rm, v5s2, v5s3, v5s4, v5s5, v5xz, v5y0, v5y1, 
            v5y2, v5y3, v5ye, v5yf, v5yg, v5yh, v5yi, v608, 
            v609, v60a, v60b, v60c, v6ux, v6uy, v6uz, v6v0, 
            v6v1, v6vw, v6vx, v6vy, v6vz, v6w0, v6wz, v6x0, 
            v6x1, v6x2, v6x3, v6xa, v6xb, v6xc, v6xd, v6xe, 
            v6y3, v6y4, v6y5, v6y6, v6y7, v6y8, v6y9, v6ya, 
            v6yb, v6yc, v6ym, v6yn, v6yo, v6yp, v6yq, v6yu, 
            v6yv, v6yw, v6yx, v6yy, 
        }=self.eval_common_stamp_values(ctx);
        let vi=(v8-vf);
        let vk=(v2-ctx.node_voltage(nodes[0]));
        let vl=(v1-v5);
        let vf0=(((sf[25]*vcu)+(sf[8]*vcx))).exp();
        let vf2=(if (sf[148]!=0.0){(sf[60]*vf0)}else{sf[367]});
        let vf6=(((sf[62]*vcu)+(sf[63]*vcx))).exp();
        let vf8=(if (sf[148]!=0.0){(sf[61]*vf6)}else{sf[372]});
        let vg5=(sf[11]*vcx);
        let vg7=(((sf[23]*vcu)+vg5)).exp();
        let vg9=(if (sf[148]!=0.0){(sf[74]*vg7)}else{sf[397]});
        let vhv=((sf[96]*vcq)).exp();
        let vhz=((sf[98]*vcq)).exp();
        let vi3=(if sb[20]{sf[31]}else{(if sb[19]{(sf[31]*vhv)}else{sf[449]})});
        let vi4=(if sb[20]{sf[97]}else{(if sb[19]{(sf[97]*vhz)}else{sf[450]})});
        let vi6=((sf[100]*vcu)).exp();
        let vi8=(if (sf[148]!=0.0){(sf[99]*vi6)}else{sf[453]});
        let vjy=(sf[24]*vcu);
        let vk1=((vjy+(sf[14]*vcx))).exp();
        let vk3=(if (sf[148]!=0.0){(sf[123]*vk1)}else{sf[497]});
        let vk5=((vg5+vjy)).exp();
        let vk7=(if (sf[148]!=0.0){(sf[124]*vk5)}else{sf[500]});
        let vl1=((sf[138]*vcu)).exp();
        let vl3=(if (sf[148]!=0.0){(sf[137]*vl1)}else{sf[525]});
        let vl5=((sf[140]*vcu)).exp();
        let vl9=((sf[142]*vcu)).exp();
        let vlb=(if (sf[148]!=0.0){(sf[141]*vl9)}else{sf[531]});
        let vld=((sf[144]*vcu)).exp();
        let vle=(sf[143]*vld);
        let vlg=(v1c+(sf[145]*vcq));
        let vli=(if (sf[148]!=0.0){(vle*vlg)}else{sf[537]});
        let vn1=(if vn0{v1c}else{(if vmp{(vmr/vms)}else{v1o})});
        let vnv=(if vnu{v1c}else{(if vnd{(vnf/vng)}else{v1o})});
        let vod=((vo6*sf[198])).exp();
        let voe=(vlr*vod);
        let vof=(vn1*voe);
        let voi=(-vm0);
        let vok=((vo2*voi)).exp();
        let vol=(vmi*vok);
        let vom=(v1c-vnv);
        let vop=(v1c-vn1);
        let vq2=(if vpl{(vpx/vpu)}else{v1o});
        let vq8=((sf[198]*vq6)).exp();
        let vry=(if vrx{v1c}else{(if vrm{(vro/vrp)}else{vn1})});
        let vsq=(if vsp{v1c}else{(if vs8{(vsa/vsb)}else{vnv})});
        let vt8=((vt1*sf[206])).exp();
        let vt9=(vqr*vt8);
        let vta=(vry*vt9);
        let vtd=(-vr0);
        let vtf=((vsx*vtd)).exp();
        let vtg=(vrg*vtf);
        let vth=(v1c-vsq);
        let vtk=(v1c-vry);
        let vuw=(if vug{(vur/vuo)}else{vq2});
        let vv2=((sf[206]*vv0)).exp();
        let vwe=(if vwd{v1c}else{(if vw2{(vw4/vw5)}else{vry})});
        let vx6=(if vx5{v1c}else{(if vwo{(vwq/vwr)}else{vsq})});
        let vxm=((sf[206]*vxh)).exp();
        let vxn=(vvj*vxm);
        let vxo=(vwe*vxn);
        let vxr=(-vvn);
        let vxt=((vxd*vxr)).exp();
        let vxu=(vvw*vxt);
        let vxv=(v1c-vx6);
        let vxy=(v1c-vwe);
        let vz8=(if vys{(vz3/vz0)}else{vuw});
        let vze=((sf[206]*vzc)).exp();
        let v10o=(if v10n{v1c}else{(if v10c{(v10e/v10f)}else{vwe})});
        let v11g=(if v11f{v1c}else{(if v10y{(v110/v111)}else{vx6})});
        let v11w=((sf[198]*v11r)).exp();
        let v11x=(vqp*v11w);
        let v11y=(v10o*v11x);
        let v121=(-vzx);
        let v123=((v11n*v121)).exp();
        let v124=(v106*v123);
        let v125=(v1c-v11g);
        let v128=(v1c-v10o);
        let v13i=(if v132{(v13d/v13a)}else{vz8});
        let v13o=((sf[198]*v13m)).exp();
        let v16t=(if (v167!=0.0){(v16o/v16l)}else{v13i});
        let v170=((v16x*sf[210])).exp();
        let v186=(if v17k{(v181/v17y)}else{v16t});
        let v18d=((v18a*sf[212])).exp();
        let v1hy=(v1hx-v1hu);
        let v1iu=(vcm*sf[241]);
        let v1iw=(if (sf[240]!=0.0){(va/v1iu)}else{v1o});
        let v1iy=(if (v1iw>vmm){v1c}else{v1o});
        let v1iz=((sf[240]!=0.0)&&(v1iy!=0.0));
        let v1j3=(if v1iz{vmm}else{v1iw});
        let v1j5=((sf[240]!=0.0)&&(!(v1iy!=0.0)));
        let v1j6=(if v1j5{v1c}else{(if v1iz{(v1c+(v1iw-vmm))}else{v1o})});
        let v1j7=scalar_limexp(v1j3);
        let v1j9=((v1j6*v1j7)-v1c);
        let v1jh=(vcm*sf[243]);
        let v1jj=(if (sf[242]!=0.0){(va/v1jh)}else{v1j3});
        let v1jl=(if (v1jj>vmm){v1c}else{v1o});
        let v1jm=((sf[242]!=0.0)&&(v1jl!=0.0));
        let v1jq=(if v1jm{vmm}else{v1jj});
        let v1js=((sf[242]!=0.0)&&(!(v1jl!=0.0)));
        let v1jt=(if v1js{v1c}else{(if v1jm{(v1c+(v1jj-vmm))}else{v1j6})});
        let v1ju=scalar_limexp(v1jq);
        let v1jw=((v1jt*v1ju)-v1c);
        let v1k1=((if sb[43]{v1o}else{(if (sf[240]!=0.0){(vf2*v1j9)}else{v1o})})+(if sb[45]{v1o}else{(if (sf[242]!=0.0){(vf8*v1jw)}else{v1o})}));
        let v1k5=(vcm*sf[245]);
        let v1k7=(if (sf[244]!=0.0){(v7/v1k5)}else{v1jq});
        let v1k9=(if (v1k7>vmm){v1c}else{v1o});
        let v1ka=((sf[244]!=0.0)&&(v1k9!=0.0));
        let v1ke=(if v1ka{vmm}else{v1k7});
        let v1kg=((sf[244]!=0.0)&&(!(v1k9!=0.0)));
        let v1kh=(if v1kg{v1c}else{(if v1ka{(v1c+(v1k7-vmm))}else{v1jt})});
        let v1ki=scalar_limexp(v1ke);
        let v1kk=((v1kh*v1ki)-v1c);
        let v1ko=(if sb[47]{v1o}else{(if (sf[244]!=0.0){(vg9*v1kk)}else{v1o})});
        let v1kp=(v1k1+v1ko);
        let v1lk=(if v1lj{v1c}else{(if v1l8{(v1la/v1lb)}else{v10o})});
        let v1mc=(if v1mb{v1c}else{(if v1lu{(v1lw/v1lx)}else{v11g})});
        let v1ms=((sf[198]*v1mn)).exp();
        let v1mt=(vg3*v1ms);
        let v1mu=(v1lk*v1mt);
        let v1mx=(-v1kt);
        let v1mz=((v1mj*v1mx)).exp();
        let v1n0=(v1l2*v1mz);
        let v1n1=(v1c-v1mc);
        let v1n4=(v1c-v1lk);
        let v1ns=(!(v1kr!=0.0));
        let v1nt=((sf[192]!=0.0)&&v1ns);
        let v1ob=(if v1nv{(v1o6/v1o3)}else{v186});
        let v1oh=((sf[198]*v1of)).exp();
        let v1om=((if v1nv{(v1ob*v1oh)}else{(if v17k{(v186*v18d)}else{(if (v167!=0.0){(v16t*v170)}else{(if v132{(v13i*v13o)}else{(if vys{(vz8*vze)}else{(if vug{(vuw*vv2)}else{(if vpl{(vq2*vq8)}else{v1o})})})})})})})+(vm3*(v1c-v1ob)));
        let v1ov=(sb[24]&&v1ns);
        let v1ow=(if v1ov{v1o}else{(if v1nv{(vg3*v1om)}else{(if v1nt{v1o}else{(if v1ks{((if v1ks{(v1kx*v1n4)}else{(if vzw{(v101*v128)}else{(if vvm{(vvr*vxy)}else{(if vqy{(vr9*vtk)}else{(if vly{(vmb*vop)}else{v1o})})})})})+((if v1ks{(v1mc*v1mu)}else{(if vzw{(v11g*v11y)}else{(if vvm{(vx6*vxo)}else{(if vqy{(vsq*vta)}else{(if vly{(vnv*vof)}else{v1o})})})})})+(if v1ks{(v1n0*v1n1)}else{(if vzw{(v124*v125)}else{(if vvm{(vxu*vxv)}else{(if vqy{(vtg*vth)}else{(if vly{(vol*vom)}else{v1o})})})})})))}else{v1o})})})});
        let v1ox=(vfx-v7);
        let v1oy=(if (sf[95]!=0.0){v1ox}else{v1o});
        let v1p0=(if (v1oy>v1o){v1c}else{v1o});
        let v1p1=((sf[95]!=0.0)&&(v1p0!=0.0));
        let v1p3=(if v1p1{(vi4/v1ow)}else{v1o});
        let v1p5=(if v1p1{(vi4/vg3)}else{v1o});
        let v1p7=(if (v1oy>v1p5){v1c}else{v1o});
        let v1p8=(v1p1&&(v1p7!=0.0));
        let v1p9=(-v1p3);
        let v1pb=((v1p9/v1p5)).exp();
        let v1pd=(if v1p8{(vi3*v1pb)}else{v1o});
        let v1pf=(v1c+(v1p3/v1p5));
        let v1pg=(v1oy-v1p5);
        let v1pi=(v1p5+(v1pf*v1pg));
        let v1pm=(v1p1&&(!(v1p7!=0.0)));
        let v1pn=(vi3*v1oy);
        let v1pp=((v1p9/v1oy)).exp();
        let v1pr=(if v1pm{(v1pn*v1pp)}else{(if v1p8{(v1pd*v1pi)}else{v1o})});
        let v1pv=((sf[95]!=0.0)&&(!(v1p0!=0.0)));
        let v1pw=(if v1pv{v1o}else{(if v1p1{(v1hx*v1pr)}else{v1o})});
        let v1py=(if (vi8>v1o){v1c}else{v1o});
        let v1q9=(if (v1py!=0.0){((((v1c+(v17h/sf[246]))+(v165/sf[247]))+(v1hx/v1b6))+(v1hu/sf[224]))}else{v1o});
        let v1qc=((v1db+(v1q9*v1q9))).sqrt();
        let v1qf=(if (v1py!=0.0){(vw*(v1q9+v1qc))}else{v1o});
        let v1qh=(if (v1py!=0.0){(vi8/v1qf)}else{v1o});
        let v1qk=((v1py!=0.0)&&((if (v1kp>v1o){v1c}else{v1o})!=0.0));
        let v1qm=(v1qh*sf[248]);
        let v1qn=(v1kp*v1qm);
        let v1qp=(if v1qk{(vco*v1qn)}else{v1o});
        let v1qs=(if (v1qp<1e-6){v1c}else{v1o});
        let v1qt=(v1qk&&(v1qs!=0.0));
        let v1qv=(v1c-(vw*v1qp));
        let v1qx=(if v1qt{(v1qh*v1qv)}else{v1qh});
        let v1qz=(v1qk&&(!(v1qs!=0.0)));
        let v1r0=(v1c+v1qp);
        let v1r1=(v1r0).ln();
        let v1r2=(v1qx*v1r1);
        let v1r5=(!(v1py!=0.0));
        let v1r7=((if (sf[148]!=0.0){(sf[139]*vl5)}else{sf[528]})+(if v1r5{v1o}else{(if v1qz{(v1r2/v1qp)}else{v1qx})}));
        let v1rc=(if (sf[249]!=0.0){(vcm*sf[250])}else{v1o});
        let v1rd=(v4/v1rc);
        let v1rg=(ve/v1rc);
        let v1rj=((if (sf[249]!=0.0){scalar_limexp(v1rd)}else{v1o})-(if (sf[249]!=0.0){scalar_limexp(v1rg)}else{v1o}));
        let v1rr=(vcm*sf[252]);
        let v1rt=(if (sf[251]!=0.0){(ve/v1rr)}else{v1ke});
        let v1rv=(if (v1rt>vmm){v1c}else{v1o});
        let v1rw=((sf[251]!=0.0)&&(v1rv!=0.0));
        let v1s0=(if v1rw{vmm}else{v1rt});
        let v1s2=((sf[251]!=0.0)&&(!(v1rv!=0.0)));
        let v1s3=(if v1s2{v1c}else{(if v1rw{(v1c+(v1rt-vmm))}else{v1kh})});
        let v1s4=scalar_limexp(v1s0);
        let v1s6=((v1s3*v1s4)-v1c);
        let v1y2=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v1y1);
        let v1y7=-1.0;
        let v22a=(sf[11]*v1z6);
        let v244=(if sb[20]{v1o}else{(if sb[19]{(sf[31]*(vhv*(sf[96]*v1yy)))}else{v1o})});
        let v245=(if sb[20]{v1o}else{(if sb[19]{(sf[97]*(vhz*(sf[98]*v1yy)))}else{v1o})});
        let v25v=(sf[24]*v1z2);
        let v28m=(vms*vms);
        let v29d=(if vn0{v1o}else{(if vmp{(((vms*v28g)-(vmr*v28g))/v28m)}else{v1o})});
        let v29e=(if vn0{v1o}else{(if vmp{(((vms*v28h)-(vmr*v28h))/v28m)}else{v1o})});
        let v29f=(if vn0{v1o}else{(if vmp{(((vms*v28i)-(vmr*v28i))/v28m)}else{v1o})});
        let v2a7=(vng*vng);
        let v2b5=(if vnu{v1o}else{(if vnd{(((vng*v2a1)-(vnf*v2a1))/v2a7)}else{v1o})});
        let v2b6=(if vnu{v1o}else{(if vnd{(((vng*v2a2)-(vnf*v2a2))/v2a7)}else{v1o})});
        let v2b7=(if vnu{v1o}else{(if vnd{(((vng*v2a3)-(vnf*v2a3))/v2a7)}else{v1o})});
        let v2hs=(vpu*vpu);
        let v2i2=(if vpl{(((vpu*v2hb)-(vpx*v2h2))/v2hs)}else{v1o});
        let v2i3=(if vpl{(((vpu*v2hc)-(vpx*v2h3))/v2hs)}else{v1o});
        let v2i4=(if vpl{(((vpu*v2hd)-(vpx*v2h4))/v2hs)}else{v1o});
        let v2l7=(vrp*vrp);
        let v2m7=(if vrx{v1o}else{(if vrm{(((vrp*v2l0)-(vro*v2l0))/v2l7)}else{v29d})});
        let v2m8=(if vrx{v1o}else{(if vrm{(((vrp*v2l1)-(vro*v2l1))/v2l7)}else{v29e})});
        let v2m9=(if vrx{v1o}else{(if vrm{(((vrp*v2l2)-(vro*v2l2))/v2l7)}else{v29f})});
        let v2ma=(if vrx{v1o}else{(if vrm{(((vrp*v2l3)-(vro*v2l3))/v2l7)}else{v1o})});
        let v2n6=(vsb*vsb);
        let v2oc=(if vsp{v1o}else{(if vs8{(((vsb*v2mz)-(vsa*v2mz))/v2n6)}else{v2b5})});
        let v2od=(if vsp{v1o}else{(if vs8{(((vsb*v2n0)-(vsa*v2n0))/v2n6)}else{v2b6})});
        let v2oe=(if vsp{v1o}else{(if vs8{(((vsb*v2n1)-(vsa*v2n1))/v2n6)}else{v2b7})});
        let v2of=(if vsp{v1o}else{(if vs8{(((vsb*v2n2)-(vsa*v2n2))/v2n6)}else{v1o})});
        let v2ww=(vuo*vuo);
        let v2xa=(if vug{(((vuo*v2wb)-(vur*v2vz))/v2ww)}else{v2i2});
        let v2xb=(if vug{(((vuo*v2wc)-(vur*v2w0))/v2ww)}else{v2i3});
        let v2xc=(if vug{(((vuo*v2wd)-(vur*v2w1))/v2ww)}else{v2i4});
        let v2xd=(if vug{(((vuo*v2we)-(vur*v2w2))/v2ww)}else{v1o});
        let v30v=(vw5*vw5);
        let v31v=(if vwd{v1o}else{(if vw2{(((vw5*v30o)-(vw4*v30o))/v30v)}else{v2m7})});
        let v31w=(if vwd{v1o}else{(if vw2{(((vw5*v30p)-(vw4*v30p))/v30v)}else{v2m8})});
        let v31x=(if vwd{v1o}else{(if vw2{(((vw5*v30q)-(vw4*v30q))/v30v)}else{v2m9})});
        let v31y=(if vwd{v1o}else{(if vw2{(((vw5*v30r)-(vw4*v30r))/v30v)}else{v2ma})});
        let v32u=(vwr*vwr);
        let v340=(if vx5{v1o}else{(if vwo{(((vwr*v32n)-(vwq*v32n))/v32u)}else{v2oc})});
        let v341=(if vx5{v1o}else{(if vwo{(((vwr*v32o)-(vwq*v32o))/v32u)}else{v2od})});
        let v342=(if vx5{v1o}else{(if vwo{(((vwr*v32p)-(vwq*v32p))/v32u)}else{v2oe})});
        let v343=(if vx5{v1o}else{(if vwo{(((vwr*v32q)-(vwq*v32q))/v32u)}else{v2of})});
        let v3ck=(vz0*vz0);
        let v3cy=(if vys{(((vz0*v3bz)-(vz3*v3bn))/v3ck)}else{v2xa});
        let v3cz=(if vys{(((vz0*v3c0)-(vz3*v3bo))/v3ck)}else{v2xb});
        let v3d0=(if vys{(((vz0*v3c1)-(vz3*v3bp))/v3ck)}else{v2xc});
        let v3d1=(if vys{(((vz0*v3c2)-(vz3*v3bq))/v3ck)}else{v2xd});
        let v3gh=(v10f*v10f);
        let v3hh=(if v10n{v1o}else{(if v10c{(((v10f*v3ga)-(v10e*v3ga))/v3gh)}else{v31v})});
        let v3hi=(if v10n{v1o}else{(if v10c{(((v10f*v3gb)-(v10e*v3gb))/v3gh)}else{v31w})});
        let v3hj=(if v10n{v1o}else{(if v10c{(((v10f*v3gc)-(v10e*v3gc))/v3gh)}else{v31x})});
        let v3hk=(if v10n{v1o}else{(if v10c{(((v10f*v3gd)-(v10e*v3gd))/v3gh)}else{v31y})});
        let v3ig=(v111*v111);
        let v3jm=(if v11f{v1o}else{(if v10y{(((v111*v3i9)-(v110*v3i9))/v3ig)}else{v340})});
        let v3jn=(if v11f{v1o}else{(if v10y{(((v111*v3ia)-(v110*v3ia))/v3ig)}else{v341})});
        let v3jo=(if v11f{v1o}else{(if v10y{(((v111*v3ib)-(v110*v3ib))/v3ig)}else{v342})});
        let v3jp=(if v11f{v1o}else{(if v10y{(((v111*v3ic)-(v110*v3ic))/v3ig)}else{v343})});
        let v3s6=(v13a*v13a);
        let v3sk=(if v132{(((v13a*v3rl)-(v13d*v3r9))/v3s6)}else{v3cy});
        let v3sl=(if v132{(((v13a*v3rm)-(v13d*v3ra))/v3s6)}else{v3cz});
        let v3sm=(if v132{(((v13a*v3rn)-(v13d*v3rb))/v3s6)}else{v3d0});
        let v3sn=(if v132{(((v13a*v3ro)-(v13d*v3rc))/v3s6)}else{v3d1});
        let v450=(v16l*v16l);
        let v45i=(if (v167!=0.0){(((v16l*v44b)-(v16o*v43w))/v450)}else{v3sk});
        let v45j=(if (v167!=0.0){(((v16l*v44c)-(v16o*v43x))/v450)}else{v3sl});
        let v45k=(if (v167!=0.0){(((v16l*v44d)-(v16o*v43y))/v450)}else{v3sm});
        let v45l=(if (v167!=0.0){(((v16l*v44e)-(v16o*v43z))/v450)}else{v3sn});
        let v45m=(if (v167!=0.0){(((v16l*v44f)-(v16o*v440))/v450)}else{v1o});
        let v4b8=(v17y*v17y);
        let v4bq=(if v17k{(((v17y*v4aj)-(v181*v4a4))/v4b8)}else{v45i});
        let v4br=(if v17k{(((v17y*v4ak)-(v181*v4a5))/v4b8)}else{v45j});
        let v4bs=(if v17k{(((v17y*v4al)-(v181*v4a6))/v4b8)}else{v45k});
        let v4bt=(if v17k{(((v17y*v4am)-(v181*v4a7))/v4b8)}else{v45l});
        let v4bu=(if v17k{(((v17y*v4an)-(v181*v4a8))/v4b8)}else{v45m});
        let v5iw=(if (sf[240]!=0.0){((-(va*(sf[241]*v1yt)))/(v1iu*v1iu))}else{v1o});
        let v5ix=(if (sf[240]!=0.0){(sf[0]/v1iu)}else{v1o});
        let v5iy=(if (sf[240]!=0.0){(sf[273]/v1iu)}else{v1o});
        let v5j2=(if v1iz{v1o}else{v5iw});
        let v5j3=(if v1iz{v1o}else{v5ix});
        let v5j4=(if v1iz{v1o}else{v5iy});
        let v5j5=(if v1j5{v1o}else{(if v1iz{v5iw}else{v1o})});
        let v5j6=(if v1j5{v1o}else{(if v1iz{v5ix}else{v1o})});
        let v5j7=(if v1j5{v1o}else{(if v1iz{v5iy}else{v1o})});
        let v5j8=scalar_limexp_derivative(v1j3);
        let v5k3=(if (sf[242]!=0.0){((-(va*(sf[243]*v1yt)))/(v1jh*v1jh))}else{v5j2});
        let v5k4=(if (sf[242]!=0.0){(sf[0]/v1jh)}else{v5j3});
        let v5k5=(if (sf[242]!=0.0){(sf[273]/v1jh)}else{v5j4});
        let v5k9=(if v1jm{v1o}else{v5k3});
        let v5ka=(if v1jm{v1o}else{v5k4});
        let v5kb=(if v1jm{v1o}else{v5k5});
        let v5kc=(if v1js{v1o}else{(if v1jm{v5k3}else{v5j5})});
        let v5kd=(if v1js{v1o}else{(if v1jm{v5k4}else{v5j6})});
        let v5ke=(if v1js{v1o}else{(if v1jm{v5k5}else{v5j7})});
        let v5kf=scalar_limexp_derivative(v1jq);
        let v5l3=((if sb[43]{v1o}else{(if (sf[240]!=0.0){((v1j9*(if (sf[148]!=0.0){(sf[60]*(vf0*((sf[25]*v1z2)+(sf[8]*v1z6))))}else{v1o}))+(vf2*((v1j7*v5j5)+(v1j6*(v5j2*v5j8)))))}else{v1o})})+(if sb[45]{v1o}else{(if (sf[242]!=0.0){((v1jw*(if (sf[148]!=0.0){(sf[61]*(vf6*((sf[62]*v1z2)+(sf[63]*v1z6))))}else{v1o}))+(vf8*((v1ju*v5kc)+(v1jt*(v5k9*v5kf)))))}else{v1o})}));
        let v5l4=((if sb[43]{v1o}else{(if (sf[240]!=0.0){(vf2*((v1j7*v5j6)+(v1j6*(v5j3*v5j8))))}else{v1o})})+(if sb[45]{v1o}else{(if (sf[242]!=0.0){(vf8*((v1ju*v5kd)+(v1jt*(v5ka*v5kf))))}else{v1o})}));
        let v5l5=((if sb[43]{v1o}else{(if (sf[240]!=0.0){(vf2*((v1j7*v5j7)+(v1j6*(v5j4*v5j8))))}else{v1o})})+(if sb[45]{v1o}else{(if (sf[242]!=0.0){(vf8*((v1ju*v5ke)+(v1jt*(v5kb*v5kf))))}else{v1o})}));
        let v5ld=(if (sf[244]!=0.0){((-(v7*(sf[245]*v1yt)))/(v1k5*v1k5))}else{v5k9});
        let v5le=(if (sf[244]!=0.0){(sf[273]/v1k5)}else{v1o});
        let v5lf=(if (sf[244]!=0.0){(sf[0]/v1k5)}else{v5ka});
        let v5lg=(if (sf[244]!=0.0){v1o}else{v5kb});
        let v5ll=(if v1ka{v1o}else{v5ld});
        let v5lm=(if v1ka{v1o}else{v5le});
        let v5ln=(if v1ka{v1o}else{v5lf});
        let v5lo=(if v1ka{v1o}else{v5lg});
        let v5lp=(if v1kg{v1o}else{(if v1ka{v5ld}else{v5kc})});
        let v5lq=(if v1kg{v1o}else{(if v1ka{v5le}else{v1o})});
        let v5lr=(if v1kg{v1o}else{(if v1ka{v5lf}else{v5kd})});
        let v5ls=(if v1kg{v1o}else{(if v1ka{v5lg}else{v5ke})});
        let v5lt=scalar_limexp_derivative(v1ke);
        let v5mk=(if sb[47]{v1o}else{(if (sf[244]!=0.0){((v1kk*(if (sf[148]!=0.0){(sf[74]*(vg7*((sf[23]*v1z2)+v22a)))}else{v1o}))+(vg9*((v1ki*v5lp)+(v1kh*(v5ll*v5lt)))))}else{v1o})});
        let v5ml=(if sb[47]{v1o}else{(if (sf[244]!=0.0){(vg9*((v1ki*v5lq)+(v1kh*(v5lm*v5lt))))}else{v1o})});
        let v5mm=(if sb[47]{v1o}else{(if (sf[244]!=0.0){(vg9*((v1ki*v5lr)+(v1kh*(v5ln*v5lt))))}else{v1o})});
        let v5mn=(if sb[47]{v1o}else{(if (sf[244]!=0.0){(vg9*((v1ki*v5ls)+(v1kh*(v5lo*v5lt))))}else{v1o})});
        let v5nj=(v1lb*v1lb);
        let v5oj=(if v1lj{v1o}else{(if v1l8{(((v1lb*v5nc)-(v1la*v5nc))/v5nj)}else{v3hh})});
        let v5ok=(if v1lj{v1o}else{(if v1l8{(((v1lb*v5nd)-(v1la*v5nd))/v5nj)}else{v3hi})});
        let v5ol=(if v1lj{v1o}else{(if v1l8{(((v1lb*v5ne)-(v1la*v5ne))/v5nj)}else{v3hj})});
        let v5om=(if v1lj{v1o}else{(if v1l8{(((v1lb*v5nf)-(v1la*v5nf))/v5nj)}else{v3hk})});
        let v5pi=(v1lx*v1lx);
        let v5qo=(if v1mb{v1o}else{(if v1lu{(((v1lx*v5pb)-(v1lw*v5pb))/v5pi)}else{v3jm})});
        let v5qp=(if v1mb{v1o}else{(if v1lu{(((v1lx*v5pc)-(v1lw*v5pc))/v5pi)}else{v3jn})});
        let v5qq=(if v1mb{v1o}else{(if v1lu{(((v1lx*v5pd)-(v1lw*v5pd))/v5pi)}else{v3jo})});
        let v5qr=(if v1mb{v1o}else{(if v1lu{(((v1lx*v5pe)-(v1lw*v5pe))/v5pi)}else{v3jp})});
        let v5uo=((if v1ks{((v1mu*v5qo)+(v1mc*((v1mt*v5oj)+(v1lk*(vg3*(v1ms*(sf[198]*v5s2)))))))}else{(if vzw{((v11y*v3jm)+(v11g*((v11x*v3hh)+(v10o*(vqp*(v11w*(sf[198]*v3l0)))))))}else{(if vvm{((vxo*v340)+(vx6*((vxn*v31v)+(vwe*(vvj*(vxm*(sf[206]*v35e)))))))}else{(if vqy{((vta*v2oc)+(vsq*((vt9*v2m7)+(vry*(vqr*(vt8*(sf[206]*v2pq)))))))}else{(if vly{((vof*v2b5)+(vnv*((voe*v29d)+(vn1*(vlr*(vod*(sf[198]*v2c8)))))))}else{v1o})})})})})+(if v1ks{((v1n1*(v1l2*(v1mz*(v1mx*v5rj))))+(v1n0*(-v5qo)))}else{(if vzw{((v125*(v106*(v123*(v121*v3kh))))+(v124*(-v3jm)))}else{(if vvm{((vxv*(vvw*(vxt*(vxr*v34v))))+(vxu*(-v340)))}else{(if vqy{((vth*(vrg*(vtf*(vtd*v2p7))))+(vtg*(-v2oc)))}else{(if vly{((vom*(vmi*(vok*(voi*v2bt))))+(vol*(-v2b5)))}else{v1o})})})})}));
        let v5up=((if v1ks{((v1mu*v5qp)+(v1mc*((v1mt*v5ok)+(v1lk*((v1ms*v228)+(vg3*(v1ms*(sf[198]*v5s3))))))))}else{(if vzw{((v11y*v3jn)+(v11g*((v11x*v3hi)+(v10o*((v11w*v2k6)+(vqp*(v11w*(sf[198]*v3l1))))))))}else{(if vvm{((vxo*v341)+(vx6*((vxn*v31w)+(vwe*((vxm*v302)+(vvj*(vxm*(sf[206]*v35f))))))))}else{(if vqy{((vta*v2od)+(vsq*((vt9*v2m8)+(vry*((vt8*v2k8)+(vqr*(vt8*(sf[206]*v2pr))))))))}else{(if vly{((vof*v2b6)+(vnv*((voe*v29e)+(vn1*((vod*v27o)+(vlr*(vod*(sf[198]*v2c9))))))))}else{v1o})})})})})+(if v1ks{((v1n1*((v1mz*v5n0)+(v1l2*(v1mz*(v1mx*v5rk)))))+(v1n0*(-v5qp)))}else{(if vzw{((v125*((v123*v3fy)+(v106*(v123*(v121*v3ki)))))+(v124*(-v3jn)))}else{(if vvm{((vxv*((vxt*v30c)+(vvw*(vxt*(vxr*v34w)))))+(vxu*(-v341)))}else{(if vqy{((vth*((vtf*v2ko)+(vrg*(vtf*(vtd*v2p8)))))+(vtg*(-v2od)))}else{(if vly{((vom*((vok*v284)+(vmi*(vok*(voi*v2bu)))))+(vol*(-v2b6)))}else{v1o})})})})}));
        let v5uq=((if v1ks{((v1mu*v5qq)+(v1mc*((v1mt*v5ol)+(v1lk*(vg3*(v1ms*(sf[198]*v5s4)))))))}else{(if vzw{((v11y*v3jo)+(v11g*((v11x*v3hj)+(v10o*(vqp*(v11w*(sf[198]*v3l2)))))))}else{(if vvm{((vxo*v342)+(vx6*((vxn*v31x)+(vwe*(vvj*(vxm*(sf[206]*v35g)))))))}else{(if vqy{((vta*v2oe)+(vsq*((vt9*v2m9)+(vry*(vqr*(vt8*(sf[206]*v2ps)))))))}else{(if vly{((vof*v2b7)+(vnv*((voe*v29f)+(vn1*(vlr*(vod*(sf[198]*v2ca)))))))}else{v1o})})})})})+(if v1ks{((v1n1*(v1l2*(v1mz*(v1mx*v5rl))))+(v1n0*(-v5qq)))}else{(if vzw{((v125*(v106*(v123*(v121*v3kj))))+(v124*(-v3jo)))}else{(if vvm{((vxv*(vvw*(vxt*(vxr*v34x))))+(vxu*(-v342)))}else{(if vqy{((vth*(vrg*(vtf*(vtd*v2p9))))+(vtg*(-v2oe)))}else{(if vly{((vom*(vmi*(vok*(voi*v2bv))))+(vol*(-v2b7)))}else{v1o})})})})}));
        let v5ur=((if v1ks{((v1mu*v5qr)+(v1mc*((v1mt*v5om)+(v1lk*(vg3*(v1ms*(sf[198]*v5s5)))))))}else{(if vzw{((v11y*v3jp)+(v11g*((v11x*v3hk)+(v10o*(vqp*(v11w*(sf[198]*v3l3)))))))}else{(if vvm{((vxo*v343)+(vx6*((vxn*v31y)+(vwe*(vvj*(vxm*(sf[206]*v35h)))))))}else{(if vqy{((vta*v2of)+(vsq*((vt9*v2ma)+(vry*(vqr*(vt8*(sf[206]*v2pt)))))))}else{v1o})})})})+(if v1ks{((v1n1*(v1l2*(v1mz*(v1mx*v5rm))))+(v1n0*(-v5qr)))}else{(if vzw{((v125*(v106*(v123*(v121*v3kk))))+(v124*(-v3jp)))}else{(if vvm{((vxv*(vvw*(vxt*(vxr*v34y))))+(vxu*(-v343)))}else{(if vqy{((vth*(vrg*(vtf*(vtd*v2pa))))+(vtg*(-v2of)))}else{v1o})})})}));
        let v5z3=(v1o3*v1o3);
        let v5zl=(if v1nv{(((v1o3*v5ye)-(v1o6*v5xz))/v5z3)}else{v4bq});
        let v5zm=(if v1nv{(((v1o3*v5yf)-(v1o6*v5y0))/v5z3)}else{v4br});
        let v5zn=(if v1nv{(((v1o3*v5yg)-(v1o6*v5y1))/v5z3)}else{v4bs});
        let v5zo=(if v1nv{(((v1o3*v5yh)-(v1o6*v5y2))/v5z3)}else{v4bt});
        let v5zp=(if v1nv{(((v1o3*v5yi)-(v1o6*v5y3))/v5z3)}else{v4bu});
        let v61t=(if v1nv{(vg3*((if v1nv{((v1oh*v5zl)+(v1ob*(v1oh*(sf[198]*v608))))}else{(if v17k{((v18d*v4bq)+(v186*(v18d*(sf[212]*v4cd))))}else{(if (v167!=0.0){((v170*v45i)+(v16t*(v170*(sf[210]*v465))))}else{(if v132{((v13o*v3sk)+(v13i*(v13o*(sf[198]*v3t3))))}else{(if vys{((vze*v3cy)+(vz8*(vze*(sf[206]*v3dh))))}else{(if vug{((vv2*v2xa)+(vuw*(vv2*(sf[206]*v2xt))))}else{(if vpl{((vq8*v2i2)+(vq2*(vq8*(sf[198]*v2ih))))}else{v1o})})})})})})})+(vm3*(-v5zl))))}else{(if v1nt{v1o}else{(if v1ks{((if v1ks{(v1kx*(-v5oj))}else{(if vzw{(v101*(-v3hh))}else{(if vvm{(vvr*(-v31v))}else{(if vqy{(vr9*(-v2m7))}else{(if vly{(vmb*(-v29d))}else{v1o})})})})})+v5uo)}else{v1o})})});
        let v61u=(if v1nv{((v1om*v228)+(vg3*((if v1nv{((v1oh*v5zm)+(v1ob*(v1oh*(sf[198]*v609))))}else{(if v17k{((v18d*v4br)+(v186*(v18d*(sf[212]*v4ce))))}else{(if (v167!=0.0){((v170*v45j)+(v16t*(v170*(sf[210]*v466))))}else{(if v132{((v13o*v3sl)+(v13i*(v13o*(sf[198]*v3t4))))}else{(if vys{((vze*v3cz)+(vz8*(vze*(sf[206]*v3di))))}else{(if vug{((vv2*v2xb)+(vuw*(vv2*(sf[206]*v2xu))))}else{(if vpl{((vq8*v2i3)+(vq2*(vq8*(sf[198]*v2ii))))}else{v1o})})})})})})})+(vm3*(-v5zm)))))}else{(if v1nt{v1o}else{(if v1ks{((if v1ks{((v1n4*v5mu)+(v1kx*(-v5ok)))}else{(if vzw{((v128*v3fs)+(v101*(-v3hi)))}else{(if vvm{((vxy*v306)+(vvr*(-v31w)))}else{(if vqy{((vtk*v2ke)+(vr9*(-v2m8)))}else{(if vly{((vop*v27u)+(vmb*(-v29e)))}else{v1o})})})})})+v5up)}else{v1o})})});
        let v61v=(if v1nv{(vg3*((if v1nv{((v1oh*v5zn)+(v1ob*(v1oh*(sf[198]*v60a))))}else{(if v17k{((v18d*v4bs)+(v186*(v18d*(sf[212]*v4cf))))}else{(if (v167!=0.0){((v170*v45k)+(v16t*(v170*(sf[210]*v467))))}else{(if v132{((v13o*v3sm)+(v13i*(v13o*(sf[198]*v3t5))))}else{(if vys{((vze*v3d0)+(vz8*(vze*(sf[206]*v3dj))))}else{(if vug{((vv2*v2xc)+(vuw*(vv2*(sf[206]*v2xv))))}else{(if vpl{((vq8*v2i4)+(vq2*(vq8*(sf[198]*v2ij))))}else{v1o})})})})})})})+(vm3*(-v5zn))))}else{(if v1nt{v1o}else{(if v1ks{((if v1ks{(v1kx*(-v5ol))}else{(if vzw{(v101*(-v3hj))}else{(if vvm{(vvr*(-v31x))}else{(if vqy{(vr9*(-v2m9))}else{(if vly{(vmb*(-v29f))}else{v1o})})})})})+v5uq)}else{v1o})})});
        let v62z=(if (sf[95]!=0.0){v21z}else{v1o});
        let v634=(v1ow*v1ow);
        let v63f=((-(vi4*(if v1ov{v1o}else{(if v1nv{(vg3*((if v1nv{((v1oh*v5zo)+(v1ob*(v1oh*(sf[198]*v60b))))}else{(if v17k{((v18d*v4bt)+(v186*(v18d*(sf[212]*v4cg))))}else{(if (v167!=0.0){((v170*v45l)+(v16t*(v170*(sf[210]*v468))))}else{(if v132{((v13o*v3sn)+(v13i*(v13o*(sf[198]*v3t6))))}else{(if vys{((vze*v3d1)+(vz8*(vze*(sf[206]*v3dk))))}else{(if vug{((vv2*v2xd)+(vuw*(vv2*(sf[206]*v2xw))))}else{v1o})})})})})})+(vm3*(-v5zo))))}else{(if v1nt{v1o}else{(if v1ks{((if v1ks{(v1kx*(-v5om))}else{(if vzw{(v101*(-v3hk))}else{(if vvm{(vvr*(-v31y))}else{(if vqy{(vr9*(-v2ma))}else{v1o})})})})+v5ur)}else{v1o})})})})))/v634);
        let v63j=(if v1p1{((-(vi4*(if v1ov{v1o}else{v61t})))/v634)}else{v1o});
        let v63k=(if v1p1{(((v1ow*v245)-(vi4*(if v1ov{v1o}else{v61u})))/v634)}else{v1o});
        let v63l=(if v1p1{((-(vi4*(if v1ov{v1o}else{v61v})))/v634)}else{v1o});
        let v63m=(if v1p1{v63f}else{v1o});
        let v63n=(if v1p1{((-(vi4*(if v1ov{v1o}else{(if v1nv{(vg3*((if v1nv{((v1oh*v5zp)+(v1ob*(v1oh*(sf[198]*v60c))))}else{(if v17k{((v18d*v4bu)+(v186*(v18d*(sf[212]*v4ch))))}else{(if (v167!=0.0){((v170*v45m)+(v16t*(v170*(sf[210]*v469))))}else{v1o})})})+(vm3*(-v5zp))))}else{v1o})})))/v634)}else{v1o});
        let v63t=(if v1p1{(((vg3*v245)-(vi4*v228))/(vg3*vg3))}else{v1o});
        let v63u=(-v63j);
        let v63v=(-v63k);
        let v63w=(-v63l);
        let v63x=(-v63m);
        let v63y=(-v63n);
        let v643=(v1p5*v1p5);
        let v663=(v1oy*v1oy);
        let v67j=(if v1pv{v1o}else{(if v1p1{((v1pr*v5du)+(v1hx*(if v1pm{(v1pn*(v1pp*(v63u/v1oy)))}else{(if v1p8{((v1pi*(if v1p8{(vi3*(v1pb*(v63u/v1p5)))}else{v1o}))+(v1pd*(v1pg*(v63j/v1p5))))}else{v1o})})))}else{v1o})});
        let v67k=(if v1pv{v1o}else{(if v1p1{((v1pr*v5dv)+(v1hx*(if v1pm{((v1pp*((v1oy*v244)+(vi3*v62z)))+(v1pn*(v1pp*(((v1oy*v63v)-(v1p9*v62z))/v663))))}else{(if v1p8{((v1pi*(if v1p8{((v1pb*v244)+(vi3*(v1pb*(((v1p5*v63v)-(v1p9*v63t))/v643))))}else{v1o}))+(v1pd*(v63t+((v1pg*(((v1p5*v63k)-(v1p3*v63t))/v643))+(v1pf*(v62z-v63t))))))}else{v1o})})))}else{v1o})});
        let v67l=(if v1pv{v1o}else{(if v1p1{((v1pr*v5dw)+(v1hx*(if v1pm{((v1pp*(vi3*sf[281]))+(v1pn*(v1pp*(((v1oy*v63w)-(v1p9*sf[281]))/v663))))}else{(if v1p8{((v1pi*(if v1p8{(vi3*(v1pb*(v63w/v1p5)))}else{v1o}))+(v1pd*((v1pg*(v63l/v1p5))+(v1pf*sf[281]))))}else{v1o})})))}else{v1o})});
        let v67m=(if v1pv{v1o}else{(if v1p1{((v1pr*v5dx)+(v1hx*(if v1pm{((v1pp*(vi3*sf[282]))+(v1pn*(v1pp*(((v1oy*v63x)-(v1p9*sf[282]))/v663))))}else{(if v1p8{((v1pi*(if v1p8{(vi3*(v1pb*(v63x/v1p5)))}else{v1o}))+(v1pd*((v1pg*(v63m/v1p5))+(v1pf*sf[282]))))}else{v1o})})))}else{v1o})});
        let v67n=(if v1pv{v1o}else{(if v1p1{((v1pr*v5dy)+(v1hx*(if v1pm{(v1pn*(v1pp*(v63y/v1oy)))}else{(if v1p8{((v1pi*(if v1p8{(vi3*(v1pb*(v63y/v1p5)))}else{v1o}))+(v1pd*(v1pg*(v63n/v1p5))))}else{v1o})})))}else{v1o})});
        let v68u=(if (v1py!=0.0){((((v48y/sf[246])+(v42v/sf[247]))+(v5du/v1b6))+(v5de/sf[224]))}else{v1o});
        let v68v=(if (v1py!=0.0){((((v493/sf[246])+(v42w/sf[247]))+(((v1b6*v5dv)-(v1hx*v4la))/v4nc))+(v5di/sf[224]))}else{v1o});
        let v68w=(if (v1py!=0.0){((((v494/sf[246])+(v42x/sf[247]))+(((v1b6*v5dw)-(v1hx*v4lb))/v4nc))+(v5dm/sf[224]))}else{v1o});
        let v68x=(if (v1py!=0.0){((((v495/sf[246])+(v42y/sf[247]))+(((v1b6*v5dx)-(v1hx*v4lc))/v4nc))+(v5dq/sf[224]))}else{v1o});
        let v68y=(if (v1py!=0.0){(((v496/sf[246])+(v5dy/v1b6))+(v5dt/sf[224]))}else{v1o});
        let v68z=(v1q9*v68u);
        let v691=(v1q9*v68v);
        let v693=(v1q9*v68w);
        let v695=(v1q9*v68x);
        let v697=(v1q9*v68y);
        let v699=(v2m*v1qc);
        let v69w=(v1qf*v1qf);
        let v6ab=(if (v1py!=0.0){((-(vi8*(if (v1py!=0.0){(vw*(v68u+((v68z+v68z)/v699)))}else{v1o})))/v69w)}else{v1o});
        let v6ac=(if (v1py!=0.0){(((v1qf*(if (sf[148]!=0.0){(sf[99]*(vi6*(sf[100]*v1z2)))}else{v1o}))-(vi8*(if (v1py!=0.0){(vw*(v68v+((v691+v691)/v699)))}else{v1o})))/v69w)}else{v1o});
        let v6ad=(if (v1py!=0.0){((-(vi8*(if (v1py!=0.0){(vw*(v68w+((v693+v693)/v699)))}else{v1o})))/v69w)}else{v1o});
        let v6ae=(if (v1py!=0.0){((-(vi8*(if (v1py!=0.0){(vw*(v68x+((v695+v695)/v699)))}else{v1o})))/v69w)}else{v1o});
        let v6af=(if (v1py!=0.0){((-(vi8*(if (v1py!=0.0){(vw*(v68y+((v697+v697)/v699)))}else{v1o})))/v69w)}else{v1o});
        let v6b5=(if v1qk{(vco*(v1kp*(sf[248]*v6ab)))}else{v1o});
        let v6b6=(if v1qk{((v1qn*v1yx)+(vco*((v1qm*(v5l3+v5mk))+(v1kp*(sf[248]*v6ac)))))}else{v1o});
        let v6b7=(if v1qk{(vco*((v1qm*v5ml)+(v1kp*(sf[248]*v6ad))))}else{v1o});
        let v6b8=(if v1qk{(vco*((v1qm*(v5l4+v5mm))+(v1kp*(sf[248]*v6ae))))}else{v1o});
        let v6b9=(if v1qk{(vco*((v1qm*(v5l5+v5mn))+(v1kp*(sf[248]*v6af))))}else{v1o});
        let v6bz=(if v1qt{((v1qv*v6ab)+(v1qh*(-(vw*v6b5))))}else{v6ab});
        let v6c0=(if v1qt{((v1qv*v6ac)+(v1qh*(-(vw*v6b6))))}else{v6ac});
        let v6c1=(if v1qt{((v1qv*v6ad)+(v1qh*(-(vw*v6b7))))}else{v6ad});
        let v6c2=(if v1qt{((v1qv*v6ae)+(v1qh*(-(vw*v6b8))))}else{v6ae});
        let v6c3=(if v1qt{((v1qv*v6af)+(v1qh*(-(vw*v6b9))))}else{v6af});
        let v6cr=(v1qp*v1qp);
        let v6dl=(if (sf[249]!=0.0){(sf[250]*v1yt)}else{v1o});
        let v6dm=(sf[0]/v1rc);
        let v6dp=(v1rc*v1rc);
        let v6dr=(sf[273]/v1rc);
        let v6ds=scalar_limexp_derivative(v1rd);
        let v6e2=scalar_limexp_derivative(v1rg);
        let v6ex=(if (sf[251]!=0.0){(sf[0]/v1rr)}else{v1o});
        let v6ey=(if (sf[251]!=0.0){((-(ve*(sf[252]*v1yt)))/(v1rr*v1rr))}else{v5ll});
        let v6ez=(if (sf[251]!=0.0){(sf[273]/v1rr)}else{v5lm});
        let v6f0=(if (sf[251]!=0.0){v1o}else{v5ln});
        let v6f1=(if (sf[251]!=0.0){v1o}else{v5lo});
        let v6fh=scalar_limexp_derivative(v1s0);
        let v6zd=-0.0;
        let v702=(v1r7*v1r7);

        stamper.stamp_current_node1_local(
            Some(6),
            Some(7),
            multiplicity * ((v9*v1o)),
            7,
            multiplicity * (v6zd),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            Some(5),
            multiplicity * ((v6*v1o)),
            5,
            multiplicity * (v6zd),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(3),
            multiplicity * ((sf[0]*(if sb[49]{v1o}else{(if (sf[249]!=0.0){(vk7*v1rj)}else{v1o})}))),
            [1, 3, 4, 5],
            [(sf[0]*(if sb[49]{v1o}else{(if (sf[249]!=0.0){(vk7*(if (sf[249]!=0.0){(v6dm*v6ds)}else{v1o}))}else{v1o})})), (sf[0]*(if sb[49]{v1o}else{(if (sf[249]!=0.0){(vk7*(-(if (sf[249]!=0.0){(v6dm*v6e2)}else{v1o})))}else{v1o})})), (sf[0]*(if sb[49]{v1o}else{(if (sf[249]!=0.0){((v1rj*(if (sf[148]!=0.0){(sf[124]*(vk5*(v22a+v25v)))}else{v1o}))+(vk7*((if (sf[249]!=0.0){(((-(v4*v6dl))/v6dp)*v6ds)}else{v1o})-(if (sf[249]!=0.0){(((-(ve*v6dl))/v6dp)*v6e2)}else{v1o}))))}else{v1o})})), (sf[0]*(if sb[49]{v1o}else{(if (sf[249]!=0.0){(vk7*((if (sf[249]!=0.0){(v6dr*v6ds)}else{v1o})-(if (sf[249]!=0.0){(v6dr*v6e2)}else{v1o})))}else{v1o})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(3),
            Some(5),
            multiplicity * ((sf[0]*(if sb[51]{v1o}else{(if (sf[251]!=0.0){(vk3*v1s6)}else{v1o})}))),
            [3, 4, 5, 6, 7],
            [(sf[0]*(if sb[51]{v1o}else{(if (sf[251]!=0.0){(vk3*((v1s4*(if v1s2{v1o}else{(if v1rw{v6ex}else{v1o})}))+(v1s3*((if v1rw{v1o}else{v6ex})*v6fh))))}else{v1o})})), (sf[0]*(if sb[51]{v1o}else{(if (sf[251]!=0.0){((v1s6*(if (sf[148]!=0.0){(sf[123]*(vk1*(v25v+(sf[14]*v1z6))))}else{v1o}))+(vk3*((v1s4*(if v1s2{v1o}else{(if v1rw{v6ey}else{v5lp})}))+(v1s3*((if v1rw{v1o}else{v6ey})*v6fh)))))}else{v1o})})), (sf[0]*(if sb[51]{v1o}else{(if (sf[251]!=0.0){(vk3*((v1s4*(if v1s2{v1o}else{(if v1rw{v6ez}else{v5lq})}))+(v1s3*((if v1rw{v1o}else{v6ez})*v6fh))))}else{v1o})})), (sf[0]*(if sb[51]{v1o}else{(if (sf[251]!=0.0){(vk3*((v1s4*(if v1s2{v1o}else{(if v1rw{v6f0}else{v5lr})}))+(v1s3*((if v1rw{v1o}else{v6f0})*v6fh))))}else{v1o})})), (sf[0]*(if sb[51]{v1o}else{(if (sf[251]!=0.0){(vk3*((v1s4*(if v1s2{v1o}else{(if v1rw{v6f1}else{v5ls})}))+(v1s3*((if v1rw{v1o}else{v6f1})*v6fh))))}else{v1o})}))],
            [],
            [],
            multiplicity,
        );
        let v1xb_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v1xb);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(5),
            multiplicity * (v1xb_ddt),
            [1, 3, 4, 5, 6, 7],
            [((v6y3) * ddt_scale), ((v6y4) * ddt_scale), ((v6y5) * ddt_scale), ((v6y6) * ddt_scale), ((v6y7) * ddt_scale), ((v6y8) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1xc_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v1xc);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v1xc_ddt),
            [1, 4, 5, 6],
            [((v6y9) * ddt_scale), ((v6ya) * ddt_scale), ((v6yb) * ddt_scale), ((v6yc) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1xd_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, v1xd);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (v1xd_ddt),
            1,
            multiplicity * (((sf[298]) * ddt_scale)),
            5,
            multiplicity * (((sf[299]) * ddt_scale)),
        );
        let v1xe_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, v1xe);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v1xe_ddt),
            1,
            multiplicity * (((sf[300]) * ddt_scale)),
            2,
            multiplicity * (((sf[301]) * ddt_scale)),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(2),
            multiplicity * ((if (sf[267]!=0.0){(vi/vlb)}else{v1o})),
            2,
            multiplicity * ((if (sf[267]!=0.0){(v1y7/vlb)}else{v1o})),
            4,
            multiplicity * ((if (sf[267]!=0.0){((-(vi*(if (sf[148]!=0.0){(sf[141]*(vl9*(sf[142]*v1z2)))}else{v1o})))/(vlb*vlb))}else{v1o})),
            7,
            multiplicity * ((if (sf[267]!=0.0){(v1c/vlb)}else{v1o})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(2),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v1o,
        );
        stamper.stamp_current_node3_local(
            Some(5),
            Some(0),
            multiplicity * ((if (sf[268]!=0.0){(vk/vl3)}else{v1o})),
            0,
            multiplicity * ((if (sf[268]!=0.0){(v1y7/vl3)}else{v1o})),
            4,
            multiplicity * ((if (sf[268]!=0.0){((-(vk*(if (sf[148]!=0.0){(sf[137]*(vl1*(sf[138]*v1z2)))}else{v1o})))/(vl3*vl3))}else{v1o})),
            5,
            multiplicity * ((if (sf[268]!=0.0){(v1c/vl3)}else{v1o})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(0),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v1o,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(6),
            multiplicity * ((if (sf[269]!=0.0){(vl/v1r7)}else{v1o})),
            [1, 4, 5, 6, 7],
            [(if (sf[269]!=0.0){((v1r7-(vl*(if v1r5{v1o}else{(if v1qz{(((v1qp*((v1r1*v6bz)+(v1qx*(v6b5/v1r0))))-(v1r2*v6b5))/v6cr)}else{v6bz})})))/v702)}else{v1o}), (if (sf[269]!=0.0){((-(vl*((if (sf[148]!=0.0){(sf[139]*(vl5*(sf[140]*v1z2)))}else{v1o})+(if v1r5{v1o}else{(if v1qz{(((v1qp*((v1r1*v6c0)+(v1qx*(v6b6/v1r0))))-(v1r2*v6b6))/v6cr)}else{v6c0})}))))/v702)}else{v1o}), (if (sf[269]!=0.0){((-(vl*(if v1r5{v1o}else{(if v1qz{(((v1qp*((v1r1*v6c1)+(v1qx*(v6b7/v1r0))))-(v1r2*v6b7))/v6cr)}else{v6c1})})))/v702)}else{v1o}), (if (sf[269]!=0.0){(((-v1r7)-(vl*(if v1r5{v1o}else{(if v1qz{(((v1qp*((v1r1*v6c2)+(v1qx*(v6b8/v1r0))))-(v1r2*v6b8))/v6cr)}else{v6c2})})))/v702)}else{v1o}), (if (sf[269]!=0.0){((-(vl*(if v1r5{v1o}else{(if v1qz{(((v1qp*((v1r1*v6c3)+(v1qx*(v6b9/v1r0))))-(v1r2*v6b9))/v6cr)}else{v6c3})})))/v702)}else{v1o})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(6),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v1o,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * ((sf[0]*(v1ko-v1pw))),
            [1, 4, 5, 6, 7],
            [(sf[0]*(-v67j)), (sf[0]*(v5mk-v67k)), (sf[0]*(v5ml-v67l)), (sf[0]*(v5mm-v67m)), (sf[0]*(v5mn-v67n))],
            [],
            [],
            multiplicity,
        );
        let v1xg_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, v1xg);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(5),
            multiplicity * (v1xg_ddt),
            [1, 4, 5, 6, 7],
            [((v6ym) * ddt_scale), ((v6yn) * ddt_scale), ((v6yo) * ddt_scale), ((v6yp) * ddt_scale), ((v6yq) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((sf[0]*v1k1)),
            4,
            multiplicity * ((sf[0]*v5l3)),
            6,
            multiplicity * ((sf[0]*v5l4)),
            7,
            multiplicity * ((sf[0]*v5l5)),
        );
        let v1xi_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, v1xi);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v1xi_ddt),
            [1, 4, 5, 6, 7, 8],
            [((v6yu) * ddt_scale), ((v6yv) * ddt_scale), ((v6yw) * ddt_scale), ((v6yx) * ddt_scale), ((v6yy) * ddt_scale), ((sf[302]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((sf[0]*(v1wr-v1hu))),
            [1, 4, 5, 6, 7, 9],
            [(sf[0]*(v6vw-v5de)), (sf[0]*(v6vx-v5di)), (sf[0]*(v6vy-v5dm)), (sf[0]*(v6vz-v5dq)), (sf[0]*(v6w0-v5dt)), sf[302]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v1o,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * ((if sb[71]{((vc7/vli)-(if (sf[260]!=0.0){((vb*v1hy)+(v1ox*v1pw))}else{v1o}))}else{v1o})),
            [1, 4, 5, 6, 7],
            [(if sb[71]{(-(if (sf[260]!=0.0){((vb*(v5du-v5de))+(v1ox*v67j))}else{v1o}))}else{v1o}), (if sb[71]{(((vli-(vc7*(if (sf[148]!=0.0){((vlg*(sf[143]*(vld*(sf[144]*v1z2))))+(vle*(sf[145]*v1yy)))}else{v1o})))/(vli*vli))-(if (sf[260]!=0.0){((vb*(v5dv-v5di))+((v1pw*v21z)+(v1ox*v67k)))}else{v1o}))}else{v1o}), (if sb[71]{(-(if (sf[260]!=0.0){(((sf[0]*v1hy)+(vb*(v5dw-v5dm)))+((sf[0]*v1pw)+(v1ox*v67l)))}else{v1o}))}else{v1o}), (if sb[71]{(-(if (sf[260]!=0.0){(((v1hy*sf[274])+(vb*(v5dx-v5dq)))+((v1pw*sf[273])+(v1ox*v67m)))}else{v1o}))}else{v1o}), (if sb[71]{(-(if (sf[260]!=0.0){(((v1hy*sf[273])+(vb*(v5dy-v5dt)))+(v1ox*v67n))}else{v1o}))}else{v1o})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((if sb[71]{(if sb[68]{v1y2}else{v1o})}else{v1o})),
            4,
            multiplicity * ((if sb[71]{(if sb[68]{(sf[270]*ddt_scale)}else{v1o})}else{v1o})),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * ((if sb[59]{v1wi}else{(if (sf[262]!=0.0){(v1wj-v1io)}else{v1o})})),
            [1, 4, 5, 6, 7, 8],
            [(if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6ux-v5if)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6uy-v5ig)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6uz-v5ih)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6v0-v5ii)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6v1-v5ij)}else{v1o})}), sf[291]],
            [],
            [],
            multiplicity,
        );
        let v1x0_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, v1x0);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            None,
            multiplicity * (v1x0_ddt),
            [1, 4, 5, 6, 7, 8],
            [((v6wz) * ddt_scale), ((v6x0) * ddt_scale), ((v6x1) * ddt_scale), ((v6x2) * ddt_scale), ((v6x3) * ddt_scale), ((sf[292]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            None,
            multiplicity * ((if sb[59]{v1wq}else{(if (sf[262]!=0.0){(v1wr-v1hx)}else{v1o})})),
            [1, 4, 5, 6, 7, 9],
            [(if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6vw-v5du)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6vx-v5dv)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6vy-v5dw)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6vz-v5dx)}else{v1o})}), (if sb[59]{v1o}else{(if (sf[262]!=0.0){(v6w0-v5dy)}else{v1o})}), sf[291]],
            [],
            [],
            multiplicity,
        );
        let v1x2_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v1x2);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            None,
            multiplicity * (v1x2_ddt),
            [1, 4, 5, 6, 7, 9],
            [((v6xa) * ddt_scale), ((v6xb) * ddt_scale), ((v6xc) * ddt_scale), ((v6xd) * ddt_scale), ((v6xe) * ddt_scale), ((sf[293]) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(6),
            multiplicity * (v1o),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(0),
            multiplicity * (v1o),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(2),
            multiplicity * (v1o),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (v1o),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(7),
            multiplicity * (v1o),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (v1o),
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
            v1, v2, v4, v5, v6, v7, v8, v9, 
            va, vb, ve, vf, vw, v1c, v1o, v2m, 
            vc7, vcm, vco, vcq, vcu, vcx, vfx, vg3, 
            vlr, vly, vm0, vm3, vmb, vmi, vmm, vmp, 
            vmr, vms, vn0, vnd, vnf, vng, vnu, vo2, 
            vo6, vpl, vpu, vpx, vq6, vqp, vqr, vqy, 
            vr0, vr9, vrg, vrm, vro, vrp, vrx, vs8, 
            vsa, vsb, vsp, vsx, vt1, vug, vuo, vur, 
            vv0, vvj, vvm, vvn, vvr, vvw, vw2, vw4, 
            vw5, vwd, vwo, vwq, vwr, vx5, vxd, vxh, 
            vys, vz0, vz3, vzc, vzw, vzx, v101, v106, 
            v10c, v10e, v10f, v10n, v10y, v110, v111, v11f, 
            v11n, v11r, v132, v13a, v13d, v13m, v165, v167, 
            v16l, v16o, v16x, v17h, v17k, v17y, v181, v18a, 
            v1b6, v1db, v1hu, v1hx, v1io, v1kr, v1ks, v1kt, 
            v1kx, v1l2, v1l8, v1la, v1lb, v1lj, v1lu, v1lw, 
            v1lx, v1mb, v1mj, v1mn, v1nv, v1o3, v1o6, v1of, 
            v1wi, v1wj, v1wq, v1wr, v1x0, v1x2, v1xb, v1xc, 
            v1xd, v1xe, v1xg, v1xi, v1y1, v1yt, v1yx, v1yy, 
            v1z2, v1z6, v21z, v228, v27o, v27u, v284, v28g, 
            v28h, v28i, v2a1, v2a2, v2a3, v2bt, v2bu, v2bv, 
            v2c8, v2c9, v2ca, v2h2, v2h3, v2h4, v2hb, v2hc, 
            v2hd, v2ih, v2ii, v2ij, v2k6, v2k8, v2ke, v2ko, 
            v2l0, v2l1, v2l2, v2l3, v2mz, v2n0, v2n1, v2n2, 
            v2p7, v2p8, v2p9, v2pa, v2pq, v2pr, v2ps, v2pt, 
            v2vz, v2w0, v2w1, v2w2, v2wb, v2wc, v2wd, v2we, 
            v2xt, v2xu, v2xv, v2xw, v302, v306, v30c, v30o, 
            v30p, v30q, v30r, v32n, v32o, v32p, v32q, v34v, 
            v34w, v34x, v34y, v35e, v35f, v35g, v35h, v3bn, 
            v3bo, v3bp, v3bq, v3bz, v3c0, v3c1, v3c2, v3dh, 
            v3di, v3dj, v3dk, v3fs, v3fy, v3ga, v3gb, v3gc, 
            v3gd, v3i9, v3ia, v3ib, v3ic, v3kh, v3ki, v3kj, 
            v3kk, v3l0, v3l1, v3l2, v3l3, v3r9, v3ra, v3rb, 
            v3rc, v3rl, v3rm, v3rn, v3ro, v3t3, v3t4, v3t5, 
            v3t6, v42v, v42w, v42x, v42y, v43w, v43x, v43y, 
            v43z, v440, v44b, v44c, v44d, v44e, v44f, v465, 
            v466, v467, v468, v469, v48y, v493, v494, v495, 
            v496, v4a4, v4a5, v4a6, v4a7, v4a8, v4aj, v4ak, 
            v4al, v4am, v4an, v4cd, v4ce, v4cf, v4cg, v4ch, 
            v4la, v4lb, v4lc, v4nc, v5de, v5di, v5dm, v5dq, 
            v5dt, v5du, v5dv, v5dw, v5dx, v5dy, v5if, v5ig, 
            v5ih, v5ii, v5ij, v5mu, v5n0, v5nc, v5nd, v5ne, 
            v5nf, v5pb, v5pc, v5pd, v5pe, v5rj, v5rk, v5rl, 
            v5rm, v5s2, v5s3, v5s4, v5s5, v5xz, v5y0, v5y1, 
            v5y2, v5y3, v5ye, v5yf, v5yg, v5yh, v5yi, v608, 
            v609, v60a, v60b, v60c, v6ux, v6uy, v6uz, v6v0, 
            v6v1, v6vw, v6vx, v6vy, v6vz, v6w0, v6wz, v6x0, 
            v6x1, v6x2, v6x3, v6xa, v6xb, v6xc, v6xd, v6xe, 
            v6y3, v6y4, v6y5, v6y6, v6y7, v6y8, v6y9, v6ya, 
            v6yb, v6yc, v6ym, v6yn, v6yo, v6yp, v6yq, v6yu, 
            v6yv, v6yw, v6yx, v6yy, 
        }=self.eval_common_stamp_values(ctx);
        let v1y2=0.0;

        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7]],
            &[v6y3, v6y4, v6y5, v6y6, v6y7, v6y8],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &[nodes[1], nodes[4], nodes[5], nodes[6]],
            &[v6y9, v6ya, v6yb, v6yc],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes[1],
            multiplicity * (sf[298]),
            nodes[5],
            multiplicity * (sf[299]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (sf[300]),
            nodes[2],
            multiplicity * (sf[301]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7]],
            &[v6ym, v6yn, v6yo, v6yp, v6yq],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8]],
            &[v6yu, v6yv, v6yw, v6yx, v6yy, sf[302]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * ((if sb[71]{(if sb[68]{(sf[270]*1.0)}else{v1o})}else{v1o})),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8]],
            &[v6wz, v6x0, v6x1, v6x2, v6x3, sf[292]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            None,
            &[nodes[1], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[v6xa, v6xb, v6xc, v6xd, v6xe, sf[293]],
            &[],
            &[],
            multiplicity,
        );
    }
}
