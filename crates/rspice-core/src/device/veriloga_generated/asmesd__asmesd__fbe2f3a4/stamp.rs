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
    v1: f64,
    v12: f64,
    v13: f64,
    v18: f64,
    v19: f64,
    v20: f64,
    v21: f64,
    v33: f64,
    v35: f64,
    v38: f64,
    v66: f64,
    v73: f64,
    v74: f64,
    v78: f64,
    v79: f64,
    v80: f64,
    v196: f64,
    v199: f64,
    v200: f64,
    v201: f64,
    v202: f64,
    v223: f64,
    v236: f64,
    v239: f64,
    v268: f64,
    v271: f64,
    v276: f64,
    v277: bool,
    v302: f64,
    v308: bool,
    v316: f64,
    v329: f64,
    v355: f64,
    v370: f64,
    v371: f64,
    v372: f64,
    v378: f64,
    v384: f64,
    v410: f64,
    v414: f64,
    v421: f64,
    v477: f64,
    v510: f64,
    v511: f64,
    v512: f64,
    v553: f64,
    v567: f64,
    v573: f64,
    v793: f64,
    v799: f64,
    v808: f64,
    v842: f64,
    v844: f64,
    v846: f64,
    v848: f64,
    v850: f64,
    v852: f64,
    v854: f64,
    v855: f64,
    v870: f64,
    v872: f64,
    v887: f64,
    v1106: f64,
    v1110: f64,
    v1120: f64,
    v1121: f64,
    v1122: f64,
    v1169: f64,
    v1170: f64,
    v1171: f64,
    v1199: f64,
    v1200: f64,
    v1206: f64,
    v1229: f64,
    v1230: f64,
    v1231: f64,
    v1253: f64,
    v1254: f64,
    v1255: f64,
    v1292: f64,
    v1293: f64,
    v1294: f64,
    v1295: f64,
    v1296: f64,
    v1297: f64,
    v1302: f64,
    v1303: f64,
    v1304: f64,
    v1305: f64,
    v1326: f64,
    v1327: f64,
    v1328: f64,
    v1329: f64,
    v1356: f64,
    v1357: f64,
    v1358: f64,
    v1359: f64,
    v1392: f64,
    v1393: f64,
    v1394: f64,
    v1395: f64,
    v1542: f64,
    v1543: f64,
    v1544: f64,
    v1545: f64,
    v1548: f64,
    v1551: f64,
    v1554: f64,
    v1555: f64,
    v1556: f64,
    v1559: f64,
    v1562: f64,
    v2028: f64,
    v2029: f64,
    v2030: f64,
    v2034: f64,
    v2035: f64,
    v2036: f64,
    v2042: f64,
    v2043: f64,
    v2044: f64,
    v2045: f64,
    v2046: f64,
    v2052: f64,
    v2053: f64,
    v2054: f64,
    v2055: f64,
    v2056: f64,
    v2061: f64,
    v2062: f64,
    v2063: f64,
    v2064: f64,
    v2068: f64,
    v2069: f64,
    v2070: f64,
    v2075: f64,
    v2076: f64,
    v2077: f64,
    v2078: f64,
    v2079: f64,
    v2080: f64,
    v2081: f64,
    v2082: f64,
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
        let v209=(v71>v13);
        let v211=(v33*self.scalar_static_f64[53]);
        let v213=(if v209{(v201/v211)}else{v13});
        let v214=(-v201);
        let v215=(v214-v96);
        let v217=(v33*self.scalar_static_f64[54]);
        let v219=(if v209{(v215/v217)}else{v13});
        let v220=(-v96);
        let v222=(if v209{(v220/v217)}else{v13});
        let v223=80.0;
        let v224=(v213>v223);
        let v225=(v209&&v224);
        let v229=(if v225{v223}else{v213});
        let v231=(v209&&(!v224));
        let v232=(if v231{v12}else{(if v225{(v12+(v213-v223))}else{v13})});
        let v233=(v229).exp();
        let v235=(if v209{(v232*v233)}else{v232});
        let v236=37.0;
        let v237=(v219>=v236);
        let v238=(!v237);
        let v239=-37.0;
        let v240=(v219<=v239);
        let v242=(v238&&(!v240));
        let v243=(v219).exp();
        let v244=(v12+v243);
        let v246=(v238&&v240);
        let v250=(v222>=v236);
        let v251=(!v250);
        let v252=(v222<=v239);
        let v254=(v251&&(!v252));
        let v255=(v222).exp();
        let v256=(v12+v255);
        let v258=(v251&&v252);
        let v263=(if v209{((if v242{(v244).ln()}else{(if v246{v243}else{(if v237{v219}else{v13})})})-(if v254{(v256).ln()}else{(if v258{v255}else{(if v250{v222}else{v13})})}))}else{v13});
        let v264=(v235-v12);
        let v266=(v91*v263);
        let v268=(v201).abs();
        let v269=f64::powf(v268,v101);
        let v271=(v12+(self.scalar_static_f64[55]*v269));
        let v275=(!v209);
        let v276=(if v275{v13}else{(if v209{((v71*v264)-(v266/v271))}else{v13})});
        let v277=(v74>v13);
        let v279=(self.scalar_static_f64[56]-v201);
        let v280=0.001;
        let v281=(v279>v280);
        let v283=(if v277{(if v281{v279}else{v280})}else{v13});
        let v285=(v214*self.scalar_static_f64[56]);
        let v287=(v33*self.scalar_static_f64[57]);
        let v288=(v283*v287);
        let v290=(if v277{(v285/v288)}else{v229});
        let v291=(v290>v223);
        let v292=(v277&&v291);
        let v296=(if v292{v223}else{v290});
        let v298=(v277&&(!v291));
        let v299=(if v298{v12}else{(if v292{(v12+(v290-v223))}else{v235})});
        let v300=(v296).exp();
        let v302=(if v277{(v299*v300)}else{v299});
        let v308=(v80>v13);
        let v309=(v33*self.scalar_static_f64[30]);
        let v311=(if v308{(v201/v309)}else{v296});
        let v313=(v33*self.scalar_static_f64[58]);
        let v315=(if v308{(v215/v313)}else{v219});
        let v316=(v220/v313);
        let v317=(if v308{v316}else{v222});
        let v318=(v311>v223);
        let v319=(v308&&v318);
        let v323=(if v319{v223}else{v311});
        let v325=(v308&&(!v318));
        let v326=(if v325{v12}else{(if v319{(v12+(v311-v223))}else{v302})});
        let v327=(v323).exp();
        let v329=(if v308{(v326*v327)}else{v326});
        let v330=(v315>=v236);
        let v331=(!v330);
        let v332=(v315<=v239);
        let v334=(v331&&(!v332));
        let v335=(v315).exp();
        let v336=(v12+v335);
        let v338=(v331&&v332);
        let v342=(v317>=v236);
        let v343=(!v342);
        let v344=(v317<=v239);
        let v346=(v343&&(!v344));
        let v347=(v317).exp();
        let v348=(v12+v347);
        let v350=(v343&&v344);
        let v355=(if v308{((if v334{(v336).ln()}else{(if v338{v335}else{(if v330{v315}else{v13})})})-(if v346{(v348).ln()}else{(if v350{v347}else{(if v342{v317}else{v13})})}))}else{v263});
        let v365=(v33*self.scalar_static_f64[59]);
        let v367=(if v209{(v21/v365)}else{v323});
        let v369=((-v21)-v96);
        let v370=(v369/v313);
        let v371=(if v209{v370}else{v315});
        let v372=(if v209{v316}else{v317});
        let v373=(v367>v223);
        let v374=(v209&&v373);
        let v378=(if v374{v223}else{v367});
        let v380=(v209&&(!v373));
        let v381=(if v380{v12}else{(if v374{(v12+(v367-v223))}else{v329})});
        let v382=(v378).exp();
        let v384=(if v209{(v381*v382)}else{v381});
        let v385=(v371>=v236);
        let v386=(!v385);
        let v387=(v371<=v239);
        let v389=(v386&&(!v387));
        let v390=(v371).exp();
        let v391=(v12+v390);
        let v393=(v386&&v387);
        let v397=(v372>=v236);
        let v398=(!v397);
        let v399=(v372<=v239);
        let v401=(v398&&(!v399));
        let v402=(v372).exp();
        let v403=(v12+v402);
        let v405=(v398&&v399);
        let v410=(if v209{((if v389{(v391).ln()}else{(if v393{v390}else{(if v385{v371}else{v13})})})-(if v401{(v403).ln()}else{(if v405{v402}else{(if v397{v372}else{v13})})}))}else{v355});
        let v411=(v384-v12);
        let v413=(v106*v410);
        let v414=(v21).abs();
        let v415=f64::powf(v414,v101);
        let v417=(v12+(self.scalar_static_f64[55]*v415));
        let v421=(if v275{v13}else{(if v209{((v71*v411)-(v413/v417))}else{v13})});
        let v477=ctx.node_voltage(nodes[9]);
        let v507=(v12+f64::powf(((v12+(((v276*(self.scalar_static_f64[20]*(v12+(v21*self.scalar_static_f64[60]))))+(self.scalar_static_f64[23]*v421))*4.0))).abs(),self.scalar_static_f64[61]));
        let v510=((((v12-(self.scalar_static_f64[17]*v201))-(v21*self.scalar_static_f64[14]))*2.0)/v507);
        let v511=(v421*v510);
        let v512=(v276*v510);
        let v553=(v202-v196);
        let v567=(self.scalar_static_f64[79]*(v12+((f64::powf((v12+f64::powf(((v553/self.scalar_static_f64[76])).abs(),self.scalar_static_f64[77])),self.scalar_static_f64[78])-v12)*self.scalar_static_f64[80])));
        let v573=ctx.node_voltage(nodes[8]);
        let v593=(v198<=v13);
        let v594=(v189*v195);
        let v597=(v12-(v198/v189));
        let v600=((self.scalar_static_f64[89]*(v597).ln())).exp();
        let v601=(v12-v600);
        let v605=(!v593);
        let v606=(v195*v198);
        let v609=(v198*self.scalar_static_f64[90]);
        let v611=(v12+(v609/v189));
        let v617=(v201+((-v151)*self.scalar_static_f64[91]));
        let v618=(v617>v13);
        let v624=(if v618{self.scalar_static_f64[96]}else{v13});
        let v627=(v12-(self.scalar_static_f64[93]*(self.scalar_static_f64[93]*v624)));
        let v633=(v617*self.scalar_static_f64[98]);
        let v635=(self.scalar_static_f64[93]+(v633/v151));
        let v639=(!v618);
        let v641=(v12-(v201/v151));
        let v644=((self.scalar_static_f64[97]*(v641).ln())).exp();
        let v645=(v12-v644);
        let v648=(if v639{((v151*v645)/self.scalar_static_f64[97])}else{(if v618{((v151*v627)/self.scalar_static_f64[97])}else{v13})});
        let v649=(if v639{v13}else{(if v618{(v624*(v617*v635))}else{v13})});
        let v650=(v648+v649);
        let v653=(self.scalar_static_f64[91]*(-v171));
        let v654=(v204+v653);
        let v655=(v654>v13);
        let v659=(if v655{self.scalar_static_f64[101]}else{v624});
        let v662=(v12-(self.scalar_static_f64[93]*(self.scalar_static_f64[93]*v659)));
        let v668=(v654*self.scalar_static_f64[103]);
        let v670=(self.scalar_static_f64[93]+(v668/v171));
        let v674=(!v655);
        let v676=(v12-(v204/v171));
        let v679=((self.scalar_static_f64[102]*(v676).ln())).exp();
        let v680=(v12-v679);
        let v683=(if v674{((v171*v680)/self.scalar_static_f64[102])}else{(if v655{((v171*v662)/self.scalar_static_f64[102])}else{v648})});
        let v684=(if v674{v13}else{(if v655{(v659*(v654*v670))}else{v649})});
        let v685=(v683+v684);
        let v690=(v21+v653);
        let v691=(v690>v13);
        let v692=(if v691{self.scalar_static_f64[101]}else{v659});
        let v695=(v12-(self.scalar_static_f64[93]*(self.scalar_static_f64[93]*v692)));
        let v699=(self.scalar_static_f64[103]*v690);
        let v701=(self.scalar_static_f64[93]+(v699/v171));
        let v705=(!v691);
        let v707=(v12-(v21/v171));
        let v710=((self.scalar_static_f64[102]*(v707).ln())).exp();
        let v711=(v12-v710);
        let v716=((if v705{((v171*v711)/self.scalar_static_f64[102])}else{(if v691{((v171*v695)/self.scalar_static_f64[102])}else{v683})})+(if v705{v13}else{(if v691{(v692*(v690*v701))}else{v684})}));
        let v732=(if self.scalar_static_bool[9]{v13}else{(if self.scalar_static_bool[8]{(v512*self.scalar_static_f64[110])}else{v13})});
        let v793=(v1*self.scalar_static_f64[125]);
        let v799=ctx.node_voltage(nodes[7]);
        let v808=(v799*self.scalar_static_f64[126]);
        let v842=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v159*v650)));
        let v844=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v276*v567)));
        let v846=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((v177*v685)*self.scalar_static_f64[105])));
        let v848=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[104]*(v177*v716))));
        let v850=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v511*self.scalar_static_f64[81])));
        let v852=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v605{(v606*v611)}else{(if v593{((v594*v601)/self.scalar_static_f64[89])}else{v13})})));
        let v854=(self.scalar_static_f64[3]*(-v732));
        let v855=(self.scalar_static_f64[3]*v732);
        let v857=(if v10{v13}else{(if v8{v12}else{v13})});
        let v870=(v32*v857);
        let v871=(v857/self.scalar_static_f64[8]);
        let v872=(v871/v34);
        let v887=((self.scalar_static_f64[24]*v872)+(((v33*(self.scalar_static_f64[25]*v871))-(v64*v870))/(v33*v33)));
        let v890=(self.scalar_static_f64[27]*(v70*v887));
        let v913=(self.scalar_static_f64[37]*(self.scalar_static_f64[38]*v871));
        let v916=(v857/v110);
        let v941=((v135*(-(v870+v870)))+(v129*((v130*(v916/v112))+(v133*(((v124*(((v118*((v115*v857)+(v11*(v114*v857))))-(v116*v857))/(v118*v118)))-(v121*(v122*(v857+v857))))/(v124*v124))))));
        let v943=((-v941)/self.scalar_static_f64[44]);
        let v944=(-v943);
        let v948=(v139*v139);
        let v957=(v112*v943);
        let v959=(v941+((v139*v916)+v957));
        let v965=(v143*v857);
        let v974=(v162*v162);
        let v984=(v941+(v957+(v162*v916)));
        let v994=((v176*((-(self.scalar_static_f64[42]*(self.scalar_static_f64[50]*(-(((v162*v944)-(v163*v943))/v974)))))/(v168*v168)))+(v169*(self.scalar_static_f64[50]*(v965-(((v162*(v984-v943))-(v172*v943))/v974)))));
        let v998=(v180*v180);
        let v1008=(v941+(v957+(v180*v916)));
        let v1018=((v194*((-(self.scalar_static_f64[43]*(self.scalar_static_f64[52]*(-(((v180*v944)-(v181*v943))/v998)))))/(v186*v186)))+(v187*(self.scalar_static_f64[52]*(v965-(((v180*(v1008-v943))-(v190*v943))/v998)))));
        let v1026=(if v209{((-(v201*(self.scalar_static_f64[53]*v870)))/(v211*v211))}else{v13});
        let v1027=(if v209{(self.scalar_static_f64[4]/v211)}else{v13});
        let v1028=(if v209{(self.scalar_static_f64[127]/v211)}else{v13});
        let v1029=(-(self.scalar_static_f64[35]*(self.scalar_static_f64[36]*v871)));
        let v1030=(self.scalar_static_f64[54]*v870);
        let v1031=(v217*v1029);
        let v1034=(v217*v217);
        let v1038=(if v209{((v1031-(v215*v1030))/v1034)}else{v13});
        let v1039=(if v209{(self.scalar_static_f64[127]/v217)}else{v13});
        let v1040=(if v209{(self.scalar_static_f64[4]/v217)}else{v13});
        let v1044=(if v209{((v1031-(v220*v1030))/v1034)}else{v13});
        let v1048=(if v225{v13}else{v1026});
        let v1049=(if v225{v13}else{v1027});
        let v1050=(if v225{v13}else{v1028});
        let v1051=(if v231{v13}else{(if v225{v1026}else{v13})});
        let v1052=(if v231{v13}else{(if v225{v1027}else{v13})});
        let v1053=(if v231{v13}else{(if v225{v1028}else{v13})});
        let v1066=(if v209{((v233*v1051)+(v232*(v233*v1048)))}else{v1051});
        let v1067=(if v209{((v233*v1052)+(v232*(v233*v1049)))}else{v1052});
        let v1068=(if v209{((v233*v1053)+(v232*(v233*v1050)))}else{v1053});
        let v1069=(v243*v1038);
        let v1070=(v243*v1039);
        let v1071=(v243*v1040);
        let v1084=(v255*v1044);
        let v1090=(if v209{((if v242{(v1069/v244)}else{(if v246{v1069}else{(if v237{v1038}else{v13})})})-(if v254{(v1084/v256)}else{(if v258{v1084}else{(if v250{v1044}else{v13})})}))}else{v13});
        let v1091=(if v209{(if v242{(v1070/v244)}else{(if v246{v1070}else{(if v237{v1039}else{v13})})})}else{v13});
        let v1092=(if v209{(if v242{(v1071/v244)}else{(if v246{v1071}else{(if v237{v1040}else{v13})})})}else{v13});
        let v1106=(self.scalar_static_f64[55]*(v913*(v269*(v268).ln())));
        let v1110=(v271*v271);
        let v1120=(if v275{v13}else{(if v209{(((v264*v890)+(v71*v1066))-(((v271*((v263*(self.scalar_static_f64[33]*(self.scalar_static_f64[34]*v871)))+(v91*v1090)))-(v266*v1106))/v1110))}else{v13})});
        let v1121=(if v275{v13}else{(if v209{((v71*v1067)-((v91*v1091)/v271))}else{v13})});
        let v1122=(if v275{v13}else{(if v209{((v71*v1068)-((v91*v1092)/v271))}else{v13})});
        let v1135=(v288*v288);
        let v1145=(if v277{((-(v285*(v283*(self.scalar_static_f64[57]*v870))))/v1135)}else{v1048});
        let v1146=(if v277{(((v288*self.scalar_static_f64[129])-(v285*(v287*(if v277{(if v281{self.scalar_static_f64[127]}else{v13})}else{v13}))))/v1135)}else{v1049});
        let v1147=(if v277{(((v288*self.scalar_static_f64[130])-(v285*(v287*(if v277{(if v281{self.scalar_static_f64[4]}else{v13})}else{v13}))))/v1135)}else{v1050});
        let v1151=(if v292{v13}else{v1145});
        let v1152=(if v292{v13}else{v1146});
        let v1153=(if v292{v13}else{v1147});
        let v1154=(if v298{v13}else{(if v292{v1145}else{v1066})});
        let v1155=(if v298{v13}else{(if v292{v1146}else{v1067})});
        let v1156=(if v298{v13}else{(if v292{v1147}else{v1068})});
        let v1169=(if v277{((v300*v1154)+(v299*(v300*v1151)))}else{v1154});
        let v1170=(if v277{((v300*v1155)+(v299*(v300*v1152)))}else{v1155});
        let v1171=(if v277{((v300*v1156)+(v299*(v300*v1153)))}else{v1156});
        let v1190=(if v308{((-(v201*(self.scalar_static_f64[30]*v870)))/(v309*v309))}else{v1151});
        let v1191=(if v308{(self.scalar_static_f64[4]/v309)}else{v1152});
        let v1192=(if v308{(self.scalar_static_f64[127]/v309)}else{v1153});
        let v1193=(self.scalar_static_f64[58]*v870);
        let v1194=(v313*v1029);
        let v1197=(v313*v313);
        let v1199=(self.scalar_static_f64[127]/v313);
        let v1200=(self.scalar_static_f64[4]/v313);
        let v1201=(if v308{((v1194-(v215*v1193))/v1197)}else{v1038});
        let v1202=(if v308{v1199}else{v1039});
        let v1203=(if v308{v1200}else{v1040});
        let v1206=((v1194-(v220*v1193))/v1197);
        let v1207=(if v308{v1206}else{v1044});
        let v1211=(if v319{v13}else{v1190});
        let v1212=(if v319{v13}else{v1191});
        let v1213=(if v319{v13}else{v1192});
        let v1214=(if v325{v13}else{(if v319{v1190}else{v1169})});
        let v1215=(if v325{v13}else{(if v319{v1191}else{v1170})});
        let v1216=(if v325{v13}else{(if v319{v1192}else{v1171})});
        let v1229=(if v308{((v327*v1214)+(v326*(v327*v1211)))}else{v1214});
        let v1230=(if v308{((v327*v1215)+(v326*(v327*v1212)))}else{v1215});
        let v1231=(if v308{((v327*v1216)+(v326*(v327*v1213)))}else{v1216});
        let v1232=(v335*v1201);
        let v1233=(v335*v1202);
        let v1234=(v335*v1203);
        let v1247=(v347*v1207);
        let v1253=(if v308{((if v334{(v1232/v336)}else{(if v338{v1232}else{(if v330{v1201}else{v13})})})-(if v346{(v1247/v348)}else{(if v350{v1247}else{(if v342{v1207}else{v13})})}))}else{v1090});
        let v1254=(if v308{(if v334{(v1233/v336)}else{(if v338{v1233}else{(if v330{v1202}else{v13})})})}else{v1091});
        let v1255=(if v308{(if v334{(v1234/v336)}else{(if v338{v1234}else{(if v330{v1203}else{v13})})})}else{v1092});
        let v1286=(if v209{((-(v21*(self.scalar_static_f64[59]*v870)))/(v365*v365))}else{v1211});
        let v1287=(if v209{(self.scalar_static_f64[127]/v365)}else{v13});
        let v1288=(if v209{(self.scalar_static_f64[4]/v365)}else{v1212});
        let v1289=(if v209{v13}else{v1213});
        let v1292=((v1194-(v369*v1193))/v1197);
        let v1293=(if v209{v1292}else{v1201});
        let v1294=(if v209{v1200}else{v13});
        let v1295=(if v209{v1199}else{v1202});
        let v1296=(if v209{v13}else{v1203});
        let v1297=(if v209{v1206}else{v1207});
        let v1302=(if v374{v13}else{v1286});
        let v1303=(if v374{v13}else{v1287});
        let v1304=(if v374{v13}else{v1288});
        let v1305=(if v374{v13}else{v1289});
        let v1306=(if v380{v13}else{(if v374{v1286}else{v1229})});
        let v1307=(if v380{v13}else{(if v374{v1287}else{v13})});
        let v1308=(if v380{v13}else{(if v374{v1288}else{v1230})});
        let v1309=(if v380{v13}else{(if v374{v1289}else{v1231})});
        let v1326=(if v209{((v382*v1306)+(v381*(v382*v1302)))}else{v1306});
        let v1327=(if v209{((v382*v1307)+(v381*(v382*v1303)))}else{v1307});
        let v1328=(if v209{((v382*v1308)+(v381*(v382*v1304)))}else{v1308});
        let v1329=(if v209{((v382*v1309)+(v381*(v382*v1305)))}else{v1309});
        let v1330=(v390*v1293);
        let v1331=(v390*v1294);
        let v1332=(v390*v1295);
        let v1333=(v390*v1296);
        let v1350=(v402*v1297);
        let v1356=(if v209{((if v389{(v1330/v391)}else{(if v393{v1330}else{(if v385{v1293}else{v13})})})-(if v401{(v1350/v403)}else{(if v405{v1350}else{(if v397{v1297}else{v13})})}))}else{v1253});
        let v1357=(if v209{(if v389{(v1331/v391)}else{(if v393{v1331}else{(if v385{v1294}else{v13})})})}else{v13});
        let v1358=(if v209{(if v389{(v1332/v391)}else{(if v393{v1332}else{(if v385{v1295}else{v13})})})}else{v1254});
        let v1359=(if v209{(if v389{(v1333/v391)}else{(if v393{v1333}else{(if v385{v1296}else{v13})})})}else{v1255});
        let v1392=(if v275{v13}else{(if v209{(((v411*v890)+(v71*v1326))-(((v417*((v410*(self.scalar_static_f64[39]*(self.scalar_static_f64[40]*v871)))+(v106*v1356)))-(v413*(self.scalar_static_f64[55]*(v913*(v415*(v414).ln())))))/(v417*v417)))}else{v13})});
        let v1393=(if v275{v13}else{(if v209{((v71*v1327)-((v106*v1357)/v417))}else{v13})});
        let v1394=(if v275{v13}else{(if v209{((v71*v1328)-((v106*v1358)/v417))}else{v13})});
        let v1395=(if v275{v13}else{(if v209{((v71*v1329)-((v106*v1359)/v417))}else{v13})});
        let v1542=(self.scalar_static_f64[139]/v507);
        let v1543=(self.scalar_static_f64[140]/v507);
        let v1544=(self.scalar_static_f64[141]/v507);
        let v1545=(v510*v1392);
        let v1548=((v510*v1393)+(v421*v1542));
        let v1551=((v510*v1394)+(v421*v1543));
        let v1554=((v510*v1395)+(v421*v1544));
        let v1555=(v510*v1120);
        let v1556=(v276*v1542);
        let v1559=((v510*v1121)+(v276*v1543));
        let v1562=((v510*v1122)+(v276*v1544));
        let v1612=(v189*v189);
        let v1664=(self.scalar_static_f64[91]*(-v959));
        let v1674=(v151*v151);
        let v1721=(if v639{(((v645*v959)+(v151*(-(v644*(self.scalar_static_f64[97]*((-((-(v201*v959))/v1674))/v641))))))/self.scalar_static_f64[97])}else{(if v618{((v627*v959)/self.scalar_static_f64[97])}else{v13})});
        let v1722=(if v639{((v151*(-(v644*(self.scalar_static_f64[97]*((-(self.scalar_static_f64[4]/v151))/v641)))))/self.scalar_static_f64[97])}else{v13});
        let v1723=(if v639{((v151*(-(v644*(self.scalar_static_f64[97]*((-(self.scalar_static_f64[127]/v151))/v641)))))/self.scalar_static_f64[97])}else{v13});
        let v1724=(if v639{v13}else{(if v618{(v624*((v635*v1664)+(v617*(((v151*(self.scalar_static_f64[98]*v1664))-(v633*v959))/v1674))))}else{v13})});
        let v1725=(if v639{v13}else{(if v618{(v624*((self.scalar_static_f64[4]*v635)+(v617*(self.scalar_static_f64[144]/v151))))}else{v13})});
        let v1726=(if v639{v13}else{(if v618{(v624*((v635*self.scalar_static_f64[127])+(v617*(self.scalar_static_f64[145]/v151))))}else{v13})});
        let v1736=(self.scalar_static_f64[91]*(-v984));
        let v1745=(self.scalar_static_f64[146]/v171);
        let v1746=(v171*(self.scalar_static_f64[103]*v1736));
        let v1749=(v171*v171);
        let v1751=(self.scalar_static_f64[147]/v171);
        let v1774=(-(self.scalar_static_f64[4]/v171));
        let v1776=(-(self.scalar_static_f64[127]/v171));
        let v1797=(if v674{((v171*(-(v679*(self.scalar_static_f64[102]*(v1774/v676)))))/self.scalar_static_f64[102])}else{v13});
        let v1798=(if v674{(((v680*v984)+(v171*(-(v679*(self.scalar_static_f64[102]*((-((-(v204*v984))/v1749))/v676))))))/self.scalar_static_f64[102])}else{(if v655{((v662*v984)/self.scalar_static_f64[102])}else{v1721})});
        let v1799=(if v674{((v171*(-(v679*(self.scalar_static_f64[102]*(v1776/v676)))))/self.scalar_static_f64[102])}else{v13});
        let v1800=(if v674{v13}else{(if v655{v13}else{v1722})});
        let v1801=(if v674{v13}else{(if v655{v13}else{v1723})});
        let v1802=(if v674{v13}else{(if v655{(v659*((self.scalar_static_f64[4]*v670)+(v654*v1745)))}else{v13})});
        let v1803=(if v674{v13}else{(if v655{(v659*((v670*v1736)+(v654*((v1746-(v668*v984))/v1749))))}else{v1724})});
        let v1804=(if v674{v13}else{(if v655{(v659*((v670*self.scalar_static_f64[127])+(v654*v1751)))}else{v13})});
        let v1805=(if v674{v13}else{(if v655{v13}else{v1725})});
        let v1806=(if v674{v13}else{(if v655{v13}else{v1726})});
        let v1910=(if self.scalar_static_bool[9]{v13}else{(if self.scalar_static_bool[8]{(self.scalar_static_f64[110]*v1555)}else{v13})});
        let v1911=(if self.scalar_static_bool[9]{v13}else{(if self.scalar_static_bool[8]{(self.scalar_static_f64[110]*v1556)}else{v13})});
        let v1912=(if self.scalar_static_bool[9]{v13}else{(if self.scalar_static_bool[8]{(self.scalar_static_f64[110]*v1559)}else{v13})});
        let v1913=(if self.scalar_static_bool[9]{v13}else{(if self.scalar_static_bool[8]{(self.scalar_static_f64[110]*v1562)}else{v13})});
        let v2028=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((v650*((v158*((-(self.scalar_static_f64[41]*(self.scalar_static_f64[46]*(-(((v139*v944)-(v140*v943))/v948)))))/(v148*v148)))+(v149*(self.scalar_static_f64[46]*(v965-(((v139*(v959-v943))-(v152*v943))/v948))))))+(v159*(v1721+v1724)))));
        let v2029=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v159*(v1722+v1725))));
        let v2030=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v159*(v1723+v1726))));
        let v2034=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v567*v1120)));
        let v2035=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v567*v1121)));
        let v2036=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(v567*v1122)));
        let v2042=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[105]*(v177*(v1797+v1802)))));
        let v2043=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[105]*((v685*v994)+(v177*(v1798+v1803))))));
        let v2044=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[105]*(v177*(v1799+v1804)))));
        let v2045=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[105]*(v177*(v1800+v1805)))));
        let v2046=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[105]*(v177*(v1801+v1806)))));
        let v2052=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[104]*(v177*((if v705{v13}else{(if v691{v13}else{v1797})})+(if v705{v13}else{(if v691{v13}else{v1802})}))))));
        let v2053=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[104]*((v716*v994)+(v177*((if v705{(((v711*v984)+(v171*(-(v710*(self.scalar_static_f64[102]*((-((-(v21*v984))/v1749))/v707))))))/self.scalar_static_f64[102])}else{(if v691{((v695*v984)/self.scalar_static_f64[102])}else{v1798})})+(if v705{v13}else{(if v691{(v692*((v701*v1736)+(v690*((v1746-(v699*v984))/v1749))))}else{v1803})})))))));
        let v2054=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[104]*(v177*((if v705{((v171*(-(v710*(self.scalar_static_f64[102]*(v1776/v707)))))/self.scalar_static_f64[102])}else{(if v691{v13}else{v1799})})+(if v705{v13}else{(if v691{(v692*((v701*self.scalar_static_f64[127])+(v690*v1751)))}else{v1804})}))))));
        let v2055=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[104]*(v177*((if v705{((v171*(-(v710*(self.scalar_static_f64[102]*(v1774/v707)))))/self.scalar_static_f64[102])}else{(if v691{v13}else{v1800})})+(if v705{v13}else{(if v691{(v692*((self.scalar_static_f64[4]*v701)+(v690*v1745)))}else{v1805})}))))));
        let v2056=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[104]*(v177*((if v705{v13}else{(if v691{v13}else{v1801})})+(if v705{v13}else{(if v691{v13}else{v1806})}))))));
        let v2061=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1545)));
        let v2062=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1548)));
        let v2063=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1551)));
        let v2064=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(self.scalar_static_f64[81]*v1554)));
        let v2068=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v605{((v611*(self.scalar_static_f64[4]*v195))+(v606*(self.scalar_static_f64[142]/v189)))}else{(if v593{((v594*(-(v600*(self.scalar_static_f64[89]*((-(self.scalar_static_f64[4]/v189))/v597)))))/self.scalar_static_f64[89])}else{v13})})));
        let v2069=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v605{((v611*(v198*v1018))+(v606*((-(v609*v1008))/v1612)))}else{(if v593{(((v601*((v195*v1008)+(v189*v1018)))+(v594*(-(v600*(self.scalar_static_f64[89]*((-((-(v198*v1008))/v1612))/v597))))))/self.scalar_static_f64[89])}else{v13})})));
        let v2070=(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(if v605{((v611*(v195*self.scalar_static_f64[127]))+(v606*(self.scalar_static_f64[143]/v189)))}else{(if v593{((v594*(-(v600*(self.scalar_static_f64[89]*((-(self.scalar_static_f64[127]/v189))/v597)))))/self.scalar_static_f64[89])}else{v13})})));
        let v2075=(self.scalar_static_f64[3]*(-v1910));
        let v2076=(self.scalar_static_f64[3]*(-v1911));
        let v2077=(self.scalar_static_f64[3]*(-v1912));
        let v2078=(self.scalar_static_f64[3]*(-v1913));
        let v2079=(self.scalar_static_f64[3]*v1910);
        let v2080=(self.scalar_static_f64[3]*v1911);
        let v2081=(self.scalar_static_f64[3]*v1912);
        let v2082=(self.scalar_static_f64[3]*v1913);

        CommonStampValues {
            v1,
            v12,
            v13,
            v18,
            v19,
            v20,
            v21,
            v33,
            v35,
            v38,
            v66,
            v73,
            v74,
            v78,
            v79,
            v80,
            v196,
            v199,
            v200,
            v201,
            v202,
            v223,
            v236,
            v239,
            v268,
            v271,
            v276,
            v277,
            v302,
            v308,
            v316,
            v329,
            v355,
            v370,
            v371,
            v372,
            v378,
            v384,
            v410,
            v414,
            v421,
            v477,
            v510,
            v511,
            v512,
            v553,
            v567,
            v573,
            v793,
            v799,
            v808,
            v842,
            v844,
            v846,
            v848,
            v850,
            v852,
            v854,
            v855,
            v870,
            v872,
            v887,
            v1106,
            v1110,
            v1120,
            v1121,
            v1122,
            v1169,
            v1170,
            v1171,
            v1199,
            v1200,
            v1206,
            v1229,
            v1230,
            v1231,
            v1253,
            v1254,
            v1255,
            v1292,
            v1293,
            v1294,
            v1295,
            v1296,
            v1297,
            v1302,
            v1303,
            v1304,
            v1305,
            v1326,
            v1327,
            v1328,
            v1329,
            v1356,
            v1357,
            v1358,
            v1359,
            v1392,
            v1393,
            v1394,
            v1395,
            v1542,
            v1543,
            v1544,
            v1545,
            v1548,
            v1551,
            v1554,
            v1555,
            v1556,
            v1559,
            v1562,
            v2028,
            v2029,
            v2030,
            v2034,
            v2035,
            v2036,
            v2042,
            v2043,
            v2044,
            v2045,
            v2046,
            v2052,
            v2053,
            v2054,
            v2055,
            v2056,
            v2061,
            v2062,
            v2063,
            v2064,
            v2068,
            v2069,
            v2070,
            v2075,
            v2076,
            v2077,
            v2078,
            v2079,
            v2080,
            v2081,
            v2082,
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
        let v284=-1.0;
        let v303=(common.v302-common.v12);
        let v306=(!common.v277);
        let v356=(common.v329-common.v12);
        let v358=(common.v13*common.v355);
        let v362=(!common.v308);
        let v422=(v86>common.v13);
        let v423=(common.v33*self.scalar_static_f64[32]);
        let v425=(if v422{(common.v21/v423)}else{common.v378});
        let v426=(if v422{common.v370}else{common.v371});
        let v427=(if v422{common.v316}else{common.v372});
        let v428=(v425>common.v223);
        let v429=(v422&&v428);
        let v435=(v422&&(!v428));
        let v436=(if v435{common.v12}else{(if v429{(common.v12+(v425-common.v223))}else{common.v384})});
        let v437=((if v429{common.v223}else{v425})).exp();
        let v440=(v426>=common.v236);
        let v441=(!v440);
        let v442=(v426<=common.v239);
        let v444=(v441&&(!v442));
        let v445=(v426).exp();
        let v446=(common.v12+v445);
        let v448=(v441&&v442);
        let v452=(v427>=common.v236);
        let v453=(!v452);
        let v454=(v427<=common.v239);
        let v456=(v453&&(!v454));
        let v457=(v427).exp();
        let v458=(common.v12+v457);
        let v460=(v453&&v454);
        let v466=((if v422{(v436*v437)}else{v436})-common.v12);
        let v471=(common.v12+(self.scalar_static_f64[55]*f64::powf(common.v414,self.scalar_static_f64[37])));
        let v475=(!v422);
        let v480=1e-9;
        let v484=(((if (common.v477<common.v201){common.v477}else{common.v201})/(if (common.v268>v480){common.v268}else{v480}))).abs();
        let v485=(common.v276-(if v306{common.v13}else{(if common.v277{(common.v74*v303)}else{common.v13})}));
        let v487=((if v362{common.v13}else{(if common.v308{((common.v80*v356)-(v358/common.v271))}else{common.v13})})+(v485/v41));
        let v489=((if v475{common.v13}else{(if v422{((v86*v466)-((common.v13*(if v422{((if v444{(v446).ln()}else{(if v448{v445}else{(if v440{v426}else{common.v13})})})-(if v456{(v458).ln()}else{(if v460{v457}else{(if v452{v427}else{common.v13})})}))}else{common.v410}))/v471))}else{common.v13})})+(common.v421/v43));
        let v517=(common.v276*self.scalar_static_f64[63]);
        let v535=((common.v35*self.scalar_static_f64[69])).exp();
        let v538=f64::powf((common.v12+f64::powf((((self.scalar_static_f64[4]*v205)/self.scalar_static_f64[64])).abs(),self.scalar_static_f64[65])),self.scalar_static_f64[70]);
        let v539=((self.scalar_static_f64[68]*v535)*v538);
        let v543=((common.v35*self.scalar_static_f64[72])).exp();
        let v544=(self.scalar_static_f64[71]*v543);
        let v548=((common.v35*self.scalar_static_f64[74])).exp();
        let v551=f64::powf((common.v12+f64::powf((((self.scalar_static_f64[4]*v207)/self.scalar_static_f64[66])).abs(),self.scalar_static_f64[67])),self.scalar_static_f64[75]);
        let v552=((self.scalar_static_f64[73]*v548)*v551);
        let v579=(common.v12+f64::powf(((common.v573).abs()/self.scalar_static_f64[83]),self.scalar_static_f64[84]));
        let v581=(if self.scalar_static_bool[4]{(v539/v579)}else{v539});
        let v757=((if self.scalar_static_bool[5]{(v581+self.scalar_static_f64[86])}else{v581})/self.scalar_static_f64[3]);
        let v761=((if self.scalar_static_bool[5]{(v552+self.scalar_static_f64[88])}else{v552})/self.scalar_static_f64[3]);
        let v765=((if self.scalar_static_bool[5]{(v544+self.scalar_static_f64[87])}else{v544})/self.scalar_static_f64[3]);
        let v768=1e-6;
        let v771=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v477);
        let v778=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v573);
        let v784=ctx.node_voltage(nodes[0]);
        let v788=((-((v487*common.v553)).abs())-((v489*(common.v202-v784))).abs());
        let v794=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v793);
        let v803=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v793);
        let v809=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v808);
        let v819=(v757>self.scalar_static_f64[123]);
        let v820=(if v819{v757}else{self.scalar_static_f64[123]});
        let v823=(v761>self.scalar_static_f64[123]);
        let v824=(if v823{v761}else{self.scalar_static_f64[123]});
        let v827=(v784-common.v19);
        let v828=(v765>self.scalar_static_f64[123]);
        let v829=(if v828{v765}else{self.scalar_static_f64[123]});
        let v865=(self.scalar_static_f64[6]*f64::powf(v25,self.scalar_static_f64[128]));
        let v874=(common.v38*(self.scalar_static_f64[9]*common.v872));
        let v876=(v29*(self.scalar_static_f64[10]*v874));
        let v877=(v40*(self.scalar_static_f64[5]*((-(if v23{self.scalar_static_f64[127]}else{common.v13}))*v865)));
        let v878=(v40*(self.scalar_static_f64[5]*((-(if v23{self.scalar_static_f64[4]}else{common.v13}))*v865)));
        let v899=(common.v38*common.v38);
        let v1403=(if v422{((-(common.v21*(self.scalar_static_f64[32]*common.v870)))/(v423*v423))}else{common.v1302});
        let v1404=(if v422{(self.scalar_static_f64[127]/v423)}else{common.v1303});
        let v1405=(if v422{(self.scalar_static_f64[4]/v423)}else{common.v1304});
        let v1406=(if v422{common.v13}else{common.v1305});
        let v1407=(if v422{common.v1292}else{common.v1293});
        let v1408=(if v422{common.v1200}else{common.v1294});
        let v1409=(if v422{common.v1199}else{common.v1295});
        let v1410=(if v422{common.v13}else{common.v1296});
        let v1411=(if v422{common.v1206}else{common.v1297});
        let v1420=(if v435{common.v13}else{(if v429{v1403}else{common.v1326})});
        let v1421=(if v435{common.v13}else{(if v429{v1404}else{common.v1327})});
        let v1422=(if v435{common.v13}else{(if v429{v1405}else{common.v1328})});
        let v1423=(if v435{common.v13}else{(if v429{v1406}else{common.v1329})});
        let v1444=(v445*v1407);
        let v1445=(v445*v1408);
        let v1446=(v445*v1409);
        let v1447=(v445*v1410);
        let v1464=(v457*v1411);
        let v1506=(v41*v41);
        let v1589=(v538*(self.scalar_static_f64[68]*(v535*(self.scalar_static_f64[69]*common.v872))));
        let v1917=ddt_scale;
        let v1948=(self.scalar_static_f64[125]*v1917);
        let v1958=-0.0;

        stamper.stamp_current_node3_local(
            Some(9),
            None,
            multiplicity * ((-(common.v201-common.v477))),
            5,
            multiplicity * (self.scalar_static_f64[127]),
            6,
            multiplicity * (self.scalar_static_f64[4]),
            9,
            multiplicity * (common.v12),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((common.v477*v768)),
            9,
            multiplicity * (v768),
        );
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * ((self.scalar_static_f64[124]*v771)),
            9,
            multiplicity * ((self.scalar_static_f64[124]*v1917)),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            None,
            multiplicity * ((if self.scalar_static_bool[4]{(common.v567*(-(common.v276/v41)))}else{common.v13})),
            [3, 4, 5, 6],
            [(if self.scalar_static_bool[4]{(common.v567*(-(((v41*common.v1120)-(common.v276*v876))/v1506)))}else{common.v13}), (if self.scalar_static_bool[4]{(common.v567*(-((-(common.v276*v877))/v1506)))}else{common.v13}), (if self.scalar_static_bool[4]{(common.v567*(-(((v41*common.v1121)-(common.v276*v878))/v1506)))}else{common.v13}), (if self.scalar_static_bool[4]{(common.v567*(-(common.v1122/v41)))}else{common.v13})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if self.scalar_static_bool[4]{common.v573}else{common.v13})),
            8,
            multiplicity * (self.scalar_static_f64[148]),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * ((if self.scalar_static_bool[4]{(common.v567*v778)}else{common.v13})),
            8,
            multiplicity * ((if self.scalar_static_bool[4]{(common.v567*v1917)}else{common.v13})),
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
            multiplicity * ((if self.scalar_static_bool[12]{v788}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[12]{(common.v1/self.scalar_static_f64[112])}else{common.v13})),
            3,
            multiplicity * (self.scalar_static_f64[150]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[12]{v794}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[12]{v1948}else{common.v13})),
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
            multiplicity * ((if self.scalar_static_bool[28]{v788}else{common.v13})),
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[28]{((common.v1-common.v799)/self.scalar_static_f64[112])}else{common.v13})),
            3,
            multiplicity * (self.scalar_static_f64[152]),
            7,
            multiplicity * (self.scalar_static_f64[153]),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{v803}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[28]{v1948}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{(common.v799/self.scalar_static_f64[113])}else{common.v13})),
            7,
            multiplicity * (self.scalar_static_f64[155]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * ((if self.scalar_static_bool[28]{v809}else{common.v13})),
            7,
            multiplicity * ((if self.scalar_static_bool[28]{(self.scalar_static_f64[126]*v1917)}else{common.v13})),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[31]{v788}else{common.v13})),
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
            multiplicity * (v1958),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(4),
            multiplicity * ((common.v13*common.v20)),
            4,
            multiplicity * (v1958),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            Some(6),
            multiplicity * ((common.v13*(common.v19-common.v199))),
            6,
            multiplicity * (v1958),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[20]{(v205/v820)}else{common.v13})),
            1,
            multiplicity * ((if self.scalar_static_bool[20]{(common.v12/v820)}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[20]{((-(v205*(if v819{((if self.scalar_static_bool[4]{(v1589/v579)}else{v1589})/self.scalar_static_f64[3])}else{common.v13})))/(v820*v820))}else{common.v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[20]{(v284/v820)}else{common.v13})),
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
            multiplicity * ((if self.scalar_static_bool[23]{(v207/v824)}else{common.v13})),
            2,
            multiplicity * ((if self.scalar_static_bool[23]{(common.v12/v824)}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[23]{((-(v207*(if v823{((v551*(self.scalar_static_f64[73]*(v548*(self.scalar_static_f64[74]*common.v872))))/self.scalar_static_f64[3])}else{common.v13})))/(v824*v824))}else{common.v13})),
            6,
            multiplicity * ((if self.scalar_static_bool[23]{(v284/v824)}else{common.v13})),
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
            multiplicity * ((if self.scalar_static_bool[26]{(v827/v829)}else{common.v13})),
            0,
            multiplicity * ((if self.scalar_static_bool[26]{(common.v12/v829)}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[26]{((-(v827*(if v828{((self.scalar_static_f64[71]*(v543*(self.scalar_static_f64[72]*common.v872)))/self.scalar_static_f64[3])}else{common.v13})))/(v829*v829))}else{common.v13})),
            4,
            multiplicity * ((if self.scalar_static_bool[26]{(v284/v829)}else{common.v13})),
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
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[4]*v487))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v362{common.v13}else{(if common.v308{(((v356*(((common.v38*(self.scalar_static_f64[29]*(common.v78*(common.v887/self.scalar_static_f64[30]))))-(common.v79*v874))/v899))+(common.v80*common.v1229))-(((common.v271*(common.v13*common.v1253))-(v358*common.v1106))/common.v1110))}else{common.v13})})+(((v41*(common.v1120-(if v306{common.v13}else{(if common.v277{((v303*(self.scalar_static_f64[28]*(common.v73*(self.scalar_static_f64[26]*common.v872))))+(common.v74*common.v1169))}else{common.v13})})))-(v485*v876))/v1506)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((-(v485*v877))/v1506))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v362{common.v13}else{(if common.v308{((common.v80*common.v1230)-((common.v13*common.v1254)/common.v271))}else{common.v13})})+(((v41*(common.v1121-(if v306{common.v13}else{(if common.v277{(common.v74*common.v1170)}else{common.v13})})))-(v485*v878))/v1506)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v362{common.v13}else{(if common.v308{((common.v80*common.v1231)-((common.v13*common.v1255)/common.v271))}else{common.v13})})+((common.v1122-(if v306{common.v13}else{(if common.v277{(common.v74*common.v1171)}else{common.v13})}))/v41))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[4]*v489))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v475{common.v13}else{(if v422{(((v466*(((common.v38*(self.scalar_static_f64[31]*(v84*(common.v887/self.scalar_static_f64[32]))))-(v85*v874))/v899))+(v86*(if v422{((v437*v1420)+(v436*(v437*(if v429{common.v13}else{v1403}))))}else{v1420})))-((common.v13*(if v422{((if v444{(v1444/v446)}else{(if v448{v1444}else{(if v440{v1407}else{common.v13})})})-(if v456{(v1464/v458)}else{(if v460{v1464}else{(if v452{v1411}else{common.v13})})}))}else{common.v1356}))/v471))}else{common.v13})})+(((v43*common.v1392)-(common.v421*(self.scalar_static_f64[11]*v874)))/(v43*v43))))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v475{common.v13}else{(if v422{((v86*(if v422{((v437*v1421)+(v436*(v437*(if v429{common.v13}else{v1404}))))}else{v1421}))-((common.v13*(if v422{(if v444{(v1445/v446)}else{(if v448{v1445}else{(if v440{v1408}else{common.v13})})})}else{common.v1357}))/v471))}else{common.v13})})+(common.v1393/v43)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v475{common.v13}else{(if v422{((v86*(if v422{((v437*v1422)+(v436*(v437*(if v429{common.v13}else{v1405}))))}else{v1422}))-((common.v13*(if v422{(if v444{(v1446/v446)}else{(if v448{v1446}else{(if v440{v1409}else{common.v13})})})}else{common.v1358}))/v471))}else{common.v13})})+(common.v1394/v43)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((if v475{common.v13}else{(if v422{((v86*(if v422{((v437*v1423)+(v436*(v437*(if v429{common.v13}else{v1406}))))}else{v1423}))-((common.v13*(if v422{(if v444{(v1447/v446)}else{(if v448{v1447}else{(if v440{v1410}else{common.v13})})})}else{common.v1359}))/v471))}else{common.v13})})+(common.v1395/v43))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v511)))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1545))), (self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1548))), (self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1551))), (self.scalar_static_f64[4]*(self.scalar_static_f64[3]*(-common.v1554)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(6),
            multiplicity * ((self.scalar_static_f64[3]*(self.scalar_static_f64[4]*(((v484*common.v512)*self.scalar_static_f64[62])+(common.v510*v517))))),
            [3, 4, 5, 6],
            [(self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v484*common.v1555))+(common.v510*(self.scalar_static_f64[63]*common.v1120))))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v484*common.v1556))+(v517*common.v1542)))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v484*common.v1559))+((v517*common.v1543)+(common.v510*(self.scalar_static_f64[63]*common.v1121)))))), (self.scalar_static_f64[3]*(self.scalar_static_f64[4]*((self.scalar_static_f64[62]*(v484*common.v1562))+((v517*common.v1544)+(common.v510*(self.scalar_static_f64[63]*common.v1122))))))],
            [],
            [],
            multiplicity,
        );
        let v842_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v842);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v842_ddt),
            3,
            multiplicity * (((common.v2028) * ddt_scale)),
            5,
            multiplicity * (((common.v2029) * ddt_scale)),
            6,
            multiplicity * (((common.v2030) * ddt_scale)),
        );
        let v844_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v844);
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (v844_ddt),
            3,
            multiplicity * (((common.v2034) * ddt_scale)),
            5,
            multiplicity * (((common.v2035) * ddt_scale)),
            6,
            multiplicity * (((common.v2036) * ddt_scale)),
        );
        let v846_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v846);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(1),
            Some(4),
            multiplicity * (v846_ddt),
            [1, 3, 4, 5, 6],
            [((common.v2042) * ddt_scale), ((common.v2043) * ddt_scale), ((common.v2044) * ddt_scale), ((common.v2045) * ddt_scale), ((common.v2046) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v848_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v848);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(4),
            multiplicity * (v848_ddt),
            [1, 3, 4, 5, 6],
            [((common.v2052) * ddt_scale), ((common.v2053) * ddt_scale), ((common.v2054) * ddt_scale), ((common.v2055) * ddt_scale), ((common.v2056) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v850_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v850);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v850_ddt),
            [3, 4, 5, 6],
            [((common.v2061) * ddt_scale), ((common.v2062) * ddt_scale), ((common.v2063) * ddt_scale), ((common.v2064) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v852_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, common.v852);
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * (v852_ddt),
            2,
            multiplicity * (((common.v2068) * ddt_scale)),
            3,
            multiplicity * (((common.v2069) * ddt_scale)),
            4,
            multiplicity * (((common.v2070) * ddt_scale)),
        );
        let v854_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, common.v854);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v854_ddt),
            [3, 4, 5, 6],
            [((common.v2075) * ddt_scale), ((common.v2076) * ddt_scale), ((common.v2077) * ddt_scale), ((common.v2078) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v855_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, common.v855);
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (v855_ddt),
            [3, 4, 5, 6],
            [((common.v2079) * ddt_scale), ((common.v2080) * ddt_scale), ((common.v2081) * ddt_scale), ((common.v2082) * ddt_scale)],
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
        let v771=0.0;
        let v778=0.0;
        let v794=0.0;
        let v803=0.0;
        let v809=0.0;
        let v1917=1.0;
        let v1948=(self.scalar_static_f64[125]*v1917);

        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * ((self.scalar_static_f64[124]*v1917)),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[4]{(common.v567*v1917)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[12]{v1948}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[28]{v1948}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * ((if self.scalar_static_bool[28]{(self.scalar_static_f64[126]*v1917)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (common.v2028),
            nodes[5],
            multiplicity * (common.v2029),
            nodes[6],
            multiplicity * (common.v2030),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[3],
            multiplicity * (common.v2034),
            nodes[5],
            multiplicity * (common.v2035),
            nodes[6],
            multiplicity * (common.v2036),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2042, common.v2043, common.v2044, common.v2045, common.v2046],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[1], nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2052, common.v2053, common.v2054, common.v2055, common.v2056],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2061, common.v2062, common.v2063, common.v2064],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes[2],
            multiplicity * (common.v2068),
            nodes[3],
            multiplicity * (common.v2069),
            nodes[4],
            multiplicity * (common.v2070),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2075, common.v2076, common.v2077, common.v2078],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6]],
            &[common.v2079, common.v2080, common.v2081, common.v2082],
            &[],
            &[],
            multiplicity,
        );
    }
}
