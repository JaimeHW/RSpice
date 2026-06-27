#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_mul_ad(154, A::div_scaled_inputs(s.ad_value(344), p.p258, A::mul(s.ad_value(347), s.ad_value(312)), 1.0), A::scale_offset(s.ad_value(316), p.p419, 1.0));
            s.store_add_scaled_inputs3_offset(155, s.ad_value(314), p.p421, s.ad_value(316), p.p422, s.ad_value(318), p.p423, p.p420);
            s.store_scaled_mul_scale_offset_rhs_ad(156, A::powf(s.ad_value(314), p.p425), 316, p.p426, 1.0, p.p424);
            s.store_scalar(157, p.p427);
            s.store_scalar(158, p.p428);
            s.store_scaled_mul_scale_offset_rhs_ad(159, A::powf(s.ad_value(314), p.p430), 316, p.p431, 1.0, p.p429);
            s.store_scalar(160, p.p433);
            s.store_scalar(161, p.p432);
            s.store_add_scaled_inputs3_offset(348, s.ad_value(314), p.p815, s.ad_value(316), p.p816, s.ad_value(318), p.p817, p.p814);
            s.store_add_scaled_inputs3_offset(349, s.ad_value(314), p.p819, s.ad_value(316), p.p820, s.ad_value(318), p.p821, p.p818);
            s.store_scalar(176, p.p450);
        }

        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset(44, s.ad_value(314), p.p452, s.ad_value(316), p.p453, s.ad_value(318), p.p454, p.p451);
        }

        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset(45, s.ad_value(314), p.p456, s.ad_value(316), p.p457, s.ad_value(318), p.p458, p.p455);
        }

        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1047]) {
            s.store_add_scaled_inputs3_offset(49, s.ad_value(314), p.p460, s.ad_value(316), p.p461, s.ad_value(318), p.p462, p.p459);
        }

        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset(50, s.ad_value(314), p.p464, s.ad_value(316), p.p465, s.ad_value(318), p.p466, p.p463);
        }

        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset(51, s.ad_value(314), p.p468, s.ad_value(316), p.p469, s.ad_value(318), p.p470, p.p467);
        }

        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1050]) {
            s.store_add_scaled_inputs3_offset(53, s.ad_value(314), p.p472, s.ad_value(316), p.p473, s.ad_value(318), p.p474, p.p471);
        }

        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset(54, s.ad_value(314), p.p476, s.ad_value(316), p.p477, s.ad_value(318), p.p478, p.p475);
        }

        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset(61, s.ad_value(314), p.p480, s.ad_value(316), p.p481, s.ad_value(318), p.p482, p.p479);
        }

        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1053]) {
            s.store_add_scaled_inputs3_offset(62, s.ad_value(314), p.p484, s.ad_value(316), p.p485, s.ad_value(318), p.p486, p.p483);
        }

        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset(55, s.ad_value(314), p.p488, s.ad_value(316), p.p489, s.ad_value(318), p.p490, p.p487);
        }

        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset(56, s.ad_value(314), p.p496, s.ad_value(316), p.p497, s.ad_value(318), p.p498, p.p495);
        }

        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset(57, s.ad_value(314), p.p492, s.ad_value(316), p.p493, s.ad_value(318), p.p494, p.p491);
        }

        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset(58, s.ad_value(314), p.p500, s.ad_value(316), p.p501, s.ad_value(318), p.p502, p.p499);
        }

        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1058]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(66, 315, s.ad_value(314), p.p504, s.ad_value(316), p.p505, s.ad_value(318), p.p506, p.p503);
        }

        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset(67, s.ad_value(314), p.p512, s.ad_value(316), p.p513, s.ad_value(318), p.p514, p.p511);
        }

        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1060]) {
            s.store_add_scaled_inputs3_offset(68, s.ad_value(314), p.p508, s.ad_value(316), p.p509, s.ad_value(318), p.p510, p.p507);
        }

        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1061]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(63, 315, s.ad_value(314), p.p516, s.ad_value(316), p.p517, s.ad_value(318), p.p518, p.p515);
        }

        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset(64, s.ad_value(314), p.p524, s.ad_value(316), p.p525, s.ad_value(318), p.p526, p.p523);
        }

        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset(65, s.ad_value(314), p.p520, s.ad_value(316), p.p521, s.ad_value(318), p.p522, p.p519);
        }

        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1064]) {
            s.store_mul_ad(69, A::div(s.ad_value(313), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p528, s.ad_value(316), p.p529, s.ad_value(318), p.p530, p.p527));
        }

        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset(70, s.ad_value(314), p.p532, s.ad_value(316), p.p533, s.ad_value(318), p.p534, p.p531);
        }

        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset(71, s.ad_value(314), p.p536, s.ad_value(316), p.p537, s.ad_value(318), p.p538, p.p535);
        }

        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset(73, s.ad_value(314), p.p540, s.ad_value(316), p.p541, s.ad_value(318), p.p542, p.p539);
        }

        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset(75, s.ad_value(314), p.p544, s.ad_value(316), p.p545, s.ad_value(318), p.p546, p.p543);
        }

        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1069]) {
            s.store_add_scaled_inputs3_offset(77, s.ad_value(314), p.p548, s.ad_value(316), p.p549, s.ad_value(318), p.p550, p.p547);
        }

        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset(79, s.ad_value(314), p.p552, s.ad_value(316), p.p553, s.ad_value(318), p.p554, p.p551);
        }

        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1071]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 316, s.ad_value(314), p.p556, s.ad_value(316), p.p557, s.ad_value(318), p.p558, p.p555);
        }

        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset(83, s.ad_value(314), p.p560, s.ad_value(316), p.p561, s.ad_value(318), p.p562, p.p559);
        }

        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset(84, s.ad_value(314), p.p564, s.ad_value(316), p.p565, s.ad_value(318), p.p566, p.p563);
        }

        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset(85, s.ad_value(314), p.p568, s.ad_value(316), p.p569, s.ad_value(318), p.p570, p.p567);
        }

        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1075]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(86, 314, s.ad_value(314), p.p572, s.ad_value(316), p.p573, s.ad_value(318), p.p574, p.p571);
        }

        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1076]) {
            s.store_add_scaled_inputs3_offset(87, s.ad_value(314), p.p576, s.ad_value(316), p.p577, s.ad_value(318), p.p578, p.p575);
        }

        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1077]) {
            s.store_add_scaled_inputs3_offset(88, s.ad_value(314), p.p580, s.ad_value(316), p.p581, s.ad_value(318), p.p582, p.p579);
        }

        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1078]) {
            s.store_add_scaled_inputs3_offset(89, s.ad_value(314), p.p584, s.ad_value(316), p.p585, s.ad_value(318), p.p586, p.p583);
        }

        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset(91, s.ad_value(314), p.p588, s.ad_value(316), p.p589, s.ad_value(318), p.p590, p.p587);
        }

        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1080]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(92, 314, s.ad_value(314), p.p592, s.ad_value(316), p.p593, s.ad_value(318), p.p594, p.p591);
        }

        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1081]) {
            s.store_add_scaled_inputs3_offset(93, s.ad_value(314), p.p596, s.ad_value(316), p.p597, s.ad_value(318), p.p598, p.p595);
        }

        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset(94, s.ad_value(314), p.p600, s.ad_value(316), p.p601, s.ad_value(318), p.p602, p.p599);
        }

        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset(96, s.ad_value(314), p.p604, s.ad_value(316), p.p605, s.ad_value(318), p.p606, p.p603);
        }

        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1084]) {
            s.store_add_scaled_inputs3_offset(98, s.ad_value(314), p.p608, s.ad_value(316), p.p609, s.ad_value(318), p.p610, p.p607);
        }

        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset(99, s.ad_value(314), p.p612, s.ad_value(316), p.p613, s.ad_value(318), p.p614, p.p611);
        }

        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset(100, s.ad_value(314), p.p616, s.ad_value(316), p.p617, s.ad_value(318), p.p618, p.p615);
        }

        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1087]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(103, 319, s.ad_value(314), p.p620, s.ad_value(316), p.p621, s.ad_value(318), p.p622, p.p619);
        }

        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1088]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(104, 317, s.ad_value(314), p.p624, s.ad_value(316), p.p625, s.ad_value(318), p.p626, p.p623);
        }

        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1089]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 317, s.ad_value(314), p.p628, s.ad_value(316), p.p629, s.ad_value(318), p.p630, p.p627);
        }

        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1090]) {
            s.store_add_scaled_inputs3_offset(106, s.ad_value(314), p.p632, s.ad_value(316), p.p633, s.ad_value(318), p.p634, p.p631);
        }

        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1091]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(114, 317, s.ad_value(314), p.p636, s.ad_value(316), p.p637, s.ad_value(318), p.p638, p.p635);
        }

        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1092]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(115, 317, s.ad_value(314), p.p640, s.ad_value(316), p.p641, s.ad_value(318), p.p642, p.p639);
        }

        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1093]) {
            s.store_add_scaled_inputs3_offset(118, s.ad_value(314), p.p644, s.ad_value(316), p.p645, s.ad_value(318), p.p646, p.p643);
        }

        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1094]) {
            s.store_add_scaled_inputs3_offset(119, s.ad_value(314), p.p648, s.ad_value(316), p.p649, s.ad_value(318), p.p650, p.p647);
        }

        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1095]) {
            s.store_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p652, s.ad_value(316), p.p653, s.ad_value(318), p.p654, p.p651), 1.0 / (1e-6), 0.0);
        }

        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1096]) {
            s.store_add_scaled_inputs3_offset(123, s.ad_value(314), p.p656, s.ad_value(316), p.p657, s.ad_value(318), p.p658, p.p655);
        }

        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1097]) {
            s.store_add_scaled_inputs3_offset(124, s.ad_value(314), p.p660, s.ad_value(316), p.p661, s.ad_value(318), p.p662, p.p659);
        }

        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(32, p.p571);
        }

        s.b[1099] = param_given[663];
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {
            s.store_scalar(32, p.p663);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(33, p.p572);
        }

        s.b[1100] = param_given[664];
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {
            s.store_scalar(33, p.p664);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(34, p.p573);
        }

        s.b[1101] = param_given[665];
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {
            s.store_scalar(34, p.p665);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(35, p.p574);
        }

        s.b[1102] = param_given[666];
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {
            s.store_scalar(35, p.p666);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_mul_ad_rhs(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));
        }

        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(32, p.p587);
        }

        s.b[1104] = param_given[667];
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {
            s.store_scalar(32, p.p667);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(33, p.p588);
        }

        s.b[1105] = param_given[668];
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {
            s.store_scalar(33, p.p668);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(34, p.p589);
        }

        s.b[1106] = param_given[669];
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {
            s.store_scalar(34, p.p669);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(35, p.p590);
        }

        s.b[1107] = param_given[670];
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {
            s.store_scalar(35, p.p670);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);
        }

        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1108]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(127, 314, s.ad_value(314), p.p672, s.ad_value(316), p.p673, s.ad_value(318), p.p674, p.p671);
        }

        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1109]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(128, 314, s.ad_value(314), p.p676, s.ad_value(316), p.p677, s.ad_value(318), p.p678, p.p675);
        }

        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1110]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(129, 322, s.ad_value(314), p.p680, s.ad_value(316), p.p681, s.ad_value(318), p.p682, p.p679);
        }

        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1111]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 322, s.ad_value(314), p.p684, s.ad_value(316), p.p685, s.ad_value(318), p.p686, p.p683);
        }

        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1112]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(134, 325, s.ad_value(314), p.p688, s.ad_value(316), p.p689, s.ad_value(318), p.p690, p.p687);
        }

        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1113]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 322, s.ad_value(314), p.p692, s.ad_value(316), p.p693, s.ad_value(318), p.p694, p.p691);
        }

        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1114]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(136, 322, s.ad_value(314), p.p696, s.ad_value(316), p.p697, s.ad_value(318), p.p698, p.p695);
        }

        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1115]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(141, 326, s.ad_value(314), p.p700, s.ad_value(316), p.p701, s.ad_value(318), p.p702, p.p699);
        }

        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1116]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(142, 326, s.ad_value(314), p.p704, s.ad_value(316), p.p705, s.ad_value(318), p.p706, p.p703);
        }

        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset(149, s.ad_value(314), p.p724, s.ad_value(316), p.p725, s.ad_value(318), p.p726, p.p723);
        }

        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset(150, s.ad_value(314), p.p728, s.ad_value(316), p.p729, s.ad_value(318), p.p730, p.p727);
        }

        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1123]) {
            s.store_add_scaled_inputs3_offset(151, s.ad_value(314), p.p732, s.ad_value(316), p.p733, s.ad_value(318), p.p734, p.p731);
        }

        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1124]) {
            s.store_add_scaled_inputs3_offset(152, s.ad_value(314), p.p736, s.ad_value(316), p.p737, s.ad_value(318), p.p738, p.p735);
        }

        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1125]) {
            s.store_add_scaled_inputs3_offset(153, s.ad_value(314), p.p740, s.ad_value(316), p.p741, s.ad_value(318), p.p742, p.p739);
        }

        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1126]) {
            s.store_mul_ad(154, A::div(s.ad_value(344), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p744, s.ad_value(316), p.p745, s.ad_value(318), p.p746, p.p743));
        }

        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1127]) {
            s.store_add_scaled_inputs3_offset(155, s.ad_value(314), p.p748, s.ad_value(316), p.p749, s.ad_value(318), p.p750, p.p747);
        }

        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1128]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(156, 315, s.ad_value(314), p.p752, s.ad_value(316), p.p753, s.ad_value(318), p.p754, p.p751);
        }

        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1129]) {
            s.store_add_scaled_inputs3_offset(157, s.ad_value(314), p.p756, s.ad_value(316), p.p757, s.ad_value(318), p.p758, p.p755);
        }

        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1130]) {
            s.store_add_scaled_inputs3_offset(158, s.ad_value(314), p.p760, s.ad_value(316), p.p761, s.ad_value(318), p.p762, p.p759);
        }

        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1131]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(159, 315, s.ad_value(314), p.p764, s.ad_value(316), p.p765, s.ad_value(318), p.p766, p.p763);
        }

        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1132]) {
            s.store_add_scaled_inputs3_offset(160, s.ad_value(314), p.p772, s.ad_value(316), p.p773, s.ad_value(318), p.p774, p.p771);
        }

        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1133]) {
            s.store_add_scaled_inputs3_offset(161, s.ad_value(314), p.p768, s.ad_value(316), p.p769, s.ad_value(318), p.p770, p.p767);
        }

        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1137]) {
            s.store_add_scaled_inputs3_offset(176, s.ad_value(314), p.p788, s.ad_value(316), p.p789, s.ad_value(318), p.p790, p.p787);
        }

        if s.b[1030] {
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(43, p.p795);
        }

        s.b[1138] = param_given[796];
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1138]) {
            s.store_scalar(43, p.p796);
        }

        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (s.v[5] - 0.5);
            let assign9340_cond_e9224: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1030] && s.b[1139]) {
                s.store_add_ad_rhs(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));
                s.store_add_ad_rhs(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));
                s.store_offset(1018, 1018, 1.0);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_mul(1003, 1019, 6);
            s.store_mul(1004, 1020, 6);
            s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * s.v[7]))));
            s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * s.v[7]))));
        }

        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) {
                s.store_offset_add(1017, 8, 311, p.p793);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p801);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p802);
            s.store_add_scaled_inputs_product_first_ad(1007, A::scale_offset(s.ad_value(1014), p.p798, 1.0), (1.0 + (p.p797 * (s.v[352] - 1.0))), 1015, (p.p799 * (1.0 + (p.p797 * (s.v[352] - 1.0)))), 1014, 1015, (p.p800 * (1.0 + (p.p797 * (s.v[352] - 1.0)))));
            s.store_div_scaled_inputs2(1008, s.ad_value(1003), p.p794, s.ad_value(1004), p.p794, s.ad_value(1007), 1.0);
            s.store_div_scaled_inputs2(1009, s.ad_value(1005), p.p794, s.ad_value(1006), p.p794, s.ad_value(1007), 1.0);
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p807);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p808);
            s.store_add_scaled_inputs_product_first_ad(1010, A::scale_offset(s.ad_value(1014), p.p804, 1.0), 1.0, 1015, p.p805, 1014, 1015, p.p806);
            s.store_add_scaled_inputs4(1012, s.ad_value(1003), 1.0, s.ad_value(1004), 1.0, s.ad_value(1005), -1.0, s.ad_value(1006), -1.0);
            s.store_div_scaled_offset_numerator(1013, s.ad_value(1008), 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);
            s.store_mul(69, 69, 1013);
            s.store_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p.p795, 1.0), 1.0, A::scale_offset(s.ad_value(1008), p.p795, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);
            s.store_mul(154, 154, 1013);
            s.store_div_scaled_inputs(1013, s.ad_value(1012), p.p803, s.ad_value(1010), 1.0);
            s.store_add(44, 44, 1013);
            s.store_add(149, 149, 1013);
            s.store_div_scaled_inputs(1013, s.ad_value(1012), p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);
            s.store_add(66, 66, 1013);
            s.store_add(159, 159, 1013);
        }

        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {
            s.store_offset(1012, 8, s.v[12]);
            s.store_scalar(1013, (1.0 / p.p811));
            s.store_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);
            s.store_div_scaled_add_product(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p.p811)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
            s.store_div_scaled_add_product(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p.p811)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
        }

        if (s.b[1030] && s.b[1140]) {
            s.store_add_scaled_inputs3(1012, s.ad_value(15), 1.0, s.ad_value(16), p.p812, s.ad_value(17), p.p813);
            s.store_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
            s.store_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
        }

        s.copy_ad(177, 44);

        s.copy_ad(178, 45);

        s.copy_ad(179, 46);

        s.copy_ad(181, 47);

        s.copy_ad(182, 48);

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

        s.copy_ad(186, 52);

        s.copy_ad(187, 53);

        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(192, 59);

        s.copy_ad(193, 60);

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

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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

        s.copy_ad(213, 80);

        s.copy_ad(214, 81);

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

        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }

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

        s.copy_ad(230, 97);

        s.copy_ad(231, 98);

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

        s.copy_ad(239, 106);

        s.copy_ad(240, 107);

        s.copy_ad(241, 108);

        s.copy_ad(242, 109);

        s.copy_ad(243, 110);

        s.copy_ad(244, 111);

        s.copy_ad(245, 112);

        s.copy_ad(246, 113);

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

        s.copy_ad(249, 116);

        s.copy_ad(250, 117);

        s.copy_ad(251, 118);

        s.copy_ad(252, 119);

        s.copy_ad(253, 120);

        s.copy_ad(254, 121);

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

        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }

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

        s.copy_ad(264, 131);

        s.copy_ad(265, 132);

        s.copy_ad(266, 133);

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

        s.copy_ad(270, 137);

        s.copy_ad(271, 138);

        s.copy_ad(272, 139);

        s.copy_ad(273, 140);

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

        s.copy_ad(282, 149);

        s.copy_ad(283, 150);

        s.copy_ad(284, 151);

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

        if ((p.p31 * s.v[5]) > 0.0) {
            s.store_scale(19, 5, p.p31);
        } else {
            s.store_scalar(19, 0.0);
        }

        s.v[20] = p.p16;

        s.v[21] = p.p15;

        s.v[22] = p.p18;

        s.v[23] = p.p17;

        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }

        s.b[1142] = (p.p44 == 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.copy_ad(193, 192);
            s.copy_ad(195, 194);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(254, 253);
            s.copy_ad(238, 237);
            s.copy_ad(244, 242);
            s.copy_ad(245, 243);
            s.copy_ad(263, 262);
            s.copy_ad(265, 264);
            s.copy_ad(269, 268);
            s.copy_ad(275, 274);
        }

        s.store_scale(768, 182, 8.8541878176e-12);

        s.store_div(769, 768, 181);

        s.store_square(770, 181);

        s.store_scale(771, 769, 6.241449993689894e18);

        s.store_mul(772, 257, 183);

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }

        s.v[773] = 0.0;

        s.b[1143] = (p.p52 > 0.0);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));
        }

        s.b[1144] = (s.v[0] == (-1.0));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if (s.b[1143] && s.b[1144]) {
            s.store_scale(773, 773, (7.448711 / 5.951993));
        }

        s.store_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));

        s.store_scale(775, 214, 0.5);

        s.v[776] = 0.5;

        s.b[1145] = (s.v[0] == (-1.0));
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_scale(775, 214, 0.3333333333333333);
            s.store_scalar(776, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(777, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(778, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(779, 1.0, 228);

        s.store_div(780, 768, 192);

        s.store_div(781, 768, 193);

        s.store_div_ad_lhs(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);

        s.store_div_ad_lhs(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);

        s.store_square(784, 782);

        s.store_square(785, 783);

        s.store_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);

        s.store_add_ad_lhs(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);

        s.store_div_from_scalar(820, 1.0, 782);

        s.store_offset_scaled(821, 782, 3.1, 8.5);

        s.store_square(789, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1146] = (s.v[820] < 0.06);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_scale(790, 820, 64.0);
        }

        s.b[1147] = (s.v[820] <= 0.45);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if ((!s.b[1146]) && s.b[1147]) {
            s.store_offset_scaled(790, 820, 22.0, 3.0);
        }

        s.b[1148] = (s.v[820] <= 1.6);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {
            s.store_offset_scaled(790, 820, (-7.2), 15.5);
        }

        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {
            s.copy_ad(790, 782);
        }

        s.store_add_scaled_inputs_product_right_ad(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));

        s.store_div_from_scalar(820, 1.0, 783);

        s.store_offset_scaled(821, 783, 3.1, 8.5);

        s.store_square(792, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1149] = (s.v[820] < 0.06);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_scale(793, 820, 64.0);
        }

        s.b[1150] = (s.v[820] <= 0.45);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if ((!s.b[1149]) && s.b[1150]) {
            s.store_offset_scaled(793, 820, 22.0, 3.0);
        }

        s.b[1151] = (s.v[820] <= 1.6);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {
            s.store_offset_scaled(793, 820, (-7.2), 15.5);
        }

        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {
            s.copy_ad(793, 783);
        }

        s.store_add_scaled_inputs_product_right_ad(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));

        if (!(s.v[728] > 0.05)) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.v[730] = 0.0;

        s.v[731] = 0.0;

        s.b[1152] = (s.v[188] > 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

        if s.b[1152] {
            if (s.v[188] > s.v[732]) {
                s.copy_ad(731, 188);
            } else {
                s.copy_ad(731, 732);
            }
        }

        if s.b[1152] {
            if (5e24 > s.v[731]) {
                s.store_scalar(731, 5e24);
            } else {
            }
        }

        if s.b[1152] {
            s.store_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * s.v[767]));
        }

        s.v[733] = ((100.0 * s.v[715]) * s.v[715]);

        s.b[1153] = (p.p52 > 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));
            s.store_mul_scaled_ad_rhs(735, 773, 0.75, A::powf(s.ad_value(734), 0.6666666666666666));
            s.store_add(728, 728, 735);
            s.store_mul_offset_ad_rhs(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_scaled_mul(738, 728, 728, 0.0025);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(738), 1.0, A::sub(s.ad_value(737), s.ad_value(740)), A::sub(s.ad_value(737), s.ad_value(740)), 1.0), (-0.5));

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);

        s.store_add_scaled_inputs3_offset(745, s.ad_value(187), 1.0, s.ad_value(256), 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);

        if (!(s.v[745] > 0.05)) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.b[1154] = (p.p52 > 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));
            s.store_mul_scaled_ad_rhs(735, 773, 0.75, A::powf(s.ad_value(734), 0.6666666666666666));
            s.store_add(745, 745, 735);
            s.store_mul_offset_ad_rhs(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_scale(747, 745, 0.95);

        s.store_scaled_mul(748, 745, 745, 0.0025);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(748), 1.0, A::sub(s.ad_value(747), s.ad_value(740)), A::sub(s.ad_value(747), s.ad_value(740)), 1.0), (-0.5));

        s.store_offset_add_scaled_product(700, s.ad_value(177), 1.0, s.ad_value(178), A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);

        s.store_exp_scaled_input(751, 180, s.v[360]);

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_scaled_input(752, 203, s.v[360]);

        s.store_mul(703, 202, 752);

        s.store_scaled_mul(716, 703, 769, s.v[20]);

        s.store_mul_ad_rhs(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));

        s.store_exp_scaled_input(753, 205, s.v[360]);

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));

        s.store_exp_scaled_input(754, 209, s.v[360]);

        s.store_mul(706, 208, 754);

        s.store_exp_scaled_input(755, 213, s.v[360]);

        s.store_mul(708, 212, 755);

        s.store_exp_scaled_input(756, 216, s.v[360]);

        s.store_mul(709, 215, 756);

        s.store_scaled_mul(757, 716, 709, 2.0);

        s.store_exp_scaled_input(758, 220, s.v[360]);

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if s.b[1155] {
            s.store_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);
            s.store_exp_scaled_input(759, 288, s.v[360]);
            s.store_mul(714, 287, 759);
            s.store_scaled_mul(717, 714, 769, s.v[22]);
            s.store_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);
            s.store_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }

        if s.b[1155] {
            s.store_div_ad_lhs(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);
            s.store_square(724, 761);
            s.store_ln(725, 724);
            s.store_scale(762, 760, 0.95);
            s.store_scaled_mul(763, 760, 760, 0.0025);
            s.copy_ad(764, 763);
            s.store_scaled_sqrt(765, 764, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(763), 1.0, A::sub(s.ad_value(762), s.ad_value(765)), A::sub(s.ad_value(762), s.ad_value(765)), 1.0), (-0.5));
        }

        if (!s.b[1155]) {
            s.store_scalar(713, 0.0);
            s.store_scalar(759, 1.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(723, s.v[715]);
            s.store_scalar(760, 0.0);
            s.store_scalar(761, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_mul(798, 796, 192);

        s.store_mul(799, 796, 193);

        s.v[800] = 0.0;

        s.b[1156] = (s.v[241] < 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_div_scaled_inputs(800, s.ad_value(240), (-0.495), s.ad_value(241), 1.0);
        }

        s.v[801] = 0.0;

        s.b[1157] = (s.v[243] < 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_div_scaled_inputs(801, s.ad_value(242), (-0.495), s.ad_value(243), 1.0);
        }

        s.b[1158] = (s.v[245] < 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_div_scaled_inputs(802, s.ad_value(244), (-0.495), s.ad_value(245), 1.0);
        }

        s.store_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));

        s.store_mul(236, 236, 803);

        s.store_mul(237, 237, 803);

        s.store_mul(238, 238, 803);

        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(710, 249, 796);

        s.store_scaled_mul(806, 710, 192, 500000000.0);

        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(711, 250, 796);

        s.store_scaled_mul(807, 711, 193, 500000000.0);

        s.v[808] = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1159] = (s.v[272] > 1e-10);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(24, 6, s.v[646]);

        s.store_scale(25, 6, s.v[647]);

        s.store_scale(26, 6, s.v[648]);

        s.store_scale(27, 6, s.v[673]);

        s.store_scale(28, 6, s.v[674]);

        s.store_scale(29, 6, s.v[675]);

        s.v[30] = 0.0;

        s.b[1167] = (p.p43 == 3.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if s.b[1167] {
            s.store_scalar(30, 1.0);
        }

        s.copy_ad(31, 313);

        s.b[1168] = (p.p39 == 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if s.b[1168] {
            s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));
        }

        s.b[1169] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if s.b[1169] {
            s.store_scale(24, 6, s.v[649]);
            s.store_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));
            s.copy_ad(26, 31);
            s.store_scale(27, 6, s.v[676]);
            s.store_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));
            s.copy_ad(29, 31);
        }

        s.b[1170] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if s.b[1170] {
            if (s.v[24] > 0.0) {
                s.copy_ad(646, 24);
            } else {
                s.store_scalar(646, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[25] > 0.0) {
                s.copy_ad(647, 25);
            } else {
                s.store_scalar(647, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[26] > 0.0) {
                s.copy_ad(648, 26);
            } else {
                s.store_scalar(648, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[27] > 0.0) {
                s.copy_ad(673, 27);
            } else {
                s.store_scalar(673, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[28] > 0.0) {
                s.copy_ad(674, 28);
            } else {
                s.store_scalar(674, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[29] > 0.0) {
                s.copy_ad(675, 29);
            } else {
                s.store_scalar(675, 0.0);
            }
        }

        if (!s.b[1170]) {
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
        }

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[657] = 0.0;

        s.v[684] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[654] = 0.0;

        s.v[681] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[651] = 1.0;

        s.v[678] = 1.0;

        s.v[652] = 1.0;

        s.v[679] = 1.0;

        s.v[653] = 1.0;

        s.v[680] = 1.0;

        s.v[501] = 0.0;

        s.b[1171] = (p.p43 > 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1172]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1172])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1173]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1173])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1174]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1174])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(654, 454, 455, 456);
        }

        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1175]) {
            s.store_exp_scaled_input(655, 654, s.v[371]);
        }

        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.store_scalar(396, s.v[393]);
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, p.p831);
            s.store_scalar(400, p.p832);
            s.store_scalar(401, p.p833);
            s.store_scalar(402, p.p828);
            s.store_scalar(403, p.p829);
            s.store_scalar(404, p.p830);
        }

        s.b[1177] = (s.v[646] == 0.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1177]) {
            s.store_scalar(396, (s.v[394] + s.v[395]));
            s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));
            s.store_scalar(402, (p.p829 + p.p830));
        }

        s.b[1178] = (s.v[647] == 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1178]) {
            s.store_scalar(397, (s.v[393] + s.v[395]));
            s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));
            s.store_scalar(403, (p.p828 + p.p830));
        }

        s.b[1179] = (s.v[648] == 0.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1179]) {
            s.store_scalar(398, (s.v[393] + s.v[394]));
            s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));
            s.store_scalar(404, (p.p828 + p.p829));
        }

        if s.b[1171] {
            s.store_min3(656, 396, 397, 398);
            s.store_scale(657, 656, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(658, 656, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1180]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1180])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1181]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1181])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1182]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1182])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(681, 454, 455, 456);
        }

        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1183]) {
            s.store_exp_scaled_input(682, 681, s.v[371]);
        }

        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.copy_ad(396, 569);
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 511);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 508);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
        }

        s.b[1185] = (s.v[673] == 0.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1185]) {
            s.store_add(396, 570, 571);
            s.store_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(402, 509, 510);
        }

        s.b[1186] = (s.v[674] == 0.0);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1186]) {
            s.store_add(397, 569, 571);
            s.store_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);
            s.store_add(403, 508, 510);
        }

        s.b[1187] = (s.v[675] == 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1187]) {
            s.store_add(398, 569, 570);
            s.store_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);
            s.store_add(404, 508, 509);
        }

        if s.b[1171] {
            s.store_min3(683, 396, 397, 398);
            s.store_scale(684, 683, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(685, 683, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1188] = (s.v[474] == 1.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_inputs3(501, s.ad_value(646), (s.v[414] * p.p929), s.ad_value(647), (s.v[415] * p.p929), s.ad_value(648), (s.v[416] * p.p929));
        }

        s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1523]) {
            s.store_scalar(651, 0.0);
        }

        s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1524]) {
            s.store_scalar(652, 0.0);
        }

        s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1525]) {
            s.store_scalar(653, 0.0);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_mul_ad_rhs(501, 553, A::add_scaled_products3(s.ad_value(673), s.ad_value(581), 1.0, s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0));
        }

        s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1813]) {
            s.store_scalar(678, 0.0);
        }

        s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1814]) {
            s.store_scalar(679, 0.0);
        }

        s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1815]) {
            s.store_scalar(680, 0.0);
        }

        s.v[2027] = 0.0;

        s.v[2028] = 0.0;

        s.v[2029] = 0.0;

        s.v[1937] = 1.0;

        s.v[1936] = 0.0;

        s.b[2102] = (s.v[0] == 1.0);
        s.v[2102] = if s.b[2102] { 1.0 } else { 0.0 };

        if s.b[2102] {
            s.store_voltage(825, ctx, nodes, Some(5), Some(6));
            s.store_voltage(826, ctx, nodes, Some(7), Some(6));
            s.store_voltage(827, ctx, nodes, Some(6), Some(8));
            s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);
        }

        if (!s.b[2102]) {
            s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(832, ctx, nodes, Some(6), Some(10));
            s.store_voltage(833, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(829, 825, 827);

        s.copy_ad(834, 825);

        s.copy_ad(835, 827);

        s.store_add(836, 826, 827);

        s.store_sub(837, 825, 826);

        s.store_scale(1817, 834, (-s.v[355]));

        s.store_scale(1818, 837, (-s.v[355]));

        s.store_scaled_sub(1819, 829, 700, (-s.v[355]));

        s.v[831] = 1.0;

        s.b[2103] = (s.v[826] < 0.0);
        s.v[2103] = if s.b[2103] { 1.0 } else { 0.0 };

        if s.b[2103] {
            s.store_scalar(831, (-1.0));
            s.store_sub(825, 825, 826);
            s.store_add(827, 827, 826);
            s.store_neg(826, 826);
        }

        s.store_add(828, 826, 827);

        s.store_div_scaled_product_offset_denominator(830, s.ad_value(826), s.ad_value(826), 1.0, A::sqrt(A::offset(A::square(s.ad_value(826)), 0.01)), 0.1, 1.0);

        s.store_add_scaled_inputs4(2107, s.ad_value(828), 0.5, s.ad_value(827), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(739), 1.0, A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827)), 1.0)), (-0.5), s.ad_value(737), 1.0);

        s.copy_ad(1820, 2107);

        s.store_add_scaled_inputs4(2030, s.ad_value(827), 1.0, s.ad_value(2107), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(738), 1.0, s.ad_value(2107), s.ad_value(2107), 1.0)), (-(-0.5)), s.ad_value(741), 1.0);

        s.copy_ad(1821, 2030);

        s.v[2031] = 0.0;

        s.b[2263] = ((p.p45 != 0.0) && (s.v[184] != 1.0));
        s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };

        if s.b[2263] {
            s.store_add_scaled_inputs3(2032, s.ad_value(2030), 1.0, s.ad_value(826), 0.5, s.ad_value(830), (-0.5));
            s.store_sub_ad_lhs(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);
            s.store_offset_div_scaled_inputs2(2027, s.ad_value(2033), 2.0, s.ad_value(743), (-2.0), s.ad_value(744), 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 0.4804530139182))), (-1.0));
            s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);
            s.store_add_scaled_inputs3(2030, s.ad_value(2035), 1.0, s.ad_value(826), (-0.5), s.ad_value(830), (-(-0.5)));
            s.store_sub(2031, 1821, 2030);
        }

        s.copy_ad(2104, 728);

        s.copy_ad(2105, 738);

        s.copy_ad(2106, 729);

        s.copy_ad(2108, 2030);

        s.copy_ad(2112, 2031);

        s.copy_ad(2109, 720);

        s.copy_ad(2110, 777);

        s.store_add_scaled_inputs3(2111, s.ad_value(829), 1.0, s.ad_value(2112), (-1.0), s.ad_value(700), -1.0);

        s.store_add_scaled_inputs3(2113, s.ad_value(2108), 1.0, s.ad_value(826), 0.5, s.ad_value(830), (-0.5));

        s.v[2125] = 1.0;

        s.b[2264] = (s.v[190] > 0.0);
        s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };

        if s.b[2264] {
            s.store_scale(2116, 2104, s.v[361]);
            s.store_scale(2117, 2113, s.v[361]);
            s.store_scale(2118, 2111, s.v[361]);
            s.store_offset_div_scaled_inputs(2028, s.ad_value(2106), 0.5, A::sqrt(s.ad_value(2116)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));
            s.store_offset_scaled(2120, 2116, 0.5, 2.0);
            s.store_add(2121, 2116, 2117);
            s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);
            s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2028, 2119, 0.5, 2122, 0.5, A::offset(A::mul(A::sub(s.ad_value(2119), s.ad_value(2122)), A::sub(s.ad_value(2119), s.ad_value(2122))), 20.0), 0.5);
            s.store_add_scaled_inputs3(2029, s.ad_value(2118), 2.0, s.ad_value(2117), (-2.0), s.ad_value(2120), -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2123, 2028, 0.5, 2029, 0.5, A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2028, 2123, 0.5, 2120, 0.5, A::offset(A::mul(A::sub(s.ad_value(2123), s.ad_value(2120)), A::sub(s.ad_value(2123), s.ad_value(2120))), 5.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0), A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0);
        }

        s.b[2265] = (s.v[2029] > (-230.25850929940458));
        s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };

        if (s.b[2264] && s.b[2265]) {
            s.store_exp(2125, 2029);
        }

        if (s.b[2264] && (!s.b[2265])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2126, 701, 2125, 1.0);

        s.store_scale(2127, 2126, s.v[715]);

        s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));

        s.store_mul_offset_rhs(2129, 2127, 2128, 1.0);

        s.store_div_from_scalar(2130, 1.0, 2129);

        s.store_mul_ad_rhs(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));

        s.store_square(2115, 2114);

        s.store_div_from_scalar(2131, 1.0, 2115);

        s.store_mul(2132, 2108, 2130);

        s.store_mul(2133, 2111, 2130);

        s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(197), s.ad_value(830)), 1.0)), 1.0, 1.0);

        s.store_mul_ad_product_rhs(2135, 196, s.ad_value(2134), A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));

        s.store_mul(2136, 2104, 2130);

        s.store_sqrt_square_add(2028, 2107, 2105);

        s.store_sqrt_ad(2029, A::add_scaled_product(s.ad_value(2105), 1.0, A::sub(s.ad_value(2107), s.ad_value(2135)), A::sub(s.ad_value(2107), s.ad_value(2135)), 1.0));

        s.store_mul_scaled_ad_rhs(2137, 2130, 0.5, A::add_scaled_inputs3(s.ad_value(2135), 1.0, s.ad_value(2028), 1.0, s.ad_value(2029), -1.0));

        s.store_add(2138, 2136, 2132);

        s.store_sub(2139, 2138, 2137);

        s.b[2266] = (p.p45 > 0.0);
        s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };

        s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);
        s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };

        if (s.b[2266] && s.b[2267]) {
            s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);
        }

        s.b[2268] = (s.v[2139] < 460.51701859880916);
        s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };

        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {
            s.store_exp_neg_input(2154, 2139);
        }

        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2139), (-460.51701859880916), A::scale_offset(s.ad_value(2139), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);
        }

        if (!s.b[2266]) {
            s.store_offset_div_scaled_inputs(2140, s.ad_value(2114), 0.5, A::sqrt(s.ad_value(2139)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2141, s.ad_value(2139), 1.0, s.ad_value(2114), A::sqrt(s.ad_value(2139)), 1.0, s.ad_value(2140), A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2(2142, s.ad_value(2133), 1.0, s.ad_value(2141), (-1.0), s.ad_value(2140), 1.0);

        s.store_mul_scaled_ad_rhs(2148, 2115, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0)));

        s.v[2147] = 0.0;

        s.v[2149] = 1.0;

        s.b[2269] = (s.v[2142] > (-30.0));
        s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };

        if s.b[2269] {
            s.store_offset_mul(2143, 2140, 2142, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2027, 2143, 2143, 10.0, 0.5);
            s.store_sub_ad_rhs(2144, 2142, A::ln(s.ad_value(2027)));
            s.store_scaled_add_sqrt_square_offset_rhs(2145, 2144, 2144, 2.0, 0.5);
        }

        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);
        s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };

        if (s.b[2269] && s.b[2270]) {
            s.store_exp_sub(2027, 2142, 2145);
        }

        if (s.b[2269] && (!s.b[2270])) {
            s.store_scaled_offset_mul_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2142), s.ad_value(2145)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[2269] {
            s.store_div(2146, 2027, 2140);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);
        }

        s.b[2271] = (s.v[2146] > 1e-6);
        s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };

        if (s.b[2269] && s.b[2271]) {
            s.store_mul_offset_ad_rhs(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2146), s.ad_value(2027)), 1.0)), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0);
        }

        if (s.b[2269] && (!s.b[2271])) {
            s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if s.b[2269] {
            s.store_add_scaled_inputs3_offset(2027, s.ad_value(2133), 0.5, s.ad_value(2147), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0), A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_ad_rhs(2148, 2115, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2149, 2148, A::add(s.ad_value(2148), s.ad_value(2147)));
            s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));
        }

        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);

        s.store_scale(2151, 2150, 1e-5);

        s.store_div_from_scalar(2152, 1.0, 2150);

        s.v[2259] = 0.0;

        s.v[2153] = 0.0;

        s.b[2272] = (s.v[2139] < 460.51701859880916);
        s.v[2272] = if s.b[2272] { 1.0 } else { 0.0 };

        if s.b[2272] {
            s.store_exp_neg_input(2154, 2139);
        }

        if (!s.b[2272]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2139), (-460.51701859880916), A::scale_offset(s.ad_value(2139), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.v[2273] = if s.b[2273] { 1.0 } else { 0.0 };

        if s.b[2273] {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2153, 2133, s.ad_value(2152), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        s.b[2274] = (s.v[2133] < (-s.v[2151]));
        s.v[2274] = if s.b[2274] { 1.0 } else { 0.0 };

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_neg(2241, 2133);
            s.store_scaled_mul(2242, 2241, 2152, 1.25);
            s.store_scaled_sub_ad(2243, A::offset(s.ad_value(2242), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2242), (-6.0), A::offset(s.ad_value(2242), (-6.0))), 64.0)), 0.5);
            s.store_sub(2238, 2241, 2243);
            s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);
            s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);
            s.store_sub_ad_lhs(2246, A::ln(A::mul(s.ad_value(2244), s.ad_value(2131))), 2243);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);
            s.store_add_ad_rhs(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));
        }

        s.b[2275] = (s.v[2247] < 230.25850929940458);
        s.v[2275] = if s.b[2275] { 1.0 } else { 0.0 };

        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {
            s.store_exp(2248, 2247);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2248, 2247, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2247), (-230.25850929940458), A::scale_offset(s.ad_value(2247), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_div_from_scalar_offset_ad(2238, 1.0, A::square(s.ad_value(2247)), 2.0);
            s.store_mul_square_lhs(2250, 2247, 2238);
            s.store_mul3_affine_lhs(2251, 2247, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), s.ad_value(2238), 2238);
            s.store_sub(2238, 2241, 2247);
            s.store_mul(2239, 2154, 2249);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2239), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2251)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2247), (-1.0), s.ad_value(2239), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(2238, 2.0, A::mul(s.ad_value(2115), A::add_scaled_inputs_product(s.ad_value(2248), 1.0, s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0))));
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2153, 2247, -1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_div_from_scalar_offset_scaled_input(2255, 1.0, 2114, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2256, A::mul_scaled_lhs(s.ad_value(2150), 1.25, s.ad_value(2255)), (-1.0), 2255);
            s.store_mul_ad_product_rhs(2257, 2133, s.ad_value(2152), A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));
        }

        s.b[2276] = ((-s.v[2257]) > (-230.25850929940458));
        s.v[2276] = if s.b[2276] { 1.0 } else { 0.0 };

        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2276]) {
            s.store_exp_neg_input(2238, 2257);
        }

        if (((!s.b[2273]) && (!s.b[2274])) && (!s.b[2276])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2238, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2257)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2257)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_sub_from_scalar(2258, 1.0, 2238);
            s.store_add_scaled_inputs_product_right_ad(2259, 2133, 1.0, 2115, 0.5, 2114, A::sqrt(A::add_scaled_inputs3(s.ad_value(2133), 1.0, s.ad_value(2115), 0.25, s.ad_value(2258), -1.0)), (-1.0));
            s.store_offset(2260, 2139, 3.0);
            s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2259), s.ad_value(2260)), A::sub(s.ad_value(2259), s.ad_value(2260))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2260)), 5.0)), 0.5));
            s.store_sub(2238, 2133, 2243);
            s.store_exp_neg_input(2239, 2243);
            s.store_div_from_scalar_offset_ad(2240, 1.0, A::square(s.ad_value(2243)), 2.0);
            s.store_mul_square_lhs(2250, 2243, 2240);
            s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), s.ad_value(2240), 2240);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_sub_from_scalar_ad(2261, 1.0, A::mul_scaled_output(s.ad_value(2115), A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2246, s.ad_value(2139), 1.0, s.ad_value(2243), (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);
            s.store_add_ad_rhs(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));
        }

        s.b[2277] = (s.v[2262] < 230.25850929940458);
        s.v[2277] = if s.b[2277] { 1.0 } else { 0.0 };

        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2277]) {
            s.store_exp(2248, 2262);
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_mul(2248, 2154, 2248);
        }

        s.b[2278] = (s.v[2262] > (s.v[2139] - 230.25850929940458));
        s.v[2278] = if s.b[2278] { 1.0 } else { 0.0 };

        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && s.b[2278]) {
            s.store_exp_sub(2248, 2262, 2139);
            s.store_div(2249, 2154, 2248);
        }

        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && (!s.b[2278])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2139), s.ad_value(2262)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2262), (-230.25850929940458), A::scale_offset(s.ad_value(2262), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_div_from_scalar_offset_ad(2238, 1.0, A::square(s.ad_value(2262)), 2.0);
            s.store_mul_square_lhs(2250, 2262, 2238);
            s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), s.ad_value(2238), 2238);
            s.store_sub(2238, 2133, 2262);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2238, 2.0, A::mul(s.ad_value(2115), A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0))));
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2153, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        s.v[2156] = 0.0;

        s.v[2157] = 0.0;

        s.v[2158] = 0.0;

        s.v[2159] = 0.0;

        s.v[2160] = 0.0;

        s.v[2161] = 0.0;

        s.v[2162] = 0.0;

        s.v[2163] = 1.0;

        s.v[2164] = 1.0;

        s.store_sub(2165, 2133, 2153);

        s.v[2166] = 0.0;

        s.store_mul(2167, 2129, 2165);

        s.v[2168] = 1.0;

        s.v[2169] = 1.0;

        s.v[2173] = 1.0;

        s.v[2174] = 1.0;

        s.v[2176] = 1.0;

        s.b[2279] = (s.v[2133] > 0.0);
        s.v[2279] = if s.b[2279] { 1.0 } else { 0.0 };

        if s.b[2279] {
            s.store_div_from_scalar_offset_ad(2027, 1.0, A::square(s.ad_value(2153)), 2.0);
            s.store_mul_square_lhs(2155, 2153, 2027);
            s.store_mul3_affine_lhs(2156, 2153, 2027, 4.0, 0.0, 2027);
            s.store_mul_ad_product_lhs(2157, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2155), 12.0), s.ad_value(2027), 2027);
            s.store_scalar(2158, 0.0);
        }

        s.b[2280] = (s.v[2153] < 230.25850929940458);
        s.v[2280] = if s.b[2280] { 1.0 } else { 0.0 };

        if (s.b[2279] && s.b[2280]) {
            s.store_exp(2158, 2153);
            s.store_div_from_scalar(2159, 1.0, 2158);
            s.store_mul(2158, 2154, 2158);
        }

        s.b[2281] = (s.v[2153] > (s.v[2139] - 230.25850929940458));
        s.v[2281] = if s.b[2281] { 1.0 } else { 0.0 };

        if ((s.b[2279] && (!s.b[2280])) && s.b[2281]) {
            s.store_exp_sub(2158, 2153, 2139);
            s.store_div(2159, 2154, 2158);
        }

        if ((s.b[2279] && (!s.b[2280])) && (!s.b[2281])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2158, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2139), s.ad_value(2153)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2159, 1e-100, 2153, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2153), (-230.25850929940458), A::scale_offset(s.ad_value(2153), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if s.b[2279] {
            s.store_add_scaled_product_right_ad(2160, 2158, 1.0, 2154, A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155)), (-1.0));
        }

        s.b[2282] = (s.v[2153] < 1e-5);
        s.v[2282] = if s.b[2282] { 1.0 } else { 0.0 };

        if (s.b[2279] && s.b[2282]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2161, 2153, 1.0, 2153, 1.0, 2153, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2160, A::mul3(s.ad_value(2154), s.ad_value(2153), s.ad_value(2153)), 2153, A::scale_offset(s.ad_value(2153), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2153), 1.0, A::scale(s.ad_value(2153), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);
            s.store_offset_div_scaled_product(2163, s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), 1.0, A::square(s.ad_value(2153)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0, 1.0);
        }

        if (s.b[2279] && (!s.b[2282])) {
            s.store_add_ad_lhs(2161, A::offset(s.ad_value(2153), (-1.0)), 2159);
            s.store_sqrt(2162, 2161);
            s.store_offset_scaled_ad(2163, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2159)), s.ad_value(2162)), 0.5, 1.0);
        }

        if s.b[2279] {
            s.store_div_scaled_offset_numerator(2164, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2113)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0), 1.0);
        }

        s.b[2283] = (s.v[2160] > 1e-100);
        s.v[2283] = if s.b[2283] { 1.0 } else { 0.0 };

        if (s.b[2279] && s.b[2283]) {
            s.store_mul_sqrt_ad_rhs(2165, 2114, A::add(s.ad_value(2161), s.ad_value(2160)));
            s.store_div_scaled_product3_mixed_iiia(2166, 2115, 2160, 2129, 1.0, A::add_scaled_product(s.ad_value(2165), 1.0, s.ad_value(2114), s.ad_value(2162), 1.0), 1.0);
            s.store_mul3_lhs(2167, 2162, 2114, 2129);
        }

        s.b[2284] = (s.v[217] < 0.0);
        s.v[2284] = if s.b[2284] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2284]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2168, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2113)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2284])) {
            s.store_offset_mul(2168, 217, 2113, 1.0);
        }

        s.b[2285] = (s.v[218] < 0.0);
        s.v[2285] = if s.b[2285] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2285]) {
            s.store_sub_from_scalar_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2166)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2285])) {
            s.store_div_from_scalar_offset_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2166)), 1.0);
        }

        if (s.b[2279] && s.b[2283]) {
            s.store_mul_product3_rhs(2170, 2166, s.ad_value(757), s.ad_value(2168), s.ad_value(2169), 1.0);
            s.store_mul_add_scaled_product_rhs(2171, 774, s.ad_value(2167), 1.0, s.ad_value(775), s.ad_value(2166), 1.0);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2161), 1.0, A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2173, A::offset(s.ad_value(2172), 1.0), s.ad_value(2170), 2164);
        }

        s.b[2286] = (s.v[221] < 0.0);
        s.v[2286] = if s.b[2286] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2286]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2174, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2113)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2286])) {
            s.store_offset_mul(2174, 221, 2113, 1.0);
        }

        if (s.b[2279] && s.b[2283]) {
            s.store_mul(2029, 2166, 2174);
            s.store_div_ad_rhs(2175, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.b[2287] = (s.v[222] < 0.0);
        s.v[2287] = if s.b[2287] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2287]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2287])) {
            s.store_offset_mul(2176, 222, 2175, 1.0);
        }

        s.copy_ad(1822, 2111);

        s.copy_ad(1823, 2113);

        s.copy_ad(1824, 2129);

        s.copy_ad(1825, 2130);

        s.copy_ad(1826, 2114);

        s.copy_ad(1827, 2115);

        s.copy_ad(1828, 2131);

        s.copy_ad(1829, 2133);

        s.copy_ad(1830, 2138);

        s.copy_ad(1831, 2139);

        s.copy_ad(1832, 2150);

        s.copy_ad(1833, 2151);

        s.copy_ad(1834, 2152);

        s.copy_ad(1835, 2259);

        s.copy_ad(1836, 2154);

        s.copy_ad(1837, 2153);

        s.copy_ad(1838, 2156);

        s.copy_ad(1839, 2157);

        s.copy_ad(1840, 2158);

        s.copy_ad(1841, 2159);

        s.copy_ad(1842, 2161);

        s.copy_ad(1843, 2160);

        s.copy_ad(1844, 2162);

        s.copy_ad(1845, 2163);

        s.copy_ad(1846, 2164);

        s.copy_ad(1847, 2165);

        s.copy_ad(1848, 2166);

        s.copy_ad(1849, 2167);

        s.copy_ad(1850, 2168);

        s.copy_ad(1851, 2169);

        s.copy_ad(1852, 2173);

        s.copy_ad(1853, 2174);

        s.copy_ad(1854, 2176);

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        s.v[2178] = 0.0;

        s.store_scale(2177, 2129, 4.60517018598809);

        s.copy_ad(2194, 2177);

        s.copy_ad(2195, 826);

        s.store_mul(2196, 826, 2130);

        s.copy_ad(2200, 2153);

        s.v[2201] = 0.0;

        s.v[2204] = 0.0;

        s.copy_ad(2206, 2159);

        s.copy_ad(2207, 2161);

        s.copy_ad(2209, 2160);

        s.copy_ad(2210, 2167);

        s.copy_ad(2211, 2153);

        s.copy_ad(2212, 2159);

        s.copy_ad(2214, 2160);

        s.copy_ad(2215, 2161);

        s.store_sub(2216, 2133, 2153);

        s.v[2217] = 1.0;

        s.v[2219] = 1.0;

        s.v[2218] = 0.0;

        s.copy_ad(2228, 2166);

        s.store_mul(2232, 2216, 2129);

        s.v[2229] = 0.0;

        s.copy_ad(2230, 2167);

        s.v[2235] = 0.0;

        s.v[2234] = 1.0;

        s.copy_ad(2237, 2109);

        s.copy_ad(2236, 2232);

        s.b[2288] = (s.v[2133] > 0.0);
        s.v[2288] = if s.b[2288] { 1.0 } else { 0.0 };

        s.b[2289] = (s.v[2160] > 1e-100);
        s.v[2289] = if s.b[2289] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2289]) {
            s.store_mul(2237, 2109, 2176);
            s.store_div(2178, 2237, 2173);
            s.store_add_scaled_inputs(2179, 2165, 1.0, 2115, 0.5);
            s.store_div_scaled_product_by_product(2027, s.ad_value(2115), s.ad_value(2158), 1.0, s.ad_value(2179), s.ad_value(2179), 1.0);
        }

        s.b[2290] = (s.v[2027] > 0.0001);
        s.v[2290] = if s.b[2290] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2289]) && s.b[2290]) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.b[2291] = (s.v[2028] < 1e-10);
        s.v[2291] = if s.b[2291] { 1.0 } else { 0.0 };

        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && s.b[2291]) {
            s.store_scalar(2029, 1.0);
        }

        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && (!s.b[2291])) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if ((s.b[2288] && s.b[2289]) && (!s.b[2290])) {
            s.store_scale(2029, 2027, 0.5);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul(2180, 2029, 2179);
        }

        s.b[2292] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));
        s.v[2292] = if s.b[2292] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {
            s.store_scaled_mul(2181, 2129, 2180, 0.475);
            s.store_add_scaled_product_indices(2027, 2166, 1.0, 2163, 2181, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2182, 2027, 2027, 1e-12, 0.5);
            s.store_add_scaled_value_products(2183, s.ad_value(2166), (-1.0), s.ad_value(2129), s.ad_value(2165), 1.0, A::offset(s.ad_value(2163), (-1.0)), s.ad_value(2181), 1.0);
            s.store_offset_div_scaled_product(2184, s.ad_value(2115), s.ad_value(2129), 0.5, s.ad_value(2183), 1.0, 1.0);
            s.store_add_scaled_product_indices(2027, 2183, 1.0, 775, 2182, 1.0);
            s.store_pow_ad(2185, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));
            s.store_mul_ad_lhs(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2184), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2185);
            s.store_div(2027, 2182, 2183);
            s.store_mul_pow_ad_rhs(2186, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));
            s.store_mul_div_scaled_product_rhs(2029, 2186, s.ad_value(707), A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, s.ad_value(2183), 1.0);
            s.store_mul_product3_rhs(2187, 2182, s.ad_value(757), s.ad_value(2168), s.ad_value(2169), 1.0);
            s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2168), s.ad_value(2169)), s.ad_value(2184), (-1.0), s.ad_value(2029), 1.0), 1.0);
        }

        s.b[2293] = (s.v[2027] < 230.25850929940458);
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && s.b[2293]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);
        }

        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && (!s.b[2293])) {
            s.copy_ad(2028, 2027);
        }

        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {
            s.store_div_scaled_product3_mixed_iiia(2188, 2181, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2185), 1.0, s.ad_value(2186), 1.0, s.ad_value(2187), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2189, 2180, A::div_scaled_value_offset_denominator(s.ad_value(2188), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2188)), 1.0)), 1.0, 1.0), 1.0);
        }

        if ((s.b[2288] && s.b[2289]) && (!s.b[2292])) {
            s.copy_ad(2189, 2180);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul3_affine_lhs(2190, 2129, 2178, 0.7071067811865475, 0.0, 2189);
        }

        s.b[2294] = (s.v[0] == (-1.0));
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2289]) && s.b[2294]) {
            s.store_div_ad_rhs(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_div_from_scalar_offset_ad(2191, 2.0, A::sqrt(A::scale_offset(s.ad_value(2190), 4.0, 1.0)), 1.0);
            s.store_mul(2027, 2191, 2190);
            s.store_mul_ad_product_rhs(2192, 2189, s.ad_value(2191), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2191)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2191), 4.0), 1.0)), 1.0));
            s.store_scale(2193, 2192, 0.99);
            s.store_div_scaled_product3_mixed_iaii(2027, 2193, A::sub_scaled_inputs(s.ad_value(2193), 1.0, s.ad_value(2179), 2.0), 2131, 1.0, 2160, 1.0);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul_sub_ad_rhs(2194, 2129, s.ad_value(2193), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2288] && (!s.b[2289])) {
            s.copy_ad(2194, 2177);
        }

        if s.b[2288] {
            s.store_offset(2027, 2110, 1.0);
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2194, 1.0);
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2195, 2194, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);
            s.store_mul(2196, 2195, 2130);
            s.store_add(2197, 2139, 2196);
        }

        s.b[2295] = (s.v[2196] < 460.51701859880916);
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2295]) {
            s.store_exp_neg_input(2198, 2196);
        }

        if (s.b[2288] && (!s.b[2295])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2198, 1e-200, 2196, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2196), (-460.51701859880916), A::scale_offset(s.ad_value(2196), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if s.b[2288] {
            s.store_mul(2199, 2154, 2198);
        }

        s.b[2296] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2296]) {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2200, 2133, s.ad_value(2152), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2199)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_offset(2260, 2197, 3.0);
            s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2259), s.ad_value(2260)), A::sub(s.ad_value(2259), s.ad_value(2260))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2260)), 5.0)), 0.5));
            s.store_sub(2238, 2133, 2243);
            s.store_exp_neg_input(2239, 2243);
            s.store_div_from_scalar_offset_ad(2240, 1.0, A::square(s.ad_value(2243)), 2.0);
            s.store_mul_square_lhs(2250, 2243, 2240);
            s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), s.ad_value(2240), 2240);
        }

        if (s.b[2288] && (!s.b[2296])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_sub_from_scalar_ad(2261, 1.0, A::mul_scaled_output(s.ad_value(2115), A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2246, s.ad_value(2197), 1.0, s.ad_value(2243), (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);
            s.store_add_ad_rhs(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));
        }

        s.b[2297] = (s.v[2262] < 230.25850929940458);
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        if ((s.b[2288] && (!s.b[2296])) && s.b[2297]) {
            s.store_exp(2248, 2262);
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_mul(2248, 2199, 2248);
        }

        s.b[2298] = (s.v[2262] > (s.v[2197] - 230.25850929940458));
        s.v[2298] = if s.b[2298] { 1.0 } else { 0.0 };

        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {
            s.store_exp_sub(2248, 2262, 2197);
            s.store_div(2249, 2199, 2248);
        }

        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2197), s.ad_value(2262)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2262), (-230.25850929940458), A::scale_offset(s.ad_value(2262), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_div_from_scalar_offset_ad(2238, 1.0, A::square(s.ad_value(2262)), 2.0);
            s.store_mul_square_lhs(2250, 2262, 2238);
            s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), s.ad_value(2238), 2238);
            s.store_sub(2238, 2133, 2262);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2238, 2.0, A::mul(s.ad_value(2115), A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0))));
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2200, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        if s.b[2288] {
            s.store_sub(2201, 2200, 2153);
        }

        s.b[2299] = (s.v[2201] < 1e-10);
        s.v[2299] = if s.b[2299] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2299]) {
            s.store_add_scaled_inputs_product_right_ad(2202, 2133, 2.0, 2153, (-2.0), 2115, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0), 1.0, s.ad_value(2199), s.ad_value(2156), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2203, A::mul_sub_from_scalar_rhs(s.ad_value(2115), 1.0, s.ad_value(2198)), 2160);
            s.store_sub_from_scalar_ad(2027, 2.0, A::mul(s.ad_value(2115), A::add_scaled_value_products(s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0, s.ad_value(2199), s.ad_value(2157), (-1.0))));
            s.store_add_scaled_square_product_indices(2027, 2202, 1.0, 2027, 2203, (-2.0));
            s.store_scaled_div_ad_rhs(2201, 2203, A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027))), 2.0);
            s.store_add(2200, 2153, 2201);
        }

        if s.b[2288] {
            s.store_mul(2204, 2201, 2129);
            s.store_div_scaled_product_offset_denominator(2205, s.ad_value(2200), s.ad_value(2200), 1.0, A::square(s.ad_value(2200)), 2.0, 1.0);
        }

        s.b[2300] = (s.v[2200] < 230.25850929940458);
        s.v[2300] = if s.b[2300] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2300]) {
            s.store_exp_neg_input(2206, 2200);
        }

        s.b[2301] = (s.v[2200] < 1e-5);
        s.v[2301] = if s.b[2301] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2300]) && s.b[2301]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2207, 2200, 1.0, 2200, 1.0, 2200, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2200), 1.0, A::scale(s.ad_value(2200), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);
            s.store_mul3_ad_middle(2209, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2200), s.ad_value(2200), 0.16666666666666666), 2200, A::scale_offset(s.ad_value(2200), 1.75, 1.0));
        }

        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {
            s.store_add_ad_lhs(2207, A::offset(s.ad_value(2200), (-1.0)), 2206);
            s.store_sqrt(2208, 2207);
            s.store_mul_add_scaled_inputs3_offset_rhs(2209, 2199, A::div_from_scalar(1.0, s.ad_value(2206)), 1.0, s.ad_value(2200), (-1.0), s.ad_value(2205), -1.0, (-1.0));
        }

        s.b[2302] = (s.v[2200] > (s.v[2197] - 230.25850929940458));
        s.v[2302] = if s.b[2302] { 1.0 } else { 0.0 };

        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {
            s.store_exp_sub(2027, 2200, 2197);
            s.store_div(2206, 2199, 2027);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {
            s.store_add_scaled_product_right_ad(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));
        }

        if ((s.b[2288] && (!s.b[2300])) && (!s.b[2302])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2206, 1e-100, 2200, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2200), (-230.25850929940458), A::scale_offset(s.ad_value(2200), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2197), s.ad_value(2200)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));
        }

        if (s.b[2288] && (!s.b[2300])) {
            s.store_add_ad_lhs(2207, A::offset(s.ad_value(2200), (-1.0)), 2206);
            s.store_sqrt(2208, 2207);
        }

        if s.b[2288] {
            s.store_mul3_lhs(2210, 2208, 2114, 2129);
            s.store_scaled_add(2211, 2153, 2200, 0.5);
            s.store_scalar(2212, 0.0);
            s.store_mul(2027, 2206, 2159);
        }

        s.b[2303] = (s.v[2027] > 0.0);
        s.v[2303] = if s.b[2303] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2303]) {
            s.store_sqrt(2212, 2027);
        }

        if s.b[2288] {
            s.store_scaled_add(2213, 2160, 2209, 0.5);
            s.store_add_scaled_product_mixed_iaa(2214, 2213, 1.0, A::square(s.ad_value(2201)), A::sub_scaled_inputs(s.ad_value(2212), 1.0, s.ad_value(2131), 2.0), 0.125);
        }

        s.b[2304] = (s.v[2211] < 1e-5);
        s.v[2304] = if s.b[2304] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2304]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2215, 2211, 1.0, 2211, 1.0, 2211, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
        }

        s.b[2305] = (s.v[730] > 0.0);
        s.v[2305] = if s.b[2305] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2304]) && s.b[2305]) {
            s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));
        }

        if (s.b[2288] && s.b[2304]) {
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2211), 1.0, A::scale(s.ad_value(2211), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2218, 2211, 2027, 0.7071067811865475);
            s.store_add_ad_rhs(2219, 2217, A::div_scaled_product(s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.5)), 1.0, A::square(s.ad_value(2211)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));
        }

        if (s.b[2288] && (!s.b[2304])) {
            s.store_add_ad_lhs(2215, A::offset(s.ad_value(2211), (-1.0)), 2212);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
        }

        s.b[2306] = (s.v[730] > 0.0);
        s.v[2306] = if s.b[2306] { 1.0 } else { 0.0 };

        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {
            s.store_add_scaled_sub_value_product_indices(2220, 1.0, 2212, 1.0, 2216, 2131, 2.0);
            s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));
            s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2217), 1.0, s.ad_value(2217), 1.0, 1.0);
            s.store_mul_product3_rhs(2221, 730, A::square(s.ad_value(2027)), s.ad_value(2115), s.ad_value(2214), 1.0);
            s.store_add_scaled_inputs_product_right_ad(2222, 2216, 2.0, 2221, (-2.0), 2115, A::add(A::sub_from_scalar(1.0, s.ad_value(2212)), s.ad_value(2214)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2223, 2221, s.ad_value(2221), 1.0, s.ad_value(2216), 2.0);
            s.store_sub_from_scalar_ad(2224, 1.0, A::mul_scaled_output(s.ad_value(2115), A::add(s.ad_value(2212), s.ad_value(2214)), 0.5));
            s.store_div_scaled_product_denominator_ad(2225, 2223, 2222, 1.0, A::add_scaled_square_product(s.ad_value(2222), 1.0, s.ad_value(2224), s.ad_value(2223), (-1.0)), 1.0);
            s.store_add(2211, 2211, 2225);
            s.store_exp(2226, 2225);
            s.store_div(2212, 2212, 2226);
            s.store_mul(2214, 2214, 2226);
            s.store_add_ad_lhs(2215, A::offset(s.ad_value(2211), (-1.0)), 2212);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
            s.store_add_ad(2227, A::sub_from_scalar(1.0, s.ad_value(2212)), A::mul3_scaled_output(s.ad_value(2216), s.ad_value(2217), s.ad_value(2131), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2201, 2201, 2226, A::add(s.ad_value(2220), s.ad_value(2213)), 1.0, A::add_scaled_product(s.ad_value(2227), 1.0, s.ad_value(2226), s.ad_value(2213), 1.0), 1.0);
            s.store_mul(2204, 2201, 2129);
        }

        if (s.b[2288] && (!s.b[2304])) {
            s.store_sqrt(2218, 2215);
            s.store_add_scaled_inputs_ad_rhs(2219, 2217, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2212)), s.ad_value(2218)), 0.5);
        }

        if s.b[2288] {
            s.store_mul_div_scaled_product_rhs(2228, 2129, s.ad_value(2115), s.ad_value(2214), 1.0, A::add_scaled_product(s.ad_value(2216), 1.0, s.ad_value(2114), s.ad_value(2218), 1.0), 1.0);
            s.store_add_scaled_product_indices(2229, 2228, 1.0, 2129, 2219, 1.0);
            s.store_mul3_lhs(2230, 2218, 2114, 2129);
        }

        s.b[2307] = (s.v[218] < 0.0);
        s.v[2307] = if s.b[2307] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2307]) {
            s.store_sub_from_scalar_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2228)));
        }

        if (s.b[2288] && (!s.b[2307])) {
            s.store_div_from_scalar_offset_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2228)), 1.0);
        }

        if s.b[2288] {
            s.store_mul_product3_rhs(2170, 2228, s.ad_value(757), s.ad_value(2168), s.ad_value(2169), 1.0);
            s.store_add_scaled_product_indices(2231, 2230, 1.0, 775, 2228, 1.0);
            s.store_add_scaled_product_indices(2232, 2230, 1.0, 776, 2228, 1.0);
            s.store_mul(2233, 774, 2231);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2215), 1.0, A::add(s.ad_value(2215), s.ad_value(2214)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2233), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2234, A::offset(s.ad_value(2172), 1.0), s.ad_value(2170), 2164);
            s.store_ln_ad(2235, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2204)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2195), s.ad_value(2204)), s.ad_value(779)), 1.0), 1.0));
            s.store_mul(2029, 2228, 2174);
            s.store_div_ad_rhs(2175, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.b[2308] = (s.v[222] < 0.0);
        s.v[2308] = if s.b[2308] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2308]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));
        }

        if (s.b[2288] && (!s.b[2308])) {
            s.store_offset_mul(2176, 222, 2175, 1.0);
        }

        if s.b[2288] {
            s.store_mul(2237, 2109, 2176);
            s.store_mul(2236, 2216, 2129);
        }

        s.copy_ad(1855, 2177);

        s.copy_ad(1857, 2195);

        s.copy_ad(1858, 2196);

        s.copy_ad(1859, 2201);

        s.copy_ad(1860, 2204);

        s.copy_ad(1862, 2211);

        s.copy_ad(1861, 2210);

        s.copy_ad(1863, 2217);

        s.copy_ad(1864, 2219);

        s.copy_ad(1865, 2228);

        s.copy_ad(1866, 2229);

        s.copy_ad(1867, 2230);

        s.copy_ad(1868, 2232);

        s.copy_ad(1869, 2234);

        s.copy_ad(1871, 2235);

        s.copy_ad(1870, 2237);

        s.copy_ad(1872, 2236);

        s.copy_ad(1931, 2216);

        s.v[1873] = 1.0;

        s.v[1874] = 1.0;

        s.v[1876] = 1.0;

        s.v[1877] = 1.0;

        s.v[838] = 0.0;

        s.b[2309] = (s.v[1829] > 0.0);
        s.v[2309] = if s.b[2309] { 1.0 } else { 0.0 };

        if s.b[2309] {
            s.store_ln_ad(2037, A::offset(A::mul(s.ad_value(830), s.ad_value(779)), 1.0));
            s.store_div_scaled_product_indices(2027, 1824, 1864, 1.0, 1866, 1.0);
            s.store_add_scaled_product_mixed_aai(2036, A::mul3(A::mul3(s.ad_value(227), s.ad_value(1867), s.ad_value(2027)), s.ad_value(2027), s.ad_value(2037)), 1.0, A::div_scaled_product(A::add(s.ad_value(225), A::div(s.ad_value(226), s.ad_value(1866))), s.ad_value(1865), 1.0, s.ad_value(1866), 1.0), 1871, 1.0);
            s.store_div_from_scalar_add_ad(1873, 1.0, A::offset(s.ad_value(2036), 1.0), A::square(s.ad_value(2036)));
            s.store_mul(1874, 1869, 1873);
            s.store_div(1875, 1870, 1874);
            s.store_mul_ad_product_lhs(2038, A::square(s.ad_value(1875)), s.ad_value(1860), 1860);
        }

        s.b[2310] = (s.v[0] == (-1.0));
        s.v[2310] = if s.b[2310] { 1.0 } else { 0.0 };

        if (s.b[2309] && s.b[2310]) {
            s.store_div_scaled_value_offset_denominator(2038, s.ad_value(2038), 1.0, A::mul(s.ad_value(1875), s.ad_value(1860)), 1.0, 1.0);
        }

        if s.b[2309] {
            s.store_mul_offset_rhs_scaled_ad_rhs(2039, 1874, A::sqrt(A::scale_offset(s.ad_value(2038), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1876, 1.0, 2039);
            s.store_mul(2027, 1874, 1876);
            s.store_mul_offset_ad_rhs(2040, 1864, A::mul3_scaled_output(s.ad_value(2038), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0);
            s.store_div_scaled_product_indices(1877, 2027, 1866, 1.0, 2040, 1.0);
            s.store_mul_product3_rhs(838, 1876, s.ad_value(716), s.ad_value(1866), s.ad_value(1860), 1.0);
        }

        s.v[2042] = 0.0;

        s.v[2043] = 0.0;

        s.v[1878] = 0.0;

        s.v[1879] = 0.0;

        s.b[2311] = (((((p.p40 != 0.0) && ((s.v[237] > 0.0) || (s.v[238] > 0.0))) || ((p.p42 != 0.0) && ((s.v[247] > 0.0) || (s.v[248] > 0.0)))) || (s.v[262] > 0.0)) || (s.v[263] > 0.0));
        s.v[2311] = if s.b[2311] { 1.0 } else { 0.0 };

        if s.b[2311] {
            s.store_scaled_add_ad_rhs(2041, 1817, A::sqrt(A::add(A::square(s.ad_value(1817)), s.ad_value(789))), 0.5);
            s.store_add_ad_lhs(2042, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(784), (-0.5), s.ad_value(782), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), 1.0), 791);
            s.store_scaled_add_ad_rhs(2041, 1818, A::sqrt(A::add(A::square(s.ad_value(1818)), s.ad_value(792))), 0.5);
            s.store_add_ad_lhs(2043, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(785), (-0.5), s.ad_value(783), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), 1.0), 794);
            s.store_scaled_add(1878, 1817, 2042, (-s.v[354]));
            s.store_scaled_add(1879, 1818, 2043, (-s.v[354]));
        }

        s.b[2312] = (p.p40 != 0.0);
        s.v[2312] = if s.b[2312] { 1.0 } else { 0.0 };

        s.b[2313] = (s.v[237] > 0.0);
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1878)), 1e-6), 795);
        }

        s.b[2314] = (s.v[243] < 0.0);
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2313]) && s.b[2314]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2044, 2044, 0.5, 801, 0.5, A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(801)), A::sub(s.ad_value(2044), s.ad_value(801))), 1e-6), (-0.5));
        }

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_offset_ad_rhs(2027, 798, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(2044), 1.0)), (-1.5));
            s.store_offset(2046, 2042, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
            s.store_scale(2048, 834, 30.0);
            s.store_scalar(818, (4.0 - 0.9));
            s.store_add(819, 2046, 2048);
            s.store_mul_ad(2027, A::div_from_scalar(2.0, s.ad_value(818)), A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));
            s.store_scalar(818, (4.0 - 0.3));
            s.store_add(819, 2047, 2027);
        }

        s.b[2317] = (s.v[238] > 0.0);
        s.v[2317] = if s.b[2317] { 1.0 } else { 0.0 };

        if (s.b[2312] && s.b[2317]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1879)), 1e-6), 795);
        }

        s.b[2318] = (s.v[245] < 0.0);
        s.v[2318] = if s.b[2318] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2317]) && s.b[2318]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2044, 2044, 0.5, 802, 0.5, A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(802)), A::sub(s.ad_value(2044), s.ad_value(802))), 1e-6), (-0.5));
        }

        if (s.b[2312] && s.b[2317]) {
            s.store_mul_offset_ad_rhs(2027, 799, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(245), s.ad_value(2044), 1.0)), (-1.5));
            s.store_offset(2046, 2043, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
            s.store_scale(2048, 837, 30.0);
            s.store_scalar(818, (4.0 - 0.9));
            s.store_add(819, 2046, 2048);
            s.store_mul_ad(2027, A::div_from_scalar(2.0, s.ad_value(818)), A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));
            s.store_scalar(818, (4.0 - 0.3));
            s.store_add(819, 2047, 2027);
        }

        s.b[2321] = (s.v[236] > 0.0);
        s.v[2321] = if s.b[2321] { 1.0 } else { 0.0 };

        s.b[2322] = (s.v[1829] <= 0.0);
        s.v[2322] = if s.b[2322] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2322]) {
            s.store_offset(2027, 777, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2312] && s.b[2321]) && s.b[2322]) {
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 1855, 1.0);
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1858, 1855, 1825, 2027, 1.0, A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))), 1.0);
        }

        s.b[2323] = ((s.v[1859] - s.v[1858]) > (-230.25850929940458));
        s.v[2323] = if s.b[2323] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2323]) {
            s.store_exp_sub(2027, 1859, 1858);
        }

        if ((s.b[2312] && s.b[2321]) && (!s.b[2323])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2027, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_add_scaled_product_right_ad(2050, 2030, 1.0, 1824, A::sub_scaled_inputs(s.ad_value(1859), 0.5, A::ln_scaled_input(A::offset(s.ad_value(2027), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(2051, 235, 1824);
            s.store_add(2052, 1872, 2051);
            s.store_scaled_sub_ad_rhs(2053, 2052, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(2052), s.ad_value(2052), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1872)), 1e-6), 795);
        }

        s.b[2324] = (s.v[241] < 0.0);
        s.v[2324] = if s.b[2324] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2324]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2044, 2044, 0.5, 800, 0.5, A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(800)), A::sub(s.ad_value(2044), s.ad_value(800))), 1e-6), (-0.5));
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_add_scaled_product_left_ad(2054, 1862, 1.0, A::add_scaled_inputs3(s.ad_value(2053), 1.0, s.ad_value(742), (-1.0), s.ad_value(2050), -1.0), 1825, 1.0);
            s.store_mul_neg_ad_lhs(2054, A::add_scaled_inputs3(s.ad_value(825), 1.0, s.ad_value(2030), 1.0, s.ad_value(2050), -1.0), 1825);
        }

        s.b[2327] = (((s.v[2054]) as f64).abs() < 230.25850929940458);
        s.v[2327] = if s.b[2327] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2327]) {
            s.store_exp(2027, 2054);
        }

        s.b[2328] = (s.v[2054] < 0.0);
        s.v[2328] = if s.b[2328] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && s.b[2328]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2027, 1e-100, (-230.25850929940458), 2054, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && (!s.b[2328])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2027, 2054, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2054), (-230.25850929940458), A::scale_offset(s.ad_value(2054), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_mul_offset_ad_rhs(2027, 797, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(2044), 1.0)), (-1.5));
        }

        s.b[2331] = ((s.v[1829] <= 0.0) || ((s.v[240] == 0.0) && (s.v[241] == 0.0)));
        s.v[2331] = if s.b[2331] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && (!s.b[2331])) {
            s.store_add_scaled_product_indices(2027, 240, 1.0, 241, 2044, 2.0);
            s.store_div_ad_rhs(2058, 246, A::mul(s.ad_value(2027), s.ad_value(797)));
            s.store_scaled_div(2059, 1860, 2058, 0.5);
        }

        s.b[2332] = (s.v[2059] < 0.001);
        s.v[2332] = if s.b[2332] { 1.0 } else { 0.0 };

        s.b[2333] = (((s.v[2059]) as f64).abs() < 230.25850929940458);
        s.v[2333] = if s.b[2333] { 1.0 } else { 0.0 };

        if ((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && s.b[2333]) {
            s.store_exp(2067, 2059);
        }

        s.b[2334] = (s.v[2059] < 0.0);
        s.v[2334] = if s.b[2334] { 1.0 } else { 0.0 };

        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && s.b[2334]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2067, 1e-100, (-230.25850929940458), 2059, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && (!s.b[2334])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2067, 2059, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2059), (-230.25850929940458), A::scale_offset(s.ad_value(2059), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) {
            s.store_div_from_scalar(2068, 1.0, 2067);
            s.store_sub(2027, 2067, 2068);
            s.store_add(2029, 2067, 2068);
        }

        s.b[2335] = (p.p42 != 0.0);
        s.v[2335] = if s.b[2335] { 1.0 } else { 0.0 };

        s.b[2336] = ((s.v[248] > 0.0) && (s.v[1879] < 0.0));
        s.v[2336] = if s.b[2336] { 1.0 } else { 0.0 };

        if (s.b[2335] && s.b[2336]) {
            s.store_sqrt_offset_ad(2071, A::add_scaled_square_product(s.ad_value(1879), 1.0, A::square(s.ad_value(254)), A::square(s.ad_value(836)), 1.0), 1e-6);
            s.store_div_scaled_inputs(2027, s.ad_value(807), -1.0, s.ad_value(2071), 1.0);
        }

        s.b[2337] = (s.v[2027] > (-230.25850929940458));
        s.v[2337] = if s.b[2337] { 1.0 } else { 0.0 };

        if ((s.b[2335] && s.b[2336]) && s.b[2337]) {
            s.store_exp(2029, 2027);
        }

        if ((s.b[2335] && s.b[2336]) && (!s.b[2337])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2029, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2338] = ((s.v[247] > 0.0) && (s.v[1878] < 0.0));
        s.v[2338] = if s.b[2338] { 1.0 } else { 0.0 };

        if (s.b[2335] && s.b[2338]) {
            s.store_sqrt_offset_ad(2072, A::add_scaled_square_product(s.ad_value(1878), 1.0, A::square(s.ad_value(253)), A::square(s.ad_value(835)), 1.0), 1e-6);
            s.store_div_scaled_inputs(2027, s.ad_value(806), -1.0, s.ad_value(2072), 1.0);
        }

        s.b[2339] = (s.v[2027] > (-230.25850929940458));
        s.v[2339] = if s.b[2339] { 1.0 } else { 0.0 };

        if ((s.b[2335] && s.b[2338]) && s.b[2339]) {
            s.store_exp(2029, 2027);
        }

        if ((s.b[2335] && s.b[2338]) && (!s.b[2339])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2029, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.v[2076] = s.v[715];

        s.v[1880] = 0.0;

        s.v[1881] = 0.0;

        s.v[1882] = 0.0;

        s.v[1883] = 1e-40;

        s.v[1884] = 1.0;

        s.v[846] = 0.0;

        s.b[2340] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.v[2340] = if s.b[2340] { 1.0 } else { 0.0 };

        if s.b[2340] {
            s.store_add_scaled_inputs4(2027, s.ad_value(828), 0.5, s.ad_value(827), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(764), 1.0, A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827)), 1.0)), (-0.5), s.ad_value(762), 1.0);
            s.store_add_scaled_inputs4(2073, s.ad_value(827), 1.0, s.ad_value(2027), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(763), 1.0, s.ad_value(2027), s.ad_value(2027), 1.0)), (-(-0.5)), s.ad_value(766), 1.0);
            s.store_add_scaled_inputs3(2074, s.ad_value(2073), 1.0, s.ad_value(826), 0.5, s.ad_value(830), (-0.5));
            s.store_mul_ad_product_rhs(2075, 289, A::offset(A::mul(s.ad_value(291), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(290), s.ad_value(2074)), 1.0));
            s.store_mul_offset_rhs(2076, 723, 2075, 1.0);
            s.store_div_from_scalar(2077, 1.0, 2076);
            s.store_div_scaled_value_offset_denominator(2078, s.ad_value(830), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(293), s.ad_value(830)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(2079, 292, s.ad_value(2078), A::offset(A::mul(s.ad_value(294), s.ad_value(2074)), 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(1880, 2077, s.ad_value(829), 1.0, s.ad_value(2079), 1.0, s.ad_value(713), -1.0, 0.0);
            s.store_mul(2080, 2077, 760);
            s.store_scaled_ln_ad(2081, A::add(A::div(s.ad_value(2080), s.ad_value(761)), A::sqrt(s.ad_value(2080))), 2.0);
            s.store_mul(2082, 2077, 2073);
            s.store_add(2087, 2080, 2082);
            s.store_add_scaled_product_right_ad(2088, 2087, 1.0, 761, A::sqrt(s.ad_value(2087)), 1.0);
            s.store_add(2089, 2088, 2081);
            s.store_offset_div_scaled_inputs(2090, s.ad_value(761), 1.0, A::sqrt(s.ad_value(2087)), 2.0, 1.0);
            s.store_div_from_scalar(2091, 1.0, 2090);
            s.store_sub(2092, 1880, 2089);
        }

        s.b[2341] = (s.v[2092] > (-12.0));
        s.v[2341] = if s.b[2341] { 1.0 } else { 0.0 };

        if (s.b[2340] && s.b[2341]) {
            s.store_offset_add(2093, 2092, 725, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2094, 2093, 2093, 10.0, 0.5);
            s.store_add_ad_lhs(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);
            s.store_scaled_add_sqrt_square_offset_rhs(2096, 2095, 2095, 2.0, 0.5);
        }

        s.b[2342] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);
        s.v[2342] = if s.b[2342] { 1.0 } else { 0.0 };

        if ((s.b[2340] && s.b[2341]) && s.b[2342]) {
            s.store_exp_sub(2097, 2092, 2096);
        }

        if ((s.b[2340] && s.b[2341]) && (!s.b[2342])) {
            s.store_scaled_offset_mul_offset_lhs_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2092), s.ad_value(2096)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2340] && s.b[2341]) {
            s.store_mul(2098, 724, 2097);
            s.store_pow_ad(2099, s.ad_value(2098), s.ad_value(2091));
            s.store_add_scaled_square_product_mixed_iai(2100, 2090, 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), 2099, 1.0);
            s.store_mul_offset_ad_rhs(2101, 2090, A::div_scaled_inputs2(A::sqrt(s.ad_value(2100)), 1.0, s.ad_value(2090), (-1.0), s.ad_value(2099), 1.0), (-1.0));
            s.store_sub(2083, 2096, 2101);
        }

        s.b[2343] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));
        s.v[2343] = if s.b[2343] { 1.0 } else { 0.0 };

        if ((s.b[2340] && (!s.b[2341])) && s.b[2343]) {
            s.store_exp_ad(2083, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if ((s.b[2340] && (!s.b[2341])) && (!s.b[2343])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2083, 1e-100, (-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if s.b[2340] {
            s.store_mul_add_rhs(2084, 2077, 1857, 2073);
        }

        s.b[2344] = ((s.v[2083] < 0.001) && (s.v[1857] < 1e-6));
        s.v[2344] = if s.b[2344] { 1.0 } else { 0.0 };

        s.b[2345] = (((-s.v[2084]) + s.v[2082]) > (-230.25850929940458));
        s.v[2345] = if s.b[2345] { 1.0 } else { 0.0 };

        if ((s.b[2340] && s.b[2344]) && s.b[2345]) {
            s.store_exp_sub(2027, 2082, 2084);
        }

        if ((s.b[2340] && s.b[2344]) && (!s.b[2345])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2027, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2340] && s.b[2344]) {
            s.store_mul_offset_rhs(1881, 2083, 2027, (-1.0));
            s.store_add(2085, 1881, 2083);
        }

        if (s.b[2340] && (!s.b[2344])) {
            s.store_add(2087, 2080, 2084);
            s.store_add_scaled_product_right_ad(2088, 2087, 1.0, 761, A::sqrt(s.ad_value(2087)), 1.0);
            s.store_add(2089, 2088, 2081);
            s.store_offset_div_scaled_inputs(2090, s.ad_value(761), 1.0, A::sqrt(s.ad_value(2087)), 2.0, 1.0);
            s.store_div_from_scalar(2091, 1.0, 2090);
            s.store_sub(2092, 1880, 2089);
        }

        s.b[2346] = (s.v[2092] > (-12.0));
        s.v[2346] = if s.b[2346] { 1.0 } else { 0.0 };

        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {
            s.store_offset_add(2093, 2092, 725, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2094, 2093, 2093, 10.0, 0.5);
            s.store_add_ad_lhs(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);
            s.store_scaled_add_sqrt_square_offset_rhs(2096, 2095, 2095, 2.0, 0.5);
        }

        s.b[2347] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);
        s.v[2347] = if s.b[2347] { 1.0 } else { 0.0 };

        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && s.b[2347]) {
            s.store_exp_sub(2097, 2092, 2096);
        }

        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && (!s.b[2347])) {
            s.store_scaled_offset_mul_offset_lhs_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2092), s.ad_value(2096)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {
            s.store_mul(2098, 724, 2097);
            s.store_pow_ad(2099, s.ad_value(2098), s.ad_value(2091));
            s.store_add_scaled_square_product_mixed_iai(2100, 2090, 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), 2099, 1.0);
            s.store_mul_offset_ad_rhs(2101, 2090, A::div_scaled_inputs2(A::sqrt(s.ad_value(2100)), 1.0, s.ad_value(2090), (-1.0), s.ad_value(2099), 1.0), (-1.0));
            s.store_sub(2085, 2096, 2101);
        }

        s.b[2348] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));
        s.v[2348] = if s.b[2348] { 1.0 } else { 0.0 };

        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && s.b[2348]) {
            s.store_exp_ad(2085, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && (!s.b[2348])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2085, 1e-100, (-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2340] && (!s.b[2344])) {
            s.store_sub(1881, 2085, 2083);
        }

        if s.b[2340] {
            s.store_scaled_add(1882, 2085, 2083, 0.5);
        }

        if s.b[2340] {
            if ((s.v[1880] - s.v[1882]) > 1e-40) {
                s.store_sub(1883, 1880, 1882);
            } else {
                s.store_scalar(1883, 1e-40);
            }
        }

        if s.b[2340] {
            s.store_sub_from_scalar_ad(1884, 1.0, A::div_scaled_inputs(s.ad_value(761), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1883), 1.0, s.ad_value(724), 0.25)), 1.0));
            s.store_div_scaled_product3_mixed_aaii(846, A::mul3_scaled_output(s.ad_value(717), s.ad_value(2076), s.ad_value(2076), -1.0), A::offset(A::mul(s.ad_value(1884), s.ad_value(1882)), 1.0), 1881, 1.0, 1869, 1.0);
        }

        s.v[1885] = 0.0;

        s.v[847] = 0.0;

        s.b[2349] = ((s.v[1829] > 0.0) && (p.p41 != 0.0));
        s.v[2349] = if s.b[2349] { 1.0 } else { 0.0 };

        if s.b[2349] {
            s.store_add_scaled_product_indices(2086, 826, 1.0, 232, 1860, (-1.0));
        }

        s.b[2350] = (s.v[2086] > 0.0);
        s.v[2350] = if s.b[2350] { 1.0 } else { 0.0 };

        if (s.b[2349] && s.b[2350]) {
            s.store_mul_div_scaled_offset_numerator_rhs(2029, 712, A::mul(s.ad_value(233), A::sub(A::sqrt(A::add(s.ad_value(728), s.ad_value(2030))), s.ad_value(736))), 1.0, 1.0, A::offset(s.ad_value(2086), 1e-30), 1.0);
        }

        s.b[2351] = ((((-s.v[2029])) as f64).abs() < 230.25850929940458);
        s.v[2351] = if s.b[2351] { 1.0 } else { 0.0 };

        if ((s.b[2349] && s.b[2350]) && s.b[2351]) {
            s.store_exp_neg_input(2027, 2029);
        }

        s.b[2352] = ((-s.v[2029]) < 0.0);
        s.v[2352] = if s.b[2352] { 1.0 } else { 0.0 };

        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && s.b[2352]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2029)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2029)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && (!s.b[2352])) {
            s.store_scaled_offset_mul_offset_lhs_ad(2027, A::neg(s.ad_value(2029)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2029)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2029)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2349] && s.b[2350]) {
            s.store_mul3_lhs(1885, 229, 2086, 2027);
            s.store_mul_add_rhs(847, 1885, 838, 846);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2353] = (s.v[847] > (0.5 * s.v[234]));
        s.v[2353] = if s.b[2353] { 1.0 } else { 0.0 };

        if ((s.b[2349] && s.b[2350]) && s.b[2353]) {
            s.store_offset_div_scaled_inputs(2027, s.ad_value(847), 2.0, s.ad_value(234), 1.0, (-1.0));
            s.store_mul_scaled_ad_rhs(847, 234, 0.5, A::offset(A::div(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1.0))), 1.0));
        }

        s.b[2547] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.v[2547] = if s.b[2547] { 1.0 } else { 0.0 };

        s.b[2548] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.v[2548] = if s.b[2548] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2548]) {
            s.copy_ad(2388, 728);
            s.copy_ad(2389, 738);
            s.copy_ad(2390, 729);
            s.copy_ad(2391, 1820);
            s.copy_ad(2392, 1821);
            s.store_scalar(2396, 0.0);
        }

        s.b[2549] = (p.p47 > 0.0);
        s.v[2549] = if s.b[2549] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2549]) {
            s.store_add_scaled_inputs4(2391, s.ad_value(828), 0.5, s.ad_value(827), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(749), 1.0, A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827)), 1.0)), (-0.5), s.ad_value(747), 1.0);
            s.store_add_scaled_inputs4(1886, s.ad_value(827), 1.0, s.ad_value(2391), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(748), 1.0, s.ad_value(2391), s.ad_value(2391), 1.0)), (-(-0.5)), s.ad_value(750), 1.0);
            s.copy_ad(2392, 1886);
            s.copy_ad(2388, 745);
            s.copy_ad(2389, 748);
            s.copy_ad(2390, 746);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_add_scaled_inputs3(2395, s.ad_value(829), 1.0, s.ad_value(2396), (-1.0), s.ad_value(700), -1.0);
            s.store_add_scaled_inputs3(2397, s.ad_value(2392), 1.0, s.ad_value(826), 0.5, s.ad_value(830), (-0.5));
            s.store_scalar(2409, 1.0);
        }

        s.b[2550] = (s.v[190] > 0.0);
        s.v[2550] = if s.b[2550] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2550]) {
            s.store_scale(2400, 2388, s.v[361]);
            s.store_scale(2401, 2397, s.v[361]);
            s.store_scale(2402, 2395, s.v[361]);
            s.store_offset_div_scaled_inputs(2028, s.ad_value(2390), 0.5, A::sqrt(s.ad_value(2400)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2029, 2400, 1.0, 2390, A::sqrt(s.ad_value(2400)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2403, A::div_scaled_inputs2(s.ad_value(2402), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2400, 0.5, A::offset(s.ad_value(191), 1.0), 2401, (-1.0));
            s.store_offset_scaled(2404, 2400, 0.5, 2.0);
            s.store_add(2405, 2400, 2401);
            s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2402), 1.0, s.ad_value(2405), (-1.0), s.ad_value(2390), A::sqrt(s.ad_value(2405)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0);
            s.store_add_scaled_inputs(2406, 2028, 2.0, 2404, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2028, 2403, 0.5, 2406, 0.5, A::offset(A::mul(A::sub(s.ad_value(2403), s.ad_value(2406)), A::sub(s.ad_value(2403), s.ad_value(2406))), 20.0), 0.5);
            s.store_add_scaled_inputs3(2029, s.ad_value(2402), 2.0, s.ad_value(2401), (-2.0), s.ad_value(2404), -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2407, 2028, 0.5, 2029, 0.5, A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2028, 2407, 0.5, 2404, 0.5, A::offset(A::mul(A::sub(s.ad_value(2407), s.ad_value(2404)), A::sub(s.ad_value(2407), s.ad_value(2404))), 5.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2408, 2028, 0.5, 2404, ((-1.0) * 0.5), A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0), A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2408), s.ad_value(2404)), 1.0);
        }

        s.b[2551] = (s.v[2029] > (-230.25850929940458));
        s.v[2551] = if s.b[2551] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && s.b[2551]) {
            s.store_exp(2409, 2029);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && (!s.b[2551])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2409, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_offset_mul(2410, 701, 2409, 1.0);
            s.store_scale(2411, 2410, s.v[715]);
            s.store_mul_ad_product_rhs(2412, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));
            s.store_mul_offset_rhs(2413, 2411, 2412, 1.0);
            s.store_div_from_scalar(2414, 1.0, 2413);
            s.store_mul_ad_rhs(2398, 2390, A::sqrt_scaled_input(s.ad_value(2414), s.v[715]));
            s.store_square(2399, 2398);
            s.store_div_from_scalar(2415, 1.0, 2399);
            s.store_mul(2416, 2392, 2414);
            s.store_mul(2417, 2395, 2414);
            s.store_div_scaled_value_offset_denominator(2418, s.ad_value(830), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(197), s.ad_value(830)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(2419, 196, s.ad_value(2418), A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));
            s.store_mul(2420, 2388, 2414);
            s.store_sqrt_square_add(2028, 2391, 2389);
            s.store_sqrt_ad(2029, A::add_scaled_product(s.ad_value(2389), 1.0, A::sub(s.ad_value(2391), s.ad_value(2419)), A::sub(s.ad_value(2391), s.ad_value(2419)), 1.0));
            s.store_mul_scaled_ad_rhs(2421, 2414, 0.5, A::add_scaled_inputs3(s.ad_value(2419), 1.0, s.ad_value(2028), 1.0, s.ad_value(2029), -1.0));
            s.store_add(2422, 2420, 2416);
            s.store_sub(2423, 2422, 2421);
        }

        s.b[2552] = (p.p45 > 0.0);
        s.v[2552] = if s.b[2552] { 1.0 } else { 0.0 };

        s.b[2553] = (((s.v[2423]) as f64).abs() < 1e-5);
        s.v[2553] = if s.b[2553] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && s.b[2553]) {
            s.store_offset_ad(2424, A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2423), 1.0, A::scale(s.ad_value(2423), 0.3125), 0.5)), 1.0);
        }

        s.b[2554] = (s.v[2423] < 460.51701859880916);
        s.v[2554] = if s.b[2554] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && s.b[2554]) {
            s.store_exp_neg_input(2438, 2423);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && (!s.b[2554])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2423), (-460.51701859880916), A::scale_offset(s.ad_value(2423), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {
            s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {
            s.store_offset_ad(2424, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2398), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2423))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2423), 1.0, s.ad_value(2438))), 2.0), 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && (!s.b[2552])) {
            s.store_offset_div_scaled_inputs(2424, s.ad_value(2398), 0.5, A::sqrt(s.ad_value(2423)), 1.0, 1.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_add_scaled_value_products(2425, s.ad_value(2423), 1.0, s.ad_value(2398), A::sqrt(s.ad_value(2423)), 1.0, s.ad_value(2424), A::ln(A::offset(s.ad_value(2424), (-1.0))), (-1.0));
            s.store_div_scaled_inputs2(2426, s.ad_value(2417), 1.0, s.ad_value(2425), (-1.0), s.ad_value(2424), 1.0);
            s.store_mul_scaled_ad_rhs(2432, 2399, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0)));
            s.store_scalar(2431, 0.0);
            s.store_scalar(2433, 1.0);
        }

        s.b[2555] = (s.v[2426] > (-30.0));
        s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_offset_mul(2427, 2424, 2426, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2027, 2427, 2427, 10.0, 0.5);
            s.store_sub_ad_rhs(2428, 2426, A::ln(s.ad_value(2027)));
            s.store_scaled_add_sqrt_square_offset_rhs(2429, 2428, 2428, 2.0, 0.5);
        }

        s.b[2556] = ((s.v[2426] - s.v[2429]) < 230.25850929940458);
        s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2556]) {
            s.store_exp_sub(2027, 2426, 2429);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2556])) {
            s.store_scaled_offset_mul_offset_lhs_ad(2027, A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2426), s.ad_value(2429)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_div(2430, 2027, 2424);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2429), 1.0, 2.0), 2430);
        }

        s.b[2557] = (s.v[2430] > 1e-6);
        s.v[2557] = if s.b[2557] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2557]) {
            s.store_mul_offset_ad_rhs(2431, 2424, A::sub(s.ad_value(2429), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2430), s.ad_value(2027)), 1.0)), 1.0, (-1.0), s.ad_value(2430), 1.0)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2557])) {
            s.store_mul_ad_affine_product_rhs(2431, 2424, s.ad_value(2430), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_add_scaled_inputs3_offset(2027, s.ad_value(2417), 0.5, s.ad_value(2431), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0), A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_ad_rhs(2432, 2399, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2433, 2432, A::add(s.ad_value(2432), s.ad_value(2431)));
            s.store_add_scaled_product_indices(2423, 2422, 1.0, 2433, 2421, (-1.0));
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);
            s.store_scale(2435, 2434, 1e-5);
            s.store_div_from_scalar(2436, 1.0, 2434);
            s.store_scalar(2543, 0.0);
            s.store_scalar(2437, 0.0);
        }

        s.b[2558] = (s.v[2423] < 460.51701859880916);
        s.v[2558] = if s.b[2558] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2558]) {
            s.store_exp_neg_input(2438, 2423);
        }

        if ((s.b[2547] && s.b[2548]) && (!s.b[2558])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2423), (-460.51701859880916), A::scale_offset(s.ad_value(2423), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2559] = (((s.v[2417]) as f64).abs() <= s.v[2435]);
        s.v[2559] = if s.b[2559] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2559]) {
            s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2437, 2417, s.ad_value(2436), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2438)), s.ad_value(2398), s.ad_value(2523)), 1.0));
        }

        s.b[2560] = (s.v[2417] < (-s.v[2435]));
        s.v[2560] = if s.b[2560] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {
            s.store_neg(2525, 2417);
            s.store_scaled_mul(2526, 2525, 2436, 1.25);
            s.store_scaled_sub_ad(2527, A::offset(s.ad_value(2526), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2526), (-6.0), A::offset(s.ad_value(2526), (-6.0))), 64.0)), 0.5);
            s.store_sub(2522, 2525, 2527);
            s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::offset(s.ad_value(2527), 1.0), 1.0);
            s.store_sub_scaled_inputs(2529, 2522, 2.0, 2399, 1.0);
            s.store_sub_ad_lhs(2530, A::ln(A::mul(s.ad_value(2528), s.ad_value(2415))), 2527);
            s.store_add(824, 2528, 2529);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.5, s.ad_value(2528), 1.0), 1.0);
            s.store_add_ad_rhs(2531, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.3333333333333333, s.ad_value(2528), 1.0))), 1.0));
        }

        s.b[2561] = (s.v[2531] < 230.25850929940458);
        s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && s.b[2561]) {
            s.store_exp(2532, 2531);
        }

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && (!s.b[2561])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2532, 2531, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2531), (-230.25850929940458), A::scale_offset(s.ad_value(2531), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_div_from_scalar_offset_ad(2522, 1.0, A::square(s.ad_value(2531)), 2.0);
            s.store_mul_square_lhs(2534, 2531, 2522);
            s.store_mul3_affine_lhs(2535, 2531, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), s.ad_value(2522), 2522);
            s.store_sub(2522, 2525, 2531);
            s.store_mul(2523, 2438, 2533);
            s.store_add_scaled_product_right_ad(2537, 2522, 2.0, 2399, A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2523), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2535)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2531), (-1.0), s.ad_value(2523), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::sub(A::offset(s.ad_value(2531), (-1.0)), s.ad_value(2534)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::add_scaled_inputs_product(s.ad_value(2532), 1.0, s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0))));
            s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2437, 2531, -1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_div_from_scalar_offset_scaled_input(2539, 1.0, 2398, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2540, A::mul_scaled_lhs(s.ad_value(2434), 1.25, s.ad_value(2539)), (-1.0), 2539);
            s.store_mul_ad_product_rhs(2541, 2417, s.ad_value(2436), A::offset(A::mul(s.ad_value(2540), s.ad_value(2417)), 1.0));
        }

        s.b[2562] = ((-s.v[2541]) > (-230.25850929940458));
        s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2562]) {
            s.store_exp_neg_input(2522, 2541);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2562])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2522, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2541)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2541)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_sub_from_scalar(2542, 1.0, 2522);
            s.store_add_scaled_inputs_product_right_ad(2543, 2417, 1.0, 2399, 0.5, 2398, A::sqrt(A::add_scaled_inputs3(s.ad_value(2417), 1.0, s.ad_value(2399), 0.25, s.ad_value(2542), -1.0)), (-1.0));
            s.store_offset(2544, 2423, 3.0);
            s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0)), 0.5));
            s.store_sub(2522, 2417, 2527);
            s.store_exp_neg_input(2523, 2527);
            s.store_div_from_scalar_offset_ad(2524, 1.0, A::square(s.ad_value(2527)), 2.0);
            s.store_mul_square_lhs(2534, 2527, 2524);
            s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), s.ad_value(2524), 2524);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2438] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::mul_scaled_output(s.ad_value(2399), A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2530, s.ad_value(2423), 1.0, s.ad_value(2527), (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);
            s.store_add(824, 2528, 2529);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);
            s.store_add_ad_rhs(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));
        }

        s.b[2563] = (s.v[2546] < 230.25850929940458);
        s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2563]) {
            s.store_exp(2532, 2546);
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_mul(2532, 2438, 2532);
        }

        s.b[2564] = (s.v[2546] > (s.v[2423] - 230.25850929940458));
        s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };

        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && s.b[2564]) {
            s.store_exp_sub(2532, 2546, 2423);
            s.store_div(2533, 2438, 2532);
        }

        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && (!s.b[2564])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2423), s.ad_value(2546)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2546), (-230.25850929940458), A::scale_offset(s.ad_value(2546), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_div_from_scalar_offset_ad(2522, 1.0, A::square(s.ad_value(2546)), 2.0);
            s.store_mul_square_lhs(2534, 2546, 2522);
            s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), s.ad_value(2522), 2522);
            s.store_sub(2522, 2417, 2546);
            s.store_add_scaled_product_right_ad(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0))));
            s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2437, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_scalar(2440, 0.0);
            s.store_scalar(2441, 0.0);
            s.store_scalar(2442, 0.0);
            s.store_scalar(2443, 0.0);
            s.store_scalar(2444, 0.0);
            s.store_scalar(2445, 0.0);
            s.store_scalar(2446, 0.0);
            s.store_scalar(2447, 1.0);
            s.store_scalar(2448, 1.0);
            s.store_sub(2449, 2417, 2437);
            s.store_scalar(2450, 0.0);
            s.store_mul(2451, 2413, 2449);
            s.store_scalar(2452, 1.0);
            s.store_scalar(2453, 1.0);
            s.store_scalar(2457, 1.0);
            s.store_scalar(2458, 1.0);
            s.store_scalar(2460, 1.0);
        }

        s.b[2565] = (s.v[2417] > 0.0);
        s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_div_from_scalar_offset_ad(2027, 1.0, A::square(s.ad_value(2437)), 2.0);
            s.store_mul_square_lhs(2439, 2437, 2027);
            s.store_mul3_affine_lhs(2440, 2437, 2027, 4.0, 0.0, 2027);
            s.store_mul_ad_product_lhs(2441, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2439), 12.0), s.ad_value(2027), 2027);
            s.store_scalar(2442, 0.0);
        }

        s.b[2566] = (s.v[2437] < 230.25850929940458);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2566]) {
            s.store_exp(2442, 2437);
            s.store_div_from_scalar(2443, 1.0, 2442);
            s.store_mul(2442, 2438, 2442);
        }

        s.b[2567] = (s.v[2437] > (s.v[2423] - 230.25850929940458));
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {
            s.store_exp_sub(2442, 2437, 2423);
            s.store_div(2443, 2438, 2442);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2442, 1e-100, A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2423), s.ad_value(2437)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2443, 1e-100, 2437, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2437), (-230.25850929940458), A::scale_offset(s.ad_value(2437), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_add_scaled_product_right_ad(2444, 2442, 1.0, 2438, A::add(A::offset(s.ad_value(2437), 1.0), s.ad_value(2439)), (-1.0));
        }

        s.b[2568] = (s.v[2437] < 1e-5);
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2568]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2445, 2437, 1.0, 2437, 1.0, 2437, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2444, A::mul3(s.ad_value(2438), s.ad_value(2437), s.ad_value(2437)), 2437, A::scale_offset(s.ad_value(2437), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2437), 1.0, A::scale(s.ad_value(2437), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2446, 2437, 2027, 0.7071067811865475);
            s.store_offset_div_scaled_product(2447, s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.5)), 1.0, A::square(s.ad_value(2437)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2568])) {
            s.store_add_ad_lhs(2445, A::offset(s.ad_value(2437), (-1.0)), 2443);
            s.store_sqrt(2446, 2445);
            s.store_offset_scaled_ad(2447, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2443)), s.ad_value(2446)), 0.5, 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_div_scaled_offset_numerator(2448, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2397)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2397)), 1.0), 1.0);
        }

        s.b[2569] = (s.v[2444] > 1e-100);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul_sqrt_ad_rhs(2449, 2398, A::add(s.ad_value(2445), s.ad_value(2444)));
            s.store_div_scaled_product3_mixed_iiia(2450, 2399, 2444, 2413, 1.0, A::add_scaled_product(s.ad_value(2449), 1.0, s.ad_value(2398), s.ad_value(2446), 1.0), 1.0);
            s.store_mul3_lhs(2451, 2446, 2398, 2413);
        }

        s.b[2570] = (s.v[217] < 0.0);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2570]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2452, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2397)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2570])) {
            s.store_offset_mul(2452, 217, 2397, 1.0);
        }

        s.b[2571] = (s.v[218] < 0.0);
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2571]) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2450)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2571])) {
            s.store_div_from_scalar_offset_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2450)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul_product3_rhs(2454, 2450, s.ad_value(757), s.ad_value(2452), s.ad_value(2453), 1.0);
            s.store_mul_add_scaled_product_rhs(2455, 774, s.ad_value(2451), 1.0, s.ad_value(775), s.ad_value(2450), 1.0);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2445), 1.0, A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2457, A::offset(s.ad_value(2456), 1.0), s.ad_value(2454), 2448);
        }

        s.b[2572] = (s.v[221] < 0.0);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2572]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2458, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2397)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2572])) {
            s.store_offset_mul(2458, 221, 2397, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul(2029, 2450, 2458);
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.b[2573] = (s.v[222] < 0.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2573]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2573])) {
            s.store_offset_mul(2460, 222, 2459, 1.0);
        }

        if (s.b[2547] && (!s.b[2548])) {
            s.copy_ad(2395, 1822);
            s.copy_ad(2397, 1823);
            s.copy_ad(2413, 1824);
            s.copy_ad(2414, 1825);
            s.copy_ad(2398, 1826);
            s.copy_ad(2399, 1827);
            s.copy_ad(2415, 1828);
            s.copy_ad(2417, 1829);
            s.copy_ad(2422, 1830);
            s.copy_ad(2423, 1831);
            s.copy_ad(2434, 1832);
            s.copy_ad(2435, 1833);
            s.copy_ad(2436, 1834);
            s.copy_ad(2543, 1835);
            s.copy_ad(2438, 1836);
            s.copy_ad(2437, 1837);
            s.copy_ad(2440, 1838);
            s.copy_ad(2441, 1839);
            s.copy_ad(2442, 1840);
            s.copy_ad(2443, 1841);
            s.copy_ad(2445, 1842);
            s.copy_ad(2444, 1843);
            s.copy_ad(2446, 1844);
            s.copy_ad(2447, 1845);
            s.copy_ad(2448, 1846);
            s.copy_ad(2449, 1847);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2547] && (!s.b[2548])) {
            s.copy_ad(2450, 1848);
            s.copy_ad(2451, 1849);
            s.copy_ad(2452, 1850);
            s.copy_ad(2453, 1851);
            s.copy_ad(2457, 1852);
            s.copy_ad(2458, 1853);
            s.copy_ad(2460, 1854);
        }

        if s.b[2547] {
            s.copy_ad(2393, 720);
            s.copy_ad(2394, 777);
        }

        s.b[2574] = (p.p48 != 0.0);
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2574]) {
            s.copy_ad(2393, 721);
            s.copy_ad(2394, 778);
        }

        if s.b[2547] {
            s.store_scalar(2462, 0.0);
            s.store_scale(2461, 2413, 4.60517018598809);
            s.copy_ad(2478, 2461);
            s.copy_ad(2479, 826);
            s.store_mul(2480, 826, 2414);
            s.copy_ad(2484, 2437);
            s.store_scalar(2485, 0.0);
            s.store_scalar(2488, 0.0);
            s.copy_ad(2490, 2443);
            s.copy_ad(2491, 2445);
            s.copy_ad(2493, 2444);
            s.copy_ad(2494, 2451);
            s.copy_ad(2495, 2437);
            s.copy_ad(2496, 2443);
            s.copy_ad(2498, 2444);
            s.copy_ad(2499, 2445);
            s.store_sub(2500, 2417, 2437);
            s.store_scalar(2501, 1.0);
            s.store_scalar(2503, 1.0);
            s.store_scalar(2502, 0.0);
            s.copy_ad(2512, 2450);
            s.store_mul(2516, 2500, 2413);
            s.store_scalar(2513, 0.0);
            s.copy_ad(2514, 2451);
            s.store_scalar(2519, 0.0);
            s.store_scalar(2518, 1.0);
            s.copy_ad(2521, 2393);
            s.copy_ad(2520, 2516);
        }

        s.b[2575] = (s.v[2417] > 0.0);
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        s.b[2576] = (s.v[2444] > 1e-100);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul(2521, 2393, 2460);
            s.store_div(2462, 2521, 2457);
            s.store_add_scaled_inputs(2463, 2449, 1.0, 2399, 0.5);
            s.store_div_scaled_product_by_product(2027, s.ad_value(2399), s.ad_value(2442), 1.0, s.ad_value(2463), s.ad_value(2463), 1.0);
        }

        s.b[2577] = (s.v[2027] > 0.0001);
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.b[2578] = (s.v[2028] < 1e-10);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_scalar(2029, 1.0);
        }

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2577])) {
            s.store_scale(2029, 2027, 0.5);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul(2464, 2029, 2463);
        }

        s.b[2579] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_scaled_mul(2465, 2413, 2464, 0.475);
            s.store_add_scaled_product_indices(2027, 2450, 1.0, 2447, 2465, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2466, 2027, 2027, 1e-12, 0.5);
            s.store_add_scaled_value_products(2467, s.ad_value(2450), (-1.0), s.ad_value(2413), s.ad_value(2449), 1.0, A::offset(s.ad_value(2447), (-1.0)), s.ad_value(2465), 1.0);
            s.store_offset_div_scaled_product(2468, s.ad_value(2399), s.ad_value(2413), 0.5, s.ad_value(2467), 1.0, 1.0);
            s.store_add_scaled_product_indices(2027, 2467, 1.0, 775, 2466, 1.0);
            s.store_pow_ad(2469, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));
            s.store_mul_ad_lhs(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2468), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2469);
            s.store_div(2027, 2466, 2467);
            s.store_mul_pow_ad_rhs(2470, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));
            s.store_mul_div_scaled_product_rhs(2029, 2470, s.ad_value(707), A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, s.ad_value(2467), 1.0);
            s.store_mul_product3_rhs(2471, 2466, s.ad_value(757), s.ad_value(2452), s.ad_value(2453), 1.0);
            s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), s.ad_value(2468), (-1.0), s.ad_value(2029), 1.0), 1.0);
        }

        s.b[2580] = (s.v[2027] < 230.25850929940458);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && s.b[2580]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);
        }

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && (!s.b[2580])) {
            s.copy_ad(2028, 2027);
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_div_scaled_product3_mixed_iiia(2472, 2465, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2469), 1.0, s.ad_value(2470), 1.0, s.ad_value(2471), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2473, 2464, A::div_scaled_value_offset_denominator(s.ad_value(2472), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2472)), 1.0)), 1.0, 1.0), 1.0);
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2579])) {
            s.copy_ad(2473, 2464);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul3_affine_lhs(2474, 2413, 2462, 0.7071067811865475, 0.0, 2473);
        }

        s.b[2581] = (s.v[0] == (-1.0));
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2581]) {
            s.store_div_ad_rhs(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_div_from_scalar_offset_ad(2475, 2.0, A::sqrt(A::scale_offset(s.ad_value(2474), 4.0, 1.0)), 1.0);
            s.store_mul(2027, 2475, 2474);
            s.store_mul_ad_product_rhs(2476, 2473, s.ad_value(2475), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2475)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2475), 4.0), 1.0)), 1.0));
            s.store_scale(2477, 2476, 0.99);
            s.store_div_scaled_product3_mixed_iaii(2027, 2477, A::sub_scaled_inputs(s.ad_value(2477), 1.0, s.ad_value(2463), 2.0), 2415, 1.0, 2444, 1.0);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul_sub_ad_rhs(2478, 2413, s.ad_value(2477), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2576])) {
            s.copy_ad(2478, 2461);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_offset(2027, 2394, 1.0);
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2478, 1.0);
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2479, 2478, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);
            s.store_mul(2480, 2479, 2414);
            s.store_add(2481, 2423, 2480);
        }

        s.b[2582] = (s.v[2480] < 460.51701859880916);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2582]) {
            s.store_exp_neg_input(2482, 2480);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2582])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2482, 1e-200, 2480, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2480), (-460.51701859880916), A::scale_offset(s.ad_value(2480), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2483, 2438, 2482);
        }

        s.b[2583] = (((s.v[2417]) as f64).abs() <= s.v[2435]);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2583]) {
            s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2484, 2417, s.ad_value(2436), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2483)), s.ad_value(2398), s.ad_value(2523)), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_offset(2544, 2481, 3.0);
            s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0)), 0.5));
            s.store_sub(2522, 2417, 2527);
            s.store_exp_neg_input(2523, 2527);
            s.store_div_from_scalar_offset_ad(2524, 1.0, A::square(s.ad_value(2527)), 2.0);
            s.store_mul_square_lhs(2534, 2527, 2524);
            s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), s.ad_value(2524), 2524);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::mul_scaled_output(s.ad_value(2399), A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2530, s.ad_value(2481), 1.0, s.ad_value(2527), (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);
            s.store_add(824, 2528, 2529);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);
            s.store_add_ad_rhs(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));
        }

        s.b[2584] = (s.v[2546] < 230.25850929940458);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && (!s.b[2583])) && s.b[2584]) {
            s.store_exp(2532, 2546);
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_mul(2532, 2483, 2532);
        }

        s.b[2585] = (s.v[2546] > (s.v[2481] - 230.25850929940458));
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {
            s.store_exp_sub(2532, 2546, 2481);
            s.store_div(2533, 2483, 2532);
        }

        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && (!s.b[2585])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2481), s.ad_value(2546)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2546), (-230.25850929940458), A::scale_offset(s.ad_value(2546), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_div_from_scalar_offset_ad(2522, 1.0, A::square(s.ad_value(2546)), 2.0);
            s.store_mul_square_lhs(2534, 2546, 2522);
            s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), s.ad_value(2522), 2522);
            s.store_sub(2522, 2417, 2546);
            s.store_add_scaled_product_right_ad(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0))));
            s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2484, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_sub(2485, 2484, 2437);
        }

        s.b[2586] = (s.v[2485] < 1e-10);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {
            s.store_add_scaled_inputs_product_right_ad(2486, 2417, 2.0, 2437, (-2.0), 2399, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), s.ad_value(2440), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2487, A::mul_sub_from_scalar_rhs(s.ad_value(2399), 1.0, s.ad_value(2482)), 2444);
            s.store_sub_from_scalar_ad(2027, 2.0, A::mul(s.ad_value(2399), A::add_scaled_value_products(s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0, s.ad_value(2483), s.ad_value(2441), (-1.0))));
            s.store_add_scaled_square_product_indices(2027, 2486, 1.0, 2027, 2487, (-2.0));
            s.store_scaled_div_ad_rhs(2485, 2487, A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027))), 2.0);
            s.store_add(2484, 2437, 2485);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2488, 2485, 2413);
            s.store_div_scaled_product_offset_denominator(2489, s.ad_value(2484), s.ad_value(2484), 1.0, A::square(s.ad_value(2484)), 2.0, 1.0);
        }

        s.b[2587] = (s.v[2484] < 230.25850929940458);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2587]) {
            s.store_exp_neg_input(2490, 2484);
        }

        s.b[2588] = (s.v[2484] < 1e-5);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && s.b[2588]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2491, 2484, 1.0, 2484, 1.0, 2484, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);
            s.store_mul3_ad_middle(2493, A::mul3_scaled_output(s.ad_value(2483), s.ad_value(2484), s.ad_value(2484), 0.16666666666666666), 2484, A::scale_offset(s.ad_value(2484), 1.75, 1.0));
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
            s.store_sqrt(2492, 2491);
            s.store_mul_add_scaled_inputs3_offset_rhs(2493, 2483, A::div_from_scalar(1.0, s.ad_value(2490)), 1.0, s.ad_value(2484), (-1.0), s.ad_value(2489), -1.0, (-1.0));
        }

        s.b[2589] = (s.v[2484] > (s.v[2481] - 230.25850929940458));
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && s.b[2589]) {
            s.store_exp_sub(2027, 2484, 2481);
            s.store_div(2490, 2483, 2027);
            s.store_add_scaled_product_right_ad(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));
        }

        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && (!s.b[2589])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2490, 1e-100, 2484, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2484), (-230.25850929940458), A::scale_offset(s.ad_value(2484), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2481), s.ad_value(2484)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2587])) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
            s.store_sqrt(2492, 2491);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul3_lhs(2494, 2492, 2398, 2413);
            s.store_scaled_add(2495, 2437, 2484, 0.5);
            s.store_scalar(2496, 0.0);
            s.store_mul(2027, 2490, 2443);
        }

        s.b[2590] = (s.v[2027] > 0.0);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2590]) {
            s.store_sqrt(2496, 2027);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_scaled_add(2497, 2444, 2493, 0.5);
            s.store_add_scaled_product_mixed_iaa(2498, 2497, 1.0, A::square(s.ad_value(2485)), A::sub_scaled_inputs(s.ad_value(2496), 1.0, s.ad_value(2415), 2.0), 0.125);
        }

        s.b[2591] = (s.v[2495] < 1e-5);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2499, 2495, 1.0, 2495, 1.0, 2495, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
        }

        s.b[2592] = (s.v[730] > 0.0);
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2591]) && s.b[2592]) {
            s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);
            s.store_add_ad_rhs(2503, 2501, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), 1.0, A::square(s.ad_value(2495)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
        }

        s.b[2593] = (s.v[730] > 0.0);
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {
            s.store_add_scaled_sub_value_product_indices(2504, 1.0, 2496, 1.0, 2500, 2415, 2.0);
            s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));
            s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2501), 1.0, s.ad_value(2501), 1.0, 1.0);
            s.store_mul_product3_rhs(2505, 730, A::square(s.ad_value(2027)), s.ad_value(2399), s.ad_value(2498), 1.0);
            s.store_add_scaled_inputs_product_right_ad(2506, 2500, 2.0, 2505, (-2.0), 2399, A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2507, 2505, s.ad_value(2505), 1.0, s.ad_value(2500), 2.0);
            s.store_sub_from_scalar_ad(2508, 1.0, A::mul_scaled_output(s.ad_value(2399), A::add(s.ad_value(2496), s.ad_value(2498)), 0.5));
            s.store_div_scaled_product_denominator_ad(2509, 2507, 2506, 1.0, A::add_scaled_square_product(s.ad_value(2506), 1.0, s.ad_value(2508), s.ad_value(2507), (-1.0)), 1.0);
            s.store_add(2495, 2495, 2509);
            s.store_exp(2510, 2509);
            s.store_div(2496, 2496, 2510);
            s.store_mul(2498, 2498, 2510);
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
            s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::mul3_scaled_output(s.ad_value(2500), s.ad_value(2501), s.ad_value(2415), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2485, 2485, 2510, A::add(s.ad_value(2504), s.ad_value(2497)), 1.0, A::add_scaled_product(s.ad_value(2511), 1.0, s.ad_value(2510), s.ad_value(2497), 1.0), 1.0);
            s.store_mul(2488, 2485, 2413);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {
            s.store_sqrt(2502, 2499);
            s.store_add_scaled_inputs_ad_rhs(2503, 2501, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2496)), s.ad_value(2502)), 0.5);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul_div_scaled_product_rhs(2512, 2413, s.ad_value(2399), s.ad_value(2498), 1.0, A::add_scaled_product(s.ad_value(2500), 1.0, s.ad_value(2398), s.ad_value(2502), 1.0), 1.0);
            s.store_add_scaled_product_indices(2513, 2512, 1.0, 2413, 2503, 1.0);
            s.store_mul3_lhs(2514, 2502, 2398, 2413);
        }

        s.b[2594] = (s.v[218] < 0.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2594]) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2512)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2594])) {
            s.store_div_from_scalar_offset_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2512)), 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul_product3_rhs(2454, 2512, s.ad_value(757), s.ad_value(2452), s.ad_value(2453), 1.0);
            s.store_add_scaled_product_indices(2515, 2514, 1.0, 775, 2512, 1.0);
            s.store_add_scaled_product_indices(2516, 2514, 1.0, 776, 2512, 1.0);
            s.store_mul(2517, 774, 2515);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2499), 1.0, A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2518, A::offset(s.ad_value(2456), 1.0), s.ad_value(2454), 2448);
            s.store_ln_ad(2519, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0), 1.0));
            s.store_mul(2029, 2512, 2458);
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.b[2595] = (s.v[222] < 0.0);
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2595]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2595])) {
            s.store_offset_mul(2460, 222, 2459, 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2521, 2393, 2460);
            s.store_mul(2520, 2500, 2413);
        }

        if s.b[2547] {
            s.copy_ad(1887, 2395);
            s.copy_ad(1888, 2413);
            s.copy_ad(1889, 2398);
            s.copy_ad(1890, 2417);
            s.copy_ad(1891, 2422);
            s.copy_ad(1892, 2451);
            s.copy_ad(1893, 2488);
            s.copy_ad(1894, 2494);
            s.copy_ad(1895, 2501);
            s.copy_ad(1896, 2503);
            s.copy_ad(1897, 2512);
            s.copy_ad(1898, 2513);
            s.copy_ad(1899, 2516);
            s.copy_ad(1900, 2518);
            s.copy_ad(1901, 2519);
            s.copy_ad(1902, 2521);
            s.copy_ad(1903, 2520);
            s.copy_ad(1932, 2414);
            s.copy_ad(1933, 2435);
            s.copy_ad(1934, 2495);
            s.copy_ad(1935, 2500);
        }

        if (!s.b[2547]) {
            s.copy_ad(745, 728);
            s.copy_ad(1887, 1822);
            s.copy_ad(1888, 1824);
            s.copy_ad(1889, 1826);
            s.copy_ad(1890, 1829);
            s.copy_ad(1891, 1830);
            s.copy_ad(1892, 1849);
            s.copy_ad(1893, 1860);
            s.copy_ad(1894, 1861);
            s.copy_ad(1895, 1863);
            s.copy_ad(1896, 1864);
            s.copy_ad(1897, 1865);
            s.copy_ad(1898, 1866);
            s.copy_ad(1899, 1868);
            s.copy_ad(1900, 1869);
            s.copy_ad(1901, 1871);
            s.copy_ad(1902, 1870);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[2547]) {
            s.copy_ad(1903, 1872);
            s.copy_ad(1932, 1825);
            s.copy_ad(1933, 1833);
            s.copy_ad(1934, 1862);
            s.copy_ad(1935, 1931);
        }

        s.copy_ad(1904, 255);

        s.b[2596] = (s.v[773] > 0.0);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        if s.b[2596] {
            s.store_div_scaled_value_offset_denominator(1904, s.ad_value(255), 1.0, A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.v[1905] = 1.0;

        s.v[1906] = 1.0;

        s.v[1907] = 0.0;

        s.v[1908] = 1.0;

        s.v[1909] = 1.0;

        s.copy_ad(2359, 1903);

        s.v[2362] = 0.0;

        s.v[2361] = 0.0;

        s.copy_ad(2363, 2359);

        s.b[2597] = (s.v[1890] > 0.0);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if s.b[2597] {
            s.store_mul_div_scaled_product_rhs(2354, 1901, A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), s.ad_value(1897), 1.0, s.ad_value(1898), 1.0);
        }

        s.b[2598] = (s.v[2354] > 0.0);
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if (s.b[2597] && s.b[2598]) {
            s.store_div_from_scalar_add_ad(1905, 1.0, A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354)));
        }

        if (s.b[2597] && (!s.b[2598])) {
            s.store_sub_from_scalar(1905, 1.0, 2354);
        }

        if s.b[2597] {
            s.store_mul(1906, 1900, 1905);
            s.store_div(1907, 1902, 1906);
            s.store_mul_ad_product_lhs(2355, A::square(s.ad_value(1907)), s.ad_value(1893), 1893);
        }

        s.b[2599] = (s.v[0] == (-1.0));
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        if (s.b[2597] && s.b[2599]) {
            s.store_div_scaled_value_offset_denominator(2355, s.ad_value(2355), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2597] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1908, 1906, A::sqrt(A::scale_offset(s.ad_value(2355), 2.0, 1.0)), 1.0, 0.5);
            s.store_div(2027, 1906, 1908);
            s.store_mul_offset_ad_rhs(2356, 1896, A::mul3_scaled_output(s.ad_value(2355), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0);
            s.store_div_scaled_product_indices(1909, 2027, 1898, 1.0, 2356, 1.0);
            s.store_scaled_div(2357, 1893, 1909, 0.5);
            s.store_square(2358, 2357);
            s.store_add_ad_rhs(2359, 1903, A::mul3_scaled_output(s.ad_value(1895), s.ad_value(1893), A::add(A::offset(A::mul_scaled_output(s.ad_value(2357), s.ad_value(1905), 0.3333333333333333), (-1.0)), s.ad_value(1905)), 0.5));
            s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);
        }

        s.b[2600] = (p.p49 == 1.0);
        s.v[2600] = if s.b[2600] { 1.0 } else { 0.0 };

        if (s.b[2597] && s.b[2600]) {
            s.store_scalar(2360, 0.0);
            s.store_mul_ad_affine_product_rhs(2361, 1905, s.ad_value(1905), A::sub(s.ad_value(1897), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 2.0, s.ad_value(2357), 3.0)), 0.5, 0.0);
        }

        if (s.b[2597] && (!s.b[2600])) {
            s.store_mul_sub_from_scalar_lhs_ad_rhs(2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(1896), s.ad_value(1893), (-0.5)));
            s.store_add_scaled_products_mixed_aaia(2361, A::square(s.ad_value(1905)), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2357)), 1.0, s.ad_value(2358), 0.2), (-1.0)), 0.5, 2360, A::offset(s.ad_value(1905), 1.0), 0.5);
        }

        if s.b[2597] {
            s.store_add_scaled_product_right_ad(2362, 2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), s.ad_value(2357), 1.0), 1.0);
            s.store_sub(2363, 2359, 2362);
        }

        s.store_mul(851, 2359, 1904);

        s.store_mul_neg_lhs(853, 2361, 1904);

        s.store_mul_neg_lhs(852, 2363, 1904);

        s.v[2379] = 0.0;

        s.v[2380] = 0.0;

        s.v[2378] = 0.0;

        s.b[2601] = ((s.v[268] > 0.0) || (s.v[269] > 0.0));
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if s.b[2601] {
            s.store_scalar(2368, 1.0);
            s.copy_ad(2367, 1887);
        }

        s.b[2602] = (s.v[272] > 1e-10);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if (s.b[2601] && s.b[2602]) {
            s.store_add_scaled_inputs3(2364, s.ad_value(1887), 1.0, s.ad_value(270), (-1.0), s.ad_value(808), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2027, 2364, 0.5, 808, 0.5, A::add_scaled_product(s.ad_value(809), 1.0, A::sub(s.ad_value(2364), s.ad_value(808)), A::sub(s.ad_value(2364), s.ad_value(808)), 1.0), 0.5);
            s.store_mul_add_scaled_inputs3_offset_rhs(2028, 2027, s.ad_value(2027), 2.0, s.ad_value(808), (-1.0), s.ad_value(2364), -1.0, 0.0);
            s.store_div(2029, 808, 2027);
            s.store_mul(2365, 2364, 2029);
            s.store_sqrt_sub_from_scalar_ad(2366, 1.0, A::mul(s.ad_value(2365), s.ad_value(272)));
            s.store_add_scaled_inputs3(2367, A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), 1.0, s.ad_value(2364), 1.0, s.ad_value(2365), -1.0);
            s.store_offset_ad(2368, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add_scaled_product(s.ad_value(2028), 1.0, s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027)), 1.0), s.ad_value(2029), 1.0, s.ad_value(2028), 1.0), 1.0);
        }

        if s.b[2601] {
            s.store_scalar(2370, 1.0);
            s.store_scalar(2371, 0.0);
        }

        s.b[2603] = (s.v[271] > 0.0);
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if (s.b[2601] && s.b[2603]) {
            s.store_add_scaled_product_right_ad(2027, 745, 0.5, 1888, A::scale_offset(s.ad_value(1889), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2369, 1887, 2027);
        }

        s.b[2604] = (((s.v[2369]) as f64).abs() < 230.25850929940458);
        s.v[2604] = if s.b[2604] { 1.0 } else { 0.0 };

        if ((s.b[2601] && s.b[2603]) && s.b[2604]) {
            s.store_div_from_scalar_offset_ad(2370, 1.0, A::exp_scaled_input(s.ad_value(2369), -1.0), 1.0);
        }

        s.b[2605] = (s.v[2369] < 0.0);
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if (((s.b[2601] && s.b[2603]) && (!s.b[2604])) && s.b[2605]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2370, 1e-100, 2369, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2369), (-230.25850929940458), A::scale_offset(s.ad_value(2369), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2606] = (s.v[2369] < 230.25850929940458);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if ((s.b[2601] && s.b[2603]) && s.b[2606]) {
            s.store_ln_one_plus_exp(2028, 2369);
        }

        if ((s.b[2601] && s.b[2603]) && (!s.b[2606])) {
            s.copy_ad(2028, 2369);
        }

        if (s.b[2601] && s.b[2603]) {
            s.store_mul(2371, 2027, 2028);
        }

        if s.b[2601] {
            s.store_add_scaled_product_right_ad(2372, 2368, 1.0, 271, A::sub(s.ad_value(2370), s.ad_value(2368)), 1.0);
            s.store_add_scaled_product_right_ad(2373, 2367, 1.0, 271, A::sub(s.ad_value(2371), s.ad_value(2367)), 1.0);
            s.store_add_scaled_inputs3(2374, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1888), s.ad_value(1891), (-1.0)), 1.0, s.ad_value(1903), (-1.0), s.ad_value(1893), (-0.5));
            s.store_add_scaled_inputs3(2375, s.ad_value(1887), 1.0, s.ad_value(2374), (-1.0), s.ad_value(1892), -1.0);
            s.store_add_scaled_inputs3(2376, s.ad_value(1893), 1.0, s.ad_value(2374), 1.0, s.ad_value(826), -1.0);
            s.store_add_scaled_inputs3(2377, s.ad_value(1887), 1.0, s.ad_value(2376), (-1.0), s.ad_value(1894), -1.0);
        }

        s.b[2607] = (s.v[831] > 0.0);
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (s.b[2601] && s.b[2607]) {
            s.store_mul_ad_rhs(2378, 2372, A::add_scaled_products(s.ad_value(269), s.ad_value(2376), 1.0, s.ad_value(268), s.ad_value(2374), 1.0));
            s.store_mul_sub_rhs(2379, 268, 2375, 2373);
            s.store_mul_sub_rhs(2380, 269, 2377, 2373);
        }

        if (s.b[2601] && (!s.b[2607])) {
            s.store_mul_ad_rhs(2378, 2372, A::add_scaled_products(s.ad_value(268), s.ad_value(2376), 1.0, s.ad_value(269), s.ad_value(2374), 1.0));
            s.store_mul_sub_rhs(2379, 269, 2375, 2373);
            s.store_mul_sub_rhs(2380, 268, 2377, 2373);
        }

        if s.b[2601] {
            s.store_add(851, 851, 2378);
            s.store_add(853, 853, 2380);
            s.store_add_scaled_inputs4(852, s.ad_value(852), 1.0, s.ad_value(2378), (-1.0), s.ad_value(2380), -1.0, s.ad_value(2379), -1.0);
        }

        s.store_mul(1910, 262, 1878);

        s.store_mul(1911, 263, 1879);

        s.v[2383] = 0.0;

        s.v[2381] = 0.0;

        s.b[2608] = ((s.v[262] > 0.0) && (s.v[264] > 0.0));
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if s.b[2608] {
            s.store_mul_add_scaled_inputs_rhs(2027, 266, s.ad_value(1819), 0.5, s.ad_value(787), 1.0);
        }

        s.b[2609] = (s.v[2027] < 230.25850929940458);
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        s.b[2610] = (s.v[2027] > (-230.25850929940458));
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if ((s.b[2608] && s.b[2609]) && s.b[2610]) {
            s.store_exp(2381, 2027);
        }

        if ((s.b[2608] && s.b[2609]) && (!s.b[2610])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2381, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2611] = (s.v[2381] > 1e-10);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if ((s.b[2608] && s.b[2609]) && s.b[2611]) {
            s.store_ln_offset_input(2382, 2381, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2382, 1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)));
        }

        if ((s.b[2608] && s.b[2609]) && (!s.b[2611])) {
            s.copy_ad(2382, 2381);
            s.store_div_scaled_value_offset_denominator(2028, s.ad_value(2382), 2.0, s.ad_value(2382), 2.0, 1.0);
        }

        if (s.b[2608] && (!s.b[2609])) {
            s.copy_ad(2382, 2027);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2382, 1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)));
        }

        if s.b[2608] {
            s.store_mul_ad_affine_product_lhs(2383, A::div_scaled_inputs(s.ad_value(264), (-2.0), s.ad_value(266), 1.0), s.ad_value(262), s.v[354], 0.0, 2028);
        }

        s.v[2386] = 0.0;

        s.v[2384] = 0.0;

        s.b[2612] = ((s.v[263] > 0.0) && (s.v[265] > 0.0));
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        if s.b[2612] {
            s.store_mul_add_scaled_inputs_rhs(2027, 266, s.ad_value(1819), 0.5, s.ad_value(788), 1.0);
        }

        s.b[2613] = (s.v[2027] < 230.25850929940458);
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        s.b[2614] = (s.v[2027] > (-230.25850929940458));
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if ((s.b[2612] && s.b[2613]) && s.b[2614]) {
            s.store_exp(2384, 2027);
        }

        if ((s.b[2612] && s.b[2613]) && (!s.b[2614])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2384, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2615] = (s.v[2384] > 1e-10);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if ((s.b[2612] && s.b[2613]) && s.b[2615]) {
            s.store_ln_offset_input(2385, 2384, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2385, 1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)));
        }

        if ((s.b[2612] && s.b[2613]) && (!s.b[2615])) {
            s.copy_ad(2385, 2384);
            s.store_div_scaled_value_offset_denominator(2028, s.ad_value(2385), 2.0, s.ad_value(2385), 2.0, 1.0);
        }

        if (s.b[2612] && (!s.b[2613])) {
            s.copy_ad(2385, 2027);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2385, 1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)));
        }

        if s.b[2612] {
            s.store_mul_ad_affine_product_lhs(2386, A::div_scaled_inputs(s.ad_value(265), (-2.0), s.ad_value(266), 1.0), s.ad_value(263), s.v[354], 0.0, 2028);
        }

        s.store_add(2387, 2383, 2386);

        s.store_add_scaled_product_indices(856, 2387, 1.0, 267, 829, 1.0);

        s.store_mul(854, 274, 834);

        s.store_mul(855, 275, 837);

        s.v[1938] = 0.0;

        s.v[1939] = 0.0;

        s.v[1940] = 0.0;

        s.v[1941] = 0.0;

        s.b[2616] = (s.v[1] != 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        s.b[2617] = (s.v[1890] <= 0.0);
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if (s.b[2616] && s.b[2617]) {
            s.store_scalar(1936, 0.5);
            s.store_scalar(1937, 1.0);
            s.copy_ad(1938, 1889);
        }

        if (s.b[2616] && (!s.b[2617])) {
            s.store_offset_scaled_div(1936, 1893, 1909, ((0.25) * (0.5)), 0.5);
            s.store_div_ad_rhs(1937, 1935, A::sub(s.ad_value(1890), s.ad_value(1934)));
            s.store_div(1938, 1889, 1937);
        }

        if s.b[2616] {
            s.store_square(1939, 1938);
            s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);
            s.store_scale(1941, 1940, 1e-5);
        }

        s.v[2618] = 0.0;

        s.v[2621] = 0.0;

        s.v[2622] = 0.0;

        s.v[2623] = 0.0;

        s.v[2624] = 0.0;

        s.v[2625] = 0.0;

        s.v[2626] = 0.0;

        s.v[2627] = 0.0;

        s.v[2628] = 0.0;

        s.v[2629] = 0.0;

        s.v[2630] = 0.0;

        s.v[2631] = 0.0;

        s.v[2632] = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[2633] = 0.0;

        s.v[2634] = 0.0;

        s.v[2635] = 0.0;

        s.v[2636] = 0.0;

        s.v[2639] = 0.0;

        s.v[2643] = 0.0;

        s.v[2646] = 0.0;

        s.v[2647] = 0.0;

        s.v[2648] = 0.0;

        s.v[2649] = 0.0;

        s.v[2650] = 0.0;

        s.v[2651] = 0.0;

        s.v[2654] = 0.0;

        s.v[2655] = 0.0;

        s.v[2656] = 0.0;

        s.v[2657] = 0.0;

        s.v[2661] = 0.0;

        s.v[2663] = 0.0;

        s.v[2664] = 0.0;

        s.v[857] = 0.0;

        s.v[1918] = 0.0;

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[858] = 0.0;

        s.v[1921] = 0.0;

        s.v[1922] = 0.0;

        s.v[1923] = 0.0;

        s.b[2665] = (p.p43 > 0.0);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        s.b[2666] = (s.v[474] == 1.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if (s.b[2665] && s.b[2666]) {
            s.store_scalar(2669, 0.0);
            s.store_scalar(2670, 0.0);
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2670, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2671] = (s.v[651] > 0.5);
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        s.b[2672] = (s.v[408] == 0.5);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && s.b[2672]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[405]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && (!s.b[2672])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2671]) {
            s.store_add_scaled_inputs3_offset(1918, s.ad_value(2669), (-s.v[417]), s.ad_value(832), s.v[420], s.ad_value(2670), (-s.v[420]), s.v[417]);
        }

        s.b[2673] = (s.v[652] > 0.5);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        s.b[2674] = (s.v[409] == 0.5);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && s.b[2674]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[406]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && (!s.b[2674])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2673]) {
            s.store_add_scaled_inputs3_offset(1919, s.ad_value(2669), (-s.v[418]), s.ad_value(832), s.v[421], s.ad_value(2670), (-s.v[421]), s.v[418]);
        }

        s.b[2675] = (s.v[653] > 0.5);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        s.b[2676] = (s.v[410] == 0.5);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && s.b[2676]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[407]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && (!s.b[2676])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2675]) {
            s.store_add_scaled_inputs3_offset(1920, s.ad_value(2669), (-s.v[419]), s.ad_value(832), s.v[422], s.ad_value(2670), (-s.v[422]), s.v[419]);
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_scalar(2669, 0.0);
            s.store_scalar(2670, 0.0);
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_scaled_product_indices(2623, 833, 1.0, 684, 2622, 1.0);
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2670, 833, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2677] = (s.v[678] > 0.5);
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        s.b[2678] = (s.v[575] == 0.5);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && s.b[2678]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(572)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && (!s.b[2678])) {
            s.store_pow_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(572))), s.ad_value(575));
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2677]) {
            s.store_add_scaled_product_mixed_aia(1921, A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2669)), 1.0, 587, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);
        }

        s.b[2679] = (s.v[679] > 0.5);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        s.b[2680] = (s.v[576] == 0.5);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && s.b[2680]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(573)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && (!s.b[2680])) {
            s.store_pow_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2679]) {
            s.store_add_scaled_product_mixed_aia(1922, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2669)), 1.0, 588, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);
        }

        s.b[2681] = (s.v[680] > 0.5);
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        s.b[2682] = (s.v[577] == 0.5);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && s.b[2682]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(574)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && (!s.b[2682])) {
            s.store_pow_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(574))), s.ad_value(577));
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2681]) {
            s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2669)), 1.0, 589, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);
        }

        s.b[2683] = (p.p872 > 0.0);
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2683]) {
            s.store_scaled_offset_ad(642, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001))), 0.5), p.p873), (-(((0.5 * 0.001)) as f64).powf(p.p873)), p.p872);
            s.store_offset(640, 642, p.p862);
            s.store_div_from_scalar(450, 1.0, 640);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2683])) {
            s.store_scalar(640, p.p862);
        }

        s.b[2684] = (p.p874 > 0.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2684]) {
            s.store_scaled_offset_ad(644, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001))), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875)), p.p874);
            s.store_mul_offset_rhs(443, 443, 644, 1.0);
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_scalar(2634, 0.0);
            s.store_scalar(2631, 0.0);
        }

        s.b[2685] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_div_scaled_product_add_scaled_denominator_indices(2628, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);
        }

        s.b[2686] = (s.v[832] < s.v[654]);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        s.b[2687] = (((((-0.5) * (s.v[832] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
            s.store_exp_scaled_input(2629, 832, (s.v[371] * (-0.5)));
        }

        s.b[2688] = (((-0.5) * (s.v[832] * s.v[371])) < 0.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && s.b[2688]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2629, 1e-100, (-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && (!s.b[2688])) {
            s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(832), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) {
            s.store_div_from_scalar(2630, 1.0, 2629);
            s.store_square(2627, 2630);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2686])) {
            s.store_mul_offset_ad_lhs(2627, A::sub_scaled_inputs(s.ad_value(832), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(2630, 2627);
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.b[2689] = (s.v[832] > 0.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2689]) {
            s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2689])) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 832);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_sub(2632, 656, 2631);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2633, 832, 0.5, 2632, 0.5, A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(2632)), A::sub(s.ad_value(832), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2634, 832, 0.5, 659, 0.5, A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(659)), A::sub(s.ad_value(832), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_ad_rhs(2635, 832, A::sqrt(A::offset(A::mul(s.ad_value(832), s.ad_value(832)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2690] = (s.v[646] == 0.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2690]) {
            s.store_scalar(1918, 0.0);
        }

        s.b[2691] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_sub_from_scalar(2639, s.v[393], 2633);
        }

        s.b[2693] = (p.p831 == 0.5);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2693]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[429]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2693])) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[429]), p.p831);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_scale(2643, 2636, s.v[423]);
        }

        s.b[2694] = (p.p845 == 0.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_div_scaled_inputs(2646, s.ad_value(2643), (s.v[408] * s.v[438]), s.ad_value(2639), 1.0);
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product_offset_denominator(s.ad_value(2648), s.ad_value(2648), 1.0, A::square(s.ad_value(2648)), 1.0, 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);
            s.store_add_scaled_value_products(2656, s.ad_value(2649), (-s.v[435]), s.ad_value(2647), s.ad_value(2650), s.v[435], s.ad_value(2646), s.ad_value(2651), 0.5);
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2697] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2697]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2697])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        s.b[2698] = (s.v[2657] > 0.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        s.b[2699] = (s.v[2656] > (-230.25850929940458));
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2700] = (p.p851 == 0.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        s.b[2701] = (p.p831 == 0.5);
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2701]) {
            s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2701])) {
            s.store_powf_ad(2636, A::scale_offset(s.ad_value(2634), (-s.v[429]), ((p.p828) * (s.v[429]))), p.p831);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {
            s.store_div_scaled_offset_numerator(2661, s.ad_value(2634), ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), s.ad_value(2636), 1.0);
        }

        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

    }
}
