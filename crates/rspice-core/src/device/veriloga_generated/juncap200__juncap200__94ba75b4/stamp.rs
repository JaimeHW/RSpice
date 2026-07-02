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
    v5: f64,
    v60: f64,
    v407: f64,
    v408: f64,
    v4925: f64,
    v4926: f64,
    v4969: f64,
    v5134: f64,
    v5136: f64,
    v5168: f64,
    v5192: f64,
    v5200: f64,
    v5224: f64,
    v5251: f64,
    v5265: f64,
    v5279: f64,
    v5283: f64,
    v5290: bool,
    v5312: f64,
    v5339: f64,
    v5363: f64,
    v5397: f64,
    v5406: f64,
    v5408: bool,
    v5418: f64,
    v5459: f64,
    v5484: f64,
    v5512: f64,
    v5526: f64,
    v5540: f64,
    v5544: f64,
    v5551: bool,
    v5573: f64,
    v5600: f64,
    v5626: f64,
    v5660: f64,
    v5669: f64,
    v5671: bool,
    v5681: f64,
    v5720: f64,
    v5745: f64,
    v5773: f64,
    v5787: f64,
    v5801: f64,
    v5805: f64,
    v5812: bool,
    v5834: f64,
    v5861: f64,
    v5887: f64,
    v5921: f64,
    v5930: f64,
    v5932: bool,
    v5942: f64,
    v6091: f64,
    v6404: f64,
    v6405: f64,
    v6409: f64,
    v6410: f64,
    v6460: f64,
    v6461: f64,
    v6507: f64,
    v6508: f64,
    v6517: f64,
    v6518: f64,
    v6522: f64,
    v6586: f64,
    v6587: f64,
    v6670: f64,
    v6673: f64,
    v6721: f64,
    v6722: f64,
    v6759: f64,
    v6760: f64,
    v6814: f64,
    v6815: f64,
    v6875: f64,
    v6876: f64,
    v6942: f64,
    v6943: f64,
    v7000: f64,
    v7001: f64,
    v7044: f64,
    v7045: f64,
    v7108: f64,
    v7109: f64,
    v7113: f64,
    v7179: f64,
    v7180: f64,
    v7265: f64,
    v7268: f64,
    v7316: f64,
    v7317: f64,
    v7354: f64,
    v7355: f64,
    v7409: f64,
    v7410: f64,
    v7470: f64,
    v7471: f64,
    v7537: f64,
    v7538: f64,
    v7595: f64,
    v7596: f64,
    v7641: f64,
    v7642: f64,
    v7703: f64,
    v7704: f64,
    v7708: f64,
    v7774: f64,
    v7775: f64,
    v7860: f64,
    v7863: f64,
    v7911: f64,
    v7912: f64,
    v7949: f64,
    v7950: f64,
    v8004: f64,
    v8005: f64,
    v8065: f64,
    v8066: f64,
    v8132: f64,
    v8133: f64,
    v8190: f64,
    v8191: f64,
    v8236: f64,
    v8237: f64,
    v8473: f64,
    v8474: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=0.0;
        let v3=0.5;
        let v5=1.0;
        let v60=2.0;
        let v61=3.0;
        let v398=230.25850929940458;
        let v407=1e-100;
        let v408=-230.25850929940458;
        let v410=0.3333333333333333;
        let v422=1e100;
        let v721=0.375;
        let v4925=(self.scalar_static_f64[564]*(ctx.node_voltage(nodes[0])-ctx.node_voltage(nodes[1])));
        let v4926=(self.scalar_static_f64[612]*v4925);
        let v4969=(-v4925);
        let v4996=(if (self.scalar_static_f64[177]!=0.0){(v4925+self.scalar_static_f64[4253])}else{v1});
        let v4998=(if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[828]+v4996)}else{v1});
        let v5000=(if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[828]-v4996)}else{v1});
        let v5003=((self.scalar_static_f64[4251]+(v5000*v5000))).sqrt();
        let v5004=(if (self.scalar_static_f64[177]!=0.0){v5003}else{v1});
        let v5005=(self.scalar_static_f64[828]*v4925);
        let v5006=(v4998+v5004);
        let v5009=(if (self.scalar_static_f64[177]!=0.0){(v60*(v5005/v5006))}else{v1});
        let v5017=(v5-(self.scalar_static_f64[677]*v5009));
        let v5018=(v5017).sqrt();
        let v5023=(if self.scalar_static_bool[743]{f64::powf(v5017,self.scalar_static_f64[20])}else{(if self.scalar_static_bool[742]{v5018}else{v1})});
        let v5026=(v4925-v5009);
        let v5037=(v5-(self.scalar_static_f64[678]*v5009));
        let v5038=(v5037).sqrt();
        let v5043=(if self.scalar_static_bool[747]{f64::powf(v5037,self.scalar_static_f64[22])}else{(if self.scalar_static_bool[746]{v5038}else{v5023})});
        let v5056=(v5-(self.scalar_static_f64[679]*v5009));
        let v5057=(v5056).sqrt();
        let v5074=(if self.scalar_static_bool[247]{(v4925+self.scalar_static_f64[4259])}else{v4996});
        let v5076=(if self.scalar_static_bool[247]{(self.scalar_static_f64[828]+v5074)}else{v4998});
        let v5078=(if self.scalar_static_bool[247]{(self.scalar_static_f64[828]-v5074)}else{v5000});
        let v5081=((self.scalar_static_f64[4257]+(v5078*v5078))).sqrt();
        let v5082=(if self.scalar_static_bool[247]{v5081}else{v5004});
        let v5083=(v5076+v5082);
        let v5086=(if self.scalar_static_bool[247]{(v60*(v5005/v5083))}else{v1});
        let v5088=(if (v4925<self.scalar_static_f64[792]){v5}else{v1});
        let v5089=(-0.5*v4926);
        let v5092=(if ((v5089).abs()<v398){v5}else{v1});
        let v5093=(self.scalar_static_bool[247]&&(v5088!=0.0));
        let v5094=((v5092!=0.0)&&v5093);
        let v5095=(v5089).exp();
        let v5098=(if (v5089<v1){v5}else{v1});
        let v5100=(v5093&&(!(v5092!=0.0)));
        let v5101=((v5098!=0.0)&&v5100);
        let v5102=(v408-v5089);
        let v5104=(v5+(v410*v5102));
        let v5107=(v5+(v3*(v5102*v5104)));
        let v5109=(v5+(v5102*v5107));
        let v5113=(v5100&&(!(v5098!=0.0)));
        let v5114=(v5089-v398);
        let v5116=(v5+(v410*v5114));
        let v5119=(v5+(v3*(v5114*v5116)));
        let v5123=(if v5113{(v422*(v5+(v5114*v5119)))}else{(if v5101{(v407/v5109)}else{(if v5094{v5095}else{v1})})});
        let v5125=(if v5093{(v5/v5123)}else{v1});
        let v5129=(self.scalar_static_bool[247]&&(!(v5088!=0.0)));
        let v5134=(if v5129{(self.scalar_static_f64[818]*(v5+(self.scalar_static_f64[612]*(v4925-self.scalar_static_f64[792]))))}else{(if v5093{(v5125*v5125)}else{v1})});
        let v5135=(v5134).sqrt();
        let v5136=(if v5129{v5135}else{v5125});
        let v5138=(if v5129{(v5/v5136)}else{v5123});
        let v5142=(if (v4925>v1){v5}else{v1});
        let v5143=(self.scalar_static_bool[247]&&(v5142!=0.0));
        let v5145=(v5+v5138);
        let v5146=(v61+v5138);
        let v5148=((v5145*v5146)).sqrt();
        let v5149=((v60+v5138)+v5148);
        let v5155=(self.scalar_static_bool[247]&&(!(v5142!=0.0)));
        let v5158=(v5+v5136);
        let v5160=(v5+(v61*v5136));
        let v5162=((v5158*v5160)).sqrt();
        let v5163=((v5+(v60*v5136))+v5162);
        let v5168=(if v5155{(v4969+(v60*(self.scalar_static_f64[611]*(v5163).ln())))}else{(if v5143{(v60*(self.scalar_static_f64[611]*(v5149).ln()))}else{v1})});
        let v5170=(if self.scalar_static_bool[247]{(self.scalar_static_f64[826]-v5168)}else{v1});
        let v5172=(v4925-v5170);
        let v5175=((self.scalar_static_f64[904]+(v5172*v5172))).sqrt();
        let v5178=(if self.scalar_static_bool[247]{(v3*((v4925+v5170)-v5175))}else{v1});
        let v5180=(v4925-self.scalar_static_f64[176]);
        let v5183=((self.scalar_static_f64[200]+(v5180*v5180))).sqrt();
        let v5186=(if self.scalar_static_bool[247]{(v3*((self.scalar_static_f64[176]+v4925)-v5183))}else{v1});
        let v5189=((4e-12+(v4925*v4925))).sqrt();
        let v5192=(if self.scalar_static_bool[247]{(v3*(v4925-v5189))}else{v1});
        let v5200=(if self.scalar_static_bool[250]{(self.scalar_static_f64[662]-v5178)}else{v1});
        let v5218=(self.scalar_static_f64[42]*v5200);
        let v5219=(v5218).sqrt();
        let v5222=(if self.scalar_static_bool[252]{f64::powf(v5218,self.scalar_static_f64[19])}else{(if self.scalar_static_bool[251]{v5219}else{v1})});
        let v5224=(if self.scalar_static_bool[250]{(self.scalar_static_f64[29]*v5222)}else{v1});
        let v5233=(self.scalar_static_f64[20]*v5224);
        let v5236=(if self.scalar_static_bool[253]{(self.scalar_static_f64[711]*(v5233/v5200))}else{v1});
        let v5238=(if self.scalar_static_bool[253]{(self.scalar_static_f64[947]/v5236)}else{v1});
        let v5240=(if self.scalar_static_bool[253]{(v5238*v5238)}else{v1});
        let v5241=(v5240*v5240);
        let v5242=(v5+v5241);
        let v5244=((v5241/v5242)).sqrt();
        let v5245=(if self.scalar_static_bool[253]{v5244}else{v1});
        let v5246=(v5245).sqrt();
        let v5247=(if self.scalar_static_bool[253]{v5246}else{v1});
        let v5249=(if self.scalar_static_bool[253]{(v5245*v5247)}else{v1});
        let v5251=(v5236*v5249);
        let v5264=((v721*(v5236/v5247))).sqrt();
        let v5265=(if self.scalar_static_bool[253]{v5264}else{v1});
        let v5269=(if self.scalar_static_bool[253]{((v60*(v5238*v5247))-v5245)}else{v1});
        let v5270=(self.scalar_static_f64[704]*v5238);
        let v5276=(if self.scalar_static_bool[253]{(((v5247*v5270)-(self.scalar_static_f64[704]*v5245))+(v3*v5251))}else{v1});
        let v5277=(v5269-v5);
        let v5279=(if self.scalar_static_bool[253]{(v5265*v5277)}else{v1});
        let v5281=(if self.scalar_static_bool[253]{(v5279*v5279)}else{v1});
        let v5283=(if (v5279>v1){v5}else{v1});
        let v5290=(self.scalar_static_bool[253]&&(!(v5283!=0.0)));
        let v5295=(v5276+(-v5281));
        let v5297=(if (v5295>v408){v5}else{v1});
        let v5298=(self.scalar_static_bool[253]&&(v5297!=0.0));
        let v5299=(v5295).exp();
        let v5302=(self.scalar_static_bool[253]&&(!(v5297!=0.0)));
        let v5303=(v408-v5295);
        let v5305=(v5+(v410*v5303));
        let v5308=(v5+(v3*(v5303*v5305)));
        let v5310=(v5+(v5303*v5308));
        let v5312=(if v5302{(v407/v5310)}else{(if v5298{v5299}else{v5222})});
        let v5324=(if (v5276>v408){v5}else{v1});
        let v5325=(v5290&&(v5324!=0.0));
        let v5326=(v5276).exp();
        let v5329=(v5290&&(!(v5324!=0.0)));
        let v5330=(v408-v5276);
        let v5332=(v5+(v410*v5330));
        let v5335=(v5+(v3*(v5330*v5332)));
        let v5337=(v5+(v5330*v5335));
        let v5339=(if v5329{(v407/v5337)}else{(if v5325{v5326}else{v5312})});
        let v5353=(self.scalar_static_f64[41]-v5186);
        let v5354=(self.scalar_static_f64[42]*v5353);
        let v5355=(v5354).sqrt();
        let v5359=(if self.scalar_static_bool[258]{f64::powf(v5354,self.scalar_static_f64[19])}else{(if self.scalar_static_bool[257]{v5355}else{v5339})});
        let v5360=(self.scalar_static_f64[38]*v5353);
        let v5363=(if self.scalar_static_bool[256]{(self.scalar_static_f64[25]*(v5360/v5359))}else{v1});
        let v5364=(self.scalar_static_f64[1053]/v5363);
        let v5367=(if ((v5364).abs()<v398){v5}else{v1});
        let v5368=(self.scalar_static_bool[256]&&(v5367!=0.0));
        let v5369=(v5364).exp();
        let v5372=(if (v5364<v1){v5}else{v1});
        let v5374=(self.scalar_static_bool[256]&&(!(v5367!=0.0)));
        let v5375=((v5372!=0.0)&&v5374);
        let v5376=(v408-v5364);
        let v5378=(v5+(v410*v5376));
        let v5381=(v5+(v3*(v5376*v5378)));
        let v5383=(v5+(v5376*v5381));
        let v5387=(v5374&&(!(v5372!=0.0)));
        let v5388=(v5364-v398);
        let v5390=(v5+(v410*v5388));
        let v5393=(v5+(v3*(v5388*v5390)));
        let v5397=(if v5387{(v422*(v5+(v5388*v5393)))}else{(if v5375{(v407/v5383)}else{(if v5368{v5369}else{v5359})})});
        let v5406=(if (v5192>self.scalar_static_f64[232]){v5}else{v1});
        let v5408=((v5406!=0.0)&&self.scalar_static_bool[260]);
        let v5409=((self.scalar_static_f64[234]!=0.0)&&v5408);
        let v5410=(self.scalar_static_f64[63]*v5192);
        let v5411=(v5410*v5410);
        let v5412=(v5410*v5411);
        let v5415=(self.scalar_static_bool[72]&&v5408);
        let v5418=(if v5415{f64::powf((v5410).abs(),self.scalar_static_f64[50])}else{(if v5409{(v5410*v5412)}else{v5397})});
        let v5436=(v5-(self.scalar_static_f64[677]*v5086));
        let v5437=(v5436).sqrt();
        let v5441=(if self.scalar_static_bool[262]{f64::powf(v5436,self.scalar_static_f64[20])}else{(if self.scalar_static_bool[261]{v5437}else{v5418})});
        let v5445=(v4925-v5086);
        let v5459=(if self.scalar_static_bool[266]{(self.scalar_static_f64[669]-v5178)}else{v5200});
        let v5478=(self.scalar_static_f64[44]*v5459);
        let v5479=(v5478).sqrt();
        let v5482=(if self.scalar_static_bool[268]{f64::powf(v5478,self.scalar_static_f64[21])}else{(if self.scalar_static_bool[267]{v5479}else{v5441})});
        let v5484=(if self.scalar_static_bool[266]{(self.scalar_static_f64[33]*v5482)}else{v5224});
        let v5494=(self.scalar_static_f64[22]*v5484);
        let v5497=(if self.scalar_static_bool[270]{(self.scalar_static_f64[716]*(v5494/v5459))}else{v5236});
        let v5499=(if self.scalar_static_bool[270]{(self.scalar_static_f64[1136]/v5497)}else{v5238});
        let v5501=(if self.scalar_static_bool[270]{(v5499*v5499)}else{v5240});
        let v5502=(v5501*v5501);
        let v5503=(v5+v5502);
        let v5505=((v5502/v5503)).sqrt();
        let v5506=(if self.scalar_static_bool[270]{v5505}else{v5245});
        let v5507=(v5506).sqrt();
        let v5508=(if self.scalar_static_bool[270]{v5507}else{v5247});
        let v5510=(if self.scalar_static_bool[270]{(v5506*v5508)}else{v5249});
        let v5512=(v5497*v5510);
        let v5525=((v721*(v5497/v5508))).sqrt();
        let v5526=(if self.scalar_static_bool[270]{v5525}else{v5265});
        let v5530=(if self.scalar_static_bool[270]{((v60*(v5499*v5508))-v5506)}else{v5269});
        let v5531=(self.scalar_static_f64[705]*v5499);
        let v5537=(if self.scalar_static_bool[270]{(((v5508*v5531)-(self.scalar_static_f64[705]*v5506))+(v3*v5512))}else{v5276});
        let v5538=(v5530-v5);
        let v5540=(if self.scalar_static_bool[270]{(v5526*v5538)}else{v5279});
        let v5542=(if self.scalar_static_bool[270]{(v5540*v5540)}else{v5281});
        let v5544=(if (v5540>v1){v5}else{v1});
        let v5551=(self.scalar_static_bool[270]&&(!(v5544!=0.0)));
        let v5556=(v5537+(-v5542));
        let v5558=(if (v5556>v408){v5}else{v1});
        let v5559=(self.scalar_static_bool[270]&&(v5558!=0.0));
        let v5560=(v5556).exp();
        let v5563=(self.scalar_static_bool[270]&&(!(v5558!=0.0)));
        let v5564=(v408-v5556);
        let v5566=(v5+(v410*v5564));
        let v5569=(v5+(v3*(v5564*v5566)));
        let v5571=(v5+(v5564*v5569));
        let v5573=(if v5563{(v407/v5571)}else{(if v5559{v5560}else{v5482})});
        let v5585=(if (v5537>v408){v5}else{v1});
        let v5586=(v5551&&(v5585!=0.0));
        let v5587=(v5537).exp();
        let v5590=(v5551&&(!(v5585!=0.0)));
        let v5591=(v408-v5537);
        let v5593=(v5+(v410*v5591));
        let v5596=(v5+(v3*(v5591*v5593)));
        let v5598=(v5+(v5591*v5596));
        let v5600=(if v5590{(v407/v5598)}else{(if v5586{v5587}else{v5573})});
        let v5616=(self.scalar_static_f64[43]-v5186);
        let v5617=(self.scalar_static_f64[44]*v5616);
        let v5618=(v5617).sqrt();
        let v5622=(if self.scalar_static_bool[276]{f64::powf(v5617,self.scalar_static_f64[21])}else{(if self.scalar_static_bool[275]{v5618}else{v5600})});
        let v5623=(self.scalar_static_f64[39]*v5616);
        let v5626=(if self.scalar_static_bool[274]{(self.scalar_static_f64[26]*(v5623/v5622))}else{v5363});
        let v5627=(self.scalar_static_f64[1243]/v5626);
        let v5630=(if ((v5627).abs()<v398){v5}else{v1});
        let v5631=(self.scalar_static_bool[274]&&(v5630!=0.0));
        let v5632=(v5627).exp();
        let v5635=(if (v5627<v1){v5}else{v1});
        let v5637=(self.scalar_static_bool[274]&&(!(v5630!=0.0)));
        let v5638=((v5635!=0.0)&&v5637);
        let v5639=(v408-v5627);
        let v5641=(v5+(v410*v5639));
        let v5644=(v5+(v3*(v5639*v5641)));
        let v5646=(v5+(v5639*v5644));
        let v5650=(v5637&&(!(v5635!=0.0)));
        let v5651=(v5627-v398);
        let v5653=(v5+(v410*v5651));
        let v5656=(v5+(v3*(v5651*v5653)));
        let v5660=(if v5650{(v422*(v5+(v5651*v5656)))}else{(if v5638{(v407/v5646)}else{(if v5631{v5632}else{v5622})})});
        let v5669=(if (v5192>self.scalar_static_f64[264]){v5}else{v1});
        let v5671=((v5669!=0.0)&&self.scalar_static_bool[278]);
        let v5672=((self.scalar_static_f64[266]!=0.0)&&v5671);
        let v5673=(self.scalar_static_f64[65]*v5192);
        let v5674=(v5673*v5673);
        let v5675=(v5673*v5674);
        let v5678=(self.scalar_static_bool[110]&&v5671);
        let v5681=(if v5678{f64::powf((v5673).abs(),self.scalar_static_f64[54])}else{(if v5672{(v5673*v5675)}else{v5660})});
        let v5699=(v5-(self.scalar_static_f64[678]*v5086));
        let v5700=(v5699).sqrt();
        let v5704=(if self.scalar_static_bool[280]{f64::powf(v5699,self.scalar_static_f64[22])}else{(if self.scalar_static_bool[279]{v5700}else{v5681})});
        let v5720=(if self.scalar_static_bool[284]{(self.scalar_static_f64[676]-v5178)}else{v5459});
        let v5739=(self.scalar_static_f64[46]*v5720);
        let v5740=(v5739).sqrt();
        let v5743=(if self.scalar_static_bool[286]{f64::powf(v5739,self.scalar_static_f64[23])}else{(if self.scalar_static_bool[285]{v5740}else{v5704})});
        let v5745=(if self.scalar_static_bool[284]{(self.scalar_static_f64[37]*v5743)}else{v5484});
        let v5755=(self.scalar_static_f64[24]*v5745);
        let v5758=(if self.scalar_static_bool[288]{(self.scalar_static_f64[721]*(v5755/v5720))}else{v5497});
        let v5760=(if self.scalar_static_bool[288]{(self.scalar_static_f64[1327]/v5758)}else{v5499});
        let v5762=(if self.scalar_static_bool[288]{(v5760*v5760)}else{v5501});
        let v5763=(v5762*v5762);
        let v5764=(v5+v5763);
        let v5766=((v5763/v5764)).sqrt();
        let v5767=(if self.scalar_static_bool[288]{v5766}else{v5506});
        let v5768=(v5767).sqrt();
        let v5769=(if self.scalar_static_bool[288]{v5768}else{v5508});
        let v5771=(if self.scalar_static_bool[288]{(v5767*v5769)}else{v5510});
        let v5773=(v5758*v5771);
        let v5786=((v721*(v5758/v5769))).sqrt();
        let v5787=(if self.scalar_static_bool[288]{v5786}else{v5526});
        let v5792=(self.scalar_static_f64[706]*v5760);
        let v5798=(if self.scalar_static_bool[288]{(((v5769*v5792)-(self.scalar_static_f64[706]*v5767))+(v3*v5773))}else{v5537});
        let v5799=((if self.scalar_static_bool[288]{((v60*(v5760*v5769))-v5767)}else{v5530})-v5);
        let v5801=(if self.scalar_static_bool[288]{(v5787*v5799)}else{v5540});
        let v5805=(if (v5801>v1){v5}else{v1});
        let v5812=(self.scalar_static_bool[288]&&(!(v5805!=0.0)));
        let v5817=(v5798+(-(if self.scalar_static_bool[288]{(v5801*v5801)}else{v5542})));
        let v5819=(if (v5817>v408){v5}else{v1});
        let v5820=(self.scalar_static_bool[288]&&(v5819!=0.0));
        let v5821=(v5817).exp();
        let v5824=(self.scalar_static_bool[288]&&(!(v5819!=0.0)));
        let v5825=(v408-v5817);
        let v5827=(v5+(v410*v5825));
        let v5830=(v5+(v3*(v5825*v5827)));
        let v5832=(v5+(v5825*v5830));
        let v5834=(if v5824{(v407/v5832)}else{(if v5820{v5821}else{v5743})});
        let v5846=(if (v5798>v408){v5}else{v1});
        let v5847=(v5812&&(v5846!=0.0));
        let v5848=(v5798).exp();
        let v5851=(v5812&&(!(v5846!=0.0)));
        let v5852=(v408-v5798);
        let v5854=(v5+(v410*v5852));
        let v5857=(v5+(v3*(v5852*v5854)));
        let v5859=(v5+(v5852*v5857));
        let v5861=(if v5851{(v407/v5859)}else{(if v5847{v5848}else{v5834})});
        let v5877=(self.scalar_static_f64[45]-v5186);
        let v5878=(self.scalar_static_f64[46]*v5877);
        let v5879=(v5878).sqrt();
        let v5883=(if self.scalar_static_bool[294]{f64::powf(v5878,self.scalar_static_f64[23])}else{(if self.scalar_static_bool[293]{v5879}else{v5861})});
        let v5884=(self.scalar_static_f64[40]*v5877);
        let v5887=(if self.scalar_static_bool[292]{(self.scalar_static_f64[27]*(v5884/v5883))}else{v5626});
        let v5888=(self.scalar_static_f64[1434]/v5887);
        let v5891=(if ((v5888).abs()<v398){v5}else{v1});
        let v5892=(self.scalar_static_bool[292]&&(v5891!=0.0));
        let v5893=(v5888).exp();
        let v5896=(if (v5888<v1){v5}else{v1});
        let v5898=(self.scalar_static_bool[292]&&(!(v5891!=0.0)));
        let v5899=((v5896!=0.0)&&v5898);
        let v5900=(v408-v5888);
        let v5902=(v5+(v410*v5900));
        let v5905=(v5+(v3*(v5900*v5902)));
        let v5907=(v5+(v5900*v5905));
        let v5911=(v5898&&(!(v5896!=0.0)));
        let v5912=(v5888-v398);
        let v5914=(v5+(v410*v5912));
        let v5917=(v5+(v3*(v5912*v5914)));
        let v5921=(if v5911{(v422*(v5+(v5912*v5917)))}else{(if v5899{(v407/v5907)}else{(if v5892{v5893}else{v5883})})});
        let v5930=(if (v5192>self.scalar_static_f64[295]){v5}else{v1});
        let v5932=((v5930!=0.0)&&self.scalar_static_bool[296]);
        let v5933=((self.scalar_static_f64[297]!=0.0)&&v5932);
        let v5934=(self.scalar_static_f64[67]*v5192);
        let v5935=(v5934*v5934);
        let v5936=(v5934*v5935);
        let v5939=(self.scalar_static_bool[148]&&v5932);
        let v5942=(if v5939{f64::powf((v5934).abs(),self.scalar_static_f64[58])}else{(if v5933{(v5934*v5936)}else{v5921})});
        let v5960=(v4925<self.scalar_static_f64[570]);
        let v5963=((v4925-self.scalar_static_f64[570])/self.scalar_static_f64[571]);
        let v5966=(v5963< -37.0);
        let v5967=(v5963).exp();
        let v5968=(v5+v5967);
        let v5973=(v5963>37.0);
        let v5976=(((self.scalar_static_f64[570]-v4925)/self.scalar_static_f64[571])).exp();
        let v5977=(v5+v5976);
        let v5983=(if self.scalar_static_bool[297]{(if v5960{(if v5966{self.scalar_static_f64[570]}else{(self.scalar_static_f64[570]+(self.scalar_static_f64[571]*(v5968).ln()))})}else{(if v5973{v4925}else{(v4925+(self.scalar_static_f64[571]*(v5977).ln()))})})}else{v1});
        let v5988=(if self.scalar_static_bool[297]{(v5983+self.scalar_static_f64[4262])}else{v5074});
        let v5990=(if self.scalar_static_bool[297]{(self.scalar_static_f64[828]+v5988)}else{v5076});
        let v5992=(if self.scalar_static_bool[297]{(self.scalar_static_f64[828]-v5988)}else{v5078});
        let v5995=((self.scalar_static_f64[4260]+(v5992*v5992))).sqrt();
        let v5996=(if self.scalar_static_bool[297]{v5995}else{v5082});
        let v5997=(self.scalar_static_f64[828]*v5983);
        let v5998=(v5990+v5996);
        let v6001=(if self.scalar_static_bool[297]{(v60*(v5997/v5998))}else{v1});
        let v6004=(v5-(self.scalar_static_f64[679]*v6001));
        let v6005=(v6004).sqrt();
        let v6009=(if self.scalar_static_bool[299]{f64::powf(v6004,self.scalar_static_f64[24])}else{(if self.scalar_static_bool[298]{v6005}else{v5942})});
        let v6016=(if self.scalar_static_bool[297]{(self.scalar_static_f64[569]*((self.scalar_static_f64[694]*(v5-v6009))+(self.scalar_static_f64[697]*(v5983-v6001))))}else{(if self.scalar_static_bool[281]{v1}else{(if self.scalar_static_bool[749]{((self.scalar_static_f64[694]*(v5-(if self.scalar_static_bool[751]{f64::powf(v5056,self.scalar_static_f64[24])}else{(if self.scalar_static_bool[750]{v5057}else{v5043})})))+(self.scalar_static_f64[697]*v5026))}else{v1})})});
        let v6019=(if self.scalar_static_bool[297]{((v4925+self.scalar_static_f64[570])-v5983)}else{v5983});
        let v6024=(if self.scalar_static_bool[297]{(v6019+self.scalar_static_f64[4265])}else{v5988});
        let v6028=(if self.scalar_static_bool[297]{(self.scalar_static_f64[828]-v6024)}else{v5992});
        let v6031=((self.scalar_static_f64[4263]+(v6028*v6028))).sqrt();
        let v6033=(self.scalar_static_f64[828]*v6019);
        let v6034=((if self.scalar_static_bool[297]{(self.scalar_static_f64[828]+v6024)}else{v5990})+(if self.scalar_static_bool[297]{v6031}else{v5996}));
        let v6037=(if self.scalar_static_bool[297]{(v60*(v6033/v6034))}else{v6001});
        let v6042=(v5-(self.scalar_static_f64[757]*v6037));
        let v6043=(v6042).sqrt();
        let v6048=(if self.scalar_static_bool[303]{f64::powf(v6042,self.scalar_static_f64[112])}else{(if self.scalar_static_bool[301]{v6043}else{v6009})});
        let v6062=(v5-(self.scalar_static_f64[679]*v5086));
        let v6063=(v6062).sqrt();
        let v6091=((((self.scalar_static_f64[143]*(if self.scalar_static_bool[249]{(self.scalar_static_f64[569]*((self.scalar_static_f64[690]*(v5-v5441))+(self.scalar_static_f64[695]*v5445)))}else{(if self.scalar_static_bool[248]{v1}else{(if self.scalar_static_bool[741]{((self.scalar_static_f64[690]*(v5-v5023))+(self.scalar_static_f64[695]*v5026))}else{v1})})}))+(self.scalar_static_f64[145]*(if self.scalar_static_bool[264]{(self.scalar_static_f64[569]*((self.scalar_static_f64[692]*(v5-v5704))+(self.scalar_static_f64[696]*v5445)))}else{(if self.scalar_static_bool[263]{v1}else{(if self.scalar_static_bool[745]{((self.scalar_static_f64[692]*(v5-v5043))+(self.scalar_static_f64[696]*v5026))}else{v1})})})))+(self.scalar_static_f64[147]*(if self.scalar_static_bool[305]{(self.scalar_static_f64[569]*((self.scalar_static_f64[694]*(v5-(if self.scalar_static_bool[307]{f64::powf(v6062,self.scalar_static_f64[24])}else{(if self.scalar_static_bool[306]{v6063}else{v6048})})))+(self.scalar_static_f64[697]*v5445)))}else{(if self.scalar_static_bool[297]{(v6016+(if self.scalar_static_bool[297]{(self.scalar_static_f64[569]*((self.scalar_static_f64[764]*(v5-v6048))+(self.scalar_static_f64[766]*(v6019-v6037))))}else{v1}))}else{v6016})})))*self.scalar_static_f64[575]);
        let v6198=(v5000*self.scalar_static_f64[585]);
        let v6200=(v5000*self.scalar_static_f64[586]);
        let v6202=(v60*v5003);
        let v6205=(if (self.scalar_static_f64[177]!=0.0){((v6198+v6198)/v6202)}else{v1});
        let v6206=(if (self.scalar_static_f64[177]!=0.0){((v6200+v6200)/v6202)}else{v1});
        let v6214=(v5006*v5006);
        let v6222=(if (self.scalar_static_f64[177]!=0.0){(v60*(((v5006*self.scalar_static_f64[4304])-(v5005*(self.scalar_static_f64[581]+v6205)))/v6214))}else{v1});
        let v6223=(if (self.scalar_static_f64[177]!=0.0){(v60*(((v5006*self.scalar_static_f64[4305])-(v5005*(self.scalar_static_f64[582]+v6206)))/v6214))}else{v1});
        let v6226=(-(self.scalar_static_f64[677]*v6222));
        let v6227=(-(self.scalar_static_f64[677]*v6223));
        let v6228=(v60*v5018);
        let v6235=(self.scalar_static_f64[20]*f64::powf(v5017,self.scalar_static_f64[587]));
        let v6238=(if self.scalar_static_bool[743]{(v6226*v6235)}else{(if self.scalar_static_bool[742]{(v6226/v6228)}else{v1})});
        let v6239=(if self.scalar_static_bool[743]{(v6227*v6235)}else{(if self.scalar_static_bool[742]{(v6227/v6228)}else{v1})});
        let v6244=(self.scalar_static_f64[564]-v6222);
        let v6245=(self.scalar_static_f64[578]-v6223);
        let v6254=(-(self.scalar_static_f64[678]*v6222));
        let v6255=(-(self.scalar_static_f64[678]*v6223));
        let v6256=(v60*v5038);
        let v6263=(self.scalar_static_f64[22]*f64::powf(v5037,self.scalar_static_f64[588]));
        let v6266=(if self.scalar_static_bool[747]{(v6254*v6263)}else{(if self.scalar_static_bool[746]{(v6254/v6256)}else{v6238})});
        let v6267=(if self.scalar_static_bool[747]{(v6255*v6263)}else{(if self.scalar_static_bool[746]{(v6255/v6256)}else{v6239})});
        let v6280=(-(self.scalar_static_f64[679]*v6222));
        let v6281=(-(self.scalar_static_f64[679]*v6223));
        let v6282=(v60*v5057);
        let v6289=(self.scalar_static_f64[24]*f64::powf(v5056,self.scalar_static_f64[589]));
        let v6312=(v5078*self.scalar_static_f64[596]);
        let v6314=(v5078*self.scalar_static_f64[597]);
        let v6316=(v60*v5081);
        let v6319=(if self.scalar_static_bool[247]{((v6312+v6312)/v6316)}else{v6205});
        let v6320=(if self.scalar_static_bool[247]{((v6314+v6314)/v6316)}else{v6206});
        let v6326=(v5083*v5083);
        let v6334=(if self.scalar_static_bool[247]{(v60*(((v5083*self.scalar_static_f64[4304])-(v5005*(self.scalar_static_f64[592]+v6319)))/v6326))}else{v1});
        let v6335=(if self.scalar_static_bool[247]{(v60*(((v5083*self.scalar_static_f64[4305])-(v5005*(self.scalar_static_f64[593]+v6320)))/v6326))}else{v1});
        let v6362=(v5109*v5109);
        let v6387=(if v5113{(v422*((v5119*self.scalar_static_f64[4306])+(v5114*(v3*((v5116*self.scalar_static_f64[4306])+(v5114*self.scalar_static_f64[4312]))))))}else{(if v5101{((-(v407*((v5107*self.scalar_static_f64[4308])+(v5102*(v3*((v5104*self.scalar_static_f64[4308])+(v5102*self.scalar_static_f64[4310])))))))/v6362)}else{(if v5094{(v5095*self.scalar_static_f64[4306])}else{v1})})});
        let v6388=(if v5113{(v422*((v5119*self.scalar_static_f64[4307])+(v5114*(v3*((v5116*self.scalar_static_f64[4307])+(v5114*self.scalar_static_f64[4313]))))))}else{(if v5101{((-(v407*((v5107*self.scalar_static_f64[4309])+(v5102*(v3*((v5104*self.scalar_static_f64[4309])+(v5102*self.scalar_static_f64[4311])))))))/v6362)}else{(if v5094{(v5095*self.scalar_static_f64[4307])}else{v1})})});
        let v6390=(v5123*v5123);
        let v6394=(if v5093{((-v6387)/v6390)}else{v1});
        let v6395=(if v5093{((-v6388)/v6390)}else{v1});
        let v6396=(v5125*v6394);
        let v6398=(v5125*v6395);
        let v6404=(if v5129{self.scalar_static_f64[4314]}else{(if v5093{(v6396+v6396)}else{v1})});
        let v6405=(if v5129{self.scalar_static_f64[4315]}else{(if v5093{(v6398+v6398)}else{v1})});
        let v6406=(v60*v5135);
        let v6409=(if v5129{(v6404/v6406)}else{v6394});
        let v6410=(if v5129{(v6405/v6406)}else{v6395});
        let v6412=(v5136*v5136);
        let v6416=(if v5129{((-v6409)/v6412)}else{v6387});
        let v6417=(if v5129{((-v6410)/v6412)}else{v6388});
        let v6424=(v60*v5148);
        let v6447=(v60*v5162);
        let v6460=(if v5155{(self.scalar_static_f64[578]+(v60*(self.scalar_static_f64[611]*(((v60*v6409)+(((v5160*v6409)+(v5158*(v61*v6409)))/v6447))/v5163))))}else{(if v5143{(v60*(self.scalar_static_f64[611]*((v6416+(((v5146*v6416)+(v5145*v6416))/v6424))/v5149)))}else{v1})});
        let v6461=(if v5155{(self.scalar_static_f64[564]+(v60*(self.scalar_static_f64[611]*(((v60*v6410)+(((v5160*v6410)+(v5158*(v61*v6410)))/v6447))/v5163))))}else{(if v5143{(v60*(self.scalar_static_f64[611]*((v6417+(((v5146*v6417)+(v5145*v6417))/v6424))/v5149)))}else{v1})});
        let v6464=(if self.scalar_static_bool[247]{(-v6460)}else{v1});
        let v6465=(if self.scalar_static_bool[247]{(-v6461)}else{v1});
        let v6470=(v5172*(self.scalar_static_f64[564]-v6464));
        let v6472=(v5172*(self.scalar_static_f64[578]-v6465));
        let v6474=(v60*v5175);
        let v6483=(self.scalar_static_f64[564]*v5180);
        let v6485=(v5180*self.scalar_static_f64[578]);
        let v6487=(v60*v5183);
        let v6496=(self.scalar_static_f64[564]*v4925);
        let v6498=(v4925*self.scalar_static_f64[578]);
        let v6500=(v60*v5189);
        let v6507=(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[564]-((v6496+v6496)/v6500)))}else{v1});
        let v6508=(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[578]-((v6498+v6498)/v6500)))}else{v1});
        let v6515=(-(if self.scalar_static_bool[247]{(v3*((self.scalar_static_f64[564]+v6464)-((v6470+v6470)/v6474)))}else{v1}));
        let v6516=(-(if self.scalar_static_bool[247]{(v3*((self.scalar_static_f64[578]+v6465)-((v6472+v6472)/v6474)))}else{v1}));
        let v6517=(if self.scalar_static_bool[250]{v6515}else{v1});
        let v6518=(if self.scalar_static_bool[250]{v6516}else{v1});
        let v6522=(v5200*v5200);
        let v6570=(self.scalar_static_f64[42]*v6517);
        let v6571=(self.scalar_static_f64[42]*v6518);
        let v6572=(v60*v5219);
        let v6579=(self.scalar_static_f64[19]*f64::powf(v5218,self.scalar_static_f64[598]));
        let v6582=(if self.scalar_static_bool[252]{(v6570*v6579)}else{(if self.scalar_static_bool[251]{(v6570/v6572)}else{v1})});
        let v6583=(if self.scalar_static_bool[252]{(v6571*v6579)}else{(if self.scalar_static_bool[251]{(v6571/v6572)}else{v1})});
        let v6586=(if self.scalar_static_bool[250]{(self.scalar_static_f64[29]*v6582)}else{v1});
        let v6587=(if self.scalar_static_bool[250]{(self.scalar_static_f64[29]*v6583)}else{v1});
        let v6620=(if self.scalar_static_bool[253]{(self.scalar_static_f64[711]*(((v5200*(self.scalar_static_f64[20]*v6586))-(v5233*v6517))/v6522))}else{v1});
        let v6621=(if self.scalar_static_bool[253]{(self.scalar_static_f64[711]*(((v5200*(self.scalar_static_f64[20]*v6587))-(v5233*v6518))/v6522))}else{v1});
        let v6624=(v5236*v5236);
        let v6629=(if self.scalar_static_bool[253]{((-(self.scalar_static_f64[947]*v6620))/v6624)}else{v1});
        let v6630=(if self.scalar_static_bool[253]{((-(self.scalar_static_f64[947]*v6621))/v6624)}else{v1});
        let v6631=(v5238*v6629);
        let v6633=(v5238*v6630);
        let v6635=(if self.scalar_static_bool[253]{(v6631+v6631)}else{v1});
        let v6636=(if self.scalar_static_bool[253]{(v6633+v6633)}else{v1});
        let v6637=(v5240*v6635);
        let v6638=(v6637+v6637);
        let v6639=(v5240*v6636);
        let v6640=(v6639+v6639);
        let v6644=(v5242*v5242);
        let v6650=(v60*v5244);
        let v6653=(if self.scalar_static_bool[253]{((((v5242*v6638)-(v5241*v6638))/v6644)/v6650)}else{v1});
        let v6654=(if self.scalar_static_bool[253]{((((v5242*v6640)-(v5241*v6640))/v6644)/v6650)}else{v1});
        let v6655=(v60*v5246);
        let v6658=(if self.scalar_static_bool[253]{(v6653/v6655)}else{v1});
        let v6659=(if self.scalar_static_bool[253]{(v6654/v6655)}else{v1});
        let v6666=(if self.scalar_static_bool[253]{((v5247*v6653)+(v5245*v6658))}else{v1});
        let v6667=(if self.scalar_static_bool[253]{((v5247*v6654)+(v5245*v6659))}else{v1});
        let v6670=((v5249*v6620)+(v5236*v6666));
        let v6673=((v5249*v6621)+(v5236*v6667));
        let v6710=(v5247*v5247);
        let v6718=(v60*v5264);
        let v6721=(if self.scalar_static_bool[253]{((v721*(((v5247*v6620)-(v5236*v6658))/v6710))/v6718)}else{v1});
        let v6722=(if self.scalar_static_bool[253]{((v721*(((v5247*v6621)-(v5236*v6659))/v6710))/v6718)}else{v1});
        let v6733=(if self.scalar_static_bool[253]{((v60*((v5247*v6629)+(v5238*v6658)))-v6653)}else{v1});
        let v6734=(if self.scalar_static_bool[253]{((v60*((v5247*v6630)+(v5238*v6659)))-v6654)}else{v1});
        let v6751=(if self.scalar_static_bool[253]{((((v5270*v6658)+(v5247*(self.scalar_static_f64[704]*v6629)))-(self.scalar_static_f64[704]*v6653))+(v3*v6670))}else{v1});
        let v6752=(if self.scalar_static_bool[253]{((((v5270*v6659)+(v5247*(self.scalar_static_f64[704]*v6630)))-(self.scalar_static_f64[704]*v6654))+(v3*v6673))}else{v1});
        let v6759=(if self.scalar_static_bool[253]{((v5277*v6721)+(v5265*v6733))}else{v1});
        let v6760=(if self.scalar_static_bool[253]{((v5277*v6722)+(v5265*v6734))}else{v1});
        let v6761=(v5279*v6759);
        let v6763=(v5279*v6760);
        let v6765=(if self.scalar_static_bool[253]{(v6761+v6761)}else{v1});
        let v6766=(if self.scalar_static_bool[253]{(v6763+v6763)}else{v1});
        let v6783=(v6751+(-v6765));
        let v6784=(v6752+(-v6766));
        let v6789=(-v6783);
        let v6790=(-v6784);
        let v6809=(v5310*v5310);
        let v6814=(if v5302{((-(v407*((v5308*v6789)+(v5303*(v3*((v5305*v6789)+(v5303*(v410*v6789))))))))/v6809)}else{(if v5298{(v5299*v6783)}else{v6582})});
        let v6815=(if v5302{((-(v407*((v5308*v6790)+(v5303*(v3*((v5305*v6790)+(v5303*(v410*v6790))))))))/v6809)}else{(if v5298{(v5299*v6784)}else{v6583})});
        let v6850=(-v6751);
        let v6851=(-v6752);
        let v6870=(v5337*v5337);
        let v6875=(if v5329{((-(v407*((v5335*v6850)+(v5330*(v3*((v5332*v6850)+(v5330*(v410*v6850))))))))/v6870)}else{(if v5325{(v5326*v6751)}else{v6814})});
        let v6876=(if v5329{((-(v407*((v5335*v6851)+(v5330*(v3*((v5332*v6851)+(v5330*(v410*v6851))))))))/v6870)}else{(if v5325{(v5326*v6752)}else{v6815})});
        let v6914=(-(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[564]-((v6483+v6483)/v6487)))}else{v1}));
        let v6915=(-(if self.scalar_static_bool[247]{(v3*(self.scalar_static_f64[578]-((v6485+v6485)/v6487)))}else{v1}));
        let v6916=(self.scalar_static_f64[42]*v6914);
        let v6917=(self.scalar_static_f64[42]*v6915);
        let v6918=(v60*v5355);
        let v6924=(self.scalar_static_f64[19]*f64::powf(v5354,self.scalar_static_f64[598]));
        let v6927=(if self.scalar_static_bool[258]{(v6916*v6924)}else{(if self.scalar_static_bool[257]{(v6916/v6918)}else{v6875})});
        let v6928=(if self.scalar_static_bool[258]{(v6917*v6924)}else{(if self.scalar_static_bool[257]{(v6917/v6918)}else{v6876})});
        let v6934=(v5359*v5359);
        let v6942=(if self.scalar_static_bool[256]{(self.scalar_static_f64[25]*(((v5359*(self.scalar_static_f64[38]*v6914))-(v5360*v6927))/v6934))}else{v1});
        let v6943=(if self.scalar_static_bool[256]{(self.scalar_static_f64[25]*(((v5359*(self.scalar_static_f64[38]*v6915))-(v5360*v6928))/v6934))}else{v1});
        let v6946=(v5363*v5363);
        let v6947=((-(self.scalar_static_f64[1053]*v6942))/v6946);
        let v6950=((-(self.scalar_static_f64[1053]*v6943))/v6946);
        let v6955=(-v6947);
        let v6956=(-v6950);
        let v6975=(v5383*v5383);
        let v7000=(if v5387{(v422*((v5393*v6947)+(v5388*(v3*((v5390*v6947)+(v5388*(v410*v6947)))))))}else{(if v5375{((-(v407*((v5381*v6955)+(v5376*(v3*((v5378*v6955)+(v5376*(v410*v6955))))))))/v6975)}else{(if v5368{(v5369*v6947)}else{v6927})})});
        let v7001=(if v5387{(v422*((v5393*v6950)+(v5388*(v3*((v5390*v6950)+(v5388*(v410*v6950)))))))}else{(if v5375{((-(v407*((v5381*v6956)+(v5376*(v3*((v5378*v6956)+(v5376*(v410*v6956))))))))/v6975)}else{(if v5368{(v5369*v6950)}else{v6928})})});
        let v7024=(self.scalar_static_f64[63]*v6507);
        let v7025=(self.scalar_static_f64[63]*v6508);
        let v7026=(v5410*v7024);
        let v7028=(v5410*v7025);
        let v7044=(if v5415{v1}else{(if v5409{((v5412*v7024)+(v5410*((v5411*v7024)+(v5410*(v7026+v7026)))))}else{v7000})});
        let v7045=(if v5415{v1}else{(if v5409{((v5412*v7025)+(v5410*((v5411*v7025)+(v5410*(v7028+v7028)))))}else{v7001})});
        let v7073=(-(self.scalar_static_f64[677]*v6334));
        let v7074=(-(self.scalar_static_f64[677]*v6335));
        let v7075=(v60*v5437);
        let v7081=(self.scalar_static_f64[20]*f64::powf(v5436,self.scalar_static_f64[587]));
        let v7084=(if self.scalar_static_bool[262]{(v7073*v7081)}else{(if self.scalar_static_bool[261]{(v7073/v7075)}else{v7044})});
        let v7085=(if self.scalar_static_bool[262]{(v7074*v7081)}else{(if self.scalar_static_bool[261]{(v7074/v7075)}else{v7045})});
        let v7090=(self.scalar_static_f64[564]-v6334);
        let v7091=(self.scalar_static_f64[578]-v6335);
        let v7108=(if self.scalar_static_bool[266]{v6515}else{v6517});
        let v7109=(if self.scalar_static_bool[266]{v6516}else{v6518});
        let v7113=(v5459*v5459);
        let v7163=(self.scalar_static_f64[44]*v7108);
        let v7164=(self.scalar_static_f64[44]*v7109);
        let v7165=(v60*v5479);
        let v7172=(self.scalar_static_f64[21]*f64::powf(v5478,self.scalar_static_f64[600]));
        let v7175=(if self.scalar_static_bool[268]{(v7163*v7172)}else{(if self.scalar_static_bool[267]{(v7163/v7165)}else{v7084})});
        let v7176=(if self.scalar_static_bool[268]{(v7164*v7172)}else{(if self.scalar_static_bool[267]{(v7164/v7165)}else{v7085})});
        let v7179=(if self.scalar_static_bool[266]{(self.scalar_static_f64[33]*v7175)}else{v6586});
        let v7180=(if self.scalar_static_bool[266]{(self.scalar_static_f64[33]*v7176)}else{v6587});
        let v7215=(if self.scalar_static_bool[270]{(self.scalar_static_f64[716]*(((v5459*(self.scalar_static_f64[22]*v7179))-(v5494*v7108))/v7113))}else{v6620});
        let v7216=(if self.scalar_static_bool[270]{(self.scalar_static_f64[716]*(((v5459*(self.scalar_static_f64[22]*v7180))-(v5494*v7109))/v7113))}else{v6621});
        let v7219=(v5497*v5497);
        let v7224=(if self.scalar_static_bool[270]{((-(self.scalar_static_f64[1136]*v7215))/v7219)}else{v6629});
        let v7225=(if self.scalar_static_bool[270]{((-(self.scalar_static_f64[1136]*v7216))/v7219)}else{v6630});
        let v7226=(v5499*v7224);
        let v7228=(v5499*v7225);
        let v7230=(if self.scalar_static_bool[270]{(v7226+v7226)}else{v6635});
        let v7231=(if self.scalar_static_bool[270]{(v7228+v7228)}else{v6636});
        let v7232=(v5501*v7230);
        let v7233=(v7232+v7232);
        let v7234=(v5501*v7231);
        let v7235=(v7234+v7234);
        let v7239=(v5503*v5503);
        let v7245=(v60*v5505);
        let v7248=(if self.scalar_static_bool[270]{((((v5503*v7233)-(v5502*v7233))/v7239)/v7245)}else{v6653});
        let v7249=(if self.scalar_static_bool[270]{((((v5503*v7235)-(v5502*v7235))/v7239)/v7245)}else{v6654});
        let v7250=(v60*v5507);
        let v7253=(if self.scalar_static_bool[270]{(v7248/v7250)}else{v6658});
        let v7254=(if self.scalar_static_bool[270]{(v7249/v7250)}else{v6659});
        let v7261=(if self.scalar_static_bool[270]{((v5508*v7248)+(v5506*v7253))}else{v6666});
        let v7262=(if self.scalar_static_bool[270]{((v5508*v7249)+(v5506*v7254))}else{v6667});
        let v7265=((v5510*v7215)+(v5497*v7261));
        let v7268=((v5510*v7216)+(v5497*v7262));
        let v7305=(v5508*v5508);
        let v7313=(v60*v5525);
        let v7316=(if self.scalar_static_bool[270]{((v721*(((v5508*v7215)-(v5497*v7253))/v7305))/v7313)}else{v6721});
        let v7317=(if self.scalar_static_bool[270]{((v721*(((v5508*v7216)-(v5497*v7254))/v7305))/v7313)}else{v6722});
        let v7328=(if self.scalar_static_bool[270]{((v60*((v5508*v7224)+(v5499*v7253)))-v7248)}else{v6733});
        let v7329=(if self.scalar_static_bool[270]{((v60*((v5508*v7225)+(v5499*v7254)))-v7249)}else{v6734});
        let v7346=(if self.scalar_static_bool[270]{((((v5531*v7253)+(v5508*(self.scalar_static_f64[705]*v7224)))-(self.scalar_static_f64[705]*v7248))+(v3*v7265))}else{v6751});
        let v7347=(if self.scalar_static_bool[270]{((((v5531*v7254)+(v5508*(self.scalar_static_f64[705]*v7225)))-(self.scalar_static_f64[705]*v7249))+(v3*v7268))}else{v6752});
        let v7354=(if self.scalar_static_bool[270]{((v5538*v7316)+(v5526*v7328))}else{v6759});
        let v7355=(if self.scalar_static_bool[270]{((v5538*v7317)+(v5526*v7329))}else{v6760});
        let v7356=(v5540*v7354);
        let v7358=(v5540*v7355);
        let v7360=(if self.scalar_static_bool[270]{(v7356+v7356)}else{v6765});
        let v7361=(if self.scalar_static_bool[270]{(v7358+v7358)}else{v6766});
        let v7378=(v7346+(-v7360));
        let v7379=(v7347+(-v7361));
        let v7384=(-v7378);
        let v7385=(-v7379);
        let v7404=(v5571*v5571);
        let v7409=(if v5563{((-(v407*((v5569*v7384)+(v5564*(v3*((v5566*v7384)+(v5564*(v410*v7384))))))))/v7404)}else{(if v5559{(v5560*v7378)}else{v7175})});
        let v7410=(if v5563{((-(v407*((v5569*v7385)+(v5564*(v3*((v5566*v7385)+(v5564*(v410*v7385))))))))/v7404)}else{(if v5559{(v5560*v7379)}else{v7176})});
        let v7445=(-v7346);
        let v7446=(-v7347);
        let v7465=(v5598*v5598);
        let v7470=(if v5590{((-(v407*((v5596*v7445)+(v5591*(v3*((v5593*v7445)+(v5591*(v410*v7445))))))))/v7465)}else{(if v5586{(v5587*v7346)}else{v7409})});
        let v7471=(if v5590{((-(v407*((v5596*v7446)+(v5591*(v3*((v5593*v7446)+(v5591*(v410*v7446))))))))/v7465)}else{(if v5586{(v5587*v7347)}else{v7410})});
        let v7511=(self.scalar_static_f64[44]*v6914);
        let v7512=(self.scalar_static_f64[44]*v6915);
        let v7513=(v60*v5618);
        let v7519=(self.scalar_static_f64[21]*f64::powf(v5617,self.scalar_static_f64[600]));
        let v7522=(if self.scalar_static_bool[276]{(v7511*v7519)}else{(if self.scalar_static_bool[275]{(v7511/v7513)}else{v7470})});
        let v7523=(if self.scalar_static_bool[276]{(v7512*v7519)}else{(if self.scalar_static_bool[275]{(v7512/v7513)}else{v7471})});
        let v7529=(v5622*v5622);
        let v7537=(if self.scalar_static_bool[274]{(self.scalar_static_f64[26]*(((v5622*(self.scalar_static_f64[39]*v6914))-(v5623*v7522))/v7529))}else{v6942});
        let v7538=(if self.scalar_static_bool[274]{(self.scalar_static_f64[26]*(((v5622*(self.scalar_static_f64[39]*v6915))-(v5623*v7523))/v7529))}else{v6943});
        let v7541=(v5626*v5626);
        let v7542=((-(self.scalar_static_f64[1243]*v7537))/v7541);
        let v7545=((-(self.scalar_static_f64[1243]*v7538))/v7541);
        let v7550=(-v7542);
        let v7551=(-v7545);
        let v7570=(v5646*v5646);
        let v7595=(if v5650{(v422*((v5656*v7542)+(v5651*(v3*((v5653*v7542)+(v5651*(v410*v7542)))))))}else{(if v5638{((-(v407*((v5644*v7550)+(v5639*(v3*((v5641*v7550)+(v5639*(v410*v7550))))))))/v7570)}else{(if v5631{(v5632*v7542)}else{v7522})})});
        let v7596=(if v5650{(v422*((v5656*v7545)+(v5651*(v3*((v5653*v7545)+(v5651*(v410*v7545)))))))}else{(if v5638{((-(v407*((v5644*v7551)+(v5639*(v3*((v5641*v7551)+(v5639*(v410*v7551))))))))/v7570)}else{(if v5631{(v5632*v7545)}else{v7523})})});
        let v7621=(self.scalar_static_f64[65]*v6507);
        let v7622=(self.scalar_static_f64[65]*v6508);
        let v7623=(v5673*v7621);
        let v7625=(v5673*v7622);
        let v7641=(if v5678{v1}else{(if v5672{((v5675*v7621)+(v5673*((v5674*v7621)+(v5673*(v7623+v7623)))))}else{v7595})});
        let v7642=(if v5678{v1}else{(if v5672{((v5675*v7622)+(v5673*((v5674*v7622)+(v5673*(v7625+v7625)))))}else{v7596})});
        let v7670=(-(self.scalar_static_f64[678]*v6334));
        let v7671=(-(self.scalar_static_f64[678]*v6335));
        let v7672=(v60*v5700);
        let v7678=(self.scalar_static_f64[22]*f64::powf(v5699,self.scalar_static_f64[588]));
        let v7681=(if self.scalar_static_bool[280]{(v7670*v7678)}else{(if self.scalar_static_bool[279]{(v7670/v7672)}else{v7641})});
        let v7682=(if self.scalar_static_bool[280]{(v7671*v7678)}else{(if self.scalar_static_bool[279]{(v7671/v7672)}else{v7642})});
        let v7703=(if self.scalar_static_bool[284]{v6515}else{v7108});
        let v7704=(if self.scalar_static_bool[284]{v6516}else{v7109});
        let v7708=(v5720*v5720);
        let v7758=(self.scalar_static_f64[46]*v7703);
        let v7759=(self.scalar_static_f64[46]*v7704);
        let v7760=(v60*v5740);
        let v7767=(self.scalar_static_f64[23]*f64::powf(v5739,self.scalar_static_f64[602]));
        let v7770=(if self.scalar_static_bool[286]{(v7758*v7767)}else{(if self.scalar_static_bool[285]{(v7758/v7760)}else{v7681})});
        let v7771=(if self.scalar_static_bool[286]{(v7759*v7767)}else{(if self.scalar_static_bool[285]{(v7759/v7760)}else{v7682})});
        let v7774=(if self.scalar_static_bool[284]{(self.scalar_static_f64[37]*v7770)}else{v7179});
        let v7775=(if self.scalar_static_bool[284]{(self.scalar_static_f64[37]*v7771)}else{v7180});
        let v7810=(if self.scalar_static_bool[288]{(self.scalar_static_f64[721]*(((v5720*(self.scalar_static_f64[24]*v7774))-(v5755*v7703))/v7708))}else{v7215});
        let v7811=(if self.scalar_static_bool[288]{(self.scalar_static_f64[721]*(((v5720*(self.scalar_static_f64[24]*v7775))-(v5755*v7704))/v7708))}else{v7216});
        let v7814=(v5758*v5758);
        let v7819=(if self.scalar_static_bool[288]{((-(self.scalar_static_f64[1327]*v7810))/v7814)}else{v7224});
        let v7820=(if self.scalar_static_bool[288]{((-(self.scalar_static_f64[1327]*v7811))/v7814)}else{v7225});
        let v7821=(v5760*v7819);
        let v7823=(v5760*v7820);
        let v7827=(v5762*(if self.scalar_static_bool[288]{(v7821+v7821)}else{v7230}));
        let v7828=(v7827+v7827);
        let v7829=(v5762*(if self.scalar_static_bool[288]{(v7823+v7823)}else{v7231}));
        let v7830=(v7829+v7829);
        let v7834=(v5764*v5764);
        let v7840=(v60*v5766);
        let v7843=(if self.scalar_static_bool[288]{((((v5764*v7828)-(v5763*v7828))/v7834)/v7840)}else{v7248});
        let v7844=(if self.scalar_static_bool[288]{((((v5764*v7830)-(v5763*v7830))/v7834)/v7840)}else{v7249});
        let v7845=(v60*v5768);
        let v7848=(if self.scalar_static_bool[288]{(v7843/v7845)}else{v7253});
        let v7849=(if self.scalar_static_bool[288]{(v7844/v7845)}else{v7254});
        let v7860=((v5771*v7810)+(v5758*(if self.scalar_static_bool[288]{((v5769*v7843)+(v5767*v7848))}else{v7261})));
        let v7863=((v5771*v7811)+(v5758*(if self.scalar_static_bool[288]{((v5769*v7844)+(v5767*v7849))}else{v7262})));
        let v7900=(v5769*v5769);
        let v7908=(v60*v5786);
        let v7911=(if self.scalar_static_bool[288]{((v721*(((v5769*v7810)-(v5758*v7848))/v7900))/v7908)}else{v7316});
        let v7912=(if self.scalar_static_bool[288]{((v721*(((v5769*v7811)-(v5758*v7849))/v7900))/v7908)}else{v7317});
        let v7941=(if self.scalar_static_bool[288]{((((v5792*v7848)+(v5769*(self.scalar_static_f64[706]*v7819)))-(self.scalar_static_f64[706]*v7843))+(v3*v7860))}else{v7346});
        let v7942=(if self.scalar_static_bool[288]{((((v5792*v7849)+(v5769*(self.scalar_static_f64[706]*v7820)))-(self.scalar_static_f64[706]*v7844))+(v3*v7863))}else{v7347});
        let v7949=(if self.scalar_static_bool[288]{((v5799*v7911)+(v5787*(if self.scalar_static_bool[288]{((v60*((v5769*v7819)+(v5760*v7848)))-v7843)}else{v7328})))}else{v7354});
        let v7950=(if self.scalar_static_bool[288]{((v5799*v7912)+(v5787*(if self.scalar_static_bool[288]{((v60*((v5769*v7820)+(v5760*v7849)))-v7844)}else{v7329})))}else{v7355});
        let v7951=(v5801*v7949);
        let v7953=(v5801*v7950);
        let v7973=(v7941+(-(if self.scalar_static_bool[288]{(v7951+v7951)}else{v7360})));
        let v7974=(v7942+(-(if self.scalar_static_bool[288]{(v7953+v7953)}else{v7361})));
        let v7979=(-v7973);
        let v7980=(-v7974);
        let v7999=(v5832*v5832);
        let v8004=(if v5824{((-(v407*((v5830*v7979)+(v5825*(v3*((v5827*v7979)+(v5825*(v410*v7979))))))))/v7999)}else{(if v5820{(v5821*v7973)}else{v7770})});
        let v8005=(if v5824{((-(v407*((v5830*v7980)+(v5825*(v3*((v5827*v7980)+(v5825*(v410*v7980))))))))/v7999)}else{(if v5820{(v5821*v7974)}else{v7771})});
        let v8040=(-v7941);
        let v8041=(-v7942);
        let v8060=(v5859*v5859);
        let v8065=(if v5851{((-(v407*((v5857*v8040)+(v5852*(v3*((v5854*v8040)+(v5852*(v410*v8040))))))))/v8060)}else{(if v5847{(v5848*v7941)}else{v8004})});
        let v8066=(if v5851{((-(v407*((v5857*v8041)+(v5852*(v3*((v5854*v8041)+(v5852*(v410*v8041))))))))/v8060)}else{(if v5847{(v5848*v7942)}else{v8005})});
        let v8106=(self.scalar_static_f64[46]*v6914);
        let v8107=(self.scalar_static_f64[46]*v6915);
        let v8108=(v60*v5879);
        let v8114=(self.scalar_static_f64[23]*f64::powf(v5878,self.scalar_static_f64[602]));
        let v8117=(if self.scalar_static_bool[294]{(v8106*v8114)}else{(if self.scalar_static_bool[293]{(v8106/v8108)}else{v8065})});
        let v8118=(if self.scalar_static_bool[294]{(v8107*v8114)}else{(if self.scalar_static_bool[293]{(v8107/v8108)}else{v8066})});
        let v8124=(v5883*v5883);
        let v8132=(if self.scalar_static_bool[292]{(self.scalar_static_f64[27]*(((v5883*(self.scalar_static_f64[40]*v6914))-(v5884*v8117))/v8124))}else{v7537});
        let v8133=(if self.scalar_static_bool[292]{(self.scalar_static_f64[27]*(((v5883*(self.scalar_static_f64[40]*v6915))-(v5884*v8118))/v8124))}else{v7538});
        let v8136=(v5887*v5887);
        let v8137=((-(self.scalar_static_f64[1434]*v8132))/v8136);
        let v8140=((-(self.scalar_static_f64[1434]*v8133))/v8136);
        let v8145=(-v8137);
        let v8146=(-v8140);
        let v8165=(v5907*v5907);
        let v8190=(if v5911{(v422*((v5917*v8137)+(v5912*(v3*((v5914*v8137)+(v5912*(v410*v8137)))))))}else{(if v5899{((-(v407*((v5905*v8145)+(v5900*(v3*((v5902*v8145)+(v5900*(v410*v8145))))))))/v8165)}else{(if v5892{(v5893*v8137)}else{v8117})})});
        let v8191=(if v5911{(v422*((v5917*v8140)+(v5912*(v3*((v5914*v8140)+(v5912*(v410*v8140)))))))}else{(if v5899{((-(v407*((v5905*v8146)+(v5900*(v3*((v5902*v8146)+(v5900*(v410*v8146))))))))/v8165)}else{(if v5892{(v5893*v8140)}else{v8118})})});
        let v8216=(self.scalar_static_f64[67]*v6507);
        let v8217=(self.scalar_static_f64[67]*v6508);
        let v8218=(v5934*v8216);
        let v8220=(v5934*v8217);
        let v8236=(if v5939{v1}else{(if v5933{((v5936*v8216)+(v5934*((v5935*v8216)+(v5934*(v8218+v8218)))))}else{v8190})});
        let v8237=(if v5939{v1}else{(if v5933{((v5936*v8217)+(v5934*((v5935*v8217)+(v5934*(v8220+v8220)))))}else{v8191})});
        let v8285=(if self.scalar_static_bool[297]{(if v5960{(if v5966{v1}else{(self.scalar_static_f64[571]*((v5967*self.scalar_static_f64[604])/v5968))})}else{(if v5973{self.scalar_static_f64[564]}else{(self.scalar_static_f64[564]+(self.scalar_static_f64[571]*((v5976*self.scalar_static_f64[605])/v5977)))})})}else{v1});
        let v8286=(if self.scalar_static_bool[297]{(if v5960{(if v5966{v1}else{(self.scalar_static_f64[571]*((v5967*self.scalar_static_f64[605])/v5968))})}else{(if v5973{self.scalar_static_f64[578]}else{(self.scalar_static_f64[578]+(self.scalar_static_f64[571]*((v5976*self.scalar_static_f64[604])/v5977)))})})}else{v1});
        let v8287=(if self.scalar_static_bool[297]{v8285}else{self.scalar_static_f64[590]});
        let v8288=(if self.scalar_static_bool[297]{v8286}else{self.scalar_static_f64[591]});
        let v8289=(if self.scalar_static_bool[297]{v8287}else{self.scalar_static_f64[592]});
        let v8290=(if self.scalar_static_bool[297]{v8288}else{self.scalar_static_f64[593]});
        let v8293=(if self.scalar_static_bool[297]{(-v8287)}else{self.scalar_static_f64[596]});
        let v8294=(if self.scalar_static_bool[297]{(-v8288)}else{self.scalar_static_f64[597]});
        let v8295=(v5992*v8293);
        let v8297=(v5992*v8294);
        let v8299=(v60*v5995);
        let v8302=(if self.scalar_static_bool[297]{((v8295+v8295)/v8299)}else{v6319});
        let v8303=(if self.scalar_static_bool[297]{((v8297+v8297)/v8299)}else{v6320});
        let v8311=(v5998*v5998);
        let v8319=(if self.scalar_static_bool[297]{(v60*(((v5998*(self.scalar_static_f64[828]*v8285))-(v5997*(v8289+v8302)))/v8311))}else{v1});
        let v8320=(if self.scalar_static_bool[297]{(v60*(((v5998*(self.scalar_static_f64[828]*v8286))-(v5997*(v8290+v8303)))/v8311))}else{v1});
        let v8323=(-(self.scalar_static_f64[679]*v8319));
        let v8324=(-(self.scalar_static_f64[679]*v8320));
        let v8325=(v60*v6005);
        let v8331=(self.scalar_static_f64[24]*f64::powf(v6004,self.scalar_static_f64[589]));
        let v8334=(if self.scalar_static_bool[299]{(v8323*v8331)}else{(if self.scalar_static_bool[298]{(v8323/v8325)}else{v8236})});
        let v8335=(if self.scalar_static_bool[299]{(v8324*v8331)}else{(if self.scalar_static_bool[298]{(v8324/v8325)}else{v8237})});
        let v8348=(if self.scalar_static_bool[297]{(self.scalar_static_f64[569]*((self.scalar_static_f64[694]*(-v8334))+(self.scalar_static_f64[697]*(v8285-v8319))))}else{(if self.scalar_static_bool[281]{v1}else{(if self.scalar_static_bool[749]{((self.scalar_static_f64[694]*(-(if self.scalar_static_bool[751]{(v6280*v6289)}else{(if self.scalar_static_bool[750]{(v6280/v6282)}else{v6266})})))+(self.scalar_static_f64[697]*v6244))}else{v1})})});
        let v8349=(if self.scalar_static_bool[297]{(self.scalar_static_f64[569]*((self.scalar_static_f64[694]*(-v8335))+(self.scalar_static_f64[697]*(v8286-v8320))))}else{(if self.scalar_static_bool[281]{v1}else{(if self.scalar_static_bool[749]{((self.scalar_static_f64[694]*(-(if self.scalar_static_bool[751]{(v6281*v6289)}else{(if self.scalar_static_bool[750]{(v6281/v6282)}else{v6267})})))+(self.scalar_static_f64[697]*v6245))}else{v1})})});
        let v8352=(if self.scalar_static_bool[297]{(self.scalar_static_f64[564]-v8285)}else{v8285});
        let v8353=(if self.scalar_static_bool[297]{(self.scalar_static_f64[578]-v8286)}else{v8286});
        let v8354=(if self.scalar_static_bool[297]{v8352}else{v8287});
        let v8355=(if self.scalar_static_bool[297]{v8353}else{v8288});
        let v8362=(v6028*(if self.scalar_static_bool[297]{(-v8354)}else{v8293}));
        let v8364=(v6028*(if self.scalar_static_bool[297]{(-v8355)}else{v8294}));
        let v8366=(v60*v6031);
        let v8378=(v6034*v6034);
        let v8386=(if self.scalar_static_bool[297]{(v60*(((v6034*(self.scalar_static_f64[828]*v8352))-(v6033*((if self.scalar_static_bool[297]{v8354}else{v8289})+(if self.scalar_static_bool[297]{((v8362+v8362)/v8366)}else{v8302}))))/v8378))}else{v8319});
        let v8387=(if self.scalar_static_bool[297]{(v60*(((v6034*(self.scalar_static_f64[828]*v8353))-(v6033*((if self.scalar_static_bool[297]{v8355}else{v8290})+(if self.scalar_static_bool[297]{((v8364+v8364)/v8366)}else{v8303}))))/v8378))}else{v8320});
        let v8390=(-(self.scalar_static_f64[757]*v8386));
        let v8391=(-(self.scalar_static_f64[757]*v8387));
        let v8392=(v60*v6043);
        let v8399=(self.scalar_static_f64[112]*f64::powf(v6042,self.scalar_static_f64[606]));
        let v8402=(if self.scalar_static_bool[303]{(v8390*v8399)}else{(if self.scalar_static_bool[301]{(v8390/v8392)}else{v8334})});
        let v8403=(if self.scalar_static_bool[303]{(v8391*v8399)}else{(if self.scalar_static_bool[301]{(v8391/v8392)}else{v8335})});
        let v8424=(-(self.scalar_static_f64[679]*v6334));
        let v8425=(-(self.scalar_static_f64[679]*v6335));
        let v8426=(v60*v6063);
        let v8432=(self.scalar_static_f64[24]*f64::powf(v6062,self.scalar_static_f64[589]));
        let v8473=(self.scalar_static_f64[575]*(((self.scalar_static_f64[143]*(if self.scalar_static_bool[249]{(self.scalar_static_f64[569]*((self.scalar_static_f64[690]*(-v7084))+(self.scalar_static_f64[695]*v7090)))}else{(if self.scalar_static_bool[248]{v1}else{(if self.scalar_static_bool[741]{((self.scalar_static_f64[690]*(-v6238))+(self.scalar_static_f64[695]*v6244))}else{v1})})}))+(self.scalar_static_f64[145]*(if self.scalar_static_bool[264]{(self.scalar_static_f64[569]*((self.scalar_static_f64[692]*(-v7681))+(self.scalar_static_f64[696]*v7090)))}else{(if self.scalar_static_bool[263]{v1}else{(if self.scalar_static_bool[745]{((self.scalar_static_f64[692]*(-v6266))+(self.scalar_static_f64[696]*v6244))}else{v1})})})))+(self.scalar_static_f64[147]*(if self.scalar_static_bool[305]{(self.scalar_static_f64[569]*((self.scalar_static_f64[694]*(-(if self.scalar_static_bool[307]{(v8424*v8432)}else{(if self.scalar_static_bool[306]{(v8424/v8426)}else{v8402})})))+(self.scalar_static_f64[697]*v7090)))}else{(if self.scalar_static_bool[297]{(v8348+(if self.scalar_static_bool[297]{(self.scalar_static_f64[569]*((self.scalar_static_f64[764]*(-v8402))+(self.scalar_static_f64[766]*(v8352-v8386))))}else{v1}))}else{v8348})}))));
        let v8474=(self.scalar_static_f64[575]*(((self.scalar_static_f64[143]*(if self.scalar_static_bool[249]{(self.scalar_static_f64[569]*((self.scalar_static_f64[690]*(-v7085))+(self.scalar_static_f64[695]*v7091)))}else{(if self.scalar_static_bool[248]{v1}else{(if self.scalar_static_bool[741]{((self.scalar_static_f64[690]*(-v6239))+(self.scalar_static_f64[695]*v6245))}else{v1})})}))+(self.scalar_static_f64[145]*(if self.scalar_static_bool[264]{(self.scalar_static_f64[569]*((self.scalar_static_f64[692]*(-v7682))+(self.scalar_static_f64[696]*v7091)))}else{(if self.scalar_static_bool[263]{v1}else{(if self.scalar_static_bool[745]{((self.scalar_static_f64[692]*(-v6267))+(self.scalar_static_f64[696]*v6245))}else{v1})})})))+(self.scalar_static_f64[147]*(if self.scalar_static_bool[305]{(self.scalar_static_f64[569]*((self.scalar_static_f64[694]*(-(if self.scalar_static_bool[307]{(v8425*v8432)}else{(if self.scalar_static_bool[306]{(v8425/v8426)}else{v8403})})))+(self.scalar_static_f64[697]*v7091)))}else{(if self.scalar_static_bool[297]{(v8349+(if self.scalar_static_bool[297]{(self.scalar_static_f64[569]*((self.scalar_static_f64[764]*(-v8403))+(self.scalar_static_f64[766]*(v8353-v8387))))}else{v1}))}else{v8349})}))));

        CommonStampValues {
            v1,
            v5,
            v60,
            v407,
            v408,
            v4925,
            v4926,
            v4969,
            v5134,
            v5136,
            v5168,
            v5192,
            v5200,
            v5224,
            v5251,
            v5265,
            v5279,
            v5283,
            v5290,
            v5312,
            v5339,
            v5363,
            v5397,
            v5406,
            v5408,
            v5418,
            v5459,
            v5484,
            v5512,
            v5526,
            v5540,
            v5544,
            v5551,
            v5573,
            v5600,
            v5626,
            v5660,
            v5669,
            v5671,
            v5681,
            v5720,
            v5745,
            v5773,
            v5787,
            v5801,
            v5805,
            v5812,
            v5834,
            v5861,
            v5887,
            v5921,
            v5930,
            v5932,
            v5942,
            v6091,
            v6404,
            v6405,
            v6409,
            v6410,
            v6460,
            v6461,
            v6507,
            v6508,
            v6517,
            v6518,
            v6522,
            v6586,
            v6587,
            v6670,
            v6673,
            v6721,
            v6722,
            v6759,
            v6760,
            v6814,
            v6815,
            v6875,
            v6876,
            v6942,
            v6943,
            v7000,
            v7001,
            v7044,
            v7045,
            v7108,
            v7109,
            v7113,
            v7179,
            v7180,
            v7265,
            v7268,
            v7316,
            v7317,
            v7354,
            v7355,
            v7409,
            v7410,
            v7470,
            v7471,
            v7537,
            v7538,
            v7595,
            v7596,
            v7641,
            v7642,
            v7703,
            v7704,
            v7708,
            v7774,
            v7775,
            v7860,
            v7863,
            v7911,
            v7912,
            v7949,
            v7950,
            v8004,
            v8005,
            v8065,
            v8066,
            v8132,
            v8133,
            v8190,
            v8191,
            v8236,
            v8237,
            v8473,
            v8474,
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
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
        let v58=0.29214664;
        let v59=0.5178164370971076;
        let v62=0.26992878119627894;
        let v63=0.43792457880372104;
        let v803=0.886226925452758;
        let v4927=(if (self.scalar_static_f64[177]!=0.0){common.v4926}else{common.v1});
        let v4928=(v4927<common.v408);
        let v4930=(common.v5+(common.v408-v4927));
        let v4932=(v4927>self.scalar_static_f64[4238]);
        let v4936=(v4927).exp();
        let v4939=(if (self.scalar_static_f64[177]!=0.0){(if v4928{(common.v407/v4930)}else{(if v4932{(self.scalar_static_f64[4240]*(common.v5+(v4927-self.scalar_static_f64[4238])))}else{v4936})})}else{common.v1});
        let v4944=(if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4123]*common.v4926)}else{v4927});
        let v4945=(v4944<common.v408);
        let v4947=(common.v5+(common.v408-v4944));
        let v4949=(v4944>self.scalar_static_f64[4242]);
        let v4953=(v4944).exp();
        let v4956=(if (self.scalar_static_f64[177]!=0.0){(if v4945{(common.v407/v4947)}else{(if v4949{(self.scalar_static_f64[4244]*(common.v5+(v4944-self.scalar_static_f64[4242])))}else{v4953})})}else{v4939});
        let v4964=(self.scalar_static_f64[4210]+(self.scalar_static_f64[4202]*common.v4925));
        let v4972=(if self.scalar_static_bool[739]{(self.scalar_static_f64[4202]*(self.scalar_static_f64[612]*common.v4969))}else{v4944});
        let v4973=(v4972<common.v408);
        let v4975=(common.v5+(common.v408-v4972));
        let v4977=(v4972>self.scalar_static_f64[4246]);
        let v4981=(v4972).exp();
        let v5140=(if self.scalar_static_bool[247]{(common.v5134-common.v5)}else{common.v5134});
        let v5197=(if self.scalar_static_bool[249]{(self.scalar_static_f64[638]*v5140)}else{common.v1});
        let v5203=((common.v5-(common.v5168/common.v5200))).sqrt();
        let v5205=(if self.scalar_static_bool[250]{(common.v5-v5203)}else{common.v1});
        let v5208=(v5205*v5205);
        let v5209=(v5205).ln();
        let v5210=(v5208*v5209);
        let v5211=(common.v5-v5205);
        let v5215=(if self.scalar_static_bool[252]{(self.scalar_static_f64[217]*(v5205+(v5210/v5211)))}else{common.v1});
        let v5217=(if self.scalar_static_bool[250]{(v5205+v5215)}else{common.v1});
        let v5225=(common.v5136-common.v5);
        let v5228=(if self.scalar_static_bool[250]{(self.scalar_static_f64[626]*(common.v5224*v5225))}else{common.v1});
        let v5231=(if self.scalar_static_bool[250]{(self.scalar_static_f64[212]*(v5217*v5228))}else{common.v1});
        let v5252=(common.v5+common.v5251);
        let v5257=(if self.scalar_static_bool[255]{f64::powf(v5252,self.scalar_static_f64[220])}else{(if self.scalar_static_bool[254]{(common.v5/v5252)}else{common.v1})});
        let v5258=(v5217*v5257);
        let v5259=(v5217+v5257);
        let v5261=(if self.scalar_static_bool[253]{(v5258/v5259)}else{common.v1});
        let v5284=(self.scalar_static_bool[253]&&(common.v5283!=0.0));
        let v5285=(v59*common.v5279);
        let v5286=(common.v5+v5285);
        let v5291=(common.v5-v5285);
        let v5293=(if common.v5290{(common.v5/v5291)}else{(if v5284{(common.v5/v5286)}else{common.v1})});
        let v5314=(v5293*v5293);
        let v5319=(((v58*v5293)+(v62*v5314))+(v63*(v5293*v5314)));
        let v5321=(if self.scalar_static_bool[253]{(common.v5312*v5319)}else{common.v1});
        let v5342=(if common.v5290{((common.v60*common.v5339)-v5321)}else{(if v5284{v5321}else{common.v1})});
        let v5343=(self.scalar_static_f64[704]*v5342);
        let v5346=(if self.scalar_static_bool[253]{(v803*(v5343/common.v5265))}else{common.v1});
        let v5347=(v5228*v5346);
        let v5350=(if self.scalar_static_bool[253]{(self.scalar_static_f64[213]*(v5261*v5347))}else{common.v1});
        let v5398=(common.v4925*common.v5363);
        let v5399=(common.v5363*v5398);
        let v5402=(if self.scalar_static_bool[256]{(self.scalar_static_f64[222]*(common.v5397*v5399))}else{common.v1});
        let v5419=(common.v5-common.v5418);
        let v5423=(self.scalar_static_bool[260]&&(!(common.v5406!=0.0)));
        let v5427=(if v5423{(self.scalar_static_f64[53]+(self.scalar_static_f64[74]*(self.scalar_static_f64[241]+common.v5192)))}else{(if common.v5408{(common.v5/v5419)}else{self.scalar_static_f64[568]})});
        let v5431=(self.scalar_static_f64[245]*(v5402+(v5350+(v5197+v5231))));
        let v5454=(if self.scalar_static_bool[264]{(self.scalar_static_f64[640]*v5140)}else{v5197});
        let v5462=((common.v5-(common.v5168/common.v5459))).sqrt();
        let v5464=(if self.scalar_static_bool[266]{(common.v5-v5462)}else{v5205});
        let v5468=(v5464*v5464);
        let v5469=(v5464).ln();
        let v5470=(v5468*v5469);
        let v5471=(common.v5-v5464);
        let v5475=(if self.scalar_static_bool[268]{(self.scalar_static_f64[251]*(v5464+(v5470/v5471)))}else{(if self.scalar_static_bool[267]{common.v1}else{v5215})});
        let v5477=(if self.scalar_static_bool[266]{(v5464+v5475)}else{v5217});
        let v5487=(if self.scalar_static_bool[266]{(self.scalar_static_f64[631]*(v5225*common.v5484))}else{v5228});
        let v5490=(if self.scalar_static_bool[266]{(self.scalar_static_f64[246]*(v5477*v5487))}else{(if self.scalar_static_bool[265]{common.v1}else{v5231})});
        let v5513=(common.v5+common.v5512);
        let v5518=(if self.scalar_static_bool[272]{f64::powf(v5513,self.scalar_static_f64[254])}else{(if self.scalar_static_bool[271]{(common.v5/v5513)}else{v5257})});
        let v5519=(v5477*v5518);
        let v5520=(v5477+v5518);
        let v5522=(if self.scalar_static_bool[270]{(v5519/v5520)}else{v5261});
        let v5545=(self.scalar_static_bool[270]&&(common.v5544!=0.0));
        let v5546=(v59*common.v5540);
        let v5547=(common.v5+v5546);
        let v5552=(common.v5-v5546);
        let v5554=(if common.v5551{(common.v5/v5552)}else{(if v5545{(common.v5/v5547)}else{v5293})});
        let v5575=(v5554*v5554);
        let v5580=(((v58*v5554)+(v62*v5575))+(v63*(v5554*v5575)));
        let v5582=(if self.scalar_static_bool[270]{(common.v5573*v5580)}else{v5321});
        let v5603=(if common.v5551{((common.v60*common.v5600)-v5582)}else{(if v5545{v5582}else{v5342})});
        let v5604=(self.scalar_static_f64[705]*v5603);
        let v5607=(if self.scalar_static_bool[270]{(v803*(v5604/common.v5526))}else{v5346});
        let v5608=(v5487*v5607);
        let v5611=(if self.scalar_static_bool[270]{(self.scalar_static_f64[247]*(v5522*v5608))}else{(if self.scalar_static_bool[269]{common.v1}else{v5350})});
        let v5661=(common.v4925*common.v5626);
        let v5662=(common.v5626*v5661);
        let v5665=(if self.scalar_static_bool[274]{(self.scalar_static_f64[256]*(common.v5660*v5662))}else{(if self.scalar_static_bool[273]{common.v1}else{v5402})});
        let v5682=(common.v5-common.v5681);
        let v5686=(self.scalar_static_bool[278]&&(!(common.v5669!=0.0)));
        let v5690=(if v5686{(self.scalar_static_f64[57]+(self.scalar_static_f64[81]*(self.scalar_static_f64[273]+common.v5192)))}else{(if common.v5671{(common.v5/v5682)}else{(if self.scalar_static_bool[277]{common.v5}else{v5427})})});
        let v5694=(self.scalar_static_f64[245]*(v5665+(v5611+(v5454+v5490))));
        let v5723=((common.v5-(common.v5168/common.v5720))).sqrt();
        let v5725=(if self.scalar_static_bool[284]{(common.v5-v5723)}else{v5464});
        let v5729=(v5725*v5725);
        let v5730=(v5725).ln();
        let v5731=(v5729*v5730);
        let v5732=(common.v5-v5725);
        let v5738=(if self.scalar_static_bool[284]{(v5725+(if self.scalar_static_bool[286]{(self.scalar_static_f64[282]*(v5725+(v5731/v5732)))}else{(if self.scalar_static_bool[285]{common.v1}else{v5475})}))}else{v5477});
        let v5748=(if self.scalar_static_bool[284]{(self.scalar_static_f64[636]*(v5225*common.v5745))}else{v5487});
        let v5774=(common.v5+common.v5773);
        let v5779=(if self.scalar_static_bool[290]{f64::powf(v5774,self.scalar_static_f64[285])}else{(if self.scalar_static_bool[289]{(common.v5/v5774)}else{v5518})});
        let v5780=(v5738*v5779);
        let v5781=(v5738+v5779);
        let v5783=(if self.scalar_static_bool[288]{(v5780/v5781)}else{v5522});
        let v5806=(self.scalar_static_bool[288]&&(common.v5805!=0.0));
        let v5807=(v59*common.v5801);
        let v5808=(common.v5+v5807);
        let v5813=(common.v5-v5807);
        let v5815=(if common.v5812{(common.v5/v5813)}else{(if v5806{(common.v5/v5808)}else{v5554})});
        let v5836=(v5815*v5815);
        let v5841=(((v58*v5815)+(v62*v5836))+(v63*(v5815*v5836)));
        let v5843=(if self.scalar_static_bool[288]{(common.v5834*v5841)}else{v5582});
        let v5865=(self.scalar_static_f64[706]*(if common.v5812{((common.v60*common.v5861)-v5843)}else{(if v5806{v5843}else{v5603})}));
        let v5868=(if self.scalar_static_bool[288]{(v803*(v5865/common.v5787))}else{v5607});
        let v5869=(v5748*v5868);
        let v5922=(common.v4925*common.v5887);
        let v5923=(common.v5887*v5922);
        let v5943=(common.v5-common.v5942);
        let v5947=(self.scalar_static_bool[296]&&(!(common.v5930!=0.0)));
        let v5951=(if v5947{(self.scalar_static_f64[61]+(self.scalar_static_f64[88]*(self.scalar_static_f64[304]+common.v5192)))}else{(if common.v5932{(common.v5/v5943)}else{(if self.scalar_static_bool[295]{common.v5}else{v5690})})});
        let v5955=(self.scalar_static_f64[245]*((if self.scalar_static_bool[292]{(self.scalar_static_f64[287]*(common.v5921*v5923))}else{(if self.scalar_static_bool[291]{common.v1}else{v5665})})+((if self.scalar_static_bool[288]{(self.scalar_static_f64[278]*(v5783*v5869))}else{(if self.scalar_static_bool[287]{common.v1}else{v5611})})+((if self.scalar_static_bool[282]{(self.scalar_static_f64[642]*v5140)}else{v5454})+(if self.scalar_static_bool[284]{(self.scalar_static_f64[277]*(v5738*v5748))}else{(if self.scalar_static_bool[283]{common.v1}else{v5490})})))));
        let v6101=(v4930*v4930);
        let v6114=(if (self.scalar_static_f64[177]!=0.0){(if v4928{(self.scalar_static_f64[4273]/v6101)}else{(if v4932{self.scalar_static_f64[4276]}else{(v4936*self.scalar_static_f64[4268])})})}else{common.v1});
        let v6115=(if (self.scalar_static_f64[177]!=0.0){(if v4928{(self.scalar_static_f64[4275]/v6101)}else{(if v4932{self.scalar_static_f64[4277]}else{(v4936*self.scalar_static_f64[4269])})})}else{common.v1});
        let v6128=(v4947*v4947);
        let v6141=(if (self.scalar_static_f64[177]!=0.0){(if v4945{(self.scalar_static_f64[4285]/v6128)}else{(if v4949{self.scalar_static_f64[4288]}else{(v4953*self.scalar_static_f64[4280])})})}else{v6114});
        let v6142=(if (self.scalar_static_f64[177]!=0.0){(if v4945{(self.scalar_static_f64[4287]/v6128)}else{(if v4949{self.scalar_static_f64[4289]}else{(v4953*self.scalar_static_f64[4281])})})}else{v6115});
        let v6165=(v4975*v4975);
        let v6513=(if self.scalar_static_bool[249]{(self.scalar_static_f64[638]*common.v6404)}else{common.v1});
        let v6514=(if self.scalar_static_bool[249]{(self.scalar_static_f64[638]*common.v6405)}else{common.v1});
        let v6530=(common.v60*v5203);
        let v6535=(if self.scalar_static_bool[250]{(-((-(((common.v5200*common.v6460)-(common.v5168*common.v6517))/common.v6522))/v6530))}else{common.v1});
        let v6536=(if self.scalar_static_bool[250]{(-((-(((common.v5200*common.v6461)-(common.v5168*common.v6518))/common.v6522))/v6530))}else{common.v1});
        let v6537=(v5205*v6535);
        let v6539=(v5205*v6536);
        let v6554=(v5211*v5211);
        let v6564=(if self.scalar_static_bool[252]{(self.scalar_static_f64[217]*(v6535+(((v5211*((v5209*(v6537+v6537))+(v5208*(v6535/v5205))))-(v5210*(-v6535)))/v6554)))}else{common.v1});
        let v6565=(if self.scalar_static_bool[252]{(self.scalar_static_f64[217]*(v6536+(((v5211*((v5209*(v6539+v6539))+(v5208*(v6536/v5205))))-(v5210*(-v6536)))/v6554)))}else{common.v1});
        let v6568=(if self.scalar_static_bool[250]{(v6535+v6564)}else{common.v1});
        let v6569=(if self.scalar_static_bool[250]{(v6536+v6565)}else{common.v1});
        let v6596=(if self.scalar_static_bool[250]{(self.scalar_static_f64[626]*((v5225*common.v6586)+(common.v5224*common.v6409)))}else{common.v1});
        let v6597=(if self.scalar_static_bool[250]{(self.scalar_static_f64[626]*((v5225*common.v6587)+(common.v5224*common.v6410)))}else{common.v1});
        let v6606=(if self.scalar_static_bool[250]{(self.scalar_static_f64[212]*((v5228*v6568)+(v5217*v6596)))}else{common.v1});
        let v6607=(if self.scalar_static_bool[250]{(self.scalar_static_f64[212]*((v5228*v6569)+(v5217*v6597)))}else{common.v1});
        let v6675=(v5252*v5252);
        let v6683=(self.scalar_static_f64[220]*f64::powf(v5252,self.scalar_static_f64[599]));
        let v6686=(if self.scalar_static_bool[255]{(common.v6670*v6683)}else{(if self.scalar_static_bool[254]{((-common.v6670)/v6675)}else{common.v1})});
        let v6687=(if self.scalar_static_bool[255]{(common.v6673*v6683)}else{(if self.scalar_static_bool[254]{((-common.v6673)/v6675)}else{common.v1})});
        let v6699=(v5259*v5259);
        let v6705=(if self.scalar_static_bool[253]{(((v5259*((v5257*v6568)+(v5217*v6686)))-(v5258*(v6568+v6686)))/v6699)}else{common.v1});
        let v6706=(if self.scalar_static_bool[253]{(((v5259*((v5257*v6569)+(v5217*v6687)))-(v5258*(v6569+v6687)))/v6699)}else{common.v1});
        let v6767=(v59*common.v6759);
        let v6768=(v59*common.v6760);
        let v6770=(v5286*v5286);
        let v6776=(v5291*v5291);
        let v6779=(if common.v5290{(v6767/v6776)}else{(if v5284{((-v6767)/v6770)}else{common.v1})});
        let v6780=(if common.v5290{(v6768/v6776)}else{(if v5284{((-v6768)/v6770)}else{common.v1})});
        let v6818=(v5293*v6779);
        let v6819=(v6818+v6818);
        let v6820=(v5293*v6780);
        let v6821=(v6820+v6820);
        let v6842=(if self.scalar_static_bool[253]{((v5319*common.v6814)+(common.v5312*(((v58*v6779)+(v62*v6819))+(v63*((v5314*v6779)+(v5293*v6819))))))}else{common.v1});
        let v6843=(if self.scalar_static_bool[253]{((v5319*common.v6815)+(common.v5312*(((v58*v6780)+(v62*v6821))+(v63*((v5314*v6780)+(v5293*v6821))))))}else{common.v1});
        let v6881=(if common.v5290{((common.v60*common.v6875)-v6842)}else{(if v5284{v6842}else{common.v1})});
        let v6882=(if common.v5290{((common.v60*common.v6876)-v6843)}else{(if v5284{v6843}else{common.v1})});
        let v6888=(common.v5265*common.v5265);
        let v6896=(if self.scalar_static_bool[253]{(v803*(((common.v5265*(self.scalar_static_f64[704]*v6881))-(v5343*common.v6721))/v6888))}else{common.v1});
        let v6897=(if self.scalar_static_bool[253]{(v803*(((common.v5265*(self.scalar_static_f64[704]*v6882))-(v5343*common.v6722))/v6888))}else{common.v1});
        let v6912=(if self.scalar_static_bool[253]{(self.scalar_static_f64[213]*((v5347*v6705)+(v5261*((v5346*v6596)+(v5228*v6896)))))}else{common.v1});
        let v6913=(if self.scalar_static_bool[253]{(self.scalar_static_f64[213]*((v5347*v6706)+(v5261*((v5346*v6597)+(v5228*v6897)))))}else{common.v1});
        let v7022=(if self.scalar_static_bool[256]{(self.scalar_static_f64[222]*((v5399*common.v7000)+(common.v5397*((v5398*common.v6942)+(common.v5363*((self.scalar_static_f64[564]*common.v5363)+(common.v4925*common.v6942)))))))}else{common.v1});
        let v7023=(if self.scalar_static_bool[256]{(self.scalar_static_f64[222]*((v5399*common.v7001)+(common.v5397*((v5398*common.v6943)+(common.v5363*((common.v5363*self.scalar_static_f64[578])+(common.v4925*common.v6943)))))))}else{common.v1});
        let v7046=(v5419*v5419);
        let v7053=(if v5423{(self.scalar_static_f64[74]*common.v6507)}else{(if common.v5408{(common.v7044/v7046)}else{common.v1})});
        let v7054=(if v5423{(self.scalar_static_f64[74]*common.v6508)}else{(if common.v5408{(common.v7045/v7046)}else{common.v1})});
        let v7104=(if self.scalar_static_bool[264]{(self.scalar_static_f64[640]*common.v6404)}else{v6513});
        let v7105=(if self.scalar_static_bool[264]{(self.scalar_static_f64[640]*common.v6405)}else{v6514});
        let v7121=(common.v60*v5462);
        let v7126=(if self.scalar_static_bool[266]{(-((-(((common.v5459*common.v6460)-(common.v5168*common.v7108))/common.v7113))/v7121))}else{v6535});
        let v7127=(if self.scalar_static_bool[266]{(-((-(((common.v5459*common.v6461)-(common.v5168*common.v7109))/common.v7113))/v7121))}else{v6536});
        let v7130=(v5464*v7126);
        let v7132=(v5464*v7127);
        let v7147=(v5471*v5471);
        let v7157=(if self.scalar_static_bool[268]{(self.scalar_static_f64[251]*(v7126+(((v5471*((v5469*(v7130+v7130))+(v5468*(v7126/v5464))))-(v5470*(-v7126)))/v7147)))}else{(if self.scalar_static_bool[267]{common.v1}else{v6564})});
        let v7158=(if self.scalar_static_bool[268]{(self.scalar_static_f64[251]*(v7127+(((v5471*((v5469*(v7132+v7132))+(v5468*(v7127/v5464))))-(v5470*(-v7127)))/v7147)))}else{(if self.scalar_static_bool[267]{common.v1}else{v6565})});
        let v7161=(if self.scalar_static_bool[266]{(v7126+v7157)}else{v6568});
        let v7162=(if self.scalar_static_bool[266]{(v7127+v7158)}else{v6569});
        let v7189=(if self.scalar_static_bool[266]{(self.scalar_static_f64[631]*((common.v5484*common.v6409)+(v5225*common.v7179)))}else{v6596});
        let v7190=(if self.scalar_static_bool[266]{(self.scalar_static_f64[631]*((common.v5484*common.v6410)+(v5225*common.v7180)))}else{v6597});
        let v7199=(if self.scalar_static_bool[266]{(self.scalar_static_f64[246]*((v5487*v7161)+(v5477*v7189)))}else{(if self.scalar_static_bool[265]{common.v1}else{v6606})});
        let v7200=(if self.scalar_static_bool[266]{(self.scalar_static_f64[246]*((v5487*v7162)+(v5477*v7190)))}else{(if self.scalar_static_bool[265]{common.v1}else{v6607})});
        let v7270=(v5513*v5513);
        let v7278=(self.scalar_static_f64[254]*f64::powf(v5513,self.scalar_static_f64[601]));
        let v7281=(if self.scalar_static_bool[272]{(common.v7265*v7278)}else{(if self.scalar_static_bool[271]{((-common.v7265)/v7270)}else{v6686})});
        let v7282=(if self.scalar_static_bool[272]{(common.v7268*v7278)}else{(if self.scalar_static_bool[271]{((-common.v7268)/v7270)}else{v6687})});
        let v7294=(v5520*v5520);
        let v7300=(if self.scalar_static_bool[270]{(((v5520*((v5518*v7161)+(v5477*v7281)))-(v5519*(v7161+v7281)))/v7294)}else{v6705});
        let v7301=(if self.scalar_static_bool[270]{(((v5520*((v5518*v7162)+(v5477*v7282)))-(v5519*(v7162+v7282)))/v7294)}else{v6706});
        let v7362=(v59*common.v7354);
        let v7363=(v59*common.v7355);
        let v7365=(v5547*v5547);
        let v7371=(v5552*v5552);
        let v7374=(if common.v5551{(v7362/v7371)}else{(if v5545{((-v7362)/v7365)}else{v6779})});
        let v7375=(if common.v5551{(v7363/v7371)}else{(if v5545{((-v7363)/v7365)}else{v6780})});
        let v7413=(v5554*v7374);
        let v7414=(v7413+v7413);
        let v7415=(v5554*v7375);
        let v7416=(v7415+v7415);
        let v7437=(if self.scalar_static_bool[270]{((v5580*common.v7409)+(common.v5573*(((v58*v7374)+(v62*v7414))+(v63*((v5575*v7374)+(v5554*v7414))))))}else{v6842});
        let v7438=(if self.scalar_static_bool[270]{((v5580*common.v7410)+(common.v5573*(((v58*v7375)+(v62*v7416))+(v63*((v5575*v7375)+(v5554*v7416))))))}else{v6843});
        let v7476=(if common.v5551{((common.v60*common.v7470)-v7437)}else{(if v5545{v7437}else{v6881})});
        let v7477=(if common.v5551{((common.v60*common.v7471)-v7438)}else{(if v5545{v7438}else{v6882})});
        let v7483=(common.v5526*common.v5526);
        let v7491=(if self.scalar_static_bool[270]{(v803*(((common.v5526*(self.scalar_static_f64[705]*v7476))-(v5604*common.v7316))/v7483))}else{v6896});
        let v7492=(if self.scalar_static_bool[270]{(v803*(((common.v5526*(self.scalar_static_f64[705]*v7477))-(v5604*common.v7317))/v7483))}else{v6897});
        let v7507=(if self.scalar_static_bool[270]{(self.scalar_static_f64[247]*((v5608*v7300)+(v5522*((v5607*v7189)+(v5487*v7491)))))}else{(if self.scalar_static_bool[269]{common.v1}else{v6912})});
        let v7508=(if self.scalar_static_bool[270]{(self.scalar_static_f64[247]*((v5608*v7301)+(v5522*((v5607*v7190)+(v5487*v7492)))))}else{(if self.scalar_static_bool[269]{common.v1}else{v6913})});
        let v7617=(if self.scalar_static_bool[274]{(self.scalar_static_f64[256]*((v5662*common.v7595)+(common.v5660*((v5661*common.v7537)+(common.v5626*((self.scalar_static_f64[564]*common.v5626)+(common.v4925*common.v7537)))))))}else{(if self.scalar_static_bool[273]{common.v1}else{v7022})});
        let v7618=(if self.scalar_static_bool[274]{(self.scalar_static_f64[256]*((v5662*common.v7596)+(common.v5660*((v5661*common.v7538)+(common.v5626*((common.v5626*self.scalar_static_f64[578])+(common.v4925*common.v7538)))))))}else{(if self.scalar_static_bool[273]{common.v1}else{v7023})});
        let v7643=(v5682*v5682);
        let v7650=(if v5686{(self.scalar_static_f64[81]*common.v6507)}else{(if common.v5671{(common.v7641/v7643)}else{(if self.scalar_static_bool[277]{common.v1}else{v7053})})});
        let v7651=(if v5686{(self.scalar_static_f64[81]*common.v6508)}else{(if common.v5671{(common.v7642/v7643)}else{(if self.scalar_static_bool[277]{common.v1}else{v7054})})});
        let v7716=(common.v60*v5723);
        let v7721=(if self.scalar_static_bool[284]{(-((-(((common.v5720*common.v6460)-(common.v5168*common.v7703))/common.v7708))/v7716))}else{v7126});
        let v7722=(if self.scalar_static_bool[284]{(-((-(((common.v5720*common.v6461)-(common.v5168*common.v7704))/common.v7708))/v7716))}else{v7127});
        let v7725=(v5725*v7721);
        let v7727=(v5725*v7722);
        let v7742=(v5732*v5732);
        let v7756=(if self.scalar_static_bool[284]{(v7721+(if self.scalar_static_bool[286]{(self.scalar_static_f64[282]*(v7721+(((v5732*((v5730*(v7725+v7725))+(v5729*(v7721/v5725))))-(v5731*(-v7721)))/v7742)))}else{(if self.scalar_static_bool[285]{common.v1}else{v7157})}))}else{v7161});
        let v7757=(if self.scalar_static_bool[284]{(v7722+(if self.scalar_static_bool[286]{(self.scalar_static_f64[282]*(v7722+(((v5732*((v5730*(v7727+v7727))+(v5729*(v7722/v5725))))-(v5731*(-v7722)))/v7742)))}else{(if self.scalar_static_bool[285]{common.v1}else{v7158})}))}else{v7162});
        let v7784=(if self.scalar_static_bool[284]{(self.scalar_static_f64[636]*((common.v5745*common.v6409)+(v5225*common.v7774)))}else{v7189});
        let v7785=(if self.scalar_static_bool[284]{(self.scalar_static_f64[636]*((common.v5745*common.v6410)+(v5225*common.v7775)))}else{v7190});
        let v7865=(v5774*v5774);
        let v7873=(self.scalar_static_f64[285]*f64::powf(v5774,self.scalar_static_f64[603]));
        let v7876=(if self.scalar_static_bool[290]{(common.v7860*v7873)}else{(if self.scalar_static_bool[289]{((-common.v7860)/v7865)}else{v7281})});
        let v7877=(if self.scalar_static_bool[290]{(common.v7863*v7873)}else{(if self.scalar_static_bool[289]{((-common.v7863)/v7865)}else{v7282})});
        let v7889=(v5781*v5781);
        let v7957=(v59*common.v7949);
        let v7958=(v59*common.v7950);
        let v7960=(v5808*v5808);
        let v7966=(v5813*v5813);
        let v7969=(if common.v5812{(v7957/v7966)}else{(if v5806{((-v7957)/v7960)}else{v7374})});
        let v7970=(if common.v5812{(v7958/v7966)}else{(if v5806{((-v7958)/v7960)}else{v7375})});
        let v8008=(v5815*v7969);
        let v8009=(v8008+v8008);
        let v8010=(v5815*v7970);
        let v8011=(v8010+v8010);
        let v8032=(if self.scalar_static_bool[288]{((v5841*common.v8004)+(common.v5834*(((v58*v7969)+(v62*v8009))+(v63*((v5836*v7969)+(v5815*v8009))))))}else{v7437});
        let v8033=(if self.scalar_static_bool[288]{((v5841*common.v8005)+(common.v5834*(((v58*v7970)+(v62*v8011))+(v63*((v5836*v7970)+(v5815*v8011))))))}else{v7438});
        let v8078=(common.v5787*common.v5787);
        let v8238=(v5943*v5943);
        let v8257=((v5955*(if v5947{(self.scalar_static_f64[88]*common.v6507)}else{(if common.v5932{(common.v8236/v8238)}else{(if self.scalar_static_bool[295]{common.v1}else{v7650})})}))+(v5951*(self.scalar_static_f64[245]*((if self.scalar_static_bool[292]{(self.scalar_static_f64[287]*((v5923*common.v8190)+(common.v5921*((v5922*common.v8132)+(common.v5887*((self.scalar_static_f64[564]*common.v5887)+(common.v4925*common.v8132)))))))}else{(if self.scalar_static_bool[291]{common.v1}else{v7617})})+((if self.scalar_static_bool[288]{(self.scalar_static_f64[278]*((v5869*(if self.scalar_static_bool[288]{(((v5781*((v5779*v7756)+(v5738*v7876)))-(v5780*(v7756+v7876)))/v7889)}else{v7300}))+(v5783*((v5868*v7784)+(v5748*(if self.scalar_static_bool[288]{(v803*(((common.v5787*(self.scalar_static_f64[706]*(if common.v5812{((common.v60*common.v8065)-v8032)}else{(if v5806{v8032}else{v7476})})))-(v5865*common.v7911))/v8078))}else{v7491}))))))}else{(if self.scalar_static_bool[287]{common.v1}else{v7507})})+((if self.scalar_static_bool[282]{(self.scalar_static_f64[642]*common.v6404)}else{v7104})+(if self.scalar_static_bool[284]{(self.scalar_static_f64[277]*((v5748*v7756)+(v5738*v7784)))}else{(if self.scalar_static_bool[283]{common.v1}else{v7199})})))))));
        let v8260=((v5955*(if v5947{(self.scalar_static_f64[88]*common.v6508)}else{(if common.v5932{(common.v8237/v8238)}else{(if self.scalar_static_bool[295]{common.v1}else{v7651})})}))+(v5951*(self.scalar_static_f64[245]*((if self.scalar_static_bool[292]{(self.scalar_static_f64[287]*((v5923*common.v8191)+(common.v5921*((v5922*common.v8133)+(common.v5887*((common.v5887*self.scalar_static_f64[578])+(common.v4925*common.v8133)))))))}else{(if self.scalar_static_bool[291]{common.v1}else{v7618})})+((if self.scalar_static_bool[288]{(self.scalar_static_f64[278]*((v5869*(if self.scalar_static_bool[288]{(((v5781*((v5779*v7757)+(v5738*v7877)))-(v5780*(v7757+v7877)))/v7889)}else{v7301}))+(v5783*((v5868*v7785)+(v5748*(if self.scalar_static_bool[288]{(v803*(((common.v5787*(self.scalar_static_f64[706]*(if common.v5812{((common.v60*common.v8066)-v8033)}else{(if v5806{v8033}else{v7477})})))-(v5865*common.v7912))/v8078))}else{v7492}))))))}else{(if self.scalar_static_bool[287]{common.v1}else{v7508})})+((if self.scalar_static_bool[282]{(self.scalar_static_f64[642]*common.v6405)}else{v7105})+(if self.scalar_static_bool[284]{(self.scalar_static_f64[277]*((v5748*v7757)+(v5738*v7785)))}else{(if self.scalar_static_bool[283]{common.v1}else{v7200})})))))));

        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (((if self.scalar_static_bool[246]{(((self.scalar_static_f64[143]*(if self.scalar_static_bool[249]{(v5427*v5431)}else{common.v1}))+(self.scalar_static_f64[145]*(if self.scalar_static_bool[264]{(v5690*v5694)}else{common.v1})))+(self.scalar_static_f64[147]*(if self.scalar_static_bool[282]{(v5951*v5955)}else{common.v1})))}else{(if (self.scalar_static_f64[177]!=0.0){((if self.scalar_static_bool[739]{(self.scalar_static_f64[4250]*((if self.scalar_static_bool[739]{(if v4973{(common.v407/v4975)}else{(if v4977{(self.scalar_static_f64[4248]*(common.v5+(v4972-self.scalar_static_f64[4246])))}else{v4981})})}else{v4956})-common.v5))}else{(if self.scalar_static_bool[737]{(common.v4925*v4964)}else{common.v1})})+((if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4103]*(v4939-common.v5))}else{common.v1})+(if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4128]*(v4956-common.v5))}else{common.v1})))}else{common.v1})})*self.scalar_static_f64[577])),
            0,
            multiplicity * ((self.scalar_static_f64[577]*(if self.scalar_static_bool[246]{(((self.scalar_static_f64[143]*(if self.scalar_static_bool[249]{((v5431*v7053)+(v5427*(self.scalar_static_f64[245]*(v7022+(v6912+(v6513+v6606))))))}else{common.v1}))+(self.scalar_static_f64[145]*(if self.scalar_static_bool[264]{((v5694*v7650)+(v5690*(self.scalar_static_f64[245]*(v7617+(v7507+(v7104+v7199))))))}else{common.v1})))+(self.scalar_static_f64[147]*(if self.scalar_static_bool[282]{v8257}else{common.v1})))}else{(if (self.scalar_static_f64[177]!=0.0){((if self.scalar_static_bool[739]{(self.scalar_static_f64[4250]*(if self.scalar_static_bool[739]{(if v4973{(self.scalar_static_f64[4299]/v6165)}else{(if v4977{self.scalar_static_f64[4302]}else{(v4981*self.scalar_static_f64[4294])})})}else{v6141}))}else{(if self.scalar_static_bool[737]{((self.scalar_static_f64[564]*v4964)+(common.v4925*self.scalar_static_f64[4290]))}else{common.v1})})+((if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4103]*v6114)}else{common.v1})+(if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4128]*v6141)}else{common.v1})))}else{common.v1})}))),
            1,
            multiplicity * ((self.scalar_static_f64[577]*(if self.scalar_static_bool[246]{(((self.scalar_static_f64[143]*(if self.scalar_static_bool[249]{((v5431*v7054)+(v5427*(self.scalar_static_f64[245]*(v7023+(v6913+(v6514+v6607))))))}else{common.v1}))+(self.scalar_static_f64[145]*(if self.scalar_static_bool[264]{((v5694*v7651)+(v5690*(self.scalar_static_f64[245]*(v7618+(v7508+(v7105+v7200))))))}else{common.v1})))+(self.scalar_static_f64[147]*(if self.scalar_static_bool[282]{v8260}else{common.v1})))}else{(if (self.scalar_static_f64[177]!=0.0){((if self.scalar_static_bool[739]{(self.scalar_static_f64[4250]*(if self.scalar_static_bool[739]{(if v4973{(self.scalar_static_f64[4301]/v6165)}else{(if v4977{self.scalar_static_f64[4303]}else{(v4981*self.scalar_static_f64[4295])})})}else{v6142}))}else{(if self.scalar_static_bool[737]{((v4964*self.scalar_static_f64[578])+(common.v4925*self.scalar_static_f64[4291]))}else{common.v1})})+((if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4103]*v6115)}else{common.v1})+(if (self.scalar_static_f64[177]!=0.0){(self.scalar_static_f64[4128]*v6142)}else{common.v1})))}else{common.v1})}))),
        );
        let v6091_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v6091);
        stamper.stamp_current_node2_local(
            Some(0),
            Some(1),
            multiplicity * (v6091_ddt),
            0,
            multiplicity * (((common.v8473) * ddt_scale)),
            1,
            multiplicity * (((common.v8474) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(1),
            multiplicity * (common.v1),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[1]),
            nodes[0],
            multiplicity * (common.v8473),
            nodes[1],
            multiplicity * (common.v8474),
        );
    }
}
