#![allow(dead_code, non_snake_case, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let n=self.nodes;
        let nodes=n;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let m=self.multiplicity;
        let multiplicity=m;
        let sf=&self.scalar_static_f64;
        let sb=&self.scalar_static_bool;
        let a=1.0;let b=0.0;let g=ctx.simparam_or("scale", a);let j=0.01;let p_=ctx.simparam_or("shrink", b);let v=(((if sb[0]{g}else{sf[2]})*(if sb[1]{(a-(j*p_))}else{sf[7]}))*1000000.0);let au=(v*sf[31]);let av=(if sb[14]{au}else{b});let az=1e99;let aC=(v*sf[29]);let aD=(if sb[16]{aC}else{b});let aF=(if sb[16]{(sf[23]+aD)}else{b});let aH=(if (aF>b){a}else{b});let aI=(sb[16]&&((aH)!=0.0));let aM=(if aI{(aF*sf[34])}else{(if sb[14]{(av+sf[32])}else{b})});let aR=(sb[16]&&(!((aH)!=0.0)));let aS=(if aR{au}else{(if aI{(aM-sf[32])}else{av})});let b5=(if sb[21]{au}else{aS});let bg=(if sb[25]{aC}else{(if sb[21]{b}else{aD})});let bm=(if sb[27]{au}else{(if sb[25]{b}else{b5})});let bo=(if sb[27]{(sf[32]+bm)}else{(if sb[25]{b}else{(if sb[21]{(sf[32]+b5)}else{(if aR{(sf[32]+aS)}else{aM})})})});let bq=(if (bo>b){a}else{b});let br_=(sb[27]&&((bq)!=0.0));let bu=(if br_{(bo*sf[38])}else{(if sb[25]{(sf[23]+bg)}else{(if sb[21]{b}else{aF})})});let bz=(sb[27]&&(!((bq)!=0.0)));let bA=(if bz{aC}else{(if br_{(bu-sf[23])}else{bg})});let bJ=(if sb[30]{aC}else{bA});let bS=(if sb[32]{au}else{(if sb[30]{b}else{bm})});let c0=(if sb[34]{(sf[32]+(if sb[34]{au}else{bS}))}else{(if sb[32]{(sf[32]+bS)}else{(if sb[30]{b}else{bo})})});let c1=(if sb[34]{aC}else{(if sb[32]{b}else{bJ})});let c3=(if sb[34]{(sf[23]+c1)}else{(if sb[32]{b}else{(if sb[30]{(sf[23]+bJ)}else{(if bz{(sf[23]+bA)}else{bu})})})});let c5=(if (c0>b){a}else{b});let c7=(if (c3>b){a}else{b});let c8=(sb[34]&&((c5)!=0.0));let ci=(if (sb[34]&&(!((c5)!=0.0))){az}else{(if (c8&&(!((c7)!=0.0))){b}else{(if (((c7)!=0.0)&&c8){(sf[33]*(c3/c0))}else{(if sb[32]{b}else{(if sb[30]{az}else{(if bz{az}else{(if br_{sf[28]}else{(if sb[25]{az}else{(if sb[21]{b}else{(if aR{b}else{(if aI{sf[28]}else{b})})})})})})})})})})});let cp=(if sb[35]{(c1+sf[41])}else{(if ((sf[40])!=0.0){(c3+sf[41])}else{b})});let cy=(((sf[17])!=0.0)&&((c7)!=0.0));let cC=(if cy{(sf[44]+(sf[46]/c3))}else{sf[44]});let cG=(if cy{(sf[45]+(sf[47]/c3))}else{sf[45]});let cI=(((sf[20])!=0.0)&&(sb[4]&&((c7)!=0.0)));let cM=(if cI{(cC+(sf[48]/c3))}else{cC});let cQ=(if cI{(cG+(sf[49]/c3))}else{cG});let d2=(a+(sf[71]*((if ((c5)!=0.0){(cM+(sf[50]/c0))}else{cM})+(sf[71]*(if ((c5)!=0.0){(cQ+(sf[51]/c0))}else{cQ})))));let df=(ci*(if (((if (d2<0.11){a}else{b}))!=0.0){(j+(0.1*(((10.0*(d2-j))-a)).exp()))}else{d2}));let di=(ctx.node_voltage(n[0])-ctx.node_voltage(n[1]));let dk=(if ((ci>b)&&sb[38]){a}else{b});let dm=(if ((dk)!=0.0){(di/cp)}else{b});let dp=(if ((dk)!=0.0){(dm*sf[52])}else{b});let ds=((a+(dp*dp))).sqrt();let dx=(if ((dk)!=0.0){(sf[53]*(dm).abs())}else{b});let dL=(!((dk)!=0.0));let dN=(df*(if dL{a}else{(if ((dk)!=0.0){((sf[55]+(sf[42]*(if ((dk)!=0.0){ds}else{b})))+(sf[43]*(if ((dk)!=0.0){f64::powf((a+(dx*(dx*dx))),0.3333333333333333)}else{b})))}else{b})}));let dY=(dp*(if ((dk)!=0.0){(sf[52]*(if ((dk)!=0.0){(a/cp)}else{b}))}else{b}));let e0=(dp*(if ((dk)!=0.0){(sf[52]*(if ((dk)!=0.0){(-1.0/cp)}else{b}))}else{b}));let e3=(ds*2.0);let ei=(dN*dN);

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * ((di/dN)),
            0,
            multiplicity * (((dN-(di*(df*(if dL{b}else{(if ((dk)!=0.0){(sf[42]*(if ((dk)!=0.0){((dY+dY)/e3)}else{b}))}else{b})}))))/ei)),
            1,
            multiplicity * ((((-dN)-(di*(df*(if dL{b}else{(if ((dk)!=0.0){(sf[42]*(if ((dk)!=0.0){((e0+e0)/e3)}else{b}))}else{b})}))))/ei)),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (b),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (b),
        );
    }

    pub fn stamp_reactive(&mut self, _ctx: &GeneratedEvalContext<'_>, _stamper: &mut GeneratedReactiveStamper<'_>) {
    }
}
