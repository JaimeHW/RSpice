#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3570_e4849: f64 = (locals.var_bin_l * p.p1086);
        let assign3570_e4850: f64 = (p.p1085 + assign3570_e4849);
        let assign3570_e4853: f64 = (locals.var_bin_w * p.p1087);
        let assign3570_e4854: f64 = (assign3570_e4850 + assign3570_e4853);
        let assign3570_e4857: f64 = (locals.var_bin_wl * p.p1088);
        let assign3570_e4858: f64 = (assign3570_e4854 + assign3570_e4857);
        locals.var_iit_i = assign3570_e4858;
        locals.var_iit_i_rv = 0.0;

        let assign3590_e4875: f64 = (locals.var_bin_l * p.p732);
        let assign3590_e4876: f64 = (p.p706 + assign3590_e4875);
        let assign3590_e4879: f64 = (locals.var_bin_w * p.p733);
        let assign3590_e4880: f64 = (assign3590_e4876 + assign3590_e4879);
        let assign3590_e4883: f64 = (locals.var_bin_wl * p.p734);
        let assign3590_e4884: f64 = (assign3590_e4880 + assign3590_e4883);
        locals.var_eigbinv_i = assign3590_e4884;
        locals.var_eigbinv_i_rv = 0.0;

        let assign3600_e4888: f64 = (locals.var_bin_l * p.p685);
        let assign3600_e4889: f64 = (p.p684 + assign3600_e4888);
        let assign3600_e4892: f64 = (locals.var_bin_w * p.p686);
        let assign3600_e4893: f64 = (assign3600_e4889 + assign3600_e4892);
        let assign3600_e4896: f64 = (locals.var_bin_wl * p.p687);
        let assign3600_e4897: f64 = (assign3600_e4893 + assign3600_e4896);
        locals.var_alphagb2_i = assign3600_e4897;
        locals.var_alphagb2_i_dn4 = 0.0;
        locals.var_alphagb2_i_dn5 = 0.0;
        locals.var_alphagb2_i_rv = 0.0;

        let assign3610_e4901: f64 = (p.p689 * locals.var_bin_l);
        let assign3610_e4902: f64 = (p.p688 + assign3610_e4901);
        let assign3610_e4905: f64 = (p.p690 * locals.var_bin_w);
        let assign3610_e4906: f64 = (assign3610_e4902 + assign3610_e4905);
        let assign3610_e4909: f64 = (p.p691 * locals.var_bin_wl);
        let assign3610_e4910: f64 = (assign3610_e4906 + assign3610_e4909);
        locals.var_alphagb2_t_i = assign3610_e4910;
        locals.var_alphagb2_t_i_rv = 0.0;

        let assign3620_e4914: f64 = (locals.var_bin_l * p.p693);
        let assign3620_e4915: f64 = (p.p692 + assign3620_e4914);
        let assign3620_e4918: f64 = (locals.var_bin_w * p.p694);
        let assign3620_e4919: f64 = (assign3620_e4915 + assign3620_e4918);
        let assign3620_e4922: f64 = (locals.var_bin_wl * p.p695);
        let assign3620_e4923: f64 = (assign3620_e4919 + assign3620_e4922);
        locals.var_betagb2_i = assign3620_e4923;
        locals.var_betagb2_i_rv = 0.0;

        let assign3630_e4927: f64 = (locals.var_bin_l * p.p673);
        let assign3630_e4928: f64 = (p.p672 + assign3630_e4927);
        let assign3630_e4931: f64 = (locals.var_bin_w * p.p674);
        let assign3630_e4932: f64 = (assign3630_e4928 + assign3630_e4931);
        let assign3630_e4935: f64 = (locals.var_bin_wl * p.p675);
        let assign3630_e4936: f64 = (assign3630_e4932 + assign3630_e4935);
        locals.var_alphagb1_i = assign3630_e4936;
        locals.var_alphagb1_i_dn4 = 0.0;
        locals.var_alphagb1_i_dn5 = 0.0;
        locals.var_alphagb1_i_rv = 0.0;

        let assign3640_e4940: f64 = (p.p677 * locals.var_bin_l);
        let assign3640_e4941: f64 = (p.p676 + assign3640_e4940);
        let assign3640_e4944: f64 = (p.p678 * locals.var_bin_w);
        let assign3640_e4945: f64 = (assign3640_e4941 + assign3640_e4944);
        let assign3640_e4948: f64 = (p.p679 * locals.var_bin_wl);
        let assign3640_e4949: f64 = (assign3640_e4945 + assign3640_e4948);
        locals.var_alphagb1_t_i = assign3640_e4949;
        locals.var_alphagb1_t_i_rv = 0.0;

        let assign3650_e4953: f64 = (locals.var_bin_l * p.p681);
        let assign3650_e4954: f64 = (p.p680 + assign3650_e4953);
        let assign3650_e4957: f64 = (locals.var_bin_w * p.p682);
        let assign3650_e4958: f64 = (assign3650_e4954 + assign3650_e4957);
        let assign3650_e4961: f64 = (locals.var_bin_wl * p.p683);
        let assign3650_e4962: f64 = (assign3650_e4958 + assign3650_e4961);
        locals.var_betagb1_i = assign3650_e4962;
        locals.var_betagb1_i_rv = 0.0;

        let assign3660_e4966: f64 = (locals.var_bin_l * p.p735);
        let assign3660_e4967: f64 = (p.p707 + assign3660_e4966);
        let assign3660_e4970: f64 = (locals.var_bin_w * p.p737);
        let assign3660_e4971: f64 = (assign3660_e4967 + assign3660_e4970);
        let assign3660_e4974: f64 = (locals.var_bin_wl * p.p739);
        let assign3660_e4975: f64 = (assign3660_e4971 + assign3660_e4974);
        locals.var_aigc_i = assign3660_e4975;
        locals.var_aigc_i_dn4 = 0.0;
        locals.var_aigc_i_dn5 = 0.0;
        locals.var_aigc_i_rv = 0.0;

        let assign3670_e4979: f64 = (p.p736 * locals.var_bin_l);
        let assign3670_e4980: f64 = (p.p726 + assign3670_e4979);
        let assign3670_e4983: f64 = (p.p738 * locals.var_bin_w);
        let assign3670_e4984: f64 = (assign3670_e4980 + assign3670_e4983);
        let assign3670_e4987: f64 = (p.p740 * locals.var_bin_wl);
        let assign3670_e4988: f64 = (assign3670_e4984 + assign3670_e4987);
        locals.var_aigc1_i = assign3670_e4988;
        locals.var_aigc1_i_rv = 0.0;

        let assign3680_e4992: f64 = (locals.var_bin_l * p.p741);
        let assign3680_e4993: f64 = (p.p708 + assign3680_e4992);
        let assign3680_e4996: f64 = (locals.var_bin_w * p.p742);
        let assign3680_e4997: f64 = (assign3680_e4993 + assign3680_e4996);
        let assign3680_e5000: f64 = (locals.var_bin_wl * p.p743);
        let assign3680_e5001: f64 = (assign3680_e4997 + assign3680_e5000);
        locals.var_bigc_i = assign3680_e5001;
        locals.var_bigc_i_rv = 0.0;

        let assign3690_e5005: f64 = (locals.var_bin_l * p.p744);
        let assign3690_e5006: f64 = (p.p709 + assign3690_e5005);
        let assign3690_e5009: f64 = (locals.var_bin_w * p.p745);
        let assign3690_e5010: f64 = (assign3690_e5006 + assign3690_e5009);
        let assign3690_e5013: f64 = (locals.var_bin_wl * p.p746);
        let assign3690_e5014: f64 = (assign3690_e5010 + assign3690_e5013);
        locals.var_cigc_i = assign3690_e5014;
        locals.var_cigc_i_rv = 0.0;

        let assign3700_e5018: f64 = (locals.var_bin_l * p.p747);
        let assign3700_e5019: f64 = (p.p710 + assign3700_e5018);
        let assign3700_e5022: f64 = (locals.var_bin_w * p.p749);
        let assign3700_e5023: f64 = (assign3700_e5019 + assign3700_e5022);
        let assign3700_e5026: f64 = (locals.var_bin_wl * p.p751);
        let assign3700_e5027: f64 = (assign3700_e5023 + assign3700_e5026);
        locals.var_aigs_i = assign3700_e5027;
        locals.var_aigs_i_dn4 = 0.0;
        locals.var_aigs_i_dn5 = 0.0;
        locals.var_aigs_i_rv = 0.0;

        let assign3710_e5031: f64 = (p.p748 * locals.var_bin_l);
        let assign3710_e5032: f64 = (p.p711 + assign3710_e5031);
        let assign3710_e5035: f64 = (p.p750 * locals.var_bin_w);
        let assign3710_e5036: f64 = (assign3710_e5032 + assign3710_e5035);
        let assign3710_e5039: f64 = (p.p752 * locals.var_bin_wl);
        let assign3710_e5040: f64 = (assign3710_e5036 + assign3710_e5039);
        locals.var_aigs1_i = assign3710_e5040;
        locals.var_aigs1_i_rv = 0.0;

        let assign3720_e5044: f64 = (locals.var_bin_l * p.p753);
        let assign3720_e5045: f64 = (p.p712 + assign3720_e5044);
        let assign3720_e5048: f64 = (locals.var_bin_w * p.p754);
        let assign3720_e5049: f64 = (assign3720_e5045 + assign3720_e5048);
        let assign3720_e5052: f64 = (locals.var_bin_wl * p.p755);
        let assign3720_e5053: f64 = (assign3720_e5049 + assign3720_e5052);
        locals.var_bigs_i = assign3720_e5053;
        locals.var_bigs_i_rv = 0.0;

        let assign3730_e5057: f64 = (locals.var_bin_l * p.p756);
        let assign3730_e5058: f64 = (p.p713 + assign3730_e5057);
        let assign3730_e5061: f64 = (locals.var_bin_w * p.p757);
        let assign3730_e5062: f64 = (assign3730_e5058 + assign3730_e5061);
        let assign3730_e5065: f64 = (locals.var_bin_wl * p.p758);
        let assign3730_e5066: f64 = (assign3730_e5062 + assign3730_e5065);
        locals.var_cigs_i = assign3730_e5066;
        locals.var_cigs_i_rv = 0.0;

        let assign3740_e5070: f64 = (locals.var_bin_l * p.p759);
        let assign3740_e5071: f64 = (p.p714 + assign3740_e5070);
        let assign3740_e5074: f64 = (locals.var_bin_w * p.p761);
        let assign3740_e5075: f64 = (assign3740_e5071 + assign3740_e5074);
        let assign3740_e5078: f64 = (locals.var_bin_wl * p.p763);
        let assign3740_e5079: f64 = (assign3740_e5075 + assign3740_e5078);
        locals.var_aigd_i = assign3740_e5079;
        locals.var_aigd_i_dn4 = 0.0;
        locals.var_aigd_i_dn5 = 0.0;
        locals.var_aigd_i_rv = 0.0;

        let assign3750_e5083: f64 = (p.p760 * locals.var_bin_l);
        let assign3750_e5084: f64 = (p.p715 + assign3750_e5083);
        let assign3750_e5087: f64 = (p.p762 * locals.var_bin_w);
        let assign3750_e5088: f64 = (assign3750_e5084 + assign3750_e5087);
        let assign3750_e5091: f64 = (p.p764 * locals.var_bin_wl);
        let assign3750_e5092: f64 = (assign3750_e5088 + assign3750_e5091);
        locals.var_aigd1_i = assign3750_e5092;
        locals.var_aigd1_i_rv = 0.0;

        let assign3760_e5096: f64 = (locals.var_bin_l * p.p765);
        let assign3760_e5097: f64 = (p.p716 + assign3760_e5096);
        let assign3760_e5100: f64 = (locals.var_bin_w * p.p766);
        let assign3760_e5101: f64 = (assign3760_e5097 + assign3760_e5100);
        let assign3760_e5104: f64 = (locals.var_bin_wl * p.p767);
        let assign3760_e5105: f64 = (assign3760_e5101 + assign3760_e5104);
        locals.var_bigd_i = assign3760_e5105;
        locals.var_bigd_i_rv = 0.0;

        let assign3770_e5109: f64 = (locals.var_bin_l * p.p768);
        let assign3770_e5110: f64 = (p.p717 + assign3770_e5109);
        let assign3770_e5113: f64 = (locals.var_bin_w * p.p769);
        let assign3770_e5114: f64 = (assign3770_e5110 + assign3770_e5113);
        let assign3770_e5117: f64 = (locals.var_bin_wl * p.p770);
        let assign3770_e5118: f64 = (assign3770_e5114 + assign3770_e5117);
        locals.var_cigd_i = assign3770_e5118;
        locals.var_cigd_i_rv = 0.0;

        let assign3780_e5122: f64 = (locals.var_bin_l * p.p771);
        let assign3780_e5123: f64 = (p.p720 + assign3780_e5122);
        let assign3780_e5126: f64 = (locals.var_bin_w * p.p772);
        let assign3780_e5127: f64 = (assign3780_e5123 + assign3780_e5126);
        let assign3780_e5130: f64 = (locals.var_bin_wl * p.p773);
        let assign3780_e5131: f64 = (assign3780_e5127 + assign3780_e5130);
        locals.var_poxedge_i = assign3780_e5131;
        locals.var_poxedge_i_rv = 0.0;

        let assign3810_e5161: f64 = (locals.var_bin_l * p.p780);
        let assign3810_e5162: f64 = (p.p721 + assign3810_e5161);
        let assign3810_e5165: f64 = (locals.var_bin_w * p.p781);
        let assign3810_e5166: f64 = (assign3810_e5162 + assign3810_e5165);
        let assign3810_e5169: f64 = (locals.var_bin_wl * p.p782);
        let assign3810_e5170: f64 = (assign3810_e5166 + assign3810_e5169);
        locals.var_ntox_i = assign3810_e5170;
        locals.var_ntox_i_rv = 0.0;

        let assign3820_e5174: f64 = (locals.var_bin_l * p.p1078);
        let assign3820_e5175: f64 = (p.p1075 + assign3820_e5174);
        let assign3820_e5178: f64 = (locals.var_bin_w * p.p1079);
        let assign3820_e5179: f64 = (assign3820_e5175 + assign3820_e5178);
        let assign3820_e5182: f64 = (locals.var_bin_wl * p.p1080);
        let assign3820_e5183: f64 = (assign3820_e5179 + assign3820_e5182);
        locals.var_kt1_i = assign3820_e5183;
        locals.var_kt1_i_rv = 0.0;

        let assign3830_e5187: f64 = (locals.var_bin_l * p.p1082);
        let assign3830_e5188: f64 = (p.p1081 + assign3830_e5187);
        let assign3830_e5191: f64 = (locals.var_bin_w * p.p1083);
        let assign3830_e5192: f64 = (assign3830_e5188 + assign3830_e5191);
        let assign3830_e5195: f64 = (locals.var_bin_wl * p.p1084);
        let assign3830_e5196: f64 = (assign3830_e5192 + assign3830_e5195);
        locals.var_kt2_i = assign3830_e5196;
        locals.var_kt2_i_rv = 0.0;

        let assign3840_e5200: f64 = (locals.var_bin_l * p.p494);
        let assign3840_e5201: f64 = (p.p489 + assign3840_e5200);
        let assign3840_e5204: f64 = (locals.var_bin_w * p.p495);
        let assign3840_e5205: f64 = (assign3840_e5201 + assign3840_e5204);
        let assign3840_e5208: f64 = (locals.var_bin_wl * p.p496);
        let assign3840_e5209: f64 = (assign3840_e5205 + assign3840_e5208);
        locals.var_psatb_i = assign3840_e5209;
        locals.var_psatb_i_rv = 0.0;

        let assign3850_e5213: f64 = (locals.var_bin_l * p.p515);
        let assign3850_e5214: f64 = (p.p514 + assign3850_e5213);
        let assign3850_e5217: f64 = (locals.var_bin_w * p.p516);
        let assign3850_e5218: f64 = (assign3850_e5214 + assign3850_e5217);
        let assign3850_e5221: f64 = (locals.var_bin_wl * p.p517);
        let assign3850_e5222: f64 = (assign3850_e5218 + assign3850_e5221);
        locals.var_a1_i = assign3850_e5222;
        locals.var_a1_i_rv = 0.0;

        let assign3860_e5226: f64 = (locals.var_bin_l * p.p519);
        let assign3860_e5227: f64 = (p.p518 + assign3860_e5226);
        let assign3860_e5230: f64 = (locals.var_bin_w * p.p520);
        let assign3860_e5231: f64 = (assign3860_e5227 + assign3860_e5230);
        let assign3860_e5234: f64 = (locals.var_bin_wl * p.p521);
        let assign3860_e5235: f64 = (assign3860_e5231 + assign3860_e5234);
        locals.var_a11_i = assign3860_e5235;
        locals.var_a11_i_rv = 0.0;

        let assign3870_e5239: f64 = (locals.var_bin_l * p.p523);
        let assign3870_e5240: f64 = (p.p522 + assign3870_e5239);
        let assign3870_e5243: f64 = (locals.var_bin_w * p.p524);
        let assign3870_e5244: f64 = (assign3870_e5240 + assign3870_e5243);
        let assign3870_e5247: f64 = (locals.var_bin_wl * p.p525);
        let assign3870_e5248: f64 = (assign3870_e5244 + assign3870_e5247);
        locals.var_a2_i = assign3870_e5248;
        locals.var_a2_i_rv = 0.0;

        let assign3880_e5252: f64 = (locals.var_bin_l * p.p527);
        let assign3880_e5253: f64 = (p.p526 + assign3880_e5252);
        let assign3880_e5256: f64 = (locals.var_bin_w * p.p528);
        let assign3880_e5257: f64 = (assign3880_e5253 + assign3880_e5256);
        let assign3880_e5260: f64 = (locals.var_bin_wl * p.p529);
        let assign3880_e5261: f64 = (assign3880_e5257 + assign3880_e5260);
        locals.var_a21_i = assign3880_e5261;
        locals.var_a21_i_rv = 0.0;

        let assign3890_e5265: f64 = (locals.var_bin_l * p.p1301);
        let assign3890_e5266: f64 = (p.p1300 + assign3890_e5265);
        let assign3890_e5269: f64 = (locals.var_bin_w * p.p1302);
        let assign3890_e5270: f64 = (assign3890_e5266 + assign3890_e5269);
        let assign3890_e5273: f64 = (locals.var_bin_wl * p.p1303);
        let assign3890_e5274: f64 = (assign3890_e5270 + assign3890_e5273);
        locals.var_k0_i = assign3890_e5274;
        locals.var_k0_i_rv = 0.0;

        let assign3900_e5278: f64 = (locals.var_bin_l * p.p1309);
        let assign3900_e5279: f64 = (p.p1308 + assign3900_e5278);
        let assign3900_e5282: f64 = (locals.var_bin_w * p.p1310);
        let assign3900_e5283: f64 = (assign3900_e5279 + assign3900_e5282);
        let assign3900_e5286: f64 = (locals.var_bin_wl * p.p1311);
        let assign3900_e5287: f64 = (assign3900_e5283 + assign3900_e5286);
        locals.var_m0_i = assign3900_e5287;
        locals.var_m0_i_rv = 0.0;

        let assign3910_e5291: f64 = (locals.var_bin_l * p.p1305);
        let assign3910_e5292: f64 = (p.p1304 + assign3910_e5291);
        let assign3910_e5295: f64 = (locals.var_bin_w * p.p1306);
        let assign3910_e5296: f64 = (assign3910_e5292 + assign3910_e5295);
        let assign3910_e5299: f64 = (locals.var_bin_wl * p.p1307);
        let assign3910_e5300: f64 = (assign3910_e5296 + assign3910_e5299);
        locals.var_k01_i = assign3910_e5300;
        locals.var_k01_i_rv = 0.0;

        let assign3920_e5304: f64 = (locals.var_bin_l * p.p1313);
        let assign3920_e5305: f64 = (p.p1312 + assign3920_e5304);
        let assign3920_e5308: f64 = (locals.var_bin_w * p.p1314);
        let assign3920_e5309: f64 = (assign3920_e5305 + assign3920_e5308);
        let assign3920_e5312: f64 = (locals.var_bin_wl * p.p1315);
        let assign3920_e5313: f64 = (assign3920_e5309 + assign3920_e5312);
        locals.var_m01_i = assign3920_e5313;
        locals.var_m01_i_rv = 0.0;

        let assign3930_e5317: f64 = (locals.var_bin_l * p.p1157);
        let assign3930_e5318: f64 = (p.p1156 + assign3930_e5317);
        let assign3930_e5321: f64 = (locals.var_bin_w * p.p1158);
        let assign3930_e5322: f64 = (assign3930_e5318 + assign3930_e5321);
        let assign3930_e5325: f64 = (locals.var_bin_wl * p.p1159);
        let assign3930_e5326: f64 = (assign3930_e5322 + assign3930_e5325);
        locals.var_nfactoredge_i = assign3930_e5326;
        locals.var_nfactoredge_i_rv = 0.0;

        let assign3940_e5330: f64 = (locals.var_bin_l * p.p1153);
        let assign3940_e5331: f64 = (p.p1152 + assign3940_e5330);
        let assign3940_e5334: f64 = (locals.var_bin_w * p.p1154);
        let assign3940_e5335: f64 = (assign3940_e5331 + assign3940_e5334);
        let assign3940_e5338: f64 = (locals.var_bin_wl * p.p1155);
        let assign3940_e5339: f64 = (assign3940_e5335 + assign3940_e5338);
        locals.var_ndepedge_i = assign3940_e5339;
        locals.var_ndepedge_i_rv = 0.0;

        let assign3950_e5343: f64 = (locals.var_bin_l * p.p1161);
        let assign3950_e5344: f64 = (p.p1160 + assign3950_e5343);
        let assign3950_e5347: f64 = (locals.var_bin_w * p.p1162);
        let assign3950_e5348: f64 = (assign3950_e5344 + assign3950_e5347);
        let assign3950_e5351: f64 = (locals.var_bin_wl * p.p1163);
        let assign3950_e5352: f64 = (assign3950_e5348 + assign3950_e5351);
        locals.var_citedge_i = assign3950_e5352;
        locals.var_citedge_i_rv = 0.0;

        let assign3960_e5356: f64 = (locals.var_bin_l * p.p1169);
        let assign3960_e5357: f64 = (p.p1168 + assign3960_e5356);
        let assign3960_e5360: f64 = (locals.var_bin_w * p.p1170);
        let assign3960_e5361: f64 = (assign3960_e5357 + assign3960_e5360);
        let assign3960_e5364: f64 = (locals.var_bin_wl * p.p1171);
        let assign3960_e5365: f64 = (assign3960_e5361 + assign3960_e5364);
        locals.var_cdscdedge_i = assign3960_e5365;
        locals.var_cdscdedge_i_rv = 0.0;

        let assign3970_e5369: f64 = (locals.var_bin_l * p.p1187);
        let assign3970_e5370: f64 = (p.p1186 + assign3970_e5369);
        let assign3970_e5373: f64 = (locals.var_bin_w * p.p1188);
        let assign3970_e5374: f64 = (assign3970_e5370 + assign3970_e5373);
        let assign3970_e5377: f64 = (locals.var_bin_wl * p.p1189);
        let assign3970_e5378: f64 = (assign3970_e5374 + assign3970_e5377);
        locals.var_cdscbedge_i = assign3970_e5378;
        locals.var_cdscbedge_i_rv = 0.0;

        let assign3980_e5382: f64 = (locals.var_bin_l * p.p1207);
        let assign3980_e5383: f64 = (p.p1206 + assign3980_e5382);
        let assign3980_e5386: f64 = (locals.var_bin_w * p.p1208);
        let assign3980_e5387: f64 = (assign3980_e5383 + assign3980_e5386);
        let assign3980_e5390: f64 = (locals.var_bin_wl * p.p1209);
        let assign3980_e5391: f64 = (assign3980_e5387 + assign3980_e5390);
        locals.var_eta0edge_i = assign3980_e5391;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        let assign3990_e5395: f64 = (locals.var_bin_l * p.p1211);
        let assign3990_e5396: f64 = (p.p1210 + assign3990_e5395);
        let assign3990_e5399: f64 = (locals.var_bin_w * p.p1212);
        let assign3990_e5400: f64 = (assign3990_e5396 + assign3990_e5399);
        let assign3990_e5403: f64 = (locals.var_bin_wl * p.p1213);
        let assign3990_e5404: f64 = (assign3990_e5400 + assign3990_e5403);
        locals.var_etabedge_i = assign3990_e5404;
        locals.var_etabedge_i_rv = 0.0;

        let assign4000_e5408: f64 = (locals.var_bin_l * p.p1215);
        let assign4000_e5409: f64 = (p.p1214 + assign4000_e5408);
        let assign4000_e5412: f64 = (locals.var_bin_w * p.p1216);
        let assign4000_e5413: f64 = (assign4000_e5409 + assign4000_e5412);
        let assign4000_e5416: f64 = (locals.var_bin_wl * p.p1217);
        let assign4000_e5417: f64 = (assign4000_e5413 + assign4000_e5416);
        locals.var_kt1edge_i = assign4000_e5417;
        locals.var_kt1edge_i_rv = 0.0;

        let assign4010_e5421: f64 = (locals.var_bin_l * p.p1219);
        let assign4010_e5422: f64 = (p.p1218 + assign4010_e5421);
        let assign4010_e5425: f64 = (locals.var_bin_w * p.p1220);
        let assign4010_e5426: f64 = (assign4010_e5422 + assign4010_e5425);
        let assign4010_e5429: f64 = (locals.var_bin_wl * p.p1221);
        let assign4010_e5430: f64 = (assign4010_e5426 + assign4010_e5429);
        locals.var_kt1ledge_i = assign4010_e5430;
        locals.var_kt1ledge_i_rv = 0.0;

        let assign4020_e5434: f64 = (locals.var_bin_l * p.p1223);
        let assign4020_e5435: f64 = (p.p1222 + assign4020_e5434);
        let assign4020_e5438: f64 = (locals.var_bin_w * p.p1224);
        let assign4020_e5439: f64 = (assign4020_e5435 + assign4020_e5438);
        let assign4020_e5442: f64 = (locals.var_bin_wl * p.p1225);
        let assign4020_e5443: f64 = (assign4020_e5439 + assign4020_e5442);
        locals.var_kt2edge_i = assign4020_e5443;
        locals.var_kt2edge_i_rv = 0.0;

        let assign4030_e5447: f64 = (locals.var_bin_l * p.p1227);
        let assign4030_e5448: f64 = (p.p1226 + assign4030_e5447);
        let assign4030_e5451: f64 = (locals.var_bin_w * p.p1228);
        let assign4030_e5452: f64 = (assign4030_e5448 + assign4030_e5451);
        let assign4030_e5455: f64 = (locals.var_bin_wl * p.p1229);
        let assign4030_e5456: f64 = (assign4030_e5452 + assign4030_e5455);
        locals.var_kt1expedge_i = assign4030_e5456;
        locals.var_kt1expedge_i_rv = 0.0;

        let assign4040_e5460: f64 = (locals.var_bin_l * p.p1231);
        let assign4040_e5461: f64 = (p.p1230 + assign4040_e5460);
        let assign4040_e5464: f64 = (locals.var_bin_w * p.p1232);
        let assign4040_e5465: f64 = (assign4040_e5461 + assign4040_e5464);
        let assign4040_e5468: f64 = (locals.var_bin_wl * p.p1233);
        let assign4040_e5469: f64 = (assign4040_e5465 + assign4040_e5468);
        locals.var_tnfactoredge_i = assign4040_e5469;
        locals.var_tnfactoredge_i_rv = 0.0;

        let assign4050_e5473: f64 = (locals.var_bin_l * p.p1235);
        let assign4050_e5474: f64 = (p.p1234 + assign4050_e5473);
        let assign4050_e5477: f64 = (locals.var_bin_w * p.p1236);
        let assign4050_e5478: f64 = (assign4050_e5474 + assign4050_e5477);
        let assign4050_e5481: f64 = (locals.var_bin_wl * p.p1237);
        let assign4050_e5482: f64 = (assign4050_e5478 + assign4050_e5481);
        locals.var_teta0edge_i = assign4050_e5482;
        locals.var_teta0edge_i_rv = 0.0;

        let assign4060_e5486: f64 = (locals.var_bin_l * p.p1272);
        let assign4060_e5487: f64 = (p.p1265 + assign4060_e5486);
        let assign4060_e5490: f64 = (locals.var_bin_w * p.p1273);
        let assign4060_e5491: f64 = (assign4060_e5487 + assign4060_e5490);
        let assign4060_e5494: f64 = (locals.var_bin_wl * p.p1274);
        let assign4060_e5495: f64 = (assign4060_e5491 + assign4060_e5494);
        locals.var_k2edge_i = assign4060_e5495;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        let assign4070_e5499: f64 = (locals.var_bin_l * p.p1276);
        let assign4070_e5500: f64 = (p.p1275 + assign4070_e5499);
        let assign4070_e5503: f64 = (locals.var_bin_w * p.p1277);
        let assign4070_e5504: f64 = (assign4070_e5500 + assign4070_e5503);
        let assign4070_e5507: f64 = (locals.var_bin_wl * p.p1278);
        let assign4070_e5508: f64 = (assign4070_e5504 + assign4070_e5507);
        locals.var_kvth0edge_i = assign4070_e5508;
        locals.var_kvth0edge_i_rv = 0.0;

        let assign4080_e5512: f64 = (locals.var_bin_l * p.p1284);
        let assign4080_e5513: f64 = (p.p1283 + assign4080_e5512);
        let assign4080_e5516: f64 = (locals.var_bin_w * p.p1285);
        let assign4080_e5517: f64 = (assign4080_e5513 + assign4080_e5516);
        let assign4080_e5520: f64 = (locals.var_bin_wl * p.p1286);
        let assign4080_e5521: f64 = (assign4080_e5517 + assign4080_e5520);
        locals.var_k2edgewe_i = assign4080_e5521;
        locals.var_k2edgewe_i_rv = 0.0;

        let assign4090_e5525: f64 = (locals.var_bin_l * p.p1280);
        let assign4090_e5526: f64 = (p.p1279 + assign4090_e5525);
        let assign4090_e5529: f64 = (locals.var_bin_w * p.p1281);
        let assign4090_e5530: f64 = (assign4090_e5526 + assign4090_e5529);
        let assign4090_e5533: f64 = (locals.var_bin_wl * p.p1282);
        let assign4090_e5534: f64 = (assign4090_e5530 + assign4090_e5533);
        locals.var_kvth0edgewe_i = assign4090_e5534;
        locals.var_kvth0edgewe_i_rv = 0.0;

        let assign4100_e5538: f64 = (locals.var_bin_l * p.p1288);
        let assign4100_e5539: f64 = (p.p1287 + assign4100_e5538);
        let assign4100_e5542: f64 = (locals.var_bin_w * p.p1289);
        let assign4100_e5543: f64 = (assign4100_e5539 + assign4100_e5542);
        let assign4100_e5546: f64 = (locals.var_bin_wl * p.p1290);
        let assign4100_e5547: f64 = (assign4100_e5543 + assign4100_e5546);
        locals.var_stk2edge_i = assign4100_e5547;
        locals.var_stk2edge_i_rv = 0.0;

        let assign4110_e5551: f64 = (locals.var_bin_l * p.p1292);
        let assign4110_e5552: f64 = (p.p1291 + assign4110_e5551);
        let assign4110_e5555: f64 = (locals.var_bin_w * p.p1293);
        let assign4110_e5556: f64 = (assign4110_e5552 + assign4110_e5555);
        let assign4110_e5559: f64 = (locals.var_bin_wl * p.p1294);
        let assign4110_e5560: f64 = (assign4110_e5556 + assign4110_e5559);
        locals.var_steta0edge_i = assign4110_e5560;
        locals.var_steta0edge_i_rv = 0.0;

        let assign4120_e5564: f64 = (locals.var_bin_l * p.p1324);
        let assign4120_e5565: f64 = (p.p1323 + assign4120_e5564);
        let assign4120_e5568: f64 = (locals.var_bin_w * p.p1325);
        let assign4120_e5569: f64 = (assign4120_e5565 + assign4120_e5568);
        let assign4120_e5572: f64 = (locals.var_bin_wl * p.p1326);
        let assign4120_e5573: f64 = (assign4120_e5569 + assign4120_e5572);
        locals.var_c0_i = assign4120_e5573;
        locals.var_c0_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4130_e5577: f64 = (locals.var_bin_l * p.p1328);
        let assign4130_e5578: f64 = (p.p1327 + assign4130_e5577);
        let assign4130_e5581: f64 = (locals.var_bin_w * p.p1329);
        let assign4130_e5582: f64 = (assign4130_e5578 + assign4130_e5581);
        let assign4130_e5585: f64 = (locals.var_bin_wl * p.p1330);
        let assign4130_e5586: f64 = (assign4130_e5582 + assign4130_e5585);
        locals.var_c01_i = assign4130_e5586;
        locals.var_c01_i_rv = 0.0;

        let assign4140_e5590: f64 = (locals.var_bin_l * p.p1332);
        let assign4140_e5591: f64 = (p.p1331 + assign4140_e5590);
        let assign4140_e5594: f64 = (locals.var_bin_w * p.p1333);
        let assign4140_e5595: f64 = (assign4140_e5591 + assign4140_e5594);
        let assign4140_e5598: f64 = (locals.var_bin_wl * p.p1334);
        let assign4140_e5599: f64 = (assign4140_e5595 + assign4140_e5598);
        locals.var_c0si_i = assign4140_e5599;
        locals.var_c0si_i_rv = 0.0;

        let assign4150_e5603: f64 = (locals.var_bin_l * p.p1336);
        let assign4150_e5604: f64 = (p.p1335 + assign4150_e5603);
        let assign4150_e5607: f64 = (locals.var_bin_w * p.p1337);
        let assign4150_e5608: f64 = (assign4150_e5604 + assign4150_e5607);
        let assign4150_e5611: f64 = (locals.var_bin_wl * p.p1338);
        let assign4150_e5612: f64 = (assign4150_e5608 + assign4150_e5611);
        locals.var_c0si1_i = assign4150_e5612;
        locals.var_c0si1_i_rv = 0.0;

        let assign4160_e5616: f64 = (locals.var_bin_l * p.p1340);
        let assign4160_e5617: f64 = (p.p1339 + assign4160_e5616);
        let assign4160_e5620: f64 = (locals.var_bin_w * p.p1341);
        let assign4160_e5621: f64 = (assign4160_e5617 + assign4160_e5620);
        let assign4160_e5624: f64 = (locals.var_bin_wl * p.p1342);
        let assign4160_e5625: f64 = (assign4160_e5621 + assign4160_e5624);
        locals.var_c0sisat_i = assign4160_e5625;
        locals.var_c0sisat_i_rv = 0.0;

        let assign4170_e5629: f64 = (locals.var_bin_l * p.p1344);
        let assign4170_e5630: f64 = (p.p1343 + assign4170_e5629);
        let assign4170_e5633: f64 = (locals.var_bin_w * p.p1345);
        let assign4170_e5634: f64 = (assign4170_e5630 + assign4170_e5633);
        let assign4170_e5637: f64 = (locals.var_bin_wl * p.p1346);
        let assign4170_e5638: f64 = (assign4170_e5634 + assign4170_e5637);
        locals.var_c0sisat1_i = assign4170_e5638;
        locals.var_c0sisat1_i_rv = 0.0;

        let assign4180_e5642: f64 = (locals.var_bin_l * p.p787);
        let assign4180_e5643: f64 = (p.p783 + assign4180_e5642);
        let assign4180_e5646: f64 = (locals.var_bin_w * p.p791);
        let assign4180_e5647: f64 = (assign4180_e5643 + assign4180_e5646);
        let assign4180_e5650: f64 = (locals.var_bin_wl * p.p795);
        let assign4180_e5651: f64 = (assign4180_e5647 + assign4180_e5650);
        locals.var_aigbcp2_i = assign4180_e5651;
        locals.var_aigbcp2_i_dn4 = 0.0;
        locals.var_aigbcp2_i_dn5 = 0.0;
        locals.var_aigbcp2_i_rv = 0.0;

        let assign4190_e5655: f64 = (p.p788 * locals.var_bin_l);
        let assign4190_e5656: f64 = (p.p784 + assign4190_e5655);
        let assign4190_e5659: f64 = (p.p792 * locals.var_bin_w);
        let assign4190_e5660: f64 = (assign4190_e5656 + assign4190_e5659);
        let assign4190_e5663: f64 = (p.p796 * locals.var_bin_wl);
        let assign4190_e5664: f64 = (assign4190_e5660 + assign4190_e5663);
        locals.var_aigbcp2_t_i = assign4190_e5664;
        locals.var_aigbcp2_t_i_rv = 0.0;

        let assign4200_e5668: f64 = (locals.var_bin_l * p.p789);
        let assign4200_e5669: f64 = (p.p785 + assign4200_e5668);
        let assign4200_e5672: f64 = (locals.var_bin_w * p.p793);
        let assign4200_e5673: f64 = (assign4200_e5669 + assign4200_e5672);
        let assign4200_e5676: f64 = (locals.var_bin_wl * p.p797);
        let assign4200_e5677: f64 = (assign4200_e5673 + assign4200_e5676);
        locals.var_bigbcp2_i = assign4200_e5677;
        locals.var_bigbcp2_i_rv = 0.0;

        let assign4210_e5681: f64 = (locals.var_bin_l * p.p790);
        let assign4210_e5682: f64 = (p.p786 + assign4210_e5681);
        let assign4210_e5685: f64 = (locals.var_bin_w * p.p794);
        let assign4210_e5686: f64 = (assign4210_e5682 + assign4210_e5685);
        let assign4210_e5689: f64 = (locals.var_bin_wl * p.p798);
        let assign4210_e5690: f64 = (assign4210_e5686 + assign4210_e5689);
        locals.var_cigbcp2_i = assign4210_e5690;
        locals.var_cigbcp2_i_rv = 0.0;

        let assign4220_e5694: f64 = (locals.var_bin_l * p.p1385);
        let assign4220_e5695: f64 = (p.p1384 + assign4220_e5694);
        let assign4220_e5698: f64 = (locals.var_bin_w * p.p1386);
        let assign4220_e5699: f64 = (assign4220_e5695 + assign4220_e5698);
        let assign4220_e5702: f64 = (locals.var_bin_wl * p.p1387);
        let assign4220_e5703: f64 = (assign4220_e5699 + assign4220_e5702);
        locals.var_nsub_i = assign4220_e5703;
        locals.var_nsub_i_rv = 0.0;

        let assign4230_e5707: f64 = (locals.var_bin_l * p.p1390);
        let assign4230_e5708: f64 = (p.p1389 + assign4230_e5707);
        let assign4230_e5711: f64 = (locals.var_bin_w * p.p1391);
        let assign4230_e5712: f64 = (assign4230_e5708 + assign4230_e5711);
        let assign4230_e5715: f64 = (locals.var_bin_wl * p.p1392);
        let assign4230_e5716: f64 = (assign4230_e5712 + assign4230_e5715);
        locals.var_kb1_i = assign4230_e5716;
        locals.var_kb1_i_rv = 0.0;

        let assign4240_e5719: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign4240_e5719;
        locals.var_guard21_rv = 0.0;

        let (assign4250_e5735, assign4250_e5735_d_n3, assign4250_e5735_d_n4, assign4250_e5735_d_n5, assign4250_e5735_d_n6, assign4250_e5735_d_n7, assign4250_e5735_d_n8, assign4250_e5735_d_n9, assign4250_e5735_d_n10, assign4250_e5735_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4250_e5724: f64 = (locals.var_bin_l * p.p1173);
        let assign4250_e5725: f64 = (p.p1172 + assign4250_e5724);
        let assign4250_e5728: f64 = (locals.var_bin_w * p.p1174);
        let assign4250_e5729: f64 = (assign4250_e5725 + assign4250_e5728);
        let assign4250_e5732: f64 = (locals.var_bin_wl * p.p1175);
        let assign4250_e5733: f64 = (assign4250_e5729 + assign4250_e5732);
        (assign4250_e5733, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdedger_i, locals.var_cdscdedger_i_dn3, locals.var_cdscdedger_i_dn4, locals.var_cdscdedger_i_dn5, locals.var_cdscdedger_i_dn6, locals.var_cdscdedger_i_dn7, locals.var_cdscdedger_i_dn8, locals.var_cdscdedger_i_dn9, locals.var_cdscdedger_i_dn10, locals.var_cdscdedger_i_dn11,)
    }
};
        locals.var_cdscdedger_i = assign4250_e5735;
        locals.var_cdscdedger_i_dn3 = assign4250_e5735_d_n3;
        locals.var_cdscdedger_i_dn4 = assign4250_e5735_d_n4;
        locals.var_cdscdedger_i_dn5 = assign4250_e5735_d_n5;
        locals.var_cdscdedger_i_dn6 = assign4250_e5735_d_n6;
        locals.var_cdscdedger_i_dn7 = assign4250_e5735_d_n7;
        locals.var_cdscdedger_i_dn8 = assign4250_e5735_d_n8;
        locals.var_cdscdedger_i_dn9 = assign4250_e5735_d_n9;
        locals.var_cdscdedger_i_dn10 = assign4250_e5735_d_n10;
        locals.var_cdscdedger_i_dn11 = assign4250_e5735_d_n11;
        locals.var_cdscdedger_i_rv = 0.0;

        let (assign4260_e5751, assign4260_e5751_d_n3, assign4260_e5751_d_n4, assign4260_e5751_d_n5, assign4260_e5751_d_n6, assign4260_e5751_d_n7, assign4260_e5751_d_n8, assign4260_e5751_d_n9, assign4260_e5751_d_n10, assign4260_e5751_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4260_e5740: f64 = (locals.var_bin_l * p.p285);
        let assign4260_e5741: f64 = (p.p284 + assign4260_e5740);
        let assign4260_e5744: f64 = (locals.var_bin_w * p.p286);
        let assign4260_e5745: f64 = (assign4260_e5741 + assign4260_e5744);
        let assign4260_e5748: f64 = (locals.var_bin_wl * p.p287);
        let assign4260_e5749: f64 = (assign4260_e5745 + assign4260_e5748);
        (assign4260_e5749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11,)
    }
};
        locals.var_cdscdr_i = assign4260_e5751;
        locals.var_cdscdr_i_dn3 = assign4260_e5751_d_n3;
        locals.var_cdscdr_i_dn4 = assign4260_e5751_d_n4;
        locals.var_cdscdr_i_dn5 = assign4260_e5751_d_n5;
        locals.var_cdscdr_i_dn6 = assign4260_e5751_d_n6;
        locals.var_cdscdr_i_dn7 = assign4260_e5751_d_n7;
        locals.var_cdscdr_i_dn8 = assign4260_e5751_d_n8;
        locals.var_cdscdr_i_dn9 = assign4260_e5751_d_n9;
        locals.var_cdscdr_i_dn10 = assign4260_e5751_d_n10;
        locals.var_cdscdr_i_dn11 = assign4260_e5751_d_n11;
        locals.var_cdscdr_i_rv = 0.0;

        let (assign4270_e5767, assign4270_e5767_d_n3, assign4270_e5767_d_n4, assign4270_e5767_d_n5, assign4270_e5767_d_n6, assign4270_e5767_d_n7, assign4270_e5767_d_n8, assign4270_e5767_d_n9, assign4270_e5767_d_n10, assign4270_e5767_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4270_e5756: f64 = (locals.var_bin_l * p.p199);
        let assign4270_e5757: f64 = (p.p198 + assign4270_e5756);
        let assign4270_e5760: f64 = (locals.var_bin_w * p.p200);
        let assign4270_e5761: f64 = (assign4270_e5757 + assign4270_e5760);
        let assign4270_e5764: f64 = (locals.var_bin_wl * p.p201);
        let assign4270_e5765: f64 = (assign4270_e5761 + assign4270_e5764);
        (assign4270_e5765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11,)
    }
};
        locals.var_eta0r_i = assign4270_e5767;
        locals.var_eta0r_i_dn3 = assign4270_e5767_d_n3;
        locals.var_eta0r_i_dn4 = assign4270_e5767_d_n4;
        locals.var_eta0r_i_dn5 = assign4270_e5767_d_n5;
        locals.var_eta0r_i_dn6 = assign4270_e5767_d_n6;
        locals.var_eta0r_i_dn7 = assign4270_e5767_d_n7;
        locals.var_eta0r_i_dn8 = assign4270_e5767_d_n8;
        locals.var_eta0r_i_dn9 = assign4270_e5767_d_n9;
        locals.var_eta0r_i_dn10 = assign4270_e5767_d_n10;
        locals.var_eta0r_i_dn11 = assign4270_e5767_d_n11;
        locals.var_eta0r_i_rv = 0.0;

        let (assign4280_e5783,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4280_e5772: f64 = (locals.var_bin_l * p.p344);
        let assign4280_e5773: f64 = (p.p343 + assign4280_e5772);
        let assign4280_e5776: f64 = (locals.var_bin_w * p.p345);
        let assign4280_e5777: f64 = (assign4280_e5773 + assign4280_e5776);
        let assign4280_e5780: f64 = (locals.var_bin_wl * p.p346);
        let assign4280_e5781: f64 = (assign4280_e5777 + assign4280_e5780);
        (assign4280_e5781,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4280_e5783;
        locals.var_u0r_i_rv = 0.0;

        let (assign4290_e5799, assign4290_e5799_d_n3, assign4290_e5799_d_n4, assign4290_e5799_d_n5, assign4290_e5799_d_n6, assign4290_e5799_d_n7, assign4290_e5799_d_n8, assign4290_e5799_d_n9, assign4290_e5799_d_n10, assign4290_e5799_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4290_e5788: f64 = (locals.var_bin_l * p.p359);
        let assign4290_e5789: f64 = (p.p358 + assign4290_e5788);
        let assign4290_e5792: f64 = (locals.var_bin_w * p.p360);
        let assign4290_e5793: f64 = (assign4290_e5789 + assign4290_e5792);
        let assign4290_e5796: f64 = (locals.var_bin_wl * p.p361);
        let assign4290_e5797: f64 = (assign4290_e5793 + assign4290_e5796);
        (assign4290_e5797, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11,)
    }
};
        locals.var_uar_i = assign4290_e5799;
        locals.var_uar_i_dn3 = assign4290_e5799_d_n3;
        locals.var_uar_i_dn4 = assign4290_e5799_d_n4;
        locals.var_uar_i_dn5 = assign4290_e5799_d_n5;
        locals.var_uar_i_dn6 = assign4290_e5799_d_n6;
        locals.var_uar_i_dn7 = assign4290_e5799_d_n7;
        locals.var_uar_i_dn8 = assign4290_e5799_d_n8;
        locals.var_uar_i_dn9 = assign4290_e5799_d_n9;
        locals.var_uar_i_dn10 = assign4290_e5799_d_n10;
        locals.var_uar_i_dn11 = assign4290_e5799_d_n11;
        locals.var_uar_i_rv = 0.0;

        let (assign4300_e5815, assign4300_e5815_d_n3, assign4300_e5815_d_n4, assign4300_e5815_d_n5, assign4300_e5815_d_n6, assign4300_e5815_d_n7, assign4300_e5815_d_n8, assign4300_e5815_d_n9, assign4300_e5815_d_n10, assign4300_e5815_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4300_e5804: f64 = (locals.var_bin_l * p.p379);
        let assign4300_e5805: f64 = (p.p378 + assign4300_e5804);
        let assign4300_e5808: f64 = (locals.var_bin_w * p.p380);
        let assign4300_e5809: f64 = (assign4300_e5805 + assign4300_e5808);
        let assign4300_e5812: f64 = (locals.var_bin_wl * p.p381);
        let assign4300_e5813: f64 = (assign4300_e5809 + assign4300_e5812);
        (assign4300_e5813, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11,)
    }
};
        locals.var_udr_i = assign4300_e5815;
        locals.var_udr_i_dn3 = assign4300_e5815_d_n3;
        locals.var_udr_i_dn4 = assign4300_e5815_d_n4;
        locals.var_udr_i_dn5 = assign4300_e5815_d_n5;
        locals.var_udr_i_dn6 = assign4300_e5815_d_n6;
        locals.var_udr_i_dn7 = assign4300_e5815_d_n7;
        locals.var_udr_i_dn8 = assign4300_e5815_d_n8;
        locals.var_udr_i_dn9 = assign4300_e5815_d_n9;
        locals.var_udr_i_dn10 = assign4300_e5815_d_n10;
        locals.var_udr_i_dn11 = assign4300_e5815_d_n11;
        locals.var_udr_i_rv = 0.0;

        let (assign4310_e5831,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4310_e5820: f64 = (locals.var_bin_l * p.p387);
        let assign4310_e5821: f64 = (p.p386 + assign4310_e5820);
        let assign4310_e5824: f64 = (locals.var_bin_w * p.p388);
        let assign4310_e5825: f64 = (assign4310_e5821 + assign4310_e5824);
        let assign4310_e5828: f64 = (locals.var_bin_wl * p.p389);
        let assign4310_e5829: f64 = (assign4310_e5825 + assign4310_e5828);
        (assign4310_e5829,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign4310_e5831;
        locals.var_ucsr_i_rv = 0.0;

        let (assign4320_e5847, assign4320_e5847_d_n3, assign4320_e5847_d_n4, assign4320_e5847_d_n5, assign4320_e5847_d_n6, assign4320_e5847_d_n7, assign4320_e5847_d_n8, assign4320_e5847_d_n9, assign4320_e5847_d_n10, assign4320_e5847_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4320_e5836: f64 = (locals.var_bin_l * p.p401);
        let assign4320_e5837: f64 = (p.p400 + assign4320_e5836);
        let assign4320_e5840: f64 = (locals.var_bin_w * p.p402);
        let assign4320_e5841: f64 = (assign4320_e5837 + assign4320_e5840);
        let assign4320_e5844: f64 = (locals.var_bin_wl * p.p403);
        let assign4320_e5845: f64 = (assign4320_e5841 + assign4320_e5844);
        (assign4320_e5845, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11,)
    }
};
        locals.var_ucr_i = assign4320_e5847;
        locals.var_ucr_i_dn3 = assign4320_e5847_d_n3;
        locals.var_ucr_i_dn4 = assign4320_e5847_d_n4;
        locals.var_ucr_i_dn5 = assign4320_e5847_d_n5;
        locals.var_ucr_i_dn6 = assign4320_e5847_d_n6;
        locals.var_ucr_i_dn7 = assign4320_e5847_d_n7;
        locals.var_ucr_i_dn8 = assign4320_e5847_d_n8;
        locals.var_ucr_i_dn9 = assign4320_e5847_d_n9;
        locals.var_ucr_i_dn10 = assign4320_e5847_d_n10;
        locals.var_ucr_i_dn11 = assign4320_e5847_d_n11;
        locals.var_ucr_i_rv = 0.0;

        let (assign4330_e5863, assign4330_e5863_d_n3, assign4330_e5863_d_n4, assign4330_e5863_d_n5, assign4330_e5863_d_n6, assign4330_e5863_d_n7, assign4330_e5863_d_n8, assign4330_e5863_d_n9, assign4330_e5863_d_n10, assign4330_e5863_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4330_e5852: f64 = (locals.var_bin_l * p.p411);
        let assign4330_e5853: f64 = (p.p410 + assign4330_e5852);
        let assign4330_e5856: f64 = (locals.var_bin_w * p.p412);
        let assign4330_e5857: f64 = (assign4330_e5853 + assign4330_e5856);
        let assign4330_e5860: f64 = (locals.var_bin_wl * p.p413);
        let assign4330_e5861: f64 = (assign4330_e5857 + assign4330_e5860);
        (assign4330_e5861, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11,)
    }
};
        locals.var_pclmr_i = assign4330_e5863;
        locals.var_pclmr_i_dn3 = assign4330_e5863_d_n3;
        locals.var_pclmr_i_dn4 = assign4330_e5863_d_n4;
        locals.var_pclmr_i_dn5 = assign4330_e5863_d_n5;
        locals.var_pclmr_i_dn6 = assign4330_e5863_d_n6;
        locals.var_pclmr_i_dn7 = assign4330_e5863_d_n7;
        locals.var_pclmr_i_dn8 = assign4330_e5863_d_n8;
        locals.var_pclmr_i_dn9 = assign4330_e5863_d_n9;
        locals.var_pclmr_i_dn10 = assign4330_e5863_d_n10;
        locals.var_pclmr_i_dn11 = assign4330_e5863_d_n11;
        locals.var_pclmr_i_rv = 0.0;

        let (assign4340_e5879, assign4340_e5879_d_n3, assign4340_e5879_d_n4, assign4340_e5879_d_n5, assign4340_e5879_d_n6, assign4340_e5879_d_n7, assign4340_e5879_d_n8, assign4340_e5879_d_n9, assign4340_e5879_d_n10, assign4340_e5879_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4340_e5868: f64 = (locals.var_bin_l * p.p537);
        let assign4340_e5869: f64 = (p.p536 + assign4340_e5868);
        let assign4340_e5872: f64 = (locals.var_bin_w * p.p538);
        let assign4340_e5873: f64 = (assign4340_e5869 + assign4340_e5872);
        let assign4340_e5876: f64 = (locals.var_bin_wl * p.p539);
        let assign4340_e5877: f64 = (assign4340_e5873 + assign4340_e5876);
        (assign4340_e5877, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11,)
    }
};
        locals.var_pdiblcr_i = assign4340_e5879;
        locals.var_pdiblcr_i_dn3 = assign4340_e5879_d_n3;
        locals.var_pdiblcr_i_dn4 = assign4340_e5879_d_n4;
        locals.var_pdiblcr_i_dn5 = assign4340_e5879_d_n5;
        locals.var_pdiblcr_i_dn6 = assign4340_e5879_d_n6;
        locals.var_pdiblcr_i_dn7 = assign4340_e5879_d_n7;
        locals.var_pdiblcr_i_dn8 = assign4340_e5879_d_n8;
        locals.var_pdiblcr_i_dn9 = assign4340_e5879_d_n9;
        locals.var_pdiblcr_i_dn10 = assign4340_e5879_d_n10;
        locals.var_pdiblcr_i_dn11 = assign4340_e5879_d_n11;
        locals.var_pdiblcr_i_rv = 0.0;

        let (assign4350_e5895, assign4350_e5895_d_n3, assign4350_e5895_d_n4, assign4350_e5895_d_n5, assign4350_e5895_d_n6, assign4350_e5895_d_n7, assign4350_e5895_d_n8, assign4350_e5895_d_n9, assign4350_e5895_d_n10, assign4350_e5895_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4350_e5884: f64 = (locals.var_bin_l * p.p306);
        let assign4350_e5885: f64 = (p.p305 + assign4350_e5884);
        let assign4350_e5888: f64 = (locals.var_bin_w * p.p307);
        let assign4350_e5889: f64 = (assign4350_e5885 + assign4350_e5888);
        let assign4350_e5892: f64 = (locals.var_bin_wl * p.p308);
        let assign4350_e5893: f64 = (assign4350_e5889 + assign4350_e5892);
        (assign4350_e5893, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11,)
    }
};
        locals.var_vsatr_i = assign4350_e5895;
        locals.var_vsatr_i_dn3 = assign4350_e5895_d_n3;
        locals.var_vsatr_i_dn4 = assign4350_e5895_d_n4;
        locals.var_vsatr_i_dn5 = assign4350_e5895_d_n5;
        locals.var_vsatr_i_dn6 = assign4350_e5895_d_n6;
        locals.var_vsatr_i_dn7 = assign4350_e5895_d_n7;
        locals.var_vsatr_i_dn8 = assign4350_e5895_d_n8;
        locals.var_vsatr_i_dn9 = assign4350_e5895_d_n9;
        locals.var_vsatr_i_dn10 = assign4350_e5895_d_n10;
        locals.var_vsatr_i_dn11 = assign4350_e5895_d_n11;
        locals.var_vsatr_i_rv = 0.0;

        let (assign4360_e5911,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4360_e5900: f64 = (locals.var_bin_l * p.p491);
        let assign4360_e5901: f64 = (p.p490 + assign4360_e5900);
        let assign4360_e5904: f64 = (locals.var_bin_w * p.p492);
        let assign4360_e5905: f64 = (assign4360_e5901 + assign4360_e5904);
        let assign4360_e5908: f64 = (locals.var_bin_wl * p.p493);
        let assign4360_e5909: f64 = (assign4360_e5905 + assign4360_e5908);
        (assign4360_e5909,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign4360_e5911;
        locals.var_psatr_i_rv = 0.0;

        let (assign4370_e5927, assign4370_e5927_d_n3, assign4370_e5927_d_n4, assign4370_e5927_d_n5, assign4370_e5927_d_n6, assign4370_e5927_d_n7, assign4370_e5927_d_n8, assign4370_e5927_d_n9, assign4370_e5927_d_n10, assign4370_e5927_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign4370_e5916: f64 = (locals.var_bin_l * p.p507);
        let assign4370_e5917: f64 = (p.p506 + assign4370_e5916);
        let assign4370_e5920: f64 = (locals.var_bin_w * p.p508);
        let assign4370_e5921: f64 = (assign4370_e5917 + assign4370_e5920);
        let assign4370_e5924: f64 = (locals.var_bin_wl * p.p509);
        let assign4370_e5925: f64 = (assign4370_e5921 + assign4370_e5924);
        (assign4370_e5925, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11,)
    }
};
        locals.var_ptwgr_i = assign4370_e5927;
        locals.var_ptwgr_i_dn3 = assign4370_e5927_d_n3;
        locals.var_ptwgr_i_dn4 = assign4370_e5927_d_n4;
        locals.var_ptwgr_i_dn5 = assign4370_e5927_d_n5;
        locals.var_ptwgr_i_dn6 = assign4370_e5927_d_n6;
        locals.var_ptwgr_i_dn7 = assign4370_e5927_d_n7;
        locals.var_ptwgr_i_dn8 = assign4370_e5927_d_n8;
        locals.var_ptwgr_i_dn9 = assign4370_e5927_d_n9;
        locals.var_ptwgr_i_dn10 = assign4370_e5927_d_n10;
        locals.var_ptwgr_i_dn11 = assign4370_e5927_d_n11;
        locals.var_ptwgr_i_rv = 0.0;

        let assign4380_e5931: f64 = (locals.var_inv_l).powf(p.p81);
        let assign4380_e5934: f64 = (locals.var_inv_llong).powf(p.p81);
        let assign4380_e5935: f64 = (assign4380_e5931 - assign4380_e5934);
        let assign4380_e5937: f64 = (assign4380_e5935).max(0.0);
        let assign4380_e5938: f64 = (p.p80 * assign4380_e5937);
        let assign4380_e5942: f64 = (locals.var_inv_l).powf(p.p83);
        let assign4380_e5945: f64 = (locals.var_inv_llong).powf(p.p83);
        let assign4380_e5946: f64 = (assign4380_e5942 - assign4380_e5945);
        let assign4380_e5948: f64 = (assign4380_e5946).max(0.0);
        let assign4380_e5949: f64 = (p.p82 * assign4380_e5948);
        let assign4380_e5950: f64 = (assign4380_e5938 + assign4380_e5949);
        locals.var_t0 = assign4380_e5950;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4390_e5954: f64 = (locals.var_inv_w).powf(p.p85);
        let assign4390_e5957: f64 = (locals.var_inv_wwide).powf(p.p85);
        let assign4390_e5958: f64 = (assign4390_e5954 - assign4390_e5957);
        let assign4390_e5960: f64 = (assign4390_e5958).max(0.0);
        let assign4390_e5961: f64 = (p.p84 * assign4390_e5960);
        let assign4390_e5965: f64 = (locals.var_inv_w * locals.var_inv_l);
        let assign4390_e5967: f64 = (assign4390_e5965).powf(p.p87);
        let assign4390_e5968: f64 = (p.p86 * assign4390_e5967);
        let assign4390_e5969: f64 = (assign4390_e5961 + assign4390_e5968);
        locals.var_t1 = assign4390_e5969;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4400_e5973: f64 = (1.0 + locals.var_t0);
        let assign4400_e5975: f64 = (assign4400_e5973 + locals.var_t1);
        let assign4400_e5976: f64 = (locals.var_ndep_i * assign4400_e5975);
        locals.var_ndep_i = assign4400_e5976;
        locals.var_ndep_i_dn3 = ((locals.var_ndep_i_dn3 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndep_i_dn4 = ((locals.var_ndep_i_dn4 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndep_i_dn5 = ((locals.var_ndep_i_dn5 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndep_i_dn6 = ((locals.var_ndep_i_dn6 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndep_i_dn7 = ((locals.var_ndep_i_dn7 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndep_i_dn8 = ((locals.var_ndep_i_dn8 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndep_i_dn9 = ((locals.var_ndep_i_dn9 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndep_i_dn10 = ((locals.var_ndep_i_dn10 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndep_i_dn11 = ((locals.var_ndep_i_dn11 * assign4400_e5975) + (locals.var_ndep_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndep_i_rv = 0.0;

        let assign4410_e5980: f64 = (locals.var_inv_l).powf(p.p238);
        let assign4410_e5983: f64 = (locals.var_inv_llong).powf(p.p238);
        let assign4410_e5984: f64 = (assign4410_e5980 - assign4410_e5983);
        let assign4410_e5986: f64 = (assign4410_e5984).max(0.0);
        let assign4410_e5987: f64 = (p.p237 * assign4410_e5986);
        locals.var_t0 = assign4410_e5987;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4420_e5991: f64 = (locals.var_inv_w).powf(p.p240);
        let assign4420_e5994: f64 = (locals.var_inv_wwide).powf(p.p240);
        let assign4420_e5995: f64 = (assign4420_e5991 - assign4420_e5994);
        let assign4420_e5997: f64 = (assign4420_e5995).max(0.0);
        let assign4420_e5998: f64 = (p.p239 * assign4420_e5997);
        let assign4420_e6002: f64 = (locals.var_inv_wl).powf(p.p242);
        let assign4420_e6003: f64 = (p.p241 * assign4420_e6002);
        let assign4420_e6004: f64 = (assign4420_e5998 + assign4420_e6003);
        locals.var_t1 = assign4420_e6004;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4430_e6008: f64 = (1.0 + locals.var_t0);
        let assign4430_e6010: f64 = (assign4430_e6008 + locals.var_t1);
        let assign4430_e6011: f64 = (locals.var_nfactor_i * assign4430_e6010);
        locals.var_nfactor_i = assign4430_e6011;
        locals.var_nfactor_i_dn3 = ((locals.var_nfactor_i_dn3 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_nfactor_i_dn4 = ((locals.var_nfactor_i_dn4 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_nfactor_i_dn5 = ((locals.var_nfactor_i_dn5 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_nfactor_i_dn6 = ((locals.var_nfactor_i_dn6 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_nfactor_i_dn7 = ((locals.var_nfactor_i_dn7 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_nfactor_i_dn8 = ((locals.var_nfactor_i_dn8 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_nfactor_i_dn9 = ((locals.var_nfactor_i_dn9 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_nfactor_i_dn10 = ((locals.var_nfactor_i_dn10 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_nfactor_i_dn11 = ((locals.var_nfactor_i_dn11 * assign4430_e6010) + (locals.var_nfactor_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_nfactor_i_rv = 0.0;

        let assign4440_e6016: f64 = (locals.var_inv_l).powf(p.p283);
        let assign4440_e6019: f64 = (locals.var_inv_llong).powf(p.p283);
        let assign4440_e6020: f64 = (assign4440_e6016 - assign4440_e6019);
        let assign4440_e6022: f64 = (assign4440_e6020).max(0.0);
        let assign4440_e6023: f64 = (p.p282 * assign4440_e6022);
        let assign4440_e6024: f64 = (1.0 + assign4440_e6023);
        locals.var_t0 = assign4440_e6024;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4450_e6027: f64 = (locals.var_cdscd_i * locals.var_t0);
        locals.var_cdscd_i = assign4450_e6027;
        locals.var_cdscd_i_dn3 = ((locals.var_cdscd_i_dn3 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn3));
        locals.var_cdscd_i_dn4 = ((locals.var_cdscd_i_dn4 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn4));
        locals.var_cdscd_i_dn5 = ((locals.var_cdscd_i_dn5 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn5));
        locals.var_cdscd_i_dn6 = ((locals.var_cdscd_i_dn6 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn6));
        locals.var_cdscd_i_dn7 = ((locals.var_cdscd_i_dn7 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn7));
        locals.var_cdscd_i_dn8 = ((locals.var_cdscd_i_dn8 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn8));
        locals.var_cdscd_i_dn9 = ((locals.var_cdscd_i_dn9 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn9));
        locals.var_cdscd_i_dn10 = ((locals.var_cdscd_i_dn10 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn10));
        locals.var_cdscd_i_dn11 = ((locals.var_cdscd_i_dn11 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn11));
        locals.var_cdscd_i_rv = 0.0;

        let assign4460_e6030: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign4460_e6030;
        locals.var_guard22_rv = 0.0;

        let (assign4470_e6036, assign4470_e6036_d_n3, assign4470_e6036_d_n4, assign4470_e6036_d_n5, assign4470_e6036_d_n6, assign4470_e6036_d_n7, assign4470_e6036_d_n8, assign4470_e6036_d_n9, assign4470_e6036_d_n10, assign4470_e6036_d_n11,) = {
    if (locals.var_guard22 != 0.0) {
        let assign4470_e6034: f64 = (locals.var_cdscdedger_i * locals.var_t0);
        (assign4470_e6034, ((locals.var_cdscdedger_i_dn3 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn3)), ((locals.var_cdscdedger_i_dn4 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn4)), ((locals.var_cdscdedger_i_dn5 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn5)), ((locals.var_cdscdedger_i_dn6 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn6)), ((locals.var_cdscdedger_i_dn7 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn7)), ((locals.var_cdscdedger_i_dn8 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn8)), ((locals.var_cdscdedger_i_dn9 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn9)), ((locals.var_cdscdedger_i_dn10 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn10)), ((locals.var_cdscdedger_i_dn11 * locals.var_t0) + (locals.var_cdscdedger_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_cdscdedger_i, locals.var_cdscdedger_i_dn3, locals.var_cdscdedger_i_dn4, locals.var_cdscdedger_i_dn5, locals.var_cdscdedger_i_dn6, locals.var_cdscdedger_i_dn7, locals.var_cdscdedger_i_dn8, locals.var_cdscdedger_i_dn9, locals.var_cdscdedger_i_dn10, locals.var_cdscdedger_i_dn11,)
    }
};
        locals.var_cdscdedger_i = assign4470_e6036;
        locals.var_cdscdedger_i_dn3 = assign4470_e6036_d_n3;
        locals.var_cdscdedger_i_dn4 = assign4470_e6036_d_n4;
        locals.var_cdscdedger_i_dn5 = assign4470_e6036_d_n5;
        locals.var_cdscdedger_i_dn6 = assign4470_e6036_d_n6;
        locals.var_cdscdedger_i_dn7 = assign4470_e6036_d_n7;
        locals.var_cdscdedger_i_dn8 = assign4470_e6036_d_n8;
        locals.var_cdscdedger_i_dn9 = assign4470_e6036_d_n9;
        locals.var_cdscdedger_i_dn10 = assign4470_e6036_d_n10;
        locals.var_cdscdedger_i_dn11 = assign4470_e6036_d_n11;
        locals.var_cdscdedger_i_rv = 0.0;

        let (assign4480_e6042, assign4480_e6042_d_n3, assign4480_e6042_d_n4, assign4480_e6042_d_n5, assign4480_e6042_d_n6, assign4480_e6042_d_n7, assign4480_e6042_d_n8, assign4480_e6042_d_n9, assign4480_e6042_d_n10, assign4480_e6042_d_n11,) = {
    if (locals.var_guard22 != 0.0) {
        let assign4480_e6040: f64 = (locals.var_cdscdr_i * locals.var_t0);
        (assign4480_e6040, ((locals.var_cdscdr_i_dn3 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn3)), ((locals.var_cdscdr_i_dn4 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn4)), ((locals.var_cdscdr_i_dn5 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn5)), ((locals.var_cdscdr_i_dn6 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn6)), ((locals.var_cdscdr_i_dn7 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn7)), ((locals.var_cdscdr_i_dn8 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn8)), ((locals.var_cdscdr_i_dn9 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn9)), ((locals.var_cdscdr_i_dn10 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn10)), ((locals.var_cdscdr_i_dn11 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11,)
    }
};
        locals.var_cdscdr_i = assign4480_e6042;
        locals.var_cdscdr_i_dn3 = assign4480_e6042_d_n3;
        locals.var_cdscdr_i_dn4 = assign4480_e6042_d_n4;
        locals.var_cdscdr_i_dn5 = assign4480_e6042_d_n5;
        locals.var_cdscdr_i_dn6 = assign4480_e6042_d_n6;
        locals.var_cdscdr_i_dn7 = assign4480_e6042_d_n7;
        locals.var_cdscdr_i_dn8 = assign4480_e6042_d_n8;
        locals.var_cdscdr_i_dn9 = assign4480_e6042_d_n9;
        locals.var_cdscdr_i_dn10 = assign4480_e6042_d_n10;
        locals.var_cdscdr_i_dn11 = assign4480_e6042_d_n11;
        locals.var_cdscdr_i_rv = 0.0;

        let assign4490_e6048: f64 = (locals.var_inv_l).powf(p.p290);
        let assign4490_e6051: f64 = (locals.var_inv_llong).powf(p.p290);
        let assign4490_e6052: f64 = (assign4490_e6048 - assign4490_e6051);
        let assign4490_e6054: f64 = (assign4490_e6052).max(0.0);
        let assign4490_e6055: f64 = (p.p289 * assign4490_e6054);
        let assign4490_e6056: f64 = (1.0 + assign4490_e6055);
        let assign4490_e6057: f64 = (locals.var_cdscb_i * assign4490_e6056);
        locals.var_cdscb_i = assign4490_e6057;
        locals.var_cdscb_i_rv = 0.0;

        let assign4500_e6060: f64 = (p.p24 * locals.var_u0_i);
        locals.var_u0_i = assign4500_e6060;
        locals.var_u0_i_rv = 0.0;

        let assign4510_e6063: f64 = if p.p42 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign4510_e6063;
        locals.var_guard23_rv = 0.0;

        let assign4520_e6066: f64 = if p.p339 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign4520_e6066;
        locals.var_guard24_rv = 0.0;

        let (assign4530_e6086,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) {
        let assign4530_e6075: f64 = (locals.var_inv_l).powf(p.p339);
        let assign4530_e6078: f64 = (locals.var_inv_llong).powf(p.p339);
        let assign4530_e6079: f64 = (assign4530_e6075 - assign4530_e6078);
        let assign4530_e6081: f64 = (assign4530_e6079).max(0.0);
        let assign4530_e6082: f64 = (p.p338 * assign4530_e6081);
        let assign4530_e6083: f64 = (1.0 - assign4530_e6082);
        let assign4530_e6084: f64 = (locals.var_u0_i * assign4530_e6083);
        (assign4530_e6084,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4530_e6086;
        locals.var_u0_i_rv = 0.0;

        let assign4540_e6089: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign4540_e6089;
        locals.var_guard25_rv = 0.0;

        let (assign4550_e6111,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) && (locals.var_guard25 != 0.0)) {
        let assign4550_e6100: f64 = (locals.var_inv_l).powf(p.p339);
        let assign4550_e6103: f64 = (locals.var_inv_llong).powf(p.p339);
        let assign4550_e6104: f64 = (assign4550_e6100 - assign4550_e6103);
        let assign4550_e6106: f64 = (assign4550_e6104).max(0.0);
        let assign4550_e6107: f64 = (p.p338 * assign4550_e6106);
        let assign4550_e6108: f64 = (1.0 - assign4550_e6107);
        let assign4550_e6109: f64 = (locals.var_u0r_i * assign4550_e6108);
        (assign4550_e6109,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4550_e6111;
        locals.var_u0r_i_rv = 0.0;

        let (assign4560_e6122,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) {
        let assign4560_e6119: f64 = (1.0 - p.p338);
        let assign4560_e6120: f64 = (locals.var_u0_i * assign4560_e6119);
        (assign4560_e6120,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4560_e6122;
        locals.var_u0_i_rv = 0.0;

        let assign4570_e6125: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign4570_e6125;
        locals.var_guard26_rv = 0.0;

        let (assign4580_e6138,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) && (locals.var_guard26 != 0.0)) {
        let assign4580_e6135: f64 = (1.0 - p.p338);
        let assign4580_e6136: f64 = (locals.var_u0r_i * assign4580_e6135);
        (assign4580_e6136,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4580_e6138;
        locals.var_u0r_i_rv = 0.0;

        let (assign4590_e6161,) = {
    if (locals.var_guard23 == 0.0) {
        let assign4590_e6145: f64 = (-locals.var_leff);
        let assign4590_e6147: f64 = (assign4590_e6145 / p.p334);
        let assign4590_e6148: f64 = { let limited_exp_arg = assign4590_e6147; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4590_e6149: f64 = (p.p333 * assign4590_e6148);
        let assign4590_e6150: f64 = (1.0 - assign4590_e6149);
        let assign4590_e6153: f64 = (-locals.var_leff);
        let assign4590_e6155: f64 = (assign4590_e6153 / p.p336);
        let assign4590_e6156: f64 = { let limited_exp_arg = assign4590_e6155; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4590_e6157: f64 = (p.p335 * assign4590_e6156);
        let assign4590_e6158: f64 = (assign4590_e6150 - assign4590_e6157);
        let assign4590_e6159: f64 = (locals.var_u0_i * assign4590_e6158);
        (assign4590_e6159,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4590_e6161;
        locals.var_u0_i_rv = 0.0;

        let assign4600_e6164: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign4600_e6164;
        locals.var_guard27_rv = 0.0;

        let (assign4610_e6189,) = {
    if ((locals.var_guard23 == 0.0) && (locals.var_guard27 != 0.0)) {
        let assign4610_e6173: f64 = (-locals.var_leff);
        let assign4610_e6175: f64 = (assign4610_e6173 / p.p334);
        let assign4610_e6176: f64 = { let limited_exp_arg = assign4610_e6175; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4610_e6177: f64 = (p.p333 * assign4610_e6176);
        let assign4610_e6178: f64 = (1.0 - assign4610_e6177);
        let assign4610_e6181: f64 = (-locals.var_leff);
        let assign4610_e6183: f64 = (assign4610_e6181 / p.p336);
        let assign4610_e6184: f64 = { let limited_exp_arg = assign4610_e6183; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign4610_e6185: f64 = (p.p335 * assign4610_e6184);
        let assign4610_e6186: f64 = (assign4610_e6178 - assign4610_e6185);
        let assign4610_e6187: f64 = (locals.var_u0r_i * assign4610_e6186);
        (assign4610_e6187,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign4610_e6189;
        locals.var_u0r_i_rv = 0.0;

        let assign4620_e6193: f64 = (locals.var_inv_l).powf(p.p350);
        let assign4620_e6196: f64 = (locals.var_inv_llong).powf(p.p350);
        let assign4620_e6197: f64 = (assign4620_e6193 - assign4620_e6196);
        let assign4620_e6199: f64 = (assign4620_e6197).max(0.0);
        let assign4620_e6200: f64 = (p.p349 * assign4620_e6199);
        locals.var_t0 = assign4620_e6200;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4630_e6204: f64 = (locals.var_inv_w).powf(p.p352);
        let assign4630_e6207: f64 = (locals.var_inv_wwide).powf(p.p352);
        let assign4630_e6208: f64 = (assign4630_e6204 - assign4630_e6207);
        let assign4630_e6210: f64 = (assign4630_e6208).max(0.0);
        let assign4630_e6211: f64 = (p.p351 * assign4630_e6210);
        let assign4630_e6215: f64 = (locals.var_inv_wl).powf(p.p354);
        let assign4630_e6216: f64 = (p.p353 * assign4630_e6215);
        let assign4630_e6217: f64 = (assign4630_e6211 + assign4630_e6216);
        locals.var_t1 = assign4630_e6217;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4640_e6221: f64 = (1.0 + locals.var_t0);
        let assign4640_e6223: f64 = (assign4640_e6221 + locals.var_t1);
        let assign4640_e6224: f64 = (locals.var_ua_i * assign4640_e6223);
        locals.var_ua_i = assign4640_e6224;
        locals.var_ua_i_dn3 = ((locals.var_ua_i_dn3 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ua_i_dn4 = ((locals.var_ua_i_dn4 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ua_i_dn5 = ((locals.var_ua_i_dn5 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ua_i_dn6 = ((locals.var_ua_i_dn6 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ua_i_dn7 = ((locals.var_ua_i_dn7 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ua_i_dn8 = ((locals.var_ua_i_dn8 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ua_i_dn9 = ((locals.var_ua_i_dn9 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ua_i_dn10 = ((locals.var_ua_i_dn10 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ua_i_dn11 = ((locals.var_ua_i_dn11 * assign4640_e6223) + (locals.var_ua_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ua_i_rv = 0.0;

        let assign4650_e6227: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign4650_e6227;
        locals.var_guard28_rv = 0.0;

        let (assign4660_e6237, assign4660_e6237_d_n3, assign4660_e6237_d_n4, assign4660_e6237_d_n5, assign4660_e6237_d_n6, assign4660_e6237_d_n7, assign4660_e6237_d_n8, assign4660_e6237_d_n9, assign4660_e6237_d_n10, assign4660_e6237_d_n11,) = {
    if (locals.var_guard28 != 0.0) {
        let assign4660_e6232: f64 = (1.0 + locals.var_t0);
        let assign4660_e6234: f64 = (assign4660_e6232 + locals.var_t1);
        let assign4660_e6235: f64 = (locals.var_uar_i * assign4660_e6234);
        (assign4660_e6235, ((locals.var_uar_i_dn3 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_uar_i_dn4 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_uar_i_dn5 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_uar_i_dn6 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_uar_i_dn7 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_uar_i_dn8 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_uar_i_dn9 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_uar_i_dn10 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_uar_i_dn11 * assign4660_e6234) + (locals.var_uar_i * (locals.var_t0_dn11 + locals.var_t1_dn11))),)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11,)
    }
};
        locals.var_uar_i = assign4660_e6237;
        locals.var_uar_i_dn3 = assign4660_e6237_d_n3;
        locals.var_uar_i_dn4 = assign4660_e6237_d_n4;
        locals.var_uar_i_dn5 = assign4660_e6237_d_n5;
        locals.var_uar_i_dn6 = assign4660_e6237_d_n6;
        locals.var_uar_i_dn7 = assign4660_e6237_d_n7;
        locals.var_uar_i_dn8 = assign4660_e6237_d_n8;
        locals.var_uar_i_dn9 = assign4660_e6237_d_n9;
        locals.var_uar_i_dn10 = assign4660_e6237_d_n10;
        locals.var_uar_i_dn11 = assign4660_e6237_d_n11;
        locals.var_uar_i_rv = 0.0;

        let assign4670_e6241: f64 = (locals.var_inv_l).powf(p.p367);
        let assign4670_e6244: f64 = (locals.var_inv_llong).powf(p.p367);
        let assign4670_e6245: f64 = (assign4670_e6241 - assign4670_e6244);
        let assign4670_e6247: f64 = (assign4670_e6245).max(0.0);
        let assign4670_e6248: f64 = (p.p366 * assign4670_e6247);
        locals.var_t0 = assign4670_e6248;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4680_e6252: f64 = (locals.var_inv_w).powf(p.p369);
        let assign4680_e6255: f64 = (locals.var_inv_wwide).powf(p.p369);
        let assign4680_e6256: f64 = (assign4680_e6252 - assign4680_e6255);
        let assign4680_e6258: f64 = (assign4680_e6256).max(0.0);
        let assign4680_e6259: f64 = (p.p368 * assign4680_e6258);
        let assign4680_e6263: f64 = (locals.var_inv_wl).powf(p.p371);
        let assign4680_e6264: f64 = (p.p370 * assign4680_e6263);
        let assign4680_e6265: f64 = (assign4680_e6259 + assign4680_e6264);
        locals.var_t1 = assign4680_e6265;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4690_e6269: f64 = (1.0 + locals.var_t0);
        let assign4690_e6271: f64 = (assign4690_e6269 + locals.var_t1);
        let assign4690_e6272: f64 = (locals.var_eu_i * assign4690_e6271);
        locals.var_eu_i = assign4690_e6272;
        locals.var_eu_i_dn3 = ((locals.var_eu_i_dn3 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_eu_i_dn4 = ((locals.var_eu_i_dn4 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_eu_i_dn5 = ((locals.var_eu_i_dn5 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_eu_i_dn6 = ((locals.var_eu_i_dn6 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_eu_i_dn7 = ((locals.var_eu_i_dn7 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_eu_i_dn8 = ((locals.var_eu_i_dn8 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_eu_i_dn9 = ((locals.var_eu_i_dn9 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_eu_i_dn10 = ((locals.var_eu_i_dn10 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_eu_i_dn11 = ((locals.var_eu_i_dn11 * assign4690_e6271) + (locals.var_eu_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_eu_i_rv = 0.0;

        let assign4700_e6277: f64 = (locals.var_inv_l).powf(p.p374);
        let assign4700_e6280: f64 = (locals.var_inv_llong).powf(p.p374);
        let assign4700_e6281: f64 = (assign4700_e6277 - assign4700_e6280);
        let assign4700_e6283: f64 = (assign4700_e6281).max(0.0);
        let assign4700_e6284: f64 = (p.p373 * assign4700_e6283);
        let assign4700_e6285: f64 = (1.0 + assign4700_e6284);
        locals.var_t0 = assign4700_e6285;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4710_e6288: f64 = (locals.var_ud_i * locals.var_t0);
        locals.var_ud_i = assign4710_e6288;
        locals.var_ud_i_dn3 = ((locals.var_ud_i_dn3 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn3));
        locals.var_ud_i_dn4 = ((locals.var_ud_i_dn4 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn4));
        locals.var_ud_i_dn5 = ((locals.var_ud_i_dn5 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn5));
        locals.var_ud_i_dn6 = ((locals.var_ud_i_dn6 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn6));
        locals.var_ud_i_dn7 = ((locals.var_ud_i_dn7 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn7));
        locals.var_ud_i_dn8 = ((locals.var_ud_i_dn8 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn8));
        locals.var_ud_i_dn9 = ((locals.var_ud_i_dn9 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn9));
        locals.var_ud_i_dn10 = ((locals.var_ud_i_dn10 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn10));
        locals.var_ud_i_dn11 = ((locals.var_ud_i_dn11 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn11));
        locals.var_ud_i_rv = 0.0;

        let assign4720_e6291: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign4720_e6291;
        locals.var_guard29_rv = 0.0;

        let (assign4730_e6297, assign4730_e6297_d_n3, assign4730_e6297_d_n4, assign4730_e6297_d_n5, assign4730_e6297_d_n6, assign4730_e6297_d_n7, assign4730_e6297_d_n8, assign4730_e6297_d_n9, assign4730_e6297_d_n10, assign4730_e6297_d_n11,) = {
    if (locals.var_guard29 != 0.0) {
        let assign4730_e6295: f64 = (locals.var_udr_i * locals.var_t0);
        (assign4730_e6295, ((locals.var_udr_i_dn3 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn3)), ((locals.var_udr_i_dn4 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn4)), ((locals.var_udr_i_dn5 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn5)), ((locals.var_udr_i_dn6 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn6)), ((locals.var_udr_i_dn7 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn7)), ((locals.var_udr_i_dn8 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn8)), ((locals.var_udr_i_dn9 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn9)), ((locals.var_udr_i_dn10 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn10)), ((locals.var_udr_i_dn11 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11,)
    }
};
        locals.var_udr_i = assign4730_e6297;
        locals.var_udr_i_dn3 = assign4730_e6297_d_n3;
        locals.var_udr_i_dn4 = assign4730_e6297_d_n4;
        locals.var_udr_i_dn5 = assign4730_e6297_d_n5;
        locals.var_udr_i_dn6 = assign4730_e6297_d_n6;
        locals.var_udr_i_dn7 = assign4730_e6297_d_n7;
        locals.var_udr_i_dn8 = assign4730_e6297_d_n8;
        locals.var_udr_i_dn9 = assign4730_e6297_d_n9;
        locals.var_udr_i_dn10 = assign4730_e6297_d_n10;
        locals.var_udr_i_dn11 = assign4730_e6297_d_n11;
        locals.var_udr_i_rv = 0.0;

        let assign4740_e6301: f64 = (locals.var_inv_l).powf(p.p392);
        let assign4740_e6304: f64 = (locals.var_inv_llong).powf(p.p392);
        let assign4740_e6305: f64 = (assign4740_e6301 - assign4740_e6304);
        let assign4740_e6307: f64 = (assign4740_e6305).max(0.0);
        let assign4740_e6308: f64 = (p.p391 * assign4740_e6307);
        locals.var_t0 = assign4740_e6308;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4750_e6312: f64 = (locals.var_inv_w).powf(p.p394);
        let assign4750_e6315: f64 = (locals.var_inv_wwide).powf(p.p394);
        let assign4750_e6316: f64 = (assign4750_e6312 - assign4750_e6315);
        let assign4750_e6318: f64 = (assign4750_e6316).max(0.0);
        let assign4750_e6319: f64 = (p.p393 * assign4750_e6318);
        let assign4750_e6323: f64 = (locals.var_inv_wl).powf(p.p396);
        let assign4750_e6324: f64 = (p.p395 * assign4750_e6323);
        let assign4750_e6325: f64 = (assign4750_e6319 + assign4750_e6324);
        locals.var_t1 = assign4750_e6325;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4760_e6329: f64 = (1.0 + locals.var_t0);
        let assign4760_e6331: f64 = (assign4760_e6329 + locals.var_t1);
        let assign4760_e6332: f64 = (locals.var_uc_i * assign4760_e6331);
        locals.var_uc_i = assign4760_e6332;
        locals.var_uc_i_dn3 = ((locals.var_uc_i_dn3 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_uc_i_dn4 = ((locals.var_uc_i_dn4 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_uc_i_dn5 = ((locals.var_uc_i_dn5 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_uc_i_dn6 = ((locals.var_uc_i_dn6 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_uc_i_dn7 = ((locals.var_uc_i_dn7 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_uc_i_dn8 = ((locals.var_uc_i_dn8 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_uc_i_dn9 = ((locals.var_uc_i_dn9 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_uc_i_dn10 = ((locals.var_uc_i_dn10 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_uc_i_dn11 = ((locals.var_uc_i_dn11 * assign4760_e6331) + (locals.var_uc_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_uc_i_rv = 0.0;

        let assign4770_e6335: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4770_e6335;
        locals.var_guard30_rv = 0.0;

        let (assign4780_e6345, assign4780_e6345_d_n3, assign4780_e6345_d_n4, assign4780_e6345_d_n5, assign4780_e6345_d_n6, assign4780_e6345_d_n7, assign4780_e6345_d_n8, assign4780_e6345_d_n9, assign4780_e6345_d_n10, assign4780_e6345_d_n11,) = {
    if (locals.var_guard30 != 0.0) {
        let assign4780_e6340: f64 = (1.0 + locals.var_t0);
        let assign4780_e6342: f64 = (assign4780_e6340 + locals.var_t1);
        let assign4780_e6343: f64 = (locals.var_ucr_i * assign4780_e6342);
        (assign4780_e6343, ((locals.var_ucr_i_dn3 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ucr_i_dn4 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ucr_i_dn5 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ucr_i_dn6 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ucr_i_dn7 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ucr_i_dn8 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ucr_i_dn9 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ucr_i_dn10 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ucr_i_dn11 * assign4780_e6342) + (locals.var_ucr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))),)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11,)
    }
};
        locals.var_ucr_i = assign4780_e6345;
        locals.var_ucr_i_dn3 = assign4780_e6345_d_n3;
        locals.var_ucr_i_dn4 = assign4780_e6345_d_n4;
        locals.var_ucr_i_dn5 = assign4780_e6345_d_n5;
        locals.var_ucr_i_dn6 = assign4780_e6345_d_n6;
        locals.var_ucr_i_dn7 = assign4780_e6345_d_n7;
        locals.var_ucr_i_dn8 = assign4780_e6345_d_n8;
        locals.var_ucr_i_dn9 = assign4780_e6345_d_n9;
        locals.var_ucr_i_dn10 = assign4780_e6345_d_n10;
        locals.var_ucr_i_dn11 = assign4780_e6345_d_n11;
        locals.var_ucr_i_rv = 0.0;

        let assign4790_e6348: f64 = (locals.var_inv_l).powf(p.p202);
        let assign4790_e6351: f64 = (locals.var_inv_llong).powf(p.p202);
        let assign4790_e6352: f64 = (assign4790_e6348 - assign4790_e6351);
        let assign4790_e6354: f64 = (assign4790_e6352).max(0.0);
        locals.var_t0 = assign4790_e6354;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4800_e6357: f64 = (locals.var_eta0_i * locals.var_t0);
        locals.var_eta0_i = assign4800_e6357;
        locals.var_eta0_i_dn3 = ((locals.var_eta0_i_dn3 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn3));
        locals.var_eta0_i_dn4 = ((locals.var_eta0_i_dn4 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn4));
        locals.var_eta0_i_dn5 = ((locals.var_eta0_i_dn5 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn5));
        locals.var_eta0_i_dn6 = ((locals.var_eta0_i_dn6 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn6));
        locals.var_eta0_i_dn7 = ((locals.var_eta0_i_dn7 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn7));
        locals.var_eta0_i_dn8 = ((locals.var_eta0_i_dn8 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn8));
        locals.var_eta0_i_dn9 = ((locals.var_eta0_i_dn9 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn9));
        locals.var_eta0_i_dn10 = ((locals.var_eta0_i_dn10 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn10));
        locals.var_eta0_i_dn11 = ((locals.var_eta0_i_dn11 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn11));
        locals.var_eta0_i_rv = 0.0;

        let assign4810_e6360: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4810_e6360;
        locals.var_guard31_rv = 0.0;

        let (assign4820_e6366, assign4820_e6366_d_n3, assign4820_e6366_d_n4, assign4820_e6366_d_n5, assign4820_e6366_d_n6, assign4820_e6366_d_n7, assign4820_e6366_d_n8, assign4820_e6366_d_n9, assign4820_e6366_d_n10, assign4820_e6366_d_n11,) = {
    if (locals.var_guard31 != 0.0) {
        let assign4820_e6364: f64 = (locals.var_eta0r_i * locals.var_t0);
        (assign4820_e6364, ((locals.var_eta0r_i_dn3 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn3)), ((locals.var_eta0r_i_dn4 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn4)), ((locals.var_eta0r_i_dn5 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn5)), ((locals.var_eta0r_i_dn6 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn6)), ((locals.var_eta0r_i_dn7 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn7)), ((locals.var_eta0r_i_dn8 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn8)), ((locals.var_eta0r_i_dn9 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn9)), ((locals.var_eta0r_i_dn10 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn10)), ((locals.var_eta0r_i_dn11 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11,)
    }
};
        locals.var_eta0r_i = assign4820_e6366;
        locals.var_eta0r_i_dn3 = assign4820_e6366_d_n3;
        locals.var_eta0r_i_dn4 = assign4820_e6366_d_n4;
        locals.var_eta0r_i_dn5 = assign4820_e6366_d_n5;
        locals.var_eta0r_i_dn6 = assign4820_e6366_d_n6;
        locals.var_eta0r_i_dn7 = assign4820_e6366_d_n7;
        locals.var_eta0r_i_dn8 = assign4820_e6366_d_n8;
        locals.var_eta0r_i_dn9 = assign4820_e6366_d_n9;
        locals.var_eta0r_i_dn10 = assign4820_e6366_d_n10;
        locals.var_eta0r_i_dn11 = assign4820_e6366_d_n11;
        locals.var_eta0r_i_rv = 0.0;

        let assign4830_e6370: f64 = (locals.var_inv_l).powf(p.p204);
        let assign4830_e6373: f64 = (locals.var_inv_llong).powf(p.p204);
        let assign4830_e6374: f64 = (assign4830_e6370 - assign4830_e6373);
        let assign4830_e6376: f64 = (assign4830_e6374).max(0.0);
        let assign4830_e6377: f64 = (locals.var_etab_i * assign4830_e6376);
        locals.var_etab_i = assign4830_e6377;
        locals.var_etab_i_rv = 0.0;

        let assign4840_e6382: f64 = (locals.var_inv_l).powf(p.p532);
        let assign4840_e6385: f64 = (locals.var_inv_llong).powf(p.p532);
        let assign4840_e6386: f64 = (assign4840_e6382 - assign4840_e6385);
        let assign4840_e6388: f64 = (assign4840_e6386).max(0.0);
        let assign4840_e6389: f64 = (p.p531 * assign4840_e6388);
        let assign4840_e6390: f64 = (1.0 + assign4840_e6389);
        locals.var_t0 = assign4840_e6390;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4850_e6393: f64 = (locals.var_pdiblc_i * locals.var_t0);
        locals.var_pdiblc_i = assign4850_e6393;
        locals.var_pdiblc_i_dn3 = ((locals.var_pdiblc_i_dn3 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn3));
        locals.var_pdiblc_i_dn4 = ((locals.var_pdiblc_i_dn4 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn4));
        locals.var_pdiblc_i_dn5 = ((locals.var_pdiblc_i_dn5 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn5));
        locals.var_pdiblc_i_dn6 = ((locals.var_pdiblc_i_dn6 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn6));
        locals.var_pdiblc_i_dn7 = ((locals.var_pdiblc_i_dn7 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn7));
        locals.var_pdiblc_i_dn8 = ((locals.var_pdiblc_i_dn8 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn8));
        locals.var_pdiblc_i_dn9 = ((locals.var_pdiblc_i_dn9 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn9));
        locals.var_pdiblc_i_dn10 = ((locals.var_pdiblc_i_dn10 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn10));
        locals.var_pdiblc_i_dn11 = ((locals.var_pdiblc_i_dn11 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn11));
        locals.var_pdiblc_i_rv = 0.0;

        let assign4860_e6396: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4860_e6396;
        locals.var_guard32_rv = 0.0;

        let (assign4870_e6402, assign4870_e6402_d_n3, assign4870_e6402_d_n4, assign4870_e6402_d_n5, assign4870_e6402_d_n6, assign4870_e6402_d_n7, assign4870_e6402_d_n8, assign4870_e6402_d_n9, assign4870_e6402_d_n10, assign4870_e6402_d_n11,) = {
    if (locals.var_guard32 != 0.0) {
        let assign4870_e6400: f64 = (locals.var_pdiblcr_i * locals.var_t0);
        (assign4870_e6400, ((locals.var_pdiblcr_i_dn3 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn3)), ((locals.var_pdiblcr_i_dn4 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn4)), ((locals.var_pdiblcr_i_dn5 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn5)), ((locals.var_pdiblcr_i_dn6 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn6)), ((locals.var_pdiblcr_i_dn7 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn7)), ((locals.var_pdiblcr_i_dn8 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn8)), ((locals.var_pdiblcr_i_dn9 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn9)), ((locals.var_pdiblcr_i_dn10 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn10)), ((locals.var_pdiblcr_i_dn11 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11,)
    }
};
        locals.var_pdiblcr_i = assign4870_e6402;
        locals.var_pdiblcr_i_dn3 = assign4870_e6402_d_n3;
        locals.var_pdiblcr_i_dn4 = assign4870_e6402_d_n4;
        locals.var_pdiblcr_i_dn5 = assign4870_e6402_d_n5;
        locals.var_pdiblcr_i_dn6 = assign4870_e6402_d_n6;
        locals.var_pdiblcr_i_dn7 = assign4870_e6402_d_n7;
        locals.var_pdiblcr_i_dn8 = assign4870_e6402_d_n8;
        locals.var_pdiblcr_i_dn9 = assign4870_e6402_d_n9;
        locals.var_pdiblcr_i_dn10 = assign4870_e6402_d_n10;
        locals.var_pdiblcr_i_dn11 = assign4870_e6402_d_n11;
        locals.var_pdiblcr_i_rv = 0.0;

        let assign4880_e6408: f64 = (locals.var_inv_l).powf(p.p314);
        let assign4880_e6411: f64 = (locals.var_inv_llong).powf(p.p314);
        let assign4880_e6412: f64 = (assign4880_e6408 - assign4880_e6411);
        let assign4880_e6414: f64 = (assign4880_e6412).max(0.0);
        let assign4880_e6415: f64 = (p.p313 * assign4880_e6414);
        let assign4880_e6416: f64 = (1.0 + assign4880_e6415);
        let assign4880_e6417: f64 = (locals.var_delta_i * assign4880_e6416);
        locals.var_t0 = assign4880_e6417;
        locals.var_t0_dn3 = (locals.var_delta_i_dn3 * assign4880_e6416);
        locals.var_t0_dn4 = (locals.var_delta_i_dn4 * assign4880_e6416);
        locals.var_t0_dn5 = (locals.var_delta_i_dn5 * assign4880_e6416);
        locals.var_t0_dn6 = (locals.var_delta_i_dn6 * assign4880_e6416);
        locals.var_t0_dn7 = (locals.var_delta_i_dn7 * assign4880_e6416);
        locals.var_t0_dn8 = (locals.var_delta_i_dn8 * assign4880_e6416);
        locals.var_t0_dn9 = (locals.var_delta_i_dn9 * assign4880_e6416);
        locals.var_t0_dn10 = (locals.var_delta_i_dn10 * assign4880_e6416);
        locals.var_t0_dn11 = (locals.var_delta_i_dn11 * assign4880_e6416);
        locals.var_t0_rv = 0.0;

        let assign4890_e6420: f64 = (locals.var_t0).min(0.5);
        locals.var_delta_i = assign4890_e6420;
        locals.var_delta_i_dn3 = if locals.var_t0 <= 0.5 { locals.var_t0_dn3 } else { 0.0 };
        locals.var_delta_i_dn4 = if locals.var_t0 <= 0.5 { locals.var_t0_dn4 } else { 0.0 };
        locals.var_delta_i_dn5 = if locals.var_t0 <= 0.5 { locals.var_t0_dn5 } else { 0.0 };
        locals.var_delta_i_dn6 = if locals.var_t0 <= 0.5 { locals.var_t0_dn6 } else { 0.0 };
        locals.var_delta_i_dn7 = if locals.var_t0 <= 0.5 { locals.var_t0_dn7 } else { 0.0 };
        locals.var_delta_i_dn8 = if locals.var_t0 <= 0.5 { locals.var_t0_dn8 } else { 0.0 };
        locals.var_delta_i_dn9 = if locals.var_t0 <= 0.5 { locals.var_t0_dn9 } else { 0.0 };
        locals.var_delta_i_dn10 = if locals.var_t0 <= 0.5 { locals.var_t0_dn10 } else { 0.0 };
        locals.var_delta_i_dn11 = if locals.var_t0 <= 0.5 { locals.var_t0_dn11 } else { 0.0 };
        locals.var_delta_i_rv = 0.0;

        let assign4900_e6426: f64 = (locals.var_inv_l).powf(p.p550);
        let assign4900_e6429: f64 = (locals.var_inv_llong).powf(p.p550);
        let assign4900_e6430: f64 = (assign4900_e6426 - assign4900_e6429);
        let assign4900_e6432: f64 = (assign4900_e6430).max(0.0);
        let assign4900_e6433: f64 = (p.p549 * assign4900_e6432);
        let assign4900_e6434: f64 = (1.0 + assign4900_e6433);
        let assign4900_e6435: f64 = (locals.var_fprout_i * assign4900_e6434);
        locals.var_fprout_i = assign4900_e6435;
        locals.var_fprout_i_rv = 0.0;

        let assign4910_e6440: f64 = (locals.var_inv_l).powf(p.p406);
        let assign4910_e6443: f64 = (locals.var_inv_llong).powf(p.p406);
        let assign4910_e6444: f64 = (assign4910_e6440 - assign4910_e6443);
        let assign4910_e6446: f64 = (assign4910_e6444).max(0.0);
        let assign4910_e6447: f64 = (p.p405 * assign4910_e6446);
        let assign4910_e6448: f64 = (1.0 + assign4910_e6447);
        locals.var_t0 = assign4910_e6448;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4920_e6451: f64 = (locals.var_pclm_i * locals.var_t0);
        locals.var_pclm_i = assign4920_e6451;
        locals.var_pclm_i_dn3 = ((locals.var_pclm_i_dn3 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn3));
        locals.var_pclm_i_dn4 = ((locals.var_pclm_i_dn4 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn4));
        locals.var_pclm_i_dn5 = ((locals.var_pclm_i_dn5 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn5));
        locals.var_pclm_i_dn6 = ((locals.var_pclm_i_dn6 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn6));
        locals.var_pclm_i_dn7 = ((locals.var_pclm_i_dn7 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn7));
        locals.var_pclm_i_dn8 = ((locals.var_pclm_i_dn8 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn8));
        locals.var_pclm_i_dn9 = ((locals.var_pclm_i_dn9 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn9));
        locals.var_pclm_i_dn10 = ((locals.var_pclm_i_dn10 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn10));
        locals.var_pclm_i_dn11 = ((locals.var_pclm_i_dn11 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn11));
        locals.var_pclm_i_rv = 0.0;

        let assign4930_e6454: f64 = (locals.var_pclm_i).max(0.0);
        locals.var_pclm_i = assign4930_e6454;
        locals.var_pclm_i_dn3 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn3 } else { 0.0 };
        locals.var_pclm_i_dn4 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn4 } else { 0.0 };
        locals.var_pclm_i_dn5 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn5 } else { 0.0 };
        locals.var_pclm_i_dn6 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn6 } else { 0.0 };
        locals.var_pclm_i_dn7 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn7 } else { 0.0 };
        locals.var_pclm_i_dn8 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn8 } else { 0.0 };
        locals.var_pclm_i_dn9 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn9 } else { 0.0 };
        locals.var_pclm_i_dn10 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn10 } else { 0.0 };
        locals.var_pclm_i_dn11 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn11 } else { 0.0 };
        locals.var_pclm_i_rv = 0.0;

        let assign4940_e6457: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4940_e6457;
        locals.var_guard33_rv = 0.0;

        let (assign4950_e6463, assign4950_e6463_d_n3, assign4950_e6463_d_n4, assign4950_e6463_d_n5, assign4950_e6463_d_n6, assign4950_e6463_d_n7, assign4950_e6463_d_n8, assign4950_e6463_d_n9, assign4950_e6463_d_n10, assign4950_e6463_d_n11,) = {
    if (locals.var_guard33 != 0.0) {
        let assign4950_e6461: f64 = (locals.var_pclmr_i * locals.var_t0);
        (assign4950_e6461, ((locals.var_pclmr_i_dn3 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn3)), ((locals.var_pclmr_i_dn4 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn4)), ((locals.var_pclmr_i_dn5 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn5)), ((locals.var_pclmr_i_dn6 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn6)), ((locals.var_pclmr_i_dn7 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn7)), ((locals.var_pclmr_i_dn8 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn8)), ((locals.var_pclmr_i_dn9 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn9)), ((locals.var_pclmr_i_dn10 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn10)), ((locals.var_pclmr_i_dn11 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11,)
    }
};
        locals.var_pclmr_i = assign4950_e6463;
        locals.var_pclmr_i_dn3 = assign4950_e6463_d_n3;
        locals.var_pclmr_i_dn4 = assign4950_e6463_d_n4;
        locals.var_pclmr_i_dn5 = assign4950_e6463_d_n5;
        locals.var_pclmr_i_dn6 = assign4950_e6463_d_n6;
        locals.var_pclmr_i_dn7 = assign4950_e6463_d_n7;
        locals.var_pclmr_i_dn8 = assign4950_e6463_d_n8;
        locals.var_pclmr_i_dn9 = assign4950_e6463_d_n9;
        locals.var_pclmr_i_dn10 = assign4950_e6463_d_n10;
        locals.var_pclmr_i_dn11 = assign4950_e6463_d_n11;
        locals.var_pclmr_i_rv = 0.0;

        let (assign4960_e6469, assign4960_e6469_d_n3, assign4960_e6469_d_n4, assign4960_e6469_d_n5, assign4960_e6469_d_n6, assign4960_e6469_d_n7, assign4960_e6469_d_n8, assign4960_e6469_d_n9, assign4960_e6469_d_n10, assign4960_e6469_d_n11,) = {
    if (locals.var_guard33 != 0.0) {
        let assign4960_e6467: f64 = (locals.var_pclmr_i).max(0.0);
        (assign4960_e6467, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn3 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn4 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn5 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn6 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn7 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn8 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn9 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn10 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn11 } else { 0.0 },)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11,)
    }
};
        locals.var_pclmr_i = assign4960_e6469;
        locals.var_pclmr_i_dn3 = assign4960_e6469_d_n3;
        locals.var_pclmr_i_dn4 = assign4960_e6469_d_n4;
        locals.var_pclmr_i_dn5 = assign4960_e6469_d_n5;
        locals.var_pclmr_i_dn6 = assign4960_e6469_d_n6;
        locals.var_pclmr_i_dn7 = assign4960_e6469_d_n7;
        locals.var_pclmr_i_dn8 = assign4960_e6469_d_n8;
        locals.var_pclmr_i_dn9 = assign4960_e6469_d_n9;
        locals.var_pclmr_i_dn10 = assign4960_e6469_d_n10;
        locals.var_pclmr_i_dn11 = assign4960_e6469_d_n11;
        locals.var_pclmr_i_rv = 0.0;

        let assign4970_e6473: f64 = (locals.var_inv_l).powf(p.p300);
        let assign4970_e6476: f64 = (locals.var_inv_llong).powf(p.p300);
        let assign4970_e6477: f64 = (assign4970_e6473 - assign4970_e6476);
        let assign4970_e6479: f64 = (assign4970_e6477).max(0.0);
        let assign4970_e6480: f64 = (p.p299 * assign4970_e6479);
        locals.var_t0 = assign4970_e6480;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign4980_e6484: f64 = (locals.var_inv_w).powf(p.p302);
        let assign4980_e6487: f64 = (locals.var_inv_wwide).powf(p.p302);
        let assign4980_e6488: f64 = (assign4980_e6484 - assign4980_e6487);
        let assign4980_e6490: f64 = (assign4980_e6488).max(0.0);
        let assign4980_e6491: f64 = (p.p301 * assign4980_e6490);
        let assign4980_e6495: f64 = (locals.var_inv_wl).powf(p.p304);
        let assign4980_e6496: f64 = (p.p303 * assign4980_e6495);
        let assign4980_e6497: f64 = (assign4980_e6491 + assign4980_e6496);
        locals.var_t1 = assign4980_e6497;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4990_e6501: f64 = (1.0 + locals.var_t0);
        let assign4990_e6503: f64 = (assign4990_e6501 + locals.var_t1);
        let assign4990_e6504: f64 = (locals.var_vsat_i * assign4990_e6503);
        locals.var_vsat_i = assign4990_e6504;
        locals.var_vsat_i_dn3 = ((locals.var_vsat_i_dn3 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsat_i_dn4 = ((locals.var_vsat_i_dn4 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsat_i_dn5 = ((locals.var_vsat_i_dn5 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsat_i_dn6 = ((locals.var_vsat_i_dn6 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsat_i_dn7 = ((locals.var_vsat_i_dn7 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsat_i_dn8 = ((locals.var_vsat_i_dn8 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsat_i_dn9 = ((locals.var_vsat_i_dn9 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsat_i_dn10 = ((locals.var_vsat_i_dn10 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsat_i_dn11 = ((locals.var_vsat_i_dn11 * assign4990_e6503) + (locals.var_vsat_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsat_i_rv = 0.0;

        let assign5000_e6507: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign5000_e6507;
        locals.var_guard34_rv = 0.0;

        let (assign5010_e6517, assign5010_e6517_d_n3, assign5010_e6517_d_n4, assign5010_e6517_d_n5, assign5010_e6517_d_n6, assign5010_e6517_d_n7, assign5010_e6517_d_n8, assign5010_e6517_d_n9, assign5010_e6517_d_n10, assign5010_e6517_d_n11,) = {
    if (locals.var_guard34 != 0.0) {
        let assign5010_e6512: f64 = (1.0 + locals.var_t0);
        let assign5010_e6514: f64 = (assign5010_e6512 + locals.var_t1);
        let assign5010_e6515: f64 = (locals.var_vsatr_i * assign5010_e6514);
        (assign5010_e6515, ((locals.var_vsatr_i_dn3 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatr_i_dn4 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatr_i_dn5 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatr_i_dn6 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatr_i_dn7 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatr_i_dn8 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatr_i_dn9 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatr_i_dn10 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatr_i_dn11 * assign5010_e6514) + (locals.var_vsatr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))),)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11,)
    }
};
        locals.var_vsatr_i = assign5010_e6517;
        locals.var_vsatr_i_dn3 = assign5010_e6517_d_n3;
        locals.var_vsatr_i_dn4 = assign5010_e6517_d_n4;
        locals.var_vsatr_i_dn5 = assign5010_e6517_d_n5;
        locals.var_vsatr_i_dn6 = assign5010_e6517_d_n6;
        locals.var_vsatr_i_dn7 = assign5010_e6517_d_n7;
        locals.var_vsatr_i_dn8 = assign5010_e6517_d_n8;
        locals.var_vsatr_i_dn9 = assign5010_e6517_d_n9;
        locals.var_vsatr_i_dn10 = assign5010_e6517_d_n10;
        locals.var_vsatr_i_dn11 = assign5010_e6517_d_n11;
        locals.var_vsatr_i_rv = 0.0;

        let assign5020_e6523: f64 = (locals.var_inv_l).powf(p.p488);
        let assign5020_e6526: f64 = (locals.var_inv_llong).powf(p.p488);
        let assign5020_e6527: f64 = (assign5020_e6523 - assign5020_e6526);
        let assign5020_e6529: f64 = (assign5020_e6527).max(0.0);
        let assign5020_e6530: f64 = (p.p487 * assign5020_e6529);
        let assign5020_e6531: f64 = (1.0 + assign5020_e6530);
        let assign5020_e6532: f64 = (locals.var_psat_i * assign5020_e6531);
        let assign5020_e6534: f64 = (assign5020_e6532).max(0.25);
        locals.var_psat_i = assign5020_e6534;
        locals.var_psat_i_rv = 0.0;

        let assign5030_e6537: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign5030_e6537;
        locals.var_guard35_rv = 0.0;

        let (assign5040_e6557,) = {
    if (locals.var_guard35 != 0.0) {
        let assign5040_e6544: f64 = (locals.var_inv_l).powf(p.p488);
        let assign5040_e6547: f64 = (locals.var_inv_llong).powf(p.p488);
        let assign5040_e6548: f64 = (assign5040_e6544 - assign5040_e6547);
        let assign5040_e6550: f64 = (assign5040_e6548).max(0.0);
        let assign5040_e6551: f64 = (p.p487 * assign5040_e6550);
        let assign5040_e6552: f64 = (1.0 + assign5040_e6551);
        let assign5040_e6553: f64 = (locals.var_psatr_i * assign5040_e6552);
        let assign5040_e6555: f64 = (assign5040_e6553).max(0.25);
        (assign5040_e6555,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign5040_e6557;
        locals.var_psatr_i_rv = 0.0;

        let assign5050_e6562: f64 = (locals.var_inv_l).powf(p.p505);
        let assign5050_e6565: f64 = (locals.var_inv_llong).powf(p.p505);
        let assign5050_e6566: f64 = (assign5050_e6562 - assign5050_e6565);
        let assign5050_e6568: f64 = (assign5050_e6566).max(0.0);
        let assign5050_e6569: f64 = (p.p502 * assign5050_e6568);
        let assign5050_e6570: f64 = (1.0 + assign5050_e6569);
        locals.var_t0 = assign5050_e6570;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5060_e6573: f64 = (locals.var_ptwg_i * locals.var_t0);
        locals.var_ptwg_i = assign5060_e6573;
        locals.var_ptwg_i_dn3 = ((locals.var_ptwg_i_dn3 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn3));
        locals.var_ptwg_i_dn4 = ((locals.var_ptwg_i_dn4 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn4));
        locals.var_ptwg_i_dn5 = ((locals.var_ptwg_i_dn5 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn5));
        locals.var_ptwg_i_dn6 = ((locals.var_ptwg_i_dn6 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn6));
        locals.var_ptwg_i_dn7 = ((locals.var_ptwg_i_dn7 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn7));
        locals.var_ptwg_i_dn8 = ((locals.var_ptwg_i_dn8 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn8));
        locals.var_ptwg_i_dn9 = ((locals.var_ptwg_i_dn9 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn9));
        locals.var_ptwg_i_dn10 = ((locals.var_ptwg_i_dn10 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn10));
        locals.var_ptwg_i_dn11 = ((locals.var_ptwg_i_dn11 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn11));
        locals.var_ptwg_i_rv = 0.0;

        let assign5070_e6576: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign5070_e6576;
        locals.var_guard36_rv = 0.0;

        let (assign5080_e6582, assign5080_e6582_d_n3, assign5080_e6582_d_n4, assign5080_e6582_d_n5, assign5080_e6582_d_n6, assign5080_e6582_d_n7, assign5080_e6582_d_n8, assign5080_e6582_d_n9, assign5080_e6582_d_n10, assign5080_e6582_d_n11,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5080_e6580: f64 = (locals.var_ptwgr_i * locals.var_t0);
        (assign5080_e6580, ((locals.var_ptwgr_i_dn3 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn3)), ((locals.var_ptwgr_i_dn4 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn4)), ((locals.var_ptwgr_i_dn5 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn5)), ((locals.var_ptwgr_i_dn6 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn6)), ((locals.var_ptwgr_i_dn7 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn7)), ((locals.var_ptwgr_i_dn8 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn8)), ((locals.var_ptwgr_i_dn9 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn9)), ((locals.var_ptwgr_i_dn10 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn10)), ((locals.var_ptwgr_i_dn11 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn11)),)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11,)
    }
};
        locals.var_ptwgr_i = assign5080_e6582;
        locals.var_ptwgr_i_dn3 = assign5080_e6582_d_n3;
        locals.var_ptwgr_i_dn4 = assign5080_e6582_d_n4;
        locals.var_ptwgr_i_dn5 = assign5080_e6582_d_n5;
        locals.var_ptwgr_i_dn6 = assign5080_e6582_d_n6;
        locals.var_ptwgr_i_dn7 = assign5080_e6582_d_n7;
        locals.var_ptwgr_i_dn8 = assign5080_e6582_d_n8;
        locals.var_ptwgr_i_dn9 = assign5080_e6582_d_n9;
        locals.var_ptwgr_i_dn10 = assign5080_e6582_d_n10;
        locals.var_ptwgr_i_dn11 = assign5080_e6582_d_n11;
        locals.var_ptwgr_i_rv = 0.0;

        let assign5090_e6588: f64 = (locals.var_inv_l).powf(p.p603);
        let assign5090_e6591: f64 = (locals.var_inv_llong).powf(p.p603);
        let assign5090_e6592: f64 = (assign5090_e6588 - assign5090_e6591);
        let assign5090_e6594: f64 = (assign5090_e6592).max(0.0);
        let assign5090_e6595: f64 = (p.p602 * assign5090_e6594);
        let assign5090_e6596: f64 = (1.0 + assign5090_e6595);
        let assign5090_e6597: f64 = (locals.var_alpha0_i * assign5090_e6596);
        locals.var_alpha0_i = assign5090_e6597;
        locals.var_alpha0_i_rv = 0.0;

        let assign5100_e6602: f64 = (p.p800 * locals.var_inv_l);
        let assign5100_e6603: f64 = (1.0 + assign5100_e6602);
        let assign5100_e6606: f64 = (p.p801 * locals.var_inv_w);
        let assign5100_e6607: f64 = (assign5100_e6603 + assign5100_e6606);
        let assign5100_e6608: f64 = (locals.var_agidl_i * assign5100_e6607);
        locals.var_agidl_i = assign5100_e6608;
        locals.var_agidl_i_rv = 0.0;

        let assign5110_e6613: f64 = (p.p822 * locals.var_inv_l);
        let assign5110_e6614: f64 = (1.0 + assign5110_e6613);
        let assign5110_e6617: f64 = (p.p823 * locals.var_inv_w);
        let assign5110_e6618: f64 = (assign5110_e6614 + assign5110_e6617);
        let assign5110_e6619: f64 = (locals.var_agisl_i * assign5110_e6618);
        locals.var_agisl_i = assign5110_e6619;
        locals.var_agisl_i_rv = 0.0;

        let assign5120_e6624: f64 = (p.p724 * locals.var_inv_l);
        let assign5120_e6625: f64 = (1.0 + assign5120_e6624);
        let assign5120_e6628: f64 = (p.p725 * locals.var_inv_w);
        let assign5120_e6629: f64 = (assign5120_e6625 + assign5120_e6628);
        let assign5120_e6630: f64 = (locals.var_aigc_i * assign5120_e6629);
        locals.var_aigc_i = assign5120_e6630;
        locals.var_aigc_i_dn4 = (locals.var_aigc_i_dn4 * assign5120_e6629);
        locals.var_aigc_i_dn5 = (locals.var_aigc_i_dn5 * assign5120_e6629);
        locals.var_aigc_i_rv = 0.0;

        let assign5130_e6635: f64 = (p.p727 * locals.var_inv_l);
        let assign5130_e6636: f64 = (1.0 + assign5130_e6635);
        let assign5130_e6639: f64 = (p.p728 * locals.var_inv_w);
        let assign5130_e6640: f64 = (assign5130_e6636 + assign5130_e6639);
        let assign5130_e6641: f64 = (locals.var_aigs_i * assign5130_e6640);
        locals.var_aigs_i = assign5130_e6641;
        locals.var_aigs_i_dn4 = (locals.var_aigs_i_dn4 * assign5130_e6640);
        locals.var_aigs_i_dn5 = (locals.var_aigs_i_dn5 * assign5130_e6640);
        locals.var_aigs_i_rv = 0.0;

        let assign5140_e6646: f64 = (p.p729 * locals.var_inv_l);
        let assign5140_e6647: f64 = (1.0 + assign5140_e6646);
        let assign5140_e6650: f64 = (p.p730 * locals.var_inv_w);
        let assign5140_e6651: f64 = (assign5140_e6647 + assign5140_e6650);
        let assign5140_e6652: f64 = (locals.var_aigd_i * assign5140_e6651);
        locals.var_aigd_i = assign5140_e6652;
        locals.var_aigd_i_dn4 = (locals.var_aigd_i_dn4 * assign5140_e6651);
        locals.var_aigd_i_dn5 = (locals.var_aigd_i_dn5 * assign5140_e6651);
        locals.var_aigd_i_rv = 0.0;

        let assign5150_e6657: f64 = (p.p731 * locals.var_inv_l);
        let assign5150_e6658: f64 = (1.0 + assign5150_e6657);
        let assign5150_e6659: f64 = (p.p723 * assign5150_e6658);
        locals.var_pigcd_i = assign5150_e6659;
        locals.var_pigcd_i_rv = 0.0;

        let assign5160_e6663: f64 = (locals.var_inv_lact).powf(p.p93);
        let assign5160_e6666: f64 = (locals.var_inv_llong).powf(p.p93);
        let assign5160_e6667: f64 = (assign5160_e6663 - assign5160_e6666);
        let assign5160_e6669: f64 = (assign5160_e6667).max(0.0);
        let assign5160_e6670: f64 = (p.p92 * assign5160_e6669);
        let assign5160_e6674: f64 = (locals.var_inv_lact).powf(p.p95);
        let assign5160_e6677: f64 = (locals.var_inv_llong).powf(p.p95);
        let assign5160_e6678: f64 = (assign5160_e6674 - assign5160_e6677);
        let assign5160_e6680: f64 = (assign5160_e6678).max(0.0);
        let assign5160_e6681: f64 = (p.p94 * assign5160_e6680);
        let assign5160_e6682: f64 = (assign5160_e6670 + assign5160_e6681);
        locals.var_t0 = assign5160_e6682;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5170_e6686: f64 = (locals.var_inv_wact).powf(p.p97);
        let assign5170_e6689: f64 = (locals.var_inv_wwide).powf(p.p97);
        let assign5170_e6690: f64 = (assign5170_e6686 - assign5170_e6689);
        let assign5170_e6692: f64 = (assign5170_e6690).max(0.0);
        let assign5170_e6693: f64 = (p.p96 * assign5170_e6692);
        let assign5170_e6697: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign5170_e6699: f64 = (assign5170_e6697).powf(p.p99);
        let assign5170_e6700: f64 = (p.p98 * assign5170_e6699);
        let assign5170_e6701: f64 = (assign5170_e6693 + assign5170_e6700);
        locals.var_t1 = assign5170_e6701;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5180_e6705: f64 = (1.0 + locals.var_t0);
        let assign5180_e6707: f64 = (assign5180_e6705 + locals.var_t1);
        let assign5180_e6708: f64 = (locals.var_ndepcv_i * assign5180_e6707);
        locals.var_ndepcv_i = assign5180_e6708;
        locals.var_ndepcv_i_dn3 = ((locals.var_ndepcv_i_dn3 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndepcv_i_dn4 = ((locals.var_ndepcv_i_dn4 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndepcv_i_dn5 = ((locals.var_ndepcv_i_dn5 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndepcv_i_dn6 = ((locals.var_ndepcv_i_dn6 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndepcv_i_dn7 = ((locals.var_ndepcv_i_dn7 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndepcv_i_dn8 = ((locals.var_ndepcv_i_dn8 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndepcv_i_dn9 = ((locals.var_ndepcv_i_dn9 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndepcv_i_dn10 = ((locals.var_ndepcv_i_dn10 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndepcv_i_dn11 = ((locals.var_ndepcv_i_dn11 * assign5180_e6707) + (locals.var_ndepcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndepcv_i_rv = 0.0;

        let assign5190_e6711: f64 = if p.p29 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5190_e6711;
        locals.var_guard37_rv = 0.0;

        let (assign5200_e6715, assign5200_e6715_d_n3, assign5200_e6715_d_n4, assign5200_e6715_d_n5, assign5200_e6715_d_n6, assign5200_e6715_d_n7, assign5200_e6715_d_n8, assign5200_e6715_d_n9, assign5200_e6715_d_n10, assign5200_e6715_d_n11,) = {
    if (locals.var_guard37 != 0.0) {
        (locals.var_ndep_i, locals.var_ndep_i_dn3, locals.var_ndep_i_dn4, locals.var_ndep_i_dn5, locals.var_ndep_i_dn6, locals.var_ndep_i_dn7, locals.var_ndep_i_dn8, locals.var_ndep_i_dn9, locals.var_ndep_i_dn10, locals.var_ndep_i_dn11,)
    } else {
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11,)
    }
};
        locals.var_ndepcv_i = assign5200_e6715;
        locals.var_ndepcv_i_dn3 = assign5200_e6715_d_n3;
        locals.var_ndepcv_i_dn4 = assign5200_e6715_d_n4;
        locals.var_ndepcv_i_dn5 = assign5200_e6715_d_n5;
        locals.var_ndepcv_i_dn6 = assign5200_e6715_d_n6;
        locals.var_ndepcv_i_dn7 = assign5200_e6715_d_n7;
        locals.var_ndepcv_i_dn8 = assign5200_e6715_d_n8;
        locals.var_ndepcv_i_dn9 = assign5200_e6715_d_n9;
        locals.var_ndepcv_i_dn10 = assign5200_e6715_d_n10;
        locals.var_ndepcv_i_dn11 = assign5200_e6715_d_n11;
        locals.var_ndepcv_i_rv = 0.0;

        let (assign5210_e6720, assign5210_e6720_d_n3, assign5210_e6720_d_n4, assign5210_e6720_d_n5, assign5210_e6720_d_n6, assign5210_e6720_d_n7, assign5210_e6720_d_n8, assign5210_e6720_d_n9, assign5210_e6720_d_n10, assign5210_e6720_d_n11,) = {
    if (locals.var_guard37 == 0.0) {
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11,)
    } else {
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11,)
    }
};
        locals.var_ndepcv_i = assign5210_e6720;
        locals.var_ndepcv_i_dn3 = assign5210_e6720_d_n3;
        locals.var_ndepcv_i_dn4 = assign5210_e6720_d_n4;
        locals.var_ndepcv_i_dn5 = assign5210_e6720_d_n5;
        locals.var_ndepcv_i_dn6 = assign5210_e6720_d_n6;
        locals.var_ndepcv_i_dn7 = assign5210_e6720_d_n7;
        locals.var_ndepcv_i_dn8 = assign5210_e6720_d_n8;
        locals.var_ndepcv_i_dn9 = assign5210_e6720_d_n9;
        locals.var_ndepcv_i_dn10 = assign5210_e6720_d_n10;
        locals.var_ndepcv_i_dn11 = assign5210_e6720_d_n11;
        locals.var_ndepcv_i_rv = 0.0;

        let assign5220_e6724: f64 = (locals.var_inv_l).powf(p.p124);
        let assign5220_e6727: f64 = (locals.var_inv_llong).powf(p.p124);
        let assign5220_e6728: f64 = (assign5220_e6724 - assign5220_e6727);
        let assign5220_e6730: f64 = (assign5220_e6728).max(0.0);
        let assign5220_e6731: f64 = (p.p123 * assign5220_e6730);
        locals.var_t0 = assign5220_e6731;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5230_e6735: f64 = (locals.var_inv_w).powf(p.p126);
        let assign5230_e6738: f64 = (locals.var_inv_wwide).powf(p.p126);
        let assign5230_e6739: f64 = (assign5230_e6735 - assign5230_e6738);
        let assign5230_e6741: f64 = (assign5230_e6739).max(0.0);
        let assign5230_e6742: f64 = (p.p125 * assign5230_e6741);
        let assign5230_e6746: f64 = (locals.var_inv_wl).powf(p.p128);
        let assign5230_e6747: f64 = (p.p127 * assign5230_e6746);
        let assign5230_e6748: f64 = (assign5230_e6742 + assign5230_e6747);
        locals.var_t1 = assign5230_e6748;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5240_e6752: f64 = (1.0 + locals.var_t0);
        let assign5240_e6754: f64 = (assign5240_e6752 + locals.var_t1);
        let assign5240_e6755: f64 = (locals.var_vfb_i * assign5240_e6754);
        locals.var_vfb_i = assign5240_e6755;
        locals.var_vfb_i_dn3 = ((locals.var_vfb_i_dn3 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfb_i_dn4 = ((locals.var_vfb_i_dn4 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfb_i_dn5 = ((locals.var_vfb_i_dn5 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfb_i_dn6 = ((locals.var_vfb_i_dn6 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfb_i_dn7 = ((locals.var_vfb_i_dn7 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfb_i_dn8 = ((locals.var_vfb_i_dn8 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfb_i_dn9 = ((locals.var_vfb_i_dn9 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfb_i_dn10 = ((locals.var_vfb_i_dn10 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfb_i_dn11 = ((locals.var_vfb_i_dn11 * assign5240_e6754) + (locals.var_vfb_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfb_i_rv = 0.0;

        let assign5250_e6759: f64 = (locals.var_inv_lact).powf(p.p134);
        let assign5250_e6762: f64 = (locals.var_inv_llong).powf(p.p134);
        let assign5250_e6763: f64 = (assign5250_e6759 - assign5250_e6762);
        let assign5250_e6765: f64 = (assign5250_e6763).max(0.0);
        let assign5250_e6766: f64 = (p.p133 * assign5250_e6765);
        locals.var_t0 = assign5250_e6766;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5260_e6770: f64 = (locals.var_inv_wact).powf(p.p136);
        let assign5260_e6773: f64 = (locals.var_inv_wwide).powf(p.p136);
        let assign5260_e6774: f64 = (assign5260_e6770 - assign5260_e6773);
        let assign5260_e6776: f64 = (assign5260_e6774).max(0.0);
        let assign5260_e6777: f64 = (p.p135 * assign5260_e6776);
        let assign5260_e6781: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign5260_e6783: f64 = (assign5260_e6781).powf(p.p138);
        let assign5260_e6784: f64 = (p.p137 * assign5260_e6783);
        let assign5260_e6785: f64 = (assign5260_e6777 + assign5260_e6784);
        locals.var_t1 = assign5260_e6785;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5270_e6789: f64 = (1.0 + locals.var_t0);
        let assign5270_e6791: f64 = (assign5270_e6789 + locals.var_t1);
        let assign5270_e6792: f64 = (locals.var_vfbcv_i * assign5270_e6791);
        locals.var_vfbcv_i = assign5270_e6792;
        locals.var_vfbcv_i_dn3 = ((locals.var_vfbcv_i_dn3 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfbcv_i_dn4 = ((locals.var_vfbcv_i_dn4 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfbcv_i_dn5 = ((locals.var_vfbcv_i_dn5 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfbcv_i_dn6 = ((locals.var_vfbcv_i_dn6 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfbcv_i_dn7 = ((locals.var_vfbcv_i_dn7 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfbcv_i_dn8 = ((locals.var_vfbcv_i_dn8 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfbcv_i_dn9 = ((locals.var_vfbcv_i_dn9 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfbcv_i_dn10 = ((locals.var_vfbcv_i_dn10 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfbcv_i_dn11 = ((locals.var_vfbcv_i_dn11 * assign5270_e6791) + (locals.var_vfbcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfbcv_i_rv = 0.0;

        let assign5280_e6796: f64 = (locals.var_inv_lact).powf(p.p320);
        let assign5280_e6799: f64 = (locals.var_inv_llong).powf(p.p320);
        let assign5280_e6800: f64 = (assign5280_e6796 - assign5280_e6799);
        let assign5280_e6802: f64 = (assign5280_e6800).max(0.0);
        let assign5280_e6803: f64 = (p.p319 * assign5280_e6802);
        locals.var_t0 = assign5280_e6803;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5290_e6807: f64 = (locals.var_inv_wact).powf(p.p322);
        let assign5290_e6810: f64 = (locals.var_inv_wwide).powf(p.p322);
        let assign5290_e6811: f64 = (assign5290_e6807 - assign5290_e6810);
        let assign5290_e6813: f64 = (assign5290_e6811).max(0.0);
        let assign5290_e6814: f64 = (p.p321 * assign5290_e6813);
        let assign5290_e6818: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign5290_e6820: f64 = (assign5290_e6818).powf(p.p324);
        let assign5290_e6821: f64 = (p.p323 * assign5290_e6820);
        let assign5290_e6822: f64 = (assign5290_e6814 + assign5290_e6821);
        locals.var_t1 = assign5290_e6822;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5300_e6826: f64 = (1.0 + locals.var_t0);
        let assign5300_e6828: f64 = (assign5300_e6826 + locals.var_t1);
        let assign5300_e6829: f64 = (locals.var_vsatcv_i * assign5300_e6828);
        locals.var_vsatcv_i = assign5300_e6829;
        locals.var_vsatcv_i_dn3 = ((locals.var_vsatcv_i_dn3 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsatcv_i_dn4 = ((locals.var_vsatcv_i_dn4 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsatcv_i_dn5 = ((locals.var_vsatcv_i_dn5 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsatcv_i_dn6 = ((locals.var_vsatcv_i_dn6 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsatcv_i_dn7 = ((locals.var_vsatcv_i_dn7 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsatcv_i_dn8 = ((locals.var_vsatcv_i_dn8 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsatcv_i_dn9 = ((locals.var_vsatcv_i_dn9 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsatcv_i_dn10 = ((locals.var_vsatcv_i_dn10 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsatcv_i_dn11 = ((locals.var_vsatcv_i_dn11 * assign5300_e6828) + (locals.var_vsatcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsatcv_i_rv = 0.0;

        let assign5310_e6835: f64 = (locals.var_inv_lact).powf(p.p417);
        let assign5310_e6838: f64 = (locals.var_inv_llong).powf(p.p417);
        let assign5310_e6839: f64 = (assign5310_e6835 - assign5310_e6838);
        let assign5310_e6841: f64 = (assign5310_e6839).max(0.0);
        let assign5310_e6842: f64 = (p.p416 * assign5310_e6841);
        let assign5310_e6843: f64 = (1.0 + assign5310_e6842);
        let assign5310_e6844: f64 = (locals.var_pclmcv_i * assign5310_e6843);
        locals.var_pclmcv_i = assign5310_e6844;
        locals.var_pclmcv_i_rv = 0.0;

        let assign5320_e6847: f64 = (locals.var_pclmcv_i).max(0.0);
        locals.var_pclmcv_i = assign5320_e6847;
        locals.var_pclmcv_i_rv = 0.0;

        let assign5330_e6851: f64 = (locals.var_inv_l).powf(p.p210);
        let assign5330_e6854: f64 = (locals.var_inv_llong).powf(p.p210);
        let assign5330_e6855: f64 = (assign5330_e6851 - assign5330_e6854);
        let assign5330_e6857: f64 = (assign5330_e6855).max(0.0);
        let assign5330_e6858: f64 = (p.p209 * assign5330_e6857);
        locals.var_t0 = assign5330_e6858;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5340_e6862: f64 = (locals.var_inv_w).powf(p.p212);
        let assign5340_e6865: f64 = (locals.var_inv_wwide).powf(p.p212);
        let assign5340_e6866: f64 = (assign5340_e6862 - assign5340_e6865);
        let assign5340_e6868: f64 = (assign5340_e6866).max(0.0);
        let assign5340_e6869: f64 = (p.p211 * assign5340_e6868);
        let assign5340_e6873: f64 = (locals.var_inv_wl).powf(p.p214);
        let assign5340_e6874: f64 = (p.p213 * assign5340_e6873);
        let assign5340_e6875: f64 = (assign5340_e6869 + assign5340_e6874);
        locals.var_t1 = assign5340_e6875;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5350_e6879: f64 = (1.0 + locals.var_t0);
        let assign5350_e6881: f64 = (assign5350_e6879 + locals.var_t1);
        let assign5350_e6882: f64 = (locals.var_k1_i * assign5350_e6881);
        locals.var_k1_i = assign5350_e6882;
        locals.var_k1_i_dn3 = ((locals.var_k1_i_dn3 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k1_i_dn4 = ((locals.var_k1_i_dn4 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k1_i_dn5 = ((locals.var_k1_i_dn5 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k1_i_dn6 = ((locals.var_k1_i_dn6 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k1_i_dn7 = ((locals.var_k1_i_dn7 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k1_i_dn8 = ((locals.var_k1_i_dn8 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k1_i_dn9 = ((locals.var_k1_i_dn9 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k1_i_dn10 = ((locals.var_k1_i_dn10 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k1_i_dn11 = ((locals.var_k1_i_dn11 * assign5350_e6881) + (locals.var_k1_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k1_i_rv = 0.0;

        let assign5360_e6886: f64 = (locals.var_inv_l).powf(p.p1198);
        let assign5360_e6889: f64 = (locals.var_inv_llong).powf(p.p1198);
        let assign5360_e6890: f64 = (assign5360_e6886 - assign5360_e6889);
        let assign5360_e6892: f64 = (assign5360_e6890).max(0.0);
        let assign5360_e6893: f64 = (p.p1197 * assign5360_e6892);
        locals.var_t0 = assign5360_e6893;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5370_e6897: f64 = (locals.var_inv_w).powf(p.p1200);
        let assign5370_e6900: f64 = (locals.var_inv_wwide).powf(p.p1200);
        let assign5370_e6901: f64 = (assign5370_e6897 - assign5370_e6900);
        let assign5370_e6903: f64 = (assign5370_e6901).max(0.0);
        let assign5370_e6904: f64 = (p.p1199 * assign5370_e6903);
        let assign5370_e6908: f64 = (locals.var_inv_wl).powf(p.p1202);
        let assign5370_e6909: f64 = (p.p1201 * assign5370_e6908);
        let assign5370_e6910: f64 = (assign5370_e6904 + assign5370_e6909);
        locals.var_t1 = assign5370_e6910;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5380_e6914: f64 = (1.0 + locals.var_t0);
        let assign5380_e6916: f64 = (assign5380_e6914 + locals.var_t1);
        let assign5380_e6917: f64 = (locals.var_k1edge_i * assign5380_e6916);
        locals.var_k1edge_i = assign5380_e6917;
        locals.var_k1edge_i_dn3 = ((locals.var_k1edge_i_dn3 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k1edge_i_dn4 = ((locals.var_k1edge_i_dn4 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k1edge_i_dn5 = ((locals.var_k1edge_i_dn5 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k1edge_i_dn6 = ((locals.var_k1edge_i_dn6 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k1edge_i_dn7 = ((locals.var_k1edge_i_dn7 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k1edge_i_dn8 = ((locals.var_k1edge_i_dn8 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k1edge_i_dn9 = ((locals.var_k1edge_i_dn9 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k1edge_i_dn10 = ((locals.var_k1edge_i_dn10 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k1edge_i_dn11 = ((locals.var_k1edge_i_dn11 * assign5380_e6916) + (locals.var_k1edge_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k1edge_i_rv = 0.0;

        let assign5390_e6921: f64 = (locals.var_inv_l).powf(p.p220);
        let assign5390_e6924: f64 = (locals.var_inv_llong).powf(p.p220);
        let assign5390_e6925: f64 = (assign5390_e6921 - assign5390_e6924);
        let assign5390_e6927: f64 = (assign5390_e6925).max(0.0);
        let assign5390_e6928: f64 = (p.p219 * assign5390_e6927);
        locals.var_t0 = assign5390_e6928;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5400_e6932: f64 = (locals.var_inv_w).powf(p.p222);
        let assign5400_e6935: f64 = (locals.var_inv_wwide).powf(p.p222);
        let assign5400_e6936: f64 = (assign5400_e6932 - assign5400_e6935);
        let assign5400_e6938: f64 = (assign5400_e6936).max(0.0);
        let assign5400_e6939: f64 = (p.p221 * assign5400_e6938);
        let assign5400_e6943: f64 = (locals.var_inv_wl).powf(p.p224);
        let assign5400_e6944: f64 = (p.p223 * assign5400_e6943);
        let assign5400_e6945: f64 = (assign5400_e6939 + assign5400_e6944);
        locals.var_t1 = assign5400_e6945;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5410_e6949: f64 = (1.0 + locals.var_t0);
        let assign5410_e6951: f64 = (assign5410_e6949 + locals.var_t1);
        let assign5410_e6952: f64 = (locals.var_k2_i * assign5410_e6951);
        locals.var_k2_i = assign5410_e6952;
        locals.var_k2_i_dn3 = ((locals.var_k2_i_dn3 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k2_i_dn4 = ((locals.var_k2_i_dn4 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k2_i_dn5 = ((locals.var_k2_i_dn5 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k2_i_dn6 = ((locals.var_k2_i_dn6 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k2_i_dn7 = ((locals.var_k2_i_dn7 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k2_i_dn8 = ((locals.var_k2_i_dn8 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k2_i_dn9 = ((locals.var_k2_i_dn9 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k2_i_dn10 = ((locals.var_k2_i_dn10 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k2_i_dn11 = ((locals.var_k2_i_dn11 * assign5410_e6951) + (locals.var_k2_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k2_i_rv = 0.0;

        let assign5420_e6956: f64 = (locals.var_inv_l).powf(p.p1267);
        let assign5420_e6959: f64 = (locals.var_inv_llong).powf(p.p1267);
        let assign5420_e6960: f64 = (assign5420_e6956 - assign5420_e6959);
        let assign5420_e6962: f64 = (assign5420_e6960).max(0.0);
        let assign5420_e6963: f64 = (p.p1266 * assign5420_e6962);
        locals.var_t0 = assign5420_e6963;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5430_e6967: f64 = (locals.var_inv_w).powf(p.p1269);
        let assign5430_e6970: f64 = (locals.var_inv_wwide).powf(p.p1269);
        let assign5430_e6971: f64 = (assign5430_e6967 - assign5430_e6970);
        let assign5430_e6973: f64 = (assign5430_e6971).max(0.0);
        let assign5430_e6974: f64 = (p.p1268 * assign5430_e6973);
        let assign5430_e6978: f64 = (locals.var_inv_wl).powf(p.p1271);
        let assign5430_e6979: f64 = (p.p1270 * assign5430_e6978);
        let assign5430_e6980: f64 = (assign5430_e6974 + assign5430_e6979);
        locals.var_t1 = assign5430_e6980;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign5440_e6984: f64 = (1.0 + locals.var_t0);
        let assign5440_e6986: f64 = (assign5440_e6984 + locals.var_t1);
        let assign5440_e6987: f64 = (locals.var_k2edge_i * assign5440_e6986);
        locals.var_k2edge_i = assign5440_e6987;
        locals.var_k2edge_i_dn3 = ((locals.var_k2edge_i_dn3 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k2edge_i_dn4 = ((locals.var_k2edge_i_dn4 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k2edge_i_dn5 = ((locals.var_k2edge_i_dn5 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k2edge_i_dn6 = ((locals.var_k2edge_i_dn6 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k2edge_i_dn7 = ((locals.var_k2edge_i_dn7 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k2edge_i_dn8 = ((locals.var_k2edge_i_dn8 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k2edge_i_dn9 = ((locals.var_k2edge_i_dn9 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k2edge_i_dn10 = ((locals.var_k2edge_i_dn10 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k2edge_i_dn11 = ((locals.var_k2edge_i_dn11 * assign5440_e6986) + (locals.var_k2edge_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k2edge_i_rv = 0.0;

        let assign5450_e6993: f64 = (locals.var_inv_l).powf(p.p448);
        let assign5450_e6996: f64 = (locals.var_inv_llong).powf(p.p448);
        let assign5450_e6997: f64 = (assign5450_e6993 - assign5450_e6996);
        let assign5450_e6999: f64 = (assign5450_e6997).max(0.0);
        let assign5450_e7000: f64 = (p.p447 * assign5450_e6999);
        let assign5450_e7001: f64 = (1.0 + assign5450_e7000);
        let assign5450_e7002: f64 = (locals.var_prwb_i * assign5450_e7001);
        locals.var_prwb_i = assign5450_e7002;
        locals.var_prwb_i_rv = 0.0;

        let assign5460_e7007: f64 = (locals.var_inv_l * p.p1036);
        let assign5460_e7008: f64 = (1.0 + assign5460_e7007);
        let assign5460_e7009: f64 = (locals.var_ute_i * assign5460_e7008);
        locals.var_ute_i = assign5460_e7009;
        locals.var_ute_i_rv = 0.0;

        let assign5470_e7014: f64 = (locals.var_inv_l * p.p1041);
        let assign5470_e7015: f64 = (1.0 + assign5470_e7014);
        let assign5470_e7016: f64 = (locals.var_ua1_i * assign5470_e7015);
        locals.var_ua1_i = assign5470_e7016;
        locals.var_ua1_i_rv = 0.0;

        let assign5480_e7021: f64 = (locals.var_inv_l * p.p1050);
        let assign5480_e7022: f64 = (1.0 + assign5480_e7021);
        let assign5480_e7023: f64 = (locals.var_ud1_i * assign5480_e7022);
        locals.var_ud1_i = assign5480_e7023;
        locals.var_ud1_i_rv = 0.0;

        let assign5490_e7028: f64 = (locals.var_inv_l * p.p1068);
        let assign5490_e7029: f64 = (1.0 + assign5490_e7028);
        let assign5490_e7030: f64 = (locals.var_at_i * assign5490_e7029);
        locals.var_at_i = assign5490_e7030;
        locals.var_at_i_rv = 0.0;

        let assign5500_e7035: f64 = (locals.var_inv_l * p.p1074);
        let assign5500_e7036: f64 = (1.0 + assign5500_e7035);
        let assign5500_e7037: f64 = (locals.var_ptwgt_i * assign5500_e7036);
        locals.var_ptwgt_i = assign5500_e7037;
        locals.var_ptwgt_i_rv = 0.0;

        let assign5510_e7040: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5510_e7040;
        locals.var_guard38_rv = 0.0;

        let (assign5520_e7058,) = {
    if (locals.var_guard38 != 0.0) {
        let assign5520_e7047: f64 = (locals.var_inv_l).powf(p.p462);
        let assign5520_e7050: f64 = (locals.var_inv_llong).powf(p.p462);
        let assign5520_e7051: f64 = (assign5520_e7047 - assign5520_e7050);
        let assign5520_e7053: f64 = (assign5520_e7051).max(0.0);
        let assign5520_e7054: f64 = (p.p461 * assign5520_e7053);
        let assign5520_e7055: f64 = (1.0 + assign5520_e7054);
        let assign5520_e7056: f64 = (locals.var_rsw_i * assign5520_e7055);
        (assign5520_e7056,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign5520_e7058;
        locals.var_rsw_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5530_e7076,) = {
    if (locals.var_guard38 != 0.0) {
        let assign5530_e7065: f64 = (locals.var_inv_l).powf(p.p472);
        let assign5530_e7068: f64 = (locals.var_inv_llong).powf(p.p472);
        let assign5530_e7069: f64 = (assign5530_e7065 - assign5530_e7068);
        let assign5530_e7071: f64 = (assign5530_e7069).max(0.0);
        let assign5530_e7072: f64 = (p.p471 * assign5530_e7071);
        let assign5530_e7073: f64 = (1.0 + assign5530_e7072);
        let assign5530_e7074: f64 = (locals.var_rdw_i * assign5530_e7073);
        (assign5530_e7074,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign5530_e7076;
        locals.var_rdw_i_rv = 0.0;

        let (assign5540_e7095,) = {
    if (locals.var_guard38 == 0.0) {
        let assign5540_e7084: f64 = (locals.var_inv_l).powf(p.p479);
        let assign5540_e7087: f64 = (locals.var_inv_llong).powf(p.p479);
        let assign5540_e7088: f64 = (assign5540_e7084 - assign5540_e7087);
        let assign5540_e7090: f64 = (assign5540_e7088).max(0.0);
        let assign5540_e7091: f64 = (p.p478 * assign5540_e7090);
        let assign5540_e7092: f64 = (1.0 + assign5540_e7091);
        let assign5540_e7093: f64 = (locals.var_rdsw_i * assign5540_e7092);
        (assign5540_e7093,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign5540_e7095;
        locals.var_rdsw_i_rv = 0.0;

        let assign5550_e7098: f64 = if locals.var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign5550_e7098;
        locals.var_guard39_rv = 0.0;

        let (assign5560_e7102,) = {
    if (locals.var_guard39 != 0.0) {
        (1.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign5560_e7102;
        locals.var_ucs_i_rv = 0.0;

        let assign5570_e7105: f64 = if locals.var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign5570_e7105;
        locals.var_guard40_rv = 0.0;

        let (assign5580_e7112,) = {
    if ((locals.var_guard39 == 0.0) && (locals.var_guard40 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign5580_e7112;
        locals.var_ucs_i_rv = 0.0;

        let assign5590_e7115: f64 = if p.p35 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign5590_e7115;
        locals.var_guard41_rv = 0.0;

        let assign5600_e7118: f64 = if locals.var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign5600_e7118;
        locals.var_guard42_rv = 0.0;

        let (assign5610_e7124,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign5610_e7124;
        locals.var_ucsr_i_rv = 0.0;

        let assign5620_e7127: f64 = if locals.var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign5620_e7127;
        locals.var_guard43_rv = 0.0;

        let (assign5630_e7136,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign5630_e7136;
        locals.var_ucsr_i_rv = 0.0;

        let assign5900_e7219: f64 = if locals.var_m0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign5900_e7219;
        locals.var_guard68_rv = 0.0;

        let (assign5910_e7223,) = {
    if (locals.var_guard68 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0_i,)
    }
};
        locals.var_m0_i = assign5910_e7223;
        locals.var_m0_i_rv = 0.0;

        let assign5920_e7226: f64 = if locals.var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign5920_e7226;
        locals.var_guard69_rv = 0.0;

        let (assign5930_e7230,) = {
    if (locals.var_guard69 != 0.0) {
        (0.067,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign5930_e7230;
        locals.var_u0_i_rv = 0.0;

        let assign5940_e7233: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign5940_e7233;
        locals.var_guard70_rv = 0.0;

        let (assign5950_e7237, assign5950_e7237_d_n3, assign5950_e7237_d_n4, assign5950_e7237_d_n5, assign5950_e7237_d_n6, assign5950_e7237_d_n7, assign5950_e7237_d_n8, assign5950_e7237_d_n9, assign5950_e7237_d_n10, assign5950_e7237_d_n11,) = {
    if (locals.var_guard70 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11,)
    }
};
        locals.var_ua_i = assign5950_e7237;
        locals.var_ua_i_dn3 = assign5950_e7237_d_n3;
        locals.var_ua_i_dn4 = assign5950_e7237_d_n4;
        locals.var_ua_i_dn5 = assign5950_e7237_d_n5;
        locals.var_ua_i_dn6 = assign5950_e7237_d_n6;
        locals.var_ua_i_dn7 = assign5950_e7237_d_n7;
        locals.var_ua_i_dn8 = assign5950_e7237_d_n8;
        locals.var_ua_i_dn9 = assign5950_e7237_d_n9;
        locals.var_ua_i_dn10 = assign5950_e7237_d_n10;
        locals.var_ua_i_dn11 = assign5950_e7237_d_n11;
        locals.var_ua_i_rv = 0.0;

        let assign5960_e7240: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign5960_e7240;
        locals.var_guard71_rv = 0.0;

        let (assign5970_e7244, assign5970_e7244_d_n3, assign5970_e7244_d_n4, assign5970_e7244_d_n5, assign5970_e7244_d_n6, assign5970_e7244_d_n7, assign5970_e7244_d_n8, assign5970_e7244_d_n9, assign5970_e7244_d_n10, assign5970_e7244_d_n11,) = {
    if (locals.var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11,)
    }
};
        locals.var_eu_i = assign5970_e7244;
        locals.var_eu_i_dn3 = assign5970_e7244_d_n3;
        locals.var_eu_i_dn4 = assign5970_e7244_d_n4;
        locals.var_eu_i_dn5 = assign5970_e7244_d_n5;
        locals.var_eu_i_dn6 = assign5970_e7244_d_n6;
        locals.var_eu_i_dn7 = assign5970_e7244_d_n7;
        locals.var_eu_i_dn8 = assign5970_e7244_d_n8;
        locals.var_eu_i_dn9 = assign5970_e7244_d_n9;
        locals.var_eu_i_dn10 = assign5970_e7244_d_n10;
        locals.var_eu_i_dn11 = assign5970_e7244_d_n11;
        locals.var_eu_i_rv = 0.0;

        let assign5980_e7247: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign5980_e7247;
        locals.var_guard72_rv = 0.0;

        let (assign5990_e7251, assign5990_e7251_d_n3, assign5990_e7251_d_n4, assign5990_e7251_d_n5, assign5990_e7251_d_n6, assign5990_e7251_d_n7, assign5990_e7251_d_n8, assign5990_e7251_d_n9, assign5990_e7251_d_n10, assign5990_e7251_d_n11,) = {
    if (locals.var_guard72 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11,)
    }
};
        locals.var_ud_i = assign5990_e7251;
        locals.var_ud_i_dn3 = assign5990_e7251_d_n3;
        locals.var_ud_i_dn4 = assign5990_e7251_d_n4;
        locals.var_ud_i_dn5 = assign5990_e7251_d_n5;
        locals.var_ud_i_dn6 = assign5990_e7251_d_n6;
        locals.var_ud_i_dn7 = assign5990_e7251_d_n7;
        locals.var_ud_i_dn8 = assign5990_e7251_d_n8;
        locals.var_ud_i_dn9 = assign5990_e7251_d_n9;
        locals.var_ud_i_dn10 = assign5990_e7251_d_n10;
        locals.var_ud_i_dn11 = assign5990_e7251_d_n11;
        locals.var_ud_i_rv = 0.0;

        let assign6000_e7254: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign6000_e7254;
        locals.var_guard73_rv = 0.0;

        let (assign6010_e7258,) = {
    if (locals.var_guard73 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign6010_e7258;
        locals.var_ucs_i_rv = 0.0;

        let assign6020_e7261: f64 = if locals.var_ndiode_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign6020_e7261;
        locals.var_guard74_rv = 0.0;

        let (assign6030_e7265,) = {
    if (locals.var_guard74 != 0.0) {
        (1.0,)
    } else {
        (locals.var_ndiode_i,)
    }
};
        locals.var_ndiode_i = assign6030_e7265;
        locals.var_ndiode_i_rv = 0.0;

        let assign6040_e7268: f64 = if locals.var_nrecr0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign6040_e7268;
        locals.var_guard75_rv = 0.0;

        let (assign6050_e7272,) = {
    if (locals.var_guard75 != 0.0) {
        (10.0,)
    } else {
        (locals.var_nrecr0_i,)
    }
};
        locals.var_nrecr0_i = assign6050_e7272;
        locals.var_nrecr0_i_rv = 0.0;

        let assign6060_e7275: f64 = if locals.var_nrecf0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign6060_e7275;
        locals.var_guard76_rv = 0.0;

        let (assign6070_e7279,) = {
    if (locals.var_guard76 != 0.0) {
        (2.0,)
    } else {
        (locals.var_nrecf0_i,)
    }
};
        locals.var_nrecf0_i = assign6070_e7279;
        locals.var_nrecf0_i_rv = 0.0;

        locals.var_nuendd = 0.0;
        locals.var_nuendd_rv = 0.0;

        locals.var_nuends = 0.0;
        locals.var_nuends_rv = 0.0;

        locals.var_nuintd = 0.0;
        locals.var_nuintd_rv = 0.0;

        locals.var_nuints = 0.0;
        locals.var_nuints_rv = 0.0;

        locals.var_rend = 0.0;
        locals.var_rend_rv = 0.0;

        locals.var_rint = 0.0;
        locals.var_rint_rv = 0.0;

        let assign6150_e7291: f64 = (p.p895 - p.p898);
        locals.var_dmcgeff = assign6150_e7291;
        locals.var_dmcgeff_rv = 0.0;

        locals.var_dmcieff = p.p896;
        locals.var_dmcieff_rv = 0.0;

        let assign6170_e7295: f64 = (p.p897 - p.p898);
        locals.var_dmdgeff = assign6170_e7295;
        locals.var_dmdgeff_rv = 0.0;

        let assign6180_e7297: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard78 = assign6180_e7297;
        locals.var_guard78_rv = 0.0;

        let (assign6190_e7303,) = {
    if (locals.var_guard78 != 0.0) {
        let assign6190_e7301: f64 = (p.p438 * p.p3);
        (assign6190_e7301,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign6190_e7303;
        locals.var_rsourcegeo_rv = 0.0;

        let assign6200_e7310: f64 = if ((p.p9 > 0.0) && (p.p438 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign6200_e7310;
        locals.var_guard79_rv = 0.0;

        let assign6210_e7313: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign6210_e7313;
        locals.var_guard80_rv = 0.0;

        let assign6220_e7316: f64 = (p.p2 % 2.0);
        let assign6220_e7318: f64 = if assign6220_e7316 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign6220_e7318;
        locals.var_guard81_rv = 0.0;

        let (assign6230_e7329,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign6230_e7329;
        locals.var_nuendd_rv = 0.0;

        let (assign6240_e7340,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign6240_e7340;
        locals.var_nuends_rv = 0.0;

        let (assign6250_e7359,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        let assign6250_e7352: f64 = (p.p2 - 1.0);
        let assign6250_e7354: f64 = (assign6250_e7352 / 2.0);
        let assign6250_e7356: f64 = (assign6250_e7354).max(0.0);
        let assign6250_e7357: f64 = (2.0 * assign6250_e7356);
        (assign6250_e7357,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign6250_e7359;
        locals.var_nuintd_rv = 0.0;

        let (assign6260_e7370,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign6260_e7370;
        locals.var_nuints_rv = 0.0;

        let assign6270_e7373: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign6270_e7373;
        locals.var_guard82_rv = 0.0;

        let (assign6280_e7387,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign6280_e7387;
        locals.var_nuendd_rv = 0.0;

        let (assign6290_e7409,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        let assign6290_e7402: f64 = (p.p2 / 2.0);
        let assign6290_e7404: f64 = (assign6290_e7402 - 1.0);
        let assign6290_e7406: f64 = (assign6290_e7404).max(0.0);
        let assign6290_e7407: f64 = (2.0 * assign6290_e7406);
        (assign6290_e7407,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign6290_e7409;
        locals.var_nuintd_rv = 0.0;

        let (assign6300_e7423,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign6300_e7423;
        locals.var_nuends_rv = 0.0;

        let (assign6310_e7437,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign6310_e7437;
        locals.var_nuints_rv = 0.0;

        let (assign6320_e7452,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign6320_e7452;
        locals.var_nuendd_rv = 0.0;

        let (assign6330_e7467,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign6330_e7467;
        locals.var_nuintd_rv = 0.0;

        let (assign6340_e7482,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign6340_e7482;
        locals.var_nuends_rv = 0.0;

        let (assign6350_e7505,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        let assign6350_e7498: f64 = (p.p2 / 2.0);
        let assign6350_e7500: f64 = (assign6350_e7498 - 1.0);
        let assign6350_e7502: f64 = (assign6350_e7500).max(0.0);
        let assign6350_e7503: f64 = (2.0 * assign6350_e7502);
        (assign6350_e7503,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign6350_e7505;
        locals.var_nuints_rv = 0.0;

        let assign6360_e7508: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign6360_e7508;
        locals.var_guard83_rv = 0.0;

        let assign6370_e7511: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign6370_e7511;
        locals.var_guard84_rv = 0.0;

        let (assign6380_e7524,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 != 0.0)) && (locals.var_guard84 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6380_e7524;
        locals.var_rint_rv = 0.0;

        let (assign6390_e7544,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 != 0.0)) && (locals.var_guard84 == 0.0)) {
        let assign6390_e7538: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6390_e7541: f64 = (locals.var_weff * locals.var_nuints);
        let assign6390_e7542: f64 = (assign6390_e7538 / assign6390_e7541);
        (assign6390_e7542,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6390_e7544;
        locals.var_rint_rv = 0.0;

        let assign6400_e7547: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign6400_e7547;
        locals.var_guard85_rv = 0.0;

        let (assign6410_e7561,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 == 0.0)) && (locals.var_guard85 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6410_e7561;
        locals.var_rint_rv = 0.0;

        let (assign6420_e7582,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) && (locals.var_guard83 == 0.0)) && (locals.var_guard85 == 0.0)) {
        let assign6420_e7576: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6420_e7579: f64 = (locals.var_weff * locals.var_nuintd);
        let assign6420_e7580: f64 = (assign6420_e7576 / assign6420_e7579);
        (assign6420_e7580,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign6420_e7582;
        locals.var_rint_rv = 0.0;

        let assign6430_e7585: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign6430_e7585;
        locals.var_guard86_rv = 0.0;

        let assign6440_e7588: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign6440_e7588;
        locals.var_guard87_rv = 0.0;

        let assign6450_e7591: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign6450_e7591;
        locals.var_guard88_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6460_e7594: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign6460_e7594;
        locals.var_guard89_rv = 0.0;

        let assign6470_e7597: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign6470_e7597;
        locals.var_guard90_rv = 0.0;

        let assign6480_e7600: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign6480_e7600;
        locals.var_guard91_rv = 0.0;

        let assign6490_e7603: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign6490_e7603;
        locals.var_guard92_rv = 0.0;

        let assign6500_e7606: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign6500_e7606;
        locals.var_guard93_rv = 0.0;

        let assign6510_e7609: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign6510_e7609;
        locals.var_guard94_rv = 0.0;

        let assign6520_e7612: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign6520_e7612;
        locals.var_guard95_rv = 0.0;

        let assign6530_e7615: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign6530_e7615;
        locals.var_guard96_rv = 0.0;

        let assign6540_e7618: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign6540_e7618;
        locals.var_guard97_rv = 0.0;

        let assign6550_e7621: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign6550_e7621;
        locals.var_guard98_rv = 0.0;

        let assign6560_e7632: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign6560_e7632;
        locals.var_guard99_rv = 0.0;

        let assign6570_e7643: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign6570_e7643;
        locals.var_guard100_rv = 0.0;

        let assign6580_e7646: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign6580_e7646;
        locals.var_guard101_rv = 0.0;

        let (assign6590_e7663,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard101 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6590_e7663;
        locals.var_rend_rv = 0.0;

        let (assign6600_e7687,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) && (locals.var_guard101 == 0.0)) {
        let assign6600_e7681: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6600_e7684: f64 = (locals.var_weff * locals.var_nuends);
        let assign6600_e7685: f64 = (assign6600_e7681 / assign6600_e7684);
        (assign6600_e7685,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6600_e7687;
        locals.var_rend_rv = 0.0;

        let assign6620_e7698: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6620_e7701: f64 = if ((locals.var_nuends == 0.0) || (assign6620_e7698 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign6620_e7701;
        locals.var_guard103_rv = 0.0;

        let (assign6630_e7721,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && ((locals.var_guard100 != 0.0) && (locals.var_guard99 == 0.0))) && (locals.var_guard103 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6630_e7721;
        locals.var_rend_rv = 0.0;

        let (assign6640_e7752,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && ((locals.var_guard100 != 0.0) && (locals.var_guard99 == 0.0))) && (locals.var_guard103 == 0.0)) {
        let assign6640_e7742: f64 = (p.p438 * locals.var_weff);
        let assign6640_e7745: f64 = (3.0 * locals.var_nuends);
        let assign6640_e7748: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6640_e7749: f64 = (assign6640_e7745 * assign6640_e7748);
        let assign6640_e7750: f64 = (assign6640_e7742 / assign6640_e7749);
        (assign6640_e7750,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6640_e7752;
        locals.var_rend_rv = 0.0;

        let (assign6650_e7770,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (!((locals.var_guard99 != 0.0) || (locals.var_guard100 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6650_e7770;
        locals.var_rend_rv = 0.0;

        let assign6660_e7781: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign6660_e7781;
        locals.var_guard104_rv = 0.0;

        let assign6670_e7792: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard105 = assign6670_e7792;
        locals.var_guard105_rv = 0.0;

        let assign6680_e7795: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign6680_e7795;
        locals.var_guard106_rv = 0.0;

        let (assign6690_e7813,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6690_e7813;
        locals.var_rend_rv = 0.0;

        let (assign6700_e7838,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign6700_e7832: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6700_e7835: f64 = (locals.var_weff * locals.var_nuends);
        let assign6700_e7836: f64 = (assign6700_e7832 / assign6700_e7835);
        (assign6700_e7836,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6700_e7838;
        locals.var_rend_rv = 0.0;

        let assign6720_e7849: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6720_e7852: f64 = if ((locals.var_nuends == 0.0) || (assign6720_e7849 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard108 = assign6720_e7852;
        locals.var_guard108_rv = 0.0;

        let (assign6730_e7873,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && ((locals.var_guard105 != 0.0) && (locals.var_guard104 == 0.0))) && (locals.var_guard108 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6730_e7873;
        locals.var_rend_rv = 0.0;

        let (assign6740_e7905,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && ((locals.var_guard105 != 0.0) && (locals.var_guard104 == 0.0))) && (locals.var_guard108 == 0.0)) {
        let assign6740_e7895: f64 = (p.p438 * locals.var_weff);
        let assign6740_e7898: f64 = (3.0 * locals.var_nuends);
        let assign6740_e7901: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6740_e7902: f64 = (assign6740_e7898 * assign6740_e7901);
        let assign6740_e7903: f64 = (assign6740_e7895 / assign6740_e7902);
        (assign6740_e7903,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6740_e7905;
        locals.var_rend_rv = 0.0;

        let (assign6750_e7924,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 == 0.0)) && (!((locals.var_guard104 != 0.0) || (locals.var_guard105 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6750_e7924;
        locals.var_rend_rv = 0.0;

        let assign6760_e7927: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign6760_e7927;
        locals.var_guard109_rv = 0.0;

        let assign6770_e7938: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6770_e7938;
        locals.var_guard110_rv = 0.0;

        let assign6780_e7949: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard111 = assign6780_e7949;
        locals.var_guard111_rv = 0.0;

        let assign6790_e7952: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6790_e7952;
        locals.var_guard112_rv = 0.0;

        let (assign6800_e7970,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6800_e7970;
        locals.var_rend_rv = 0.0;

        let (assign6810_e7995,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) && (locals.var_guard112 == 0.0)) {
        let assign6810_e7989: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6810_e7992: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6810_e7993: f64 = (assign6810_e7989 / assign6810_e7992);
        (assign6810_e7993,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6810_e7995;
        locals.var_rend_rv = 0.0;

        let assign6830_e8006: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6830_e8009: f64 = if ((locals.var_nuendd == 0.0) || (assign6830_e8006 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6830_e8009;
        locals.var_guard114_rv = 0.0;

        let (assign6840_e8030,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && ((locals.var_guard111 != 0.0) && (locals.var_guard110 == 0.0))) && (locals.var_guard114 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6840_e8030;
        locals.var_rend_rv = 0.0;

        let (assign6850_e8062,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && ((locals.var_guard111 != 0.0) && (locals.var_guard110 == 0.0))) && (locals.var_guard114 == 0.0)) {
        let assign6850_e8052: f64 = (p.p438 * locals.var_weff);
        let assign6850_e8055: f64 = (3.0 * locals.var_nuendd);
        let assign6850_e8058: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6850_e8059: f64 = (assign6850_e8055 * assign6850_e8058);
        let assign6850_e8060: f64 = (assign6850_e8052 / assign6850_e8059);
        (assign6850_e8060,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6850_e8062;
        locals.var_rend_rv = 0.0;

        let (assign6860_e8081,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 != 0.0)) && (!((locals.var_guard110 != 0.0) || (locals.var_guard111 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6860_e8081;
        locals.var_rend_rv = 0.0;

        let assign6870_e8092: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6870_e8092;
        locals.var_guard115_rv = 0.0;

        let assign6880_e8103: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6880_e8103;
        locals.var_guard116_rv = 0.0;

        let assign6890_e8106: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign6890_e8106;
        locals.var_guard117_rv = 0.0;

        let (assign6900_e8125,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && (locals.var_guard115 != 0.0)) && (locals.var_guard117 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6900_e8125;
        locals.var_rend_rv = 0.0;

        let (assign6910_e8151,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && (locals.var_guard115 != 0.0)) && (locals.var_guard117 == 0.0)) {
        let assign6910_e8145: f64 = (p.p438 * locals.var_dmcgeff);
        let assign6910_e8148: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6910_e8149: f64 = (assign6910_e8145 / assign6910_e8148);
        (assign6910_e8149,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6910_e8151;
        locals.var_rend_rv = 0.0;

        let assign6930_e8162: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6930_e8165: f64 = if ((locals.var_nuendd == 0.0) || (assign6930_e8162 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign6930_e8165;
        locals.var_guard119_rv = 0.0;

        let (assign6940_e8187,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && ((locals.var_guard116 != 0.0) && (locals.var_guard115 == 0.0))) && (locals.var_guard119 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6940_e8187;
        locals.var_rend_rv = 0.0;

        let (assign6950_e8220,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && ((locals.var_guard116 != 0.0) && (locals.var_guard115 == 0.0))) && (locals.var_guard119 == 0.0)) {
        let assign6950_e8210: f64 = (p.p438 * locals.var_weff);
        let assign6950_e8213: f64 = (3.0 * locals.var_nuendd);
        let assign6950_e8216: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6950_e8217: f64 = (assign6950_e8213 * assign6950_e8216);
        let assign6950_e8218: f64 = (assign6950_e8210 / assign6950_e8217);
        (assign6950_e8218,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6950_e8220;
        locals.var_rend_rv = 0.0;

        let (assign6960_e8240,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard86 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard109 == 0.0)) && (!((locals.var_guard115 != 0.0) || (locals.var_guard116 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6960_e8240;
        locals.var_rend_rv = 0.0;

        let assign6970_e8243: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign6970_e8243;
        locals.var_guard120_rv = 0.0;

        let assign6980_e8246: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign6980_e8246;
        locals.var_guard121_rv = 0.0;

        let assign6990_e8257: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign6990_e8257;
        locals.var_guard122_rv = 0.0;

        let assign7000_e8268: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign7000_e8268;
        locals.var_guard123_rv = 0.0;

        let assign7010_e8271: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign7010_e8271;
        locals.var_guard124_rv = 0.0;

        let (assign7020_e8291,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard122 != 0.0)) && (locals.var_guard124 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7020_e8291;
        locals.var_rend_rv = 0.0;

        let (assign7030_e8318,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard122 != 0.0)) && (locals.var_guard124 == 0.0)) {
        let assign7030_e8312: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7030_e8315: f64 = (locals.var_weff * locals.var_nuends);
        let assign7030_e8316: f64 = (assign7030_e8312 / assign7030_e8315);
        (assign7030_e8316,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7030_e8318;
        locals.var_rend_rv = 0.0;

        let assign7050_e8329: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7050_e8332: f64 = if ((locals.var_nuends == 0.0) || (assign7050_e8329 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign7050_e8332;
        locals.var_guard126_rv = 0.0;

        let (assign7060_e8355,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && ((locals.var_guard123 != 0.0) && (locals.var_guard122 == 0.0))) && (locals.var_guard126 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7060_e8355;
        locals.var_rend_rv = 0.0;

        let (assign7070_e8389,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && ((locals.var_guard123 != 0.0) && (locals.var_guard122 == 0.0))) && (locals.var_guard126 == 0.0)) {
        let assign7070_e8379: f64 = (p.p438 * locals.var_weff);
        let assign7070_e8382: f64 = (3.0 * locals.var_nuends);
        let assign7070_e8385: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7070_e8386: f64 = (assign7070_e8382 * assign7070_e8385);
        let assign7070_e8387: f64 = (assign7070_e8379 / assign7070_e8386);
        (assign7070_e8387,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7070_e8389;
        locals.var_rend_rv = 0.0;

        let (assign7080_e8410,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (!((locals.var_guard122 != 0.0) || (locals.var_guard123 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7080_e8410;
        locals.var_rend_rv = 0.0;

        let assign7090_e8421: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign7090_e8421;
        locals.var_guard127_rv = 0.0;

        let assign7100_e8432: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign7100_e8432;
        locals.var_guard128_rv = 0.0;

        let assign7110_e8435: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign7110_e8435;
        locals.var_guard129_rv = 0.0;

        let (assign7120_e8456,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && (locals.var_guard127 != 0.0)) && (locals.var_guard129 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7120_e8456;
        locals.var_rend_rv = 0.0;

        let (assign7130_e8484,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && (locals.var_guard127 != 0.0)) && (locals.var_guard129 == 0.0)) {
        let assign7130_e8478: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7130_e8481: f64 = (locals.var_weff * locals.var_nuends);
        let assign7130_e8482: f64 = (assign7130_e8478 / assign7130_e8481);
        (assign7130_e8482,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7130_e8484;
        locals.var_rend_rv = 0.0;

        let assign7150_e8495: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7150_e8498: f64 = if ((locals.var_nuends == 0.0) || (assign7150_e8495 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign7150_e8498;
        locals.var_guard131_rv = 0.0;

        let (assign7160_e8522,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && ((locals.var_guard128 != 0.0) && (locals.var_guard127 == 0.0))) && (locals.var_guard131 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7160_e8522;
        locals.var_rend_rv = 0.0;

        let (assign7170_e8557,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && ((locals.var_guard128 != 0.0) && (locals.var_guard127 == 0.0))) && (locals.var_guard131 == 0.0)) {
        let assign7170_e8547: f64 = (p.p438 * locals.var_weff);
        let assign7170_e8550: f64 = (3.0 * locals.var_nuends);
        let assign7170_e8553: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7170_e8554: f64 = (assign7170_e8550 * assign7170_e8553);
        let assign7170_e8555: f64 = (assign7170_e8547 / assign7170_e8554);
        (assign7170_e8555,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7170_e8557;
        locals.var_rend_rv = 0.0;

        let (assign7180_e8579,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 == 0.0)) && (!((locals.var_guard127 != 0.0) || (locals.var_guard128 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7180_e8579;
        locals.var_rend_rv = 0.0;

        let assign7190_e8582: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign7190_e8582;
        locals.var_guard132_rv = 0.0;

        let assign7200_e8593: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7200_e8593;
        locals.var_guard133_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7210_e8604: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7210_e8604;
        locals.var_guard134_rv = 0.0;

        let assign7220_e8607: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign7220_e8607;
        locals.var_guard135_rv = 0.0;

        let (assign7230_e8628,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard133 != 0.0)) && (locals.var_guard135 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7230_e8628;
        locals.var_rend_rv = 0.0;

        let (assign7240_e8656,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard133 != 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign7240_e8650: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7240_e8653: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7240_e8654: f64 = (assign7240_e8650 / assign7240_e8653);
        (assign7240_e8654,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7240_e8656;
        locals.var_rend_rv = 0.0;

        let assign7260_e8666: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign7260_e8666;
        locals.var_guard137_rv = 0.0;

        let (assign7270_e8690,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && ((locals.var_guard134 != 0.0) && (locals.var_guard133 == 0.0))) && (locals.var_guard137 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7270_e8690;
        locals.var_rend_rv = 0.0;

        let (assign7280_e8723,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && ((locals.var_guard134 != 0.0) && (locals.var_guard133 == 0.0))) && (locals.var_guard137 == 0.0)) {
        let assign7280_e8715: f64 = (p.p438 * locals.var_weff);
        let assign7280_e8718: f64 = (6.0 * locals.var_nuendd);
        let assign7280_e8720: f64 = (assign7280_e8718 * locals.var_dmcgeff);
        let assign7280_e8721: f64 = (assign7280_e8715 / assign7280_e8720);
        (assign7280_e8721,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7280_e8723;
        locals.var_rend_rv = 0.0;

        let (assign7290_e8745,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 != 0.0)) && (!((locals.var_guard133 != 0.0) || (locals.var_guard134 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7290_e8745;
        locals.var_rend_rv = 0.0;

        let assign7300_e8756: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign7300_e8756;
        locals.var_guard138_rv = 0.0;

        let assign7310_e8767: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign7310_e8767;
        locals.var_guard139_rv = 0.0;

        let assign7320_e8770: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign7320_e8770;
        locals.var_guard140_rv = 0.0;

        let (assign7330_e8792,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard140 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7330_e8792;
        locals.var_rend_rv = 0.0;

        let (assign7340_e8821,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign7340_e8815: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7340_e8818: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7340_e8819: f64 = (assign7340_e8815 / assign7340_e8818);
        (assign7340_e8819,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7340_e8821;
        locals.var_rend_rv = 0.0;

        let assign7360_e8831: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7360_e8831;
        locals.var_guard142_rv = 0.0;

        let (assign7370_e8856,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && ((locals.var_guard139 != 0.0) && (locals.var_guard138 == 0.0))) && (locals.var_guard142 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7370_e8856;
        locals.var_rend_rv = 0.0;

        let (assign7380_e8890,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && ((locals.var_guard139 != 0.0) && (locals.var_guard138 == 0.0))) && (locals.var_guard142 == 0.0)) {
        let assign7380_e8882: f64 = (p.p438 * locals.var_weff);
        let assign7380_e8885: f64 = (6.0 * locals.var_nuendd);
        let assign7380_e8887: f64 = (assign7380_e8885 * locals.var_dmcgeff);
        let assign7380_e8888: f64 = (assign7380_e8882 / assign7380_e8887);
        (assign7380_e8888,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7380_e8890;
        locals.var_rend_rv = 0.0;

        let (assign7390_e8913,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard87 != 0.0) && (locals.var_guard86 == 0.0))) && (locals.var_guard120 == 0.0)) && (locals.var_guard132 == 0.0)) && (!((locals.var_guard138 != 0.0) || (locals.var_guard139 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7390_e8913;
        locals.var_rend_rv = 0.0;

        let assign7400_e8916: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7400_e8916;
        locals.var_guard143_rv = 0.0;

        let assign7410_e8919: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7410_e8919;
        locals.var_guard144_rv = 0.0;

        let assign7420_e8930: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7420_e8930;
        locals.var_guard145_rv = 0.0;

        let assign7430_e8941: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7430_e8941;
        locals.var_guard146_rv = 0.0;

        let assign7440_e8944: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7440_e8944;
        locals.var_guard147_rv = 0.0;

        let (assign7450_e8966,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 != 0.0)) && (locals.var_guard147 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7450_e8966;
        locals.var_rend_rv = 0.0;

        let (assign7460_e8995,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7460_e8989: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7460_e8992: f64 = (locals.var_weff * locals.var_nuends);
        let assign7460_e8993: f64 = (assign7460_e8989 / assign7460_e8992);
        (assign7460_e8993,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7460_e8995;
        locals.var_rend_rv = 0.0;

        let assign7480_e9005: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7480_e9005;
        locals.var_guard149_rv = 0.0;

        let (assign7490_e9030,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && ((locals.var_guard146 != 0.0) && (locals.var_guard145 == 0.0))) && (locals.var_guard149 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7490_e9030;
        locals.var_rend_rv = 0.0;

        let (assign7500_e9064,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && ((locals.var_guard146 != 0.0) && (locals.var_guard145 == 0.0))) && (locals.var_guard149 == 0.0)) {
        let assign7500_e9056: f64 = (p.p438 * locals.var_weff);
        let assign7500_e9059: f64 = (6.0 * locals.var_nuends);
        let assign7500_e9061: f64 = (assign7500_e9059 * locals.var_dmcgeff);
        let assign7500_e9062: f64 = (assign7500_e9056 / assign7500_e9061);
        (assign7500_e9062,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7500_e9064;
        locals.var_rend_rv = 0.0;

        let (assign7510_e9087,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (!((locals.var_guard145 != 0.0) || (locals.var_guard146 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7510_e9087;
        locals.var_rend_rv = 0.0;

        let assign7520_e9098: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7520_e9098;
        locals.var_guard150_rv = 0.0;

        let assign7530_e9109: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7530_e9109;
        locals.var_guard151_rv = 0.0;

        let assign7540_e9112: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7540_e9112;
        locals.var_guard152_rv = 0.0;

        let (assign7550_e9135,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && (locals.var_guard150 != 0.0)) && (locals.var_guard152 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7550_e9135;
        locals.var_rend_rv = 0.0;

        let (assign7560_e9165,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && (locals.var_guard150 != 0.0)) && (locals.var_guard152 == 0.0)) {
        let assign7560_e9159: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7560_e9162: f64 = (locals.var_weff * locals.var_nuends);
        let assign7560_e9163: f64 = (assign7560_e9159 / assign7560_e9162);
        (assign7560_e9163,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7560_e9165;
        locals.var_rend_rv = 0.0;

        let assign7580_e9175: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7580_e9175;
        locals.var_guard154_rv = 0.0;

        let (assign7590_e9201,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && ((locals.var_guard151 != 0.0) && (locals.var_guard150 == 0.0))) && (locals.var_guard154 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7590_e9201;
        locals.var_rend_rv = 0.0;

        let (assign7600_e9236,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && ((locals.var_guard151 != 0.0) && (locals.var_guard150 == 0.0))) && (locals.var_guard154 == 0.0)) {
        let assign7600_e9228: f64 = (p.p438 * locals.var_weff);
        let assign7600_e9231: f64 = (6.0 * locals.var_nuends);
        let assign7600_e9233: f64 = (assign7600_e9231 * locals.var_dmcgeff);
        let assign7600_e9234: f64 = (assign7600_e9228 / assign7600_e9233);
        (assign7600_e9234,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7600_e9236;
        locals.var_rend_rv = 0.0;

        let (assign7610_e9260,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 == 0.0)) && (!((locals.var_guard150 != 0.0) || (locals.var_guard151 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7610_e9260;
        locals.var_rend_rv = 0.0;

        let assign7620_e9263: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign7620_e9263;
        locals.var_guard155_rv = 0.0;

        let assign7630_e9274: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard156 = assign7630_e9274;
        locals.var_guard156_rv = 0.0;

        let assign7640_e9285: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard157 = assign7640_e9285;
        locals.var_guard157_rv = 0.0;

        let assign7650_e9288: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign7650_e9288;
        locals.var_guard158_rv = 0.0;

        let (assign7660_e9311,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard156 != 0.0)) && (locals.var_guard158 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7660_e9311;
        locals.var_rend_rv = 0.0;

        let (assign7670_e9341,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard156 != 0.0)) && (locals.var_guard158 == 0.0)) {
        let assign7670_e9335: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7670_e9338: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7670_e9339: f64 = (assign7670_e9335 / assign7670_e9338);
        (assign7670_e9339,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7670_e9341;
        locals.var_rend_rv = 0.0;

        let assign7690_e9352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7690_e9355: f64 = if ((locals.var_nuendd == 0.0) || (assign7690_e9352 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign7690_e9355;
        locals.var_guard160_rv = 0.0;

        let (assign7700_e9381,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && ((locals.var_guard157 != 0.0) && (locals.var_guard156 == 0.0))) && (locals.var_guard160 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7700_e9381;
        locals.var_rend_rv = 0.0;

        let (assign7710_e9418,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && ((locals.var_guard157 != 0.0) && (locals.var_guard156 == 0.0))) && (locals.var_guard160 == 0.0)) {
        let assign7710_e9408: f64 = (p.p438 * locals.var_weff);
        let assign7710_e9411: f64 = (3.0 * locals.var_nuendd);
        let assign7710_e9414: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7710_e9415: f64 = (assign7710_e9411 * assign7710_e9414);
        let assign7710_e9416: f64 = (assign7710_e9408 / assign7710_e9415);
        (assign7710_e9416,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7710_e9418;
        locals.var_rend_rv = 0.0;

        let (assign7720_e9442,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 != 0.0)) && (!((locals.var_guard156 != 0.0) || (locals.var_guard157 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7720_e9442;
        locals.var_rend_rv = 0.0;

        let assign7730_e9453: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard161 = assign7730_e9453;
        locals.var_guard161_rv = 0.0;

        let assign7740_e9464: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard162 = assign7740_e9464;
        locals.var_guard162_rv = 0.0;

        let assign7750_e9467: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign7750_e9467;
        locals.var_guard163_rv = 0.0;

        let (assign7760_e9491,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && (locals.var_guard161 != 0.0)) && (locals.var_guard163 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7760_e9491;
        locals.var_rend_rv = 0.0;

        let (assign7770_e9522,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && (locals.var_guard161 != 0.0)) && (locals.var_guard163 == 0.0)) {
        let assign7770_e9516: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7770_e9519: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7770_e9520: f64 = (assign7770_e9516 / assign7770_e9519);
        (assign7770_e9520,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7770_e9522;
        locals.var_rend_rv = 0.0;

        let assign7790_e9533: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7790_e9536: f64 = if ((locals.var_nuendd == 0.0) || (assign7790_e9533 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard165 = assign7790_e9536;
        locals.var_guard165_rv = 0.0;

        let (assign7800_e9563,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && ((locals.var_guard162 != 0.0) && (locals.var_guard161 == 0.0))) && (locals.var_guard165 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7800_e9563;
        locals.var_rend_rv = 0.0;

        let (assign7810_e9601,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && ((locals.var_guard162 != 0.0) && (locals.var_guard161 == 0.0))) && (locals.var_guard165 == 0.0)) {
        let assign7810_e9591: f64 = (p.p438 * locals.var_weff);
        let assign7810_e9594: f64 = (3.0 * locals.var_nuendd);
        let assign7810_e9597: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7810_e9598: f64 = (assign7810_e9594 * assign7810_e9597);
        let assign7810_e9599: f64 = (assign7810_e9591 / assign7810_e9598);
        (assign7810_e9599,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7810_e9601;
        locals.var_rend_rv = 0.0;

        let (assign7820_e9626,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard88 != 0.0) && (!((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0))))) && (locals.var_guard143 == 0.0)) && (locals.var_guard155 == 0.0)) && (!((locals.var_guard161 != 0.0) || (locals.var_guard162 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7820_e9626;
        locals.var_rend_rv = 0.0;

        let assign7830_e9629: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign7830_e9629;
        locals.var_guard166_rv = 0.0;

        let assign7840_e9632: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign7840_e9632;
        locals.var_guard167_rv = 0.0;

        let assign7850_e9643: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard168 = assign7850_e9643;
        locals.var_guard168_rv = 0.0;

        let assign7860_e9654: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard169 = assign7860_e9654;
        locals.var_guard169_rv = 0.0;

        let assign7870_e9657: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign7870_e9657;
        locals.var_guard170_rv = 0.0;

        let (assign7880_e9681,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard170 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7880_e9681;
        locals.var_rend_rv = 0.0;

        let (assign7890_e9712,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard168 != 0.0)) && (locals.var_guard170 == 0.0)) {
        let assign7890_e9706: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7890_e9709: f64 = (locals.var_weff * locals.var_nuends);
        let assign7890_e9710: f64 = (assign7890_e9706 / assign7890_e9709);
        (assign7890_e9710,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7890_e9712;
        locals.var_rend_rv = 0.0;

        let assign7910_e9722: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard172 = assign7910_e9722;
        locals.var_guard172_rv = 0.0;

        let (assign7920_e9749,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && ((locals.var_guard169 != 0.0) && (locals.var_guard168 == 0.0))) && (locals.var_guard172 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7920_e9749;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7930_e9785,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && ((locals.var_guard169 != 0.0) && (locals.var_guard168 == 0.0))) && (locals.var_guard172 == 0.0)) {
        let assign7930_e9777: f64 = (p.p438 * locals.var_weff);
        let assign7930_e9780: f64 = (6.0 * locals.var_nuends);
        let assign7930_e9782: f64 = (assign7930_e9780 * locals.var_dmcgeff);
        let assign7930_e9783: f64 = (assign7930_e9777 / assign7930_e9782);
        (assign7930_e9783,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7930_e9785;
        locals.var_rend_rv = 0.0;

        let (assign7940_e9810,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (!((locals.var_guard168 != 0.0) || (locals.var_guard169 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7940_e9810;
        locals.var_rend_rv = 0.0;

        let assign7950_e9821: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard173 = assign7950_e9821;
        locals.var_guard173_rv = 0.0;

        let assign7960_e9832: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard174 = assign7960_e9832;
        locals.var_guard174_rv = 0.0;

        let assign7970_e9835: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard175 = assign7970_e9835;
        locals.var_guard175_rv = 0.0;

        let (assign7980_e9860,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard173 != 0.0)) && (locals.var_guard175 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7980_e9860;
        locals.var_rend_rv = 0.0;

        let (assign7990_e9892,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && (locals.var_guard173 != 0.0)) && (locals.var_guard175 == 0.0)) {
        let assign7990_e9886: f64 = (p.p438 * locals.var_dmcgeff);
        let assign7990_e9889: f64 = (locals.var_weff * locals.var_nuends);
        let assign7990_e9890: f64 = (assign7990_e9886 / assign7990_e9889);
        (assign7990_e9890,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7990_e9892;
        locals.var_rend_rv = 0.0;

        let assign8010_e9902: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard177 = assign8010_e9902;
        locals.var_guard177_rv = 0.0;

        let (assign8020_e9930,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && ((locals.var_guard174 != 0.0) && (locals.var_guard173 == 0.0))) && (locals.var_guard177 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8020_e9930;
        locals.var_rend_rv = 0.0;

        let (assign8030_e9967,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && ((locals.var_guard174 != 0.0) && (locals.var_guard173 == 0.0))) && (locals.var_guard177 == 0.0)) {
        let assign8030_e9959: f64 = (p.p438 * locals.var_weff);
        let assign8030_e9962: f64 = (6.0 * locals.var_nuends);
        let assign8030_e9964: f64 = (assign8030_e9962 * locals.var_dmcgeff);
        let assign8030_e9965: f64 = (assign8030_e9959 / assign8030_e9964);
        (assign8030_e9965,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8030_e9967;
        locals.var_rend_rv = 0.0;

        let (assign8040_e9993,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 == 0.0)) && (!((locals.var_guard173 != 0.0) || (locals.var_guard174 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8040_e9993;
        locals.var_rend_rv = 0.0;

        let assign8050_e9996: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard178 = assign8050_e9996;
        locals.var_guard178_rv = 0.0;

        let assign8060_e10007: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard179 = assign8060_e10007;
        locals.var_guard179_rv = 0.0;

        let assign8070_e10018: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard180 = assign8070_e10018;
        locals.var_guard180_rv = 0.0;

        let assign8080_e10021: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard181 = assign8080_e10021;
        locals.var_guard181_rv = 0.0;

        let (assign8090_e10046,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard179 != 0.0)) && (locals.var_guard181 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8090_e10046;
        locals.var_rend_rv = 0.0;

        let (assign8100_e10078,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard179 != 0.0)) && (locals.var_guard181 == 0.0)) {
        let assign8100_e10072: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8100_e10075: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8100_e10076: f64 = (assign8100_e10072 / assign8100_e10075);
        (assign8100_e10076,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8100_e10078;
        locals.var_rend_rv = 0.0;

        let assign8120_e10088: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard183 = assign8120_e10088;
        locals.var_guard183_rv = 0.0;

        let (assign8130_e10116,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && ((locals.var_guard180 != 0.0) && (locals.var_guard179 == 0.0))) && (locals.var_guard183 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8130_e10116;
        locals.var_rend_rv = 0.0;

        let (assign8140_e10153,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && ((locals.var_guard180 != 0.0) && (locals.var_guard179 == 0.0))) && (locals.var_guard183 == 0.0)) {
        let assign8140_e10145: f64 = (p.p438 * locals.var_weff);
        let assign8140_e10148: f64 = (6.0 * locals.var_nuendd);
        let assign8140_e10150: f64 = (assign8140_e10148 * locals.var_dmcgeff);
        let assign8140_e10151: f64 = (assign8140_e10145 / assign8140_e10150);
        (assign8140_e10151,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8140_e10153;
        locals.var_rend_rv = 0.0;

        let (assign8150_e10179,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 != 0.0)) && (!((locals.var_guard179 != 0.0) || (locals.var_guard180 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8150_e10179;
        locals.var_rend_rv = 0.0;

        let assign8160_e10190: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard184 = assign8160_e10190;
        locals.var_guard184_rv = 0.0;

        let assign8170_e10201: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard185 = assign8170_e10201;
        locals.var_guard185_rv = 0.0;

        let assign8180_e10204: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign8180_e10204;
        locals.var_guard186_rv = 0.0;

        let (assign8190_e10230,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && (locals.var_guard184 != 0.0)) && (locals.var_guard186 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8190_e10230;
        locals.var_rend_rv = 0.0;

        let (assign8200_e10263,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && (locals.var_guard184 != 0.0)) && (locals.var_guard186 == 0.0)) {
        let assign8200_e10257: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8200_e10260: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8200_e10261: f64 = (assign8200_e10257 / assign8200_e10260);
        (assign8200_e10261,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8200_e10263;
        locals.var_rend_rv = 0.0;

        let assign8220_e10273: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard188 = assign8220_e10273;
        locals.var_guard188_rv = 0.0;

        let (assign8230_e10302,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && ((locals.var_guard185 != 0.0) && (locals.var_guard184 == 0.0))) && (locals.var_guard188 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8230_e10302;
        locals.var_rend_rv = 0.0;

        let (assign8240_e10340,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && ((locals.var_guard185 != 0.0) && (locals.var_guard184 == 0.0))) && (locals.var_guard188 == 0.0)) {
        let assign8240_e10332: f64 = (p.p438 * locals.var_weff);
        let assign8240_e10335: f64 = (6.0 * locals.var_nuendd);
        let assign8240_e10337: f64 = (assign8240_e10335 * locals.var_dmcgeff);
        let assign8240_e10338: f64 = (assign8240_e10332 / assign8240_e10337);
        (assign8240_e10338,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8240_e10340;
        locals.var_rend_rv = 0.0;

        let (assign8250_e10367,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard89 != 0.0) && (!(((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard166 == 0.0)) && (locals.var_guard178 == 0.0)) && (!((locals.var_guard184 != 0.0) || (locals.var_guard185 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8250_e10367;
        locals.var_rend_rv = 0.0;

        let assign8260_e10370: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign8260_e10370;
        locals.var_guard189_rv = 0.0;

        let assign8270_e10373: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign8270_e10373;
        locals.var_guard190_rv = 0.0;

        let assign8280_e10384: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard191 = assign8280_e10384;
        locals.var_guard191_rv = 0.0;

        let assign8290_e10395: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard192 = assign8290_e10395;
        locals.var_guard192_rv = 0.0;

        let assign8300_e10398: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign8300_e10398;
        locals.var_guard193_rv = 0.0;

        let (assign8310_e10424,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard191 != 0.0)) && (locals.var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8310_e10424;
        locals.var_rend_rv = 0.0;

        let (assign8320_e10457,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard191 != 0.0)) && (locals.var_guard193 == 0.0)) {
        let assign8320_e10451: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8320_e10454: f64 = (locals.var_weff * locals.var_nuends);
        let assign8320_e10455: f64 = (assign8320_e10451 / assign8320_e10454);
        (assign8320_e10455,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8320_e10457;
        locals.var_rend_rv = 0.0;

        let assign8340_e10468: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8340_e10471: f64 = if ((locals.var_nuends == 0.0) || (assign8340_e10468 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard195 = assign8340_e10471;
        locals.var_guard195_rv = 0.0;

        let (assign8350_e10500,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && ((locals.var_guard192 != 0.0) && (locals.var_guard191 == 0.0))) && (locals.var_guard195 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8350_e10500;
        locals.var_rend_rv = 0.0;

        let (assign8360_e10540,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && ((locals.var_guard192 != 0.0) && (locals.var_guard191 == 0.0))) && (locals.var_guard195 == 0.0)) {
        let assign8360_e10530: f64 = (p.p438 * locals.var_weff);
        let assign8360_e10533: f64 = (3.0 * locals.var_nuends);
        let assign8360_e10536: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8360_e10537: f64 = (assign8360_e10533 * assign8360_e10536);
        let assign8360_e10538: f64 = (assign8360_e10530 / assign8360_e10537);
        (assign8360_e10538,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8360_e10540;
        locals.var_rend_rv = 0.0;

        let (assign8370_e10567,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (!((locals.var_guard191 != 0.0) || (locals.var_guard192 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8370_e10567;
        locals.var_rend_rv = 0.0;

        let assign8380_e10578: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard196 = assign8380_e10578;
        locals.var_guard196_rv = 0.0;

        let assign8390_e10589: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard197 = assign8390_e10589;
        locals.var_guard197_rv = 0.0;

        let assign8400_e10592: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign8400_e10592;
        locals.var_guard198_rv = 0.0;

        let (assign8410_e10619,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && (locals.var_guard196 != 0.0)) && (locals.var_guard198 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8410_e10619;
        locals.var_rend_rv = 0.0;

        let (assign8420_e10653,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && (locals.var_guard196 != 0.0)) && (locals.var_guard198 == 0.0)) {
        let assign8420_e10647: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8420_e10650: f64 = (locals.var_weff * locals.var_nuends);
        let assign8420_e10651: f64 = (assign8420_e10647 / assign8420_e10650);
        (assign8420_e10651,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8420_e10653;
        locals.var_rend_rv = 0.0;

        let assign8440_e10664: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8440_e10667: f64 = if ((locals.var_nuends == 0.0) || (assign8440_e10664 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard200 = assign8440_e10667;
        locals.var_guard200_rv = 0.0;

        let (assign8450_e10697,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && ((locals.var_guard197 != 0.0) && (locals.var_guard196 == 0.0))) && (locals.var_guard200 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8450_e10697;
        locals.var_rend_rv = 0.0;

        let (assign8460_e10738,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && ((locals.var_guard197 != 0.0) && (locals.var_guard196 == 0.0))) && (locals.var_guard200 == 0.0)) {
        let assign8460_e10728: f64 = (p.p438 * locals.var_weff);
        let assign8460_e10731: f64 = (3.0 * locals.var_nuends);
        let assign8460_e10734: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8460_e10735: f64 = (assign8460_e10731 * assign8460_e10734);
        let assign8460_e10736: f64 = (assign8460_e10728 / assign8460_e10735);
        (assign8460_e10736,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8460_e10738;
        locals.var_rend_rv = 0.0;

        let (assign8470_e10766,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 == 0.0)) && (!((locals.var_guard196 != 0.0) || (locals.var_guard197 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8470_e10766;
        locals.var_rend_rv = 0.0;

        let (assign8480_e10791,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard90 != 0.0) && (!((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard189 == 0.0)) {
        let assign8480_e10787: f64 = (p.p438 * locals.var_dmdgeff);
        let assign8480_e10789: f64 = (assign8480_e10787 / locals.var_weff);
        (assign8480_e10789,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8480_e10791;
        locals.var_rend_rv = 0.0;

        let assign8490_e10794: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign8490_e10794;
        locals.var_guard201_rv = 0.0;

        let assign8500_e10797: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard202 = assign8500_e10797;
        locals.var_guard202_rv = 0.0;

        let assign8510_e10808: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard203 = assign8510_e10808;
        locals.var_guard203_rv = 0.0;

        let assign8520_e10819: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard204 = assign8520_e10819;
        locals.var_guard204_rv = 0.0;

        let assign8530_e10822: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard205 = assign8530_e10822;
        locals.var_guard205_rv = 0.0;

        let (assign8540_e10850,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard203 != 0.0)) && (locals.var_guard205 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8540_e10850;
        locals.var_rend_rv = 0.0;

        let (assign8550_e10885,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard203 != 0.0)) && (locals.var_guard205 == 0.0)) {
        let assign8550_e10879: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8550_e10882: f64 = (locals.var_weff * locals.var_nuends);
        let assign8550_e10883: f64 = (assign8550_e10879 / assign8550_e10882);
        (assign8550_e10883,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8550_e10885;
        locals.var_rend_rv = 0.0;

        let assign8570_e10895: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard207 = assign8570_e10895;
        locals.var_guard207_rv = 0.0;

        let (assign8580_e10926,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && ((locals.var_guard204 != 0.0) && (locals.var_guard203 == 0.0))) && (locals.var_guard207 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8580_e10926;
        locals.var_rend_rv = 0.0;

        let (assign8590_e10966,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && ((locals.var_guard204 != 0.0) && (locals.var_guard203 == 0.0))) && (locals.var_guard207 == 0.0)) {
        let assign8590_e10958: f64 = (p.p438 * locals.var_weff);
        let assign8590_e10961: f64 = (6.0 * locals.var_nuends);
        let assign8590_e10963: f64 = (assign8590_e10961 * locals.var_dmcgeff);
        let assign8590_e10964: f64 = (assign8590_e10958 / assign8590_e10963);
        (assign8590_e10964,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8590_e10966;
        locals.var_rend_rv = 0.0;

        let (assign8600_e10995,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (!((locals.var_guard203 != 0.0) || (locals.var_guard204 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8600_e10995;
        locals.var_rend_rv = 0.0;

        let assign8610_e11006: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard208 = assign8610_e11006;
        locals.var_guard208_rv = 0.0;

        let assign8620_e11017: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard209 = assign8620_e11017;
        locals.var_guard209_rv = 0.0;

        let assign8630_e11020: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard210 = assign8630_e11020;
        locals.var_guard210_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8640_e11049,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && (locals.var_guard208 != 0.0)) && (locals.var_guard210 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8640_e11049;
        locals.var_rend_rv = 0.0;

        let (assign8650_e11085,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && (locals.var_guard208 != 0.0)) && (locals.var_guard210 == 0.0)) {
        let assign8650_e11079: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8650_e11082: f64 = (locals.var_weff * locals.var_nuends);
        let assign8650_e11083: f64 = (assign8650_e11079 / assign8650_e11082);
        (assign8650_e11083,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8650_e11085;
        locals.var_rend_rv = 0.0;

        let assign8670_e11095: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard212 = assign8670_e11095;
        locals.var_guard212_rv = 0.0;

        let (assign8680_e11127,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && ((locals.var_guard209 != 0.0) && (locals.var_guard208 == 0.0))) && (locals.var_guard212 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8680_e11127;
        locals.var_rend_rv = 0.0;

        let (assign8690_e11168,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && ((locals.var_guard209 != 0.0) && (locals.var_guard208 == 0.0))) && (locals.var_guard212 == 0.0)) {
        let assign8690_e11160: f64 = (p.p438 * locals.var_weff);
        let assign8690_e11163: f64 = (6.0 * locals.var_nuends);
        let assign8690_e11165: f64 = (assign8690_e11163 * locals.var_dmcgeff);
        let assign8690_e11166: f64 = (assign8690_e11160 / assign8690_e11165);
        (assign8690_e11166,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8690_e11168;
        locals.var_rend_rv = 0.0;

        let (assign8700_e11198,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 == 0.0)) && (!((locals.var_guard208 != 0.0) || (locals.var_guard209 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8700_e11198;
        locals.var_rend_rv = 0.0;

        let assign8710_e11201: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign8710_e11201;
        locals.var_guard213_rv = 0.0;

        let (assign8720_e11226,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 == 0.0)) && (locals.var_guard213 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8720_e11226;
        locals.var_rend_rv = 0.0;

        let (assign8730_e11258,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard91 != 0.0) && (!(((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard201 == 0.0)) && (locals.var_guard213 == 0.0)) {
        let assign8730_e11252: f64 = (p.p438 * locals.var_dmdgeff);
        let assign8730_e11255: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8730_e11256: f64 = (assign8730_e11252 / assign8730_e11255);
        (assign8730_e11256,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8730_e11258;
        locals.var_rend_rv = 0.0;

        let assign8740_e11261: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign8740_e11261;
        locals.var_guard214_rv = 0.0;

        let (assign8750_e11289,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 != 0.0)) {
        let assign8750_e11285: f64 = (p.p438 * locals.var_dmdgeff);
        let assign8750_e11287: f64 = (assign8750_e11285 / locals.var_weff);
        (assign8750_e11287,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8750_e11289;
        locals.var_rend_rv = 0.0;

        let assign8760_e11292: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard215 = assign8760_e11292;
        locals.var_guard215_rv = 0.0;

        let assign8770_e11303: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard216 = assign8770_e11303;
        locals.var_guard216_rv = 0.0;

        let assign8780_e11314: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard217 = assign8780_e11314;
        locals.var_guard217_rv = 0.0;

        let assign8790_e11317: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard218 = assign8790_e11317;
        locals.var_guard218_rv = 0.0;

        let (assign8800_e11348,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard216 != 0.0)) && (locals.var_guard218 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8800_e11348;
        locals.var_rend_rv = 0.0;

        let (assign8810_e11386,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard216 != 0.0)) && (locals.var_guard218 == 0.0)) {
        let assign8810_e11380: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8810_e11383: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8810_e11384: f64 = (assign8810_e11380 / assign8810_e11383);
        (assign8810_e11384,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8810_e11386;
        locals.var_rend_rv = 0.0;

        let assign8830_e11397: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8830_e11400: f64 = if ((locals.var_nuendd == 0.0) || (assign8830_e11397 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard220 = assign8830_e11400;
        locals.var_guard220_rv = 0.0;

        let (assign8840_e11434,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && ((locals.var_guard217 != 0.0) && (locals.var_guard216 == 0.0))) && (locals.var_guard220 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8840_e11434;
        locals.var_rend_rv = 0.0;

        let (assign8850_e11479,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && ((locals.var_guard217 != 0.0) && (locals.var_guard216 == 0.0))) && (locals.var_guard220 == 0.0)) {
        let assign8850_e11469: f64 = (p.p438 * locals.var_weff);
        let assign8850_e11472: f64 = (3.0 * locals.var_nuendd);
        let assign8850_e11475: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8850_e11476: f64 = (assign8850_e11472 * assign8850_e11475);
        let assign8850_e11477: f64 = (assign8850_e11469 / assign8850_e11476);
        (assign8850_e11477,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8850_e11479;
        locals.var_rend_rv = 0.0;

        let (assign8860_e11511,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 != 0.0)) && (!((locals.var_guard216 != 0.0) || (locals.var_guard217 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8860_e11511;
        locals.var_rend_rv = 0.0;

        let assign8870_e11522: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard221 = assign8870_e11522;
        locals.var_guard221_rv = 0.0;

        let assign8880_e11533: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard222 = assign8880_e11533;
        locals.var_guard222_rv = 0.0;

        let assign8890_e11536: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign8890_e11536;
        locals.var_guard223_rv = 0.0;

        let (assign8900_e11568,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && (locals.var_guard221 != 0.0)) && (locals.var_guard223 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8900_e11568;
        locals.var_rend_rv = 0.0;

        let (assign8910_e11607,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && (locals.var_guard221 != 0.0)) && (locals.var_guard223 == 0.0)) {
        let assign8910_e11601: f64 = (p.p438 * locals.var_dmcgeff);
        let assign8910_e11604: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8910_e11605: f64 = (assign8910_e11601 / assign8910_e11604);
        (assign8910_e11605,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8910_e11607;
        locals.var_rend_rv = 0.0;

        let assign8930_e11618: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8930_e11621: f64 = if ((locals.var_nuendd == 0.0) || (assign8930_e11618 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard225 = assign8930_e11621;
        locals.var_guard225_rv = 0.0;

        let (assign8940_e11656,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && ((locals.var_guard222 != 0.0) && (locals.var_guard221 == 0.0))) && (locals.var_guard225 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8940_e11656;
        locals.var_rend_rv = 0.0;

        let (assign8950_e11702,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && ((locals.var_guard222 != 0.0) && (locals.var_guard221 == 0.0))) && (locals.var_guard225 == 0.0)) {
        let assign8950_e11692: f64 = (p.p438 * locals.var_weff);
        let assign8950_e11695: f64 = (3.0 * locals.var_nuendd);
        let assign8950_e11698: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8950_e11699: f64 = (assign8950_e11695 * assign8950_e11698);
        let assign8950_e11700: f64 = (assign8950_e11692 / assign8950_e11699);
        (assign8950_e11700,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8950_e11702;
        locals.var_rend_rv = 0.0;

        let (assign8960_e11735,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard92 != 0.0) && (!((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard214 == 0.0)) && (locals.var_guard215 == 0.0)) && (!((locals.var_guard221 != 0.0) || (locals.var_guard222 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8960_e11735;
        locals.var_rend_rv = 0.0;

        let assign8970_e11738: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign8970_e11738;
        locals.var_guard226_rv = 0.0;

        let assign8980_e11741: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign8980_e11741;
        locals.var_guard227_rv = 0.0;

        let (assign8990_e11769,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 != 0.0)) && (locals.var_guard227 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8990_e11769;
        locals.var_rend_rv = 0.0;

        let (assign9000_e11804,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 != 0.0)) && (locals.var_guard227 == 0.0)) {
        let assign9000_e11798: f64 = (p.p438 * locals.var_dmdgeff);
        let assign9000_e11801: f64 = (locals.var_weff * locals.var_nuends);
        let assign9000_e11802: f64 = (assign9000_e11798 / assign9000_e11801);
        (assign9000_e11802,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9000_e11804;
        locals.var_rend_rv = 0.0;

        let assign9010_e11807: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard228 = assign9010_e11807;
        locals.var_guard228_rv = 0.0;

        let assign9020_e11818: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign9020_e11818;
        locals.var_guard229_rv = 0.0;

        let assign9030_e11829: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard230 = assign9030_e11829;
        locals.var_guard230_rv = 0.0;

        let assign9040_e11832: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard231 = assign9040_e11832;
        locals.var_guard231_rv = 0.0;

        let (assign9050_e11865,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard229 != 0.0)) && (locals.var_guard231 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9050_e11865;
        locals.var_rend_rv = 0.0;

        let (assign9060_e11905,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard229 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign9060_e11899: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9060_e11902: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9060_e11903: f64 = (assign9060_e11899 / assign9060_e11902);
        (assign9060_e11903,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9060_e11905;
        locals.var_rend_rv = 0.0;

        let assign9080_e11915: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign9080_e11915;
        locals.var_guard233_rv = 0.0;

        let (assign9090_e11951,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && ((locals.var_guard230 != 0.0) && (locals.var_guard229 == 0.0))) && (locals.var_guard233 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9090_e11951;
        locals.var_rend_rv = 0.0;

        let (assign9100_e11996,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && ((locals.var_guard230 != 0.0) && (locals.var_guard229 == 0.0))) && (locals.var_guard233 == 0.0)) {
        let assign9100_e11988: f64 = (p.p438 * locals.var_weff);
        let assign9100_e11991: f64 = (6.0 * locals.var_nuendd);
        let assign9100_e11993: f64 = (assign9100_e11991 * locals.var_dmcgeff);
        let assign9100_e11994: f64 = (assign9100_e11988 / assign9100_e11993);
        (assign9100_e11994,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9100_e11996;
        locals.var_rend_rv = 0.0;

        let (assign9110_e12030,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 != 0.0)) && (!((locals.var_guard229 != 0.0) || (locals.var_guard230 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9110_e12030;
        locals.var_rend_rv = 0.0;

        let assign9120_e12041: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard234 = assign9120_e12041;
        locals.var_guard234_rv = 0.0;

        let assign9130_e12052: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard235 = assign9130_e12052;
        locals.var_guard235_rv = 0.0;

        let assign9140_e12055: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard236 = assign9140_e12055;
        locals.var_guard236_rv = 0.0;

        let (assign9150_e12089,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && (locals.var_guard234 != 0.0)) && (locals.var_guard236 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9150_e12089;
        locals.var_rend_rv = 0.0;

        let (assign9160_e12130,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && (locals.var_guard234 != 0.0)) && (locals.var_guard236 == 0.0)) {
        let assign9160_e12124: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9160_e12127: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9160_e12128: f64 = (assign9160_e12124 / assign9160_e12127);
        (assign9160_e12128,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9160_e12130;
        locals.var_rend_rv = 0.0;

        let assign9180_e12140: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard238 = assign9180_e12140;
        locals.var_guard238_rv = 0.0;

        let (assign9190_e12177,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && ((locals.var_guard235 != 0.0) && (locals.var_guard234 == 0.0))) && (locals.var_guard238 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9190_e12177;
        locals.var_rend_rv = 0.0;

        let (assign9200_e12223,) = {
    if (((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && ((locals.var_guard235 != 0.0) && (locals.var_guard234 == 0.0))) && (locals.var_guard238 == 0.0)) {
        let assign9200_e12215: f64 = (p.p438 * locals.var_weff);
        let assign9200_e12218: f64 = (6.0 * locals.var_nuendd);
        let assign9200_e12220: f64 = (assign9200_e12218 * locals.var_dmcgeff);
        let assign9200_e12221: f64 = (assign9200_e12215 / assign9200_e12220);
        (assign9200_e12221,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9200_e12223;
        locals.var_rend_rv = 0.0;

        let (assign9210_e12258,) = {
    if ((((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard93 != 0.0) && (!(((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) && (locals.var_guard226 == 0.0)) && (locals.var_guard228 == 0.0)) && (!((locals.var_guard234 != 0.0) || (locals.var_guard235 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9210_e12258;
        locals.var_rend_rv = 0.0;

        let (assign9220_e12288,) = {
    if (((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard94 != 0.0) && (!((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) {
        let assign9220_e12284: f64 = (p.p438 * locals.var_dmdgeff);
        let assign9220_e12286: f64 = (assign9220_e12284 / locals.var_weff);
        (assign9220_e12286,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9220_e12288;
        locals.var_rend_rv = 0.0;

        let assign9230_e12291: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign9230_e12291;
        locals.var_guard239_rv = 0.0;

        let (assign9240_e12327,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 != 0.0)) {
        let assign9240_e12321: f64 = (0.5 * p.p438);
        let assign9240_e12323: f64 = (assign9240_e12321 * locals.var_dmcgeff);
        let assign9240_e12325: f64 = (assign9240_e12323 / locals.var_weff);
        (assign9240_e12325,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9240_e12327;
        locals.var_rend_rv = 0.0;

        let assign9250_e12330: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign9250_e12330;
        locals.var_guard240_rv = 0.0;

        let (assign9260_e12362,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 != 0.0)) && (locals.var_guard240 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9260_e12362;
        locals.var_rint_rv = 0.0;

        let (assign9270_e12403,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 != 0.0)) && (locals.var_guard240 == 0.0)) {
        let assign9270_e12395: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9270_e12399: f64 = (p.p2 - 2.0);
        let assign9270_e12400: f64 = (locals.var_weff * assign9270_e12399);
        let assign9270_e12401: f64 = (assign9270_e12395 / assign9270_e12400);
        (assign9270_e12401,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9270_e12403;
        locals.var_rint_rv = 0.0;

        let (assign9280_e12434,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9280_e12434;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign9290_e12471,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard95 != 0.0) && (!(((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard239 == 0.0)) {
        let assign9290_e12465: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9290_e12468: f64 = (locals.var_weff * p.p2);
        let assign9290_e12469: f64 = (assign9290_e12465 / assign9290_e12468);
        (assign9290_e12469,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9290_e12471;
        locals.var_rint_rv = 0.0;

        let assign9300_e12474: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign9300_e12474;
        locals.var_guard241_rv = 0.0;

        let (assign9310_e12506,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9310_e12506;
        locals.var_rend_rv = 0.0;

        let (assign9320_e12544,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 != 0.0)) {
        let assign9320_e12538: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9320_e12541: f64 = (locals.var_weff * p.p2);
        let assign9320_e12542: f64 = (assign9320_e12538 / assign9320_e12541);
        (assign9320_e12542,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9320_e12544;
        locals.var_rint_rv = 0.0;

        let (assign9330_e12583,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 == 0.0)) {
        let assign9330_e12577: f64 = (0.5 * p.p438);
        let assign9330_e12579: f64 = (assign9330_e12577 * locals.var_dmcgeff);
        let assign9330_e12581: f64 = (assign9330_e12579 / locals.var_weff);
        (assign9330_e12581,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9330_e12583;
        locals.var_rend_rv = 0.0;

        let assign9340_e12586: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign9340_e12586;
        locals.var_guard242_rv = 0.0;

        let (assign9350_e12621,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9350_e12621;
        locals.var_rint_rv = 0.0;

        let (assign9360_e12665,) = {
    if (((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && ((locals.var_guard96 != 0.0) && (!((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0))))) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 == 0.0)) {
        let assign9360_e12657: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9360_e12661: f64 = (p.p2 - 2.0);
        let assign9360_e12662: f64 = (locals.var_weff * assign9360_e12661);
        let assign9360_e12663: f64 = (assign9360_e12657 / assign9360_e12662);
        (assign9360_e12663,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9360_e12665;
        locals.var_rint_rv = 0.0;

        let (assign9370_e12695,) = {
    if (((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (!(((((((((((locals.var_guard86 != 0.0) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0)) || (locals.var_guard96 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9370_e12695;
        locals.var_rint_rv = 0.0;

        let assign9380_e12698: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign9380_e12698;
        locals.var_guard243_rv = 0.0;

        let (assign9390_e12707,) = {
    if (((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard243 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9390_e12707;
        locals.var_rsourcegeo_rv = 0.0;

        let assign9400_e12710: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign9400_e12710;
        locals.var_guard244_rv = 0.0;

        let (assign9410_e12722,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9410_e12722;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign9420_e12741,) = {
    if ((((locals.var_guard78 == 0.0) && (locals.var_guard79 != 0.0)) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 == 0.0)) {
        let assign9420_e12735: f64 = (locals.var_rint * locals.var_rend);
        let assign9420_e12738: f64 = (locals.var_rint + locals.var_rend);
        let assign9420_e12739: f64 = (assign9420_e12735 / assign9420_e12738);
        (assign9420_e12739,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9420_e12741;
        locals.var_rsourcegeo_rv = 0.0;

        let (assign9440_e12752,) = {
    if ((locals.var_guard78 == 0.0) && (locals.var_guard79 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign9440_e12752;
        locals.var_rsourcegeo_rv = 0.0;

        let assign9450_e12754: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard246 = assign9450_e12754;
        locals.var_guard246_rv = 0.0;

        let (assign9460_e12760,) = {
    if (locals.var_guard246 != 0.0) {
        let assign9460_e12758: f64 = (p.p438 * p.p4);
        (assign9460_e12758,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign9460_e12760;
        locals.var_rdraingeo_rv = 0.0;

        let assign9470_e12767: f64 = if ((p.p9 > 0.0) && (p.p438 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard247 = assign9470_e12767;
        locals.var_guard247_rv = 0.0;

        let assign9480_e12770: f64 = if p.p8 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign9480_e12770;
        locals.var_guard248_rv = 0.0;

        let assign9490_e12773: f64 = (p.p2 % 2.0);
        let assign9490_e12775: f64 = if assign9490_e12773 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign9490_e12775;
        locals.var_guard249_rv = 0.0;

        let (assign9500_e12786,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign9500_e12786;
        locals.var_nuendd_rv = 0.0;

        let (assign9510_e12797,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign9510_e12797;
        locals.var_nuends_rv = 0.0;

        let (assign9520_e12816,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign9520_e12809: f64 = (p.p2 - 1.0);
        let assign9520_e12811: f64 = (assign9520_e12809 / 2.0);
        let assign9520_e12813: f64 = (assign9520_e12811).max(0.0);
        let assign9520_e12814: f64 = (2.0 * assign9520_e12813);
        (assign9520_e12814,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign9520_e12816;
        locals.var_nuintd_rv = 0.0;

        let (assign9530_e12827,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign9530_e12827;
        locals.var_nuints_rv = 0.0;

        let assign9540_e12830: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign9540_e12830;
        locals.var_guard250_rv = 0.0;

        let (assign9550_e12844,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign9550_e12844;
        locals.var_nuendd_rv = 0.0;

        let (assign9560_e12866,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign9560_e12859: f64 = (p.p2 / 2.0);
        let assign9560_e12861: f64 = (assign9560_e12859 - 1.0);
        let assign9560_e12863: f64 = (assign9560_e12861).max(0.0);
        let assign9560_e12864: f64 = (2.0 * assign9560_e12863);
        (assign9560_e12864,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign9560_e12866;
        locals.var_nuintd_rv = 0.0;

        let (assign9570_e12880,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign9570_e12880;
        locals.var_nuends_rv = 0.0;

        let (assign9580_e12894,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign9580_e12894;
        locals.var_nuints_rv = 0.0;

        let (assign9590_e12909,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign9590_e12909;
        locals.var_nuendd_rv = 0.0;

        let (assign9600_e12924,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign9600_e12924;
        locals.var_nuintd_rv = 0.0;

        let (assign9610_e12939,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign9610_e12939;
        locals.var_nuends_rv = 0.0;

        let (assign9620_e12962,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign9620_e12955: f64 = (p.p2 / 2.0);
        let assign9620_e12957: f64 = (assign9620_e12955 - 1.0);
        let assign9620_e12959: f64 = (assign9620_e12957).max(0.0);
        let assign9620_e12960: f64 = (2.0 * assign9620_e12959);
        (assign9620_e12960,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign9620_e12962;
        locals.var_nuints_rv = 0.0;

        let assign9630_e12965: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign9630_e12965;
        locals.var_guard251_rv = 0.0;

        let assign9640_e12968: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign9640_e12968;
        locals.var_guard252_rv = 0.0;

        let (assign9650_e12981,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 != 0.0)) && (locals.var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9650_e12981;
        locals.var_rint_rv = 0.0;

        let (assign9660_e13001,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 != 0.0)) && (locals.var_guard252 == 0.0)) {
        let assign9660_e12995: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9660_e12998: f64 = (locals.var_weff * locals.var_nuints);
        let assign9660_e12999: f64 = (assign9660_e12995 / assign9660_e12998);
        (assign9660_e12999,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9660_e13001;
        locals.var_rint_rv = 0.0;

        let assign9670_e13004: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign9670_e13004;
        locals.var_guard253_rv = 0.0;

        let (assign9680_e13018,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 == 0.0)) && (locals.var_guard253 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9680_e13018;
        locals.var_rint_rv = 0.0;

        let (assign9690_e13039,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) && (locals.var_guard251 == 0.0)) && (locals.var_guard253 == 0.0)) {
        let assign9690_e13033: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9690_e13036: f64 = (locals.var_weff * locals.var_nuintd);
        let assign9690_e13037: f64 = (assign9690_e13033 / assign9690_e13036);
        (assign9690_e13037,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign9690_e13039;
        locals.var_rint_rv = 0.0;

        let assign9700_e13042: f64 = if p.p8 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign9700_e13042;
        locals.var_guard254_rv = 0.0;

        let assign9710_e13045: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign9710_e13045;
        locals.var_guard255_rv = 0.0;

        let assign9720_e13048: f64 = if p.p8 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign9720_e13048;
        locals.var_guard256_rv = 0.0;

        let assign9730_e13051: f64 = if p.p8 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign9730_e13051;
        locals.var_guard257_rv = 0.0;

        let assign9740_e13054: f64 = if p.p8 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign9740_e13054;
        locals.var_guard258_rv = 0.0;

        let assign9750_e13057: f64 = if p.p8 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign9750_e13057;
        locals.var_guard259_rv = 0.0;

        let assign9760_e13060: f64 = if p.p8 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign9760_e13060;
        locals.var_guard260_rv = 0.0;

        let assign9770_e13063: f64 = if p.p8 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign9770_e13063;
        locals.var_guard261_rv = 0.0;

        let assign9780_e13066: f64 = if p.p8 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign9780_e13066;
        locals.var_guard262_rv = 0.0;

        let assign9790_e13069: f64 = if p.p8 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9790_e13069;
        locals.var_guard263_rv = 0.0;

        let assign9800_e13072: f64 = if p.p8 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9800_e13072;
        locals.var_guard264_rv = 0.0;

        let assign9810_e13075: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign9810_e13075;
        locals.var_guard265_rv = 0.0;

        let assign9820_e13078: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign9820_e13078;
        locals.var_guard266_rv = 0.0;

        let assign9830_e13089: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard267 = assign9830_e13089;
        locals.var_guard267_rv = 0.0;

        let assign9840_e13100: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard268 = assign9840_e13100;
        locals.var_guard268_rv = 0.0;

        let assign9850_e13103: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign9850_e13103;
        locals.var_guard269_rv = 0.0;

        let (assign9860_e13120,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 != 0.0)) && (locals.var_guard269 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9860_e13120;
        locals.var_rend_rv = 0.0;

        let (assign9870_e13144,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 != 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign9870_e13138: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9870_e13141: f64 = (locals.var_weff * locals.var_nuends);
        let assign9870_e13142: f64 = (assign9870_e13138 / assign9870_e13141);
        (assign9870_e13142,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9870_e13144;
        locals.var_rend_rv = 0.0;

        let assign9890_e13155: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9890_e13158: f64 = if ((locals.var_nuends == 0.0) || (assign9890_e13155 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign9890_e13158;
        locals.var_guard271_rv = 0.0;

        let (assign9900_e13178,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && ((locals.var_guard268 != 0.0) && (locals.var_guard267 == 0.0))) && (locals.var_guard271 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9900_e13178;
        locals.var_rend_rv = 0.0;

        let (assign9910_e13209,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && ((locals.var_guard268 != 0.0) && (locals.var_guard267 == 0.0))) && (locals.var_guard271 == 0.0)) {
        let assign9910_e13199: f64 = (p.p438 * locals.var_weff);
        let assign9910_e13202: f64 = (3.0 * locals.var_nuends);
        let assign9910_e13205: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9910_e13206: f64 = (assign9910_e13202 * assign9910_e13205);
        let assign9910_e13207: f64 = (assign9910_e13199 / assign9910_e13206);
        (assign9910_e13207,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9910_e13209;
        locals.var_rend_rv = 0.0;

        let (assign9920_e13227,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (!((locals.var_guard267 != 0.0) || (locals.var_guard268 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9920_e13227;
        locals.var_rend_rv = 0.0;

        let assign9930_e13238: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign9930_e13238;
        locals.var_guard272_rv = 0.0;

        let assign9940_e13249: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard273 = assign9940_e13249;
        locals.var_guard273_rv = 0.0;

        let assign9950_e13252: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign9950_e13252;
        locals.var_guard274_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9960_e13270,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard272 != 0.0)) && (locals.var_guard274 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9960_e13270;
        locals.var_rend_rv = 0.0;

        let (assign9970_e13295,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard272 != 0.0)) && (locals.var_guard274 == 0.0)) {
        let assign9970_e13289: f64 = (p.p438 * locals.var_dmcgeff);
        let assign9970_e13292: f64 = (locals.var_weff * locals.var_nuends);
        let assign9970_e13293: f64 = (assign9970_e13289 / assign9970_e13292);
        (assign9970_e13293,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9970_e13295;
        locals.var_rend_rv = 0.0;

        let assign9990_e13306: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9990_e13309: f64 = if ((locals.var_nuends == 0.0) || (assign9990_e13306 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard276 = assign9990_e13309;
        locals.var_guard276_rv = 0.0;

        let (assign10000_e13330,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && ((locals.var_guard273 != 0.0) && (locals.var_guard272 == 0.0))) && (locals.var_guard276 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10000_e13330;
        locals.var_rend_rv = 0.0;

        let (assign10010_e13362,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && ((locals.var_guard273 != 0.0) && (locals.var_guard272 == 0.0))) && (locals.var_guard276 == 0.0)) {
        let assign10010_e13352: f64 = (p.p438 * locals.var_weff);
        let assign10010_e13355: f64 = (3.0 * locals.var_nuends);
        let assign10010_e13358: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10010_e13359: f64 = (assign10010_e13355 * assign10010_e13358);
        let assign10010_e13360: f64 = (assign10010_e13352 / assign10010_e13359);
        (assign10010_e13360,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10010_e13362;
        locals.var_rend_rv = 0.0;

        let (assign10020_e13381,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) && (!((locals.var_guard272 != 0.0) || (locals.var_guard273 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10020_e13381;
        locals.var_rend_rv = 0.0;

        let assign10030_e13384: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign10030_e13384;
        locals.var_guard277_rv = 0.0;

        let assign10040_e13395: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign10040_e13395;
        locals.var_guard278_rv = 0.0;

        let assign10050_e13406: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard279 = assign10050_e13406;
        locals.var_guard279_rv = 0.0;

        let assign10060_e13409: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign10060_e13409;
        locals.var_guard280_rv = 0.0;

        let (assign10070_e13427,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard280 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10070_e13427;
        locals.var_rend_rv = 0.0;

        let (assign10080_e13452,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard280 == 0.0)) {
        let assign10080_e13446: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10080_e13449: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10080_e13450: f64 = (assign10080_e13446 / assign10080_e13449);
        (assign10080_e13450,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10080_e13452;
        locals.var_rend_rv = 0.0;

        let assign10100_e13463: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10100_e13466: f64 = if ((locals.var_nuendd == 0.0) || (assign10100_e13463 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign10100_e13466;
        locals.var_guard282_rv = 0.0;

        let (assign10110_e13487,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && ((locals.var_guard279 != 0.0) && (locals.var_guard278 == 0.0))) && (locals.var_guard282 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10110_e13487;
        locals.var_rend_rv = 0.0;

        let (assign10120_e13519,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && ((locals.var_guard279 != 0.0) && (locals.var_guard278 == 0.0))) && (locals.var_guard282 == 0.0)) {
        let assign10120_e13509: f64 = (p.p438 * locals.var_weff);
        let assign10120_e13512: f64 = (3.0 * locals.var_nuendd);
        let assign10120_e13515: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10120_e13516: f64 = (assign10120_e13512 * assign10120_e13515);
        let assign10120_e13517: f64 = (assign10120_e13509 / assign10120_e13516);
        (assign10120_e13517,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10120_e13519;
        locals.var_rend_rv = 0.0;

        let (assign10130_e13538,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 != 0.0)) && (!((locals.var_guard278 != 0.0) || (locals.var_guard279 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10130_e13538;
        locals.var_rend_rv = 0.0;

        let assign10140_e13549: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard283 = assign10140_e13549;
        locals.var_guard283_rv = 0.0;

        let assign10150_e13560: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard284 = assign10150_e13560;
        locals.var_guard284_rv = 0.0;

        let assign10160_e13563: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign10160_e13563;
        locals.var_guard285_rv = 0.0;

        let (assign10170_e13582,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard285 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10170_e13582;
        locals.var_rend_rv = 0.0;

        let (assign10180_e13608,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard285 == 0.0)) {
        let assign10180_e13602: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10180_e13605: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10180_e13606: f64 = (assign10180_e13602 / assign10180_e13605);
        (assign10180_e13606,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10180_e13608;
        locals.var_rend_rv = 0.0;

        let assign10200_e13619: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10200_e13622: f64 = if ((locals.var_nuendd == 0.0) || (assign10200_e13619 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard287 = assign10200_e13622;
        locals.var_guard287_rv = 0.0;

        let (assign10210_e13644,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && ((locals.var_guard284 != 0.0) && (locals.var_guard283 == 0.0))) && (locals.var_guard287 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10210_e13644;
        locals.var_rend_rv = 0.0;

        let (assign10220_e13677,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && ((locals.var_guard284 != 0.0) && (locals.var_guard283 == 0.0))) && (locals.var_guard287 == 0.0)) {
        let assign10220_e13667: f64 = (p.p438 * locals.var_weff);
        let assign10220_e13670: f64 = (3.0 * locals.var_nuendd);
        let assign10220_e13673: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10220_e13674: f64 = (assign10220_e13670 * assign10220_e13673);
        let assign10220_e13675: f64 = (assign10220_e13667 / assign10220_e13674);
        (assign10220_e13675,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10220_e13677;
        locals.var_rend_rv = 0.0;

        let (assign10230_e13697,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard277 == 0.0)) && (!((locals.var_guard283 != 0.0) || (locals.var_guard284 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10230_e13697;
        locals.var_rend_rv = 0.0;

        let assign10240_e13700: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign10240_e13700;
        locals.var_guard288_rv = 0.0;

        let assign10250_e13703: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign10250_e13703;
        locals.var_guard289_rv = 0.0;

        let assign10260_e13714: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign10260_e13714;
        locals.var_guard290_rv = 0.0;

        let assign10270_e13725: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard291 = assign10270_e13725;
        locals.var_guard291_rv = 0.0;

        let assign10280_e13728: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign10280_e13728;
        locals.var_guard292_rv = 0.0;

        let (assign10290_e13748,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard290 != 0.0)) && (locals.var_guard292 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10290_e13748;
        locals.var_rend_rv = 0.0;

        let (assign10300_e13775,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard290 != 0.0)) && (locals.var_guard292 == 0.0)) {
        let assign10300_e13769: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10300_e13772: f64 = (locals.var_weff * locals.var_nuends);
        let assign10300_e13773: f64 = (assign10300_e13769 / assign10300_e13772);
        (assign10300_e13773,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10300_e13775;
        locals.var_rend_rv = 0.0;

        let assign10320_e13786: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10320_e13789: f64 = if ((locals.var_nuends == 0.0) || (assign10320_e13786 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard294 = assign10320_e13789;
        locals.var_guard294_rv = 0.0;

        let (assign10330_e13812,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && ((locals.var_guard291 != 0.0) && (locals.var_guard290 == 0.0))) && (locals.var_guard294 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10330_e13812;
        locals.var_rend_rv = 0.0;

        let (assign10340_e13846,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && ((locals.var_guard291 != 0.0) && (locals.var_guard290 == 0.0))) && (locals.var_guard294 == 0.0)) {
        let assign10340_e13836: f64 = (p.p438 * locals.var_weff);
        let assign10340_e13839: f64 = (3.0 * locals.var_nuends);
        let assign10340_e13842: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10340_e13843: f64 = (assign10340_e13839 * assign10340_e13842);
        let assign10340_e13844: f64 = (assign10340_e13836 / assign10340_e13843);
        (assign10340_e13844,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10340_e13846;
        locals.var_rend_rv = 0.0;

        let (assign10350_e13867,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (!((locals.var_guard290 != 0.0) || (locals.var_guard291 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10350_e13867;
        locals.var_rend_rv = 0.0;

        let assign10360_e13878: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard295 = assign10360_e13878;
        locals.var_guard295_rv = 0.0;

        let assign10370_e13889: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard296 = assign10370_e13889;
        locals.var_guard296_rv = 0.0;

        let assign10380_e13892: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign10380_e13892;
        locals.var_guard297_rv = 0.0;

        let (assign10390_e13913,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10390_e13913;
        locals.var_rend_rv = 0.0;

        let (assign10400_e13941,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 == 0.0)) {
        let assign10400_e13935: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10400_e13938: f64 = (locals.var_weff * locals.var_nuends);
        let assign10400_e13939: f64 = (assign10400_e13935 / assign10400_e13938);
        (assign10400_e13939,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10400_e13941;
        locals.var_rend_rv = 0.0;

        let assign10420_e13952: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10420_e13955: f64 = if ((locals.var_nuends == 0.0) || (assign10420_e13952 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard299 = assign10420_e13955;
        locals.var_guard299_rv = 0.0;

        let (assign10430_e13979,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && ((locals.var_guard296 != 0.0) && (locals.var_guard295 == 0.0))) && (locals.var_guard299 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10430_e13979;
        locals.var_rend_rv = 0.0;

        let (assign10440_e14014,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && ((locals.var_guard296 != 0.0) && (locals.var_guard295 == 0.0))) && (locals.var_guard299 == 0.0)) {
        let assign10440_e14004: f64 = (p.p438 * locals.var_weff);
        let assign10440_e14007: f64 = (3.0 * locals.var_nuends);
        let assign10440_e14010: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10440_e14011: f64 = (assign10440_e14007 * assign10440_e14010);
        let assign10440_e14012: f64 = (assign10440_e14004 / assign10440_e14011);
        (assign10440_e14012,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10440_e14014;
        locals.var_rend_rv = 0.0;

        let (assign10450_e14036,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 == 0.0)) && (!((locals.var_guard295 != 0.0) || (locals.var_guard296 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10450_e14036;
        locals.var_rend_rv = 0.0;

        let assign10460_e14039: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign10460_e14039;
        locals.var_guard300_rv = 0.0;

        let assign10470_e14050: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign10470_e14050;
        locals.var_guard301_rv = 0.0;

        let assign10480_e14061: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard302 = assign10480_e14061;
        locals.var_guard302_rv = 0.0;

        let assign10490_e14064: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign10490_e14064;
        locals.var_guard303_rv = 0.0;

        let (assign10500_e14085,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard303 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10500_e14085;
        locals.var_rend_rv = 0.0;

        let (assign10510_e14113,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard303 == 0.0)) {
        let assign10510_e14107: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10510_e14110: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10510_e14111: f64 = (assign10510_e14107 / assign10510_e14110);
        (assign10510_e14111,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10510_e14113;
        locals.var_rend_rv = 0.0;

        let assign10530_e14123: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign10530_e14123;
        locals.var_guard305_rv = 0.0;

        let (assign10540_e14147,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && ((locals.var_guard302 != 0.0) && (locals.var_guard301 == 0.0))) && (locals.var_guard305 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10540_e14147;
        locals.var_rend_rv = 0.0;

        let (assign10550_e14180,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && ((locals.var_guard302 != 0.0) && (locals.var_guard301 == 0.0))) && (locals.var_guard305 == 0.0)) {
        let assign10550_e14172: f64 = (p.p438 * locals.var_weff);
        let assign10550_e14175: f64 = (6.0 * locals.var_nuendd);
        let assign10550_e14177: f64 = (assign10550_e14175 * locals.var_dmcgeff);
        let assign10550_e14178: f64 = (assign10550_e14172 / assign10550_e14177);
        (assign10550_e14178,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10550_e14180;
        locals.var_rend_rv = 0.0;

        let (assign10560_e14202,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 != 0.0)) && (!((locals.var_guard301 != 0.0) || (locals.var_guard302 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10560_e14202;
        locals.var_rend_rv = 0.0;

        let assign10570_e14213: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign10570_e14213;
        locals.var_guard306_rv = 0.0;

        let assign10580_e14224: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard307 = assign10580_e14224;
        locals.var_guard307_rv = 0.0;

        let assign10590_e14227: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign10590_e14227;
        locals.var_guard308_rv = 0.0;

        let (assign10600_e14249,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10600_e14249;
        locals.var_rend_rv = 0.0;

        let (assign10610_e14278,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign10610_e14272: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10610_e14275: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10610_e14276: f64 = (assign10610_e14272 / assign10610_e14275);
        (assign10610_e14276,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10610_e14278;
        locals.var_rend_rv = 0.0;

        let assign10630_e14288: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard310 = assign10630_e14288;
        locals.var_guard310_rv = 0.0;

        let (assign10640_e14313,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && ((locals.var_guard307 != 0.0) && (locals.var_guard306 == 0.0))) && (locals.var_guard310 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10640_e14313;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10650_e14347,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && ((locals.var_guard307 != 0.0) && (locals.var_guard306 == 0.0))) && (locals.var_guard310 == 0.0)) {
        let assign10650_e14339: f64 = (p.p438 * locals.var_weff);
        let assign10650_e14342: f64 = (6.0 * locals.var_nuendd);
        let assign10650_e14344: f64 = (assign10650_e14342 * locals.var_dmcgeff);
        let assign10650_e14345: f64 = (assign10650_e14339 / assign10650_e14344);
        (assign10650_e14345,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10650_e14347;
        locals.var_rend_rv = 0.0;

        let (assign10660_e14370,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard255 != 0.0) && (locals.var_guard254 == 0.0))) && (locals.var_guard288 == 0.0)) && (locals.var_guard300 == 0.0)) && (!((locals.var_guard306 != 0.0) || (locals.var_guard307 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10660_e14370;
        locals.var_rend_rv = 0.0;

        let assign10670_e14373: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign10670_e14373;
        locals.var_guard311_rv = 0.0;

        let assign10680_e14376: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign10680_e14376;
        locals.var_guard312_rv = 0.0;

        let assign10690_e14387: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard313 = assign10690_e14387;
        locals.var_guard313_rv = 0.0;

        let assign10700_e14398: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard314 = assign10700_e14398;
        locals.var_guard314_rv = 0.0;

        let assign10710_e14401: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign10710_e14401;
        locals.var_guard315_rv = 0.0;

        let (assign10720_e14423,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard315 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10720_e14423;
        locals.var_rend_rv = 0.0;

        let (assign10730_e14452,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign10730_e14446: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10730_e14449: f64 = (locals.var_weff * locals.var_nuends);
        let assign10730_e14450: f64 = (assign10730_e14446 / assign10730_e14449);
        (assign10730_e14450,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10730_e14452;
        locals.var_rend_rv = 0.0;

        let assign10750_e14462: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard317 = assign10750_e14462;
        locals.var_guard317_rv = 0.0;

        let (assign10760_e14487,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && ((locals.var_guard314 != 0.0) && (locals.var_guard313 == 0.0))) && (locals.var_guard317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10760_e14487;
        locals.var_rend_rv = 0.0;

        let (assign10770_e14521,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && ((locals.var_guard314 != 0.0) && (locals.var_guard313 == 0.0))) && (locals.var_guard317 == 0.0)) {
        let assign10770_e14513: f64 = (p.p438 * locals.var_weff);
        let assign10770_e14516: f64 = (6.0 * locals.var_nuends);
        let assign10770_e14518: f64 = (assign10770_e14516 * locals.var_dmcgeff);
        let assign10770_e14519: f64 = (assign10770_e14513 / assign10770_e14518);
        (assign10770_e14519,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10770_e14521;
        locals.var_rend_rv = 0.0;

        let (assign10780_e14544,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (!((locals.var_guard313 != 0.0) || (locals.var_guard314 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10780_e14544;
        locals.var_rend_rv = 0.0;

        let assign10790_e14555: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard318 = assign10790_e14555;
        locals.var_guard318_rv = 0.0;

        let assign10800_e14566: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard319 = assign10800_e14566;
        locals.var_guard319_rv = 0.0;

        let assign10810_e14569: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign10810_e14569;
        locals.var_guard320_rv = 0.0;

        let (assign10820_e14592,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10820_e14592;
        locals.var_rend_rv = 0.0;

        let (assign10830_e14622,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard320 == 0.0)) {
        let assign10830_e14616: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10830_e14619: f64 = (locals.var_weff * locals.var_nuends);
        let assign10830_e14620: f64 = (assign10830_e14616 / assign10830_e14619);
        (assign10830_e14620,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10830_e14622;
        locals.var_rend_rv = 0.0;

        let assign10850_e14632: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard322 = assign10850_e14632;
        locals.var_guard322_rv = 0.0;

        let (assign10860_e14658,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && ((locals.var_guard319 != 0.0) && (locals.var_guard318 == 0.0))) && (locals.var_guard322 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10860_e14658;
        locals.var_rend_rv = 0.0;

        let (assign10870_e14693,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && ((locals.var_guard319 != 0.0) && (locals.var_guard318 == 0.0))) && (locals.var_guard322 == 0.0)) {
        let assign10870_e14685: f64 = (p.p438 * locals.var_weff);
        let assign10870_e14688: f64 = (6.0 * locals.var_nuends);
        let assign10870_e14690: f64 = (assign10870_e14688 * locals.var_dmcgeff);
        let assign10870_e14691: f64 = (assign10870_e14685 / assign10870_e14690);
        (assign10870_e14691,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10870_e14693;
        locals.var_rend_rv = 0.0;

        let (assign10880_e14717,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) && (!((locals.var_guard318 != 0.0) || (locals.var_guard319 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10880_e14717;
        locals.var_rend_rv = 0.0;

        let assign10890_e14720: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign10890_e14720;
        locals.var_guard323_rv = 0.0;

        let assign10900_e14731: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard324 = assign10900_e14731;
        locals.var_guard324_rv = 0.0;

        let assign10910_e14742: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard325 = assign10910_e14742;
        locals.var_guard325_rv = 0.0;

        let assign10920_e14745: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign10920_e14745;
        locals.var_guard326_rv = 0.0;

        let (assign10930_e14768,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard326 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10930_e14768;
        locals.var_rend_rv = 0.0;

        let (assign10940_e14798,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard324 != 0.0)) && (locals.var_guard326 == 0.0)) {
        let assign10940_e14792: f64 = (p.p438 * locals.var_dmcgeff);
        let assign10940_e14795: f64 = (locals.var_weff * locals.var_nuendd);
        let assign10940_e14796: f64 = (assign10940_e14792 / assign10940_e14795);
        (assign10940_e14796,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10940_e14798;
        locals.var_rend_rv = 0.0;

        let assign10960_e14809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10960_e14812: f64 = if ((locals.var_nuendd == 0.0) || (assign10960_e14809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard328 = assign10960_e14812;
        locals.var_guard328_rv = 0.0;

        let (assign10970_e14838,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && ((locals.var_guard325 != 0.0) && (locals.var_guard324 == 0.0))) && (locals.var_guard328 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10970_e14838;
        locals.var_rend_rv = 0.0;

        let (assign10980_e14875,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && ((locals.var_guard325 != 0.0) && (locals.var_guard324 == 0.0))) && (locals.var_guard328 == 0.0)) {
        let assign10980_e14865: f64 = (p.p438 * locals.var_weff);
        let assign10980_e14868: f64 = (3.0 * locals.var_nuendd);
        let assign10980_e14871: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10980_e14872: f64 = (assign10980_e14868 * assign10980_e14871);
        let assign10980_e14873: f64 = (assign10980_e14865 / assign10980_e14872);
        (assign10980_e14873,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10980_e14875;
        locals.var_rend_rv = 0.0;

        let (assign10990_e14899,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 != 0.0)) && (!((locals.var_guard324 != 0.0) || (locals.var_guard325 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign10990_e14899;
        locals.var_rend_rv = 0.0;

        let assign11000_e14910: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard329 = assign11000_e14910;
        locals.var_guard329_rv = 0.0;

        let assign11010_e14921: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard330 = assign11010_e14921;
        locals.var_guard330_rv = 0.0;

        let assign11020_e14924: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign11020_e14924;
        locals.var_guard331_rv = 0.0;

        let (assign11030_e14948,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard329 != 0.0)) && (locals.var_guard331 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11030_e14948;
        locals.var_rend_rv = 0.0;

        let (assign11040_e14979,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (locals.var_guard329 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign11040_e14973: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11040_e14976: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11040_e14977: f64 = (assign11040_e14973 / assign11040_e14976);
        (assign11040_e14977,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11040_e14979;
        locals.var_rend_rv = 0.0;

        let assign11060_e14990: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11060_e14993: f64 = if ((locals.var_nuendd == 0.0) || (assign11060_e14990 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard333 = assign11060_e14993;
        locals.var_guard333_rv = 0.0;

        let (assign11070_e15020,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && ((locals.var_guard330 != 0.0) && (locals.var_guard329 == 0.0))) && (locals.var_guard333 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11070_e15020;
        locals.var_rend_rv = 0.0;

        let (assign11080_e15058,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && ((locals.var_guard330 != 0.0) && (locals.var_guard329 == 0.0))) && (locals.var_guard333 == 0.0)) {
        let assign11080_e15048: f64 = (p.p438 * locals.var_weff);
        let assign11080_e15051: f64 = (3.0 * locals.var_nuendd);
        let assign11080_e15054: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11080_e15055: f64 = (assign11080_e15051 * assign11080_e15054);
        let assign11080_e15056: f64 = (assign11080_e15048 / assign11080_e15055);
        (assign11080_e15056,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11080_e15058;
        locals.var_rend_rv = 0.0;

        let (assign11090_e15083,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard256 != 0.0) && (!((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0))))) && (locals.var_guard311 == 0.0)) && (locals.var_guard323 == 0.0)) && (!((locals.var_guard329 != 0.0) || (locals.var_guard330 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11090_e15083;
        locals.var_rend_rv = 0.0;

        let assign11100_e15086: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign11100_e15086;
        locals.var_guard334_rv = 0.0;

        let assign11110_e15089: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign11110_e15089;
        locals.var_guard335_rv = 0.0;

        let assign11120_e15100: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard336 = assign11120_e15100;
        locals.var_guard336_rv = 0.0;

        let assign11130_e15111: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard337 = assign11130_e15111;
        locals.var_guard337_rv = 0.0;

        let assign11140_e15114: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign11140_e15114;
        locals.var_guard338_rv = 0.0;

        let (assign11150_e15138,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard338 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11150_e15138;
        locals.var_rend_rv = 0.0;

        let (assign11160_e15169,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard338 == 0.0)) {
        let assign11160_e15163: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11160_e15166: f64 = (locals.var_weff * locals.var_nuends);
        let assign11160_e15167: f64 = (assign11160_e15163 / assign11160_e15166);
        (assign11160_e15167,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11160_e15169;
        locals.var_rend_rv = 0.0;

        let assign11180_e15179: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign11180_e15179;
        locals.var_guard340_rv = 0.0;

        let (assign11190_e15206,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && ((locals.var_guard337 != 0.0) && (locals.var_guard336 == 0.0))) && (locals.var_guard340 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11190_e15206;
        locals.var_rend_rv = 0.0;

        let (assign11200_e15242,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && ((locals.var_guard337 != 0.0) && (locals.var_guard336 == 0.0))) && (locals.var_guard340 == 0.0)) {
        let assign11200_e15234: f64 = (p.p438 * locals.var_weff);
        let assign11200_e15237: f64 = (6.0 * locals.var_nuends);
        let assign11200_e15239: f64 = (assign11200_e15237 * locals.var_dmcgeff);
        let assign11200_e15240: f64 = (assign11200_e15234 / assign11200_e15239);
        (assign11200_e15240,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11200_e15242;
        locals.var_rend_rv = 0.0;

        let (assign11210_e15267,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (!((locals.var_guard336 != 0.0) || (locals.var_guard337 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11210_e15267;
        locals.var_rend_rv = 0.0;

        let assign11220_e15278: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign11220_e15278;
        locals.var_guard341_rv = 0.0;

        let assign11230_e15289: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard342 = assign11230_e15289;
        locals.var_guard342_rv = 0.0;

        let assign11240_e15292: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard343 = assign11240_e15292;
        locals.var_guard343_rv = 0.0;

        let (assign11250_e15317,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard343 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11250_e15317;
        locals.var_rend_rv = 0.0;

        let (assign11260_e15349,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (locals.var_guard341 != 0.0)) && (locals.var_guard343 == 0.0)) {
        let assign11260_e15343: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11260_e15346: f64 = (locals.var_weff * locals.var_nuends);
        let assign11260_e15347: f64 = (assign11260_e15343 / assign11260_e15346);
        (assign11260_e15347,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11260_e15349;
        locals.var_rend_rv = 0.0;

        let assign11280_e15359: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard345 = assign11280_e15359;
        locals.var_guard345_rv = 0.0;

        let (assign11290_e15387,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && ((locals.var_guard342 != 0.0) && (locals.var_guard341 == 0.0))) && (locals.var_guard345 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11290_e15387;
        locals.var_rend_rv = 0.0;

        let (assign11300_e15424,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && ((locals.var_guard342 != 0.0) && (locals.var_guard341 == 0.0))) && (locals.var_guard345 == 0.0)) {
        let assign11300_e15416: f64 = (p.p438 * locals.var_weff);
        let assign11300_e15419: f64 = (6.0 * locals.var_nuends);
        let assign11300_e15421: f64 = (assign11300_e15419 * locals.var_dmcgeff);
        let assign11300_e15422: f64 = (assign11300_e15416 / assign11300_e15421);
        (assign11300_e15422,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11300_e15424;
        locals.var_rend_rv = 0.0;

        let (assign11310_e15450,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 == 0.0)) && (!((locals.var_guard341 != 0.0) || (locals.var_guard342 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11310_e15450;
        locals.var_rend_rv = 0.0;

        let assign11320_e15453: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard346 = assign11320_e15453;
        locals.var_guard346_rv = 0.0;

        let assign11330_e15464: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign11330_e15464;
        locals.var_guard347_rv = 0.0;

        let assign11340_e15475: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard348 = assign11340_e15475;
        locals.var_guard348_rv = 0.0;

        let assign11350_e15478: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign11350_e15478;
        locals.var_guard349_rv = 0.0;

        let (assign11360_e15503,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) && (locals.var_guard349 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11360_e15503;
        locals.var_rend_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11370_e15535,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) && (locals.var_guard349 == 0.0)) {
        let assign11370_e15529: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11370_e15532: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11370_e15533: f64 = (assign11370_e15529 / assign11370_e15532);
        (assign11370_e15533,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11370_e15535;
        locals.var_rend_rv = 0.0;

        let assign11390_e15545: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign11390_e15545;
        locals.var_guard351_rv = 0.0;

        let (assign11400_e15573,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && ((locals.var_guard348 != 0.0) && (locals.var_guard347 == 0.0))) && (locals.var_guard351 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11400_e15573;
        locals.var_rend_rv = 0.0;

        let (assign11410_e15610,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && ((locals.var_guard348 != 0.0) && (locals.var_guard347 == 0.0))) && (locals.var_guard351 == 0.0)) {
        let assign11410_e15602: f64 = (p.p438 * locals.var_weff);
        let assign11410_e15605: f64 = (6.0 * locals.var_nuendd);
        let assign11410_e15607: f64 = (assign11410_e15605 * locals.var_dmcgeff);
        let assign11410_e15608: f64 = (assign11410_e15602 / assign11410_e15607);
        (assign11410_e15608,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11410_e15610;
        locals.var_rend_rv = 0.0;

        let (assign11420_e15636,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 != 0.0)) && (!((locals.var_guard347 != 0.0) || (locals.var_guard348 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11420_e15636;
        locals.var_rend_rv = 0.0;

        let assign11430_e15647: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign11430_e15647;
        locals.var_guard352_rv = 0.0;

        let assign11440_e15658: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard353 = assign11440_e15658;
        locals.var_guard353_rv = 0.0;

        let assign11450_e15661: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard354 = assign11450_e15661;
        locals.var_guard354_rv = 0.0;

        let (assign11460_e15687,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) && (locals.var_guard354 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11460_e15687;
        locals.var_rend_rv = 0.0;

        let (assign11470_e15720,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) && (locals.var_guard354 == 0.0)) {
        let assign11470_e15714: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11470_e15717: f64 = (locals.var_weff * locals.var_nuendd);
        let assign11470_e15718: f64 = (assign11470_e15714 / assign11470_e15717);
        (assign11470_e15718,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11470_e15720;
        locals.var_rend_rv = 0.0;

        let assign11490_e15730: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard356 = assign11490_e15730;
        locals.var_guard356_rv = 0.0;

        let (assign11500_e15759,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && ((locals.var_guard353 != 0.0) && (locals.var_guard352 == 0.0))) && (locals.var_guard356 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11500_e15759;
        locals.var_rend_rv = 0.0;

        let (assign11510_e15797,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && ((locals.var_guard353 != 0.0) && (locals.var_guard352 == 0.0))) && (locals.var_guard356 == 0.0)) {
        let assign11510_e15789: f64 = (p.p438 * locals.var_weff);
        let assign11510_e15792: f64 = (6.0 * locals.var_nuendd);
        let assign11510_e15794: f64 = (assign11510_e15792 * locals.var_dmcgeff);
        let assign11510_e15795: f64 = (assign11510_e15789 / assign11510_e15794);
        (assign11510_e15795,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11510_e15797;
        locals.var_rend_rv = 0.0;

        let (assign11520_e15824,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard257 != 0.0) && (!(((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard334 == 0.0)) && (locals.var_guard346 == 0.0)) && (!((locals.var_guard352 != 0.0) || (locals.var_guard353 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11520_e15824;
        locals.var_rend_rv = 0.0;

        let assign11530_e15827: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign11530_e15827;
        locals.var_guard357_rv = 0.0;

        let assign11540_e15830: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign11540_e15830;
        locals.var_guard358_rv = 0.0;

        let assign11550_e15841: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard359 = assign11550_e15841;
        locals.var_guard359_rv = 0.0;

        let assign11560_e15852: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard360 = assign11560_e15852;
        locals.var_guard360_rv = 0.0;

        let assign11570_e15855: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign11570_e15855;
        locals.var_guard361_rv = 0.0;

        let (assign11580_e15881,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11580_e15881;
        locals.var_rend_rv = 0.0;

        let (assign11590_e15914,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard359 != 0.0)) && (locals.var_guard361 == 0.0)) {
        let assign11590_e15908: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11590_e15911: f64 = (locals.var_weff * locals.var_nuends);
        let assign11590_e15912: f64 = (assign11590_e15908 / assign11590_e15911);
        (assign11590_e15912,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11590_e15914;
        locals.var_rend_rv = 0.0;

        let assign11610_e15925: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11610_e15928: f64 = if ((locals.var_nuends == 0.0) || (assign11610_e15925 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard363 = assign11610_e15928;
        locals.var_guard363_rv = 0.0;

        let (assign11620_e15957,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && ((locals.var_guard360 != 0.0) && (locals.var_guard359 == 0.0))) && (locals.var_guard363 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11620_e15957;
        locals.var_rend_rv = 0.0;

        let (assign11630_e15997,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && ((locals.var_guard360 != 0.0) && (locals.var_guard359 == 0.0))) && (locals.var_guard363 == 0.0)) {
        let assign11630_e15987: f64 = (p.p438 * locals.var_weff);
        let assign11630_e15990: f64 = (3.0 * locals.var_nuends);
        let assign11630_e15993: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11630_e15994: f64 = (assign11630_e15990 * assign11630_e15993);
        let assign11630_e15995: f64 = (assign11630_e15987 / assign11630_e15994);
        (assign11630_e15995,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11630_e15997;
        locals.var_rend_rv = 0.0;

        let (assign11640_e16024,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (!((locals.var_guard359 != 0.0) || (locals.var_guard360 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11640_e16024;
        locals.var_rend_rv = 0.0;

        let assign11650_e16035: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign11650_e16035;
        locals.var_guard364_rv = 0.0;

        let assign11660_e16046: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard365 = assign11660_e16046;
        locals.var_guard365_rv = 0.0;

        let assign11670_e16049: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard366 = assign11670_e16049;
        locals.var_guard366_rv = 0.0;

        let (assign11680_e16076,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (locals.var_guard364 != 0.0)) && (locals.var_guard366 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11680_e16076;
        locals.var_rend_rv = 0.0;

        let (assign11690_e16110,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (locals.var_guard364 != 0.0)) && (locals.var_guard366 == 0.0)) {
        let assign11690_e16104: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11690_e16107: f64 = (locals.var_weff * locals.var_nuends);
        let assign11690_e16108: f64 = (assign11690_e16104 / assign11690_e16107);
        (assign11690_e16108,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11690_e16110;
        locals.var_rend_rv = 0.0;

        let assign11710_e16121: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11710_e16124: f64 = if ((locals.var_nuends == 0.0) || (assign11710_e16121 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard368 = assign11710_e16124;
        locals.var_guard368_rv = 0.0;

        let (assign11720_e16154,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && ((locals.var_guard365 != 0.0) && (locals.var_guard364 == 0.0))) && (locals.var_guard368 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11720_e16154;
        locals.var_rend_rv = 0.0;

        let (assign11730_e16195,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && ((locals.var_guard365 != 0.0) && (locals.var_guard364 == 0.0))) && (locals.var_guard368 == 0.0)) {
        let assign11730_e16185: f64 = (p.p438 * locals.var_weff);
        let assign11730_e16188: f64 = (3.0 * locals.var_nuends);
        let assign11730_e16191: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11730_e16192: f64 = (assign11730_e16188 * assign11730_e16191);
        let assign11730_e16193: f64 = (assign11730_e16185 / assign11730_e16192);
        (assign11730_e16193,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11730_e16195;
        locals.var_rend_rv = 0.0;

        let (assign11740_e16223,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 == 0.0)) && (!((locals.var_guard364 != 0.0) || (locals.var_guard365 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11740_e16223;
        locals.var_rend_rv = 0.0;

        let (assign11750_e16248,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard258 != 0.0) && (!((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard357 == 0.0)) {
        let assign11750_e16244: f64 = (p.p438 * locals.var_dmdgeff);
        let assign11750_e16246: f64 = (assign11750_e16244 / locals.var_weff);
        (assign11750_e16246,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11750_e16248;
        locals.var_rend_rv = 0.0;

        let assign11760_e16251: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign11760_e16251;
        locals.var_guard369_rv = 0.0;

        let assign11770_e16254: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard370 = assign11770_e16254;
        locals.var_guard370_rv = 0.0;

        let assign11780_e16265: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign11780_e16265;
        locals.var_guard371_rv = 0.0;

        let assign11790_e16276: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard372 = assign11790_e16276;
        locals.var_guard372_rv = 0.0;

        let assign11800_e16279: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign11800_e16279;
        locals.var_guard373_rv = 0.0;

        let (assign11810_e16307,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard371 != 0.0)) && (locals.var_guard373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11810_e16307;
        locals.var_rend_rv = 0.0;

        let (assign11820_e16342,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard371 != 0.0)) && (locals.var_guard373 == 0.0)) {
        let assign11820_e16336: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11820_e16339: f64 = (locals.var_weff * locals.var_nuends);
        let assign11820_e16340: f64 = (assign11820_e16336 / assign11820_e16339);
        (assign11820_e16340,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11820_e16342;
        locals.var_rend_rv = 0.0;

        let assign11840_e16352: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign11840_e16352;
        locals.var_guard375_rv = 0.0;

        let (assign11850_e16383,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && ((locals.var_guard372 != 0.0) && (locals.var_guard371 == 0.0))) && (locals.var_guard375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11850_e16383;
        locals.var_rend_rv = 0.0;

        let (assign11860_e16423,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && ((locals.var_guard372 != 0.0) && (locals.var_guard371 == 0.0))) && (locals.var_guard375 == 0.0)) {
        let assign11860_e16415: f64 = (p.p438 * locals.var_weff);
        let assign11860_e16418: f64 = (6.0 * locals.var_nuends);
        let assign11860_e16420: f64 = (assign11860_e16418 * locals.var_dmcgeff);
        let assign11860_e16421: f64 = (assign11860_e16415 / assign11860_e16420);
        (assign11860_e16421,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11860_e16423;
        locals.var_rend_rv = 0.0;

        let (assign11870_e16452,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (!((locals.var_guard371 != 0.0) || (locals.var_guard372 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11870_e16452;
        locals.var_rend_rv = 0.0;

        let assign11880_e16463: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign11880_e16463;
        locals.var_guard376_rv = 0.0;

        let assign11890_e16474: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard377 = assign11890_e16474;
        locals.var_guard377_rv = 0.0;

        let assign11900_e16477: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign11900_e16477;
        locals.var_guard378_rv = 0.0;

        let (assign11910_e16506,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (locals.var_guard376 != 0.0)) && (locals.var_guard378 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11910_e16506;
        locals.var_rend_rv = 0.0;

        let (assign11920_e16542,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (locals.var_guard376 != 0.0)) && (locals.var_guard378 == 0.0)) {
        let assign11920_e16536: f64 = (p.p438 * locals.var_dmcgeff);
        let assign11920_e16539: f64 = (locals.var_weff * locals.var_nuends);
        let assign11920_e16540: f64 = (assign11920_e16536 / assign11920_e16539);
        (assign11920_e16540,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11920_e16542;
        locals.var_rend_rv = 0.0;

        let assign11940_e16552: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard380 = assign11940_e16552;
        locals.var_guard380_rv = 0.0;

        let (assign11950_e16584,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && ((locals.var_guard377 != 0.0) && (locals.var_guard376 == 0.0))) && (locals.var_guard380 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11950_e16584;
        locals.var_rend_rv = 0.0;

        let (assign11960_e16625,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && ((locals.var_guard377 != 0.0) && (locals.var_guard376 == 0.0))) && (locals.var_guard380 == 0.0)) {
        let assign11960_e16617: f64 = (p.p438 * locals.var_weff);
        let assign11960_e16620: f64 = (6.0 * locals.var_nuends);
        let assign11960_e16622: f64 = (assign11960_e16620 * locals.var_dmcgeff);
        let assign11960_e16623: f64 = (assign11960_e16617 / assign11960_e16622);
        (assign11960_e16623,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11960_e16625;
        locals.var_rend_rv = 0.0;

        let (assign11970_e16655,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 == 0.0)) && (!((locals.var_guard376 != 0.0) || (locals.var_guard377 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11970_e16655;
        locals.var_rend_rv = 0.0;

        let assign11980_e16658: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign11980_e16658;
        locals.var_guard381_rv = 0.0;

        let (assign11990_e16683,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 == 0.0)) && (locals.var_guard381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign11990_e16683;
        locals.var_rend_rv = 0.0;

        let (assign12000_e16715,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard259 != 0.0) && (!(((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard369 == 0.0)) && (locals.var_guard381 == 0.0)) {
        let assign12000_e16709: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12000_e16712: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12000_e16713: f64 = (assign12000_e16709 / assign12000_e16712);
        (assign12000_e16713,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12000_e16715;
        locals.var_rend_rv = 0.0;

        let assign12010_e16718: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign12010_e16718;
        locals.var_guard382_rv = 0.0;

        let (assign12020_e16746,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 != 0.0)) {
        let assign12020_e16742: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12020_e16744: f64 = (assign12020_e16742 / locals.var_weff);
        (assign12020_e16744,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12020_e16746;
        locals.var_rend_rv = 0.0;

        let assign12030_e16749: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign12030_e16749;
        locals.var_guard383_rv = 0.0;

        let assign12040_e16760: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign12040_e16760;
        locals.var_guard384_rv = 0.0;

        let assign12050_e16771: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard385 = assign12050_e16771;
        locals.var_guard385_rv = 0.0;

        let assign12060_e16774: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign12060_e16774;
        locals.var_guard386_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12070_e16805,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) && (locals.var_guard386 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12070_e16805;
        locals.var_rend_rv = 0.0;

        let (assign12080_e16843,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign12080_e16837: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12080_e16840: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12080_e16841: f64 = (assign12080_e16837 / assign12080_e16840);
        (assign12080_e16841,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12080_e16843;
        locals.var_rend_rv = 0.0;

        let assign12100_e16854: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12100_e16857: f64 = if ((locals.var_nuendd == 0.0) || (assign12100_e16854 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign12100_e16857;
        locals.var_guard388_rv = 0.0;

        let (assign12110_e16891,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && ((locals.var_guard385 != 0.0) && (locals.var_guard384 == 0.0))) && (locals.var_guard388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12110_e16891;
        locals.var_rend_rv = 0.0;

        let (assign12120_e16936,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && ((locals.var_guard385 != 0.0) && (locals.var_guard384 == 0.0))) && (locals.var_guard388 == 0.0)) {
        let assign12120_e16926: f64 = (p.p438 * locals.var_weff);
        let assign12120_e16929: f64 = (3.0 * locals.var_nuendd);
        let assign12120_e16932: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12120_e16933: f64 = (assign12120_e16929 * assign12120_e16932);
        let assign12120_e16934: f64 = (assign12120_e16926 / assign12120_e16933);
        (assign12120_e16934,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12120_e16936;
        locals.var_rend_rv = 0.0;

        let (assign12130_e16968,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 != 0.0)) && (!((locals.var_guard384 != 0.0) || (locals.var_guard385 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12130_e16968;
        locals.var_rend_rv = 0.0;

        let assign12140_e16979: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign12140_e16979;
        locals.var_guard389_rv = 0.0;

        let assign12150_e16990: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard390 = assign12150_e16990;
        locals.var_guard390_rv = 0.0;

        let assign12160_e16993: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign12160_e16993;
        locals.var_guard391_rv = 0.0;

        let (assign12170_e17025,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard389 != 0.0)) && (locals.var_guard391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12170_e17025;
        locals.var_rend_rv = 0.0;

        let (assign12180_e17064,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (locals.var_guard389 != 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12180_e17058: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12180_e17061: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12180_e17062: f64 = (assign12180_e17058 / assign12180_e17061);
        (assign12180_e17062,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12180_e17064;
        locals.var_rend_rv = 0.0;

        let assign12200_e17075: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12200_e17078: f64 = if ((locals.var_nuendd == 0.0) || (assign12200_e17075 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard393 = assign12200_e17078;
        locals.var_guard393_rv = 0.0;

        let (assign12210_e17113,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && ((locals.var_guard390 != 0.0) && (locals.var_guard389 == 0.0))) && (locals.var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12210_e17113;
        locals.var_rend_rv = 0.0;

        let (assign12220_e17159,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && ((locals.var_guard390 != 0.0) && (locals.var_guard389 == 0.0))) && (locals.var_guard393 == 0.0)) {
        let assign12220_e17149: f64 = (p.p438 * locals.var_weff);
        let assign12220_e17152: f64 = (3.0 * locals.var_nuendd);
        let assign12220_e17155: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign12220_e17156: f64 = (assign12220_e17152 * assign12220_e17155);
        let assign12220_e17157: f64 = (assign12220_e17149 / assign12220_e17156);
        (assign12220_e17157,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12220_e17159;
        locals.var_rend_rv = 0.0;

        let (assign12230_e17192,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard260 != 0.0) && (!((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard382 == 0.0)) && (locals.var_guard383 == 0.0)) && (!((locals.var_guard389 != 0.0) || (locals.var_guard390 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12230_e17192;
        locals.var_rend_rv = 0.0;

        let assign12240_e17195: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign12240_e17195;
        locals.var_guard394_rv = 0.0;

        let assign12250_e17198: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign12250_e17198;
        locals.var_guard395_rv = 0.0;

        let (assign12260_e17226,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12260_e17226;
        locals.var_rend_rv = 0.0;

        let (assign12270_e17261,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 == 0.0)) {
        let assign12270_e17255: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12270_e17258: f64 = (locals.var_weff * locals.var_nuends);
        let assign12270_e17259: f64 = (assign12270_e17255 / assign12270_e17258);
        (assign12270_e17259,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12270_e17261;
        locals.var_rend_rv = 0.0;

        let assign12280_e17264: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign12280_e17264;
        locals.var_guard396_rv = 0.0;

        let assign12290_e17275: f64 = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign12290_e17275;
        locals.var_guard397_rv = 0.0;

        let assign12300_e17286: f64 = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard398 = assign12300_e17286;
        locals.var_guard398_rv = 0.0;

        let assign12310_e17289: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign12310_e17289;
        locals.var_guard399_rv = 0.0;

        let (assign12320_e17322,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard399 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12320_e17322;
        locals.var_rend_rv = 0.0;

        let (assign12330_e17362,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard397 != 0.0)) && (locals.var_guard399 == 0.0)) {
        let assign12330_e17356: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12330_e17359: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12330_e17360: f64 = (assign12330_e17356 / assign12330_e17359);
        (assign12330_e17360,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12330_e17362;
        locals.var_rend_rv = 0.0;

        let assign12350_e17372: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign12350_e17372;
        locals.var_guard401_rv = 0.0;

        let (assign12360_e17408,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && ((locals.var_guard398 != 0.0) && (locals.var_guard397 == 0.0))) && (locals.var_guard401 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12360_e17408;
        locals.var_rend_rv = 0.0;

        let (assign12370_e17453,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && ((locals.var_guard398 != 0.0) && (locals.var_guard397 == 0.0))) && (locals.var_guard401 == 0.0)) {
        let assign12370_e17445: f64 = (p.p438 * locals.var_weff);
        let assign12370_e17448: f64 = (6.0 * locals.var_nuendd);
        let assign12370_e17450: f64 = (assign12370_e17448 * locals.var_dmcgeff);
        let assign12370_e17451: f64 = (assign12370_e17445 / assign12370_e17450);
        (assign12370_e17451,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12370_e17453;
        locals.var_rend_rv = 0.0;

        let (assign12380_e17487,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 != 0.0)) && (!((locals.var_guard397 != 0.0) || (locals.var_guard398 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12380_e17487;
        locals.var_rend_rv = 0.0;

        let assign12390_e17498: f64 = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign12390_e17498;
        locals.var_guard402_rv = 0.0;

        let assign12400_e17509: f64 = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard403 = assign12400_e17509;
        locals.var_guard403_rv = 0.0;

        let assign12410_e17512: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign12410_e17512;
        locals.var_guard404_rv = 0.0;

        let (assign12420_e17546,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard402 != 0.0)) && (locals.var_guard404 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12420_e17546;
        locals.var_rend_rv = 0.0;

        let (assign12430_e17587,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard402 != 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12430_e17581: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12430_e17584: f64 = (locals.var_weff * locals.var_nuendd);
        let assign12430_e17585: f64 = (assign12430_e17581 / assign12430_e17584);
        (assign12430_e17585,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12430_e17587;
        locals.var_rend_rv = 0.0;

        let assign12450_e17597: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard406 = assign12450_e17597;
        locals.var_guard406_rv = 0.0;

        let (assign12460_e17634,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && ((locals.var_guard403 != 0.0) && (locals.var_guard402 == 0.0))) && (locals.var_guard406 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12460_e17634;
        locals.var_rend_rv = 0.0;

        let (assign12470_e17680,) = {
    if (((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && ((locals.var_guard403 != 0.0) && (locals.var_guard402 == 0.0))) && (locals.var_guard406 == 0.0)) {
        let assign12470_e17672: f64 = (p.p438 * locals.var_weff);
        let assign12470_e17675: f64 = (6.0 * locals.var_nuendd);
        let assign12470_e17677: f64 = (assign12470_e17675 * locals.var_dmcgeff);
        let assign12470_e17678: f64 = (assign12470_e17672 / assign12470_e17677);
        (assign12470_e17678,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12470_e17680;
        locals.var_rend_rv = 0.0;

        let (assign12480_e17715,) = {
    if ((((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard261 != 0.0) && (!(((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) && (locals.var_guard394 == 0.0)) && (locals.var_guard396 == 0.0)) && (!((locals.var_guard402 != 0.0) || (locals.var_guard403 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12480_e17715;
        locals.var_rend_rv = 0.0;

        let (assign12490_e17745,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard262 != 0.0) && (!((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) {
        let assign12490_e17741: f64 = (p.p438 * locals.var_dmdgeff);
        let assign12490_e17743: f64 = (assign12490_e17741 / locals.var_weff);
        (assign12490_e17743,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12490_e17745;
        locals.var_rend_rv = 0.0;

        let assign12500_e17748: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign12500_e17748;
        locals.var_guard407_rv = 0.0;

        let (assign12510_e17784,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) {
        let assign12510_e17778: f64 = (0.5 * p.p438);
        let assign12510_e17780: f64 = (assign12510_e17778 * locals.var_dmcgeff);
        let assign12510_e17782: f64 = (assign12510_e17780 / locals.var_weff);
        (assign12510_e17782,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12510_e17784;
        locals.var_rend_rv = 0.0;

        let assign12520_e17787: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign12520_e17787;
        locals.var_guard408_rv = 0.0;

        let (assign12530_e17819,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) && (locals.var_guard408 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12530_e17819;
        locals.var_rint_rv = 0.0;

        let (assign12540_e17860,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 != 0.0)) && (locals.var_guard408 == 0.0)) {
        let assign12540_e17852: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12540_e17856: f64 = (p.p2 - 2.0);
        let assign12540_e17857: f64 = (locals.var_weff * assign12540_e17856);
        let assign12540_e17858: f64 = (assign12540_e17852 / assign12540_e17857);
        (assign12540_e17858,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12540_e17860;
        locals.var_rint_rv = 0.0;

        let (assign12550_e17891,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12550_e17891;
        locals.var_rend_rv = 0.0;

        let (assign12560_e17928,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard263 != 0.0) && (!(((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard407 == 0.0)) {
        let assign12560_e17922: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12560_e17925: f64 = (locals.var_weff * p.p2);
        let assign12560_e17926: f64 = (assign12560_e17922 / assign12560_e17925);
        (assign12560_e17926,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12560_e17928;
        locals.var_rint_rv = 0.0;

        let assign12570_e17931: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign12570_e17931;
        locals.var_guard409_rv = 0.0;

        let (assign12580_e17963,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12580_e17963;
        locals.var_rend_rv = 0.0;

        let (assign12590_e18001,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 != 0.0)) {
        let assign12590_e17995: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12590_e17998: f64 = (locals.var_weff * p.p2);
        let assign12590_e17999: f64 = (assign12590_e17995 / assign12590_e17998);
        (assign12590_e17999,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12590_e18001;
        locals.var_rint_rv = 0.0;

        let (assign12600_e18040,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) {
        let assign12600_e18034: f64 = (0.5 * p.p438);
        let assign12600_e18036: f64 = (assign12600_e18034 * locals.var_dmcgeff);
        let assign12600_e18038: f64 = (assign12600_e18036 / locals.var_weff);
        (assign12600_e18038,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign12600_e18040;
        locals.var_rend_rv = 0.0;

        let assign12610_e18043: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign12610_e18043;
        locals.var_guard410_rv = 0.0;

        let (assign12620_e18078,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12620_e18078;
        locals.var_rint_rv = 0.0;

        let (assign12630_e18122,) = {
    if (((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && ((locals.var_guard264 != 0.0) && (!((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0))))) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) {
        let assign12630_e18114: f64 = (p.p438 * locals.var_dmcgeff);
        let assign12630_e18118: f64 = (p.p2 - 2.0);
        let assign12630_e18119: f64 = (locals.var_weff * assign12630_e18118);
        let assign12630_e18120: f64 = (assign12630_e18114 / assign12630_e18119);
        (assign12630_e18120,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12630_e18122;
        locals.var_rint_rv = 0.0;

        let (assign12640_e18152,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (!(((((((((((locals.var_guard254 != 0.0) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)) || (locals.var_guard264 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign12640_e18152;
        locals.var_rint_rv = 0.0;

        let assign12650_e18155: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign12650_e18155;
        locals.var_guard411_rv = 0.0;

        let (assign12660_e18164,) = {
    if (((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12660_e18164;
        locals.var_rdraingeo_rv = 0.0;

        let assign12670_e18167: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign12670_e18167;
        locals.var_guard412_rv = 0.0;

        let (assign12680_e18179,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12680_e18179;
        locals.var_rdraingeo_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (assign12690_e18198,) = {
    if ((((locals.var_guard246 == 0.0) && (locals.var_guard247 != 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 == 0.0)) {
        let assign12690_e18192: f64 = (locals.var_rint * locals.var_rend);
        let assign12690_e18195: f64 = (locals.var_rint + locals.var_rend);
        let assign12690_e18196: f64 = (assign12690_e18192 / assign12690_e18195);
        (assign12690_e18196,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12690_e18198;
        locals.var_rdraingeo_rv = 0.0;

        let (assign12710_e18209,) = {
    if ((locals.var_guard246 == 0.0) && (locals.var_guard247 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12710_e18209;
        locals.var_rdraingeo_rv = 0.0;

        let assign12720_e18212: f64 = if p.p33 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign12720_e18212;
        locals.var_guard414_rv = 0.0;

        let assign12730_e18215: f64 = if locals.var_rsourcegeo < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign12730_e18215;
        locals.var_guard415_rv = 0.0;

        let (assign12740_e18221,) = {
    if ((locals.var_guard414 != 0.0) && (locals.var_guard415 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign12740_e18221;
        locals.var_rsourcegeo_rv = 0.0;

        let assign12750_e18224: f64 = if locals.var_rdraingeo < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign12750_e18224;
        locals.var_guard416_rv = 0.0;

        let (assign12760_e18230,) = {
    if ((locals.var_guard414 != 0.0) && (locals.var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12760_e18230;
        locals.var_rdraingeo_rv = 0.0;

        let assign12770_e18233: f64 = if locals.var_rsourcegeo <= p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign12770_e18233;
        locals.var_guard417_rv = 0.0;

        let (assign12780_e18240,) = {
    if ((locals.var_guard414 == 0.0) && (locals.var_guard417 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign12780_e18240;
        locals.var_rsourcegeo_rv = 0.0;

        let assign12790_e18243: f64 = if locals.var_rdraingeo <= p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign12790_e18243;
        locals.var_guard418_rv = 0.0;

        let (assign12800_e18250,) = {
    if ((locals.var_guard414 == 0.0) && (locals.var_guard418 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign12800_e18250;
        locals.var_rdraingeo_rv = 0.0;

        let assign12810_e18253: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign12810_e18253;
        locals.var_guard419_rv = 0.0;

        let assign12820_e18256: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign12820_e18256;
        locals.var_guard420_rv = 0.0;

        let (assign12830_e18262,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard420 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rswmin_i,)
    }
};
        locals.var_rswmin_i = assign12830_e18262;
        locals.var_rswmin_i_rv = 0.0;

        let assign12840_e18265: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign12840_e18265;
        locals.var_guard421_rv = 0.0;

        let (assign12850_e18271,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard421 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdwmin_i,)
    }
};
        locals.var_rdwmin_i = assign12850_e18271;
        locals.var_rdwmin_i_rv = 0.0;

        let assign12860_e18274: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign12860_e18274;
        locals.var_guard422_rv = 0.0;

        let (assign12870_e18280,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard422 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign12870_e18280;
        locals.var_rsw_i_rv = 0.0;

        let assign12880_e18283: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign12880_e18283;
        locals.var_guard423_rv = 0.0;

        let (assign12890_e18289,) = {
    if ((locals.var_guard419 != 0.0) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign12890_e18289;
        locals.var_rdw_i_rv = 0.0;

        let assign12900_e18292: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign12900_e18292;
        locals.var_guard424_rv = 0.0;

        let (assign12910_e18299,) = {
    if ((locals.var_guard419 == 0.0) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdswmin_i,)
    }
};
        locals.var_rdswmin_i = assign12910_e18299;
        locals.var_rdswmin_i_rv = 0.0;

        let assign12920_e18302: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign12920_e18302;
        locals.var_guard425_rv = 0.0;

        let (assign12930_e18309,) = {
    if ((locals.var_guard419 == 0.0) && (locals.var_guard425 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign12930_e18309;
        locals.var_rdsw_i_rv = 0.0;

        let assign12940_e18314: f64 = (locals.var_weffcj / 3.0);
        let assign12940_e18316: f64 = (assign12940_e18314 / p.p22);
        let assign12940_e18317: f64 = (p.p21 + assign12940_e18316);
        let assign12940_e18318: f64 = (p.p900 * assign12940_e18317);
        let assign12940_e18321: f64 = (p.p22 * p.p2);
        let assign12940_e18324: f64 = (locals.var_lnew - p.p899);
        let assign12940_e18325: f64 = (assign12940_e18321 * assign12940_e18324);
        let assign12940_e18326: f64 = (assign12940_e18318 / assign12940_e18325);
        locals.var_grgeltd = assign12940_e18326;
        locals.var_grgeltd_rv = 0.0;

        let assign12950_e18329: f64 = if locals.var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign12950_e18329;
        locals.var_guard426_rv = 0.0;

        let (assign12960_e18335,) = {
    if (locals.var_guard426 != 0.0) {
        let assign12960_e18333: f64 = (1.0 / locals.var_grgeltd);
        (assign12960_e18333,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12960_e18335;
        locals.var_grgeltd_rv = 0.0;

        let (assign12970_e18340,) = {
    if (locals.var_guard426 == 0.0) {
        (1000.0,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign12970_e18340;
        locals.var_grgeltd_rv = 0.0;

        let assign12990_e18346: f64 = (p.p76 * p.p76);
        locals.var_t0 = assign12990_e18346;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign13000_e18349: f64 = (p.p76 * locals.var_poxedge_i);
        locals.var_t1 = assign13000_e18349;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13010_e18352: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign13010_e18352;
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_rv = 0.0;

        let assign13020_e18356: f64 = (p.p722 / p.p76);
        let assign13020_e18358: f64 = (assign13020_e18356).max(1e-38);
        let assign13020_e18359: f64 = (assign13020_e18358).ln();
        let assign13020_e18360: f64 = (locals.var_ntox_i * assign13020_e18359);
        let assign13020_e18361: f64 = { let limited_exp_arg = assign13020_e18360; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13020_e18363: f64 = (assign13020_e18361 / locals.var_t0);
        locals.var_toxratio = assign13020_e18363;
        locals.var_toxratio_dn3 = (-((assign13020_e18361 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn4 = (-((assign13020_e18361 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn5 = (-((assign13020_e18361 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn6 = (-((assign13020_e18361 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn7 = (-((assign13020_e18361 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn8 = (-((assign13020_e18361 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn9 = (-((assign13020_e18361 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn10 = (-((assign13020_e18361 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn11 = (-((assign13020_e18361 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_rv = 0.0;

        let (assign13050_e18386,) = {
    if (p.p30 == 1.0) {
        (p.p705,)
    } else {
        (p.p704,)
    }
};
        locals.var_bechvb = assign13050_e18386;
        locals.var_bechvb_rv = 0.0;

        let assign13080_e18406: f64 = (-locals.var_bechvb);
        let assign13080_e18408: f64 = (assign13080_e18406 * p.p76);
        let assign13080_e18410: f64 = (assign13080_e18408 * locals.var_poxedge_i);
        locals.var_bechvbedge = assign13080_e18410;
        locals.var_bechvbedge_rv = 0.0;

        let assign13100_e18425: f64 = (-locals.var_bechvb);
        let assign13100_e18427: f64 = (assign13100_e18425 * p.p76);
        locals.var_bechvb = assign13100_e18427;
        locals.var_bechvb_rv = 0.0;

        let assign13110_e18430: f64 = (p.p1101 + locals.var_weff);
        locals.var_weff_sh = assign13110_e18430;
        locals.var_weff_sh_rv = 0.0;

        let assign13150_e18459: f64 = if (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard431 = assign13150_e18459;
        locals.var_guard431_rv = 0.0;

        let (assign13160_e18467,) = {
    if (locals.var_guard431 != 0.0) {
        let assign13160_e18463: f64 = (locals.var_weff_sh * p.p2);
        let assign13160_e18465: f64 = (assign13160_e18463 / p.p1099);
        (assign13160_e18465,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign13160_e18467;
        locals.var_gth_rv = 0.0;

        let (assign13170_e18475,) = {
    if (locals.var_guard431 != 0.0) {
        let assign13170_e18471: f64 = (p.p1100 * locals.var_weff_sh);
        let assign13170_e18473: f64 = (assign13170_e18471 * p.p2);
        (assign13170_e18473,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign13170_e18475;
        locals.var_cth_rv = 0.0;

        let (assign13180_e18480,) = {
    if (locals.var_guard431 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gth,)
    }
};
        locals.var_gth = assign13180_e18480;
        locals.var_gth_rv = 0.0;

        let (assign13190_e18485,) = {
    if (locals.var_guard431 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign13190_e18485;
        locals.var_cth_rv = 0.0;

        let assign13200_e18488: f64 = (-273.15);
        let assign13200_e18489: f64 = if p.p1028 <= assign13200_e18488 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign13200_e18489;
        locals.var_guard432_rv = 0.0;

        let (assign13210_e18495, assign13210_e18495_d_n3, assign13210_e18495_d_n4, assign13210_e18495_d_n5, assign13210_e18495_d_n6, assign13210_e18495_d_n7, assign13210_e18495_d_n8, assign13210_e18495_d_n9, assign13210_e18495_d_n10, assign13210_e18495_d_n11,) = {
    if (locals.var_guard432 != 0.0) {
        let assign13210_e18493: f64 = (300.15 - 273.15);
        (assign13210_e18493, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign13210_e18495;
        locals.var_t0_dn3 = assign13210_e18495_d_n3;
        locals.var_t0_dn4 = assign13210_e18495_d_n4;
        locals.var_t0_dn5 = assign13210_e18495_d_n5;
        locals.var_t0_dn6 = assign13210_e18495_d_n6;
        locals.var_t0_dn7 = assign13210_e18495_d_n7;
        locals.var_t0_dn8 = assign13210_e18495_d_n8;
        locals.var_t0_dn9 = assign13210_e18495_d_n9;
        locals.var_t0_dn10 = assign13210_e18495_d_n10;
        locals.var_t0_dn11 = assign13210_e18495_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign13220_e18499,) = {
    if (locals.var_guard432 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13220_e18499;
        locals.var_tnom_rv = 0.0;

        let (assign13230_e18506,) = {
    if (locals.var_guard432 == 0.0) {
        let assign13230_e18504: f64 = (p.p1028 + 273.15);
        (assign13230_e18504,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13230_e18506;
        locals.var_tnom_rv = 0.0;

        let assign13240_e18507: f64 = ctx_temp;
        let assign13240_e18509: f64 = (assign13240_e18507 + p.p23);
        locals.var_devtemp = assign13240_e18509;
        locals.var_devtemp_dn4 = 0.0;
        locals.var_devtemp_dn5 = 0.0;
        locals.var_devtemp_rv = 0.0;

        let assign13250_e18516: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard433 = assign13250_e18516;
        locals.var_guard433_rv = 0.0;

        let assign13260_e18523: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard434 = assign13260_e18523;
        locals.var_guard434_rv = 0.0;

        let assign13270_e18525: f64 = 1.0;
        locals.var_guard435 = assign13270_e18525;
        locals.var_guard435_rv = 0.0;

        let (assign13280_e18533, assign13280_e18533_d_n4, assign13280_e18533_d_n5,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) && (locals.var_guard435 != 0.0)) {
        ((nv4 - 0.0), 1.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13280_e18533;
        locals.var_deltemp1_dn4 = assign13280_e18533_d_n4;
        locals.var_deltemp1_dn5 = assign13280_e18533_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let (assign13290_e18542, assign13290_e18542_d_n4, assign13290_e18542_d_n5,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) && (locals.var_guard435 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13290_e18542;
        locals.var_deltemp1_dn4 = assign13290_e18542_d_n4;
        locals.var_deltemp1_dn5 = assign13290_e18542_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let (assign13300_e18549, assign13300_e18549_d_n4, assign13300_e18549_d_n5,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13300_e18549;
        locals.var_deltemp1_dn4 = assign13300_e18549_d_n4;
        locals.var_deltemp1_dn5 = assign13300_e18549_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let (assign13310_e18554, assign13310_e18554_d_n4, assign13310_e18554_d_n5,) = {
    if (locals.var_guard433 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4, locals.var_deltemp1_dn5,)
    }
};
        locals.var_deltemp1 = assign13310_e18554;
        locals.var_deltemp1_dn4 = assign13310_e18554_d_n4;
        locals.var_deltemp1_dn5 = assign13310_e18554_d_n5;
        locals.var_deltemp1_rv = 0.0;

        let assign13320_e18557: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        locals.var_devtemp = assign13320_e18557;
        locals.var_devtemp_dn4 = (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4);
        locals.var_devtemp_dn5 = (locals.var_deltemp1_dn5 + locals.var_devtemp_dn5);
        locals.var_devtemp_rv = 0.0;

        let assign13360_e18565: f64 = (locals.var_kboq * locals.var_devtemp);
        locals.var_vt = assign13360_e18565;
        locals.var_vt_dn4 = (locals.var_kboq * locals.var_devtemp_dn4);
        locals.var_vt_dn5 = (locals.var_kboq * locals.var_devtemp_dn5);
        locals.var_vt_rv = 0.0;

        let assign13370_e18568: f64 = (1.0 / locals.var_vt);
        locals.var_inv_vt = assign13370_e18568;
        locals.var_inv_vt_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_dn5 = (-(locals.var_vt_dn5 / (locals.var_vt * locals.var_vt)));
        locals.var_inv_vt_rv = 0.0;

        let assign13380_e18571: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign13380_e18571;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tratio_dn5 = (locals.var_devtemp_dn5 / locals.var_tnom);
        locals.var_tratio_rv = 0.0;

        let assign13390_e18574: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign13390_e18574;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;
        locals.var_deltemp_dn5 = locals.var_devtemp_dn5;
        locals.var_deltemp_rv = 0.0;

        let assign13400_e18577: f64 = (locals.var_kboq * locals.var_devtemp);
        locals.var_vtm = assign13400_e18577;
        locals.var_vtm_dn4 = (locals.var_kboq * locals.var_devtemp_dn4);
        locals.var_vtm_dn5 = (locals.var_kboq * locals.var_devtemp_dn5);
        locals.var_vtm_rv = 0.0;

        let assign13410_e18580: f64 = (locals.var_kboq * locals.var_tnom);
        locals.var_vtm0 = assign13410_e18580;
        locals.var_vtm0_rv = 0.0;

        let assign13420_e18584: f64 = (p.p1029 * locals.var_devtemp);
        let assign13420_e18586: f64 = (assign13420_e18584 * locals.var_devtemp);
        let assign13420_e18589: f64 = (locals.var_devtemp + p.p1030);
        let assign13420_e18590: f64 = (assign13420_e18586 / assign13420_e18589);
        let assign13420_e18591: f64 = (p.p108 - assign13420_e18590);
        locals.var_eg = assign13420_e18591;
        locals.var_eg_dn4 = (-((((((p.p1029 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13420_e18584 * locals.var_devtemp_dn4)) * assign13420_e18589) - (assign13420_e18586 * locals.var_devtemp_dn4)) / (assign13420_e18589 * assign13420_e18589)));
        locals.var_eg_dn5 = (-((((((p.p1029 * locals.var_devtemp_dn5) * locals.var_devtemp) + (assign13420_e18584 * locals.var_devtemp_dn5)) * assign13420_e18589) - (assign13420_e18586 * locals.var_devtemp_dn5)) / (assign13420_e18589 * assign13420_e18589)));
        locals.var_eg_rv = 0.0;

    }
}
