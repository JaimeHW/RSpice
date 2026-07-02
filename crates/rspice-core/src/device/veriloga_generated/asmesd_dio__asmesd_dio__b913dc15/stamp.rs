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
        let v116=(if (v34>v13){v12}else{v13});
        let v118=(v20*self.scalar_static_f64[24]);
        let v120=(if (v116!=0.0){(v108/v118)}else{v13});
        let v121=(-v108);
        let v122=(v121-v47);
        let v124=(v20*self.scalar_static_f64[25]);
        let v126=(if (v116!=0.0){(v122/v124)}else{v13});
        let v127=(-v47);
        let v129=(if (v116!=0.0){(v127/v124)}else{v13});
        let v130=80.0;
        let v132=(if (v120>v130){v12}else{v13});
        let v133=((v116!=0.0)&&(v132!=0.0));
        let v137=(if v133{v130}else{v120});
        let v139=((v116!=0.0)&&(!(v132!=0.0)));
        let v140=(if v139{v12}else{(if v133{(v12+(v120-v130))}else{v13})});
        let v141=(v137).exp();
        let v143=(if (v116!=0.0){(v140*v141)}else{v140});
        let v144=37.0;
        let v145=(v126>=v144);
        let v146=(!v145);
        let v147=-37.0;
        let v148=(v126<=v147);
        let v150=(v146&&(!v148));
        let v151=(v126).exp();
        let v152=(v12+v151);
        let v154=(v146&&v148);
        let v158=(v129>=v144);
        let v159=(!v158);
        let v160=(v129<=v147);
        let v162=(v159&&(!v160));
        let v163=(v129).exp();
        let v164=(v12+v163);
        let v166=(v159&&v160);
        let v171=(if (v116!=0.0){((if v150{(v152).ln()}else{(if v154{v151}else{(if v145{v126}else{v13})})})-(if v162{(v164).ln()}else{(if v166{v163}else{(if v158{v129}else{v13})})}))}else{v13});
        let v172=(v143-v12);
        let v174=(v42*v171);
        let v176=(v108).abs();
        let v177=f64::powf(v176,(self.scalar_static_f64[15]*(v12+(v26*self.scalar_static_f64[16]))));
        let v179=(v12+(self.scalar_static_f64[26]*v177));
        let v183=(!(v116!=0.0));
        let v184=(if v183{v13}else{(if (v116!=0.0){((v34*v172)-(v174/v179))}else{v13})});
        let v186=(if (v37>v13){v12}else{v13});
        let v188=(self.scalar_static_f64[27]-v108);
        let v189=0.001;
        let v190=(v188>v189);
        let v192=(if (v186!=0.0){(if v190{v188}else{v189})}else{v13});
        let v193=-1.0;
        let v194=(v121*self.scalar_static_f64[27]);
        let v196=(v20*self.scalar_static_f64[28]);
        let v197=(v192*v196);
        let v199=(if (v186!=0.0){(v194/v197)}else{v137});
        let v201=(if (v199>v130){v12}else{v13});
        let v202=((v186!=0.0)&&(v201!=0.0));
        let v208=((v186!=0.0)&&(!(v201!=0.0)));
        let v209=(if v208{v12}else{(if v202{(v12+(v199-v130))}else{v143})});
        let v210=((if v202{v130}else{v199})).exp();
        let v213=((if (v186!=0.0){(v209*v210)}else{v209})-v12);
        let v216=(!(v186!=0.0));
        let v218=(v184-(if v216{v13}else{(if (v186!=0.0){(v37*v213)}else{v13})}));
        let v234=((v22*self.scalar_static_f64[34])).exp();
        let v237=f64::powf((v12+f64::powf((((self.scalar_static_f64[23]*v110)/self.scalar_static_f64[29])).abs(),self.scalar_static_f64[30])),self.scalar_static_f64[35]);
        let v238=((self.scalar_static_f64[33]*v234)*v237);
        let v242=((v22*self.scalar_static_f64[37])).exp();
        let v245=f64::powf((v12+f64::powf((((self.scalar_static_f64[23]*v113)/self.scalar_static_f64[31])).abs(),self.scalar_static_f64[32])),self.scalar_static_f64[38]);
        let v246=((self.scalar_static_f64[36]*v242)*v245);
        let v252=(if (self.scalar_static_f64[40]!=0.0){(v238+self.scalar_static_f64[41])}else{v238});
        let v256=(v109-v112);
        let v270=(self.scalar_static_f64[46]*(v12+((f64::powf((v12+f64::powf(((v256/self.scalar_static_f64[43])).abs(),self.scalar_static_f64[44])),self.scalar_static_f64[45])-v12)*self.scalar_static_f64[47])));
        let v275=ctx.node_voltage(nodes[6]);
        let v281=(v12+f64::powf(((v275).abs()/self.scalar_static_f64[50]),self.scalar_static_f64[51]));
        let v287=(v108+((-v95)*self.scalar_static_f64[52]));
        let v289=(if (v287>v13){v12}else{v13});
        let v295=(if (v289!=0.0){self.scalar_static_f64[57]}else{v13});
        let v298=(v12-(self.scalar_static_f64[54]*(self.scalar_static_f64[54]*v295)));
        let v305=(v287*self.scalar_static_f64[59]);
        let v307=(self.scalar_static_f64[54]+(v305/v95));
        let v311=(!(v289!=0.0));
        let v313=(v12-(v108/v95));
        let v316=((self.scalar_static_f64[58]*(v313).ln())).exp();
        let v317=(v12-v316);
        let v322=((if v311{((v95*v317)/self.scalar_static_f64[58])}else{(if (v289!=0.0){((v95*v298)/self.scalar_static_f64[58])}else{v13})})+(if v311{v13}else{(if (v289!=0.0){(v295*(v287*v307))}else{v13})}));
        let v350=((if (self.scalar_static_f64[49]!=0.0){(v252/v281)}else{v252})/self.scalar_static_f64[3]);
        let v355=((if (self.scalar_static_f64[40]!=0.0){(v246+self.scalar_static_f64[42])}else{v246})/self.scalar_static_f64[3]);
        let v360=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, v275);
        let v365=(-((v218*v256)).abs());
        let v370=(v1*self.scalar_static_f64[75]);
        let v371=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, v370);
        let v376=ctx.node_voltage(nodes[5]);
        let v380=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, v370);
        let v386=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (v376*self.scalar_static_f64[76]));
        let v393=(v350>self.scalar_static_f64[72]);
        let v394=(if v393{v350}else{self.scalar_static_f64[72]});
        let v397=(v355>self.scalar_static_f64[72]);
        let v398=(if v397{v355}else{self.scalar_static_f64[72]});
        let v408=(if v10{v13}else{(if v8{v12}else{v13})});
        let v409=(v19*v408);
        let v410=(v408/self.scalar_static_f64[5]);
        let v411=(v410/v21);
        let v431=(v408/v54);
        let v456=((v79*(-(v409+v409)))+(v73*((v74*(v431/v56))+(v77*(((v68*(((v62*((v59*v408)+(v11*(v58*v408))))-(v60*v408))/(v62*v62)))-(v65*(v66*(v408+v408))))/(v68*v68))))));
        let v458=((-v456)/self.scalar_static_f64[18]);
        let v463=(v83*v83);
        let v474=(v456+((v83*v431)+(v56*v458)));
        let v494=(if (v116!=0.0){((-(v108*(self.scalar_static_f64[24]*v409)))/(v118*v118))}else{v13});
        let v495=(if (v116!=0.0){(self.scalar_static_f64[23]/v118)}else{v13});
        let v496=(if (v116!=0.0){(self.scalar_static_f64[77]/v118)}else{v13});
        let v498=(self.scalar_static_f64[25]*v409);
        let v499=(v124*(-(self.scalar_static_f64[13]*(self.scalar_static_f64[14]*v410))));
        let v502=(v124*v124);
        let v506=(if (v116!=0.0){((v499-(v122*v498))/v502)}else{v13});
        let v507=(if (v116!=0.0){(self.scalar_static_f64[77]/v124)}else{v13});
        let v508=(if (v116!=0.0){(self.scalar_static_f64[23]/v124)}else{v13});
        let v512=(if (v116!=0.0){((v499-(v127*v498))/v502)}else{v13});
        let v516=(if v133{v13}else{v494});
        let v517=(if v133{v13}else{v495});
        let v518=(if v133{v13}else{v496});
        let v519=(if v139{v13}else{(if v133{v494}else{v13})});
        let v520=(if v139{v13}else{(if v133{v495}else{v13})});
        let v521=(if v139{v13}else{(if v133{v496}else{v13})});
        let v534=(if (v116!=0.0){((v141*v519)+(v140*(v141*v516)))}else{v519});
        let v535=(if (v116!=0.0){((v141*v520)+(v140*(v141*v517)))}else{v520});
        let v536=(if (v116!=0.0){((v141*v521)+(v140*(v141*v518)))}else{v521});
        let v537=(v151*v506);
        let v538=(v151*v507);
        let v539=(v151*v508);
        let v552=(v163*v512);
        let v588=(if v183{v13}else{(if (v116!=0.0){(((v172*(self.scalar_static_f64[9]*(v33*((self.scalar_static_f64[6]*v411)+(((v20*(self.scalar_static_f64[7]*v410))-(v27*v409))/(v20*v20))))))+(v34*v534))-(((v179*((v171*(self.scalar_static_f64[11]*(self.scalar_static_f64[12]*v410)))+(v42*(if (v116!=0.0){((if v150{(v537/v152)}else{(if v154{v537}else{(if v145{v506}else{v13})})})-(if v162{(v552/v164)}else{(if v166{v552}else{(if v158{v512}else{v13})})}))}else{v13}))))-(v174*(self.scalar_static_f64[26]*((self.scalar_static_f64[15]*(self.scalar_static_f64[16]*v410))*(v177*(v176).ln())))))/(v179*v179)))}else{v13})});
        let v589=(if v183{v13}else{(if (v116!=0.0){((v34*v535)-((v42*(if (v116!=0.0){(if v150{(v538/v152)}else{(if v154{v538}else{(if v145{v507}else{v13})})})}else{v13}))/v179))}else{v13})});
        let v590=(if v183{v13}else{(if (v116!=0.0){((v34*v536)-((v42*(if (v116!=0.0){(if v150{(v539/v152)}else{(if v154{v539}else{(if v145{v508}else{v13})})})}else{v13}))/v179))}else{v13})});
        let v603=(v197*v197);
        let v613=(if (v186!=0.0){((-(v194*(v192*(self.scalar_static_f64[28]*v409))))/v603)}else{v516});
        let v614=(if (v186!=0.0){(((v197*self.scalar_static_f64[78])-(v194*(v196*(if (v186!=0.0){(if v190{self.scalar_static_f64[77]}else{v13})}else{v13}))))/v603)}else{v517});
        let v615=(if (v186!=0.0){(((v197*self.scalar_static_f64[79])-(v194*(v196*(if (v186!=0.0){(if v190{self.scalar_static_f64[23]}else{v13})}else{v13}))))/v603)}else{v518});
        let v622=(if v208{v13}else{(if v202{v613}else{v534})});
        let v623=(if v208{v13}else{(if v202{v614}else{v535})});
        let v624=(if v208{v13}else{(if v202{v615}else{v536})});
        let v657=(v237*(self.scalar_static_f64[33]*(v234*(self.scalar_static_f64[34]*v411))));
        let v668=(self.scalar_static_f64[52]*(-v474));
        let v678=(v95*v95);
        let v751=ddt_scale;
        let v756=(self.scalar_static_f64[75]*v751);

        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*(-v184))}else{v13})),
            2,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*(-v588))}else{v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*(-v589))}else{v13})),
            4,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*(-v590))}else{v13})),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){v275}else{v13})),
            6,
            multiplicity * (self.scalar_static_f64[82]),
        );
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*v360)}else{v13})),
            6,
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*v751)}else{v13})),
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
            multiplicity * ((if (self.scalar_static_f64[62]!=0.0){v365}else{v13})),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[62]!=0.0){(v1/self.scalar_static_f64[61])}else{v13})),
            2,
            multiplicity * (self.scalar_static_f64[84]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if (self.scalar_static_f64[62]!=0.0){v371}else{v13})),
            2,
            multiplicity * ((if (self.scalar_static_f64[62]!=0.0){v756}else{v13})),
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
            multiplicity * ((if self.scalar_static_bool[17]{v365}else{v13})),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[17]{((v1-v376)/self.scalar_static_f64[61])}else{v13})),
            2,
            multiplicity * (self.scalar_static_f64[86]),
            5,
            multiplicity * (self.scalar_static_f64[87]),
        );
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{v380}else{v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[17]{v756}else{v13})),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{(v376/self.scalar_static_f64[63])}else{v13})),
            5,
            multiplicity * (self.scalar_static_f64[89]),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * ((if self.scalar_static_bool[17]{v386}else{v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[76]*v751)}else{v13})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            None,
            multiplicity * ((if self.scalar_static_bool[20]{v365}else{v13})),
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
            multiplicity * ((if (self.scalar_static_f64[73]!=0.0){(v110/v394)}else{v13})),
            0,
            multiplicity * ((if (self.scalar_static_f64[73]!=0.0){(v12/v394)}else{v13})),
            2,
            multiplicity * ((if (self.scalar_static_f64[73]!=0.0){((-(v110*(if v393{((if (self.scalar_static_f64[49]!=0.0){(v657/v281)}else{v657})/self.scalar_static_f64[3])}else{v13})))/(v394*v394))}else{v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[73]!=0.0){(v193/v394)}else{v13})),
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
            multiplicity * ((if (self.scalar_static_f64[74]!=0.0){(v113/v398)}else{v13})),
            1,
            multiplicity * ((if (self.scalar_static_f64[74]!=0.0){(v12/v398)}else{v13})),
            2,
            multiplicity * ((if (self.scalar_static_f64[74]!=0.0){((-(v113*(if v397{((v245*(self.scalar_static_f64[36]*(v242*(self.scalar_static_f64[37]*v411))))/self.scalar_static_f64[3])}else{v13})))/(v398*v398))}else{v13})),
            4,
            multiplicity * ((if (self.scalar_static_f64[74]!=0.0){(v193/v398)}else{v13})),
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
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*v218))),
            2,
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v588-(if v216{v13}else{(if (v186!=0.0){((v213*(self.scalar_static_f64[10]*(v36*(self.scalar_static_f64[8]*v411))))+(v37*(if (v186!=0.0){((v210*v622)+(v209*(v210*(if v202{v13}else{v613}))))}else{v622})))}else{v13})}))))),
            3,
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v589-(if v216{v13}else{(if (v186!=0.0){(v37*(if (v186!=0.0){((v210*v623)+(v209*(v210*(if v202{v13}else{v614}))))}else{v623}))}else{v13})}))))),
            4,
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v590-(if v216{v13}else{(if (v186!=0.0){(v37*(if (v186!=0.0){((v210*v624)+(v209*(v210*(if v202{v13}else{v615}))))}else{v624}))}else{v13})}))))),
        );
        let v404_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*v322))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v404_ddt),
            2,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*((v322*((v102*((-(self.scalar_static_f64[17]*(self.scalar_static_f64[20]*(-(((v83*(-v458))-(v84*v458))/v463)))))/(v92*v92)))+(v93*(self.scalar_static_f64[20]*((v87*v408)-(((v83*(v474-v458))-(v96*v458))/v463))))))+(v103*((if v311{(((v317*v474)+(v95*(-(v316*(self.scalar_static_f64[58]*((-((-(v108*v474))/v678))/v313))))))/self.scalar_static_f64[58])}else{(if (v289!=0.0){((v298*v474)/self.scalar_static_f64[58])}else{v13})})+(if v311{v13}else{(if (v289!=0.0){(v295*((v307*v668)+(v287*(((v95*(self.scalar_static_f64[59]*v668))-(v305*v474))/v678))))}else{v13})}))))))) * ddt_scale)),
            3,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v311{((v95*(-(v316*(self.scalar_static_f64[58]*((-(self.scalar_static_f64[23]/v95))/v313)))))/self.scalar_static_f64[58])}else{v13})+(if v311{v13}else{(if (v289!=0.0){(v295*((self.scalar_static_f64[23]*v307)+(v287*(self.scalar_static_f64[80]/v95))))}else{v13})})))))) * ddt_scale)),
            4,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v311{((v95*(-(v316*(self.scalar_static_f64[58]*((-(self.scalar_static_f64[77]/v95))/v313)))))/self.scalar_static_f64[58])}else{v13})+(if v311{v13}else{(if (v289!=0.0){(v295*((v307*self.scalar_static_f64[77])+(v287*(self.scalar_static_f64[81]/v95))))}else{v13})})))))) * ddt_scale)),
        );
        let v406_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v184*v270))));
        stamper.stamp_current_node3_local(
            Some(3),
            Some(4),
            multiplicity * (v406_ddt),
            2,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v270*v588)))) * ddt_scale)),
            3,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v270*v589)))) * ddt_scale)),
            4,
            multiplicity * ((((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v270*v590)))) * ddt_scale)),
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
        let v116=(if (v34>v13){v12}else{v13});
        let v118=(v20*self.scalar_static_f64[24]);
        let v120=(if (v116!=0.0){(v108/v118)}else{v13});
        let v122=((-v108)-v47);
        let v124=(v20*self.scalar_static_f64[25]);
        let v126=(if (v116!=0.0){(v122/v124)}else{v13});
        let v127=(-v47);
        let v129=(if (v116!=0.0){(v127/v124)}else{v13});
        let v130=80.0;
        let v132=(if (v120>v130){v12}else{v13});
        let v133=((v116!=0.0)&&(v132!=0.0));
        let v139=((v116!=0.0)&&(!(v132!=0.0)));
        let v140=(if v139{v12}else{(if v133{(v12+(v120-v130))}else{v13})});
        let v141=((if v133{v130}else{v120})).exp();
        let v144=37.0;
        let v145=(v126>=v144);
        let v146=(!v145);
        let v147=-37.0;
        let v148=(v126<=v147);
        let v150=(v146&&(!v148));
        let v151=(v126).exp();
        let v152=(v12+v151);
        let v154=(v146&&v148);
        let v158=(v129>=v144);
        let v159=(!v158);
        let v160=(v129<=v147);
        let v162=(v159&&(!v160));
        let v163=(v129).exp();
        let v164=(v12+v163);
        let v166=(v159&&v160);
        let v171=(if (v116!=0.0){((if v150{(v152).ln()}else{(if v154{v151}else{(if v145{v126}else{v13})})})-(if v162{(v164).ln()}else{(if v166{v163}else{(if v158{v129}else{v13})})}))}else{v13});
        let v172=((if (v116!=0.0){(v140*v141)}else{v140})-v12);
        let v174=(v42*v171);
        let v176=(v108).abs();
        let v177=f64::powf(v176,(self.scalar_static_f64[15]*(v12+(v26*self.scalar_static_f64[16]))));
        let v179=(v12+(self.scalar_static_f64[26]*v177));
        let v183=(!(v116!=0.0));
        let v270=(self.scalar_static_f64[46]*(v12+((f64::powf((v12+f64::powf((((ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1]))/self.scalar_static_f64[43])).abs(),self.scalar_static_f64[44])),self.scalar_static_f64[45])-v12)*self.scalar_static_f64[47])));
        let v287=(v108+((-v95)*self.scalar_static_f64[52]));
        let v289=(if (v287>v13){v12}else{v13});
        let v295=(if (v289!=0.0){self.scalar_static_f64[57]}else{v13});
        let v298=(v12-(self.scalar_static_f64[54]*(self.scalar_static_f64[54]*v295)));
        let v305=(v287*self.scalar_static_f64[59]);
        let v307=(self.scalar_static_f64[54]+(v305/v95));
        let v311=(!(v289!=0.0));
        let v313=(v12-(v108/v95));
        let v316=((self.scalar_static_f64[58]*(v313).ln())).exp();
        let v317=(v12-v316);
        let v322=((if v311{((v95*v317)/self.scalar_static_f64[58])}else{(if (v289!=0.0){((v95*v298)/self.scalar_static_f64[58])}else{v13})})+(if v311{v13}else{(if (v289!=0.0){(v295*(v287*v307))}else{v13})}));
        let v360=0.0;
        let v370=(v1*self.scalar_static_f64[75]);
        let v371=0.0;
        let v380=0.0;
        let v386=0.0;
        let v408=(if v10{v13}else{(if v8{v12}else{v13})});
        let v409=(v19*v408);
        let v410=(v408/self.scalar_static_f64[5]);
        let v431=(v408/v54);
        let v456=((v79*(-(v409+v409)))+(v73*((v74*(v431/v56))+(v77*(((v68*(((v62*((v59*v408)+(v11*(v58*v408))))-(v60*v408))/(v62*v62)))-(v65*(v66*(v408+v408))))/(v68*v68))))));
        let v458=((-v456)/self.scalar_static_f64[18]);
        let v463=(v83*v83);
        let v474=(v456+((v83*v431)+(v56*v458)));
        let v494=(if (v116!=0.0){((-(v108*(self.scalar_static_f64[24]*v409)))/(v118*v118))}else{v13});
        let v495=(if (v116!=0.0){(self.scalar_static_f64[23]/v118)}else{v13});
        let v496=(if (v116!=0.0){(self.scalar_static_f64[77]/v118)}else{v13});
        let v498=(self.scalar_static_f64[25]*v409);
        let v499=(v124*(-(self.scalar_static_f64[13]*(self.scalar_static_f64[14]*v410))));
        let v502=(v124*v124);
        let v506=(if (v116!=0.0){((v499-(v122*v498))/v502)}else{v13});
        let v507=(if (v116!=0.0){(self.scalar_static_f64[77]/v124)}else{v13});
        let v508=(if (v116!=0.0){(self.scalar_static_f64[23]/v124)}else{v13});
        let v512=(if (v116!=0.0){((v499-(v127*v498))/v502)}else{v13});
        let v519=(if v139{v13}else{(if v133{v494}else{v13})});
        let v520=(if v139{v13}else{(if v133{v495}else{v13})});
        let v521=(if v139{v13}else{(if v133{v496}else{v13})});
        let v537=(v151*v506);
        let v538=(v151*v507);
        let v539=(v151*v508);
        let v552=(v163*v512);
        let v668=(self.scalar_static_f64[52]*(-v474));
        let v678=(v95*v95);
        let v751=1.0;
        let v756=(self.scalar_static_f64[75]*v751);

        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * ((if (self.scalar_static_f64[49]!=0.0){(v270*v751)}else{v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if (self.scalar_static_f64[62]!=0.0){v756}else{v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * ((if self.scalar_static_bool[17]{v756}else{v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[76]*v751)}else{v13})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*((v322*((v102*((-(self.scalar_static_f64[17]*(self.scalar_static_f64[20]*(-(((v83*(-v458))-(v84*v458))/v463)))))/(v92*v92)))+(v93*(self.scalar_static_f64[20]*((v87*v408)-(((v83*(v474-v458))-(v96*v458))/v463))))))+(v103*((if v311{(((v317*v474)+(v95*(-(v316*(self.scalar_static_f64[58]*((-((-(v108*v474))/v678))/v313))))))/self.scalar_static_f64[58])}else{(if (v289!=0.0){((v298*v474)/self.scalar_static_f64[58])}else{v13})})+(if v311{v13}else{(if (v289!=0.0){(v295*((v307*v668)+(v287*(((v95*(self.scalar_static_f64[59]*v668))-(v305*v474))/v678))))}else{v13})}))))))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v311{((v95*(-(v316*(self.scalar_static_f64[58]*((-(self.scalar_static_f64[23]/v95))/v313)))))/self.scalar_static_f64[58])}else{v13})+(if v311{v13}else{(if (v289!=0.0){(v295*((self.scalar_static_f64[23]*v307)+(v287*(self.scalar_static_f64[80]/v95))))}else{v13})})))))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v103*((if v311{((v95*(-(v316*(self.scalar_static_f64[58]*((-(self.scalar_static_f64[77]/v95))/v313)))))/self.scalar_static_f64[58])}else{v13})+(if v311{v13}else{(if (v289!=0.0){(v295*((v307*self.scalar_static_f64[77])+(v287*(self.scalar_static_f64[81]/v95))))}else{v13})})))))),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v270*(if v183{v13}else{(if (v116!=0.0){(((v172*(self.scalar_static_f64[9]*(v33*((self.scalar_static_f64[6]*(v410/v21))+(((v20*(self.scalar_static_f64[7]*v410))-(v27*v409))/(v20*v20))))))+(v34*(if (v116!=0.0){((v141*v519)+(v140*(v141*(if v133{v13}else{v494}))))}else{v519})))-(((v179*((v171*(self.scalar_static_f64[11]*(self.scalar_static_f64[12]*v410)))+(v42*(if (v116!=0.0){((if v150{(v537/v152)}else{(if v154{v537}else{(if v145{v506}else{v13})})})-(if v162{(v552/v164)}else{(if v166{v552}else{(if v158{v512}else{v13})})}))}else{v13}))))-(v174*(self.scalar_static_f64[26]*((self.scalar_static_f64[15]*(self.scalar_static_f64[16]*v410))*(v177*(v176).ln())))))/(v179*v179)))}else{v13})}))))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v270*(if v183{v13}else{(if (v116!=0.0){((v34*(if (v116!=0.0){((v141*v520)+(v140*(v141*(if v133{v13}else{v495}))))}else{v520}))-((v42*(if (v116!=0.0){(if v150{(v538/v152)}else{(if v154{v538}else{(if v145{v507}else{v13})})})}else{v13}))/v179))}else{v13})}))))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[23]*(v270*(if v183{v13}else{(if (v116!=0.0){((v34*(if (v116!=0.0){((v141*v521)+(v140*(v141*(if v133{v13}else{v496}))))}else{v521}))-((v42*(if (v116!=0.0){(if v150{(v539/v152)}else{(if v154{v539}else{(if v145{v508}else{v13})})})}else{v13}))/v179))}else{v13})}))))),
        );
    }
}
