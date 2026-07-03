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

impl Instance {
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
        let v1=ctx.node_voltage(nodes[2]);let v4=((ctx.temperature()+v1)+sf[0]);let v6=1300.0;let v7=173.14999999999998;let v8=(v4>v7);let v9=(if v8{v4}else{v7});let va=(v6<v9);let vb=(if va{v6}else{v9});let vc=1.0;let vd=0.0;let vj=8.6170869e-5;let vk=(vb*vj);let vl=(vb/sf[5]);let vm=(vl).ln();let vq=(vl-vc);let vr=(sf[7]*vq);let vx=(((vm*sf[6])+(vr/vk))).exp();let vy=(sf[9]*vx);let v10=((vm*sf[8])).exp();let v11=(sf[10]*v10);let v16=(sf[11]*(vc+(vq*sf[12])));let v1b=(sf[13]*(vc+(vq*sf[14])));let v1i=300.15;let v1k=(vb/v1i);let v1m=0.000702;let v1n=(vb*v1m);let v1o=(vb*v1n);let v1q=(vb+1108.0);let v1t=(-(1.16-(v1o/v1q)));let v1u=1.3806226e-23;let v1w=(v1u*(vb+vb));let v21=(-(vk+vk));let v22=1.5;let v25=1.6021918e-19;let v27=((v22*(v1k).ln())+(((v1t/v1w)+1.3454442398941469e20)*v25));let v28=(v21*v27);let v2b=((sf[19]-v28)/sf[18]);let v2c=(sf[19]-v2b);let v2f=0.0004;let v2k=(vc+(sf[20]*(sf[22]-(v2c/v2b))));let v2l=(sf[17]/v2k);let v2n=(v28+(v1k*v2b));let v2o=(v2n-v2b);let v2u=(vc+(sf[20]*((v2f*(vb-v1i))-(v2o/v2b))));let v2v=(v2l*v2u);let v2x=ctx.node_voltage(nodes[3]);let v2y=ctx.node_voltage(nodes[4]);let v2z=(v2x-v2y);let v30=(sf[23]*v2z);let v31=ctx.node_voltage(nodes[0]);let v32=(v31-v2x);let v34=ctx.node_voltage(nodes[1]);let v35=(v34-v2y);let v38=(if (vy>vd){vc}else{vd});let v3a=(vk*sf[24]);let v3c=(if (v38!=0.0){(v30/v3a)}else{vd});let v3d=(-v30);let v3e=(v3d-v1b);let v3g=(vk*sf[25]);let v3i=(if (v38!=0.0){(v3e/v3g)}else{vd});let v3j=(-v1b);let v3l=(if (v38!=0.0){(v3j/v3g)}else{vd});let v3m=80.0;let v3o=(if (v3c>v3m){vc}else{vd});let v3p=((v38!=0.0)&&(v3o!=0.0));let v3t=(if v3p{v3m}else{v3c});let v3v=((v38!=0.0)&&(!(v3o!=0.0)));let v3w=(if v3v{vc}else{(if v3p{(vc+(v3c-v3m))}else{vd})});let v3x=(v3t).exp();let v3z=(if (v38!=0.0){(v3w*v3x)}else{v3w});let v40=37.0;let v41=(v3i>=v40);let v42=(!v41);let v43=-37.0;let v44=(v3i<=v43);let v46=(v42&&(!v44));let v47=(v3i).exp();let v48=(vc+v47);let v4a=(v42&&v44);let v4e=(v3l>=v40);let v4f=(!v4e);let v4g=(v3l<=v43);let v4i=(v4f&&(!v4g));let v4j=(v3l).exp();let v4k=(vc+v4j);let v4m=(v4f&&v4g);let v4r=(if (v38!=0.0){((if v46{(v48).ln()}else{(if v4a{v47}else{(if v41{v3i}else{vd})})})-(if v4i{(v4k).ln()}else{(if v4m{v4j}else{(if v4e{v3l}else{vd})})}))}else{vd});let v4s=(v3z-vc);let v4u=(v16*v4r);let v4w=(v30).abs();let v4x=f64::powf(v4w,(sf[15]*(vc+(vq*sf[16]))));let v4z=(vc+(sf[26]*v4x));let v53=(!(v38!=0.0));let v54=(if v53{vd}else{(if (v38!=0.0){((vy*v4s)-(v4u/v4z))}else{vd})});let v56=(if (v11>vd){vc}else{vd});let v58=(sf[27]-v30);let v59=0.001;let v5a=(v58>v59);let v5c=(if (v56!=0.0){(if v5a{v58}else{v59})}else{vd});let v5d=-1.0;let v5e=(v3d*sf[27]);let v5g=(vk*sf[28]);let v5h=(v5c*v5g);let v5j=(if (v56!=0.0){(v5e/v5h)}else{v3t});let v5l=(if (v5j>v3m){vc}else{vd});let v5m=((v56!=0.0)&&(v5l!=0.0));let v5s=((v56!=0.0)&&(!(v5l!=0.0)));let v5t=(if v5s{vc}else{(if v5m{(vc+(v5j-v3m))}else{v3z})});let v5u=((if v5m{v3m}else{v5j})).exp();let v5x=((if (v56!=0.0){(v5t*v5u)}else{v5t})-vc);let v60=(!(v56!=0.0));let v62=(v54-(if v60{vd}else{(if (v56!=0.0){(v11*v5x)}else{vd})}));let v6i=((vm*sf[34])).exp();let v6l=f64::powf((vc+f64::powf((((sf[23]*v32)/sf[29])).abs(),sf[30])),sf[35]);let v6m=((sf[33]*v6i)*v6l);let v6q=((vm*sf[37])).exp();let v6t=f64::powf((vc+f64::powf((((sf[23]*v35)/sf[31])).abs(),sf[32])),sf[38]);let v6u=((sf[36]*v6q)*v6t);let v70=(if (sf[40]!=0.0){(v6m+sf[41])}else{v6m});let v74=(v31-v34);let v7i=(sf[46]*(vc+((f64::powf((vc+f64::powf(((v74/sf[43])).abs(),sf[44])),sf[45])-vc)*sf[47])));let v7n=ctx.node_voltage(nodes[6]);let v7t=(vc+f64::powf(((v7n).abs()/sf[50]),sf[51]));let v7z=(v30+((-v2n)*sf[52]));let v81=(if (v7z>vd){vc}else{vd});let v87=(if (v81!=0.0){sf[57]}else{vd});let v8a=(vc-(sf[54]*(sf[54]*v87)));let v8h=(v7z*sf[59]);let v8j=(sf[54]+(v8h/v2n));let v8n=(!(v81!=0.0));let v8p=(vc-(v30/v2n));let v8s=((sf[58]*(v8p).ln())).exp();let v8t=(vc-v8s);let v8y=((if v8n{((v2n*v8t)/sf[58])}else{(if (v81!=0.0){((v2n*v8a)/sf[58])}else{vd})})+(if v8n{vd}else{(if (v81!=0.0){(v87*(v7z*v8j))}else{vd})}));let v9q=((if (sf[49]!=0.0){(v70/v7t)}else{v70})/sf[3]);
        let v9v=((if (sf[40]!=0.0){(v6u+sf[42])}else{v6u})/sf[3]);let va0=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v7n);let va5=(-((v62*v74)).abs());let vaa=(v1*sf[75]);let vab=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, vaa);let vag=ctx.node_voltage(nodes[5]);let vak=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, vaa);let vaq=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (vag*sf[76]));let vax=(v9q>sf[72]);let vay=(if vax{v9q}else{sf[72]});let vb1=(v9v>sf[72]);let vb2=(if vb1{v9v}else{sf[72]});let vbc=(if va{vd}else{(if v8{vc}else{vd})});let vbd=(vj*vbc);let vbe=(vbc/sf[5]);let vbf=(vbe/vl);let vbz=(vbc/v1i);let vco=((v27*(-(vbd+vbd)))+(v21*((v22*(vbz/v1k))+(v25*(((v1w*(((v1q*((v1n*vbc)+(vb*(v1m*vbc))))-(v1o*vbc))/(v1q*v1q)))-(v1t*(v1u*(vbc+vbc))))/(v1w*v1w))))));let vcq=((-vco)/sf[18]);let vcv=(v2b*v2b);let vd6=(vco+((v2b*vbz)+(v1k*vcq)));let vdq=(if (v38!=0.0){((-(v30*(sf[24]*vbd)))/(v3a*v3a))}else{vd});let vdr=(if (v38!=0.0){(sf[23]/v3a)}else{vd});let vds=(if (v38!=0.0){(sf[77]/v3a)}else{vd});let vdu=(sf[25]*vbd);let vdv=(v3g*(-(sf[13]*(sf[14]*vbe))));let vdy=(v3g*v3g);let ve2=(if (v38!=0.0){((vdv-(v3e*vdu))/vdy)}else{vd});let ve3=(if (v38!=0.0){(sf[77]/v3g)}else{vd});let ve4=(if (v38!=0.0){(sf[23]/v3g)}else{vd});let ve8=(if (v38!=0.0){((vdv-(v3j*vdu))/vdy)}else{vd});let vec=(if v3p{vd}else{vdq});let ved=(if v3p{vd}else{vdr});let vee=(if v3p{vd}else{vds});let vef=(if v3v{vd}else{(if v3p{vdq}else{vd})});let veg=(if v3v{vd}else{(if v3p{vdr}else{vd})});let veh=(if v3v{vd}else{(if v3p{vds}else{vd})});let veu=(if (v38!=0.0){((v3x*vef)+(v3w*(v3x*vec)))}else{vef});let vev=(if (v38!=0.0){((v3x*veg)+(v3w*(v3x*ved)))}else{veg});let vew=(if (v38!=0.0){((v3x*veh)+(v3w*(v3x*vee)))}else{veh});let vex=(v47*ve2);let vey=(v47*ve3);let vez=(v47*ve4);let vfc=(v4j*ve8);let vgc=(if v53{vd}else{(if (v38!=0.0){(((v4s*(sf[9]*(vx*((sf[6]*vbf)+(((vk*(sf[7]*vbe))-(vr*vbd))/(vk*vk))))))+(vy*veu))-(((v4z*((v4r*(sf[11]*(sf[12]*vbe)))+(v16*(if (v38!=0.0){((if v46{(vex/v48)}else{(if v4a{vex}else{(if v41{ve2}else{vd})})})-(if v4i{(vfc/v4k)}else{(if v4m{vfc}else{(if v4e{ve8}else{vd})})}))}else{vd}))))-(v4u*(sf[26]*((sf[15]*(sf[16]*vbe))*(v4x*(v4w).ln())))))/(v4z*v4z)))}else{vd})});let vgd=(if v53{vd}else{(if (v38!=0.0){((vy*vev)-((v16*(if (v38!=0.0){(if v46{(vey/v48)}else{(if v4a{vey}else{(if v41{ve3}else{vd})})})}else{vd}))/v4z))}else{vd})});let vge=(if v53{vd}else{(if (v38!=0.0){((vy*vew)-((v16*(if (v38!=0.0){(if v46{(vez/v48)}else{(if v4a{vez}else{(if v41{ve4}else{vd})})})}else{vd}))/v4z))}else{vd})});let vgr=(v5h*v5h);let vh1=(if (v56!=0.0){((-(v5e*(v5c*(sf[28]*vbd))))/vgr)}else{vec});let vh2=(if (v56!=0.0){(((v5h*sf[78])-(v5e*(v5g*(if (v56!=0.0){(if v5a{sf[77]}else{vd})}else{vd}))))/vgr)}else{ved});let vh3=(if (v56!=0.0){(((v5h*sf[79])-(v5e*(v5g*(if (v56!=0.0){(if v5a{sf[23]}else{vd})}else{vd}))))/vgr)}else{vee});let vha=(if v5s{vd}else{(if v5m{vh1}else{veu})});let vhb=(if v5s{vd}else{(if v5m{vh2}else{vev})});let vhc=(if v5s{vd}else{(if v5m{vh3}else{vew})});let vi9=(v6l*(sf[33]*(v6i*(sf[34]*vbf))));let vik=(sf[52]*(-vd6));let viu=(v2n*v2n);let vkv=ddt_scale;let vl0=(sf[75]*vkv);

        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * ((if (sf[49]!=0.0){(v7i*(-v54))}else{vd})),
            2,
            multiplicity * ((if (sf[49]!=0.0){(v7i*(-vgc))}else{vd})),
            3,
            multiplicity * ((if (sf[49]!=0.0){(v7i*(-vgd))}else{vd})),
            4,
            multiplicity * ((if (sf[49]!=0.0){(v7i*(-vge))}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if (sf[49]!=0.0){v7n}else{vd})),
            6,
            multiplicity * (sf[82]),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if (sf[49]!=0.0){(v7i*va0)}else{vd})),
            6,
            multiplicity * ((if (sf[49]!=0.0){(v7i*vkv)}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            vd,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if (sf[62]!=0.0){va5}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (sf[62]!=0.0){(v1/sf[61])}else{vd})),
            2,
            multiplicity * (sf[84]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (sf[62]!=0.0){vab}else{vd})),
            2,
            multiplicity * ((if (sf[62]!=0.0){vl0}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            vd,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if sb[17]{va5}else{vd})),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * ((if sb[17]{((v1-vag)/sf[61])}else{vd})),
            2,
            multiplicity * (sf[86]),
            5,
            multiplicity * (sf[87]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if sb[17]{vak}else{vd})),
            2,
            multiplicity * ((if sb[17]{vl0}else{vd})),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if sb[17]{(vag/sf[63])}else{vd})),
            5,
            multiplicity * (sf[89]),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if sb[17]{vaq}else{vd})),
            5,
            multiplicity * ((if sb[17]{(sf[76]*vkv)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if sb[20]{va5}else{vd})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            vd,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            vd,
        );
        stamper.stamp_current_node1_local(
            Some(3),
            Some(4),
            multiplicity * ((vd*v2z)),
            4,
            multiplicity * (-0.0),
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(3),
            multiplicity * ((if (sf[73]!=0.0){(v32/vay)}else{vd})),
            0,
            multiplicity * ((if (sf[73]!=0.0){(vc/vay)}else{vd})),
            2,
            multiplicity * ((if (sf[73]!=0.0){((-(v32*(if vax{((if (sf[49]!=0.0){(vi9/v7t)}else{vi9})/sf[3])}else{vd})))/(vay*vay))}else{vd})),
            3,
            multiplicity * ((if (sf[73]!=0.0){(v5d/vay)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (vd),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            vd,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * ((if (sf[74]!=0.0){(v35/vb2)}else{vd})),
            1,
            multiplicity * ((if (sf[74]!=0.0){(vc/vb2)}else{vd})),
            2,
            multiplicity * ((if (sf[74]!=0.0){((-(v35*(if vb1{((v6t*(sf[36]*(v6q*(sf[37]*vbf))))/sf[3])}else{vd})))/(vb2*vb2))}else{vd})),
            4,
            multiplicity * ((if (sf[74]!=0.0){(v5d/vb2)}else{vd})),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (vd),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            vd,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * ((sf[3]*(sf[23]*v62))),
            2,
            multiplicity * ((sf[3]*(sf[23]*(vgc-(if v60{vd}else{(if (v56!=0.0){((v5x*(sf[10]*(v10*(sf[8]*vbf))))+(v11*(if (v56!=0.0){((v5u*vha)+(v5t*(v5u*(if v5m{vd}else{vh1}))))}else{vha})))}else{vd})}))))),
            3,
            multiplicity * ((sf[3]*(sf[23]*(vgd-(if v60{vd}else{(if (v56!=0.0){(v11*(if (v56!=0.0){((v5u*vhb)+(v5t*(v5u*(if v5m{vd}else{vh2}))))}else{vhb}))}else{vd})}))))),
            4,
            multiplicity * ((sf[3]*(sf[23]*(vge-(if v60{vd}else{(if (v56!=0.0){(v11*(if (v56!=0.0){((v5u*vhc)+(v5t*(v5u*(if v5m{vd}else{vh3}))))}else{vhc}))}else{vd})}))))),
        );
        let vb8_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (sf[3]*(sf[23]*(v2v*v8y))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (vb8_ddt),
            2,
            multiplicity * ((((sf[3]*(sf[23]*((v8y*((v2u*((-(sf[17]*(sf[20]*(-(((v2b*(-vcq))-(v2c*vcq))/vcv)))))/(v2k*v2k)))+(v2l*(sf[20]*((v2f*vbc)-(((v2b*(vd6-vcq))-(v2o*vcq))/vcv))))))+(v2v*((if v8n{(((v8t*vd6)+(v2n*(-(v8s*(sf[58]*((-((-(v30*vd6))/viu))/v8p))))))/sf[58])}else{(if (v81!=0.0){((v8a*vd6)/sf[58])}else{vd})})+(if v8n{vd}else{(if (v81!=0.0){(v87*((v8j*vik)+(v7z*(((v2n*(sf[59]*vik))-(v8h*vd6))/viu))))}else{vd})}))))))) * ddt_scale)),
            3,
            multiplicity * ((((sf[3]*(sf[23]*(v2v*((if v8n{((v2n*(-(v8s*(sf[58]*((-(sf[23]/v2n))/v8p)))))/sf[58])}else{vd})+(if v8n{vd}else{(if (v81!=0.0){(v87*((sf[23]*v8j)+(v7z*(sf[80]/v2n))))}else{vd})})))))) * ddt_scale)),
            4,
            multiplicity * ((((sf[3]*(sf[23]*(v2v*((if v8n{((v2n*(-(v8s*(sf[58]*((-(sf[77]/v2n))/v8p)))))/sf[58])}else{vd})+(if v8n{vd}else{(if (v81!=0.0){(v87*((v8j*sf[77])+(v7z*(sf[81]/v2n))))}else{vd})})))))) * ddt_scale)),
        );
        let vba_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (sf[3]*(sf[23]*(v54*v7i))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (vba_ddt),
            2,
            multiplicity * ((((sf[3]*(sf[23]*(v7i*vgc)))) * ddt_scale)),
            3,
            multiplicity * ((((sf[3]*(sf[23]*(v7i*vgd)))) * ddt_scale)),
            4,
            multiplicity * ((((sf[3]*(sf[23]*(v7i*vge)))) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (vd),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
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
        let v1=ctx.node_voltage(nodes[2]);let v4=((ctx.temperature()+v1)+sf[0]);let v6=1300.0;let v7=173.14999999999998;let v8=(v4>v7);let v9=(if v8{v4}else{v7});let va=(v6<v9);let vb=(if va{v6}else{v9});let vc=1.0;let vd=0.0;let vj=8.6170869e-5;let vk=(vb*vj);let vl=(vb/sf[5]);let vq=(vl-vc);let vr=(sf[7]*vq);let vx=((((vl).ln()*sf[6])+(vr/vk))).exp();let vy=(sf[9]*vx);let v16=(sf[11]*(vc+(vq*sf[12])));let v1b=(sf[13]*(vc+(vq*sf[14])));let v1i=300.15;let v1k=(vb/v1i);let v1m=0.000702;let v1n=(vb*v1m);let v1o=(vb*v1n);let v1q=(vb+1108.0);let v1t=(-(1.16-(v1o/v1q)));let v1u=1.3806226e-23;let v1w=(v1u*(vb+vb));let v21=(-(vk+vk));let v22=1.5;let v25=1.6021918e-19;let v27=((v22*(v1k).ln())+(((v1t/v1w)+1.3454442398941469e20)*v25));let v28=(v21*v27);let v2b=((sf[19]-v28)/sf[18]);let v2c=(sf[19]-v2b);let v2f=0.0004;let v2k=(vc+(sf[20]*(sf[22]-(v2c/v2b))));let v2l=(sf[17]/v2k);let v2n=(v28+(v1k*v2b));let v2o=(v2n-v2b);let v2u=(vc+(sf[20]*((v2f*(vb-v1i))-(v2o/v2b))));let v2v=(v2l*v2u);let v30=(sf[23]*(ctx.node_voltage(nodes[3])-ctx.node_voltage(nodes[4])));let v38=(if (vy>vd){vc}else{vd});let v3a=(vk*sf[24]);let v3c=(if (v38!=0.0){(v30/v3a)}else{vd});let v3e=((-v30)-v1b);let v3g=(vk*sf[25]);let v3i=(if (v38!=0.0){(v3e/v3g)}else{vd});let v3j=(-v1b);let v3l=(if (v38!=0.0){(v3j/v3g)}else{vd});let v3m=80.0;let v3o=(if (v3c>v3m){vc}else{vd});let v3p=((v38!=0.0)&&(v3o!=0.0));let v3v=((v38!=0.0)&&(!(v3o!=0.0)));let v3w=(if v3v{vc}else{(if v3p{(vc+(v3c-v3m))}else{vd})});let v3x=((if v3p{v3m}else{v3c})).exp();let v40=37.0;let v41=(v3i>=v40);let v42=(!v41);let v43=-37.0;let v44=(v3i<=v43);let v46=(v42&&(!v44));let v47=(v3i).exp();let v48=(vc+v47);let v4a=(v42&&v44);let v4e=(v3l>=v40);let v4f=(!v4e);let v4g=(v3l<=v43);let v4i=(v4f&&(!v4g));let v4j=(v3l).exp();let v4k=(vc+v4j);let v4m=(v4f&&v4g);let v4r=(if (v38!=0.0){((if v46{(v48).ln()}else{(if v4a{v47}else{(if v41{v3i}else{vd})})})-(if v4i{(v4k).ln()}else{(if v4m{v4j}else{(if v4e{v3l}else{vd})})}))}else{vd});let v4s=((if (v38!=0.0){(v3w*v3x)}else{v3w})-vc);let v4u=(v16*v4r);let v4w=(v30).abs();let v4x=f64::powf(v4w,(sf[15]*(vc+(vq*sf[16]))));let v4z=(vc+(sf[26]*v4x));let v53=(!(v38!=0.0));let v7i=(sf[46]*(vc+((f64::powf((vc+f64::powf((((ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]))/sf[43])).abs(),sf[44])),sf[45])-vc)*sf[47])));let v7z=(v30+((-v2n)*sf[52]));let v81=(if (v7z>vd){vc}else{vd});let v87=(if (v81!=0.0){sf[57]}else{vd});let v8a=(vc-(sf[54]*(sf[54]*v87)));let v8h=(v7z*sf[59]);let v8j=(sf[54]+(v8h/v2n));let v8n=(!(v81!=0.0));let v8p=(vc-(v30/v2n));let v8s=((sf[58]*(v8p).ln())).exp();let v8t=(vc-v8s);let v8y=((if v8n{((v2n*v8t)/sf[58])}else{(if (v81!=0.0){((v2n*v8a)/sf[58])}else{vd})})+(if v8n{vd}else{(if (v81!=0.0){(v87*(v7z*v8j))}else{vd})}));let va0=0.0;let vaa=(v1*sf[75]);let vab=0.0;let vak=0.0;let vaq=0.0;let vbc=(if va{vd}else{(if v8{vc}else{vd})});let vbd=(vj*vbc);let vbe=(vbc/sf[5]);let vbz=(vbc/v1i);let vco=((v27*(-(vbd+vbd)))+(v21*((v22*(vbz/v1k))+(v25*(((v1w*(((v1q*((v1n*vbc)+(vb*(v1m*vbc))))-(v1o*vbc))/(v1q*v1q)))-(v1t*(v1u*(vbc+vbc))))/(v1w*v1w))))));let vcq=((-vco)/sf[18]);let vcv=(v2b*v2b);let vd6=(vco+((v2b*vbz)+(v1k*vcq)));let vdq=(if (v38!=0.0){((-(v30*(sf[24]*vbd)))/(v3a*v3a))}else{vd});let vdr=(if (v38!=0.0){(sf[23]/v3a)}else{vd});let vds=(if (v38!=0.0){(sf[77]/v3a)}else{vd});let vdu=(sf[25]*vbd);let vdv=(v3g*(-(sf[13]*(sf[14]*vbe))));let vdy=(v3g*v3g);let ve2=(if (v38!=0.0){((vdv-(v3e*vdu))/vdy)}else{vd});let ve3=(if (v38!=0.0){(sf[77]/v3g)}else{vd});let ve4=(if (v38!=0.0){(sf[23]/v3g)}else{vd});let ve8=(if (v38!=0.0){((vdv-(v3j*vdu))/vdy)}else{vd});let vef=(if v3v{vd}else{(if v3p{vdq}else{vd})});let veg=(if v3v{vd}else{(if v3p{vdr}else{vd})});let veh=(if v3v{vd}else{(if v3p{vds}else{vd})});let vex=(v47*ve2);let vey=(v47*ve3);let vez=(v47*ve4);let vfc=(v4j*ve8);let vik=(sf[52]*(-vd6));let viu=(v2n*v2n);let vkv=1.0;let vl0=(sf[75]*vkv);

        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * ((if (sf[49]!=0.0){(v7i*vkv)}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if (sf[62]!=0.0){vl0}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if sb[17]{vl0}else{vd})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * ((if sb[17]{(sf[76]*vkv)}else{vd})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * ((sf[3]*(sf[23]*((v8y*((v2u*((-(sf[17]*(sf[20]*(-(((v2b*(-vcq))-(v2c*vcq))/vcv)))))/(v2k*v2k)))+(v2l*(sf[20]*((v2f*vbc)-(((v2b*(vd6-vcq))-(v2o*vcq))/vcv))))))+(v2v*((if v8n{(((v8t*vd6)+(v2n*(-(v8s*(sf[58]*((-((-(v30*vd6))/viu))/v8p))))))/sf[58])}else{(if (v81!=0.0){((v8a*vd6)/sf[58])}else{vd})})+(if v8n{vd}else{(if (v81!=0.0){(v87*((v8j*vik)+(v7z*(((v2n*(sf[59]*vik))-(v8h*vd6))/viu))))}else{vd})}))))))),
            nodes[3],
            multiplicity * ((sf[3]*(sf[23]*(v2v*((if v8n{((v2n*(-(v8s*(sf[58]*((-(sf[23]/v2n))/v8p)))))/sf[58])}else{vd})+(if v8n{vd}else{(if (v81!=0.0){(v87*((sf[23]*v8j)+(v7z*(sf[80]/v2n))))}else{vd})})))))),
            nodes[4],
            multiplicity * ((sf[3]*(sf[23]*(v2v*((if v8n{((v2n*(-(v8s*(sf[58]*((-(sf[77]/v2n))/v8p)))))/sf[58])}else{vd})+(if v8n{vd}else{(if (v81!=0.0){(v87*((v8j*sf[77])+(v7z*(sf[81]/v2n))))}else{vd})})))))),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * ((sf[3]*(sf[23]*(v7i*(if v53{vd}else{(if (v38!=0.0){(((v4s*(sf[9]*(vx*((sf[6]*(vbe/vl))+(((vk*(sf[7]*vbe))-(vr*vbd))/(vk*vk))))))+(vy*(if (v38!=0.0){((v3x*vef)+(v3w*(v3x*(if v3p{vd}else{vdq}))))}else{vef})))-(((v4z*((v4r*(sf[11]*(sf[12]*vbe)))+(v16*(if (v38!=0.0){((if v46{(vex/v48)}else{(if v4a{vex}else{(if v41{ve2}else{vd})})})-(if v4i{(vfc/v4k)}else{(if v4m{vfc}else{(if v4e{ve8}else{vd})})}))}else{vd}))))-(v4u*(sf[26]*((sf[15]*(sf[16]*vbe))*(v4x*(v4w).ln())))))/(v4z*v4z)))}else{vd})}))))),
            nodes[3],
            multiplicity * ((sf[3]*(sf[23]*(v7i*(if v53{vd}else{(if (v38!=0.0){((vy*(if (v38!=0.0){((v3x*veg)+(v3w*(v3x*(if v3p{vd}else{vdr}))))}else{veg}))-((v16*(if (v38!=0.0){(if v46{(vey/v48)}else{(if v4a{vey}else{(if v41{ve3}else{vd})})})}else{vd}))/v4z))}else{vd})}))))),
            nodes[4],
            multiplicity * ((sf[3]*(sf[23]*(v7i*(if v53{vd}else{(if (v38!=0.0){((vy*(if (v38!=0.0){((v3x*veh)+(v3w*(v3x*(if v3p{vd}else{vds}))))}else{veh}))-((v16*(if (v38!=0.0){(if v46{(vez/v48)}else{(if v4a{vez}else{(if v41{ve4}else{vd})})})}else{vd}))/v4z))}else{vd})}))))),
        );
    }
}
