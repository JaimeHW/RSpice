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
    v1: f64, v12: f64, v13: f64, v18: f64, v19: f64, v20: f64, 
    v21: f64, v33: f64, v35: f64, v38: f64, v66: f64, v73: f64, 
    v74: f64, v78: f64, v79: f64, v80: f64, v196: f64, v199: f64, 
    v200: f64, v201: f64, v202: f64, v224: f64, v238: f64, v241: f64, 
    v270: f64, v273: f64, v278: f64, v280: f64, v306: f64, v313: f64, 
    v321: f64, v335: f64, v361: f64, v376: f64, v377: f64, v378: f64, 
    v385: f64, v391: f64, v417: f64, v421: f64, v428: f64, v486: f64, 
    v519: f64, v520: f64, v521: f64, v562: f64, v576: f64, v583: f64, 
    v815: f64, v821: f64, v830: f64, v864: f64, v866: f64, v868: f64, 
    v870: f64, v872: f64, v874: f64, v876: f64, v877: f64, v892: f64, 
    v894: f64, v909: f64, v1128: f64, v1132: f64, v1142: f64, v1143: f64, 
    v1144: f64, v1191: f64, v1192: f64, v1193: f64, v1221: f64, v1222: f64, 
    v1228: f64, v1251: f64, v1252: f64, v1253: f64, v1275: f64, v1276: f64, 
    v1277: f64, v1314: f64, v1315: f64, v1316: f64, v1317: f64, v1318: f64, 
    v1319: f64, v1324: f64, v1325: f64, v1326: f64, v1327: f64, v1348: f64, 
    v1349: f64, v1350: f64, v1351: f64, v1378: f64, v1379: f64, v1380: f64, 
    v1381: f64, v1414: f64, v1415: f64, v1416: f64, v1417: f64, v1564: f64, 
    v1565: f64, v1566: f64, v1567: f64, v1570: f64, v1573: f64, v1576: f64, 
    v1577: f64, v1578: f64, v1581: f64, v1584: f64, v2050: f64, v2051: f64, 
    v2052: f64, v2056: f64, v2057: f64, v2058: f64, v2064: f64, v2065: f64, 
    v2066: f64, v2067: f64, v2068: f64, v2074: f64, v2075: f64, v2076: f64, 
    v2077: f64, v2078: f64, v2083: f64, v2084: f64, v2085: f64, v2086: f64, 
    v2090: f64, v2091: f64, v2092: f64, v2097: f64, v2098: f64, v2099: f64, 
    v2100: f64, v2101: f64, v2102: f64, v2103: f64, v2104: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        let v1=ctx.node_voltage(nodes[3]);
        let v4=((ctx.temperature()+v1)+self.scalar_static_f64[0]);
        let v6=1300.0;
        let v7=173.14999999999998;
        let v8=(v4>v7);
        let v9=(if v8{v4}else{v7});
        let v10=(v6<v9);
        let v11=(if v10{v6}else{v9});
        let v12=1.0;
        let v13=0.0;
        let v18=ctx.node_voltage(nodes[5]);
        let v19=ctx.node_voltage(nodes[4]);
        let v20=(v18-v19);
        let v21=(self.scalar_static_f64[4]*v20);
        let v32=8.6170869e-5;
        let v33=(v11*v32);
        let v34=(v11/self.scalar_static_f64[8]);
        let v35=(v34).ln();
        let v38=((v35*self.scalar_static_f64[9])).exp();
        let v63=(v34-v12);
        let v64=(self.scalar_static_f64[25]*v63);
        let v66=((v35*self.scalar_static_f64[24])+(v64/v33));
        let v70=(v66).exp();
        let v71=(self.scalar_static_f64[27]*v70);
        let v73=((v35*self.scalar_static_f64[26])).exp();
        let v74=(self.scalar_static_f64[28]*v73);
        let v78=((v66/self.scalar_static_f64[30])).exp();
        let v79=(self.scalar_static_f64[29]*v78);
        let v80=(v79/v38);
        let v91=(self.scalar_static_f64[33]*(v12+(v63*self.scalar_static_f64[34])));
        let v96=(self.scalar_static_f64[35]*(v12+(v63*self.scalar_static_f64[36])));
        let v101=(self.scalar_static_f64[37]*(v12+(v63*self.scalar_static_f64[38])));
        let v106=(self.scalar_static_f64[39]*(v12+(v63*self.scalar_static_f64[40])));
        let v110=300.15;
        let v112=(v11/v110);
        let v114=0.000702;
        let v115=(v11*v114);
        let v116=(v11*v115);
        let v118=(v11+1108.0);
        let v121=(-(1.16-(v116/v118)));
        let v122=1.3806226e-23;
        let v124=(v122*(v11+v11));
        let v129=(-(v33+v33));
        let v130=1.5;
        let v133=1.6021918e-19;
        let v135=((v130*(v112).ln())+(((v121/v124)+1.3454442398941469e20)*v133));
        let v136=(v129*v135);
        let v139=((self.scalar_static_f64[45]-v136)/self.scalar_static_f64[44]);
        let v140=(self.scalar_static_f64[45]-v139);
        let v143=0.0004;
        let v148=(v12+(self.scalar_static_f64[46]*(self.scalar_static_f64[48]-(v140/v139))));
        let v149=(self.scalar_static_f64[41]/v148);
        let v151=(v136+(v112*v139));
        let v152=(v151-v139);
        let v155=(v143*(v11-v110));
        let v158=(v12+(self.scalar_static_f64[46]*(v155-(v152/v139))));
        let v159=(v149*v158);
        let v162=((self.scalar_static_f64[49]-v136)/self.scalar_static_f64[44]);
        let v163=(self.scalar_static_f64[49]-v162);
        let v168=(v12+(self.scalar_static_f64[50]*(self.scalar_static_f64[48]-(v163/v162))));
        let v169=(self.scalar_static_f64[42]/v168);
        let v171=(v136+(v112*v162));
        let v172=(v171-v162);
        let v176=(v12+(self.scalar_static_f64[50]*(v155-(v172/v162))));
        let v177=(v169*v176);
        let v180=((self.scalar_static_f64[51]-v136)/self.scalar_static_f64[44]);
        let v181=(self.scalar_static_f64[51]-v180);
        let v186=(v12+(self.scalar_static_f64[52]*(self.scalar_static_f64[48]-(v181/v180))));
        let v187=(self.scalar_static_f64[43]/v186);
        let v189=(v136+(v112*v180));
        let v190=(v189-v180);
        let v194=(v12+(self.scalar_static_f64[52]*(v155-(v190/v180))));
        let v195=(v187*v194);
        let v196=ctx.node_voltage(nodes[2]);
        let v198=(self.scalar_static_f64[4]*(v196-v19));
        let v199=ctx.node_voltage(nodes[6]);
        let v200=(v18-v199);
        let v201=(self.scalar_static_f64[4]*v200);
        let v202=ctx.node_voltage(nodes[1]);
        let v204=(self.scalar_static_f64[4]*(v202-v19));
        let v210=(if (v71>v13){v12}else{v13});
        let v212=(v33*self.scalar_static_f64[53]);
        let v214=(if (v210!=0.0){(v201/v212)}else{v13});
        let v215=(-v201);
        let v216=(v215-v96);
        let v218=(v33*self.scalar_static_f64[54]);
        let v220=(if (v210!=0.0){(v216/v218)}else{v13});
        let v221=(-v96);
        let v223=(if (v210!=0.0){(v221/v218)}else{v13});
        let v224=80.0;
        let v226=(if (v214>v224){v12}else{v13});
        let v227=((v210!=0.0)&&(v226!=0.0));
        let v231=(if v227{v224}else{v214});
        let v233=((v210!=0.0)&&(!(v226!=0.0)));
        let v234=(if v233{v12}else{(if v227{(v12+(v214-v224))}else{v13})});
        let v235=(v231).exp();
        let v237=(if (v210!=0.0){(v234*v235)}else{v234});
        let v238=37.0;
        let v239=(v220>=v238);
        let v240=(!v239);
        let v241=-37.0;
        let v242=(v220<=v241);
        let v244=(v240&&(!v242));
        let v245=(v220).exp();
        let v246=(v12+v245);
        let v248=(v240&&v242);
        let v252=(v223>=v238);
        let v253=(!v252);
        let v254=(v223<=v241);
        let v256=(v253&&(!v254));
        let v257=(v223).exp();
        let v258=(v12+v257);
        let v260=(v253&&v254);
        let v265=(if (v210!=0.0){((if v244{(v246).ln()}else{(if v248{v245}else{(if v239{v220}else{v13})})})-(if v256{(v258).ln()}else{(if v260{v257}else{(if v252{v223}else{v13})})}))}else{v13});
        let v266=(v237-v12);
        let v268=(v91*v265);
        let v270=(v201).abs();
        let v271=f64::powf(v270,v101);
        let v273=(v12+(self.scalar_static_f64[55]*v271));
        let v277=(!(v210!=0.0));
        let v278=(if v277{v13}else{(if (v210!=0.0){((v71*v266)-(v268/v273))}else{v13})});
        let v280=(if (v74>v13){v12}else{v13});
        let v282=(self.scalar_static_f64[56]-v201);
        let v283=0.001;
        let v284=(v282>v283);
        let v286=(if (v280!=0.0){(if v284{v282}else{v283})}else{v13});
        let v288=(v215*self.scalar_static_f64[56]);
        let v290=(v33*self.scalar_static_f64[57]);
        let v291=(v286*v290);
        let v293=(if (v280!=0.0){(v288/v291)}else{v231});
        let v295=(if (v293>v224){v12}else{v13});
        let v296=((v280!=0.0)&&(v295!=0.0));
        let v300=(if v296{v224}else{v293});
        let v302=((v280!=0.0)&&(!(v295!=0.0)));
        let v303=(if v302{v12}else{(if v296{(v12+(v293-v224))}else{v237})});
        let v304=(v300).exp();
        let v306=(if (v280!=0.0){(v303*v304)}else{v303});
        let v313=(if (v80>v13){v12}else{v13});
        let v314=(v33*self.scalar_static_f64[30]);
        let v316=(if (v313!=0.0){(v201/v314)}else{v300});
        let v318=(v33*self.scalar_static_f64[58]);
        let v320=(if (v313!=0.0){(v216/v318)}else{v220});
        let v321=(v221/v318);
        let v322=(if (v313!=0.0){v321}else{v223});
        let v324=(if (v316>v224){v12}else{v13});
        let v325=((v313!=0.0)&&(v324!=0.0));
        let v329=(if v325{v224}else{v316});
        let v331=((v313!=0.0)&&(!(v324!=0.0)));
        let v332=(if v331{v12}else{(if v325{(v12+(v316-v224))}else{v306})});
        let v333=(v329).exp();
        let v335=(if (v313!=0.0){(v332*v333)}else{v332});
        let v336=(v320>=v238);
        let v337=(!v336);
        let v338=(v320<=v241);
        let v340=(v337&&(!v338));
        let v341=(v320).exp();
        let v342=(v12+v341);
        let v344=(v337&&v338);
        let v348=(v322>=v238);
        let v349=(!v348);
        let v350=(v322<=v241);
        let v352=(v349&&(!v350));
        let v353=(v322).exp();
        let v354=(v12+v353);
        let v356=(v349&&v350);
        let v361=(if (v313!=0.0){((if v340{(v342).ln()}else{(if v344{v341}else{(if v336{v320}else{v13})})})-(if v352{(v354).ln()}else{(if v356{v353}else{(if v348{v322}else{v13})})}))}else{v265});
        let v371=(v33*self.scalar_static_f64[59]);
        let v373=(if (v210!=0.0){(v21/v371)}else{v329});
        let v375=((-v21)-v96);
        let v376=(v375/v318);
        let v377=(if (v210!=0.0){v376}else{v320});
        let v378=(if (v210!=0.0){v321}else{v322});
        let v380=(if (v373>v224){v12}else{v13});
        let v381=((v210!=0.0)&&(v380!=0.0));
        let v385=(if v381{v224}else{v373});
        let v387=((v210!=0.0)&&(!(v380!=0.0)));
        let v388=(if v387{v12}else{(if v381{(v12+(v373-v224))}else{v335})});
        let v389=(v385).exp();
        let v391=(if (v210!=0.0){(v388*v389)}else{v388});
        let v392=(v377>=v238);
        let v393=(!v392);
        let v394=(v377<=v241);
        let v396=(v393&&(!v394));
        let v397=(v377).exp();
        let v398=(v12+v397);
        let v400=(v393&&v394);
        let v404=(v378>=v238);
        let v405=(!v404);
        let v406=(v378<=v241);
        let v408=(v405&&(!v406));
        let v409=(v378).exp();
        let v410=(v12+v409);
        let v412=(v405&&v406);
        let v417=(if (v210!=0.0){((if v396{(v398).ln()}else{(if v400{v397}else{(if v392{v377}else{v13})})})-(if v408{(v410).ln()}else{(if v412{v409}else{(if v404{v378}else{v13})})}))}else{v361});
        let v418=(v391-v12);
        let v420=(v106*v417);
        let v421=(v21).abs();
        let v422=f64::powf(v421,v101);
        let v424=(v12+(self.scalar_static_f64[55]*v422));
        let v428=(if v277{v13}else{(if (v210!=0.0){((v71*v418)-(v420/v424))}else{v13})});
        let v486=ctx.node_voltage(nodes[9]);
        let v516=(v12+f64::powf(((v12+(((v278*(self.scalar_static_f64[20]*(v12+(v21*self.scalar_static_f64[60]))))+(self.scalar_static_f64[23]*v428))*4.0))).abs(),self.scalar_static_f64[61]));
        let v519=((((v12-(self.scalar_static_f64[17]*v201))-(v21*self.scalar_static_f64[14]))*2.0)/v516);
        let v520=(v428*v519);
        let v521=(v278*v519);
        let v562=(v202-v196);
        let v576=(self.scalar_static_f64[79]*(v12+((f64::powf((v12+f64::powf(((v562/self.scalar_static_f64[76])).abs(),self.scalar_static_f64[77])),self.scalar_static_f64[78])-v12)*self.scalar_static_f64[80])));
        let v583=ctx.node_voltage(nodes[8]);
        let v605=(if (v198<=v13){v12}else{v13});
        let v606=(v189*v195);
        let v609=(v12-(v198/v189));
        let v612=((self.scalar_static_f64[91]*(v609).ln())).exp();
        let v613=(v12-v612);
        let v617=(!(v605!=0.0));
        let v618=(v195*v198);
        let v621=(v198*self.scalar_static_f64[92]);
        let v623=(v12+(v621/v189));
        let v629=(v201+((-v151)*self.scalar_static_f64[93]));
        let v631=(if (v629>v13){v12}else{v13});
        let v637=(if (v631!=0.0){self.scalar_static_f64[98]}else{v13});
        let v640=(v12-(self.scalar_static_f64[95]*(self.scalar_static_f64[95]*v637)));
        let v646=(v629*self.scalar_static_f64[100]);
        let v648=(self.scalar_static_f64[95]+(v646/v151));
        let v652=(!(v631!=0.0));
        let v654=(v12-(v201/v151));
        let v657=((self.scalar_static_f64[99]*(v654).ln())).exp();
        let v658=(v12-v657);
        let v661=(if v652{((v151*v658)/self.scalar_static_f64[99])}else{(if (v631!=0.0){((v151*v640)/self.scalar_static_f64[99])}else{v13})});
        let v662=(if v652{v13}else{(if (v631!=0.0){(v637*(v629*v648))}else{v13})});
        let v663=(v661+v662);
        let v666=(self.scalar_static_f64[93]*(-v171));
        let v667=(v204+v666);
        let v669=(if (v667>v13){v12}else{v13});
        let v673=(if (v669!=0.0){self.scalar_static_f64[103]}else{v637});
        let v676=(v12-(self.scalar_static_f64[95]*(self.scalar_static_f64[95]*v673)));
        let v682=(v667*self.scalar_static_f64[105]);
        let v684=(self.scalar_static_f64[95]+(v682/v171));
        let v688=(!(v669!=0.0));
        let v690=(v12-(v204/v171));
        let v693=((self.scalar_static_f64[104]*(v690).ln())).exp();
        let v694=(v12-v693);
        let v697=(if v688{((v171*v694)/self.scalar_static_f64[104])}else{(if (v669!=0.0){((v171*v676)/self.scalar_static_f64[104])}else{v661})});
        let v698=(if v688{v13}else{(if (v669!=0.0){(v673*(v667*v684))}else{v662})});
        let v699=(v697+v698);
        let v704=(v21+v666);
        let v706=(if (v704>v13){v12}else{v13});
        let v707=(if (v706!=0.0){self.scalar_static_f64[103]}else{v673});
        let v710=(v12-(self.scalar_static_f64[95]*(self.scalar_static_f64[95]*v707)));
        let v714=(self.scalar_static_f64[105]*v704);
        let v716=(self.scalar_static_f64[95]+(v714/v171));
        let v720=(!(v706!=0.0));
        let v722=(v12-(v21/v171));
        let v725=((self.scalar_static_f64[104]*(v722).ln())).exp();
        let v726=(v12-v725);
        let v731=((if v720{((v171*v726)/self.scalar_static_f64[104])}else{(if (v706!=0.0){((v171*v710)/self.scalar_static_f64[104])}else{v697})})+(if v720{v13}else{(if (v706!=0.0){(v707*(v704*v716))}else{v698})}));
        let v748=(if self.scalar_static_bool[9]{v13}else{(if (self.scalar_static_f64[109]!=0.0){(v521*self.scalar_static_f64[113])}else{v13})});
        let v815=(v1*self.scalar_static_f64[134]);
        let v821=ctx.node_voltage(nodes[7]);
        let v830=(v821*self.scalar_static_f64[135]);
        let v864=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v159*v663)));
        let v866=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v278*v576)));
        let v868=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((v177*v699)*self.scalar_static_f64[107])));
        let v870=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[106]*(v177*v731))));
        let v872=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v520*self.scalar_static_f64[81])));
        let v874=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v617{(v618*v623)}else{(if (v605!=0.0){((v606*v613)/self.scalar_static_f64[91])}else{v13})})));
        let v876=(self.scalar_static_f64[3]*(-v748));
        let v877=(self.scalar_static_f64[3]*v748);
        let v879=(if v10{v13}else{(if v8{v12}else{v13})});
        let v892=(v32*v879);
        let v893=(v879/self.scalar_static_f64[8]);
        let v894=(v893/v34);
        let v909=((self.scalar_static_f64[24]*v894)+(((v33*(self.scalar_static_f64[25]*v893))-(v64*v892))/(v33*v33)));
        let v912=(self.scalar_static_f64[27]*(v70*v909));
        let v935=(self.scalar_static_f64[37]*(self.scalar_static_f64[38]*v893));
        let v938=(v879/v110);
        let v963=((v135*(-(v892+v892)))+(v129*((v130*(v938/v112))+(v133*(((v124*(((v118*((v115*v879)+(v11*(v114*v879))))-(v116*v879))/(v118*v118)))-(v121*(v122*(v879+v879))))/(v124*v124))))));
        let v965=((-v963)/self.scalar_static_f64[44]);
        let v966=(-v965);
        let v970=(v139*v139);
        let v979=(v112*v965);
        let v981=(v963+((v139*v938)+v979));
        let v987=(v143*v879);
        let v996=(v162*v162);
        let v1006=(v963+(v979+(v162*v938)));
        let v1016=((v176*((-(self.scalar_static_f64[42]*(self.scalar_static_f64[50]*(-(((v162*v966)-(v163*v965))/v996)))))/(v168*v168)))+(v169*(self.scalar_static_f64[50]*(v987-(((v162*(v1006-v965))-(v172*v965))/v996)))));
        let v1020=(v180*v180);
        let v1030=(v963+(v979+(v180*v938)));
        let v1040=((v194*((-(self.scalar_static_f64[43]*(self.scalar_static_f64[52]*(-(((v180*v966)-(v181*v965))/v1020)))))/(v186*v186)))+(v187*(self.scalar_static_f64[52]*(v987-(((v180*(v1030-v965))-(v190*v965))/v1020)))));
        let v1048=(if (v210!=0.0){((-(v201*(self.scalar_static_f64[53]*v892)))/(v212*v212))}else{v13});
        let v1049=(if (v210!=0.0){(self.scalar_static_f64[4]/v212)}else{v13});
        let v1050=(if (v210!=0.0){(self.scalar_static_f64[136]/v212)}else{v13});
        let v1051=(-(self.scalar_static_f64[35]*(self.scalar_static_f64[36]*v893)));
        let v1052=(self.scalar_static_f64[54]*v892);
        let v1053=(v218*v1051);
        let v1056=(v218*v218);
        let v1060=(if (v210!=0.0){((v1053-(v216*v1052))/v1056)}else{v13});
        let v1061=(if (v210!=0.0){(self.scalar_static_f64[136]/v218)}else{v13});
        let v1062=(if (v210!=0.0){(self.scalar_static_f64[4]/v218)}else{v13});
        let v1066=(if (v210!=0.0){((v1053-(v221*v1052))/v1056)}else{v13});
        let v1070=(if v227{v13}else{v1048});
        let v1071=(if v227{v13}else{v1049});
        let v1072=(if v227{v13}else{v1050});
        let v1073=(if v233{v13}else{(if v227{v1048}else{v13})});
        let v1074=(if v233{v13}else{(if v227{v1049}else{v13})});
        let v1075=(if v233{v13}else{(if v227{v1050}else{v13})});
        let v1088=(if (v210!=0.0){((v235*v1073)+(v234*(v235*v1070)))}else{v1073});
        let v1089=(if (v210!=0.0){((v235*v1074)+(v234*(v235*v1071)))}else{v1074});
        let v1090=(if (v210!=0.0){((v235*v1075)+(v234*(v235*v1072)))}else{v1075});
        let v1091=(v245*v1060);
        let v1092=(v245*v1061);
        let v1093=(v245*v1062);
        let v1106=(v257*v1066);
        let v1112=(if (v210!=0.0){((if v244{(v1091/v246)}else{(if v248{v1091}else{(if v239{v1060}else{v13})})})-(if v256{(v1106/v258)}else{(if v260{v1106}else{(if v252{v1066}else{v13})})}))}else{v13});
        let v1113=(if (v210!=0.0){(if v244{(v1092/v246)}else{(if v248{v1092}else{(if v239{v1061}else{v13})})})}else{v13});
        let v1114=(if (v210!=0.0){(if v244{(v1093/v246)}else{(if v248{v1093}else{(if v239{v1062}else{v13})})})}else{v13});
        let v1128=(self.scalar_static_f64[55]*(v935*(v271*(v270).ln())));
        let v1132=(v273*v273);
        let v1142=(if v277{v13}else{(if (v210!=0.0){(((v266*v912)+(v71*v1088))-(((v273*((v265*(self.scalar_static_f64[33]*(self.scalar_static_f64[34]*v893)))+(v91*v1112)))-(v268*v1128))/v1132))}else{v13})});
        let v1143=(if v277{v13}else{(if (v210!=0.0){((v71*v1089)-((v91*v1113)/v273))}else{v13})});
        let v1144=(if v277{v13}else{(if (v210!=0.0){((v71*v1090)-((v91*v1114)/v273))}else{v13})});
        let v1157=(v291*v291);
        let v1167=(if (v280!=0.0){((-(v288*(v286*(self.scalar_static_f64[57]*v892))))/v1157)}else{v1070});
        let v1168=(if (v280!=0.0){(((v291*self.scalar_static_f64[138])-(v288*(v290*(if (v280!=0.0){(if v284{self.scalar_static_f64[136]}else{v13})}else{v13}))))/v1157)}else{v1071});
        let v1169=(if (v280!=0.0){(((v291*self.scalar_static_f64[139])-(v288*(v290*(if (v280!=0.0){(if v284{self.scalar_static_f64[4]}else{v13})}else{v13}))))/v1157)}else{v1072});
        let v1173=(if v296{v13}else{v1167});
        let v1174=(if v296{v13}else{v1168});
        let v1175=(if v296{v13}else{v1169});
        let v1176=(if v302{v13}else{(if v296{v1167}else{v1088})});
        let v1177=(if v302{v13}else{(if v296{v1168}else{v1089})});
        let v1178=(if v302{v13}else{(if v296{v1169}else{v1090})});
        let v1191=(if (v280!=0.0){((v304*v1176)+(v303*(v304*v1173)))}else{v1176});
        let v1192=(if (v280!=0.0){((v304*v1177)+(v303*(v304*v1174)))}else{v1177});
        let v1193=(if (v280!=0.0){((v304*v1178)+(v303*(v304*v1175)))}else{v1178});
        let v1212=(if (v313!=0.0){((-(v201*(self.scalar_static_f64[30]*v892)))/(v314*v314))}else{v1173});
        let v1213=(if (v313!=0.0){(self.scalar_static_f64[4]/v314)}else{v1174});
        let v1214=(if (v313!=0.0){(self.scalar_static_f64[136]/v314)}else{v1175});
        let v1215=(self.scalar_static_f64[58]*v892);
        let v1216=(v318*v1051);
        let v1219=(v318*v318);
        let v1221=(self.scalar_static_f64[136]/v318);
        let v1222=(self.scalar_static_f64[4]/v318);
        let v1223=(if (v313!=0.0){((v1216-(v216*v1215))/v1219)}else{v1060});
        let v1224=(if (v313!=0.0){v1221}else{v1061});
        let v1225=(if (v313!=0.0){v1222}else{v1062});
        let v1228=((v1216-(v221*v1215))/v1219);
        let v1229=(if (v313!=0.0){v1228}else{v1066});
        let v1233=(if v325{v13}else{v1212});
        let v1234=(if v325{v13}else{v1213});
        let v1235=(if v325{v13}else{v1214});
        let v1236=(if v331{v13}else{(if v325{v1212}else{v1191})});
        let v1237=(if v331{v13}else{(if v325{v1213}else{v1192})});
        let v1238=(if v331{v13}else{(if v325{v1214}else{v1193})});
        let v1251=(if (v313!=0.0){((v333*v1236)+(v332*(v333*v1233)))}else{v1236});
        let v1252=(if (v313!=0.0){((v333*v1237)+(v332*(v333*v1234)))}else{v1237});
        let v1253=(if (v313!=0.0){((v333*v1238)+(v332*(v333*v1235)))}else{v1238});
        let v1254=(v341*v1223);
        let v1255=(v341*v1224);
        let v1256=(v341*v1225);
        let v1269=(v353*v1229);
        let v1275=(if (v313!=0.0){((if v340{(v1254/v342)}else{(if v344{v1254}else{(if v336{v1223}else{v13})})})-(if v352{(v1269/v354)}else{(if v356{v1269}else{(if v348{v1229}else{v13})})}))}else{v1112});
        let v1276=(if (v313!=0.0){(if v340{(v1255/v342)}else{(if v344{v1255}else{(if v336{v1224}else{v13})})})}else{v1113});
        let v1277=(if (v313!=0.0){(if v340{(v1256/v342)}else{(if v344{v1256}else{(if v336{v1225}else{v13})})})}else{v1114});
        let v1308=(if (v210!=0.0){((-(v21*(self.scalar_static_f64[59]*v892)))/(v371*v371))}else{v1233});
        let v1309=(if (v210!=0.0){(self.scalar_static_f64[136]/v371)}else{v13});
        let v1310=(if (v210!=0.0){(self.scalar_static_f64[4]/v371)}else{v1234});
        let v1311=(if (v210!=0.0){v13}else{v1235});
        let v1314=((v1216-(v375*v1215))/v1219);
        let v1315=(if (v210!=0.0){v1314}else{v1223});
        let v1316=(if (v210!=0.0){v1222}else{v13});
        let v1317=(if (v210!=0.0){v1221}else{v1224});
        let v1318=(if (v210!=0.0){v13}else{v1225});
        let v1319=(if (v210!=0.0){v1228}else{v1229});
        let v1324=(if v381{v13}else{v1308});
        let v1325=(if v381{v13}else{v1309});
        let v1326=(if v381{v13}else{v1310});
        let v1327=(if v381{v13}else{v1311});
        let v1328=(if v387{v13}else{(if v381{v1308}else{v1251})});
        let v1329=(if v387{v13}else{(if v381{v1309}else{v13})});
        let v1330=(if v387{v13}else{(if v381{v1310}else{v1252})});
        let v1331=(if v387{v13}else{(if v381{v1311}else{v1253})});
        let v1348=(if (v210!=0.0){((v389*v1328)+(v388*(v389*v1324)))}else{v1328});
        let v1349=(if (v210!=0.0){((v389*v1329)+(v388*(v389*v1325)))}else{v1329});
        let v1350=(if (v210!=0.0){((v389*v1330)+(v388*(v389*v1326)))}else{v1330});
        let v1351=(if (v210!=0.0){((v389*v1331)+(v388*(v389*v1327)))}else{v1331});
        let v1352=(v397*v1315);
        let v1353=(v397*v1316);
        let v1354=(v397*v1317);
        let v1355=(v397*v1318);
        let v1372=(v409*v1319);
        let v1378=(if (v210!=0.0){((if v396{(v1352/v398)}else{(if v400{v1352}else{(if v392{v1315}else{v13})})})-(if v408{(v1372/v410)}else{(if v412{v1372}else{(if v404{v1319}else{v13})})}))}else{v1275});
        let v1379=(if (v210!=0.0){(if v396{(v1353/v398)}else{(if v400{v1353}else{(if v392{v1316}else{v13})})})}else{v13});
        let v1380=(if (v210!=0.0){(if v396{(v1354/v398)}else{(if v400{v1354}else{(if v392{v1317}else{v13})})})}else{v1276});
        let v1381=(if (v210!=0.0){(if v396{(v1355/v398)}else{(if v400{v1355}else{(if v392{v1318}else{v13})})})}else{v1277});
        let v1414=(if v277{v13}else{(if (v210!=0.0){(((v418*v912)+(v71*v1348))-(((v424*((v417*(self.scalar_static_f64[39]*(self.scalar_static_f64[40]*v893)))+(v106*v1378)))-(v420*(self.scalar_static_f64[55]*(v935*(v422*(v421).ln())))))/(v424*v424)))}else{v13})});
        let v1415=(if v277{v13}else{(if (v210!=0.0){((v71*v1349)-((v106*v1379)/v424))}else{v13})});
        let v1416=(if v277{v13}else{(if (v210!=0.0){((v71*v1350)-((v106*v1380)/v424))}else{v13})});
        let v1417=(if v277{v13}else{(if (v210!=0.0){((v71*v1351)-((v106*v1381)/v424))}else{v13})});
        let v1564=(self.scalar_static_f64[148]/v516);
        let v1565=(self.scalar_static_f64[149]/v516);
        let v1566=(self.scalar_static_f64[150]/v516);
        let v1567=(v519*v1414);
        let v1570=((v519*v1415)+(v428*v1564));
        let v1573=((v519*v1416)+(v428*v1565));
        let v1576=((v519*v1417)+(v428*v1566));
        let v1577=(v519*v1142);
        let v1578=(v278*v1564);
        let v1581=((v519*v1143)+(v278*v1565));
        let v1584=((v519*v1144)+(v278*v1566));
        let v1634=(v189*v189);
        let v1686=(self.scalar_static_f64[93]*(-v981));
        let v1696=(v151*v151);
        let v1743=(if v652{(((v658*v981)+(v151*(-(v657*(self.scalar_static_f64[99]*((-((-(v201*v981))/v1696))/v654))))))/self.scalar_static_f64[99])}else{(if (v631!=0.0){((v640*v981)/self.scalar_static_f64[99])}else{v13})});
        let v1744=(if v652{((v151*(-(v657*(self.scalar_static_f64[99]*((-(self.scalar_static_f64[4]/v151))/v654)))))/self.scalar_static_f64[99])}else{v13});
        let v1745=(if v652{((v151*(-(v657*(self.scalar_static_f64[99]*((-(self.scalar_static_f64[136]/v151))/v654)))))/self.scalar_static_f64[99])}else{v13});
        let v1746=(if v652{v13}else{(if (v631!=0.0){(v637*((v648*v1686)+(v629*(((v151*(self.scalar_static_f64[100]*v1686))-(v646*v981))/v1696))))}else{v13})});
        let v1747=(if v652{v13}else{(if (v631!=0.0){(v637*((self.scalar_static_f64[4]*v648)+(v629*(self.scalar_static_f64[153]/v151))))}else{v13})});
        let v1748=(if v652{v13}else{(if (v631!=0.0){(v637*((v648*self.scalar_static_f64[136])+(v629*(self.scalar_static_f64[154]/v151))))}else{v13})});
        let v1758=(self.scalar_static_f64[93]*(-v1006));
        let v1767=(self.scalar_static_f64[155]/v171);
        let v1768=(v171*(self.scalar_static_f64[105]*v1758));
        let v1771=(v171*v171);
        let v1773=(self.scalar_static_f64[156]/v171);
        let v1796=(-(self.scalar_static_f64[4]/v171));
        let v1798=(-(self.scalar_static_f64[136]/v171));
        let v1819=(if v688{((v171*(-(v693*(self.scalar_static_f64[104]*(v1796/v690)))))/self.scalar_static_f64[104])}else{v13});
        let v1820=(if v688{(((v694*v1006)+(v171*(-(v693*(self.scalar_static_f64[104]*((-((-(v204*v1006))/v1771))/v690))))))/self.scalar_static_f64[104])}else{(if (v669!=0.0){((v676*v1006)/self.scalar_static_f64[104])}else{v1743})});
        let v1821=(if v688{((v171*(-(v693*(self.scalar_static_f64[104]*(v1798/v690)))))/self.scalar_static_f64[104])}else{v13});
        let v1822=(if v688{v13}else{(if (v669!=0.0){v13}else{v1744})});
        let v1823=(if v688{v13}else{(if (v669!=0.0){v13}else{v1745})});
        let v1824=(if v688{v13}else{(if (v669!=0.0){(v673*((self.scalar_static_f64[4]*v684)+(v667*v1767)))}else{v13})});
        let v1825=(if v688{v13}else{(if (v669!=0.0){(v673*((v684*v1758)+(v667*((v1768-(v682*v1006))/v1771))))}else{v1746})});
        let v1826=(if v688{v13}else{(if (v669!=0.0){(v673*((v684*self.scalar_static_f64[136])+(v667*v1773)))}else{v13})});
        let v1827=(if v688{v13}else{(if (v669!=0.0){v13}else{v1747})});
        let v1828=(if v688{v13}else{(if (v669!=0.0){v13}else{v1748})});
        let v1932=(if self.scalar_static_bool[9]{v13}else{(if (self.scalar_static_f64[109]!=0.0){(self.scalar_static_f64[113]*v1577)}else{v13})});
        let v1933=(if self.scalar_static_bool[9]{v13}else{(if (self.scalar_static_f64[109]!=0.0){(self.scalar_static_f64[113]*v1578)}else{v13})});
        let v1934=(if self.scalar_static_bool[9]{v13}else{(if (self.scalar_static_f64[109]!=0.0){(self.scalar_static_f64[113]*v1581)}else{v13})});
        let v1935=(if self.scalar_static_bool[9]{v13}else{(if (self.scalar_static_f64[109]!=0.0){(self.scalar_static_f64[113]*v1584)}else{v13})});
        let v2050=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((v663*((v158*((-(self.scalar_static_f64[41]*(self.scalar_static_f64[46]*(-(((v139*v966)-(v140*v965))/v970)))))/(v148*v148)))+(v149*(self.scalar_static_f64[46]*(v987-(((v139*(v981-v965))-(v152*v965))/v970))))))+(v159*(v1743+v1746)))));
        let v2051=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v159*(v1744+v1747))));
        let v2052=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v159*(v1745+v1748))));
        let v2056=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v576*v1142)));
        let v2057=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v576*v1143)));
        let v2058=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v576*v1144)));
        let v2064=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[107]*(v177*(v1819+v1824)))));
        let v2065=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[107]*((v699*v1016)+(v177*(v1820+v1825))))));
        let v2066=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[107]*(v177*(v1821+v1826)))));
        let v2067=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[107]*(v177*(v1822+v1827)))));
        let v2068=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[107]*(v177*(v1823+v1828)))));
        let v2074=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[106]*(v177*((if v720{v13}else{(if (v706!=0.0){v13}else{v1819})})+(if v720{v13}else{(if (v706!=0.0){v13}else{v1824})}))))));
        let v2075=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[106]*((v731*v1016)+(v177*((if v720{(((v726*v1006)+(v171*(-(v725*(self.scalar_static_f64[104]*((-((-(v21*v1006))/v1771))/v722))))))/self.scalar_static_f64[104])}else{(if (v706!=0.0){((v710*v1006)/self.scalar_static_f64[104])}else{v1820})})+(if v720{v13}else{(if (v706!=0.0){(v707*((v716*v1758)+(v704*((v1768-(v714*v1006))/v1771))))}else{v1825})})))))));
        let v2076=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[106]*(v177*((if v720{((v171*(-(v725*(self.scalar_static_f64[104]*(v1798/v722)))))/self.scalar_static_f64[104])}else{(if (v706!=0.0){v13}else{v1821})})+(if v720{v13}else{(if (v706!=0.0){(v707*((v716*self.scalar_static_f64[136])+(v704*v1773)))}else{v1826})}))))));
        let v2077=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[106]*(v177*((if v720{((v171*(-(v725*(self.scalar_static_f64[104]*(v1796/v722)))))/self.scalar_static_f64[104])}else{(if (v706!=0.0){v13}else{v1822})})+(if v720{v13}else{(if (v706!=0.0){(v707*((self.scalar_static_f64[4]*v716)+(v704*v1767)))}else{v1827})}))))));
        let v2078=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[106]*(v177*((if v720{v13}else{(if (v706!=0.0){v13}else{v1823})})+(if v720{v13}else{(if (v706!=0.0){v13}else{v1828})}))))));
        let v2083=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1567)));
        let v2084=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1570)));
        let v2085=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1573)));
        let v2086=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1576)));
        let v2090=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v617{((v623*(self.scalar_static_f64[4]*v195))+(v618*(self.scalar_static_f64[151]/v189)))}else{(if (v605!=0.0){((v606*(-(v612*(self.scalar_static_f64[91]*((-(self.scalar_static_f64[4]/v189))/v609)))))/self.scalar_static_f64[91])}else{v13})})));
        let v2091=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v617{((v623*(v198*v1040))+(v618*((-(v621*v1030))/v1634)))}else{(if (v605!=0.0){(((v613*((v195*v1030)+(v189*v1040)))+(v606*(-(v612*(self.scalar_static_f64[91]*((-((-(v198*v1030))/v1634))/v609))))))/self.scalar_static_f64[91])}else{v13})})));
        let v2092=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v617{((v623*(v195*self.scalar_static_f64[136]))+(v618*(self.scalar_static_f64[152]/v189)))}else{(if (v605!=0.0){((v606*(-(v612*(self.scalar_static_f64[91]*((-(self.scalar_static_f64[136]/v189))/v609)))))/self.scalar_static_f64[91])}else{v13})})));
        let v2097=(self.scalar_static_f64[3]*(-v1932));
        let v2098=(self.scalar_static_f64[3]*(-v1933));
        let v2099=(self.scalar_static_f64[3]*(-v1934));
        let v2100=(self.scalar_static_f64[3]*(-v1935));
        let v2101=(self.scalar_static_f64[3]*v1932);
        let v2102=(self.scalar_static_f64[3]*v1933);
        let v2103=(self.scalar_static_f64[3]*v1934);
        let v2104=(self.scalar_static_f64[3]*v1935);

        CommonStampValues {
            v1, v12, v13, v18, v19, v20, v21, v33, 
            v35, v38, v66, v73, v74, v78, v79, v80, 
            v196, v199, v200, v201, v202, v224, v238, v241, 
            v270, v273, v278, v280, v306, v313, v321, v335, 
            v361, v376, v377, v378, v385, v391, v417, v421, 
            v428, v486, v519, v520, v521, v562, v576, v583, 
            v815, v821, v830, v864, v866, v868, v870, v872, 
            v874, v876, v877, v892, v894, v909, v1128, v1132, 
            v1142, v1143, v1144, v1191, v1192, v1193, v1221, v1222, 
            v1228, v1251, v1252, v1253, v1275, v1276, v1277, v1314, 
            v1315, v1316, v1317, v1318, v1319, v1324, v1325, v1326, 
            v1327, v1348, v1349, v1350, v1351, v1378, v1379, v1380, 
            v1381, v1414, v1415, v1416, v1417, v1564, v1565, v1566, 
            v1567, v1570, v1573, v1576, v1577, v1578, v1581, v1584, 
            v2050, v2051, v2052, v2056, v2057, v2058, v2064, v2065, 
            v2066, v2067, v2068, v2074, v2075, v2076, v2077, v2078, 
            v2083, v2084, v2085, v2086, v2090, v2091, v2092, v2097, 
            v2098, v2099, v2100, v2101, v2102, v2103, v2104, 
        }
    }

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
        let common=self.eval_common_stamp_values(ctx);
        let v23=(common.v21<common.v13);
        let v25=(-(if v23{common.v21}else{common.v13}));
        let v29=(common.v12+(self.scalar_static_f64[5]*f64::powf(v25,self.scalar_static_f64[6])));
        let v40=(common.v38*self.scalar_static_f64[10]);
        let v41=(v29*v40);
        let v43=(common.v38*self.scalar_static_f64[11]);
        let v84=((common.v66/self.scalar_static_f64[32])).exp();
        let v85=(self.scalar_static_f64[31]*v84);
        let v86=(v85/common.v38);
        let v205=(common.v202-common.v18);
        let v207=(common.v196-common.v199);
        let v287=-1.0;
        let v307=(common.v306-common.v12);
        let v310=(!(common.v280!=0.0));
        let v362=(common.v335-common.v12);
        let v364=(common.v13*common.v361);
        let v368=(!(common.v313!=0.0));
        let v430=(if (v86>common.v13){common.v12}else{common.v13});
        let v431=(common.v33*self.scalar_static_f64[32]);
        let v433=(if (v430!=0.0){(common.v21/v431)}else{common.v385});
        let v434=(if (v430!=0.0){common.v376}else{common.v377});
        let v435=(if (v430!=0.0){common.v321}else{common.v378});
        let v437=(if (v433>common.v224){common.v12}else{common.v13});
        let v438=((v430!=0.0)&&(v437!=0.0));
        let v444=((v430!=0.0)&&(!(v437!=0.0)));
        let v445=(if v444{common.v12}else{(if v438{(common.v12+(v433-common.v224))}else{common.v391})});
        let v446=((if v438{common.v224}else{v433})).exp();
        let v449=(v434>=common.v238);
        let v450=(!v449);
        let v451=(v434<=common.v241);
        let v453=(v450&&(!v451));
        let v454=(v434).exp();
        let v455=(common.v12+v454);
        let v457=(v450&&v451);
        let v461=(v435>=common.v238);
        let v462=(!v461);
        let v463=(v435<=common.v241);
        let v465=(v462&&(!v463));
        let v466=(v435).exp();
        let v467=(common.v12+v466);
        let v469=(v462&&v463);
        let v475=((if (v430!=0.0){(v445*v446)}else{v445})-common.v12);
        let v480=(common.v12+(self.scalar_static_f64[55]*f64::powf(common.v421,self.scalar_static_f64[37])));
        let v484=(!(v430!=0.0));
        let v489=1e-9;
        let v493=(((if (common.v486<common.v201){common.v486}else{common.v201})/(if (common.v270>v489){common.v270}else{v489}))).abs();
        let v494=(common.v278-(if v310{common.v13}else{(if (common.v280!=0.0){(common.v74*v307)}else{common.v13})}));
        let v496=((if v368{common.v13}else{(if (common.v313!=0.0){((common.v80*v362)-(v364/common.v273))}else{common.v13})})+(v494/v41));
        let v498=((if v484{common.v13}else{(if (v430!=0.0){((v86*v475)-((common.v13*(if (v430!=0.0){((if v453{(v455).ln()}else{(if v457{v454}else{(if v449{v434}else{common.v13})})})-(if v465{(v467).ln()}else{(if v469{v466}else{(if v461{v435}else{common.v13})})}))}else{common.v417}))/v480))}else{common.v13})})+(common.v428/v43));
        let v526=(common.v278*self.scalar_static_f64[63]);
        let v544=((common.v35*self.scalar_static_f64[69])).exp();
        let v547=f64::powf((common.v12+f64::powf((((self.scalar_static_f64[4]*v205)/self.scalar_static_f64[64])).abs(),self.scalar_static_f64[65])),self.scalar_static_f64[70]);
        let v548=((self.scalar_static_f64[68]*v544)*v547);
        let v552=((common.v35*self.scalar_static_f64[72])).exp();
        let v553=(self.scalar_static_f64[71]*v552);
        let v557=((common.v35*self.scalar_static_f64[74])).exp();
        let v560=f64::powf((common.v12+f64::powf((((self.scalar_static_f64[4]*v207)/self.scalar_static_f64[66])).abs(),self.scalar_static_f64[67])),self.scalar_static_f64[75]);
        let v561=((self.scalar_static_f64[73]*v557)*v560);
        let v589=(common.v12+f64::powf(((common.v583).abs()/self.scalar_static_f64[84]),self.scalar_static_f64[85]));
        let v591=(if (self.scalar_static_f64[83]!=0.0){(v548/v589)}else{v548});
        let v777=((if (self.scalar_static_f64[87]!=0.0){(v591+self.scalar_static_f64[88])}else{v591})/self.scalar_static_f64[3]);
        let v782=((if (self.scalar_static_f64[87]!=0.0){(v561+self.scalar_static_f64[90])}else{v561})/self.scalar_static_f64[3]);
        let v787=((if (self.scalar_static_f64[87]!=0.0){(v553+self.scalar_static_f64[89])}else{v553})/self.scalar_static_f64[3]);
        let v790=1e-6;
        let v793=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v486);
        let v800=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v583);
        let v806=ctx.node_voltage(nodes[0]);
        let v810=((-((v496*common.v562)).abs())-((v498*(common.v202-v806))).abs());
        let v816=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v815);
        let v825=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v815);
        let v831=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v830);
        let v841=(v777>self.scalar_static_f64[129]);
        let v842=(if v841{v777}else{self.scalar_static_f64[129]});
        let v845=(v782>self.scalar_static_f64[129]);
        let v846=(if v845{v782}else{self.scalar_static_f64[129]});
        let v849=(v806-common.v19);
        let v850=(v787>self.scalar_static_f64[129]);
        let v851=(if v850{v787}else{self.scalar_static_f64[129]});
        let v887=(self.scalar_static_f64[6]*f64::powf(v25,self.scalar_static_f64[137]));
        let v896=(common.v38*(self.scalar_static_f64[9]*common.v894));
        let v898=(v29*(self.scalar_static_f64[10]*v896));
        let v899=(v40*(self.scalar_static_f64[5]*((-(if v23{self.scalar_static_f64[136]}else{common.v13}))*v887)));
        let v900=(v40*(self.scalar_static_f64[5]*((-(if v23{self.scalar_static_f64[4]}else{common.v13}))*v887)));
        let v921=(common.v38*common.v38);
        let v1425=(if (v430!=0.0){((-(common.v21*(self.scalar_static_f64[32]*common.v892)))/(v431*v431))}else{common.v1324});
        let v1426=(if (v430!=0.0){(self.scalar_static_f64[136]/v431)}else{common.v1325});
        let v1427=(if (v430!=0.0){(self.scalar_static_f64[4]/v431)}else{common.v1326});
        let v1428=(if (v430!=0.0){common.v13}else{common.v1327});
        let v1429=(if (v430!=0.0){common.v1314}else{common.v1315});
        let v1430=(if (v430!=0.0){common.v1222}else{common.v1316});
        let v1431=(if (v430!=0.0){common.v1221}else{common.v1317});
        let v1432=(if (v430!=0.0){common.v13}else{common.v1318});
        let v1433=(if (v430!=0.0){common.v1228}else{common.v1319});
        let v1442=(if v444{common.v13}else{(if v438{v1425}else{common.v1348})});
        let v1443=(if v444{common.v13}else{(if v438{v1426}else{common.v1349})});
        let v1444=(if v444{common.v13}else{(if v438{v1427}else{common.v1350})});
        let v1445=(if v444{common.v13}else{(if v438{v1428}else{common.v1351})});
        let v1466=(v454*v1429);
        let v1467=(v454*v1430);
        let v1468=(v454*v1431);
        let v1469=(v454*v1432);
        let v1486=(v466*v1433);
        let v1528=(v41*v41);
        let v1611=(v547*(self.scalar_static_f64[68]*(v544*(self.scalar_static_f64[69]*common.v894))));
        let v1939=ddt_scale;
        let v1970=(self.scalar_static_f64[134]*v1939);
        let v1980=-0.0;

        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * ((-(common.v201-common.v486))),
            5,
            multiplicity * (self.scalar_static_f64[136]),
            6,
            multiplicity * (self.scalar_static_f64[4]),
            9,
            multiplicity * (common.v12),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((common.v486*v790)),
            9,
            multiplicity * (v790),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((self.scalar_static_f64[133]*v793)),
            9,
            multiplicity * ((self.scalar_static_f64[133]*v1939)),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * ((if (self.scalar_static_f64[83]!=0.0){(common.v576*(-(common.v278/v41)))}else{common.v13})),
            [3, 4, 5, 6],
            [(if (self.scalar_static_f64[83]!=0.0){(common.v576*(-(((v41*common.v1142)-(common.v278*v898))/v1528)))}else{common.v13}), (if (self.scalar_static_f64[83]!=0.0){(common.v576*(-((-(common.v278*v899))/v1528)))}else{common.v13}), (if (self.scalar_static_f64[83]!=0.0){(common.v576*(-(((v41*common.v1143)-(common.v278*v900))/v1528)))}else{common.v13}), (if (self.scalar_static_f64[83]!=0.0){(common.v576*(-(common.v1144/v41)))}else{common.v13})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if (self.scalar_static_f64[83]!=0.0){common.v583}else{common.v13})),
            8,
            multiplicity * (self.scalar_static_f64[157]),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if (self.scalar_static_f64[83]!=0.0){(common.v576*v800)}else{common.v13})),
            8,
            multiplicity * ((if (self.scalar_static_f64[83]!=0.0){(common.v576*v1939)}else{common.v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            None,
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v13,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if (self.scalar_static_f64[116]!=0.0){v810}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (self.scalar_static_f64[116]!=0.0){(common.v1/self.scalar_static_f64[115])}else{common.v13})),
            3,
            multiplicity * (self.scalar_static_f64[159]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (self.scalar_static_f64[116]!=0.0){v816}else{common.v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[116]!=0.0){v1970}else{common.v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v13,
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{v810}else{common.v13})),
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[28]{((common.v1-common.v821)/self.scalar_static_f64[115])}else{common.v13})),
            3,
            multiplicity * (self.scalar_static_f64[161]),
            7,
            multiplicity * (self.scalar_static_f64[162]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{v825}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[28]{v1970}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{(common.v821/self.scalar_static_f64[117])}else{common.v13})),
            7,
            multiplicity * (self.scalar_static_f64[164]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{v831}else{common.v13})),
            7,
            multiplicity * ((if self.scalar_static_bool[28]{(self.scalar_static_f64[135]*v1939)}else{common.v13})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[31]{v810}else{common.v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            common.v13,
        );
        stamper.stamp_potential_branch_local(
            Some(3),
            None,
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            common.v13,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            None,
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            common.v13,
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(6),
            multiplicity * ((common.v13*common.v200)),
            6,
            multiplicity * (v1980),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(4),
            multiplicity * ((common.v13*common.v20)),
            4,
            multiplicity * (v1980),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(6),
            multiplicity * ((common.v13*(common.v19-common.v199))),
            6,
            multiplicity * (v1980),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((if (self.scalar_static_f64[130]!=0.0){(v205/v842)}else{common.v13})),
            1,
            multiplicity * ((if (self.scalar_static_f64[130]!=0.0){(common.v12/v842)}else{common.v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[130]!=0.0){((-(v205*(if v841{((if (self.scalar_static_f64[83]!=0.0){(v1611/v589)}else{v1611})/self.scalar_static_f64[3])}else{common.v13})))/(v842*v842))}else{common.v13})),
            5,
            multiplicity * ((if (self.scalar_static_f64[130]!=0.0){(v287/v842)}else{common.v13})),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (common.v13),
        );
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(5),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            common.v13,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(6),
            multiplicity * ((if (self.scalar_static_f64[131]!=0.0){(v207/v846)}else{common.v13})),
            2,
            multiplicity * ((if (self.scalar_static_f64[131]!=0.0){(common.v12/v846)}else{common.v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[131]!=0.0){((-(v207*(if v845{((v560*(self.scalar_static_f64[73]*(v557*(self.scalar_static_f64[74]*common.v894))))/self.scalar_static_f64[3])}else{common.v13})))/(v846*v846))}else{common.v13})),
            6,
            multiplicity * ((if (self.scalar_static_f64[131]!=0.0){(v287/v846)}else{common.v13})),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (common.v13),
        );
        stamper.stamp_potential_branch_local(
            Some(2),
            Some(6),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            common.v13,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * ((if (self.scalar_static_f64[132]!=0.0){(v849/v851)}else{common.v13})),
            0,
            multiplicity * ((if (self.scalar_static_f64[132]!=0.0){(common.v12/v851)}else{common.v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[132]!=0.0){((-(v849*(if v850{((self.scalar_static_f64[71]*(v552*(self.scalar_static_f64[72]*common.v894)))/self.scalar_static_f64[3])}else{common.v13})))/(v851*v851))}else{common.v13})),
            4,
            multiplicity * ((if (self.scalar_static_f64[132]!=0.0){(v287/v851)}else{common.v13})),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (common.v13),
        );
        stamper.stamp_potential_branch_local(
            Some(0),
            Some(4),
            7,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            7,
            common.v13,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[4]*v496))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v368{common.v13}else{(if (common.v313!=0.0){(((v362*(((common.v38*(self.scalar_static_f64[29]*(common.v78*(common.v909/self.scalar_static_f64[30]))))-(common.v79*v896))/v921))+(common.v80*common.v1251))-(((common.v273*(common.v13*common.v1275))-(v364*common.v1128))/common.v1132))}else{common.v13})})+(((v41*(common.v1142-(if v310{common.v13}else{(if (common.v280!=0.0){((v307*(self.scalar_static_f64[28]*(common.v73*(self.scalar_static_f64[26]*common.v894))))+(common.v74*common.v1191))}else{common.v13})})))-(v494*v898))/v1528)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((-(v494*v899))/v1528))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v368{common.v13}else{(if (common.v313!=0.0){((common.v80*common.v1252)-((common.v13*common.v1276)/common.v273))}else{common.v13})})+(((v41*(common.v1143-(if v310{common.v13}else{(if (common.v280!=0.0){(common.v74*common.v1192)}else{common.v13})})))-(v494*v900))/v1528)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v368{common.v13}else{(if (common.v313!=0.0){((common.v80*common.v1253)-((common.v13*common.v1277)/common.v273))}else{common.v13})})+((common.v1144-(if v310{common.v13}else{(if (common.v280!=0.0){(common.v74*common.v1193)}else{common.v13})}))/v41))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[4]*v498))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v484{common.v13}else{(if (v430!=0.0){(((v475*(((common.v38*(self.scalar_static_f64[31]*(v84*(common.v909/self.scalar_static_f64[32]))))-(v85*v896))/v921))+(v86*(if (v430!=0.0){((v446*v1442)+(v445*(v446*(if v438{common.v13}else{v1425}))))}else{v1442})))-((common.v13*(if (v430!=0.0){((if v453{(v1466/v455)}else{(if v457{v1466}else{(if v449{v1429}else{common.v13})})})-(if v465{(v1486/v467)}else{(if v469{v1486}else{(if v461{v1433}else{common.v13})})}))}else{common.v1378}))/v480))}else{common.v13})})+(((v43*common.v1414)-(common.v428*(self.scalar_static_f64[11]*v896)))/(v43*v43))))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v484{common.v13}else{(if (v430!=0.0){((v86*(if (v430!=0.0){((v446*v1443)+(v445*(v446*(if v438{common.v13}else{v1426}))))}else{v1443}))-((common.v13*(if (v430!=0.0){(if v453{(v1467/v455)}else{(if v457{v1467}else{(if v449{v1430}else{common.v13})})})}else{common.v1379}))/v480))}else{common.v13})})+(common.v1415/v43)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v484{common.v13}else{(if (v430!=0.0){((v86*(if (v430!=0.0){((v446*v1444)+(v445*(v446*(if v438{common.v13}else{v1427}))))}else{v1444}))-((common.v13*(if (v430!=0.0){(if v453{(v1468/v455)}else{(if v457{v1468}else{(if v449{v1431}else{common.v13})})})}else{common.v1380}))/v480))}else{common.v13})})+(common.v1416/v43)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v484{common.v13}else{(if (v430!=0.0){((v86*(if (v430!=0.0){((v446*v1445)+(v445*(v446*(if v438{common.v13}else{v1428}))))}else{v1445}))-((common.v13*(if (v430!=0.0){(if v453{(v1469/v455)}else{(if v457{v1469}else{(if v449{v1432}else{common.v13})})})}else{common.v1381}))/v480))}else{common.v13})})+(common.v1417/v43))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v520)))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1567))), (self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1570))), (self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1573))), (self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1576)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(((v493*common.v521)*self.scalar_static_f64[62])+(common.v519*v526))))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v493*common.v1577))+(common.v519*(self.scalar_static_f64[63]*common.v1142))))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v493*common.v1578))+(v526*common.v1564)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v493*common.v1581))+((v526*common.v1565)+(common.v519*(self.scalar_static_f64[63]*common.v1143)))))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v493*common.v1584))+((v526*common.v1566)+(common.v519*(self.scalar_static_f64[63]*common.v1144))))))],
            [],
            [],
            multiplicity,
        );
        let v864_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v864);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v864_ddt),
            3,
            multiplicity * (((common.v2050) * ddt_scale)),
            5,
            multiplicity * (((common.v2051) * ddt_scale)),
            6,
            multiplicity * (((common.v2052) * ddt_scale)),
        );
        let v866_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v866);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v866_ddt),
            3,
            multiplicity * (((common.v2056) * ddt_scale)),
            5,
            multiplicity * (((common.v2057) * ddt_scale)),
            6,
            multiplicity * (((common.v2058) * ddt_scale)),
        );
        let v868_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v868);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (v868_ddt),
            [1, 3, 4, 5, 6],
            [((common.v2064) * ddt_scale), ((common.v2065) * ddt_scale), ((common.v2066) * ddt_scale), ((common.v2067) * ddt_scale), ((common.v2068) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v870_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v870);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (v870_ddt),
            [1, 3, 4, 5, 6],
            [((common.v2074) * ddt_scale), ((common.v2075) * ddt_scale), ((common.v2076) * ddt_scale), ((common.v2077) * ddt_scale), ((common.v2078) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v872_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v872);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v872_ddt),
            [3, 4, 5, 6],
            [((common.v2083) * ddt_scale), ((common.v2084) * ddt_scale), ((common.v2085) * ddt_scale), ((common.v2086) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v874_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, common.v874);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (v874_ddt),
            2,
            multiplicity * (((common.v2090) * ddt_scale)),
            3,
            multiplicity * (((common.v2091) * ddt_scale)),
            4,
            multiplicity * (((common.v2092) * ddt_scale)),
        );
        let v876_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, common.v876);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v876_ddt),
            [3, 4, 5, 6],
            [((common.v2097) * ddt_scale), ((common.v2098) * ddt_scale), ((common.v2099) * ddt_scale), ((common.v2100) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v877_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, common.v877);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v877_ddt),
            [3, 4, 5, 6],
            [((common.v2101) * ddt_scale), ((common.v2102) * ddt_scale), ((common.v2103) * ddt_scale), ((common.v2104) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(6),
            multiplicity * (common.v13),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v793=0.0;
        let v800=0.0;
        let v816=0.0;
        let v825=0.0;
        let v831=0.0;
        let v1939=1.0;
        let v1970=(self.scalar_static_f64[134]*v1939);

        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * ((self.scalar_static_f64[133]*v1939)),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * ((if (self.scalar_static_f64[83]!=0.0){(common.v576*v1939)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if (self.scalar_static_f64[116]!=0.0){v1970}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[28]{v1970}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * ((if self.scalar_static_bool[28]{(self.scalar_static_f64[135]*v1939)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (common.v2050),
            nodes[5],
            multiplicity * (common.v2051),
            nodes[6],
            multiplicity * (common.v2052),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (common.v2056),
            nodes[5],
            multiplicity * (common.v2057),
            nodes[6],
            multiplicity * (common.v2058),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2064, common.v2065, common.v2066, common.v2067, common.v2068],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2074, common.v2075, common.v2076, common.v2077, common.v2078],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2083, common.v2084, common.v2085, common.v2086],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (common.v2090),
            nodes[3],
            multiplicity * (common.v2091),
            nodes[4],
            multiplicity * (common.v2092),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2097, common.v2098, common.v2099, common.v2100],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2101, common.v2102, common.v2103, common.v2104],
            &[],
            &[],
            multiplicity,
        );
    }
}
