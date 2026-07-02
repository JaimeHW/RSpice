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
        let v1=ctx.node_voltage(nodes[2]);
        let v4=((ctx.temperature()+v1)+self.scalar_static_f64[0]);
        let v6=1300.0;
        let v7=173.14999999999998;
        let v8=(v4>v7);
        let v9=(if v8{v4}else{v7});
        let v10=(v6<v9);
        let v11=(if v10{v6}else{v9});
        let v12=1.0;
        let v13=0.0;
        let v19=8.6170869e-5;
        let v20=(v11*v19);
        let v21=(v11/self.scalar_static_f64[5]);
        let v22=(v21).ln();
        let v26=(v21-v12);
        let v27=(self.scalar_static_f64[7]*v26);
        let v33=(((v22*self.scalar_static_f64[6])+(v27/v20))).exp();
        let v34=(self.scalar_static_f64[9]*v33);
        let v36=((v22*self.scalar_static_f64[8])).exp();
        let v37=(self.scalar_static_f64[10]*v36);
        let v42=(self.scalar_static_f64[11]*(v12+(v26*self.scalar_static_f64[12])));
        let v47=(self.scalar_static_f64[13]*(v12+(v26*self.scalar_static_f64[14])));
        let v54=300.15;
        let v56=(v11/v54);
        let v58=0.000702;
        let v59=(v11*v58);
        let v60=(v11*v59);
        let v62=(v11+1108.0);
        let v65=(-(1.16-(v60/v62)));
        let v66=1.3806226e-23;
        let v68=(v66*(v11+v11));
        let v73=(-(v20+v20));
        let v74=1.5;
        let v77=1.6021918e-19;
        let v79=((v74*(v56).ln())+(((v65/v68)+1.3454442398941469e20)*v77));
        let v80=(v73*v79);
        let v83=((self.scalar_static_f64[19]-v80)/self.scalar_static_f64[18]);
        let v84=(self.scalar_static_f64[19]-v83);
        let v87=0.0004;
        let v92=(v12+(self.scalar_static_f64[20]*(self.scalar_static_f64[22]-(v84/v83))));
        let v93=(self.scalar_static_f64[17]/v92);
        let v95=(v80+(v56*v83));
        let v96=(v95-v83);
        let v102=(v12+(self.scalar_static_f64[20]*((v87*(v11-v54))-(v96/v83))));
        let v103=(v93*v102);
        let v105=ctx.node_voltage(nodes[3]);
        let v106=ctx.node_voltage(nodes[4]);
        let v107=(v105-v106);
        let v108=(self.scalar_static_f64[23]*v107);
        let v109=ctx.node_voltage(nodes[0]);
        let v110=(v109-v105);
        let v112=ctx.node_voltage(nodes[1]);
        let v113=(v112-v106);
        let v115=(v34>v13);
        let v117=(v20*self.scalar_static_f64[24]);
        let v119=(if v115{(v108/v117)}else{v13});
        let v120=(-v108);
        let v121=(v120-v47);
        let v123=(v20*self.scalar_static_f64[25]);
        let v125=(if v115{(v121/v123)}else{v13});
        let v126=(-v47);
        let v128=(if v115{(v126/v123)}else{v13});
        let v129=80.0;
        let v130=(v119>v129);
        let v131=(v115&&v130);
        let v135=(if v131{v129}else{v119});
        let v137=(v115&&(!v130));
        let v138=(if v137{v12}else{(if v131{(v12+(v119-v129))}else{v13})});
        let v139=(v135).exp();
        let v141=(if v115{(v138*v139)}else{v138});
        let v142=37.0;
        let v143=(v125>=v142);
        let v144=(!v143);
        let v145=-37.0;
        let v146=(v125<=v145);
        let v148=(v144&&(!v146));
        let v149=(v125).exp();
        let v150=(v12+v149);
        let v152=(v144&&v146);
        let v156=(v128>=v142);
        let v157=(!v156);
        let v158=(v128<=v145);
        let v160=(v157&&(!v158));
        let v161=(v128).exp();
        let v162=(v12+v161);
        let v164=(v157&&v158);
        let v169=(if v115{((if v148{(v150).ln()}else{(if v152{v149}else{(if v143{v125}else{v13})})})-(if v160{(v162).ln()}else{(if v164{v161}else{(if v156{v128}else{v13})})}))}else{v13});
        let v170=(v141-v12);
        let v172=(v42*v169);
        let v174=(v108).abs();
        let v175=f64::powf(v174,(self.scalar_static_f64[15]*(v12+(v26*self.scalar_static_f64[16]))));
        let v177=(v12+(self.scalar_static_f64[26]*v175));
        let v181=(!v115);
        let v182=(if v181{v13}else{(if v115{((v34*v170)-(v172/v177))}else{v13})});
        let v183=(v37>v13);
        let v185=(self.scalar_static_f64[27]-v108);
        let v186=0.001;
        let v187=(v185>v186);
        let v189=(if v183{(if v187{v185}else{v186})}else{v13});
        let v190=-1.0;
        let v191=(v120*self.scalar_static_f64[27]);
        let v193=(v20*self.scalar_static_f64[28]);
        let v194=(v189*v193);
        let v196=(if v183{(v191/v194)}else{v135});
        let v197=(v196>v129);
        let v198=(v183&&v197);
        let v204=(v183&&(!v197));
        let v205=(if v204{v12}else{(if v198{(v12+(v196-v129))}else{v141})});
        let v206=((if v198{v129}else{v196})).exp();
        let v209=((if v183{(v205*v206)}else{v205})-v12);
        let v212=(!v183);
        let v214=(v182-(if v212{v13}else{(if v183{(v37*v209)}else{v13})}));
        let v230=((v22*self.scalar_static_f64[34])).exp();
        let v233=f64::powf((v12+f64::powf((((self.scalar_static_f64[23]*v110)/self.scalar_static_f64[29])).abs(),self.scalar_static_f64[30])),self.scalar_static_f64[35]);
        let v234=((self.scalar_static_f64[33]*v230)*v233);
        let v238=((v22*self.scalar_static_f64[37])).exp();
        let v241=f64::powf((v12+f64::powf((((self.scalar_static_f64[23]*v113)/self.scalar_static_f64[31])).abs(),self.scalar_static_f64[32])),self.scalar_static_f64[38]);
        let v242=((self.scalar_static_f64[36]*v238)*v241);
        let v247=(if self.scalar_static_bool[0]{(v234+self.scalar_static_f64[40])}else{v234});
        let v251=(v109-v112);
        let v265=(self.scalar_static_f64[45]*(v12+((f64::powf((v12+f64::powf(((v251/self.scalar_static_f64[42])).abs(),self.scalar_static_f64[43])),self.scalar_static_f64[44])-v12)*self.scalar_static_f64[46])));
        let v269=ctx.node_voltage(nodes[6]);
        let v275=(v12+f64::powf(((v269).abs()/self.scalar_static_f64[48]),self.scalar_static_f64[49]));
        let v281=(v108+((-v95)*self.scalar_static_f64[50]));
        let v282=(v281>v13);
        let v288=(if v282{self.scalar_static_f64[55]}else{v13});
        let v291=(v12-(self.scalar_static_f64[52]*(self.scalar_static_f64[52]*v288)));
        let v298=(v281*self.scalar_static_f64[57]);
        let v300=(self.scalar_static_f64[52]+(v298/v95));
        let v304=(!v282);
        let v306=(v12-(v108/v95));
        let v309=((self.scalar_static_f64[56]*(v306).ln())).exp();
        let v310=(v12-v309);
        let v315=((if v304{((v95*v310)/self.scalar_static_f64[56])}else{(if v282{((v95*v291)/self.scalar_static_f64[56])}else{v13})})+(if v304{v13}else{(if v282{(v288*(v281*v300))}else{v13})}));
        let v339=((if self.scalar_static_bool[1]{(v247/v275)}else{v247})/self.scalar_static_f64[3]);
        let v343=((if self.scalar_static_bool[0]{(v242+self.scalar_static_f64[41])}else{v242})/self.scalar_static_f64[3]);
        let v348=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v269);
        let v353=(-((v214*v251)).abs());
        let v358=(v1*self.scalar_static_f64[68]);
        let v359=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v358);
        let v364=ctx.node_voltage(nodes[5]);
        let v368=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v358);
        let v374=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (v364*self.scalar_static_f64[69]));
        let v381=(v339>self.scalar_static_f64[67]);
        let v382=(if v381{v339}else{self.scalar_static_f64[67]});
        let v385=(v343>self.scalar_static_f64[67]);
        let v386=(if v385{v343}else{self.scalar_static_f64[67]});
        let v396=(if v10{v13}else{(if v8{v12}else{v13})});
        let v397=(v19*v396);
        let v398=(v396/self.scalar_static_f64[5]);
        let v399=(v398/v21);
        let v419=(v396/v54);
        let v444=((v79*(-(v397+v397)))+(v73*((v74*(v419/v56))+(v77*(((v68*(((v62*((v59*v396)+(v11*(v58*v396))))-(v60*v396))/(v62*v62)))-(v65*(v66*(v396+v396))))/(v68*v68))))));
        let v446=((-v444)/self.scalar_static_f64[18]);
        let v451=(v83*v83);
        let v462=(v444+((v83*v419)+(v56*v446)));
        let v482=(if v115{((-(v108*(self.scalar_static_f64[24]*v397)))/(v117*v117))}else{v13});
        let v483=(if v115{(self.scalar_static_f64[23]/v117)}else{v13});
        let v484=(if v115{(self.scalar_static_f64[70]/v117)}else{v13});
        let v486=(self.scalar_static_f64[25]*v397);
        let v487=(v123*(-(self.scalar_static_f64[13]*(self.scalar_static_f64[14]*v398))));
        let v490=(v123*v123);
        let v494=(if v115{((v487-(v121*v486))/v490)}else{v13});
        let v495=(if v115{(self.scalar_static_f64[70]/v123)}else{v13});
        let v496=(if v115{(self.scalar_static_f64[23]/v123)}else{v13});
        let v500=(if v115{((v487-(v126*v486))/v490)}else{v13});
        let v504=(if v131{v13}else{v482});
        let v505=(if v131{v13}else{v483});
        let v506=(if v131{v13}else{v484});
        let v507=(if v137{v13}else{(if v131{v482}else{v13})});
        let v508=(if v137{v13}else{(if v131{v483}else{v13})});
        let v509=(if v137{v13}else{(if v131{v484}else{v13})});
        let v522=(if v115{((v139*v507)+(v138*(v139*v504)))}else{v507});
        let v523=(if v115{((v139*v508)+(v138*(v139*v505)))}else{v508});
        let v524=(if v115{((v139*v509)+(v138*(v139*v506)))}else{v509});
        let v525=(v149*v494);
        let v526=(v149*v495);
        let v527=(v149*v496);
        let v540=(v161*v500);
        let v576=(if v181{v13}else{(if v115{(((v170*(self.scalar_static_f64[9]*(v33*((self.scalar_static_f64[6]*v399)+(((v20*(self.scalar_static_f64[7]*v398))-(v27*v397))/(v20*v20))))))+(v34*v522))-(((v177*((v169*(self.scalar_static_f64[11]*(self.scalar_static_f64[12]*v398)))+(v42*(if v115{((if v148{(v525/v150)}else{(if v152{v525}else{(if v143{v494}else{v13})})})-(if v160{(v540/v162)}else{(if v164{v540}else{(if v156{v500}else{v13})})}))}else{v13}))))-(v172*(self.scalar_static_f64[26]*((self.scalar_static_f64[15]*(self.scalar_static_f64[16]*v398))*(v175*(v174).ln())))))/(v177*v177)))}else{v13})});
        let v577=(if v181{v13}else{(if v115{((v34*v523)-((v42*(if v115{(if v148{(v526/v150)}else{(if v152{v526}else{(if v143{v495}else{v13})})})}else{v13}))/v177))}else{v13})});
        let v578=(if v181{v13}else{(if v115{((v34*v524)-((v42*(if v115{(if v148{(v527/v150)}else{(if v152{v527}else{(if v143{v496}else{v13})})})}else{v13}))/v177))}else{v13})});
        let v591=(v194*v194);
        let v601=(if v183{((-(v191*(v189*(self.scalar_static_f64[28]*v397))))/v591)}else{v504});
        let v602=(if v183{(((v194*self.scalar_static_f64[71])-(v191*(v193*(if v183{(if v187{self.scalar_static_f64[70]}else{v13})}else{v13}))))/v591)}else{v505});
        let v603=(if v183{(((v194*self.scalar_static_f64[72])-(v191*(v193*(if v183{(if v187{self.scalar_static_f64[23]}else{v13})}else{v13}))))/v591)}else{v506});
        let v610=(if v204{v13}else{(if v198{v601}else{v522})});
        let v611=(if v204{v13}else{(if v198{v602}else{v523})});
        let v612=(if v204{v13}else{(if v198{v603}else{v524})});
        let v645=(v233*(self.scalar_static_f64[33]*(v230*(self.scalar_static_f64[34]*v399))));
        let v656=(self.scalar_static_f64[50]*(-v462));
        let v666=(v95*v95);
        let v739=ddt_scale;
        let v744=(self.scalar_static_f64[68]*v739);

        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * ((if self.scalar_static_bool[1]{(v265*(-v182))}else{v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[1]{(v265*(-v576))}else{v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[1]{(v265*(-v577))}else{v13})),
            4,
            multiplicity * ((if self.scalar_static_bool[1]{(v265*(-v578))}else{v13})),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if self.scalar_static_bool[1]{v269}else{v13})),
            6,
            multiplicity * (self.scalar_static_f64[75]),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if self.scalar_static_bool[1]{(v265*v348)}else{v13})),
            6,
            multiplicity * ((if self.scalar_static_bool[1]{(v265*v739)}else{v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(6),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[4]{v353}else{v13})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[4]{(v1/self.scalar_static_f64[59])}else{v13})),
            2,
            multiplicity * (self.scalar_static_f64[77]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[4]{v359}else{v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[4]{v744}else{v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v13,
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{v353}else{v13})),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[17]{((v1-v364)/self.scalar_static_f64[59])}else{v13})),
            2,
            multiplicity * (self.scalar_static_f64[79]),
            5,
            multiplicity * (self.scalar_static_f64[80]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{v368}else{v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[17]{v744}else{v13})),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{(v364/self.scalar_static_f64[60])}else{v13})),
            5,
            multiplicity * (self.scalar_static_f64[82]),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{v374}else{v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[69]*v739)}else{v13})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[20]{v353}else{v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            v13,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            v13,
        );
        stamper.stamp_current_node1_local(
            Some(3),
            Some(4),
            multiplicity * ((v13*v107)),
            4,
            multiplicity * (-0.0),
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(3),
            multiplicity * ((if self.scalar_static_bool[12]{(v110/v382)}else{v13})),
            0,
            multiplicity * ((if self.scalar_static_bool[12]{(v12/v382)}else{v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[12]{((-(v110*(if v381{((if self.scalar_static_bool[1]{(v645/v275)}else{v645})/self.scalar_static_f64[3])}else{v13})))/(v382*v382))}else{v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[12]{(v190/v382)}else{v13})),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(3),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            v13,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(4),
            multiplicity * ((if self.scalar_static_bool[15]{(v113/v386)}else{v13})),
            1,
            multiplicity * ((if self.scalar_static_bool[15]{(v12/v386)}else{v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[15]{((-(v113*(if v385{((v241*(self.scalar_static_f64[36]*(v238*(self.scalar_static_f64[37]*v399))))/self.scalar_static_f64[3])}else{v13})))/(v386*v386))}else{v13})),
            4,
            multiplicity * ((if self.scalar_static_bool[15]{(v190/v386)}else{v13})),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (v13),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(4),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            v13,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*v214))),
            2,
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v576-(if v212{v13}else{(if v183{((v209*(self.scalar_static_f64[10]*(v36*(self.scalar_static_f64[8]*v399))))+(v37*(if v183{((v206*v610)+(v205*(v206*(if v198{v13}else{v601}))))}else{v610})))}else{v13})}))))),
            3,
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v577-(if v212{v13}else{(if v183{(v37*(if v183{((v206*v611)+(v205*(v206*(if v198{v13}else{v602}))))}else{v611}))}else{v13})}))))),
            4,
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v578-(if v212{v13}else{(if v183{(v37*(if v183{((v206*v612)+(v205*(v206*(if v198{v13}else{v603}))))}else{v612}))}else{v13})}))))),
        );
        let v392_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*v315))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v392_ddt),
            2,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*((v315*((v102*((-(self.scalar_static_f64[17]*(self.scalar_static_f64[20]*(-(((v83*(-v446))-(v84*v446))/v451)))))/(v92*v92)))+(v93*(self.scalar_static_f64[20]*((v87*v396)-(((v83*(v462-v446))-(v96*v446))/v451))))))+(v103*((if v304{(((v310*v462)+(v95*(-(v309*(self.scalar_static_f64[56]*((-((-(v108*v462))/v666))/v306))))))/self.scalar_static_f64[56])}else{(if v282{((v291*v462)/self.scalar_static_f64[56])}else{v13})})+(if v304{v13}else{(if v282{(v288*((v300*v656)+(v281*(((v95*(self.scalar_static_f64[57]*v656))-(v298*v462))/v666))))}else{v13})}))))))) * ddt_scale)),
            3,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v304{((v95*(-(v309*(self.scalar_static_f64[56]*((-(self.scalar_static_f64[23]/v95))/v306)))))/self.scalar_static_f64[56])}else{v13})+(if v304{v13}else{(if v282{(v288*((self.scalar_static_f64[23]*v300)+(v281*(self.scalar_static_f64[73]/v95))))}else{v13})})))))) * ddt_scale)),
            4,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v304{((v95*(-(v309*(self.scalar_static_f64[56]*((-(self.scalar_static_f64[70]/v95))/v306)))))/self.scalar_static_f64[56])}else{v13})+(if v304{v13}else{(if v282{(v288*((v300*self.scalar_static_f64[70])+(v281*(self.scalar_static_f64[74]/v95))))}else{v13})})))))) * ddt_scale)),
        );
        let v394_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v182*v265))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v394_ddt),
            2,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v265*v576)))) * ddt_scale)),
            3,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v265*v577)))) * ddt_scale)),
            4,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v265*v578)))) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (v13),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (v13),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v1=ctx.node_voltage(nodes[2]);
        let v4=((ctx.temperature()+v1)+self.scalar_static_f64[0]);
        let v6=1300.0;
        let v7=173.14999999999998;
        let v8=(v4>v7);
        let v9=(if v8{v4}else{v7});
        let v10=(v6<v9);
        let v11=(if v10{v6}else{v9});
        let v12=1.0;
        let v13=0.0;
        let v19=8.6170869e-5;
        let v20=(v11*v19);
        let v21=(v11/self.scalar_static_f64[5]);
        let v26=(v21-v12);
        let v27=(self.scalar_static_f64[7]*v26);
        let v33=((((v21).ln()*self.scalar_static_f64[6])+(v27/v20))).exp();
        let v34=(self.scalar_static_f64[9]*v33);
        let v42=(self.scalar_static_f64[11]*(v12+(v26*self.scalar_static_f64[12])));
        let v47=(self.scalar_static_f64[13]*(v12+(v26*self.scalar_static_f64[14])));
        let v54=300.15;
        let v56=(v11/v54);
        let v58=0.000702;
        let v59=(v11*v58);
        let v60=(v11*v59);
        let v62=(v11+1108.0);
        let v65=(-(1.16-(v60/v62)));
        let v66=1.3806226e-23;
        let v68=(v66*(v11+v11));
        let v73=(-(v20+v20));
        let v74=1.5;
        let v77=1.6021918e-19;
        let v79=((v74*(v56).ln())+(((v65/v68)+1.3454442398941469e20)*v77));
        let v80=(v73*v79);
        let v83=((self.scalar_static_f64[19]-v80)/self.scalar_static_f64[18]);
        let v84=(self.scalar_static_f64[19]-v83);
        let v87=0.0004;
        let v92=(v12+(self.scalar_static_f64[20]*(self.scalar_static_f64[22]-(v84/v83))));
        let v93=(self.scalar_static_f64[17]/v92);
        let v95=(v80+(v56*v83));
        let v96=(v95-v83);
        let v102=(v12+(self.scalar_static_f64[20]*((v87*(v11-v54))-(v96/v83))));
        let v103=(v93*v102);
        let v108=(self.scalar_static_f64[23]*(ctx.node_voltage(nodes[3])-ctx.node_voltage(nodes[4])));
        let v115=(v34>v13);
        let v117=(v20*self.scalar_static_f64[24]);
        let v119=(if v115{(v108/v117)}else{v13});
        let v121=((-v108)-v47);
        let v123=(v20*self.scalar_static_f64[25]);
        let v125=(if v115{(v121/v123)}else{v13});
        let v126=(-v47);
        let v128=(if v115{(v126/v123)}else{v13});
        let v129=80.0;
        let v130=(v119>v129);
        let v131=(v115&&v130);
        let v137=(v115&&(!v130));
        let v138=(if v137{v12}else{(if v131{(v12+(v119-v129))}else{v13})});
        let v139=((if v131{v129}else{v119})).exp();
        let v142=37.0;
        let v143=(v125>=v142);
        let v144=(!v143);
        let v145=-37.0;
        let v146=(v125<=v145);
        let v148=(v144&&(!v146));
        let v149=(v125).exp();
        let v150=(v12+v149);
        let v152=(v144&&v146);
        let v156=(v128>=v142);
        let v157=(!v156);
        let v158=(v128<=v145);
        let v160=(v157&&(!v158));
        let v161=(v128).exp();
        let v162=(v12+v161);
        let v164=(v157&&v158);
        let v169=(if v115{((if v148{(v150).ln()}else{(if v152{v149}else{(if v143{v125}else{v13})})})-(if v160{(v162).ln()}else{(if v164{v161}else{(if v156{v128}else{v13})})}))}else{v13});
        let v170=((if v115{(v138*v139)}else{v138})-v12);
        let v172=(v42*v169);
        let v174=(v108).abs();
        let v175=f64::powf(v174,(self.scalar_static_f64[15]*(v12+(v26*self.scalar_static_f64[16]))));
        let v177=(v12+(self.scalar_static_f64[26]*v175));
        let v181=(!v115);
        let v265=(self.scalar_static_f64[45]*(v12+((f64::powf((v12+f64::powf((((ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]))/self.scalar_static_f64[42])).abs(),self.scalar_static_f64[43])),self.scalar_static_f64[44])-v12)*self.scalar_static_f64[46])));
        let v281=(v108+((-v95)*self.scalar_static_f64[50]));
        let v282=(v281>v13);
        let v288=(if v282{self.scalar_static_f64[55]}else{v13});
        let v291=(v12-(self.scalar_static_f64[52]*(self.scalar_static_f64[52]*v288)));
        let v298=(v281*self.scalar_static_f64[57]);
        let v300=(self.scalar_static_f64[52]+(v298/v95));
        let v304=(!v282);
        let v306=(v12-(v108/v95));
        let v309=((self.scalar_static_f64[56]*(v306).ln())).exp();
        let v310=(v12-v309);
        let v315=((if v304{((v95*v310)/self.scalar_static_f64[56])}else{(if v282{((v95*v291)/self.scalar_static_f64[56])}else{v13})})+(if v304{v13}else{(if v282{(v288*(v281*v300))}else{v13})}));
        let v348=0.0;
        let v358=(v1*self.scalar_static_f64[68]);
        let v359=0.0;
        let v368=0.0;
        let v374=0.0;
        let v396=(if v10{v13}else{(if v8{v12}else{v13})});
        let v397=(v19*v396);
        let v398=(v396/self.scalar_static_f64[5]);
        let v419=(v396/v54);
        let v444=((v79*(-(v397+v397)))+(v73*((v74*(v419/v56))+(v77*(((v68*(((v62*((v59*v396)+(v11*(v58*v396))))-(v60*v396))/(v62*v62)))-(v65*(v66*(v396+v396))))/(v68*v68))))));
        let v446=((-v444)/self.scalar_static_f64[18]);
        let v451=(v83*v83);
        let v462=(v444+((v83*v419)+(v56*v446)));
        let v482=(if v115{((-(v108*(self.scalar_static_f64[24]*v397)))/(v117*v117))}else{v13});
        let v483=(if v115{(self.scalar_static_f64[23]/v117)}else{v13});
        let v484=(if v115{(self.scalar_static_f64[70]/v117)}else{v13});
        let v486=(self.scalar_static_f64[25]*v397);
        let v487=(v123*(-(self.scalar_static_f64[13]*(self.scalar_static_f64[14]*v398))));
        let v490=(v123*v123);
        let v494=(if v115{((v487-(v121*v486))/v490)}else{v13});
        let v495=(if v115{(self.scalar_static_f64[70]/v123)}else{v13});
        let v496=(if v115{(self.scalar_static_f64[23]/v123)}else{v13});
        let v500=(if v115{((v487-(v126*v486))/v490)}else{v13});
        let v507=(if v137{v13}else{(if v131{v482}else{v13})});
        let v508=(if v137{v13}else{(if v131{v483}else{v13})});
        let v509=(if v137{v13}else{(if v131{v484}else{v13})});
        let v525=(v149*v494);
        let v526=(v149*v495);
        let v527=(v149*v496);
        let v540=(v161*v500);
        let v656=(self.scalar_static_f64[50]*(-v462));
        let v666=(v95*v95);
        let v739=1.0;
        let v744=(self.scalar_static_f64[68]*v739);

        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * ((if self.scalar_static_bool[1]{(v265*v739)}else{v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if self.scalar_static_bool[4]{v744}else{v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if self.scalar_static_bool[17]{v744}else{v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[69]*v739)}else{v13})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*((v315*((v102*((-(self.scalar_static_f64[17]*(self.scalar_static_f64[20]*(-(((v83*(-v446))-(v84*v446))/v451)))))/(v92*v92)))+(v93*(self.scalar_static_f64[20]*((v87*v396)-(((v83*(v462-v446))-(v96*v446))/v451))))))+(v103*((if v304{(((v310*v462)+(v95*(-(v309*(self.scalar_static_f64[56]*((-((-(v108*v462))/v666))/v306))))))/self.scalar_static_f64[56])}else{(if v282{((v291*v462)/self.scalar_static_f64[56])}else{v13})})+(if v304{v13}else{(if v282{(v288*((v300*v656)+(v281*(((v95*(self.scalar_static_f64[57]*v656))-(v298*v462))/v666))))}else{v13})}))))))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v304{((v95*(-(v309*(self.scalar_static_f64[56]*((-(self.scalar_static_f64[23]/v95))/v306)))))/self.scalar_static_f64[56])}else{v13})+(if v304{v13}else{(if v282{(v288*((self.scalar_static_f64[23]*v300)+(v281*(self.scalar_static_f64[73]/v95))))}else{v13})})))))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v304{((v95*(-(v309*(self.scalar_static_f64[56]*((-(self.scalar_static_f64[70]/v95))/v306)))))/self.scalar_static_f64[56])}else{v13})+(if v304{v13}else{(if v282{(v288*((v300*self.scalar_static_f64[70])+(v281*(self.scalar_static_f64[74]/v95))))}else{v13})})))))),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v265*(if v181{v13}else{(if v115{(((v170*(self.scalar_static_f64[9]*(v33*((self.scalar_static_f64[6]*(v398/v21))+(((v20*(self.scalar_static_f64[7]*v398))-(v27*v397))/(v20*v20))))))+(v34*(if v115{((v139*v507)+(v138*(v139*(if v131{v13}else{v482}))))}else{v507})))-(((v177*((v169*(self.scalar_static_f64[11]*(self.scalar_static_f64[12]*v398)))+(v42*(if v115{((if v148{(v525/v150)}else{(if v152{v525}else{(if v143{v494}else{v13})})})-(if v160{(v540/v162)}else{(if v164{v540}else{(if v156{v500}else{v13})})}))}else{v13}))))-(v172*(self.scalar_static_f64[26]*((self.scalar_static_f64[15]*(self.scalar_static_f64[16]*v398))*(v175*(v174).ln())))))/(v177*v177)))}else{v13})}))))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v265*(if v181{v13}else{(if v115{((v34*(if v115{((v139*v508)+(v138*(v139*(if v131{v13}else{v483}))))}else{v508}))-((v42*(if v115{(if v148{(v526/v150)}else{(if v152{v526}else{(if v143{v495}else{v13})})})}else{v13}))/v177))}else{v13})}))))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v265*(if v181{v13}else{(if v115{((v34*(if v115{((v139*v509)+(v138*(v139*(if v131{v13}else{v484}))))}else{v509}))-((v42*(if v115{(if v148{(v527/v150)}else{(if v152{v527}else{(if v143{v496}else{v13})})})}else{v13}))/v177))}else{v13})}))))),
        );
    }
}
