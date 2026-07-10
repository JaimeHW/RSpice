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

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
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
        let b=ctx.node_voltage(n[2]);let e=((ctx.temperature()+b)+sf[0]);let g=1300.0;let h=173.14999999999998;let i=(e>h);let j=(if i{e}else{h});let k=(g<j);let l=(if k{g}else{j});let m_=1.0;let n_=0.0;let t=8.6170869e-5;let u=(l*t);let v=(l/sf[5]);let w=(v).ln();let A=(v-m_);let B=(sf[7]*A);let H=(((w*sf[6])+(B/u))).exp();let I=(sf[9]*H);let K=((w*sf[8])).exp();let L=(sf[10]*K);let Q=(sf[11]*(m_+(A*sf[12])));let V=(sf[13]*(m_+(A*sf[14])));let a2=300.15;let a4=(l/a2);let a6=0.000702;let a7=(l*a6);let a8=(l*a7);let aa=(l+1108.0);let ad=(-(1.16-(a8/aa)));let ae=1.3806226e-23;let ag=(ae*(l+l));let al=(-(u+u));let am=1.5;let ap=1.6021918e-19;let ar=((am*(a4).ln())+(((ad/ag)+1.3454442398941469e20)*ap));let as_=(al*ar);let av=((sf[19]-as_)/sf[18]);let aw=(sf[19]-av);let az=0.0004;let aE=(m_+(sf[20]*(sf[22]-(aw/av))));let aF=(sf[17]/aE);let aH=(as_+(a4*av));let aI=(aH-av);let aO=(m_+(sf[20]*((az*(l-a2))-(aI/av))));let aP=(aF*aO);let aR=ctx.node_voltage(n[3]);let aS=ctx.node_voltage(n[4]);let aT=(aR-aS);let aU=(sf[23]*aT);let aV=ctx.node_voltage(n[0]);let aW=(aV-aR);let aY=ctx.node_voltage(n[1]);let aZ=(aY-aS);let b2=(if (I>n_){m_}else{n_});let b4=(u*sf[24]);let b6=(if ((b2)!=0.0){(aU/b4)}else{n_});let b7=(-aU);let b8=(b7-V);let ba=(u*sf[25]);let bc=(if ((b2)!=0.0){(b8/ba)}else{n_});let bd=(-V);let bf=(if ((b2)!=0.0){(bd/ba)}else{n_});let bg=80.0;let bi=(if (b6>bg){m_}else{n_});let bj=(((b2)!=0.0)&&((bi)!=0.0));let bn=(if bj{bg}else{b6});let bp=(((b2)!=0.0)&&(!((bi)!=0.0)));let bq=(if bp{m_}else{(if bj{(m_+(b6-bg))}else{n_})});let br_=(bn).exp();let bt=(if ((b2)!=0.0){(bq*br_)}else{bq});let bu=37.0;let bv=(bc>=bu);let bw=(!bv);let bx=-37.0;let by=(bc<=bx);let bA=(bw&&(!by));let bB=(bc).exp();let bC=(m_+bB);let bE=(bw&&by);let bI=(bf>=bu);let bJ=(!bI);let bK=(bf<=bx);let bM=(bJ&&(!bK));let bN=(bf).exp();let bO=(m_+bN);let bQ=(bJ&&bK);let bV=(if ((b2)!=0.0){((if bA{(bC).ln()}else{(if bE{bB}else{(if bv{bc}else{n_})})})-(if bM{(bO).ln()}else{(if bQ{bN}else{(if bI{bf}else{n_})})}))}else{n_});let bW=(bt-m_);let bY=(Q*bV);let c0=(aU).abs();let c1=f64::powf(c0,(sf[15]*(m_+(A*sf[16]))));let c3=(m_+(sf[26]*c1));let c7=(!((b2)!=0.0));let c8=(if c7{n_}else{(if ((b2)!=0.0){((I*bW)-(bY/c3))}else{n_})});let ca=(if (L>n_){m_}else{n_});let cc=(sf[27]-aU);let cd=0.001;let ce=(cc>cd);let cg=(if ((ca)!=0.0){(if ce{cc}else{cd})}else{n_});let ch=-1.0;let ci=(b7*sf[27]);let ck=(u*sf[28]);let cl=(cg*ck);let cn=(if ((ca)!=0.0){(ci/cl)}else{bn});let cp=(if (cn>bg){m_}else{n_});let cq=(((ca)!=0.0)&&((cp)!=0.0));let cw=(((ca)!=0.0)&&(!((cp)!=0.0)));let cx=(if cw{m_}else{(if cq{(m_+(cn-bg))}else{bt})});let cy=((if cq{bg}else{cn})).exp();let cB=((if ((ca)!=0.0){(cx*cy)}else{cx})-m_);let cE=(!((ca)!=0.0));let cG=(c8-(if cE{n_}else{(if ((ca)!=0.0){(L*cB)}else{n_})}));let cW=((w*sf[34])).exp();let cZ=f64::powf((m_+f64::powf((((sf[23]*aW)/sf[29])).abs(),sf[30])),sf[35]);let d0=((sf[33]*cW)*cZ);let d4=((w*sf[37])).exp();let d7=f64::powf((m_+f64::powf((((sf[23]*aZ)/sf[31])).abs(),sf[32])),sf[38]);let d8=((sf[36]*d4)*d7);let de=(if ((sf[40])!=0.0){(d0+sf[41])}else{d0});let di=(aV-aY);let dw=(sf[46]*(m_+((f64::powf((m_+f64::powf(((di/sf[43])).abs(),sf[44])),sf[45])-m_)*sf[47])));let dB=ctx.node_voltage(n[6]);let dH=(m_+f64::powf(((dB).abs()/sf[50]),sf[51]));let dN=(aU+((-aH)*sf[52]));let dP=(if (dN>n_){m_}else{n_});let dV=(if ((dP)!=0.0){sf[57]}else{n_});let dY=(m_-(sf[54]*(sf[54]*dV)));let e5=(dN*sf[59]);let e7=(sf[54]+(e5/aH));let eb=(!((dP)!=0.0));let ed=(m_-(aU/aH));let eg=((sf[58]*(ed).ln())).exp();let eh=(m_-eg);let em=((if eb{((aH*eh)/sf[58])}else{(if ((dP)!=0.0){((aH*dY)/sf[58])}else{n_})})+(if eb{n_}else{(if ((dP)!=0.0){(dV*(dN*e7))}else{n_})}));let eO=((if ((sf[49])!=0.0){(de/dH)}else{de})/sf[3]);let eT=((if ((sf[40])!=0.0){(d8+sf[42])}else{d8})/sf[3]);let eY=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, dB);let f3=(-((cG*di)).abs());let f8=(b*sf[75]);
        let f9=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, f8);let fe=ctx.node_voltage(n[5]);let fi=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, f8);let fo=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (fe*sf[76]));let fv=(eO>sf[72]);let fw=(if fv{eO}else{sf[72]});let fz=(eT>sf[72]);let fA=(if fz{eT}else{sf[72]});let fK=(if k{n_}else{(if i{m_}else{n_})});let fL=(t*fK);let fM=(fK/sf[5]);let fN=(fM/v);let g7=(fK/a2);let gw=((ar*(-(fL+fL)))+(al*((am*(g7/a4))+(ap*(((ag*(((aa*((a7*fK)+(l*(a6*fK))))-(a8*fK))/(aa*aa)))-(ad*(ae*(fK+fK))))/(ag*ag))))));let gy=((-gw)/sf[18]);let gD=(av*av);let gO=(gw+((av*g7)+(a4*gy)));let h8=(if ((b2)!=0.0){((-(aU*(sf[24]*fL)))/(b4*b4))}else{n_});let h9=(if ((b2)!=0.0){(sf[23]/b4)}else{n_});let ha=(if ((b2)!=0.0){(sf[77]/b4)}else{n_});let hc=(sf[25]*fL);let hd=(ba*(-(sf[13]*(sf[14]*fM))));let hg=(ba*ba);let hk=(if ((b2)!=0.0){((hd-(b8*hc))/hg)}else{n_});let hl=(if ((b2)!=0.0){(sf[77]/ba)}else{n_});let hm=(if ((b2)!=0.0){(sf[23]/ba)}else{n_});let hq=(if ((b2)!=0.0){((hd-(bd*hc))/hg)}else{n_});let hu=(if bj{n_}else{h8});let hv=(if bj{n_}else{h9});let hw=(if bj{n_}else{ha});let hx=(if bp{n_}else{(if bj{h8}else{n_})});let hy=(if bp{n_}else{(if bj{h9}else{n_})});let hz=(if bp{n_}else{(if bj{ha}else{n_})});let hM=(if ((b2)!=0.0){((br_*hx)+(bq*(br_*hu)))}else{hx});let hN=(if ((b2)!=0.0){((br_*hy)+(bq*(br_*hv)))}else{hy});let hO=(if ((b2)!=0.0){((br_*hz)+(bq*(br_*hw)))}else{hz});let hP=(bB*hk);let hQ=(bB*hl);let hR=(bB*hm);let i4=(bN*hq);let iE=(if c7{n_}else{(if ((b2)!=0.0){(((bW*(sf[9]*(H*((sf[6]*fN)+(((u*(sf[7]*fM))-(B*fL))/(u*u))))))+(I*hM))-(((c3*((bV*(sf[11]*(sf[12]*fM)))+(Q*(if ((b2)!=0.0){((if bA{(hP/bC)}else{(if bE{hP}else{(if bv{hk}else{n_})})})-(if bM{(i4/bO)}else{(if bQ{i4}else{(if bI{hq}else{n_})})}))}else{n_}))))-(bY*(sf[26]*((sf[15]*(sf[16]*fM))*(c1*(c0).ln())))))/(c3*c3)))}else{n_})});let iF=(if c7{n_}else{(if ((b2)!=0.0){((I*hN)-((Q*(if ((b2)!=0.0){(if bA{(hQ/bC)}else{(if bE{hQ}else{(if bv{hl}else{n_})})})}else{n_}))/c3))}else{n_})});let iG=(if c7{n_}else{(if ((b2)!=0.0){((I*hO)-((Q*(if ((b2)!=0.0){(if bA{(hR/bC)}else{(if bE{hR}else{(if bv{hm}else{n_})})})}else{n_}))/c3))}else{n_})});let iT=(cl*cl);let j3=(if ((ca)!=0.0){((-(ci*(cg*(sf[28]*fL))))/iT)}else{hu});let j4=(if ((ca)!=0.0){(((cl*sf[78])-(ci*(ck*(if ((ca)!=0.0){(if ce{sf[77]}else{n_})}else{n_}))))/iT)}else{hv});let j5=(if ((ca)!=0.0){(((cl*sf[79])-(ci*(ck*(if ((ca)!=0.0){(if ce{sf[23]}else{n_})}else{n_}))))/iT)}else{hw});let jc=(if cw{n_}else{(if cq{j3}else{hM})});let jd=(if cw{n_}else{(if cq{j4}else{hN})});let je=(if cw{n_}else{(if cq{j5}else{hO})});let jL=(cZ*(sf[33]*(cW*(sf[34]*fN))));let jW=(sf[52]*(-gO));let k6=(aH*aH);let lh=ddt_scale;let lm=(sf[75]*lh);

        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * ((if ((sf[49])!=0.0){(dw*(-c8))}else{n_})),
            2,
            multiplicity * ((if ((sf[49])!=0.0){(dw*(-iE))}else{n_})),
            3,
            multiplicity * ((if ((sf[49])!=0.0){(dw*(-iF))}else{n_})),
            4,
            multiplicity * ((if ((sf[49])!=0.0){(dw*(-iG))}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if ((sf[49])!=0.0){dB}else{n_})),
            6,
            multiplicity * (sf[82]),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if ((sf[49])!=0.0){(dw*eY)}else{n_})),
            6,
            multiplicity * ((if ((sf[49])!=0.0){(dw*lh)}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            n_,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[62])!=0.0){f3}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[62])!=0.0){(b/sf[61])}else{n_})),
            2,
            multiplicity * (sf[84]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if ((sf[62])!=0.0){f9}else{n_})),
            2,
            multiplicity * ((if ((sf[62])!=0.0){lm}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            n_,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if sb[17]{f3}else{n_})),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * ((if sb[17]{((b-fe)/sf[61])}else{n_})),
            2,
            multiplicity * (sf[86]),
            5,
            multiplicity * (sf[87]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if sb[17]{fi}else{n_})),
            2,
            multiplicity * ((if sb[17]{lm}else{n_})),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if sb[17]{(fe/sf[63])}else{n_})),
            5,
            multiplicity * (sf[89]),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if sb[17]{fo}else{n_})),
            5,
            multiplicity * ((if sb[17]{(sf[76]*lh)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if sb[20]{f3}else{n_})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            n_,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            n_,
        );
        stamper.stamp_current_node1_local(
            Some(3),
            Some(4),
            multiplicity * ((n_*aT)),
            4,
            multiplicity * (-0.0),
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(3),
            multiplicity * ((if ((sf[73])!=0.0){(aW/fw)}else{n_})),
            0,
            multiplicity * ((if ((sf[73])!=0.0){(m_/fw)}else{n_})),
            2,
            multiplicity * ((if ((sf[73])!=0.0){((-(aW*(if fv{((if ((sf[49])!=0.0){(jL/dH)}else{jL})/sf[3])}else{n_})))/(fw*fw))}else{n_})),
            3,
            multiplicity * ((if ((sf[73])!=0.0){(ch/fw)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (n_),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            n_,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * ((if ((sf[74])!=0.0){(aZ/fA)}else{n_})),
            1,
            multiplicity * ((if ((sf[74])!=0.0){(m_/fA)}else{n_})),
            2,
            multiplicity * ((if ((sf[74])!=0.0){((-(aZ*(if fz{((d7*(sf[36]*(d4*(sf[37]*fN))))/sf[3])}else{n_})))/(fA*fA))}else{n_})),
            4,
            multiplicity * ((if ((sf[74])!=0.0){(ch/fA)}else{n_})),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (n_),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            n_,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * ((sf[3]*(sf[23]*cG))),
            2,
            multiplicity * ((sf[3]*(sf[23]*(iE-(if cE{n_}else{(if ((ca)!=0.0){((cB*(sf[10]*(K*(sf[8]*fN))))+(L*(if ((ca)!=0.0){((cy*jc)+(cx*(cy*(if cq{n_}else{j3}))))}else{jc})))}else{n_})}))))),
            3,
            multiplicity * ((sf[3]*(sf[23]*(iF-(if cE{n_}else{(if ((ca)!=0.0){(L*(if ((ca)!=0.0){((cy*jd)+(cx*(cy*(if cq{n_}else{j4}))))}else{jd}))}else{n_})}))))),
            4,
            multiplicity * ((sf[3]*(sf[23]*(iG-(if cE{n_}else{(if ((ca)!=0.0){(L*(if ((ca)!=0.0){((cy*je)+(cx*(cy*(if cq{n_}else{j5}))))}else{je}))}else{n_})}))))),
        );
        let fG_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (sf[3]*(sf[23]*(aP*em))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (fG_ddt),
            2,
            multiplicity * ((((sf[3]*(sf[23]*((em*((aO*((-(sf[17]*(sf[20]*(-(((av*(-gy))-(aw*gy))/gD)))))/(aE*aE)))+(aF*(sf[20]*((az*fK)-(((av*(gO-gy))-(aI*gy))/gD))))))+(aP*((if eb{(((eh*gO)+(aH*(-(eg*(sf[58]*((-((-(aU*gO))/k6))/ed))))))/sf[58])}else{(if ((dP)!=0.0){((dY*gO)/sf[58])}else{n_})})+(if eb{n_}else{(if ((dP)!=0.0){(dV*((e7*jW)+(dN*(((aH*(sf[59]*jW))-(e5*gO))/k6))))}else{n_})}))))))) * ddt_scale)),
            3,
            multiplicity * ((((sf[3]*(sf[23]*(aP*((if eb{((aH*(-(eg*(sf[58]*((-(sf[23]/aH))/ed)))))/sf[58])}else{n_})+(if eb{n_}else{(if ((dP)!=0.0){(dV*((sf[23]*e7)+(dN*(sf[80]/aH))))}else{n_})})))))) * ddt_scale)),
            4,
            multiplicity * ((((sf[3]*(sf[23]*(aP*((if eb{((aH*(-(eg*(sf[58]*((-(sf[77]/aH))/ed)))))/sf[58])}else{n_})+(if eb{n_}else{(if ((dP)!=0.0){(dV*((e7*sf[77])+(dN*(sf[81]/aH))))}else{n_})})))))) * ddt_scale)),
        );
        let fI_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (sf[3]*(sf[23]*(c8*dw))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (fI_ddt),
            2,
            multiplicity * ((((sf[3]*(sf[23]*(dw*iE)))) * ddt_scale)),
            3,
            multiplicity * ((((sf[3]*(sf[23]*(dw*iF)))) * ddt_scale)),
            4,
            multiplicity * ((((sf[3]*(sf[23]*(dw*iG)))) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (n_),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (n_),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        let br=self.branches;
        let branches=br;
        let p=&(*self.params);
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let b=ctx.node_voltage(n[2]);let e=((ctx.temperature()+b)+sf[0]);let g=1300.0;let h=173.14999999999998;let i=(e>h);let j=(if i{e}else{h});let k=(g<j);let l=(if k{g}else{j});let m_=1.0;let n_=0.0;let t=8.6170869e-5;let u=(l*t);let v=(l/sf[5]);let A=(v-m_);let B=(sf[7]*A);let H=((((v).ln()*sf[6])+(B/u))).exp();let I=(sf[9]*H);let Q=(sf[11]*(m_+(A*sf[12])));let V=(sf[13]*(m_+(A*sf[14])));let a2=300.15;let a4=(l/a2);let a6=0.000702;let a7=(l*a6);let a8=(l*a7);let aa=(l+1108.0);let ad=(-(1.16-(a8/aa)));let ae=1.3806226e-23;let ag=(ae*(l+l));let al=(-(u+u));let am=1.5;let ap=1.6021918e-19;let ar=((am*(a4).ln())+(((ad/ag)+1.3454442398941469e20)*ap));let as_=(al*ar);let av=((sf[19]-as_)/sf[18]);let aw=(sf[19]-av);let az=0.0004;let aE=(m_+(sf[20]*(sf[22]-(aw/av))));let aF=(sf[17]/aE);let aH=(as_+(a4*av));let aI=(aH-av);let aO=(m_+(sf[20]*((az*(l-a2))-(aI/av))));let aP=(aF*aO);let aU=(sf[23]*(ctx.node_voltage(n[3])-ctx.node_voltage(n[4])));let b2=(if (I>n_){m_}else{n_});let b4=(u*sf[24]);let b6=(if ((b2)!=0.0){(aU/b4)}else{n_});let b8=((-aU)-V);let ba=(u*sf[25]);let bc=(if ((b2)!=0.0){(b8/ba)}else{n_});let bd=(-V);let bf=(if ((b2)!=0.0){(bd/ba)}else{n_});let bg=80.0;let bi=(if (b6>bg){m_}else{n_});let bj=(((b2)!=0.0)&&((bi)!=0.0));let bp=(((b2)!=0.0)&&(!((bi)!=0.0)));let bq=(if bp{m_}else{(if bj{(m_+(b6-bg))}else{n_})});let br_=((if bj{bg}else{b6})).exp();let bu=37.0;let bv=(bc>=bu);let bw=(!bv);let bx=-37.0;let by=(bc<=bx);let bA=(bw&&(!by));let bB=(bc).exp();let bC=(m_+bB);let bE=(bw&&by);let bI=(bf>=bu);let bJ=(!bI);let bK=(bf<=bx);let bM=(bJ&&(!bK));let bN=(bf).exp();let bO=(m_+bN);let bQ=(bJ&&bK);let bV=(if ((b2)!=0.0){((if bA{(bC).ln()}else{(if bE{bB}else{(if bv{bc}else{n_})})})-(if bM{(bO).ln()}else{(if bQ{bN}else{(if bI{bf}else{n_})})}))}else{n_});let bW=((if ((b2)!=0.0){(bq*br_)}else{bq})-m_);let bY=(Q*bV);let c0=(aU).abs();let c1=f64::powf(c0,(sf[15]*(m_+(A*sf[16]))));let c3=(m_+(sf[26]*c1));let c7=(!((b2)!=0.0));let dw=(sf[46]*(m_+((f64::powf((m_+f64::powf((((ctx.node_voltage(n[0])-ctx.node_voltage(n[1]))/sf[43])).abs(),sf[44])),sf[45])-m_)*sf[47])));let dN=(aU+((-aH)*sf[52]));let dP=(if (dN>n_){m_}else{n_});let dV=(if ((dP)!=0.0){sf[57]}else{n_});let dY=(m_-(sf[54]*(sf[54]*dV)));let e5=(dN*sf[59]);let e7=(sf[54]+(e5/aH));let eb=(!((dP)!=0.0));let ed=(m_-(aU/aH));let eg=((sf[58]*(ed).ln())).exp();let eh=(m_-eg);let em=((if eb{((aH*eh)/sf[58])}else{(if ((dP)!=0.0){((aH*dY)/sf[58])}else{n_})})+(if eb{n_}else{(if ((dP)!=0.0){(dV*(dN*e7))}else{n_})}));let eY=0.0;let f8=(b*sf[75]);let f9=0.0;let fi=0.0;let fo=0.0;let fK=(if k{n_}else{(if i{m_}else{n_})});let fL=(t*fK);let fM=(fK/sf[5]);let g7=(fK/a2);let gw=((ar*(-(fL+fL)))+(al*((am*(g7/a4))+(ap*(((ag*(((aa*((a7*fK)+(l*(a6*fK))))-(a8*fK))/(aa*aa)))-(ad*(ae*(fK+fK))))/(ag*ag))))));let gy=((-gw)/sf[18]);let gD=(av*av);let gO=(gw+((av*g7)+(a4*gy)));let h8=(if ((b2)!=0.0){((-(aU*(sf[24]*fL)))/(b4*b4))}else{n_});let h9=(if ((b2)!=0.0){(sf[23]/b4)}else{n_});let ha=(if ((b2)!=0.0){(sf[77]/b4)}else{n_});let hc=(sf[25]*fL);let hd=(ba*(-(sf[13]*(sf[14]*fM))));let hg=(ba*ba);let hk=(if ((b2)!=0.0){((hd-(b8*hc))/hg)}else{n_});let hl=(if ((b2)!=0.0){(sf[77]/ba)}else{n_});let hm=(if ((b2)!=0.0){(sf[23]/ba)}else{n_});let hq=(if ((b2)!=0.0){((hd-(bd*hc))/hg)}else{n_});let hx=(if bp{n_}else{(if bj{h8}else{n_})});let hy=(if bp{n_}else{(if bj{h9}else{n_})});let hz=(if bp{n_}else{(if bj{ha}else{n_})});let hP=(bB*hk);let hQ=(bB*hl);let hR=(bB*hm);let i4=(bN*hq);let jW=(sf[52]*(-gO));let k6=(aH*aH);let lh=1.0;let lm=(sf[75]*lh);

        stamper.stamp_current_reactive_node1_local(
            Some(6),
            None,
            6,
            multiplicity * ((if ((sf[49])!=0.0){(dw*lh)}else{n_})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(2),
            None,
            2,
            multiplicity * ((if ((sf[62])!=0.0){lm}else{n_})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(2),
            None,
            2,
            multiplicity * ((if sb[17]{lm}else{n_})),
        );
        stamper.stamp_current_reactive_node1_local(
            Some(5),
            None,
            5,
            multiplicity * ((if sb[17]{(sf[76]*lh)}else{n_})),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(3),
            Some(4),
            2,
            multiplicity * ((sf[3]*(sf[23]*((em*((aO*((-(sf[17]*(sf[20]*(-(((av*(-gy))-(aw*gy))/gD)))))/(aE*aE)))+(aF*(sf[20]*((az*fK)-(((av*(gO-gy))-(aI*gy))/gD))))))+(aP*((if eb{(((eh*gO)+(aH*(-(eg*(sf[58]*((-((-(aU*gO))/k6))/ed))))))/sf[58])}else{(if ((dP)!=0.0){((dY*gO)/sf[58])}else{n_})})+(if eb{n_}else{(if ((dP)!=0.0){(dV*((e7*jW)+(dN*(((aH*(sf[59]*jW))-(e5*gO))/k6))))}else{n_})}))))))),
            3,
            multiplicity * ((sf[3]*(sf[23]*(aP*((if eb{((aH*(-(eg*(sf[58]*((-(sf[23]/aH))/ed)))))/sf[58])}else{n_})+(if eb{n_}else{(if ((dP)!=0.0){(dV*((sf[23]*e7)+(dN*(sf[80]/aH))))}else{n_})})))))),
            4,
            multiplicity * ((sf[3]*(sf[23]*(aP*((if eb{((aH*(-(eg*(sf[58]*((-(sf[77]/aH))/ed)))))/sf[58])}else{n_})+(if eb{n_}else{(if ((dP)!=0.0){(dV*((e7*sf[77])+(dN*(sf[81]/aH))))}else{n_})})))))),
        );
        stamper.stamp_current_reactive_node3_local(
            Some(3),
            Some(4),
            2,
            multiplicity * ((sf[3]*(sf[23]*(dw*(if c7{n_}else{(if ((b2)!=0.0){(((bW*(sf[9]*(H*((sf[6]*(fM/v))+(((u*(sf[7]*fM))-(B*fL))/(u*u))))))+(I*(if ((b2)!=0.0){((br_*hx)+(bq*(br_*(if bj{n_}else{h8}))))}else{hx})))-(((c3*((bV*(sf[11]*(sf[12]*fM)))+(Q*(if ((b2)!=0.0){((if bA{(hP/bC)}else{(if bE{hP}else{(if bv{hk}else{n_})})})-(if bM{(i4/bO)}else{(if bQ{i4}else{(if bI{hq}else{n_})})}))}else{n_}))))-(bY*(sf[26]*((sf[15]*(sf[16]*fM))*(c1*(c0).ln())))))/(c3*c3)))}else{n_})}))))),
            3,
            multiplicity * ((sf[3]*(sf[23]*(dw*(if c7{n_}else{(if ((b2)!=0.0){((I*(if ((b2)!=0.0){((br_*hy)+(bq*(br_*(if bj{n_}else{h9}))))}else{hy}))-((Q*(if ((b2)!=0.0){(if bA{(hQ/bC)}else{(if bE{hQ}else{(if bv{hl}else{n_})})})}else{n_}))/c3))}else{n_})}))))),
            4,
            multiplicity * ((sf[3]*(sf[23]*(dw*(if c7{n_}else{(if ((b2)!=0.0){((I*(if ((b2)!=0.0){((br_*hz)+(bq*(br_*(if bj{n_}else{ha}))))}else{hz}))-((Q*(if ((b2)!=0.0){(if bA{(hR/bC)}else{(if bE{hR}else{(if bv{hm}else{n_})})})}else{n_}))/c3))}else{n_})}))))),
        );
    }
}
