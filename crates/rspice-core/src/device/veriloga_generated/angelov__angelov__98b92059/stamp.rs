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

impl Instance {
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
        let v0=ctx.node_voltage(nodes[8]);let v1=ctx.node_voltage(nodes[5]);let v2=(v0-v1);let v3=ctx.node_voltage(nodes[4]);let v4=ctx.node_voltage(nodes[3]);let v5=(v3-v4);let v6=(-v5);let v7=(v4-v1);let v8=ctx.node_voltage(nodes[7]);let v9=(v8-v4);let va=ctx.node_voltage(nodes[13]);let vb=0.0;let vu=ctx.node_voltage(nodes[11]);let vx=(if (sf[10]!=0.0){(sf[160]+(vu).abs())}else{sf[160]});let v11=((vx-sf[9])).abs();let v16=1.0;let v17=(if ((v11>vb)||sb[2]){v16}else{vb});let v35=(!(v17!=0.0));
        let v36=(if v35{sf[12]}else{(if (v17!=0.0){(sf[12]*(v16+(v11*sf[13])))}else{vb})});let v37=(if v35{sf[14]}else{(if (v17!=0.0){(sf[14]*(v16+(v11*sf[15])))}else{vb})});let v38=(if v35{sf[16]}else{(if (v17!=0.0){(sf[16]*(v16+(v11*sf[17])))}else{vb})});let v39=(if v35{sf[18]}else{(if (v17!=0.0){(sf[18]*(v16+(v11*sf[19])))}else{vb})});let v3a=(if v35{sf[20]}else{(if (v17!=0.0){(sf[20]*(v16+(v11*sf[21])))}else{vb})});let v3b=(if v35{sf[22]}else{(if (v17!=0.0){(sf[22]*(v16+(v11*sf[23])))}else{vb})});
        let v3c=(if v35{sf[24]}else{(if (v17!=0.0){(sf[24]*(v16+(v11*sf[25])))}else{vb})});let v3e=(if v35{sf[28]}else{(if (v17!=0.0){(sf[28]+(v11*sf[30]))}else{vb})});let v3f=(if v35{sf[31]}else{(if (v17!=0.0){(sf[31]+(v11*sf[33]))}else{vb})});let v3g=(if v35{sf[34]}else{(if (v17!=0.0){(sf[34]+(v11*sf[35]))}else{vb})});let v3h=(if v35{sf[36]}else{(if (v17!=0.0){(sf[36]+(v11*sf[37]))}else{vb})});let v3n=0.5;let v3u=(if sb[5]{sf[43]}else{(if (sf[40]!=0.0){(sf[42]/(vx*8.617333262145179e-5))}else{vb})});
        let v3w=(v7*sf[44]);let v3x=(v3w).cosh();let v3z=(v3x*v3x);let v42=(v37*(v16+(sf[45]/v3z)));let v47=((v7*sf[47])).tanh();let v4c=(sf[48]*(v6-sf[36]));let v4d=(v6-v3h);let v4f=((((if v35{sf[26]}else{(if (v17!=0.0){(sf[26]+(v11*sf[27]))}else{vb})})-sf[46])+(sf[46]*v47))-(v4c*v4d));let v4g=(v2-v4f);let v4h=(v4g*v4g);let v4n=(v4g*sf[50]);let v4p=(((v42*v4g)+(v4h*sf[49]))+(v4h*v4n));let v4q=(v4p).tanh();let v4r=(v16+v4q);let v4t=(-v4p);let v4x=((v3n*(scalar_limexp(v4p)-scalar_limexp(v4t)))).tanh();
        let v51=(sf[51]+(sf[47]*v4r));let v53=((v7*v51)).tanh();let v5f=(v36*v4r);let v5g=(v53*v5f);let v5l=(v38*scalar_limexp(v4d));let v5m=((v16+(v7*sf[57]))+v5l);let v5r=(v5-v4f);let v5s=(if sb[11]{v5r}else{v3x});let v5u=(if sb[11]{(v5s*v5s)}else{v4g});let v5w=(if sb[11]{(v5s*v5u)}else{v4h});let v62=(if sb[11]{(((v42*v5s)+(sf[49]*v5u))+(sf[50]*v5w))}else{vb});let v63=(v62).tanh();let v65=(if sb[11]{(v16+v63)}else{vb});let v68=(if sb[11]{(sf[51]+(sf[47]*v65))}else{vb});
        let v6c=(if sb[11]{(sf[57]+(v4r*sf[58]))}else{vb});let v6d=(v16+v53);let v6e=(v5f*v6d);let v6h=(v7-v3h);let v6j=(v38*scalar_limexp(v6h));let v6k=((v16+(v7*v6c))+v6j);let v6m=(if sb[11]{(v6e*v6k)}else{vb});let v6p=(if sb[11]{(sf[57]+(v65*sf[58]))}else{vb});let v6r=((v7*v68)).tanh();let v6t=(v36*v65);let v6u=(v16-(if sb[11]{v6r}else{vb}));let v6v=(v6t*v6u);let v6x=(v16-(v7*v6p));let v6z=(if sb[11]{(v6v*v6x)}else{vb});let v76=(if sb[14]{v4g}else{v5s});let v78=(if sb[14]{(v76*v76)}else{v5u});
        let v7b=(sf[50]*v78);let v7d=((v76+(sf[49]*v78))+(v76*v7b));let v7f=(if sb[14]{(v42*v7d)}else{v4p});let v7h=(-v7f);let v7l=((v3n*(scalar_limexp(v7f)-scalar_limexp(v7h)))).tanh();let v7n=(if sb[14]{(v16+v7l)}else{(v16+v4x)});let v7q=(if sb[14]{(sf[51]+(sf[47]*v7n))}else{vb});let v7s=((v7*v7q)).tanh();let v7t=(if sb[14]{v7s}else{vb});let v7w=(if sb[14]{(sf[57]+(sf[58]*v7n))}else{v6c});let v7x=(v36*v7n);let v7y=(v7t*v7x);let v81=(v5l+(v16+(v7*v7w)));let v87=(if sb[17]{v4g}else{v76});
        let v89=(if sb[17]{(v87*v87)}else{v78});let v8c=(sf[50]*v89);let v8e=((v87+(sf[49]*v89))+(v87*v8c));let v8g=(if sb[17]{(v42*v8e)}else{v7f});let v8h=(if sb[17]{v5r}else{v5w});let v8j=(if sb[17]{(v8h*v8h)}else{vb});let v8m=(sf[50]*v8h);let v8o=((v8h+(sf[49]*v8j))+(v8j*v8m));let v8q=(if sb[17]{(v42*v8o)}else{v62});let v8s=(-v8g);let v8w=((v3n*(scalar_limexp(v8g)-scalar_limexp(v8s)))).tanh();let v8y=(if sb[17]{(v16+v8w)}else{v7n});let v90=(-v8q);
        let v94=((v3n*(scalar_limexp(v8q)-scalar_limexp(v90)))).tanh();let v96=(if sb[17]{(v16+v94)}else{vb});let v99=(if sb[17]{(sf[51]+(sf[47]*v8y))}else{v7q});let v9c=(if sb[17]{(sf[51]+(sf[47]*v96))}else{vb});let v9e=((v7*v99)).tanh();let v9h=((v7*v9c)).tanh();let v9l=(if sb[17]{(sf[57]+(sf[58]*v96))}else{vb});let v9o=(if sb[17]{(sf[57]+(sf[58]*v8y))}else{vb});let v9p=(v36*v8y);let v9q=(v16+(if sb[17]{v9e}else{v7t}));let v9r=(v9p*v9q);let v9u=(v6j+(v16+(v7*v9o)));let v9x=(v36*v96);
        let v9y=(v16-(if sb[17]{v9h}else{vb}));let v9z=(v9x*v9y);let va1=(v16-(v7*v9l));let vaa=(v16+v4r);let vag=(v4r*sf[62]);let van=(v16+v8y);let vaq=(if sb[19]{(sf[60]+(v3b/van))}else{(if (sf[59]!=0.0){(sf[60]+(v3b/vaa))}else{vb})});let var=(v8y*sf[62]);let vat=(if sb[19]{(sf[61]+var)}else{(if (sf[59]!=0.0){(sf[61]+vag)}else{vb})});let vav=(if sb[19]{(sf[63]+var)}else{(if (sf[59]!=0.0){(vag+sf[63])}else{vb})});let vax=(if ((v11!=0.0)||sb[2]){v16}else{vb});let vb0=(v16+(v11*sf[64]));
        let vb5=(!(vax!=0.0));let vb6=(if vb5{vat}else{(if (vax!=0.0){(vat*vb0)}else{vb})});let vb7=(if vb5{vav}else{(if (vax!=0.0){(vav*vb0)}else{vb})});let vbb=-1.0;let vbh=(v2-v3g);let vbj=(v9-v3g);let vbp=(if sb[21]{scalar_limexp((v3g*(-v3u)))}else{(if (sf[66]!=0.0){scalar_limexp((v3u*((-v3g)).tanh()))}else{v87})});let vbt=(vbh).tanh();let vbv=(vbj).tanh();let vc2=(v3u*(if sb[25]{vbh}else{(if sb[23]{vbt}else{(if (sf[66]!=0.0){vbh}else{vb})})}));let vc5=(sf[68]*(scalar_limexp(vc2)-vbp));
        let vc6=(v3u*(if sb[25]{vbj}else{(if sb[23]{vbv}else{(if (sf[66]!=0.0){vbj}else{vb})})}));let vcd=(v7*sf[69]);let vce=((v3e+(v2*sf[29]))+vcd);let vcf=(vce).tanh();let vcl=((sf[70]+(v7*sf[71]))).tanh();let vcm=(v16+vcl);let vcr=((sf[72]-(v7*sf[73]))).tanh();let vct=((v16+vcr)-sf[69]);let vcw=((v3f+(v9*sf[32]))-vcd);let vcx=(vcw).tanh();let vcy=(v16+vcx);let vdc=(v39*(v16+vcf));let vdq=(if sb[33]{(vcm-sf[69])}else{vcm});let vdr=(v3e+vcd);let vdt=(if sb[33]{(vdr).cosh()}else{vb});
        let vdx=(if sb[33]{(vce).cosh()}else{vb});let ve3=((vce+(if sb[33]{(vdx).ln()}else{vb}))-(if sb[33]{(vdr+(if sb[33]{(vdt).ln()}else{vb}))}else{vb}));let vec=(v3f-vcd);let vee=(if sb[33]{(vec).cosh()}else{vdt});let vei=(if sb[33]{(vcw).cosh()}else{vdx});let veo=((vcw+(if sb[33]{(vei).ln()}else{vb}))-(if sb[33]{(vec+(if sb[33]{(vee).ln()}else{vb}))}else{vb}));let v1cf=(vce).sinh();let v1cl=(if sb[33]{(sf[29]*v1cf)}else{vb});
        let v1dm=(if sb[33]{(sf[78]+(v39*(sf[82]+((vdq*(sf[29]+(if sb[33]{(v1cl/vdx)}else{vb})))/sf[29]))))}else{vb});let vex=v1dm;let vey=(if sb[33]{vex}else{(if sb[30]{(sf[78]+(vcm*vdc))}else{sf[79]})});let v1dw=(vcw).sinh();let v1f8=(if sb[33]{(sf[80]+(v3a*(sf[82]+((vct*(sf[32]+(if sb[33]{((if sb[33]{(sf[32]*v1dw)}else{vb})/vei)}else{vb})))/sf[32]))))}else{vb});let vez=v1f8;let vf0=(if sb[33]{vez}else{(if sb[30]{(sf[80]+(v3a*((vct*vcy)+sf[82])))}else{sf[81]})});
        let vga=(if sb[49]{((v39*((vx*5.5226012e-23)*sf[104]))*sf[106])}else{vb});let vge=(if sb[49]{((v16-(vga*vga))).sqrt()}else{vb});let vgg=3.141592653589793;let vgi=(if sb[49]{((-vga)*vgg)}else{vb});
        let vgu=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (if sb[33]{((v3a*(((vct*veo)/sf[32])+(v9*sf[82])))+(v9*sf[80]))}else{vb}));
        let vgw=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (if sb[33]{((v39*(((vdq*ve3)/sf[29])+(v2*sf[82])))+(v2*sf[78]))}else{vb}));
        let vh0=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (v9*vf0));
        let vh3=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v2*vey));let vhb=ctx.node_voltage(nodes[10]);let vhe=(vhb-v1);let vhi=ctx.node_voltage(nodes[9]);
        let vhz=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (sf[92]*ctx.branch_current(branches[6])));
        let vi5=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, (sf[92]*ctx.branch_current(branches[8])));let vi7=ctx.branch_current(branches[10]);let vid=ctx.branch_current(branches[14]);
        let vii=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, (sf[96]*ctx.branch_current(branches[15])));
        let vio=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, (sf[96]*ctx.branch_current(branches[17])));let viq=ctx.node_voltage(nodes[14]);let vir=(if sb[49]{viq}else{vb});let vis=ctx.node_voltage(nodes[15]);let viy=(-(if sb[49]{(vga*vgg)}else{vb}));
        let vj0=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, (viq*viy));
        let vj4=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, (vu*sf[114]));let vjk=(v3w).sinh();let vjl=(sf[44]*vjk);let vjm=(sf[115]*vjk);let vjn=(v3x*vjl);let vjp=(v3x*vjm);let vjt=(v3z*v3z);let vjy=(v37*((-(sf[45]*(vjn+vjn)))/vjt));let vjz=(v37*((-(sf[45]*(vjp+vjp)))/vjt));let vk2=(v16-(v47*v47));
        let vk6=(sf[46]*(sf[116]*vk2));let vkc=((v4d*sf[117])+(-v4c));let vkd=((sf[46]*(sf[47]*vk2))-(v4c+(sf[48]*v4d)));let vkf=(-vkd);let vkg=(vbb-vk6);let vkh=(v4g*vkf);let vki=(vkh+vkh);let vkj=(v4g*vkc);let vkk=(vkj+vkj);let vkl=(v4g*vkg);let vkm=(vkl+vkl);let vkn=(v4g+v4g);let vli=((((v4g*vjy)+(v42*vkf))+(sf[49]*vki))+((v4n*vki)+(v4h*(sf[50]*vkf))));let vlj=(((v42*vkc)+(sf[49]*vkk))+((v4n*vkk)+(v4h*(sf[50]*vkc))));let vlk=((((v4g*vjz)+(v42*vkg))+(sf[49]*vkm))+((v4n*vkm)+(v4h*(sf[50]*vkg))));
        let vll=((v42+(sf[49]*vkn))+((v4n*vkn)+(v4h*sf[50])));let vln=(v16-(v4q*v4q));let vlo=(vli*vln);let vlp=(vlj*vln);let vlq=(vlk*vln);let vlr=(vll*vln);let vls=scalar_limexp_derivative(v4p);let vm1=scalar_limexp_derivative(v4t);let vmf=(v16-(v4x*v4x));let vmw=(v16-(v53*v53));let vn1=(v36*vlo);let vn2=(v36*vlp);let vn3=(v36*vlq);let vn4=(v36*vlr);let vn5=(v5f*((v51+(v7*(sf[47]*vlo)))*vmw));let vn8=(v5f*((v7*(sf[47]*vlp))*vmw));let vnb=(v5f*(((-v51)+(v7*(sf[47]*vlq)))*vmw));
        let vne=(v5f*((v7*(sf[47]*vlr))*vmw));let vni=scalar_limexp_derivative(v4d);let vnk=(v38*vni);let vnl=(v38*(-vni));let vo1=(vbb-vkd);let vo2=(v16-(-vkc));let vo3=(-vk6);let vo4=(if sb[11]{vo1}else{vjl});let vo5=(if sb[11]{vo2}else{vb});let vo6=(if sb[11]{vo3}else{vjm});let vo7=(v5s*vo4);let vo9=(v5s*vo5);let vob=(v5s*vo6);let vod=(if sb[11]{(vo7+vo7)}else{vkf});let voe=(if sb[11]{(vo9+vo9)}else{vkc});let vof=(if sb[11]{(vob+vob)}else{vkg});let vor=(if sb[11]{((v5u*vo4)+(v5s*vod))}else{vki});
        let vos=(if sb[11]{((v5u*vo5)+(v5s*voe))}else{vkk});let vot=(if sb[11]{((v5u*vo6)+(v5s*vof))}else{vkm});let vou=(if sb[11]{(v5s*sf[119])}else{vkn});let vph=(if sb[11]{((((v5s*vjy)+(v42*vo4))+(sf[49]*vod))+(sf[50]*vor))}else{vb});let vpi=(if sb[11]{(((v42*vo5)+(sf[49]*voe))+(sf[50]*vos))}else{vb});let vpj=(if sb[11]{((((v5s*vjz)+(v42*vo6))+(sf[49]*vof))+(sf[50]*vot))}else{vb});let vpk=(if sb[11]{(sf[120]+(sf[50]*vou))}else{vb});let vpm=(v16-(v63*v63));let vpr=(if sb[11]{(vph*vpm)}else{vb});
        let vps=(if sb[11]{(vpi*vpm)}else{vb});let vpt=(if sb[11]{(vpj*vpm)}else{vb});let vpu=(if sb[11]{(vpk*vpm)}else{vb});let vq7=(if sb[11]{(sf[58]*vlo)}else{vb});let vq8=(if sb[11]{(sf[58]*vlp)}else{vb});let vq9=(if sb[11]{(sf[58]*vlq)}else{vb});let vqa=(if sb[11]{(sf[58]*vlr)}else{vb});let vqq=scalar_limexp_derivative(v6h);let vqs=(v38*vqq);let vqt=(v38*(-vqq));let vr8=(if sb[11]{((v6k*(vn5+(v6d*vn1)))+(v6e*((v6c+(v7*vq7))+vqs)))}else{vb});
        let vr9=(if sb[11]{((v6k*(vn8+(v6d*vn2)))+(v6e*(v7*vq8)))}else{vb});let vra=(if sb[11]{((v6k*(vnb+(v6d*vn3)))+(v6e*(((-v6c)+(v7*vq9))+vqt)))}else{vb});let vrb=(if sb[11]{((v6k*(vne+(v6d*vn4)))+(v6e*(v7*vqa)))}else{vb});let vrs=(v16-(v6r*v6r));let vt8=(if sb[11]{((v6x*((v6u*(v36*vpr))+(v6t*(-(if sb[11]{((v68+(v7*(if sb[11]{(sf[47]*vpr)}else{vb})))*vrs)}else{vb})))))+(v6v*(-(v6p+(v7*(if sb[11]{(sf[58]*vpr)}else{vb}))))))}else{vb});
        let vt9=(if sb[11]{((v6x*((v6u*(v36*vps))+(v6t*(-(if sb[11]{((v7*(if sb[11]{(sf[47]*vps)}else{vb}))*vrs)}else{vb})))))+(v6v*(-(v7*(if sb[11]{(sf[58]*vps)}else{vb})))))}else{vb});let vta=(if sb[11]{((v6x*((v6u*(v36*vpt))+(v6t*(-(if sb[11]{(((-v68)+(v7*(if sb[11]{(sf[47]*vpt)}else{vb})))*vrs)}else{vb})))))+(v6v*(-((-v6p)+(v7*(if sb[11]{(sf[58]*vpt)}else{vb}))))))}else{vb});
        let vtb=(if sb[11]{((v6x*((v6u*(v36*vpu))+(v6t*(-(if sb[11]{((v7*(if sb[11]{(sf[47]*vpu)}else{vb}))*vrs)}else{vb})))))+(v6v*(-(v7*(if sb[11]{(sf[58]*vpu)}else{vb})))))}else{vb});let vto=(if sb[14]{vkf}else{vo4});let vtp=(if sb[14]{vkc}else{vo5});let vtq=(if sb[14]{vkg}else{vo6});let vts=(v76*vto);let vtu=(v76*vtp);let vtw=(v76*vtq);let vty=(v76*sf[121]);let vu0=(if sb[14]{(vts+vts)}else{vod});let vu1=(if sb[14]{(vtu+vtu)}else{voe});let vu2=(if sb[14]{(vtw+vtw)}else{vof});
        let vu3=(if sb[14]{(vty+vty)}else{sf[119]});let vv4=(if sb[14]{((v7d*vjy)+(v42*((vto+(sf[49]*vu0))+((v7b*vto)+(v76*(sf[50]*vu0))))))}else{vli});let vv5=(if sb[14]{(v42*((vtp+(sf[49]*vu1))+((v7b*vtp)+(v76*(sf[50]*vu1)))))}else{vlj});let vv6=(if sb[14]{((v7d*vjz)+(v42*((vtq+(sf[49]*vu2))+((v7b*vtq)+(v76*(sf[50]*vu2))))))}else{vlk});let vv7=(if sb[14]{(v42*((sf[121]+(sf[49]*vu3))+((v7b*sf[121])+(v76*(sf[50]*vu3)))))}else{vll});let vv8=scalar_limexp_derivative(v7f);let vvh=scalar_limexp_derivative(v7h);
        let vvv=(v16-(v7l*v7l));let vw0=(if sb[14]{((v3n*((vv4*vv8)-((-vv4)*vvh)))*vvv)}else{((v3n*((vli*vls)-((-vli)*vm1)))*vmf)});let vw1=(if sb[14]{((v3n*((vv5*vv8)-((-vv5)*vvh)))*vvv)}else{((v3n*((vlj*vls)-((-vlj)*vm1)))*vmf)});let vw2=(if sb[14]{((v3n*((vv6*vv8)-((-vv6)*vvh)))*vvv)}else{((v3n*((vlk*vls)-((-vlk)*vm1)))*vmf)});let vw3=(if sb[14]{((v3n*((vv7*vv8)-((-vv7)*vvh)))*vvv)}else{((v3n*((vll*vls)-((-vll)*vm1)))*vmf)});let vw8=(if sb[14]{(sf[47]*vw0)}else{vb});
        let vw9=(if sb[14]{(sf[47]*vw1)}else{vb});let vwa=(if sb[14]{(sf[47]*vw2)}else{vb});let vwb=(if sb[14]{(sf[47]*vw3)}else{vb});let vwk=(v16-(v7s*v7s));let vwp=(if sb[14]{((v7q+(v7*vw8))*vwk)}else{vb});let vwq=(if sb[14]{((v7*vw9)*vwk)}else{vb});let vwr=(if sb[14]{(((-v7q)+(v7*vwa))*vwk)}else{vb});let vws=(if sb[14]{((v7*vwb)*vwk)}else{vb});let vy6=(if sb[17]{vkf}else{vto});let vy7=(if sb[17]{vkc}else{vtp});let vy8=(if sb[17]{vkg}else{vtq});let vya=(v87*vy6);let vyc=(v87*vy7);let vye=(v87*vy8);
        let vyg=(v87*sf[122]);let vyi=(if sb[17]{(vya+vya)}else{vu0});let vyj=(if sb[17]{(vyc+vyc)}else{vu1});let vyk=(if sb[17]{(vye+vye)}else{vu2});let vyl=(if sb[17]{(vyg+vyg)}else{vu3});let vzm=(if sb[17]{((v8e*vjy)+(v42*((vy6+(sf[49]*vyi))+((v8c*vy6)+(v87*(sf[50]*vyi))))))}else{vv4});let vzn=(if sb[17]{(v42*((vy7+(sf[49]*vyj))+((v8c*vy7)+(v87*(sf[50]*vyj)))))}else{vv5});let vzo=(if sb[17]{((v8e*vjz)+(v42*((vy8+(sf[49]*vyk))+((v8c*vy8)+(v87*(sf[50]*vyk))))))}else{vv6});
        let vzp=(if sb[17]{(v42*((sf[122]+(sf[49]*vyl))+((v8c*sf[122])+(v87*(sf[50]*vyl)))))}else{vv7});let vzq=(if sb[17]{vo1}else{vor});let vzr=(if sb[17]{vo2}else{vos});let vzs=(if sb[17]{vo3}else{vot});let vzt=(if sb[17]{vb}else{vou});let vzu=(v8h*vzq);let vzw=(v8h*vzr);let vzy=(v8h*vzs);let v100=(v8h*vzt);let v102=(if sb[17]{(vzu+vzu)}else{vb});let v103=(if sb[17]{(vzw+vzw)}else{vb});let v104=(if sb[17]{(vzy+vzy)}else{vb});let v105=(if sb[17]{(v100+v100)}else{vb});
        let v116=(if sb[17]{((v8o*vjy)+(v42*((vzq+(sf[49]*v102))+((v8m*v102)+(v8j*(sf[50]*vzq))))))}else{vph});let v117=(if sb[17]{(v42*((vzr+(sf[49]*v103))+((v8m*v103)+(v8j*(sf[50]*vzr)))))}else{vpi});let v118=(if sb[17]{((v8o*vjz)+(v42*((vzs+(sf[49]*v104))+((v8m*v104)+(v8j*(sf[50]*vzs))))))}else{vpj});let v119=(if sb[17]{(v42*((vzt+(sf[49]*v105))+((v8m*v105)+(v8j*(sf[50]*vzt)))))}else{vpk});let v11a=scalar_limexp_derivative(v8g);let v11j=scalar_limexp_derivative(v8s);let v11x=(v16-(v8w*v8w));
        let v122=(if sb[17]{((v3n*((vzm*v11a)-((-vzm)*v11j)))*v11x)}else{vw0});let v123=(if sb[17]{((v3n*((vzn*v11a)-((-vzn)*v11j)))*v11x)}else{vw1});let v124=(if sb[17]{((v3n*((vzo*v11a)-((-vzo)*v11j)))*v11x)}else{vw2});let v125=(if sb[17]{((v3n*((vzp*v11a)-((-vzp)*v11j)))*v11x)}else{vw3});let v126=scalar_limexp_derivative(v8q);let v12f=scalar_limexp_derivative(v90);let v12t=(v16-(v94*v94));let v12y=(if sb[17]{((v3n*((v116*v126)-((-v116)*v12f)))*v12t)}else{vb});
        let v12z=(if sb[17]{((v3n*((v117*v126)-((-v117)*v12f)))*v12t)}else{vb});let v130=(if sb[17]{((v3n*((v118*v126)-((-v118)*v12f)))*v12t)}else{vb});let v131=(if sb[17]{((v3n*((v119*v126)-((-v119)*v12f)))*v12t)}else{vb});let v13q=(v16-(v9e*v9e));let v147=(v16-(v9h*v9h));
        let v17k=(if sb[17]{(v3n*((if sb[17]{((v9u*((v9q*(v36*v122))+(v9p*(if sb[17]{((v99+(v7*(if sb[17]{(sf[47]*v122)}else{vw8})))*v13q)}else{vwp}))))+(v9r*(vqs+(v9o+(v7*(if sb[17]{(sf[58]*v122)}else{vb}))))))}else{vr8})-(if sb[17]{((va1*((v9y*(v36*v12y))+(v9x*(-(if sb[17]{((v9c+(v7*(if sb[17]{(sf[47]*v12y)}else{vb})))*v147)}else{vb})))))+(v9z*(-(v9l+(v7*(if sb[17]{(sf[58]*v12y)}else{vb}))))))}else{vt8})))}else{(if sb[14]{((v81*((v7x*vwp)+(v7t*(v36*vw0))))+(v7y*(vnk+(v7w+(v7*(if sb[14]{(sf[58]*vw0)}else{vq7}))))))}else{(if sb[11]{(v3n*(vr8-vt8))}else{(if (sf[53]!=0.0){((v5m*(vn5+(v53*vn1)))+(v5g*(sf[57]+vnk)))}else{vb})})})});
        let v17m=(if sb[17]{(v3n*((if sb[17]{((v9u*((v9q*(v36*v124))+(v9p*(if sb[17]{(((-v99)+(v7*(if sb[17]{(sf[47]*v124)}else{vwa})))*v13q)}else{vwr}))))+(v9r*(vqt+((-v9o)+(v7*(if sb[17]{(sf[58]*v124)}else{vb}))))))}else{vra})-(if sb[17]{((va1*((v9y*(v36*v130))+(v9x*(-(if sb[17]{(((-v9c)+(v7*(if sb[17]{(sf[47]*v130)}else{vb})))*v147)}else{vb})))))+(v9z*(-((-v9l)+(v7*(if sb[17]{(sf[58]*v130)}else{vb}))))))}else{vta})))}else{(if sb[14]{((v81*((v7x*vwr)+(v7t*(v36*vw2))))+(v7y*((-v7w)+(v7*(if sb[14]{(sf[58]*vw2)}else{vq9})))))}else{(if sb[11]{(v3n*(vra-vta))}else{(if (sf[53]!=0.0){((v5m*(vnb+(v53*vn3)))+(v5g*sf[118]))}else{vb})})})});
        let v17q=(vaa*vaa);let v18f=(van*van);let v18y=(if sb[19]{(sf[62]*v122)}else{(if (sf[59]!=0.0){(sf[62]*vlo)}else{vb})});let v18z=(if sb[19]{(sf[62]*v123)}else{(if (sf[59]!=0.0){(sf[62]*vlp)}else{vb})});let v190=(if sb[19]{(sf[62]*v124)}else{(if (sf[59]!=0.0){(sf[62]*vlq)}else{vb})});let v191=(if sb[19]{(sf[62]*v125)}else{(if (sf[59]!=0.0){(sf[62]*vlr)}else{vb})});let v19a=(if vb5{v18y}else{(if (vax!=0.0){(vb0*v18y)}else{vb})});let v19b=(if vb5{v18z}else{(if (vax!=0.0){(vb0*v18z)}else{vb})});
        let v19c=(if vb5{v190}else{(if (vax!=0.0){(vb0*v190)}else{vb})});let v19d=(if vb5{v191}else{(if (vax!=0.0){(vb0*v191)}else{vb})});let v19k=(if sb[21]{vb}else{(if (sf[66]!=0.0){vb}else{vy6})});let v19m=(if sb[21]{vb}else{(if (sf[66]!=0.0){vb}else{vy8})});let v19p=(v16-(vbt*vbt));let v19u=(v16-(vbv*vbv));let v1a4=scalar_limexp_derivative(vc2);let v1ac=(sf[68]*(-(if sb[21]{vb}else{(if (sf[66]!=0.0){vb}else{vy7})})));let v1ah=scalar_limexp_derivative(vc6);let v1av=(v16-(vcf*vcf));
        let v1b1=(v16-(vcl*vcl));let v1b2=(sf[71]*v1b1);let v1b3=(sf[132]*v1b1);let v1b6=(v16-(vcr*vcr));let v1b7=(sf[133]*v1b6);let v1b8=(sf[73]*v1b6);let v1bc=(v16-(vcx*vcx));let v1c6=(vdr).sinh();let v1c9=(if sb[33]{(sf[69]*v1c6)}else{vb});let v1ca=(if sb[33]{(sf[130]*v1c6)}else{vb});let v1cj=(if sb[33]{(sf[69]*v1cf)}else{vb});let v1ck=(if sb[33]{(sf[131]*v1cf)}else{vb});let v1dn=(vec).sinh();
        let v1fh=(-(if sb[17]{(v3n*((if sb[17]{((v9u*((v9q*(v36*v123))+(v9p*(if sb[17]{((v7*(if sb[17]{(sf[47]*v123)}else{vw9}))*v13q)}else{vwq}))))+(v9r*(v7*(if sb[17]{(sf[58]*v123)}else{vb}))))}else{vr9})-(if sb[17]{((va1*((v9y*(v36*v12z))+(v9x*(-(if sb[17]{((v7*(if sb[17]{(sf[47]*v12z)}else{vb}))*v147)}else{vb})))))+(v9z*(-(v7*(if sb[17]{(sf[58]*v12z)}else{vb})))))}else{vt9})))}else{(if sb[14]{((v81*((v7x*vwq)+(v7t*(v36*vw1))))+(v7y*(vnl+(v7*(if sb[14]{(sf[58]*vw1)}else{vq8})))))}else{(if sb[11]{(v3n*(vr9-vt9))}else{(if (sf[53]!=0.0){((v5m*(vn8+(v53*vn2)))+(v5g*vnl))}else{vb})})})}));
        let v1fk=ddt_scale;let v1gs=(vaq*vaq);let v1ho=(sf[92]*v1fk);let v1i9=(sf[96]*v1fk);

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(12),
            None,
            multiplicity * ((-(if sb[17]{(v3n*((if sb[17]{(v9r*v9u)}else{v6m})-(if sb[17]{(v9z*va1)}else{v6z})))}else{(if sb[14]{(v7y*v81)}else{(if sb[11]{(v3n*(v6m-v6z))}else{(if (sf[53]!=0.0){(v5g*v5m)}else{vb})})})}))),
            [3, 4, 5, 8],
            [(-v17k), v1fh, (-v17m), (-(if sb[17]{(v3n*((if sb[17]{((v9u*((v9q*(v36*v125))+(v9p*(if sb[17]{((v7*(if sb[17]{(sf[47]*v125)}else{vwb}))*v13q)}else{vws}))))+(v9r*(v7*(if sb[17]{(sf[58]*v125)}else{vb}))))}else{vrb})-(if sb[17]{((va1*((v9y*(v36*v131))+(v9x*(-(if sb[17]{((v7*(if sb[17]{(sf[47]*v131)}else{vb}))*v147)}else{vb})))))+(v9z*(-(v7*(if sb[17]{(sf[58]*v131)}else{vb})))))}else{vtb})))}else{(if sb[14]{((v81*((v7x*vws)+(v7t*(v36*vw3))))+(v7y*(v7*(if sb[14]{(sf[58]*vw3)}else{vqa}))))}else{(if sb[11]{(v3n*(vrb-vtb))}else{(if (sf[53]!=0.0){(v5m*(vne+(v53*vn4)))}else{vb})})})}))],
            [],
            [],
            multiplicity,
        );
        let vgq_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (sf[108]*ctx.node_voltage(nodes[12])));
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (vgq_ddt),
            12,
            multiplicity * (((sf[108]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (va),
            13,
            multiplicity * (v16),
        );
        let vgt_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (sf[109]*ctx.branch_current(branches[0])));
        stamper.stamp_potential_branch_local(
            Some(12),
            Some(13),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            vgt_ddt,
            0,
            ((sf[109]) * ddt_scale),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            Some(5),
            multiplicity * (va),
            13,
            multiplicity * (v16),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (vc5),
            [3, 4, 5, 8],
            [(sf[68]*(-v19k)), v1ac, (sf[68]*(((v3u*(if sb[25]{vbb}else{(if sb[23]{(-v19p)}else{sf[124]})}))*v1a4)-v19m)), (sf[68]*(((v3u*(if sb[25]{v16}else{(if sb[23]{v19p}else{sf[125]})}))*v1a4)-sf[126]))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * ((sf[68]*(scalar_limexp(vc6)-vbp))),
            [3, 4, 5, 7, 8],
            [(sf[68]*(((v3u*(if sb[25]{vbb}else{(if sb[23]{(-v19u)}else{sf[124]})}))*v1ah)-v19k)), v1ac, (sf[68]*(-v19m)), (sf[68]*((v3u*(if sb[25]{v16}else{(if sb[23]{v19u}else{sf[125]})}))*v1ah)), sf[128]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((if (sf[77]!=0.0){vgu}else{vb})),
            [3, 5, 7, 8],
            [(if (sf[77]!=0.0){((if sb[33]{((v3a*(sf[136]+(((veo*v1b7)+(vct*((sf[135]+(if sb[33]{((if sb[33]{(sf[135]*v1dw)}else{v1cj})/vei)}else{vb}))-(if sb[33]{(sf[130]+(if sb[33]{((if sb[33]{(sf[130]*v1dn)}else{v1c9})/vee)}else{vb}))}else{vb}))))/sf[32])))+sf[138])}else{vb})*v1fk)}else{vb}), (if (sf[77]!=0.0){((if sb[33]{(v3a*(((veo*v1b8)+(vct*((sf[69]+(if sb[33]{((if sb[33]{(sf[69]*v1dw)}else{v1ck})/vei)}else{vb}))-(if sb[33]{(sf[69]+(if sb[33]{((if sb[33]{(sf[69]*v1dn)}else{v1ca})/vee)}else{vb}))}else{vb}))))/sf[32]))}else{vb})*v1fk)}else{vb}), (if (sf[77]!=0.0){(v1f8*v1fk)}else{vb}), (if (sf[77]!=0.0){((if sb[33]{(v3a*((vct*(if sb[33]{((if sb[33]{vb}else{v1cl})/vei)}else{vb}))/sf[32]))}else{vb})*v1fk)}else{vb})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if (sf[77]!=0.0){vgw}else{vb})),
            3,
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{(v39*(((ve3*v1b2)+(vdq*((sf[69]+(if sb[33]{(v1cj/vdx)}else{vb}))-(if sb[33]{(sf[69]+(if sb[33]{(v1c9/vdt)}else{vb}))}else{vb}))))/sf[29]))}else{vb})*v1fk)}else{vb})),
            5,
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{((v39*((((ve3*v1b3)+(vdq*((sf[131]+(if sb[33]{(v1ck/vdx)}else{vb}))-(if sb[33]{(sf[130]+(if sb[33]{(v1ca/vdt)}else{vb}))}else{vb}))))/sf[29])+sf[136]))+sf[137])}else{vb})*v1fk)}else{vb})),
            8,
            multiplicity * ((if (sf[77]!=0.0){(v1dm*v1fk)}else{vb})),
        );
        stamper.stamp_current_node3_local(
            Some(7),
            Some(3),
            multiplicity * ((if sb[51]{vh0}else{vb})),
            3,
            multiplicity * ((if sb[51]{(v1fk*((-vf0)+(v9*(if sb[33]{vb}else{(if sb[30]{(v3a*((vcy*v1b7)+(vct*(sf[135]*v1bc))))}else{vb})}))))}else{vb})),
            5,
            multiplicity * ((if sb[51]{(v1fk*(v9*(if sb[33]{vb}else{(if sb[30]{(v3a*((vcy*v1b8)+(vct*(sf[69]*v1bc))))}else{vb})})))}else{vb})),
            7,
            multiplicity * ((if sb[51]{(v1fk*(vf0+(v9*(if sb[33]{vb}else{(if sb[30]{(v3a*(vct*(sf[32]*v1bc)))}else{vb})}))))}else{vb})),
        );
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * ((if sb[51]{vh3}else{vb})),
            3,
            multiplicity * ((if sb[51]{(v1fk*(v2*(if sb[33]{vb}else{(if sb[30]{((vdc*v1b2)+(vcm*(v39*(sf[69]*v1av))))}else{vb})})))}else{vb})),
            5,
            multiplicity * ((if sb[51]{(v1fk*((-vey)+(v2*(if sb[33]{vb}else{(if sb[30]{((vdc*v1b3)+(vcm*(v39*(sf[131]*v1av))))}else{vb})}))))}else{vb})),
            8,
            multiplicity * ((if sb[51]{(v1fk*(vey+(v2*(if sb[33]{vb}else{(if sb[30]{(vcm*(v39*(sf[29]*v1av)))}else{vb})}))))}else{vb})),
        );
        let vh8_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (sf[110]*(ctx.node_voltage(nodes[1])-v4)));
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (vh8_ddt),
            1,
            multiplicity * (((sf[110]) * ddt_scale)),
            3,
            multiplicity * (((sf[139]) * ddt_scale)),
        );
        let vha_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (v7*sf[111]));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(5),
            multiplicity * (vha_ddt),
            3,
            multiplicity * (((sf[111]) * ddt_scale)),
            5,
            multiplicity * (((sf[140]) * ddt_scale)),
        );
        let vhd_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (v3c*(v4-vhb)));
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (vhd_ddt),
            3,
            multiplicity * (((v3c) * ddt_scale)),
            10,
            multiplicity * ((((-v3c)) * ddt_scale)),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if (sf[83]!=0.0){(vhe/vaq)}else{vb})),
            [3, 4, 5, 8, 10],
            [(if (sf[83]!=0.0){((-(vhe*(if sb[19]{((-(v3b*v122))/v18f)}else{(if (sf[59]!=0.0){((-(v3b*vlo))/v17q)}else{vb})})))/v1gs)}else{vb}), (if (sf[83]!=0.0){((-(vhe*(if sb[19]{((-(v3b*v123))/v18f)}else{(if (sf[59]!=0.0){((-(v3b*vlp))/v17q)}else{vb})})))/v1gs)}else{vb}), (if (sf[83]!=0.0){(((-vaq)-(vhe*(if sb[19]{((-(v3b*v124))/v18f)}else{(if (sf[59]!=0.0){((-(v3b*vlq))/v17q)}else{vb})})))/v1gs)}else{vb}), (if (sf[83]!=0.0){((-(vhe*(if sb[19]{((-(v3b*v125))/v18f)}else{(if (sf[59]!=0.0){((-(v3b*vlr))/v17q)}else{vb})})))/v1gs)}else{vb}), (if (sf[83]!=0.0){(v16/vaq)}else{vb})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(5),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            vb,
        );
        let vhk_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (sf[112]*(vhi-v0)));
        stamper.stamp_current_node2_local(
            Some(9),
            Some(8),
            multiplicity * (vhk_ddt),
            8,
            multiplicity * (((sf[141]) * ddt_scale)),
            9,
            multiplicity * (((sf[112]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(5),
            multiplicity * ((if (sf[85]!=0.0){((vhi-v1)/sf[84])}else{vb})),
            5,
            multiplicity * (sf[144]),
            9,
            multiplicity * (sf[145]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(5),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            vb,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(7),
            multiplicity * ((if (sf[87]!=0.0){((v3-v8)/sf[86])}else{vb})),
            4,
            multiplicity * (sf[148]),
            7,
            multiplicity * (sf[149]),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(7),
            multiplicity * (vb),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(7),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            vb,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(8),
            multiplicity * ((if (sf[89]!=0.0){((v3-v0)/sf[88])}else{vb})),
            4,
            multiplicity * (sf[152]),
            8,
            multiplicity * (sf[153]),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            vb,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            5,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            5,
            (if (sf[91]!=0.0){(sf[90]*ctx.branch_current(branches[5]))}else{vb}),
            5,
            sf[154],
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            6,
            (if (sf[91]!=0.0){vhz}else{vb}),
            6,
            (if (sf[91]!=0.0){v1ho}else{vb}),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            vb,
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            8,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            8,
            (if sb[53]{vi5}else{vb}),
            8,
            (if sb[53]{v1ho}else{vb}),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            vb,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            10,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            10,
            (if (sf[94]!=0.0){(vb7*vi7)}else{vb}),
            [3, 4, 5, 8],
            [(if (sf[94]!=0.0){(vi7*v19a)}else{vb}), (if (sf[94]!=0.0){(vi7*v19b)}else{vb}), (if (sf[94]!=0.0){(vi7*v19c)}else{vb}), (if (sf[94]!=0.0){(vi7*v19d)}else{vb})],
            [10],
            [(if (sf[94]!=0.0){vb7}else{vb})],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            11,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            11,
            vb,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            vb,
        );
        let vic_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, (sf[113]*ctx.branch_current(branches[13])));
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(2),
            13,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            13,
            vic_ddt,
            13,
            ((sf[113]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            14,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<4, 1>(
            14,
            (if (sf[95]!=0.0){(vb6*vid)}else{vb}),
            [3, 4, 5, 8],
            [(if (sf[95]!=0.0){(vid*v19a)}else{vb}), (if (sf[95]!=0.0){(vid*v19b)}else{vb}), (if (sf[95]!=0.0){(vid*v19c)}else{vb}), (if (sf[95]!=0.0){(vid*v19d)}else{vb})],
            [14],
            [(if (sf[95]!=0.0){vb6}else{vb})],
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            15,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            15,
            (if (sf[95]!=0.0){vii}else{vb}),
            15,
            (if (sf[95]!=0.0){v1i9}else{vb}),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            vb,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            17,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            17,
            (if sb[55]{vio}else{vb}),
            17,
            (if sb[55]{v1i9}else{vb}),
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            18,
            vb,
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (vb),
        );
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (vir),
            14,
            multiplicity * (sf[155]),
        );
        stamper.stamp_current_const_local(
            Some(15),
            None,
            multiplicity * (vb),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * ((if sb[49]{vis}else{vb})),
            15,
            multiplicity * (sf[155]),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(5),
            multiplicity * (vir),
            14,
            multiplicity * (sf[155]),
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((if sb[49]{((vgi*viq)+(vge*vis))}else{vb})),
            14,
            multiplicity * ((if sb[49]{vgi}else{vb})),
            15,
            multiplicity * ((if sb[49]{vge}else{vb})),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(3),
            multiplicity * ((if sb[49]{vj0}else{vb})),
            14,
            multiplicity * ((if sb[49]{(viy*v1fk)}else{vb})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (viq),
            14,
            multiplicity * (v16),
        );
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (vis),
            15,
            multiplicity * (v16),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (vb),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(3),
            multiplicity * (vb),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if (sf[107]!=0.0){vj4}else{vb})),
            11,
            multiplicity * ((if (sf[107]!=0.0){(sf[114]*v1fk)}else{vb})),
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * ((if (sf[107]!=0.0){(-(((v7*(-va))+(v2*vc5))).abs())}else{vb})),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if (sf[107]!=0.0){(vu/sf[11])}else{vb})),
            11,
            multiplicity * (sf[157]),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * ((if sb[56]{(vu*1e-12)}else{vb})),
            11,
            multiplicity * (sf[158]),
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
        let v0=ctx.node_voltage(nodes[8]);let v1=ctx.node_voltage(nodes[5]);let v2=(v0-v1);let v4=ctx.node_voltage(nodes[3]);let v7=(v4-v1);let v9=(ctx.node_voltage(nodes[7])-v4);let vb=0.0;let vu=ctx.node_voltage(nodes[11]);let vx=(if (sf[10]!=0.0){(sf[160]+(vu).abs())}else{sf[160]});let v11=((vx-sf[9])).abs();let v16=1.0;let v17=(if ((v11>vb)||sb[2]){v16}else{vb});let v35=(!(v17!=0.0));let v39=(if v35{sf[18]}else{(if (v17!=0.0){(sf[18]*(v16+(v11*sf[19])))}else{vb})});
        let v3a=(if v35{sf[20]}else{(if (v17!=0.0){(sf[20]*(v16+(v11*sf[21])))}else{vb})});let v3c=(if v35{sf[24]}else{(if (v17!=0.0){(sf[24]*(v16+(v11*sf[25])))}else{vb})});let v3e=(if v35{sf[28]}else{(if (v17!=0.0){(sf[28]+(v11*sf[30]))}else{vb})});let v3f=(if v35{sf[31]}else{(if (v17!=0.0){(sf[31]+(v11*sf[33]))}else{vb})});let vcd=(v7*sf[69]);let vce=((v3e+(v2*sf[29]))+vcd);let vcf=(vce).tanh();let vcl=((sf[70]+(v7*sf[71]))).tanh();let vcm=(v16+vcl);let vcr=((sf[72]-(v7*sf[73]))).tanh();
        let vct=((v16+vcr)-sf[69]);let vcw=((v3f+(v9*sf[32]))-vcd);let vcx=(vcw).tanh();let vcy=(v16+vcx);let vdc=(v39*(v16+vcf));let vdq=(if sb[33]{(vcm-sf[69])}else{vcm});let vdr=(v3e+vcd);let vdt=(if sb[33]{(vdr).cosh()}else{vb});let vdx=(if sb[33]{(vce).cosh()}else{vb});let ve3=((vce+(if sb[33]{(vdx).ln()}else{vb}))-(if sb[33]{(vdr+(if sb[33]{(vdt).ln()}else{vb}))}else{vb}));let vec=(v3f-vcd);let vee=(if sb[33]{(vec).cosh()}else{vdt});let vei=(if sb[33]{(vcw).cosh()}else{vdx});
        let veo=((vcw+(if sb[33]{(vei).ln()}else{vb}))-(if sb[33]{(vec+(if sb[33]{(vee).ln()}else{vb}))}else{vb}));let v1cf=(vce).sinh();let v1cl=(if sb[33]{(sf[29]*v1cf)}else{vb});let v1dm=(if sb[33]{(sf[78]+(v39*(sf[82]+((vdq*(sf[29]+(if sb[33]{(v1cl/vdx)}else{vb})))/sf[29]))))}else{vb});let vex=v1dm;let vey=(if sb[33]{vex}else{(if sb[30]{(sf[78]+(vcm*vdc))}else{sf[79]})});let v1dw=(vcw).sinh();
        let v1f8=(if sb[33]{(sf[80]+(v3a*(sf[82]+((vct*(sf[32]+(if sb[33]{((if sb[33]{(sf[32]*v1dw)}else{vb})/vei)}else{vb})))/sf[32]))))}else{vb});let vez=v1f8;let vf0=(if sb[33]{vez}else{(if sb[30]{(sf[80]+(v3a*((vct*vcy)+sf[82])))}else{sf[81]})});let vgu=0.0;let vgw=0.0;let vh0=0.0;let vh3=0.0;let vhz=0.0;let vi5=0.0;let vii=0.0;let vio=0.0;let viy=(-(if sb[49]{((if sb[49]{((v39*((vx*5.5226012e-23)*sf[104]))*sf[106])}else{vb})*3.141592653589793)}else{vb}));let vj0=0.0;let vj4=0.0;
        let v1av=(v16-(vcf*vcf));let v1b1=(v16-(vcl*vcl));let v1b2=(sf[71]*v1b1);let v1b3=(sf[132]*v1b1);let v1b6=(v16-(vcr*vcr));let v1b7=(sf[133]*v1b6);let v1b8=(sf[73]*v1b6);let v1bc=(v16-(vcx*vcx));let v1c6=(vdr).sinh();let v1c9=(if sb[33]{(sf[69]*v1c6)}else{vb});let v1ca=(if sb[33]{(sf[130]*v1c6)}else{vb});let v1cj=(if sb[33]{(sf[69]*v1cf)}else{vb});let v1ck=(if sb[33]{(sf[131]*v1cf)}else{vb});let v1dn=(vec).sinh();let v1fk=1.0;let v1ho=(sf[92]*v1fk);let v1i9=(sf[96]*v1fk);

        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (sf[108]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[12]),
            Some(nodes[13]),
            branches[0],
            multiplicity * (sf[109]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            &[nodes[3], nodes[5], nodes[7], nodes[8]],
            &[(if (sf[77]!=0.0){((if sb[33]{((v3a*(sf[136]+(((veo*v1b7)+(vct*((sf[135]+(if sb[33]{((if sb[33]{(sf[135]*v1dw)}else{v1cj})/vei)}else{vb}))-(if sb[33]{(sf[130]+(if sb[33]{((if sb[33]{(sf[130]*v1dn)}else{v1c9})/vee)}else{vb}))}else{vb}))))/sf[32])))+sf[138])}else{vb})*v1fk)}else{vb}), (if (sf[77]!=0.0){((if sb[33]{(v3a*(((veo*v1b8)+(vct*((sf[69]+(if sb[33]{((if sb[33]{(sf[69]*v1dw)}else{v1ck})/vei)}else{vb}))-(if sb[33]{(sf[69]+(if sb[33]{((if sb[33]{(sf[69]*v1dn)}else{v1ca})/vee)}else{vb}))}else{vb}))))/sf[32]))}else{vb})*v1fk)}else{vb}), (if (sf[77]!=0.0){(v1f8*v1fk)}else{vb}), (if (sf[77]!=0.0){((if sb[33]{(v3a*((vct*(if sb[33]{((if sb[33]{vb}else{v1cl})/vei)}else{vb}))/sf[32]))}else{vb})*v1fk)}else{vb})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{(v39*(((ve3*v1b2)+(vdq*((sf[69]+(if sb[33]{(v1cj/vdx)}else{vb}))-(if sb[33]{(sf[69]+(if sb[33]{(v1c9/vdt)}else{vb}))}else{vb}))))/sf[29]))}else{vb})*v1fk)}else{vb})),
            nodes[5],
            multiplicity * ((if (sf[77]!=0.0){((if sb[33]{((v39*((((ve3*v1b3)+(vdq*((sf[131]+(if sb[33]{(v1ck/vdx)}else{vb}))-(if sb[33]{(sf[130]+(if sb[33]{(v1ca/vdt)}else{vb}))}else{vb}))))/sf[29])+sf[136]))+sf[137])}else{vb})*v1fk)}else{vb})),
            nodes[8],
            multiplicity * ((if (sf[77]!=0.0){(v1dm*v1fk)}else{vb})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * ((if sb[51]{(v1fk*((-vf0)+(v9*(if sb[33]{vb}else{(if sb[30]{(v3a*((vcy*v1b7)+(vct*(sf[135]*v1bc))))}else{vb})}))))}else{vb})),
            nodes[5],
            multiplicity * ((if sb[51]{(v1fk*(v9*(if sb[33]{vb}else{(if sb[30]{(v3a*((vcy*v1b8)+(vct*(sf[69]*v1bc))))}else{vb})})))}else{vb})),
            nodes[7],
            multiplicity * ((if sb[51]{(v1fk*(vf0+(v9*(if sb[33]{vb}else{(if sb[30]{(v3a*(vct*(sf[32]*v1bc)))}else{vb})}))))}else{vb})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * ((if sb[51]{(v1fk*(v2*(if sb[33]{vb}else{(if sb[30]{((vdc*v1b2)+(vcm*(v39*(sf[69]*v1av))))}else{vb})})))}else{vb})),
            nodes[5],
            multiplicity * ((if sb[51]{(v1fk*((-vey)+(v2*(if sb[33]{vb}else{(if sb[30]{((vdc*v1b3)+(vcm*(v39*(sf[131]*v1av))))}else{vb})}))))}else{vb})),
            nodes[8],
            multiplicity * ((if sb[51]{(v1fk*(vey+(v2*(if sb[33]{vb}else{(if sb[30]{(vcm*(v39*(sf[29]*v1av)))}else{vb})}))))}else{vb})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[1],
            multiplicity * (sf[110]),
            nodes[3],
            multiplicity * (sf[139]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (sf[111]),
            nodes[5],
            multiplicity * (sf[140]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes[3],
            multiplicity * (v3c),
            nodes[10],
            multiplicity * ((-v3c)),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * (sf[141]),
            nodes[9],
            multiplicity * (sf[112]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[6],
            multiplicity * ((if (sf[91]!=0.0){v1ho}else{vb})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[8],
            multiplicity * ((if sb[53]{v1ho}else{vb})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[2]),
            branches[13],
            multiplicity * (sf[113]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[15],
            multiplicity * ((if (sf[95]!=0.0){v1i9}else{vb})),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[3]),
            Some(nodes[0]),
            branches[17],
            multiplicity * ((if sb[55]{v1i9}else{vb})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[14],
            multiplicity * ((if sb[49]{(viy*v1fk)}else{vb})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * ((if (sf[107]!=0.0){(sf[114]*v1fk)}else{vb})),
        );
    }
}
