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
    v0: f64, v1: f64, v2: f64, v1m: f64, v1v: f64, v2z: f64, 
    v49: f64, v4i: f64, v4p: f64, v69: f64, v6b: f64, v6c: f64, 
    v6d: f64, v6x: f64, v72: f64, v74: f64, v75: f64, v8h: f64, 
    vb2: f64, vb5: f64, vb6: f64, vbb: f64, vbd: f64, vbw: f64, 
    vby: f64, vbz: f64, vde: f64, ver: f64, vf1: f64, v1cf: f64, 
    v1cg: f64, v1ch: f64, v1d9: f64, v1db: f64, v1dc: f64, v1ec: f64, 
    v1ee: f64, v3oq: f64, v3or: f64, v3os: f64, v3ot: f64, v3ou: f64, 
    v3ov: f64, v3ow: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let v0=1.0;let v1=0.0;let v2=multiplicity;let vc=273.15;let v14=1.3806505e-23;let v16=1.60217653e-19;let v1m=2.0;let v1v=0.5;let v2z=(v2*sf[61]);let v30=(v2z).sqrt();let v33=(if (sf[62]!=0.0){(sf[66]+(sf[69]/v30))}else{sf[55]});let v3c=((v2*sf[60])).sqrt();let v3f=(if (sf[62]!=0.0){(sf[73]+(sf[76]/v3c))}else{sf[56]});let v42=(if sb[9]{(sf[68]/v30)}else{v1});let v49=(if sb[9]{(v33+(sf[63]*((sf[84]+(v42*v42))).sqrt()))}else{v33});let v4i=(if sb[15]{(sf[75]/v3c)}else{v42});
        let v4p=(if sb[15]{(v3f+(sf[70]*((sf[86]+(v4i*v4i))).sqrt()))}else{v3f});let v5g=(if sb[24]{sf[15]}else{(if (sf[91]!=0.0){v49}else{sf[60]})});let v5h=(if sb[24]{sf[17]}else{(if (sf[91]!=0.0){v4p}else{sf[61]})});let v5k=(v0/f64::powf(v5g,sf[92]));let v5n=(v0/f64::powf(v5h,sf[93]));let v68=((((sf[94]*(v0+(v5k*sf[95])))*(v0+(v5n*sf[96])))*(v0+(v5n*(v5k*sf[97]))))*sf[276]);let v69=0.1;let v6b=(if (v68>v69){v68}else{v69});let v6c=(v6b).sqrt();let v6d=10000.0;let v6f=(v6c/(v6b+v6d));
        let v6s=(if (sf[100]!=0.0){v1}else{(sf[101]+((((v5h*sf[102])+(v5g*sf[103]))+sf[104])/(v5g*v5h)))});let v6u=(if (v6s<v6f){v0}else{v1});let v6x=(if (v6u!=0.0){(if (v6s>v1){v6s}else{v1})}else{v6s});let v72=(if (!(v6u!=0.0)){(v6x*v6x)}else{(if (v6u!=0.0){(v6f*v6f)}else{v1})});let v74=(v1v*v6b);let v75=((v1v/v72)-v74);let v8h=4.0;let vb2=ctx.node_voltage(nodes[3]);let vb5=ctx.node_voltage(nodes[5]);let vb6=ctx.node_voltage(nodes[4]);let vb9=ctx.node_voltage(nodes[1]);let vbb=(sf[173]*(vb9-vb6));
        let vbd=(sf[173]*(vb9-vb5));let vbf=((sf[254]+vb2)-vc);let vbh=(if (vbf<sf[11]){v0}else{v1});let vbk=(((vbf-sf[10])-v0)).exp();let vbm=(if (vbh!=0.0){(sf[10]+vbk)}else{vbf});let vbq=(((if (vbm>sf[13]){v0}else{v1})!=0.0)&&(!(vbh!=0.0)));let vbt=(((sf[12]-vbm)-v0)).exp();let vbw=(vc+(if vbq{(sf[12]-vbt)}else{vbm}));let vby=((v14*vbw)/v16);let vbz=(vbw/sf[8]);let vde=(vbz).ln();let vel=(v1m*(vby/vbz));let veo=(vbz*sf[189]);let veq=((veo/vby)).exp();let ver=-0.5;let vet=(vbz*sf[190]);
        let vev=((vet/vby)).exp();let vew=(veq-vev);let vex=(vew).ln();let vez=(if (sf[187]!=0.0){(vel*vex)}else{v1});let vf1=3.0;let vf2=(vby*vf1);let vf3=(vde*vf2);let vf6=(sf[179]*(vbz-v0));let vf8=(if (sf[187]!=0.0){(((vbz*vez)-vf3)-vf6)}else{v1});let vf9=(v1m*vby);let vfa=(-vf8);let vfc=((vfa/vby)).exp();let vff=((v0+(v8h*vfc))).sqrt();let vfh=(v1v*(v0+vff));let vfi=(vfh).ln();let vfl=(if (sf[187]!=0.0){(vf8+(vf9*vfi))}else{v1});let vfm=(sf[188]/vfl);let vfs=(if sb[44]{sf[188]}else{vfl});
        let vfy=(vbz*sf[194]);let vg0=((vfy/vby)).exp();let vg2=(vbz*sf[195]);let vg4=((vg2/vby)).exp();let vg5=(vg0-vg4);let vg6=(vg5).ln();let vg8=(if (sf[192]!=0.0){(vel*vg6)}else{v1});let vgc=(if (sf[192]!=0.0){(((vbz*vg8)-vf3)-vf6)}else{v1});let vgd=(-vgc);let vgf=((vgd/vby)).exp();let vgi=((v0+(v8h*vgf))).sqrt();let vgk=(v1v*(v0+vgi));let vgl=(vgk).ln();let vgo=(if (sf[192]!=0.0){(vgc+(vf9*vgl))}else{v1});let vgp=(sf[193]/vgo);let vgv=(if sb[46]{sf[193]}else{vgo});
        let vgw=(if sb[46]{v1}else{(if (sf[192]!=0.0){(sf[167]*f64::powf(vgp,sf[196]))}else{v1})});let v12p=(v75+vbb);let v12r=0.04;let v12t=(((v12p*v12p)+v12r)).sqrt();let v12y=(if sb[61]{vbb}else{(if sb[60]{(v1v*((vbb-v75)+v12t))}else{v1})});let v12z=(sf[20]*(if sb[44]{v1}else{(if (sf[187]!=0.0){(sf[165]*f64::powf(vfm,sf[191]))}else{v1})}));let v130=(if (sf[221]!=0.0){v12z}else{v1});let v132=(if (sf[221]!=0.0){(sf[22]*vgw)}else{v1});let v134=(if (v130>v1){v0}else{v1});
        let v135=((sf[221]!=0.0)&&(v134!=0.0));let v136=(-vfs);let v138=(v136*sf[222]);let v139=(if v135{v138}else{v1});let v13d=(v135&&(sf[224]!=0.0));let v13e=(v12y+v139);let v13f=(if v13d{v13e}else{v1});let v13h=(if (v13f>v1){v0}else{v1});let v13i=(v13d&&(v13h!=0.0));let v13m=(if v13i{sf[227]}else{v1});let v13o=(v0-(sf[225]*v13m));let v13u=(v13f*sf[229]);let v13v=(vfs*sf[225]);let v13x=(v0+(v13u/v13v));let v142=(v13d&&(!(v13h!=0.0)));let v144=(v0-(v12y/vfs));let v146=(v0-f64::powf(v144,sf[228]));
        let v149=(if v142{((vfs*v146)/sf[228])}else{(if v13i{((vfs*v13o)/sf[228])}else{v1})});let v14e=(v135&&sb[63]);let v14j=(((v139*v139)+sf[231])).sqrt();let v14o=(if v14e{v13e}else{v1});let v14r=((sf[231]+(v14o*v14o))).sqrt();let v14w=(if v14e{((v1v*(v14o-(if v14e{v14r}else{v1})))-v139)}else{v1});let v14y=(v0-(v14w/vfs));let v14z=f64::powf(v14y,sf[228]);let v154=((if v14e{(ver*(v139+(if v14e{v14j}else{v1})))}else{v1})+(v12y-v14w));let v155=(sf[227]*v154);let v156=(sf[229]*v154);
        let v158=(v0+(v156/v13v));let v15d=((sf[221]!=0.0)&&(!(v134!=0.0)));let v15e=(if v15d{v1}else{(if v14e{((if v14e{((v136*v14z)/sf[228])}else{v149})+(v155*v158))}else{(if v13d{(v149+(if v142{v1}else{(if v13i{(v13m*(v13f*v13x))}else{v1})}))}else{v1})})});let v15g=(if (v132>v1){v0}else{v1});let v15h=((sf[221]!=0.0)&&(v15g!=0.0));let v15i=(-vgv);let v15j=(sf[222]*v15i);let v15k=(if v15h{v15j}else{v1});let v15o=(v15h&&(sf[233]!=0.0));let v15p=(v12y+v15k);let v15q=(if v15o{v15p}else{v1});
        let v15s=(if (v15q>v1){v0}else{v1});let v15t=(v15o&&(v15s!=0.0));let v15w=(if v15t{sf[235]}else{v1});let v15y=(v0-(sf[225]*v15w));let v164=(v15q*sf[237]);let v165=(vgv*sf[225]);let v167=(v0+(v164/v165));let v16c=(v15o&&(!(v15s!=0.0)));let v16e=(v0-(v12y/vgv));let v16g=(v0-f64::powf(v16e,sf[236]));let v16j=(if v16c{((vgv*v16g)/sf[236])}else{(if v15t{((vgv*v15y)/sf[236])}else{v1})});let v16o=(v15h&&sb[65]);let v16t=(((v15k*v15k)+sf[239])).sqrt();let v16y=(if v16o{v15p}else{v1});
        let v171=((sf[239]+(v16y*v16y))).sqrt();let v176=(if v16o{((v1v*(v16y-(if v16o{v171}else{v1})))-v15k)}else{v1});let v178=(v0-(v176/vgv));let v179=f64::powf(v178,sf[236]);let v17e=((if v16o{(ver*(v15k+(if v16o{v16t}else{v1})))}else{v1})+(v12y-v176));let v17f=(sf[235]*v17e);let v17g=(sf[237]*v17e);let v17i=(v0+(v17g/v165));let v17n=((sf[221]!=0.0)&&(!(v15g!=0.0)));
        let v17o=(if v17n{v1}else{(if v16o{((if v16o{((v15i*v179)/sf[236])}else{v16j})+(v17f*v17i))}else{(if v15o{(v16j+(if v16c{v1}else{(if v15t{(v15w*(v15q*v167))}else{v1})}))}else{v1})})});let v17z=(v75+vbd);let v182=((v12r+(v17z*v17z))).sqrt();let v187=(if sb[69]{vbd}else{(if sb[68]{(v1v*((vbd-v75)+v182))}else{v12y})});let v188=(if (sf[240]!=0.0){v12z}else{v1});let v18a=(if (sf[240]!=0.0){(sf[24]*vgw)}else{v1});let v18c=(if (v188>v1){v0}else{v1});let v18d=((sf[240]!=0.0)&&(v18c!=0.0));
        let v18e=(if v18d{v138}else{v1});let v18f=((sf[224]!=0.0)&&v18d);let v18g=(v187+v18e);let v18h=(if v18f{v18g}else{v1});let v18j=(if (v18h>v1){v0}else{v1});let v18k=(v18f&&(v18j!=0.0));let v18l=(if v18k{sf[227]}else{v1});let v18n=(v0-(sf[225]*v18l));let v18r=(sf[229]*v18h);let v18t=(v0+(v18r/v13v));let v18y=(v18f&&(!(v18j!=0.0)));let v190=(v0-(v187/vfs));let v192=(v0-f64::powf(v190,sf[228]));let v195=(if v18y{((vfs*v192)/sf[228])}else{(if v18k{((vfs*v18n)/sf[228])}else{v1})});let v199=(sb[63]&&v18d);
        let v19c=((sf[231]+(v18e*v18e))).sqrt();let v19h=(if v199{v18g}else{v1});let v19k=((sf[231]+(v19h*v19h))).sqrt();let v19p=(if v199{((v1v*(v19h-(if v199{v19k}else{v1})))-v18e)}else{v1});let v19r=(v0-(v19p/vfs));let v19s=f64::powf(v19r,sf[228]);let v19x=((if v199{(ver*(v18e+(if v199{v19c}else{v1})))}else{v1})+(v187-v19p));let v19y=(sf[227]*v19x);let v19z=(sf[229]*v19x);let v1a1=(v0+(v19z/v13v));let v1a6=((sf[240]!=0.0)&&(!(v18c!=0.0)));
        let v1a7=(if v1a6{v1}else{(if v199{((if v199{((v136*v19s)/sf[228])}else{v195})+(v19y*v1a1))}else{(if v18f{(v195+(if v18y{v1}else{(if v18k{(v18l*(v18h*v18t))}else{v1})}))}else{v1})})});let v1a9=(if (v18a>v1){v0}else{v1});let v1aa=((sf[240]!=0.0)&&(v1a9!=0.0));let v1ab=(if v1aa{v15j}else{v1});let v1ac=((sf[233]!=0.0)&&v1aa);let v1ad=(v187+v1ab);let v1ae=(if v1ac{v1ad}else{v1});let v1ag=(if (v1ae>v1){v0}else{v1});let v1ah=(v1ac&&(v1ag!=0.0));let v1ai=(if v1ah{sf[235]}else{v1});
        let v1ak=(v0-(sf[225]*v1ai));let v1ao=(sf[237]*v1ae);let v1aq=(v0+(v1ao/v165));let v1av=(v1ac&&(!(v1ag!=0.0)));let v1ax=(v0-(v187/vgv));let v1az=(v0-f64::powf(v1ax,sf[236]));let v1b2=(if v1av{((vgv*v1az)/sf[236])}else{(if v1ah{((vgv*v1ak)/sf[236])}else{v1})});let v1b6=(sb[65]&&v1aa);let v1b9=((sf[239]+(v1ab*v1ab))).sqrt();let v1be=(if v1b6{v1ad}else{v1});let v1bh=((sf[239]+(v1be*v1be))).sqrt();let v1bm=(if v1b6{((v1v*(v1be-(if v1b6{v1bh}else{v1})))-v1ab)}else{v1});let v1bo=(v0-(v1bm/vgv));
        let v1bp=f64::powf(v1bo,sf[236]);let v1bu=((if v1b6{(ver*(v1ab+(if v1b6{v1b9}else{v1})))}else{v1})+(v187-v1bm));let v1bv=(sf[235]*v1bu);let v1bw=(sf[237]*v1bu);let v1by=(v0+(v1bw/v165));let v1c3=((sf[240]!=0.0)&&(!(v1a9!=0.0)));let v1c4=(if v1c3{v1}else{(if v1b6{((if v1b6{((v15i*v1bp)/sf[236])}else{v1b2})+(v1bv*v1by))}else{(if v1ac{(v1b2+(if v1av{v1}else{(if v1ah{(v1ai*(v1ae*v1aq))}else{v1})}))}else{v1})})});
        let v1cf=(sf[173]*((if sb[66]{v1}else{(if (sf[221]!=0.0){((v130*v15e)+(v132*v17o))}else{v1})})+(sf[162]*vbb)));let v1cg=(sf[173]*((if sb[70]{v1}else{(if (sf[240]!=0.0){((v188*v1a7)+(v18a*v1c4))}else{v1})})+(sf[164]*vbd)));let v1ch=(sf[149]*vb2);let v1d5=(if (vbh!=0.0){vbk}else{v0});let v1d9=(if vbq{(-(vbt*(-v1d5)))}else{v1d5});let v1db=((v14*v1d9)/v16);let v1dc=(v1d9/sf[8]);let v1ec=(vby*vby);let v1ee=(v1dc/vbz);let v1fl=(v1m*(((vbz*v1db)-(vby*v1dc))/(vbz*vbz)));
        let v1ga=((vf2*v1ee)+(vde*(vf1*v1db)));let v1gc=(sf[179]*v1dc);let v1ge=(if (sf[187]!=0.0){((((vez*v1dc)+(vbz*(if (sf[187]!=0.0){((vex*v1fl)+(vel*(((veq*(((vby*(sf[189]*v1dc))-(veo*v1db))/v1ec))-(vev*(((vby*(sf[190]*v1dc))-(vet*v1db))/v1ec)))/vew)))}else{v1})))-v1ga)-v1gc)}else{v1});let v1gf=(v1m*v1db);let v1gv=(if (sf[187]!=0.0){(v1ge+((vfi*v1gf)+(vf9*((v1v*((v8h*(vfc*(((vby*(-v1ge))-(vfa*v1db))/v1ec)))/(v1m*vff)))/vfh))))}else{v1});let v1h6=(if sb[44]{v1}else{v1gv});
        let v1hv=(if (sf[192]!=0.0){((((vg8*v1dc)+(vbz*(if (sf[192]!=0.0){((vg6*v1fl)+(vel*(((vg0*(((vby*(sf[194]*v1dc))-(vfy*v1db))/v1ec))-(vg4*(((vby*(sf[195]*v1dc))-(vg2*v1db))/v1ec)))/vg5)))}else{v1})))-v1ga)-v1gc)}else{v1});let v1ib=(if (sf[192]!=0.0){(v1hv+((vgl*v1gf)+(vf9*((v1v*((v8h*(vgf*(((vby*(-v1hv))-(vgd*v1db))/v1ec)))/(v1m*vgi)))/vgk))))}else{v1});let v1im=(if sb[46]{v1}else{v1ib});
        let v1in=(if sb[46]{v1}else{(if (sf[192]!=0.0){(sf[167]*(((-(sf[193]*v1ib))/(vgo*vgo))*(sf[196]*f64::powf(vgp,sf[244]))))}else{v1})});let v31s=(sf[173]*v12p);let v31u=(sf[172]*v12p);let v31w=(v1m*v12t);let v325=(if sb[61]{sf[173]}else{(if sb[60]{(v1v*(sf[173]+((v31s+v31s)/v31w)))}else{v1})});let v326=(if sb[61]{sf[172]}else{(if sb[60]{(v1v*(sf[172]+((v31u+v31u)/v31w)))}else{v1})});
        let v327=(sf[20]*(if sb[44]{v1}else{(if (sf[187]!=0.0){(sf[165]*(((-(sf[188]*v1gv))/(vfl*vfl))*(sf[191]*f64::powf(vfm,sf[243]))))}else{v1})}));let v32b=(-v1h6);let v32c=(sf[222]*v32b);let v32d=(if v135{v32c}else{v1});let v32e=(if v13d{v325}else{v1});let v32f=(if v13d{v32d}else{v1});let v32g=(if v13d{v326}else{v1});let v32n=(sf[225]*v1h6);let v32s=(v13v*v13v);let v33d=(vfs*vfs);let v33l=(sf[228]*f64::powf(v144,sf[248]));let v340=(if v142{((vfs*(-((-(v325/vfs))*v33l)))/sf[228])}else{v1});
        let v341=(if v142{(((v146*v1h6)+(vfs*(-((-((-(v12y*v1h6))/v33d))*v33l))))/sf[228])}else{(if v13i{((v13o*v1h6)/sf[228])}else{v1})});let v342=(if v142{((vfs*(-((-(v326/vfs))*v33l)))/sf[228])}else{v1});let v34c=(v139*v32d);let v34k=(if v14e{v325}else{v1});let v34l=(if v14e{v32d}else{v1});let v34m=(if v14e{v326}else{v1});let v34n=(v14o*v34k);let v34p=(v14o*v34l);let v34r=(v14o*v34m);let v34t=(v1m*v14r);let v357=(if v14e{(v1v*(v34k-(if v14e{((v34n+v34n)/v34t)}else{v1})))}else{v1});
        let v358=(if v14e{((v1v*(v34l-(if v14e{((v34p+v34p)/v34t)}else{v1})))-v32d)}else{v1});let v359=(if v14e{(v1v*(v34m-(if v14e{((v34r+v34r)/v34t)}else{v1})))}else{v1});let v35k=(sf[228]*f64::powf(v14y,sf[248]));let v35z=(v325-v357);let v361=(v326-v359);let v362=((if v14e{(ver*(v32d+(if v14e{((v34c+v34c)/(v1m*v14j))}else{v1})))}else{v1})+(-v358));let v36x=(-v1im);let v36y=(sf[222]*v36x);let v36z=(if v15h{v36y}else{v1});let v370=(if v15o{v325}else{v1});let v371=(if v15o{v36z}else{v1});
        let v372=(if v15o{v326}else{v1});let v379=(sf[225]*v1im);let v37e=(v165*v165);let v37z=(vgv*vgv);let v387=(sf[236]*f64::powf(v16e,sf[249]));let v38m=(if v16c{((vgv*(-((-(v325/vgv))*v387)))/sf[236])}else{v1});let v38n=(if v16c{(((v16g*v1im)+(vgv*(-((-((-(v12y*v1im))/v37z))*v387))))/sf[236])}else{(if v15t{((v15y*v1im)/sf[236])}else{v1})});let v38o=(if v16c{((vgv*(-((-(v326/vgv))*v387)))/sf[236])}else{v1});let v38y=(v15k*v36z);let v396=(if v16o{v325}else{v1});let v397=(if v16o{v36z}else{v1});
        let v398=(if v16o{v326}else{v1});let v399=(v16y*v396);let v39b=(v16y*v397);let v39d=(v16y*v398);let v39f=(v1m*v171);let v39t=(if v16o{(v1v*(v396-(if v16o{((v399+v399)/v39f)}else{v1})))}else{v1});let v39u=(if v16o{((v1v*(v397-(if v16o{((v39b+v39b)/v39f)}else{v1})))-v36z)}else{v1});let v39v=(if v16o{(v1v*(v398-(if v16o{((v39d+v39d)/v39f)}else{v1})))}else{v1});let v3a6=(sf[236]*f64::powf(v178,sf[249]));let v3al=(v325-v39t);let v3an=(v326-v39v);
        let v3ao=((if v16o{(ver*(v36z+(if v16o{((v38y+v38y)/(v1m*v16t))}else{v1})))}else{v1})+(-v39u));
        let v3bu=(((v15e*(if (sf[221]!=0.0){v327}else{v1}))+(v130*(if v15d{v1}else{(if v14e{((if v14e{(((v14z*v32b)+(v136*((-(((vfs*v358)-(v14w*v1h6))/v33d))*v35k)))/sf[228])}else{v341})+((v158*(sf[227]*v362))+(v155*(((v13v*(sf[229]*v362))-(v156*v32n))/v32s))))}else{(if v13d{(v341+(if v142{v1}else{(if v13i{(v13m*((v13x*v32f)+(v13f*(((v13v*(sf[229]*v32f))-(v13u*v32n))/v32s))))}else{v1})}))}else{v1})})})))+((v17o*(if (sf[221]!=0.0){(sf[22]*v1in)}else{v1}))+(v132*(if v17n{v1}else{(if v16o{((if v16o{(((v179*v36x)+(v15i*((-(((vgv*v39u)-(v176*v1im))/v37z))*v3a6)))/sf[236])}else{v38n})+((v17i*(sf[235]*v3ao))+(v17f*(((v165*(sf[237]*v3ao))-(v17g*v379))/v37e))))}else{(if v15o{(v38n+(if v16c{v1}else{(if v15t{(v15w*((v167*v371)+(v15q*(((v165*(sf[237]*v371))-(v164*v379))/v37e))))}else{v1})}))}else{v1})})}))));
        let v3c2=(sf[173]*v17z);let v3c4=(sf[172]*v17z);let v3c6=(v1m*v182);let v3cg=(if sb[69]{sf[173]}else{(if sb[68]{(v1v*(sf[173]+((v3c2+v3c2)/v3c6)))}else{v325})});let v3ch=(if sb[69]{v1}else{(if sb[68]{v1}else{v326})});let v3ci=(if sb[69]{sf[172]}else{(if sb[68]{(v1v*(sf[172]+((v3c4+v3c4)/v3c6)))}else{v1})});let v3cm=(if v18d{v32c}else{v1});let v3cn=(if v18f{v3cg}else{v1});let v3co=(if v18f{v3cm}else{v1});let v3cp=(if v18f{v3ch}else{v1});let v3cq=(if v18f{v3ci}else{v1});
        let v3e0=(sf[228]*f64::powf(v190,sf[248]));let v3ej=(if v18y{((vfs*(-((-(v3cg/vfs))*v3e0)))/sf[228])}else{v1});let v3ek=(if v18y{(((v192*v1h6)+(vfs*(-((-((-(v187*v1h6))/v33d))*v3e0))))/sf[228])}else{(if v18k{((v18n*v1h6)/sf[228])}else{v1})});let v3el=(if v18y{((vfs*(-((-(v3ch/vfs))*v3e0)))/sf[228])}else{v1});let v3em=(if v18y{((vfs*(-((-(v3ci/vfs))*v3e0)))/sf[228])}else{v1});let v3ez=(v18e*v3cm);let v3f7=(if v199{v3cg}else{v1});let v3f8=(if v199{v3cm}else{v1});let v3f9=(if v199{v3ch}else{v1});
        let v3fa=(if v199{v3ci}else{v1});let v3fb=(v19h*v3f7);let v3fd=(v19h*v3f8);let v3ff=(v19h*v3f9);let v3fh=(v19h*v3fa);let v3fj=(v1m*v19k);let v3g1=(if v199{(v1v*(v3f7-(if v199{((v3fb+v3fb)/v3fj)}else{v1})))}else{v1});let v3g2=(if v199{((v1v*(v3f8-(if v199{((v3fd+v3fd)/v3fj)}else{v1})))-v3cm)}else{v1});let v3g3=(if v199{(v1v*(v3f9-(if v199{((v3ff+v3ff)/v3fj)}else{v1})))}else{v1});let v3g4=(if v199{(v1v*(v3fa-(if v199{((v3fh+v3fh)/v3fj)}else{v1})))}else{v1});let v3gh=(sf[228]*f64::powf(v19r,sf[248]));
        let v3h0=(v3cg-v3g1);let v3h2=(v3ch-v3g3);let v3h3=(v3ci-v3g4);let v3h4=((if v199{(ver*(v3cm+(if v199{((v3ez+v3ez)/(v1m*v19c))}else{v1})))}else{v1})+(-v3g2));let v3i8=(if v1aa{v36y}else{v1});let v3i9=(if v1ac{v3cg}else{v1});let v3ia=(if v1ac{v3i8}else{v1});let v3ib=(if v1ac{v3ch}else{v1});let v3ic=(if v1ac{v3ci}else{v1});let v3jm=(sf[236]*f64::powf(v1ax,sf[249]));let v3k5=(if v1av{((vgv*(-((-(v3cg/vgv))*v3jm)))/sf[236])}else{v1});
        let v3k6=(if v1av{(((v1az*v1im)+(vgv*(-((-((-(v187*v1im))/v37z))*v3jm))))/sf[236])}else{(if v1ah{((v1ak*v1im)/sf[236])}else{v1})});let v3k7=(if v1av{((vgv*(-((-(v3ch/vgv))*v3jm)))/sf[236])}else{v1});let v3k8=(if v1av{((vgv*(-((-(v3ci/vgv))*v3jm)))/sf[236])}else{v1});let v3kl=(v1ab*v3i8);let v3kt=(if v1b6{v3cg}else{v1});let v3ku=(if v1b6{v3i8}else{v1});let v3kv=(if v1b6{v3ch}else{v1});let v3kw=(if v1b6{v3ci}else{v1});let v3kx=(v1be*v3kt);let v3kz=(v1be*v3ku);let v3l1=(v1be*v3kv);let v3l3=(v1be*v3kw);
        let v3l5=(v1m*v1bh);let v3ln=(if v1b6{(v1v*(v3kt-(if v1b6{((v3kx+v3kx)/v3l5)}else{v1})))}else{v1});let v3lo=(if v1b6{((v1v*(v3ku-(if v1b6{((v3kz+v3kz)/v3l5)}else{v1})))-v3i8)}else{v1});let v3lp=(if v1b6{(v1v*(v3kv-(if v1b6{((v3l1+v3l1)/v3l5)}else{v1})))}else{v1});let v3lq=(if v1b6{(v1v*(v3kw-(if v1b6{((v3l3+v3l3)/v3l5)}else{v1})))}else{v1});let v3m3=(sf[236]*f64::powf(v1bo,sf[249]));let v3mm=(v3cg-v3ln);let v3mo=(v3ch-v3lp);let v3mp=(v3ci-v3lq);
        let v3mq=((if v1b6{(ver*(v3i8+(if v1b6{((v3kl+v3kl)/(v1m*v1b9))}else{v1})))}else{v1})+(-v3lo));
        let v3o7=(((v1a7*(if (sf[240]!=0.0){v327}else{v1}))+(v188*(if v1a6{v1}else{(if v199{((if v199{(((v19s*v32b)+(v136*((-(((vfs*v3g2)-(v19p*v1h6))/v33d))*v3gh)))/sf[228])}else{v3ek})+((v1a1*(sf[227]*v3h4))+(v19y*(((v13v*(sf[229]*v3h4))-(v19z*v32n))/v32s))))}else{(if v18f{(v3ek+(if v18y{v1}else{(if v18k{(v18l*((v18t*v3co)+(v18h*(((v13v*(sf[229]*v3co))-(v18r*v32n))/v32s))))}else{v1})}))}else{v1})})})))+((v1c4*(if (sf[240]!=0.0){(sf[24]*v1in)}else{v1}))+(v18a*(if v1c3{v1}else{(if v1b6{((if v1b6{(((v1bp*v36x)+(v15i*((-(((vgv*v3lo)-(v1bm*v1im))/v37z))*v3m3)))/sf[236])}else{v3k6})+((v1by*(sf[235]*v3mq))+(v1bv*(((v165*(sf[237]*v3mq))-(v1bw*v379))/v37e))))}else{(if v1ac{(v3k6+(if v1av{v1}else{(if v1ah{(v1ai*((v1aq*v3ia)+(v1ae*(((v165*(sf[237]*v3ia))-(v1ao*v379))/v37e))))}else{v1})}))}else{v1})})}))));
        let v3oq=(sf[173]*((if sb[66]{v1}else{(if (sf[221]!=0.0){((v130*(if v15d{v1}else{(if v14e{((if v14e{((v136*((-(v357/vfs))*v35k))/sf[228])}else{v340})+((v158*(sf[227]*v35z))+(v155*((sf[229]*v35z)/v13v))))}else{(if v13d{(v340+(if v142{v1}else{(if v13i{(v13m*((v13x*v32e)+(v13f*((sf[229]*v32e)/v13v))))}else{v1})}))}else{v1})})}))+(v132*(if v17n{v1}else{(if v16o{((if v16o{((v15i*((-(v39t/vgv))*v3a6))/sf[236])}else{v38m})+((v17i*(sf[235]*v3al))+(v17f*((sf[237]*v3al)/v165))))}else{(if v15o{(v38m+(if v16c{v1}else{(if v15t{(v15w*((v167*v370)+(v15q*((sf[237]*v370)/v165))))}else{v1})}))}else{v1})})})))}else{v1})})+sf[250]));
        let v3or=(sf[173]*(if sb[66]{v1}else{(if (sf[221]!=0.0){v3bu}else{v1})}));
        let v3os=(sf[173]*((if sb[66]{v1}else{(if (sf[221]!=0.0){((v130*(if v15d{v1}else{(if v14e{((if v14e{((v136*((-(v359/vfs))*v35k))/sf[228])}else{v342})+((v158*(sf[227]*v361))+(v155*((sf[229]*v361)/v13v))))}else{(if v13d{(v342+(if v142{v1}else{(if v13i{(v13m*((v13x*v32g)+(v13f*((sf[229]*v32g)/v13v))))}else{v1})}))}else{v1})})}))+(v132*(if v17n{v1}else{(if v16o{((if v16o{((v15i*((-(v39v/vgv))*v3a6))/sf[236])}else{v38o})+((v17i*(sf[235]*v3an))+(v17f*((sf[237]*v3an)/v165))))}else{(if v15o{(v38o+(if v16c{v1}else{(if v15t{(v15w*((v167*v372)+(v15q*((sf[237]*v372)/v165))))}else{v1})}))}else{v1})})})))}else{v1})})+sf[251]));
        let v3ot=(sf[173]*((if sb[70]{v1}else{(if (sf[240]!=0.0){((v188*(if v1a6{v1}else{(if v199{((if v199{((v136*((-(v3g1/vfs))*v3gh))/sf[228])}else{v3ej})+((v1a1*(sf[227]*v3h0))+(v19y*((sf[229]*v3h0)/v13v))))}else{(if v18f{(v3ej+(if v18y{v1}else{(if v18k{(v18l*((v18t*v3cn)+(v18h*((sf[229]*v3cn)/v13v))))}else{v1})}))}else{v1})})}))+(v18a*(if v1c3{v1}else{(if v1b6{((if v1b6{((v15i*((-(v3ln/vgv))*v3m3))/sf[236])}else{v3k5})+((v1by*(sf[235]*v3mm))+(v1bv*((sf[237]*v3mm)/v165))))}else{(if v1ac{(v3k5+(if v1av{v1}else{(if v1ah{(v1ai*((v1aq*v3i9)+(v1ae*((sf[237]*v3i9)/v165))))}else{v1})}))}else{v1})})})))}else{v1})})+sf[252]));
        let v3ou=(sf[173]*(if sb[70]{v1}else{(if (sf[240]!=0.0){v3o7}else{v1})}));
        let v3ov=(sf[173]*(if sb[70]{v1}else{(if (sf[240]!=0.0){((v188*(if v1a6{v1}else{(if v199{((if v199{((v136*((-(v3g3/vfs))*v3gh))/sf[228])}else{v3el})+((v1a1*(sf[227]*v3h2))+(v19y*((sf[229]*v3h2)/v13v))))}else{(if v18f{(v3el+(if v18y{v1}else{(if v18k{(v18l*((v18t*v3cp)+(v18h*((sf[229]*v3cp)/v13v))))}else{v1})}))}else{v1})})}))+(v18a*(if v1c3{v1}else{(if v1b6{((if v1b6{((v15i*((-(v3lp/vgv))*v3m3))/sf[236])}else{v3k7})+((v1by*(sf[235]*v3mo))+(v1bv*((sf[237]*v3mo)/v165))))}else{(if v1ac{(v3k7+(if v1av{v1}else{(if v1ah{(v1ai*((v1aq*v3ib)+(v1ae*((sf[237]*v3ib)/v165))))}else{v1})}))}else{v1})})})))}else{v1})}));
        let v3ow=(sf[173]*((if sb[70]{v1}else{(if (sf[240]!=0.0){((v188*(if v1a6{v1}else{(if v199{((if v199{((v136*((-(v3g4/vfs))*v3gh))/sf[228])}else{v3em})+((v1a1*(sf[227]*v3h3))+(v19y*((sf[229]*v3h3)/v13v))))}else{(if v18f{(v3em+(if v18y{v1}else{(if v18k{(v18l*((v18t*v3cq)+(v18h*((sf[229]*v3cq)/v13v))))}else{v1})}))}else{v1})})}))+(v18a*(if v1c3{v1}else{(if v1b6{((if v1b6{((v15i*((-(v3lq/vgv))*v3m3))/sf[236])}else{v3k8})+((v1by*(sf[235]*v3mp))+(v1bv*((sf[237]*v3mp)/v165))))}else{(if v1ac{(v3k8+(if v1av{v1}else{(if v1ah{(v1ai*((v1aq*v3ic)+(v1ae*((sf[237]*v3ic)/v165))))}else{v1})}))}else{v1})})})))}else{v1})})+sf[253]));

        CommonStampValues {
            v0, v1, v2, v1m, v1v, v2z, v49, v4i, 
            v4p, v69, v6b, v6c, v6d, v6x, v72, v74, 
            v75, v8h, vb2, vb5, vb6, vbb, vbd, vbw, 
            vby, vbz, vde, ver, vf1, v1cf, v1cg, v1ch, 
            v1d9, v1db, v1dc, v1ec, v1ee, v3oq, v3or, v3os, 
            v3ot, v3ou, v3ov, v3ow, 
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
        let common=self.eval_common_stamp_values(ctx);
        let v3=0.01;let v9=1000000.0;let v3n=((sf[60]*common.v2z)).sqrt();let v4y=(if sb[21]{(sf[81]/v3n)}else{common.v4i});let v5b=(common.v4p+sf[90]);let v7b=(sf[108]/common.v72);let v7s=(if sb[30]{common.v75}else{(if sb[28]{(common.v75-(v7b).sqrt())}else{(if (sf[106]!=0.0){(common.v75-v7b)}else{common.v1})})});let v7y=(sf[110]/(common.v0+(sf[111]/common.v4p)));let v8j=(v7y*(v7y*common.v8h));
        let v8m=(if sb[30]{v8j}else{(if sb[28]{v8j}else{(if (sf[106]!=0.0){(if sb[31]{(sf[279]*(common.v0+(((-v7y)/sf[270])).exp()))}else{sf[280]})}else{common.v1})})});let v8s=(common.v0-(common.v6c*common.v6x));let v8t=((((if sb[23]{common.v0}else{(if sb[21]{((sf[88]*((sf[89]+(v4y*v4y))).sqrt())).exp()}else{(if (sf[62]!=0.0){((v3*(sf[79]+(sf[82]/v3n)))).exp()}else{common.v1})})})*sf[114])*(common.v4p/common.v49))*v8s);
        let van=((sf[154]+(sf[155]/common.v49))+((sf[32]*(sf[156]+(sf[157]/common.v49)))/common.v4p));let vb8=(sf[173]*(common.vb5-common.vb6));let vc0=(common.vbw-sf[8]);let vc2=(((sf[150]+(sf[151]/common.v49))+((sf[32]*(sf[152]+(sf[153]/common.v49)))/common.v4p))+(van*vc0));let vc4=(common.v0+(vc0*vc2));let vc5=0.11;let vc7=(if (vc4<vc5){common.v0}else{common.v1});let vc8=10.0;let vcc=(((vc8*(vc4-v3))-common.v0)).exp();let vcf=(if (vc7!=0.0){(v3+(common.v69*vcc))}else{vc4});let vcg=(v8s*v8t);
        let vch=(vcf*vcg);let vcl=(v8t*vcf);let vcn=(if sb[38]{(common.v0/vcl)}else{(if (sf[105]!=0.0){(common.v0/vch)}else{common.v1})});let vcr=(sf[174]+(vc0*sf[175]));let vct=(common.v0+(vc0*vcr));let vcv=(if (vct<vc5){common.v0}else{common.v1});let vcz=(((vc8*(vct-v3))-common.v0)).exp();let vd2=(if (vcv!=0.0){(v3+(common.v69*vcz))}else{vct});let vd4=f64::powf(common.vbz,sf[176]);let vdb=(sf[180]*(common.v0-common.vbz));let vdg=((vdb/common.vby)+(sf[181]*common.vde));let vdj=((vdg/sf[182])).exp();
        let vdl=(if (sf[178]!=0.0){(sf[177]*vdj)}else{common.v1});let vdm=(common.vby*sf[182]);let vdp=(common.v0+(sf[183]/vdl));let vdq=(vdp).ln();let vdv=(if sb[40]{common.v1}else{(if (sf[178]!=0.0){(vdm*vdq)}else{common.v1})});let ve1=((vdg/sf[186])).exp();let ve3=(if (sf[185]!=0.0){(sf[184]*ve1)}else{common.v1});let ve4=(common.vby*sf[186]);let ve6=(common.v0+(sf[183]/ve3));let ve7=(ve6).ln();let veb=(if sb[42]{common.v1}else{ve3});
        let vec=(if sb[42]{common.v1}else{(if (sf[185]!=0.0){(ve4*ve7)}else{common.v1})});let ved=(sf[20]*(if sb[40]{common.v1}else{vdl}));let vee=(sf[22]*veb);let veg=(sf[24]*veb);let vh3=(sf[199]+(vc0*sf[200]));let vh7=(if (sf[198]!=0.0){(sf[197]*(common.v0+(vc0*vh3)))}else{common.v1});let vh8=(vh7>common.v1);let vha=(if (sf[198]!=0.0){(if vh8{vh7}else{common.v1})}else{vh7});let vhg=(if (sf[198]!=0.0){(sf[201]*(common.v0+(vc0*sf[202])))}else{common.v1});let vhh=(common.vby*vhg);let vhi=(-vha);
        let vhk=((vhi/vhh)).exp();let vhn=(vhk+sf[204]);let vho=(vhn).ln();let vhs=(if sb[48]{sf[197]}else{vha});let vht=(if sb[48]{sf[201]}else{vhg});let vhu=(if sb[48]{common.v0}else{(if (sf[198]!=0.0){(vhh*vho)}else{common.v1})});let vi2=(vd4*sf[208]);let vi5=(vd4*sf[205]);let via=(if sb[53]{sf[208]}else{(if sb[51]{(vcf*vi2)}else{common.v1})});let vib=(if sb[53]{sf[205]}else{(if sb[51]{(vcf*vi5)}else{common.v1})});let vig=(vib*sf[211]);let vij=(((via*via)+(vib*vig))).sqrt();
        let vin=(if (sf[206]!=0.0){(vij-(vib*sf[212]))}else{common.v1});let vio=(sf[209]*vin);let viq=(if (sf[206]!=0.0){(vio/vib)}else{common.v1});let vir=(vin*vin);let vis=(vib*vib);let viw=(((vir/vis)+(common.v8h*viq))).sqrt();let vj3=(if sb[54]{common.v1}else{vin});let vj4=(if sb[54]{common.v1}else{viq});let vj5=(if sb[54]{common.v1}else{(if (sf[206]!=0.0){viw}else{common.v1})});let vj8=(if sb[54]{common.v1}else{(if (sf[206]!=0.0){(common.v0/vib)}else{common.v1})});
        let vj9=(v5b*(if sb[54]{1000.0}else{(if (sf[206]!=0.0){(vib-via)}else{common.v1})}));let vja=100000.0;let vjc=(if (vj9>vja){common.v0}else{common.v1});let vjd=(if (vjc!=0.0){vja}else{vj9});let vjf=(if (vb8<common.v1){common.v0}else{common.v1});let vjg=-1.0;let vjm=(!(vjf!=0.0));let vjn=(if vjm{common.v0}else{(if (vjf!=0.0){vjg}else{common.v1})});let vjp=(if vjm{(-common.vbb)}else{(if (vjf!=0.0){(-common.vbd)}else{common.v1})});let vjq=(if vjm{vb8}else{(if (vjf!=0.0){(-vb8)}else{common.v1})});
        let vjs=(if (vjp>v7s){common.v0}else{common.v1});let vjv=(((v7s-vjp)/sf[283])).exp();let vjw=(common.v0+vjv);let vk1=(!(vjs!=0.0));let vk4=(((vjp-v7s)/sf[283])).exp();let vk5=(common.v0+vk4);let vk9=(if vk1{(vjp-(sf[283]*(vk5).ln()))}else{(if (vjs!=0.0){(v7s-(sf[283]*(vjw).ln()))}else{common.v1})});let vka=-0.4;let vkb=(v7s-vk9);let vkc=(vjq<vkb);let vkf=(vka*(common.v6b+(if vkc{vjq}else{vkb})));let vkh=(if (vk9<vkf){common.v0}else{common.v1});let vki=((sf[105]!=0.0)&&(vkh!=0.0));
        let vkl=((sf[105]!=0.0)&&(!(vkh!=0.0)));let vkn=(common.v6b*vka);let vkp=(if (vk9<vkn){common.v0}else{common.v1});let vkq=(sb[38]&&(vkp!=0.0));let vkt=(sb[38]&&(!(vkp!=0.0)));let vku=(if vkt{vk9}else{(if vkq{vkn}else{(if vkl{vk9}else{(if vki{vkf}else{common.v1})})})});let vkw=(common.v6b+(common.v1m*vku));let vky=(if (vj8>common.v1){common.v0}else{common.v1});let vkz=(common.v72*vkw);let vl2=(if (vky!=0.0){((vkw*vkz)-vkw)}else{common.v1});let vl3=(common.v72*common.vf1);let vl4=(vkw*vl3);
        let vl6=(if (vky!=0.0){(vjg+vl4)}else{common.v1});let vl7=9.0;let vlc=(if (vky!=0.0){(common.v72*(2.25+(vkw/vjd)))}else{common.v1});let vld=1.5;let vle=(common.v72*vld);let vlg=(if (vky!=0.0){(vle/vjd)}else{common.v1});let vlh=(common.v8h*vjd);let vlk=(if (vky!=0.0){((vjd*vlh)/common.v72)}else{common.v1});let vlm=(if (vky!=0.0){(vl2*vlk)}else{common.v1});let vlo=(if (vky!=0.0){(vl6*vlk)}else{common.v1});let vlq=(if (vky!=0.0){(vlc*vlk)}else{common.v1});
        let vls=(if (vky!=0.0){(vlg*vlk)}else{common.v1});let vlu=(if (vky!=0.0){(vls*vls)}else{common.v1});let vlw=(if (vky!=0.0){(-vlq)}else{common.v1});let vm0=(if (vky!=0.0){((vlo*vls)-(common.v8h*vlm))}else{common.v1});let vm1=(common.v8h*vlq);let vm7=(if (vky!=0.0){(((vlm*vm1)-(vlo*vlo))-(vlm*vlu))}else{common.v1});let vm9=0.3333333333333333;let vmc=(if (vky!=0.0){(vm0-((vlw*vlw)*vm9))}else{common.v1});let vme=(vm0+(common.v1m*vmc));let vmi=(if (vky!=0.0){(vm7-((vlw*vme)/vl7))}else{common.v1});
        let vmj=(vmc*vmc);let vml=27.0;let vmn=(if (vky!=0.0){((vmc*vmj)/vml)}else{common.v1});let vmo=0.25;let vmp=(vmi*vmo);let vmt=((if (vky!=0.0){(vmn+(vmi*vmp))}else{common.v1})).sqrt();let vmu=(if (vky!=0.0){vmt}else{common.v1});let vmw=(if (vmi<common.v1){common.v0}else{common.v1});let vmx=((vky!=0.0)&&(vmw!=0.0));let vmy=(common.ver*vmi);let vn0=(if vmx{(vmu+vmy)}else{common.v1});let vn1=(-vmn);let vn5=((vky!=0.0)&&(!(vmw!=0.0)));let vn7=(if vn5{(vmy-vmu)}else{(if vmx{(vn1/vn0)}else{common.v1})});
        let vn9=(if vn5{(vn1/vn7)}else{vn0});let vna=1e-6;let vnc=(if (vn9>vna){common.v0}else{common.v1});let vnd=((vky!=0.0)&&(vnc!=0.0));let vng=-1e-6;let vni=(if (vn9<vng){common.v0}else{common.v1});let vnk=((vky!=0.0)&&(!(vnc!=0.0)));let vnl=((vni!=0.0)&&vnk);let vnm=(-vn9);let vnr=(vnk&&(!(vni!=0.0)));let vnv=(if (vn7>vna){common.v0}else{common.v1});let vnw=((vky!=0.0)&&(vnv!=0.0));let vo0=(if (vn7<vng){common.v0}else{common.v1});let vo2=((vky!=0.0)&&(!(vnv!=0.0)));let vo3=((vo0!=0.0)&&vo2);
        let vo4=(-vn7);let vo9=(vo2&&(!(vo0!=0.0)));let vog=(vlu*vmo);let voj=(((if (vky!=0.0){(((if vnr{(common.v6d*vn9)}else{(if vnl{(-f64::powf(vnm,vm9))}else{(if vnd{f64::powf(vn9,vm9)}else{common.v1})})})+(if vo9{(common.v6d*vn7)}else{(if vo3{(-f64::powf(vo4,vm9))}else{(if vnw{f64::powf(vn7,vm9)}else{common.v1})})}))-(vlw*vm9))}else{common.v1})+(vog-vlq))).sqrt();let vok=(if (vky!=0.0){voj}else{vm7});let vol=0.75;let von=(vok*vok);
        let vor=(if (vky!=0.0){(((vlu*vol)-von)-(common.v1m*vlq))}else{common.v1});let vow=(((vlq*vls)-(common.v1m*vlo))-(vls*vog));let voy=(if (vky!=0.0){(vow/vok)}else{common.v1});let vp0=(if (vky!=0.0){(vor+voy)}else{common.v1});let vp2=(if (vp0>common.v1){common.v0}else{common.v1});let vp3=((vky!=0.0)&&(vp2!=0.0));let vp4=(vp0).sqrt();let vp5=(if vp3{vp4}else{common.v1});let vp6=-0.25;let vp7=(vls*vp6);let vpd=((vky!=0.0)&&(!(vp2!=0.0)));let vpf=(if vpd{(vor-voy)}else{common.v1});
        let vpj=(((vpf*vpf)+0.0001)).sqrt();let vpk=(vpj).sqrt();let vpr=(if (vku>(if sb[30]{common.v1}else{(if sb[28]{common.v1}else{(if (sf[106]!=0.0){((0.1666666666666667/common.v72)-common.v74)}else{common.v1})})})){common.v0}else{common.v1});let vps=(!(vky!=0.0));let vpt=((vpr!=0.0)&&vps);let vpu=(common.v75-vku);let vpw=(if vpt{(common.v72*vpu)}else{common.v1});let vpz=(common.v1m*(common.v0-(common.v1m*vpw)));let vq0=(vpu*vpz);let vq5=((common.v0-(vld*vpw))).sqrt();
        let vq6=((common.v0-(common.vf1*vpw))+vq5);let vqa=(vps&&(!(vpr!=0.0)));let vqb=(if vqa{vl4}else{vpw});let vqe=((common.v0+vqb)).sqrt();let vqh=(common.v72*4.5);let vqj=(if vqa{(((common.v0-vqb)+vqe)/vqh)}else{(if vpt{(vq0/vq6)}else{(if vpd{(vp7+(common.v1v*((if vpd{vpk}else{vp5})-vok)))}else{(if vp3{(vp7+(common.v1v*(vok+vp5)))}else{common.v1})})})});let vqn=(if (sb[25]&&(common.v6x>1e-9)){common.v0}else{common.v1});let vqp=(if (vqn!=0.0){(sf[270]+vqj)}else{common.v1});let vqq=(vkw+vqj);
        let vqr=(vqq).sqrt();let vqt=(if (vqn!=0.0){(common.v6x*vqr)}else{common.v1});let vqu=((vky!=0.0)&&(vqn!=0.0));let vqv=(vqp/v5b);let vqx=(common.v1v*(vqv-vj3));let vqz=(if vqu{(vj8*vqx)}else{common.v1});let vr1=(common.v1v*(vj3+vqv));let vr3=(if vqu{(vj8*vr1)}else{common.v1});let vr6=((vj4+(vqz*vqz))).sqrt();let vr7=(if vqu{vr6}else{common.v1});let vra=((vj4+(vr3*vr3))).sqrt();let vrb=(if vqu{vra}else{common.v1});let vre=(if vqu{((vr7+vrb)-vj5)}else{common.v1});
        let vri=(common.v1v*((vqz/vr7)+(vr3/vrb)));let vrl=(if vqu{((vj8*vri)/v5b)}else{common.v1});let vrm=(common.v1m*vqt);let vrn=(common.v0-vqt);let vro=(vrm*vrn);let vrp=(vqp*vrl);let vrq=(common.v0+vre);let vrs=(common.v0-(vrp/vrq));let vrt=(vro*vrs);let vrv=((vrt/vqp)).sqrt();let vrx=(vps&&(vqn!=0.0));let vrz=((vro/vqp)).sqrt();let vs0=(if vrx{vrz}else{(if vqu{vrv}else{common.v1})});let vs1=(common.v72*vqq);let vs2=(vs0*vs0);let vs6=(sf[110]*vqj);let vs7=(sf[110]+vqp);
        let vsa=(if (vqn!=0.0){(v8m+(vs6/vs7))}else{common.v1});let vsb=(common.v8h*vsa);let vsd=(if (vqn!=0.0){(vsa*vsb)}else{common.v1});let vse=(common.v1m*vjq);let vsf=(vqp*vse);let vsg=(vjq-vqp);let vsh=(vsg*vsg);let vsj=((vsd+vsh)).sqrt();let vsk=(vjq+vqp);let vsl=(vsk*vsk);let vsn=((vsd+vsl)).sqrt();let vso=(vsj+vsn);let vsq=(if (vqn!=0.0){(vsf/vso)}else{common.v1});let vss=((vqn!=0.0)&&(sf[213]!=0.0));let vst=(sf[110]*vsq);let vsw=(if vss{(v8m+(vst/vs7))}else{vsa});let vsx=(common.v8h*vsw);
        let vsz=(if vss{(vsw*vsx)}else{vsd});let vt1=((vsh+vsz)).sqrt();let vt3=((vsl+vsz)).sqrt();let vt4=(vt1+vt3);let vt6=(if vss{(vsf/vt4)}else{vsq});let vt8=(((if (vqn!=0.0){((vs1/vs2)-vqp)}else{common.v1})+vt6)).sqrt();let vtc=(vt6/v5b);let vte=(common.v1v*(vtc-vj3));let vtg=(if vqu{(vj8*vte)}else{vqz});let vti=(common.v1v*(vj3+vtc));let vtk=(if vqu{(vj8*vti)}else{vr3});let vtn=((vj4+(vtg*vtg))).sqrt();let vto=(if vqu{vtn}else{vr7});let vtr=((vj4+(vtk*vtk))).sqrt();let vts=(if vqu{vtr}else{vrb});
        let vtx=(!(vqn!=0.0));let vty=(vqj*vse);let vtz=(vjq-vqj);let vu2=((v8m+(vtz*vtz))).sqrt();let vu3=(vjq+vqj);let vu6=((v8m+(vu3*vu3))).sqrt();let vu7=(vu2+vu6);let vu9=(if vtx{(vty/vu7)}else{vt6});let vua=((vky!=0.0)&&vtx);let vub=(vu9/v5b);let vud=(common.v1v*(vub-vj3));let vuf=(if vua{(vj8*vud)}else{vtg});let vuh=(common.v1v*(vj3+vub));let vuj=(if vua{(vj8*vuh)}else{vtk});let vum=((vj4+(vuf*vuf))).sqrt();let vuq=((vj4+(vuj*vuj))).sqrt();let vuv=(vps&&vtx);let vuy=((vkw+vu9)).sqrt();
        let vv1=(if vtx{(common.v0-(common.v6x*vuy))}else{(if (vqn!=0.0){(common.v0-(vs0*vt8))}else{common.v1})});let vv3=(if (vv1<sf[107]){common.v0}else{common.v1});let vv4=(if (vv3!=0.0){sf[107]}else{vv1});let vv5=(vcn*vv4);let vv6=(common.v0+(if vuv{common.v1}else{(if vua{(((if vua{vum}else{vto})+(if vua{vuq}else{vts}))-vj5)}else{(if vrx{common.v1}else{(if vqu{((vto+vts)-vj5)}else{vre})})})}));let vv8=(vjn*(vv5/vv6));let vv9=(vu9*vv8);let vvb=(if ((ved+vee)>common.v1){common.v0}else{common.v1});
        let vvc=(if (vvb!=0.0){ved}else{common.v1});let vvd=(if (vvb!=0.0){vee}else{common.v1});let vvf=(if (vvc>common.v1){common.v0}else{common.v1});let vvg=((vvb!=0.0)&&(vvf!=0.0));let vvh=(common.v0/vdm);let vvi=(if vvg{vvh}else{common.v1});let vvk=(if (common.vbb<vdv){common.v0}else{common.v1});let vvl=(vvg&&(vvk!=0.0));let vvn=((common.vbb*vvi)).exp();let vvq=(vvg&&(!(vvk!=0.0)));let vvs=((vdv*vvi)).exp();let vvt=(common.vbb-vdv);let vvv=(common.v0+(vvi*vvt));
        let vvx=(if vvq{(vvs*vvv)}else{(if vvl{vvn}else{common.v1})});let vvy=(vvx-common.v0);let vw2=((vvb!=0.0)&&(!(vvf!=0.0)));let vw5=(if (vvd>common.v1){common.v0}else{common.v1});let vw6=((vvb!=0.0)&&(vw5!=0.0));let vw7=(common.v0/ve4);let vw8=(if vw6{vw7}else{vvi});let vwa=(if (common.vbb<vec){common.v0}else{common.v1});let vwb=(vw6&&(vwa!=0.0));let vwd=((common.vbb*vw8)).exp();let vwg=(vw6&&(!(vwa!=0.0)));let vwi=((vec*vw8)).exp();let vwj=(common.vbb-vec);let vwl=(common.v0+(vw8*vwj));
        let vwo=((if vwg{(vwi*vwl)}else{(if vwb{vwd}else{vvx})})-common.v0);let vws=((vvb!=0.0)&&(!(vw5!=0.0)));let vwx=(if (vhs>common.v1){common.v0}else{common.v1});let vwy=((vvb!=0.0)&&(vwx!=0.0));let vwz=(-vhs);let vx1=(if vwy{(vwz-common.vbb)}else{common.v1});let vx2=(common.vby*vht);let vx3=(common.v0/vx2);let vx4=(if vwy{vx3}else{common.v1});let vx6=(if (vx1<vhu){common.v0}else{common.v1});let vx7=(vwy&&(vx6!=0.0));let vx9=((vx1*vx4)).exp();let vxc=(vwy&&(!(vx6!=0.0)));let vxe=((vhu*vx4)).exp();
        let vxf=(vx1-vhu);let vxh=(common.v0+(vx4*vxf));let vxm=((vwz*vx4)).exp();let vxq=(!(vwx!=0.0));let vxr=((vvb!=0.0)&&vxq);let vxx=(!(vvb!=0.0));
        let vxy=(if vxx{common.v1}else{(if (vvb!=0.0){(((if (vvb!=0.0){((if vw2{common.v1}else{(if vvg{(vvc*vvy)}else{common.v1})})+(if vws{common.v1}else{(if vw6{(vvd*vwo)}else{common.v1})}))}else{common.v1})+(if vxr{common.v1}else{(if vwy{(sf[214]*((if vxc{(vxe*vxh)}else{(if vx7{vx9}else{common.v1})})-vxm))}else{common.v1})}))+(common.v1*common.vbb))}else{common.v1})});let vy0=(if ((ved+veg)>common.v1){common.v0}else{common.v1});let vy1=(if (vy0!=0.0){ved}else{common.v1});
        let vy2=(if (vy0!=0.0){veg}else{common.v1});let vy4=(if (vy1>common.v1){common.v0}else{common.v1});let vy5=((vy0!=0.0)&&(vy4!=0.0));let vy6=(if vy5{vvh}else{common.v1});let vy8=(if (common.vbd<vdv){common.v0}else{common.v1});let vy9=(vy5&&(vy8!=0.0));let vyb=((common.vbd*vy6)).exp();let vye=(vy5&&(!(vy8!=0.0)));let vyg=((vdv*vy6)).exp();let vyh=(common.vbd-vdv);let vyj=(common.v0+(vy6*vyh));let vyl=(if vye{(vyg*vyj)}else{(if vy9{vyb}else{common.v1})});let vym=(vyl-common.v0);
        let vyq=((vy0!=0.0)&&(!(vy4!=0.0)));let vyt=(if (vy2>common.v1){common.v0}else{common.v1});let vyu=((vy0!=0.0)&&(vyt!=0.0));let vyv=(if vyu{vw7}else{vy6});let vyx=(if (common.vbd<vec){common.v0}else{common.v1});let vyy=(vyu&&(vyx!=0.0));let vz0=((common.vbd*vyv)).exp();let vz3=(vyu&&(!(vyx!=0.0)));let vz5=((vec*vyv)).exp();let vz6=(common.vbd-vec);let vz8=(common.v0+(vyv*vz6));let vzb=((if vz3{(vz5*vz8)}else{(if vyy{vz0}else{vyl})})-common.v0);let vzf=((vy0!=0.0)&&(!(vyt!=0.0)));
        let vzj=((vwx!=0.0)&&(vy0!=0.0));let vzl=(if vzj{(vwz-common.vbd)}else{common.v1});let vzm=(if vzj{vx3}else{common.v1});let vzo=(if (vzl<vhu){common.v0}else{common.v1});let vzp=(vzj&&(vzo!=0.0));let vzr=((vzl*vzm)).exp();let vzu=(vzj&&(!(vzo!=0.0)));let vzw=((vhu*vzm)).exp();let vzx=(vzl-vhu);let vzz=(common.v0+(vzm*vzx));let v103=((vwz*vzm)).exp();let v107=(vxq&&(vy0!=0.0));let v10d=(!(vy0!=0.0));
        let v10e=(if v10d{common.v1}else{(if (vy0!=0.0){(((if (vy0!=0.0){((if vyq{common.v1}else{(if vy5{(vy1*vym)}else{common.v1})})+(if vzf{common.v1}else{(if vyu{(vy2*vzb)}else{common.v1})}))}else{common.v1})+(if v107{common.v1}else{(if vzj{(sf[214]*((if vzu{(vzw*vzz)}else{(if vzp{vzr}else{common.v1})})-v103))}else{common.v1})}))+(common.v1*common.vbd))}else{common.v1})});let v10k=ctx.branch_current(branches[0]);let v10m=(ctx.node_voltage(nodes[0])-common.vb6);let v10p=ctx.branch_current(branches[1]);
        let v10r=(ctx.node_voltage(nodes[2])-common.vb5);let v114=(sf[286]*common.vb2);let v120=(common.v0+(common.vb2/sf[302]));let v12b=(common.v0+((common.vb2*sf[220])/sf[302]));let v12i=(sf[173]*vv9);let v12j=(sf[173]*vxy);let v12k=(sf[173]*v10e);let v1cl=(if ((sf[122]/common.v2)<=sf[241]){common.v0}else{common.v1});let v1co=(if ((sf[126]/common.v2)<=sf[241]){common.v0}else{common.v1});let v1cp=(sf[122]*vd2);let v1cr=(sf[126]*vd2);let v1ct=(sf[122]*v10k);let v1cw=(!(v1cl!=0.0));let v1cz=(sf[126]*v10p);
        let v1d2=(!(v1co!=0.0));let v1dg=((vc2*common.v1d9)+(vc0*(van*common.v1d9)));let v1dk=(if (vc7!=0.0){(common.v69*(vcc*(vc8*v1dg)))}else{v1dg});let v1dy=((vcr*common.v1d9)+(vc0*(sf[175]*common.v1d9)));let v1e2=(if (vcv!=0.0){(common.v69*(vcz*(vc8*v1dy)))}else{v1dy});let v1e6=(common.v1dc*(sf[176]*f64::powf(common.vbz,sf[242])));let v1eg=((((common.vby*(sf[180]*(-common.v1dc)))-(vdb*common.v1db))/common.v1ec)+(sf[181]*common.v1ee));
        let v1ek=(if (sf[178]!=0.0){(sf[177]*(vdj*(v1eg/sf[182])))}else{common.v1});let v1el=(sf[182]*common.v1db);let v1ew=(if sb[40]{common.v1}else{(if (sf[178]!=0.0){((vdq*v1el)+(vdm*(((-(sf[183]*v1ek))/(vdl*vdl))/vdp)))}else{common.v1})});let v1f0=(if (sf[185]!=0.0){(sf[184]*(ve1*(v1eg/sf[186])))}else{common.v1});let v1f1=(sf[186]*common.v1db);let v1fb=(if sb[42]{common.v1}else{v1f0});
        let v1fc=(if sb[42]{common.v1}else{(if (sf[185]!=0.0){((ve7*v1f1)+(ve4*(((-(sf[183]*v1f0))/(ve3*ve3))/ve6)))}else{common.v1})});let v1fd=(sf[20]*(if sb[40]{common.v1}else{v1ek}));let v1it=(if (sf[198]!=0.0){(sf[197]*((vh3*common.v1d9)+(vc0*(sf[200]*common.v1d9))))}else{common.v1});let v1iv=(if (sf[198]!=0.0){(if vh8{v1it}else{common.v1})}else{v1it});let v1iy=(if (sf[198]!=0.0){(sf[201]*(sf[202]*common.v1d9))}else{common.v1});let v1j1=((vhg*common.v1db)+(common.vby*v1iy));
        let v1jg=(if sb[48]{common.v1}else{(if (sf[198]!=0.0){((vho*v1j1)+(vhh*((vhk*(((vhh*(-v1iv))-(vhi*v1j1))/(vhh*vhh)))/vhn)))}else{common.v1})});let v1jr=(if sb[53]{common.v1}else{(if sb[51]{((vi2*v1dk)+(vcf*(sf[208]*v1e6)))}else{common.v1})});let v1js=(if sb[53]{common.v1}else{(if sb[51]{((vi5*v1dk)+(vcf*(sf[205]*v1e6)))}else{common.v1})});let v1jt=(via*v1jr);let v1k4=(if (sf[206]!=0.0){((((v1jt+v1jt)+((vig*v1js)+(vib*(sf[211]*v1js))))/(common.v1m*vij))-(sf[212]*v1js))}else{common.v1});
        let v1ka=(if (sf[206]!=0.0){(((vib*(sf[209]*v1k4))-(vio*v1js))/vis)}else{common.v1});let v1kb=(vin*v1k4);let v1kd=(vib*v1js);let v1ku=(if sb[54]{common.v1}else{v1k4});let v1kv=(if sb[54]{common.v1}else{v1ka});let v1kw=(if sb[54]{common.v1}else{(if (sf[206]!=0.0){(((((vis*(v1kb+v1kb))-(vir*(v1kd+v1kd)))/(vis*vis))+(common.v8h*v1ka))/(common.v1m*viw))}else{common.v1})});let v1ky=(if sb[54]{common.v1}else{(if (sf[206]!=0.0){((-v1js)/vis)}else{common.v1})});
        let v1l0=(if (vjc!=0.0){common.v1}else{(v5b*(if sb[54]{common.v1}else{(if (sf[206]!=0.0){(v1js-v1jr)}else{common.v1})}))});let v1l1=(if (vjf!=0.0){sf[172]}else{common.v1});let v1l2=(if (vjf!=0.0){sf[173]}else{common.v1});let v1l3=(if vjm{sf[172]}else{v1l1});let v1l4=(if vjm{sf[173]}else{common.v1});let v1l5=(if vjm{common.v1}else{v1l2});let v1l6=(if vjm{sf[172]}else{v1l2});let v1l7=(if vjm{sf[173]}else{v1l1});
        let v1m8=(if vk1{(v1l3-(sf[283]*((vk4*(v1l3/sf[283]))/vk5)))}else{(if (vjs!=0.0){(-(sf[283]*((vjv*((-v1l3)/sf[283]))/vjw)))}else{common.v1})});let v1m9=(if vk1{(v1l4-(sf[283]*((vk4*(v1l4/sf[283]))/vk5)))}else{(if (vjs!=0.0){(-(sf[283]*((vjv*((-v1l4)/sf[283]))/vjw)))}else{common.v1})});let v1ma=(if vk1{(v1l5-(sf[283]*((vk4*(v1l5/sf[283]))/vk5)))}else{(if (vjs!=0.0){(-(sf[283]*((vjv*((-v1l5)/sf[283]))/vjw)))}else{common.v1})});
        let v1mt=(if vkt{v1m8}else{(if vkq{common.v1}else{(if vkl{v1m8}else{(if vki{(vka*(if vkc{common.v1}else{(-v1m8)}))}else{common.v1})})})});let v1mu=(if vkt{v1m9}else{(if vkq{common.v1}else{(if vkl{v1m9}else{(if vki{(vka*(if vkc{v1l6}else{(-v1m9)}))}else{common.v1})})})});let v1mv=(if vkt{v1ma}else{(if vkq{common.v1}else{(if vkl{v1ma}else{(if vki{(vka*(if vkc{v1l7}else{(-v1ma)}))}else{common.v1})})})});let v1mw=(common.v1m*v1mt);let v1mx=(common.v1m*v1mu);let v1my=(common.v1m*v1mv);
        let v1nh=(vl3*v1mw);let v1ni=(vl3*v1mx);let v1nj=(vl3*v1my);let v1nq=(vjd*vjd);let v1ob=(if (vky!=0.0){(((vlh*v1l0)+(vjd*(common.v8h*v1l0)))/common.v72)}else{common.v1});let v1og=(if (vky!=0.0){(vlk*(if (vky!=0.0){(((vkz*v1mw)+(vkw*(common.v72*v1mw)))-v1mw)}else{common.v1}))}else{common.v1});let v1oh=(if (vky!=0.0){(vl2*v1ob)}else{common.v1});let v1oi=(if (vky!=0.0){(vlk*(if (vky!=0.0){(((vkz*v1mx)+(vkw*(common.v72*v1mx)))-v1mx)}else{common.v1}))}else{common.v1});
        let v1oj=(if (vky!=0.0){(vlk*(if (vky!=0.0){(((vkz*v1my)+(vkw*(common.v72*v1my)))-v1my)}else{common.v1}))}else{common.v1});let v1oo=(if (vky!=0.0){(vlk*(if (vky!=0.0){v1nh}else{common.v1}))}else{common.v1});let v1op=(if (vky!=0.0){(vl6*v1ob)}else{common.v1});let v1oq=(if (vky!=0.0){(vlk*(if (vky!=0.0){v1ni}else{common.v1}))}else{common.v1});let v1or=(if (vky!=0.0){(vlk*(if (vky!=0.0){v1nj}else{common.v1}))}else{common.v1});
        let v1oy=(if (vky!=0.0){(vlk*(if (vky!=0.0){(common.v72*(v1mw/vjd))}else{common.v1}))}else{common.v1});let v1oz=(if (vky!=0.0){((vlk*(if (vky!=0.0){(common.v72*((-(vkw*v1l0))/v1nq))}else{common.v1}))+(vlc*v1ob))}else{common.v1});let v1p0=(if (vky!=0.0){(vlk*(if (vky!=0.0){(common.v72*(v1mx/vjd))}else{common.v1}))}else{common.v1});let v1p1=(if (vky!=0.0){(vlk*(if (vky!=0.0){(common.v72*(v1my/vjd))}else{common.v1}))}else{common.v1});
        let v1p5=(if (vky!=0.0){((vlk*(if (vky!=0.0){((-(vle*v1l0))/v1nq)}else{common.v1}))+(vlg*v1ob))}else{common.v1});let v1p6=(vls*v1p5);let v1p8=(if (vky!=0.0){(v1p6+v1p6)}else{common.v1});let v1p9=(-v1oy);let v1pb=(-v1p0);let v1pc=(-v1p1);let v1pd=(if (vky!=0.0){v1p9}else{common.v1});let v1pe=(if (vky!=0.0){(-v1oz)}else{common.v1});let v1pf=(if (vky!=0.0){v1pb}else{common.v1});let v1pg=(if (vky!=0.0){v1pc}else{common.v1});let v1pv=(if (vky!=0.0){((vls*v1oo)-(common.v8h*v1og))}else{common.v1});
        let v1pw=(if (vky!=0.0){(((vls*v1op)+(vlo*v1p5))-(common.v8h*v1oh))}else{common.v1});let v1px=(if (vky!=0.0){((vls*v1oq)-(common.v8h*v1oi))}else{common.v1});let v1py=(if (vky!=0.0){((vls*v1or)-(common.v8h*v1oj))}else{common.v1});let v1qf=(vlo*v1oo);let v1qh=(vlo*v1op);let v1qj=(vlo*v1oq);let v1ql=(vlo*v1or);let v1r1=(if (vky!=0.0){((((vm1*v1og)+(vlm*(common.v8h*v1oy)))-(v1qf+v1qf))-(vlu*v1og))}else{common.v1});
        let v1r2=(if (vky!=0.0){((((vm1*v1oh)+(vlm*(common.v8h*v1oz)))-(v1qh+v1qh))-((vlu*v1oh)+(vlm*v1p8)))}else{common.v1});let v1r3=(if (vky!=0.0){((((vm1*v1oi)+(vlm*(common.v8h*v1p0)))-(v1qj+v1qj))-(vlu*v1oi))}else{common.v1});let v1r4=(if (vky!=0.0){((((vm1*v1oj)+(vlm*(common.v8h*v1p1)))-(v1ql+v1ql))-(vlu*v1oj))}else{common.v1});let v1r5=(vlw*v1pd);let v1r7=(vlw*v1pe);let v1r9=(vlw*v1pf);let v1rb=(vlw*v1pg);let v1rl=(if (vky!=0.0){(v1pv-(vm9*(v1r5+v1r5)))}else{common.v1});
        let v1rm=(if (vky!=0.0){(v1pw-(vm9*(v1r7+v1r7)))}else{common.v1});let v1rn=(if (vky!=0.0){(v1px-(vm9*(v1r9+v1r9)))}else{common.v1});let v1ro=(if (vky!=0.0){(v1py-(vm9*(v1rb+v1rb)))}else{common.v1});let v1sh=(if (vky!=0.0){(v1r1-(((vme*v1pd)+(vlw*(v1pv+(common.v1m*v1rl))))/vl7))}else{common.v1});let v1si=(if (vky!=0.0){(v1r2-(((vme*v1pe)+(vlw*(v1pw+(common.v1m*v1rm))))/vl7))}else{common.v1});let v1sj=(if (vky!=0.0){(v1r3-(((vme*v1pf)+(vlw*(v1px+(common.v1m*v1rn))))/vl7))}else{common.v1});
        let v1sk=(if (vky!=0.0){(v1r4-(((vme*v1pg)+(vlw*(v1py+(common.v1m*v1ro))))/vl7))}else{common.v1});let v1sl=(vmc*v1rl);let v1sn=(vmc*v1rm);let v1sp=(vmc*v1rn);let v1sr=(vmc*v1ro);let v1t9=(if (vky!=0.0){(((vmj*v1rl)+(vmc*(v1sl+v1sl)))/vml)}else{common.v1});let v1ta=(if (vky!=0.0){(((vmj*v1rm)+(vmc*(v1sn+v1sn)))/vml)}else{common.v1});let v1tb=(if (vky!=0.0){(((vmj*v1rn)+(vmc*(v1sp+v1sp)))/vml)}else{common.v1});let v1tc=(if (vky!=0.0){(((vmj*v1ro)+(vmc*(v1sr+v1sr)))/vml)}else{common.v1});
        let v1u1=(common.v1m*vmt);let v1u6=(if (vky!=0.0){((if (vky!=0.0){(v1t9+((vmp*v1sh)+(vmi*(vmo*v1sh))))}else{common.v1})/v1u1)}else{common.v1});let v1u7=(if (vky!=0.0){((if (vky!=0.0){(v1ta+((vmp*v1si)+(vmi*(vmo*v1si))))}else{common.v1})/v1u1)}else{common.v1});let v1u8=(if (vky!=0.0){((if (vky!=0.0){(v1tb+((vmp*v1sj)+(vmi*(vmo*v1sj))))}else{common.v1})/v1u1)}else{common.v1});let v1u9=(if (vky!=0.0){((if (vky!=0.0){(v1tc+((vmp*v1sk)+(vmi*(vmo*v1sk))))}else{common.v1})/v1u1)}else{common.v1});
        let v1ua=(common.ver*v1sh);let v1ub=(common.ver*v1si);let v1uc=(common.ver*v1sj);let v1ud=(common.ver*v1sk);let v1ui=(if vmx{(v1u6+v1ua)}else{common.v1});let v1uj=(if vmx{(v1u7+v1ub)}else{common.v1});let v1uk=(if vmx{(v1u8+v1uc)}else{common.v1});let v1ul=(if vmx{(v1u9+v1ud)}else{common.v1});let v1um=(-v1t9);let v1un=(-v1ta);let v1uo=(-v1tb);let v1up=(-v1tc);let v1ut=(vn0*vn0);let v1vf=(if vn5{(v1ua-v1u6)}else{(if vmx{(((vn0*v1um)-(vn1*v1ui))/v1ut)}else{common.v1})});
        let v1vg=(if vn5{(v1ub-v1u7)}else{(if vmx{(((vn0*v1un)-(vn1*v1uj))/v1ut)}else{common.v1})});let v1vh=(if vn5{(v1uc-v1u8)}else{(if vmx{(((vn0*v1uo)-(vn1*v1uk))/v1ut)}else{common.v1})});let v1vi=(if vn5{(v1ud-v1u9)}else{(if vmx{(((vn0*v1up)-(vn1*v1ul))/v1ut)}else{common.v1})});let v1vm=(vn7*vn7);let v1w0=(if vn5{(((vn7*v1um)-(vn1*v1vf))/v1vm)}else{v1ui});let v1w1=(if vn5{(((vn7*v1un)-(vn1*v1vg))/v1vm)}else{v1uj});let v1w2=(if vn5{(((vn7*v1uo)-(vn1*v1vh))/v1vm)}else{v1uk});
        let v1w3=(if vn5{(((vn7*v1up)-(vn1*v1vi))/v1vm)}else{v1ul});let v1w4=-0.6666666666666667;let v1w6=(vm9*f64::powf(vn9,v1w4));let v1wk=(vm9*f64::powf(vnm,v1w4));let v1x6=(vm9*f64::powf(vn7,v1w4));let v1xk=(vm9*f64::powf(vo4,v1w4));let v1yl=(vmo*v1p8);let v1yr=(common.v1m*voj);
        let v1yw=(if (vky!=0.0){((v1p9+(if (vky!=0.0){(((if vnr{(common.v6d*v1w0)}else{(if vnl{(-((-v1w0)*v1wk))}else{(if vnd{(v1w0*v1w6)}else{common.v1})})})+(if vo9{(common.v6d*v1vf)}else{(if vo3{(-((-v1vf)*v1xk))}else{(if vnw{(v1vf*v1x6)}else{common.v1})})}))-(vm9*v1pd))}else{common.v1}))/v1yr)}else{v1r1});
        let v1yx=(if (vky!=0.0){(((if (vky!=0.0){(((if vnr{(common.v6d*v1w1)}else{(if vnl{(-((-v1w1)*v1wk))}else{(if vnd{(v1w1*v1w6)}else{common.v1})})})+(if vo9{(common.v6d*v1vg)}else{(if vo3{(-((-v1vg)*v1xk))}else{(if vnw{(v1vg*v1x6)}else{common.v1})})}))-(vm9*v1pe))}else{common.v1})+(v1yl-v1oz))/v1yr)}else{v1r2});
        let v1yy=(if (vky!=0.0){((v1pb+(if (vky!=0.0){(((if vnr{(common.v6d*v1w2)}else{(if vnl{(-((-v1w2)*v1wk))}else{(if vnd{(v1w2*v1w6)}else{common.v1})})})+(if vo9{(common.v6d*v1vh)}else{(if vo3{(-((-v1vh)*v1xk))}else{(if vnw{(v1vh*v1x6)}else{common.v1})})}))-(vm9*v1pf))}else{common.v1}))/v1yr)}else{v1r3});
        let v1yz=(if (vky!=0.0){((v1pc+(if (vky!=0.0){(((if vnr{(common.v6d*v1w3)}else{(if vnl{(-((-v1w3)*v1wk))}else{(if vnd{(v1w3*v1w6)}else{common.v1})})})+(if vo9{(common.v6d*v1vi)}else{(if vo3{(-((-v1vi)*v1xk))}else{(if vnw{(v1vi*v1x6)}else{common.v1})})}))-(vm9*v1pg))}else{common.v1}))/v1yr)}else{v1r4});let v1z1=(vok*v1yw);let v1z3=(vok*v1yx);let v1z5=(vok*v1yy);let v1z7=(vok*v1yz);let v1zl=(if (vky!=0.0){((-(v1z1+v1z1))-(common.v1m*v1oy))}else{common.v1});
        let v1zm=(if (vky!=0.0){(((vol*v1p8)-(v1z3+v1z3))-(common.v1m*v1oz))}else{common.v1});let v1zn=(if (vky!=0.0){((-(v1z5+v1z5))-(common.v1m*v1p0))}else{common.v1});let v1zo=(if (vky!=0.0){((-(v1z7+v1z7))-(common.v1m*v1p1))}else{common.v1});let v20n=(if (vky!=0.0){(((vok*((vls*v1oy)-(common.v1m*v1oo)))-(vow*v1yw))/von)}else{common.v1});let v20o=(if (vky!=0.0){(((vok*((((vls*v1oz)+(vlq*v1p5))-(common.v1m*v1op))-((vog*v1p5)+(vls*v1yl))))-(vow*v1yx))/von)}else{common.v1});
        let v20p=(if (vky!=0.0){(((vok*((vls*v1p0)-(common.v1m*v1oq)))-(vow*v1yy))/von)}else{common.v1});let v20q=(if (vky!=0.0){(((vok*((vls*v1p1)-(common.v1m*v1or)))-(vow*v1yz))/von)}else{common.v1});let v20z=(common.v1m*vp4);let v214=(if vp3{((if (vky!=0.0){(v1zl+v20n)}else{common.v1})/v20z)}else{common.v1});let v215=(if vp3{((if (vky!=0.0){(v1zm+v20o)}else{common.v1})/v20z)}else{common.v1});let v216=(if vp3{((if (vky!=0.0){(v1zn+v20p)}else{common.v1})/v20z)}else{common.v1});
        let v217=(if vp3{((if (vky!=0.0){(v1zo+v20q)}else{common.v1})/v20z)}else{common.v1});let v218=(vp6*v1p5);let v21u=(vpf*(if vpd{(v1zl-v20n)}else{common.v1}));let v21w=(vpf*(if vpd{(v1zm-v20o)}else{common.v1}));let v21y=(vpf*(if vpd{(v1zn-v20p)}else{common.v1}));let v220=(vpf*(if vpd{(v1zo-v20q)}else{common.v1}));let v222=(common.v1m*vpj);let v227=(common.v1m*vpk);let v22t=(-v1mt);let v22u=(-v1mu);let v22v=(-v1mv);let v22z=(if vpt{(common.v72*v22t)}else{common.v1});
        let v230=(if vpt{(common.v72*v22u)}else{common.v1});let v231=(if vpt{(common.v72*v22v)}else{common.v1});let v23w=(common.v1m*vq5);let v246=(vq6*vq6);let v24k=(if vqa{v1nh}else{v22z});let v24l=(if vqa{v1ni}else{v230});let v24m=(if vqa{v1nj}else{v231});let v24q=(common.v1m*vqe);
        let v250=(if vqa{(((-v24k)+(v24k/v24q))/vqh)}else{(if vpt{(((vq6*((vpz*v22t)+(vpu*(common.v1m*(-(common.v1m*v22z))))))-(vq0*((-(common.vf1*v22z))+((-(vld*v22z))/v23w))))/v246)}else{(if vpd{(common.v1v*((if vpd{(((v21u+v21u)/v222)/v227)}else{v214})-v1yw))}else{(if vp3{(common.v1v*(v1yw+v214))}else{common.v1})})})});
        let v251=(if vqa{common.v1}else{(if vpt{common.v1}else{(if vpd{(v218+(common.v1v*((if vpd{(((v21w+v21w)/v222)/v227)}else{v215})-v1yx)))}else{(if vp3{(v218+(common.v1v*(v1yx+v215)))}else{common.v1})})})});
        let v252=(if vqa{(((-v24l)+(v24l/v24q))/vqh)}else{(if vpt{(((vq6*((vpz*v22u)+(vpu*(common.v1m*(-(common.v1m*v230))))))-(vq0*((-(common.vf1*v230))+((-(vld*v230))/v23w))))/v246)}else{(if vpd{(common.v1v*((if vpd{(((v21y+v21y)/v222)/v227)}else{v216})-v1yy))}else{(if vp3{(common.v1v*(v1yy+v216))}else{common.v1})})})});
        let v253=(if vqa{(((-v24m)+(v24m/v24q))/vqh)}else{(if vpt{(((vq6*((vpz*v22v)+(vpu*(common.v1m*(-(common.v1m*v231))))))-(vq0*((-(common.vf1*v231))+((-(vld*v231))/v23w))))/v246)}else{(if vpd{(common.v1v*((if vpd{(((v220+v220)/v222)/v227)}else{v217})-v1yz))}else{(if vp3{(common.v1v*(v1yz+v217))}else{common.v1})})})});let v254=(if (vqn!=0.0){v250}else{common.v1});let v255=(if (vqn!=0.0){v251}else{common.v1});let v256=(if (vqn!=0.0){v252}else{common.v1});let v257=(if (vqn!=0.0){v253}else{common.v1});
        let v258=(v1mw+v250);let v259=(v1mx+v252);let v25a=(v1my+v253);let v25b=(common.v1m*vqr);let v25k=(if (vqn!=0.0){(common.v6x*(v258/v25b))}else{common.v1});let v25l=(if (vqn!=0.0){(common.v6x*(v251/v25b))}else{common.v1});let v25m=(if (vqn!=0.0){(common.v6x*(v259/v25b))}else{common.v1});let v25n=(if (vqn!=0.0){(common.v6x*(v25a/v25b))}else{common.v1});let v25p=(v255/v5b);let v263=(if vqu{(vj8*(common.v1v*(v254/v5b)))}else{common.v1});
        let v264=(if vqu{((vqx*v1ky)+(vj8*(common.v1v*(v25p-v1ku))))}else{common.v1});let v265=(if vqu{(vj8*(common.v1v*(v256/v5b)))}else{common.v1});let v266=(if vqu{(vj8*(common.v1v*(v257/v5b)))}else{common.v1});let v26c=(if vqu{((vr1*v1ky)+(vj8*(common.v1v*(v1ku+v25p))))}else{common.v1});let v26d=(vqz*v263);let v26f=(vqz*v264);let v26h=(vqz*v265);let v26j=(vqz*v266);let v26m=(common.v1m*vr6);let v26r=(if vqu{((v26d+v26d)/v26m)}else{common.v1});let v26s=(if vqu{((v1kv+(v26f+v26f))/v26m)}else{common.v1});
        let v26t=(if vqu{((v26h+v26h)/v26m)}else{common.v1});let v26u=(if vqu{((v26j+v26j)/v26m)}else{common.v1});let v26v=(vr3*v263);let v26x=(vr3*v26c);let v26z=(vr3*v265);let v271=(vr3*v266);let v274=(common.v1m*vra);let v279=(if vqu{((v26v+v26v)/v274)}else{common.v1});let v27a=(if vqu{((v1kv+(v26x+v26x))/v274)}else{common.v1});let v27b=(if vqu{((v26z+v26z)/v274)}else{common.v1});let v27c=(if vqu{((v271+v271)/v274)}else{common.v1});let v27i=(if vqu{(v26r+v279)}else{common.v1});
        let v27j=(if vqu{((v26s+v27a)-v1kw)}else{common.v1});let v27k=(if vqu{(v26t+v27b)}else{common.v1});let v27l=(if vqu{(v26u+v27c)}else{common.v1});let v27p=(vr7*vr7);let v286=(vrb*vrb);let v29g=((vrn*(common.v1m*v25k))+(vrm*(-v25k)));let v29j=((vrn*(common.v1m*v25l))+(vrm*(-v25l)));let v29m=((vrn*(common.v1m*v25m))+(vrm*(-v25m)));let v29p=((vrn*(common.v1m*v25n))+(vrm*(-v25n)));let v2a5=(vrq*vrq);let v2b2=(vqp*vqp);let v2bg=(common.v1m*vrv);let v2c5=(common.v1m*vrz);
        let v2ca=(if vrx{((((vqp*v29g)-(vro*v254))/v2b2)/v2c5)}else{(if vqu{((((vqp*((vrs*v29g)+(vro*(-(((vrq*((vrl*v254)+(vqp*(if vqu{((vj8*(common.v1v*((((vr7*v263)-(vqz*v26r))/v27p)+(((vrb*v263)-(vr3*v279))/v286))))/v5b)}else{common.v1}))))-(vrp*v27i))/v2a5)))))-(vrt*v254))/v2b2)/v2bg)}else{common.v1})});
        let v2cb=(if vrx{((((vqp*v29j)-(vro*v255))/v2b2)/v2c5)}else{(if vqu{((((vqp*((vrs*v29j)+(vro*(-(((vrq*((vrl*v255)+(vqp*(if vqu{(((vri*v1ky)+(vj8*(common.v1v*((((vr7*v264)-(vqz*v26s))/v27p)+(((vrb*v26c)-(vr3*v27a))/v286)))))/v5b)}else{common.v1}))))-(vrp*v27j))/v2a5)))))-(vrt*v255))/v2b2)/v2bg)}else{common.v1})});
        let v2cc=(if vrx{((((vqp*v29m)-(vro*v256))/v2b2)/v2c5)}else{(if vqu{((((vqp*((vrs*v29m)+(vro*(-(((vrq*((vrl*v256)+(vqp*(if vqu{((vj8*(common.v1v*((((vr7*v265)-(vqz*v26t))/v27p)+(((vrb*v265)-(vr3*v27b))/v286))))/v5b)}else{common.v1}))))-(vrp*v27k))/v2a5)))))-(vrt*v256))/v2b2)/v2bg)}else{common.v1})});
        let v2cd=(if vrx{((((vqp*v29p)-(vro*v257))/v2b2)/v2c5)}else{(if vqu{((((vqp*((vrs*v29p)+(vro*(-(((vrq*((vrl*v257)+(vqp*(if vqu{((vj8*(common.v1v*((((vr7*v266)-(vqz*v26u))/v27p)+(((vrb*v266)-(vr3*v27c))/v286))))/v5b)}else{common.v1}))))-(vrp*v27l))/v2a5)))))-(vrt*v257))/v2b2)/v2bg)}else{common.v1})});let v2ci=(vs0*v2ca);let v2ck=(vs0*v2cb);let v2cm=(vs0*v2cc);let v2co=(vs0*v2cd);let v2ct=(vs2*vs2);let v2dm=(vs7*vs7);let v2e0=(if (vqn!=0.0){(((vs7*(sf[110]*v250))-(vs6*v254))/v2dm)}else{common.v1});
        let v2e1=(if (vqn!=0.0){(((vs7*(sf[110]*v251))-(vs6*v255))/v2dm)}else{common.v1});let v2e2=(if (vqn!=0.0){(((vs7*(sf[110]*v252))-(vs6*v256))/v2dm)}else{common.v1});let v2e3=(if (vqn!=0.0){(((vs7*(sf[110]*v253))-(vs6*v257))/v2dm)}else{common.v1});let v2ek=(if (vqn!=0.0){((vsb*v2e0)+(vsa*(common.v8h*v2e0)))}else{common.v1});let v2el=(if (vqn!=0.0){((vsb*v2e1)+(vsa*(common.v8h*v2e1)))}else{common.v1});let v2em=(if (vqn!=0.0){((vsb*v2e2)+(vsa*(common.v8h*v2e2)))}else{common.v1});
        let v2en=(if (vqn!=0.0){((vsb*v2e3)+(vsa*(common.v8h*v2e3)))}else{common.v1});let v2eo=(common.v1m*v1l6);let v2ep=(common.v1m*v1l7);let v2eq=(vse*v254);let v2er=(vse*v255);let v2eu=((vse*v256)+(vqp*v2eo));let v2ex=((vse*v257)+(vqp*v2ep));let v2f2=(vsg*(-v254));let v2f3=(v2f2+v2f2);let v2f4=(vsg*(-v255));let v2f5=(v2f4+v2f4);let v2f6=(vsg*(v1l6-v256));let v2f7=(v2f6+v2f6);let v2f8=(vsg*(v1l7-v257));let v2f9=(v2f8+v2f8);let v2fe=(common.v1m*vsj);let v2fl=(vsk*v254);let v2fm=(v2fl+v2fl);
        let v2fn=(vsk*v255);let v2fo=(v2fn+v2fn);let v2fp=(vsk*(v1l6+v256));let v2fq=(v2fp+v2fp);let v2fr=(vsk*(v1l7+v257));let v2fs=(v2fr+v2fr);let v2fx=(common.v1m*vsn);let v2g9=(vso*vso);let v2gn=(if (vqn!=0.0){(((vso*v2eq)-(vsf*(((v2ek+v2f3)/v2fe)+((v2ek+v2fm)/v2fx))))/v2g9)}else{common.v1});let v2go=(if (vqn!=0.0){(((vso*v2er)-(vsf*(((v2el+v2f5)/v2fe)+((v2el+v2fo)/v2fx))))/v2g9)}else{common.v1});let v2gp=(if (vqn!=0.0){(((vso*v2eu)-(vsf*(((v2em+v2f7)/v2fe)+((v2em+v2fq)/v2fx))))/v2g9)}else{common.v1});
        let v2gq=(if (vqn!=0.0){(((vso*v2ex)-(vsf*(((v2en+v2f9)/v2fe)+((v2en+v2fs)/v2fx))))/v2g9)}else{common.v1});let v2hb=(if vss{(((vs7*(sf[110]*v2gn))-(vst*v254))/v2dm)}else{v2e0});let v2hc=(if vss{(((vs7*(sf[110]*v2go))-(vst*v255))/v2dm)}else{v2e1});let v2hd=(if vss{(((vs7*(sf[110]*v2gp))-(vst*v256))/v2dm)}else{v2e2});let v2he=(if vss{(((vs7*(sf[110]*v2gq))-(vst*v257))/v2dm)}else{v2e3});let v2hv=(if vss{((vsx*v2hb)+(vsw*(common.v8h*v2hb)))}else{v2ek});
        let v2hw=(if vss{((vsx*v2hc)+(vsw*(common.v8h*v2hc)))}else{v2el});let v2hx=(if vss{((vsx*v2hd)+(vsw*(common.v8h*v2hd)))}else{v2em});let v2hy=(if vss{((vsx*v2he)+(vsw*(common.v8h*v2he)))}else{v2en});let v2i3=(common.v1m*vt1);let v2ic=(common.v1m*vt3);let v2io=(vt4*vt4);let v2j2=(if vss{(((vt4*v2eq)-(vsf*(((v2f3+v2hv)/v2i3)+((v2fm+v2hv)/v2ic))))/v2io)}else{v2gn});let v2j3=(if vss{(((vt4*v2er)-(vsf*(((v2f5+v2hw)/v2i3)+((v2fo+v2hw)/v2ic))))/v2io)}else{v2go});
        let v2j4=(if vss{(((vt4*v2eu)-(vsf*(((v2f7+v2hx)/v2i3)+((v2fq+v2hx)/v2ic))))/v2io)}else{v2gp});let v2j5=(if vss{(((vt4*v2ex)-(vsf*(((v2f9+v2hy)/v2i3)+((v2fs+v2hy)/v2ic))))/v2io)}else{v2gq});let v2ja=(common.v1m*vt8);let v2k0=(v2j3/v5b);let v2ke=(if vqu{(vj8*(common.v1v*(v2j2/v5b)))}else{v263});let v2kf=(if vqu{((vte*v1ky)+(vj8*(common.v1v*(v2k0-v1ku))))}else{v264});let v2kg=(if vqu{(vj8*(common.v1v*(v2j4/v5b)))}else{v265});let v2kh=(if vqu{(vj8*(common.v1v*(v2j5/v5b)))}else{v266});
        let v2kn=(if vqu{((vti*v1ky)+(vj8*(common.v1v*(v1ku+v2k0))))}else{v26c});let v2ko=(vtg*v2ke);let v2kq=(vtg*v2kf);let v2ks=(vtg*v2kg);let v2ku=(vtg*v2kh);let v2kx=(common.v1m*vtn);let v2l2=(if vqu{((v2ko+v2ko)/v2kx)}else{v26r});let v2l3=(if vqu{((v1kv+(v2kq+v2kq))/v2kx)}else{v26s});let v2l4=(if vqu{((v2ks+v2ks)/v2kx)}else{v26t});let v2l5=(if vqu{((v2ku+v2ku)/v2kx)}else{v26u});let v2l6=(vtk*v2ke);let v2l8=(vtk*v2kn);let v2la=(vtk*v2kg);let v2lc=(vtk*v2kh);let v2lf=(common.v1m*vtr);
        let v2lk=(if vqu{((v2l6+v2l6)/v2lf)}else{v279});let v2ll=(if vqu{((v1kv+(v2l8+v2l8))/v2lf)}else{v27a});let v2lm=(if vqu{((v2la+v2la)/v2lf)}else{v27b});let v2ln=(if vqu{((v2lc+v2lc)/v2lf)}else{v27c});let v2md=(vtz*(-v250));let v2mf=(vtz*(-v251));let v2mh=(vtz*(v1l6-v252));let v2mj=(vtz*(v1l7-v253));let v2ml=(common.v1m*vu2);let v2ms=(vu3*v250);let v2mu=(vu3*v251);let v2mw=(vu3*(v1l6+v252));let v2my=(vu3*(v1l7+v253));let v2n0=(common.v1m*vu6);let v2nc=(vu7*vu7);
        let v2nq=(if vtx{(((vu7*(vse*v250))-(vty*(((v2md+v2md)/v2ml)+((v2ms+v2ms)/v2n0))))/v2nc)}else{v2j2});let v2nr=(if vtx{(((vu7*(vse*v251))-(vty*(((v2mf+v2mf)/v2ml)+((v2mu+v2mu)/v2n0))))/v2nc)}else{v2j3});let v2ns=(if vtx{(((vu7*((vse*v252)+(vqj*v2eo)))-(vty*(((v2mh+v2mh)/v2ml)+((v2mw+v2mw)/v2n0))))/v2nc)}else{v2j4});let v2nt=(if vtx{(((vu7*((vse*v253)+(vqj*v2ep)))-(vty*(((v2mj+v2mj)/v2ml)+((v2my+v2my)/v2n0))))/v2nc)}else{v2j5});let v2nv=(v2nr/v5b);
        let v2o9=(if vua{(vj8*(common.v1v*(v2nq/v5b)))}else{v2ke});let v2ob=(if vua{(vj8*(common.v1v*(v2ns/v5b)))}else{v2kg});let v2oc=(if vua{(vj8*(common.v1v*(v2nt/v5b)))}else{v2kh});let v2oj=(vuf*v2o9);let v2ol=(vuf*(if vua{((vud*v1ky)+(vj8*(common.v1v*(v2nv-v1ku))))}else{v2kf}));let v2on=(vuf*v2ob);let v2op=(vuf*v2oc);let v2os=(common.v1m*vum);let v2p1=(vuj*v2o9);let v2p3=(vuj*(if vua{((vuh*v1ky)+(vj8*(common.v1v*(v1ku+v2nv))))}else{v2kn}));let v2p5=(vuj*v2ob);let v2p7=(vuj*v2oc);
        let v2pa=(common.v1m*vuq);let v2pz=(common.v1m*vuy);let v2qt=(vv6*vv6);
        let v2rd=((vv8*v2nq)+(vu9*(vjn*(((vv6*(vcn*(if (vv3!=0.0){common.v1}else{(if vtx{(-(common.v6x*((v1mw+v2nq)/v2pz)))}else{(if (vqn!=0.0){(-((vt8*v2ca)+(vs0*(((if (vqn!=0.0){((((vs2*(common.v72*v258))-(vs1*(v2ci+v2ci)))/v2ct)-v254)}else{common.v1})+v2j2)/v2ja))))}else{common.v1})})})))-(vv5*(if vuv{common.v1}else{(if vua{((if vua{((v2oj+v2oj)/v2os)}else{v2l2})+(if vua{((v2p1+v2p1)/v2pa)}else{v2lk}))}else{(if vrx{common.v1}else{(if vqu{(v2l2+v2lk)}else{v27i})})})})))/v2qt))));
        let v2rg=((vv8*v2nr)+(vu9*(vjn*(((vv6*((vv4*(if sb[38]{((-(v8t*v1dk))/(vcl*vcl))}else{(if (sf[105]!=0.0){((-(vcg*v1dk))/(vch*vch))}else{common.v1})}))+(vcn*(if (vv3!=0.0){common.v1}else{(if vtx{(-(common.v6x*(v2nr/v2pz)))}else{(if (vqn!=0.0){(-((vt8*v2cb)+(vs0*(((if (vqn!=0.0){((((vs2*(common.v72*v251))-(vs1*(v2ck+v2ck)))/v2ct)-v255)}else{common.v1})+v2j3)/v2ja))))}else{common.v1})})}))))-(vv5*(if vuv{common.v1}else{(if vua{(((if vua{((v1kv+(v2ol+v2ol))/v2os)}else{v2l3})+(if vua{((v1kv+(v2p3+v2p3))/v2pa)}else{v2ll}))-v1kw)}else{(if vrx{common.v1}else{(if vqu{((v2l3+v2ll)-v1kw)}else{v27j})})})})))/v2qt))));
        let v2rj=((vv8*v2ns)+(vu9*(vjn*(((vv6*(vcn*(if (vv3!=0.0){common.v1}else{(if vtx{(-(common.v6x*((v1mx+v2ns)/v2pz)))}else{(if (vqn!=0.0){(-((vt8*v2cc)+(vs0*(((if (vqn!=0.0){((((vs2*(common.v72*v259))-(vs1*(v2cm+v2cm)))/v2ct)-v256)}else{common.v1})+v2j4)/v2ja))))}else{common.v1})})})))-(vv5*(if vuv{common.v1}else{(if vua{((if vua{((v2on+v2on)/v2os)}else{v2l4})+(if vua{((v2p5+v2p5)/v2pa)}else{v2lm}))}else{(if vrx{common.v1}else{(if vqu{(v2l4+v2lm)}else{v27k})})})})))/v2qt))));
        let v2rm=((vv8*v2nt)+(vu9*(vjn*(((vv6*(vcn*(if (vv3!=0.0){common.v1}else{(if vtx{(-(common.v6x*((v1my+v2nt)/v2pz)))}else{(if (vqn!=0.0){(-((vt8*v2cd)+(vs0*(((if (vqn!=0.0){((((vs2*(common.v72*v25a))-(vs1*(v2co+v2co)))/v2ct)-v257)}else{common.v1})+v2j5)/v2ja))))}else{common.v1})})})))-(vv5*(if vuv{common.v1}else{(if vua{((if vua{((v2op+v2op)/v2os)}else{v2l5})+(if vua{((v2p7+v2p7)/v2pa)}else{v2ln}))}else{(if vrx{common.v1}else{(if vqu{(v2l5+v2ln)}else{v27l})})})})))/v2qt))));
        let v2rr=((-v1el)/(vdm*vdm));let v2rs=(if vvg{v2rr}else{common.v1});let v2rt=(sf[173]*vvi);let v2rv=(sf[172]*vvi);let v2s6=(-v1ew);let v2sf=(if vvq{(vvs*v2rt)}else{(if vvl{(vvn*v2rt)}else{common.v1})});let v2sg=(if vvq{((vvv*(vvs*((vvi*v1ew)+(vdv*v2rs))))+(vvs*((vvt*v2rs)+(vvi*v2s6))))}else{(if vvl{(vvn*(common.vbb*v2rs))}else{common.v1})});let v2sh=(if vvq{(vvs*v2rv)}else{(if vvl{(vvn*v2rv)}else{common.v1})});let v2sv=((-v1f1)/(ve4*ve4));let v2sw=(if vw6{v2sv}else{v2rs});let v2sx=(sf[173]*vw8);
        let v2sz=(sf[172]*vw8);let v2ta=(-v1fc);let v2u3=(-(if sb[48]{common.v1}else{v1iv}));let v2u5=(if vwy{v2u3}else{common.v1});let v2uc=((-((vht*common.v1db)+(common.vby*(if sb[48]{common.v1}else{v1iy}))))/(vx2*vx2));let v2ud=(if vwy{v2uc}else{common.v1});let v2ue=(vx4*(if vwy{sf[172]}else{common.v1}));let v2ui=(vx4*(if vwy{sf[173]}else{common.v1}));
        let v2vt=(if vxx{common.v1}else{(if (vvb!=0.0){(((if (vvb!=0.0){((if vw2{common.v1}else{(if vvg{(vvc*v2sf)}else{common.v1})})+(if vws{common.v1}else{(if vw6{(vvd*(if vwg{(vwi*v2sx)}else{(if vwb{(vwd*v2sx)}else{v2sf})}))}else{common.v1})}))}else{common.v1})+(if vxr{common.v1}else{(if vwy{(sf[214]*(if vxc{(vxe*v2ue)}else{(if vx7{(vx9*v2ue)}else{common.v1})}))}else{common.v1})}))+sf[245])}else{common.v1})});
        let v2vu=(if vxx{common.v1}else{(if (vvb!=0.0){((if (vvb!=0.0){((if vw2{common.v1}else{(if vvg{((vvy*(if (vvb!=0.0){v1fd}else{common.v1}))+(vvc*v2sg))}else{common.v1})})+(if vws{common.v1}else{(if vw6{((vwo*(if (vvb!=0.0){(sf[22]*v1fb)}else{common.v1}))+(vvd*(if vwg{((vwl*(vwi*((vw8*v1fc)+(vec*v2sw))))+(vwi*((vwj*v2sw)+(vw8*v2ta))))}else{(if vwb{(vwd*(common.vbb*v2sw))}else{v2sg})})))}else{common.v1})}))}else{common.v1})+(if vxr{common.v1}else{(if vwy{(sf[214]*((if vxc{((vxh*(vxe*((vx4*v1jg)+(vhu*v2ud))))+(vxe*((vxf*v2ud)+(vx4*(v2u5-v1jg)))))}else{(if vx7{(vx9*((vx4*v2u5)+(vx1*v2ud)))}else{common.v1})})-(vxm*((vx4*v2u3)+(vwz*v2ud)))))}else{common.v1})}))}else{common.v1})});
        let v2vv=(if vxx{common.v1}else{(if (vvb!=0.0){(((if (vvb!=0.0){((if vw2{common.v1}else{(if vvg{(vvc*v2sh)}else{common.v1})})+(if vws{common.v1}else{(if vw6{(vvd*(if vwg{(vwi*v2sz)}else{(if vwb{(vwd*v2sz)}else{v2sh})}))}else{common.v1})}))}else{common.v1})+(if vxr{common.v1}else{(if vwy{(sf[214]*(if vxc{(vxe*v2ui)}else{(if vx7{(vx9*v2ui)}else{common.v1})}))}else{common.v1})}))+sf[246])}else{common.v1})});let v2vy=(if vy5{v2rr}else{common.v1});let v2vz=(sf[173]*vy6);let v2w1=(sf[172]*vy6);
        let v2wk=(if vye{(vyg*v2vz)}else{(if vy9{(vyb*v2vz)}else{common.v1})});let v2wl=(if vye{((vyj*(vyg*((vy6*v1ew)+(vdv*v2vy))))+(vyg*((vyh*v2vy)+(vy6*v2s6))))}else{(if vy9{(vyb*(common.vbd*v2vy))}else{common.v1})});let v2wm=(if vye{(vyg*v2w1)}else{(if vy9{(vyb*v2w1)}else{common.v1})});let v2wy=(if vyu{v2sv}else{v2vy});let v2wz=(sf[173]*vyv);let v2x1=(sf[172]*vyv);let v2y5=(if vzj{v2u3}else{common.v1});let v2y7=(if vzj{v2uc}else{common.v1});let v2y8=(vzm*(if vzj{sf[172]}else{common.v1}));
        let v2yc=(vzm*(if vzj{sf[173]}else{common.v1}));let v2zl=(if v10d{common.v1}else{(if (vy0!=0.0){(sf[245]+((if (vy0!=0.0){((if vyq{common.v1}else{(if vy5{(vy1*v2wk)}else{common.v1})})+(if vzf{common.v1}else{(if vyu{(vy2*(if vz3{(vz5*v2wz)}else{(if vyy{(vz0*v2wz)}else{v2wk})}))}else{common.v1})}))}else{common.v1})+(if v107{common.v1}else{(if vzj{(sf[214]*(if vzu{(vzw*v2y8)}else{(if vzp{(vzr*v2y8)}else{common.v1})}))}else{common.v1})})))}else{common.v1})});
        let v2zm=(if v10d{common.v1}else{(if (vy0!=0.0){((if (vy0!=0.0){((if vyq{common.v1}else{(if vy5{((vym*(if (vy0!=0.0){v1fd}else{common.v1}))+(vy1*v2wl))}else{common.v1})})+(if vzf{common.v1}else{(if vyu{((vzb*(if (vy0!=0.0){(sf[24]*v1fb)}else{common.v1}))+(vy2*(if vz3{((vz8*(vz5*((vyv*v1fc)+(vec*v2wy))))+(vz5*((vz6*v2wy)+(vyv*v2ta))))}else{(if vyy{(vz0*(common.vbd*v2wy))}else{v2wl})})))}else{common.v1})}))}else{common.v1})+(if v107{common.v1}else{(if vzj{(sf[214]*((if vzu{((vzz*(vzw*((vzm*v1jg)+(vhu*v2y7))))+(vzw*((vzx*v2y7)+(vzm*(v2y5-v1jg)))))}else{(if vzp{(vzr*((vzm*v2y5)+(vzl*v2y7)))}else{common.v1})})-(v103*((vzm*v2u3)+(vwz*v2y7)))))}else{common.v1})}))}else{common.v1})});
        let v2zn=(if v10d{common.v1}else{(if (vy0!=0.0){(sf[246]+((if (vy0!=0.0){((if vyq{common.v1}else{(if vy5{(vy1*v2wm)}else{common.v1})})+(if vzf{common.v1}else{(if vyu{(vy2*(if vz3{(vz5*v2x1)}else{(if vyy{(vz0*v2x1)}else{v2wm})}))}else{common.v1})}))}else{common.v1})+(if v107{common.v1}else{(if vzj{(sf[214]*(if vzu{(vzw*v2yc)}else{(if vzp{(vzr*v2yc)}else{common.v1})}))}else{common.v1})})))}else{common.v1})});let v30d=(-v10k);let v30f=(-v10p);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v12i),
            [1, 3, 4, 5],
            [(sf[173]*v2rd), (sf[173]*v2rg), (sf[173]*v2rj), (sf[173]*v2rm)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v12j),
            1,
            multiplicity * ((sf[173]*v2vt)),
            3,
            multiplicity * ((sf[173]*v2vu)),
            4,
            multiplicity * ((sf[173]*v2vv)),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * (v12k),
            1,
            multiplicity * ((sf[173]*v2zl)),
            3,
            multiplicity * ((sf[173]*v2zm)),
            5,
            multiplicity * ((sf[173]*v2zn)),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if sb[88]{(v9*common.vb2)}else{(if sb[87]{(v114*v12b)}else{(if sb[86]{((sf[303]*(f64::powf(v120,sf[217])-common.v0))/sf[217])}else{(if sb[78]{v114}else{common.v1})})})})),
            3,
            multiplicity * ((if sb[88]{v9}else{(if sb[87]{((sf[286]*v12b)+(v114*sf[306]))}else{(if sb[86]{((sf[303]*(sf[305]*(sf[217]*f64::powf(v120,sf[247]))))/sf[217])}else{sf[304]})})})),
        );
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * ((if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-(((((vb8*vv9)+(common.vbb*vxy))+(common.vbd*v10e))+(v10k*v10m))+(v10p*v10r)))}else{common.v1})})),
            &[(if sb[88]{common.v1}else{(if (sf[287]!=0.0){v30d}else{common.v1})}),(if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-(((vb8*v2rd)+(v12j+(common.vbb*v2vt)))+(v12k+(common.vbd*v2zl))))}else{common.v1})}),(if sb[88]{common.v1}else{(if (sf[287]!=0.0){v30f}else{common.v1})}),(if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-(((vb8*v2rg)+(common.vbb*v2vu))+(common.vbd*v2zm)))}else{common.v1})}),(if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-((((sf[172]*vv9)+(vb8*v2rj))+((sf[172]*vxy)+(common.vbb*v2vv)))+v30d))}else{common.v1})}),(if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-(((v12i+(vb8*v2rm))+((sf[172]*v10e)+(common.vbd*v2zn)))+v30f))}else{common.v1})})],
            &[(if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-v10m)}else{common.v1})}),(if sb[88]{common.v1}else{(if (sf[287]!=0.0){(-v10r)}else{common.v1})})],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            0,
            multiplicity,
        );
        stamper.stamp_potential_node1_branch1_local(
            0,
            (if (v1cl!=0.0){(vd2*v1ct)}else{common.v1}),
            3,
            (if (v1cl!=0.0){(v1ct*v1e2)}else{common.v1}),
            0,
            (if (v1cl!=0.0){v1cp}else{common.v1}),
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * ((if v1cw{(v10m/v1cp)}else{common.v1})),
            0,
            multiplicity * ((if v1cw{(common.v0/v1cp)}else{common.v1})),
            3,
            multiplicity * ((if v1cw{((-(v10m*(sf[122]*v1e2)))/(v1cp*v1cp))}else{common.v1})),
            4,
            multiplicity * ((if v1cw{(vjg/v1cp)}else{common.v1})),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_node1_branch1_local(
            1,
            (if (v1co!=0.0){(vd2*v1cz)}else{common.v1}),
            3,
            (if (v1co!=0.0){(v1cz*v1e2)}else{common.v1}),
            1,
            (if (v1co!=0.0){v1cr}else{common.v1}),
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * ((if v1d2{(v10r/v1cr)}else{common.v1})),
            2,
            multiplicity * ((if v1d2{(common.v0/v1cr)}else{common.v1})),
            3,
            multiplicity * ((if v1d2{((-(v10r*(sf[126]*v1e2)))/(v1cr*v1cr))}else{common.v1})),
            5,
            multiplicity * ((if v1d2{(vjg/v1cr)}else{common.v1})),
        );
        let v1cf_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v1cf);
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * (v1cf_ddt),
            1,
            multiplicity * (((common.v3oq) * ddt_scale)),
            3,
            multiplicity * (((common.v3or) * ddt_scale)),
            4,
            multiplicity * (((common.v3os) * ddt_scale)),
        );
        let v1cg_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v1cg);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (v1cg_ddt),
            [1, 3, 4, 5],
            [((common.v3ot) * ddt_scale), ((common.v3ou) * ddt_scale), ((common.v3ov) * ddt_scale), ((common.v3ow) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v1ch_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v1ch);
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (v1ch_ddt),
            3,
            multiplicity * (((sf[149]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(5),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (common.v1),
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
        let common=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_node3(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes[1],
            multiplicity * (common.v3oq),
            nodes[3],
            multiplicity * (common.v3or),
            nodes[4],
            multiplicity * (common.v3os),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &[nodes[1], nodes[3], nodes[4], nodes[5]],
            &[common.v3ot, common.v3ou, common.v3ov, common.v3ow],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (sf[149]),
        );
    }
}
