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

struct CommonStampValues {
    v0: f64,
    v1: f64,
    v3: f64,
    v5: f64,
    v6: f64,
    v7: f64,
    v8: f64,
    v9: f64,
    v11: f64,
    v13: f64,
    v32: f64,
    v35: f64,
    v39: f64,
    v43: bool,
    v44: f64,
    v46: f64,
    v83: f64,
    v118: bool,
    v122: f64,
    v129: bool,
    v134: bool,
    v140: f64,
    v142: f64,
    v147: f64,
    v152: f64,
    v167: f64,
    v176: f64,
    v181: f64,
    v191: f64,
    v199: f64,
    v208: f64,
    v238: f64,
    v239: f64,
    v241: f64,
    v308: f64,
    v331: f64,
    v358: f64,
    v443: f64,
    v456: f64,
    v560: f64,
    v566: f64,
    v659: f64,
    v664: f64,
    v702: f64,
    v718: f64,
    v758: f64,
    v764: f64,
    v773: f64,
    v776: f64,
    v791: f64,
    v793: f64,
    v795: f64,
    v796: f64,
    v799: f64,
    v800: f64,
    v807: f64,
    v811: f64,
    v813: f64,
    v829: f64,
    v835: f64,
    v841: f64,
    v846: f64,
    v854: f64,
    v855: f64,
    v868: f64,
    v892: f64,
    v893: f64,
    v913: f64,
    v918: f64,
    v920: f64,
    v922: f64,
    v924: f64,
    v925: f64,
    v970: f64,
    v971: f64,
    v972: f64,
    v973: f64,
    v974: f64,
    v1071: f64,
    v1072: f64,
    v1073: f64,
    v1074: f64,
    v1075: f64,
    v1076: f64,
    v1077: f64,
    v1086: f64,
    v1087: f64,
    v1088: f64,
    v1089: f64,
    v1425: f64,
    v1426: f64,
    v1427: f64,
    v1428: f64,
    v1429: f64,
    v1525: f64,
    v1526: f64,
    v1527: f64,
    v1528: f64,
    v1697: f64,
    v1698: f64,
    v1699: f64,
    v1700: f64,
    v1701: f64,
    v2293: f64,
    v2351: f64,
    v2502: f64,
    v2503: f64,
    v2504: f64,
    v2505: f64,
    v2553: f64,
    v2554: f64,
    v2555: f64,
    v2556: f64,
    v2586: f64,
    v2588: f64,
    v2589: f64,
    v2597: f64,
    v2599: f64,
    v2601: f64,
    v2611: f64,
    v2620: f64,
    v2621: f64,
    v2622: f64,
    v2623: f64,
    v2624: f64,
    v2630: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v0=ctx.node_voltage(nodes[12]);
        let v1=ctx.node_voltage(nodes[8]);
        let v2=(v0-v1);
        let v3=ctx.node_voltage(nodes[10]);
        let v4=ctx.node_voltage(nodes[5]);
        let v5=(v3-v4);
        let v6=(-v5);
        let v7=(v4-v1);
        let v8=ctx.node_voltage(nodes[11]);
        let v9=(v8-v1);
        let v10=ctx.node_voltage(nodes[4]);
        let v11=(v10-v1);
        let v13=0.0;
        let v32=ctx.node_voltage(nodes[3]);
        let v35=(if (self.scalar_static_f64[10]!=0.0){(self.scalar_static_f64[191]+(v32).abs())}else{self.scalar_static_f64[191]});
        let v39=((v35-self.scalar_static_f64[9])).abs();
        let v43=((v39>v13)||self.scalar_static_bool[2]);
        let v44=1.0;
        let v46=(v39).abs();
        let v83=(v44+(v46*self.scalar_static_f64[24]));
        let v94=(v44+(v39*self.scalar_static_f64[28]));
        let v118=(v43&&self.scalar_static_bool[7]);
        let v122=(v44+(self.scalar_static_f64[24]*(v39*v39)));
        let v129=(v43&&self.scalar_static_bool[8]);
        let v134=(!v43);
        let v137=(if v134{self.scalar_static_f64[17]}else{(if v43{(self.scalar_static_f64[17]*(v44+(v46*self.scalar_static_f64[18])))}else{v13})});
        let v138=(if v134{self.scalar_static_f64[19]}else{(if v43{(self.scalar_static_f64[19]*(v44+(v46*self.scalar_static_f64[20])))}else{v13})});
        let v139=(if v134{self.scalar_static_f64[21]}else{(if v43{(self.scalar_static_f64[21]*(v44+(v46*self.scalar_static_f64[22])))}else{v13})});
        let v140=(if v134{self.scalar_static_f64[23]}else{(if v43{(self.scalar_static_f64[23]*v83)}else{v13})});
        let v142=(if v134{self.scalar_static_f64[37]}else{(if v129{(v83*self.scalar_static_f64[37])}else{(if v118{(v122*self.scalar_static_f64[37])}else{v13})})});
        let v144=(if v134{self.scalar_static_f64[27]}else{(if v43{(self.scalar_static_f64[27]*v94)}else{v13})});
        let v145=(if v134{self.scalar_static_f64[29]}else{(if v43{(v94*self.scalar_static_f64[29])}else{v13})});
        let v147=(if v134{self.scalar_static_f64[32]}else{(if v43{(self.scalar_static_f64[32]+(v39*self.scalar_static_f64[33]))}else{v13})});
        let v152=0.5;
        let v161=(v7*self.scalar_static_f64[43]);
        let v162=(v161).cosh();
        let v167=1e-12;
        let v169=(v167+(v162*v162));
        let v175=(v44+(v46*self.scalar_static_f64[47]));
        let v176=((self.scalar_static_f64[45]*(v44+(self.scalar_static_f64[46]/v169)))*v175);
        let v181=(self.scalar_static_f64[48]*(v44+(v46*self.scalar_static_f64[49])));
        let v186=((v7*self.scalar_static_f64[51])).tanh();
        let v191=(v6-v147);
        let v192=(self.scalar_static_f64[52]*v191);
        let v196=(v44+(v46*self.scalar_static_f64[26]));
        let v197=((((((if v134{self.scalar_static_f64[25]}else{(if v43{(self.scalar_static_f64[25]+(v39*self.scalar_static_f64[26]))}else{v13})})-self.scalar_static_f64[50])+(self.scalar_static_f64[50]*v186))-(v11*self.scalar_static_f64[44]))-(v191*v192))*v196);
        let v198=(v2-v197);
        let v199=(v198*v198);
        let v204=(v181*v198);
        let v206=(((v176*v198)+(v199*self.scalar_static_f64[53]))+(v199*v204));
        let v207=(v206).tanh();
        let v208=(v44+v207);
        let v210=(-v206);
        let v214=((v152*(scalar_limexp(v206)-scalar_limexp(v210)))).tanh();
        let v222=2.0;
        let v238=(v5-v197);
        let v239=(if self.scalar_static_bool[16]{v238}else{v162});
        let v241=(if self.scalar_static_bool[16]{(v239*v239)}else{v198});
        let v291=(if self.scalar_static_bool[19]{v198}else{v239});
        let v293=(if self.scalar_static_bool[19]{(v291*v291)}else{v241});
        let v296=(v181*v293);
        let v298=((v291+(self.scalar_static_f64[53]*v293))+(v291*v296));
        let v300=(if self.scalar_static_bool[19]{(v176*v298)}else{v206});
        let v302=(-v300);
        let v306=((v152*(scalar_limexp(v300)-scalar_limexp(v302)))).tanh();
        let v308=(if self.scalar_static_bool[19]{(v44+v306)}else{(v44+v214)});
        let v331=(if self.scalar_static_bool[22]{v198}else{v291});
        let v333=(if self.scalar_static_bool[22]{(v331*v331)}else{v293});
        let v336=(v181*v333);
        let v338=((v331+(self.scalar_static_f64[53]*v333))+(v331*v336));
        let v340=(if self.scalar_static_bool[22]{(v176*v338)}else{v300});
        let v352=(-v340);
        let v356=((v152*(scalar_limexp(v340)-scalar_limexp(v352)))).tanh();
        let v358=(if self.scalar_static_bool[22]{(v44+v356)}else{v308});
        let v427=(v44+v208);
        let v440=(v44+v358);
        let v443=(if self.scalar_static_bool[27]{(self.scalar_static_f64[59]+(v139/v440))}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[59]+(v139/v427))}else{v13})});
        let v456=-1.0;
        let v524=(v7*self.scalar_static_f64[79]);
        let v525=((v144+(v9*self.scalar_static_f64[78]))+v524);
        let v526=(v525).tanh();
        let v527=(v44+v526);
        let v532=((self.scalar_static_f64[80]+(v7*self.scalar_static_f64[81]))).tanh();
        let v533=(v44+v532);
        let v538=((self.scalar_static_f64[82]-(v7*self.scalar_static_f64[83]))).tanh();
        let v540=((v44+v538)-self.scalar_static_f64[79]);
        let v544=((v145+(v5*self.scalar_static_f64[84]))-v524);
        let v545=(v544).tanh();
        let v546=(v44+v545);
        let v557=(v137*v527);
        let v560=(if self.scalar_static_bool[39]{(self.scalar_static_f64[85]+(v533*v557))}else{self.scalar_static_f64[86]});
        let v566=(if self.scalar_static_bool[39]{(self.scalar_static_f64[87]+(v138*((v540*v546)+self.scalar_static_f64[89])))}else{self.scalar_static_f64[88]});
        let v571=(if self.scalar_static_bool[42]{(v533-self.scalar_static_f64[79])}else{v533});
        let v572=(v144+v524);
        let v573=(v572).cosh();
        let v574=(if self.scalar_static_bool[42]{v573}else{v13});
        let v576=(if self.scalar_static_bool[42]{(v574).ln()}else{v13});
        let v577=(v525).cosh();
        let v578=(if self.scalar_static_bool[42]{v577}else{v13});
        let v580=(if self.scalar_static_bool[42]{(v578).ln()}else{v13});
        let v582=(if self.scalar_static_bool[42]{(v572+v576)}else{v13});
        let v584=((v525+v580)-v582);
        let v587=(v9*self.scalar_static_f64[89]);
        let v590=(v9*self.scalar_static_f64[85]);
        let v593=(v145-v524);
        let v594=(v593).cosh();
        let v595=(if self.scalar_static_bool[42]{v594}else{v574});
        let v597=(if self.scalar_static_bool[42]{(v595).ln()}else{v13});
        let v598=(v544).cosh();
        let v599=(if self.scalar_static_bool[42]{v598}else{v578});
        let v601=(if self.scalar_static_bool[42]{(v599).ln()}else{v13});
        let v603=(if self.scalar_static_bool[42]{(v593+v597)}else{v13});
        let v605=((v544+v601)-v603);
        let v608=(v5*self.scalar_static_f64[89]);
        let v611=(v5*self.scalar_static_f64[87]);
        let v622=(v9/self.scalar_static_f64[90]);
        let v624=(if self.scalar_static_bool[45]{(v622-v44)}else{v13});
        let v627=(v624*v624);
        let v628=(self.scalar_static_f64[92]+v627);
        let v630=f64::powf(v628,self.scalar_static_f64[93]);
        let v634=(self.scalar_static_f64[92]+(v627*self.scalar_static_f64[95]));
        let v640=((v144+(self.scalar_static_f64[78]*(v9+v524)))).tanh();
        let v643=(if self.scalar_static_bool[45]{v533}else{v571});
        let v645=(v538+self.scalar_static_f64[96]);
        let v646=(if self.scalar_static_bool[45]{v645}else{v540});
        let v651=((v145+(self.scalar_static_f64[84]*(v5+(v7*self.scalar_static_f64[96]))))).tanh();
        let v653=(if self.scalar_static_bool[45]{(v44+v651)}else{v546});
        let v657=(v137*((if self.scalar_static_bool[45]{(v44+v640)}else{v527})+((if self.scalar_static_bool[45]{(v630*v634)}else{v13})*self.scalar_static_f64[97])));
        let v659=(self.scalar_static_f64[85]+(v643*v657));
        let v664=(self.scalar_static_f64[87]+(v138*(self.scalar_static_f64[89]+(v646*v653))));
        let v669=(if self.scalar_static_bool[48]{v573}else{v595});
        let v672=(if self.scalar_static_bool[48]{v577}else{v599});
        let v677=(self.scalar_static_f64[97]*(v9+self.scalar_static_f64[90]));
        let v678=(v456+v622);
        let v680=(self.scalar_static_f64[92]+f64::powf(v678,v222));
        let v682=f64::powf(v680,self.scalar_static_f64[99]);
        let v695=(((if self.scalar_static_bool[48]{(v677*v682)}else{v13})+((v525+(if self.scalar_static_bool[48]{(v672).ln()}else{v580}))-(if self.scalar_static_bool[48]{(v572+(if self.scalar_static_bool[48]{(v669).ln()}else{v576}))}else{v582})))-self.scalar_static_f64[104]);
        let v696=(v532+self.scalar_static_f64[96]);
        let v702=(if self.scalar_static_bool[48]{(v590+(v137*(v587+((v695*v696)/self.scalar_static_f64[78]))))}else{(if self.scalar_static_bool[42]{((v137*(((v571*v584)/self.scalar_static_f64[78])+v587))+v590)}else{v13})});
        let v703=(if self.scalar_static_bool[48]{v594}else{v669});
        let v706=(if self.scalar_static_bool[48]{v598}else{v672});
        let v712=((v544+(if self.scalar_static_bool[48]{(v706).ln()}else{v601}))-(if self.scalar_static_bool[48]{(v593+(if self.scalar_static_bool[48]{(v703).ln()}else{v597}))}else{v603}));
        let v718=(if self.scalar_static_bool[48]{(v611+(v138*(v608+((v645*v712)/self.scalar_static_f64[84]))))}else{(if self.scalar_static_bool[42]{((v138*(((v540*v605)/self.scalar_static_f64[84])+v608))+v611)}else{v13})});
        let v758=(if self.scalar_static_bool[66]{((v137*((v35*5.5226012e-23)*self.scalar_static_f64[114]))*self.scalar_static_f64[116])}else{v13});
        let v764=3.141592653589793;
        let v773=(self.scalar_static_f64[117]*ctx.node_voltage(nodes[15]));
        let v776=(self.scalar_static_f64[118]*ctx.branch_current(branches[0]));
        let v791=(self.scalar_static_f64[119]*(ctx.node_voltage(nodes[7])-v4));
        let v793=(v7*self.scalar_static_f64[120]);
        let v795=(ctx.node_voltage(nodes[6])-v10);
        let v796=(v140*v795);
        let v798=ctx.branch_current(branches[1]);
        let v799=(v443*v798);
        let v800=(self.scalar_static_f64[105]*v798);
        let v807=(v2*v142);
        let v811=ctx.node_voltage(nodes[14]);
        let v813=(self.scalar_static_f64[121]*(v8-v811));
        let v829=(self.scalar_static_f64[122]*ctx.branch_current(branches[10]));
        let v835=(self.scalar_static_f64[123]*ctx.branch_current(branches[14]));
        let v841=(self.scalar_static_f64[124]*ctx.branch_current(branches[18]));
        let v846=ctx.node_voltage(nodes[17]);
        let v854=(-(if self.scalar_static_bool[66]{(v758*v764)}else{v13}));
        let v855=(v846*v854);
        let v868=(v32*self.scalar_static_f64[125]);
        let v875=(v161).sinh();
        let v876=(self.scalar_static_f64[43]*v875);
        let v877=(self.scalar_static_f64[126]*v875);
        let v879=(v162*v876);
        let v881=(v162*v877);
        let v885=(v169*v169);
        let v892=(v175*(self.scalar_static_f64[45]*((-(self.scalar_static_f64[46]*(v879+v879)))/v885)));
        let v893=(v175*(self.scalar_static_f64[45]*((-(self.scalar_static_f64[46]*(v881+v881)))/v885)));
        let v896=(v44-(v186*v186));
        let v910=(v196*((self.scalar_static_f64[50]*(self.scalar_static_f64[51]*v896))-(v192+v192)));
        let v911=(v196*((self.scalar_static_f64[50]*(self.scalar_static_f64[128]*v896))-self.scalar_static_f64[127]));
        let v912=(v196*(-((-v192)+(v191*self.scalar_static_f64[129]))));
        let v913=(-(v196*self.scalar_static_f64[127]));
        let v914=(-v910);
        let v915=(v456-v911);
        let v916=(-v912);
        let v917=(v198*v913);
        let v918=(v917+v917);
        let v919=(v198*v914);
        let v920=(v919+v919);
        let v921=(v198*v915);
        let v922=(v921+v921);
        let v923=(v198*v916);
        let v924=(v923+v923);
        let v925=(v198+v198);
        let v963=(((v176*v913)+(self.scalar_static_f64[53]*v918))+((v204*v918)+(v199*(v181*v913))));
        let v964=((((v198*v892)+(v176*v914))+(self.scalar_static_f64[53]*v920))+((v204*v920)+(v199*(v181*v914))));
        let v965=((((v198*v893)+(v176*v915))+(self.scalar_static_f64[53]*v922))+((v204*v922)+(v199*(v181*v915))));
        let v966=(((v176*v916)+(self.scalar_static_f64[53]*v924))+((v204*v924)+(v199*(v181*v916))));
        let v967=((v176+(self.scalar_static_f64[53]*v925))+((v204*v925)+(v181*v199)));
        let v969=(v44-(v207*v207));
        let v970=(v963*v969);
        let v971=(v964*v969);
        let v972=(v965*v969);
        let v973=(v966*v969);
        let v974=(v967*v969);
        let v975=scalar_limexp_derivative(v206);
        let v986=scalar_limexp_derivative(v210);
        let v1003=(v44-(v214*v214));
        let v1071=(v456-v910);
        let v1072=(-v911);
        let v1073=(v44-v912);
        let v1074=(if self.scalar_static_bool[16]{v913}else{v13});
        let v1075=(if self.scalar_static_bool[16]{v1071}else{v876});
        let v1076=(if self.scalar_static_bool[16]{v1072}else{v877});
        let v1077=(if self.scalar_static_bool[16]{v1073}else{v13});
        let v1078=(v239*v1074);
        let v1080=(v239*v1075);
        let v1082=(v239*v1076);
        let v1084=(v239*v1077);
        let v1086=(if self.scalar_static_bool[16]{(v1078+v1078)}else{v913});
        let v1087=(if self.scalar_static_bool[16]{(v1080+v1080)}else{v914});
        let v1088=(if self.scalar_static_bool[16]{(v1082+v1082)}else{v915});
        let v1089=(if self.scalar_static_bool[16]{(v1084+v1084)}else{v916});
        let v1322=(if self.scalar_static_bool[19]{v913}else{v1074});
        let v1323=(if self.scalar_static_bool[19]{v914}else{v1075});
        let v1324=(if self.scalar_static_bool[19]{v915}else{v1076});
        let v1325=(if self.scalar_static_bool[19]{v916}else{v1077});
        let v1327=(v291*v1322);
        let v1329=(v291*v1323);
        let v1331=(v291*v1324);
        let v1333=(v291*v1325);
        let v1335=(v291*self.scalar_static_f64[134]);
        let v1337=(if self.scalar_static_bool[19]{(v1327+v1327)}else{v1086});
        let v1338=(if self.scalar_static_bool[19]{(v1329+v1329)}else{v1087});
        let v1339=(if self.scalar_static_bool[19]{(v1331+v1331)}else{v1088});
        let v1340=(if self.scalar_static_bool[19]{(v1333+v1333)}else{v1089});
        let v1341=(if self.scalar_static_bool[19]{(v1335+v1335)}else{self.scalar_static_f64[131]});
        let v1386=(if self.scalar_static_bool[19]{(v176*((v1322+(self.scalar_static_f64[53]*v1337))+((v296*v1322)+(v291*(v181*v1337)))))}else{v963});
        let v1387=(if self.scalar_static_bool[19]{((v298*v892)+(v176*((v1323+(self.scalar_static_f64[53]*v1338))+((v296*v1323)+(v291*(v181*v1338))))))}else{v964});
        let v1388=(if self.scalar_static_bool[19]{((v298*v893)+(v176*((v1324+(self.scalar_static_f64[53]*v1339))+((v296*v1324)+(v291*(v181*v1339))))))}else{v965});
        let v1389=(if self.scalar_static_bool[19]{(v176*((v1325+(self.scalar_static_f64[53]*v1340))+((v296*v1325)+(v291*(v181*v1340)))))}else{v966});
        let v1390=(if self.scalar_static_bool[19]{(v176*((self.scalar_static_f64[134]+(self.scalar_static_f64[53]*v1341))+((v296*self.scalar_static_f64[134])+(v291*(v181*v1341)))))}else{v967});
        let v1391=scalar_limexp_derivative(v300);
        let v1402=scalar_limexp_derivative(v302);
        let v1419=(v44-(v306*v306));
        let v1425=(if self.scalar_static_bool[19]{((v152*((v1386*v1391)-((-v1386)*v1402)))*v1419)}else{((v152*((v963*v975)-((-v963)*v986)))*v1003)});
        let v1426=(if self.scalar_static_bool[19]{((v152*((v1387*v1391)-((-v1387)*v1402)))*v1419)}else{((v152*((v964*v975)-((-v964)*v986)))*v1003)});
        let v1427=(if self.scalar_static_bool[19]{((v152*((v1388*v1391)-((-v1388)*v1402)))*v1419)}else{((v152*((v965*v975)-((-v965)*v986)))*v1003)});
        let v1428=(if self.scalar_static_bool[19]{((v152*((v1389*v1391)-((-v1389)*v1402)))*v1419)}else{((v152*((v966*v975)-((-v966)*v986)))*v1003)});
        let v1429=(if self.scalar_static_bool[19]{((v152*((v1390*v1391)-((-v1390)*v1402)))*v1419)}else{((v152*((v967*v975)-((-v967)*v986)))*v1003)});
        let v1525=(if self.scalar_static_bool[22]{v913}else{v1322});
        let v1526=(if self.scalar_static_bool[22]{v914}else{v1323});
        let v1527=(if self.scalar_static_bool[22]{v915}else{v1324});
        let v1528=(if self.scalar_static_bool[22]{v916}else{v1325});
        let v1530=(v331*v1525);
        let v1532=(v331*v1526);
        let v1534=(v331*v1527);
        let v1536=(v331*v1528);
        let v1538=(v331*self.scalar_static_f64[135]);
        let v1540=(if self.scalar_static_bool[22]{(v1530+v1530)}else{v1337});
        let v1541=(if self.scalar_static_bool[22]{(v1532+v1532)}else{v1338});
        let v1542=(if self.scalar_static_bool[22]{(v1534+v1534)}else{v1339});
        let v1543=(if self.scalar_static_bool[22]{(v1536+v1536)}else{v1340});
        let v1544=(if self.scalar_static_bool[22]{(v1538+v1538)}else{v1341});
        let v1589=(if self.scalar_static_bool[22]{(v176*((v1525+(self.scalar_static_f64[53]*v1540))+((v336*v1525)+(v331*(v181*v1540)))))}else{v1386});
        let v1590=(if self.scalar_static_bool[22]{((v338*v892)+(v176*((v1526+(self.scalar_static_f64[53]*v1541))+((v336*v1526)+(v331*(v181*v1541))))))}else{v1387});
        let v1591=(if self.scalar_static_bool[22]{((v338*v893)+(v176*((v1527+(self.scalar_static_f64[53]*v1542))+((v336*v1527)+(v331*(v181*v1542))))))}else{v1388});
        let v1592=(if self.scalar_static_bool[22]{(v176*((v1528+(self.scalar_static_f64[53]*v1543))+((v336*v1528)+(v331*(v181*v1543)))))}else{v1389});
        let v1593=(if self.scalar_static_bool[22]{(v176*((self.scalar_static_f64[135]+(self.scalar_static_f64[53]*v1544))+((v336*self.scalar_static_f64[135])+(v331*(v181*v1544)))))}else{v1390});
        let v1663=scalar_limexp_derivative(v340);
        let v1674=scalar_limexp_derivative(v352);
        let v1691=(v44-(v356*v356));
        let v1697=(if self.scalar_static_bool[22]{((v152*((v1589*v1663)-((-v1589)*v1674)))*v1691)}else{v1425});
        let v1698=(if self.scalar_static_bool[22]{((v152*((v1590*v1663)-((-v1590)*v1674)))*v1691)}else{v1426});
        let v1699=(if self.scalar_static_bool[22]{((v152*((v1591*v1663)-((-v1591)*v1674)))*v1691)}else{v1427});
        let v1700=(if self.scalar_static_bool[22]{((v152*((v1592*v1663)-((-v1592)*v1674)))*v1691)}else{v1428});
        let v1701=(if self.scalar_static_bool[22]{((v152*((v1593*v1663)-((-v1593)*v1674)))*v1691)}else{v1429});
        let v2054=(v427*v427);
        let v2085=(v440*v440);
        let v2194=(v44-(v526*v526));
        let v2195=(self.scalar_static_f64[79]*v2194);
        let v2196=(self.scalar_static_f64[150]*v2194);
        let v2197=(self.scalar_static_f64[78]*v2194);
        let v2200=(v44-(v532*v532));
        let v2201=(self.scalar_static_f64[81]*v2200);
        let v2202=(self.scalar_static_f64[151]*v2200);
        let v2205=(v44-(v538*v538));
        let v2206=(self.scalar_static_f64[152]*v2205);
        let v2207=(self.scalar_static_f64[83]*v2205);
        let v2211=(v44-(v545*v545));
        let v2212=(self.scalar_static_f64[154]*v2211);
        let v2213=(self.scalar_static_f64[79]*v2211);
        let v2214=(self.scalar_static_f64[84]*v2211);
        let v2241=(v572).sinh();
        let v2242=(self.scalar_static_f64[79]*v2241);
        let v2243=(self.scalar_static_f64[149]*v2241);
        let v2244=(if self.scalar_static_bool[42]{v2242}else{v13});
        let v2245=(if self.scalar_static_bool[42]{v2243}else{v13});
        let v2248=(if self.scalar_static_bool[42]{(v2244/v574)}else{v13});
        let v2249=(if self.scalar_static_bool[42]{(v2245/v574)}else{v13});
        let v2250=(v525).sinh();
        let v2251=(self.scalar_static_f64[79]*v2250);
        let v2252=(self.scalar_static_f64[150]*v2250);
        let v2253=(self.scalar_static_f64[78]*v2250);
        let v2254=(if self.scalar_static_bool[42]{v2251}else{v13});
        let v2255=(if self.scalar_static_bool[42]{v2252}else{v13});
        let v2256=(if self.scalar_static_bool[42]{v2253}else{v13});
        let v2260=(if self.scalar_static_bool[42]{(v2254/v578)}else{v13});
        let v2261=(if self.scalar_static_bool[42]{(v2255/v578)}else{v13});
        let v2262=(if self.scalar_static_bool[42]{(v2256/v578)}else{v13});
        let v2265=(if self.scalar_static_bool[42]{(self.scalar_static_f64[79]+v2248)}else{v13});
        let v2266=(if self.scalar_static_bool[42]{(self.scalar_static_f64[149]+v2249)}else{v13});
        let v2293=(if self.scalar_static_bool[42]{(self.scalar_static_f64[85]+(v137*(self.scalar_static_f64[89]+((v571*(self.scalar_static_f64[78]+v2262))/self.scalar_static_f64[78]))))}else{v13});
        let v2294=(v593).sinh();
        let v2295=(self.scalar_static_f64[149]*v2294);
        let v2296=(self.scalar_static_f64[79]*v2294);
        let v2297=(if self.scalar_static_bool[42]{v2295}else{v2244});
        let v2298=(if self.scalar_static_bool[42]{v2296}else{v2245});
        let v2301=(if self.scalar_static_bool[42]{(v2297/v595)}else{v13});
        let v2302=(if self.scalar_static_bool[42]{(v2298/v595)}else{v13});
        let v2303=(v544).sinh();
        let v2304=(self.scalar_static_f64[154]*v2303);
        let v2305=(self.scalar_static_f64[79]*v2303);
        let v2306=(self.scalar_static_f64[84]*v2303);
        let v2307=(if self.scalar_static_bool[42]{v2304}else{v2254});
        let v2308=(if self.scalar_static_bool[42]{v2305}else{v2255});
        let v2309=(if self.scalar_static_bool[42]{v2306}else{v13});
        let v2310=(if self.scalar_static_bool[42]{v13}else{v2256});
        let v2315=(if self.scalar_static_bool[42]{(v2307/v599)}else{v13});
        let v2316=(if self.scalar_static_bool[42]{(v2308/v599)}else{v13});
        let v2317=(if self.scalar_static_bool[42]{(v2309/v599)}else{v13});
        let v2318=(if self.scalar_static_bool[42]{(v2310/v599)}else{v13});
        let v2321=(if self.scalar_static_bool[42]{(self.scalar_static_f64[149]+v2301)}else{v13});
        let v2322=(if self.scalar_static_bool[42]{(self.scalar_static_f64[79]+v2302)}else{v13});
        let v2351=(if self.scalar_static_bool[42]{(self.scalar_static_f64[87]+(v138*(self.scalar_static_f64[89]+((v540*(self.scalar_static_f64[84]+v2317))/self.scalar_static_f64[84]))))}else{v13});
        let v2363=(v624*self.scalar_static_f64[160]);
        let v2364=(v2363+v2363);
        let v2365=(v624*self.scalar_static_f64[161]);
        let v2366=(v2365+v2365);
        let v2369=(self.scalar_static_f64[93]*f64::powf(v628,self.scalar_static_f64[162]));
        let v2386=(v44-(v640*v640));
        let v2398=(v44-(v651*v651));
        let v2435=(if self.scalar_static_bool[48]{v2242}else{v2297});
        let v2436=(if self.scalar_static_bool[48]{v2243}else{v2298});
        let v2441=(if self.scalar_static_bool[48]{v2251}else{v2307});
        let v2442=(if self.scalar_static_bool[48]{v2252}else{v2308});
        let v2443=(if self.scalar_static_bool[48]{v13}else{v2309});
        let v2444=(if self.scalar_static_bool[48]{v2253}else{v2310});
        let v2455=(v222*f64::powf(v678,v44));
        let v2460=(self.scalar_static_f64[99]*f64::powf(v680,self.scalar_static_f64[171]));
        let v2502=(if self.scalar_static_bool[48]{(v137*(((v696*((self.scalar_static_f64[79]+(if self.scalar_static_bool[48]{(v2441/v672)}else{v2260}))-(if self.scalar_static_bool[48]{(self.scalar_static_f64[79]+(if self.scalar_static_bool[48]{(v2435/v669)}else{v2248}))}else{v2265})))+(v695*v2201))/self.scalar_static_f64[78]))}else{(if self.scalar_static_bool[42]{(v137*(((v584*v2201)+(v571*((self.scalar_static_f64[79]+v2260)-v2265)))/self.scalar_static_f64[78]))}else{v13})});
        let v2503=(if self.scalar_static_bool[48]{(self.scalar_static_f64[156]+(v137*(self.scalar_static_f64[155]+(((v696*((if self.scalar_static_bool[48]{((v682*self.scalar_static_f64[170])+(v677*((self.scalar_static_f64[158]*v2455)*v2460)))}else{v13})+((self.scalar_static_f64[150]+(if self.scalar_static_bool[48]{(v2442/v672)}else{v2261}))-(if self.scalar_static_bool[48]{(self.scalar_static_f64[149]+(if self.scalar_static_bool[48]{(v2436/v669)}else{v2249}))}else{v2266}))))+(v695*v2202))/self.scalar_static_f64[78]))))}else{(if self.scalar_static_bool[42]{((v137*((((v584*v2202)+(v571*((self.scalar_static_f64[150]+v2261)-v2266)))/self.scalar_static_f64[78])+self.scalar_static_f64[155]))+self.scalar_static_f64[156])}else{v13})});
        let v2504=(if self.scalar_static_bool[48]{(v137*((v696*(if self.scalar_static_bool[48]{(v2443/v672)}else{v13}))/self.scalar_static_f64[78]))}else{v13});
        let v2505=(if self.scalar_static_bool[48]{(self.scalar_static_f64[85]+(v137*(self.scalar_static_f64[89]+((v696*((if self.scalar_static_bool[48]{((self.scalar_static_f64[97]*v682)+(v677*((self.scalar_static_f64[159]*v2455)*v2460)))}else{v13})+(self.scalar_static_f64[78]+(if self.scalar_static_bool[48]{(v2444/v672)}else{v2262}))))/self.scalar_static_f64[78]))))}else{v2293});
        let v2553=(if self.scalar_static_bool[48]{(self.scalar_static_f64[157]+(v138*(self.scalar_static_f64[155]+(((v712*v2206)+(v645*((self.scalar_static_f64[154]+(if self.scalar_static_bool[48]{((if self.scalar_static_bool[48]{v2304}else{v2441})/v706)}else{v2315}))-(if self.scalar_static_bool[48]{(self.scalar_static_f64[149]+(if self.scalar_static_bool[48]{((if self.scalar_static_bool[48]{v2295}else{v2435})/v703)}else{v2301}))}else{v2321}))))/self.scalar_static_f64[84]))))}else{(if self.scalar_static_bool[42]{((v138*(self.scalar_static_f64[155]+(((v605*v2206)+(v540*((self.scalar_static_f64[154]+v2315)-v2321)))/self.scalar_static_f64[84])))+self.scalar_static_f64[157])}else{v13})});
        let v2554=(if self.scalar_static_bool[48]{(v138*(((v712*v2207)+(v645*((self.scalar_static_f64[79]+(if self.scalar_static_bool[48]{((if self.scalar_static_bool[48]{v2305}else{v2442})/v706)}else{v2316}))-(if self.scalar_static_bool[48]{(self.scalar_static_f64[79]+(if self.scalar_static_bool[48]{((if self.scalar_static_bool[48]{v2296}else{v2436})/v703)}else{v2302}))}else{v2322}))))/self.scalar_static_f64[84]))}else{(if self.scalar_static_bool[42]{(v138*(((v605*v2207)+(v540*((self.scalar_static_f64[79]+v2316)-v2322)))/self.scalar_static_f64[84]))}else{v13})});
        let v2555=(if self.scalar_static_bool[48]{(self.scalar_static_f64[87]+(v138*(self.scalar_static_f64[89]+((v645*(self.scalar_static_f64[84]+(if self.scalar_static_bool[48]{((if self.scalar_static_bool[48]{v2306}else{v2443})/v706)}else{v2317})))/self.scalar_static_f64[84]))))}else{v2351});
        let v2556=(if self.scalar_static_bool[48]{(v138*((v645*(if self.scalar_static_bool[48]{((if self.scalar_static_bool[48]{v13}else{v2444})/v706)}else{v2318}))/self.scalar_static_f64[84]))}else{(if self.scalar_static_bool[42]{(v138*((v540*v2318)/self.scalar_static_f64[84]))}else{v13})});
        let v2586=(v5*(if self.scalar_static_bool[48]{v13}else{(if self.scalar_static_bool[45]{(v138*((v653*v2206)+(v646*(if self.scalar_static_bool[45]{(self.scalar_static_f64[168]*v2398)}else{v2212}))))}else{(if self.scalar_static_bool[42]{v13}else{(if self.scalar_static_bool[39]{(v138*((v546*v2206)+(v540*v2212)))}else{v13})})})}));
        let v2588=(v5*(if self.scalar_static_bool[48]{v13}else{(if self.scalar_static_bool[45]{(v138*((v653*v2207)+(v646*(if self.scalar_static_bool[45]{(self.scalar_static_f64[169]*v2398)}else{v2213}))))}else{(if self.scalar_static_bool[42]{v13}else{(if self.scalar_static_bool[39]{(v138*((v546*v2207)+(v540*v2213)))}else{v13})})})}));
        let v2589=(v5*(if self.scalar_static_bool[48]{v13}else{(if self.scalar_static_bool[45]{(v138*(v646*(if self.scalar_static_bool[45]{(self.scalar_static_f64[84]*v2398)}else{v2214})))}else{(if self.scalar_static_bool[42]{v13}else{(if self.scalar_static_bool[39]{(v138*(v540*v2214))}else{v13})})})}));
        let v2597=(v9*(if self.scalar_static_bool[48]{v13}else{(if self.scalar_static_bool[45]{((v657*v2201)+(v643*(v137*(if self.scalar_static_bool[45]{(self.scalar_static_f64[164]*v2386)}else{v2195}))))}else{(if self.scalar_static_bool[42]{v13}else{(if self.scalar_static_bool[39]{((v557*v2201)+(v533*(v137*v2195)))}else{v13})})})}));
        let v2599=(v9*(if self.scalar_static_bool[48]{v13}else{(if self.scalar_static_bool[45]{((v657*v2202)+(v643*(v137*((if self.scalar_static_bool[45]{(self.scalar_static_f64[165]*v2386)}else{v2196})+(self.scalar_static_f64[97]*(if self.scalar_static_bool[45]{((v634*(v2364*v2369))+(v630*(self.scalar_static_f64[95]*v2364)))}else{v13}))))))}else{(if self.scalar_static_bool[42]{v13}else{(if self.scalar_static_bool[39]{((v557*v2202)+(v533*(v137*v2196)))}else{v13})})})}));
        let v2601=(v9*(if self.scalar_static_bool[48]{v13}else{(if self.scalar_static_bool[45]{(v643*(v137*((if self.scalar_static_bool[45]{(self.scalar_static_f64[78]*v2386)}else{v2197})+(self.scalar_static_f64[97]*(if self.scalar_static_bool[45]{((v634*(v2366*v2369))+(v630*(self.scalar_static_f64[95]*v2366)))}else{v13})))))}else{(if self.scalar_static_bool[42]{v13}else{(if self.scalar_static_bool[39]{(v533*(v137*v2197))}else{v13})})})}));
        let v2611=(-v140);
        let v2620=(if self.scalar_static_bool[50]{(v798*(if self.scalar_static_bool[27]{((-(v139*v1697))/v2085)}else{(if self.scalar_static_bool[26]{((-(v139*v970))/v2054)}else{v13})}))}else{v13});
        let v2621=(if self.scalar_static_bool[50]{(v798*(if self.scalar_static_bool[27]{((-(v139*v1698))/v2085)}else{(if self.scalar_static_bool[26]{((-(v139*v971))/v2054)}else{v13})}))}else{v13});
        let v2622=(if self.scalar_static_bool[50]{(v798*(if self.scalar_static_bool[27]{((-(v139*v1699))/v2085)}else{(if self.scalar_static_bool[26]{((-(v139*v972))/v2054)}else{v13})}))}else{v13});
        let v2623=(if self.scalar_static_bool[50]{(v798*(if self.scalar_static_bool[27]{((-(v139*v1700))/v2085)}else{(if self.scalar_static_bool[26]{((-(v139*v973))/v2054)}else{v13})}))}else{v13});
        let v2624=(if self.scalar_static_bool[50]{(v798*(if self.scalar_static_bool[27]{((-(v139*v1701))/v2085)}else{(if self.scalar_static_bool[26]{((-(v139*v974))/v2054)}else{v13})}))}else{v13});
        let v2630=(-v142);

        CommonStampValues {
            v0,
            v1,
            v3,
            v5,
            v6,
            v7,
            v8,
            v9,
            v11,
            v13,
            v32,
            v35,
            v39,
            v43,
            v44,
            v46,
            v83,
            v118,
            v122,
            v129,
            v134,
            v140,
            v142,
            v147,
            v152,
            v167,
            v176,
            v181,
            v191,
            v199,
            v208,
            v238,
            v239,
            v241,
            v308,
            v331,
            v358,
            v443,
            v456,
            v560,
            v566,
            v659,
            v664,
            v702,
            v718,
            v758,
            v764,
            v773,
            v776,
            v791,
            v793,
            v795,
            v796,
            v799,
            v800,
            v807,
            v811,
            v813,
            v829,
            v835,
            v841,
            v846,
            v854,
            v855,
            v868,
            v892,
            v893,
            v913,
            v918,
            v920,
            v922,
            v924,
            v925,
            v970,
            v971,
            v972,
            v973,
            v974,
            v1071,
            v1072,
            v1073,
            v1074,
            v1075,
            v1076,
            v1077,
            v1086,
            v1087,
            v1088,
            v1089,
            v1425,
            v1426,
            v1427,
            v1428,
            v1429,
            v1525,
            v1526,
            v1527,
            v1528,
            v1697,
            v1698,
            v1699,
            v1700,
            v1701,
            v2293,
            v2351,
            v2502,
            v2503,
            v2504,
            v2505,
            v2553,
            v2554,
            v2555,
            v2556,
            v2586,
            v2588,
            v2589,
            v2597,
            v2599,
            v2601,
            v2611,
            v2620,
            v2621,
            v2622,
            v2623,
            v2624,
            v2630,
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
        let common=self.eval_common_stamp_values(ctx);
        let v12=ctx.node_voltage(nodes[16]);
        let v49=(self.scalar_static_f64[11]*(common.v44+(self.scalar_static_f64[12]*common.v46)));
        let v135=(if common.v134{self.scalar_static_f64[13]}else{(if common.v43{(self.scalar_static_f64[13]*(common.v44+(common.v46*self.scalar_static_f64[14])))}else{common.v13})});
        let v136=(if common.v134{self.scalar_static_f64[15]}else{(if common.v43{(self.scalar_static_f64[15]*(common.v44+(common.v46*self.scalar_static_f64[16])))}else{common.v13})});
        let v141=(if common.v134{self.scalar_static_f64[36]}else{(if common.v129{(common.v83*self.scalar_static_f64[36])}else{(if common.v118{(self.scalar_static_f64[36]*common.v122)}else{common.v13})})});
        let v146=(if common.v134{self.scalar_static_f64[30]}else{(if common.v43{(self.scalar_static_f64[30]+(common.v39*self.scalar_static_f64[31]))}else{common.v13})});
        let v159=(if self.scalar_static_bool[11]{self.scalar_static_f64[42]}else{(if self.scalar_static_bool[10]{(self.scalar_static_f64[41]/(common.v35*8.617333262145179e-5))}else{common.v13})});
        let v218=(self.scalar_static_f64[54]+(self.scalar_static_f64[51]*common.v208));
        let v220=((common.v7*v218)).tanh();
        let v226=(v135*common.v208);
        let v227=(v220*v226);
        let v233=((common.v44+(common.v7*self.scalar_static_f64[55]))+(v136*scalar_limexp(common.v191)));
        let v243=(if self.scalar_static_bool[16]{(common.v239*common.v241)}else{common.v199});
        let v249=(if self.scalar_static_bool[16]{(((common.v176*common.v239)+(self.scalar_static_f64[53]*common.v241))+(common.v181*v243))}else{common.v13});
        let v250=(v249).tanh();
        let v252=(if self.scalar_static_bool[16]{(common.v44+v250)}else{common.v13});
        let v255=(if self.scalar_static_bool[16]{(self.scalar_static_f64[54]+(self.scalar_static_f64[51]*v252))}else{common.v13});
        let v258=(self.scalar_static_f64[55]+(common.v208*self.scalar_static_f64[56]));
        let v259=(if self.scalar_static_bool[16]{v258}else{common.v13});
        let v260=(common.v44+v220);
        let v261=(v226*v260);
        let v266=(self.scalar_static_f64[57]*(common.v7-common.v147));
        let v268=(v136*scalar_limexp(v266));
        let v269=((common.v44+(common.v7*v259))+v268);
        let v271=(if self.scalar_static_bool[16]{(v261*v269)}else{common.v13});
        let v274=(if self.scalar_static_bool[16]{(self.scalar_static_f64[55]+(v252*self.scalar_static_f64[56]))}else{common.v13});
        let v276=((common.v7*v255)).tanh();
        let v278=(v135*v252);
        let v279=(common.v44-(if self.scalar_static_bool[16]{v276}else{common.v13}));
        let v280=(v278*v279);
        let v282=(common.v44-(common.v7*v274));
        let v284=(if self.scalar_static_bool[16]{(v280*v282)}else{common.v13});
        let v311=(if self.scalar_static_bool[19]{(self.scalar_static_f64[54]+(self.scalar_static_f64[51]*common.v308))}else{common.v13});
        let v313=((common.v7*v311)).tanh();
        let v314=(if self.scalar_static_bool[19]{v313}else{common.v13});
        let v317=(if self.scalar_static_bool[19]{(self.scalar_static_f64[55]+(self.scalar_static_f64[56]*common.v308))}else{v259});
        let v318=(v135*common.v308);
        let v319=(v314*v318);
        let v322=(common.v191*self.scalar_static_f64[57]);
        let v325=((common.v44+(common.v7*v317))+(v136*scalar_limexp(v322)));
        let v341=(if self.scalar_static_bool[22]{common.v238}else{v243});
        let v343=(if self.scalar_static_bool[22]{(v341*v341)}else{common.v13});
        let v346=(common.v181*v341);
        let v348=((v341+(self.scalar_static_f64[53]*v343))+(v343*v346));
        let v350=(if self.scalar_static_bool[22]{(common.v176*v348)}else{v249});
        let v360=(-v350);
        let v364=((common.v152*(scalar_limexp(v350)-scalar_limexp(v360)))).tanh();
        let v366=(if self.scalar_static_bool[22]{(common.v44+v364)}else{common.v13});
        let v368=(self.scalar_static_f64[54]+(self.scalar_static_f64[51]*common.v358));
        let v369=(if self.scalar_static_bool[22]{v368}else{v311});
        let v372=(if self.scalar_static_bool[22]{(self.scalar_static_f64[54]+(self.scalar_static_f64[51]*v366))}else{common.v13});
        let v374=((common.v7*v369)).tanh();
        let v375=(if self.scalar_static_bool[22]{v374}else{v314});
        let v377=((common.v7*v372)).tanh();
        let v381=(if self.scalar_static_bool[22]{(self.scalar_static_f64[55]+(self.scalar_static_f64[56]*v366))}else{common.v13});
        let v384=(if self.scalar_static_bool[22]{(self.scalar_static_f64[55]+(self.scalar_static_f64[56]*common.v358))}else{common.v13});
        let v385=(v135*common.v358);
        let v386=(common.v44+v375);
        let v387=(v385*v386);
        let v390=(v268+(common.v44+(common.v7*v384)));
        let v393=(v135*v366);
        let v394=(common.v44-(if self.scalar_static_bool[22]{v377}else{common.v13}));
        let v395=(v393*v394);
        let v397=(common.v44-(common.v7*v381));
        let v406=(if self.scalar_static_bool[25]{v258}else{v317});
        let v407=(if self.scalar_static_bool[25]{v368}else{v369});
        let v409=((common.v7*v407)).tanh();
        let v412=((common.v11*v407)).tanh();
        let v416=((if self.scalar_static_bool[25]{v409}else{v375})+((if self.scalar_static_bool[25]{v412}else{common.v13})*self.scalar_static_f64[58]));
        let v417=(v226*v416);
        let v419=(common.v7+(common.v11*self.scalar_static_f64[58]));
        let v422=(v268+(common.v44+(v406*v419)));
        let v424=(if self.scalar_static_bool[25]{(v417*v422)}else{(if self.scalar_static_bool[22]{(common.v152*((if self.scalar_static_bool[22]{(v387*v390)}else{v271})-(if self.scalar_static_bool[22]{(v395*v397)}else{v284})))}else{(if self.scalar_static_bool[19]{(v319*v325)}else{(if self.scalar_static_bool[16]{(common.v152*(v271-v284))}else{(if self.scalar_static_bool[12]{(v227*v233)}else{common.v13})})})})});
        let v433=(common.v208*self.scalar_static_f64[61]);
        let v444=(common.v358*self.scalar_static_f64[61]);
        let v451=(common.v44+(common.v46*self.scalar_static_f64[63]));
        let v452=((if self.scalar_static_bool[27]{(self.scalar_static_f64[62]+v444)}else{(if self.scalar_static_bool[26]{(v433+self.scalar_static_f64[62])}else{common.v13})})*v451);
        let v453=((if self.scalar_static_bool[27]{(self.scalar_static_f64[60]+v444)}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[60]+v433)}else{common.v13})})*v451);
        let v462=(common.v9-v146);
        let v466=((-common.v9)-self.scalar_static_f64[65]);
        let v468=(common.v5-v146);
        let v471=(common.v6-self.scalar_static_f64[66]);
        let v477=(if self.scalar_static_bool[29]{scalar_limexp((v146*(-v159)))}else{(if self.scalar_static_bool[28]{scalar_limexp((v159*((-v146)).tanh()))}else{common.v331})});
        let v488=(v462).tanh();
        let v490=(v468).tanh();
        let v498=(self.scalar_static_f64[67]*(if self.scalar_static_bool[29]{v466}else{(if self.scalar_static_bool[28]{v466}else{common.v13})}));
        let v502=(v159*(if self.scalar_static_bool[33]{v462}else{(if self.scalar_static_bool[31]{v488}else{(if self.scalar_static_bool[28]{v462}else{common.v13})})}));
        let v510=(self.scalar_static_f64[75]*((scalar_limexp(v502)-((scalar_limexp(v498)-self.scalar_static_f64[71])*self.scalar_static_f64[77]))-v477));
        let v511=(self.scalar_static_f64[67]*(if self.scalar_static_bool[29]{v471}else{(if self.scalar_static_bool[28]{v471}else{common.v13})}));
        let v514=(v159*(if self.scalar_static_bool[33]{v468}else{(if self.scalar_static_bool[31]{v490}else{(if self.scalar_static_bool[28]{v468}else{common.v13})})}));
        let v614=common.v2293;
        let v616=common.v2351;
        let v719=common.v2505;
        let v720=(if self.scalar_static_bool[48]{v719}else{(if self.scalar_static_bool[45]{common.v659}else{(if self.scalar_static_bool[42]{v614}else{common.v560})})});
        let v721=common.v2555;
        let v722=(if self.scalar_static_bool[48]{v721}else{(if self.scalar_static_bool[45]{common.v664}else{(if self.scalar_static_bool[42]{v616}else{common.v566})})});
        let v762=(if self.scalar_static_bool[66]{((common.v44-(common.v758*common.v758))).sqrt()}else{common.v13});
        let v766=(if self.scalar_static_bool[66]{((-common.v758)*common.v764)}else{common.v13});
        let v777=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v718);
        let v779=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v702);
        let v783=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (common.v5*v722));
        let v786=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (common.v9*v720));
        let v801=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v800);
        let v808=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, common.v807);
        let v817=ctx.node_voltage(nodes[13]);
        let v830=ctx.branch_current(branches[11]);
        let v836=ctx.branch_current(branches[15]);
        let v847=(if self.scalar_static_bool[66]{common.v846}else{common.v13});
        let v848=ctx.node_voltage(nodes[18]);
        let v856=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, common.v855);
        let v869=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, common.v868);
        let v1023=(common.v44-(v220*v220));
        let v1029=(v135*common.v970);
        let v1030=(v135*common.v971);
        let v1031=(v135*common.v972);
        let v1032=(v135*common.v973);
        let v1033=(v135*common.v974);
        let v1034=(v226*((common.v7*(self.scalar_static_f64[51]*common.v970))*v1023));
        let v1037=(v226*((v218+(common.v7*(self.scalar_static_f64[51]*common.v971)))*v1023));
        let v1040=(v226*(((-v218)+(common.v7*(self.scalar_static_f64[51]*common.v972)))*v1023));
        let v1043=(v226*((common.v7*(self.scalar_static_f64[51]*common.v973))*v1023));
        let v1046=(v226*((common.v7*(self.scalar_static_f64[51]*common.v974))*v1023));
        let v1050=scalar_limexp_derivative(common.v191);
        let v1104=(if self.scalar_static_bool[16]{((common.v241*common.v1074)+(common.v239*common.v1086))}else{common.v918});
        let v1105=(if self.scalar_static_bool[16]{((common.v241*common.v1075)+(common.v239*common.v1087))}else{common.v920});
        let v1106=(if self.scalar_static_bool[16]{((common.v241*common.v1076)+(common.v239*common.v1088))}else{common.v922});
        let v1107=(if self.scalar_static_bool[16]{((common.v241*common.v1077)+(common.v239*common.v1089))}else{common.v924});
        let v1108=(if self.scalar_static_bool[16]{(common.v239*self.scalar_static_f64[131])}else{common.v925});
        let v1136=(if self.scalar_static_bool[16]{(((common.v176*common.v1074)+(self.scalar_static_f64[53]*common.v1086))+(common.v181*v1104))}else{common.v13});
        let v1137=(if self.scalar_static_bool[16]{((((common.v239*common.v892)+(common.v176*common.v1075))+(self.scalar_static_f64[53]*common.v1087))+(common.v181*v1105))}else{common.v13});
        let v1138=(if self.scalar_static_bool[16]{((((common.v239*common.v893)+(common.v176*common.v1076))+(self.scalar_static_f64[53]*common.v1088))+(common.v181*v1106))}else{common.v13});
        let v1139=(if self.scalar_static_bool[16]{(((common.v176*common.v1077)+(self.scalar_static_f64[53]*common.v1089))+(common.v181*v1107))}else{common.v13});
        let v1140=(if self.scalar_static_bool[16]{(self.scalar_static_f64[132]+(common.v181*v1108))}else{common.v13});
        let v1142=(common.v44-(v250*v250));
        let v1148=(if self.scalar_static_bool[16]{(v1136*v1142)}else{common.v13});
        let v1149=(if self.scalar_static_bool[16]{(v1137*v1142)}else{common.v13});
        let v1150=(if self.scalar_static_bool[16]{(v1138*v1142)}else{common.v13});
        let v1151=(if self.scalar_static_bool[16]{(v1139*v1142)}else{common.v13});
        let v1152=(if self.scalar_static_bool[16]{(v1140*v1142)}else{common.v13});
        let v1163=(self.scalar_static_f64[56]*common.v970);
        let v1164=(self.scalar_static_f64[56]*common.v971);
        let v1165=(self.scalar_static_f64[56]*common.v972);
        let v1166=(self.scalar_static_f64[56]*common.v973);
        let v1167=(self.scalar_static_f64[56]*common.v974);
        let v1168=(if self.scalar_static_bool[16]{v1163}else{common.v13});
        let v1169=(if self.scalar_static_bool[16]{v1164}else{common.v13});
        let v1170=(if self.scalar_static_bool[16]{v1165}else{common.v13});
        let v1171=(if self.scalar_static_bool[16]{v1166}else{common.v13});
        let v1172=(if self.scalar_static_bool[16]{v1167}else{common.v13});
        let v1192=scalar_limexp_derivative(v266);
        let v1195=(v136*(self.scalar_static_f64[57]*v1192));
        let v1196=(v136*(self.scalar_static_f64[133]*v1192));
        let v1214=(if self.scalar_static_bool[16]{((v269*(v1034+(v260*v1029)))+(v261*(common.v7*v1168)))}else{common.v13});
        let v1215=(if self.scalar_static_bool[16]{((v269*(v1037+(v260*v1030)))+(v261*((v259+(common.v7*v1169))+v1195)))}else{common.v13});
        let v1216=(if self.scalar_static_bool[16]{((v269*(v1040+(v260*v1031)))+(v261*(((-v259)+(common.v7*v1170))+v1196)))}else{common.v13});
        let v1217=(if self.scalar_static_bool[16]{((v269*(v1043+(v260*v1032)))+(v261*(common.v7*v1171)))}else{common.v13});
        let v1218=(if self.scalar_static_bool[16]{((v269*(v1046+(v260*v1033)))+(v261*(common.v7*v1172)))}else{common.v13});
        let v1238=(common.v44-(v276*v276));
        let v1302=(if self.scalar_static_bool[16]{((v282*((v279*(v135*v1148))+(v278*(-(if self.scalar_static_bool[16]{((common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[51]*v1148)}else{common.v13}))*v1238)}else{common.v13})))))+(v280*(-(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[56]*v1148)}else{common.v13})))))}else{common.v13});
        let v1303=(if self.scalar_static_bool[16]{((v282*((v279*(v135*v1149))+(v278*(-(if self.scalar_static_bool[16]{((v255+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[51]*v1149)}else{common.v13})))*v1238)}else{common.v13})))))+(v280*(-(v274+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[56]*v1149)}else{common.v13}))))))}else{common.v13});
        let v1304=(if self.scalar_static_bool[16]{((v282*((v279*(v135*v1150))+(v278*(-(if self.scalar_static_bool[16]{(((-v255)+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[51]*v1150)}else{common.v13})))*v1238)}else{common.v13})))))+(v280*(-((-v274)+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[56]*v1150)}else{common.v13}))))))}else{common.v13});
        let v1305=(if self.scalar_static_bool[16]{((v282*((v279*(v135*v1151))+(v278*(-(if self.scalar_static_bool[16]{((common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[51]*v1151)}else{common.v13}))*v1238)}else{common.v13})))))+(v280*(-(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[56]*v1151)}else{common.v13})))))}else{common.v13});
        let v1306=(if self.scalar_static_bool[16]{((v282*((v279*(v135*v1152))+(v278*(-(if self.scalar_static_bool[16]{((common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[51]*v1152)}else{common.v13}))*v1238)}else{common.v13})))))+(v280*(-(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[56]*v1152)}else{common.v13})))))}else{common.v13});
        let v1435=(if self.scalar_static_bool[19]{(self.scalar_static_f64[51]*common.v1425)}else{common.v13});
        let v1436=(if self.scalar_static_bool[19]{(self.scalar_static_f64[51]*common.v1426)}else{common.v13});
        let v1437=(if self.scalar_static_bool[19]{(self.scalar_static_f64[51]*common.v1427)}else{common.v13});
        let v1438=(if self.scalar_static_bool[19]{(self.scalar_static_f64[51]*common.v1428)}else{common.v13});
        let v1439=(if self.scalar_static_bool[19]{(self.scalar_static_f64[51]*common.v1429)}else{common.v13});
        let v1449=(common.v44-(v313*v313));
        let v1455=(if self.scalar_static_bool[19]{((common.v7*v1435)*v1449)}else{common.v13});
        let v1456=(if self.scalar_static_bool[19]{((v311+(common.v7*v1436))*v1449)}else{common.v13});
        let v1457=(if self.scalar_static_bool[19]{(((-v311)+(common.v7*v1437))*v1449)}else{common.v13});
        let v1458=(if self.scalar_static_bool[19]{((common.v7*v1438)*v1449)}else{common.v13});
        let v1459=(if self.scalar_static_bool[19]{((common.v7*v1439)*v1449)}else{common.v13});
        let v1465=(if self.scalar_static_bool[19]{(self.scalar_static_f64[56]*common.v1425)}else{v1168});
        let v1466=(if self.scalar_static_bool[19]{(self.scalar_static_f64[56]*common.v1426)}else{v1169});
        let v1467=(if self.scalar_static_bool[19]{(self.scalar_static_f64[56]*common.v1427)}else{v1170});
        let v1468=(if self.scalar_static_bool[19]{(self.scalar_static_f64[56]*common.v1428)}else{v1171});
        let v1469=(if self.scalar_static_bool[19]{(self.scalar_static_f64[56]*common.v1429)}else{v1172});
        let v1498=scalar_limexp_derivative(v322);
        let v1594=(if self.scalar_static_bool[22]{common.v913}else{v1104});
        let v1595=(if self.scalar_static_bool[22]{common.v1071}else{v1105});
        let v1596=(if self.scalar_static_bool[22]{common.v1072}else{v1106});
        let v1597=(if self.scalar_static_bool[22]{common.v1073}else{v1107});
        let v1598=(if self.scalar_static_bool[22]{common.v13}else{v1108});
        let v1599=(v341*v1594);
        let v1601=(v341*v1595);
        let v1603=(v341*v1596);
        let v1605=(v341*v1597);
        let v1607=(v341*v1598);
        let v1609=(if self.scalar_static_bool[22]{(v1599+v1599)}else{common.v13});
        let v1610=(if self.scalar_static_bool[22]{(v1601+v1601)}else{common.v13});
        let v1611=(if self.scalar_static_bool[22]{(v1603+v1603)}else{common.v13});
        let v1612=(if self.scalar_static_bool[22]{(v1605+v1605)}else{common.v13});
        let v1613=(if self.scalar_static_bool[22]{(v1607+v1607)}else{common.v13});
        let v1658=(if self.scalar_static_bool[22]{(common.v176*((v1594+(self.scalar_static_f64[53]*v1609))+((v346*v1609)+(v343*(common.v181*v1594)))))}else{v1136});
        let v1659=(if self.scalar_static_bool[22]{((v348*common.v892)+(common.v176*((v1595+(self.scalar_static_f64[53]*v1610))+((v346*v1610)+(v343*(common.v181*v1595))))))}else{v1137});
        let v1660=(if self.scalar_static_bool[22]{((v348*common.v893)+(common.v176*((v1596+(self.scalar_static_f64[53]*v1611))+((v346*v1611)+(v343*(common.v181*v1596))))))}else{v1138});
        let v1661=(if self.scalar_static_bool[22]{(common.v176*((v1597+(self.scalar_static_f64[53]*v1612))+((v346*v1612)+(v343*(common.v181*v1597)))))}else{v1139});
        let v1662=(if self.scalar_static_bool[22]{(common.v176*((v1598+(self.scalar_static_f64[53]*v1613))+((v346*v1613)+(v343*(common.v181*v1598)))))}else{v1140});
        let v1702=scalar_limexp_derivative(v350);
        let v1713=scalar_limexp_derivative(v360);
        let v1730=(common.v44-(v364*v364));
        let v1736=(if self.scalar_static_bool[22]{((common.v152*((v1658*v1702)-((-v1658)*v1713)))*v1730)}else{common.v13});
        let v1737=(if self.scalar_static_bool[22]{((common.v152*((v1659*v1702)-((-v1659)*v1713)))*v1730)}else{common.v13});
        let v1738=(if self.scalar_static_bool[22]{((common.v152*((v1660*v1702)-((-v1660)*v1713)))*v1730)}else{common.v13});
        let v1739=(if self.scalar_static_bool[22]{((common.v152*((v1661*v1702)-((-v1661)*v1713)))*v1730)}else{common.v13});
        let v1740=(if self.scalar_static_bool[22]{((common.v152*((v1662*v1702)-((-v1662)*v1713)))*v1730)}else{common.v13});
        let v1741=(self.scalar_static_f64[51]*common.v1697);
        let v1742=(self.scalar_static_f64[51]*common.v1698);
        let v1743=(self.scalar_static_f64[51]*common.v1699);
        let v1744=(self.scalar_static_f64[51]*common.v1700);
        let v1745=(self.scalar_static_f64[51]*common.v1701);
        let v1746=(if self.scalar_static_bool[22]{v1741}else{v1435});
        let v1747=(if self.scalar_static_bool[22]{v1742}else{v1436});
        let v1748=(if self.scalar_static_bool[22]{v1743}else{v1437});
        let v1749=(if self.scalar_static_bool[22]{v1744}else{v1438});
        let v1750=(if self.scalar_static_bool[22]{v1745}else{v1439});
        let v1770=(common.v44-(v374*v374));
        let v1776=(if self.scalar_static_bool[22]{((common.v7*v1746)*v1770)}else{v1455});
        let v1777=(if self.scalar_static_bool[22]{((v369+(common.v7*v1747))*v1770)}else{v1456});
        let v1778=(if self.scalar_static_bool[22]{(((-v369)+(common.v7*v1748))*v1770)}else{v1457});
        let v1779=(if self.scalar_static_bool[22]{((common.v7*v1749)*v1770)}else{v1458});
        let v1780=(if self.scalar_static_bool[22]{((common.v7*v1750)*v1770)}else{v1459});
        let v1790=(common.v44-(v377*v377));
        let v1940=(if self.scalar_static_bool[22]{(common.v152*((if self.scalar_static_bool[22]{((v390*((v386*(v135*common.v1698))+(v385*v1777)))+(v387*(v1195+(v384+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*common.v1698)}else{common.v13}))))))}else{v1215})-(if self.scalar_static_bool[22]{((v397*((v394*(v135*v1737))+(v393*(-(if self.scalar_static_bool[22]{((v372+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[51]*v1737)}else{common.v13})))*v1790)}else{common.v13})))))+(v395*(-(v381+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*v1737)}else{common.v13}))))))}else{v1303})))}else{(if self.scalar_static_bool[19]{((v325*((v318*v1456)+(v314*(v135*common.v1426))))+(v319*((v317+(common.v7*v1466))+(v136*(self.scalar_static_f64[57]*v1498)))))}else{(if self.scalar_static_bool[16]{(common.v152*(v1215-v1303))}else{(if self.scalar_static_bool[12]{((v233*(v1037+(v220*v1030)))+(v227*(self.scalar_static_f64[55]+(v136*v1050))))}else{common.v13})})})});
        let v1949=(if self.scalar_static_bool[25]{v1741}else{v1746});
        let v1950=(if self.scalar_static_bool[25]{v1742}else{v1747});
        let v1951=(if self.scalar_static_bool[25]{v1743}else{v1748});
        let v1952=(if self.scalar_static_bool[25]{v1744}else{v1749});
        let v1953=(if self.scalar_static_bool[25]{v1745}else{v1750});
        let v1957=(-v407);
        let v1963=(common.v44-(v409*v409));
        let v1982=(common.v44-(v412*v412));
        let v2047=(if self.scalar_static_bool[25]{((v422*((v416*v1029)+(v226*((if self.scalar_static_bool[25]{((common.v7*v1949)*v1963)}else{v1776})+(self.scalar_static_f64[58]*(if self.scalar_static_bool[25]{((v407+(common.v11*v1949))*v1982)}else{common.v13}))))))+(v417*((v419*(if self.scalar_static_bool[25]{v1163}else{v1465}))+(v406*self.scalar_static_f64[58]))))}else{(if self.scalar_static_bool[22]{(common.v152*((if self.scalar_static_bool[22]{((v390*((v386*(v135*common.v1697))+(v385*v1776)))+(v387*(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*common.v1697)}else{common.v13}))))}else{v1214})-(if self.scalar_static_bool[22]{((v397*((v394*(v135*v1736))+(v393*(-(if self.scalar_static_bool[22]{((common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[51]*v1736)}else{common.v13}))*v1790)}else{common.v13})))))+(v395*(-(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*v1736)}else{common.v13})))))}else{v1302})))}else{(if self.scalar_static_bool[19]{((v325*((v318*v1455)+(v314*(v135*common.v1425))))+(v319*(common.v7*v1465)))}else{(if self.scalar_static_bool[16]{(common.v152*(v1214-v1302))}else{(if self.scalar_static_bool[12]{(v233*(v1034+(v220*v1029)))}else{common.v13})})})})});
        let v2049=(if self.scalar_static_bool[25]{((v422*((v416*v1031)+(v226*((if self.scalar_static_bool[25]{((v1957+(common.v7*v1951))*v1963)}else{v1778})+(self.scalar_static_f64[58]*(if self.scalar_static_bool[25]{((v1957+(common.v11*v1951))*v1982)}else{common.v13}))))))+(v417*(v1196+((v419*(if self.scalar_static_bool[25]{v1165}else{v1467}))+(v406*self.scalar_static_f64[137])))))}else{(if self.scalar_static_bool[22]{(common.v152*((if self.scalar_static_bool[22]{((v390*((v386*(v135*common.v1699))+(v385*v1778)))+(v387*(v1196+((-v384)+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*common.v1699)}else{common.v13}))))))}else{v1216})-(if self.scalar_static_bool[22]{((v397*((v394*(v135*v1738))+(v393*(-(if self.scalar_static_bool[22]{(((-v372)+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[51]*v1738)}else{common.v13})))*v1790)}else{common.v13})))))+(v395*(-((-v381)+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*v1738)}else{common.v13}))))))}else{v1304})))}else{(if self.scalar_static_bool[19]{((v325*((v318*v1457)+(v314*(v135*common.v1427))))+(v319*((-v317)+(common.v7*v1467))))}else{(if self.scalar_static_bool[16]{(common.v152*(v1216-v1304))}else{(if self.scalar_static_bool[12]{((v233*(v1040+(v220*v1031)))+(v227*self.scalar_static_f64[130]))}else{common.v13})})})})});
        let v2050=(if self.scalar_static_bool[25]{((v422*((v416*v1032)+(v226*((if self.scalar_static_bool[25]{((common.v7*v1952)*v1963)}else{v1779})+(self.scalar_static_f64[58]*(if self.scalar_static_bool[25]{((common.v11*v1952)*v1982)}else{common.v13}))))))+(v417*(v419*(if self.scalar_static_bool[25]{v1166}else{v1468}))))}else{(if self.scalar_static_bool[22]{(common.v152*((if self.scalar_static_bool[22]{((v390*((v386*(v135*common.v1700))+(v385*v1779)))+(v387*(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*common.v1700)}else{common.v13}))))}else{v1217})-(if self.scalar_static_bool[22]{((v397*((v394*(v135*v1739))+(v393*(-(if self.scalar_static_bool[22]{((common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[51]*v1739)}else{common.v13}))*v1790)}else{common.v13})))))+(v395*(-(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*v1739)}else{common.v13})))))}else{v1305})))}else{(if self.scalar_static_bool[19]{((v325*((v318*v1458)+(v314*(v135*common.v1428))))+(v319*((common.v7*v1468)+(v136*(self.scalar_static_f64[133]*v1498)))))}else{(if self.scalar_static_bool[16]{(common.v152*(v1217-v1305))}else{(if self.scalar_static_bool[12]{((v233*(v1043+(v220*v1032)))+(v227*(v136*(-v1050))))}else{common.v13})})})})});
        let v2051=(if self.scalar_static_bool[25]{((v422*((v416*v1033)+(v226*((if self.scalar_static_bool[25]{((common.v7*v1953)*v1963)}else{v1780})+(self.scalar_static_f64[58]*(if self.scalar_static_bool[25]{((common.v11*v1953)*v1982)}else{common.v13}))))))+(v417*(v419*(if self.scalar_static_bool[25]{v1167}else{v1469}))))}else{(if self.scalar_static_bool[22]{(common.v152*((if self.scalar_static_bool[22]{((v390*((v386*(v135*common.v1701))+(v385*v1780)))+(v387*(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*common.v1701)}else{common.v13}))))}else{v1218})-(if self.scalar_static_bool[22]{((v397*((v394*(v135*v1740))+(v393*(-(if self.scalar_static_bool[22]{((common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[51]*v1740)}else{common.v13}))*v1790)}else{common.v13})))))+(v395*(-(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]*v1740)}else{common.v13})))))}else{v1306})))}else{(if self.scalar_static_bool[19]{((v325*((v318*v1459)+(v314*(v135*common.v1429))))+(v319*(common.v7*v1469)))}else{(if self.scalar_static_bool[16]{(common.v152*(v1218-v1306))}else{(if self.scalar_static_bool[12]{(v233*(v1046+(v220*v1033)))}else{common.v13})})})})});
        let v2114=(v451*(if self.scalar_static_bool[27]{(self.scalar_static_f64[61]*common.v1697)}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[61]*common.v970)}else{common.v13})}));
        let v2115=(v451*(if self.scalar_static_bool[27]{(self.scalar_static_f64[61]*common.v1698)}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[61]*common.v971)}else{common.v13})}));
        let v2116=(v451*(if self.scalar_static_bool[27]{(self.scalar_static_f64[61]*common.v1699)}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[61]*common.v972)}else{common.v13})}));
        let v2117=(v451*(if self.scalar_static_bool[27]{(self.scalar_static_f64[61]*common.v1700)}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[61]*common.v973)}else{common.v13})}));
        let v2118=(v451*(if self.scalar_static_bool[27]{(self.scalar_static_f64[61]*common.v1701)}else{(if self.scalar_static_bool[26]{(self.scalar_static_f64[61]*common.v974)}else{common.v13})}));
        let v2127=(if self.scalar_static_bool[29]{common.v13}else{(if self.scalar_static_bool[28]{common.v13}else{common.v1526})});
        let v2128=(if self.scalar_static_bool[29]{common.v13}else{(if self.scalar_static_bool[28]{common.v13}else{common.v1527})});
        let v2129=(if self.scalar_static_bool[29]{common.v13}else{(if self.scalar_static_bool[28]{common.v13}else{common.v1528})});
        let v2132=(common.v44-(v488*v488));
        let v2137=(common.v44-(v490*v490));
        let v2149=scalar_limexp_derivative(v498);
        let v2154=scalar_limexp_derivative(v502);
        let v2166=(self.scalar_static_f64[75]*(-(if self.scalar_static_bool[29]{common.v13}else{(if self.scalar_static_bool[28]{common.v13}else{common.v1525})})));
        let v2172=scalar_limexp_derivative(v511);
        let v2177=scalar_limexp_derivative(v514);
        let v2568=ddt_scale;
        let v2612=-1e-12;

        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * ((-v424)),
            [4, 5, 8, 10, 12],
            [(-v2047), (-(if self.scalar_static_bool[25]{((v422*((v416*v1030)+(v226*((if self.scalar_static_bool[25]{((v407+(common.v7*v1950))*v1963)}else{v1777})+(self.scalar_static_f64[58]*(if self.scalar_static_bool[25]{((common.v11*v1950)*v1982)}else{common.v13}))))))+(v417*(v1195+(v406+(v419*(if self.scalar_static_bool[25]{v1164}else{v1466}))))))}else{v1940})), (-v2049), (-v2050), (-v2051)],
            [],
            [],
            multiplicity,
        );
        let v773_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v773);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v773_ddt),
            15,
            multiplicity * (((self.scalar_static_f64[117]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v12),
            16,
            multiplicity * (common.v44),
        );
        let v776_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v776);
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v776_ddt,
            0,
            ((self.scalar_static_f64[118]) * ddt_scale),
        );
        stamper.stamp_current_node1_local(
            Some(5),
            Some(8),
            multiplicity * (v12),
            16,
            multiplicity * (common.v44),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(8),
            multiplicity * (v510),
            [4, 5, 8, 10, 11, 12],
            [v2166, (self.scalar_static_f64[75]*(-v2127)), (self.scalar_static_f64[75]*((((v159*(if self.scalar_static_bool[33]{common.v456}else{(if self.scalar_static_bool[31]{(-v2132)}else{self.scalar_static_f64[139]})}))*v2154)-(self.scalar_static_f64[77]*(self.scalar_static_f64[144]*v2149)))-v2128)), (self.scalar_static_f64[75]*(-v2129)), (self.scalar_static_f64[75]*(((v159*(if self.scalar_static_bool[33]{common.v44}else{(if self.scalar_static_bool[31]{v2132}else{self.scalar_static_f64[140]})}))*v2154)-(self.scalar_static_f64[77]*(self.scalar_static_f64[145]*v2149)))), self.scalar_static_f64[147]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((self.scalar_static_f64[75]*((scalar_limexp(v514)-(self.scalar_static_f64[77]*(scalar_limexp(v511)-self.scalar_static_f64[74])))-v477))),
            [4, 5, 8, 10, 12],
            [v2166, (self.scalar_static_f64[75]*((((v159*(if self.scalar_static_bool[33]{common.v456}else{(if self.scalar_static_bool[31]{(-v2137)}else{self.scalar_static_f64[139]})}))*v2177)-(self.scalar_static_f64[77]*(self.scalar_static_f64[144]*v2172)))-v2127)), (self.scalar_static_f64[75]*(-v2128)), (self.scalar_static_f64[75]*((((v159*(if self.scalar_static_bool[33]{common.v44}else{(if self.scalar_static_bool[31]{v2137}else{self.scalar_static_f64[140]})}))*v2177)-(self.scalar_static_f64[77]*(self.scalar_static_f64[145]*v2172)))-v2129)), self.scalar_static_f64[147]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[49]{v777}else{common.v13})),
            [5, 8, 10, 11],
            [(if self.scalar_static_bool[49]{(common.v2553*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2554*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2555*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2556*v2568)}else{common.v13})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[49]{v779}else{common.v13})),
            [5, 8, 10, 11],
            [(if self.scalar_static_bool[49]{(common.v2502*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2503*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2504*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2505*v2568)}else{common.v13})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(10),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[68]{v783}else{common.v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*((-v722)+common.v2586))}else{common.v13})),
            8,
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*common.v2588)}else{common.v13})),
            10,
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*(v722+common.v2589))}else{common.v13})),
        );
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[68]{v786}else{common.v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*common.v2597)}else{common.v13})),
            8,
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*((-v720)+common.v2599))}else{common.v13})),
            11,
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*(v720+common.v2601))}else{common.v13})),
        );
        let v791_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v791);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v791_ddt),
            5,
            multiplicity * (((self.scalar_static_f64[172]) * ddt_scale)),
            7,
            multiplicity * (((self.scalar_static_f64[119]) * ddt_scale)),
        );
        let v793_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v793);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v793_ddt),
            5,
            multiplicity * (((self.scalar_static_f64[120]) * ddt_scale)),
            8,
            multiplicity * (((self.scalar_static_f64[173]) * ddt_scale)),
        );
        let v796_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v796);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v796_ddt),
            4,
            multiplicity * (((common.v2611) * ddt_scale)),
            6,
            multiplicity * (((common.v140) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * ((common.v167*common.v795)),
            4,
            multiplicity * (v2612),
            6,
            multiplicity * (common.v167),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            1,
            (if self.scalar_static_bool[50]{(common.v799+v801)}else{common.v13}),
            [4, 5, 8, 10, 12],
            [common.v2620, common.v2621, common.v2622, common.v2623, common.v2624],
            [1],
            [(if self.scalar_static_bool[50]{(common.v443+(self.scalar_static_f64[105]*v2568))}else{common.v13})],
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            2,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            2,
            common.v13,
        );
        stamper.stamp_current_node2_local(
            Some(11),
            Some(12),
            multiplicity * ((if self.scalar_static_bool[53]{((common.v8-common.v0)/v141)}else{common.v13})),
            11,
            multiplicity * ((if self.scalar_static_bool[53]{(common.v44/v141)}else{common.v13})),
            12,
            multiplicity * ((if self.scalar_static_bool[53]{(common.v456/v141)}else{common.v13})),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[53]{v808}else{common.v13})),
            8,
            multiplicity * ((if self.scalar_static_bool[53]{(v2568*common.v2630)}else{common.v13})),
            12,
            multiplicity * ((if self.scalar_static_bool[53]{(common.v142*v2568)}else{common.v13})),
        );
        stamper.stamp_potential_branch_local(
            Some(11),
            Some(8),
            3,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            3,
            common.v13,
        );
        let v813_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, common.v813);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v813_ddt),
            11,
            multiplicity * (((self.scalar_static_f64[121]) * ddt_scale)),
            14,
            multiplicity * (((self.scalar_static_f64[174]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[54]{((common.v811-common.v1)/self.scalar_static_f64[106])}else{common.v13})),
            8,
            multiplicity * (self.scalar_static_f64[177]),
            14,
            multiplicity * (self.scalar_static_f64[178]),
        );
        stamper.stamp_potential_branch_local(
            Some(14),
            Some(8),
            4,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            4,
            common.v13,
        );
        stamper.stamp_current_node2_local(
            Some(13),
            Some(10),
            multiplicity * ((if self.scalar_static_bool[55]{((v817-common.v3)/self.scalar_static_f64[107])}else{common.v13})),
            10,
            multiplicity * (self.scalar_static_f64[181]),
            13,
            multiplicity * (self.scalar_static_f64[182]),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(10),
            5,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            5,
            common.v13,
        );
        stamper.stamp_current_const_local(
            Some(13),
            Some(10),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_node2_local(
            Some(13),
            Some(11),
            multiplicity * ((if self.scalar_static_bool[56]{((v817-common.v8)/self.scalar_static_f64[108])}else{common.v13})),
            11,
            multiplicity * (self.scalar_static_f64[185]),
            13,
            multiplicity * (self.scalar_static_f64[186]),
        );
        stamper.stamp_potential_branch_local(
            Some(13),
            Some(11),
            6,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            6,
            common.v13,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            7,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            7,
            (if self.scalar_static_bool[57]{(self.scalar_static_f64[109]*ctx.branch_current(branches[7]))}else{common.v13}),
            7,
            self.scalar_static_f64[187],
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            8,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            8,
            common.v13,
        );
        stamper.stamp_potential_branch_local(
            Some(7),
            Some(13),
            9,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            9,
            common.v13,
        );
        let v829_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, common.v829);
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            10,
            v829_ddt,
            10,
            ((self.scalar_static_f64[122]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            11,
            (if self.scalar_static_bool[58]{(v452*v830)}else{common.v13}),
            [4, 5, 8, 10, 12],
            [(if self.scalar_static_bool[58]{(v830*v2114)}else{common.v13}), (if self.scalar_static_bool[58]{(v830*v2115)}else{common.v13}), (if self.scalar_static_bool[58]{(v830*v2116)}else{common.v13}), (if self.scalar_static_bool[58]{(v830*v2117)}else{common.v13}), (if self.scalar_static_bool[58]{(v830*v2118)}else{common.v13})],
            [11],
            [(if self.scalar_static_bool[58]{v452}else{common.v13})],
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            12,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            12,
            common.v13,
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            13,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            13,
            common.v13,
        );
        let v835_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, common.v835);
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            14,
            v835_ddt,
            14,
            ((self.scalar_static_f64[123]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            15,
            (if self.scalar_static_bool[61]{(v453*v836)}else{common.v13}),
            [4, 5, 8, 10, 12],
            [(if self.scalar_static_bool[61]{(v836*v2114)}else{common.v13}), (if self.scalar_static_bool[61]{(v836*v2115)}else{common.v13}), (if self.scalar_static_bool[61]{(v836*v2116)}else{common.v13}), (if self.scalar_static_bool[61]{(v836*v2117)}else{common.v13}), (if self.scalar_static_bool[61]{(v836*v2118)}else{common.v13})],
            [15],
            [(if self.scalar_static_bool[61]{v453}else{common.v13})],
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            16,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            16,
            common.v13,
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            17,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            17,
            common.v13,
        );
        let v841_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, common.v841);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            18,
            v841_ddt,
            18,
            ((self.scalar_static_f64[124]) * ddt_scale),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (1e-15),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (common.v167),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * ((common.v167*(common.v0-ctx.node_voltage(nodes[2])))),
            2,
            multiplicity * (v2612),
            12,
            multiplicity * (common.v167),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(17),
            None,
            multiplicity * (common.v13),
        );
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (v847),
            17,
            multiplicity * (self.scalar_static_f64[188]),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (common.v13),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * ((if self.scalar_static_bool[66]{v848}else{common.v13})),
            18,
            multiplicity * (self.scalar_static_f64[188]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v847),
            17,
            multiplicity * (self.scalar_static_f64[188]),
        );
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[66]{((v766*common.v846)+(v762*v848))}else{common.v13})),
            17,
            multiplicity * ((if self.scalar_static_bool[66]{v766}else{common.v13})),
            18,
            multiplicity * ((if self.scalar_static_bool[66]{v762}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[66]{v856}else{common.v13})),
            17,
            multiplicity * ((if self.scalar_static_bool[66]{(common.v854*v2568)}else{common.v13})),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (common.v846),
            17,
            multiplicity * (common.v44),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v848),
            18,
            multiplicity * (common.v44),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(11),
            Some(8),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (common.v13),
        );
        stamper.stamp_current_const_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[67]{(-(((common.v7*v424)).abs()+((common.v9*v510)).abs()))}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[67]{(common.v32/v49)}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[67]{(common.v44/v49)}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[67]{v869}else{common.v13})),
            3,
            multiplicity * ((if self.scalar_static_bool[67]{(self.scalar_static_f64[125]*v2568)}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[69]{(common.v32*common.v167)}else{common.v13})),
            3,
            multiplicity * (self.scalar_static_f64[189]),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v614=common.v2293;
        let v616=common.v2351;
        let v719=common.v2505;
        let v720=(if self.scalar_static_bool[48]{v719}else{(if self.scalar_static_bool[45]{common.v659}else{(if self.scalar_static_bool[42]{v614}else{common.v560})})});
        let v721=common.v2555;
        let v722=(if self.scalar_static_bool[48]{v721}else{(if self.scalar_static_bool[45]{common.v664}else{(if self.scalar_static_bool[42]{v616}else{common.v566})})});
        let v777=0.0;
        let v779=0.0;
        let v783=0.0;
        let v786=0.0;
        let v801=0.0;
        let v808=0.0;
        let v856=0.0;
        let v869=0.0;
        let v2568=1.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (self.scalar_static_f64[117]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[15]),
            Some(nodes[16]),
            branches[0],
            multiplicity * (self.scalar_static_f64[118]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if self.scalar_static_bool[49]{(common.v2553*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2554*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2555*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2556*v2568)}else{common.v13})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if self.scalar_static_bool[49]{(common.v2502*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2503*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2504*v2568)}else{common.v13}), (if self.scalar_static_bool[49]{(common.v2505*v2568)}else{common.v13})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*((-v722)+common.v2586))}else{common.v13})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*common.v2588)}else{common.v13})),
            nodes[10],
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*(v722+common.v2589))}else{common.v13})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*common.v2597)}else{common.v13})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*((-v720)+common.v2599))}else{common.v13})),
            nodes[11],
            multiplicity * ((if self.scalar_static_bool[68]{(v2568*(v720+common.v2601))}else{common.v13})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[172]),
            nodes[7],
            multiplicity * (self.scalar_static_f64[119]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[120]),
            nodes[8],
            multiplicity * (self.scalar_static_f64[173]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (common.v2611),
            nodes[6],
            multiplicity * (common.v140),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[8], nodes[10], nodes[12]],
            &[common.v2620, common.v2621, common.v2622, common.v2623, common.v2624],
            &[branches[1]],
            &[(if self.scalar_static_bool[50]{(common.v443+(self.scalar_static_f64[105]*v2568))}else{common.v13})],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[53]{(v2568*common.v2630)}else{common.v13})),
            nodes[12],
            multiplicity * ((if self.scalar_static_bool[53]{(common.v142*v2568)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (self.scalar_static_f64[121]),
            nodes[14],
            multiplicity * (self.scalar_static_f64[174]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[10],
            multiplicity * (self.scalar_static_f64[122]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[9]),
            Some(nodes[2]),
            branches[14],
            multiplicity * (self.scalar_static_f64[123]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[0]),
            branches[18],
            multiplicity * (self.scalar_static_f64[124]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[17],
            multiplicity * ((if self.scalar_static_bool[66]{(common.v854*v2568)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if self.scalar_static_bool[67]{(self.scalar_static_f64[125]*v2568)}else{common.v13})),
        );
    }
}
