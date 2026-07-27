#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_scalar(160, p[433]);s.store_scalar(161, p[432]);s.store_primal_add_scaled_inputs3_offset_indices(348, 314, p[815], 316, p[816], 318, p[817], p[814]);s.store_primal_add_scaled_inputs3_offset_indices(349, 314, p[819], 316, p[820], 318, p[821], p[818]);s.store_primal_add_scaled_inputs3_mixed_aai(167, A::div_scaled_inputs2(s.ad_value(329), ((0.3333333333333333 * 1.0 / (s.v[18])) * p[442]), s.ad_value(330), p[442], s.ad_value(328), s.v[18]), 1.0, A::div_from_scalar((p[440] + p[441]), A::mul(s.ad_value(329), s.ad_value(327))), 1.0, 5, p[439]);}
        if s.b[1030] {s.store_scalar(168, (if (p[444] > 0.0) { p[444] } else { 0.0 }));}
        if s.b[1030] {s.store_scalar(169, (if (p[445] > 0.0) { p[445] } else { 0.0 }));}
        s.b[1044] = (p[44] == 0.0);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1044]) {s.copy_ad(169, 168);}
        if s.b[1030] {s.store_primal_scaled_mul(170, 5, 168, p[12]);s.store_primal_scaled_mul(171, 5, 169, p[13]);s.store_primal_scale(172, 5, p[447]);s.store_primal_scale(173, 5, p[446]);s.store_primal_scale(174, 5, p[448]);s.store_primal_scale(175, 5, p[449]);s.store_scalar(176, p[450]);}
        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1045]) {s.store_primal_add_scaled_inputs3_offset_indices(44, 314, p[452], 316, p[453], 318, p[454], p[451]);}
        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1046]) {s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p[456], 316, p[457], 318, p[458], p[455]);}
        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1047]) {s.store_primal_add_scaled_inputs3_offset_indices(49, 314, p[460], 316, p[461], 318, p[462], p[459]);}
        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1048]) {s.store_primal_add_scaled_inputs3_offset_indices(50, 314, p[464], 316, p[465], 318, p[466], p[463]);}
        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1049]) {s.store_primal_add_scaled_inputs3_offset_indices(51, 314, p[468], 316, p[469], 318, p[470], p[467]);}
        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1050]) {s.store_primal_add_scaled_inputs3_offset_indices(53, 314, p[472], 316, p[473], 318, p[474], p[471]);}
        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1051]) {s.store_primal_add_scaled_inputs3_offset_indices(54, 314, p[476], 316, p[477], 318, p[478], p[475]);}
        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1052]) {s.store_primal_add_scaled_inputs3_offset_indices(61, 314, p[480], 316, p[481], 318, p[482], p[479]);}
        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1053]) {s.store_primal_add_scaled_inputs3_offset_indices(62, 314, p[484], 316, p[485], 318, p[486], p[483]);}
        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1054]) {s.store_primal_add_scaled_inputs3_offset_indices(55, 314, p[488], 316, p[489], 318, p[490], p[487]);}
        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1055]) {s.store_primal_add_scaled_inputs3_offset_indices(56, 314, p[496], 316, p[497], 318, p[498], p[495]);}
        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1056]) {s.store_primal_add_scaled_inputs3_offset_indices(57, 314, p[492], 316, p[493], 318, p[494], p[491]);}
        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1057]) {s.store_primal_add_scaled_inputs3_offset_indices(58, 314, p[500], 316, p[501], 318, p[502], p[499]);}
        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1058]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(66, 315, 314, p[504], 316, p[505], 318, p[506], p[503]);}
        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1059]) {s.store_primal_add_scaled_inputs3_offset_indices(67, 314, p[512], 316, p[513], 318, p[514], p[511]);}
        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1060]) {s.store_primal_add_scaled_inputs3_offset_indices(68, 314, p[508], 316, p[509], 318, p[510], p[507]);}
        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1061]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(63, 315, 314, p[516], 316, p[517], 318, p[518], p[515]);}
        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1062]) {s.store_primal_add_scaled_inputs3_offset_indices(64, 314, p[524], 316, p[525], 318, p[526], p[523]);}
        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1063]) {s.store_primal_add_scaled_inputs3_offset_indices(65, 314, p[520], 316, p[521], 318, p[522], p[519]);}
        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1064]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(69, A::add_scaled_inputs3_offset(s.ad_value(314), p[528], s.ad_value(316), p[529], s.ad_value(318), p[530], p[527]), 313, 1.0, 312, 1.0);}
        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1065]) {s.store_primal_add_scaled_inputs3_offset_indices(70, 314, p[532], 316, p[533], 318, p[534], p[531]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1066]) {s.store_primal_add_scaled_inputs3_offset_indices(71, 314, p[536], 316, p[537], 318, p[538], p[535]);}
        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1067]) {s.store_primal_add_scaled_inputs3_offset_indices(73, 314, p[540], 316, p[541], 318, p[542], p[539]);}
        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1068]) {s.store_primal_add_scaled_inputs3_offset_indices(75, 314, p[544], 316, p[545], 318, p[546], p[543]);}
        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1069]) {s.store_primal_add_scaled_inputs3_offset_indices(77, 314, p[548], 316, p[549], 318, p[550], p[547]);}
        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1070]) {s.store_primal_add_scaled_inputs3_offset_indices(79, 314, p[552], 316, p[553], 318, p[554], p[551]);}
        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1071]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(82, 316, 314, p[556], 316, p[557], 318, p[558], p[555]);}
        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1072]) {s.store_primal_add_scaled_inputs3_offset_indices(83, 314, p[560], 316, p[561], 318, p[562], p[559]);}
        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1073]) {s.store_primal_add_scaled_inputs3_offset_indices(84, 314, p[564], 316, p[565], 318, p[566], p[563]);}
        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1074]) {s.store_primal_add_scaled_inputs3_offset_indices(85, 314, p[568], 316, p[569], 318, p[570], p[567]);}
        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1075]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(86, 314, 314, p[572], 316, p[573], 318, p[574], p[571]);}
        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1076]) {s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p[576], 316, p[577], 318, p[578], p[575]);}
        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1077]) {s.store_primal_add_scaled_inputs3_offset_indices(88, 314, p[580], 316, p[581], 318, p[582], p[579]);}
        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1078]) {s.store_primal_add_scaled_inputs3_offset_indices(89, 314, p[584], 316, p[585], 318, p[586], p[583]);}
        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1079]) {s.store_primal_add_scaled_inputs3_offset_indices(91, 314, p[588], 316, p[589], 318, p[590], p[587]);}
        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1080]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(92, 314, 314, p[592], 316, p[593], 318, p[594], p[591]);}
        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1081]) {s.store_primal_add_scaled_inputs3_offset_indices(93, 314, p[596], 316, p[597], 318, p[598], p[595]);}
        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1082]) {s.store_primal_add_scaled_inputs3_offset_indices(94, 314, p[600], 316, p[601], 318, p[602], p[599]);}
        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1083]) {s.store_primal_add_scaled_inputs3_offset_indices(96, 314, p[604], 316, p[605], 318, p[606], p[603]);}
        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1084]) {s.store_primal_add_scaled_inputs3_offset_indices(98, 314, p[608], 316, p[609], 318, p[610], p[607]);}
        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1085]) {s.store_primal_add_scaled_inputs3_offset_indices(99, 314, p[612], 316, p[613], 318, p[614], p[611]);}
        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1086]) {s.store_primal_add_scaled_inputs3_offset_indices(100, 314, p[616], 316, p[617], 318, p[618], p[615]);}
        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1087]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(103, 319, 314, p[620], 316, p[621], 318, p[622], p[619]);}
        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1088]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(104, 317, 314, p[624], 316, p[625], 318, p[626], p[623]);}
        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1089]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(105, 317, 314, p[628], 316, p[629], 318, p[630], p[627]);}
        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1090]) {s.store_primal_add_scaled_inputs3_offset_indices(106, 314, p[632], 316, p[633], 318, p[634], p[631]);}
        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1091]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(114, 317, 314, p[636], 316, p[637], 318, p[638], p[635]);}
        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1092]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(115, 317, 314, p[640], 316, p[641], 318, p[642], p[639]);}
        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1093]) {s.store_primal_add_scaled_inputs3_offset_indices(118, 314, p[644], 316, p[645], 318, p[646], p[643]);}
        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1094]) {s.store_primal_add_scaled_inputs3_offset_indices(119, 314, p[648], 316, p[649], 318, p[650], p[647]);}
        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1095]) {s.store_primal_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p[652], s.ad_value(316), p[653], s.ad_value(318), p[654], p[651]), 1.0 / (1e-6), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1096]) {s.store_primal_add_scaled_inputs3_offset_indices(123, 314, p[656], 316, p[657], 318, p[658], p[655]);}
        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1097]) {s.store_primal_add_scaled_inputs3_offset_indices(124, 314, p[660], 316, p[661], 318, p[662], p[659]);}
        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1098]) {s.store_scalar(32, p[571]);}
        s.b[1099] = param_given[663];s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {s.store_scalar(32, p[663]);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(33, p[572]);}
        s.b[1100] = param_given[664];s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {s.store_scalar(33, p[664]);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(34, p[573]);}
        s.b[1101] = param_given[665];s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {s.store_scalar(34, p[665]);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(35, p[574]);}
        s.b[1102] = param_given[666];s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {s.store_scalar(35, p[666]);}
        if (s.b[1030] && s.b[1098]) {s.store_primal_mul_mixed_ia(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));}
        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1103]) {s.store_scalar(32, p[587]);}
        s.b[1104] = param_given[667];s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {s.store_scalar(32, p[667]);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(33, p[588]);}
        s.b[1105] = param_given[668];s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {s.store_scalar(33, p[668]);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(34, p[589]);}
        s.b[1106] = param_given[669];s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {s.store_scalar(34, p[669]);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(35, p[590]);}
        s.b[1107] = param_given[670];s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {s.store_scalar(35, p[670]);}
        if (s.b[1030] && s.b[1103]) {s.store_primal_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);}
        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1108]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(127, 314, 314, p[672], 316, p[673], 318, p[674], p[671]);}
        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1109]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(128, 314, 314, p[676], 316, p[677], 318, p[678], p[675]);}
        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1110]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(129, 322, 314, p[680], 316, p[681], 318, p[682], p[679]);}
        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1111]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(130, 322, 314, p[684], 316, p[685], 318, p[686], p[683]);}
        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1112]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(134, 325, 314, p[688], 316, p[689], 318, p[690], p[687]);}
        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1113]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(135, 322, 314, p[692], 316, p[693], 318, p[694], p[691]);}
        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1114]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(136, 322, 314, p[696], 316, p[697], 318, p[698], p[695]);}
        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1115]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(141, 326, 314, p[700], 316, p[701], 318, p[702], p[699]);}
        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1116]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(142, 326, 314, p[704], 316, p[705], 318, p[706], p[703]);}
        s.b[1117] = (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1117]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(144, 315, 314, p[708], 316, p[709], 318, p[710], p[707]);}
        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1121]) {s.store_primal_add_scaled_inputs3_offset_indices(149, 314, p[724], 316, p[725], 318, p[726], p[723]);}
        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1122]) {s.store_primal_add_scaled_inputs3_offset_indices(150, 314, p[728], 316, p[729], 318, p[730], p[727]);}
        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1123]) {s.store_primal_add_scaled_inputs3_offset_indices(151, 314, p[732], 316, p[733], 318, p[734], p[731]);}
        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1124]) {s.store_primal_add_scaled_inputs3_offset_indices(152, 314, p[736], 316, p[737], 318, p[738], p[735]);}
        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1030] && s.b[1125]) {s.store_primal_add_scaled_inputs3_offset_indices(153, 314, p[740], 316, p[741], 318, p[742], p[739]);}
        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1126]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(154, A::add_scaled_inputs3_offset(s.ad_value(314), p[744], s.ad_value(316), p[745], s.ad_value(318), p[746], p[743]), 344, 1.0, 312, 1.0);}
        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1127]) {s.store_primal_add_scaled_inputs3_offset_indices(155, 314, p[748], 316, p[749], 318, p[750], p[747]);}
        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1128]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(156, 315, 314, p[752], 316, p[753], 318, p[754], p[751]);}
        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1129]) {s.store_primal_add_scaled_inputs3_offset_indices(157, 314, p[756], 316, p[757], 318, p[758], p[755]);}
        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1130]) {s.store_primal_add_scaled_inputs3_offset_indices(158, 314, p[760], 316, p[761], 318, p[762], p[759]);}
        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1131]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(159, 315, 314, p[764], 316, p[765], 318, p[766], p[763]);}
        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1132]) {s.store_primal_add_scaled_inputs3_offset_indices(160, 314, p[772], 316, p[773], 318, p[774], p[771]);}
        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1133]) {s.store_primal_add_scaled_inputs3_offset_indices(161, 314, p[768], 316, p[769], 318, p[770], p[767]);}
        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1137]) {s.store_primal_add_scaled_inputs3_offset_indices(176, 314, p[788], 316, p[789], 318, p[790], p[787]);}
        if s.b[1030] {s.store_scalar(1019, 0.0);s.store_scalar(1020, 0.0);s.store_scalar(1018, 0.0);s.store_scalar(43, p[795]);}
        s.b[1138] = param_given[796];s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1138]) {s.store_scalar(43, p[796]);}
        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t2: usize = 0;
        while {
            let t0: f64 = (s.v[5] - 0.5);let t1: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;
            if t2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1030] && s.b[1139]) {s.store_primal_add_mixed_ia(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));s.store_primal_add_mixed_ia(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));s.store_primal_offset(1018, 1018, 1.0);}
        }
        if (s.b[1030] && s.b[1139]) {s.store_primal_mul(1003, 1019, 6);s.store_primal_mul(1004, 1020, 6);s.store_scalar(1005, (1.0 / (p[791] + (0.5 * s.v[7]))));s.store_scalar(1006, (1.0 / (p[792] + (0.5 * s.v[7]))));}
        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_primal_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }
        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p[793]) > 1e-9) {
                s.store_primal_offset_add(1017, 8, 311, p[793]);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }
        if (s.b[1030] && s.b[1139]) {s.store_primal_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p[801]);s.store_primal_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p[802]);s.store_primal_add_scaled_inputs_product_mixed_aiii(1007, A::scale_offset(s.ad_value(1014), p[798], 1.0), (1.0 + (p[797] * (s.v[352] - 1.0))), 1015, (p[799] * (1.0 + (p[797] * (s.v[352] - 1.0)))), 1014, 1015, (p[800] * (1.0 + (p[797] * (s.v[352] - 1.0)))));s.store_primal_div_scaled_inputs2_indices(1008, 1003, p[794], 1004, p[794], 1007, 1.0);s.store_primal_div_scaled_inputs2_indices(1009, 1005, p[794], 1006, p[794], 1007, 1.0);s.store_primal_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p[807]);s.store_primal_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p[808]);s.store_primal_add_scaled_inputs_product_mixed_aiii(1010, A::scale_offset(s.ad_value(1014), p[804], 1.0), 1.0, 1015, p[805], 1014, 1015, p[806]);s.store_primal_add_scaled_inputs4_indices(1012, 1003, 1.0, 1004, 1.0, 1005, -1.0, 1006, -1.0);s.store_primal_div_scaled_offset_numerator_mixed_ia(1013, 1008, 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);s.store_primal_mul(69, 69, 1013);s.store_primal_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p[795], 1.0), 1.0, A::scale_offset(s.ad_value(1008), p[795], 1.0), 1.0);s.store_primal_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);s.store_primal_mul(154, 154, 1013);s.store_primal_div_scaled_inputs_indices(1013, 1012, p[803], 1010, 1.0);s.store_primal_add(44, 44, 1013);s.store_primal_add(149, 149, 1013);s.store_primal_div_scaled_inputs_mixed_ia(1013, 1012, p[809], A::powf(s.ad_value(1010), p[810]), 1.0);s.store_primal_add(66, 66, 1013);s.store_primal_add(159, 159, 1013);}
        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {s.store_primal_offset(1012, 8, s.v[12]);s.store_scalar(1013, (1.0 / p[811]));s.store_primal_div_from_scalar_scaled_input(15, (p[811] * p[811]), 1012, s.v[12]);s.store_primal_div_scaled_add_product_mixed_aaai(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p[811])), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p[811])), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), 8, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {s.store_primal_div_scaled_add_product_mixed_aaai(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p[811])), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p[811])), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), 8, 1.0);}
        if (s.b[1030] && s.b[1140]) {s.store_primal_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p[812], 17, p[813]);s.store_primal_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);s.store_primal_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);}
        s.copy_ad(177, 44);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
    ) {
        s.copy_ad(178, 45);s.copy_ad(179, 46);s.copy_ad(181, 47);s.copy_ad(182, 48);
        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }
        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }
        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }
        s.copy_ad(186, 52);s.copy_ad(187, 53);
        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }
        s.copy_ad(192, 59);s.copy_ad(193, 60);
        if (s.v[61] > 1e23) {
            if (s.v[61] < 1e27) {
                s.copy_ad(194, 61);
            } else {
                s.store_scalar(194, 1e27);
            }
        } else {
            s.store_scalar(194, 1e23);
        }
        if (s.v[62] > 1e23) {
            if (s.v[62] < 1e27) {
                s.copy_ad(195, 62);
            } else {
                s.store_scalar(195, 1e27);
            }
        } else {
            s.store_scalar(195, 1e23);
        }
        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }
        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }
        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }
        s.copy_ad(180, 58);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }
        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }
        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }
        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }
        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }
        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }
        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        s.copy_ad(203, 70);
        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }
        s.copy_ad(205, 72);
        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }
        s.copy_ad(207, 74);
        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }
        s.copy_ad(209, 76);
        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }
        s.copy_ad(211, 78);
        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }
        s.copy_ad(213, 80);s.copy_ad(214, 81);
        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }
        s.copy_ad(216, 83);
        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }
        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        s.copy_ad(220, 87);
        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }
        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }
        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }
        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }
        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }
        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }
        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }
        s.copy_ad(228, 95);
        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }
        s.copy_ad(230, 97);s.copy_ad(231, 98);
        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }
        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }
        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }
        s.copy_ad(235, 102);
        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }
        if (s.v[104] > 0.0) {
            s.copy_ad(237, 104);
        } else {
            s.store_scalar(237, 0.0);
        }
        if (s.v[105] > 0.0) {
            s.copy_ad(238, 105);
        } else {
            s.store_scalar(238, 0.0);
        }
        s.copy_ad(239, 106);s.copy_ad(240, 107);s.copy_ad(241, 108);s.copy_ad(242, 109);s.copy_ad(243, 110);s.copy_ad(244, 111);s.copy_ad(245, 112);s.copy_ad(246, 113);
        if (s.v[114] > 0.0) {
            s.copy_ad(247, 114);
        } else {
            s.store_scalar(247, 0.0);
        }
        if (s.v[115] > 0.0) {
            s.copy_ad(248, 115);
        } else {
            s.store_scalar(248, 0.0);
        }
        s.copy_ad(249, 116);s.copy_ad(250, 117);s.copy_ad(251, 118);s.copy_ad(252, 119);s.copy_ad(253, 120);s.copy_ad(254, 121);
        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }
        s.copy_ad(256, 123);
        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }
        s.copy_ad(260, 127);
        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }
        if (s.v[129] > 0.0) {
            s.copy_ad(262, 129);
        } else {
            s.store_scalar(262, 0.0);
        }
        if (s.v[130] > 0.0) {
            s.copy_ad(263, 130);
        } else {
            s.store_scalar(263, 0.0);
        }
        s.copy_ad(264, 131);s.copy_ad(265, 132);s.copy_ad(266, 133);
        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }
        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }
        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }
        s.copy_ad(270, 137);s.copy_ad(271, 138);s.copy_ad(272, 139);s.copy_ad(273, 140);
        if (s.v[141] > 0.0) {
            s.copy_ad(274, 141);
        } else {
            s.store_scalar(274, 0.0);
        }
        if (s.v[142] > 0.0) {
            s.copy_ad(275, 142);
        } else {
            s.store_scalar(275, 0.0);
        }
        s.copy_ad(276, 143);
        if (s.v[144] > 0.0) {
            s.copy_ad(277, 144);
        } else {
            s.store_scalar(277, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        s.copy_ad(282, 149);s.copy_ad(283, 150);s.copy_ad(284, 151);
        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }
        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }
        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }
        s.copy_ad(288, 155);
        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }
        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }
        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }
        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }
        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }
        if (s.v[167] > 0.0) {
            s.copy_ad(300, 167);
        } else {
            s.store_scalar(300, 0.0);
        }
        s.copy_ad(301, 170);s.copy_ad(302, 171);s.copy_ad(303, 173);s.copy_ad(304, 174);s.copy_ad(305, 175);s.copy_ad(306, 172);
        if ((p[31] * s.v[5]) > 0.0) {
            s.store_primal_scale(19, 5, p[31]);
        } else {
            s.store_scalar(19, 0.0);
        }
        s.store_scalar(20, p[16]);s.store_scalar(21, p[15]);s.store_scalar(22, p[18]);s.store_scalar(23, p[17]);
        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }
        s.b[1142] = (p[44] == 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.copy_ad(193, 192);s.copy_ad(195, 194);s.copy_ad(248, 247);s.copy_ad(250, 249);s.copy_ad(252, 251);s.copy_ad(254, 253);s.copy_ad(238, 237);s.copy_ad(244, 242);s.copy_ad(245, 243);s.copy_ad(263, 262);s.copy_ad(265, 264);s.copy_ad(269, 268);s.copy_ad(275, 274);}
        s.store_primal_scale(768, 182, 8.8541878176e-12);s.store_primal_div(769, 768, 181);s.store_primal_square(770, 181);s.store_primal_scale(771, 769, 6.241449993689894e18);s.store_primal_mul(772, 257, 183);
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }
        s.store_scalar(773, 0.0);s.b[1143] = (p[52] > 0.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_primal_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p[52]));}
        s.b[1144] = (s.v[0] == (-1.0));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if (s.b[1143] && s.b[1144]) {s.store_primal_scale(773, 773, (7.448711 / 5.951993));}
        s.store_primal_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));s.store_primal_scale(775, 214, 0.5);s.store_scalar(776, 0.5);s.b[1145] = (s.v[0] == (-1.0));s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_primal_scale(775, 214, 0.3333333333333333);s.store_scalar(776, 0.3333333333333333);}
        s.store_primal_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(777, 1011, (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(778, 1011, (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_div_from_scalar(779, 1.0, 228);s.store_primal_div(780, 768, 192);s.store_primal_div(781, 768, 193);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        s.store_primal_div_mixed_ai(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);s.store_primal_div_mixed_ai(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);s.store_primal_square(784, 782);s.store_primal_square(785, 783);s.store_primal_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));s.store_primal_add_mixed_ai(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);s.store_primal_add_mixed_ai(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);s.store_primal_div_from_scalar(820, 1.0, 782);s.store_primal_offset_scaled(821, 782, 3.1, 8.5);s.store_primal_square(789, 821);s.store_primal_scale(822, 821, 0.5);s.b[1146] = (s.v[820] < 0.06);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_primal_scale(790, 820, 64.0);}
        s.b[1147] = (s.v[820] <= 0.45);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if ((!s.b[1146]) && s.b[1147]) {s.store_primal_offset_scaled(790, 820, 22.0, 3.0);}
        s.b[1148] = (s.v[820] <= 1.6);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {s.store_primal_offset_scaled(790, 820, (-7.2), 15.5);}
        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {s.copy_ad(790, 782);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));s.store_primal_div_from_scalar(820, 1.0, 783);s.store_primal_offset_scaled(821, 783, 3.1, 8.5);s.store_primal_square(792, 821);s.store_primal_scale(822, 821, 0.5);s.b[1149] = (s.v[820] < 0.06);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_primal_scale(793, 820, 64.0);}
        s.b[1150] = (s.v[820] <= 0.45);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if ((!s.b[1149]) && s.b[1150]) {s.store_primal_offset_scaled(793, 820, 22.0, 3.0);}
        s.b[1151] = (s.v[820] <= 1.6);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {s.store_primal_offset_scaled(793, 820, (-7.2), 15.5);}
        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {s.copy_ad(793, 783);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));s.store_primal_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));
        if (!(s.v[728] > 0.05)) {s.store_scalar(728, 0.05);}
        s.store_primal_div_mixed_ai(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);
    }
}
