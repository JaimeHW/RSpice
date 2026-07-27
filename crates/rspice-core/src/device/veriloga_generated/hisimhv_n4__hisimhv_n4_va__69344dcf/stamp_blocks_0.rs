#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[623] = param_given[12];s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });s.b[769] = param_given[268];s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });s.b[768] = param_given[269];s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });s.store_scalar(294, 0.0);s.store_scalar(295, 0.0);s.store_scalar(708, 0.0);s.store_scalar(4, 0.0);s.store_scalar(5, 0.0);s.store_scalar(321, 0.0);s.store_scalar(78, 0.0);s.store_scalar(74, 0.0);s.store_scalar(347, 0.0);s.store_scalar(697, 0.0);s.store_scalar(698, 0.0);s.store_scalar(69, 0.8);s.store_scalar(70, 0.4);s.store_scalar(77, 0.0);s.store_scalar(79, 0.0);s.store_scalar(80, 0.0);s.store_scalar(81, 0.0);s.store_scalar(83, 0.0);s.store_scalar(84, 0.0);s.store_scalar(85, 0.0);s.store_scalar(86, 0.0);s.store_scalar(87, 0.0);s.store_scalar(88, 0.0);s.store_scalar(89, 0.0);s.store_scalar(90, 0.0);s.store_scalar(91, 0.0);s.store_scalar(92, 0.0);s.store_scalar(93, 0.0);s.store_scalar(94, 0.0);s.store_scalar(95, 0.0);s.store_scalar(96, 0.0);s.store_scalar(97, 0.0);s.store_scalar(98, 0.0);s.store_scalar(99, 0.0);s.store_scalar(100, 0.0);s.store_scalar(101, 0.0);s.store_scalar(102, 0.0);s.store_scalar(103, 0.0);s.store_scalar(104, 0.0);s.store_scalar(105, 0.0);s.store_scalar(106, 0.0);s.store_scalar(107, 0.0);s.store_scalar(108, 0.0);s.store_scalar(109, 0.0);s.store_scalar(110, 0.0);s.store_scalar(111, 0.0);s.store_scalar(112, 0.0);s.store_scalar(113, 0.0);s.store_scalar(114, 0.0);s.store_scalar(115, 0.0);s.store_scalar(116, 0.0);s.store_scalar(415, 0.0);s.store_scalar(117, 0.0);s.store_scalar(118, 0.0);s.store_scalar(119, 0.0);s.store_scalar(120, 0.0);s.store_scalar(121, 0.0);s.store_scalar(122, 0.0);s.store_scalar(123, 0.0);s.store_scalar(124, 0.0);s.store_scalar(125, 0.0);s.store_scalar(126, 0.0);s.store_scalar(127, 0.0);s.store_scalar(128, 0.0);s.store_scalar(129, 0.0);s.store_scalar(130, 0.0);s.store_scalar(20, 0.0);s.store_scalar(131, 0.0);s.store_scalar(132, 0.0);s.store_scalar(133, 0.0);s.store_scalar(19, 0.0);s.store_scalar(134, 0.0);s.store_scalar(135, 0.0);s.store_scalar(137, 0.0);s.store_scalar(138, 0.0);s.store_scalar(139, 0.0);s.store_scalar(140, 0.0);s.store_scalar(141, 0.0);s.store_scalar(142, 0.0);s.store_scalar(143, 0.0);s.store_scalar(144, 0.0);s.store_scalar(145, 0.0);s.store_scalar(146, 0.0);s.store_scalar(147, 0.0);s.store_scalar(148, 0.0);s.store_scalar(149, 0.0);s.store_scalar(150, 0.0);s.store_scalar(151, 0.0);s.store_scalar(152, 0.0);s.store_scalar(153, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
    ) {
        s.store_scalar(154, 0.0);s.store_scalar(155, 0.0);s.store_scalar(156, 0.0);s.store_scalar(157, 0.0);s.store_scalar(158, 0.0);s.store_scalar(159, 0.0);s.store_scalar(160, 0.0);s.store_scalar(161, 0.0);s.store_scalar(162, 0.0);s.store_scalar(163, 0.0);s.store_scalar(164, 0.0);s.store_scalar(165, 0.0);s.store_scalar(166, 0.0);s.store_scalar(167, 0.0);s.store_scalar(168, 0.0);s.store_scalar(169, 0.0);s.store_scalar(170, 0.0);s.store_scalar(171, 0.0);s.store_scalar(172, 0.0);s.store_scalar(173, 0.0);s.store_scalar(174, 0.0);s.store_scalar(175, 0.0);s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(178, 0.0);s.store_scalar(179, 0.0);s.store_scalar(180, 0.0);s.store_scalar(181, 0.0);s.store_scalar(182, 0.0);s.store_scalar(184, 0.0);s.store_scalar(185, 0.0);s.store_scalar(186, 0.0);s.store_scalar(187, 0.0);s.store_scalar(188, 0.0);s.store_scalar(412, 0.0);s.store_scalar(189, 0.0);s.store_scalar(190, 0.0);s.store_scalar(191, 0.0);s.store_scalar(192, 0.0);s.store_scalar(193, 0.0);s.store_scalar(194, 0.0);s.store_scalar(195, 0.0);s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(200, 0.0);s.store_scalar(201, 0.0);s.store_scalar(202, 0.0);s.store_scalar(203, 0.0);s.store_scalar(204, 0.0);s.store_scalar(205, 0.0);s.store_scalar(206, 0.0);s.store_scalar(207, 0.0);s.store_scalar(208, 0.0);s.store_scalar(209, 0.0);s.store_scalar(210, 0.0);s.store_scalar(211, 0.0);s.store_scalar(212, 0.0);s.store_scalar(213, 0.0);s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);s.store_scalar(218, 0.0);s.store_scalar(219, 0.0);s.store_scalar(220, 0.0);s.store_scalar(221, 0.0);s.store_scalar(222, 0.0);s.store_scalar(223, 0.0);s.store_scalar(224, 0.0);s.store_scalar(225, 0.0);s.store_scalar(226, 0.0);s.store_scalar(227, 0.0);s.store_scalar(228, 0.0);s.store_scalar(229, 0.0);s.store_scalar(230, 0.0);s.store_scalar(231, 0.0);s.store_scalar(232, 0.0);s.store_scalar(233, 0.0);s.store_scalar(234, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(237, 0.0);s.store_scalar(238, 0.0);s.store_scalar(239, 0.0);s.store_scalar(240, 0.0);s.store_scalar(241, 0.0);s.store_scalar(242, 0.0);s.store_scalar(243, 0.0);s.store_scalar(244, 0.0);s.store_scalar(245, 0.0);s.store_scalar(246, 0.0);s.store_scalar(247, 0.5);s.store_scalar(248, 0.0);s.store_scalar(249, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
    ) {
        s.store_scalar(250, 0.0);s.store_scalar(251, 0.0);s.store_scalar(252, 0.0);s.store_scalar(253, 0.0);s.store_scalar(254, 0.0);s.store_scalar(255, 0.0);s.store_scalar(256, 0.0);s.store_scalar(258, 0.0);s.store_scalar(259, 0.0);s.store_scalar(260, 0.0);s.store_scalar(261, 0.0);s.store_scalar(262, 0.0);s.store_scalar(263, 0.0);s.store_scalar(264, 0.0);s.store_scalar(265, 0.0);s.store_scalar(266, 0.0);s.store_scalar(267, 0.0);s.store_scalar(268, 0.0);s.store_scalar(269, 0.0);s.store_scalar(270, 0.0);s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);s.store_scalar(273, 0.0);s.store_scalar(274, 0.0);s.store_scalar(275, 0.0);s.store_scalar(276, 0.0);s.store_scalar(277, 0.0);s.store_scalar(278, 0.0);s.store_scalar(279, 0.0);s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);s.store_scalar(286, 0.0);s.store_scalar(289, 0.0);s.store_scalar(290, 0.0);s.store_scalar(291, 0.0);s.store_scalar(292, 0.0);s.store_scalar(293, 0.0);s.store_scalar(296, 0.0);s.store_scalar(297, 0.0);s.store_scalar(298, 0.0);s.store_scalar(299, 0.0);s.store_scalar(300, 0.0);s.store_scalar(301, 0.0);s.store_scalar(302, 0.0);s.store_scalar(303, 0.0);s.store_scalar(304, 0.0);s.store_scalar(305, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(313, 0.0);s.store_scalar(314, 0.0);s.store_scalar(315, 0.0);s.store_scalar(316, 0.0);s.store_scalar(317, 0.0);s.store_scalar(318, 0.0);s.store_scalar(319, 0.0);s.store_scalar(320, 0.0);s.store_scalar(322, 0.0);s.store_scalar(323, 0.0);s.store_scalar(324, 0.0);s.store_scalar(325, 0.0);s.store_scalar(326, 0.0);s.store_scalar(327, 0.0);s.store_scalar(328, 0.0);s.store_scalar(329, 0.0);s.store_scalar(330, 0.0);s.store_scalar(331, 0.0);s.store_scalar(332, 0.0);s.store_scalar(333, 0.0);s.store_scalar(334, 0.0);s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);s.store_scalar(340, 0.0);s.store_scalar(341, 0.0);s.store_scalar(342, 0.0);s.store_scalar(343, 0.0);s.store_scalar(344, 0.0);s.store_scalar(345, 0.0);s.store_scalar(346, 0.0);s.store_scalar(348, 0.0);s.store_scalar(349, 0.0);s.store_scalar(350, 0.0);s.store_scalar(351, 0.0);s.store_scalar(352, 0.0);s.store_scalar(353, 0.0);s.store_scalar(354, 0.0);s.store_scalar(355, 0.0);s.store_scalar(356, 0.0);s.store_scalar(357, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(358, 0.0);s.store_scalar(359, 0.0);s.store_scalar(364, 0.0);s.store_scalar(366, 0.0);s.store_scalar(367, 0.0);s.store_scalar(368, 0.0);s.store_scalar(369, 0.0);s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(373, 0.0);s.store_scalar(374, 0.0);s.store_scalar(375, 0.0);s.store_scalar(376, 0.0);s.store_scalar(377, 0.0);s.store_scalar(380, 0.0);s.store_scalar(381, 0.0);s.store_scalar(382, 0.0);s.store_scalar(383, 0.0);s.store_scalar(387, 0.0);s.store_scalar(388, 0.0);s.store_scalar(389, 0.0);s.store_scalar(390, 0.0);s.store_scalar(391, 0.0);s.store_scalar(392, 0.0);s.store_scalar(393, 0.0);s.store_scalar(394, 0.0);s.store_scalar(395, 0.0);s.store_scalar(396, 0.0);s.store_scalar(397, 0.0);s.store_scalar(398, 0.0);s.store_scalar(399, 0.0);s.store_scalar(400, 0.0);s.store_scalar(402, 0.0);s.store_scalar(403, 0.0);s.store_scalar(404, 0.0);s.store_scalar(405, 0.0);s.store_scalar(385, p[334]);s.store_scalar(386, p[334]);s.store_scalar(409, 0.0);s.store_scalar(410, 0.0);s.store_scalar(434, 0.0093868);s.store_scalar(435, (-0.1047839));s.store_scalar(447, 0.0);s.store_scalar(573, 0.0);s.store_scalar(574, 0.0);s.store_scalar(575, 0.0);s.store_scalar(576, 0.0);s.store_scalar(577, 0.0);s.store_scalar(578, 0.0);s.store_scalar(579, 0.0);s.store_scalar(580, 0.0);s.store_scalar(581, 0.0);s.store_scalar(582, 0.0);s.store_scalar(583, 0.0);s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(586, 0.0);s.store_scalar(587, 0.0);s.store_scalar(588, 0.0);s.store_scalar(589, 0.0);s.store_scalar(590, 0.0);s.store_scalar(591, 0.0);s.store_scalar(592, 0.0);s.store_scalar(593, 0.0);s.store_scalar(594, 0.0);s.store_scalar(595, 0.0);s.store_scalar(596, 0.0);s.store_scalar(597, 0.0);s.store_scalar(739, 0.0);s.store_scalar(598, 0.0);s.store_scalar(770, 0.0);s.store_scalar(727, 0.0);s.store_scalar(728, 0.0);s.store_scalar(729, 0.0);s.store_scalar(730, 0.0);s.store_scalar(731, 0.0);s.store_scalar(732, 0.0);s.store_scalar(733, 0.0);s.store_scalar(734, 0.0);s.store_scalar(735, 0.0);s.store_scalar(736, 0.0);s.store_scalar(737, 0.0);s.store_scalar(738, 0.0);s.store_scalar(740, 0.0);s.store_scalar(18, 0.0);s.store_scalar(741, 0.0);s.store_scalar(745, 0.0);s.store_scalar(746, 0.0);s.store_scalar(747, 0.0);s.store_scalar(748, 0.0);s.store_scalar(749, 0.0);s.store_scalar(750, 0.0);s.store_scalar(751, 0.0);s.store_scalar(752, 0.0);s.store_scalar(753, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(754, 0.0);s.store_scalar(757, 0.0);s.store_scalar(682, 0.0);s.store_scalar(688, 0.0);s.store_scalar(689, 0.0);s.store_scalar(787, 0.0);s.store_scalar(794, 0.0);s.store_scalar(788, 0.0);s.store_scalar(690, 0.0);s.store_scalar(692, 0.0);s.store_scalar(691, 0.0);s.store_scalar(693, 0.0);s.store_scalar(795, 0.0);s.store_scalar(676, 0.0);s.store_scalar(681, 0.0);s.store_scalar(678, 0.0);s.store_scalar(686, 0.0);s.store_scalar(687, 0.0);s.store_scalar(694, 0.0);s.store_scalar(679, 0.0);s.store_scalar(683, 0.0);s.store_scalar(680, 0.0);s.store_scalar(677, 0.0);s.store_scalar(684, 0.0);s.store_scalar(685, 0.0);s.store_scalar(956, p[436]);s.store_scalar(959, p[437]);s.store_scalar(986, 0.0);s.store_scalar(987, 0.0);s.store_scalar(988, 0.0);s.store_scalar(961, 0.0);s.store_scalar(960, 0.0);s.store_scalar(427, p[447]);s.store_scalar(957, p[193]);s.store_scalar(977, 0.0);s.store_scalar(978, 0.0);s.store_scalar(421, 40.0);s.store_scalar(828, 0.0);s.store_scalar(829, 0.0);s.store_scalar(830, 0.0);s.store_scalar(831, 0.0);s.store_scalar(862, 0.0);s.store_scalar(861, 0.0);s.store_scalar(870, 0.0);s.store_scalar(869, 0.0);s.store_scalar(66, 0.0);s.store_scalar(65, 0.0);s.store_scalar(68, 0.0);s.store_scalar(67, 0.0);s.store_scalar(832, 0.0);s.store_scalar(833, 0.0);s.store_scalar(834, 0.0);s.store_scalar(835, 0.0);s.store_scalar(838, 0.0);s.store_scalar(839, 0.0);s.store_scalar(841, 0.0);s.store_scalar(842, 0.0);s.store_scalar(843, 0.0);s.store_scalar(844, 0.0);s.store_scalar(845, 0.0);s.store_scalar(846, 0.0);s.store_scalar(840, 0.0);s.store_scalar(857, 0.0);s.store_scalar(858, 0.0);s.store_scalar(859, 0.0);s.store_scalar(860, 0.0);s.store_scalar(865, 0.0);s.store_scalar(866, 0.0);s.store_scalar(867, 0.0);s.store_scalar(868, 0.0);s.store_scalar(849, 0.0);s.store_scalar(854, 0.0);s.store_scalar(847, 0.0);s.store_scalar(852, 0.0);s.store_scalar(851, 0.0);s.store_scalar(856, 0.0);s.store_scalar(848, 0.0);s.store_scalar(853, 0.0);s.store_scalar(850, 0.0);s.store_scalar(855, 0.0);s.store_scalar(946, 0.0);s.store_scalar(944, 0.0);s.store_scalar(947, 0.0);s.store_scalar(945, 0.0);s.store_scalar(948, 0.0);s.store_scalar(816, 0.0);s.store_scalar(815, 0.0);s.store_scalar(873, 0.0);s.store_scalar(874, 0.0);s.store_scalar(875, 0.0);s.store_scalar(876, 0.0);s.store_scalar(877, 0.0);s.store_scalar(878, 0.0);s.store_scalar(879, 0.0);s.store_scalar(880, 0.0);s.store_scalar(881, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(882, 0.0);s.store_scalar(883, 0.0);s.store_scalar(884, 0.0);s.store_scalar(360, 0.0);s.store_scalar(362, 0.0);s.store_scalar(361, 0.0);s.store_scalar(363, 0.0);s.store_scalar(603, 0.0);s.store_scalar(45, 0.0);s.store_scalar(46, 0.0);s.store_scalar(413, 0.0);s.store_scalar(932, 0.0);s.store_scalar(926, 0.0);s.store_scalar(927, 0.0);s.store_scalar(287, 0.0);s.store_scalar(407, 0.0);s.store_scalar(924, 0.0);s.store_scalar(925, 0.0);s.store_scalar(931, 0.0);s.store_scalar(990, 0.0);s.store_scalar(411, 0.0);s.store_scalar(429, 0.0);s.store_scalar(288, 0.0);s.store_scalar(308, 0.0);
        let (t12,) = {
    if (p[40] != 0.0) {
        (0.0,)
    } else {
        (p[17],)
    }
};
        s.store_scalar(448, t12);s.store_scalar(450, p[104]);s.store_scalar(451, p[294]);s.store_scalar(452, p[222]);s.store_scalar(453, p[420]);s.store_scalar(365, 1.0);s.b[1004] = (s.v[452] < 0.0);s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });
        if s.b[1004] {s.store_scalar(452, 0.0);}
        s.b[1005] = (s.v[452] > 0.0);s.store_scalar(1005, if s.b[1005] { 1.0 } else { 0.0 });
        if s.b[1005] {s.store_scalar(452, 0.0);}
        s.b[1007] = (s.v[451] < 0.0);s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
        if s.b[1007] {s.store_scalar(451, 0.0);}
        s.b[1010] = (s.v[453] < 0.0);s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });
        if s.b[1010] {s.store_scalar(453, 0.0);}
        s.b[1011] = (s.v[453] > 1.0);s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
        if s.b[1011] {s.store_scalar(453, 1.0);}
        s.store_scalar(964, p[340]);s.store_scalar(965, p[343]);s.store_scalar(963, p[42]);s.store_scalar(967, p[354]);s.store_scalar(969, p[355]);s.store_scalar(966, p[346]);s.store_scalar(968, p[349]);s.store_scalar(970, p[352]);s.store_scalar(972, p[360]);s.store_scalar(973, p[367]);s.store_scalar(976, p[364]);s.store_scalar(971, p[377]);s.store_scalar(974, p[370]);s.store_scalar(975, p[371]);s.b[1106] = ((s.v[963] < 3.0) && (s.v[963] > 0.0));s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });s.b[1109] = (s.v[964] < 5000000000000000.0);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1109]) {s.store_scalar(964, 5000000000000000.0);}
        s.b[1110] = (s.v[964] > 1e18);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1110]) {s.store_scalar(964, 1e18);}
        s.b[1113] = (s.v[965] < 1e-8);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1113]) {s.store_scalar(965, 1e-8);}
        s.b[1114] = (s.v[965] > 1e-6);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1114]) {s.store_scalar(965, 1e-6);}
        s.b[1117] = (s.v[966] < 1.0);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1117]) {s.store_scalar(966, 1.0);}
        s.b[1118] = (s.v[966] > 100000.0);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1118]) {s.store_scalar(966, 100000.0);}
        s.b[1121] = (s.v[967] < 1.0);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1121]) {s.store_scalar(967, 1.0);}
        s.b[1122] = (s.v[967] > 100000.0);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1122]) {s.store_scalar(967, 100000.0);}
        s.b[1125] = (s.v[971] < 1.0);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1125]) {s.store_scalar(971, 1.0);}
        s.b[1126] = (s.v[971] > 100000.0);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1126]) {s.store_scalar(971, 100000.0);}
        s.b[1129] = (s.v[975] < 0.1);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1129]) {s.store_scalar(975, 0.1);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1130] = (s.v[975] > 4.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1130]) {s.store_scalar(975, 4.0);}
        s.b[1133] = (s.v[972] < 0.0);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1133]) {s.store_scalar(972, 0.0);}
        s.b[1134] = (s.v[972] > 5.0);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });
        if (s.b[1106] && s.b[1134]) {s.store_scalar(972, 5.0);}
        s.b[1135] = (s.v[963] == 3.0);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });s.b[1138] = (s.v[964] < 5000000000000000.0);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1138]) {s.store_scalar(964, 5000000000000000.0);}
        s.b[1139] = (s.v[964] > 1e18);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1139]) {s.store_scalar(964, 1e18);}
        s.b[1142] = (s.v[965] < 1e-8);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1142]) {s.store_scalar(965, 1e-8);}
        s.b[1143] = (s.v[965] > 1e-6);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1143]) {s.store_scalar(965, 1e-6);}
        s.b[1146] = (s.v[966] < 1.0);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1146]) {s.store_scalar(966, 1.0);}
        s.b[1147] = (s.v[966] > 10000000000.0);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1147]) {s.store_scalar(966, 10000000000.0);}
        s.b[1150] = (s.v[971] < 100.0);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1150]) {s.store_scalar(971, 100.0);}
        s.b[1151] = (s.v[971] > 2000000000.0);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1151]) {s.store_scalar(971, 2000000000.0);}
        s.b[1154] = (s.v[972] < 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1154]) {s.store_scalar(972, 0.0);}
        s.b[1155] = (s.v[972] > 5.0);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if (((!s.b[1106]) && s.b[1135]) && s.b[1155]) {s.store_scalar(972, 5.0);}
        s.store_scalar(543, p[96]);s.b[1164] = (s.v[543] < p[95]);s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if s.b[1164] {s.store_scalar(543, p[95]);}
        s.b[1165] = (s.v[543] > 5e-7);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if s.b[1165] {s.store_scalar(543, 5e-7);}
        s.store_scalar(545, (p[120] / ((100.0) as f64).powf(p[122])));s.store_scalar(546, (p[123] / ((100.0) as f64).powf(p[129])));s.store_scalar(547, (p[198] / ((100.0) as f64).powf(p[199])));s.store_scalar(548, (p[200] / ((100.0) as f64).powf(p[201])));s.store_scalar(549, (p[183] / ((100.0) as f64).powf(p[184])));s.store_scalar(550, (p[202] / ((100.0) as f64).powf(p[203])));s.store_scalar(551, (p[190] / ((100.0) as f64).powf(p[191])));s.store_scalar(552, (p[186] / 100.0));s.store_scalar(553, (p[192] / 100.0));s.store_scalar(554, (p[73] * 100.0));s.store_scalar(555, (p[311] / 100.0));s.store_scalar(556, (p[312] / 100.0));s.store_scalar(557, (p[313] / 100.0));s.store_scalar(558, (p[314] / 100.0));s.store_scalar(544, (p[336] / 1e-6));s.store_scalar(559, (p[255] * 100.0));s.store_scalar(560, (p[248] * 100.0));s.store_scalar(561, (p[249] * 100.0));s.store_scalar(562, (p[251] / 10000.0));s.store_scalar(563, (p[266] * 10000.0));s.store_scalar(564, (p[275] / 100.0));s.store_scalar(565, (p[272] / 10000.0));s.store_scalar(572, (p[273] / 10000.0));s.store_scalar(566, (p[293] * 100.0));s.store_scalar(567, (p[409] / 10000.0));s.store_scalar(568, (p[412] / 100.0));s.store_scalar(569, (p[413] / 10000.0));s.store_scalar(570, (p[414] / 100.0));s.store_scale(964, 964, 1000000.0);s.store_scalar(489, (p[453] / 1e-6));s.store_scalar(764, (p[274] + 273.15));s.store_scalar(582, (p[0] + p[116]));s.store_scalar(583, ((p[1] / p[7]) + p[117]));s.store_scalar(576, (s.v[582] * 1000000.0));s.store_scalar(580, (s.v[583] * 1000000.0));s.store_scalar(774, ((s.v[576]) as f64).powf(p[553]));s.store_scalar(775, ((s.v[580]) as f64).powf(p[554]));s.store_scalar(776, (s.v[774] * s.v[775]));s.store_scalar(454, (((p[89] + (p[555] / s.v[774])) + (p[643] / s.v[775])) + (p[731] / s.v[776])));s.store_scalar(455, (((p[92] + (p[556] / s.v[774])) + (p[644] / s.v[775])) + (p[732] / s.v[776])));s.store_scalar(456, (((p[93] + (p[557] / s.v[774])) + (p[645] / s.v[775])) + (p[733] / s.v[776])));s.store_scalar(457, (((p[94] + (p[558] / s.v[774])) + (p[646] / s.v[775])) + (p[734] / s.v[776])));s.store_scalar(458, (((p[110] + (p[559] / s.v[774])) + (p[647] / s.v[775])) + (p[735] / s.v[776])));s.store_scalar(459, (((p[111] + (p[560] / s.v[774])) + (p[648] / s.v[775])) + (p[736] / s.v[776])));s.store_scalar(460, (((p[112] + (p[561] / s.v[774])) + (p[649] / s.v[775])) + (p[737] / s.v[776])));s.store_scalar(461, (((p[126] + (p[562] / s.v[774])) + (p[650] / s.v[775])) + (p[738] / s.v[776])));s.store_scalar(462, (((p[136] + (p[563] / s.v[774])) + (p[651] / s.v[775])) + (p[739] / s.v[776])));s.store_scalar(463, (((p[138] + (p[564] / s.v[774])) + (p[652] / s.v[775])) + (p[740] / s.v[776])));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(464, (((p[141] + (p[565] / s.v[774])) + (p[653] / s.v[775])) + (p[741] / s.v[776])));s.store_scalar(465, (((p[144] + (p[566] / s.v[774])) + (p[654] / s.v[775])) + (p[742] / s.v[776])));s.store_scalar(466, (((p[145] + (p[567] / s.v[774])) + (p[655] / s.v[775])) + (p[743] / s.v[776])));s.store_scalar(467, (((p[146] + (p[568] / s.v[774])) + (p[656] / s.v[775])) + (p[744] / s.v[776])));s.store_scalar(468, (((p[147] + (p[569] / s.v[774])) + (p[657] / s.v[775])) + (p[745] / s.v[776])));s.store_scalar(469, (((p[148] + (p[570] / s.v[774])) + (p[658] / s.v[775])) + (p[746] / s.v[776])));s.store_scalar(470, (((p[149] + (p[571] / s.v[774])) + (p[659] / s.v[775])) + (p[747] / s.v[776])));s.store_scalar(471, (((p[151] + (p[572] / s.v[774])) + (p[660] / s.v[775])) + (p[748] / s.v[776])));s.store_scalar(472, (((p[154] + (p[573] / s.v[774])) + (p[661] / s.v[775])) + (p[749] / s.v[776])));s.store_scalar(473, (((p[157] + (p[574] / s.v[774])) + (p[662] / s.v[775])) + (p[750] / s.v[776])));s.store_scalar(474, (((p[158] + (p[575] / s.v[774])) + (p[663] / s.v[775])) + (p[751] / s.v[776])));s.store_scalar(475, (((p[159] + (p[576] / s.v[774])) + (p[664] / s.v[775])) + (p[752] / s.v[776])));s.store_scalar(476, (((p[161] + (p[577] / s.v[774])) + (p[665] / s.v[775])) + (p[753] / s.v[776])));s.store_scalar(477, (((p[169] + (p[578] / s.v[774])) + (p[666] / s.v[775])) + (p[754] / s.v[776])));s.store_scalar(478, (((p[170] + (p[579] / s.v[774])) + (p[667] / s.v[775])) + (p[755] / s.v[776])));s.store_scalar(479, (((p[172] + (p[580] / s.v[774])) + (p[668] / s.v[775])) + (p[756] / s.v[776])));s.store_scalar(480, (((p[177] + (p[581] / s.v[774])) + (p[669] / s.v[775])) + (p[757] / s.v[776])));s.store_scalar(481, (((p[179] + (p[582] / s.v[774])) + (p[670] / s.v[775])) + (p[758] / s.v[776])));s.store_scalar(482, (((p[180] + (p[583] / s.v[774])) + (p[671] / s.v[775])) + (p[759] / s.v[776])));s.store_scalar(483, (((p[185] + (p[584] / s.v[774])) + (p[672] / s.v[775])) + (p[760] / s.v[776])));s.store_scalar(484, (((p[182] + (p[585] / s.v[774])) + (p[673] / s.v[775])) + (p[761] / s.v[776])));s.store_scalar(485, (((p[181] + (p[586] / s.v[774])) + (p[674] / s.v[775])) + (p[762] / s.v[776])));s.store_scalar(486, (((p[187] + (p[587] / s.v[774])) + (p[675] / s.v[775])) + (p[763] / s.v[776])));s.store_scalar(487, (((p[188] + (p[588] / s.v[774])) + (p[676] / s.v[775])) + (p[764] / s.v[776])));s.store_scalar(488, (((p[189] + (p[589] / s.v[774])) + (p[677] / s.v[775])) + (p[765] / s.v[776])));s.store_scalar(490, (((p[194] + (p[590] / s.v[774])) + (p[678] / s.v[775])) + (p[766] / s.v[776])));s.store_scalar(491, (((p[195] + (p[591] / s.v[774])) + (p[679] / s.v[775])) + (p[767] / s.v[776])));s.store_scalar(492, (((p[196] + (p[592] / s.v[774])) + (p[680] / s.v[775])) + (p[768] / s.v[776])));s.store_scalar(493, (((p[197] + (p[593] / s.v[774])) + (p[681] / s.v[775])) + (p[769] / s.v[776])));s.store_scalar(494, (((p[204] + (p[594] / s.v[774])) + (p[682] / s.v[775])) + (p[770] / s.v[776])));s.store_scalar(495, (((p[205] + (p[595] / s.v[774])) + (p[683] / s.v[775])) + (p[771] / s.v[776])));s.store_scalar(496, (((p[210] + (p[596] / s.v[774])) + (p[684] / s.v[775])) + (p[772] / s.v[776])));s.store_scalar(497, (((p[211] + (p[597] / s.v[774])) + (p[685] / s.v[775])) + (p[773] / s.v[776])));s.store_scalar(498, (((p[212] + (p[598] / s.v[774])) + (p[686] / s.v[775])) + (p[774] / s.v[776])));s.store_scalar(499, (((p[214] + (p[599] / s.v[774])) + (p[687] / s.v[775])) + (p[775] / s.v[776])));s.store_scalar(500, (((p[215] + (p[600] / s.v[774])) + (p[688] / s.v[775])) + (p[776] / s.v[776])));s.store_scalar(501, (((p[216] + (p[601] / s.v[774])) + (p[689] / s.v[775])) + (p[777] / s.v[776])));s.store_scalar(502, (((p[217] + (p[602] / s.v[774])) + (p[690] / s.v[775])) + (p[778] / s.v[776])));s.store_scalar(503, (((p[218] + (p[603] / s.v[774])) + (p[691] / s.v[775])) + (p[779] / s.v[776])));s.store_scalar(504, (((p[219] + (p[604] / s.v[774])) + (p[692] / s.v[775])) + (p[780] / s.v[776])));
        s.store_scalar(505, (((p[269] + (p[605] / s.v[774])) + (p[693] / s.v[775])) + (p[781] / s.v[776])));s.store_scalar(506, (((p[268] + (p[606] / s.v[774])) + (p[694] / s.v[775])) + (p[782] / s.v[776])));s.store_scalar(507, (((p[226] + (p[607] / s.v[774])) + (p[695] / s.v[775])) + (p[783] / s.v[776])));s.store_scalar(508, (((p[227] + (p[608] / s.v[774])) + (p[696] / s.v[775])) + (p[784] / s.v[776])));s.store_scalar(509, (((p[228] + (p[609] / s.v[774])) + (p[697] / s.v[775])) + (p[785] / s.v[776])));s.store_scalar(510, (((p[232] + (p[610] / s.v[774])) + (p[698] / s.v[775])) + (p[786] / s.v[776])));s.store_scalar(511, (((p[240] + (p[611] / s.v[774])) + (p[699] / s.v[775])) + (p[787] / s.v[776])));s.store_scalar(512, (((p[241] + (p[612] / s.v[774])) + (p[700] / s.v[775])) + (p[788] / s.v[776])));s.store_scalar(513, (((p[245] + (p[613] / s.v[774])) + (p[701] / s.v[775])) + (p[789] / s.v[776])));s.store_scalar(514, (((p[246] + (p[614] / s.v[774])) + (p[702] / s.v[775])) + (p[790] / s.v[776])));s.store_scalar(515, (((p[247] + (p[615] / s.v[774])) + (p[703] / s.v[775])) + (p[791] / s.v[776])));s.store_scalar(516, (((p[250] + (p[616] / s.v[774])) + (p[704] / s.v[775])) + (p[792] / s.v[776])));s.store_scalar(517, (((p[253] + (p[617] / s.v[774])) + (p[705] / s.v[775])) + (p[793] / s.v[776])));s.store_scalar(518, (((p[254] + (p[618] / s.v[774])) + (p[706] / s.v[775])) + (p[794] / s.v[776])));s.store_scalar(519, (((p[256] + (p[619] / s.v[774])) + (p[707] / s.v[775])) + (p[795] / s.v[776])));s.store_scalar(520, (((p[257] + (p[620] / s.v[774])) + (p[708] / s.v[775])) + (p[796] / s.v[776])));s.store_scalar(522, (((p[265] + (p[622] / s.v[774])) + (p[710] / s.v[775])) + (p[798] / s.v[776])));s.store_scalar(523, (((p[278] + (p[623] / s.v[774])) + (p[711] / s.v[775])) + (p[799] / s.v[776])));s.store_scalar(524, (((p[281] + (p[624] / s.v[774])) + (p[712] / s.v[775])) + (p[800] / s.v[776])));s.store_scalar(525, (((p[79] + (p[625] / s.v[774])) + (p[713] / s.v[775])) + (p[801] / s.v[776])));s.store_scalar(526, (((p[86] + (p[626] / s.v[774])) + (p[714] / s.v[775])) + (p[802] / s.v[776])));s.store_scalar(528, (((p[76] + (p[628] / s.v[774])) + (p[716] / s.v[775])) + (p[804] / s.v[776])));s.store_scalar(529, (((p[81] + (p[629] / s.v[774])) + (p[717] / s.v[775])) + (p[805] / s.v[776])));s.store_scalar(530, (((p[74] + (p[630] / s.v[774])) + (p[718] / s.v[775])) + (p[806] / s.v[776])));s.store_scalar(531, (((p[298] + (p[631] / s.v[774])) + (p[719] / s.v[775])) + (p[807] / s.v[776])));s.store_scalar(532, (((p[83] + (p[632] / s.v[774])) + (p[720] / s.v[775])) + (p[808] / s.v[776])));s.store_scalar(533, (((p[84] + (p[633] / s.v[774])) + (p[721] / s.v[775])) + (p[809] / s.v[776])));s.store_scalar(534, (((p[62] + (p[634] / s.v[774])) + (p[722] / s.v[775])) + (p[810] / s.v[776])));s.store_scalar(535, (((p[59] + (p[635] / s.v[774])) + (p[723] / s.v[775])) + (p[811] / s.v[776])));s.store_scalar(536, (((p[60] + (p[636] / s.v[774])) + (p[724] / s.v[775])) + (p[812] / s.v[776])));s.store_scalar(537, (((p[85] + (p[637] / s.v[774])) + (p[725] / s.v[775])) + (p[813] / s.v[776])));s.store_scalar(538, (((p[82] + (p[638] / s.v[774])) + (p[726] / s.v[775])) + (p[814] / s.v[776])));s.store_scalar(539, (((p[61] + (p[639] / s.v[774])) + (p[727] / s.v[775])) + (p[815] / s.v[776])));s.store_scalar(540, (((p[75] + (p[640] / s.v[774])) + (p[728] / s.v[775])) + (p[816] / s.v[776])));s.store_scalar(541, (((p[80] + (p[641] / s.v[774])) + (p[729] / s.v[775])) + (p[817] / s.v[776])));s.store_scalar(542, (((p[77] + (p[642] / s.v[774])) + (p[730] / s.v[775])) + (p[818] / s.v[776])));s.store_scalar(818, (((p[493] + (p[824] / s.v[774])) + (p[839] / s.v[775])) + (p[854] / s.v[776])));s.store_scalar(819, (((p[494] + (p[825] / s.v[774])) + (p[840] / s.v[775])) + (p[855] / s.v[776])));s.store_scalar(820, (((p[496] + (p[826] / s.v[774])) + (p[841] / s.v[775])) + (p[856] / s.v[776])));s.store_scalar(821, (((p[513] + (p[827] / s.v[774])) + (p[842] / s.v[775])) + (p[857] / s.v[776])));s.store_scalar(822, (((p[515] + (p[828] / s.v[774])) + (p[843] / s.v[775])) + (p[858] / s.v[776])));
        s.store_scalar(823, (((p[516] + (p[829] / s.v[774])) + (p[844] / s.v[775])) + (p[859] / s.v[776])));s.store_scalar(824, (((p[517] + (p[830] / s.v[774])) + (p[845] / s.v[775])) + (p[860] / s.v[776])));s.store_scalar(825, (((p[519] + (p[831] / s.v[774])) + (p[846] / s.v[775])) + (p[861] / s.v[776])));s.store_scalar(826, (((p[536] + (p[832] / s.v[774])) + (p[847] / s.v[775])) + (p[862] / s.v[776])));s.store_scalar(827, (((p[538] + (p[833] / s.v[774])) + (p[848] / s.v[775])) + (p[863] / s.v[776])));s.b[1181] = (s.v[963] != 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[342]));s.store_mul_scale_offset_mixed_ia(964, 964, A::div_from_scalar(p[341], s.ad_value(337)), 1.0, 1.0);}
        s.b[1182] = (s.v[964] < 1e21);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1182]) {s.store_scalar(964, 1e21);}
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[369]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1181] {s.store_scaled_offset_ad(973, A::div_from_scalar(p[368], s.ad_value(337)), 1.0, s.v[973]);s.store_scalar(337, ((s.v[576]) as f64).powf(p[362]));s.store_scaled_offset_ad(972, A::div_from_scalar(p[361], s.ad_value(337)), 1.0, p[360]);}
        s.b[1183] = (s.v[972] < 0.0);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1183]) {s.store_scalar(972, 0.0);}
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[348]));s.store_scaled_offset_ad(966, A::div_from_scalar(p[347], s.ad_value(337)), 1.0, p[346]);}
        s.b[1184] = (s.v[966] < 1.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1184]) {s.store_scalar(966, 1.0);}
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[351]));s.store_scaled_offset_ad(968, A::div_from_scalar(p[350], s.ad_value(337)), 1.0, p[349]);}
        s.b[1185] = (s.v[968] < 0.0);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1185]) {s.store_scalar(968, 0.0);}
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[357]));s.store_scaled_offset_ad(967, A::div_from_scalar(p[356], s.ad_value(337)), 1.0, p[354]);}
        s.b[1186] = (s.v[967] < 0.0);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1186]) {s.store_scalar(967, 0.0);}
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[359]));s.store_scaled_offset_ad(969, A::div_from_scalar(p[358], s.ad_value(337)), 1.0, p[355]);}
        s.b[1187] = (s.v[969] < 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1187]) {s.store_scalar(969, 0.0);}
        if s.b[1181] {s.store_scalar(337, ((s.v[576]) as f64).powf(p[373]));s.store_scaled_offset_ad(974, A::div_from_scalar(p[372], s.ad_value(337)), 1.0, s.v[974]);s.store_scalar(337, ((s.v[576]) as f64).powf(p[375]));s.store_mul_scale_offset_mixed_ia(975, 975, A::div_from_scalar(p[374], s.ad_value(337)), 1.0, 1.0);}
        s.b[1188] = (s.v[975] < 0.1);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if (s.b[1181] && s.b[1188]) {s.store_scalar(975, 0.1);}
        if (!s.b[1181]) {s.store_scalar(964, 0.0);s.store_scalar(973, 0.0);s.store_scalar(972, 0.0);s.store_scalar(966, 0.0);s.store_scalar(968, 0.0);s.store_scalar(967, 0.0);s.store_scalar(969, 0.0);s.store_scalar(974, 0.0);s.store_scalar(975, 0.0);}
        s.b[1240] = ((s.v[450] * s.v[451]) > 1.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if s.b[1240] {s.store_primal_div_from_scalar(450, 1.0, 451);}
        s.b[1242] = ((p[40] == 1.0) && (((p[19] > 0.0) && (s.v[459] == 0.0)) || ((p[18] > 0.0) && (s.v[460] == 0.0))));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        let (t0,) = {
    if s.b[1242] {
        (0.0,)
    } else {
        (s.v[449],)
    }
};
        s.store_scalar(449, t0);
        let (t1,) = {
    if (!s.b[1242]) {
        (p[40],)
    } else {
        (s.v[449],)
    }
};
        s.store_scalar(449, t1);s.b[1243] = (s.v[449] == 1.0);s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        let (t3,) = {
    if s.b[1243] {
        let (t2,) = {
            if (p[19] > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (t2,)
    } else {
        (s.v[75],)
    }
};
        s.store_scalar(75, t3);
        let (t5,) = {
    if s.b[1243] {
        let (t4,) = {
            if (p[18] > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (t4,)
    } else {
        (s.v[76],)
    }
};
        s.store_scalar(76, t5);s.b[1244] = ((p[17] == 0.0) || (p[17] == 2.0));s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        let (t6,) = {
    if ((!s.b[1243]) && s.b[1244]) {
        (0.0,)
    } else {
        (s.v[75],)
    }
};
        s.store_scalar(75, t6);
        let (t7,) = {
    if ((!s.b[1243]) && s.b[1244]) {
        (0.0,)
    } else {
        (s.v[76],)
    }
};
        s.store_scalar(76, t7);
        if ((!s.b[1243]) && (!s.b[1244])) {s.store_scalar(335, (((p[130] * p[2]) * p[7]) + (((s.v[530] + s.v[538]) * (((p[67] * s.v[536]) * 1000000.0) + s.v[534])) * (((p[68] * p[100]) * 1000000.0) + p[101]))));}
        let (t9,) = {
    if ((!s.b[1243]) && (!s.b[1244])) {
        let (t8,) = {
            if (s.v[335] > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (t8,)
    } else {
        (s.v[75],)
    }
};
        s.store_scalar(75, t9);
        if ((!s.b[1243]) && (!s.b[1244])) {s.store_scalar(335, (((p[131] * p[3]) * p[7]) + ((s.v[540] * (((p[69] * s.v[536]) * 1000000.0) + s.v[534])) * (((p[70] * p[100]) * 1000000.0) + p[101]))));}
        let (tb,) = {
    if ((!s.b[1243]) && (!s.b[1244])) {
        let (ta,) = {
            if (s.v[335] > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (ta,)
    } else {
        (s.v[76],)
    }
};
        s.store_scalar(76, tb);s.store_scalar(571, (p[12] / 1e-6));s.store_scalar(554, (p[73] * 100.0));s.store_scalar(463, (s.v[463] / 1e-6));s.store_scalar(464, (s.v[464] / 1e-6));s.store_scalar(494, (s.v[494] / 1e-6));s.store_scalar(459, (s.v[459] / 1e-6));s.store_scalar(460, (s.v[460] / 1e-6));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(502, (s.v[502] / 100.0));s.store_scalar(499, (s.v[499] / 100.0));s.store_scalar(454, (s.v[454] / 100.0));s.store_scalar(510, (s.v[510] * 10000.0));s.store_scalar(517, (s.v[517] / 100.0));s.store_scalar(518, (s.v[518] * 100.0));s.store_scalar(514, (s.v[514] * 100.0));s.store_scalar(520, (s.v[520] * 100.0));s.store_scalar(491, (s.v[491] * 100.0));s.store_scalar(511, (s.v[511] / 10.0));s.store_scalar(512, (s.v[512] * 100.0));s.store_scalar(522, (s.v[522] / 100.0));s.store_scalar(528, (s.v[528] / 1e-6));s.store_scalar(531, (s.v[531] / 100.0));s.store_scalar(532, (s.v[532] / 100.0));s.store_scalar(533, (s.v[533] / 100.0));s.store_scalar(538, (s.v[538] / 100.0));s.store_scalar(541, (s.v[541] / 100.0));s.store_scalar(458, (-s.v[458]));s.store_scale(973, 973, 0.01);s.store_scalar(81, p[28]);s.b[82] = ((p[133] != 0.0) || (p[134] != 0.0));s.store_scalar(82, if s.b[82] { 1.0 } else { 0.0 });s.b[1246] = (((p[235] == 0.0) && (p[237] == 0.0)) || (p[236] == 0.0));s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        let (tc,) = {
    if s.b[1246] {
        (0.0,)
    } else {
        (s.v[765],)
    }
};
        s.store_scalar(765, tc);
        let (td,) = {
    if (!s.b[1246]) {
        (1.0,)
    } else {
        (s.v[765],)
    }
};
        s.store_scalar(765, td);s.store_scalar(581, (s.v[580] * s.v[576]));s.store_scalar(777, (p[289] * 1000000.0));s.store_scalar(616, (s.v[457] - (s.v[764] * (9.025e-5 + (s.v[764] * 1e-7)))));s.store_scalar(617, (8.8541878e-12 * p[267]));s.copy_ad(618, 452);s.b[1247] = (s.v[471] == 0.0);s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
        let (te,) = {
    if s.b[1247] {
        (0.0,)
    } else {
        (s.v[615],)
    }
};
        s.store_scalar(615, te);
        if s.b[1247] {s.store_scalar(642, 0.0);}
        let (tf,) = {
    if (!s.b[1247]) {
        (1.0,)
    } else {
        (s.v[615],)
    }
};
        s.store_scalar(615, tf);
        if (!s.b[1247]) {s.store_scalar(642, ((((1.0 + (1.0 / s.v[576]))) as f64).powf(p[153]) * s.v[471]));}
        s.store_scalar(619, (1.0 + (((s.v[576]) as f64).powf(p[229]) * p[230])));s.store_scalar(335, ((1.0 / (p[118] + (0.5 * p[0]))) + (1.0 / (p[119] + (0.5 * p[0])))));s.store_scalar(589, (2.0 / s.v[335]));s.b[1248] = (((p[8] > 0.0) && (p[9] > 0.0)) && ((p[7] == 1.0) || ((p[7] > 1.0) && (p[10] > 0.0))));s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if s.b[1248] {s.store_scalar(335, 0.0);s.store_scalar(721, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (s.b[1248] && (s.v[721] < p[7])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[1248] {s.store_add_scaled_inputs3_mixed_iaa(335, 335, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p[10] + p[0]), (p[8] + (0.5 * p[0])))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p[10] + p[0]), (p[9] + (0.5 * p[0])))), 1.0);s.store_primal_offset(721, 721, 1.0);}
        }
        if s.b[1248] {s.store_div_from_scalar(588, (2.0 * p[7]), 335);}
        if (!s.b[1248]) {s.store_scalar(588, 0.0);}
        s.store_scalar(773, s.v[528]);s.store_scalar(620, s.v[476]);s.store_scalar(621, s.v[464]);s.store_scalar(622, s.v[463]);s.b[1249] = ((p[32] == 1.0) && s.b[623]);s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if s.b[1249] {s.store_scalar(620, (s.v[620] * ((p[282] * (((s.v[571]) as f64).ln() - ((s.v[622]) as f64).ln())) + 1.0)));s.store_scalar(621, ((s.v[621] + s.v[571]) - s.v[622]));s.store_scalar(773, ((s.v[773] + s.v[571]) - s.v[622]));s.store_scalar(622, s.v[571]);}
        s.store_scale(573, 620, ((1.0 + (p[162] / ((s.v[580]) as f64).powf(p[163]))) * ((1.0 + (p[164] / ((s.v[576]) as f64).powf(p[165]))) * (1.0 + (p[167] / ((s.v[581]) as f64).powf(p[168]))))));s.b[1251] = (s.v[588] > 0.0);s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if s.b[1251] {s.store_scalar(335, (1.0 / (1.0 + s.v[500])));s.store_powf_ad(336, A::div_from_scalar(s.v[499], s.ad_value(588)), s.v[501]);s.store_scalar(337, (((s.v[499] / s.v[589])) as f64).powf(s.v[501]));s.store_div_scaled_product_offset_denominator_mixed_iaa(573, 573, A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);}
        s.store_scalar(624, ((p[171] * (1.0 + (p[173] / ((s.v[576]) as f64).powf(p[176])))) * (1.0 + (p[174] / ((s.v[580]) as f64).powf(p[175])))));
        if (s.v[573] < 1e-25) {s.store_scalar(573, 1e-25);}
        if (s.v[624] < 1e-25) {s.store_scalar(624, 1e-25);}
        s.store_scalar(335, ((s.v[576]) as f64).powf(p[156]));s.store_scalar(625, (((s.v[472] * s.v[335]) / (s.v[335] + p[155])) / 1.034943e-10));s.store_scalar(626, (s.v[473] / 1.034943e-10));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(627, ((p[319] * (1.0 + (p[320] / ((s.v[576]) as f64).powf(p[321])))) * (1.0 + (p[322] / ((s.v[580]) as f64).powf(p[323])))));s.store_scalar(335, ((1.0 + (p[386] / ((s.v[576]) as f64).powf(p[387]))) * (1.0 + (p[388] / ((s.v[580]) as f64).powf(p[389])))));s.store_scalar(633, (p[384] * s.v[335]));s.store_scalar(634, (p[385] * s.v[335]));s.store_scalar(574, (p[97] + (s.v[545] / (((s.v[582] + p[121])) as f64).powf(p[122]))));s.store_primal_offset(575, 451, (s.v[545] / (((s.v[582] + p[121])) as f64).powf(p[122])));s.store_scalar(577, (p[114] + (s.v[546] / (((s.v[583] + p[128])) as f64).powf(p[129]))));s.store_scalar(578, (p[295] + (s.v[546] / (((s.v[583] + p[128])) as f64).powf(p[129]))));s.store_scalar(579, (p[115] + (s.v[546] / (((s.v[583] + p[128])) as f64).powf(p[129]))));s.store_primal_sub_from_scalar_ad(162, s.v[582], A::offset(s.ad_value(575), s.v[574]));s.store_scalar(628, (s.v[582] + (p[124] / ((s.v[581]) as f64).powf(p[125]))));s.store_scalar(629, (s.v[461] / ((s.v[581]) as f64).powf(p[127])));s.store_scalar(335, (1.0 + (p[206] / (((s.v[628] * 1000000.0)) as f64).powf(p[207]))));s.store_scalar(336, (1.0 + (p[208] / ((s.v[580]) as f64).powf(p[209]))));s.store_scalar(495, ((s.v[495] * s.v[335]) * s.v[336]));s.store_scalar(163, (s.v[583] - (2.0 * s.v[577])));s.store_scalar(630, (s.v[583] - (2.0 * s.v[578])));s.store_scalar(631, (s.v[583] - (2.0 * s.v[579])));s.store_scalar(632, (s.v[163] * p[7]));s.store_scalar(635, (s.v[631] * p[7]));s.store_scale(584, 621, (1.0 + (p[142] / ((s.v[580]) as f64).powf(p[143]))));s.store_scale(622, 622, (1.0 + (p[233] / ((s.v[580]) as f64).powf(p[234]))));s.store_scale(335, 622, 1e-6);s.store_scale(336, 584, 1e-6);s.b[1259] = (s.v[335] < 1000000000000000.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if s.b[1259] {s.store_scalar(335, 1000000000000000.0);}
        s.store_scale(622, 335, 1000000.0);s.b[1261] = (s.v[336] < 1000000000000000.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if s.b[1261] {s.store_scalar(336, 1000000000000000.0);}
        s.store_scale(584, 336, 1000000.0);s.b[1262] = (s.v[588] > 0.0);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if s.b[1262] {s.store_scalar(335, (1.0 / (1.0 + s.v[503])));s.store_powf_ad(336, A::div_from_scalar(s.v[502], s.ad_value(588)), s.v[504]);s.store_scalar(337, (((s.v[502] / s.v[589])) as f64).powf(s.v[504]));s.store_div_scaled_product_offset_denominator_mixed_iaa(585, 584, A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);}
        if (!s.b[1262]) {s.copy_ad(585, 584);}
        s.b[1263] = ((s.v[582] > p[140]) || (p[140] <= 0.0));s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if s.b[1263] {s.store_add_scaled_inputs(586, 622, ((s.v[582] - p[140]) * 1.0 / (s.v[582])), 585, (p[140] * 1.0 / (s.v[582])));}
        if (!s.b[1263]) {s.store_add_scaled_inputs3_indices(586, 585, 1.0, 585, ((p[140] - s.v[582]) * 1.0 / (p[140])), 622, (-((p[140] - s.v[582]) * 1.0 / (p[140]))));}
        s.store_scalar(337, ((0.5 * s.v[582]) - p[140]));s.store_scalar(781, ((s.v[337] - 1e-9) - 1e-10));s.store_scalar(782, ((4.0 * 1e-9) * 1e-10));
        if (!(s.v[782] > 0.0)) {s.store_scalar(782, (-s.v[782]));}
        s.store_sqrt_offset_input(782, 782, (s.v[781] * s.v[781]));s.store_scaled_offset_ad(334, A::div_from_scalar(s.v[781], s.ad_value(782)), 1.0, 0.5);s.store_offset_scaled(337, 782, 0.5, ((((s.v[781]) * (0.5))) + (1e-9)));s.store_div_from_scalar_offset_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(337)), (1.0 / p[220]));
        if (0.0 >= s.v[335]) {
            s.store_scalar(336, 0.0);
        } else {
            s.copy_ad(336, 335);
        }
        s.store_add_scaled_product_right_sub(586, 586, 1.0, 336, 773, 622, 1.0 / (s.v[582]));s.store_scale(166, 586, 1.6021918e-19);s.store_scale(636, 166, 1.034943e-10);s.store_scale(637, 636, 2.0);s.b[1264] = ((s.v[582] <= (2.0 * p[140])) && (p[140] > 0.0));s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if s.b[1264] {s.store_add_scaled_inputs4_indices(587, 585, 2.0, 585, (-(s.v[582] * 1.0 / (p[140]))), 622, (-(-(s.v[582] * 1.0 / (p[140])))), 622, -1.0);s.store_ln_div(638, 587, 622);}
        if (!s.b[1264]) {s.store_scalar(638, 0.0);}
        s.store_scalar(639, (((((2.0 * 1.6021918e-19) * s.v[494]) * 1.034943e-10)) as f64).sqrt());s.store_scalar(640, (1.0 / (s.v[494] * s.v[494])));s.store_scalar(641, ((1.0 + (s.v[542] / ((s.v[576]) as f64).powf(p[231]))) * (1.0 + (p[238] / ((s.v[581]) as f64).powf(p[239])))));s.store_scaled_ln_scaled_input(158, 586, 1.0 / (1.04e16), (2.0 / 38.68283));s.store_scaled_ln_scaled_input(159, 622, 1.0 / (1.04e16), (2.0 / 38.68283));s.b[1265] = (p[51] == 1.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if s.b[1265] {s.store_scalar(335, (p[5] + (s.v[163] / (3.0 * p[4]))));s.store_scalar(336, (s.v[582] - p[6]));s.store_div_scaled_inputs_indices(643, 335, p[132], 336, (p[4] * p[7]));}
        s.b[1266] = (s.v[643] > 0.001);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if (s.b[1265] && s.b[1266]) {s.store_div_from_scalar(643, s.v[365], 643);}
        if (s.b[1265] && (!s.b[1266])) {s.store_scalar(643, (s.v[365] * 1000.0));}
        if (!s.b[1265]) {s.store_scalar(643, (1.0 / p[444]));}
        s.b[1267] = (p[130] > 0.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if s.b[1267] {s.store_scalar(644, (p[130] * p[2]));s.store_scalar(648, (p[130] * p[3]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1267]) {s.store_scalar(644, 0.0);s.store_scalar(648, 0.0);}
        s.b[1268] = (p[131] > 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if s.b[1268] {s.store_scalar(648, (p[131] * p[3]));}
        if (!s.b[1268]) {s.store_scalar(648, 0.0);}
        s.b[1269] = (s.v[449] == 0.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });s.b[1270] = ((s.v[530] > 0.0) || (s.v[540] > 0.0));s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (s.b[1269] && s.b[1270]) {s.store_scalar(645, (1.0 + (p[309] / ((s.v[581]) as f64).powf(p[310]))));}
        s.b[1271] = (s.v[538] != 0.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {s.store_scalar(341, (1.0 + (p[303] / ((s.v[581]) as f64).powf(p[304]))));s.store_scalar(340, ((-p[301]) * ((s.v[576]) as f64).powf(p[302])));}
        s.b[1272] = (s.v[340] > 60.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (((s.b[1269] && s.b[1270]) && s.b[1271]) && s.b[1272]) {s.store_scalar(340, 60.0);}
        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {s.store_exp(340, 340);s.store_mul(646, 340, 341);}
        if ((s.b[1269] && s.b[1270]) && (!s.b[1271])) {s.store_scalar(646, 0.0);}
        if (s.b[1269] && (!s.b[1270])) {s.store_scalar(645, 0.0);s.store_scalar(646, 0.0);}
        s.b[1273] = (s.v[532] != 0.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if (s.b[1269] && s.b[1273]) {s.store_scalar(336, (1.0 + (p[307] / ((s.v[581]) as f64).powf(p[308]))));s.store_scalar(335, ((-p[305]) * ((s.v[576]) as f64).powf(p[306])));}
        s.b[1274] = (s.v[335] > 60.0);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if ((s.b[1269] && s.b[1273]) && s.b[1274]) {s.store_scalar(335, 60.0);}
        if (s.b[1269] && s.b[1273]) {s.store_exp(335, 335);s.store_scaled_mul(337, 336, 335, s.v[532]);s.store_scaled_add_mixed_ia(647, 337, A::sqrt_square_offset(s.ad_value(337), ((((4.0 * 1e-6) / 100.0) * 1e-6) / 100.0)), 0.5);}
        if (s.b[1269] && (!s.b[1273])) {s.store_scalar(647, 0.0);}
        if s.b[1269] {s.store_scalar(649, 0.0);s.store_scalar(614, 0.0);s.store_scalar(786, 0.0);s.store_scalar(652, 0.0);s.store_scalar(653, 0.0);s.store_scalar(654, 0.0);}
        if (!s.b[1269]) {s.store_primal_sqrt_square_offset(649, 451, (p[419] * p[419]));s.store_scalar(614, ((((p[419] * p[419]) + (p[97] * p[97]))) as f64).sqrt());s.store_scalar(786, (1.0 + (p[424] / ((s.v[580]) as f64).powf(p[425]))));s.store_scalar(652, (1.0 + (p[426] / ((s.v[576]) as f64).powf(p[427]))));s.store_scalar(653, (1.0 + (p[428] / ((s.v[576]) as f64).powf(p[429]))));s.store_scalar(654, 1.0);s.store_scalar(645, 0.0);s.store_scalar(646, 0.0);s.store_scalar(647, 0.0);}
        s.b[1275] = (s.v[459] > 0.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if s.b[1275] {s.store_scalar(650, ((2.0 * 1.034943e-10) / (1.6021918e-19 * s.v[459])));s.store_div_scaled_value_offset_denominator(651, s.ad_value(622), (((2.0 * 1.034943e-10) / 1.6021918e-19) * 1.0 / (s.v[459])), s.ad_value(622), s.v[459], 1.0);}
        if (!s.b[1275]) {s.store_scalar(650, 0.0);s.store_scalar(651, 0.0);}
        s.b[1276] = (p[52] == 1.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });s.b[1277] = (p[56] < 0.001);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1277]) {s.store_scalar(655, (s.v[365] * 1000.0));}
        if (s.b[1276] && (!s.b[1277])) {s.store_scalar(655, (s.v[365] * (p[277] + (1.0 / p[56]))));}
        s.b[1278] = (p[58] < 0.001);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1278]) {s.store_scalar(656, (s.v[365] * 1000.0));}
        if (s.b[1276] && (!s.b[1278])) {s.store_scalar(656, (s.v[365] * (p[277] + (1.0 / p[58]))));}
        s.b[1279] = (p[57] < 0.001);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if (s.b[1276] && s.b[1279]) {s.store_scalar(657, (s.v[365] * 1000.0));}
        if (s.b[1276] && (!s.b[1279])) {s.store_scalar(657, (s.v[365] * (p[277] + (1.0 / p[57]))));}
        if (!s.b[1276]) {s.store_scalar(655, (s.v[365] * 1000.0));s.copy_ad(656, 655);s.copy_ad(657, 655);}
        s.b[1280] = (p[44] == 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if s.b[1280] {s.store_scalar(335, ((p[108] * s.v[576]) + p[109]));}
        s.b[1281] = (s.v[335] < 0.0);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if (s.b[1280] && s.b[1281]) {s.store_scalar(335, 0.0);}
        if s.b[1280] {s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p[107], s.ad_value(335), p[107], 1.0), 1.0);}
        if (!s.b[1280]) {s.store_scalar(335, (p[108] * s.v[576]));}
        s.b[1282] = (s.v[335] < 0.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if ((!s.b[1280]) && s.b[1282]) {s.store_scalar(335, 0.0);}
        if (!s.b[1280]) {s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p[107], s.ad_value(335), p[107], 1.0), ((p[109]) + (1e-25)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.b[1284] = (s.v[658] < 0.1);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if s.b[1284] {s.store_scalar(658, 0.1);}
        if (p[23] != 0.0) {s.store_scalar(336, ((s.v[163]) as f64).powf(p[201]));s.store_div_scaled_value_offset_denominator(659, s.ad_value(336), (s.v[485] * (1.0 + (s.v[547] / ((s.v[582]) as f64).powf(p[199])))), s.ad_value(336), s.v[548], 1.0);s.store_scalar(660, (s.v[484] * (1.0 + (s.v[549] / ((s.v[582]) as f64).powf(p[184])))));s.store_scalar(661, (s.v[552] * (1.0 + (s.v[550] / ((s.v[582]) as f64).powf(p[203])))));s.store_scalar(662, (s.v[481] * (1.0 + (s.v[551] / ((s.v[582]) as f64).powf(p[191])))));s.store_scalar(663, (s.v[482] * (1.0 + (s.v[553] / s.v[582]))));s.copy_ad(668, 662);s.copy_ad(669, 663);s.copy_ad(665, 659);s.copy_ad(666, 660);s.copy_ad(667, 661);}
        if ((p[23] != 0.0) && (p[46] != 0.0)) {s.store_scalar(668, (s.v[486] * (1.0 + (s.v[551] / ((s.v[582]) as f64).powf(p[191])))));s.store_scalar(669, (s.v[487] * (1.0 + (s.v[553] / s.v[582]))));}
        if (p[23] != 0.0) {s.store_scalar(664, (p[72] * (1.0 + (p[102] / ((s.v[576]) as f64).powf(p[103])))));}
        if (p[23] == 0.0) {s.store_scalar(659, 0.0);s.store_scalar(660, 0.0);s.store_scalar(661, 0.0);s.store_scalar(662, 0.0);s.store_scalar(663, 0.0);s.store_scalar(664, 0.0);s.store_scalar(665, 0.0);s.store_scalar(666, 0.0);s.store_scalar(667, 0.0);s.store_scalar(668, 0.0);s.store_scalar(669, 0.0);}
        s.store_scalar(523, (if (s.v[523] != 0.0) { (s.v[523] * (1.0 + (p[279] / ((s.v[576]) as f64).powf(p[280])))) } else { 0.0 }));s.store_scalar(670, (((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[635]) * (((1.0 + (p[225] / p[95]))) as f64).ln()));s.store_scalar(671, (if (p[134] != 0.0) { (((1000000.0 * s.v[635]) * p[134]) / ((s.v[576]) as f64).powf(p[135])) } else { 0.0 }));s.store_scalar(672, (p[283] * ((s.v[576]) as f64).powf((-p[286]))));s.store_scalar(673, (p[290] * ((s.v[576]) as f64).powf((-p[291]))));s.store_scalar(674, (p[287] * (((s.v[576] + s.v[777])) as f64).powf((-p[288]))));s.store_scalar(766, (((s.v[541] / (s.v[365] * s.v[632])) * (1.0 + (p[317] / ((s.v[576]) as f64).powf(p[318])))) * (1.0 + (p[315] / ((s.v[580]) as f64).powf(p[316])))));s.store_scalar(767, (s.v[566] * (s.v[365] * s.v[632])));s.store_scalar(766, (s.v[766] * (1.0 / ((p[7]) as f64).powf(p[327]))));s.store_scalar(675, ((((1.0 / ((p[7]) as f64).powf(p[327])) / (s.v[365] * s.v[632])) * (1.0 + (p[317] / ((s.v[576]) as f64).powf(p[318])))) * (1.0 + (p[315] / ((s.v[580]) as f64).powf(p[316])))));s.b[1285] = ((p[53] == 0.0) || (s.v[541] == 0.0));s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if s.b[1285] {s.store_scalar(686, 0.0);s.store_scalar(687, 0.0);s.store_scalar(387, (ctx_temp + p[11]));s.copy_ad(388, 387);s.store_offset(387, 387, s.v[732]);s.store_offset(389, 388, (-s.v[764]));s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));s.store_offset(391, 387, (-s.v[764]));s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));s.store_scale(676, 387, 1.0 / (s.v[764]));s.store_ln(590, 676);s.store_sub_scaled_inputs_mixed_ai(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 1.0, 392, s.v[456]);s.store_sqrt(677, 393);s.store_div_from_scalar(335, 1.0, 387);s.store_scalar(336, (1.0 / s.v[764]));s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p[260], 336, (-p[260]), A::square(s.ad_value(335)), p[261], A::square(s.ad_value(336)), (-p[261]), (s.v[616] + p[259]));s.store_sqrt(192, 337);s.store_mul(193, 337, 192);s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);s.store_div_from_scalar(155, 1.0, 154);s.store_square(156, 154);s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);s.store_exp_scaled_input(335, 590, s.v[480]);s.store_div(679, 335, 573);}
        s.b[1286] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1286]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p[380]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && s.b[1286]) {s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[379]), p[379]));s.store_div(973, 973, 334);}
        s.b[1288] = (s.v[973] < 1000.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1286]) && s.b[1288]) {s.store_scalar(973, 1000.0);}
        if (s.b[1285] && s.b[1286]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p[381]));s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p[382]);}
        s.b[1289] = (s.v[963] == 3.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div_scaled_product_by_product_indices(210, 394, 394, 1.0, 964, 964, 1.0);s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));s.store_exp_scaled_input(335, 590, p[380]);s.store_div(977, 335, 971);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[379]), p[379]));s.store_div(973, 973, 334);}
        s.b[1291] = (s.v[973] < 1000.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        if (((s.b[1285] && (!s.b[1286])) && s.b[1289]) && s.b[1291]) {s.store_scalar(973, 1000.0);}
        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {s.store_div_mixed_ia(966, 966, A::powf(s.ad_value(676), p[381]));s.store_offset_scaled(976, 676, p[365], (((((-1.0)) * (p[365]))) + (p[364])));}
        if ((s.b[1285] && (!s.b[1286])) && (!s.b[1289])) {s.store_scalar(961, 0.0);s.store_mul_ln_mixed_ia(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));s.store_scalar(977, 0.0);}
        if s.b[1285] {s.store_mul(680, 638, 155);s.store_scale(335, 387, 1.0 / (s.v[764]));s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));}
        s.b[1292] = (p[39] != 2.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1292]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p[90], 1.0), 1.0, s.ad_value(390), p[91]));}
        if (s.b[1285] && (!s.b[1292])) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p[90], 1.0), 1.0, s.ad_value(392), p[91]));}
        s.b[1294] = (p[39] != 2.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1294]) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(389), p[324], 1.0), s.v[627], 390, (p[325] * s.v[627]));s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(389), p[390], 1.0), 1.0, 390, p[391]);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        if (s.b[1285] && (!s.b[1294])) {s.store_add_scaled_inputs_mixed_ai(682, A::scale_offset(s.ad_value(391), p[324], 1.0), s.v[627], 392, (p[325] * s.v[627]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1285] && (!s.b[1294])) {s.store_add_scaled_inputs_mixed_ai(335, A::scale_offset(s.ad_value(391), p[390], 1.0), 1.0, 392, p[391]);s.store_scale(688, 335, s.v[633]);s.store_scale(689, 335, s.v[634]);}
        s.b[1296] = (s.v[682] < 0.0);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1296]) {s.store_scalar(682, 0.0);}
        s.b[1298] = (s.v[688] < 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1298]) {s.store_scalar(688, 0.0);}
        s.b[1300] = (s.v[689] < 0.0);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1300]) {s.store_scalar(689, 0.0);}
        if (s.b[1285] && (p[53] != 0.0)) {s.store_add_scaled_inputs_mixed_ai(766, A::scale_offset(s.ad_value(389), p[328], s.v[541]), s.v[675], 390, (p[329] * s.v[675]));}
        s.b[1302] = (s.v[766] < 0.0001);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (p[53] != 0.0)) && s.b[1302]) {s.store_scalar(766, 0.0001);}
        if s.b[1285] {s.store_add_scaled_inputs_mixed_ai(336, A::scale_offset(s.ad_value(389), p[330], s.v[529]), 1.0, 390, p[331]);s.store_offset(781, 336, (-0.05));s.store_scalar(782, 0.0);}
        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1285] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1285] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));s.store_sqrt_div(684, 335, 586);s.store_sqrt_div(685, 335, 621);}
        s.b[1303] = (s.v[963] == 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
        if (s.b[1285] && s.b[1303]) {s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);s.store_div(335, 394, 586);s.store_square(210, 335);}
        s.b[1304] = (s.v[963] == 0.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });s.b[1305] = (s.v[459] != 0.0);s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1304]) && s.b[1305]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));}
        s.b[1306] = (s.v[460] != 0.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1304]) && s.b[1306]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));}
        s.b[1307] = (s.v[459] != 0.0);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (!s.b[1304])) && s.b[1307]) {s.store_mul_sqrt_mixed_ia(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));}
        s.b[1308] = (s.v[460] != 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if ((s.b[1285] && (!s.b[1304])) && s.b[1308]) {s.store_mul_sqrt_mixed_ia(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));}
        s.b[1309] = (s.v[449] == 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });s.b[1310] = (s.v[530] > 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1309]) && s.b[1310]) {s.store_scale(336, 645, ((((p[67] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[68] * p[100]) * 1000000.0) + p[101])));}
        s.b[1311] = (p[39] == 1.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, 390, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, 392, s.v[556]);s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));}
        if ((s.b[1285] && s.b[1309]) && (!s.b[1310])) {s.store_scalar(690, 0.0);}
        s.b[1312] = (s.v[540] > 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1309]) && s.b[1312]) {s.store_scale(336, 645, ((((p[69] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[70] * p[100]) * 1000000.0) + p[101])));}
        s.b[1313] = (p[39] == 1.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, 390, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, 392, s.v[556]);s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));}
        if ((s.b[1285] && s.b[1309]) && (!s.b[1312])) {s.store_scalar(691, 0.0);}
        s.b[1314] = (s.v[538] > 0.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_scale(338, 646, ((((p[67] * s.v[536]) * 1000000.0) + s.v[534]) * (((p[68] * p[100]) * 1000000.0) + p[101])));s.store_scalar(335, (((1.0 - s.v[535]) * p[63]) * 1000000.0));s.store_scalar(782, ((((p[99] * p[99]) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());s.store_scaled_offset_ad(334, A::div_from_scalar(p[99], s.ad_value(782)), 1.0, 0.5);s.store_scaled_offset(336, 782, p[99], 0.5);}
        s.b[1315] = (s.v[336] < 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1315]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_div_from_scalar(342, (-p[98]), 336);s.store_offset_scaled(337, 342, (p[63] * 1000000.0), ((1.0) + (p[98])));s.store_offset_add_scaled_product_indices(781, 338, (-1.0), 337, 338, 1.0, (-0.01));s.store_scale(782, 338, (4.0 * 0.01));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);s.store_offset_sub_scaled_inputs_indices(781, 338, (p[98] + 1.0), 339, 1.0, (-5e-5));s.store_scale(782, 338, ((p[98] + 1.0) * (4.0 * 5e-5)));}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 338, (p[98] + 1.0), 781, (-0.5), 782, (-0.5));s.store_offset_add_scaled_product_indices(781, 341, 1.0, 335, 338, 1.0, (-5e-5));s.store_scalar(782, 0.0);}
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_scaled_add(336, 781, 782, 0.5);}
        s.b[1316] = ((p[39] == 0.0) || (p[39] == 1.0));s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, 390, s.v[558]);s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));}
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
    }
}
