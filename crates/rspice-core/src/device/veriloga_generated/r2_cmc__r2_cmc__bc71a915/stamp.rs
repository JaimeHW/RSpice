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
        let a=1.0;let b=0.0;let g=ctx.simparam_or("scale", a);let j=0.01;let p_=ctx.simparam_or("shrink", b);let v=(((if sb[0]{g}else{sf[2]})*(if sb[1]{(a-(j*p_))}else{sf[7]}))*1000000.0);let au=(v*sf[31]);let ax=(au+sf[32]);let az=1e99;let aC=(v*sf[29]);let aD=(if sb[16]{aC}else{b});let aE=(sf[23]+aC);let aF=(if sb[16]{aE}else{b});let aH=(if (aF>b){a}else{b});let aI=(sb[16]&&((aH)!=0.0));let aT=(sb[16]&&(!((aH)!=0.0)));let bl=(if sb[27]{au}else{(if sb[25]{b}else{(if sb[21]{au}else{(if aT{au}else{(if aI{((aF*sf[34])-sf[32])}else{(if sb[14]{au}else{b})})})})})});let bm=(if sb[27]{ax}else{(if sb[25]{b}else{(if sb[21]{ax}else{(if aT{ax}else{(if aI{(sf[34]*(sf[23]+aD))}else{(if sb[14]{ax}else{b})})})})})});let bo=(if (bm>b){a}else{b});let bp=(sb[27]&&((bo)!=0.0));let bz=(sb[27]&&(!((bo)!=0.0)));let bW=(if sb[34]{ax}else{(if sb[32]{ax}else{(if sb[30]{b}else{bm})})});let bX=(if sb[34]{aC}else{(if sb[32]{b}else{(if sb[30]{aC}else{(if bz{aC}else{(if bp{((bm*sf[38])-sf[23])}else{(if sb[25]{aC}else{(if sb[21]{b}else{aD})})})})})})});let bY=(if sb[34]{aE}else{(if sb[32]{b}else{(if sb[30]{aE}else{(if bz{aE}else{(if bp{(sf[38]*(sf[32]+bl))}else{(if sb[25]{aE}else{(if sb[21]{b}else{aF})})})})})})});let c0=(if (bW>b){a}else{b});let c2=(if (bY>b){a}else{b});let c3=(sb[34]&&((c0)!=0.0));let cf=(if (sb[34]&&(!((c0)!=0.0))){az}else{(if (c3&&(!((c2)!=0.0))){b}else{(if (((c2)!=0.0)&&c3){(sf[33]*((sf[23]+bX)/(sf[32]+(if sb[34]{au}else{(if sb[32]{au}else{(if sb[30]{b}else{bl})})}))))}else{(if sb[32]{b}else{(if sb[30]{az}else{(if bz{az}else{(if bp{sf[28]}else{(if sb[25]{az}else{(if sb[21]{b}else{(if aT{b}else{(if aI{sf[28]}else{b})})})})})})})})})})});let cl=(if sb[35]{(bX+sf[41])}else{(bY+sf[41])});let cu=(((sf[17])!=0.0)&&((c2)!=0.0));let cy=(if cu{(sf[44]+(sf[46]/bY))}else{sf[44]});let cC=(if cu{(sf[45]+(sf[47]/bY))}else{sf[45]});let cE=(((sf[20])!=0.0)&&(sb[4]&&((c2)!=0.0)));let cY=(a+(sf[71]*((if ((c0)!=0.0){(sf[44]+(sf[50]/bW))}else{(if cE{(cy+(sf[48]/bY))}else{cy})})+(sf[71]*(if ((c0)!=0.0){(sf[45]+(sf[51]/bW))}else{(if cE{(cC+(sf[49]/bY))}else{cC})})))));let dc=(cf*(if (!(((if (cY<0.11){a}else{b}))!=0.0)){cY}else{(j+(0.1*(((10.0*(cY-j))-a)).exp()))}));let df=(ctx.node_voltage(n[0])-ctx.node_voltage(n[1]));let dh=(if ((cf>b)&&sb[38]){a}else{b});let dj=(if ((dh)!=0.0){(df/cl)}else{b});let dl=(dj*sf[52]);let do_=((a+(dl*dl))).sqrt();let ds=(sf[53]*(dj).abs());let dB=(!((dh)!=0.0));let dH=(dc*(if dB{a}else{((sf[55]+(sf[42]*(if ((dh)!=0.0){do_}else{b})))+(sf[43]*(if ((dh)!=0.0){f64::powf((a+(ds*(ds*ds))),0.3333333333333333)}else{b})))}));let dQ=(dl*(sf[52]*(if ((dh)!=0.0){(a/cl)}else{b})));let dS=(dl*(sf[52]*(if ((dh)!=0.0){(-1.0/cl)}else{b})));let dV=(do_*2.0);let e8=(dH*dH);

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * ((df/dH)),
            0,
            multiplicity * (((dH-(df*(dc*(if dB{b}else{(sf[42]*(if ((dh)!=0.0){((dQ+dQ)/dV)}else{b}))}))))/e8)),
            1,
            multiplicity * ((((-dH)-(df*(dc*(if dB{b}else{(sf[42]*(if ((dh)!=0.0){((dS+dS)/dV)}else{b}))}))))/e8)),
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
