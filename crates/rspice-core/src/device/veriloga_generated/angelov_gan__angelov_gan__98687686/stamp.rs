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
    v44: f64,
    v45: f64,
    v47: f64,
    v84: f64,
    v120: bool,
    v124: f64,
    v131: bool,
    v136: bool,
    v142: f64,
    v144: f64,
    v149: f64,
    v155: f64,
    v170: f64,
    v179: f64,
    v184: f64,
    v194: f64,
    v202: f64,
    v211: f64,
    v246: f64,
    v247: f64,
    v249: f64,
    v316: f64,
    v339: f64,
    v366: f64,
    v453: f64,
    v467: f64,
    v577: f64,
    v583: f64,
    v676: f64,
    v681: f64,
    v719: f64,
    v735: f64,
    v786: f64,
    v792: f64,
    v802: f64,
    v805: f64,
    v820: f64,
    v822: f64,
    v824: f64,
    v825: f64,
    v828: f64,
    v829: f64,
    v836: f64,
    v840: f64,
    v842: f64,
    v858: f64,
    v864: f64,
    v870: f64,
    v875: f64,
    v883: f64,
    v884: f64,
    v897: f64,
    v921: f64,
    v922: f64,
    v942: f64,
    v947: f64,
    v949: f64,
    v951: f64,
    v953: f64,
    v954: f64,
    v999: f64,
    v1000: f64,
    v1001: f64,
    v1002: f64,
    v1003: f64,
    v1100: f64,
    v1101: f64,
    v1102: f64,
    v1103: f64,
    v1104: f64,
    v1105: f64,
    v1106: f64,
    v1115: f64,
    v1116: f64,
    v1117: f64,
    v1118: f64,
    v1454: f64,
    v1455: f64,
    v1456: f64,
    v1457: f64,
    v1458: f64,
    v1554: f64,
    v1555: f64,
    v1556: f64,
    v1557: f64,
    v1726: f64,
    v1727: f64,
    v1728: f64,
    v1729: f64,
    v1730: f64,
    v2322: f64,
    v2380: f64,
    v2531: f64,
    v2532: f64,
    v2533: f64,
    v2534: f64,
    v2582: f64,
    v2583: f64,
    v2584: f64,
    v2585: f64,
    v2615: f64,
    v2617: f64,
    v2618: f64,
    v2626: f64,
    v2628: f64,
    v2630: f64,
    v2640: f64,
    v2649: f64,
    v2650: f64,
    v2651: f64,
    v2652: f64,
    v2653: f64,
    v2659: f64,
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
        let v35=(if (self.scalar_static_f64[10]!=0.0){(self.scalar_static_f64[218]+(v32).abs())}else{self.scalar_static_f64[218]});
        let v39=((v35-self.scalar_static_f64[9])).abs();
        let v44=1.0;
        let v45=(if ((v39>v13)||self.scalar_static_bool[2]){v44}else{v13});
        let v47=(v39).abs();
        let v84=(v44+(v47*self.scalar_static_f64[24]));
        let v95=(v44+(v39*self.scalar_static_f64[28]));
        let v120=((v45!=0.0)&&(self.scalar_static_f64[36]!=0.0));
        let v124=(v44+(self.scalar_static_f64[24]*(v39*v39)));
        let v131=((v45!=0.0)&&self.scalar_static_bool[8]);
        let v136=(!(v45!=0.0));
        let v139=(if v136{self.scalar_static_f64[17]}else{(if (v45!=0.0){(self.scalar_static_f64[17]*(v44+(v47*self.scalar_static_f64[18])))}else{v13})});
        let v140=(if v136{self.scalar_static_f64[19]}else{(if (v45!=0.0){(self.scalar_static_f64[19]*(v44+(v47*self.scalar_static_f64[20])))}else{v13})});
        let v141=(if v136{self.scalar_static_f64[21]}else{(if (v45!=0.0){(self.scalar_static_f64[21]*(v44+(v47*self.scalar_static_f64[22])))}else{v13})});
        let v142=(if v136{self.scalar_static_f64[23]}else{(if (v45!=0.0){(self.scalar_static_f64[23]*v84)}else{v13})});
        let v144=(if v136{self.scalar_static_f64[38]}else{(if v131{(v84*self.scalar_static_f64[38])}else{(if v120{(v124*self.scalar_static_f64[38])}else{v13})})});
        let v146=(if v136{self.scalar_static_f64[27]}else{(if (v45!=0.0){(self.scalar_static_f64[27]*v95)}else{v13})});
        let v147=(if v136{self.scalar_static_f64[29]}else{(if (v45!=0.0){(v95*self.scalar_static_f64[29])}else{v13})});
        let v149=(if v136{self.scalar_static_f64[32]}else{(if (v45!=0.0){(self.scalar_static_f64[32]+(v39*self.scalar_static_f64[33]))}else{v13})});
        let v155=0.5;
        let v164=(v7*self.scalar_static_f64[45]);
        let v165=(v164).cosh();
        let v170=1e-12;
        let v172=(v170+(v165*v165));
        let v178=(v44+(v47*self.scalar_static_f64[49]));
        let v179=((self.scalar_static_f64[47]*(v44+(self.scalar_static_f64[48]/v172)))*v178);
        let v184=(self.scalar_static_f64[50]*(v44+(v47*self.scalar_static_f64[51])));
        let v189=((v7*self.scalar_static_f64[53])).tanh();
        let v194=(v6-v149);
        let v195=(self.scalar_static_f64[54]*v194);
        let v199=(v44+(v47*self.scalar_static_f64[26]));
        let v200=((((((if v136{self.scalar_static_f64[25]}else{(if (v45!=0.0){(self.scalar_static_f64[25]+(v39*self.scalar_static_f64[26]))}else{v13})})-self.scalar_static_f64[52])+(self.scalar_static_f64[52]*v189))-(v11*self.scalar_static_f64[46]))-(v194*v195))*v199);
        let v201=(v2-v200);
        let v202=(v201*v201);
        let v207=(v184*v201);
        let v209=(((v179*v201)+(v202*self.scalar_static_f64[55]))+(v202*v207));
        let v210=(v209).tanh();
        let v211=(v44+v210);
        let v213=(-v209);
        let v217=((v155*(scalar_limexp(v209)-scalar_limexp(v213)))).tanh();
        let v227=2.0;
        let v246=(v5-v200);
        let v247=(if self.scalar_static_bool[16]{v246}else{v165});
        let v249=(if self.scalar_static_bool[16]{(v247*v247)}else{v201});
        let v299=(if self.scalar_static_bool[19]{v201}else{v247});
        let v301=(if self.scalar_static_bool[19]{(v299*v299)}else{v249});
        let v304=(v184*v301);
        let v306=((v299+(self.scalar_static_f64[55]*v301))+(v299*v304));
        let v308=(if self.scalar_static_bool[19]{(v179*v306)}else{v209});
        let v310=(-v308);
        let v314=((v155*(scalar_limexp(v308)-scalar_limexp(v310)))).tanh();
        let v316=(if self.scalar_static_bool[19]{(v44+v314)}else{(v44+v217)});
        let v339=(if self.scalar_static_bool[22]{v201}else{v299});
        let v341=(if self.scalar_static_bool[22]{(v339*v339)}else{v301});
        let v344=(v184*v341);
        let v346=((v339+(self.scalar_static_f64[55]*v341))+(v339*v344));
        let v348=(if self.scalar_static_bool[22]{(v179*v346)}else{v308});
        let v360=(-v348);
        let v364=((v155*(scalar_limexp(v348)-scalar_limexp(v360)))).tanh();
        let v366=(if self.scalar_static_bool[22]{(v44+v364)}else{v316});
        let v437=(v44+v211);
        let v450=(v44+v366);
        let v453=(if self.scalar_static_bool[28]{(self.scalar_static_f64[67]+(v141/v450))}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[67]+(v141/v437))}else{v13})});
        let v467=-1.0;
        let v536=(v7*self.scalar_static_f64[89]);
        let v537=((v146+(v9*self.scalar_static_f64[88]))+v536);
        let v538=(v537).tanh();
        let v539=(v44+v538);
        let v544=((self.scalar_static_f64[90]+(v7*self.scalar_static_f64[91]))).tanh();
        let v545=(v44+v544);
        let v550=((self.scalar_static_f64[92]-(v7*self.scalar_static_f64[93]))).tanh();
        let v552=((v44+v550)-self.scalar_static_f64[89]);
        let v556=((v147+(v5*self.scalar_static_f64[94]))-v536);
        let v557=(v556).tanh();
        let v558=(v44+v557);
        let v574=(v139*v539);
        let v577=(if self.scalar_static_bool[40]{(self.scalar_static_f64[100]+(v545*v574))}else{self.scalar_static_f64[101]});
        let v583=(if self.scalar_static_bool[40]{(self.scalar_static_f64[102]+(v140*((v552*v558)+self.scalar_static_f64[104])))}else{self.scalar_static_f64[103]});
        let v588=(if self.scalar_static_bool[43]{(v545-self.scalar_static_f64[89])}else{v545});
        let v589=(v146+v536);
        let v590=(v589).cosh();
        let v591=(if self.scalar_static_bool[43]{v590}else{v13});
        let v593=(if self.scalar_static_bool[43]{(v591).ln()}else{v13});
        let v594=(v537).cosh();
        let v595=(if self.scalar_static_bool[43]{v594}else{v13});
        let v597=(if self.scalar_static_bool[43]{(v595).ln()}else{v13});
        let v599=(if self.scalar_static_bool[43]{(v589+v593)}else{v13});
        let v601=((v537+v597)-v599);
        let v604=(v9*self.scalar_static_f64[104]);
        let v607=(v9*self.scalar_static_f64[100]);
        let v610=(v147-v536);
        let v611=(v610).cosh();
        let v612=(if self.scalar_static_bool[43]{v611}else{v591});
        let v614=(if self.scalar_static_bool[43]{(v612).ln()}else{v13});
        let v615=(v556).cosh();
        let v616=(if self.scalar_static_bool[43]{v615}else{v595});
        let v618=(if self.scalar_static_bool[43]{(v616).ln()}else{v13});
        let v620=(if self.scalar_static_bool[43]{(v610+v614)}else{v13});
        let v622=((v556+v618)-v620);
        let v625=(v5*self.scalar_static_f64[104]);
        let v628=(v5*self.scalar_static_f64[102]);
        let v639=(v9/self.scalar_static_f64[105]);
        let v641=(if self.scalar_static_bool[46]{(v639-v44)}else{v13});
        let v644=(v641*v641);
        let v645=(self.scalar_static_f64[107]+v644);
        let v647=f64::powf(v645,self.scalar_static_f64[108]);
        let v651=(self.scalar_static_f64[107]+(v644*self.scalar_static_f64[110]));
        let v657=((v146+(self.scalar_static_f64[88]*(v9+v536)))).tanh();
        let v660=(if self.scalar_static_bool[46]{v545}else{v588});
        let v662=(v550+self.scalar_static_f64[111]);
        let v663=(if self.scalar_static_bool[46]{v662}else{v552});
        let v668=((v147+(self.scalar_static_f64[94]*(v5+(v7*self.scalar_static_f64[111]))))).tanh();
        let v670=(if self.scalar_static_bool[46]{(v44+v668)}else{v558});
        let v674=(v139*((if self.scalar_static_bool[46]{(v44+v657)}else{v539})+((if self.scalar_static_bool[46]{(v647*v651)}else{v13})*self.scalar_static_f64[112])));
        let v676=(self.scalar_static_f64[100]+(v660*v674));
        let v681=(self.scalar_static_f64[102]+(v140*(self.scalar_static_f64[104]+(v663*v670))));
        let v686=(if self.scalar_static_bool[49]{v590}else{v612});
        let v689=(if self.scalar_static_bool[49]{v594}else{v616});
        let v694=(self.scalar_static_f64[112]*(v9+self.scalar_static_f64[105]));
        let v695=(v467+v639);
        let v697=(self.scalar_static_f64[107]+f64::powf(v695,v227));
        let v699=f64::powf(v697,self.scalar_static_f64[114]);
        let v712=(((if self.scalar_static_bool[49]{(v694*v699)}else{v13})+((v537+(if self.scalar_static_bool[49]{(v689).ln()}else{v597}))-(if self.scalar_static_bool[49]{(v589+(if self.scalar_static_bool[49]{(v686).ln()}else{v593}))}else{v599})))-self.scalar_static_f64[119]);
        let v713=(v544+self.scalar_static_f64[111]);
        let v719=(if self.scalar_static_bool[49]{(v607+(v139*(v604+((v712*v713)/self.scalar_static_f64[88]))))}else{(if self.scalar_static_bool[43]{((v139*(((v588*v601)/self.scalar_static_f64[88])+v604))+v607)}else{v13})});
        let v720=(if self.scalar_static_bool[49]{v611}else{v686});
        let v723=(if self.scalar_static_bool[49]{v615}else{v689});
        let v729=((v556+(if self.scalar_static_bool[49]{(v723).ln()}else{v618}))-(if self.scalar_static_bool[49]{(v610+(if self.scalar_static_bool[49]{(v720).ln()}else{v614}))}else{v620}));
        let v735=(if self.scalar_static_bool[49]{(v628+(v140*(v625+((v662*v729)/self.scalar_static_f64[94]))))}else{(if self.scalar_static_bool[43]{((v140*(((v552*v622)/self.scalar_static_f64[94])+v625))+v628)}else{v13})});
        let v786=(if self.scalar_static_bool[67]{((v139*((v35*5.5226012e-23)*self.scalar_static_f64[140]))*self.scalar_static_f64[142])}else{v13});
        let v792=3.141592653589793;
        let v802=(self.scalar_static_f64[144]*ctx.node_voltage(nodes[15]));
        let v805=(self.scalar_static_f64[145]*ctx.branch_current(branches[0]));
        let v820=(self.scalar_static_f64[146]*(ctx.node_voltage(nodes[7])-v4));
        let v822=(v7*self.scalar_static_f64[147]);
        let v824=(ctx.node_voltage(nodes[6])-v10);
        let v825=(v142*v824);
        let v827=ctx.branch_current(branches[1]);
        let v828=(v453*v827);
        let v829=(self.scalar_static_f64[121]*v827);
        let v836=(v2*v144);
        let v840=ctx.node_voltage(nodes[14]);
        let v842=(self.scalar_static_f64[148]*(v8-v840));
        let v858=(self.scalar_static_f64[149]*ctx.branch_current(branches[10]));
        let v864=(self.scalar_static_f64[150]*ctx.branch_current(branches[14]));
        let v870=(self.scalar_static_f64[151]*ctx.branch_current(branches[18]));
        let v875=ctx.node_voltage(nodes[17]);
        let v883=(-(if self.scalar_static_bool[67]{(v786*v792)}else{v13}));
        let v884=(v875*v883);
        let v897=(v32*self.scalar_static_f64[152]);
        let v904=(v164).sinh();
        let v905=(self.scalar_static_f64[45]*v904);
        let v906=(self.scalar_static_f64[153]*v904);
        let v908=(v165*v905);
        let v910=(v165*v906);
        let v914=(v172*v172);
        let v921=(v178*(self.scalar_static_f64[47]*((-(self.scalar_static_f64[48]*(v908+v908)))/v914)));
        let v922=(v178*(self.scalar_static_f64[47]*((-(self.scalar_static_f64[48]*(v910+v910)))/v914)));
        let v925=(v44-(v189*v189));
        let v939=(v199*((self.scalar_static_f64[52]*(self.scalar_static_f64[53]*v925))-(v195+v195)));
        let v940=(v199*((self.scalar_static_f64[52]*(self.scalar_static_f64[155]*v925))-self.scalar_static_f64[154]));
        let v941=(v199*(-((-v195)+(v194*self.scalar_static_f64[156]))));
        let v942=(-(v199*self.scalar_static_f64[154]));
        let v943=(-v939);
        let v944=(v467-v940);
        let v945=(-v941);
        let v946=(v201*v942);
        let v947=(v946+v946);
        let v948=(v201*v943);
        let v949=(v948+v948);
        let v950=(v201*v944);
        let v951=(v950+v950);
        let v952=(v201*v945);
        let v953=(v952+v952);
        let v954=(v201+v201);
        let v992=(((v179*v942)+(self.scalar_static_f64[55]*v947))+((v207*v947)+(v202*(v184*v942))));
        let v993=((((v201*v921)+(v179*v943))+(self.scalar_static_f64[55]*v949))+((v207*v949)+(v202*(v184*v943))));
        let v994=((((v201*v922)+(v179*v944))+(self.scalar_static_f64[55]*v951))+((v207*v951)+(v202*(v184*v944))));
        let v995=(((v179*v945)+(self.scalar_static_f64[55]*v953))+((v207*v953)+(v202*(v184*v945))));
        let v996=((v179+(self.scalar_static_f64[55]*v954))+((v207*v954)+(v184*v202)));
        let v998=(v44-(v210*v210));
        let v999=(v992*v998);
        let v1000=(v993*v998);
        let v1001=(v994*v998);
        let v1002=(v995*v998);
        let v1003=(v996*v998);
        let v1004=scalar_limexp_derivative(v209);
        let v1015=scalar_limexp_derivative(v213);
        let v1032=(v44-(v217*v217));
        let v1100=(v467-v939);
        let v1101=(-v940);
        let v1102=(v44-v941);
        let v1103=(if self.scalar_static_bool[16]{v942}else{v13});
        let v1104=(if self.scalar_static_bool[16]{v1100}else{v905});
        let v1105=(if self.scalar_static_bool[16]{v1101}else{v906});
        let v1106=(if self.scalar_static_bool[16]{v1102}else{v13});
        let v1107=(v247*v1103);
        let v1109=(v247*v1104);
        let v1111=(v247*v1105);
        let v1113=(v247*v1106);
        let v1115=(if self.scalar_static_bool[16]{(v1107+v1107)}else{v942});
        let v1116=(if self.scalar_static_bool[16]{(v1109+v1109)}else{v943});
        let v1117=(if self.scalar_static_bool[16]{(v1111+v1111)}else{v944});
        let v1118=(if self.scalar_static_bool[16]{(v1113+v1113)}else{v945});
        let v1351=(if self.scalar_static_bool[19]{v942}else{v1103});
        let v1352=(if self.scalar_static_bool[19]{v943}else{v1104});
        let v1353=(if self.scalar_static_bool[19]{v944}else{v1105});
        let v1354=(if self.scalar_static_bool[19]{v945}else{v1106});
        let v1356=(v299*v1351);
        let v1358=(v299*v1352);
        let v1360=(v299*v1353);
        let v1362=(v299*v1354);
        let v1364=(v299*self.scalar_static_f64[161]);
        let v1366=(if self.scalar_static_bool[19]{(v1356+v1356)}else{v1115});
        let v1367=(if self.scalar_static_bool[19]{(v1358+v1358)}else{v1116});
        let v1368=(if self.scalar_static_bool[19]{(v1360+v1360)}else{v1117});
        let v1369=(if self.scalar_static_bool[19]{(v1362+v1362)}else{v1118});
        let v1370=(if self.scalar_static_bool[19]{(v1364+v1364)}else{self.scalar_static_f64[158]});
        let v1415=(if self.scalar_static_bool[19]{(v179*((v1351+(self.scalar_static_f64[55]*v1366))+((v304*v1351)+(v299*(v184*v1366)))))}else{v992});
        let v1416=(if self.scalar_static_bool[19]{((v306*v921)+(v179*((v1352+(self.scalar_static_f64[55]*v1367))+((v304*v1352)+(v299*(v184*v1367))))))}else{v993});
        let v1417=(if self.scalar_static_bool[19]{((v306*v922)+(v179*((v1353+(self.scalar_static_f64[55]*v1368))+((v304*v1353)+(v299*(v184*v1368))))))}else{v994});
        let v1418=(if self.scalar_static_bool[19]{(v179*((v1354+(self.scalar_static_f64[55]*v1369))+((v304*v1354)+(v299*(v184*v1369)))))}else{v995});
        let v1419=(if self.scalar_static_bool[19]{(v179*((self.scalar_static_f64[161]+(self.scalar_static_f64[55]*v1370))+((v304*self.scalar_static_f64[161])+(v299*(v184*v1370)))))}else{v996});
        let v1420=scalar_limexp_derivative(v308);
        let v1431=scalar_limexp_derivative(v310);
        let v1448=(v44-(v314*v314));
        let v1454=(if self.scalar_static_bool[19]{((v155*((v1415*v1420)-((-v1415)*v1431)))*v1448)}else{((v155*((v992*v1004)-((-v992)*v1015)))*v1032)});
        let v1455=(if self.scalar_static_bool[19]{((v155*((v1416*v1420)-((-v1416)*v1431)))*v1448)}else{((v155*((v993*v1004)-((-v993)*v1015)))*v1032)});
        let v1456=(if self.scalar_static_bool[19]{((v155*((v1417*v1420)-((-v1417)*v1431)))*v1448)}else{((v155*((v994*v1004)-((-v994)*v1015)))*v1032)});
        let v1457=(if self.scalar_static_bool[19]{((v155*((v1418*v1420)-((-v1418)*v1431)))*v1448)}else{((v155*((v995*v1004)-((-v995)*v1015)))*v1032)});
        let v1458=(if self.scalar_static_bool[19]{((v155*((v1419*v1420)-((-v1419)*v1431)))*v1448)}else{((v155*((v996*v1004)-((-v996)*v1015)))*v1032)});
        let v1554=(if self.scalar_static_bool[22]{v942}else{v1351});
        let v1555=(if self.scalar_static_bool[22]{v943}else{v1352});
        let v1556=(if self.scalar_static_bool[22]{v944}else{v1353});
        let v1557=(if self.scalar_static_bool[22]{v945}else{v1354});
        let v1559=(v339*v1554);
        let v1561=(v339*v1555);
        let v1563=(v339*v1556);
        let v1565=(v339*v1557);
        let v1567=(v339*self.scalar_static_f64[162]);
        let v1569=(if self.scalar_static_bool[22]{(v1559+v1559)}else{v1366});
        let v1570=(if self.scalar_static_bool[22]{(v1561+v1561)}else{v1367});
        let v1571=(if self.scalar_static_bool[22]{(v1563+v1563)}else{v1368});
        let v1572=(if self.scalar_static_bool[22]{(v1565+v1565)}else{v1369});
        let v1573=(if self.scalar_static_bool[22]{(v1567+v1567)}else{v1370});
        let v1618=(if self.scalar_static_bool[22]{(v179*((v1554+(self.scalar_static_f64[55]*v1569))+((v344*v1554)+(v339*(v184*v1569)))))}else{v1415});
        let v1619=(if self.scalar_static_bool[22]{((v346*v921)+(v179*((v1555+(self.scalar_static_f64[55]*v1570))+((v344*v1555)+(v339*(v184*v1570))))))}else{v1416});
        let v1620=(if self.scalar_static_bool[22]{((v346*v922)+(v179*((v1556+(self.scalar_static_f64[55]*v1571))+((v344*v1556)+(v339*(v184*v1571))))))}else{v1417});
        let v1621=(if self.scalar_static_bool[22]{(v179*((v1557+(self.scalar_static_f64[55]*v1572))+((v344*v1557)+(v339*(v184*v1572)))))}else{v1418});
        let v1622=(if self.scalar_static_bool[22]{(v179*((self.scalar_static_f64[162]+(self.scalar_static_f64[55]*v1573))+((v344*self.scalar_static_f64[162])+(v339*(v184*v1573)))))}else{v1419});
        let v1692=scalar_limexp_derivative(v348);
        let v1703=scalar_limexp_derivative(v360);
        let v1720=(v44-(v364*v364));
        let v1726=(if self.scalar_static_bool[22]{((v155*((v1618*v1692)-((-v1618)*v1703)))*v1720)}else{v1454});
        let v1727=(if self.scalar_static_bool[22]{((v155*((v1619*v1692)-((-v1619)*v1703)))*v1720)}else{v1455});
        let v1728=(if self.scalar_static_bool[22]{((v155*((v1620*v1692)-((-v1620)*v1703)))*v1720)}else{v1456});
        let v1729=(if self.scalar_static_bool[22]{((v155*((v1621*v1692)-((-v1621)*v1703)))*v1720)}else{v1457});
        let v1730=(if self.scalar_static_bool[22]{((v155*((v1622*v1692)-((-v1622)*v1703)))*v1720)}else{v1458});
        let v2083=(v437*v437);
        let v2114=(v450*v450);
        let v2223=(v44-(v538*v538));
        let v2224=(self.scalar_static_f64[89]*v2223);
        let v2225=(self.scalar_static_f64[177]*v2223);
        let v2226=(self.scalar_static_f64[88]*v2223);
        let v2229=(v44-(v544*v544));
        let v2230=(self.scalar_static_f64[91]*v2229);
        let v2231=(self.scalar_static_f64[178]*v2229);
        let v2234=(v44-(v550*v550));
        let v2235=(self.scalar_static_f64[179]*v2234);
        let v2236=(self.scalar_static_f64[93]*v2234);
        let v2240=(v44-(v557*v557));
        let v2241=(self.scalar_static_f64[181]*v2240);
        let v2242=(self.scalar_static_f64[89]*v2240);
        let v2243=(self.scalar_static_f64[94]*v2240);
        let v2270=(v589).sinh();
        let v2271=(self.scalar_static_f64[89]*v2270);
        let v2272=(self.scalar_static_f64[176]*v2270);
        let v2273=(if self.scalar_static_bool[43]{v2271}else{v13});
        let v2274=(if self.scalar_static_bool[43]{v2272}else{v13});
        let v2277=(if self.scalar_static_bool[43]{(v2273/v591)}else{v13});
        let v2278=(if self.scalar_static_bool[43]{(v2274/v591)}else{v13});
        let v2279=(v537).sinh();
        let v2280=(self.scalar_static_f64[89]*v2279);
        let v2281=(self.scalar_static_f64[177]*v2279);
        let v2282=(self.scalar_static_f64[88]*v2279);
        let v2283=(if self.scalar_static_bool[43]{v2280}else{v13});
        let v2284=(if self.scalar_static_bool[43]{v2281}else{v13});
        let v2285=(if self.scalar_static_bool[43]{v2282}else{v13});
        let v2289=(if self.scalar_static_bool[43]{(v2283/v595)}else{v13});
        let v2290=(if self.scalar_static_bool[43]{(v2284/v595)}else{v13});
        let v2291=(if self.scalar_static_bool[43]{(v2285/v595)}else{v13});
        let v2294=(if self.scalar_static_bool[43]{(self.scalar_static_f64[89]+v2277)}else{v13});
        let v2295=(if self.scalar_static_bool[43]{(self.scalar_static_f64[176]+v2278)}else{v13});
        let v2322=(if self.scalar_static_bool[43]{(self.scalar_static_f64[100]+(v139*(self.scalar_static_f64[104]+((v588*(self.scalar_static_f64[88]+v2291))/self.scalar_static_f64[88]))))}else{v13});
        let v2323=(v610).sinh();
        let v2324=(self.scalar_static_f64[176]*v2323);
        let v2325=(self.scalar_static_f64[89]*v2323);
        let v2326=(if self.scalar_static_bool[43]{v2324}else{v2273});
        let v2327=(if self.scalar_static_bool[43]{v2325}else{v2274});
        let v2330=(if self.scalar_static_bool[43]{(v2326/v612)}else{v13});
        let v2331=(if self.scalar_static_bool[43]{(v2327/v612)}else{v13});
        let v2332=(v556).sinh();
        let v2333=(self.scalar_static_f64[181]*v2332);
        let v2334=(self.scalar_static_f64[89]*v2332);
        let v2335=(self.scalar_static_f64[94]*v2332);
        let v2336=(if self.scalar_static_bool[43]{v2333}else{v2283});
        let v2337=(if self.scalar_static_bool[43]{v2334}else{v2284});
        let v2338=(if self.scalar_static_bool[43]{v2335}else{v13});
        let v2339=(if self.scalar_static_bool[43]{v13}else{v2285});
        let v2344=(if self.scalar_static_bool[43]{(v2336/v616)}else{v13});
        let v2345=(if self.scalar_static_bool[43]{(v2337/v616)}else{v13});
        let v2346=(if self.scalar_static_bool[43]{(v2338/v616)}else{v13});
        let v2347=(if self.scalar_static_bool[43]{(v2339/v616)}else{v13});
        let v2350=(if self.scalar_static_bool[43]{(self.scalar_static_f64[176]+v2330)}else{v13});
        let v2351=(if self.scalar_static_bool[43]{(self.scalar_static_f64[89]+v2331)}else{v13});
        let v2380=(if self.scalar_static_bool[43]{(self.scalar_static_f64[102]+(v140*(self.scalar_static_f64[104]+((v552*(self.scalar_static_f64[94]+v2346))/self.scalar_static_f64[94]))))}else{v13});
        let v2392=(v641*self.scalar_static_f64[187]);
        let v2393=(v2392+v2392);
        let v2394=(v641*self.scalar_static_f64[188]);
        let v2395=(v2394+v2394);
        let v2398=(self.scalar_static_f64[108]*f64::powf(v645,self.scalar_static_f64[189]));
        let v2415=(v44-(v657*v657));
        let v2427=(v44-(v668*v668));
        let v2464=(if self.scalar_static_bool[49]{v2271}else{v2326});
        let v2465=(if self.scalar_static_bool[49]{v2272}else{v2327});
        let v2470=(if self.scalar_static_bool[49]{v2280}else{v2336});
        let v2471=(if self.scalar_static_bool[49]{v2281}else{v2337});
        let v2472=(if self.scalar_static_bool[49]{v13}else{v2338});
        let v2473=(if self.scalar_static_bool[49]{v2282}else{v2339});
        let v2484=(v227*f64::powf(v695,v44));
        let v2489=(self.scalar_static_f64[114]*f64::powf(v697,self.scalar_static_f64[198]));
        let v2531=(if self.scalar_static_bool[49]{(v139*(((v713*((self.scalar_static_f64[89]+(if self.scalar_static_bool[49]{(v2470/v689)}else{v2289}))-(if self.scalar_static_bool[49]{(self.scalar_static_f64[89]+(if self.scalar_static_bool[49]{(v2464/v686)}else{v2277}))}else{v2294})))+(v712*v2230))/self.scalar_static_f64[88]))}else{(if self.scalar_static_bool[43]{(v139*(((v601*v2230)+(v588*((self.scalar_static_f64[89]+v2289)-v2294)))/self.scalar_static_f64[88]))}else{v13})});
        let v2532=(if self.scalar_static_bool[49]{(self.scalar_static_f64[183]+(v139*(self.scalar_static_f64[182]+(((v713*((if self.scalar_static_bool[49]{((v699*self.scalar_static_f64[197])+(v694*((self.scalar_static_f64[185]*v2484)*v2489)))}else{v13})+((self.scalar_static_f64[177]+(if self.scalar_static_bool[49]{(v2471/v689)}else{v2290}))-(if self.scalar_static_bool[49]{(self.scalar_static_f64[176]+(if self.scalar_static_bool[49]{(v2465/v686)}else{v2278}))}else{v2295}))))+(v712*v2231))/self.scalar_static_f64[88]))))}else{(if self.scalar_static_bool[43]{((v139*((((v601*v2231)+(v588*((self.scalar_static_f64[177]+v2290)-v2295)))/self.scalar_static_f64[88])+self.scalar_static_f64[182]))+self.scalar_static_f64[183])}else{v13})});
        let v2533=(if self.scalar_static_bool[49]{(v139*((v713*(if self.scalar_static_bool[49]{(v2472/v689)}else{v13}))/self.scalar_static_f64[88]))}else{v13});
        let v2534=(if self.scalar_static_bool[49]{(self.scalar_static_f64[100]+(v139*(self.scalar_static_f64[104]+((v713*((if self.scalar_static_bool[49]{((self.scalar_static_f64[112]*v699)+(v694*((self.scalar_static_f64[186]*v2484)*v2489)))}else{v13})+(self.scalar_static_f64[88]+(if self.scalar_static_bool[49]{(v2473/v689)}else{v2291}))))/self.scalar_static_f64[88]))))}else{v2322});
        let v2582=(if self.scalar_static_bool[49]{(self.scalar_static_f64[184]+(v140*(self.scalar_static_f64[182]+(((v729*v2235)+(v662*((self.scalar_static_f64[181]+(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{v2333}else{v2470})/v723)}else{v2344}))-(if self.scalar_static_bool[49]{(self.scalar_static_f64[176]+(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{v2324}else{v2464})/v720)}else{v2330}))}else{v2350}))))/self.scalar_static_f64[94]))))}else{(if self.scalar_static_bool[43]{((v140*(self.scalar_static_f64[182]+(((v622*v2235)+(v552*((self.scalar_static_f64[181]+v2344)-v2350)))/self.scalar_static_f64[94])))+self.scalar_static_f64[184])}else{v13})});
        let v2583=(if self.scalar_static_bool[49]{(v140*(((v729*v2236)+(v662*((self.scalar_static_f64[89]+(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{v2334}else{v2471})/v723)}else{v2345}))-(if self.scalar_static_bool[49]{(self.scalar_static_f64[89]+(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{v2325}else{v2465})/v720)}else{v2331}))}else{v2351}))))/self.scalar_static_f64[94]))}else{(if self.scalar_static_bool[43]{(v140*(((v622*v2236)+(v552*((self.scalar_static_f64[89]+v2345)-v2351)))/self.scalar_static_f64[94]))}else{v13})});
        let v2584=(if self.scalar_static_bool[49]{(self.scalar_static_f64[102]+(v140*(self.scalar_static_f64[104]+((v662*(self.scalar_static_f64[94]+(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{v2335}else{v2472})/v723)}else{v2346})))/self.scalar_static_f64[94]))))}else{v2380});
        let v2585=(if self.scalar_static_bool[49]{(v140*((v662*(if self.scalar_static_bool[49]{((if self.scalar_static_bool[49]{v13}else{v2473})/v723)}else{v2347}))/self.scalar_static_f64[94]))}else{(if self.scalar_static_bool[43]{(v140*((v552*v2347)/self.scalar_static_f64[94]))}else{v13})});
        let v2615=(v5*(if self.scalar_static_bool[49]{v13}else{(if self.scalar_static_bool[46]{(v140*((v670*v2235)+(v663*(if self.scalar_static_bool[46]{(self.scalar_static_f64[195]*v2427)}else{v2241}))))}else{(if self.scalar_static_bool[43]{v13}else{(if self.scalar_static_bool[40]{(v140*((v558*v2235)+(v552*v2241)))}else{v13})})})}));
        let v2617=(v5*(if self.scalar_static_bool[49]{v13}else{(if self.scalar_static_bool[46]{(v140*((v670*v2236)+(v663*(if self.scalar_static_bool[46]{(self.scalar_static_f64[196]*v2427)}else{v2242}))))}else{(if self.scalar_static_bool[43]{v13}else{(if self.scalar_static_bool[40]{(v140*((v558*v2236)+(v552*v2242)))}else{v13})})})}));
        let v2618=(v5*(if self.scalar_static_bool[49]{v13}else{(if self.scalar_static_bool[46]{(v140*(v663*(if self.scalar_static_bool[46]{(self.scalar_static_f64[94]*v2427)}else{v2243})))}else{(if self.scalar_static_bool[43]{v13}else{(if self.scalar_static_bool[40]{(v140*(v552*v2243))}else{v13})})})}));
        let v2626=(v9*(if self.scalar_static_bool[49]{v13}else{(if self.scalar_static_bool[46]{((v674*v2230)+(v660*(v139*(if self.scalar_static_bool[46]{(self.scalar_static_f64[191]*v2415)}else{v2224}))))}else{(if self.scalar_static_bool[43]{v13}else{(if self.scalar_static_bool[40]{((v574*v2230)+(v545*(v139*v2224)))}else{v13})})})}));
        let v2628=(v9*(if self.scalar_static_bool[49]{v13}else{(if self.scalar_static_bool[46]{((v674*v2231)+(v660*(v139*((if self.scalar_static_bool[46]{(self.scalar_static_f64[192]*v2415)}else{v2225})+(self.scalar_static_f64[112]*(if self.scalar_static_bool[46]{((v651*(v2393*v2398))+(v647*(self.scalar_static_f64[110]*v2393)))}else{v13}))))))}else{(if self.scalar_static_bool[43]{v13}else{(if self.scalar_static_bool[40]{((v574*v2231)+(v545*(v139*v2225)))}else{v13})})})}));
        let v2630=(v9*(if self.scalar_static_bool[49]{v13}else{(if self.scalar_static_bool[46]{(v660*(v139*((if self.scalar_static_bool[46]{(self.scalar_static_f64[88]*v2415)}else{v2226})+(self.scalar_static_f64[112]*(if self.scalar_static_bool[46]{((v651*(v2395*v2398))+(v647*(self.scalar_static_f64[110]*v2395)))}else{v13})))))}else{(if self.scalar_static_bool[43]{v13}else{(if self.scalar_static_bool[40]{(v545*(v139*v2226))}else{v13})})})}));
        let v2640=(-v142);
        let v2649=(if (self.scalar_static_f64[122]!=0.0){(v827*(if self.scalar_static_bool[28]{((-(v141*v1726))/v2114)}else{(if (self.scalar_static_f64[66]!=0.0){((-(v141*v999))/v2083)}else{v13})}))}else{v13});
        let v2650=(if (self.scalar_static_f64[122]!=0.0){(v827*(if self.scalar_static_bool[28]{((-(v141*v1727))/v2114)}else{(if (self.scalar_static_f64[66]!=0.0){((-(v141*v1000))/v2083)}else{v13})}))}else{v13});
        let v2651=(if (self.scalar_static_f64[122]!=0.0){(v827*(if self.scalar_static_bool[28]{((-(v141*v1728))/v2114)}else{(if (self.scalar_static_f64[66]!=0.0){((-(v141*v1001))/v2083)}else{v13})}))}else{v13});
        let v2652=(if (self.scalar_static_f64[122]!=0.0){(v827*(if self.scalar_static_bool[28]{((-(v141*v1729))/v2114)}else{(if (self.scalar_static_f64[66]!=0.0){((-(v141*v1002))/v2083)}else{v13})}))}else{v13});
        let v2653=(if (self.scalar_static_f64[122]!=0.0){(v827*(if self.scalar_static_bool[28]{((-(v141*v1730))/v2114)}else{(if (self.scalar_static_f64[66]!=0.0){((-(v141*v1003))/v2083)}else{v13})}))}else{v13});
        let v2659=(-v144);

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
            v44,
            v45,
            v47,
            v84,
            v120,
            v124,
            v131,
            v136,
            v142,
            v144,
            v149,
            v155,
            v170,
            v179,
            v184,
            v194,
            v202,
            v211,
            v246,
            v247,
            v249,
            v316,
            v339,
            v366,
            v453,
            v467,
            v577,
            v583,
            v676,
            v681,
            v719,
            v735,
            v786,
            v792,
            v802,
            v805,
            v820,
            v822,
            v824,
            v825,
            v828,
            v829,
            v836,
            v840,
            v842,
            v858,
            v864,
            v870,
            v875,
            v883,
            v884,
            v897,
            v921,
            v922,
            v942,
            v947,
            v949,
            v951,
            v953,
            v954,
            v999,
            v1000,
            v1001,
            v1002,
            v1003,
            v1100,
            v1101,
            v1102,
            v1103,
            v1104,
            v1105,
            v1106,
            v1115,
            v1116,
            v1117,
            v1118,
            v1454,
            v1455,
            v1456,
            v1457,
            v1458,
            v1554,
            v1555,
            v1556,
            v1557,
            v1726,
            v1727,
            v1728,
            v1729,
            v1730,
            v2322,
            v2380,
            v2531,
            v2532,
            v2533,
            v2534,
            v2582,
            v2583,
            v2584,
            v2585,
            v2615,
            v2617,
            v2618,
            v2626,
            v2628,
            v2630,
            v2640,
            v2649,
            v2650,
            v2651,
            v2652,
            v2653,
            v2659,
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
        let v50=(self.scalar_static_f64[11]*(common.v44+(self.scalar_static_f64[12]*common.v47)));
        let v137=(if common.v136{self.scalar_static_f64[13]}else{(if (common.v45!=0.0){(self.scalar_static_f64[13]*(common.v44+(common.v47*self.scalar_static_f64[14])))}else{common.v13})});
        let v138=(if common.v136{self.scalar_static_f64[15]}else{(if (common.v45!=0.0){(self.scalar_static_f64[15]*(common.v44+(common.v47*self.scalar_static_f64[16])))}else{common.v13})});
        let v143=(if common.v136{self.scalar_static_f64[37]}else{(if common.v131{(common.v84*self.scalar_static_f64[37])}else{(if common.v120{(self.scalar_static_f64[37]*common.v124)}else{common.v13})})});
        let v148=(if common.v136{self.scalar_static_f64[30]}else{(if (common.v45!=0.0){(self.scalar_static_f64[30]+(common.v39*self.scalar_static_f64[31]))}else{common.v13})});
        let v162=(if self.scalar_static_bool[11]{self.scalar_static_f64[44]}else{(if (self.scalar_static_f64[41]!=0.0){(self.scalar_static_f64[43]/(common.v35*8.617333262145179e-5))}else{common.v13})});
        let v221=(self.scalar_static_f64[56]+(self.scalar_static_f64[53]*common.v211));
        let v223=((common.v7*v221)).tanh();
        let v234=(v137*common.v211);
        let v235=(v223*v234);
        let v241=((common.v44+(common.v7*self.scalar_static_f64[62]))+(v138*scalar_limexp(common.v194)));
        let v251=(if self.scalar_static_bool[16]{(common.v247*common.v249)}else{common.v202});
        let v257=(if self.scalar_static_bool[16]{(((common.v179*common.v247)+(self.scalar_static_f64[55]*common.v249))+(common.v184*v251))}else{common.v13});
        let v258=(v257).tanh();
        let v260=(if self.scalar_static_bool[16]{(common.v44+v258)}else{common.v13});
        let v263=(if self.scalar_static_bool[16]{(self.scalar_static_f64[56]+(self.scalar_static_f64[53]*v260))}else{common.v13});
        let v266=(self.scalar_static_f64[62]+(common.v211*self.scalar_static_f64[63]));
        let v267=(if self.scalar_static_bool[16]{v266}else{common.v13});
        let v268=(common.v44+v223);
        let v269=(v234*v268);
        let v274=(self.scalar_static_f64[64]*(common.v7-common.v149));
        let v276=(v138*scalar_limexp(v274));
        let v277=((common.v44+(common.v7*v267))+v276);
        let v279=(if self.scalar_static_bool[16]{(v269*v277)}else{common.v13});
        let v282=(if self.scalar_static_bool[16]{(self.scalar_static_f64[62]+(v260*self.scalar_static_f64[63]))}else{common.v13});
        let v284=((common.v7*v263)).tanh();
        let v286=(v137*v260);
        let v287=(common.v44-(if self.scalar_static_bool[16]{v284}else{common.v13}));
        let v288=(v286*v287);
        let v290=(common.v44-(common.v7*v282));
        let v292=(if self.scalar_static_bool[16]{(v288*v290)}else{common.v13});
        let v319=(if self.scalar_static_bool[19]{(self.scalar_static_f64[56]+(self.scalar_static_f64[53]*common.v316))}else{common.v13});
        let v321=((common.v7*v319)).tanh();
        let v322=(if self.scalar_static_bool[19]{v321}else{common.v13});
        let v325=(if self.scalar_static_bool[19]{(self.scalar_static_f64[62]+(self.scalar_static_f64[63]*common.v316))}else{v267});
        let v326=(v137*common.v316);
        let v327=(v322*v326);
        let v330=(common.v194*self.scalar_static_f64[64]);
        let v333=((common.v44+(common.v7*v325))+(v138*scalar_limexp(v330)));
        let v349=(if self.scalar_static_bool[22]{common.v246}else{v251});
        let v351=(if self.scalar_static_bool[22]{(v349*v349)}else{common.v13});
        let v354=(common.v184*v349);
        let v356=((v349+(self.scalar_static_f64[55]*v351))+(v351*v354));
        let v358=(if self.scalar_static_bool[22]{(common.v179*v356)}else{v257});
        let v368=(-v358);
        let v372=((common.v155*(scalar_limexp(v358)-scalar_limexp(v368)))).tanh();
        let v374=(if self.scalar_static_bool[22]{(common.v44+v372)}else{common.v13});
        let v376=(self.scalar_static_f64[56]+(self.scalar_static_f64[53]*common.v366));
        let v377=(if self.scalar_static_bool[22]{v376}else{v319});
        let v380=(if self.scalar_static_bool[22]{(self.scalar_static_f64[56]+(self.scalar_static_f64[53]*v374))}else{common.v13});
        let v382=((common.v7*v377)).tanh();
        let v383=(if self.scalar_static_bool[22]{v382}else{v322});
        let v385=((common.v7*v380)).tanh();
        let v389=(if self.scalar_static_bool[22]{(self.scalar_static_f64[62]+(self.scalar_static_f64[63]*v374))}else{common.v13});
        let v392=(if self.scalar_static_bool[22]{(self.scalar_static_f64[62]+(self.scalar_static_f64[63]*common.v366))}else{common.v13});
        let v393=(v137*common.v366);
        let v394=(common.v44+v383);
        let v395=(v393*v394);
        let v398=(v276+(common.v44+(common.v7*v392)));
        let v401=(v137*v374);
        let v402=(common.v44-(if self.scalar_static_bool[22]{v385}else{common.v13}));
        let v403=(v401*v402);
        let v405=(common.v44-(common.v7*v389));
        let v414=(if self.scalar_static_bool[25]{v266}else{v325});
        let v415=(if self.scalar_static_bool[25]{v376}else{v377});
        let v417=((common.v7*v415)).tanh();
        let v420=((common.v11*v415)).tanh();
        let v424=((if self.scalar_static_bool[25]{v417}else{v383})+((if self.scalar_static_bool[25]{v420}else{common.v13})*self.scalar_static_f64[65]));
        let v425=(v234*v424);
        let v427=(common.v7+(common.v11*self.scalar_static_f64[65]));
        let v430=(v276+(common.v44+(v414*v427)));
        let v432=(if self.scalar_static_bool[25]{(v425*v430)}else{(if self.scalar_static_bool[22]{(common.v155*((if self.scalar_static_bool[22]{(v395*v398)}else{v279})-(if self.scalar_static_bool[22]{(v403*v405)}else{v292})))}else{(if self.scalar_static_bool[19]{(v327*v333)}else{(if self.scalar_static_bool[16]{(common.v155*(v279-v292))}else{(if (self.scalar_static_f64[57]!=0.0){(v235*v241)}else{common.v13})})})})});
        let v443=(common.v211*self.scalar_static_f64[69]);
        let v454=(common.v366*self.scalar_static_f64[69]);
        let v461=(common.v44+(common.v47*self.scalar_static_f64[71]));
        let v462=((if self.scalar_static_bool[28]{(self.scalar_static_f64[70]+v454)}else{(if (self.scalar_static_f64[66]!=0.0){(v443+self.scalar_static_f64[70])}else{common.v13})})*v461);
        let v463=((if self.scalar_static_bool[28]{(self.scalar_static_f64[68]+v454)}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[68]+v443)}else{common.v13})})*v461);
        let v473=(common.v9-v148);
        let v477=((-common.v9)-self.scalar_static_f64[74]);
        let v479=(common.v5-v148);
        let v482=(common.v6-self.scalar_static_f64[75]);
        let v488=(if self.scalar_static_bool[30]{scalar_limexp((v148*(-v162)))}else{(if (self.scalar_static_f64[73]!=0.0){scalar_limexp((v162*((-v148)).tanh()))}else{common.v339})});
        let v500=(v473).tanh();
        let v502=(v479).tanh();
        let v510=(self.scalar_static_f64[76]*(if self.scalar_static_bool[30]{v477}else{(if (self.scalar_static_f64[73]!=0.0){v477}else{common.v13})}));
        let v514=(v162*(if self.scalar_static_bool[34]{v473}else{(if self.scalar_static_bool[32]{v500}else{(if (self.scalar_static_f64[73]!=0.0){v473}else{common.v13})})}));
        let v522=(self.scalar_static_f64[85]*((scalar_limexp(v514)-((scalar_limexp(v510)-self.scalar_static_f64[80])*self.scalar_static_f64[87]))-v488));
        let v523=(self.scalar_static_f64[76]*(if self.scalar_static_bool[30]{v482}else{(if (self.scalar_static_f64[73]!=0.0){v482}else{common.v13})}));
        let v526=(v162*(if self.scalar_static_bool[34]{v479}else{(if self.scalar_static_bool[32]{v502}else{(if (self.scalar_static_f64[73]!=0.0){v479}else{common.v13})})}));
        let v631=common.v2322;
        let v633=common.v2380;
        let v736=common.v2534;
        let v737=(if self.scalar_static_bool[49]{v736}else{(if self.scalar_static_bool[46]{common.v676}else{(if self.scalar_static_bool[43]{v631}else{common.v577})})});
        let v738=common.v2584;
        let v739=(if self.scalar_static_bool[49]{v738}else{(if self.scalar_static_bool[46]{common.v681}else{(if self.scalar_static_bool[43]{v633}else{common.v583})})});
        let v790=(if self.scalar_static_bool[67]{((common.v44-(common.v786*common.v786))).sqrt()}else{common.v13});
        let v794=(if self.scalar_static_bool[67]{((-common.v786)*common.v792)}else{common.v13});
        let v806=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v735);
        let v808=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v719);
        let v812=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (common.v5*v739));
        let v815=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (common.v9*v737));
        let v830=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v829);
        let v837=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, common.v836);
        let v846=ctx.node_voltage(nodes[13]);
        let v859=ctx.branch_current(branches[11]);
        let v865=ctx.branch_current(branches[15]);
        let v876=(if self.scalar_static_bool[67]{common.v875}else{common.v13});
        let v877=ctx.node_voltage(nodes[18]);
        let v885=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, common.v884);
        let v898=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, common.v897);
        let v1052=(common.v44-(v223*v223));
        let v1058=(v137*common.v999);
        let v1059=(v137*common.v1000);
        let v1060=(v137*common.v1001);
        let v1061=(v137*common.v1002);
        let v1062=(v137*common.v1003);
        let v1063=(v234*((common.v7*(self.scalar_static_f64[53]*common.v999))*v1052));
        let v1066=(v234*((v221+(common.v7*(self.scalar_static_f64[53]*common.v1000)))*v1052));
        let v1069=(v234*(((-v221)+(common.v7*(self.scalar_static_f64[53]*common.v1001)))*v1052));
        let v1072=(v234*((common.v7*(self.scalar_static_f64[53]*common.v1002))*v1052));
        let v1075=(v234*((common.v7*(self.scalar_static_f64[53]*common.v1003))*v1052));
        let v1079=scalar_limexp_derivative(common.v194);
        let v1133=(if self.scalar_static_bool[16]{((common.v249*common.v1103)+(common.v247*common.v1115))}else{common.v947});
        let v1134=(if self.scalar_static_bool[16]{((common.v249*common.v1104)+(common.v247*common.v1116))}else{common.v949});
        let v1135=(if self.scalar_static_bool[16]{((common.v249*common.v1105)+(common.v247*common.v1117))}else{common.v951});
        let v1136=(if self.scalar_static_bool[16]{((common.v249*common.v1106)+(common.v247*common.v1118))}else{common.v953});
        let v1137=(if self.scalar_static_bool[16]{(common.v247*self.scalar_static_f64[158])}else{common.v954});
        let v1165=(if self.scalar_static_bool[16]{(((common.v179*common.v1103)+(self.scalar_static_f64[55]*common.v1115))+(common.v184*v1133))}else{common.v13});
        let v1166=(if self.scalar_static_bool[16]{((((common.v247*common.v921)+(common.v179*common.v1104))+(self.scalar_static_f64[55]*common.v1116))+(common.v184*v1134))}else{common.v13});
        let v1167=(if self.scalar_static_bool[16]{((((common.v247*common.v922)+(common.v179*common.v1105))+(self.scalar_static_f64[55]*common.v1117))+(common.v184*v1135))}else{common.v13});
        let v1168=(if self.scalar_static_bool[16]{(((common.v179*common.v1106)+(self.scalar_static_f64[55]*common.v1118))+(common.v184*v1136))}else{common.v13});
        let v1169=(if self.scalar_static_bool[16]{(self.scalar_static_f64[159]+(common.v184*v1137))}else{common.v13});
        let v1171=(common.v44-(v258*v258));
        let v1177=(if self.scalar_static_bool[16]{(v1165*v1171)}else{common.v13});
        let v1178=(if self.scalar_static_bool[16]{(v1166*v1171)}else{common.v13});
        let v1179=(if self.scalar_static_bool[16]{(v1167*v1171)}else{common.v13});
        let v1180=(if self.scalar_static_bool[16]{(v1168*v1171)}else{common.v13});
        let v1181=(if self.scalar_static_bool[16]{(v1169*v1171)}else{common.v13});
        let v1192=(self.scalar_static_f64[63]*common.v999);
        let v1193=(self.scalar_static_f64[63]*common.v1000);
        let v1194=(self.scalar_static_f64[63]*common.v1001);
        let v1195=(self.scalar_static_f64[63]*common.v1002);
        let v1196=(self.scalar_static_f64[63]*common.v1003);
        let v1197=(if self.scalar_static_bool[16]{v1192}else{common.v13});
        let v1198=(if self.scalar_static_bool[16]{v1193}else{common.v13});
        let v1199=(if self.scalar_static_bool[16]{v1194}else{common.v13});
        let v1200=(if self.scalar_static_bool[16]{v1195}else{common.v13});
        let v1201=(if self.scalar_static_bool[16]{v1196}else{common.v13});
        let v1221=scalar_limexp_derivative(v274);
        let v1224=(v138*(self.scalar_static_f64[64]*v1221));
        let v1225=(v138*(self.scalar_static_f64[160]*v1221));
        let v1243=(if self.scalar_static_bool[16]{((v277*(v1063+(v268*v1058)))+(v269*(common.v7*v1197)))}else{common.v13});
        let v1244=(if self.scalar_static_bool[16]{((v277*(v1066+(v268*v1059)))+(v269*((v267+(common.v7*v1198))+v1224)))}else{common.v13});
        let v1245=(if self.scalar_static_bool[16]{((v277*(v1069+(v268*v1060)))+(v269*(((-v267)+(common.v7*v1199))+v1225)))}else{common.v13});
        let v1246=(if self.scalar_static_bool[16]{((v277*(v1072+(v268*v1061)))+(v269*(common.v7*v1200)))}else{common.v13});
        let v1247=(if self.scalar_static_bool[16]{((v277*(v1075+(v268*v1062)))+(v269*(common.v7*v1201)))}else{common.v13});
        let v1267=(common.v44-(v284*v284));
        let v1331=(if self.scalar_static_bool[16]{((v290*((v287*(v137*v1177))+(v286*(-(if self.scalar_static_bool[16]{((common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[53]*v1177)}else{common.v13}))*v1267)}else{common.v13})))))+(v288*(-(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[63]*v1177)}else{common.v13})))))}else{common.v13});
        let v1332=(if self.scalar_static_bool[16]{((v290*((v287*(v137*v1178))+(v286*(-(if self.scalar_static_bool[16]{((v263+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[53]*v1178)}else{common.v13})))*v1267)}else{common.v13})))))+(v288*(-(v282+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[63]*v1178)}else{common.v13}))))))}else{common.v13});
        let v1333=(if self.scalar_static_bool[16]{((v290*((v287*(v137*v1179))+(v286*(-(if self.scalar_static_bool[16]{(((-v263)+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[53]*v1179)}else{common.v13})))*v1267)}else{common.v13})))))+(v288*(-((-v282)+(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[63]*v1179)}else{common.v13}))))))}else{common.v13});
        let v1334=(if self.scalar_static_bool[16]{((v290*((v287*(v137*v1180))+(v286*(-(if self.scalar_static_bool[16]{((common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[53]*v1180)}else{common.v13}))*v1267)}else{common.v13})))))+(v288*(-(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[63]*v1180)}else{common.v13})))))}else{common.v13});
        let v1335=(if self.scalar_static_bool[16]{((v290*((v287*(v137*v1181))+(v286*(-(if self.scalar_static_bool[16]{((common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[53]*v1181)}else{common.v13}))*v1267)}else{common.v13})))))+(v288*(-(common.v7*(if self.scalar_static_bool[16]{(self.scalar_static_f64[63]*v1181)}else{common.v13})))))}else{common.v13});
        let v1464=(if self.scalar_static_bool[19]{(self.scalar_static_f64[53]*common.v1454)}else{common.v13});
        let v1465=(if self.scalar_static_bool[19]{(self.scalar_static_f64[53]*common.v1455)}else{common.v13});
        let v1466=(if self.scalar_static_bool[19]{(self.scalar_static_f64[53]*common.v1456)}else{common.v13});
        let v1467=(if self.scalar_static_bool[19]{(self.scalar_static_f64[53]*common.v1457)}else{common.v13});
        let v1468=(if self.scalar_static_bool[19]{(self.scalar_static_f64[53]*common.v1458)}else{common.v13});
        let v1478=(common.v44-(v321*v321));
        let v1484=(if self.scalar_static_bool[19]{((common.v7*v1464)*v1478)}else{common.v13});
        let v1485=(if self.scalar_static_bool[19]{((v319+(common.v7*v1465))*v1478)}else{common.v13});
        let v1486=(if self.scalar_static_bool[19]{(((-v319)+(common.v7*v1466))*v1478)}else{common.v13});
        let v1487=(if self.scalar_static_bool[19]{((common.v7*v1467)*v1478)}else{common.v13});
        let v1488=(if self.scalar_static_bool[19]{((common.v7*v1468)*v1478)}else{common.v13});
        let v1494=(if self.scalar_static_bool[19]{(self.scalar_static_f64[63]*common.v1454)}else{v1197});
        let v1495=(if self.scalar_static_bool[19]{(self.scalar_static_f64[63]*common.v1455)}else{v1198});
        let v1496=(if self.scalar_static_bool[19]{(self.scalar_static_f64[63]*common.v1456)}else{v1199});
        let v1497=(if self.scalar_static_bool[19]{(self.scalar_static_f64[63]*common.v1457)}else{v1200});
        let v1498=(if self.scalar_static_bool[19]{(self.scalar_static_f64[63]*common.v1458)}else{v1201});
        let v1527=scalar_limexp_derivative(v330);
        let v1623=(if self.scalar_static_bool[22]{common.v942}else{v1133});
        let v1624=(if self.scalar_static_bool[22]{common.v1100}else{v1134});
        let v1625=(if self.scalar_static_bool[22]{common.v1101}else{v1135});
        let v1626=(if self.scalar_static_bool[22]{common.v1102}else{v1136});
        let v1627=(if self.scalar_static_bool[22]{common.v13}else{v1137});
        let v1628=(v349*v1623);
        let v1630=(v349*v1624);
        let v1632=(v349*v1625);
        let v1634=(v349*v1626);
        let v1636=(v349*v1627);
        let v1638=(if self.scalar_static_bool[22]{(v1628+v1628)}else{common.v13});
        let v1639=(if self.scalar_static_bool[22]{(v1630+v1630)}else{common.v13});
        let v1640=(if self.scalar_static_bool[22]{(v1632+v1632)}else{common.v13});
        let v1641=(if self.scalar_static_bool[22]{(v1634+v1634)}else{common.v13});
        let v1642=(if self.scalar_static_bool[22]{(v1636+v1636)}else{common.v13});
        let v1687=(if self.scalar_static_bool[22]{(common.v179*((v1623+(self.scalar_static_f64[55]*v1638))+((v354*v1638)+(v351*(common.v184*v1623)))))}else{v1165});
        let v1688=(if self.scalar_static_bool[22]{((v356*common.v921)+(common.v179*((v1624+(self.scalar_static_f64[55]*v1639))+((v354*v1639)+(v351*(common.v184*v1624))))))}else{v1166});
        let v1689=(if self.scalar_static_bool[22]{((v356*common.v922)+(common.v179*((v1625+(self.scalar_static_f64[55]*v1640))+((v354*v1640)+(v351*(common.v184*v1625))))))}else{v1167});
        let v1690=(if self.scalar_static_bool[22]{(common.v179*((v1626+(self.scalar_static_f64[55]*v1641))+((v354*v1641)+(v351*(common.v184*v1626)))))}else{v1168});
        let v1691=(if self.scalar_static_bool[22]{(common.v179*((v1627+(self.scalar_static_f64[55]*v1642))+((v354*v1642)+(v351*(common.v184*v1627)))))}else{v1169});
        let v1731=scalar_limexp_derivative(v358);
        let v1742=scalar_limexp_derivative(v368);
        let v1759=(common.v44-(v372*v372));
        let v1765=(if self.scalar_static_bool[22]{((common.v155*((v1687*v1731)-((-v1687)*v1742)))*v1759)}else{common.v13});
        let v1766=(if self.scalar_static_bool[22]{((common.v155*((v1688*v1731)-((-v1688)*v1742)))*v1759)}else{common.v13});
        let v1767=(if self.scalar_static_bool[22]{((common.v155*((v1689*v1731)-((-v1689)*v1742)))*v1759)}else{common.v13});
        let v1768=(if self.scalar_static_bool[22]{((common.v155*((v1690*v1731)-((-v1690)*v1742)))*v1759)}else{common.v13});
        let v1769=(if self.scalar_static_bool[22]{((common.v155*((v1691*v1731)-((-v1691)*v1742)))*v1759)}else{common.v13});
        let v1770=(self.scalar_static_f64[53]*common.v1726);
        let v1771=(self.scalar_static_f64[53]*common.v1727);
        let v1772=(self.scalar_static_f64[53]*common.v1728);
        let v1773=(self.scalar_static_f64[53]*common.v1729);
        let v1774=(self.scalar_static_f64[53]*common.v1730);
        let v1775=(if self.scalar_static_bool[22]{v1770}else{v1464});
        let v1776=(if self.scalar_static_bool[22]{v1771}else{v1465});
        let v1777=(if self.scalar_static_bool[22]{v1772}else{v1466});
        let v1778=(if self.scalar_static_bool[22]{v1773}else{v1467});
        let v1779=(if self.scalar_static_bool[22]{v1774}else{v1468});
        let v1799=(common.v44-(v382*v382));
        let v1805=(if self.scalar_static_bool[22]{((common.v7*v1775)*v1799)}else{v1484});
        let v1806=(if self.scalar_static_bool[22]{((v377+(common.v7*v1776))*v1799)}else{v1485});
        let v1807=(if self.scalar_static_bool[22]{(((-v377)+(common.v7*v1777))*v1799)}else{v1486});
        let v1808=(if self.scalar_static_bool[22]{((common.v7*v1778)*v1799)}else{v1487});
        let v1809=(if self.scalar_static_bool[22]{((common.v7*v1779)*v1799)}else{v1488});
        let v1819=(common.v44-(v385*v385));
        let v1969=(if self.scalar_static_bool[22]{(common.v155*((if self.scalar_static_bool[22]{((v398*((v394*(v137*common.v1727))+(v393*v1806)))+(v395*(v1224+(v392+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*common.v1727)}else{common.v13}))))))}else{v1244})-(if self.scalar_static_bool[22]{((v405*((v402*(v137*v1766))+(v401*(-(if self.scalar_static_bool[22]{((v380+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[53]*v1766)}else{common.v13})))*v1819)}else{common.v13})))))+(v403*(-(v389+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*v1766)}else{common.v13}))))))}else{v1332})))}else{(if self.scalar_static_bool[19]{((v333*((v326*v1485)+(v322*(v137*common.v1455))))+(v327*((v325+(common.v7*v1495))+(v138*(self.scalar_static_f64[64]*v1527)))))}else{(if self.scalar_static_bool[16]{(common.v155*(v1244-v1332))}else{(if (self.scalar_static_f64[57]!=0.0){((v241*(v1066+(v223*v1059)))+(v235*(self.scalar_static_f64[62]+(v138*v1079))))}else{common.v13})})})});
        let v1978=(if self.scalar_static_bool[25]{v1770}else{v1775});
        let v1979=(if self.scalar_static_bool[25]{v1771}else{v1776});
        let v1980=(if self.scalar_static_bool[25]{v1772}else{v1777});
        let v1981=(if self.scalar_static_bool[25]{v1773}else{v1778});
        let v1982=(if self.scalar_static_bool[25]{v1774}else{v1779});
        let v1986=(-v415);
        let v1992=(common.v44-(v417*v417));
        let v2011=(common.v44-(v420*v420));
        let v2076=(if self.scalar_static_bool[25]{((v430*((v424*v1058)+(v234*((if self.scalar_static_bool[25]{((common.v7*v1978)*v1992)}else{v1805})+(self.scalar_static_f64[65]*(if self.scalar_static_bool[25]{((v415+(common.v11*v1978))*v2011)}else{common.v13}))))))+(v425*((v427*(if self.scalar_static_bool[25]{v1192}else{v1494}))+(v414*self.scalar_static_f64[65]))))}else{(if self.scalar_static_bool[22]{(common.v155*((if self.scalar_static_bool[22]{((v398*((v394*(v137*common.v1726))+(v393*v1805)))+(v395*(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*common.v1726)}else{common.v13}))))}else{v1243})-(if self.scalar_static_bool[22]{((v405*((v402*(v137*v1765))+(v401*(-(if self.scalar_static_bool[22]{((common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[53]*v1765)}else{common.v13}))*v1819)}else{common.v13})))))+(v403*(-(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*v1765)}else{common.v13})))))}else{v1331})))}else{(if self.scalar_static_bool[19]{((v333*((v326*v1484)+(v322*(v137*common.v1454))))+(v327*(common.v7*v1494)))}else{(if self.scalar_static_bool[16]{(common.v155*(v1243-v1331))}else{(if (self.scalar_static_f64[57]!=0.0){(v241*(v1063+(v223*v1058)))}else{common.v13})})})})});
        let v2078=(if self.scalar_static_bool[25]{((v430*((v424*v1060)+(v234*((if self.scalar_static_bool[25]{((v1986+(common.v7*v1980))*v1992)}else{v1807})+(self.scalar_static_f64[65]*(if self.scalar_static_bool[25]{((v1986+(common.v11*v1980))*v2011)}else{common.v13}))))))+(v425*(v1225+((v427*(if self.scalar_static_bool[25]{v1194}else{v1496}))+(v414*self.scalar_static_f64[164])))))}else{(if self.scalar_static_bool[22]{(common.v155*((if self.scalar_static_bool[22]{((v398*((v394*(v137*common.v1728))+(v393*v1807)))+(v395*(v1225+((-v392)+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*common.v1728)}else{common.v13}))))))}else{v1245})-(if self.scalar_static_bool[22]{((v405*((v402*(v137*v1767))+(v401*(-(if self.scalar_static_bool[22]{(((-v380)+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[53]*v1767)}else{common.v13})))*v1819)}else{common.v13})))))+(v403*(-((-v389)+(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*v1767)}else{common.v13}))))))}else{v1333})))}else{(if self.scalar_static_bool[19]{((v333*((v326*v1486)+(v322*(v137*common.v1456))))+(v327*((-v325)+(common.v7*v1496))))}else{(if self.scalar_static_bool[16]{(common.v155*(v1245-v1333))}else{(if (self.scalar_static_f64[57]!=0.0){((v241*(v1069+(v223*v1060)))+(v235*self.scalar_static_f64[157]))}else{common.v13})})})})});
        let v2079=(if self.scalar_static_bool[25]{((v430*((v424*v1061)+(v234*((if self.scalar_static_bool[25]{((common.v7*v1981)*v1992)}else{v1808})+(self.scalar_static_f64[65]*(if self.scalar_static_bool[25]{((common.v11*v1981)*v2011)}else{common.v13}))))))+(v425*(v427*(if self.scalar_static_bool[25]{v1195}else{v1497}))))}else{(if self.scalar_static_bool[22]{(common.v155*((if self.scalar_static_bool[22]{((v398*((v394*(v137*common.v1729))+(v393*v1808)))+(v395*(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*common.v1729)}else{common.v13}))))}else{v1246})-(if self.scalar_static_bool[22]{((v405*((v402*(v137*v1768))+(v401*(-(if self.scalar_static_bool[22]{((common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[53]*v1768)}else{common.v13}))*v1819)}else{common.v13})))))+(v403*(-(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*v1768)}else{common.v13})))))}else{v1334})))}else{(if self.scalar_static_bool[19]{((v333*((v326*v1487)+(v322*(v137*common.v1457))))+(v327*((common.v7*v1497)+(v138*(self.scalar_static_f64[160]*v1527)))))}else{(if self.scalar_static_bool[16]{(common.v155*(v1246-v1334))}else{(if (self.scalar_static_f64[57]!=0.0){((v241*(v1072+(v223*v1061)))+(v235*(v138*(-v1079))))}else{common.v13})})})})});
        let v2080=(if self.scalar_static_bool[25]{((v430*((v424*v1062)+(v234*((if self.scalar_static_bool[25]{((common.v7*v1982)*v1992)}else{v1809})+(self.scalar_static_f64[65]*(if self.scalar_static_bool[25]{((common.v11*v1982)*v2011)}else{common.v13}))))))+(v425*(v427*(if self.scalar_static_bool[25]{v1196}else{v1498}))))}else{(if self.scalar_static_bool[22]{(common.v155*((if self.scalar_static_bool[22]{((v398*((v394*(v137*common.v1730))+(v393*v1809)))+(v395*(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*common.v1730)}else{common.v13}))))}else{v1247})-(if self.scalar_static_bool[22]{((v405*((v402*(v137*v1769))+(v401*(-(if self.scalar_static_bool[22]{((common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[53]*v1769)}else{common.v13}))*v1819)}else{common.v13})))))+(v403*(-(common.v7*(if self.scalar_static_bool[22]{(self.scalar_static_f64[63]*v1769)}else{common.v13})))))}else{v1335})))}else{(if self.scalar_static_bool[19]{((v333*((v326*v1488)+(v322*(v137*common.v1458))))+(v327*(common.v7*v1498)))}else{(if self.scalar_static_bool[16]{(common.v155*(v1247-v1335))}else{(if (self.scalar_static_f64[57]!=0.0){(v241*(v1075+(v223*v1062)))}else{common.v13})})})})});
        let v2143=(v461*(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]*common.v1726)}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[69]*common.v999)}else{common.v13})}));
        let v2144=(v461*(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]*common.v1727)}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[69]*common.v1000)}else{common.v13})}));
        let v2145=(v461*(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]*common.v1728)}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[69]*common.v1001)}else{common.v13})}));
        let v2146=(v461*(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]*common.v1729)}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[69]*common.v1002)}else{common.v13})}));
        let v2147=(v461*(if self.scalar_static_bool[28]{(self.scalar_static_f64[69]*common.v1730)}else{(if (self.scalar_static_f64[66]!=0.0){(self.scalar_static_f64[69]*common.v1003)}else{common.v13})}));
        let v2156=(if self.scalar_static_bool[30]{common.v13}else{(if (self.scalar_static_f64[73]!=0.0){common.v13}else{common.v1555})});
        let v2157=(if self.scalar_static_bool[30]{common.v13}else{(if (self.scalar_static_f64[73]!=0.0){common.v13}else{common.v1556})});
        let v2158=(if self.scalar_static_bool[30]{common.v13}else{(if (self.scalar_static_f64[73]!=0.0){common.v13}else{common.v1557})});
        let v2161=(common.v44-(v500*v500));
        let v2166=(common.v44-(v502*v502));
        let v2178=scalar_limexp_derivative(v510);
        let v2183=scalar_limexp_derivative(v514);
        let v2195=(self.scalar_static_f64[85]*(-(if self.scalar_static_bool[30]{common.v13}else{(if (self.scalar_static_f64[73]!=0.0){common.v13}else{common.v1554})})));
        let v2201=scalar_limexp_derivative(v523);
        let v2206=scalar_limexp_derivative(v526);
        let v2597=ddt_scale;
        let v2641=-1e-12;

        stamper.stamp_current_sparse_local::<5, 0>(
            Some(15),
            None,
            multiplicity * ((-v432)),
            [4, 5, 8, 10, 12],
            [(-v2076), (-(if self.scalar_static_bool[25]{((v430*((v424*v1059)+(v234*((if self.scalar_static_bool[25]{((v415+(common.v7*v1979))*v1992)}else{v1806})+(self.scalar_static_f64[65]*(if self.scalar_static_bool[25]{((common.v11*v1979)*v2011)}else{common.v13}))))))+(v425*(v1224+(v414+(v427*(if self.scalar_static_bool[25]{v1193}else{v1495}))))))}else{v1969})), (-v2078), (-v2079), (-v2080)],
            [],
            [],
            multiplicity,
        );
        let v802_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v802);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (v802_ddt),
            15,
            multiplicity * (((self.scalar_static_f64[144]) * ddt_scale)),
        );
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (v12),
            16,
            multiplicity * (common.v44),
        );
        let v805_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v805);
        stamper.stamp_potential_branch_local(
            Some(15),
            Some(16),
            0,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            0,
            v805_ddt,
            0,
            ((self.scalar_static_f64[145]) * ddt_scale),
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
            multiplicity * (v522),
            [4, 5, 8, 10, 11, 12],
            [v2195, (self.scalar_static_f64[85]*(-v2156)), (self.scalar_static_f64[85]*((((v162*(if self.scalar_static_bool[34]{common.v467}else{(if self.scalar_static_bool[32]{(-v2161)}else{self.scalar_static_f64[166]})}))*v2183)-(self.scalar_static_f64[87]*(self.scalar_static_f64[171]*v2178)))-v2157)), (self.scalar_static_f64[85]*(-v2158)), (self.scalar_static_f64[85]*(((v162*(if self.scalar_static_bool[34]{common.v44}else{(if self.scalar_static_bool[32]{v2161}else{self.scalar_static_f64[167]})}))*v2183)-(self.scalar_static_f64[87]*(self.scalar_static_f64[172]*v2178)))), self.scalar_static_f64[174]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * ((self.scalar_static_f64[85]*((scalar_limexp(v526)-(self.scalar_static_f64[87]*(scalar_limexp(v523)-self.scalar_static_f64[83])))-v488))),
            [4, 5, 8, 10, 12],
            [v2195, (self.scalar_static_f64[85]*((((v162*(if self.scalar_static_bool[34]{common.v467}else{(if self.scalar_static_bool[32]{(-v2166)}else{self.scalar_static_f64[166]})}))*v2206)-(self.scalar_static_f64[87]*(self.scalar_static_f64[171]*v2201)))-v2156)), (self.scalar_static_f64[85]*(-v2157)), (self.scalar_static_f64[85]*((((v162*(if self.scalar_static_bool[34]{common.v44}else{(if self.scalar_static_bool[32]{v2166}else{self.scalar_static_f64[167]})}))*v2206)-(self.scalar_static_f64[87]*(self.scalar_static_f64[172]*v2201)))-v2158)), self.scalar_static_f64[174]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(5),
            multiplicity * ((if (self.scalar_static_f64[120]!=0.0){v806}else{common.v13})),
            [5, 8, 10, 11],
            [(if (self.scalar_static_f64[120]!=0.0){(common.v2582*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2583*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2584*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2585*v2597)}else{common.v13})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(8),
            multiplicity * ((if (self.scalar_static_f64[120]!=0.0){v808}else{common.v13})),
            [5, 8, 10, 11],
            [(if (self.scalar_static_f64[120]!=0.0){(common.v2531*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2532*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2533*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2534*v2597)}else{common.v13})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(10),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[69]{v812}else{common.v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*((-v739)+common.v2615))}else{common.v13})),
            8,
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*common.v2617)}else{common.v13})),
            10,
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*(v739+common.v2618))}else{common.v13})),
        );
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[69]{v815}else{common.v13})),
            5,
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*common.v2626)}else{common.v13})),
            8,
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*((-v737)+common.v2628))}else{common.v13})),
            11,
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*(v737+common.v2630))}else{common.v13})),
        );
        let v820_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v820);
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (v820_ddt),
            5,
            multiplicity * (((self.scalar_static_f64[199]) * ddt_scale)),
            7,
            multiplicity * (((self.scalar_static_f64[146]) * ddt_scale)),
        );
        let v822_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v822);
        stamper.stamp_current_node2_local(
            Some(5),
            Some(8),
            multiplicity * (v822_ddt),
            5,
            multiplicity * (((self.scalar_static_f64[147]) * ddt_scale)),
            8,
            multiplicity * (((self.scalar_static_f64[200]) * ddt_scale)),
        );
        let v825_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v825);
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * (v825_ddt),
            4,
            multiplicity * (((common.v2640) * ddt_scale)),
            6,
            multiplicity * (((common.v142) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(6),
            Some(4),
            multiplicity * ((common.v170*common.v824)),
            4,
            multiplicity * (v2641),
            6,
            multiplicity * (common.v170),
        );
        stamper.stamp_potential_branch_local(
            Some(4),
            Some(8),
            1,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            1,
            (if (self.scalar_static_f64[122]!=0.0){(common.v828+v830)}else{common.v13}),
            [4, 5, 8, 10, 12],
            [common.v2649, common.v2650, common.v2651, common.v2652, common.v2653],
            [1],
            [(if (self.scalar_static_f64[122]!=0.0){(common.v453+(self.scalar_static_f64[121]*v2597))}else{common.v13})],
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
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){((common.v8-common.v0)/v143)}else{common.v13})),
            11,
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){(common.v44/v143)}else{common.v13})),
            12,
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){(common.v467/v143)}else{common.v13})),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(8),
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){v837}else{common.v13})),
            8,
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){(v2597*common.v2659)}else{common.v13})),
            12,
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){(common.v144*v2597)}else{common.v13})),
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
        let v842_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, common.v842);
        stamper.stamp_current_node2_local(
            Some(11),
            Some(14),
            multiplicity * (v842_ddt),
            11,
            multiplicity * (((self.scalar_static_f64[148]) * ddt_scale)),
            14,
            multiplicity * (((self.scalar_static_f64[201]) * ddt_scale)),
        );
        stamper.stamp_current_node2_local(
            Some(14),
            Some(8),
            multiplicity * ((if (self.scalar_static_f64[125]!=0.0){((common.v840-common.v1)/self.scalar_static_f64[124])}else{common.v13})),
            8,
            multiplicity * (self.scalar_static_f64[204]),
            14,
            multiplicity * (self.scalar_static_f64[205]),
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
            multiplicity * ((if (self.scalar_static_f64[127]!=0.0){((v846-common.v3)/self.scalar_static_f64[126])}else{common.v13})),
            10,
            multiplicity * (self.scalar_static_f64[208]),
            13,
            multiplicity * (self.scalar_static_f64[209]),
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
            multiplicity * ((if (self.scalar_static_f64[129]!=0.0){((v846-common.v8)/self.scalar_static_f64[128])}else{common.v13})),
            11,
            multiplicity * (self.scalar_static_f64[212]),
            13,
            multiplicity * (self.scalar_static_f64[213]),
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
            (if (self.scalar_static_f64[131]!=0.0){(self.scalar_static_f64[130]*ctx.branch_current(branches[7]))}else{common.v13}),
            7,
            self.scalar_static_f64[214],
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
        let v858_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, common.v858);
        stamper.stamp_potential_branch_local(
            Some(1),
            Some(7),
            10,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            10,
            v858_ddt,
            10,
            ((self.scalar_static_f64[149]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            11,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            11,
            (if (self.scalar_static_f64[132]!=0.0){(v462*v859)}else{common.v13}),
            [4, 5, 8, 10, 12],
            [(if (self.scalar_static_f64[132]!=0.0){(v859*v2143)}else{common.v13}), (if (self.scalar_static_f64[132]!=0.0){(v859*v2144)}else{common.v13}), (if (self.scalar_static_f64[132]!=0.0){(v859*v2145)}else{common.v13}), (if (self.scalar_static_f64[132]!=0.0){(v859*v2146)}else{common.v13}), (if (self.scalar_static_f64[132]!=0.0){(v859*v2147)}else{common.v13})],
            [11],
            [(if (self.scalar_static_f64[132]!=0.0){v462}else{common.v13})],
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
        let v864_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, common.v864);
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(2),
            14,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            14,
            v864_ddt,
            14,
            ((self.scalar_static_f64[150]) * ddt_scale),
        );
        stamper.stamp_potential_branch_local(
            Some(5),
            Some(6),
            15,
            multiplicity,
        );
        stamper.stamp_potential_sparse_local::<5, 1>(
            15,
            (if (self.scalar_static_f64[133]!=0.0){(v463*v865)}else{common.v13}),
            [4, 5, 8, 10, 12],
            [(if (self.scalar_static_f64[133]!=0.0){(v865*v2143)}else{common.v13}), (if (self.scalar_static_f64[133]!=0.0){(v865*v2144)}else{common.v13}), (if (self.scalar_static_f64[133]!=0.0){(v865*v2145)}else{common.v13}), (if (self.scalar_static_f64[133]!=0.0){(v865*v2146)}else{common.v13}), (if (self.scalar_static_f64[133]!=0.0){(v865*v2147)}else{common.v13})],
            [15],
            [(if (self.scalar_static_f64[133]!=0.0){v463}else{common.v13})],
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
        let v870_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, common.v870);
        stamper.stamp_potential_branch_local(
            Some(6),
            Some(0),
            18,
            multiplicity,
        );
        stamper.stamp_potential_branch1_local(
            18,
            v870_ddt,
            18,
            ((self.scalar_static_f64[151]) * ddt_scale),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (1e-15),
        );
        stamper.stamp_current_const_local(
            Some(14),
            Some(2),
            multiplicity * (common.v170),
        );
        stamper.stamp_current_node2_local(
            Some(12),
            Some(2),
            multiplicity * ((common.v170*(common.v0-ctx.node_voltage(nodes[2])))),
            2,
            multiplicity * (v2641),
            12,
            multiplicity * (common.v170),
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
            multiplicity * (v876),
            17,
            multiplicity * (self.scalar_static_f64[215]),
        );
        stamper.stamp_current_const_local(
            Some(18),
            None,
            multiplicity * (common.v13),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * ((if self.scalar_static_bool[67]{v877}else{common.v13})),
            18,
            multiplicity * (self.scalar_static_f64[215]),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(8),
            multiplicity * (v876),
            17,
            multiplicity * (self.scalar_static_f64[215]),
        );
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[67]{((v794*common.v875)+(v790*v877))}else{common.v13})),
            17,
            multiplicity * ((if self.scalar_static_bool[67]{v794}else{common.v13})),
            18,
            multiplicity * ((if self.scalar_static_bool[67]{v790}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((if self.scalar_static_bool[67]{v885}else{common.v13})),
            17,
            multiplicity * ((if self.scalar_static_bool[67]{(common.v883*v2597)}else{common.v13})),
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
            multiplicity * (common.v875),
            17,
            multiplicity * (common.v44),
        );
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (v877),
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
            multiplicity * ((if (self.scalar_static_f64[143]!=0.0){(-(((common.v7*v432)).abs()+((common.v9*v522)).abs()))}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (self.scalar_static_f64[143]!=0.0){(common.v32/v50)}else{common.v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[143]!=0.0){(common.v44/v50)}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if (self.scalar_static_f64[143]!=0.0){v898}else{common.v13})),
            3,
            multiplicity * ((if (self.scalar_static_f64[143]!=0.0){(self.scalar_static_f64[152]*v2597)}else{common.v13})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[70]{(common.v32*common.v170)}else{common.v13})),
            3,
            multiplicity * (self.scalar_static_f64[216]),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v631=common.v2322;
        let v633=common.v2380;
        let v736=common.v2534;
        let v737=(if self.scalar_static_bool[49]{v736}else{(if self.scalar_static_bool[46]{common.v676}else{(if self.scalar_static_bool[43]{v631}else{common.v577})})});
        let v738=common.v2584;
        let v739=(if self.scalar_static_bool[49]{v738}else{(if self.scalar_static_bool[46]{common.v681}else{(if self.scalar_static_bool[43]{v633}else{common.v583})})});
        let v806=0.0;
        let v808=0.0;
        let v812=0.0;
        let v815=0.0;
        let v830=0.0;
        let v837=0.0;
        let v885=0.0;
        let v898=0.0;
        let v2597=1.0;

        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (self.scalar_static_f64[144]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[15]),
            Some(nodes[16]),
            branches[0],
            multiplicity * (self.scalar_static_f64[145]),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if (self.scalar_static_f64[120]!=0.0){(common.v2582*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2583*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2584*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2585*v2597)}else{common.v13})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            &[nodes[5], nodes[8], nodes[10], nodes[11]],
            &[(if (self.scalar_static_f64[120]!=0.0){(common.v2531*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2532*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2533*v2597)}else{common.v13}), (if (self.scalar_static_f64[120]!=0.0){(common.v2534*v2597)}else{common.v13})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*((-v739)+common.v2615))}else{common.v13})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*common.v2617)}else{common.v13})),
            nodes[10],
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*(v739+common.v2618))}else{common.v13})),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*common.v2626)}else{common.v13})),
            nodes[8],
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*((-v737)+common.v2628))}else{common.v13})),
            nodes[11],
            multiplicity * ((if self.scalar_static_bool[69]{(v2597*(v737+common.v2630))}else{common.v13})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[199]),
            nodes[7],
            multiplicity * (self.scalar_static_f64[146]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes[5],
            multiplicity * (self.scalar_static_f64[147]),
            nodes[8],
            multiplicity * (self.scalar_static_f64[200]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (common.v2640),
            nodes[6],
            multiplicity * (common.v142),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[8], nodes[10], nodes[12]],
            &[common.v2649, common.v2650, common.v2651, common.v2652, common.v2653],
            &[branches[1]],
            &[(if (self.scalar_static_f64[122]!=0.0){(common.v453+(self.scalar_static_f64[121]*v2597))}else{common.v13})],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[8],
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){(v2597*common.v2659)}else{common.v13})),
            nodes[12],
            multiplicity * ((if (self.scalar_static_f64[123]!=0.0){(common.v144*v2597)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[11]),
            Some(nodes[14]),
            nodes[11],
            multiplicity * (self.scalar_static_f64[148]),
            nodes[14],
            multiplicity * (self.scalar_static_f64[201]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[1]),
            Some(nodes[7]),
            branches[10],
            multiplicity * (self.scalar_static_f64[149]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[9]),
            Some(nodes[2]),
            branches[14],
            multiplicity * (self.scalar_static_f64[150]),
        );
        stamper.stamp_current_reactive_branch1(
            Some(nodes[6]),
            Some(nodes[0]),
            branches[18],
            multiplicity * (self.scalar_static_f64[151]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[17],
            multiplicity * ((if self.scalar_static_bool[67]{(common.v883*v2597)}else{common.v13})),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((if (self.scalar_static_f64[143]!=0.0){(self.scalar_static_f64[152]*v2597)}else{common.v13})),
        );
    }
}
