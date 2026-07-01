#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_c_eox_slot: &mut f64,
        var_cgdo_given_slot: &mut f64,
        var_cgdoe_slot: &mut f64,
        var_cgso_given_slot: &mut f64,
        var_cgsoe_slot: &mut f64,
        var_cox0_slot: &mut f64,
        var_coxb0_slot: &mut f64,
        var_qgso_slot: &mut f64,
        var_qgso_db0_slot: &mut f64,
        var_qgso_db1_slot: &mut f64,
        var_qgso_db10_slot: &mut f64,
        var_qgso_db11_slot: &mut f64,
        var_qgso_db2_slot: &mut f64,
        var_qgso_db3_slot: &mut f64,
        var_qgso_db4_slot: &mut f64,
        var_qgso_db5_slot: &mut f64,
        var_qgso_db6_slot: &mut f64,
        var_qgso_db7_slot: &mut f64,
        var_qgso_db8_slot: &mut f64,
        var_qgso_db9_slot: &mut f64,
        var_qgso_dn0_slot: &mut f64,
        var_qgso_dn1_slot: &mut f64,
        var_qgso_dn10_slot: &mut f64,
        var_qgso_dn11_slot: &mut f64,
        var_qgso_dn12_slot: &mut f64,
        var_qgso_dn13_slot: &mut f64,
        var_qgso_dn14_slot: &mut f64,
        var_qgso_dn15_slot: &mut f64,
        var_qgso_dn16_slot: &mut f64,
        var_qgso_dn17_slot: &mut f64,
        var_qgso_dn2_slot: &mut f64,
        var_qgso_dn3_slot: &mut f64,
        var_qgso_dn4_slot: &mut f64,
        var_qgso_dn5_slot: &mut f64,
        var_qgso_dn6_slot: &mut f64,
        var_qgso_dn7_slot: &mut f64,
        var_qgso_dn8_slot: &mut f64,
        var_qgso_dn9_slot: &mut f64,
        var_tox0_slot: &mut f64,
    ) {
        let mut var_c_eox: f64 = *var_c_eox_slot;
        let mut var_cgdo_given: f64 = *var_cgdo_given_slot;
        let mut var_cgdoe: f64 = *var_cgdoe_slot;
        let mut var_cgso_given: f64 = *var_cgso_given_slot;
        let mut var_cgsoe: f64 = *var_cgsoe_slot;
        let mut var_cox0: f64 = *var_cox0_slot;
        let mut var_coxb0: f64 = *var_coxb0_slot;
        let mut var_qgso: f64 = *var_qgso_slot;
        let mut var_qgso_db0: f64 = *var_qgso_db0_slot;
        let mut var_qgso_db1: f64 = *var_qgso_db1_slot;
        let mut var_qgso_db10: f64 = *var_qgso_db10_slot;
        let mut var_qgso_db11: f64 = *var_qgso_db11_slot;
        let mut var_qgso_db2: f64 = *var_qgso_db2_slot;
        let mut var_qgso_db3: f64 = *var_qgso_db3_slot;
        let mut var_qgso_db4: f64 = *var_qgso_db4_slot;
        let mut var_qgso_db5: f64 = *var_qgso_db5_slot;
        let mut var_qgso_db6: f64 = *var_qgso_db6_slot;
        let mut var_qgso_db7: f64 = *var_qgso_db7_slot;
        let mut var_qgso_db8: f64 = *var_qgso_db8_slot;
        let mut var_qgso_db9: f64 = *var_qgso_db9_slot;
        let mut var_qgso_dn0: f64 = *var_qgso_dn0_slot;
        let mut var_qgso_dn1: f64 = *var_qgso_dn1_slot;
        let mut var_qgso_dn10: f64 = *var_qgso_dn10_slot;
        let mut var_qgso_dn11: f64 = *var_qgso_dn11_slot;
        let mut var_qgso_dn12: f64 = *var_qgso_dn12_slot;
        let mut var_qgso_dn13: f64 = *var_qgso_dn13_slot;
        let mut var_qgso_dn14: f64 = *var_qgso_dn14_slot;
        let mut var_qgso_dn15: f64 = *var_qgso_dn15_slot;
        let mut var_qgso_dn16: f64 = *var_qgso_dn16_slot;
        let mut var_qgso_dn17: f64 = *var_qgso_dn17_slot;
        let mut var_qgso_dn2: f64 = *var_qgso_dn2_slot;
        let mut var_qgso_dn3: f64 = *var_qgso_dn3_slot;
        let mut var_qgso_dn4: f64 = *var_qgso_dn4_slot;
        let mut var_qgso_dn5: f64 = *var_qgso_dn5_slot;
        let mut var_qgso_dn6: f64 = *var_qgso_dn6_slot;
        let mut var_qgso_dn7: f64 = *var_qgso_dn7_slot;
        let mut var_qgso_dn8: f64 = *var_qgso_dn8_slot;
        let mut var_qgso_dn9: f64 = *var_qgso_dn9_slot;
        let mut var_tox0: f64 = *var_tox0_slot;

        s.b[623] = param_given[12];
        s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });

        let assign20_e1396: f64 = if param_given[268] { 1.0 } else { 0.0 };
        var_cgdo_given = assign20_e1396;

        let assign30_e1398: f64 = if param_given[269] { 1.0 } else { 0.0 };
        var_cgso_given = assign30_e1398;

        var_cgdoe = 0.0;

        var_cgsoe = 0.0;

        s.store_scalar(708, 0.0);

        s.store_scalar(4, 0.0);

        s.store_scalar(5, 0.0);

        s.store_scalar(321, 0.0);

        s.store_scalar(78, 0.0);

        s.store_scalar(74, 0.0);

        s.store_scalar(347, 0.0);

        s.store_scalar(697, 0.0);

        s.store_scalar(698, 0.0);

        s.store_scalar(69, 0.8);

        s.store_scalar(70, 0.4);

        s.store_scalar(77, 0.0);

        s.store_scalar(79, 0.0);

        s.store_scalar(80, 0.0);

        s.store_scalar(81, 0.0);

        s.store_scalar(83, 0.0);

        s.store_scalar(84, 0.0);

        s.store_scalar(85, 0.0);

        s.store_scalar(86, 0.0);

        s.store_scalar(87, 0.0);

        s.store_scalar(88, 0.0);

        s.store_scalar(89, 0.0);

        s.store_scalar(90, 0.0);

        s.store_scalar(91, 0.0);

        s.store_scalar(92, 0.0);

        s.store_scalar(93, 0.0);

        s.store_scalar(94, 0.0);

        s.store_scalar(95, 0.0);

        s.store_scalar(96, 0.0);

        s.store_scalar(97, 0.0);

        s.store_scalar(98, 0.0);

        s.store_scalar(99, 0.0);

        s.store_scalar(100, 0.0);

        s.store_scalar(101, 0.0);

        s.store_scalar(102, 0.0);

        s.store_scalar(103, 0.0);

        s.store_scalar(104, 0.0);

        s.store_scalar(105, 0.0);

        s.store_scalar(106, 0.0);

        s.store_scalar(107, 0.0);

        s.store_scalar(108, 0.0);

        s.store_scalar(109, 0.0);

        s.store_scalar(110, 0.0);

        s.store_scalar(111, 0.0);

        s.store_scalar(112, 0.0);

        s.store_scalar(113, 0.0);

        s.store_scalar(114, 0.0);

        s.store_scalar(115, 0.0);

        s.store_scalar(116, 0.0);

        s.store_scalar(415, 0.0);

        s.store_scalar(117, 0.0);

        s.store_scalar(118, 0.0);

        s.store_scalar(119, 0.0);

        s.store_scalar(120, 0.0);

        s.store_scalar(121, 0.0);

        s.store_scalar(122, 0.0);

        s.store_scalar(123, 0.0);

        s.store_scalar(124, 0.0);

        s.store_scalar(125, 0.0);

        s.store_scalar(126, 0.0);

        s.store_scalar(127, 0.0);

        s.store_scalar(128, 0.0);

        s.store_scalar(129, 0.0);

        s.store_scalar(130, 0.0);

        s.store_scalar(20, 0.0);

        s.store_scalar(131, 0.0);

        s.store_scalar(132, 0.0);

        s.store_scalar(133, 0.0);

        s.store_scalar(19, 0.0);

        s.store_scalar(134, 0.0);

        s.store_scalar(135, 0.0);

        s.store_scalar(137, 0.0);

        s.store_scalar(138, 0.0);

        s.store_scalar(139, 0.0);

        s.store_scalar(140, 0.0);

        s.store_scalar(141, 0.0);

        s.store_scalar(142, 0.0);

        s.store_scalar(143, 0.0);

        s.store_scalar(144, 0.0);

        s.store_scalar(145, 0.0);

        s.store_scalar(146, 0.0);

        s.store_scalar(147, 0.0);

        s.store_scalar(148, 0.0);

        s.store_scalar(149, 0.0);

        s.store_scalar(150, 0.0);

        s.store_scalar(151, 0.0);

        s.store_scalar(152, 0.0);

        s.store_scalar(153, 0.0);

        s.store_scalar(154, 0.0);

        s.store_scalar(155, 0.0);

        s.store_scalar(156, 0.0);

        s.store_scalar(157, 0.0);

        s.store_scalar(158, 0.0);

        s.store_scalar(159, 0.0);

        s.store_scalar(160, 0.0);

        var_c_eox = 0.0;

        s.store_scalar(162, 0.0);

        s.store_scalar(163, 0.0);

        s.store_scalar(164, 0.0);

        s.store_scalar(165, 0.0);

        s.store_scalar(166, 0.0);

        s.store_scalar(167, 0.0);

        s.store_scalar(168, 0.0);

        s.store_scalar(169, 0.0);

        s.store_scalar(170, 0.0);

        s.store_scalar(171, 0.0);

        s.store_scalar(172, 0.0);

        s.store_scalar(173, 0.0);

        s.store_scalar(174, 0.0);

        s.store_scalar(175, 0.0);

        s.store_scalar(176, 0.0);

        s.store_scalar(177, 0.0);

        s.store_scalar(178, 0.0);

        s.store_scalar(179, 0.0);

        s.store_scalar(180, 0.0);

        s.store_scalar(181, 0.0);

        s.store_scalar(182, 0.0);

        s.store_scalar(184, 0.0);

        s.store_scalar(185, 0.0);

        s.store_scalar(186, 0.0);

        var_tox0 = 0.0;

        var_cox0 = 0.0;

        var_coxb0 = 0.0;

        s.store_scalar(189, 0.0);

        s.store_scalar(190, 0.0);

        s.store_scalar(191, 0.0);

        s.store_scalar(192, 0.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(194, 0.0);

        s.store_scalar(195, 0.0);

        s.store_scalar(196, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(198, 0.0);

        s.store_scalar(199, 0.0);

        s.store_scalar(200, 0.0);

        s.store_scalar(201, 0.0);

        s.store_scalar(202, 0.0);

        s.store_scalar(203, 0.0);

        s.store_scalar(204, 0.0);

        s.store_scalar(205, 0.0);

        s.store_scalar(206, 0.0);

        s.store_scalar(207, 0.0);

        s.store_scalar(208, 0.0);

        s.store_scalar(209, 0.0);

        s.store_scalar(210, 0.0);

        s.store_scalar(211, 0.0);

        s.store_scalar(212, 0.0);

        s.store_scalar(213, 0.0);

        s.store_scalar(214, 0.0);

        s.store_scalar(215, 0.0);

        s.store_scalar(216, 0.0);

        s.store_scalar(217, 0.0);

        s.store_scalar(218, 0.0);

        s.store_scalar(219, 0.0);

        s.store_scalar(220, 0.0);

        s.store_scalar(221, 0.0);

        s.store_scalar(222, 0.0);

        s.store_scalar(223, 0.0);

        s.store_scalar(224, 0.0);

        s.store_scalar(225, 0.0);

        s.store_scalar(226, 0.0);

        s.store_scalar(227, 0.0);

        s.store_scalar(228, 0.0);

        s.store_scalar(229, 0.0);

        s.store_scalar(230, 0.0);

        s.store_scalar(231, 0.0);

        s.store_scalar(232, 0.0);

        s.store_scalar(233, 0.0);

        s.store_scalar(234, 0.0);

        s.store_scalar(235, 0.0);

        s.store_scalar(236, 0.0);

        s.store_scalar(237, 0.0);

        s.store_scalar(238, 0.0);

        s.store_scalar(239, 0.0);

        s.store_scalar(240, 0.0);

        s.store_scalar(241, 0.0);

        s.store_scalar(242, 0.0);

        s.store_scalar(243, 0.0);

        s.store_scalar(244, 0.0);

        s.store_scalar(245, 0.0);

        s.store_scalar(246, 0.0);

        s.store_scalar(247, 0.5);

        s.store_scalar(248, 0.0);

        s.store_scalar(249, 0.0);

        s.store_scalar(250, 0.0);

        s.store_scalar(251, 0.0);

        s.store_scalar(252, 0.0);

        s.store_scalar(253, 0.0);

        s.store_scalar(254, 0.0);

        s.store_scalar(255, 0.0);

        s.store_scalar(256, 0.0);

        s.store_scalar(258, 0.0);

        s.store_scalar(259, 0.0);

        s.store_scalar(260, 0.0);

        s.store_scalar(261, 0.0);

        s.store_scalar(262, 0.0);

        s.store_scalar(263, 0.0);

        s.store_scalar(264, 0.0);

        s.store_scalar(265, 0.0);

        s.store_scalar(266, 0.0);

        s.store_scalar(267, 0.0);

        s.store_scalar(268, 0.0);

        s.store_scalar(269, 0.0);

        s.store_scalar(270, 0.0);

        s.store_scalar(271, 0.0);

        s.store_scalar(272, 0.0);

        s.store_scalar(273, 0.0);

        s.store_scalar(274, 0.0);

        s.store_scalar(275, 0.0);

        s.store_scalar(276, 0.0);

        s.store_scalar(277, 0.0);

        s.store_scalar(278, 0.0);

        s.store_scalar(279, 0.0);

        s.store_scalar(280, 0.0);

        s.store_scalar(281, 0.0);

        s.store_scalar(282, 0.0);

        s.store_scalar(283, 0.0);

        s.store_scalar(284, 0.0);

        s.store_scalar(285, 0.0);

        s.store_scalar(286, 0.0);

        s.store_scalar(289, 0.0);

        s.store_scalar(290, 0.0);

        s.store_scalar(291, 0.0);

        s.store_scalar(292, 0.0);

        s.store_scalar(293, 0.0);

        s.store_scalar(296, 0.0);

        var_qgso = 0.0;
        var_qgso_dn0 = 0.0;
        var_qgso_dn1 = 0.0;
        var_qgso_dn2 = 0.0;
        var_qgso_dn3 = 0.0;
        var_qgso_dn4 = 0.0;
        var_qgso_dn5 = 0.0;
        var_qgso_dn6 = 0.0;
        var_qgso_dn7 = 0.0;
        var_qgso_dn8 = 0.0;
        var_qgso_dn9 = 0.0;
        var_qgso_dn10 = 0.0;
        var_qgso_dn11 = 0.0;
        var_qgso_dn12 = 0.0;
        var_qgso_dn13 = 0.0;
        var_qgso_dn14 = 0.0;
        var_qgso_dn15 = 0.0;
        var_qgso_dn16 = 0.0;
        var_qgso_dn17 = 0.0;
        var_qgso_db0 = 0.0;
        var_qgso_db1 = 0.0;
        var_qgso_db2 = 0.0;
        var_qgso_db3 = 0.0;
        var_qgso_db4 = 0.0;
        var_qgso_db5 = 0.0;
        var_qgso_db6 = 0.0;
        var_qgso_db7 = 0.0;
        var_qgso_db8 = 0.0;
        var_qgso_db9 = 0.0;
        var_qgso_db10 = 0.0;
        var_qgso_db11 = 0.0;

        *var_c_eox_slot = var_c_eox;
        *var_cgdo_given_slot = var_cgdo_given;
        *var_cgdoe_slot = var_cgdoe;
        *var_cgso_given_slot = var_cgso_given;
        *var_cgsoe_slot = var_cgsoe;
        *var_cox0_slot = var_cox0;
        *var_coxb0_slot = var_coxb0;
        *var_qgso_slot = var_qgso;
        *var_qgso_db0_slot = var_qgso_db0;
        *var_qgso_db1_slot = var_qgso_db1;
        *var_qgso_db10_slot = var_qgso_db10;
        *var_qgso_db11_slot = var_qgso_db11;
        *var_qgso_db2_slot = var_qgso_db2;
        *var_qgso_db3_slot = var_qgso_db3;
        *var_qgso_db4_slot = var_qgso_db4;
        *var_qgso_db5_slot = var_qgso_db5;
        *var_qgso_db6_slot = var_qgso_db6;
        *var_qgso_db7_slot = var_qgso_db7;
        *var_qgso_db8_slot = var_qgso_db8;
        *var_qgso_db9_slot = var_qgso_db9;
        *var_qgso_dn0_slot = var_qgso_dn0;
        *var_qgso_dn1_slot = var_qgso_dn1;
        *var_qgso_dn10_slot = var_qgso_dn10;
        *var_qgso_dn11_slot = var_qgso_dn11;
        *var_qgso_dn12_slot = var_qgso_dn12;
        *var_qgso_dn13_slot = var_qgso_dn13;
        *var_qgso_dn14_slot = var_qgso_dn14;
        *var_qgso_dn15_slot = var_qgso_dn15;
        *var_qgso_dn16_slot = var_qgso_dn16;
        *var_qgso_dn17_slot = var_qgso_dn17;
        *var_qgso_dn2_slot = var_qgso_dn2;
        *var_qgso_dn3_slot = var_qgso_dn3;
        *var_qgso_dn4_slot = var_qgso_dn4;
        *var_qgso_dn5_slot = var_qgso_dn5;
        *var_qgso_dn6_slot = var_qgso_dn6;
        *var_qgso_dn7_slot = var_qgso_dn7;
        *var_qgso_dn8_slot = var_qgso_dn8;
        *var_qgso_dn9_slot = var_qgso_dn9;
        *var_tox0_slot = var_tox0;
    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        var_cfd_slot: &mut f64,
        var_cfs_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_lg_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_qfd_slot: &mut f64,
        var_qfd_db0_slot: &mut f64,
        var_qfd_db1_slot: &mut f64,
        var_qfd_db10_slot: &mut f64,
        var_qfd_db11_slot: &mut f64,
        var_qfd_db2_slot: &mut f64,
        var_qfd_db3_slot: &mut f64,
        var_qfd_db4_slot: &mut f64,
        var_qfd_db5_slot: &mut f64,
        var_qfd_db6_slot: &mut f64,
        var_qfd_db7_slot: &mut f64,
        var_qfd_db8_slot: &mut f64,
        var_qfd_db9_slot: &mut f64,
        var_qfd_dn0_slot: &mut f64,
        var_qfd_dn1_slot: &mut f64,
        var_qfd_dn10_slot: &mut f64,
        var_qfd_dn11_slot: &mut f64,
        var_qfd_dn12_slot: &mut f64,
        var_qfd_dn13_slot: &mut f64,
        var_qfd_dn14_slot: &mut f64,
        var_qfd_dn15_slot: &mut f64,
        var_qfd_dn16_slot: &mut f64,
        var_qfd_dn17_slot: &mut f64,
        var_qfd_dn2_slot: &mut f64,
        var_qfd_dn3_slot: &mut f64,
        var_qfd_dn4_slot: &mut f64,
        var_qfd_dn5_slot: &mut f64,
        var_qfd_dn6_slot: &mut f64,
        var_qfd_dn7_slot: &mut f64,
        var_qfd_dn8_slot: &mut f64,
        var_qfd_dn9_slot: &mut f64,
        var_qfs_slot: &mut f64,
        var_qfs_db0_slot: &mut f64,
        var_qfs_db1_slot: &mut f64,
        var_qfs_db10_slot: &mut f64,
        var_qfs_db11_slot: &mut f64,
        var_qfs_db2_slot: &mut f64,
        var_qfs_db3_slot: &mut f64,
        var_qfs_db4_slot: &mut f64,
        var_qfs_db5_slot: &mut f64,
        var_qfs_db6_slot: &mut f64,
        var_qfs_db7_slot: &mut f64,
        var_qfs_db8_slot: &mut f64,
        var_qfs_db9_slot: &mut f64,
        var_qfs_dn0_slot: &mut f64,
        var_qfs_dn1_slot: &mut f64,
        var_qfs_dn10_slot: &mut f64,
        var_qfs_dn11_slot: &mut f64,
        var_qfs_dn12_slot: &mut f64,
        var_qfs_dn13_slot: &mut f64,
        var_qfs_dn14_slot: &mut f64,
        var_qfs_dn15_slot: &mut f64,
        var_qfs_dn16_slot: &mut f64,
        var_qfs_dn17_slot: &mut f64,
        var_qfs_dn2_slot: &mut f64,
        var_qfs_dn3_slot: &mut f64,
        var_qfs_dn4_slot: &mut f64,
        var_qfs_dn5_slot: &mut f64,
        var_qfs_dn6_slot: &mut f64,
        var_qfs_dn7_slot: &mut f64,
        var_qfs_dn8_slot: &mut f64,
        var_qfs_dn9_slot: &mut f64,
        var_qgdo_slot: &mut f64,
        var_qgdo_db0_slot: &mut f64,
        var_qgdo_db1_slot: &mut f64,
        var_qgdo_db10_slot: &mut f64,
        var_qgdo_db11_slot: &mut f64,
        var_qgdo_db2_slot: &mut f64,
        var_qgdo_db3_slot: &mut f64,
        var_qgdo_db4_slot: &mut f64,
        var_qgdo_db5_slot: &mut f64,
        var_qgdo_db6_slot: &mut f64,
        var_qgdo_db7_slot: &mut f64,
        var_qgdo_db8_slot: &mut f64,
        var_qgdo_db9_slot: &mut f64,
        var_qgdo_dn0_slot: &mut f64,
        var_qgdo_dn1_slot: &mut f64,
        var_qgdo_dn10_slot: &mut f64,
        var_qgdo_dn11_slot: &mut f64,
        var_qgdo_dn12_slot: &mut f64,
        var_qgdo_dn13_slot: &mut f64,
        var_qgdo_dn14_slot: &mut f64,
        var_qgdo_dn15_slot: &mut f64,
        var_qgdo_dn16_slot: &mut f64,
        var_qgdo_dn17_slot: &mut f64,
        var_qgdo_dn2_slot: &mut f64,
        var_qgdo_dn3_slot: &mut f64,
        var_qgdo_dn4_slot: &mut f64,
        var_qgdo_dn5_slot: &mut f64,
        var_qgdo_dn6_slot: &mut f64,
        var_qgdo_dn7_slot: &mut f64,
        var_qgdo_dn8_slot: &mut f64,
        var_qgdo_dn9_slot: &mut f64,
        var_vdsei_slot: &mut f64,
        var_vdsei_db0_slot: &mut f64,
        var_vdsei_db1_slot: &mut f64,
        var_vdsei_db10_slot: &mut f64,
        var_vdsei_db11_slot: &mut f64,
        var_vdsei_db2_slot: &mut f64,
        var_vdsei_db3_slot: &mut f64,
        var_vdsei_db4_slot: &mut f64,
        var_vdsei_db5_slot: &mut f64,
        var_vdsei_db6_slot: &mut f64,
        var_vdsei_db7_slot: &mut f64,
        var_vdsei_db8_slot: &mut f64,
        var_vdsei_db9_slot: &mut f64,
        var_vdsei_dn0_slot: &mut f64,
        var_vdsei_dn1_slot: &mut f64,
        var_vdsei_dn10_slot: &mut f64,
        var_vdsei_dn11_slot: &mut f64,
        var_vdsei_dn12_slot: &mut f64,
        var_vdsei_dn13_slot: &mut f64,
        var_vdsei_dn14_slot: &mut f64,
        var_vdsei_dn15_slot: &mut f64,
        var_vdsei_dn16_slot: &mut f64,
        var_vdsei_dn17_slot: &mut f64,
        var_vdsei_dn2_slot: &mut f64,
        var_vdsei_dn3_slot: &mut f64,
        var_vdsei_dn4_slot: &mut f64,
        var_vdsei_dn5_slot: &mut f64,
        var_vdsei_dn6_slot: &mut f64,
        var_vdsei_dn7_slot: &mut f64,
        var_vdsei_dn8_slot: &mut f64,
        var_vdsei_dn9_slot: &mut f64,
        var_vgsei_slot: &mut f64,
        var_vgsei_db0_slot: &mut f64,
        var_vgsei_db1_slot: &mut f64,
        var_vgsei_db10_slot: &mut f64,
        var_vgsei_db11_slot: &mut f64,
        var_vgsei_db2_slot: &mut f64,
        var_vgsei_db3_slot: &mut f64,
        var_vgsei_db4_slot: &mut f64,
        var_vgsei_db5_slot: &mut f64,
        var_vgsei_db6_slot: &mut f64,
        var_vgsei_db7_slot: &mut f64,
        var_vgsei_db8_slot: &mut f64,
        var_vgsei_db9_slot: &mut f64,
        var_vgsei_dn0_slot: &mut f64,
        var_vgsei_dn1_slot: &mut f64,
        var_vgsei_dn10_slot: &mut f64,
        var_vgsei_dn11_slot: &mut f64,
        var_vgsei_dn12_slot: &mut f64,
        var_vgsei_dn13_slot: &mut f64,
        var_vgsei_dn14_slot: &mut f64,
        var_vgsei_dn15_slot: &mut f64,
        var_vgsei_dn16_slot: &mut f64,
        var_vgsei_dn17_slot: &mut f64,
        var_vgsei_dn2_slot: &mut f64,
        var_vgsei_dn3_slot: &mut f64,
        var_vgsei_dn4_slot: &mut f64,
        var_vgsei_dn5_slot: &mut f64,
        var_vgsei_dn6_slot: &mut f64,
        var_vgsei_dn7_slot: &mut f64,
        var_vgsei_dn8_slot: &mut f64,
        var_vgsei_dn9_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wgate_slot: &mut f64,
    ) {
        let mut var_cfd: f64 = *var_cfd_slot;
        let mut var_cfs: f64 = *var_cfs_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_lg: f64 = *var_lg_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_qfd: f64 = *var_qfd_slot;
        let mut var_qfd_db0: f64 = *var_qfd_db0_slot;
        let mut var_qfd_db1: f64 = *var_qfd_db1_slot;
        let mut var_qfd_db10: f64 = *var_qfd_db10_slot;
        let mut var_qfd_db11: f64 = *var_qfd_db11_slot;
        let mut var_qfd_db2: f64 = *var_qfd_db2_slot;
        let mut var_qfd_db3: f64 = *var_qfd_db3_slot;
        let mut var_qfd_db4: f64 = *var_qfd_db4_slot;
        let mut var_qfd_db5: f64 = *var_qfd_db5_slot;
        let mut var_qfd_db6: f64 = *var_qfd_db6_slot;
        let mut var_qfd_db7: f64 = *var_qfd_db7_slot;
        let mut var_qfd_db8: f64 = *var_qfd_db8_slot;
        let mut var_qfd_db9: f64 = *var_qfd_db9_slot;
        let mut var_qfd_dn0: f64 = *var_qfd_dn0_slot;
        let mut var_qfd_dn1: f64 = *var_qfd_dn1_slot;
        let mut var_qfd_dn10: f64 = *var_qfd_dn10_slot;
        let mut var_qfd_dn11: f64 = *var_qfd_dn11_slot;
        let mut var_qfd_dn12: f64 = *var_qfd_dn12_slot;
        let mut var_qfd_dn13: f64 = *var_qfd_dn13_slot;
        let mut var_qfd_dn14: f64 = *var_qfd_dn14_slot;
        let mut var_qfd_dn15: f64 = *var_qfd_dn15_slot;
        let mut var_qfd_dn16: f64 = *var_qfd_dn16_slot;
        let mut var_qfd_dn17: f64 = *var_qfd_dn17_slot;
        let mut var_qfd_dn2: f64 = *var_qfd_dn2_slot;
        let mut var_qfd_dn3: f64 = *var_qfd_dn3_slot;
        let mut var_qfd_dn4: f64 = *var_qfd_dn4_slot;
        let mut var_qfd_dn5: f64 = *var_qfd_dn5_slot;
        let mut var_qfd_dn6: f64 = *var_qfd_dn6_slot;
        let mut var_qfd_dn7: f64 = *var_qfd_dn7_slot;
        let mut var_qfd_dn8: f64 = *var_qfd_dn8_slot;
        let mut var_qfd_dn9: f64 = *var_qfd_dn9_slot;
        let mut var_qfs: f64 = *var_qfs_slot;
        let mut var_qfs_db0: f64 = *var_qfs_db0_slot;
        let mut var_qfs_db1: f64 = *var_qfs_db1_slot;
        let mut var_qfs_db10: f64 = *var_qfs_db10_slot;
        let mut var_qfs_db11: f64 = *var_qfs_db11_slot;
        let mut var_qfs_db2: f64 = *var_qfs_db2_slot;
        let mut var_qfs_db3: f64 = *var_qfs_db3_slot;
        let mut var_qfs_db4: f64 = *var_qfs_db4_slot;
        let mut var_qfs_db5: f64 = *var_qfs_db5_slot;
        let mut var_qfs_db6: f64 = *var_qfs_db6_slot;
        let mut var_qfs_db7: f64 = *var_qfs_db7_slot;
        let mut var_qfs_db8: f64 = *var_qfs_db8_slot;
        let mut var_qfs_db9: f64 = *var_qfs_db9_slot;
        let mut var_qfs_dn0: f64 = *var_qfs_dn0_slot;
        let mut var_qfs_dn1: f64 = *var_qfs_dn1_slot;
        let mut var_qfs_dn10: f64 = *var_qfs_dn10_slot;
        let mut var_qfs_dn11: f64 = *var_qfs_dn11_slot;
        let mut var_qfs_dn12: f64 = *var_qfs_dn12_slot;
        let mut var_qfs_dn13: f64 = *var_qfs_dn13_slot;
        let mut var_qfs_dn14: f64 = *var_qfs_dn14_slot;
        let mut var_qfs_dn15: f64 = *var_qfs_dn15_slot;
        let mut var_qfs_dn16: f64 = *var_qfs_dn16_slot;
        let mut var_qfs_dn17: f64 = *var_qfs_dn17_slot;
        let mut var_qfs_dn2: f64 = *var_qfs_dn2_slot;
        let mut var_qfs_dn3: f64 = *var_qfs_dn3_slot;
        let mut var_qfs_dn4: f64 = *var_qfs_dn4_slot;
        let mut var_qfs_dn5: f64 = *var_qfs_dn5_slot;
        let mut var_qfs_dn6: f64 = *var_qfs_dn6_slot;
        let mut var_qfs_dn7: f64 = *var_qfs_dn7_slot;
        let mut var_qfs_dn8: f64 = *var_qfs_dn8_slot;
        let mut var_qfs_dn9: f64 = *var_qfs_dn9_slot;
        let mut var_qgdo: f64 = *var_qgdo_slot;
        let mut var_qgdo_db0: f64 = *var_qgdo_db0_slot;
        let mut var_qgdo_db1: f64 = *var_qgdo_db1_slot;
        let mut var_qgdo_db10: f64 = *var_qgdo_db10_slot;
        let mut var_qgdo_db11: f64 = *var_qgdo_db11_slot;
        let mut var_qgdo_db2: f64 = *var_qgdo_db2_slot;
        let mut var_qgdo_db3: f64 = *var_qgdo_db3_slot;
        let mut var_qgdo_db4: f64 = *var_qgdo_db4_slot;
        let mut var_qgdo_db5: f64 = *var_qgdo_db5_slot;
        let mut var_qgdo_db6: f64 = *var_qgdo_db6_slot;
        let mut var_qgdo_db7: f64 = *var_qgdo_db7_slot;
        let mut var_qgdo_db8: f64 = *var_qgdo_db8_slot;
        let mut var_qgdo_db9: f64 = *var_qgdo_db9_slot;
        let mut var_qgdo_dn0: f64 = *var_qgdo_dn0_slot;
        let mut var_qgdo_dn1: f64 = *var_qgdo_dn1_slot;
        let mut var_qgdo_dn10: f64 = *var_qgdo_dn10_slot;
        let mut var_qgdo_dn11: f64 = *var_qgdo_dn11_slot;
        let mut var_qgdo_dn12: f64 = *var_qgdo_dn12_slot;
        let mut var_qgdo_dn13: f64 = *var_qgdo_dn13_slot;
        let mut var_qgdo_dn14: f64 = *var_qgdo_dn14_slot;
        let mut var_qgdo_dn15: f64 = *var_qgdo_dn15_slot;
        let mut var_qgdo_dn16: f64 = *var_qgdo_dn16_slot;
        let mut var_qgdo_dn17: f64 = *var_qgdo_dn17_slot;
        let mut var_qgdo_dn2: f64 = *var_qgdo_dn2_slot;
        let mut var_qgdo_dn3: f64 = *var_qgdo_dn3_slot;
        let mut var_qgdo_dn4: f64 = *var_qgdo_dn4_slot;
        let mut var_qgdo_dn5: f64 = *var_qgdo_dn5_slot;
        let mut var_qgdo_dn6: f64 = *var_qgdo_dn6_slot;
        let mut var_qgdo_dn7: f64 = *var_qgdo_dn7_slot;
        let mut var_qgdo_dn8: f64 = *var_qgdo_dn8_slot;
        let mut var_qgdo_dn9: f64 = *var_qgdo_dn9_slot;
        let mut var_vdsei: f64 = *var_vdsei_slot;
        let mut var_vdsei_db0: f64 = *var_vdsei_db0_slot;
        let mut var_vdsei_db1: f64 = *var_vdsei_db1_slot;
        let mut var_vdsei_db10: f64 = *var_vdsei_db10_slot;
        let mut var_vdsei_db11: f64 = *var_vdsei_db11_slot;
        let mut var_vdsei_db2: f64 = *var_vdsei_db2_slot;
        let mut var_vdsei_db3: f64 = *var_vdsei_db3_slot;
        let mut var_vdsei_db4: f64 = *var_vdsei_db4_slot;
        let mut var_vdsei_db5: f64 = *var_vdsei_db5_slot;
        let mut var_vdsei_db6: f64 = *var_vdsei_db6_slot;
        let mut var_vdsei_db7: f64 = *var_vdsei_db7_slot;
        let mut var_vdsei_db8: f64 = *var_vdsei_db8_slot;
        let mut var_vdsei_db9: f64 = *var_vdsei_db9_slot;
        let mut var_vdsei_dn0: f64 = *var_vdsei_dn0_slot;
        let mut var_vdsei_dn1: f64 = *var_vdsei_dn1_slot;
        let mut var_vdsei_dn10: f64 = *var_vdsei_dn10_slot;
        let mut var_vdsei_dn11: f64 = *var_vdsei_dn11_slot;
        let mut var_vdsei_dn12: f64 = *var_vdsei_dn12_slot;
        let mut var_vdsei_dn13: f64 = *var_vdsei_dn13_slot;
        let mut var_vdsei_dn14: f64 = *var_vdsei_dn14_slot;
        let mut var_vdsei_dn15: f64 = *var_vdsei_dn15_slot;
        let mut var_vdsei_dn16: f64 = *var_vdsei_dn16_slot;
        let mut var_vdsei_dn17: f64 = *var_vdsei_dn17_slot;
        let mut var_vdsei_dn2: f64 = *var_vdsei_dn2_slot;
        let mut var_vdsei_dn3: f64 = *var_vdsei_dn3_slot;
        let mut var_vdsei_dn4: f64 = *var_vdsei_dn4_slot;
        let mut var_vdsei_dn5: f64 = *var_vdsei_dn5_slot;
        let mut var_vdsei_dn6: f64 = *var_vdsei_dn6_slot;
        let mut var_vdsei_dn7: f64 = *var_vdsei_dn7_slot;
        let mut var_vdsei_dn8: f64 = *var_vdsei_dn8_slot;
        let mut var_vdsei_dn9: f64 = *var_vdsei_dn9_slot;
        let mut var_vgsei: f64 = *var_vgsei_slot;
        let mut var_vgsei_db0: f64 = *var_vgsei_db0_slot;
        let mut var_vgsei_db1: f64 = *var_vgsei_db1_slot;
        let mut var_vgsei_db10: f64 = *var_vgsei_db10_slot;
        let mut var_vgsei_db11: f64 = *var_vgsei_db11_slot;
        let mut var_vgsei_db2: f64 = *var_vgsei_db2_slot;
        let mut var_vgsei_db3: f64 = *var_vgsei_db3_slot;
        let mut var_vgsei_db4: f64 = *var_vgsei_db4_slot;
        let mut var_vgsei_db5: f64 = *var_vgsei_db5_slot;
        let mut var_vgsei_db6: f64 = *var_vgsei_db6_slot;
        let mut var_vgsei_db7: f64 = *var_vgsei_db7_slot;
        let mut var_vgsei_db8: f64 = *var_vgsei_db8_slot;
        let mut var_vgsei_db9: f64 = *var_vgsei_db9_slot;
        let mut var_vgsei_dn0: f64 = *var_vgsei_dn0_slot;
        let mut var_vgsei_dn1: f64 = *var_vgsei_dn1_slot;
        let mut var_vgsei_dn10: f64 = *var_vgsei_dn10_slot;
        let mut var_vgsei_dn11: f64 = *var_vgsei_dn11_slot;
        let mut var_vgsei_dn12: f64 = *var_vgsei_dn12_slot;
        let mut var_vgsei_dn13: f64 = *var_vgsei_dn13_slot;
        let mut var_vgsei_dn14: f64 = *var_vgsei_dn14_slot;
        let mut var_vgsei_dn15: f64 = *var_vgsei_dn15_slot;
        let mut var_vgsei_dn16: f64 = *var_vgsei_dn16_slot;
        let mut var_vgsei_dn17: f64 = *var_vgsei_dn17_slot;
        let mut var_vgsei_dn2: f64 = *var_vgsei_dn2_slot;
        let mut var_vgsei_dn3: f64 = *var_vgsei_dn3_slot;
        let mut var_vgsei_dn4: f64 = *var_vgsei_dn4_slot;
        let mut var_vgsei_dn5: f64 = *var_vgsei_dn5_slot;
        let mut var_vgsei_dn6: f64 = *var_vgsei_dn6_slot;
        let mut var_vgsei_dn7: f64 = *var_vgsei_dn7_slot;
        let mut var_vgsei_dn8: f64 = *var_vgsei_dn8_slot;
        let mut var_vgsei_dn9: f64 = *var_vgsei_dn9_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wgate: f64 = *var_wgate_slot;

        var_qgdo = 0.0;
        var_qgdo_dn0 = 0.0;
        var_qgdo_dn1 = 0.0;
        var_qgdo_dn2 = 0.0;
        var_qgdo_dn3 = 0.0;
        var_qgdo_dn4 = 0.0;
        var_qgdo_dn5 = 0.0;
        var_qgdo_dn6 = 0.0;
        var_qgdo_dn7 = 0.0;
        var_qgdo_dn8 = 0.0;
        var_qgdo_dn9 = 0.0;
        var_qgdo_dn10 = 0.0;
        var_qgdo_dn11 = 0.0;
        var_qgdo_dn12 = 0.0;
        var_qgdo_dn13 = 0.0;
        var_qgdo_dn14 = 0.0;
        var_qgdo_dn15 = 0.0;
        var_qgdo_dn16 = 0.0;
        var_qgdo_dn17 = 0.0;
        var_qgdo_db0 = 0.0;
        var_qgdo_db1 = 0.0;
        var_qgdo_db2 = 0.0;
        var_qgdo_db3 = 0.0;
        var_qgdo_db4 = 0.0;
        var_qgdo_db5 = 0.0;
        var_qgdo_db6 = 0.0;
        var_qgdo_db7 = 0.0;
        var_qgdo_db8 = 0.0;
        var_qgdo_db9 = 0.0;
        var_qgdo_db10 = 0.0;
        var_qgdo_db11 = 0.0;

        var_qfd = 0.0;
        var_qfd_dn0 = 0.0;
        var_qfd_dn1 = 0.0;
        var_qfd_dn2 = 0.0;
        var_qfd_dn3 = 0.0;
        var_qfd_dn4 = 0.0;
        var_qfd_dn5 = 0.0;
        var_qfd_dn6 = 0.0;
        var_qfd_dn7 = 0.0;
        var_qfd_dn8 = 0.0;
        var_qfd_dn9 = 0.0;
        var_qfd_dn10 = 0.0;
        var_qfd_dn11 = 0.0;
        var_qfd_dn12 = 0.0;
        var_qfd_dn13 = 0.0;
        var_qfd_dn14 = 0.0;
        var_qfd_dn15 = 0.0;
        var_qfd_dn16 = 0.0;
        var_qfd_dn17 = 0.0;
        var_qfd_db0 = 0.0;
        var_qfd_db1 = 0.0;
        var_qfd_db2 = 0.0;
        var_qfd_db3 = 0.0;
        var_qfd_db4 = 0.0;
        var_qfd_db5 = 0.0;
        var_qfd_db6 = 0.0;
        var_qfd_db7 = 0.0;
        var_qfd_db8 = 0.0;
        var_qfd_db9 = 0.0;
        var_qfd_db10 = 0.0;
        var_qfd_db11 = 0.0;

        var_cfd = 0.0;

        var_qfs = 0.0;
        var_qfs_dn0 = 0.0;
        var_qfs_dn1 = 0.0;
        var_qfs_dn2 = 0.0;
        var_qfs_dn3 = 0.0;
        var_qfs_dn4 = 0.0;
        var_qfs_dn5 = 0.0;
        var_qfs_dn6 = 0.0;
        var_qfs_dn7 = 0.0;
        var_qfs_dn8 = 0.0;
        var_qfs_dn9 = 0.0;
        var_qfs_dn10 = 0.0;
        var_qfs_dn11 = 0.0;
        var_qfs_dn12 = 0.0;
        var_qfs_dn13 = 0.0;
        var_qfs_dn14 = 0.0;
        var_qfs_dn15 = 0.0;
        var_qfs_dn16 = 0.0;
        var_qfs_dn17 = 0.0;
        var_qfs_db0 = 0.0;
        var_qfs_db1 = 0.0;
        var_qfs_db2 = 0.0;
        var_qfs_db3 = 0.0;
        var_qfs_db4 = 0.0;
        var_qfs_db5 = 0.0;
        var_qfs_db6 = 0.0;
        var_qfs_db7 = 0.0;
        var_qfs_db8 = 0.0;
        var_qfs_db9 = 0.0;
        var_qfs_db10 = 0.0;
        var_qfs_db11 = 0.0;

        var_cfs = 0.0;

        s.store_scalar(303, 0.0);

        s.store_scalar(304, 0.0);

        s.store_scalar(305, 0.0);

        s.store_scalar(306, 0.0);

        s.store_scalar(307, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(314, 0.0);

        s.store_scalar(315, 0.0);

        s.store_scalar(316, 0.0);

        s.store_scalar(317, 0.0);

        s.store_scalar(318, 0.0);

        s.store_scalar(319, 0.0);

        s.store_scalar(320, 0.0);

        s.store_scalar(322, 0.0);

        s.store_scalar(323, 0.0);

        s.store_scalar(324, 0.0);

        s.store_scalar(325, 0.0);

        s.store_scalar(326, 0.0);

        s.store_scalar(327, 0.0);

        s.store_scalar(328, 0.0);

        s.store_scalar(329, 0.0);

        s.store_scalar(330, 0.0);

        s.store_scalar(331, 0.0);

        s.store_scalar(332, 0.0);

        s.store_scalar(333, 0.0);

        s.store_scalar(334, 0.0);

        s.store_scalar(335, 0.0);

        s.store_scalar(336, 0.0);

        s.store_scalar(337, 0.0);

        s.store_scalar(338, 0.0);

        s.store_scalar(339, 0.0);

        s.store_scalar(340, 0.0);

        s.store_scalar(341, 0.0);

        s.store_scalar(342, 0.0);

        s.store_scalar(343, 0.0);

        s.store_scalar(344, 0.0);

        s.store_scalar(345, 0.0);

        s.store_scalar(346, 0.0);

        s.store_scalar(348, 0.0);

        s.store_scalar(349, 0.0);

        s.store_scalar(350, 0.0);

        s.store_scalar(351, 0.0);

        s.store_scalar(352, 0.0);

        s.store_scalar(353, 0.0);

        s.store_scalar(354, 0.0);

        s.store_scalar(355, 0.0);

        s.store_scalar(356, 0.0);

        s.store_scalar(357, 0.0);

        s.store_scalar(358, 0.0);

        s.store_scalar(359, 0.0);

        s.store_scalar(364, 0.0);

        s.store_scalar(366, 0.0);

        s.store_scalar(367, 0.0);

        s.store_scalar(368, 0.0);

        s.store_scalar(369, 0.0);

        s.store_scalar(370, 0.0);

        s.store_scalar(371, 0.0);

        s.store_scalar(372, 0.0);

        s.store_scalar(373, 0.0);

        s.store_scalar(374, 0.0);

        s.store_scalar(375, 0.0);

        s.store_scalar(376, 0.0);

        s.store_scalar(377, 0.0);

        s.store_scalar(380, 0.0);

        s.store_scalar(381, 0.0);

        s.store_scalar(382, 0.0);

        s.store_scalar(383, 0.0);

        s.store_scalar(387, 0.0);

        s.store_scalar(388, 0.0);

        s.store_scalar(389, 0.0);

        s.store_scalar(390, 0.0);

        s.store_scalar(391, 0.0);

        s.store_scalar(392, 0.0);

        s.store_scalar(393, 0.0);

        s.store_scalar(394, 0.0);

        s.store_scalar(395, 0.0);

        s.store_scalar(396, 0.0);

        s.store_scalar(397, 0.0);

        s.store_scalar(398, 0.0);

        s.store_scalar(399, 0.0);

        s.store_scalar(400, 0.0);

        s.store_scalar(402, 0.0);

        s.store_scalar(403, 0.0);

        s.store_scalar(404, 0.0);

        s.store_scalar(405, 0.0);

        s.store_scalar(385, p.p334);

        s.store_scalar(386, p.p334);

        s.store_scalar(409, 0.0);

        s.store_scalar(410, 0.0);

        s.store_scalar(434, 0.0093868);

        s.store_scalar(435, (-0.1047839));

        s.store_scalar(447, 0.0);

        s.store_scalar(573, 0.0);

        s.store_scalar(574, 0.0);

        s.store_scalar(575, 0.0);

        var_lg = 0.0;

        s.store_scalar(577, 0.0);

        s.store_scalar(578, 0.0);

        var_dwcv = 0.0;

        var_wg = 0.0;

        s.store_scalar(581, 0.0);

        var_lgate = 0.0;

        var_wgate = 0.0;

        s.store_scalar(584, 0.0);

        s.store_scalar(585, 0.0);

        s.store_scalar(586, 0.0);

        s.store_scalar(587, 0.0);

        s.store_scalar(588, 0.0);

        s.store_scalar(589, 0.0);

        s.store_scalar(590, 0.0);

        s.store_scalar(591, 0.0);

        s.store_scalar(592, 0.0);

        s.store_scalar(593, 0.0);

        s.store_scalar(594, 0.0);

        s.store_scalar(595, 0.0);

        s.store_scalar(596, 0.0);

        s.store_scalar(597, 0.0);

        s.store_scalar(739, 0.0);

        s.store_scalar(598, 0.0);

        s.store_scalar(770, 0.0);

        s.store_scalar(727, 0.0);

        s.store_scalar(728, 0.0);

        s.store_scalar(729, 0.0);

        s.store_scalar(730, 0.0);

        s.store_scalar(731, 0.0);

        s.store_scalar(732, 0.0);

        var_vdsei = 0.0;
        var_vdsei_dn0 = 0.0;
        var_vdsei_dn1 = 0.0;
        var_vdsei_dn2 = 0.0;
        var_vdsei_dn3 = 0.0;
        var_vdsei_dn4 = 0.0;
        var_vdsei_dn5 = 0.0;
        var_vdsei_dn6 = 0.0;
        var_vdsei_dn7 = 0.0;
        var_vdsei_dn8 = 0.0;
        var_vdsei_dn9 = 0.0;
        var_vdsei_dn10 = 0.0;
        var_vdsei_dn11 = 0.0;
        var_vdsei_dn12 = 0.0;
        var_vdsei_dn13 = 0.0;
        var_vdsei_dn14 = 0.0;
        var_vdsei_dn15 = 0.0;
        var_vdsei_dn16 = 0.0;
        var_vdsei_dn17 = 0.0;
        var_vdsei_db0 = 0.0;
        var_vdsei_db1 = 0.0;
        var_vdsei_db2 = 0.0;
        var_vdsei_db3 = 0.0;
        var_vdsei_db4 = 0.0;
        var_vdsei_db5 = 0.0;
        var_vdsei_db6 = 0.0;
        var_vdsei_db7 = 0.0;
        var_vdsei_db8 = 0.0;
        var_vdsei_db9 = 0.0;
        var_vdsei_db10 = 0.0;
        var_vdsei_db11 = 0.0;

        var_vgsei = 0.0;
        var_vgsei_dn0 = 0.0;
        var_vgsei_dn1 = 0.0;
        var_vgsei_dn2 = 0.0;
        var_vgsei_dn3 = 0.0;
        var_vgsei_dn4 = 0.0;
        var_vgsei_dn5 = 0.0;
        var_vgsei_dn6 = 0.0;
        var_vgsei_dn7 = 0.0;
        var_vgsei_dn8 = 0.0;
        var_vgsei_dn9 = 0.0;
        var_vgsei_dn10 = 0.0;
        var_vgsei_dn11 = 0.0;
        var_vgsei_dn12 = 0.0;
        var_vgsei_dn13 = 0.0;
        var_vgsei_dn14 = 0.0;
        var_vgsei_dn15 = 0.0;
        var_vgsei_dn16 = 0.0;
        var_vgsei_dn17 = 0.0;
        var_vgsei_db0 = 0.0;
        var_vgsei_db1 = 0.0;
        var_vgsei_db2 = 0.0;
        var_vgsei_db3 = 0.0;
        var_vgsei_db4 = 0.0;
        var_vgsei_db5 = 0.0;
        var_vgsei_db6 = 0.0;
        var_vgsei_db7 = 0.0;
        var_vgsei_db8 = 0.0;
        var_vgsei_db9 = 0.0;
        var_vgsei_db10 = 0.0;
        var_vgsei_db11 = 0.0;

        s.store_scalar(735, 0.0);

        s.store_scalar(736, 0.0);

        s.store_scalar(737, 0.0);

        s.store_scalar(738, 0.0);

        s.store_scalar(740, 0.0);

        s.store_scalar(18, 0.0);

        s.store_scalar(741, 0.0);

        s.store_scalar(745, 0.0);

        s.store_scalar(746, 0.0);

        s.store_scalar(747, 0.0);

        s.store_scalar(748, 0.0);

        s.store_scalar(749, 0.0);

        s.store_scalar(750, 0.0);

        s.store_scalar(751, 0.0);

        s.store_scalar(752, 0.0);

        s.store_scalar(753, 0.0);

        s.store_scalar(754, 0.0);

        s.store_scalar(757, 0.0);

        s.store_scalar(682, 0.0);

        s.store_scalar(688, 0.0);

        s.store_scalar(689, 0.0);

        s.store_scalar(787, 0.0);

        s.store_scalar(794, 0.0);

        s.store_scalar(788, 0.0);

        s.store_scalar(690, 0.0);

        s.store_scalar(692, 0.0);

        s.store_scalar(691, 0.0);

        s.store_scalar(693, 0.0);

        s.store_scalar(795, 0.0);

        s.store_scalar(676, 0.0);

        s.store_scalar(681, 0.0);

        s.store_scalar(678, 0.0);

        s.store_scalar(686, 0.0);

        s.store_scalar(687, 0.0);

        s.store_scalar(694, 0.0);

        s.store_scalar(679, 0.0);

        s.store_scalar(683, 0.0);

        s.store_scalar(680, 0.0);

        s.store_scalar(677, 0.0);

        s.store_scalar(684, 0.0);

        s.store_scalar(685, 0.0);

        s.store_scalar(956, p.p436);

        s.store_scalar(959, p.p437);

        s.store_scalar(986, 0.0);

        s.store_scalar(987, 0.0);

        s.store_scalar(988, 0.0);

        s.store_scalar(961, 0.0);

        s.store_scalar(960, 0.0);

        *var_cfd_slot = var_cfd;
        *var_cfs_slot = var_cfs;
        *var_dwcv_slot = var_dwcv;
        *var_lg_slot = var_lg;
        *var_lgate_slot = var_lgate;
        *var_qfd_slot = var_qfd;
        *var_qfd_db0_slot = var_qfd_db0;
        *var_qfd_db1_slot = var_qfd_db1;
        *var_qfd_db10_slot = var_qfd_db10;
        *var_qfd_db11_slot = var_qfd_db11;
        *var_qfd_db2_slot = var_qfd_db2;
        *var_qfd_db3_slot = var_qfd_db3;
        *var_qfd_db4_slot = var_qfd_db4;
        *var_qfd_db5_slot = var_qfd_db5;
        *var_qfd_db6_slot = var_qfd_db6;
        *var_qfd_db7_slot = var_qfd_db7;
        *var_qfd_db8_slot = var_qfd_db8;
        *var_qfd_db9_slot = var_qfd_db9;
        *var_qfd_dn0_slot = var_qfd_dn0;
        *var_qfd_dn1_slot = var_qfd_dn1;
        *var_qfd_dn10_slot = var_qfd_dn10;
        *var_qfd_dn11_slot = var_qfd_dn11;
        *var_qfd_dn12_slot = var_qfd_dn12;
        *var_qfd_dn13_slot = var_qfd_dn13;
        *var_qfd_dn14_slot = var_qfd_dn14;
        *var_qfd_dn15_slot = var_qfd_dn15;
        *var_qfd_dn16_slot = var_qfd_dn16;
        *var_qfd_dn17_slot = var_qfd_dn17;
        *var_qfd_dn2_slot = var_qfd_dn2;
        *var_qfd_dn3_slot = var_qfd_dn3;
        *var_qfd_dn4_slot = var_qfd_dn4;
        *var_qfd_dn5_slot = var_qfd_dn5;
        *var_qfd_dn6_slot = var_qfd_dn6;
        *var_qfd_dn7_slot = var_qfd_dn7;
        *var_qfd_dn8_slot = var_qfd_dn8;
        *var_qfd_dn9_slot = var_qfd_dn9;
        *var_qfs_slot = var_qfs;
        *var_qfs_db0_slot = var_qfs_db0;
        *var_qfs_db1_slot = var_qfs_db1;
        *var_qfs_db10_slot = var_qfs_db10;
        *var_qfs_db11_slot = var_qfs_db11;
        *var_qfs_db2_slot = var_qfs_db2;
        *var_qfs_db3_slot = var_qfs_db3;
        *var_qfs_db4_slot = var_qfs_db4;
        *var_qfs_db5_slot = var_qfs_db5;
        *var_qfs_db6_slot = var_qfs_db6;
        *var_qfs_db7_slot = var_qfs_db7;
        *var_qfs_db8_slot = var_qfs_db8;
        *var_qfs_db9_slot = var_qfs_db9;
        *var_qfs_dn0_slot = var_qfs_dn0;
        *var_qfs_dn1_slot = var_qfs_dn1;
        *var_qfs_dn10_slot = var_qfs_dn10;
        *var_qfs_dn11_slot = var_qfs_dn11;
        *var_qfs_dn12_slot = var_qfs_dn12;
        *var_qfs_dn13_slot = var_qfs_dn13;
        *var_qfs_dn14_slot = var_qfs_dn14;
        *var_qfs_dn15_slot = var_qfs_dn15;
        *var_qfs_dn16_slot = var_qfs_dn16;
        *var_qfs_dn17_slot = var_qfs_dn17;
        *var_qfs_dn2_slot = var_qfs_dn2;
        *var_qfs_dn3_slot = var_qfs_dn3;
        *var_qfs_dn4_slot = var_qfs_dn4;
        *var_qfs_dn5_slot = var_qfs_dn5;
        *var_qfs_dn6_slot = var_qfs_dn6;
        *var_qfs_dn7_slot = var_qfs_dn7;
        *var_qfs_dn8_slot = var_qfs_dn8;
        *var_qfs_dn9_slot = var_qfs_dn9;
        *var_qgdo_slot = var_qgdo;
        *var_qgdo_db0_slot = var_qgdo_db0;
        *var_qgdo_db1_slot = var_qgdo_db1;
        *var_qgdo_db10_slot = var_qgdo_db10;
        *var_qgdo_db11_slot = var_qgdo_db11;
        *var_qgdo_db2_slot = var_qgdo_db2;
        *var_qgdo_db3_slot = var_qgdo_db3;
        *var_qgdo_db4_slot = var_qgdo_db4;
        *var_qgdo_db5_slot = var_qgdo_db5;
        *var_qgdo_db6_slot = var_qgdo_db6;
        *var_qgdo_db7_slot = var_qgdo_db7;
        *var_qgdo_db8_slot = var_qgdo_db8;
        *var_qgdo_db9_slot = var_qgdo_db9;
        *var_qgdo_dn0_slot = var_qgdo_dn0;
        *var_qgdo_dn1_slot = var_qgdo_dn1;
        *var_qgdo_dn10_slot = var_qgdo_dn10;
        *var_qgdo_dn11_slot = var_qgdo_dn11;
        *var_qgdo_dn12_slot = var_qgdo_dn12;
        *var_qgdo_dn13_slot = var_qgdo_dn13;
        *var_qgdo_dn14_slot = var_qgdo_dn14;
        *var_qgdo_dn15_slot = var_qgdo_dn15;
        *var_qgdo_dn16_slot = var_qgdo_dn16;
        *var_qgdo_dn17_slot = var_qgdo_dn17;
        *var_qgdo_dn2_slot = var_qgdo_dn2;
        *var_qgdo_dn3_slot = var_qgdo_dn3;
        *var_qgdo_dn4_slot = var_qgdo_dn4;
        *var_qgdo_dn5_slot = var_qgdo_dn5;
        *var_qgdo_dn6_slot = var_qgdo_dn6;
        *var_qgdo_dn7_slot = var_qgdo_dn7;
        *var_qgdo_dn8_slot = var_qgdo_dn8;
        *var_qgdo_dn9_slot = var_qgdo_dn9;
        *var_vdsei_slot = var_vdsei;
        *var_vdsei_db0_slot = var_vdsei_db0;
        *var_vdsei_db1_slot = var_vdsei_db1;
        *var_vdsei_db10_slot = var_vdsei_db10;
        *var_vdsei_db11_slot = var_vdsei_db11;
        *var_vdsei_db2_slot = var_vdsei_db2;
        *var_vdsei_db3_slot = var_vdsei_db3;
        *var_vdsei_db4_slot = var_vdsei_db4;
        *var_vdsei_db5_slot = var_vdsei_db5;
        *var_vdsei_db6_slot = var_vdsei_db6;
        *var_vdsei_db7_slot = var_vdsei_db7;
        *var_vdsei_db8_slot = var_vdsei_db8;
        *var_vdsei_db9_slot = var_vdsei_db9;
        *var_vdsei_dn0_slot = var_vdsei_dn0;
        *var_vdsei_dn1_slot = var_vdsei_dn1;
        *var_vdsei_dn10_slot = var_vdsei_dn10;
        *var_vdsei_dn11_slot = var_vdsei_dn11;
        *var_vdsei_dn12_slot = var_vdsei_dn12;
        *var_vdsei_dn13_slot = var_vdsei_dn13;
        *var_vdsei_dn14_slot = var_vdsei_dn14;
        *var_vdsei_dn15_slot = var_vdsei_dn15;
        *var_vdsei_dn16_slot = var_vdsei_dn16;
        *var_vdsei_dn17_slot = var_vdsei_dn17;
        *var_vdsei_dn2_slot = var_vdsei_dn2;
        *var_vdsei_dn3_slot = var_vdsei_dn3;
        *var_vdsei_dn4_slot = var_vdsei_dn4;
        *var_vdsei_dn5_slot = var_vdsei_dn5;
        *var_vdsei_dn6_slot = var_vdsei_dn6;
        *var_vdsei_dn7_slot = var_vdsei_dn7;
        *var_vdsei_dn8_slot = var_vdsei_dn8;
        *var_vdsei_dn9_slot = var_vdsei_dn9;
        *var_vgsei_slot = var_vgsei;
        *var_vgsei_db0_slot = var_vgsei_db0;
        *var_vgsei_db1_slot = var_vgsei_db1;
        *var_vgsei_db10_slot = var_vgsei_db10;
        *var_vgsei_db11_slot = var_vgsei_db11;
        *var_vgsei_db2_slot = var_vgsei_db2;
        *var_vgsei_db3_slot = var_vgsei_db3;
        *var_vgsei_db4_slot = var_vgsei_db4;
        *var_vgsei_db5_slot = var_vgsei_db5;
        *var_vgsei_db6_slot = var_vgsei_db6;
        *var_vgsei_db7_slot = var_vgsei_db7;
        *var_vgsei_db8_slot = var_vgsei_db8;
        *var_vgsei_db9_slot = var_vgsei_db9;
        *var_vgsei_dn0_slot = var_vgsei_dn0;
        *var_vgsei_dn1_slot = var_vgsei_dn1;
        *var_vgsei_dn10_slot = var_vgsei_dn10;
        *var_vgsei_dn11_slot = var_vgsei_dn11;
        *var_vgsei_dn12_slot = var_vgsei_dn12;
        *var_vgsei_dn13_slot = var_vgsei_dn13;
        *var_vgsei_dn14_slot = var_vgsei_dn14;
        *var_vgsei_dn15_slot = var_vgsei_dn15;
        *var_vgsei_dn16_slot = var_vgsei_dn16;
        *var_vgsei_dn17_slot = var_vgsei_dn17;
        *var_vgsei_dn2_slot = var_vgsei_dn2;
        *var_vgsei_dn3_slot = var_vgsei_dn3;
        *var_vgsei_dn4_slot = var_vgsei_dn4;
        *var_vgsei_dn5_slot = var_vgsei_dn5;
        *var_vgsei_dn6_slot = var_vgsei_dn6;
        *var_vgsei_dn7_slot = var_vgsei_dn7;
        *var_vgsei_dn8_slot = var_vgsei_dn8;
        *var_vgsei_dn9_slot = var_vgsei_dn9;
        *var_wg_slot = var_wg;
        *var_wgate_slot = var_wgate;
    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        var_guard168_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_mfactor_slot: &mut f64,
        var_mks_wl_slot: &mut f64,
        var_uc_toxb_slot: &mut f64,
    ) {
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_mfactor: f64 = *var_mfactor_slot;
        let mut var_mks_wl: f64 = *var_mks_wl_slot;
        let mut var_uc_toxb: f64 = *var_uc_toxb_slot;

        s.store_scalar(427, p.p447);

        s.store_scalar(957, p.p193);

        s.store_scalar(977, 0.0);

        s.store_scalar(978, 0.0);

        s.store_scalar(421, 40.0);

        s.store_scalar(828, 0.0);

        s.store_scalar(829, 0.0);

        s.store_scalar(830, 0.0);

        s.store_scalar(831, 0.0);

        s.store_scalar(862, 0.0);

        s.store_scalar(861, 0.0);

        s.store_scalar(870, 0.0);

        s.store_scalar(869, 0.0);

        s.store_scalar(66, 0.0);

        s.store_scalar(65, 0.0);

        s.store_scalar(68, 0.0);

        s.store_scalar(67, 0.0);

        s.store_scalar(832, 0.0);

        s.store_scalar(833, 0.0);

        s.store_scalar(834, 0.0);

        s.store_scalar(835, 0.0);

        s.store_scalar(838, 0.0);

        s.store_scalar(839, 0.0);

        s.store_scalar(841, 0.0);

        s.store_scalar(842, 0.0);

        s.store_scalar(843, 0.0);

        s.store_scalar(844, 0.0);

        s.store_scalar(845, 0.0);

        s.store_scalar(846, 0.0);

        s.store_scalar(840, 0.0);

        s.store_scalar(857, 0.0);

        s.store_scalar(858, 0.0);

        s.store_scalar(859, 0.0);

        s.store_scalar(860, 0.0);

        s.store_scalar(865, 0.0);

        s.store_scalar(866, 0.0);

        s.store_scalar(867, 0.0);

        s.store_scalar(868, 0.0);

        s.store_scalar(849, 0.0);

        s.store_scalar(854, 0.0);

        s.store_scalar(847, 0.0);

        s.store_scalar(852, 0.0);

        s.store_scalar(851, 0.0);

        s.store_scalar(856, 0.0);

        s.store_scalar(848, 0.0);

        s.store_scalar(853, 0.0);

        s.store_scalar(850, 0.0);

        s.store_scalar(855, 0.0);

        s.store_scalar(946, 0.0);

        s.store_scalar(944, 0.0);

        s.store_scalar(947, 0.0);

        s.store_scalar(945, 0.0);

        s.store_scalar(948, 0.0);

        s.store_scalar(816, 0.0);

        s.store_scalar(815, 0.0);

        s.store_scalar(873, 0.0);

        s.store_scalar(874, 0.0);

        s.store_scalar(875, 0.0);

        s.store_scalar(876, 0.0);

        s.store_scalar(877, 0.0);

        s.store_scalar(878, 0.0);

        s.store_scalar(879, 0.0);

        s.store_scalar(880, 0.0);

        s.store_scalar(881, 0.0);

        s.store_scalar(882, 0.0);

        s.store_scalar(883, 0.0);

        s.store_scalar(884, 0.0);

        s.store_scalar(360, 0.0);

        s.store_scalar(362, 0.0);

        s.store_scalar(361, 0.0);

        s.store_scalar(363, 0.0);

        s.store_scalar(603, 0.0);

        s.store_scalar(45, 0.0);

        s.store_scalar(46, 0.0);

        s.store_scalar(413, 0.0);

        s.store_scalar(932, 0.0);

        s.store_scalar(926, 0.0);

        s.store_scalar(927, 0.0);

        s.store_scalar(287, 0.0);

        s.store_scalar(407, 0.0);

        s.store_scalar(924, 0.0);

        s.store_scalar(925, 0.0);

        s.store_scalar(931, 0.0);

        s.store_scalar(990, 0.0);

        s.store_scalar(411, 0.0);

        s.store_scalar(429, 0.0);

        s.store_scalar(288, 0.0);

        s.store_scalar(308, 0.0);

        let (assign5320_e1936,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        s.store_scalar(448, assign5320_e1936);

        s.store_scalar(450, p.p104);

        s.store_scalar(451, p.p294);

        s.store_scalar(452, p.p222);

        s.store_scalar(453, p.p420);

        var_mfactor = 1.0;

        s.b[1004] = (s.v[452] < 0.0);
        s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });

        if s.b[1004] {
            s.store_scalar(452, 0.0);
        }

        s.b[1005] = (s.v[452] > 0.0);
        s.store_scalar(1005, if s.b[1005] { 1.0 } else { 0.0 });

        if s.b[1005] {
            s.store_scalar(452, 0.0);
        }

        s.b[1007] = (s.v[451] < 0.0);
        s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });

        if s.b[1007] {
            s.store_scalar(451, 0.0);
        }

        s.b[1010] = (s.v[453] < 0.0);
        s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });

        if s.b[1010] {
            s.store_scalar(453, 0.0);
        }

        s.b[1011] = (s.v[453] > 1.0);
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if s.b[1011] {
            s.store_scalar(453, 1.0);
        }

        s.store_scalar(964, p.p340);

        s.store_scalar(965, p.p343);

        s.store_scalar(963, p.p42);

        s.store_scalar(967, p.p354);

        s.store_scalar(969, p.p355);

        s.store_scalar(966, p.p346);

        s.store_scalar(968, p.p349);

        s.store_scalar(970, p.p352);

        s.store_scalar(972, p.p360);

        s.store_scalar(973, p.p367);

        s.store_scalar(976, p.p364);

        s.store_scalar(971, p.p377);

        s.store_scalar(974, p.p370);

        s.store_scalar(975, p.p371);

        s.b[1106] = ((s.v[963] < 3.0) && (s.v[963] > 0.0));
        s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });

        s.b[1109] = (s.v[964] < 5000000000000000.0);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1109]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1110] = (s.v[964] > 1e18);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1110]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1113] = (s.v[965] < 1e-8);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1113]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1114] = (s.v[965] > 1e-6);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1114]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1117] = (s.v[966] < 1.0);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1117]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1118] = (s.v[966] > 100000.0);
        s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1118]) {
            s.store_scalar(966, 100000.0);
        }

        s.b[1121] = (s.v[967] < 1.0);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1121]) {
            s.store_scalar(967, 1.0);
        }

        s.b[1122] = (s.v[967] > 100000.0);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1122]) {
            s.store_scalar(967, 100000.0);
        }

        s.b[1125] = (s.v[971] < 1.0);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1125]) {
            s.store_scalar(971, 1.0);
        }

        s.b[1126] = (s.v[971] > 100000.0);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1126]) {
            s.store_scalar(971, 100000.0);
        }

        s.b[1129] = (s.v[975] < 0.1);
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1129]) {
            s.store_scalar(975, 0.1);
        }

        s.b[1130] = (s.v[975] > 4.0);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1130]) {
            s.store_scalar(975, 4.0);
        }

        s.b[1133] = (s.v[972] < 0.0);
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1133]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1134] = (s.v[972] > 5.0);
        s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1134]) {
            s.store_scalar(972, 5.0);
        }

        s.b[1135] = (s.v[963] == 3.0);
        s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });

        s.b[1138] = (s.v[964] < 5000000000000000.0);
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1138]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1139] = (s.v[964] > 1e18);
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1139]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1142] = (s.v[965] < 1e-8);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1142]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1143] = (s.v[965] > 1e-6);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1143]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1146] = (s.v[966] < 1.0);
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1146]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1147] = (s.v[966] > 10000000000.0);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1147]) {
            s.store_scalar(966, 10000000000.0);
        }

        s.b[1150] = (s.v[971] < 100.0);
        s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1150]) {
            s.store_scalar(971, 100.0);
        }

        s.b[1151] = (s.v[971] > 2000000000.0);
        s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1151]) {
            s.store_scalar(971, 2000000000.0);
        }

        s.b[1154] = (s.v[972] < 0.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1154]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1155] = (s.v[972] > 5.0);
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1155]) {
            s.store_scalar(972, 5.0);
        }

        var_uc_toxb = p.p96;

        let assign7520_e3106: f64 = if var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        var_guard168 = assign7520_e3106;

        let (assign7530_e3110,) = {
    if (var_guard168 != 0.0) {
        (p.p95,)
    } else {
        (var_uc_toxb,)
    }
};
        var_uc_toxb = assign7530_e3110;

        let assign7540_e3113: f64 = if var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        var_guard169 = assign7540_e3113;

        let (assign7550_e3117,) = {
    if (var_guard169 != 0.0) {
        (5e-7,)
    } else {
        (var_uc_toxb,)
    }
};
        var_uc_toxb = assign7550_e3117;

        s.store_scalar(545, (p.p120 / ((100.0) as f64).powf(p.p122)));

        let assign7570_e3126: f64 = (100.0_f64).powf(p.p129);
        let assign7570_e3127: f64 = (p.p123 / assign7570_e3126);
        var_mks_wl = assign7570_e3127;

        s.store_scalar(547, (p.p198 / ((100.0) as f64).powf(p.p199)));

        s.store_scalar(548, (p.p200 / ((100.0) as f64).powf(p.p201)));

        s.store_scalar(549, (p.p183 / ((100.0) as f64).powf(p.p184)));

        s.store_scalar(550, (p.p202 / ((100.0) as f64).powf(p.p203)));

        s.store_scalar(551, (p.p190 / ((100.0) as f64).powf(p.p191)));

        s.store_scalar(552, (p.p186 / 100.0));

        s.store_scalar(553, (p.p192 / 100.0));

        s.store_scalar(554, (p.p73 * 100.0));

        s.store_scalar(555, (p.p311 / 100.0));

        s.store_scalar(556, (p.p312 / 100.0));

        s.store_scalar(557, (p.p313 / 100.0));

        s.store_scalar(558, (p.p314 / 100.0));

        s.store_scalar(544, (p.p336 / 1e-6));

        s.store_scalar(559, (p.p255 * 100.0));

        s.store_scalar(560, (p.p248 * 100.0));

        s.store_scalar(561, (p.p249 * 100.0));

        s.store_scalar(562, (p.p251 / 10000.0));

        s.store_scalar(563, (p.p266 * 10000.0));

        s.store_scalar(564, (p.p275 / 100.0));

        s.store_scalar(565, (p.p272 / 10000.0));

        s.store_scalar(572, (p.p273 / 10000.0));

        s.store_scalar(567, (p.p409 / 10000.0));

        s.store_scalar(568, (p.p412 / 100.0));

        s.store_scalar(569, (p.p413 / 10000.0));

        *var_guard168_slot = var_guard168;
        *var_guard169_slot = var_guard169;
        *var_mfactor_slot = var_mfactor;
        *var_mks_wl_slot = var_mks_wl;
        *var_uc_toxb_slot = var_uc_toxb;
    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        var_lbin_slot: &mut f64,
        var_lg_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_lwbin_slot: &mut f64,
        var_uc_cgdo_slot: &mut f64,
        var_uc_cgso_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_wbin_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wgate_slot: &mut f64,
    ) {
        let mut var_lbin: f64 = *var_lbin_slot;
        let mut var_lg: f64 = *var_lg_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_lwbin: f64 = *var_lwbin_slot;
        let mut var_uc_cgdo: f64 = *var_uc_cgdo_slot;
        let mut var_uc_cgso: f64 = *var_uc_cgso_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_wbin: f64 = *var_wbin_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wgate: f64 = *var_wgate_slot;

        s.store_scalar(570, (p.p414 / 100.0));

        s.store_scale(964, 964, 1000000.0);

        s.store_scalar(489, (p.p453 / 1e-6));

        s.store_scalar(764, (p.p274 + 273.15));

        let assign7910_e3247: f64 = (p.p0 + p.p116);
        var_lgate = assign7910_e3247;

        let assign7920_e3250: f64 = (p.p1 / p.p7);
        let assign7920_e3252: f64 = (assign7920_e3250 + p.p117);
        var_wgate = assign7920_e3252;

        let assign8070_e3352: f64 = (var_lgate * 1000000.0);
        var_lg = assign8070_e3352;

        let assign8080_e3355: f64 = (var_wgate * 1000000.0);
        var_wg = assign8080_e3355;

        let assign8090_e3358: f64 = (var_lg).powf(p.p553);
        var_lbin = assign8090_e3358;

        let assign8100_e3361: f64 = (var_wg).powf(p.p554);
        var_wbin = assign8100_e3361;

        let assign8110_e3364: f64 = (var_lbin * var_wbin);
        var_lwbin = assign8110_e3364;

        s.store_scalar(454, (((p.p89 + (p.p555 / var_lbin)) + (p.p643 / var_wbin)) + (p.p731 / var_lwbin)));

        s.store_scalar(455, (((p.p92 + (p.p556 / var_lbin)) + (p.p644 / var_wbin)) + (p.p732 / var_lwbin)));

        s.store_scalar(456, (((p.p93 + (p.p557 / var_lbin)) + (p.p645 / var_wbin)) + (p.p733 / var_lwbin)));

        s.store_scalar(457, (((p.p94 + (p.p558 / var_lbin)) + (p.p646 / var_wbin)) + (p.p734 / var_lwbin)));

        s.store_scalar(458, (((p.p110 + (p.p559 / var_lbin)) + (p.p647 / var_wbin)) + (p.p735 / var_lwbin)));

        let assign8170_e3433: f64 = (p.p560 / var_lbin);
        let assign8170_e3434: f64 = (p.p111 + assign8170_e3433);
        let assign8170_e3437: f64 = (p.p648 / var_wbin);
        let assign8170_e3438: f64 = (assign8170_e3434 + assign8170_e3437);
        let assign8170_e3441: f64 = (p.p736 / var_lwbin);
        let assign8170_e3442: f64 = (assign8170_e3438 + assign8170_e3441);
        var_uc_nover = assign8170_e3442;

        let assign8180_e3446: f64 = (p.p561 / var_lbin);
        let assign8180_e3447: f64 = (p.p112 + assign8180_e3446);
        let assign8180_e3450: f64 = (p.p649 / var_wbin);
        let assign8180_e3451: f64 = (assign8180_e3447 + assign8180_e3450);
        let assign8180_e3454: f64 = (p.p737 / var_lwbin);
        let assign8180_e3455: f64 = (assign8180_e3451 + assign8180_e3454);
        var_uc_novers = assign8180_e3455;

        s.store_scalar(461, (((p.p126 + (p.p562 / var_lbin)) + (p.p650 / var_wbin)) + (p.p738 / var_lwbin)));

        s.store_scalar(462, (((p.p136 + (p.p563 / var_lbin)) + (p.p651 / var_wbin)) + (p.p739 / var_lwbin)));

        s.store_scalar(463, (((p.p138 + (p.p564 / var_lbin)) + (p.p652 / var_wbin)) + (p.p740 / var_lwbin)));

        s.store_scalar(464, (((p.p141 + (p.p565 / var_lbin)) + (p.p653 / var_wbin)) + (p.p741 / var_lwbin)));

        s.store_scalar(465, (((p.p144 + (p.p566 / var_lbin)) + (p.p654 / var_wbin)) + (p.p742 / var_lwbin)));

        s.store_scalar(466, (((p.p145 + (p.p567 / var_lbin)) + (p.p655 / var_wbin)) + (p.p743 / var_lwbin)));

        s.store_scalar(467, (((p.p146 + (p.p568 / var_lbin)) + (p.p656 / var_wbin)) + (p.p744 / var_lwbin)));

        s.store_scalar(468, (((p.p147 + (p.p569 / var_lbin)) + (p.p657 / var_wbin)) + (p.p745 / var_lwbin)));

        s.store_scalar(469, (((p.p148 + (p.p570 / var_lbin)) + (p.p658 / var_wbin)) + (p.p746 / var_lwbin)));

        s.store_scalar(470, (((p.p149 + (p.p571 / var_lbin)) + (p.p659 / var_wbin)) + (p.p747 / var_lwbin)));

        s.store_scalar(471, (((p.p151 + (p.p572 / var_lbin)) + (p.p660 / var_wbin)) + (p.p748 / var_lwbin)));

        s.store_scalar(472, (((p.p154 + (p.p573 / var_lbin)) + (p.p661 / var_wbin)) + (p.p749 / var_lwbin)));

        s.store_scalar(473, (((p.p157 + (p.p574 / var_lbin)) + (p.p662 / var_wbin)) + (p.p750 / var_lwbin)));

        s.store_scalar(474, (((p.p158 + (p.p575 / var_lbin)) + (p.p663 / var_wbin)) + (p.p751 / var_lwbin)));

        s.store_scalar(475, (((p.p159 + (p.p576 / var_lbin)) + (p.p664 / var_wbin)) + (p.p752 / var_lwbin)));

        s.store_scalar(476, (((p.p161 + (p.p577 / var_lbin)) + (p.p665 / var_wbin)) + (p.p753 / var_lwbin)));

        s.store_scalar(477, (((p.p169 + (p.p578 / var_lbin)) + (p.p666 / var_wbin)) + (p.p754 / var_lwbin)));

        s.store_scalar(478, (((p.p170 + (p.p579 / var_lbin)) + (p.p667 / var_wbin)) + (p.p755 / var_lwbin)));

        s.store_scalar(479, (((p.p172 + (p.p580 / var_lbin)) + (p.p668 / var_wbin)) + (p.p756 / var_lwbin)));

        s.store_scalar(480, (((p.p177 + (p.p581 / var_lbin)) + (p.p669 / var_wbin)) + (p.p757 / var_lwbin)));

        s.store_scalar(481, (((p.p179 + (p.p582 / var_lbin)) + (p.p670 / var_wbin)) + (p.p758 / var_lwbin)));

        s.store_scalar(482, (((p.p180 + (p.p583 / var_lbin)) + (p.p671 / var_wbin)) + (p.p759 / var_lwbin)));

        s.store_scalar(483, (((p.p185 + (p.p584 / var_lbin)) + (p.p672 / var_wbin)) + (p.p760 / var_lwbin)));

        s.store_scalar(484, (((p.p182 + (p.p585 / var_lbin)) + (p.p673 / var_wbin)) + (p.p761 / var_lwbin)));

        s.store_scalar(485, (((p.p181 + (p.p586 / var_lbin)) + (p.p674 / var_wbin)) + (p.p762 / var_lwbin)));

        s.store_scalar(486, (((p.p187 + (p.p587 / var_lbin)) + (p.p675 / var_wbin)) + (p.p763 / var_lwbin)));

        s.store_scalar(487, (((p.p188 + (p.p588 / var_lbin)) + (p.p676 / var_wbin)) + (p.p764 / var_lwbin)));

        s.store_scalar(488, (((p.p189 + (p.p589 / var_lbin)) + (p.p677 / var_wbin)) + (p.p765 / var_lwbin)));

        s.store_scalar(490, (((p.p194 + (p.p590 / var_lbin)) + (p.p678 / var_wbin)) + (p.p766 / var_lwbin)));

        s.store_scalar(491, (((p.p195 + (p.p591 / var_lbin)) + (p.p679 / var_wbin)) + (p.p767 / var_lwbin)));

        s.store_scalar(492, (((p.p196 + (p.p592 / var_lbin)) + (p.p680 / var_wbin)) + (p.p768 / var_lwbin)));

        s.store_scalar(493, (((p.p197 + (p.p593 / var_lbin)) + (p.p681 / var_wbin)) + (p.p769 / var_lwbin)));

        s.store_scalar(494, (((p.p204 + (p.p594 / var_lbin)) + (p.p682 / var_wbin)) + (p.p770 / var_lwbin)));

        s.store_scalar(495, (((p.p205 + (p.p595 / var_lbin)) + (p.p683 / var_wbin)) + (p.p771 / var_lwbin)));

        s.store_scalar(496, (((p.p210 + (p.p596 / var_lbin)) + (p.p684 / var_wbin)) + (p.p772 / var_lwbin)));

        s.store_scalar(497, (((p.p211 + (p.p597 / var_lbin)) + (p.p685 / var_wbin)) + (p.p773 / var_lwbin)));

        s.store_scalar(498, (((p.p212 + (p.p598 / var_lbin)) + (p.p686 / var_wbin)) + (p.p774 / var_lwbin)));

        s.store_scalar(499, (((p.p214 + (p.p599 / var_lbin)) + (p.p687 / var_wbin)) + (p.p775 / var_lwbin)));

        s.store_scalar(500, (((p.p215 + (p.p600 / var_lbin)) + (p.p688 / var_wbin)) + (p.p776 / var_lwbin)));

        s.store_scalar(501, (((p.p216 + (p.p601 / var_lbin)) + (p.p689 / var_wbin)) + (p.p777 / var_lwbin)));

        s.store_scalar(502, (((p.p217 + (p.p602 / var_lbin)) + (p.p690 / var_wbin)) + (p.p778 / var_lwbin)));

        s.store_scalar(503, (((p.p218 + (p.p603 / var_lbin)) + (p.p691 / var_wbin)) + (p.p779 / var_lwbin)));

        s.store_scalar(504, (((p.p219 + (p.p604 / var_lbin)) + (p.p692 / var_wbin)) + (p.p780 / var_lwbin)));

        let assign8620_e4018: f64 = (p.p605 / var_lbin);
        let assign8620_e4019: f64 = (p.p269 + assign8620_e4018);
        let assign8620_e4022: f64 = (p.p693 / var_wbin);
        let assign8620_e4023: f64 = (assign8620_e4019 + assign8620_e4022);
        let assign8620_e4026: f64 = (p.p781 / var_lwbin);
        let assign8620_e4027: f64 = (assign8620_e4023 + assign8620_e4026);
        var_uc_cgso = assign8620_e4027;

        let assign8630_e4031: f64 = (p.p606 / var_lbin);
        let assign8630_e4032: f64 = (p.p268 + assign8630_e4031);
        let assign8630_e4035: f64 = (p.p694 / var_wbin);
        let assign8630_e4036: f64 = (assign8630_e4032 + assign8630_e4035);
        let assign8630_e4039: f64 = (p.p782 / var_lwbin);
        let assign8630_e4040: f64 = (assign8630_e4036 + assign8630_e4039);
        var_uc_cgdo = assign8630_e4040;

        s.store_scalar(507, (((p.p226 + (p.p607 / var_lbin)) + (p.p695 / var_wbin)) + (p.p783 / var_lwbin)));

        s.store_scalar(508, (((p.p227 + (p.p608 / var_lbin)) + (p.p696 / var_wbin)) + (p.p784 / var_lwbin)));

        s.store_scalar(509, (((p.p228 + (p.p609 / var_lbin)) + (p.p697 / var_wbin)) + (p.p785 / var_lwbin)));

        s.store_scalar(510, (((p.p232 + (p.p610 / var_lbin)) + (p.p698 / var_wbin)) + (p.p786 / var_lwbin)));

        s.store_scalar(511, (((p.p240 + (p.p611 / var_lbin)) + (p.p699 / var_wbin)) + (p.p787 / var_lwbin)));

        s.store_scalar(512, (((p.p241 + (p.p612 / var_lbin)) + (p.p700 / var_wbin)) + (p.p788 / var_lwbin)));

        s.store_scalar(513, (((p.p245 + (p.p613 / var_lbin)) + (p.p701 / var_wbin)) + (p.p789 / var_lwbin)));

        s.store_scalar(514, (((p.p246 + (p.p614 / var_lbin)) + (p.p702 / var_wbin)) + (p.p790 / var_lwbin)));

        s.store_scalar(515, (((p.p247 + (p.p615 / var_lbin)) + (p.p703 / var_wbin)) + (p.p791 / var_lwbin)));

        s.store_scalar(516, (((p.p250 + (p.p616 / var_lbin)) + (p.p704 / var_wbin)) + (p.p792 / var_lwbin)));

        s.store_scalar(517, (((p.p253 + (p.p617 / var_lbin)) + (p.p705 / var_wbin)) + (p.p793 / var_lwbin)));

        s.store_scalar(518, (((p.p254 + (p.p618 / var_lbin)) + (p.p706 / var_wbin)) + (p.p794 / var_lwbin)));

        s.store_scalar(519, (((p.p256 + (p.p619 / var_lbin)) + (p.p707 / var_wbin)) + (p.p795 / var_lwbin)));

        s.store_scalar(520, (((p.p257 + (p.p620 / var_lbin)) + (p.p708 / var_wbin)) + (p.p796 / var_lwbin)));

        s.store_scalar(522, (((p.p265 + (p.p622 / var_lbin)) + (p.p710 / var_wbin)) + (p.p798 / var_lwbin)));

        s.store_scalar(523, (((p.p278 + (p.p623 / var_lbin)) + (p.p711 / var_wbin)) + (p.p799 / var_lwbin)));

        s.store_scalar(524, (((p.p281 + (p.p624 / var_lbin)) + (p.p712 / var_wbin)) + (p.p800 / var_lwbin)));

        s.store_scalar(525, (((p.p79 + (p.p625 / var_lbin)) + (p.p713 / var_wbin)) + (p.p801 / var_lwbin)));

        s.store_scalar(526, (((p.p86 + (p.p626 / var_lbin)) + (p.p714 / var_wbin)) + (p.p802 / var_lwbin)));

        s.store_scalar(528, (((p.p76 + (p.p628 / var_lbin)) + (p.p716 / var_wbin)) + (p.p804 / var_lwbin)));

        s.store_scalar(529, (((p.p81 + (p.p629 / var_lbin)) + (p.p717 / var_wbin)) + (p.p805 / var_lwbin)));

        s.store_scalar(530, (((p.p74 + (p.p630 / var_lbin)) + (p.p718 / var_wbin)) + (p.p806 / var_lwbin)));

        s.store_scalar(531, (((p.p298 + (p.p631 / var_lbin)) + (p.p719 / var_wbin)) + (p.p807 / var_lwbin)));

        s.store_scalar(532, (((p.p83 + (p.p632 / var_lbin)) + (p.p720 / var_wbin)) + (p.p808 / var_lwbin)));

        s.store_scalar(533, (((p.p84 + (p.p633 / var_lbin)) + (p.p721 / var_wbin)) + (p.p809 / var_lwbin)));

        s.store_scalar(534, (((p.p62 + (p.p634 / var_lbin)) + (p.p722 / var_wbin)) + (p.p810 / var_lwbin)));

        s.store_scalar(535, (((p.p59 + (p.p635 / var_lbin)) + (p.p723 / var_wbin)) + (p.p811 / var_lwbin)));

        s.store_scalar(536, (((p.p60 + (p.p636 / var_lbin)) + (p.p724 / var_wbin)) + (p.p812 / var_lwbin)));

        s.store_scalar(537, (((p.p85 + (p.p637 / var_lbin)) + (p.p725 / var_wbin)) + (p.p813 / var_lwbin)));

        s.store_scalar(538, (((p.p82 + (p.p638 / var_lbin)) + (p.p726 / var_wbin)) + (p.p814 / var_lwbin)));

        s.store_scalar(539, (((p.p61 + (p.p639 / var_lbin)) + (p.p727 / var_wbin)) + (p.p815 / var_lwbin)));

        s.store_scalar(540, (((p.p75 + (p.p640 / var_lbin)) + (p.p728 / var_wbin)) + (p.p816 / var_lwbin)));

        s.store_scalar(541, (((p.p80 + (p.p641 / var_lbin)) + (p.p729 / var_wbin)) + (p.p817 / var_lwbin)));

        s.store_scalar(542, (((p.p77 + (p.p642 / var_lbin)) + (p.p730 / var_wbin)) + (p.p818 / var_lwbin)));

        s.store_scalar(818, (((p.p493 + (p.p824 / var_lbin)) + (p.p839 / var_wbin)) + (p.p854 / var_lwbin)));

        s.store_scalar(819, (((p.p494 + (p.p825 / var_lbin)) + (p.p840 / var_wbin)) + (p.p855 / var_lwbin)));

        s.store_scalar(820, (((p.p496 + (p.p826 / var_lbin)) + (p.p841 / var_wbin)) + (p.p856 / var_lwbin)));

        s.store_scalar(821, (((p.p513 + (p.p827 / var_lbin)) + (p.p842 / var_wbin)) + (p.p857 / var_lwbin)));

        s.store_scalar(822, (((p.p515 + (p.p828 / var_lbin)) + (p.p843 / var_wbin)) + (p.p858 / var_lwbin)));

        s.store_scalar(823, (((p.p516 + (p.p829 / var_lbin)) + (p.p844 / var_wbin)) + (p.p859 / var_lwbin)));

        s.store_scalar(824, (((p.p517 + (p.p830 / var_lbin)) + (p.p845 / var_wbin)) + (p.p860 / var_lwbin)));

        s.store_scalar(825, (((p.p519 + (p.p831 / var_lbin)) + (p.p846 / var_wbin)) + (p.p861 / var_lwbin)));

        s.store_scalar(826, (((p.p536 + (p.p832 / var_lbin)) + (p.p847 / var_wbin)) + (p.p862 / var_lwbin)));

        s.store_scalar(827, (((p.p538 + (p.p833 / var_lbin)) + (p.p848 / var_wbin)) + (p.p863 / var_lwbin)));

        s.b[1181] = (s.v[963] != 0.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p342));
            s.store_mul_offset_ad_rhs(964, 964, A::div_from_scalar(p.p341, s.ad_value(337)), 1.0);
        }

        s.b[1182] = (s.v[964] < 1e21);
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1182]) {
            s.store_scalar(964, 1e21);
        }

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p369));
            s.store_scaled_offset_ad(973, A::div_from_scalar(p.p368, s.ad_value(337)), 1.0, s.v[973]);
            s.store_scalar(337, ((var_lg) as f64).powf(p.p362));
            s.store_scaled_offset_ad(972, A::div_from_scalar(p.p361, s.ad_value(337)), 1.0, p.p360);
        }

        s.b[1183] = (s.v[972] < 0.0);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1183]) {
            s.store_scalar(972, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p348));
            s.store_scaled_offset_ad(966, A::div_from_scalar(p.p347, s.ad_value(337)), 1.0, p.p346);
        }

        s.b[1184] = (s.v[966] < 1.0);
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1184]) {
            s.store_scalar(966, 1.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p351));
            s.store_scaled_offset_ad(968, A::div_from_scalar(p.p350, s.ad_value(337)), 1.0, p.p349);
        }

        s.b[1185] = (s.v[968] < 0.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1185]) {
            s.store_scalar(968, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p357));
            s.store_scaled_offset_ad(967, A::div_from_scalar(p.p356, s.ad_value(337)), 1.0, p.p354);
        }

        s.b[1186] = (s.v[967] < 0.0);
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1186]) {
            s.store_scalar(967, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p359));
            s.store_scaled_offset_ad(969, A::div_from_scalar(p.p358, s.ad_value(337)), 1.0, p.p355);
        }

        s.b[1187] = (s.v[969] < 0.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1187]) {
            s.store_scalar(969, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((var_lg) as f64).powf(p.p373));
            s.store_scaled_offset_ad(974, A::div_from_scalar(p.p372, s.ad_value(337)), 1.0, s.v[974]);
            s.store_scalar(337, ((var_lg) as f64).powf(p.p375));
            s.store_mul_offset_ad_rhs(975, 975, A::div_from_scalar(p.p374, s.ad_value(337)), 1.0);
        }

        s.b[1188] = (s.v[975] < 0.1);
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1188]) {
            s.store_scalar(975, 0.1);
        }

        if (!s.b[1181]) {
            s.store_scalar(964, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
        }

        s.b[1240] = ((s.v[450] * s.v[451]) > 1.0);
        s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });

        if s.b[1240] {
            s.store_div_from_scalar(450, 1.0, 451);
        }

        s.b[1242] = ((p.p40 == 1.0) && (((p.p19 > 0.0) && (s.v[459] == 0.0)) || ((p.p18 > 0.0) && (s.v[460] == 0.0))));
        s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });

        let (assign10160_e5354,) = {
    if s.b[1242] {
        (0.0,)
    } else {
        (s.v[449],)
    }
};
        s.store_scalar(449, assign10160_e5354);

        let (assign10170_e5359,) = {
    if (!s.b[1242]) {
        (p.p40,)
    } else {
        (s.v[449],)
    }
};
        s.store_scalar(449, assign10170_e5359);

        s.b[1243] = (s.v[449] == 1.0);
        s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });

        let (assign10190_e5371,) = {
    if s.b[1243] {
        let (assign10190_e5369,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10190_e5369,)
    } else {
        (s.v[75],)
    }
};
        s.store_scalar(75, assign10190_e5371);

        let (assign10200_e5380,) = {
    if s.b[1243] {
        let (assign10200_e5378,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10200_e5378,)
    } else {
        (s.v[76],)
    }
};
        s.store_scalar(76, assign10200_e5380);

        s.b[1244] = ((p.p17 == 0.0) || (p.p17 == 2.0));
        s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });

        let (assign10220_e5394,) = {
    if ((!s.b[1243]) && s.b[1244]) {
        (0.0,)
    } else {
        (s.v[75],)
    }
};
        s.store_scalar(75, assign10220_e5394);

        let (assign10230_e5401,) = {
    if ((!s.b[1243]) && s.b[1244]) {
        (0.0,)
    } else {
        (s.v[76],)
    }
};
        s.store_scalar(76, assign10230_e5401);

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(335, (((p.p130 * p.p2) * p.p7) + (((s.v[530] + s.v[538]) * (((p.p67 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p68 * p.p100) * 1000000.0) + p.p101))));
        }

        let (assign10250_e5446,) = {
    if ((!s.b[1243]) && (!s.b[1244])) {
        let (assign10250_e5444,) = {
            if (s.v[335] > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10250_e5444,)
    } else {
        (s.v[75],)
    }
};
        s.store_scalar(75, assign10250_e5446);

        *var_lbin_slot = var_lbin;
        *var_lg_slot = var_lg;
        *var_lgate_slot = var_lgate;
        *var_lwbin_slot = var_lwbin;
        *var_uc_cgdo_slot = var_uc_cgdo;
        *var_uc_cgso_slot = var_uc_cgso;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_novers_slot = var_uc_novers;
        *var_wbin_slot = var_wbin;
        *var_wg_slot = var_wg;
        *var_wgate_slot = var_wgate;
    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        var_lg: f64,
        var_lgate: f64,
        var_mks_wl: f64,
        var_wg: f64,
        var_wgate: f64,
        var_cecox_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_weff_cv_slot: &mut f64,
        var_weffcv_nf_slot: &mut f64,
    ) {
        let mut var_cecox: f64 = *var_cecox_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_weff_cv: f64 = *var_weff_cv_slot;
        let mut var_weffcv_nf: f64 = *var_weffcv_nf_slot;

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(335, (((p.p131 * p.p3) * p.p7) + ((s.v[540] * (((p.p69 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p70 * p.p100) * 1000000.0) + p.p101))));
        }

        let (assign10270_e5489,) = {
    if ((!s.b[1243]) && (!s.b[1244])) {
        let (assign10270_e5487,) = {
            if (s.v[335] > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5487,)
    } else {
        (s.v[76],)
    }
};
        s.store_scalar(76, assign10270_e5489);

        s.store_scalar(571, (p.p12 / 1e-6));

        s.store_scalar(554, (p.p73 * 100.0));

        s.store_scalar(463, (s.v[463] / 1e-6));

        s.store_scalar(464, (s.v[464] / 1e-6));

        s.store_scalar(494, (s.v[494] / 1e-6));

        let assign10330_e5507: f64 = (var_uc_nover / 1e-6);
        var_uc_nover = assign10330_e5507;

        let assign10340_e5510: f64 = (var_uc_novers / 1e-6);
        var_uc_novers = assign10340_e5510;

        s.store_scalar(502, (s.v[502] / 100.0));

        s.store_scalar(499, (s.v[499] / 100.0));

        s.store_scalar(454, (s.v[454] / 100.0));

        s.store_scalar(510, (s.v[510] * 10000.0));

        s.store_scalar(517, (s.v[517] / 100.0));

        s.store_scalar(518, (s.v[518] * 100.0));

        s.store_scalar(514, (s.v[514] * 100.0));

        s.store_scalar(520, (s.v[520] * 100.0));

        s.store_scalar(491, (s.v[491] * 100.0));

        s.store_scalar(511, (s.v[511] / 10.0));

        s.store_scalar(512, (s.v[512] * 100.0));

        s.store_scalar(522, (s.v[522] / 100.0));

        s.store_scalar(528, (s.v[528] / 1e-6));

        s.store_scalar(531, (s.v[531] / 100.0));

        s.store_scalar(532, (s.v[532] / 100.0));

        s.store_scalar(533, (s.v[533] / 100.0));

        s.store_scalar(538, (s.v[538] / 100.0));

        s.store_scalar(541, (s.v[541] / 100.0));

        s.store_scalar(458, (-s.v[458]));

        s.store_scale(973, 973, 0.01);

        s.store_scalar(81, p.p28);

        s.b[82] = ((p.p133 != 0.0) || (p.p134 != 0.0));
        s.store_scalar(82, if s.b[82] { 1.0 } else { 0.0 });

        s.b[1246] = (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0));
        s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });

        let (assign10600_e5601,) = {
    if s.b[1246] {
        (0.0,)
    } else {
        (s.v[765],)
    }
};
        s.store_scalar(765, assign10600_e5601);

        let (assign10610_e5606,) = {
    if (!s.b[1246]) {
        (1.0,)
    } else {
        (s.v[765],)
    }
};
        s.store_scalar(765, assign10610_e5606);

        s.store_scalar(581, (var_wg * var_lg));

        s.store_scalar(777, (p.p289 * 1000000.0));

        s.store_scalar(616, (s.v[457] - (s.v[764] * (9.025e-5 + (s.v[764] * 1e-7)))));

        let assign10650_e5624: f64 = (8.8541878e-12 * p.p267);
        var_cecox = assign10650_e5624;

        s.copy_ad(618, 452);

        s.b[1247] = (s.v[471] == 0.0);
        s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });

        let (assign10680_e5632,) = {
    if s.b[1247] {
        (0.0,)
    } else {
        (s.v[615],)
    }
};
        s.store_scalar(615, assign10680_e5632);

        if s.b[1247] {
            s.store_scalar(642, 0.0);
        }

        let (assign10700_e5641,) = {
    if (!s.b[1247]) {
        (1.0,)
    } else {
        (s.v[615],)
    }
};
        s.store_scalar(615, assign10700_e5641);

        if (!s.b[1247]) {
            s.store_scalar(642, ((((1.0 + (1.0 / var_lg))) as f64).powf(p.p153) * s.v[471]));
        }

        s.store_scalar(619, (1.0 + (((var_lg) as f64).powf(p.p229) * p.p230)));

        s.store_scalar(335, ((1.0 / (p.p118 + (0.5 * p.p0))) + (1.0 / (p.p119 + (0.5 * p.p0)))));

        s.store_scalar(589, (2.0 / s.v[335]));

        s.b[1248] = (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0))));
        s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });

        if s.b[1248] {
            s.store_scalar(335, 0.0);
            s.store_scalar(721, 0.0);
        }

        let mut assign10780_loop_guard: usize = 0;
        while {
            let assign10780_cond_e5711: f64 = if (s.b[1248] && (s.v[721] < p.p7)) { 1.0 } else { 0.0 };
            assign10780_cond_e5711 != 0.0
        } {
            assign10780_loop_guard += 1;
            assert!(assign10780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1248] {
                s.store_add_scaled_inputs3_mixed_iaa(335, 335, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p8 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p9 + (0.5 * p.p0)))), 1.0);
                s.store_offset(721, 721, 1.0);
            }
        }

        if s.b[1248] {
            s.store_div_from_scalar(588, (2.0 * p.p7), 335);
        }

        if (!s.b[1248]) {
            s.store_scalar(588, 0.0);
        }

        s.store_scalar(773, s.v[528]);

        s.store_scalar(620, s.v[476]);

        s.store_scalar(621, s.v[464]);

        s.store_scalar(622, s.v[463]);

        s.b[1249] = ((p.p32 == 1.0) && s.b[623]);
        s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });

        if s.b[1249] {
            s.store_scalar(620, (s.v[620] * ((p.p282 * (((s.v[571]) as f64).ln() - ((s.v[622]) as f64).ln())) + 1.0)));
            s.store_scalar(621, ((s.v[621] + s.v[571]) - s.v[622]));
            s.store_scalar(773, ((s.v[773] + s.v[571]) - s.v[622]));
            s.store_scalar(622, s.v[571]);
        }

        s.store_scale(573, 620, ((1.0 + (p.p162 / ((var_wg) as f64).powf(p.p163))) * ((1.0 + (p.p164 / ((var_lg) as f64).powf(p.p165))) * (1.0 + (p.p167 / ((s.v[581]) as f64).powf(p.p168))))));

        s.b[1251] = (s.v[588] > 0.0);
        s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });

        if s.b[1251] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[500])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[499], s.ad_value(588)), s.v[501]);
            s.store_scalar(337, (((s.v[499] / s.v[589])) as f64).powf(s.v[501]));
            s.store_div_scaled_product_offset_denominator(573, s.ad_value(573), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        s.store_scalar(624, ((p.p171 * (1.0 + (p.p173 / ((var_lg) as f64).powf(p.p176)))) * (1.0 + (p.p174 / ((var_wg) as f64).powf(p.p175)))));

        if (s.v[573] < 1e-25) {
            s.store_scalar(573, 1e-25);
        }

        if (s.v[624] < 1e-25) {
            s.store_scalar(624, 1e-25);
        }

        s.store_scalar(335, ((var_lg) as f64).powf(p.p156));

        s.store_scalar(625, (((s.v[472] * s.v[335]) / (s.v[335] + p.p155)) / 1.034943e-10));

        s.store_scalar(626, (s.v[473] / 1.034943e-10));

        s.store_scalar(627, ((p.p319 * (1.0 + (p.p320 / ((var_lg) as f64).powf(p.p321)))) * (1.0 + (p.p322 / ((var_wg) as f64).powf(p.p323)))));

        s.store_scalar(335, ((1.0 + (p.p386 / ((var_lg) as f64).powf(p.p387))) * (1.0 + (p.p388 / ((var_wg) as f64).powf(p.p389)))));

        s.store_scalar(633, (p.p384 * s.v[335]));

        s.store_scalar(634, (p.p385 * s.v[335]));

        s.store_scalar(574, (p.p97 + (s.v[545] / (((var_lgate + p.p121)) as f64).powf(p.p122))));

        s.store_offset(575, 451, (s.v[545] / (((var_lgate + p.p121)) as f64).powf(p.p122)));

        s.store_scalar(577, (p.p114 + (var_mks_wl / (((var_wgate + p.p128)) as f64).powf(p.p129))));

        s.store_scalar(578, (p.p295 + (var_mks_wl / (((var_wgate + p.p128)) as f64).powf(p.p129))));

        let assign11130_e6021: f64 = (var_wgate + p.p128);
        let assign11130_e6023: f64 = (assign11130_e6021).powf(p.p129);
        let assign11130_e6024: f64 = (var_mks_wl / assign11130_e6023);
        let assign11130_e6025: f64 = (p.p115 + assign11130_e6024);
        var_dwcv = assign11130_e6025;

        s.store_sub_from_scalar_ad(162, var_lgate, A::offset(s.ad_value(575), s.v[574]));

        s.store_scalar(628, (var_lgate + (p.p124 / ((s.v[581]) as f64).powf(p.p125))));

        s.store_scalar(629, (s.v[461] / ((s.v[581]) as f64).powf(p.p127)));

        s.store_scalar(335, (1.0 + (p.p206 / (((s.v[628] * 1000000.0)) as f64).powf(p.p207))));

        s.store_scalar(336, (1.0 + (p.p208 / ((var_wg) as f64).powf(p.p209))));

        s.store_scalar(495, ((s.v[495] * s.v[335]) * s.v[336]));

        s.store_scalar(163, (var_wgate - (2.0 * s.v[577])));

        s.store_scalar(630, (var_wgate - (2.0 * s.v[578])));

        let assign11240_e6084: f64 = (2.0 * var_dwcv);
        let assign11240_e6085: f64 = (var_wgate - assign11240_e6084);
        var_weff_cv = assign11240_e6085;

        s.store_scalar(632, (s.v[163] * p.p7));

        let assign11320_e6112: f64 = (var_weff_cv * p.p7);
        var_weffcv_nf = assign11320_e6112;

        s.store_scale(584, 621, (1.0 + (p.p142 / ((var_wg) as f64).powf(p.p143))));

        s.store_scale(622, 622, (1.0 + (p.p233 / ((var_wg) as f64).powf(p.p234))));

        s.store_scale(335, 622, 1e-6);

        s.store_scale(336, 584, 1e-6);

        s.b[1259] = (s.v[335] < 1000000000000000.0);
        s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });

        if s.b[1259] {
            s.store_scalar(335, 1000000000000000.0);
        }

        s.store_scale(622, 335, 1000000.0);

        s.b[1261] = (s.v[336] < 1000000000000000.0);
        s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });

        if s.b[1261] {
            s.store_scalar(336, 1000000000000000.0);
        }

        s.store_scale(584, 336, 1000000.0);

        s.b[1262] = (s.v[588] > 0.0);
        s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });

        if s.b[1262] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[503])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[502], s.ad_value(588)), s.v[504]);
            s.store_scalar(337, (((s.v[502] / s.v[589])) as f64).powf(s.v[504]));
            s.store_div_scaled_product_offset_denominator(585, s.ad_value(584), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        if (!s.b[1262]) {
            s.copy_ad(585, 584);
        }

        s.b[1263] = ((s.v[582] > p.p140) || (p.p140 <= 0.0));
        s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });

        if s.b[1263] {
            s.store_add_scaled_inputs(586, 622, ((var_lgate - p.p140) * 1.0 / (var_lgate)), 585, (p.p140 * 1.0 / (var_lgate)));
        }

        if (!s.b[1263]) {
            s.store_add_scaled_inputs3_indices(586, 585, 1.0, 585, ((p.p140 - var_lgate) * 1.0 / (p.p140)), 622, (-((p.p140 - var_lgate) * 1.0 / (p.p140))));
        }

        s.store_scalar(337, ((0.5 * var_lgate) - p.p140));

        s.store_scalar(781, ((s.v[337] - 1e-9) - 1e-10));

        s.store_scalar(782, ((4.0 * 1e-9) * 1e-10));

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_offset_input(782, 782, (s.v[781] * s.v[781]));

        s.store_scaled_offset_ad(334, A::div_from_scalar(s.v[781], s.ad_value(782)), 1.0, 0.5);

        s.store_offset_scaled(337, 782, 0.5, ((((s.v[781]) * (0.5))) + (1e-9)));

        s.store_div_from_scalar_offset_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(337)), (1.0 / p.p220));

        if (0.0 >= s.v[335]) {
            s.store_scalar(336, 0.0);
        } else {
            s.copy_ad(336, 335);
        }

        s.store_add_scaled_product_right_sub(586, 586, 1.0, 336, 773, 622, 1.0 / (var_lgate));

        s.store_scale(166, 586, 1.6021918e-19);

        s.store_scale(636, 166, 1.034943e-10);

        s.store_scale(637, 636, 2.0);

        s.b[1264] = ((s.v[582] <= (2.0 * p.p140)) && (p.p140 > 0.0));
        s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });

        if s.b[1264] {
            s.store_add_scaled_inputs4_indices(587, 585, 2.0, 585, (-(var_lgate * 1.0 / (p.p140))), 622, (-(-(var_lgate * 1.0 / (p.p140)))), 622, -1.0);
            s.store_ln_div(638, 587, 622);
        }

        if (!s.b[1264]) {
            s.store_scalar(638, 0.0);
        }

        s.store_scalar(639, (((((2.0 * 1.6021918e-19) * s.v[494]) * 1.034943e-10)) as f64).sqrt());

        s.store_scalar(640, (1.0 / (s.v[494] * s.v[494])));

        s.store_scalar(641, ((1.0 + (s.v[542] / ((var_lg) as f64).powf(p.p231))) * (1.0 + (p.p238 / ((s.v[581]) as f64).powf(p.p239)))));

        s.store_scaled_ln_scaled_input(158, 586, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(159, 622, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.b[1265] = (p.p51 == 1.0);
        s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });

        if s.b[1265] {
            s.store_scalar(335, (p.p5 + (s.v[163] / (3.0 * p.p4))));
            s.store_scalar(336, (var_lgate - p.p6));
        }

        s.b[1267] = (p.p130 > 0.0);
        s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });

        if s.b[1267] {
            s.store_scalar(644, (p.p130 * p.p2));
            s.store_scalar(648, (p.p130 * p.p3));
        }

        if (!s.b[1267]) {
            s.store_scalar(644, 0.0);
            s.store_scalar(648, 0.0);
        }

        s.b[1268] = (p.p131 > 0.0);
        s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });

        if s.b[1268] {
            s.store_scalar(648, (p.p131 * p.p3));
        }

        if (!s.b[1268]) {
            s.store_scalar(648, 0.0);
        }

        s.b[1269] = (s.v[449] == 0.0);
        s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });

        s.b[1270] = ((s.v[530] > 0.0) || (s.v[540] > 0.0));
        s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });

        if (s.b[1269] && s.b[1270]) {
            s.store_scalar(645, (1.0 + (p.p309 / ((s.v[581]) as f64).powf(p.p310))));
        }

        s.b[1271] = (s.v[538] != 0.0);
        s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });

        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {
            s.store_scalar(341, (1.0 + (p.p303 / ((s.v[581]) as f64).powf(p.p304))));
            s.store_scalar(340, ((-p.p301) * ((var_lg) as f64).powf(p.p302)));
        }

        s.b[1272] = (s.v[340] > 60.0);
        s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });

        if (((s.b[1269] && s.b[1270]) && s.b[1271]) && s.b[1272]) {
            s.store_scalar(340, 60.0);
        }

        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {
            s.store_exp(340, 340);
            s.store_mul(646, 340, 341);
        }

        if ((s.b[1269] && s.b[1270]) && (!s.b[1271])) {
            s.store_scalar(646, 0.0);
        }

        if (s.b[1269] && (!s.b[1270])) {
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
        }

        s.b[1273] = (s.v[532] != 0.0);
        s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });

        if (s.b[1269] && s.b[1273]) {
            s.store_scalar(336, (1.0 + (p.p307 / ((s.v[581]) as f64).powf(p.p308))));
            s.store_scalar(335, ((-p.p305) * ((var_lg) as f64).powf(p.p306)));
        }

        *var_cecox_slot = var_cecox;
        *var_dwcv_slot = var_dwcv;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_novers_slot = var_uc_novers;
        *var_weff_cv_slot = var_weff_cv;
        *var_weffcv_nf_slot = var_weffcv_nf;
    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        var_lg: f64,
        var_lgate: f64,
        var_mfactor: f64,
        var_uc_nover: f64,
        var_weffcv_nf: f64,
        var_wg: f64,
        var_cfrng_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_cfrng: f64 = *var_cfrng_slot;

        s.b[1274] = (s.v[335] > 60.0);
        s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });

        if ((s.b[1269] && s.b[1273]) && s.b[1274]) {
            s.store_scalar(335, 60.0);
        }

        if (s.b[1269] && s.b[1273]) {
            s.store_exp(335, 335);
            s.store_scaled_mul(337, 336, 335, s.v[532]);
            s.store_scaled_add_sqrt_square_offset_rhs(647, 337, 337, ((((4.0 * 1e-6) / 100.0) * 1e-6) / 100.0), 0.5);
        }

        if (s.b[1269] && (!s.b[1273])) {
            s.store_scalar(647, 0.0);
        }

        if s.b[1269] {
            s.store_scalar(649, 0.0);
            s.store_scalar(614, 0.0);
            s.store_scalar(786, 0.0);
            s.store_scalar(652, 0.0);
            s.store_scalar(653, 0.0);
            s.store_scalar(654, 0.0);
        }

        if (!s.b[1269]) {
            s.store_sqrt_square_offset(649, 451, (p.p419 * p.p419));
            s.store_scalar(614, ((((p.p419 * p.p419) + (p.p97 * p.p97))) as f64).sqrt());
            s.store_scalar(786, (1.0 + (p.p424 / ((var_wg) as f64).powf(p.p425))));
            s.store_scalar(652, (1.0 + (p.p426 / ((var_lg) as f64).powf(p.p427))));
            s.store_scalar(653, (1.0 + (p.p428 / ((var_lg) as f64).powf(p.p429))));
            s.store_scalar(654, 1.0);
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
        }

        s.b[1275] = (s.v[459] > 0.0);
        s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });

        if s.b[1275] {
            s.store_scalar(650, ((2.0 * 1.034943e-10) / (1.6021918e-19 * var_uc_nover)));
            s.store_div_scaled_value_offset_denominator(651, s.ad_value(622), (((2.0 * 1.034943e-10) / 1.6021918e-19) * 1.0 / (var_uc_nover)), s.ad_value(622), var_uc_nover, 1.0);
        }

        if (!s.b[1275]) {
            s.store_scalar(650, 0.0);
            s.store_scalar(651, 0.0);
        }

        s.b[1280] = (p.p44 == 0.0);
        s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });

        if s.b[1280] {
            s.store_scalar(335, ((p.p108 * var_lg) + p.p109));
        }

        s.b[1281] = (s.v[335] < 0.0);
        s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });

        if (s.b[1280] && s.b[1281]) {
            s.store_scalar(335, 0.0);
        }

        if s.b[1280] {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), 1.0);
        }

        if (!s.b[1280]) {
            s.store_scalar(335, (p.p108 * var_lg));
        }

        s.b[1282] = (s.v[335] < 0.0);
        s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });

        if ((!s.b[1280]) && s.b[1282]) {
            s.store_scalar(335, 0.0);
        }

        if (!s.b[1280]) {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), ((p.p109) + (1e-25)));
        }

        s.b[1284] = (s.v[658] < 0.1);
        s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });

        if s.b[1284] {
            s.store_scalar(658, 0.1);
        }

        if (p.p23 != 0.0) {
            s.store_scalar(336, ((s.v[163]) as f64).powf(p.p201));
            s.store_div_scaled_value_offset_denominator(659, s.ad_value(336), (s.v[485] * (1.0 + (s.v[547] / ((var_lgate) as f64).powf(p.p199)))), s.ad_value(336), s.v[548], 1.0);
            s.store_scalar(660, (s.v[484] * (1.0 + (s.v[549] / ((var_lgate) as f64).powf(p.p184)))));
            s.store_scalar(661, (s.v[552] * (1.0 + (s.v[550] / ((var_lgate) as f64).powf(p.p203)))));
            s.store_scalar(662, (s.v[481] * (1.0 + (s.v[551] / ((var_lgate) as f64).powf(p.p191)))));
            s.store_scalar(663, (s.v[482] * (1.0 + (s.v[553] / var_lgate))));
            s.copy_ad(668, 662);
            s.copy_ad(669, 663);
            s.copy_ad(665, 659);
            s.copy_ad(666, 660);
            s.copy_ad(667, 661);
        }

        if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
            s.store_scalar(668, (s.v[486] * (1.0 + (s.v[551] / ((var_lgate) as f64).powf(p.p191)))));
            s.store_scalar(669, (s.v[487] * (1.0 + (s.v[553] / var_lgate))));
        }

        if (p.p23 != 0.0) {
            s.store_scalar(664, (p.p72 * (1.0 + (p.p102 / ((var_lg) as f64).powf(p.p103)))));
        }

        if (p.p23 == 0.0) {
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
        }

        s.store_scalar(523, (if (s.v[523] != 0.0) { (s.v[523] * (1.0 + (p.p279 / ((var_lg) as f64).powf(p.p280)))) } else { 0.0 }));

        let assign12850_e7203: f64 = (3.141592653589793 / 2.0);
        let assign12850_e7204: f64 = (3.453133e-11 / assign12850_e7203);
        let assign12850_e7206: f64 = (assign12850_e7204 * var_weffcv_nf);
        let assign12850_e7210: f64 = (p.p225 / p.p95);
        let assign12850_e7211: f64 = (1.0 + assign12850_e7210);
        let assign12850_e7212: f64 = (assign12850_e7211).ln();
        let assign12850_e7213: f64 = (assign12850_e7206 * assign12850_e7212);
        var_cfrng = assign12850_e7213;

        s.store_scalar(671, (if (p.p134 != 0.0) { (((1000000.0 * var_weffcv_nf) * p.p134) / ((var_lg) as f64).powf(p.p135)) } else { 0.0 }));

        s.store_scalar(672, (p.p283 * ((var_lg) as f64).powf((-p.p286))));

        s.store_scalar(673, (p.p290 * ((var_lg) as f64).powf((-p.p291))));

        s.store_scalar(674, (p.p287 * (((var_lg + s.v[777])) as f64).powf((-p.p288))));

        s.store_scalar(766, (((s.v[541] / (var_mfactor * s.v[632])) * (1.0 + (p.p317 / ((var_lg) as f64).powf(p.p318)))) * (1.0 + (p.p315 / ((var_wg) as f64).powf(p.p316)))));

        s.store_scalar(766, (s.v[766] * (1.0 / ((p.p7) as f64).powf(p.p327))));

        s.store_scalar(675, ((((1.0 / ((p.p7) as f64).powf(p.p327)) / (var_mfactor * s.v[632])) * (1.0 + (p.p317 / ((var_lg) as f64).powf(p.p318)))) * (1.0 + (p.p315 / ((var_wg) as f64).powf(p.p316)))));

        s.b[1285] = ((p.p53 == 0.0) || (s.v[541] == 0.0));
        s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });

        if s.b[1285] {
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(387, (ctx_temp + p.p11));
            s.copy_ad(388, 387);
            s.store_offset(387, 387, s.v[732]);
            s.store_offset(389, 388, (-s.v[764]));
            s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));
            s.store_offset(391, 387, (-s.v[764]));
            s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));
            s.store_scale(676, 387, 1.0 / (s.v[764]));
            s.store_ln(590, 676);
            s.store_sub_scaled_ad_lhs(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 392, s.v[456]);
            s.store_sqrt(677, 393);
            s.store_div_from_scalar(335, 1.0, 387);
            s.store_scalar(336, (1.0 / s.v[764]));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p.p260, 336, (-p.p260), A::square(s.ad_value(335)), p.p261, A::square(s.ad_value(336)), (-p.p261), (s.v[616] + p.p259));
            s.store_sqrt(192, 337);
            s.store_mul(193, 337, 192);
            s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);
            s.store_div_from_scalar(155, 1.0, 154);
            s.store_square(156, 154);
            s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));
            s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);
            s.store_exp_scaled_input(335, 590, s.v[480]);
            s.store_div(679, 335, 573);
        }

        s.b[1286] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1286]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1288] = (s.v[973] < 1000.0);
        s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1286]) && s.b[1288]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1285] && s.b[1286]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p.p382);
        }

        s.b[1289] = (s.v[963] == 3.0);
        s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1291] = (s.v[973] < 1000.0);
        s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });

        if (((s.b[1285] && (!s.b[1286])) && s.b[1289]) && s.b[1291]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1285] && (!s.b[1286])) && (!s.b[1289])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1285] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1292] = (p.p39 != 2.0);
        s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1292]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1285] && (!s.b[1292])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1294] = (p.p39 != 2.0);
        s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1294]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
        }

        *var_cfrng_slot = var_cfrng;
    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        var_uc_nover: f64,
        var_uc_novers: f64,
    ) {
        if (s.b[1285] && s.b[1294]) {
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1285] && (!s.b[1294])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1296] = (s.v[682] < 0.0);
        s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1296]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1298] = (s.v[688] < 0.0);
        s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1298]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1300] = (s.v[689] < 0.0);
        s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1300]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1285] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1302] = (s.v[766] < 0.0001);
        s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (p.p53 != 0.0)) && s.b[1302]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1285] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1285] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1285] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1303] = (s.v[963] == 0.0);
        s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1303]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1304] = (s.v[963] == 0.0);
        s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });

        s.b[1305] = (s.v[459] != 0.0);
        s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1304]) && s.b[1305]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(var_uc_nover, s.ad_value(586)));
        }

        s.b[1306] = (s.v[460] != 0.0);
        s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1304]) && s.b[1306]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(var_uc_novers, s.ad_value(586)));
        }

        s.b[1307] = (s.v[459] != 0.0);
        s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (!s.b[1304])) && s.b[1307]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(var_uc_nover, s.ad_value(964)));
        }

        s.b[1308] = (s.v[460] != 0.0);
        s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (!s.b[1304])) && s.b[1308]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(var_uc_novers, s.ad_value(964)));
        }

        s.b[1309] = (s.v[449] == 0.0);
        s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });

        s.b[1310] = (s.v[530] > 0.0);
        s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1309]) && s.b[1310]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1311] = (p.p39 == 1.0);
        s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1310])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1312] = (s.v[540] > 0.0);
        s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1309]) && s.b[1312]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1313] = (p.p39 == 1.0);
        s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1312])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1314] = (s.v[538] > 0.0);
        s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1315] = (s.v[336] < 0.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1315]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1316] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1316]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1316])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1317] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1317]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && (!s.b[1317])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1314])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1285] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1320] = (s.v[957] < 0.0);
        s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1320]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1321] = (s.v[957] > 0.005);
        s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1321]) {
            s.store_scalar(957, 0.005);
        }

        s.b[1322] = (s.v[449] > 0.0);
        s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1322]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }

        if (s.b[1285] && s.b[1322]) {
            s.store_div_from_scalar(794, s.v[569], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));
            s.store_div_from_scalar(795, s.v[570], 334);
            s.store_offset_scaled(959, 387, p.p439, (((((-s.v[764])) * (p.p439))) + (s.v[959])));
        }

        if (s.b[1285] && s.b[1322]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }

        if (s.b[1285] && s.b[1322]) {
            s.store_div_from_scalar(787, s.v[567], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));
            s.store_div_from_scalar(788, s.v[568], 334);
            s.store_offset_scaled(956, 387, p.p438, (((((-s.v[764])) * (p.p438))) + (s.v[956])));
        }

        s.b[1324] = (s.v[956] < 0.1);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1322]) && s.b[1324]) {
            s.store_scalar(956, 0.1);
        }

        if s.b[1285] {
            s.store_square(334, 676);
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);
        }

        s.b[1325] = (p.p48 > 0.0);
        s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });

        s.b[1326] = (p.p15 > s.v[632]);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1325]) && s.b[1326]) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, (p.p15 - s.v[632]));
            s.store_scale(876, 831, (p.p15 - s.v[632]));
            s.store_scale(877, 836, s.v[632]);
            s.store_scale(878, 837, s.v[632]);
        }

        if ((s.b[1285] && s.b[1325]) && (!s.b[1326])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scalar(875, 0.0);
            s.store_scalar(876, 0.0);
            s.store_scale(877, 836, p.p15);
            s.store_scale(878, 837, p.p15);
        }

        if (s.b[1285] && (!s.b[1325])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, p.p15);
            s.store_scale(876, 831, p.p15);
            s.store_scalar(877, 0.0);
            s.store_scalar(878, 0.0);
        }

        if s.b[1285] {
            s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);
        }

        s.b[1327] = (s.v[847] > 0.0);
        s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1327]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));
            s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);
            s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));
            s.store_exp_mul(851, 848, 850);
        }

        if s.b[1285] {
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);
        }

        s.b[1328] = (p.p48 > 0.0);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        s.b[1329] = (p.p16 > s.v[632]);
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1328]) && s.b[1329]) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, (p.p16 - s.v[632]));
            s.store_scale(882, 831, (p.p16 - s.v[632]));
            s.store_scale(883, 836, s.v[632]);
            s.store_scale(884, 837, s.v[632]);
        }

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_vdsei_slot: &mut f64,
        var_vdsei_db0_slot: &mut f64,
        var_vdsei_db1_slot: &mut f64,
        var_vdsei_db10_slot: &mut f64,
        var_vdsei_db11_slot: &mut f64,
        var_vdsei_db2_slot: &mut f64,
        var_vdsei_db3_slot: &mut f64,
        var_vdsei_db4_slot: &mut f64,
        var_vdsei_db5_slot: &mut f64,
        var_vdsei_db6_slot: &mut f64,
        var_vdsei_db7_slot: &mut f64,
        var_vdsei_db8_slot: &mut f64,
        var_vdsei_db9_slot: &mut f64,
        var_vdsei_dn0_slot: &mut f64,
        var_vdsei_dn1_slot: &mut f64,
        var_vdsei_dn10_slot: &mut f64,
        var_vdsei_dn11_slot: &mut f64,
        var_vdsei_dn12_slot: &mut f64,
        var_vdsei_dn13_slot: &mut f64,
        var_vdsei_dn14_slot: &mut f64,
        var_vdsei_dn15_slot: &mut f64,
        var_vdsei_dn16_slot: &mut f64,
        var_vdsei_dn17_slot: &mut f64,
        var_vdsei_dn2_slot: &mut f64,
        var_vdsei_dn3_slot: &mut f64,
        var_vdsei_dn4_slot: &mut f64,
        var_vdsei_dn5_slot: &mut f64,
        var_vdsei_dn6_slot: &mut f64,
        var_vdsei_dn7_slot: &mut f64,
        var_vdsei_dn8_slot: &mut f64,
        var_vdsei_dn9_slot: &mut f64,
        var_vgsei_slot: &mut f64,
        var_vgsei_db0_slot: &mut f64,
        var_vgsei_db1_slot: &mut f64,
        var_vgsei_db10_slot: &mut f64,
        var_vgsei_db11_slot: &mut f64,
        var_vgsei_db2_slot: &mut f64,
        var_vgsei_db3_slot: &mut f64,
        var_vgsei_db4_slot: &mut f64,
        var_vgsei_db5_slot: &mut f64,
        var_vgsei_db6_slot: &mut f64,
        var_vgsei_db7_slot: &mut f64,
        var_vgsei_db8_slot: &mut f64,
        var_vgsei_db9_slot: &mut f64,
        var_vgsei_dn0_slot: &mut f64,
        var_vgsei_dn1_slot: &mut f64,
        var_vgsei_dn10_slot: &mut f64,
        var_vgsei_dn11_slot: &mut f64,
        var_vgsei_dn12_slot: &mut f64,
        var_vgsei_dn13_slot: &mut f64,
        var_vgsei_dn14_slot: &mut f64,
        var_vgsei_dn15_slot: &mut f64,
        var_vgsei_dn16_slot: &mut f64,
        var_vgsei_dn17_slot: &mut f64,
        var_vgsei_dn2_slot: &mut f64,
        var_vgsei_dn3_slot: &mut f64,
        var_vgsei_dn4_slot: &mut f64,
        var_vgsei_dn5_slot: &mut f64,
        var_vgsei_dn6_slot: &mut f64,
        var_vgsei_dn7_slot: &mut f64,
        var_vgsei_dn8_slot: &mut f64,
        var_vgsei_dn9_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_vdsei: f64 = *var_vdsei_slot;
        let mut var_vdsei_db0: f64 = *var_vdsei_db0_slot;
        let mut var_vdsei_db1: f64 = *var_vdsei_db1_slot;
        let mut var_vdsei_db10: f64 = *var_vdsei_db10_slot;
        let mut var_vdsei_db11: f64 = *var_vdsei_db11_slot;
        let mut var_vdsei_db2: f64 = *var_vdsei_db2_slot;
        let mut var_vdsei_db3: f64 = *var_vdsei_db3_slot;
        let mut var_vdsei_db4: f64 = *var_vdsei_db4_slot;
        let mut var_vdsei_db5: f64 = *var_vdsei_db5_slot;
        let mut var_vdsei_db6: f64 = *var_vdsei_db6_slot;
        let mut var_vdsei_db7: f64 = *var_vdsei_db7_slot;
        let mut var_vdsei_db8: f64 = *var_vdsei_db8_slot;
        let mut var_vdsei_db9: f64 = *var_vdsei_db9_slot;
        let mut var_vdsei_dn0: f64 = *var_vdsei_dn0_slot;
        let mut var_vdsei_dn1: f64 = *var_vdsei_dn1_slot;
        let mut var_vdsei_dn10: f64 = *var_vdsei_dn10_slot;
        let mut var_vdsei_dn11: f64 = *var_vdsei_dn11_slot;
        let mut var_vdsei_dn12: f64 = *var_vdsei_dn12_slot;
        let mut var_vdsei_dn13: f64 = *var_vdsei_dn13_slot;
        let mut var_vdsei_dn14: f64 = *var_vdsei_dn14_slot;
        let mut var_vdsei_dn15: f64 = *var_vdsei_dn15_slot;
        let mut var_vdsei_dn16: f64 = *var_vdsei_dn16_slot;
        let mut var_vdsei_dn17: f64 = *var_vdsei_dn17_slot;
        let mut var_vdsei_dn2: f64 = *var_vdsei_dn2_slot;
        let mut var_vdsei_dn3: f64 = *var_vdsei_dn3_slot;
        let mut var_vdsei_dn4: f64 = *var_vdsei_dn4_slot;
        let mut var_vdsei_dn5: f64 = *var_vdsei_dn5_slot;
        let mut var_vdsei_dn6: f64 = *var_vdsei_dn6_slot;
        let mut var_vdsei_dn7: f64 = *var_vdsei_dn7_slot;
        let mut var_vdsei_dn8: f64 = *var_vdsei_dn8_slot;
        let mut var_vdsei_dn9: f64 = *var_vdsei_dn9_slot;
        let mut var_vgsei: f64 = *var_vgsei_slot;
        let mut var_vgsei_db0: f64 = *var_vgsei_db0_slot;
        let mut var_vgsei_db1: f64 = *var_vgsei_db1_slot;
        let mut var_vgsei_db10: f64 = *var_vgsei_db10_slot;
        let mut var_vgsei_db11: f64 = *var_vgsei_db11_slot;
        let mut var_vgsei_db2: f64 = *var_vgsei_db2_slot;
        let mut var_vgsei_db3: f64 = *var_vgsei_db3_slot;
        let mut var_vgsei_db4: f64 = *var_vgsei_db4_slot;
        let mut var_vgsei_db5: f64 = *var_vgsei_db5_slot;
        let mut var_vgsei_db6: f64 = *var_vgsei_db6_slot;
        let mut var_vgsei_db7: f64 = *var_vgsei_db7_slot;
        let mut var_vgsei_db8: f64 = *var_vgsei_db8_slot;
        let mut var_vgsei_db9: f64 = *var_vgsei_db9_slot;
        let mut var_vgsei_dn0: f64 = *var_vgsei_dn0_slot;
        let mut var_vgsei_dn1: f64 = *var_vgsei_dn1_slot;
        let mut var_vgsei_dn10: f64 = *var_vgsei_dn10_slot;
        let mut var_vgsei_dn11: f64 = *var_vgsei_dn11_slot;
        let mut var_vgsei_dn12: f64 = *var_vgsei_dn12_slot;
        let mut var_vgsei_dn13: f64 = *var_vgsei_dn13_slot;
        let mut var_vgsei_dn14: f64 = *var_vgsei_dn14_slot;
        let mut var_vgsei_dn15: f64 = *var_vgsei_dn15_slot;
        let mut var_vgsei_dn16: f64 = *var_vgsei_dn16_slot;
        let mut var_vgsei_dn17: f64 = *var_vgsei_dn17_slot;
        let mut var_vgsei_dn2: f64 = *var_vgsei_dn2_slot;
        let mut var_vgsei_dn3: f64 = *var_vgsei_dn3_slot;
        let mut var_vgsei_dn4: f64 = *var_vgsei_dn4_slot;
        let mut var_vgsei_dn5: f64 = *var_vgsei_dn5_slot;
        let mut var_vgsei_dn6: f64 = *var_vgsei_dn6_slot;
        let mut var_vgsei_dn7: f64 = *var_vgsei_dn7_slot;
        let mut var_vgsei_dn8: f64 = *var_vgsei_dn8_slot;
        let mut var_vgsei_dn9: f64 = *var_vgsei_dn9_slot;

        if ((s.b[1285] && s.b[1328]) && (!s.b[1329])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scalar(881, 0.0);
            s.store_scalar(882, 0.0);
            s.store_scale(883, 836, p.p16);
            s.store_scale(884, 837, p.p16);
        }

        if (s.b[1285] && (!s.b[1328])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, p.p16);
            s.store_scale(882, 831, p.p16);
            s.store_scalar(883, 0.0);
            s.store_scalar(884, 0.0);
        }

        if s.b[1285] {
            s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);
        }

        s.b[1330] = (s.v[852] > 0.0);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1330]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));
            s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);
            s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));
            s.store_exp_mul(856, 853, 855);
        }

        if s.b[1285] {
            s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));
        }

        s.b[1331] = (p.p15 > s.v[632]);
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1331]) {
            s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));
        }

        if (s.b[1285] && (!s.b[1331])) {
            s.store_scalar(833, 0.0);
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));
        }

        s.b[1332] = (s.v[832] < 0.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1332]) {
            s.store_scalar(832, 0.0);
        }

        s.b[1333] = (s.v[833] < 0.0);
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1333]) {
            s.store_scalar(833, 0.0);
        }

        s.b[1334] = (s.v[834] < 0.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1334]) {
            s.store_scalar(834, 0.0);
        }

        if s.b[1285] {
            s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);
            s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);
            s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);
        }

        s.b[1335] = ((s.v[841] < 0.01) && (p.p13 > 0.0));
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1335]) {
            s.store_scalar(841, 0.01);
        }

        s.b[1336] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1336]) {
            s.store_scalar(842, 0.01);
        }

        s.b[1337] = ((s.v[843] < 0.01) && (p.p15 > 0.0));
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1337]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1285] {
            s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));
        }

        s.b[1338] = (p.p16 > s.v[632]);
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1338]) {
            s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));
        }

        if (s.b[1285] && (!s.b[1338])) {
            s.store_scalar(838, 0.0);
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));
        }

        s.b[1339] = (s.v[835] < 0.0);
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1339]) {
            s.store_scalar(835, 0.0);
        }

        s.b[1340] = (s.v[838] < 0.0);
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1340]) {
            s.store_scalar(838, 0.0);
        }

        s.b[1341] = (s.v[839] < 0.0);
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1341]) {
            s.store_scalar(839, 0.0);
        }

        if s.b[1285] {
            s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);
            s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);
            s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);
        }

        s.b[1342] = ((s.v[844] < 0.01) && (p.p14 > 0.0));
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1342]) {
            s.store_scalar(844, 0.01);
        }

        s.b[1343] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1343]) {
            s.store_scalar(845, 0.01);
        }

        s.b[1344] = ((s.v[846] < 0.01) && (p.p16 > 0.0));
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1344]) {
            s.store_scalar(846, 0.01);
        }

        s.store_scaled_voltage(729, ctx, nodes, Some(5), Some(7), p.p87);

        s.store_scaled_voltage(731, ctx, nodes, Some(6), Some(7), p.p87);

        s.store_scaled_voltage(728, ctx, nodes, Some(8), Some(7), p.p87);

        let assign16660_e11450: f64 = (p.p87 * (nv0 - nv2));
        var_vdsei = assign16660_e11450;
        var_vdsei_dn0 = p.p87;
        var_vdsei_dn1 = 0.0;
        var_vdsei_dn2 = (-p.p87);
        var_vdsei_dn3 = 0.0;
        var_vdsei_dn4 = 0.0;
        var_vdsei_dn5 = 0.0;
        var_vdsei_dn6 = 0.0;
        var_vdsei_dn7 = 0.0;
        var_vdsei_dn8 = 0.0;
        var_vdsei_dn9 = 0.0;
        var_vdsei_dn10 = 0.0;
        var_vdsei_dn11 = 0.0;
        var_vdsei_dn12 = 0.0;
        var_vdsei_dn13 = 0.0;
        var_vdsei_dn14 = 0.0;
        var_vdsei_dn15 = 0.0;
        var_vdsei_dn16 = 0.0;
        var_vdsei_dn17 = 0.0;
        var_vdsei_db0 = 0.0;
        var_vdsei_db1 = 0.0;
        var_vdsei_db2 = 0.0;
        var_vdsei_db3 = 0.0;
        var_vdsei_db4 = 0.0;
        var_vdsei_db5 = 0.0;
        var_vdsei_db6 = 0.0;
        var_vdsei_db7 = 0.0;
        var_vdsei_db8 = 0.0;
        var_vdsei_db9 = 0.0;
        var_vdsei_db10 = 0.0;
        var_vdsei_db11 = 0.0;

        let assign16670_e11453: f64 = (p.p87 * (nv6 - nv2));
        var_vgsei = assign16670_e11453;
        var_vgsei_dn0 = 0.0;
        var_vgsei_dn1 = 0.0;
        var_vgsei_dn2 = (-p.p87);
        var_vgsei_dn3 = 0.0;
        var_vgsei_dn4 = 0.0;
        var_vgsei_dn5 = 0.0;
        var_vgsei_dn6 = p.p87;
        var_vgsei_dn7 = 0.0;
        var_vgsei_dn8 = 0.0;
        var_vgsei_dn9 = 0.0;
        var_vgsei_dn10 = 0.0;
        var_vgsei_dn11 = 0.0;
        var_vgsei_dn12 = 0.0;
        var_vgsei_dn13 = 0.0;
        var_vgsei_dn14 = 0.0;
        var_vgsei_dn15 = 0.0;
        var_vgsei_dn16 = 0.0;
        var_vgsei_dn17 = 0.0;
        var_vgsei_db0 = 0.0;
        var_vgsei_db1 = 0.0;
        var_vgsei_db2 = 0.0;
        var_vgsei_db3 = 0.0;
        var_vgsei_db4 = 0.0;
        var_vgsei_db5 = 0.0;
        var_vgsei_db6 = 0.0;
        var_vgsei_db7 = 0.0;
        var_vgsei_db8 = 0.0;
        var_vgsei_db9 = 0.0;
        var_vgsei_db10 = 0.0;
        var_vgsei_db11 = 0.0;

        s.store_scaled_voltage(735, ctx, nodes, Some(8), Some(2), p.p87);

        s.store_scaled_voltage(799, ctx, nodes, Some(0), Some(5), p.p87);

        s.store_scaled_voltage(804, ctx, nodes, Some(7), Some(2), p.p87);

        s.store_scaled_voltage(857, ctx, nodes, Some(10), Some(2), p.p87);

        s.store_scaled_voltage(858, ctx, nodes, Some(9), Some(0), p.p87);

        s.store_scaled_voltage(865, ctx, nodes, Some(8), Some(7), p.p87);

        s.store_scaled_voltage(866, ctx, nodes, Some(8), Some(5), p.p87);

        s.copy_ad(859, 857);

        s.copy_ad(860, 858);

        s.copy_ad(867, 865);

        s.copy_ad(868, 866);

        s.store_scalar(798, 0.0);

        if (s.v[81] != 0.0) {
            s.store_voltage(747, ctx, nodes, Some(11), None);
            s.store_voltage(748, ctx, nodes, Some(12), None);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(747, 0.0);
            s.store_scalar(748, 0.0);
        }

        s.store_sub(730, 731, 729);

        s.store_sub(727, 728, 729);

        s.b[1345] = (s.v[729] >= 0.0);
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if s.b[1345] {
            s.store_scalar(949, 1.0);
            s.copy_ad(790, 729);
            s.copy_ad(791, 731);
            s.copy_ad(792, 728);
            s.store_ad_value(793, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]));
            s.store_ad_value(796, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]));
            s.copy_ad(797, 735);
        }

        if (!s.b[1345]) {
            s.store_scalar(949, (-1.0));
            s.store_neg(790, 729);
            s.copy_ad(791, 730);
            s.copy_ad(792, 727);
            s.store_neg_ad(793, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]));
            s.store_sub_ad(796, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]), A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]));
            s.store_sub_ad_rhs(797, 735, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]));
        }

        s.b[1348] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });

        if s.b[1348] {
            s.store_voltage(732, ctx, nodes, Some(4), None);
        }

        s.b[1349] = (p.p53 == 2.0);
        s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1349]) {
            s.store_offset_sub_from_scalar_ad(781, p.p433, s.ad_value(732), (-(p.p337 * 10.0)));
            s.store_scalar(782, ((4.0 * p.p433) * (p.p337 * 10.0)));
        }

        if (s.b[1348] && s.b[1349]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1348] && s.b[1349]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(732, 781, (-0.5), 782, (-0.5), p.p433);
        }

        if s.b[1348] {
            s.store_scalar(387, (ctx_temp + p.p11));
            s.copy_ad(388, 387);
            s.store_add(387, 387, 732);
            s.store_offset(389, 388, (-s.v[764]));
            s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));
            s.store_offset(391, 387, (-s.v[764]));
            s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));
            s.store_scale(676, 387, 1.0 / (s.v[764]));
            s.store_ln(590, 676);
            s.store_sub_scaled_ad_lhs(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 392, s.v[456]);
            s.store_sqrt(677, 393);
            s.store_div_from_scalar(335, 1.0, 387);
            s.store_scalar(336, (1.0 / s.v[764]));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p.p260, 336, (-p.p260), A::square(s.ad_value(335)), p.p261, A::square(s.ad_value(336)), (-p.p261), (s.v[616] + p.p259));
            s.store_sqrt(192, 337);
            s.store_mul(193, 337, 192);
            s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);
        }

        *var_vdsei_slot = var_vdsei;
        *var_vdsei_db0_slot = var_vdsei_db0;
        *var_vdsei_db1_slot = var_vdsei_db1;
        *var_vdsei_db10_slot = var_vdsei_db10;
        *var_vdsei_db11_slot = var_vdsei_db11;
        *var_vdsei_db2_slot = var_vdsei_db2;
        *var_vdsei_db3_slot = var_vdsei_db3;
        *var_vdsei_db4_slot = var_vdsei_db4;
        *var_vdsei_db5_slot = var_vdsei_db5;
        *var_vdsei_db6_slot = var_vdsei_db6;
        *var_vdsei_db7_slot = var_vdsei_db7;
        *var_vdsei_db8_slot = var_vdsei_db8;
        *var_vdsei_db9_slot = var_vdsei_db9;
        *var_vdsei_dn0_slot = var_vdsei_dn0;
        *var_vdsei_dn1_slot = var_vdsei_dn1;
        *var_vdsei_dn10_slot = var_vdsei_dn10;
        *var_vdsei_dn11_slot = var_vdsei_dn11;
        *var_vdsei_dn12_slot = var_vdsei_dn12;
        *var_vdsei_dn13_slot = var_vdsei_dn13;
        *var_vdsei_dn14_slot = var_vdsei_dn14;
        *var_vdsei_dn15_slot = var_vdsei_dn15;
        *var_vdsei_dn16_slot = var_vdsei_dn16;
        *var_vdsei_dn17_slot = var_vdsei_dn17;
        *var_vdsei_dn2_slot = var_vdsei_dn2;
        *var_vdsei_dn3_slot = var_vdsei_dn3;
        *var_vdsei_dn4_slot = var_vdsei_dn4;
        *var_vdsei_dn5_slot = var_vdsei_dn5;
        *var_vdsei_dn6_slot = var_vdsei_dn6;
        *var_vdsei_dn7_slot = var_vdsei_dn7;
        *var_vdsei_dn8_slot = var_vdsei_dn8;
        *var_vdsei_dn9_slot = var_vdsei_dn9;
        *var_vgsei_slot = var_vgsei;
        *var_vgsei_db0_slot = var_vgsei_db0;
        *var_vgsei_db1_slot = var_vgsei_db1;
        *var_vgsei_db10_slot = var_vgsei_db10;
        *var_vgsei_db11_slot = var_vgsei_db11;
        *var_vgsei_db2_slot = var_vgsei_db2;
        *var_vgsei_db3_slot = var_vgsei_db3;
        *var_vgsei_db4_slot = var_vgsei_db4;
        *var_vgsei_db5_slot = var_vgsei_db5;
        *var_vgsei_db6_slot = var_vgsei_db6;
        *var_vgsei_db7_slot = var_vgsei_db7;
        *var_vgsei_db8_slot = var_vgsei_db8;
        *var_vgsei_db9_slot = var_vgsei_db9;
        *var_vgsei_dn0_slot = var_vgsei_dn0;
        *var_vgsei_dn1_slot = var_vgsei_dn1;
        *var_vgsei_dn10_slot = var_vgsei_dn10;
        *var_vgsei_dn11_slot = var_vgsei_dn11;
        *var_vgsei_dn12_slot = var_vgsei_dn12;
        *var_vgsei_dn13_slot = var_vgsei_dn13;
        *var_vgsei_dn14_slot = var_vgsei_dn14;
        *var_vgsei_dn15_slot = var_vgsei_dn15;
        *var_vgsei_dn16_slot = var_vgsei_dn16;
        *var_vgsei_dn17_slot = var_vgsei_dn17;
        *var_vgsei_dn2_slot = var_vgsei_dn2;
        *var_vgsei_dn3_slot = var_vgsei_dn3;
        *var_vgsei_dn4_slot = var_vgsei_dn4;
        *var_vgsei_dn5_slot = var_vgsei_dn5;
        *var_vgsei_dn6_slot = var_vgsei_dn6;
        *var_vgsei_dn7_slot = var_vgsei_dn7;
        *var_vgsei_dn8_slot = var_vgsei_dn8;
        *var_vgsei_dn9_slot = var_vgsei_dn9;
    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
        var_uc_nover: f64,
        var_uc_novers: f64,
    ) {
        if s.b[1348] {
            s.store_div_from_scalar(155, 1.0, 154);
            s.store_square(156, 154);
            s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));
            s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);
            s.store_exp_scaled_input(335, 590, s.v[480]);
            s.store_div(679, 335, 573);
        }

        s.b[1351] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1351]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1353] = (s.v[973] < 1000.0);
        s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });

        if ((s.b[1348] && s.b[1351]) && s.b[1353]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1348] && s.b[1351]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_ad_rhs(970, 970, A::powf(s.ad_value(676), p.p382));
        }

        s.b[1354] = (s.v[963] == 3.0);
        s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });

        if ((s.b[1348] && (!s.b[1351])) && s.b[1354]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1356] = (s.v[973] < 1000.0);
        s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });

        if (((s.b[1348] && (!s.b[1351])) && s.b[1354]) && s.b[1356]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1348] && (!s.b[1351])) && s.b[1354]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1348] && (!s.b[1351])) && (!s.b[1354])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1348] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1357] = (p.p39 != 2.0);
        s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1357]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1348] && (!s.b[1357])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1359] = (p.p39 != 2.0);
        s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1359]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1348] && (!s.b[1359])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1361] = (s.v[682] < 0.0);
        s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1361]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1363] = (s.v[688] < 0.0);
        s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1363]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1365] = (s.v[689] < 0.0);
        s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1365]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1348] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1367] = (s.v[766] < 0.0001);
        s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });

        if ((s.b[1348] && (p.p53 != 0.0)) && s.b[1367]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1348] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1348] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1348] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1348] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1348] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1368] = (s.v[963] == 0.0);
        s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1368]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1369] = (s.v[963] == 0.0);
        s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });

        s.b[1370] = (s.v[459] != 0.0);
        s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });

        if ((s.b[1348] && s.b[1369]) && s.b[1370]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(var_uc_nover, s.ad_value(586)));
        }

        s.b[1371] = (s.v[460] != 0.0);
        s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });

        if ((s.b[1348] && s.b[1369]) && s.b[1371]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(var_uc_novers, s.ad_value(586)));
        }

        s.b[1372] = (s.v[459] != 0.0);
        s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });

        if ((s.b[1348] && (!s.b[1369])) && s.b[1372]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(var_uc_nover, s.ad_value(964)));
        }

        s.b[1373] = (s.v[460] != 0.0);
        s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });

        if ((s.b[1348] && (!s.b[1369])) && s.b[1373]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(var_uc_novers, s.ad_value(964)));
        }

        s.b[1374] = (s.v[449] == 0.0);
        s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });

        s.b[1375] = (s.v[530] > 0.0);
        s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });

        if ((s.b[1348] && s.b[1374]) && s.b[1375]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1376] = (p.p39 == 1.0);
        s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && s.b[1376]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1375]) && (!s.b[1376])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1348] && s.b[1374]) && (!s.b[1375])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1377] = (s.v[540] > 0.0);
        s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });

        if ((s.b[1348] && s.b[1374]) && s.b[1377]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1378] = (p.p39 == 1.0);
        s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && s.b[1378]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_cecox: f64,
        var_uc_toxb: f64,
        var_c_eox_slot: &mut f64,
        var_cox0_slot: &mut f64,
        var_coxb0_slot: &mut f64,
        var_tox0_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_c_eox: f64 = *var_c_eox_slot;
        let mut var_cox0: f64 = *var_cox0_slot;
        let mut var_coxb0: f64 = *var_coxb0_slot;
        let mut var_tox0: f64 = *var_tox0_slot;

        if (((s.b[1348] && s.b[1374]) && s.b[1377]) && (!s.b[1378])) {
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1348] && s.b[1374]) && (!s.b[1377])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1379] = (s.v[538] > 0.0);
        s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1380] = (s.v[336] < 0.0);
        s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1380]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1381] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1381]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1381])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1348] && s.b[1374]) && s.b[1379]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1382] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && s.b[1382]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1348] && s.b[1374]) && s.b[1379]) && (!s.b[1382])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1348] && s.b[1374]) && (!s.b[1379])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1348] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1385] = (s.v[957] < 0.0);
        s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1385]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1386] = (s.v[957] > 0.005);
        s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });

        if (s.b[1348] && s.b[1386]) {
            s.store_scalar(957, 0.005);
        }

        if (!s.b[1348]) {
            s.store_scalar(387, (ctx_temp + p.p11));
        }

        s.store_scalar(164, (s.v[630] * p.p7));

        s.store_scalar(165, (p.p67 + p.p68));

        s.store_scalar(160, s.v[462]);

        s.copy_ad(257, 681);

        var_c_eox = var_cecox;

        var_tox0 = p.p95;

        let assign19630_e14619: f64 = (var_c_eox / var_tox0);
        var_cox0 = assign19630_e14619;

        s.store_scalar(189, (1.0 / var_cox0));

        let assign19650_e14625: f64 = (var_c_eox / var_uc_toxb);
        var_coxb0 = assign19650_e14625;

        s.store_scalar(270, (p.p87 * p.p434));

        s.store_offset_sub_from_scalar_ad(781, 0.8, A::offset(s.ad_value(157), (-p.p262)), (-0.1));

        s.store_scalar(782, ((4.0 * 0.8) * 0.1));

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_square_add(782, 781, 782);

        s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(335, 781, (-0.5), 782, (-0.5), 0.8);

        s.copy_ad(69, 335);

        s.b[1387] = ((s.v[158] - p.p262) < s.v[69]);
        s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });

        if s.b[1387] {
            s.store_offset(69, 158, (-p.p262));
        }

        s.b[1388] = ((s.v[159] - p.p262) < s.v[69]);
        s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });

        if s.b[1388] {
            s.store_offset(69, 159, (-p.p262));
        }

        s.b[1389] = ((s.v[963] > 0.0) && (s.v[963] <= 3.0));
        s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });

        s.b[1390] = ((s.v[961] - p.p262) < s.v[69]);
        s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });

        if (s.b[1389] && s.b[1390]) {
            s.store_offset(69, 961, (-p.p262));
        }

        s.b[1391] = ((s.v[960] - p.p262) < s.v[69]);
        s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });

        if (s.b[1389] && s.b[1391]) {
            s.store_offset(69, 960, (-p.p262));
        }

        s.b[1392] = (s.v[70] > (s.v[69] * 0.5));
        s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });

        if s.b[1392] {
            s.store_scale(70, 69, 0.5);
        }

        s.b[1393] = param_given[338];
        s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });

        *var_c_eox_slot = var_c_eox;
        *var_cox0_slot = var_cox0;
        *var_coxb0_slot = var_coxb0;
        *var_tox0_slot = var_tox0;
    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_uc_nover: f64,
        var_vdsei: f64,
        var_vdsei_db0: f64,
        var_vdsei_db1: f64,
        var_vdsei_db10: f64,
        var_vdsei_db11: f64,
        var_vdsei_db2: f64,
        var_vdsei_db3: f64,
        var_vdsei_db4: f64,
        var_vdsei_db5: f64,
        var_vdsei_db6: f64,
        var_vdsei_db7: f64,
        var_vdsei_db8: f64,
        var_vdsei_db9: f64,
        var_vdsei_dn0: f64,
        var_vdsei_dn1: f64,
        var_vdsei_dn10: f64,
        var_vdsei_dn11: f64,
        var_vdsei_dn12: f64,
        var_vdsei_dn13: f64,
        var_vdsei_dn14: f64,
        var_vdsei_dn15: f64,
        var_vdsei_dn16: f64,
        var_vdsei_dn17: f64,
        var_vdsei_dn2: f64,
        var_vdsei_dn3: f64,
        var_vdsei_dn4: f64,
        var_vdsei_dn5: f64,
        var_vdsei_dn6: f64,
        var_vdsei_dn7: f64,
        var_vdsei_dn8: f64,
        var_vdsei_dn9: f64,
        var_vgsei: f64,
        var_vgsei_db0: f64,
        var_vgsei_db1: f64,
        var_vgsei_db10: f64,
        var_vgsei_db11: f64,
        var_vgsei_db2: f64,
        var_vgsei_db3: f64,
        var_vgsei_db4: f64,
        var_vgsei_db5: f64,
        var_vgsei_db6: f64,
        var_vgsei_db7: f64,
        var_vgsei_db8: f64,
        var_vgsei_db9: f64,
        var_vgsei_dn0: f64,
        var_vgsei_dn1: f64,
        var_vgsei_dn10: f64,
        var_vgsei_dn11: f64,
        var_vgsei_dn12: f64,
        var_vgsei_dn13: f64,
        var_vgsei_dn14: f64,
        var_vgsei_dn15: f64,
        var_vgsei_dn16: f64,
        var_vgsei_dn17: f64,
        var_vgsei_dn2: f64,
        var_vgsei_dn3: f64,
        var_vgsei_dn4: f64,
        var_vgsei_dn5: f64,
        var_vgsei_dn6: f64,
        var_vgsei_dn7: f64,
        var_vgsei_dn8: f64,
        var_vgsei_dn9: f64,
    ) {
        if s.b[1393] {
            s.store_scalar(72, p.p338);
        }

        if (!s.b[1393]) {
            s.copy_ad(72, 69);
        }

        s.b[1394] = param_given[339];
        s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });

        if s.b[1394] {
            s.store_scalar(73, p.p339);
        }

        s.b[1395] = param_given[338];
        s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });

        if ((!s.b[1394]) && s.b[1395]) {
            s.store_scale(73, 72, 0.5);
        }

        if ((!s.b[1394]) && (!s.b[1395])) {
            s.copy_ad(73, 70);
        }

        s.b[1396] = (s.v[73] > (s.v[72] * 0.5));
        s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });

        if s.b[1396] {
            s.store_scale(73, 72, 0.5);
        }

        s.b[1397] = ((s.v[691] > 0.0) || (s.v[690] > 0.0));
        s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });

        s.b[1398] = (s.v[448] == 1.0);
        s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });

        let (assign19970_e14797,) = {
    if (s.b[1397] && s.b[1398]) {
        (1.0,)
    } else {
        (s.v[74],)
    }
};
        s.store_scalar(74, assign19970_e14797);

        s.b[1399] = (s.v[448] == 2.0);
        s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });

        let (assign19990_e14806,) = {
    if (s.b[1397] && s.b[1399]) {
        (2.0,)
    } else {
        (s.v[74],)
    }
};
        s.store_scalar(74, assign19990_e14806);

        s.b[1400] = (s.v[448] == 3.0);
        s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });

        let (assign20010_e14815,) = {
    if (s.b[1397] && s.b[1400]) {
        (3.0,)
    } else {
        (s.v[74],)
    }
};
        s.store_scalar(74, assign20010_e14815);

        s.store_scalar(77, 0.0);

        s.b[1401] = (((s.v[449] == 1.0) && (p.p54 == 1.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });

        if s.b[1401] {
            s.store_ad_value(373, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]));
        }

        s.b[1402] = (s.v[373] >= 0.0);
        s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });

        if (s.b[1401] && s.b[1402]) {
            s.copy_ad(376, 373);
            s.store_scalar(383, s.v[798]);
        }

        if (s.b[1401] && (!s.b[1402])) {
            s.store_neg(376, 373);
            s.store_sub_from_scalar(383, s.v[798], 373);
        }

        if s.b[1401] {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1403] = (s.v[108] < 1e-12);
        s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });

        if (s.b[1401] && s.b[1403]) {
            s.store_scalar(108, 1e-12);
        }

        if s.b[1401] {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1404] = (s.v[335] < 0.0);
        s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });

        if (s.b[1401] && s.b[1404]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if s.b[1401] {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (var_uc_nover * (s.v[544] + var_uc_nover))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if s.b[1401] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1401] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
        }

        if (!s.b[1401]) {
            s.store_scalar(384, 0.0);
        }

        s.b[1405] = ((s.v[74] == 1.0) || (s.v[74] == 3.0));
        s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });

        if s.b[1405] {
            s.store_ad_value(373, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]));
            s.store_ad_value(374, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]));
            s.copy_ad(372, 735);
        }

        s.b[1406] = (s.v[373] >= 0.0);
        s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });

        if (s.b[1405] && s.b[1406]) {
            s.store_scalar(370, 1.0);
            s.store_scalar(371, 0.0);
            s.copy_ad(376, 373);
            s.copy_ad(377, 374);
            s.copy_ad(375, 372);
            s.store_scalar(383, s.v[798]);
        }

        if (s.b[1405] && (!s.b[1406])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 1.0);
            s.store_neg(376, 373);
            s.store_sub(377, 374, 373);
            s.store_sub(375, 372, 373);
            s.store_sub_from_scalar(383, s.v[798], 373);
        }

        s.b[1407] = (((((s.v[692] > 0.0) || (s.v[693] > 0.0)) || (s.v[539] > 0.0)) || (s.v[537] > 0.0)) || (p.p54 == 1.0));
        s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });

        if (s.b[1405] && s.b[1407]) {
            s.store_scale(781, 376, (0.5 * (2.0 * 1.0 / (p.p262))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(108, p.p262, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[1408] = (s.v[108] < 1e-12);
        s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });

        if ((s.b[1405] && s.b[1407]) && s.b[1408]) {
            s.store_scalar(108, 1e-12);
        }

        if (s.b[1405] && s.b[1407]) {
            s.store_add_scaled_inputs(380, 376, 1.0, 108, 2.0);
            s.store_add(381, 377, 108);
            s.store_add(382, 375, 108);
        }

        s.b[1409] = ((p.p34 == 1.0) || (s.v[370] == 1.0));
        s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_add_scaled_products_indices(335, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(334, 370, 692, 1.0, 371, 693, 1.0);
            s.store_add_scaled_product_indices(338, 335, 1.0, 334, 380, 1.0);
            s.store_scalar(782, ((((p.p292 * p.p292) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p292, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(344, 782, p.p292, 0.5);
        }

        s.b[1410] = (s.v[344] < 0.0);
        s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });

        if (((s.b[1405] && s.b[1407]) && s.b[1409]) && s.b[1410]) {
            s.store_scalar(344, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(366, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1411] = (s.v[337] < 0.0);
        s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });

        if (((s.b[1405] && s.b[1407]) && s.b[1409]) && s.b[1411]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1409]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 366);
            s.store_mul(366, 366, 337);
        }

        if ((s.b[1405] && s.b[1407]) && (!s.b[1409])) {
            s.copy_ad(366, 691);
        }

        if (s.b[1405] && s.b[1407]) {
            s.store_add_scaled_products_indices(338, 370, 691, 1.0, 371, 690, 1.0);
        }

        s.b[1412] = ((p.p34 == 1.0) || (s.v[371] == 1.0));
        s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_add_scaled_products_indices(334, 370, 693, 1.0, 371, 692, 1.0);
            s.store_add_scaled_inputs(338, 338, 1.0, 334, (2.0 * p.p262));
            s.store_scalar(344, (p.p292 + 1e-25));
            s.store_mul_ad_rhs(335, 338, A::scale_offset(A::div(s.ad_value(381), s.ad_value(344)), (-s.v[539]), ((s.v[539]) + (1.0))));
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
        var_cox0: f64,
        var_tox0: f64,
        var_uc_nover: f64,
    ) {
        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, (-((2.0 * 0.01) * 0.01)), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (1.0 + s.v[539]));
            s.store_offset_sub(781, 337, 336, (-(5e-5 * 0.01)));
            s.store_scale(782, 337, (4.0 * (5e-5 * 0.01)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_scaled_ad(339, A::div_scaled_offset_numerator(s.ad_value(781), 1.0, ((2.0 * 5e-5) * 0.01), s.ad_value(782), 1.0), (-0.5), 0.5);
            s.store_add_scaled_inputs3_indices(367, 337, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub_from_scalar_scaled_input(335, 1.0, 382, s.v[537]);
            s.store_sqrt_square_offset(782, 335, ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)));
            s.store_offset_scaled_div(338, 335, 782, 0.5, 0.5);
            s.store_scaled_add(337, 335, 782, 0.5);
        }

        s.b[1413] = (s.v[337] < 0.0);
        s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });

        if (((s.b[1405] && s.b[1407]) && s.b[1412]) && s.b[1413]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1412]) {
            s.store_offset(337, 337, 1e-25);
            s.copy_ad(334, 367);
            s.store_mul(367, 367, 337);
        }

        if ((s.b[1405] && s.b[1407]) && (!s.b[1412])) {
            s.copy_ad(367, 691);
        }

        s.b[1414] = (((p.p54 == 1.0) && (p.p34 == 0.0)) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));
        s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            s.store_sub_scaled_ad_lhs(334, A::sub_from_scalar(p.p335, A::scale(s.ad_value(380), p.p333)), 383, p.p332);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 10.0) * 10.0));
            s.store_offset_scaled_div(336, 334, 782, 0.5, 0.5);
            s.store_scaled_add(335, 334, 782, 0.5);
        }

        s.b[1415] = (s.v[335] < 0.0);
        s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });

        if (((s.b[1405] && s.b[1407]) && s.b[1414]) && s.b[1415]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            s.store_offset(335, 335, (10.0 * 2.220446049250313e-16));
            s.store_scalar(334, (s.v[544] / (var_uc_nover * (s.v[544] + var_uc_nover))));
            s.store_scale(338, 334, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_offset_sqrt_ad(384, A::mul(s.ad_value(338), s.ad_value(335)), 1e-25);
            s.store_offset_sub_from_scalar_ad(781, p.p334, s.ad_value(384), (-(0.1 * p.p334)));
            s.store_scalar(782, ((4.0 * p.p334) * (0.1 * p.p334)));
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1405] && s.b[1407]) && s.b[1414]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(384, 781, (-0.5), 782, (-0.5), p.p334);
            s.store_div_from_scalar_sub_from_scalar_ad(340, s.v[165], p.p334, s.ad_value(384));
            s.store_mul(334, 366, 340);
            s.store_mul(335, 367, 340);
            s.store_add_scaled_products_indices(366, 334, 370, 1.0, 366, 371, 1.0);
            s.store_add_scaled_products_indices(367, 335, 371, 1.0, 367, 370, 1.0);
        }

        if ((s.b[1405] && s.b[1407]) && (!s.b[1414])) {
            s.store_scalar(384, 0.0);
        }

        if (s.b[1405] && s.b[1407]) {
            s.copy_ad(4, 366);
            s.copy_ad(5, 367);
        }

        if (s.b[1405] && (!s.b[1407])) {
            s.store_add_scaled_products_indices(4, 370, 690, 1.0, 371, 691, 1.0);
            s.store_add_scaled_products_indices(5, 370, 691, 1.0, 371, 690, 1.0);
        }

        if s.b[1405] {
            s.store_scale(4, 4, 1.0 / (s.v[164]));
            s.store_scale(5, 5, 1.0 / (s.v[164]));
            s.store_add_scaled_value_products(4, s.ad_value(4), 1.0, s.ad_value(370), s.ad_value(644), 1.0, s.ad_value(371), s.ad_value(648), 1.0);
            s.store_add_scaled_value_products(5, s.ad_value(5), 1.0, s.ad_value(370), s.ad_value(648), 1.0, s.ad_value(371), s.ad_value(644), 1.0);
            s.store_add_scaled_products_indices(334, 370, 4, 1.0, 371, 5, 1.0);
            s.store_add_scaled_products_indices(334, 370, 5, 1.0, 371, 4, 1.0);
        }

        s.b[1418] = (s.v[792] > s.v[70]);
        s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });

        if s.b[1418] {
            s.store_sub(335, 792, 70);
            s.store_sub(336, 69, 70);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(84, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 84, 1.0);
            s.store_neg(84, 84);
            s.store_add(83, 70, 333);
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_mul(338, 335, 337);
            s.store_square(339, 338);
            s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);
            s.store_div_scaled_inputs_product(84, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, s.ad_value(339), 3.0, s.ad_value(338), s.ad_value(339), 4.0, A::square(s.ad_value(341)), 1.0);
        }

        if (!s.b[1418]) {
            s.copy_ad(83, 792);
            s.store_scalar(84, 1.0);
        }

        s.store_scaled_mul(335, 84, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(108, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1419] = (s.v[108] < 1e-12);
        s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });

        if s.b[1419] {
            s.store_scalar(108, 1e-12);
        }

        s.store_add(105, 83, 108);

        s.store_add_scaled_inputs(106, 790, 1.0, 108, 2.0);

        s.store_add(107, 791, 108);

        s.store_scale(335, 636, (s.v[189] * s.v[189]));

        s.store_offset(336, 791, (-s.v[160]));

        s.store_offset_mul_ad(337, A::div_from_scalar(2.0, s.ad_value(335)), A::add_scaled_inputs3(s.ad_value(336), 1.0, A::div_from_scalar(1.0, s.ad_value(678)), (-1.0), s.ad_value(83), -1.0), 1.0);

        s.store_sqrt_square_offset(782, 337, ((4.0 * 0.001) * 0.001));

        s.store_offset_scaled_div(339, 337, 782, 0.5, 0.5);

        s.store_scaled_add(338, 337, 782, 0.5);

        s.b[1420] = (s.v[338] < 0.0);
        s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });

        if s.b[1420] {
            s.store_scalar(338, 0.0);
            s.store_scalar(339, 0.0);
        }

        s.store_offset(338, 338, 1e-25);

        s.store_sqrt(332, 338);

        s.store_add_mul_sub_from_scalar_rhs_indices(128, 336, 335, 1.0, 332);

        s.store_sub(129, 128, 159);

        s.store_offset(781, 129, (((-0.1)) + ((-0.05))));

        s.store_scalar(782, ((4.0 * 0.1) * 0.05));

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_square_add(782, 781, 782);

        s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);

        s.store_offset_add_scaled_inputs_indices(129, 781, 0.5, 782, 0.5, 0.1);

        s.store_div(335, 790, 129);

        s.copy_ad(781, 335);

        s.store_square(782, 781);

        s.store_mul(783, 782, 781);

        s.store_square(784, 782);

        s.store_div_from_scalar_ad(332, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));

        s.store_mul_ad_affine_product_lhs(334, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(332), -1.0, 0.0, 332);

        s.store_sub_from_scalar(332, 1.0, 332);

        s.store_neg(334, 334);

        s.store_square(208, 332);

        s.b[1421] = (s.v[765] == 0.0);
        s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });

        let (assign22340_e17189,) = {
    if s.b[1421] {
        (0.0,)
    } else {
        (s.v[80],)
    }
};
        s.store_scalar(80, assign22340_e17189);

        let (assign22350_e17194,) = {
    if (!s.b[1421]) {
        (1.0,)
    } else {
        (s.v[80],)
    }
};
        s.store_scalar(80, assign22350_e17194);

        s.copy_ad(335, 637);

        s.store_sqrt_mul(336, 335, 158);

        s.store_add_scaled_ad_lhs(190, A::offset(s.ad_value(158), s.v[160]), 336, s.v[189]);

        s.b[1422] = (s.v[80] == 0.0);
        s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });

        if s.b[1422] {
            s.store_scalar(183, var_tox0);
            s.store_scalar(185, var_cox0);
            s.store_scalar(186, s.v[189]);
            s.store_mul_square_lhs(334, 209, 186);
            s.store_mul(211, 334, 186);
        }

        if (!s.b[1422]) {
            s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p.p236);
            s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));
            s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);
            s.store_scaled_add(336, 339, 782, 0.5);
        }

        s.b[1423] = (s.v[336] < 0.0);
        s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });

        if ((!s.b[1422]) && s.b[1423]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(337, 0.0);
        }

        if (!s.b[1422]) {
            s.store_offset(336, 336, 1e-25);
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));
            s.store_scaled_abs(338, 190, 2.0);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
        var_c_eox: f64,
        var_lgate: f64,
        var_tox0: f64,
        var_wg: f64,
    ) {
        if (!s.b[1422]) {
            s.store_offset_sub(340, 339, 791, s.v[160]);
        }

        s.b[1424] = (s.v[340] > s.v[338]);
        s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });

        if ((!s.b[1422]) && s.b[1424]) {
            s.copy_ad(338, 340);
        }

        if (!s.b[1422]) {
            s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));
            s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));
        }

        if (!s.b[1422]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (!s.b[1422]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));
            s.store_offset_scaled(184, 336, p.p235, p.p237);
            s.store_scalar(341, p.p235);
        }

        s.b[1425] = ((s.v[184] * 1000000000000.0) < s.v[187]);
        s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });

        if ((!s.b[1422]) && s.b[1425]) {
            s.store_scalar(184, 0.0);
        }

        let (assign22690_e17456,) = {
    if ((!s.b[1422]) && s.b[1425]) {
        (0.0,)
    } else {
        (s.v[80],)
    }
};
        s.store_scalar(80, assign22690_e17456);

        if (!s.b[1422]) {
            s.store_offset(183, 184, var_tox0);
            s.store_div_from_scalar(185, var_c_eox, 183);
            s.store_div_from_scalar_square_ad(335, (-var_c_eox), s.ad_value(183));
            s.store_scale(186, 183, 1.0 / (var_c_eox));
            s.store_scalar(335, (1.0 / var_c_eox));
            s.store_mul_square_lhs(334, 209, 186);
            s.store_mul(211, 334, 186);
        }

        s.copy_ad(364, 105);

        s.copy_ad(335, 637);

        s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);

        s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);

        s.store_add_ad_lhs(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);

        s.copy_ad(123, 158);

        s.store_scalar(334, 0.95);

        s.b[338] = (!(s.v[963] > 1.0));
        s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });

        s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));

        s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));

        s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));

        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p.p366);
        } else {
            s.store_scalar(339, 0.0);
        }

        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);

        s.store_sqrt(181, 180);

        s.b[1426] = (p.p140 != 0.0);
        s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });

        if s.b[1426] {
            s.copy_ad(335, 637);
            s.store_sub_from_scalar(336, p.p224, 364);
            s.store_offset(337, 336, 1e-25);
            s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));
            s.store_scaled_add(339, 337, 338, 0.5);
            s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);
            s.store_div_from_scalar(341, 1.0, 339);
            s.store_scale(175, 341, p.p223);
            s.store_mul_neg_lhs(342, 175, 341);
            s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));
            s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));
        }

        if s.b[1426] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1426] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));
            s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);
            s.store_div(343, 334, 176);
            s.store_mul_sub_lhs(177, 239, 176, 186);
            s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);
            s.store_add_scaled_product_value_ad(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);
            s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);
            s.store_scale(335, 186, 1.034943e-10);
            s.copy_ad(336, 685);
            s.store_scalar(338, (1.0 / (p.p140 * p.p140)));
            s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);
            s.store_mul(121, 339, 181);
            s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);
            s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));
            s.store_sub(335, 173, 119);
            s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p.p140)), s.v[465]);
            s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);
            s.store_offset(178, 106, p.p221);
            s.store_square(179, 178);
            s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);
        }

        if (!s.b[1426]) {
            s.store_scalar(174, 0.0);
        }

        s.store_scale(335, 186, 1.034943e-10);

        s.copy_ad(336, 684);

        s.store_scalar(337, (var_lgate - p.p139));

        s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));

        s.store_mul_scaled_ad_lhs(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338]);

        s.store_mul(121, 339, 181);

        s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);

        s.store_mul_scale_ad_lhs(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 181);

        s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);

        s.store_scalar(335, (s.v[470] / var_lgate));

        s.store_offset_scaled(338, 180, s.v[335], s.v[468]);

        s.store_add_scaled_product_right_ad(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p.p150, 1.0), s.v[469]);

        s.store_mul(122, 121, 339);

        s.store_div_from_scalar(335, 1.0, 185);

        s.store_square(336, 335);

        s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));

        s.store_square(338, 337);

        s.store_sub(339, 335, 337);

        s.store_mul_sub_rhs(340, 239, 336, 338);

        s.store_offset_mul(124, 239, 339, (s.v[478] / var_wg));

        s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);

        s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);

        let assign23510_e18030: f64 = (s.v[157] + s.v[160]);
        let assign23510_e18033: f64 = (s.v[336] * s.v[189]);
        let assign23510_e18034: f64 = (assign23510_e18030 + assign23510_e18033);
        let assign23510_e18036: f64 = (assign23510_e18034 - s.v[120]);
        s.store_scalar(118, assign23510_e18036);

        s.store_mul(212, 209, 186);

        s.store_square(213, 212);

        s.store_scalar(182, 0.0);

        s.b[1427] = (s.v[615] == 1.0);
        s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });

        if s.b[1427] {
            s.copy_ad(341, 107);
            s.copy_ad(334, 642);
            s.store_offset(337, 341, (-p.p152));
        }

        s.b[1428] = (s.v[337] < (-3.0));
        s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });

        if (s.b[1427] && s.b[1428]) {
            s.store_scalar(340, 0.0);
            s.store_scalar(182, 0.0);
        }

        s.b[1429] = (s.v[337] < 0.0);
        s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });

        if ((s.b[1427] && (!s.b[1428])) && s.b[1429]) {
            s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);
        }

        if ((s.b[1427] && (!s.b[1428])) && (!s.b[1429])) {
            s.store_offset_mul_offset_rhs_ad_rhs(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);
        }

        if s.b[1427] {
            s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));
            s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);
        }

        s.b[1430] = (s.v[182] < 0.0);
        s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });

        if (s.b[1427] && s.b[1430]) {
            s.store_scalar(182, 0.0);
            s.store_scalar(340, 0.0);
        }

        if s.b[1427] {
            s.store_mul(182, 182, 334);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1427] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1427] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);
        }

        s.b[1437] = (s.v[792] > s.v[73]);
        s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });

        if ((p.p37 != 0.0) && s.b[1437]) {
            s.store_sub(335, 792, 73);
            s.store_sub(336, 72, 73);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(1432, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1432, 1.0);
            s.store_neg(1432, 1432);
            s.store_add(1431, 73, 333);
            s.store_div_from_scalar(337, 1.0, 336);
            s.store_mul(338, 335, 337);
            s.store_square(339, 338);
            s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);
            s.store_div_scaled_inputs_product(1432, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, s.ad_value(339), 3.0, s.ad_value(338), s.ad_value(339), 4.0, A::square(s.ad_value(341)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((p.p37 != 0.0) && (!s.b[1437])) {
            s.copy_ad(1431, 792);
            s.store_scalar(1432, 1.0);
        }

        if (p.p37 == 0.0) {
            s.copy_ad(1431, 792);
            s.store_scalar(1432, 1.0);
        }

        s.store_scaled_mul(335, 1432, 790, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p262)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(1433, p.p262, 782);

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[1438] = (s.v[1433] < 1e-12);
        s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });

        if s.b[1438] {
            s.store_scalar(1433, 1e-12);
        }

        s.store_add(1434, 1431, 1433);

        s.store_add_scaled_inputs(1435, 790, 1.0, 1433, 2.0);

        s.store_add(1436, 791, 1433);

        s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));

        let assign24140_e18636: f64 = (s.v[160] - s.v[120]);
        let assign24140_e18638: f64 = (assign24140_e18636 + s.v[182]);
        let assign24140_e18640: f64 = (assign24140_e18638 + s.v[1431]);
        s.store_scalar(86, assign24140_e18640);

        s.b[1439] = (s.v[963] != 0.0);
        s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });

        s.b[1440] = (p.p42 == 1.0);
        s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });

        s.b[1441] = (p.p42 == 2.0);
        s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });

        s.b[1442] = (p.p42 == 3.0);
        s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });

        if (s.b[1439] && s.b[1440]) {
            s.copy_ad(1459, 960);
            s.store_scale(1542, 964, 1.6021918e-19);
            s.store_square(1541, 964);
            s.store_scale(1498, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_scale(1540, 622, 1.6021918e-19);
            s.store_scalar(1537, (1.6021918e-19 * 1.6021918e-19));
            s.store_scalar(1538, (1.034943e-10 * 1.034943e-10));
            s.store_square(1539, 965);
            s.store_div_from_scalar(1543, (2.0 * 1.034943e-10), 1542);
            s.store_scale(1544, 1542, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_scale(1545, 1542, (2.0 * 1.034943e-10));
            s.store_div_from_scalar(1546, (2.0 * 1.034943e-10), 1540);
            s.store_scale(1547, 1540, 1.0 / ((2.0 * 1.034943e-10)));
            s.store_div(1532, 964, 622);
            s.store_div_from_scalar_offset_input(1531, 1.0, 1532, 1.0);
            s.store_scalar(1548, (1e-12 * 1000.0));
            s.store_scalar(1549, (1e-10 * 1000.0));
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1471, 0.0);
            s.store_scalar(1472, 0.0);
            s.store_scalar(1513, 0.0);
            s.store_scalar(1514, 0.0);
            s.store_scalar(1493, 0.0);
            s.store_scalar(1495, 0.0);
            s.store_scalar(1494, 0.0);
            s.store_scalar(1496, 0.0);
            s.store_scalar(1516, 0.0);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));
            s.store_div_scaled_product_by_product(1452, s.ad_value(185), s.ad_value(185), 1.0, s.ad_value(209), s.ad_value(209), 1.0);
            s.store_mul_ad_lhs(1455, A::div_scaled_value_by_product(s.ad_value(1452), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1541);
            s.store_sqrt_mul_ad(1449, A::div_scaled_product(s.ad_value(1543), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1459), s.ad_value(1431)));
        }

        s.b[1555] = (s.v[1449] > s.v[965]);
        s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });

        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
            s.store_scalar(1462, 0.0);
            s.copy_ad(1443, 965);
            s.store_scalar(1479, 0.0);
            s.store_sub_ad_rhs(1460, 1479, A::mul3(s.ad_value(1544), s.ad_value(1443), s.ad_value(1443)));
            s.store_scalar(1507, 0.0);
        }

        let (assign24570_e18977,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        (s.v[1462],)
    } else {
        (s.v[1506],)
    }
};
        s.store_scalar(1506, assign24570_e18977);

        let (assign24580_e18985,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        (s.v[1460],)
    } else {
        (s.v[1468],)
    }
};
        s.store_scalar(1468, assign24580_e18985);

        let (assign24590_e18993,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign24590_e18993);

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
    ) {
        let mut assign24600_loop_guard: usize = 0;
        while {
            let assign24600_cond_e19002: f64 = (150.0 + 1.0);
            let assign24600_cond_e19004: f64 = if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (s.v[97] <= assign24600_cond_e19002)) { 1.0 } else { 0.0 };
            assign24600_cond_e19004 != 0.0
        } {
            assign24600_loop_guard += 1;
            assert!(assign24600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.store_sqrt_mul_sub_rhs(1443, 1543, 1479, 1460);
            }
            s.b[1556] = ((s.v[1443] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
                s.store_offset_sub(781, 1443, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign24600_body7_e19094,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign24600_body7_e19094);
            let (assign24600_body8_e19104,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body8_e19104);
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1557] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
            s.b[1558] = (2.0 == 1.0);
            s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
            let (assign24600_body19_e19226,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && s.b[1558]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body19_e19226);
            s.b[1559] = (2.0 == 2.0);
            s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
            let (assign24600_body21_e19246,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && s.b[1559]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body21_e19246);
            s.b[1560] = (2.0 == 4.0);
            s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            let (assign24600_body23_e19269,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && (!s.b[1559])) && s.b[1560]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body23_e19269);
            s.b[1561] = (2.0 == 8.0);
            s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            let (assign24600_body25_e19295,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (!s.b[1558])) && (!s.b[1559])) && (!s.b[1560])) && s.b[1561]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body25_e19295);
            let (assign24600_body26_e19307,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign24600_body26_e19307);
            let mut assign24600_body27_loop_guard: usize = 0;
            while {
                let assign24600_body27_cond_e19320: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24600_body27_cond_e19320 != 0.0
            } {
                assign24600_body27_loop_guard += 1;
                assert!(assign24600_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {
                    s.store_sqrt(726, 726);
                }
                let (assign24600_body27_body1_e19347,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && s.b[1557]) {
        let assign24600_body27_body1_e19345: f64 = (s.v[719] + 1.0);
        (assign24600_body27_body1_e19345,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, assign24600_body27_body1_e19347);
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) && (!s.b[1557])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1443, 965, (-1e-8), 780);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1556]) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1556])) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1556])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.store_add_scaled_inputs3_indices(335, 1460, 1.0, 1431, (-1.0), 1459, 1.0);
            }
            s.b[1562] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign24600_body43_e19548,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign24600_body43_e19548);
            let (assign24600_body44_e19558,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body44_e19558);
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1563] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            s.b[1564] = (2.0 == 1.0);
            s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            let (assign24600_body55_e19680,) = {
    if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && s.b[1564]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body55_e19680);
            s.b[1565] = (2.0 == 2.0);
            s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
            let (assign24600_body57_e19700,) = {
    if ((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && s.b[1565]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body57_e19700);
            s.b[1566] = (2.0 == 4.0);
            s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            let (assign24600_body59_e19723,) = {
    if (((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && (!s.b[1565])) && s.b[1566]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body59_e19723);
            s.b[1567] = (2.0 == 8.0);
            s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            let (assign24600_body61_e19749,) = {
    if ((((((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (!s.b[1564])) && (!s.b[1565])) && (!s.b[1566])) && s.b[1567]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, assign24600_body61_e19749);
            let (assign24600_body62_e19761,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign24600_body62_e19761);
            let mut assign24600_body63_loop_guard: usize = 0;
            while {
                let assign24600_body63_cond_e19774: f64 = if (((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign24600_body63_cond_e19774 != 0.0
            } {
                assign24600_body63_loop_guard += 1;
                assert!(assign24600_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {
                    s.store_sqrt(726, 726);
                }
                let (assign24600_body63_body1_e19801,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && s.b[1563]) {
        let assign24600_body63_body1_e19799: f64 = (s.v[719] + 1.0);
        (assign24600_body63_body1_e19799,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, assign24600_body63_body1_e19801);
            }
            if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) && (!s.b[1563])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1562]) {
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1562])) {
                s.copy_ad(336, 335);
                s.store_scalar(341, 1.0);
            }
            if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
                s.store_sqrt_mul(1447, 1546, 336);
                s.store_mul(1493, 1443, 1542);
                s.store_mul_div_from_scalar_lhs(1525, (-1.034943e-10), 1443, 334);
                s.store_mul_neg_lhs(1494, 1447, 1540);
                s.store_mul_div_from_scalar_lhs(1527, (-1.034943e-10), 1447, 341);
                s.store_add_ad_lhs(1481, A::add_scaled_product(s.ad_value(1493), 1.0, s.ad_value(185), A::sub(s.ad_value(1462), s.ad_value(1479)), 1.0), 1494);
                s.copy_ad(1483, 185);
                s.store_add(1484, 1525, 1527);
                s.store_add_scaled_product_right_ad(1482, 1460, 1.0, 1531, A::sub(A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1532), s.ad_value(1479), 1.0), s.ad_value(1459)), (-1.0));
                s.store_scalar(1485, 0.0);
                s.store_scalar(1486, 1.0);
                s.store_add_scaled_products_indices(1487, 1483, 1486, 1.0, 1485, 1484, (-1.0));
                s.store_div(1488, 1486, 1487);
                s.store_div_scaled_inputs_indices(1489, 1484, -1.0, 1487, 1.0);
                s.store_div_scaled_inputs_indices(1490, 1485, -1.0, 1487, 1.0);
                s.store_div(1491, 1483, 1487);
            }
            s.b[1568] = (((((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482]))) as f64).abs() > 0.5);
            s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1568]) {
                s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1488] * s.v[1481]) + (s.v[1489] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1568]) {
                s.store_offset(1460, 1460, (-(0.5 * (if (((s.v[1490] * s.v[1481]) + (s.v[1491] * s.v[1482])) >= 0.0) { 1.0 } else { (-1.0) }))));
            }
            if (((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1568])) {
                s.store_sub_ad_rhs(1462, 1462, A::add_scaled_products(s.ad_value(1488), s.ad_value(1481), 1.0, s.ad_value(1489), s.ad_value(1482), 1.0));
                s.store_sub_ad_rhs(1460, 1460, A::add_scaled_products(s.ad_value(1490), s.ad_value(1481), 1.0, s.ad_value(1491), s.ad_value(1482), 1.0));
            }
            s.b[1569] = (((((s.v[1462] - s.v[1506])) as f64).abs() <= 1e-12) && ((((s.v[1460] - s.v[1468])) as f64).abs() <= 1e-12));
            s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            let (assign24600_body94_e20220,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1569]) {
        let assign24600_body94_e20218: f64 = (150.0 + 1.0);
        (assign24600_body94_e20218,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign24600_body94_e20220);
            let (assign24600_body95_e20228,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        (s.v[1462],)
    } else {
        (s.v[1506],)
    }
};
            s.store_scalar(1506, assign24600_body95_e20228);
            let (assign24600_body96_e20236,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        (s.v[1460],)
    } else {
        (s.v[1468],)
    }
};
            s.store_scalar(1468, assign24600_body96_e20236);
            let (assign24600_body97_e20246,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        let assign24600_body97_e20244: f64 = (s.v[97] + 1.0);
        (assign24600_body97_e20244,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign24600_body97_e20246);
        }

        if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
            s.copy_ad(1509, 1460);
            s.store_mul(1447, 965, 1532);
            s.store_add_scaled_inputs3_mixed_aii(1460, A::mul3(s.ad_value(1547), s.ad_value(1447), s.ad_value(1447)), 1.0, 1431, 1.0, 1459, -1.0);
            s.store_add_scaled_product_indices(1479, 1460, 1.0, 1544, 1539, 1.0);
            s.copy_ad(1457, 1479);
            s.copy_ad(1463, 1479);
        }

        let (assign24670_e20316,) = {
    if ((s.b[1439] && s.b[1440]) && s.b[1555]) {
        (s.v[1479],)
    } else {
        (s.v[1505],)
    }
};
        s.store_scalar(1505, assign24670_e20316);

        s.b[1570] = (s.v[85] > s.v[1462]);
        s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });

        let (assign24690_e20329,) = {
    if (((s.b[1439] && s.b[1440]) && s.b[1555]) && s.b[1570]) {
        (1.0,)
    } else {
        (s.v[1475],)
    }
};
        s.store_scalar(1475, assign24690_e20329);

        s.b[1571] = (s.v[85] > s.v[1505]);
        s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });

        let (assign24710_e20345,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1570])) && s.b[1571]) {
        (3.0,)
    } else {
        (s.v[1475],)
    }
};
        s.store_scalar(1475, assign24710_e20345);

        let (assign24720_e20359,) = {
    if ((((s.b[1439] && s.b[1440]) && s.b[1555]) && (!s.b[1570])) && (!s.b[1571])) {
        (2.0,)
    } else {
        (s.v[1475],)
    }
};
        s.store_scalar(1475, assign24720_e20359);

        if ((s.b[1439] && s.b[1440]) && (!s.b[1555])) {
            s.store_scalar(1462, 0.0);
        }

        let (assign24740_e20377,) = {
    if ((s.b[1439] && s.b[1440]) && (!s.b[1555])) {
        (s.v[1462],)
    } else {
        (s.v[1505],)
    }
};
        s.store_scalar(1505, assign24740_e20377);

        if ((s.b[1439] && s.b[1440]) && (!s.b[1555])) {
            s.store_scalar(1463, 0.0);
            s.copy_ad(1507, 1462);
        }

    }
}
