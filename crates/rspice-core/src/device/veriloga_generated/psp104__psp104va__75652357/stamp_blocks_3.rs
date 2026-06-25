#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1019] != 0.0) {
            s.store_scalar(1008, 0.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(1009, 0.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(1007, 0.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(39, p.p788);
        }

        s.v[1126] = if (if self.param_given[789] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1126] != 0.0)) {
            s.store_scalar(39, p.p789);
        }

        s.v[1127] = if (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0)))) { 1.0 } else { 0.0 };

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (s.v[1] - 0.5);
            let assign9160_cond_e8971: f64 = if (((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) && (s.v[1007] < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1008, 1008, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1007), (s.v[7] + s.v[3])), (s.v[5] + (0.5 * s.v[3])))));
            }
            if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1009, 1009, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1007), (s.v[7] + s.v[3])), (s.v[6] + (0.5 * s.v[3])))));
            }
            if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_offset(1007, 1007, 1.0);
            }
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(992, 1008, 2);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(993, 1009, 2);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(994, (1.0 / (p.p784 + (0.5 * s.v[3]))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(995, (1.0 / (p.p785 + (0.5 * s.v[3]))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1005, &{
                if ((s.v[3] + s.v[304]) > 1e-9) {
                    A::offset(s.ad_value(304), s.v[3])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1006, &{
                if (((s.v[4] + s.v[305]) + p.p786) > 1e-9) {
                    A::offset(A::add(s.ad_value(4), s.ad_value(305)), p.p786)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1003, 1.0, A::powf(s.ad_value(1005), p.p794));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1004, 1.0, A::powf(s.ad_value(1006), p.p795));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scale_ad(996, A::add(A::add(A::offset(A::scale(s.ad_value(1003), p.p791), 1.0), A::scale(s.ad_value(1004), p.p792)), A::mul(A::scale(s.ad_value(1003), p.p793), s.ad_value(1004))), (1.0 + (p.p790 * (s.v[346] - 1.0))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(997, A::scale(A::add(s.ad_value(992), s.ad_value(993)), p.p787), 996);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(998, A::scale(A::add(s.ad_value(994), s.ad_value(995)), p.p787), 996);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1003, 1.0, A::powf(s.ad_value(1005), p.p800));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1004, 1.0, A::powf(s.ad_value(1006), p.p801));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add_ad(999, A::add(A::offset(A::scale(s.ad_value(1003), p.p797), 1.0), A::scale(s.ad_value(1004), p.p798)), A::mul(A::scale(s.ad_value(1003), p.p799), s.ad_value(1004)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_sub_ad_lhs(1001, A::sub(A::add(s.ad_value(992), s.ad_value(993)), s.ad_value(994)), 995);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(1002, A::offset(s.ad_value(997), 1.0), A::offset(s.ad_value(998), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(65, 65, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(82, A::mul(A::mul(s.ad_value(82), s.ad_value(1002)), A::offset(A::scale(s.ad_value(998), p.p788), 1.0)), A::offset(A::scale(s.ad_value(997), p.p788), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(121, A::mul(A::mul(s.ad_value(121), s.ad_value(1002)), A::offset(A::mul(s.ad_value(39), s.ad_value(998)), 1.0)), A::offset(A::mul(s.ad_value(39), s.ad_value(997)), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(150, 150, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(1002, A::scale(s.ad_value(1001), p.p796), 999);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(40, 40, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(145, 145, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(1002, A::scale(s.ad_value(1001), p.p802), A::powf(s.ad_value(999), p.p803));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(62, 62, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(155, 155, 1002);
        }

        s.v[1128] = if ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1129] = if (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_offset(1001, 4, s.v[8]);
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_scalar(1002, (1.0 / p.p804));
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_from_scalar_ad(11, (p.p804 * p.p804), A::scale(s.ad_value(1001), s.v[8]));
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(12, A::sub(A::scale(A::exp(A::scale(s.ad_value(1002), ((-10.0) * s.v[8]))), ((0.1 * s.v[8]) + (0.01 * p.p804))), A::mul(A::offset(A::scale(s.ad_value(1001), 0.1), (0.01 * p.p804)), A::exp(A::mul(A::scale(s.ad_value(1001), (-10.0)), s.ad_value(1002))))), 4);
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(13, A::sub(A::scale(A::exp(A::scale(s.ad_value(1002), ((-20.0) * s.v[8]))), ((0.05 * s.v[8]) + (0.0025 * p.p804))), A::mul(A::offset(A::scale(s.ad_value(1001), 0.05), (0.0025 * p.p804)), A::exp(A::mul(A::scale(s.ad_value(1001), (-20.0)), s.ad_value(1002))))), 4);
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad(1001, A::add(s.ad_value(11), A::scale(s.ad_value(12), p.p805)), A::scale(s.ad_value(13), p.p806));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(40, 40, A::mul(s.ad_value(342), s.ad_value(1001)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(65, 65, A::offset(A::mul(s.ad_value(343), s.ad_value(1001)), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(145, 145, A::mul(s.ad_value(342), s.ad_value(1001)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(150, 150, A::offset(A::mul(s.ad_value(343), s.ad_value(1001)), 1.0));
        }

        s.copy_ad(172, 40);

        s.copy_ad(173, 41);

        s.copy_ad(174, 42);

        s.copy_ad(176, 43);

        s.copy_ad(177, 44);

        if (s.v[45] > 1e20) {
            s.store_ad(178, &{
                if (s.v[45] < 1e26) {
                    s.ad_value(45)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(178, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(179, 46);
        } else {
            s.store_scalar(179, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(180, 47);
        } else {
            s.store_scalar(180, 0.0);
        }

        s.copy_ad(181, 48);

        s.copy_ad(182, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(183, 50);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(187, 55);

        s.copy_ad(188, 56);

        if (s.v[57] > 1e23) {
            s.store_ad(189, &{
                if (s.v[57] < 1e27) {
                    s.ad_value(57)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(189, 1e23);
        }

        if (s.v[58] > 1e23) {
            s.store_ad(190, &{
                if (s.v[58] < 1e27) {
                    s.ad_value(58)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(190, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(184, 51);
        } else {
            s.store_scalar(184, 0.0);
        }

        if (s.v[53] > 0.0) {
            s.store_ad(186, &{
                if (s.v[53] < 0.5) {
                    s.ad_value(53)
                } else {
                    A::constant(0.5)
                }
            });
        } else {
            s.store_scalar(186, 0.0);
        }

        if (s.v[52] > 0.0) {
            s.store_ad(185, &{
                if (s.v[52] < 1.0) {
                    s.ad_value(52)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(175, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(191, 62);
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.store_ad(193, &{
                if (s.v[64] < 1.0) {
                    s.ad_value(64)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(193, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(192, 63);
        } else {
            s.store_scalar(192, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(194, 59);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[61] > 0.0) {
            s.store_ad(195, &{
                if (s.v[61] < 1.0) {
                    s.ad_value(61)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(196, 60);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(197, 65);
        } else {
            s.store_scalar(197, 0.0);
        }

        s.copy_ad(198, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(199, 67);
        } else {
            s.store_scalar(199, 0.0);
        }

        s.copy_ad(200, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(201, 69);
        } else {
            s.store_scalar(201, 0.0);
        }

        s.copy_ad(202, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(203, 71);
        } else {
            s.store_scalar(203, 0.0);
        }

        s.copy_ad(204, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(205, 73);
        } else {
            s.store_scalar(205, 0.0);
        }

        s.copy_ad(206, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(207, 75);
        } else {
            s.store_scalar(207, 0.0);
        }

        s.copy_ad(208, 76);

        s.copy_ad(209, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(210, 78);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 79);

        if (s.v[80] > (-0.5)) {
            s.store_ad(212, &{
                if (s.v[80] < 1.0) {
                    s.ad_value(80)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(212, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(213, 81);
        } else {
            s.store_scalar(213, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(214, 82);
        } else {
            s.store_scalar(214, 0.0);
        }

        s.copy_ad(215, 83);

        if (s.v[84] > (-0.5)) {
            s.store_ad(216, &{
                if (s.v[84] < 1.0) {
                    s.ad_value(84)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(217, 85);
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(218, 86);
        } else {
            s.store_scalar(218, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(219, 87);
        } else {
            s.store_scalar(219, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(220, 88);
        } else {
            s.store_scalar(220, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(221, 89);
        } else {
            s.store_scalar(221, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(222, 90);
        } else {
            s.store_scalar(222, 0.0);
        }

        s.copy_ad(223, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(224, 92);
        } else {
            s.store_scalar(224, 0.0);
        }

        s.copy_ad(225, 93);

        s.copy_ad(226, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(227, 95);
        } else {
            s.store_scalar(227, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(228, 96);
        } else {
            s.store_scalar(228, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[97] > 1e-12) {
            s.copy_ad(229, 97);
        } else {
            s.store_scalar(229, 1e-12);
        }

        s.copy_ad(230, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(231, 99);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(232, 100);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(233, 101);
        } else {
            s.store_scalar(233, 0.0);
        }

        s.copy_ad(234, 102);

        s.copy_ad(235, 103);

        s.copy_ad(236, 104);

        s.copy_ad(237, 105);

        s.copy_ad(238, 106);

        s.copy_ad(239, 107);

        s.copy_ad(240, 108);

        s.copy_ad(241, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(242, 110);
        } else {
            s.store_scalar(242, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(243, 111);
        } else {
            s.store_scalar(243, 0.0);
        }

        s.copy_ad(244, 112);

        s.copy_ad(245, 113);

        s.copy_ad(246, 114);

        s.copy_ad(247, 115);

        s.copy_ad(248, 116);

        s.copy_ad(249, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(250, 118);
        } else {
            s.store_scalar(250, 0.0);
        }

        s.copy_ad(251, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(252, 120);
        } else {
            s.store_scalar(252, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(253, 121);
        } else {
            s.store_scalar(253, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(254, 122);
        } else {
            s.store_scalar(254, 2.0);
        }

        s.copy_ad(255, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(256, 124);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(257, 125);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(258, 126);
        } else {
            s.store_scalar(258, 0.0);
        }

        s.copy_ad(259, 127);

        s.copy_ad(260, 128);

        s.copy_ad(261, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(262, 130);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(263, 131);
        } else {
            s.store_scalar(263, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(264, 132);
        } else {
            s.store_scalar(264, 0.0);
        }

        s.copy_ad(265, 133);

        s.copy_ad(266, 134);

        s.copy_ad(267, 135);

        s.copy_ad(268, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(269, 137);
        } else {
            s.store_scalar(269, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(270, 138);
        } else {
            s.store_scalar(270, 0.0);
        }

        s.copy_ad(271, 139);

        s.copy_ad(277, 145);

        s.copy_ad(278, 146);

        s.copy_ad(279, 147);

        if (s.v[148] > 1e20) {
            s.store_ad(280, &{
                if (s.v[148] < 1e26) {
                    s.ad_value(148)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(280, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(281, 149);
        } else {
            s.store_scalar(281, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(282, 150);
        } else {
            s.store_scalar(282, 0.0);
        }

        s.copy_ad(283, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(284, 152);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[153] > 0.0) {
            s.store_ad(285, &{
                if (s.v[153] < 1.0) {
                    s.ad_value(153)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(285, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(286, 154);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(287, 155);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[157] > 0.0) {
            s.store_ad(289, &{
                if (s.v[157] < 1.0) {
                    s.ad_value(157)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
        }

        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }

        s.v[16] = p.p16;

        s.v[17] = p.p15;

        s.v[18] = p.p18;

        s.v[19] = p.p17;

        s.v[1130] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1130] != 0.0) {
            s.copy_ad(188, 187);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(190, 189);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(243, 242);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(245, 244);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(247, 246);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(249, 248);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(233, 232);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(239, 237);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(240, 238);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(258, 257);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(260, 259);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(264, 263);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(270, 269);
        }

        s.store_scale(762, 177, 8.8541878176e-12);

        s.store_div(763, 762, 176);

        s.store_square(764, 176);

        s.store_scale(765, 763, 6.241449993689894e18);

        s.store_mul(766, 252, 178);

        if (s.v[766] > 1e20) {
            s.store_ad(766, &{
                if (s.v[766] < 1e26) {
                    s.ad_value(766)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(766, 1e20);
        }

        s.v[767] = 0.0;

        s.v[1131] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1131] != 0.0) {
            s.store_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.v[1132] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_scale(767, 767, (7.448711 / 5.951993));
        }

        s.store_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));

        s.store_scale(769, 209, 0.5);

        s.v[770] = 0.5;

        s.v[1133] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[1133] != 0.0) {
            s.store_scale(769, 209, 0.3333333333333333);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(770, 0.3333333333333333);
        }

        s.store_offset_ad(1000, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0)), (-1.0));

        s.store_ad(771, &A::div(A::mul(A::offset(s.ad_value(1000), (-1.0)), A::offset(s.ad_value(1000), (-1.0))), {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_offset_ad(1000, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0)), (-1.0));

        s.store_ad(772, &A::div(A::mul(A::offset(s.ad_value(1000), (-1.0)), A::offset(s.ad_value(1000), (-1.0))), {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_div_from_scalar(773, 1.0, 223);

        s.store_div(774, 762, 187);

        s.store_div(775, 762, 188);

        s.store_div_ad_lhs(776, A::sqrt(A::scale(s.ad_value(189), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[349])))), 774);

        s.store_div_ad_lhs(777, A::sqrt(A::scale(s.ad_value(190), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[349])))), 775);

        s.store_square(778, 776);

        s.store_square(779, 777);

        s.store_offset_ad(780, A::div(A::ln(A::offset(A::exp(A::scale(s.ad_value(261), (0.005 * s.v[349]))), (-1.0))), s.ad_value(261)), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(781, A::ln(A::scale(s.ad_value(776), 0.5)), 780);

        s.store_add_ad_lhs(782, A::ln(A::scale(s.ad_value(777), 0.5)), 780);

        s.store_div_from_scalar(814, 1.0, 776);

        s.store_offset_scaled(815, 776, 3.1, 8.5);

        s.store_square(783, 815);

        s.store_scale(816, 815, 0.5);

        s.v[1134] = if (s.v[814] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1134] != 0.0) {
            s.store_scale(784, 814, 64.0);
        }

        s.v[1135] = if (s.v[814] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1134] != 0.0)) && (s.v[1135] != 0.0)) {
            s.store_offset_scaled(784, 814, 22.0, 3.0);
        }

        s.v[1136] = if (s.v[814] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (s.v[1136] != 0.0)) {
            s.store_offset_scaled(784, 814, (-7.2), 15.5);
        }

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (!(s.v[1136] != 0.0))) {
            s.copy_ad(784, 776);
        }

        s.store_sub_ad(785, A::add(s.ad_value(816), A::scale(s.ad_value(778), 0.5)), A::mul(s.ad_value(776), A::sqrt(A::add(A::add(s.ad_value(816), A::scale(s.ad_value(778), 0.25)), s.ad_value(784)))));

        s.store_div_from_scalar(814, 1.0, 777);

        s.store_offset_scaled(815, 777, 3.1, 8.5);

        s.store_square(786, 815);

        s.store_scale(816, 815, 0.5);

        s.v[1137] = if (s.v[814] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1137] != 0.0) {
            s.store_scale(787, 814, 64.0);
        }

        s.v[1138] = if (s.v[814] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1137] != 0.0)) && (s.v[1138] != 0.0)) {
            s.store_offset_scaled(787, 814, 22.0, 3.0);
        }

        s.v[1139] = if (s.v[814] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (s.v[1139] != 0.0)) {
            s.store_offset_scaled(787, 814, (-7.2), 15.5);
        }

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (!(s.v[1139] != 0.0))) {
            s.copy_ad(787, 777);
        }

        s.store_sub_ad(788, A::add(s.ad_value(816), A::scale(s.ad_value(779), 0.5)), A::mul(s.ad_value(777), A::sqrt(A::add(A::add(s.ad_value(816), A::scale(s.ad_value(779), 0.25)), s.ad_value(787)))));

        s.store_add_ad(722, A::offset(s.ad_value(182), s.v[356]), A::scale(A::ln(A::scale(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26)), (2.0 * s.v[709])));

        if !(s.v[722] > 0.05) {
            s.store_scalar(722, 0.05);
        }

        s.store_div_ad_lhs(723, A::sqrt(A::scale(s.ad_value(178), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[355])))), 763);

        s.v[724] = 0.0;

        s.v[725] = 0.0;

        s.v[1140] = if (s.v[183] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1140] != 0.0) {
            s.store_div_from_scalar(726, 80000000.0, 764);
        }

        if (s.v[1140] != 0.0) {
            s.store_ad(725, &{
                if (s.v[183] > s.v[726]) {
                    s.ad_value(183)
                } else {
                    s.ad_value(726)
                }
            });
        }

        if (s.v[1140] != 0.0) {
            s.store_ad(725, &{
                if (5e24 > s.v[725]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(725)
                }
            });
        }

        if (s.v[1140] != 0.0) {
            s.store_div_ad(724, A::scale(A::mul(A::scale(s.ad_value(763), 2.0), s.ad_value(763)), s.v[709]), A::scale(s.ad_value(725), (1.6021918e-19 * s.v[761])));
        }

        s.v[727] = ((100.0 * s.v[709]) * s.v[709]);

        s.v[1141] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1141] != 0.0) {
            s.store_sqrt_ad(728, A::mul(A::mul(A::scale(s.ad_value(723), s.v[709]), s.ad_value(723)), s.ad_value(722)));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1141] != 0.0) {
            s.store_mul_ad(729, A::scale(s.ad_value(767), 0.75), A::powf(s.ad_value(728), 0.6666666666666666));
        }

        if (s.v[1141] != 0.0) {
            s.store_add(722, 722, 729);
        }

        if (s.v[1141] != 0.0) {
            s.store_mul_ad_rhs(723, 723, A::offset(A::div(A::scale(s.ad_value(729), (2.0 * 0.6666666666666666)), s.ad_value(728)), 1.0));
        }

        s.store_sqrt(730, 722);

        s.store_scale(731, 722, 0.95);

        s.store_mul_ad_lhs(732, A::scale(s.ad_value(722), 0.0025), 722);

        s.copy_ad(733, 732);

        s.store_scaled_sqrt(734, 733, 0.5);

        s.store_scale_ad(735, A::sub(A::sub(s.ad_value(731), s.ad_value(734)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(731), s.ad_value(734)), A::sub(s.ad_value(731), s.ad_value(734))), s.ad_value(732)))), 0.5);

        s.store_scaled_offset(736, 722, s.v[356], 0.5);

        s.store_sub_ad_lhs(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);

        s.store_sub_ad_lhs(738, A::sub(A::sqrt(A::add(A::add(s.ad_value(180), s.ad_value(181)), s.ad_value(722))), s.ad_value(730)), 737);

        s.store_add_ad(739, A::add(A::offset(s.ad_value(182), s.v[356]), s.ad_value(251)), A::scale(A::ln(A::scale(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26)), (2.0 * s.v[709])));

        if !(s.v[739] > 0.05) {
            s.store_scalar(739, 0.05);
        }

        s.store_div_ad_lhs(740, A::sqrt(A::scale(s.ad_value(766), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[355])))), 763);

        s.v[1142] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.store_sqrt_ad(728, A::mul(A::mul(A::scale(s.ad_value(740), s.v[709]), s.ad_value(740)), s.ad_value(739)));
        }

        if (s.v[1142] != 0.0) {
            s.store_mul_ad(729, A::scale(s.ad_value(767), 0.75), A::powf(s.ad_value(728), 0.6666666666666666));
        }

        if (s.v[1142] != 0.0) {
            s.store_add(739, 739, 729);
        }

        if (s.v[1142] != 0.0) {
            s.store_mul_ad_rhs(740, 740, A::offset(A::div(A::scale(s.ad_value(729), (2.0 * 0.6666666666666666)), s.ad_value(728)), 1.0));
        }

        s.store_scale(741, 739, 0.95);

        s.store_mul_ad_lhs(742, A::scale(s.ad_value(739), 0.0025), 739);

        s.copy_ad(743, 742);

        s.store_scaled_sqrt(734, 743, 0.5);

        s.store_scale_ad(744, A::sub(A::sub(s.ad_value(741), s.ad_value(734)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(741), s.ad_value(734)), A::sub(s.ad_value(741), s.ad_value(734))), s.ad_value(742)))), 0.5);

        s.store_offset_ad(694, A::add(s.ad_value(172), A::mul(A::scale(s.ad_value(173), s.v[352]), A::offset(A::scale(s.ad_value(174), s.v[352]), 1.0))), s.v[17]);

        s.store_exp_ad(745, A::scale(s.ad_value(175), s.v[354]));

        s.store_mul(695, 184, 745);

        s.store_scale(696, 185, 1.0 / (s.v[353]));

        s.store_exp_ad(746, A::scale(s.ad_value(198), s.v[354]));

        s.store_mul(697, 197, 746);

        s.store_mul_ad_lhs(710, A::scale(s.ad_value(697), s.v[16]), 763);

        s.store_mul_ad_rhs(699, 201, A::exp(A::scale(s.ad_value(202), s.v[354])));

        s.store_exp_ad(747, A::scale(s.ad_value(200), s.v[354]));

        s.store_mul(698, 199, 747);

        s.store_mul_ad_rhs(701, 205, A::exp(A::scale(s.ad_value(206), s.v[354])));

        s.store_exp_ad(748, A::scale(s.ad_value(204), s.v[354]));

        s.store_mul(700, 203, 748);

        s.store_exp_ad(749, A::scale(s.ad_value(208), s.v[354]));

        s.store_mul(702, 207, 749);

        s.store_exp_ad(750, A::scale(s.ad_value(211), s.v[354]));

        s.store_mul(703, 210, 750);

        s.store_mul_ad_lhs(751, A::scale(s.ad_value(710), 2.0), 703);

        s.store_exp_ad(752, A::scale(s.ad_value(215), s.v[354]));

        s.store_mul(714, 214, 752);

        s.store_mul(715, 253, 752);

        s.store_mul_ad_rhs(706, 225, A::exp(A::scale(A::neg(s.ad_value(226)), s.v[354])));

        s.store_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));

        s.v[1143] = if ((p.p46 != 0.0) && (s.v[282] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_offset_ad(707, A::add(s.ad_value(277), A::scale(s.ad_value(278), s.v[352])), s.v[19]);
        }

        if (s.v[1143] != 0.0) {
            s.store_exp_ad(753, A::scale(s.ad_value(283), s.v[354]));
        }

        if (s.v[1143] != 0.0) {
            s.store_mul(708, 282, 753);
        }

        if (s.v[1143] != 0.0) {
            s.store_mul_ad_lhs(711, A::scale(s.ad_value(708), s.v[18]), 763);
        }

        if (s.v[1143] != 0.0) {
            s.store_scale_ad(717, A::offset(A::scale(s.ad_value(281), s.v[353]), 1.0), s.v[709]);
        }

        if (s.v[1143] != 0.0) {
            s.store_add_ad(754, A::offset(s.ad_value(279), s.v[356]), A::mul(A::scale(s.ad_value(717), 2.0), A::ln(A::scale(A::mul(s.ad_value(280), A::powf(s.ad_value(357), (-0.75))), 4e-26))));
        }

        if (s.v[1143] != 0.0) {
            s.store_ad(754, &{
                if (s.v[754] > 0.05) {
                    s.ad_value(754)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[1143] != 0.0) {
            s.store_div_ad_lhs(755, A::sqrt(A::scale(s.ad_value(280), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[355])))), 763);
        }

        if (s.v[1143] != 0.0) {
            s.store_square(718, 755);
        }

        if (s.v[1143] != 0.0) {
            s.store_ln(719, 718);
        }

        if (s.v[1143] != 0.0) {
            s.store_scale(756, 754, 0.95);
        }

        if (s.v[1143] != 0.0) {
            s.store_mul_ad_lhs(757, A::scale(s.ad_value(754), 0.0025), 754);
        }

        if (s.v[1143] != 0.0) {
            s.copy_ad(758, 757);
        }

        if (s.v[1143] != 0.0) {
            s.store_scaled_sqrt(759, 758, 0.5);
        }

        if (s.v[1143] != 0.0) {
            s.store_scale_ad(760, A::sub(A::sub(s.ad_value(756), s.ad_value(759)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(756), s.ad_value(759)), A::sub(s.ad_value(756), s.ad_value(759))), s.ad_value(757)))), 0.5);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(707, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(753, 1.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(708, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(711, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(717, s.v[709]);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(754, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(755, 1.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(718, 1.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(756, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(757, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(758, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(759, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(760, 0.0);
        }

        s.store_div_from_scalar(789, 1.0, 241);

        s.store_scale_ad(790, A::sqrt(A::scale(s.ad_value(241), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(791, 790, 176);

        s.store_mul(792, 790, 187);

        s.store_mul(793, 790, 188);

        s.v[794] = 0.0;

        s.v[1144] = if (s.v[236] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1144] != 0.0) {
            s.store_div_ad_lhs(794, A::scale(s.ad_value(235), (-0.495)), 236);
        }

        s.v[795] = 0.0;

        s.v[1145] = if (s.v[238] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1145] != 0.0) {
            s.store_div_ad_lhs(795, A::scale(s.ad_value(237), (-0.495)), 238);
        }

        s.v[1146] = if (s.v[240] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1146] != 0.0) {
            s.store_div_ad_lhs(796, A::scale(s.ad_value(239), (-0.495)), 240);
        }

        s.store_ad(797, &A::pow_from_scalar(s.v[346], s.ad_value(234)));

        s.store_mul(231, 231, 797);

        s.store_mul(232, 232, 797);

        s.store_mul(233, 233, 797);

        if ((1.0 + (s.v[246] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 246, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(704, 244, 790);

        s.store_scaled_mul(800, 704, 187, 500000000.0);

        if ((1.0 + (s.v[247] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 247, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(705, 245, 790);

        s.store_scaled_mul(801, 705, 188, 500000000.0);

        s.v[802] = 0.0;

        s.v[1147] = if (s.v[267] > 1e-10) { 1.0 } else { 0.0 };

        if (s.v[1147] != 0.0) {
            s.store_div_from_scalar(802, 0.75, 267);
        }

        s.store_square(803, 268);

        s.store_scale(20, 2, s.v[640]);

        s.store_scale(21, 2, s.v[641]);

        s.store_scale(22, 2, s.v[642]);

        s.store_scale(23, 2, s.v[667]);

        s.store_scale(24, 2, s.v[668]);

        s.store_scale(25, 2, s.v[669]);

        s.v[26] = 0.0;

        s.v[1155] = if (p.p43 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1155] != 0.0) {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 307);

        s.v[1156] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.v[1157] = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1157] != 0.0) {
            s.store_scale(20, 2, s.v[643]);
        }

        if (s.v[1157] != 0.0) {
            s.store_sub_ad(21, A::scale(s.ad_value(2), s.v[644]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1157] != 0.0) {
            s.copy_ad(22, 27);
        }

        if (s.v[1157] != 0.0) {
            s.store_scale(23, 2, s.v[670]);
        }

        if (s.v[1157] != 0.0) {
            s.store_sub_ad(24, A::scale(s.ad_value(2), s.v[671]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1157] != 0.0) {
            s.copy_ad(25, 27);
        }

        s.v[1158] = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_ad(640, &{
                if (s.v[20] > 0.0) {
                    s.ad_value(20)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(641, &{
                if (s.v[21] > 0.0) {
                    s.ad_value(21)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(642, &{
                if (s.v[22] > 0.0) {
                    s.ad_value(22)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(667, &{
                if (s.v[23] > 0.0) {
                    s.ad_value(23)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(668, &{
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(669, &{
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(640, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(641, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(642, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(667, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(668, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(669, 0.0);
        }

        s.v[650] = 0.0;

        s.v[677] = 0.0;

        s.v[652] = 0.0;

        s.v[679] = 0.0;

        s.v[651] = 0.0;

        s.v[678] = 0.0;

        s.v[653] = 0.0;

        s.v[680] = 0.0;

        s.v[648] = 0.0;

        s.v[675] = 0.0;

        s.v[649] = 0.0;

        s.v[676] = 0.0;

        s.v[645] = 1.0;

        s.v[672] = 1.0;

        s.v[646] = 1.0;

        s.v[673] = 1.0;

        s.v[647] = 1.0;

        s.v[674] = 1.0;

        s.v[495] = 0.0;

        s.v[1159] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[1160] = if ((s.v[381] * s.v[640]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_scale_ad(448, A::ln(A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(640), s.v[381])), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_scalar(448, 100000000.0);
        }

        s.v[1161] = if ((s.v[382] * s.v[641]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scale_ad(449, A::ln(A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(641), s.v[382])), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1161] != 0.0))) {
            s.store_scalar(449, 100000000.0);
        }

        s.v[1162] = if ((s.v[383] * s.v[642]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scale_ad(450, A::ln(A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(642), s.v[383])), 1.0)), s.v[364]);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1159] != 0.0) && (!(s.v[1162] != 0.0))) {
            s.store_scalar(450, 100000000.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(648, &A::min(A::min(s.ad_value(448), s.ad_value(449)), s.ad_value(450)));
        }

        s.v[1163] = if ((((s.v[648] * s.v[365])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_exp_ad(649, A::scale(s.ad_value(648), s.v[365]));
        }

        s.v[1164] = if ((s.v[648] * s.v[365]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1163] != 0.0))) && (s.v[1164] != 0.0)) {
            s.store_div_from_scalar_ad(649, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(648), s.v[365])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(648), s.v[365])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(648), s.v[365])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1163] != 0.0))) && (!(s.v[1164] != 0.0))) {
            s.store_scale_ad(649, A::offset(A::mul(A::offset(A::scale(s.ad_value(648), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(648), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(648), s.v[365]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(390, s.v[387]);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(391, s.v[388]);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(392, s.v[389]);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(393, p.p824);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(394, p.p825);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(395, p.p826);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(396, p.p821);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(397, p.p822);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(398, p.p823);
        }

        s.v[1165] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scalar(390, (s.v[388] + s.v[389]));
        }

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scalar(393, (0.9 * (p.p825).min(p.p826)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scalar(396, (p.p822 + p.p823));
        }

        s.v[1166] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scalar(391, (s.v[387] + s.v[389]));
        }

        if ((s.v[1159] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scalar(394, (0.9 * (p.p824).min(p.p826)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scalar(397, (p.p821 + p.p823));
        }

        s.v[1167] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_scalar(392, (s.v[387] + s.v[388]));
        }

        if ((s.v[1159] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_scalar(395, (0.9 * (p.p824).min(p.p825)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_scalar(398, (p.p821 + p.p822));
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(650, &A::min(A::min(s.ad_value(390), s.ad_value(391)), s.ad_value(392)));
        }

        if (s.v[1159] != 0.0) {
            s.store_scale(651, 650, 0.1);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(371, &A::max(A::max(s.ad_value(393), s.ad_value(394)), s.ad_value(395)));
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_rhs(652, 650, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371)))));
        }

        if (s.v[1159] != 0.0) {
            s.store_offset_ad(653, A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)), (-0.05));
        }

        s.v[1168] = if ((s.v[557] * s.v[667]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1168] != 0.0)) {
            s.store_scale_ad(448, A::ln(A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(557), s.ad_value(667))), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1168] != 0.0))) {
            s.store_scalar(448, 100000000.0);
        }

        s.v[1169] = if ((s.v[558] * s.v[668]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_scale_ad(449, A::ln(A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(558), s.ad_value(668))), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1169] != 0.0))) {
            s.store_scalar(449, 100000000.0);
        }

        s.v[1170] = if ((s.v[559] * s.v[669]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_scale_ad(450, A::ln(A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(559), s.ad_value(669))), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1170] != 0.0))) {
            s.store_scalar(450, 100000000.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(675, &A::min(A::min(s.ad_value(448), s.ad_value(449)), s.ad_value(450)));
        }

        s.v[1171] = if ((((s.v[675] * s.v[365])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_exp_ad(676, A::scale(s.ad_value(675), s.v[365]));
        }

        s.v[1172] = if ((s.v[675] * s.v[365]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1171] != 0.0))) && (s.v[1172] != 0.0)) {
            s.store_div_from_scalar_ad(676, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(675), s.v[365])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(675), s.v[365])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(675), s.v[365])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1171] != 0.0))) && (!(s.v[1172] != 0.0))) {
            s.store_scale_ad(676, A::offset(A::mul(A::offset(A::scale(s.ad_value(675), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(675), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(675), s.v[365]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(390, 563);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(391, 564);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(392, 565);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(393, 505);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(394, 506);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(395, 507);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(396, 502);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(397, 503);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(398, 504);
        }

        s.v[1173] = if (s.v[667] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_add(390, 564, 565);
        }

        if ((s.v[1159] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);
        }

        if ((s.v[1159] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_add(396, 503, 504);
        }

        s.v[1174] = if (s.v[668] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_add(391, 563, 565);
        }

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);
        }

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_add(397, 502, 504);
        }

        s.v[1175] = if (s.v[669] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add(392, 563, 564);
        }

        if ((s.v[1159] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);
        }

        if ((s.v[1159] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add(398, 502, 503);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(677, &A::min(A::min(s.ad_value(390), s.ad_value(391)), s.ad_value(392)));
        }

        if (s.v[1159] != 0.0) {
            s.store_scale(678, 677, 0.1);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(371, &A::max(A::max(s.ad_value(393), s.ad_value(394)), s.ad_value(395)));
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_rhs(679, 677, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371)))));
        }

        if (s.v[1159] != 0.0) {
            s.store_offset_ad(680, A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)), (-0.05));
        }

        s.v[1176] = if (s.v[468] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scale_ad(495, A::add(A::add(A::scale(s.ad_value(640), s.v[408]), A::scale(s.ad_value(641), s.v[409])), A::scale(s.ad_value(642), s.v[410])), p.p922);
        }

        s.v[1511] = if ((s.v[640] * s.v[408]) <= s.v[495]) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1511] != 0.0)) {
            s.store_scalar(645, 0.0);
        }

        s.v[1512] = if ((s.v[641] * s.v[409]) <= s.v[495]) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1512] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        s.v[1513] = if ((s.v[642] * s.v[410]) <= s.v[495]) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1513] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_mul_ad_rhs(495, 547, A::add(A::add(A::mul(s.ad_value(667), s.ad_value(575)), A::mul(s.ad_value(668), s.ad_value(576))), A::mul(s.ad_value(669), s.ad_value(577))));
        }

        s.v[1801] = if ((s.v[667] * s.v[575]) <= s.v[495]) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1801] != 0.0)) {
            s.store_scalar(672, 0.0);
        }

        s.v[1802] = if ((s.v[668] * s.v[576]) <= s.v[495]) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1802] != 0.0)) {
            s.store_scalar(673, 0.0);
        }

        s.v[1803] = if ((s.v[669] * s.v[577]) <= s.v[495]) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1803] != 0.0)) {
            s.store_scalar(674, 0.0);
        }

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[1921] = 0.0;

        s.v[1994] = if (s.v[0] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1994] != 0.0) {
            s.store_ad(819, &A::voltage(ctx, &nodes, Some(5), Some(6)));
        }

        if (s.v[1994] != 0.0) {
            s.store_ad(820, &A::voltage(ctx, &nodes, Some(7), Some(6)));
        }

        if (s.v[1994] != 0.0) {
            s.store_ad(821, &A::voltage(ctx, &nodes, Some(6), Some(8)));
        }

        if (s.v[1994] != 0.0) {
            s.store_ad(826, &A::neg(A::voltage(ctx, &nodes, Some(6), Some(10))));
        }

        if (s.v[1994] != 0.0) {
            s.store_ad(827, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(11))));
        }

        if (!(s.v[1994] != 0.0)) {
            s.store_ad(819, &A::neg(A::voltage(ctx, &nodes, Some(5), Some(6))));
        }

        if (!(s.v[1994] != 0.0)) {
            s.store_ad(820, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(6))));
        }

        if (!(s.v[1994] != 0.0)) {
            s.store_ad(821, &A::neg(A::voltage(ctx, &nodes, Some(6), Some(8))));
        }

        if (!(s.v[1994] != 0.0)) {
            s.store_ad(826, &A::voltage(ctx, &nodes, Some(6), Some(10)));
        }

        if (!(s.v[1994] != 0.0)) {
            s.store_ad(827, &A::voltage(ctx, &nodes, Some(7), Some(11)));
        }

        s.store_add(823, 819, 821);

        s.copy_ad(828, 819);

        s.copy_ad(829, 821);

        s.store_add(830, 820, 821);

        s.store_sub(831, 819, 820);

        s.store_scale_ad(1805, A::neg(s.ad_value(828)), s.v[349]);

        s.store_scale_ad(1806, A::neg(s.ad_value(831)), s.v[349]);

        s.store_scale_ad(1807, A::neg(A::sub(s.ad_value(823), s.ad_value(694))), s.v[349]);

        s.v[825] = 1.0;

        s.v[1995] = if (s.v[820] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1995] != 0.0) {
            s.store_scalar(825, (-1.0));
        }

        if (s.v[1995] != 0.0) {
            s.store_sub(819, 819, 820);
        }

        if (s.v[1995] != 0.0) {
            s.store_add(821, 821, 820);
        }

        if (s.v[1995] != 0.0) {
            s.store_neg(820, 820);
        }

        s.store_add(822, 820, 821);

        s.store_div_ad(824, A::square(s.ad_value(820)), A::offset(A::sqrt(A::offset(A::square(s.ad_value(820)), 0.01)), 0.1));

        s.store_add_ad_lhs(1999, A::scale(A::sub(A::add(s.ad_value(822), s.ad_value(821)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(733)))), 0.5), 731);

        s.copy_ad(1808, 1999);

        s.store_add_ad_lhs(1922, A::sub(s.ad_value(821), A::scale(A::sub(s.ad_value(1999), A::sqrt(A::add(A::mul(s.ad_value(1999), s.ad_value(1999)), s.ad_value(732)))), 0.5)), 735);

        s.copy_ad(1809, 1922);

        s.v[1923] = 0.0;

        s.v[2155] = if ((p.p45 != 0.0) && (s.v[179] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[2155] != 0.0) {
            s.store_add_ad_rhs(1924, 1922, A::scale(A::sub(s.ad_value(820), s.ad_value(824)), 0.5));
        }

        if (s.v[2155] != 0.0) {
            s.store_sub_ad_lhs(1925, A::sqrt(A::add(s.ad_value(1924), s.ad_value(722))), 730);
        }

        if (s.v[2155] != 0.0) {
            s.store_offset_ad(1919, A::div(A::scale(A::sub(s.ad_value(1925), s.ad_value(737)), 2.0), s.ad_value(738)), (-1.0));
        }

        if (s.v[2155] != 0.0) {
            s.store_sub_ad_rhs(1926, 1925, A::mul(A::mul(A::scale(A::sub_from_scalar(1.0, s.ad_value(179)), 0.25), s.ad_value(738)), A::add(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 0.4804530139182)))));
        }

        if (s.v[2155] != 0.0) {
            s.store_add_ad(1927, A::square(s.ad_value(1926)), A::mul(A::scale(s.ad_value(730), 2.0), s.ad_value(1926)));
        }

        if (s.v[2155] != 0.0) {
            s.store_sub_ad_rhs(1922, 1927, A::scale(A::sub(s.ad_value(820), s.ad_value(824)), 0.5));
        }

        if (s.v[2155] != 0.0) {
            s.store_sub(1923, 1809, 1922);
        }

        s.copy_ad(1996, 722);

        s.copy_ad(1997, 732);

        s.copy_ad(1998, 723);

        s.copy_ad(2000, 1922);

        s.copy_ad(2004, 1923);

        s.copy_ad(2001, 714);

        s.copy_ad(2002, 771);

        s.store_sub_ad_lhs(2003, A::sub(s.ad_value(823), s.ad_value(2004)), 694);

        s.store_add_ad_rhs(2005, 2000, A::scale(A::sub(s.ad_value(820), s.ad_value(824)), 0.5));

        s.v[2017] = 1.0;

        s.v[2156] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2156] != 0.0) {
            s.store_scale(2008, 1996, s.v[355]);
        }

        if (s.v[2156] != 0.0) {
            s.store_scale(2009, 2005, s.v[355]);
        }

        if (s.v[2156] != 0.0) {
            s.store_scale(2010, 2003, s.v[355]);
        }

        if (s.v[2156] != 0.0) {
            s.store_offset_ad(1920, A::div(A::scale(s.ad_value(1998), 0.5), A::sqrt(s.ad_value(2008))), 1.0);
        }

        if (s.v[2156] != 0.0) {
            s.store_add_ad_rhs(1921, 2008, A::mul(s.ad_value(1998), A::sqrt(s.ad_value(2008))));
        }

        if (s.v[2156] != 0.0) {
            s.store_sub_ad(2011, A::add(A::div(A::sub(s.ad_value(2010), s.ad_value(1921)), s.ad_value(1920)), A::scale(s.ad_value(2008), 0.5)), A::mul(A::offset(s.ad_value(186), 1.0), s.ad_value(2009)));
        }

        if (s.v[2156] != 0.0) {
            s.store_offset_scaled(2012, 2008, 0.5, 2.0);
        }

        if (s.v[2156] != 0.0) {
            s.store_add(2013, 2008, 2009);
        }

        if (s.v[2156] != 0.0) {
            s.store_sub_ad(1920, A::sub(A::sub(s.ad_value(2010), s.ad_value(2013)), A::mul(s.ad_value(1998), A::sqrt(s.ad_value(2013)))), A::scale(A::ln(A::add(A::div(s.ad_value(2008), s.ad_value(1998)), A::sqrt(s.ad_value(2008)))), 2.0));
        }

        if (s.v[2156] != 0.0) {
            s.store_add_ad_lhs(2014, A::scale(s.ad_value(1920), 2.0), 2012);
        }

        if (s.v[2156] != 0.0) {
            s.store_scale_ad(1920, A::add(A::add(s.ad_value(2011), s.ad_value(2014)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2011), s.ad_value(2014)), A::sub(s.ad_value(2011), s.ad_value(2014))), 20.0))), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[2156] != 0.0) {
            s.store_sub_ad_lhs(1921, A::scale(A::sub(s.ad_value(2010), s.ad_value(2009)), 2.0), 2012);
        }

        if (s.v[2156] != 0.0) {
            s.store_scale_ad(2015, A::sub(A::add(s.ad_value(1920), s.ad_value(1921)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), s.ad_value(1921)), A::sub(s.ad_value(1920), s.ad_value(1921))), 20.0))), 0.5);
        }

        if (s.v[2156] != 0.0) {
            s.store_scale_ad(1920, A::sub(A::add(s.ad_value(2015), s.ad_value(2012)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2015), s.ad_value(2012)), A::sub(s.ad_value(2015), s.ad_value(2012))), 5.0))), 0.5);
        }

        if (s.v[2156] != 0.0) {
            s.store_scale_ad(2016, A::add(A::sub(s.ad_value(1920), s.ad_value(2012)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), A::neg(s.ad_value(2012))), A::sub(s.ad_value(1920), A::neg(s.ad_value(2012)))), 20.0))), 0.5);
        }

        if (s.v[2156] != 0.0) {
            s.store_mul_ad_rhs(1921, 696, A::offset(A::div(s.ad_value(2016), s.ad_value(2012)), 1.0));
        }

        s.v[2157] = if (s.v[1921] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((s.v[2156] != 0.0) && (s.v[2157] != 0.0)) {
            s.store_exp(2017, 1921);
        }

        if ((s.v[2156] != 0.0) && (!(s.v[2157] != 0.0))) {
            s.store_div_from_scalar_ad(2017, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.store_offset_ad(2018, A::mul(s.ad_value(695), s.ad_value(2017)), 1.0);

        s.store_scale(2019, 2018, s.v[709]);

        s.store_mul_ad(2020, A::mul(s.ad_value(194), A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0)), A::offset(A::mul(s.ad_value(195), s.ad_value(2005)), 1.0));

        s.store_mul_ad_rhs(2021, 2019, A::offset(s.ad_value(2020), 1.0));

        s.store_div_from_scalar(2022, 1.0, 2021);

        s.store_mul_ad_rhs(2006, 1998, A::sqrt(A::scale(s.ad_value(2022), s.v[709])));

        s.store_square(2007, 2006);

        s.store_div_from_scalar(2023, 1.0, 2007);

        s.store_mul(2024, 2000, 2022);

        s.store_mul(2025, 2003, 2022);

        s.store_div_ad(2026, A::scale(s.ad_value(824), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(824)), 1.0)), 1.0));

        s.store_mul_ad(2027, A::mul(s.ad_value(191), s.ad_value(2026)), A::offset(A::mul(s.ad_value(193), s.ad_value(2005)), 1.0));

        s.store_mul(2028, 1996, 2022);

        s.store_sqrt_ad(1920, A::add(A::square(s.ad_value(1999)), s.ad_value(1997)));

        s.store_sqrt_ad(1921, A::add(A::mul(A::sub(s.ad_value(1999), s.ad_value(2027)), A::sub(s.ad_value(1999), s.ad_value(2027))), s.ad_value(1997)));

        s.store_mul_ad(2029, A::scale(s.ad_value(2022), 0.5), A::sub(A::add(s.ad_value(2027), s.ad_value(1920)), s.ad_value(1921)));

        s.store_add(2030, 2028, 2024);

        s.store_sub(2031, 2030, 2029);

        s.v[2158] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2159] = if (((s.v[2031]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2158] != 0.0) && (s.v[2159] != 0.0)) {
            s.store_offset_ad(2032, A::mul(s.ad_value(2006), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2031), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2031), 0.3125))))), 1.0);
        }

        s.v[2160] = if (s.v[2031] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2158] != 0.0) && (!(s.v[2159] != 0.0))) && (s.v[2160] != 0.0)) {
            s.store_exp_ad(2046, A::neg(s.ad_value(2031)));
        }

        if (((s.v[2158] != 0.0) && (!(s.v[2159] != 0.0))) && (!(s.v[2160] != 0.0))) {
            s.store_div_from_scalar_ad(2046, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2031), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2031), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2031), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2158] != 0.0) && (!(s.v[2159] != 0.0))) {
            s.store_scalar(1919, (if (s.v[2031] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((s.v[2158] != 0.0) && (!(s.v[2159] != 0.0))) {
            s.store_offset_ad(2032, A::div(A::mul(A::mul(s.ad_value(1919), s.ad_value(2006)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2046), A::sub_from_scalar(1.0, s.ad_value(2031))))), A::scale(A::sqrt(A::mul(s.ad_value(2031), A::sub_from_scalar(1.0, s.ad_value(2046)))), 2.0)), 1.0);
        }

        if (!(s.v[2158] != 0.0)) {
            s.store_offset_ad(2032, A::div(A::scale(s.ad_value(2006), 0.5), A::sqrt(s.ad_value(2031))), 1.0);
        }

        s.store_sub_ad(2033, A::add(s.ad_value(2031), A::mul(s.ad_value(2006), A::sqrt(s.ad_value(2031)))), A::mul(s.ad_value(2032), A::ln(A::offset(s.ad_value(2032), (-1.0)))));

        s.store_div_ad_lhs(2034, A::sub(s.ad_value(2025), s.ad_value(2033)), 2032);

        s.store_mul_ad(2040, A::scale(s.ad_value(2007), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2007)), 1.0)), (-1.0)));

        s.v[2039] = 0.0;

        s.v[2041] = 1.0;

        s.v[2161] = if (s.v[2034] > (-30.0)) { 1.0 } else { 0.0 };

        if (s.v[2161] != 0.0) {
            s.store_offset_ad(2035, A::mul(s.ad_value(2032), s.ad_value(2034)), (-1.0));
        }

        if (s.v[2161] != 0.0) {
            s.store_scale_ad(1919, A::add(s.ad_value(2035), A::sqrt(A::offset(A::square(s.ad_value(2035)), 10.0))), 0.5);
        }

        if (s.v[2161] != 0.0) {
            s.store_sub_ad_rhs(2036, 2034, A::ln(s.ad_value(1919)));
        }

        if (s.v[2161] != 0.0) {
            s.store_scale_ad(2037, A::add(s.ad_value(2036), A::sqrt(A::offset(A::square(s.ad_value(2036)), 2.0))), 0.5);
        }

        s.v[2162] = if ((s.v[2034] - s.v[2037]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2161] != 0.0) && (s.v[2162] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(2034), s.ad_value(2037)));
        }

        if ((s.v[2161] != 0.0) && (!(s.v[2162] != 0.0))) {
            s.store_scale_ad(1919, A::offset(A::mul(A::offset(A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[2161] != 0.0) {
            s.store_div(2038, 1919, 2032);
        }

        if (s.v[2161] != 0.0) {
            s.store_sub_ad_lhs(1919, A::scale(A::offset(s.ad_value(2037), 1.0), 2.0), 2038);
        }

        s.v[2163] = if (s.v[2038] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[2161] != 0.0) && (s.v[2163] != 0.0)) {
            s.store_mul_ad_rhs(2039, 2032, A::offset(A::sub(s.ad_value(2037), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2038), s.ad_value(1919)), 1.0)), (-1.0)), s.ad_value(2038))), 1.0));
        }

        if ((s.v[2161] != 0.0) && (!(s.v[2163] != 0.0))) {
            s.store_mul_ad(2039, A::mul(A::scale(s.ad_value(2032), 0.5), s.ad_value(2038)), A::offset(A::mul(A::scale(s.ad_value(1919), 0.25), s.ad_value(1919)), 1.0));
        }

        if (s.v[2161] != 0.0) {
            s.store_scale_ad(1919, A::add(A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0)), A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0))), 1.0))), 0.5);
        }

        if (s.v[2161] != 0.0) {
            s.store_mul_ad(2040, A::scale(s.ad_value(2007), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2007)), s.ad_value(1919)), 1.0)), (-1.0)));
        }

        if (s.v[2161] != 0.0) {
            s.store_div_ad_rhs(2041, 2040, A::add(s.ad_value(2040), s.ad_value(2039)));
        }

        if (s.v[2161] != 0.0) {
            s.store_sub_ad_rhs(2031, 2030, A::mul(s.ad_value(2041), s.ad_value(2029)));
        }

        s.store_offset_scaled(2042, 2006, 0.7071067811865475, 1.0);

        s.store_scale(2043, 2042, 1e-5);

        s.store_div_from_scalar(2044, 1.0, 2042);

        s.v[2151] = 0.0;

        s.v[2045] = 0.0;

        s.v[2164] = if (s.v[2031] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[2164] != 0.0) {
            s.store_exp_ad(2046, A::neg(s.ad_value(2031)));
        }

        if (!(s.v[2164] != 0.0)) {
            s.store_div_from_scalar_ad(2046, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2031), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2031), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2031), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2165] = if (((s.v[2025]) as f64).abs() <= s.v[2043]) { 1.0 } else { 0.0 };

        if (s.v[2165] != 0.0) {
            s.store_scale_ad(2131, A::square(s.ad_value(2044)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (s.v[2165] != 0.0) {
            s.store_mul_ad(2045, A::mul(s.ad_value(2025), s.ad_value(2044)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2025), A::sub_from_scalar(1.0, s.ad_value(2046))), s.ad_value(2006)), s.ad_value(2131)), 1.0));
        }

        s.v[2166] = if (s.v[2025] < (-s.v[2043])) { 1.0 } else { 0.0 };

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_neg(2133, 2025);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_scaled_mul(2134, 2133, 2044, 1.25);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_scale_ad(2135, A::sub(A::offset(s.ad_value(2134), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2134), (-6.0)), A::offset(s.ad_value(2134), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub(2130, 2133, 2135);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_add_ad(2136, A::square(s.ad_value(2130)), A::mul(s.ad_value(2007), A::offset(s.ad_value(2135), 1.0)));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub_ad_lhs(2137, A::scale(s.ad_value(2130), 2.0), 2007);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub_ad_lhs(2138, A::ln(A::mul(s.ad_value(2136), s.ad_value(2023))), 2135);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_add(818, 2136, 2137);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2138), A::sub(A::scale(A::square(s.ad_value(2137)), 0.5), s.ad_value(2136))));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_add_ad_rhs(2139, 2135, A::div(A::mul(A::mul(s.ad_value(2136), s.ad_value(818)), s.ad_value(2138)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138)), s.ad_value(2138)), s.ad_value(2137)), A::sub(A::scale(A::square(s.ad_value(2137)), 0.3333333333333333), s.ad_value(2136))))));
        }

        s.v[2167] = if (s.v[2139] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) && (s.v[2167] != 0.0)) {
            s.store_exp(2140, 2139);
        }

        if (((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) && (!(s.v[2167] != 0.0))) {
            s.store_scale_ad(2140, A::offset(A::mul(A::offset(s.ad_value(2139), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2139), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2139), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_div_from_scalar(2141, 1.0, 2140);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_div_from_scalar_ad(2130, 1.0, A::offset(A::square(s.ad_value(2139)), 2.0));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_mul_ad_lhs(2142, A::square(s.ad_value(2139)), 2130);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_scale_ad(2143, A::mul(A::mul(s.ad_value(2139), s.ad_value(2130)), s.ad_value(2130)), 4.0);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_mul_ad_lhs(2144, A::mul(A::sub(A::scale(s.ad_value(2130), 8.0), A::scale(s.ad_value(2142), 12.0)), s.ad_value(2130)), 2130);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub(2130, 2133, 2139);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_mul(2131, 2046, 2141);
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_add_ad(2145, A::scale(s.ad_value(2130), 2.0), A::mul(s.ad_value(2007), A::add(A::sub(A::offset(s.ad_value(2140), (-1.0)), s.ad_value(2131)), A::mul(s.ad_value(2046), A::sub_from_scalar(1.0, s.ad_value(2143))))));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub_ad(2146, A::square(s.ad_value(2130)), A::mul(s.ad_value(2007), A::add(A::add(A::offset(A::sub(s.ad_value(2140), s.ad_value(2139)), (-1.0)), s.ad_value(2131)), A::mul(s.ad_value(2046), A::sub(A::offset(s.ad_value(2139), (-1.0)), s.ad_value(2142))))));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub_from_scalar_ad(2130, 2.0, A::mul(s.ad_value(2007), A::sub(A::add(s.ad_value(2140), s.ad_value(2131)), A::mul(s.ad_value(2046), s.ad_value(2144)))));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub_ad(2130, A::square(s.ad_value(2145)), A::scale(A::mul(s.ad_value(2146), s.ad_value(2130)), 2.0));
        }

        if ((!(s.v[2165] != 0.0)) && (s.v[2166] != 0.0)) {
            s.store_sub_ad(2045, A::neg(s.ad_value(2139)), A::scale(A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_div_from_scalar_ad(2147, 1.0, A::offset(A::scale(s.ad_value(2006), 0.7324648775608221), 1.25));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_mul_ad_lhs(2148, A::offset(A::mul(A::scale(s.ad_value(2042), 1.25), s.ad_value(2147)), (-1.0)), 2147);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_mul_ad(2149, A::mul(s.ad_value(2025), s.ad_value(2044)), A::offset(A::mul(s.ad_value(2148), s.ad_value(2025)), 1.0));
        }

        s.v[2168] = if ((-s.v[2149]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (s.v[2168] != 0.0)) {
            s.store_exp_ad(2130, A::neg(s.ad_value(2149)));
        }

        if (((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (!(s.v[2168] != 0.0))) {
            s.store_div_from_scalar_ad(2130, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2149))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2149))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2149))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_from_scalar(2150, 1.0, 2130);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_ad(2151, A::add(s.ad_value(2025), A::scale(s.ad_value(2007), 0.5)), A::mul(s.ad_value(2006), A::sqrt(A::sub(A::add(s.ad_value(2025), A::scale(s.ad_value(2007), 0.25)), s.ad_value(2150)))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_offset(2152, 2031, 3.0);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_ad(2135, A::scale(A::sub(A::add(s.ad_value(2151), s.ad_value(2152)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2151), s.ad_value(2152)), A::sub(s.ad_value(2151), s.ad_value(2152))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2152), A::sqrt(A::offset(A::square(s.ad_value(2152)), 5.0))), 0.5));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub(2130, 2025, 2135);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_exp_ad(2131, A::neg(s.ad_value(2135)));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_div_from_scalar_ad(2132, 1.0, A::offset(A::square(s.ad_value(2135)), 2.0));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_mul_ad_lhs(2142, A::square(s.ad_value(2135)), 2132);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_scale_ad(2143, A::mul(A::mul(s.ad_value(2135), s.ad_value(2132)), s.ad_value(2132)), 4.0);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_mul_ad_lhs(2144, A::mul(A::sub(A::scale(s.ad_value(2132), 8.0), A::scale(s.ad_value(2142), 12.0)), s.ad_value(2132)), 2132);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            let assign42040_ad_e55199: A = {
                if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2046] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2130)), A::mul(s.ad_value(2007), A::sub(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), A::mul(s.ad_value(2046), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142))))))
                }
            };
            s.store_ad(2136, &assign42040_ad_e55199);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_from_scalar_ad(2153, 1.0, A::scale(A::mul(s.ad_value(2007), A::sub(s.ad_value(2131), A::mul(s.ad_value(2046), s.ad_value(2144)))), 0.5));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_add_ad(2137, A::scale(s.ad_value(2130), 2.0), A::mul(s.ad_value(2007), A::sub(A::sub_from_scalar(1.0, s.ad_value(2131)), A::mul(s.ad_value(2046), A::offset(s.ad_value(2143), 1.0)))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_add_ad(2138, A::sub(s.ad_value(2031), s.ad_value(2135)), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_add(818, 2136, 2137);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2138), A::sub(A::scale(A::square(s.ad_value(2137)), 0.5), A::mul(s.ad_value(2136), s.ad_value(2153)))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            let assign42100_ad_e55322: A = A::add(s.ad_value(2135), A::div(A::mul(A::mul(s.ad_value(2136), s.ad_value(818)), s.ad_value(2138)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138)), s.ad_value(2138)), s.ad_value(2137)), A::sub(A::scale(A::square(s.ad_value(2137)), 0.3333333333333333), A::mul(s.ad_value(2136), s.ad_value(2153)))))));
            s.store_ad(2154, &assign42100_ad_e55322);
        }

        s.v[2169] = if (s.v[2154] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (s.v[2169] != 0.0)) {
            s.store_exp(2140, 2154);
        }

        if (((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (s.v[2169] != 0.0)) {
            s.store_div_from_scalar(2141, 1.0, 2140);
        }

        if (((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (s.v[2169] != 0.0)) {
            s.store_mul(2140, 2046, 2140);
        }

        s.v[2170] = if (s.v[2154] > (s.v[2031] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (!(s.v[2169] != 0.0))) && (s.v[2170] != 0.0)) {
            s.store_exp_ad(2140, A::sub(s.ad_value(2154), s.ad_value(2031)));
        }

        if ((((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (!(s.v[2169] != 0.0))) && (s.v[2170] != 0.0)) {
            s.store_div(2141, 2046, 2140);
        }

        if ((((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (!(s.v[2169] != 0.0))) && (!(s.v[2170] != 0.0))) {
            s.store_div_from_scalar_ad(2140, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) && (!(s.v[2169] != 0.0))) && (!(s.v[2170] != 0.0))) {
            s.store_div_from_scalar_ad(2141, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2154), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2154), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2154), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_div_from_scalar_ad(2130, 1.0, A::offset(A::square(s.ad_value(2154)), 2.0));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_mul_ad_lhs(2142, A::square(s.ad_value(2154)), 2130);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_scale_ad(2143, A::mul(A::mul(s.ad_value(2154), s.ad_value(2130)), s.ad_value(2130)), 4.0);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_mul_ad_lhs(2144, A::mul(A::sub(A::scale(s.ad_value(2130), 8.0), A::scale(s.ad_value(2142), 12.0)), s.ad_value(2130)), 2130);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub(2130, 2025, 2154);
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_add_ad(2145, A::scale(s.ad_value(2130), 2.0), A::mul(s.ad_value(2007), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2141)), s.ad_value(2140)), A::mul(s.ad_value(2046), A::offset(s.ad_value(2143), 1.0)))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_ad(2146, A::square(s.ad_value(2130)), A::mul(s.ad_value(2007), A::sub(A::add(A::offset(A::add(s.ad_value(2141), s.ad_value(2154)), (-1.0)), s.ad_value(2140)), A::mul(s.ad_value(2046), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142))))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_from_scalar_ad(2130, 2.0, A::mul(s.ad_value(2007), A::sub(A::add(s.ad_value(2141), s.ad_value(2140)), A::mul(s.ad_value(2046), s.ad_value(2144)))));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_sub_ad(2130, A::square(s.ad_value(2145)), A::scale(A::mul(s.ad_value(2146), s.ad_value(2130)), 2.0));
        }

        if ((!(s.v[2165] != 0.0)) && (!(s.v[2166] != 0.0))) {
            s.store_add_ad_rhs(2045, 2154, A::scale(A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0));
        }

        s.v[2048] = 0.0;

        s.v[2049] = 0.0;

        s.v[2050] = 0.0;

        s.v[2051] = 0.0;

        s.v[2052] = 0.0;

        s.v[2053] = 0.0;

        s.v[2054] = 0.0;

        s.v[2055] = 1.0;

        s.v[2056] = 1.0;

        s.store_sub(2057, 2025, 2045);

        s.v[2058] = 0.0;

        s.store_mul(2059, 2021, 2057);

        s.v[2060] = 1.0;

        s.v[2061] = 1.0;

        s.v[2065] = 1.0;

        s.v[2066] = 1.0;

        s.v[2068] = 1.0;

        s.v[2171] = if (s.v[2025] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2171] != 0.0) {
            s.store_div_from_scalar_ad(1919, 1.0, A::offset(A::square(s.ad_value(2045)), 2.0));
        }

        if (s.v[2171] != 0.0) {
            s.store_mul_ad_lhs(2047, A::square(s.ad_value(2045)), 1919);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[2171] != 0.0) {
            s.store_scale_ad(2048, A::mul(A::mul(s.ad_value(2045), s.ad_value(1919)), s.ad_value(1919)), 4.0);
        }

        if (s.v[2171] != 0.0) {
            s.store_mul_ad_lhs(2049, A::mul(A::sub(A::scale(s.ad_value(1919), 8.0), A::scale(s.ad_value(2047), 12.0)), s.ad_value(1919)), 1919);
        }

        if (s.v[2171] != 0.0) {
            s.store_scalar(2050, 0.0);
        }

        s.v[2172] = if (s.v[2045] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2171] != 0.0) && (s.v[2172] != 0.0)) {
            s.store_exp(2050, 2045);
        }

        if ((s.v[2171] != 0.0) && (s.v[2172] != 0.0)) {
            s.store_div_from_scalar(2051, 1.0, 2050);
        }

        if ((s.v[2171] != 0.0) && (s.v[2172] != 0.0)) {
            s.store_mul(2050, 2046, 2050);
        }

        s.v[2173] = if (s.v[2045] > (s.v[2031] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2171] != 0.0) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_exp_ad(2050, A::sub(s.ad_value(2045), s.ad_value(2031)));
        }

        if (((s.v[2171] != 0.0) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_div(2051, 2046, 2050);
        }

        if (((s.v[2171] != 0.0) && (!(s.v[2172] != 0.0))) && (!(s.v[2173] != 0.0))) {
            s.store_div_from_scalar_ad(2050, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2171] != 0.0) && (!(s.v[2172] != 0.0))) && (!(s.v[2173] != 0.0))) {
            s.store_div_from_scalar_ad(2051, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2045), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2045), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2045), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (s.v[2171] != 0.0) {
            s.store_sub_ad_rhs(2052, 2050, A::mul(s.ad_value(2046), A::add(A::offset(s.ad_value(2045), 1.0), s.ad_value(2047))));
        }

        s.v[2174] = if (s.v[2045] < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2171] != 0.0) && (s.v[2174] != 0.0)) {
            s.store_scale_ad(2053, A::mul(A::square(s.ad_value(2045)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2045), A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((s.v[2171] != 0.0) && (s.v[2174] != 0.0)) {
            s.store_scale_ad(2052, A::mul(A::mul(A::mul(A::mul(s.ad_value(2046), s.ad_value(2045)), s.ad_value(2045)), s.ad_value(2045)), A::offset(A::scale(s.ad_value(2045), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((s.v[2171] != 0.0) && (s.v[2174] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2045), A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.25))), 0.3333333333333333)));
        }

        if ((s.v[2171] != 0.0) && (s.v[2174] != 0.0)) {
            s.store_scaled_mul(2054, 2045, 1919, 0.7071067811865475);
        }

        if ((s.v[2171] != 0.0) && (s.v[2174] != 0.0)) {
            s.store_offset_ad(2055, A::scale(A::div(A::mul(s.ad_value(2006), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.5)), A::scale(A::square(s.ad_value(2045)), 0.16666666666666666))), s.ad_value(1919)), 0.7071067811865475), 1.0);
        }

        if ((s.v[2171] != 0.0) && (!(s.v[2174] != 0.0))) {
            s.store_add_ad_lhs(2053, A::offset(s.ad_value(2045), (-1.0)), 2051);
        }

        if ((s.v[2171] != 0.0) && (!(s.v[2174] != 0.0))) {
            s.store_sqrt(2054, 2053);
        }

        if ((s.v[2171] != 0.0) && (!(s.v[2174] != 0.0))) {
            s.store_offset_ad(2055, A::scale(A::div(A::mul(s.ad_value(2006), A::sub_from_scalar(1.0, s.ad_value(2051))), s.ad_value(2054)), 0.5), 1.0);
        }

        if (s.v[2171] != 0.0) {
            s.store_div_ad(2056, A::offset(A::mul(A::scale(s.ad_value(702), 0.2), s.ad_value(2005)), 1.0), A::offset(A::mul(s.ad_value(702), s.ad_value(2005)), 1.0));
        }

        s.v[2175] = if (s.v[2052] > 1e-100) { 1.0 } else { 0.0 };

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_mul_ad_rhs(2057, 2006, A::sqrt(A::add(s.ad_value(2053), s.ad_value(2052))));
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_div_ad(2058, A::mul(A::mul(s.ad_value(2007), s.ad_value(2052)), s.ad_value(2021)), A::add(s.ad_value(2057), A::mul(s.ad_value(2006), s.ad_value(2054))));
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_mul_ad_lhs(2059, A::mul(s.ad_value(2054), s.ad_value(2006)), 2021);
        }

        s.v[2176] = if (s.v[212] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (s.v[2176] != 0.0)) {
            s.store_div_from_scalar_ad(2060, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(212), s.ad_value(2005))));
        }

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (!(s.v[2176] != 0.0))) {
            s.store_offset_ad(2060, A::mul(s.ad_value(212), s.ad_value(2005)), 1.0);
        }

        s.v[2177] = if (s.v[213] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (s.v[2177] != 0.0)) {
            s.store_sub_from_scalar_ad(2061, 1.0, A::mul(s.ad_value(213), s.ad_value(2058)));
        }

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (!(s.v[2177] != 0.0))) {
            s.store_div_from_scalar_ad(2061, 1.0, A::offset(A::mul(s.ad_value(213), s.ad_value(2058)), 1.0));
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_mul_ad_lhs(2062, A::mul(A::mul(s.ad_value(751), s.ad_value(2060)), s.ad_value(2061)), 2058);
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_mul_ad_rhs(2063, 768, A::add(s.ad_value(2059), A::mul(s.ad_value(769), s.ad_value(2058))));
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_ln_ad(1920, A::div(s.ad_value(2053), A::offset(A::add(s.ad_value(2053), s.ad_value(2052)), 1e-14)));
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_add_ad(2064, A::pow(A::mul(s.ad_value(2063), s.ad_value(698)), s.ad_value(699)), A::mul(s.ad_value(700), A::exp(A::mul(A::scale(s.ad_value(701), 0.5), s.ad_value(1920)))));
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_mul_ad_lhs(2065, A::add(A::offset(s.ad_value(2064), 1.0), s.ad_value(2062)), 2056);
        }

        s.v[2178] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (s.v[2178] != 0.0)) {
            s.store_div_from_scalar_ad(2066, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(2005))));
        }

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (!(s.v[2178] != 0.0))) {
            s.store_offset_ad(2066, A::mul(s.ad_value(216), s.ad_value(2005)), 1.0);
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_mul(1921, 2058, 2066);
        }

        if ((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) {
            s.store_div_ad_rhs(2067, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.v[2179] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (s.v[2179] != 0.0)) {
            s.store_div_from_scalar_ad(2068, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2067))));
        }

        if (((s.v[2171] != 0.0) && (s.v[2175] != 0.0)) && (!(s.v[2179] != 0.0))) {
            s.store_offset_ad(2068, A::mul(s.ad_value(217), s.ad_value(2067)), 1.0);
        }

        s.copy_ad(1810, 2003);

        s.copy_ad(1811, 2005);

        s.copy_ad(1812, 2021);

        s.copy_ad(1813, 2022);

        s.copy_ad(1814, 2006);

        s.copy_ad(1815, 2007);

        s.copy_ad(1816, 2023);

        s.copy_ad(1817, 2025);

        s.copy_ad(1818, 2030);

        s.copy_ad(1819, 2031);

        s.copy_ad(1820, 2042);

        s.copy_ad(1821, 2043);

        s.copy_ad(1822, 2044);

        s.copy_ad(1823, 2151);

        s.copy_ad(1824, 2046);

        s.copy_ad(1825, 2045);

        s.copy_ad(1826, 2048);

        s.copy_ad(1827, 2049);

        s.copy_ad(1828, 2050);

        s.copy_ad(1829, 2051);

        s.copy_ad(1830, 2053);

        s.copy_ad(1831, 2052);

        s.copy_ad(1832, 2054);

        s.copy_ad(1833, 2055);

        s.copy_ad(1834, 2056);

        s.copy_ad(1835, 2057);

        s.copy_ad(1836, 2058);

        s.copy_ad(1837, 2059);

        s.copy_ad(1838, 2060);

        s.copy_ad(1839, 2061);

        s.copy_ad(1840, 2065);

        s.copy_ad(1841, 2066);

        s.copy_ad(1842, 2068);

        s.v[2070] = 0.0;

        s.store_scale(2069, 2021, 4.60517018598809);

        s.copy_ad(2086, 2069);

        s.copy_ad(2087, 820);

        s.store_mul(2088, 820, 2022);

        s.copy_ad(2092, 2045);

        s.v[2093] = 0.0;

        s.v[2096] = 0.0;

        s.copy_ad(2098, 2051);

        s.copy_ad(2099, 2053);

        s.copy_ad(2101, 2052);

        s.copy_ad(2102, 2059);

        s.copy_ad(2103, 2045);

        s.copy_ad(2104, 2051);

        s.copy_ad(2106, 2052);

        s.copy_ad(2107, 2053);

        s.store_sub(2108, 2025, 2045);

        s.v[2109] = 1.0;

        s.v[2111] = 1.0;

        s.v[2110] = 0.0;

        s.copy_ad(2120, 2058);

        s.store_mul(2124, 2108, 2021);

        s.v[2121] = 0.0;

        s.copy_ad(2122, 2059);

        s.v[2127] = 0.0;

        s.v[2126] = 1.0;

        s.copy_ad(2129, 2001);

        s.copy_ad(2128, 2124);

        s.v[2180] = if (s.v[2025] > 0.0) { 1.0 } else { 0.0 };

        s.v[2181] = if (s.v[2052] > 1e-100) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_mul(2129, 2001, 2068);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_div(2070, 2129, 2065);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_add_ad_rhs(2071, 2057, A::scale(s.ad_value(2007), 0.5));
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_div_ad_lhs(1919, A::div(A::mul(s.ad_value(2007), s.ad_value(2050)), s.ad_value(2071)), 2071);
        }

        s.v[2182] = if (s.v[1919] > 0.0001) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2182] != 0.0)) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.v[2183] = if (s.v[1920] < 1e-10) { 1.0 } else { 0.0 };

        if ((((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2182] != 0.0)) && (s.v[2183] != 0.0)) {
            s.store_scalar(1921, 1.0);
        }

        if ((((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2182] != 0.0)) && (!(s.v[2183] != 0.0))) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (!(s.v[2182] != 0.0))) {
            s.store_scale(1921, 1919, 0.5);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_mul(2072, 1921, 2071);
        }

        s.v[2184] = if ((s.v[700] > 0.0) && (s.v[701] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_mul_ad_lhs(2073, A::scale(s.ad_value(2021), 0.475), 2072);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_sub_ad_rhs(1919, 2058, A::mul(s.ad_value(2055), s.ad_value(2073)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_scale_ad(2074, A::add(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 1e-12))), 0.5);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_add_ad(2075, A::sub(A::mul(s.ad_value(2021), s.ad_value(2057)), s.ad_value(2058)), A::mul(A::offset(s.ad_value(2055), (-1.0)), s.ad_value(2073)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_offset_ad(2076, A::div(A::mul(A::scale(s.ad_value(2007), 0.5), s.ad_value(2021)), s.ad_value(2075)), 1.0);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_add_ad_rhs(1919, 2075, A::mul(s.ad_value(769), s.ad_value(2074)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_ad(2077, &A::pow(A::mul(A::mul(s.ad_value(768), s.ad_value(1919)), s.ad_value(698)), s.ad_value(699)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_mul_ad_lhs(1920, A::div(A::mul(s.ad_value(699), A::offset(A::mul(s.ad_value(2076), A::sub_from_scalar(1.0, s.ad_value(769))), (-1.0))), s.ad_value(1919)), 2077);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_div(1919, 2074, 2075);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_mul_ad_rhs(2078, 700, A::pow(A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701))));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_mul_ad_lhs(1921, A::div(A::mul(s.ad_value(701), A::add(A::offset(s.ad_value(2076), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1919), 1.0)))), s.ad_value(2075)), 2078);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_mul_ad_lhs(2079, A::mul(A::mul(s.ad_value(751), s.ad_value(2060)), s.ad_value(2061)), 2074);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_offset_ad(1919, A::div(A::sub(s.ad_value(1920), A::mul(A::mul(A::mul(s.ad_value(751), s.ad_value(2060)), s.ad_value(2061)), s.ad_value(2076))), s.ad_value(1921)), 1.0);
        }

        s.v[2185] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) && (s.v[2185] != 0.0)) {
            s.store_scale_ad(1920, A::ln(A::offset(A::exp(A::scale(s.ad_value(1919), 2.0)), 1.0)), 0.5);
        }

        if ((((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) && (!(s.v[2185] != 0.0))) {
            s.copy_ad(1920, 1919);
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_div_ad(2080, A::mul(A::mul(A::neg(s.ad_value(2073)), s.ad_value(1921)), s.ad_value(1920)), A::add(A::add(A::offset(s.ad_value(2077), 1.0), s.ad_value(2078)), s.ad_value(2079)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2184] != 0.0)) {
            s.store_mul_ad_rhs(2081, 2072, A::offset(A::div(s.ad_value(2080), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2080)), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (!(s.v[2184] != 0.0))) {
            s.copy_ad(2081, 2072);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_scale_ad(2082, A::mul(A::mul(s.ad_value(2021), s.ad_value(2070)), s.ad_value(2081)), 0.7071067811865475);
        }

        s.v[2186] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) && (s.v[2186] != 0.0)) {
            s.store_div_ad_rhs(2082, 2082, A::sqrt(A::offset(s.ad_value(2082), 1.0)));
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_div_from_scalar_ad(2083, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2082), 4.0), 1.0)), 1.0));
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_mul(1919, 2083, 2082);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_mul_ad(2084, A::mul(s.ad_value(2081), s.ad_value(2083)), A::offset(A::div(A::mul(A::scale(s.ad_value(1919), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(1919), s.ad_value(2083)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(1919), 4.0), s.ad_value(1919)), s.ad_value(2083)), 1.0)), 1.0));
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_scale(2085, 2084, 0.99);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_div_ad_lhs(1919, A::mul(A::mul(s.ad_value(2085), A::sub(s.ad_value(2085), A::scale(s.ad_value(2071), 2.0))), s.ad_value(2023)), 2052);
        }

        if ((s.v[2180] != 0.0) && (s.v[2181] != 0.0)) {
            s.store_mul_ad_rhs(2086, 2021, A::sub(s.ad_value(2085), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2181] != 0.0))) {
            s.copy_ad(2086, 2069);
        }

        if (s.v[2180] != 0.0) {
            s.store_offset(1919, 2002, 1.0);
        }

        if (s.v[2180] != 0.0) {
            s.store_div_ad_lhs(1920, A::mul(A::sqrt(s.ad_value(1919)), s.ad_value(820)), 2086);
        }

        if (s.v[2180] != 0.0) {
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
        }

        if (s.v[2180] != 0.0) {
            s.store_scale(1919, 1920, 2.0);
        }

        if (s.v[2180] != 0.0) {
            s.store_div_ad(2087, A::mul(s.ad_value(2086), s.ad_value(1919)), A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(2088, 2087, 2022);
        }

        if (s.v[2180] != 0.0) {
            s.store_add(2089, 2031, 2088);
        }

        s.v[2187] = if (s.v[2088] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2187] != 0.0)) {
            s.store_exp_ad(2090, A::neg(s.ad_value(2088)));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2187] != 0.0))) {
            s.store_div_from_scalar_ad(2090, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2088), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2088), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2088), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(2091, 2046, 2090);
        }

        s.v[2188] = if (((s.v[2025]) as f64).abs() <= s.v[2043]) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_scale_ad(2131, A::square(s.ad_value(2044)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((s.v[2180] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_mul_ad(2092, A::mul(s.ad_value(2025), s.ad_value(2044)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2025), A::sub_from_scalar(1.0, s.ad_value(2091))), s.ad_value(2006)), s.ad_value(2131)), 1.0));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_offset(2152, 2089, 3.0);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub_ad(2135, A::scale(A::sub(A::add(s.ad_value(2151), s.ad_value(2152)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2151), s.ad_value(2152)), A::sub(s.ad_value(2151), s.ad_value(2152))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2152), A::sqrt(A::offset(A::square(s.ad_value(2152)), 5.0))), 0.5));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub(2130, 2025, 2135);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_exp_ad(2131, A::neg(s.ad_value(2135)));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_div_from_scalar_ad(2132, 1.0, A::offset(A::square(s.ad_value(2135)), 2.0));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_mul_ad_lhs(2142, A::square(s.ad_value(2135)), 2132);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_scale_ad(2143, A::mul(A::mul(s.ad_value(2135), s.ad_value(2132)), s.ad_value(2132)), 4.0);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_mul_ad_lhs(2144, A::mul(A::sub(A::scale(s.ad_value(2132), 8.0), A::scale(s.ad_value(2142), 12.0)), s.ad_value(2132)), 2132);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            let assign44220_ad_e57143: A = {
                if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2091] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2130)), A::mul(s.ad_value(2007), A::sub(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), A::mul(s.ad_value(2091), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142))))))
                }
            };
            s.store_ad(2136, &assign44220_ad_e57143);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub_from_scalar_ad(2153, 1.0, A::scale(A::mul(s.ad_value(2007), A::sub(s.ad_value(2131), A::mul(s.ad_value(2091), s.ad_value(2144)))), 0.5));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add_ad(2137, A::scale(s.ad_value(2130), 2.0), A::mul(s.ad_value(2007), A::sub(A::sub_from_scalar(1.0, s.ad_value(2131)), A::mul(s.ad_value(2091), A::offset(s.ad_value(2143), 1.0)))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add_ad(2138, A::sub(s.ad_value(2089), s.ad_value(2135)), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add(818, 2136, 2137);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2138), A::sub(A::scale(A::square(s.ad_value(2137)), 0.5), A::mul(s.ad_value(2136), s.ad_value(2153)))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            let assign44280_ad_e57260: A = A::add(s.ad_value(2135), A::div(A::mul(A::mul(s.ad_value(2136), s.ad_value(818)), s.ad_value(2138)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138)), s.ad_value(2138)), s.ad_value(2137)), A::sub(A::scale(A::square(s.ad_value(2137)), 0.3333333333333333), A::mul(s.ad_value(2136), s.ad_value(2153)))))));
            s.store_ad(2154, &assign44280_ad_e57260);
        }

        s.v[2189] = if (s.v[2154] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (s.v[2189] != 0.0)) {
            s.store_exp(2140, 2154);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (s.v[2189] != 0.0)) {
            s.store_div_from_scalar(2141, 1.0, 2140);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (s.v[2189] != 0.0)) {
            s.store_mul(2140, 2091, 2140);
        }

        s.v[2190] = if (s.v[2154] > (s.v[2089] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (!(s.v[2189] != 0.0))) && (s.v[2190] != 0.0)) {
            s.store_exp_ad(2140, A::sub(s.ad_value(2154), s.ad_value(2089)));
        }

        if ((((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (!(s.v[2189] != 0.0))) && (s.v[2190] != 0.0)) {
            s.store_div(2141, 2091, 2140);
        }

        if ((((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (!(s.v[2189] != 0.0))) && (!(s.v[2190] != 0.0))) {
            s.store_div_from_scalar_ad(2140, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) && (!(s.v[2189] != 0.0))) && (!(s.v[2190] != 0.0))) {
            s.store_div_from_scalar_ad(2141, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2154), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2154), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2154), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_div_from_scalar_ad(2130, 1.0, A::offset(A::square(s.ad_value(2154)), 2.0));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_mul_ad_lhs(2142, A::square(s.ad_value(2154)), 2130);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_scale_ad(2143, A::mul(A::mul(s.ad_value(2154), s.ad_value(2130)), s.ad_value(2130)), 4.0);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_mul_ad_lhs(2144, A::mul(A::sub(A::scale(s.ad_value(2130), 8.0), A::scale(s.ad_value(2142), 12.0)), s.ad_value(2130)), 2130);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub(2130, 2025, 2154);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add_ad(2145, A::scale(s.ad_value(2130), 2.0), A::mul(s.ad_value(2007), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2141)), s.ad_value(2140)), A::mul(s.ad_value(2091), A::offset(s.ad_value(2143), 1.0)))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub_ad(2146, A::square(s.ad_value(2130)), A::mul(s.ad_value(2007), A::sub(A::add(A::offset(A::add(s.ad_value(2141), s.ad_value(2154)), (-1.0)), s.ad_value(2140)), A::mul(s.ad_value(2091), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142))))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub_from_scalar_ad(2130, 2.0, A::mul(s.ad_value(2007), A::sub(A::add(s.ad_value(2141), s.ad_value(2140)), A::mul(s.ad_value(2091), s.ad_value(2144)))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sub_ad(2130, A::square(s.ad_value(2145)), A::scale(A::mul(s.ad_value(2146), s.ad_value(2130)), 2.0));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add_ad_rhs(2092, 2154, A::scale(A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0));
        }

        if (s.v[2180] != 0.0) {
            s.store_sub(2093, 2092, 2045);
        }

        s.v[2191] = if (s.v[2093] < 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2191] != 0.0)) {
            s.store_add_ad(2094, A::scale(A::sub(s.ad_value(2025), s.ad_value(2045)), 2.0), A::mul(s.ad_value(2007), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2051)), A::mul(s.ad_value(2050), s.ad_value(2090))), A::mul(s.ad_value(2091), A::offset(s.ad_value(2048), 1.0)))));
        }

        if ((s.v[2180] != 0.0) && (s.v[2191] != 0.0)) {
            s.store_mul_ad_lhs(2095, A::mul(s.ad_value(2007), A::sub_from_scalar(1.0, s.ad_value(2090))), 2052);
        }

        if ((s.v[2180] != 0.0) && (s.v[2191] != 0.0)) {
            s.store_sub_from_scalar_ad(1919, 2.0, A::mul(s.ad_value(2007), A::sub(A::add(s.ad_value(2051), A::mul(s.ad_value(2050), s.ad_value(2090))), A::mul(s.ad_value(2091), s.ad_value(2049)))));
        }

        if ((s.v[2180] != 0.0) && (s.v[2191] != 0.0)) {
            s.store_sub_ad(1919, A::square(s.ad_value(2094)), A::scale(A::mul(s.ad_value(1919), s.ad_value(2095)), 2.0));
        }

        if ((s.v[2180] != 0.0) && (s.v[2191] != 0.0)) {
            s.store_scale_ad(2093, A::div(s.ad_value(2095), A::add(s.ad_value(2094), A::sqrt(s.ad_value(1919)))), 2.0);
        }

        if ((s.v[2180] != 0.0) && (s.v[2191] != 0.0)) {
            s.store_add(2092, 2045, 2093);
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(2096, 2093, 2021);
        }

        if (s.v[2180] != 0.0) {
            s.store_div_ad(2097, A::square(s.ad_value(2092)), A::offset(A::square(s.ad_value(2092)), 2.0));
        }

        s.v[2192] = if (s.v[2092] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) {
            s.store_exp_ad(2098, A::neg(s.ad_value(2092)));
        }

        s.v[2193] = if (s.v[2092] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (s.v[2193] != 0.0)) {
            s.store_scale_ad(2099, A::mul(A::square(s.ad_value(2092)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2092), A::sub_from_scalar(1.0, A::scale(s.ad_value(2092), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (s.v[2193] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2092), A::sub_from_scalar(1.0, A::scale(s.ad_value(2092), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (s.v[2193] != 0.0)) {
            s.store_scaled_mul(2100, 2092, 1919, 0.7071067811865475);
        }

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (s.v[2193] != 0.0)) {
            s.store_mul_ad(2101, A::mul(A::mul(A::mul(A::scale(s.ad_value(2091), 0.16666666666666666), s.ad_value(2092)), s.ad_value(2092)), s.ad_value(2092)), A::offset(A::scale(s.ad_value(2092), 1.75), 1.0));
        }

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (!(s.v[2193] != 0.0))) {
            s.store_add_ad_lhs(2099, A::offset(s.ad_value(2092), (-1.0)), 2098);
        }

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (!(s.v[2193] != 0.0))) {
            s.store_sqrt(2100, 2099);
        }

        if (((s.v[2180] != 0.0) && (s.v[2192] != 0.0)) && (!(s.v[2193] != 0.0))) {
            s.store_mul_ad_rhs(2101, 2091, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2098)), s.ad_value(2092)), (-1.0)), s.ad_value(2097)));
        }

        s.v[2194] = if (s.v[2092] > (s.v[2089] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) && (s.v[2194] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(2092), s.ad_value(2089)));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) && (s.v[2194] != 0.0)) {
            s.store_div(2098, 2091, 1919);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) && (s.v[2194] != 0.0)) {
            s.store_sub_ad_rhs(2101, 1919, A::mul(s.ad_value(2091), A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097))));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) && (!(s.v[2194] != 0.0))) {
            s.store_div_from_scalar_ad(2098, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2092), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2092), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2092), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) && (!(s.v[2194] != 0.0))) {
            s.store_div_from_scalar_ad(1919, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) && (!(s.v[2194] != 0.0))) {
            s.store_sub_ad_rhs(2101, 1919, A::mul(s.ad_value(2091), A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) {
            s.store_add_ad_lhs(2099, A::offset(s.ad_value(2092), (-1.0)), 2098);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2192] != 0.0))) {
            s.store_sqrt(2100, 2099);
        }

        if (s.v[2180] != 0.0) {
            s.store_mul_ad_lhs(2102, A::mul(s.ad_value(2100), s.ad_value(2006)), 2021);
        }

        if (s.v[2180] != 0.0) {
            s.store_scaled_add(2103, 2045, 2092, 0.5);
        }

        if (s.v[2180] != 0.0) {
            s.store_scalar(2104, 0.0);
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(1919, 2098, 2051);
        }

        s.v[2195] = if (s.v[1919] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_sqrt(2104, 1919);
        }

        if (s.v[2180] != 0.0) {
            s.store_scaled_add(2105, 2052, 2101, 0.5);
        }

        if (s.v[2180] != 0.0) {
            s.store_add_ad_rhs(2106, 2105, A::scale(A::mul(A::square(s.ad_value(2093)), A::sub(s.ad_value(2104), A::scale(s.ad_value(2023), 2.0))), 0.125));
        }

        s.v[2196] = if (s.v[2103] < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2196] != 0.0)) {
            s.store_scale_ad(2107, A::mul(A::square(s.ad_value(2103)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2103), A::sub_from_scalar(1.0, A::scale(s.ad_value(2103), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((s.v[2180] != 0.0) && (s.v[2196] != 0.0)) {
            s.store_mul_ad_rhs(2108, 2006, A::sqrt(A::add(s.ad_value(2106), s.ad_value(2107))));
        }

        s.v[2197] = if (s.v[724] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (s.v[2196] != 0.0)) && (s.v[2197] != 0.0)) {
            s.store_div_from_scalar_ad(2109, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0)));
        }

        if ((s.v[2180] != 0.0) && (s.v[2196] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2103), A::sub_from_scalar(1.0, A::scale(s.ad_value(2103), 0.25))), 0.3333333333333333)));
        }

        if ((s.v[2180] != 0.0) && (s.v[2196] != 0.0)) {
            s.store_scaled_mul(2110, 2103, 1919, 0.7071067811865475);
        }

        if ((s.v[2180] != 0.0) && (s.v[2196] != 0.0)) {
            s.store_add_ad_rhs(2111, 2109, A::scale(A::div(A::mul(s.ad_value(2006), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2103), 0.5)), A::scale(A::square(s.ad_value(2103)), 0.16666666666666666))), s.ad_value(1919)), 0.7071067811865475));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) {
            s.store_add_ad_lhs(2107, A::offset(s.ad_value(2103), (-1.0)), 2104);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) {
            s.store_mul_ad_rhs(2108, 2006, A::sqrt(A::add(s.ad_value(2106), s.ad_value(2107))));
        }

        s.v[2198] = if (s.v[724] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_add_ad(2112, A::sub_from_scalar(1.0, s.ad_value(2104)), A::scale(A::mul(s.ad_value(2108), s.ad_value(2023)), 2.0));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_div_from_scalar_ad(2109, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0)));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_div_ad_rhs(1919, 2109, A::offset(s.ad_value(2109), 1.0));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_rhs(2113, 724, A::mul(A::mul(A::square(s.ad_value(1919)), s.ad_value(2007)), s.ad_value(2106)));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_add_ad(2114, A::scale(A::sub(s.ad_value(2108), s.ad_value(2113)), 2.0), A::mul(s.ad_value(2007), A::add(A::sub_from_scalar(1.0, s.ad_value(2104)), s.ad_value(2106))));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_rhs(2115, 2113, A::sub(s.ad_value(2113), A::scale(s.ad_value(2108), 2.0)));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_sub_from_scalar_ad(2116, 1.0, A::scale(A::mul(s.ad_value(2007), A::add(s.ad_value(2104), s.ad_value(2106))), 0.5));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_div_ad(2117, A::mul(s.ad_value(2115), s.ad_value(2114)), A::sub(A::square(s.ad_value(2114)), A::mul(s.ad_value(2116), s.ad_value(2115))));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_add(2103, 2103, 2117);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_exp(2118, 2117);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_div(2104, 2104, 2118);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_mul(2106, 2106, 2118);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_add_ad_lhs(2107, A::offset(s.ad_value(2103), (-1.0)), 2104);
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_rhs(2108, 2006, A::sqrt(A::add(s.ad_value(2106), s.ad_value(2107))));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_add_ad(2119, A::sub_from_scalar(1.0, s.ad_value(2104)), A::scale(A::mul(A::mul(s.ad_value(2108), s.ad_value(2109)), s.ad_value(2023)), 2.0));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_div_ad(2093, A::mul(A::mul(s.ad_value(2093), s.ad_value(2118)), A::add(s.ad_value(2112), s.ad_value(2105))), A::add(s.ad_value(2119), A::mul(s.ad_value(2118), s.ad_value(2105))));
        }

        if (((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) && (s.v[2198] != 0.0)) {
            s.store_mul(2096, 2093, 2021);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) {
            s.store_sqrt(2110, 2107);
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2196] != 0.0))) {
            s.store_add_ad_rhs(2111, 2109, A::scale(A::div(A::mul(s.ad_value(2006), A::sub_from_scalar(1.0, s.ad_value(2104))), s.ad_value(2110)), 0.5));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul_ad_rhs(2120, 2021, A::div(A::mul(s.ad_value(2007), s.ad_value(2106)), A::add(s.ad_value(2108), A::mul(s.ad_value(2006), s.ad_value(2110)))));
        }

        if (s.v[2180] != 0.0) {
            s.store_add_ad_rhs(2121, 2120, A::mul(s.ad_value(2021), s.ad_value(2111)));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul_ad_lhs(2122, A::mul(s.ad_value(2110), s.ad_value(2006)), 2021);
        }

        s.v[2199] = if (s.v[213] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2199] != 0.0)) {
            s.store_sub_from_scalar_ad(2061, 1.0, A::mul(s.ad_value(213), s.ad_value(2120)));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2199] != 0.0))) {
            s.store_div_from_scalar_ad(2061, 1.0, A::offset(A::mul(s.ad_value(213), s.ad_value(2120)), 1.0));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul_ad_lhs(2062, A::mul(A::mul(s.ad_value(751), s.ad_value(2060)), s.ad_value(2061)), 2120);
        }

        if (s.v[2180] != 0.0) {
            s.store_add_ad_rhs(2123, 2122, A::mul(s.ad_value(769), s.ad_value(2120)));
        }

        if (s.v[2180] != 0.0) {
            s.store_add_ad_rhs(2124, 2122, A::mul(s.ad_value(770), s.ad_value(2120)));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(2125, 768, 2123);
        }

        if (s.v[2180] != 0.0) {
            s.store_ln_ad(1920, A::div(s.ad_value(2107), A::offset(A::add(s.ad_value(2107), s.ad_value(2106)), 1e-14)));
        }

        if (s.v[2180] != 0.0) {
            s.store_add_ad(2064, A::pow(A::mul(s.ad_value(2125), s.ad_value(698)), s.ad_value(699)), A::mul(s.ad_value(700), A::exp(A::mul(A::scale(s.ad_value(701), 0.5), s.ad_value(1920)))));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul_ad_lhs(2126, A::add(A::offset(s.ad_value(2064), 1.0), s.ad_value(2062)), 2056);
        }

        if (s.v[2180] != 0.0) {
            s.store_ln_ad(2127, A::div(A::offset(A::mul(A::sub(s.ad_value(820), s.ad_value(2096)), s.ad_value(773)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2087), s.ad_value(2096)), s.ad_value(773)), 1.0)));
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(1921, 2120, 2066);
        }

        if (s.v[2180] != 0.0) {
            s.store_div_ad_rhs(2067, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.v[2200] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2180] != 0.0) && (s.v[2200] != 0.0)) {
            s.store_div_from_scalar_ad(2068, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2067))));
        }

        if ((s.v[2180] != 0.0) && (!(s.v[2200] != 0.0))) {
            s.store_offset_ad(2068, A::mul(s.ad_value(217), s.ad_value(2067)), 1.0);
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(2129, 2001, 2068);
        }

        if (s.v[2180] != 0.0) {
            s.store_mul(2128, 2108, 2021);
        }

        s.copy_ad(1843, 2069);

        s.copy_ad(1845, 2087);

        s.copy_ad(1846, 2088);

        s.copy_ad(1847, 2093);

        s.copy_ad(1848, 2096);

        s.copy_ad(1850, 2103);

        s.copy_ad(1849, 2102);

        s.copy_ad(1851, 2109);

        s.copy_ad(1852, 2111);

        s.copy_ad(1853, 2120);

        s.copy_ad(1854, 2121);

        s.copy_ad(1855, 2122);

        s.copy_ad(1856, 2124);

        s.copy_ad(1857, 2126);

        s.copy_ad(1859, 2127);

        s.copy_ad(1858, 2129);

        s.copy_ad(1860, 2128);

        s.v[1861] = 1.0;

        s.v[1862] = 1.0;

        s.v[1864] = 1.0;

        s.v[1865] = 1.0;

        s.v[832] = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[2201] = if (s.v[1817] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2201] != 0.0) {
            s.store_ln_ad(1929, A::offset(A::mul(s.ad_value(824), s.ad_value(773)), 1.0));
        }

        if (s.v[2201] != 0.0) {
            s.store_div_ad_lhs(1919, A::mul(s.ad_value(1812), s.ad_value(1852)), 1854);
        }

        if (s.v[2201] != 0.0) {
            s.store_add_ad(1928, A::mul(A::div(A::mul(A::add(s.ad_value(220), A::div(s.ad_value(221), s.ad_value(1854))), s.ad_value(1853)), s.ad_value(1854)), s.ad_value(1859)), A::mul(A::mul(A::mul(A::mul(s.ad_value(222), s.ad_value(1855)), s.ad_value(1919)), s.ad_value(1919)), s.ad_value(1929)));
        }

        if (s.v[2201] != 0.0) {
            s.store_div_from_scalar_ad(1861, 1.0, A::add(A::offset(s.ad_value(1928), 1.0), A::square(s.ad_value(1928))));
        }

        if (s.v[2201] != 0.0) {
            s.store_mul(1862, 1857, 1861);
        }

        if (s.v[2201] != 0.0) {
            s.store_div(1863, 1858, 1862);
        }

        if (s.v[2201] != 0.0) {
            s.store_mul_ad_lhs(1930, A::mul(A::square(s.ad_value(1863)), s.ad_value(1848)), 1848);
        }

        s.v[2202] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2201] != 0.0) && (s.v[2202] != 0.0)) {
            s.store_div_ad_rhs(1930, 1930, A::offset(A::mul(s.ad_value(1863), s.ad_value(1848)), 1.0));
        }

        if (s.v[2201] != 0.0) {
            s.store_scale_ad(1931, A::mul(s.ad_value(1862), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(1930), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2201] != 0.0) {
            s.store_div_from_scalar(1864, 1.0, 1931);
        }

        if (s.v[2201] != 0.0) {
            s.store_mul(1919, 1862, 1864);
        }

        if (s.v[2201] != 0.0) {
            s.store_mul_ad_rhs(1932, 1852, A::offset(A::scale(A::mul(A::mul(s.ad_value(1930), s.ad_value(1919)), s.ad_value(1919)), 0.5), 1.0));
        }

        if (s.v[2201] != 0.0) {
            s.store_div_ad_lhs(1865, A::mul(s.ad_value(1919), s.ad_value(1854)), 1932);
        }

        if (s.v[2201] != 0.0) {
            s.store_mul_ad_lhs(832, A::mul(A::mul(s.ad_value(710), s.ad_value(1854)), s.ad_value(1848)), 1864);
        }

        s.v[1934] = 0.0;

        s.v[1935] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 0.0;

        s.v[2203] = if (((((p.p40 != 0.0) && ((s.v[232] > 0.0) || (s.v[233] > 0.0))) || ((p.p42 != 0.0) && ((s.v[242] > 0.0) || (s.v[243] > 0.0)))) || (s.v[257] > 0.0)) || (s.v[258] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2203] != 0.0) {
            s.store_scale_ad(1933, A::add(s.ad_value(1805), A::sqrt(A::add(A::square(s.ad_value(1805)), s.ad_value(783)))), 0.5);
        }

        if (s.v[2203] != 0.0) {
            s.store_add_ad_lhs(1934, A::add(A::sub(A::neg(s.ad_value(1933)), A::scale(s.ad_value(778), 0.5)), A::mul(s.ad_value(776), A::sqrt(A::add(A::add(s.ad_value(1933), A::scale(s.ad_value(778), 0.25)), s.ad_value(784))))), 785);
        }

        if (s.v[2203] != 0.0) {
            s.store_scale_ad(1933, A::add(s.ad_value(1806), A::sqrt(A::add(A::square(s.ad_value(1806)), s.ad_value(786)))), 0.5);
        }

        if (s.v[2203] != 0.0) {
            s.store_add_ad_lhs(1935, A::add(A::sub(A::neg(s.ad_value(1933)), A::scale(s.ad_value(779), 0.5)), A::mul(s.ad_value(777), A::sqrt(A::add(A::add(s.ad_value(1933), A::scale(s.ad_value(779), 0.25)), s.ad_value(787))))), 788);
        }

        if (s.v[2203] != 0.0) {
            s.store_scaled_add(1866, 1805, 1934, (-s.v[348]));
        }

        if (s.v[2203] != 0.0) {
            s.store_scaled_add(1867, 1806, 1935, (-s.v[348]));
        }

        s.v[2204] = if (p.p40 != 0.0) { 1.0 } else { 0.0 };

        s.v[2205] = if (s.v[232] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_mul_ad_lhs(1936, A::sqrt(A::offset(A::square(s.ad_value(1866)), 1e-6)), 789);
        }

        s.v[2206] = if (s.v[238] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) && (s.v[2206] != 0.0)) {
            s.store_scale_ad(1936, A::sub(A::add(s.ad_value(1936), s.ad_value(795)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(795)), A::sub(s.ad_value(1936), s.ad_value(795))), 1e-6))), 0.5);
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_mul_ad_rhs(1919, 792, A::offset(A::mul(s.ad_value(1936), A::add(s.ad_value(237), A::mul(s.ad_value(238), s.ad_value(1936)))), (-1.5)));
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_offset(1938, 1934, 3.0);
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_sub_from_scalar(1939, (-3.0), 230);
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_scale(1940, 828, 30.0);
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_scalar(812, (4.0 - 0.9));
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_add(813, 1938, 1940);
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_mul_ad(1919, A::div_from_scalar(2.0, s.ad_value(812)), A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul(A::mul(s.ad_value(812), s.ad_value(1938)), s.ad_value(1940))))));
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_scalar(812, (4.0 - 0.3));
        }

        if ((s.v[2204] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_add(813, 1939, 1919);
        }

        s.v[2209] = if (s.v[233] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_mul_ad_lhs(1936, A::sqrt(A::offset(A::square(s.ad_value(1867)), 1e-6)), 789);
        }

        s.v[2210] = if (s.v[240] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) && (s.v[2210] != 0.0)) {
            s.store_scale_ad(1936, A::sub(A::add(s.ad_value(1936), s.ad_value(796)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(796)), A::sub(s.ad_value(1936), s.ad_value(796))), 1e-6))), 0.5);
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_mul_ad_rhs(1919, 793, A::offset(A::mul(s.ad_value(1936), A::add(s.ad_value(239), A::mul(s.ad_value(240), s.ad_value(1936)))), (-1.5)));
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_offset(1938, 1935, 3.0);
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_sub_from_scalar(1939, (-3.0), 230);
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_scale(1940, 831, 30.0);
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_scalar(812, (4.0 - 0.9));
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_add(813, 1938, 1940);
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_mul_ad(1919, A::div_from_scalar(2.0, s.ad_value(812)), A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul(A::mul(s.ad_value(812), s.ad_value(1938)), s.ad_value(1940))))));
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_scalar(812, (4.0 - 0.3));
        }

        if ((s.v[2204] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_add(813, 1939, 1919);
        }

        s.v[2213] = if (s.v[231] > 0.0) { 1.0 } else { 0.0 };

        s.v[2214] = if (s.v[1817] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2214] != 0.0)) {
            s.store_offset(1919, 771, 1.0);
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2214] != 0.0)) {
            s.store_div_ad_lhs(1920, A::mul(A::sqrt(s.ad_value(1919)), s.ad_value(820)), 1843);
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2214] != 0.0)) {
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2214] != 0.0)) {
            s.store_scale(1919, 1920, 2.0);
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2214] != 0.0)) {
            s.store_div_ad(1846, A::mul(A::mul(s.ad_value(1843), s.ad_value(1813)), s.ad_value(1919)), A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))));
        }

        s.v[2215] = if ((s.v[1847] - s.v[1846]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2215] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(1847), s.ad_value(1846)));
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2215] != 0.0))) {
            let assign46520_ad_e59616: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1919, &assign46520_ad_e59616);
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_add_ad_rhs(1942, 1922, A::mul(s.ad_value(1812), A::sub(A::scale(s.ad_value(1847), 0.5), A::ln(A::scale(A::offset(s.ad_value(1919), 1.0), 0.5)))));
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_mul(1943, 230, 1812);
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_add(1944, 1860, 1943);
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_scale_ad(1945, A::sub(s.ad_value(1944), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(1944)), A::neg(s.ad_value(1944))), 0.01))), 0.5);
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_mul_ad_lhs(1936, A::sqrt(A::offset(A::square(s.ad_value(1860)), 1e-6)), 789);
        }

        s.v[2216] = if (s.v[236] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2216] != 0.0)) {
            s.store_scale_ad(1936, A::sub(A::add(s.ad_value(1936), s.ad_value(794)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(794)), A::sub(s.ad_value(1936), s.ad_value(794))), 1e-6))), 0.5);
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_add_ad_rhs(1946, 1850, A::mul(A::sub(A::sub(s.ad_value(1945), s.ad_value(736)), s.ad_value(1942)), s.ad_value(1813)));
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_mul_ad_lhs(1946, A::neg(A::sub(A::add(s.ad_value(819), s.ad_value(1922)), s.ad_value(1942))), 1813);
        }

        s.v[2219] = if (((s.v[1946]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (s.v[2219] != 0.0)) {
            s.store_exp(1919, 1946);
        }

        s.v[2220] = if (s.v[1946] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2219] != 0.0))) && (s.v[2220] != 0.0)) {
            s.store_div_from_scalar_ad(1919, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1946)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1946)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1946)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2219] != 0.0))) && (!(s.v[2220] != 0.0))) {
            s.store_scale_ad(1919, A::offset(A::mul(A::offset(s.ad_value(1946), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(1946), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(1946), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_mul_ad_rhs(1919, 791, A::offset(A::mul(s.ad_value(1936), A::add(s.ad_value(235), A::mul(s.ad_value(236), s.ad_value(1936)))), (-1.5)));
        }

        s.v[2223] = if ((s.v[1817] <= 0.0) || ((s.v[235] == 0.0) && (s.v[236] == 0.0))) { 1.0 } else { 0.0 };

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) {
            s.store_add_ad_rhs(1919, 235, A::mul(A::scale(s.ad_value(236), 2.0), s.ad_value(1936)));
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) {
            s.store_div_ad_rhs(1950, 241, A::mul(s.ad_value(1919), s.ad_value(791)));
        }

        if (((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) {
            s.store_scaled_div(1951, 1848, 1950, 0.5);
        }

        s.v[2224] = if (s.v[1951] < 0.001) { 1.0 } else { 0.0 };

        s.v[2225] = if (((s.v[1951]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) && (!(s.v[2224] != 0.0))) && (s.v[2225] != 0.0)) {
            s.store_exp(1959, 1951);
        }

        s.v[2226] = if (s.v[1951] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) && (!(s.v[2224] != 0.0))) && (!(s.v[2225] != 0.0))) && (s.v[2226] != 0.0)) {
            s.store_div_from_scalar_ad(1959, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1951)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1951)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1951)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) && (!(s.v[2224] != 0.0))) && (!(s.v[2225] != 0.0))) && (!(s.v[2226] != 0.0))) {
            s.store_scale_ad(1959, A::offset(A::mul(A::offset(s.ad_value(1951), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(1951), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(1951), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) && (!(s.v[2224] != 0.0))) {
            s.store_div_from_scalar(1960, 1.0, 1959);
        }

        if ((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) && (!(s.v[2224] != 0.0))) {
            s.store_sub(1919, 1959, 1960);
        }

        if ((((s.v[2204] != 0.0) && (s.v[2213] != 0.0)) && (!(s.v[2223] != 0.0))) && (!(s.v[2224] != 0.0))) {
            s.store_add(1921, 1959, 1960);
        }

        s.v[2227] = if (p.p42 != 0.0) { 1.0 } else { 0.0 };

        s.v[2228] = if ((s.v[243] > 0.0) && (s.v[1867] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2227] != 0.0) && (s.v[2228] != 0.0)) {
            s.store_sqrt_ad(1963, A::offset(A::add(A::square(s.ad_value(1867)), A::mul(A::square(s.ad_value(249)), A::square(s.ad_value(830)))), 1e-6));
        }

        if ((s.v[2227] != 0.0) && (s.v[2228] != 0.0)) {
            s.store_div_ad_lhs(1919, A::neg(s.ad_value(801)), 1963);
        }

        s.v[2229] = if (s.v[1919] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2227] != 0.0) && (s.v[2228] != 0.0)) && (s.v[2229] != 0.0)) {
            s.store_exp(1921, 1919);
        }

        if (((s.v[2227] != 0.0) && (s.v[2228] != 0.0)) && (!(s.v[2229] != 0.0))) {
            s.store_div_from_scalar_ad(1921, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2230] = if ((s.v[242] > 0.0) && (s.v[1866] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2227] != 0.0) && (s.v[2230] != 0.0)) {
            s.store_sqrt_ad(1964, A::offset(A::add(A::square(s.ad_value(1866)), A::mul(A::square(s.ad_value(248)), A::square(s.ad_value(829)))), 1e-6));
        }

        if ((s.v[2227] != 0.0) && (s.v[2230] != 0.0)) {
            s.store_div_ad_lhs(1919, A::neg(s.ad_value(800)), 1964);
        }

        s.v[2231] = if (s.v[1919] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2227] != 0.0) && (s.v[2230] != 0.0)) && (s.v[2231] != 0.0)) {
            s.store_exp(1921, 1919);
        }

        if (((s.v[2227] != 0.0) && (s.v[2230] != 0.0)) && (!(s.v[2231] != 0.0))) {
            s.store_div_from_scalar_ad(1921, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[1968] = s.v[709];

        s.v[1868] = 0.0;

        s.v[1869] = 0.0;

        s.v[1870] = 0.0;

        s.v[1871] = 1e-40;

        s.v[1872] = 1.0;

        s.v[840] = 0.0;

        s.v[2232] = if ((p.p46 != 0.0) && (s.v[282] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2232] != 0.0) {
            s.store_add_ad_lhs(1919, A::scale(A::sub(A::add(s.ad_value(822), s.ad_value(821)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(758)))), 0.5), 756);
        }

        if (s.v[2232] != 0.0) {
            s.store_add_ad_lhs(1965, A::sub(s.ad_value(821), A::scale(A::sub(s.ad_value(1919), A::sqrt(A::add(A::mul(s.ad_value(1919), s.ad_value(1919)), s.ad_value(757)))), 0.5)), 760);
        }

        if (s.v[2232] != 0.0) {
            s.store_add_ad_rhs(1966, 1965, A::scale(A::sub(s.ad_value(820), s.ad_value(824)), 0.5));
        }

        if (s.v[2232] != 0.0) {
            s.store_mul_ad(1967, A::mul(s.ad_value(284), A::offset(A::mul(s.ad_value(286), s.ad_value(824)), 1.0)), A::offset(A::mul(s.ad_value(285), s.ad_value(1966)), 1.0));
        }

        if (s.v[2232] != 0.0) {
            s.store_mul_ad_rhs(1968, 717, A::offset(s.ad_value(1967), 1.0));
        }

        if (s.v[2232] != 0.0) {
            s.store_div_from_scalar(1969, 1.0, 1968);
        }

        if (s.v[2232] != 0.0) {
            s.store_div_ad(1970, A::scale(s.ad_value(824), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(288), s.ad_value(824)), 1.0)), 1.0));
        }

        if (s.v[2232] != 0.0) {
            s.store_mul_ad(1971, A::mul(s.ad_value(287), s.ad_value(1970)), A::offset(A::mul(s.ad_value(289), s.ad_value(1966)), 1.0));
        }

        if (s.v[2232] != 0.0) {
            s.store_mul_ad_rhs(1868, 1969, A::sub(A::add(s.ad_value(823), s.ad_value(1971)), s.ad_value(707)));
        }

        if (s.v[2232] != 0.0) {
            s.store_mul(1972, 1969, 754);
        }

        if (s.v[2232] != 0.0) {
            s.store_scale_ad(1973, A::ln(A::add(A::div(s.ad_value(1972), s.ad_value(755)), A::sqrt(s.ad_value(1972)))), 2.0);
        }

        if (s.v[2232] != 0.0) {
            s.store_mul(1974, 1969, 1965);
        }

        if (s.v[2232] != 0.0) {
            s.store_add(1979, 1972, 1974);
        }

        if (s.v[2232] != 0.0) {
            s.store_add_ad_rhs(1980, 1979, A::mul(s.ad_value(755), A::sqrt(s.ad_value(1979))));
        }

        if (s.v[2232] != 0.0) {
            s.store_add(1981, 1980, 1973);
        }

        if (s.v[2232] != 0.0) {
            s.store_offset_ad(1982, A::div(s.ad_value(755), A::scale(A::sqrt(s.ad_value(1979)), 2.0)), 1.0);
        }

        if (s.v[2232] != 0.0) {
            s.store_div_from_scalar(1983, 1.0, 1982);
        }

        if (s.v[2232] != 0.0) {
            s.store_sub(1984, 1868, 1981);
        }

        s.v[2233] = if (s.v[1984] > (-12.0)) { 1.0 } else { 0.0 };

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_offset_ad(1985, A::add(s.ad_value(1984), s.ad_value(719)), (-1.0));
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_scale_ad(1986, A::add(s.ad_value(1985), A::sqrt(A::offset(A::square(s.ad_value(1985)), 10.0))), 0.5);
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_add_ad_lhs(1987, A::sub(s.ad_value(1984), A::mul(s.ad_value(1982), A::ln(s.ad_value(1986)))), 719);
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_scale_ad(1988, A::add(s.ad_value(1987), A::sqrt(A::offset(A::square(s.ad_value(1987)), 2.0))), 0.5);
        }

        s.v[2234] = if ((s.v[1984] - s.v[1988]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) && (s.v[2234] != 0.0)) {
            s.store_exp_ad(1989, A::sub(s.ad_value(1984), s.ad_value(1988)));
        }

        if (((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) && (!(s.v[2234] != 0.0))) {
            s.store_scale_ad(1989, A::offset(A::mul(A::offset(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_mul(1990, 718, 1989);
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_ad(1991, &A::pow(s.ad_value(1990), s.ad_value(1983)));
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_add_ad(1992, A::square(s.ad_value(1982)), A::mul(A::sub(A::scale(A::add(s.ad_value(1988), s.ad_value(1982)), 2.0), s.ad_value(1991)), s.ad_value(1991)));
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_mul_ad_rhs(1993, 1982, A::offset(A::div(A::sub(A::sqrt(s.ad_value(1992)), s.ad_value(1982)), s.ad_value(1991)), (-1.0)));
        }

        if ((s.v[2232] != 0.0) && (s.v[2233] != 0.0)) {
            s.store_sub(1975, 1988, 1993);
        }

        s.v[2235] = if ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2232] != 0.0) && (!(s.v[2233] != 0.0))) && (s.v[2235] != 0.0)) {
            s.store_exp_ad(1975, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2232] != 0.0) && (!(s.v[2233] != 0.0))) && (!(s.v[2235] != 0.0))) {
            let assign47670_ad_e61124: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            s.store_div_from_scalar_ad(1975, 1e-100, A::offset(assign47670_ad_e61124, 1.0));
        }

        if (s.v[2232] != 0.0) {
            s.store_mul_ad_rhs(1976, 1969, A::add(s.ad_value(1845), s.ad_value(1965)));
        }

        s.v[2236] = if ((s.v[1975] < 0.001) && (s.v[1845] < 1e-6)) { 1.0 } else { 0.0 };

        s.v[2237] = if (((-s.v[1976]) + s.v[1974]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2232] != 0.0) && (s.v[2236] != 0.0)) && (s.v[2237] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(1974), s.ad_value(1976)));
        }

        if (((s.v[2232] != 0.0) && (s.v[2236] != 0.0)) && (!(s.v[2237] != 0.0))) {
            let assign47720_ad_e61203: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1919, &assign47720_ad_e61203);
        }

        if ((s.v[2232] != 0.0) && (s.v[2236] != 0.0)) {
            s.store_mul_ad_rhs(1869, 1975, A::offset(s.ad_value(1919), (-1.0)));
        }

        if ((s.v[2232] != 0.0) && (s.v[2236] != 0.0)) {
            s.store_add(1977, 1869, 1975);
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_add(1979, 1972, 1976);
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_add_ad_rhs(1980, 1979, A::mul(s.ad_value(755), A::sqrt(s.ad_value(1979))));
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_add(1981, 1980, 1973);
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_offset_ad(1982, A::div(s.ad_value(755), A::scale(A::sqrt(s.ad_value(1979)), 2.0)), 1.0);
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_div_from_scalar(1983, 1.0, 1982);
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_sub(1984, 1868, 1981);
        }

        s.v[2238] = if (s.v[1984] > (-12.0)) { 1.0 } else { 0.0 };

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_offset_ad(1985, A::add(s.ad_value(1984), s.ad_value(719)), (-1.0));
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_scale_ad(1986, A::add(s.ad_value(1985), A::sqrt(A::offset(A::square(s.ad_value(1985)), 10.0))), 0.5);
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_add_ad_lhs(1987, A::sub(s.ad_value(1984), A::mul(s.ad_value(1982), A::ln(s.ad_value(1986)))), 719);
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_scale_ad(1988, A::add(s.ad_value(1987), A::sqrt(A::offset(A::square(s.ad_value(1987)), 2.0))), 0.5);
        }

        s.v[2239] = if ((s.v[1984] - s.v[1988]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) && (s.v[2239] != 0.0)) {
            s.store_exp_ad(1989, A::sub(s.ad_value(1984), s.ad_value(1988)));
        }

        if ((((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) && (!(s.v[2239] != 0.0))) {
            s.store_scale_ad(1989, A::offset(A::mul(A::offset(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_mul(1990, 718, 1989);
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_ad(1991, &A::pow(s.ad_value(1990), s.ad_value(1983)));
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_add_ad(1992, A::square(s.ad_value(1982)), A::mul(A::sub(A::scale(A::add(s.ad_value(1988), s.ad_value(1982)), 2.0), s.ad_value(1991)), s.ad_value(1991)));
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_mul_ad_rhs(1993, 1982, A::offset(A::div(A::sub(A::sqrt(s.ad_value(1992)), s.ad_value(1982)), s.ad_value(1991)), (-1.0)));
        }

        if (((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (s.v[2238] != 0.0)) {
            s.store_sub(1977, 1988, 1993);
        }

        s.v[2240] = if ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (!(s.v[2238] != 0.0))) && (s.v[2240] != 0.0)) {
            s.store_exp_ad(1977, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if ((((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) && (!(s.v[2238] != 0.0))) && (!(s.v[2240] != 0.0))) {
            let assign47960_ad_e61556: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            s.store_div_from_scalar_ad(1977, 1e-100, A::offset(assign47960_ad_e61556, 1.0));
        }

        if ((s.v[2232] != 0.0) && (!(s.v[2236] != 0.0))) {
            s.store_sub(1869, 1977, 1975);
        }

        if (s.v[2232] != 0.0) {
            s.store_scaled_add(1870, 1977, 1975, 0.5);
        }

        if (s.v[2232] != 0.0) {
            s.store_ad(1871, &{
                if ((s.v[1868] - s.v[1870]) > 1e-40) {
                    A::sub(s.ad_value(1868), s.ad_value(1870))
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.v[2232] != 0.0) {
            s.store_sub_from_scalar_ad(1872, 1.0, A::div(A::scale(s.ad_value(755), 0.5), A::sqrt(A::add(s.ad_value(1871), A::scale(s.ad_value(718), 0.25)))));
        }

        if (s.v[2232] != 0.0) {
            s.store_div_ad_lhs(840, A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(711)), s.ad_value(1968)), s.ad_value(1968)), A::offset(A::mul(s.ad_value(1872), s.ad_value(1870)), 1.0)), s.ad_value(1869)), 1857);
        }

        s.v[1873] = 0.0;

        s.v[841] = 0.0;

        s.v[2241] = if ((s.v[1817] > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2241] != 0.0) {
            s.store_sub_ad_rhs(1978, 820, A::mul(s.ad_value(227), s.ad_value(1848)));
        }

        s.v[2242] = if (s.v[1978] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) {
            s.store_mul_ad_rhs(1921, 706, A::div(A::offset(A::mul(s.ad_value(228), A::sub(A::sqrt(A::add(s.ad_value(722), s.ad_value(1922))), s.ad_value(730))), 1.0), A::offset(s.ad_value(1978), 1e-30)));
        }

        s.v[2243] = if ((((-s.v[1921])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (s.v[2243] != 0.0)) {
            s.store_exp_ad(1919, A::neg(s.ad_value(1921)));
        }

        s.v[2244] = if ((-s.v[1921]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (!(s.v[2243] != 0.0))) && (s.v[2244] != 0.0)) {
            s.store_div_from_scalar_ad(1919, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1921))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1921))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1921))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (!(s.v[2243] != 0.0))) && (!(s.v[2244] != 0.0))) {
            s.store_scale_ad(1919, A::offset(A::mul(A::offset(A::neg(s.ad_value(1921)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1921)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1921)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) {
            s.store_mul_ad_rhs(1873, 224, A::mul(s.ad_value(1978), s.ad_value(1919)));
        }

        if ((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) {
            s.store_mul_ad_rhs(841, 1873, A::add(s.ad_value(832), s.ad_value(840)));
        }

        s.v[2245] = if (s.v[841] > (0.5 * s.v[229])) { 1.0 } else { 0.0 };

        if (((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (s.v[2245] != 0.0)) {
            s.store_offset_ad(1919, A::div(A::scale(s.ad_value(841), 2.0), s.ad_value(229)), (-1.0));
        }

        if (((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (s.v[2245] != 0.0)) {
            s.store_mul_ad(841, A::scale(s.ad_value(229), 0.5), A::offset(A::div(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 1.0))), 1.0));
        }

        s.v[2439] = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        s.v[2440] = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.copy_ad(2280, 722);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.copy_ad(2281, 732);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.copy_ad(2282, 723);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.copy_ad(2283, 1808);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.copy_ad(2284, 1809);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2288, 0.0);
        }

        s.v[2441] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.store_add_ad_lhs(2283, A::scale(A::sub(A::add(s.ad_value(822), s.ad_value(821)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(743)))), 0.5), 741);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.store_add_ad_lhs(1874, A::sub(s.ad_value(821), A::scale(A::sub(s.ad_value(2283), A::sqrt(A::add(A::mul(s.ad_value(2283), s.ad_value(2283)), s.ad_value(742)))), 0.5)), 744);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2284, 1874);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2280, 739);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2281, 742);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2282, 740);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub_ad_lhs(2287, A::sub(s.ad_value(823), s.ad_value(2288)), 694);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_add_ad_rhs(2289, 2284, A::scale(A::sub(s.ad_value(820), s.ad_value(824)), 0.5));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2301, 1.0);
        }

        s.v[2442] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale(2292, 2280, s.v[355]);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale(2293, 2289, s.v[355]);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale(2294, 2287, s.v[355]);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_offset_ad(1920, A::div(A::scale(s.ad_value(2282), 0.5), A::sqrt(s.ad_value(2292))), 1.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_add_ad_rhs(1921, 2292, A::mul(s.ad_value(2282), A::sqrt(s.ad_value(2292))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_sub_ad(2295, A::add(A::div(A::sub(s.ad_value(2294), s.ad_value(1921)), s.ad_value(1920)), A::scale(s.ad_value(2292), 0.5)), A::mul(A::offset(s.ad_value(186), 1.0), s.ad_value(2293)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_offset_scaled(2296, 2292, 0.5, 2.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_add(2297, 2292, 2293);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_sub_ad(1920, A::sub(A::sub(s.ad_value(2294), s.ad_value(2297)), A::mul(s.ad_value(2282), A::sqrt(s.ad_value(2297)))), A::scale(A::ln(A::add(A::div(s.ad_value(2292), s.ad_value(2282)), A::sqrt(s.ad_value(2292)))), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_add_ad_lhs(2298, A::scale(s.ad_value(1920), 2.0), 2296);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(1920, A::add(A::add(s.ad_value(2295), s.ad_value(2298)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2295), s.ad_value(2298)), A::sub(s.ad_value(2295), s.ad_value(2298))), 20.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_sub_ad_lhs(1921, A::scale(A::sub(s.ad_value(2294), s.ad_value(2293)), 2.0), 2296);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(2299, A::sub(A::add(s.ad_value(1920), s.ad_value(1921)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), s.ad_value(1921)), A::sub(s.ad_value(1920), s.ad_value(1921))), 20.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(1920, A::sub(A::add(s.ad_value(2299), s.ad_value(2296)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2299), s.ad_value(2296)), A::sub(s.ad_value(2299), s.ad_value(2296))), 5.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(2300, A::add(A::sub(s.ad_value(1920), s.ad_value(2296)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), A::neg(s.ad_value(2296))), A::sub(s.ad_value(1920), A::neg(s.ad_value(2296)))), 20.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_mul_ad_rhs(1921, 696, A::offset(A::div(s.ad_value(2300), s.ad_value(2296)), 1.0));
        }

        s.v[2443] = if (s.v[1921] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) && (s.v[2443] != 0.0)) {
            s.store_exp(2301, 1921);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) && (!(s.v[2443] != 0.0))) {
            s.store_div_from_scalar_ad(2301, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_offset_ad(2302, A::mul(s.ad_value(695), s.ad_value(2301)), 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scale(2303, 2302, s.v[709]);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2304, A::mul(s.ad_value(194), A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0)), A::offset(A::mul(s.ad_value(195), s.ad_value(2289)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad_rhs(2305, 2303, A::offset(s.ad_value(2304), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_from_scalar(2306, 1.0, 2305);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad_rhs(2290, 2282, A::sqrt(A::scale(s.ad_value(2306), s.v[709])));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_square(2291, 2290);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_from_scalar(2307, 1.0, 2291);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2308, 2284, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2309, 2287, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_ad(2310, A::scale(s.ad_value(824), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(824)), 1.0)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2311, A::mul(s.ad_value(191), s.ad_value(2310)), A::offset(A::mul(s.ad_value(193), s.ad_value(2289)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2312, 2280, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sqrt_ad(1920, A::add(A::square(s.ad_value(2283)), s.ad_value(2281)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sqrt_ad(1921, A::add(A::mul(A::sub(s.ad_value(2283), s.ad_value(2311)), A::sub(s.ad_value(2283), s.ad_value(2311))), s.ad_value(2281)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2313, A::scale(s.ad_value(2306), 0.5), A::sub(A::add(s.ad_value(2311), s.ad_value(1920)), s.ad_value(1921)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_add(2314, 2312, 2308);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub(2315, 2314, 2313);
        }

        s.v[2444] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2445] = if (((s.v[2315]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (s.v[2445] != 0.0)) {
            s.store_offset_ad(2316, A::mul(s.ad_value(2290), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2315), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2315), 0.3125))))), 1.0);
        }

        s.v[2446] = if (s.v[2315] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) && (s.v[2446] != 0.0)) {
            s.store_exp_ad(2330, A::neg(s.ad_value(2315)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) && (!(s.v[2446] != 0.0))) {
            s.store_div_from_scalar_ad(2330, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2315), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) {
            s.store_scalar(1919, (if (s.v[2315] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) {
            s.store_offset_ad(2316, A::div(A::mul(A::mul(s.ad_value(1919), s.ad_value(2290)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2330), A::sub_from_scalar(1.0, s.ad_value(2315))))), A::scale(A::sqrt(A::mul(s.ad_value(2315), A::sub_from_scalar(1.0, s.ad_value(2330)))), 2.0)), 1.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2444] != 0.0))) {
            s.store_offset_ad(2316, A::div(A::scale(s.ad_value(2290), 0.5), A::sqrt(s.ad_value(2315))), 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub_ad(2317, A::add(s.ad_value(2315), A::mul(s.ad_value(2290), A::sqrt(s.ad_value(2315)))), A::mul(s.ad_value(2316), A::ln(A::offset(s.ad_value(2316), (-1.0)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_ad_lhs(2318, A::sub(s.ad_value(2309), s.ad_value(2317)), 2316);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2324, A::scale(s.ad_value(2291), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2291)), 1.0)), (-1.0)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2323, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2325, 1.0);
        }

        s.v[2447] = if (s.v[2318] > (-30.0)) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_offset_ad(2319, A::mul(s.ad_value(2316), s.ad_value(2318)), (-1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_scale_ad(1919, A::add(s.ad_value(2319), A::sqrt(A::offset(A::square(s.ad_value(2319)), 10.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_sub_ad_rhs(2320, 2318, A::ln(s.ad_value(1919)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_scale_ad(2321, A::add(s.ad_value(2320), A::sqrt(A::offset(A::square(s.ad_value(2320)), 2.0))), 0.5);
        }

        s.v[2448] = if ((s.v[2318] - s.v[2321]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (s.v[2448] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(2318), s.ad_value(2321)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (!(s.v[2448] != 0.0))) {
            s.store_scale_ad(1919, A::offset(A::mul(A::offset(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_div(2322, 1919, 2316);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_sub_ad_lhs(1919, A::scale(A::offset(s.ad_value(2321), 1.0), 2.0), 2322);
        }

        s.v[2449] = if (s.v[2322] > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (s.v[2449] != 0.0)) {
            s.store_mul_ad_rhs(2323, 2316, A::offset(A::sub(s.ad_value(2321), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2322), s.ad_value(1919)), 1.0)), (-1.0)), s.ad_value(2322))), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (!(s.v[2449] != 0.0))) {
            s.store_mul_ad(2323, A::mul(A::scale(s.ad_value(2316), 0.5), s.ad_value(2322)), A::offset(A::mul(A::scale(s.ad_value(1919), 0.25), s.ad_value(1919)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_scale_ad(1919, A::add(A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0)), A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0))), 1.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_mul_ad(2324, A::scale(s.ad_value(2291), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2291)), s.ad_value(1919)), 1.0)), (-1.0)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_div_ad_rhs(2325, 2324, A::add(s.ad_value(2324), s.ad_value(2323)));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_sub_ad_rhs(2315, 2314, A::mul(s.ad_value(2325), s.ad_value(2313)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_offset_scaled(2326, 2290, 0.7071067811865475, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scale(2327, 2326, 1e-5);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_from_scalar(2328, 1.0, 2326);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2435, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2329, 0.0);
        }

        s.v[2450] = if (s.v[2315] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2450] != 0.0)) {
            s.store_exp_ad(2330, A::neg(s.ad_value(2315)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2450] != 0.0))) {
            s.store_div_from_scalar_ad(2330, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2315), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2451] = if (((s.v[2309]) as f64).abs() <= s.v[2327]) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2451] != 0.0)) {
            s.store_scale_ad(2415, A::square(s.ad_value(2328)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2451] != 0.0)) {
            s.store_mul_ad(2329, A::mul(s.ad_value(2309), s.ad_value(2328)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2309), A::sub_from_scalar(1.0, s.ad_value(2330))), s.ad_value(2290)), s.ad_value(2415)), 1.0));
        }

        s.v[2452] = if (s.v[2309] < (-s.v[2327])) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_neg(2417, 2309);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_scaled_mul(2418, 2417, 2328, 1.25);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_scale_ad(2419, A::sub(A::offset(s.ad_value(2418), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2418), (-6.0)), A::offset(s.ad_value(2418), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub(2414, 2417, 2419);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad(2420, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::offset(s.ad_value(2419), 1.0)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad_lhs(2421, A::scale(s.ad_value(2414), 2.0), 2291);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad_lhs(2422, A::ln(A::mul(s.ad_value(2420), s.ad_value(2307))), 2419);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add(818, 2420, 2421);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2422), A::sub(A::scale(A::square(s.ad_value(2421)), 0.5), s.ad_value(2420))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad_rhs(2423, 2419, A::div(A::mul(A::mul(s.ad_value(2420), s.ad_value(818)), s.ad_value(2422)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422)), s.ad_value(2422)), s.ad_value(2421)), A::sub(A::scale(A::square(s.ad_value(2421)), 0.3333333333333333), s.ad_value(2420))))));
        }

        s.v[2453] = if (s.v[2423] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) && (s.v[2453] != 0.0)) {
            s.store_exp(2424, 2423);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) && (!(s.v[2453] != 0.0))) {
            s.store_scale_ad(2424, A::offset(A::mul(A::offset(s.ad_value(2423), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2423), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2423), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_div_from_scalar(2425, 1.0, 2424);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_div_from_scalar_ad(2414, 1.0, A::offset(A::square(s.ad_value(2423)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2423)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2423), s.ad_value(2414)), s.ad_value(2414)), 4.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2414), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2414)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub(2414, 2417, 2423);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_mul(2415, 2330, 2425);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad(2429, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::add(A::sub(A::offset(s.ad_value(2424), (-1.0)), s.ad_value(2415)), A::mul(s.ad_value(2330), A::sub_from_scalar(1.0, s.ad_value(2427))))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad(2430, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::add(A::add(A::offset(A::sub(s.ad_value(2424), s.ad_value(2423)), (-1.0)), s.ad_value(2415)), A::mul(s.ad_value(2330), A::sub(A::offset(s.ad_value(2423), (-1.0)), s.ad_value(2426))))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2424), s.ad_value(2415)), A::mul(s.ad_value(2330), s.ad_value(2428)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad(2414, A::square(s.ad_value(2429)), A::scale(A::mul(s.ad_value(2430), s.ad_value(2414)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad(2329, A::neg(s.ad_value(2423)), A::scale(A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_div_from_scalar_ad(2431, 1.0, A::offset(A::scale(s.ad_value(2290), 0.7324648775608221), 1.25));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2432, A::offset(A::mul(A::scale(s.ad_value(2326), 1.25), s.ad_value(2431)), (-1.0)), 2431);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad(2433, A::mul(s.ad_value(2309), s.ad_value(2328)), A::offset(A::mul(s.ad_value(2432), s.ad_value(2309)), 1.0));
        }

        s.v[2454] = if ((-s.v[2433]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2454] != 0.0)) {
            s.store_exp_ad(2414, A::neg(s.ad_value(2433)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2454] != 0.0))) {
            s.store_div_from_scalar_ad(2414, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2433))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2433))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2433))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_from_scalar(2434, 1.0, 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2435, A::add(s.ad_value(2309), A::scale(s.ad_value(2291), 0.5)), A::mul(s.ad_value(2290), A::sqrt(A::sub(A::add(s.ad_value(2309), A::scale(s.ad_value(2291), 0.25)), s.ad_value(2434)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_offset(2436, 2315, 3.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2419, A::scale(A::sub(A::add(s.ad_value(2435), s.ad_value(2436)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2436), A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0))), 0.5));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub(2414, 2309, 2419);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_exp_ad(2415, A::neg(s.ad_value(2419)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_div_from_scalar_ad(2416, 1.0, A::offset(A::square(s.ad_value(2419)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2419)), 2416);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2419), s.ad_value(2416)), s.ad_value(2416)), 4.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2416), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2416)), 2416);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            let assign49580_ad_e63936: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2330] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), A::mul(s.ad_value(2330), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426))))))
                }
            };
            s.store_ad(2420, &assign49580_ad_e63936);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::scale(A::mul(s.ad_value(2291), A::sub(s.ad_value(2415), A::mul(s.ad_value(2330), s.ad_value(2428)))), 0.5));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(2421, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::sub_from_scalar(1.0, s.ad_value(2415)), A::mul(s.ad_value(2330), A::offset(s.ad_value(2427), 1.0)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(2422, A::sub(s.ad_value(2315), s.ad_value(2419)), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add(818, 2420, 2421);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2422), A::sub(A::scale(A::square(s.ad_value(2421)), 0.5), A::mul(s.ad_value(2420), s.ad_value(2437)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            let assign49640_ad_e64083: A = A::add(s.ad_value(2419), A::div(A::mul(A::mul(s.ad_value(2420), s.ad_value(818)), s.ad_value(2422)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422)), s.ad_value(2422)), s.ad_value(2421)), A::sub(A::scale(A::square(s.ad_value(2421)), 0.3333333333333333), A::mul(s.ad_value(2420), s.ad_value(2437)))))));
            s.store_ad(2438, &assign49640_ad_e64083);
        }

        s.v[2455] = if (s.v[2438] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2455] != 0.0)) {
            s.store_exp(2424, 2438);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2455] != 0.0)) {
            s.store_div_from_scalar(2425, 1.0, 2424);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2455] != 0.0)) {
            s.store_mul(2424, 2330, 2424);
        }

        s.v[2456] = if (s.v[2438] > (s.v[2315] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (s.v[2456] != 0.0)) {
            s.store_exp_ad(2424, A::sub(s.ad_value(2438), s.ad_value(2315)));
        }

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (s.v[2456] != 0.0)) {
            s.store_div(2425, 2330, 2424);
        }

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (!(s.v[2456] != 0.0))) {
            s.store_div_from_scalar_ad(2424, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (!(s.v[2456] != 0.0))) {
            s.store_div_from_scalar_ad(2425, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2438), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_div_from_scalar_ad(2414, 1.0, A::offset(A::square(s.ad_value(2438)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2438)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2438), s.ad_value(2414)), s.ad_value(2414)), 4.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2414), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2414)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub(2414, 2309, 2438);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(2429, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2425)), s.ad_value(2424)), A::mul(s.ad_value(2330), A::offset(s.ad_value(2427), 1.0)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2430, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::add(A::offset(A::add(s.ad_value(2425), s.ad_value(2438)), (-1.0)), s.ad_value(2424)), A::mul(s.ad_value(2330), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426))))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2425), s.ad_value(2424)), A::mul(s.ad_value(2330), s.ad_value(2428)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2414, A::square(s.ad_value(2429)), A::scale(A::mul(s.ad_value(2430), s.ad_value(2414)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad_rhs(2329, 2438, A::scale(A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2332, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2333, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2334, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2335, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2336, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2337, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2338, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2339, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2340, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub(2341, 2309, 2329);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2342, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2343, 2305, 2341);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2344, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2345, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2349, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2350, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2352, 1.0);
        }

        s.v[2457] = if (s.v[2309] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_div_from_scalar_ad(1919, 1.0, A::offset(A::square(s.ad_value(2329)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_mul_ad_lhs(2331, A::square(s.ad_value(2329)), 1919);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_scale_ad(2332, A::mul(A::mul(s.ad_value(2329), s.ad_value(1919)), s.ad_value(1919)), 4.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_mul_ad_lhs(2333, A::mul(A::sub(A::scale(s.ad_value(1919), 8.0), A::scale(s.ad_value(2331), 12.0)), s.ad_value(1919)), 1919);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_scalar(2334, 0.0);
        }

        s.v[2458] = if (s.v[2329] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2458] != 0.0)) {
            s.store_exp(2334, 2329);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2458] != 0.0)) {
            s.store_div_from_scalar(2335, 1.0, 2334);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2458] != 0.0)) {
            s.store_mul(2334, 2330, 2334);
        }

        s.v[2459] = if (s.v[2329] > (s.v[2315] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (s.v[2459] != 0.0)) {
            s.store_exp_ad(2334, A::sub(s.ad_value(2329), s.ad_value(2315)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (s.v[2459] != 0.0)) {
            s.store_div(2335, 2330, 2334);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (!(s.v[2459] != 0.0))) {
            s.store_div_from_scalar_ad(2334, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (!(s.v[2459] != 0.0))) {
            s.store_div_from_scalar_ad(2335, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2329), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2329), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2329), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_sub_ad_rhs(2336, 2334, A::mul(s.ad_value(2330), A::add(A::offset(s.ad_value(2329), 1.0), s.ad_value(2331))));
        }

        s.v[2460] = if (s.v[2329] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_scale_ad(2337, A::mul(A::square(s.ad_value(2329)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2329), A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_scale_ad(2336, A::mul(A::mul(A::mul(A::mul(s.ad_value(2330), s.ad_value(2329)), s.ad_value(2329)), s.ad_value(2329)), A::offset(A::scale(s.ad_value(2329), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2329), A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_scaled_mul(2338, 2329, 1919, 0.7071067811865475);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_offset_ad(2339, A::scale(A::div(A::mul(s.ad_value(2290), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.5)), A::scale(A::square(s.ad_value(2329)), 0.16666666666666666))), s.ad_value(1919)), 0.7071067811865475), 1.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2460] != 0.0))) {
            s.store_add_ad_lhs(2337, A::offset(s.ad_value(2329), (-1.0)), 2335);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2460] != 0.0))) {
            s.store_sqrt(2338, 2337);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2460] != 0.0))) {
            s.store_offset_ad(2339, A::scale(A::div(A::mul(s.ad_value(2290), A::sub_from_scalar(1.0, s.ad_value(2335))), s.ad_value(2338)), 0.5), 1.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_div_ad(2340, A::offset(A::mul(A::scale(s.ad_value(702), 0.2), s.ad_value(2289)), 1.0), A::offset(A::mul(s.ad_value(702), s.ad_value(2289)), 1.0));
        }

        s.v[2461] = if (s.v[2336] > 1e-100) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_rhs(2341, 2290, A::sqrt(A::add(s.ad_value(2337), s.ad_value(2336))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div_ad(2342, A::mul(A::mul(s.ad_value(2291), s.ad_value(2336)), s.ad_value(2305)), A::add(s.ad_value(2341), A::mul(s.ad_value(2290), s.ad_value(2338))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_lhs(2343, A::mul(s.ad_value(2338), s.ad_value(2290)), 2305);
        }

        s.v[2462] = if (s.v[212] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2462] != 0.0)) {
            s.store_div_from_scalar_ad(2344, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(212), s.ad_value(2289))));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2462] != 0.0))) {
            s.store_offset_ad(2344, A::mul(s.ad_value(212), s.ad_value(2289)), 1.0);
        }

        s.v[2463] = if (s.v[213] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2463] != 0.0)) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2342)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2463] != 0.0))) {
            s.store_div_from_scalar_ad(2345, 1.0, A::offset(A::mul(s.ad_value(213), s.ad_value(2342)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_lhs(2346, A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), 2342);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_rhs(2347, 768, A::add(s.ad_value(2343), A::mul(s.ad_value(769), s.ad_value(2342))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_ln_ad(1920, A::div(s.ad_value(2337), A::offset(A::add(s.ad_value(2337), s.ad_value(2336)), 1e-14)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_add_ad(2348, A::pow(A::mul(s.ad_value(2347), s.ad_value(698)), s.ad_value(699)), A::mul(s.ad_value(700), A::exp(A::mul(A::scale(s.ad_value(701), 0.5), s.ad_value(1920)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_lhs(2349, A::add(A::offset(s.ad_value(2348), 1.0), s.ad_value(2346)), 2340);
        }

        s.v[2464] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2464] != 0.0)) {
            s.store_div_from_scalar_ad(2350, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(2289))));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2464] != 0.0))) {
            s.store_offset_ad(2350, A::mul(s.ad_value(216), s.ad_value(2289)), 1.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul(1921, 2342, 2350);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.v[2465] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2465] != 0.0)) {
            s.store_div_from_scalar_ad(2352, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2351))));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2465] != 0.0))) {
            s.store_offset_ad(2352, A::mul(s.ad_value(217), s.ad_value(2351)), 1.0);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2287, 1810);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2289, 1811);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2305, 1812);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2306, 1813);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2290, 1814);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2291, 1815);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2307, 1816);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2309, 1817);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2314, 1818);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2315, 1819);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2326, 1820);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2327, 1821);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2328, 1822);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2435, 1823);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2330, 1824);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2329, 1825);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2332, 1826);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2333, 1827);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2334, 1828);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2335, 1829);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2337, 1830);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2336, 1831);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2338, 1832);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2339, 1833);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2340, 1834);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2341, 1835);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2342, 1836);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2343, 1837);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2344, 1838);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2345, 1839);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2349, 1840);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2350, 1841);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2352, 1842);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2285, 714);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2286, 771);
        }

        s.v[2466] = if (p.p48 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2439] != 0.0) && (s.v[2466] != 0.0)) {
            s.copy_ad(2285, 715);
        }

        if ((s.v[2439] != 0.0) && (s.v[2466] != 0.0)) {
            s.copy_ad(2286, 772);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2354, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scale(2353, 2305, 4.60517018598809);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2370, 2353);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2371, 820);
        }

        if (s.v[2439] != 0.0) {
            s.store_mul(2372, 820, 2306);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2376, 2329);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2377, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2380, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2382, 2335);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2383, 2337);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2385, 2336);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2386, 2343);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2387, 2329);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2388, 2335);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2390, 2336);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2391, 2337);
        }

        if (s.v[2439] != 0.0) {
            s.store_sub(2392, 2309, 2329);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2393, 1.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2395, 1.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2394, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2404, 2342);
        }

        if (s.v[2439] != 0.0) {
            s.store_mul(2408, 2392, 2305);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2405, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2406, 2343);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2411, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2410, 1.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2413, 2285);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2412, 2408);
        }

        s.v[2467] = if (s.v[2309] > 0.0) { 1.0 } else { 0.0 };

        s.v[2468] = if (s.v[2336] > 1e-100) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul(2413, 2285, 2352);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div(2354, 2413, 2349);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_add_ad_rhs(2355, 2341, A::scale(s.ad_value(2291), 0.5));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div_ad_lhs(1919, A::div(A::mul(s.ad_value(2291), s.ad_value(2334)), s.ad_value(2355)), 2355);
        }

        s.v[2469] = if (s.v[1919] > 0.0001) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2469] != 0.0)) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.v[2470] = if (s.v[1920] < 1e-10) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2469] != 0.0)) && (s.v[2470] != 0.0)) {
            s.store_scalar(1921, 1.0);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2469] != 0.0)) && (!(s.v[2470] != 0.0))) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (!(s.v[2469] != 0.0))) {
            s.store_scale(1921, 1919, 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul(2356, 1921, 2355);
        }

        s.v[2471] = if ((s.v[700] > 0.0) && (s.v[701] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2357, A::scale(s.ad_value(2305), 0.475), 2356);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_sub_ad_rhs(1919, 2342, A::mul(s.ad_value(2339), s.ad_value(2357)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_scale_ad(2358, A::add(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 1e-12))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_add_ad(2359, A::sub(A::mul(s.ad_value(2305), s.ad_value(2341)), s.ad_value(2342)), A::mul(A::offset(s.ad_value(2339), (-1.0)), s.ad_value(2357)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_offset_ad(2360, A::div(A::mul(A::scale(s.ad_value(2291), 0.5), s.ad_value(2305)), s.ad_value(2359)), 1.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_add_ad_rhs(1919, 2359, A::mul(s.ad_value(769), s.ad_value(2358)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_ad(2361, &A::pow(A::mul(A::mul(s.ad_value(768), s.ad_value(1919)), s.ad_value(698)), s.ad_value(699)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(1920, A::div(A::mul(s.ad_value(699), A::offset(A::mul(s.ad_value(2360), A::sub_from_scalar(1.0, s.ad_value(769))), (-1.0))), s.ad_value(1919)), 2361);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div(1919, 2358, 2359);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_rhs(2362, 700, A::pow(A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(1921, A::div(A::mul(s.ad_value(701), A::add(A::offset(s.ad_value(2360), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1919), 1.0)))), s.ad_value(2359)), 2362);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2363, A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), 2358);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_offset_ad(1919, A::div(A::sub(s.ad_value(1920), A::mul(A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), s.ad_value(2360))), s.ad_value(1921)), 1.0);
        }

        s.v[2472] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_scale_ad(1920, A::ln(A::offset(A::exp(A::scale(s.ad_value(1919), 2.0)), 1.0)), 0.5);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) {
            s.copy_ad(1920, 1919);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div_ad(2364, A::mul(A::mul(A::neg(s.ad_value(2357)), s.ad_value(1921)), s.ad_value(1920)), A::add(A::add(A::offset(s.ad_value(2361), 1.0), s.ad_value(2362)), s.ad_value(2363)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_rhs(2365, 2356, A::offset(A::div(s.ad_value(2364), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2364)), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (!(s.v[2471] != 0.0))) {
            s.copy_ad(2365, 2356);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_scale_ad(2366, A::mul(A::mul(s.ad_value(2305), s.ad_value(2354)), s.ad_value(2365)), 0.7071067811865475);
        }

        s.v[2473] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2473] != 0.0)) {
            s.store_div_ad_rhs(2366, 2366, A::sqrt(A::offset(s.ad_value(2366), 1.0)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div_from_scalar_ad(2367, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2366), 4.0), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul(1919, 2367, 2366);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul_ad(2368, A::mul(s.ad_value(2365), s.ad_value(2367)), A::offset(A::div(A::mul(A::scale(s.ad_value(1919), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(1919), s.ad_value(2367)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(1919), 4.0), s.ad_value(1919)), s.ad_value(2367)), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_scale(2369, 2368, 0.99);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div_ad_lhs(1919, A::mul(A::mul(s.ad_value(2369), A::sub(s.ad_value(2369), A::scale(s.ad_value(2355), 2.0))), s.ad_value(2307)), 2336);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul_ad_rhs(2370, 2305, A::sub(s.ad_value(2369), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2468] != 0.0))) {
            s.copy_ad(2370, 2353);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_offset(1919, 2286, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad_lhs(1920, A::mul(A::sqrt(s.ad_value(1919)), s.ad_value(820)), 2370);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scale(1919, 1920, 2.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad(2371, A::mul(s.ad_value(2370), s.ad_value(1919)), A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2372, 2371, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add(2373, 2315, 2372);
        }

        s.v[2474] = if (s.v[2372] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_exp_ad(2374, A::neg(s.ad_value(2372)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_div_from_scalar_ad(2374, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2372), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2372), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2372), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2375, 2330, 2374);
        }

        s.v[2475] = if (((s.v[2309]) as f64).abs() <= s.v[2327]) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_scale_ad(2415, A::square(s.ad_value(2328)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad(2376, A::mul(s.ad_value(2309), s.ad_value(2328)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2309), A::sub_from_scalar(1.0, s.ad_value(2375))), s.ad_value(2290)), s.ad_value(2415)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_offset(2436, 2373, 3.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_ad(2419, A::scale(A::sub(A::add(s.ad_value(2435), s.ad_value(2436)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2436), A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0))), 0.5));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub(2414, 2309, 2419);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_exp_ad(2415, A::neg(s.ad_value(2419)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_div_from_scalar_ad(2416, 1.0, A::offset(A::square(s.ad_value(2419)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2419)), 2416);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2419), s.ad_value(2416)), s.ad_value(2416)), 4.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2416), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2416)), 2416);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            let assign51910_ad_e66735: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2375] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426))))))
                }
            };
            s.store_ad(2420, &assign51910_ad_e66735);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::scale(A::mul(s.ad_value(2291), A::sub(s.ad_value(2415), A::mul(s.ad_value(2375), s.ad_value(2428)))), 0.5));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(2421, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::sub_from_scalar(1.0, s.ad_value(2415)), A::mul(s.ad_value(2375), A::offset(s.ad_value(2427), 1.0)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(2422, A::sub(s.ad_value(2373), s.ad_value(2419)), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add(818, 2420, 2421);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2422), A::sub(A::scale(A::square(s.ad_value(2421)), 0.5), A::mul(s.ad_value(2420), s.ad_value(2437)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            let assign51970_ad_e66864: A = A::add(s.ad_value(2419), A::div(A::mul(A::mul(s.ad_value(2420), s.ad_value(818)), s.ad_value(2422)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422)), s.ad_value(2422)), s.ad_value(2421)), A::sub(A::scale(A::square(s.ad_value(2421)), 0.3333333333333333), A::mul(s.ad_value(2420), s.ad_value(2437)))))));
            s.store_ad(2438, &assign51970_ad_e66864);
        }

        s.v[2476] = if (s.v[2438] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (s.v[2476] != 0.0)) {
            s.store_exp(2424, 2438);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (s.v[2476] != 0.0)) {
            s.store_div_from_scalar(2425, 1.0, 2424);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (s.v[2476] != 0.0)) {
            s.store_mul(2424, 2375, 2424);
        }

        s.v[2477] = if (s.v[2438] > (s.v[2373] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (s.v[2477] != 0.0)) {
            s.store_exp_ad(2424, A::sub(s.ad_value(2438), s.ad_value(2373)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (s.v[2477] != 0.0)) {
            s.store_div(2425, 2375, 2424);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (!(s.v[2477] != 0.0))) {
            s.store_div_from_scalar_ad(2424, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (!(s.v[2477] != 0.0))) {
            s.store_div_from_scalar_ad(2425, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2438), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_div_from_scalar_ad(2414, 1.0, A::offset(A::square(s.ad_value(2438)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2438)), 2414);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2438), s.ad_value(2414)), s.ad_value(2414)), 4.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2414), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2414)), 2414);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub(2414, 2309, 2438);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(2429, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2425)), s.ad_value(2424)), A::mul(s.ad_value(2375), A::offset(s.ad_value(2427), 1.0)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_ad(2430, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::add(A::offset(A::add(s.ad_value(2425), s.ad_value(2438)), (-1.0)), s.ad_value(2424)), A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426))))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2425), s.ad_value(2424)), A::mul(s.ad_value(2375), s.ad_value(2428)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_ad(2414, A::square(s.ad_value(2429)), A::scale(A::mul(s.ad_value(2430), s.ad_value(2414)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad_rhs(2376, 2438, A::scale(A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_sub(2377, 2376, 2329);
        }

        s.v[2478] = if (s.v[2377] < 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_add_ad(2378, A::scale(A::sub(s.ad_value(2309), s.ad_value(2329)), 2.0), A::mul(s.ad_value(2291), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2335)), A::mul(s.ad_value(2334), s.ad_value(2374))), A::mul(s.ad_value(2375), A::offset(s.ad_value(2332), 1.0)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_mul_ad_lhs(2379, A::mul(s.ad_value(2291), A::sub_from_scalar(1.0, s.ad_value(2374))), 2336);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_sub_from_scalar_ad(1919, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2335), A::mul(s.ad_value(2334), s.ad_value(2374))), A::mul(s.ad_value(2375), s.ad_value(2333)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_sub_ad(1919, A::square(s.ad_value(2378)), A::scale(A::mul(s.ad_value(1919), s.ad_value(2379)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_scale_ad(2377, A::div(s.ad_value(2379), A::add(s.ad_value(2378), A::sqrt(s.ad_value(1919)))), 2.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_add(2376, 2329, 2377);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2380, 2377, 2305);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad(2381, A::square(s.ad_value(2376)), A::offset(A::square(s.ad_value(2376)), 2.0));
        }

        s.v[2479] = if (s.v[2376] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) {
            s.store_exp_ad(2382, A::neg(s.ad_value(2376)));
        }

        s.v[2480] = if (s.v[2376] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_scale_ad(2383, A::mul(A::square(s.ad_value(2376)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2376), A::sub_from_scalar(1.0, A::scale(s.ad_value(2376), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2376), A::sub_from_scalar(1.0, A::scale(s.ad_value(2376), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_scaled_mul(2384, 2376, 1919, 0.7071067811865475);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_mul_ad(2385, A::mul(A::mul(A::mul(A::scale(s.ad_value(2375), 0.16666666666666666), s.ad_value(2376)), s.ad_value(2376)), s.ad_value(2376)), A::offset(A::scale(s.ad_value(2376), 1.75), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (!(s.v[2480] != 0.0))) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (!(s.v[2480] != 0.0))) {
            s.store_sqrt(2384, 2383);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (!(s.v[2480] != 0.0))) {
            s.store_mul_ad_rhs(2385, 2375, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2382)), s.ad_value(2376)), (-1.0)), s.ad_value(2381)));
        }

        s.v[2481] = if (s.v[2376] > (s.v[2373] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (s.v[2481] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(2376), s.ad_value(2373)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (s.v[2481] != 0.0)) {
            s.store_div(2382, 2375, 1919);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (s.v[2481] != 0.0)) {
            s.store_sub_ad_rhs(2385, 1919, A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (!(s.v[2481] != 0.0))) {
            s.store_div_from_scalar_ad(2382, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2376), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2376), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2376), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (!(s.v[2481] != 0.0))) {
            s.store_div_from_scalar_ad(1919, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (!(s.v[2481] != 0.0))) {
            s.store_sub_ad_rhs(2385, 1919, A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) {
            s.store_sqrt(2384, 2383);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2386, A::mul(s.ad_value(2384), s.ad_value(2290)), 2305);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scaled_add(2387, 2329, 2376, 0.5);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scalar(2388, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(1919, 2382, 2335);
        }

        s.v[2482] = if (s.v[1919] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_sqrt(2388, 1919);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scaled_add(2389, 2336, 2385, 0.5);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2390, 2389, A::scale(A::mul(A::square(s.ad_value(2377)), A::sub(s.ad_value(2388), A::scale(s.ad_value(2307), 2.0))), 0.125));
        }

        s.v[2483] = if (s.v[2387] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_scale_ad(2391, A::mul(A::square(s.ad_value(2387)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2387), A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_mul_ad_rhs(2392, 2290, A::sqrt(A::add(s.ad_value(2390), s.ad_value(2391))));
        }

        s.v[2484] = if (s.v[724] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) && (s.v[2484] != 0.0)) {
            s.store_div_from_scalar_ad(2393, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2387), A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_scaled_mul(2394, 2387, 1919, 0.7071067811865475);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_add_ad_rhs(2395, 2393, A::scale(A::div(A::mul(s.ad_value(2290), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.5)), A::scale(A::square(s.ad_value(2387)), 0.16666666666666666))), s.ad_value(1919)), 0.7071067811865475));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_mul_ad_rhs(2392, 2290, A::sqrt(A::add(s.ad_value(2390), s.ad_value(2391))));
        }

        s.v[2485] = if (s.v[724] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2396, A::sub_from_scalar(1.0, s.ad_value(2388)), A::scale(A::mul(s.ad_value(2392), s.ad_value(2307)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_from_scalar_ad(2393, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_ad_rhs(1919, 2393, A::offset(s.ad_value(2393), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2397, 724, A::mul(A::mul(A::square(s.ad_value(1919)), s.ad_value(2291)), s.ad_value(2390)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2398, A::scale(A::sub(s.ad_value(2392), s.ad_value(2397)), 2.0), A::mul(s.ad_value(2291), A::add(A::sub_from_scalar(1.0, s.ad_value(2388)), s.ad_value(2390))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2399, 2397, A::sub(s.ad_value(2397), A::scale(s.ad_value(2392), 2.0)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_sub_from_scalar_ad(2400, 1.0, A::scale(A::mul(s.ad_value(2291), A::add(s.ad_value(2388), s.ad_value(2390))), 0.5));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_ad(2401, A::mul(s.ad_value(2399), s.ad_value(2398)), A::sub(A::square(s.ad_value(2398)), A::mul(s.ad_value(2400), s.ad_value(2399))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add(2387, 2387, 2401);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_exp(2402, 2401);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div(2388, 2388, 2402);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul(2390, 2390, 2402);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2392, 2290, A::sqrt(A::add(s.ad_value(2390), s.ad_value(2391))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2403, A::sub_from_scalar(1.0, s.ad_value(2388)), A::scale(A::mul(A::mul(s.ad_value(2392), s.ad_value(2393)), s.ad_value(2307)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_ad(2377, A::mul(A::mul(s.ad_value(2377), s.ad_value(2402)), A::add(s.ad_value(2396), s.ad_value(2389))), A::add(s.ad_value(2403), A::mul(s.ad_value(2402), s.ad_value(2389))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul(2380, 2377, 2305);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_sqrt(2394, 2391);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_add_ad_rhs(2395, 2393, A::scale(A::div(A::mul(s.ad_value(2290), A::sub_from_scalar(1.0, s.ad_value(2388))), s.ad_value(2394)), 0.5));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_rhs(2404, 2305, A::div(A::mul(s.ad_value(2291), s.ad_value(2390)), A::add(s.ad_value(2392), A::mul(s.ad_value(2290), s.ad_value(2394)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2405, 2404, A::mul(s.ad_value(2305), s.ad_value(2395)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2406, A::mul(s.ad_value(2394), s.ad_value(2290)), 2305);
        }

        s.v[2486] = if (s.v[213] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2486] != 0.0)) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2404)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2486] != 0.0))) {
            s.store_div_from_scalar_ad(2345, 1.0, A::offset(A::mul(s.ad_value(213), s.ad_value(2404)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2346, A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), 2404);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2407, 2406, A::mul(s.ad_value(769), s.ad_value(2404)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2408, 2406, A::mul(s.ad_value(770), s.ad_value(2404)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2409, 768, 2407);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_ln_ad(1920, A::div(s.ad_value(2391), A::offset(A::add(s.ad_value(2391), s.ad_value(2390)), 1e-14)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad(2348, A::pow(A::mul(s.ad_value(2409), s.ad_value(698)), s.ad_value(699)), A::mul(s.ad_value(700), A::exp(A::mul(A::scale(s.ad_value(701), 0.5), s.ad_value(1920)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2410, A::add(A::offset(s.ad_value(2348), 1.0), s.ad_value(2346)), 2340);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_ln_ad(2411, A::div(A::offset(A::mul(A::sub(s.ad_value(820), s.ad_value(2380)), s.ad_value(773)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2371), s.ad_value(2380)), s.ad_value(773)), 1.0)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(1921, 2404, 2350);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.v[2487] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2487] != 0.0)) {
            s.store_div_from_scalar_ad(2352, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2351))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2487] != 0.0))) {
            s.store_offset_ad(2352, A::mul(s.ad_value(217), s.ad_value(2351)), 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2413, 2285, 2352);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2412, 2392, 2305);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1875, 2287);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1876, 2305);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1877, 2290);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1878, 2309);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1879, 2314);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1880, 2343);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[2439] != 0.0) {
            s.copy_ad(1881, 2380);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1882, 2386);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1883, 2393);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1884, 2395);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1885, 2404);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1886, 2405);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1887, 2408);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1888, 2410);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1889, 2411);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1890, 2413);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1891, 2412);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(739, 722);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1875, 1810);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1876, 1812);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1877, 1814);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1878, 1817);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1879, 1818);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1880, 1837);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1881, 1848);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1882, 1849);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1883, 1851);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1884, 1852);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1885, 1853);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1886, 1854);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1887, 1856);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1888, 1857);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1889, 1859);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1890, 1858);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1891, 1860);
        }

        s.copy_ad(1892, 250);

        s.v[2488] = if (s.v[767] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2488] != 0.0) {
            s.store_div_ad_rhs(1892, 250, A::offset(A::mul(s.ad_value(767), A::powf(A::offset(A::square(s.ad_value(1887)), s.v[727]), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1893] = 1.0;

        s.v[1894] = 1.0;

        s.v[1895] = 0.0;

        s.v[1896] = 1.0;

        s.v[1897] = 1.0;

        s.copy_ad(2251, 1891);

        s.v[2254] = 0.0;

        s.v[2253] = 0.0;

        s.copy_ad(2255, 2251);

        s.v[2489] = if (s.v[1878] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2489] != 0.0) {
            s.store_mul_ad_lhs(2246, A::div(A::mul(A::add(s.ad_value(255), A::div(s.ad_value(256), s.ad_value(1886))), s.ad_value(1885)), s.ad_value(1886)), 1889);
        }

        s.v[2490] = if (s.v[2246] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2489] != 0.0) && (s.v[2490] != 0.0)) {
            s.store_div_from_scalar_ad(1893, 1.0, A::add(A::offset(s.ad_value(2246), 1.0), A::square(s.ad_value(2246))));
        }

        if ((s.v[2489] != 0.0) && (!(s.v[2490] != 0.0))) {
            s.store_sub_from_scalar(1893, 1.0, 2246);
        }

        if (s.v[2489] != 0.0) {
            s.store_mul(1894, 1888, 1893);
        }

        if (s.v[2489] != 0.0) {
            s.store_div(1895, 1890, 1894);
        }

        if (s.v[2489] != 0.0) {
            s.store_mul_ad_lhs(2247, A::mul(A::square(s.ad_value(1895)), s.ad_value(1881)), 1881);
        }

        s.v[2491] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2489] != 0.0) && (s.v[2491] != 0.0)) {
            s.store_div_ad_rhs(2247, 2247, A::offset(A::mul(s.ad_value(1895), s.ad_value(1881)), 1.0));
        }

        if (s.v[2489] != 0.0) {
            s.store_scale_ad(1896, A::mul(s.ad_value(1894), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2247), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2489] != 0.0) {
            s.store_div(1919, 1894, 1896);
        }

        if (s.v[2489] != 0.0) {
            s.store_mul_ad_rhs(2248, 1884, A::offset(A::scale(A::mul(A::mul(s.ad_value(2247), s.ad_value(1919)), s.ad_value(1919)), 0.5), 1.0));
        }

        if (s.v[2489] != 0.0) {
            s.store_div_ad_lhs(1897, A::mul(s.ad_value(1919), s.ad_value(1886)), 2248);
        }

        if (s.v[2489] != 0.0) {
            s.store_scaled_div(2249, 1881, 1897, 0.5);
        }

        if (s.v[2489] != 0.0) {
            s.store_square(2250, 2249);
        }

        if (s.v[2489] != 0.0) {
            s.store_add_ad_rhs(2251, 1891, A::scale(A::mul(A::mul(s.ad_value(1883), s.ad_value(1881)), A::add(A::offset(A::scale(A::mul(s.ad_value(2249), s.ad_value(1893)), 0.3333333333333333), (-1.0)), s.ad_value(1893))), 0.5));
        }

        if (s.v[2489] != 0.0) {
            s.store_scaled_mul(1919, 1884, 1881, 0.16666666666666666);
        }

        s.v[2492] = if (p.p49 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2489] != 0.0) && (s.v[2492] != 0.0)) {
            s.store_scalar(2252, 0.0);
        }

        if ((s.v[2489] != 0.0) && (s.v[2492] != 0.0)) {
            s.store_mul_ad(2253, A::mul(A::scale(s.ad_value(1893), 0.5), s.ad_value(1893)), A::sub(s.ad_value(1885), A::mul(A::scale(s.ad_value(1919), 3.0), A::sub_from_scalar(2.0, s.ad_value(2249)))));
        }

        if ((s.v[2489] != 0.0) && (!(s.v[2492] != 0.0))) {
            s.store_mul_ad(2252, A::sub_from_scalar(1.0, s.ad_value(1893)), A::sub(s.ad_value(1885), A::scale(A::mul(s.ad_value(1884), s.ad_value(1881)), 0.5)));
        }

        if ((s.v[2489] != 0.0) && (!(s.v[2492] != 0.0))) {
            s.store_scale_ad(2253, A::add(A::mul(A::square(s.ad_value(1893)), A::sub(s.ad_value(1885), A::mul(s.ad_value(1919), A::sub(A::sub_from_scalar(1.0, s.ad_value(2249)), A::scale(s.ad_value(2250), 0.2))))), A::mul(s.ad_value(2252), A::offset(s.ad_value(1893), 1.0))), 0.5);
        }

        if (s.v[2489] != 0.0) {
            s.store_add_ad_lhs(2254, A::mul(s.ad_value(1893), A::add(s.ad_value(1885), A::mul(s.ad_value(1919), s.ad_value(2249)))), 2252);
        }

        if (s.v[2489] != 0.0) {
            s.store_sub(2255, 2251, 2254);
        }

        s.store_mul(845, 2251, 1892);

        s.store_mul_ad_lhs(847, A::neg(s.ad_value(2253)), 1892);

        s.store_mul_ad_lhs(846, A::neg(s.ad_value(2255)), 1892);

        s.v[2271] = 0.0;

        s.v[2272] = 0.0;

        s.v[2270] = 0.0;

        s.v[2493] = if ((s.v[263] > 0.0) || (s.v[264] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2493] != 0.0) {
            s.store_scalar(2260, 1.0);
        }

        if (s.v[2493] != 0.0) {
            s.copy_ad(2259, 1875);
        }

        s.v[2494] = if (s.v[267] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_add_ad_lhs(2256, A::sub(s.ad_value(1875), s.ad_value(265)), 802);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_scale_ad(1919, A::add(A::add(s.ad_value(2256), s.ad_value(802)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(2256), s.ad_value(802)), A::sub(s.ad_value(2256), s.ad_value(802))), s.ad_value(803)))), 0.5);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_mul_ad_rhs(1920, 1919, A::sub(A::sub(A::scale(s.ad_value(1919), 2.0), s.ad_value(802)), s.ad_value(2256)));
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_div(1921, 802, 1919);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_mul(2257, 2256, 1921);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_sqrt_ad(2258, A::sub_from_scalar(1.0, A::mul(s.ad_value(2257), s.ad_value(267))));
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_sub_ad_lhs(2259, A::add(A::div(A::sub_from_scalar(1.0, s.ad_value(2258)), s.ad_value(267)), s.ad_value(2256)), 2257);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_offset_ad(2260, A::div(A::mul(A::mul(A::offset(A::div_from_scalar(0.5, s.ad_value(2258)), (-1.0)), A::add(s.ad_value(1920), A::mul(s.ad_value(2256), A::sub(s.ad_value(802), s.ad_value(1919))))), s.ad_value(1921)), s.ad_value(1920)), 1.0);
        }

        if (s.v[2493] != 0.0) {
            s.store_scalar(2262, 1.0);
        }

        if (s.v[2493] != 0.0) {
            s.store_scalar(2263, 0.0);
        }

        s.v[2495] = if (s.v[266] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) {
            s.store_add_ad(1919, A::scale(s.ad_value(739), 0.5), A::mul(s.ad_value(1876), A::offset(A::scale(s.ad_value(1877), 0.7071067811865475), 1.0)));
        }

        if ((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) {
            s.store_div(2261, 1875, 1919);
        }

        s.v[2496] = if (((s.v[2261]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (s.v[2496] != 0.0)) {
            s.store_div_from_scalar_ad(2262, 1.0, A::offset(A::exp(A::neg(s.ad_value(2261))), 1.0));
        }

        s.v[2497] = if (s.v[2261] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (!(s.v[2496] != 0.0))) && (s.v[2497] != 0.0)) {
            s.store_div_from_scalar_ad(2262, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2261), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2261), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2261), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2498] = if (s.v[2261] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (s.v[2498] != 0.0)) {
            s.store_ln_ad(1920, A::offset(A::exp(s.ad_value(2261)), 1.0));
        }

        if (((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (!(s.v[2498] != 0.0))) {
            s.copy_ad(1920, 2261);
        }

        if ((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) {
            s.store_mul(2263, 1919, 1920);
        }

        if (s.v[2493] != 0.0) {
            s.store_add_ad_lhs(2264, A::mul(s.ad_value(266), A::sub(s.ad_value(2262), s.ad_value(2260))), 2260);
        }

        if (s.v[2493] != 0.0) {
            s.store_add_ad_lhs(2265, A::mul(s.ad_value(266), A::sub(s.ad_value(2263), s.ad_value(2259))), 2259);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad(2266, A::sub(A::sub(s.ad_value(1875), A::mul(s.ad_value(1876), s.ad_value(1879))), s.ad_value(1891)), A::scale(s.ad_value(1881), 0.5));
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(2267, A::sub(s.ad_value(1875), s.ad_value(2266)), 1880);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(2268, A::add(s.ad_value(1881), s.ad_value(2266)), 820);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(2269, A::sub(s.ad_value(1875), s.ad_value(2268)), 1882);
        }

        s.v[2499] = if (s.v[825] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2493] != 0.0) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2270, 2264, A::add(A::mul(s.ad_value(264), s.ad_value(2268)), A::mul(s.ad_value(263), s.ad_value(2266))));
        }

        if ((s.v[2493] != 0.0) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2271, 263, A::sub(s.ad_value(2267), s.ad_value(2265)));
        }

        if ((s.v[2493] != 0.0) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2272, 264, A::sub(s.ad_value(2269), s.ad_value(2265)));
        }

        if ((s.v[2493] != 0.0) && (!(s.v[2499] != 0.0))) {
            s.store_mul_ad_rhs(2270, 2264, A::add(A::mul(s.ad_value(263), s.ad_value(2268)), A::mul(s.ad_value(264), s.ad_value(2266))));
        }

        if ((s.v[2493] != 0.0) && (!(s.v[2499] != 0.0))) {
            s.store_mul_ad_rhs(2271, 264, A::sub(s.ad_value(2267), s.ad_value(2265)));
        }

        if ((s.v[2493] != 0.0) && (!(s.v[2499] != 0.0))) {
            s.store_mul_ad_rhs(2272, 263, A::sub(s.ad_value(2269), s.ad_value(2265)));
        }

        if (s.v[2493] != 0.0) {
            s.store_add(845, 845, 2270);
        }

        if (s.v[2493] != 0.0) {
            s.store_add(847, 847, 2272);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(846), s.ad_value(2270)), s.ad_value(2272)), 2271);
        }

        s.store_mul(1898, 257, 1866);

        s.store_mul(1899, 258, 1867);

        s.v[2275] = 0.0;

        s.v[2273] = 0.0;

        s.v[2500] = if ((s.v[257] > 0.0) && (s.v[259] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2500] != 0.0) {
            s.store_mul_ad_rhs(1919, 261, A::add(A::scale(s.ad_value(1807), 0.5), s.ad_value(781)));
        }

        s.v[2501] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2502] = if (s.v[1919] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (s.v[2502] != 0.0)) {
            s.store_exp(2273, 1919);
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (!(s.v[2502] != 0.0))) {
            s.store_div_from_scalar_ad(2273, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2503] = if (s.v[2273] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (s.v[2503] != 0.0)) {
            s.store_ln_ad(2274, A::offset(s.ad_value(2273), 1.0));
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (s.v[2503] != 0.0)) {
            s.store_mul_ad_rhs(1920, 2274, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0))));
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (!(s.v[2503] != 0.0))) {
            s.copy_ad(2274, 2273);
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (!(s.v[2503] != 0.0))) {
            s.store_div_ad(1920, A::scale(s.ad_value(2274), 2.0), A::offset(s.ad_value(2274), 2.0));
        }

        if ((s.v[2500] != 0.0) && (!(s.v[2501] != 0.0))) {
            s.copy_ad(2274, 1919);
        }

        if ((s.v[2500] != 0.0) && (!(s.v[2501] != 0.0))) {
            s.store_mul_ad_rhs(1920, 2274, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0))));
        }

        if (s.v[2500] != 0.0) {
            s.store_mul_ad_lhs(2275, A::scale(A::mul(A::div(A::scale(s.ad_value(259), (-2.0)), s.ad_value(261)), s.ad_value(257)), s.v[348]), 1920);
        }

        s.v[2278] = 0.0;

        s.v[2276] = 0.0;

        s.v[2504] = if ((s.v[258] > 0.0) && (s.v[260] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2504] != 0.0) {
            s.store_mul_ad_rhs(1919, 261, A::add(A::scale(s.ad_value(1807), 0.5), s.ad_value(782)));
        }

        s.v[2505] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2506] = if (s.v[1919] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (s.v[2506] != 0.0)) {
            s.store_exp(2276, 1919);
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (!(s.v[2506] != 0.0))) {
            s.store_div_from_scalar_ad(2276, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2507] = if (s.v[2276] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (s.v[2507] != 0.0)) {
            s.store_ln_ad(2277, A::offset(s.ad_value(2276), 1.0));
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (s.v[2507] != 0.0)) {
            s.store_mul_ad_rhs(1920, 2277, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0))));
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (!(s.v[2507] != 0.0))) {
            s.copy_ad(2277, 2276);
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (!(s.v[2507] != 0.0))) {
            s.store_div_ad(1920, A::scale(s.ad_value(2277), 2.0), A::offset(s.ad_value(2277), 2.0));
        }

        if ((s.v[2504] != 0.0) && (!(s.v[2505] != 0.0))) {
            s.copy_ad(2277, 1919);
        }

        if ((s.v[2504] != 0.0) && (!(s.v[2505] != 0.0))) {
            s.store_mul_ad_rhs(1920, 2277, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0))));
        }

        if (s.v[2504] != 0.0) {
            s.store_mul_ad_lhs(2278, A::scale(A::mul(A::div(A::scale(s.ad_value(260), (-2.0)), s.ad_value(261)), s.ad_value(258)), s.v[348]), 1920);
        }

        s.store_add(2279, 2275, 2278);

        s.store_add_ad_lhs(850, A::mul(s.ad_value(262), s.ad_value(823)), 2279);

        s.store_mul(848, 269, 828);

        s.store_mul(849, 270, 831);

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[2508] = 0.0;

        s.v[2511] = 0.0;

        s.v[2512] = 0.0;

        s.v[2513] = 0.0;

        s.v[2514] = 0.0;

        s.v[2515] = 0.0;

        s.v[2516] = 0.0;

        s.v[2517] = 0.0;

        s.v[2518] = 0.0;

        s.v[2519] = 0.0;

        s.v[2520] = 0.0;

        s.v[2521] = 0.0;

        s.v[2522] = 0.0;

        s.v[2523] = 0.0;

        s.v[2524] = 0.0;

        s.v[2525] = 0.0;

        s.v[2526] = 0.0;

        s.v[2529] = 0.0;

        s.v[2533] = 0.0;

        s.v[2536] = 0.0;

        s.v[2537] = 0.0;

        s.v[2538] = 0.0;

        s.v[2539] = 0.0;

        s.v[2540] = 0.0;

        s.v[2541] = 0.0;

        s.v[2544] = 0.0;

        s.v[2545] = 0.0;

        s.v[2546] = 0.0;

        s.v[2547] = 0.0;

        s.v[2551] = 0.0;

        s.v[2553] = 0.0;

        s.v[2554] = 0.0;

        s.v[851] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.v[1908] = 0.0;

        s.v[852] = 0.0;

        s.v[1909] = 0.0;

        s.v[1910] = 0.0;

        s.v[1911] = 0.0;

        s.v[2555] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[2556] = if (s.v[468] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2559, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2560, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add_ad_rhs(2513, 826, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scale_ad(2560, A::div(A::mul(s.ad_value(826), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2561] = if (s.v[645] > 0.5) { 1.0 } else { 0.0 };

        s.v[2562] = if (s.v[402] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2561] != 0.0)) && (s.v[2562] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2561] != 0.0)) && (!(s.v[2562] != 0.0))) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])), s.v[402]);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2561] != 0.0)) {
            s.store_add_ad(1906, A::scale(A::sub_from_scalar(1.0, s.ad_value(2559)), s.v[411]), A::scale(A::sub(s.ad_value(826), s.ad_value(2560)), s.v[414]));
        }

        s.v[2563] = if (s.v[646] > 0.5) { 1.0 } else { 0.0 };

        s.v[2564] = if (s.v[403] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2563] != 0.0)) && (s.v[2564] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2563] != 0.0)) && (!(s.v[2564] != 0.0))) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])), s.v[403]);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2563] != 0.0)) {
            s.store_add_ad(1907, A::scale(A::sub_from_scalar(1.0, s.ad_value(2559)), s.v[412]), A::scale(A::sub(s.ad_value(826), s.ad_value(2560)), s.v[415]));
        }

        s.v[2565] = if (s.v[647] > 0.5) { 1.0 } else { 0.0 };

        s.v[2566] = if (s.v[404] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])), s.v[404]);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_add_ad(1908, A::scale(A::sub_from_scalar(1.0, s.ad_value(2559)), s.v[413]), A::scale(A::sub(s.ad_value(826), s.ad_value(2560)), s.v[416]));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2559, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2560, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add_ad_rhs(2513, 827, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scale_ad(2560, A::div(A::mul(s.ad_value(827), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2567] = if (s.v[672] > 0.5) { 1.0 } else { 0.0 };

        s.v[2568] = if (s.v[569] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2567] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(566))));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2567] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_ad(2559, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(566))), s.ad_value(569)));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2567] != 0.0)) {
            s.store_add_ad(1909, A::mul(s.ad_value(578), A::sub_from_scalar(1.0, s.ad_value(2559))), A::mul(s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2560))));
        }

        s.v[2569] = if (s.v[673] > 0.5) { 1.0 } else { 0.0 };

        s.v[2570] = if (s.v[570] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2570] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(567))));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2570] != 0.0))) {
            s.store_ad(2559, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(567))), s.ad_value(570)));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_add_ad(1910, A::mul(s.ad_value(579), A::sub_from_scalar(1.0, s.ad_value(2559))), A::mul(s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2560))));
        }

        s.v[2571] = if (s.v[674] > 0.5) { 1.0 } else { 0.0 };

        s.v[2572] = if (s.v[571] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2571] != 0.0)) && (s.v[2572] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(568))));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2571] != 0.0)) && (!(s.v[2572] != 0.0))) {
            s.store_ad(2559, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(568))), s.ad_value(571)));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2571] != 0.0)) {
            s.store_add_ad(1911, A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(2559))), A::mul(s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2560))));
        }

        s.v[2573] = if (p.p865 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_scale_ad(636, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), p.p866), (-(((0.5 * 0.001)) as f64).powf(p.p866))), p.p865);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_offset(634, 636, p.p855);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_div_from_scalar(444, 1.0, 634);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2573] != 0.0))) {
            s.store_scalar(634, p.p855);
        }

        s.v[2574] = if (p.p867 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2574] != 0.0)) {
            s.store_scale_ad(638, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), p.p868), (-(((0.5 * 0.001)) as f64).powf(p.p868))), p.p867);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2574] != 0.0)) {
            s.store_mul_ad_rhs(437, 437, A::offset(s.ad_value(638), 1.0));
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2524, 0.0);
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2521, 0.0);
        }

        s.v[2575] = if !(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2513, 826, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2518, A::div(A::mul(s.ad_value(826), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2576] = if (s.v[826] < s.v[648]) { 1.0 } else { 0.0 };

        s.v[2577] = if (((((-0.5) * (s.v[826] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) {
            s.store_exp_ad(2519, A::scale(s.ad_value(826), (s.v[365] * (-0.5))));
        }

        s.v[2578] = if (((-0.5) * (s.v[826] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2577] != 0.0))) && (s.v[2578] != 0.0)) {
            let assign56250_ad_e70980: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2519, &assign56250_ad_e70980);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2577] != 0.0))) && (!(s.v[2578] != 0.0))) {
            s.store_scale_ad(2519, A::offset(A::mul(A::offset(A::scale(s.ad_value(826), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(826), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(826), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_from_scalar(2520, 1.0, 2519);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_square(2517, 2520);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_mul_ad_lhs(2517, A::offset(A::scale(A::sub(s.ad_value(826), s.ad_value(648)), s.v[365]), 1.0), 649);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_sqrt(2520, 2517);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.v[2579] = if (s.v[826] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_scale_ad(2521, A::ln(A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2519), 1.0), A::offset(s.ad_value(2519), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2579] != 0.0))) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2520), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2520), 1.0), A::offset(A::scale(s.ad_value(2520), 3.0), 1.0))))), (s.v[364] * 2.0)), 826);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_sub(2522, 650, 2521);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2523, A::sub(A::add(s.ad_value(826), s.ad_value(2522)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2522)), A::sub(s.ad_value(826), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2524, A::sub(A::add(s.ad_value(826), s.ad_value(653)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(653)), A::sub(s.ad_value(826), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2525, A::sub(s.ad_value(826), A::sqrt(A::offset(A::mul(s.ad_value(826), s.ad_value(826)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2580] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2580] != 0.0)) {
            s.store_scalar(1906, 0.0);
        }

        s.v[2581] = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_sub_from_scalar(2529, s.v[387], 2523);
        }

        s.v[2583] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) && (s.v[2583] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(s.ad_value(2529), s.v[423]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) && (!(s.v[2583] != 0.0))) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[423]), p.p824);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_scale(2533, 2526, s.v[417]);
        }

        s.v[2584] = if (p.p838 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_scale_ad(2536, A::div(A::scale(s.ad_value(2533), s.v[402]), s.ad_value(2529)), s.v[432]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[429]), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::scale(s.ad_value(2537), s.v[429]), s.ad_value(2540)), A::scale(s.ad_value(2539), s.v[429])), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2587] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2587] != 0.0))) {
            let assign56800_ad_e71930: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign56800_ad_e71930);
        }

        s.v[2588] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        s.v[2589] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2588] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2588] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2590] = if (p.p844 == 0.0) { 1.0 } else { 0.0 };

        s.v[2591] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (s.v[2591] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (!(s.v[2591] != 0.0))) {
            s.store_powf_ad(2526, A::scale(A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]), p.p824);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) {
            s.store_scale_ad(2551, A::div(A::scale(A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[420]), s.ad_value(2526)), s.v[405]);
        }

        s.v[2592] = if (((((-s.v[435]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (s.v[2592] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(435)), s.ad_value(2551)));
        }

        s.v[2593] = if (((-s.v[435]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (!(s.v[2592] != 0.0))) && (s.v[2593] != 0.0)) {
            let assign56990_ad_e72270: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign56990_ad_e72270);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (!(s.v[2592] != 0.0))) && (!(s.v[2593] != 0.0))) {
            let assign57000_ad_e72321: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign57000_ad_e72321);
        }

        s.v[2594] = if (p.p853 > 1000.0) { 1.0 } else { 0.0 };

        s.v[2595] = if (s.v[2525] > ((-s.v[438]) * p.p853)) { 1.0 } else { 0.0 };

        s.v[2596] = if (p.p856 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2595] != 0.0)) && (s.v[2596] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::scale(s.ad_value(2525), s.v[442]), A::scale(s.ad_value(2525), s.v[442])), A::scale(s.ad_value(2525), s.v[442])), A::scale(s.ad_value(2525), s.v[442]));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2595] != 0.0)) && (!(s.v[2596] != 0.0))) {
            s.store_powf_ad(2526, A::abs(A::scale(s.ad_value(2525), s.v[442])), p.p856);
        }

        s.v[2597] = if (s.v[402] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (s.v[2597] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2597] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])), s.v[402]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) {
            s.store_scale_ad(1906, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[411]), A::scale(A::sub(s.ad_value(826), s.ad_value(2518)), s.v[414])), p.p30);
        }

        s.v[2598] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2598] != 0.0)) {
            s.store_scalar(1907, 0.0);
        }

        s.v[2599] = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_sub_from_scalar(2529, s.v[388], 2523);
        }

        s.v[2601] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) && (s.v[2601] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(s.ad_value(2529), s.v[424]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) && (!(s.v[2601] != 0.0))) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[424]), p.p825);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_scale(2533, 2526, s.v[418]);
        }

        s.v[2602] = if (p.p839 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_scale_ad(2536, A::div(A::scale(s.ad_value(2533), s.v[403]), s.ad_value(2529)), s.v[433]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[430]), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::scale(s.ad_value(2537), s.v[430]), s.ad_value(2540)), A::scale(s.ad_value(2539), s.v[430])), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2605] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2605] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2605] != 0.0))) {
            let assign57550_ad_e73196: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign57550_ad_e73196);
        }

        s.v[2606] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        s.v[2607] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2606] != 0.0))) && (s.v[2607] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2606] != 0.0))) && (!(s.v[2607] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2608] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        s.v[2609] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2609] != 0.0))) {
            s.store_powf_ad(2526, A::scale(A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]), p.p825);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) {
            s.store_scale_ad(2551, A::div(A::scale(A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[421]), s.ad_value(2526)), s.v[406]);
        }

        s.v[2610] = if (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2610] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(436)), s.ad_value(2551)));
        }

        s.v[2611] = if (((-s.v[436]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2610] != 0.0))) && (s.v[2611] != 0.0)) {
            let assign57740_ad_e73536: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign57740_ad_e73536);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2610] != 0.0))) && (!(s.v[2611] != 0.0))) {
            let assign57750_ad_e73587: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign57750_ad_e73587);
        }

        s.v[2612] = if (p.p854 > 1000.0) { 1.0 } else { 0.0 };

        s.v[2613] = if (s.v[2525] > ((-s.v[438]) * p.p854)) { 1.0 } else { 0.0 };

        s.v[2614] = if (p.p857 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2613] != 0.0)) && (s.v[2614] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::scale(s.ad_value(2525), s.v[443]), A::scale(s.ad_value(2525), s.v[443])), A::scale(s.ad_value(2525), s.v[443])), A::scale(s.ad_value(2525), s.v[443]));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2613] != 0.0)) && (!(s.v[2614] != 0.0))) {
            s.store_powf_ad(2526, A::abs(A::scale(s.ad_value(2525), s.v[443])), p.p857);
        }

        s.v[2615] = if (s.v[403] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2615] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2615] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale_ad(1907, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[412]), A::scale(A::sub(s.ad_value(826), s.ad_value(2518)), s.v[415])), p.p30);
        }

        s.v[2616] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2616] != 0.0)) {
            s.store_scalar(1908, 0.0);
        }

        s.v[2617] = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_sub_from_scalar(2529, s.v[389], 2523);
        }

        s.v[2619] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) && (s.v[2619] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(s.ad_value(2529), s.v[425]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) && (!(s.v[2619] != 0.0))) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[425]), p.p826);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_scale(2533, 2526, s.v[419]);
        }

        s.v[2620] = if (p.p840 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_scale_ad(2536, A::div(A::scale(s.ad_value(2533), s.v[404]), s.ad_value(2529)), s.v[434]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::scale(s.ad_value(2537), s.v[431]), s.ad_value(2540)), A::scale(s.ad_value(2539), s.v[431])), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2623] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2623] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2623] != 0.0))) {
            let assign58300_ad_e74462: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign58300_ad_e74462);
        }

        s.v[2624] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        s.v[2625] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2624] != 0.0))) && (s.v[2625] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2624] != 0.0))) && (!(s.v[2625] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2626] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        s.v[2627] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2627] != 0.0))) {
            s.store_powf_ad(2526, A::scale(A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]), p.p826);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) {
            s.store_scale_ad(2551, A::div(A::scale(A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[422]), s.ad_value(2526)), s.v[407]);
        }

        s.v[2628] = if (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2628] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(437)), s.ad_value(2551)));
        }

        s.v[2629] = if (((-s.v[437]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2628] != 0.0))) && (s.v[2629] != 0.0)) {
            let assign58490_ad_e74802: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign58490_ad_e74802);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2628] != 0.0))) && (!(s.v[2629] != 0.0))) {
            let assign58500_ad_e74853: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign58500_ad_e74853);
        }

        s.v[2630] = if (s.v[634] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2631] = if (s.v[2525] > ((-s.v[438]) * s.v[634])) { 1.0 } else { 0.0 };

        s.v[2632] = if (p.p858 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2631] != 0.0)) && (s.v[2632] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(444)), A::mul(s.ad_value(2525), s.ad_value(444))), A::mul(s.ad_value(2525), s.ad_value(444))), A::mul(s.ad_value(2525), s.ad_value(444)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2631] != 0.0)) && (!(s.v[2632] != 0.0))) {
            s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);
        }

        s.v[2633] = if (s.v[467] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            let assign58620_ad_e75078: A = {
                if (s.v[826] < p.p863) {
                    {
                        if (((s.v[826] - p.p863) / p.p864) < (-37.0)) {
                            A::constant(p.p863)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(826), (-p.p863)), 1.0 / (p.p864))), 1.0)), p.p864), p.p863)
                        }
                    }
                } else {
                    {
                        if (((s.v[826] - p.p863) / p.p864) > 37.0) {
                            s.ad_value(826)
                        } else {
                            A::add(s.ad_value(826), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p863, s.ad_value(826)), 1.0 / (p.p864))), 1.0)), p.p864))
                        }
                    }
                }
            };
            s.store_ad(2553, &assign58620_ad_e75078);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2634] = if (s.v[404] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (s.v[2634] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (!(s.v[2634] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])), s.v[404]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(1908, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[413]), A::scale(A::sub(s.ad_value(2553), s.ad_value(2554)), s.v[416])), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sub_ad_lhs(2553, A::offset(s.ad_value(826), p.p863), 2553);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2635] = if (s.v[461] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (s.v[2635] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(460))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (!(s.v[2635] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(460))), s.ad_value(461)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(466, A::add(A::mul(s.ad_value(464), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(465), A::sub(s.ad_value(2553), s.ad_value(2554)))), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add(1908, 1908, 466);
        }

        s.v[2636] = if (s.v[404] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2633] != 0.0))) && (s.v[2636] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2633] != 0.0))) && (!(s.v[2636] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2633] != 0.0))) {
            s.store_scale_ad(1908, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[413]), A::scale(A::sub(s.ad_value(826), s.ad_value(2518)), s.v[416])), p.p30);
        }

        s.v[2637] = if (s.v[630] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_mul_ad_rhs(637, 630, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631))));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_add(635, 536, 637);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_div_from_scalar(610, 1.0, 635);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2637] != 0.0))) {
            s.copy_ad(635, 536);
        }

        s.v[2638] = if (s.v[632] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2638] != 0.0)) {
            s.store_mul_ad_rhs(639, 632, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), s.ad_value(633)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(633))));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2638] != 0.0)) {
            s.store_mul_ad_rhs(604, 604, A::offset(s.ad_value(639), 1.0));
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2524, 0.0);
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2521, 0.0);
        }

        s.v[2639] = if !(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_add_ad_rhs(2513, 827, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2518, A::div(A::mul(s.ad_value(827), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2640] = if (s.v[827] < s.v[675]) { 1.0 } else { 0.0 };

        s.v[2641] = if (((((-0.5) * (s.v[827] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) && (s.v[2641] != 0.0)) {
            s.store_exp_ad(2519, A::scale(s.ad_value(827), (s.v[365] * (-0.5))));
        }

        s.v[2642] = if (((-0.5) * (s.v[827] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) && (!(s.v[2641] != 0.0))) && (s.v[2642] != 0.0)) {
            let assign59150_ad_e75912: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2519, &assign59150_ad_e75912);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) && (!(s.v[2641] != 0.0))) && (!(s.v[2642] != 0.0))) {
            s.store_scale_ad(2519, A::offset(A::mul(A::offset(A::scale(s.ad_value(827), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(827), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(827), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) {
            s.store_div_from_scalar(2520, 1.0, 2519);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) {
            s.store_square(2517, 2520);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2640] != 0.0))) {
            s.store_mul_ad_lhs(2517, A::offset(A::scale(A::sub(s.ad_value(827), s.ad_value(675)), s.v[365]), 1.0), 676);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2640] != 0.0))) {
            s.store_sqrt(2520, 2517);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2640] != 0.0))) {
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.v[2643] = if (s.v[827] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2643] != 0.0)) {
            s.store_scale_ad(2521, A::ln(A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2519), 1.0), A::offset(s.ad_value(2519), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2643] != 0.0))) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2520), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2520), 1.0), A::offset(A::scale(s.ad_value(2520), 3.0), 1.0))))), (s.v[364] * 2.0)), 827);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_sub(2522, 677, 2521);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2523, A::sub(A::add(s.ad_value(827), s.ad_value(2522)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(2522)), A::sub(s.ad_value(827), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2524, A::sub(A::add(s.ad_value(827), s.ad_value(680)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(680)), A::sub(s.ad_value(827), s.ad_value(680))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2525, A::sub(s.ad_value(827), A::sqrt(A::offset(A::mul(s.ad_value(827), s.ad_value(827)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2644] = if (s.v[667] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2644] != 0.0)) {
            s.store_scalar(1909, 0.0);
        }

        s.v[2645] = if ((s.v[516] == 0.0) && (s.v[519] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_sub(2529, 563, 2523);
        }

        s.v[2647] = if (s.v[505] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(s.ad_value(2529), s.ad_value(590)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) && (!(s.v[2647] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(s.ad_value(2529), s.ad_value(590)), s.ad_value(505)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_mul(2533, 584, 2526);
        }

        s.v[2648] = if (s.v[519] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul_ad_rhs(2536, 599, A::div(A::mul(s.ad_value(2533), s.ad_value(569)), s.ad_value(2529)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_div_ad_lhs(2537, A::scale(s.ad_value(596), 0.666666666666667), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::mul(s.ad_value(596), s.ad_value(2537)), s.ad_value(2540)), A::mul(s.ad_value(596), s.ad_value(2539))), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2651] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2651] != 0.0))) {
            let assign59700_ad_e76862: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign59700_ad_e76862);
        }

        s.v[2652] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        s.v[2653] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2652] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2652] != 0.0))) && (!(s.v[2653] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2654] = if (s.v[525] == 0.0) { 1.0 } else { 0.0 };

        s.v[2655] = if (s.v[505] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (s.v[2655] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (!(s.v[2655] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590)), s.ad_value(505)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) {
            s.store_mul_ad_rhs(2551, 572, A::div(A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(587)), s.ad_value(2526)));
        }

        s.v[2656] = if (((((-s.v[602]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (s.v[2656] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(602)), s.ad_value(2551)));
        }

        s.v[2657] = if (((-s.v[602]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (!(s.v[2656] != 0.0))) && (s.v[2657] != 0.0)) {
            let assign59890_ad_e77202: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(602)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(602)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(602)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign59890_ad_e77202);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (!(s.v[2656] != 0.0))) && (!(s.v[2657] != 0.0))) {
            let assign59900_ad_e77253: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(602)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(602)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(602)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign59900_ad_e77253);
        }

        s.v[2658] = if (s.v[534] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2659] = if (s.v[2525] > ((-s.v[438]) * s.v[534])) { 1.0 } else { 0.0 };

        s.v[2660] = if (s.v[537] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2659] != 0.0)) && (s.v[2660] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(608)), A::mul(s.ad_value(2525), s.ad_value(608))), A::mul(s.ad_value(2525), s.ad_value(608))), A::mul(s.ad_value(2525), s.ad_value(608)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2659] != 0.0)) && (!(s.v[2660] != 0.0))) {
            s.store_ad(2526, &A::pow(A::abs(A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(537)));
        }

        s.v[2661] = if (s.v[569] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2661] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(566))));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2661] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(566))), s.ad_value(569)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) {
            s.store_scale_ad(1909, A::add(A::mul(s.ad_value(578), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2518)))), p.p30);
        }

        s.v[2662] = if (s.v[668] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2662] != 0.0)) {
            s.store_scalar(1910, 0.0);
        }

        s.v[2663] = if ((s.v[517] == 0.0) && (s.v[520] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_sub(2529, 564, 2523);
        }

        s.v[2665] = if (s.v[506] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) && (s.v[2665] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(s.ad_value(2529), s.ad_value(591)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) && (!(s.v[2665] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(s.ad_value(2529), s.ad_value(591)), s.ad_value(506)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_mul(2533, 585, 2526);
        }

        s.v[2666] = if (s.v[520] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul_ad_rhs(2536, 600, A::div(A::mul(s.ad_value(2533), s.ad_value(570)), s.ad_value(2529)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_div_ad_lhs(2537, A::scale(s.ad_value(597), 0.666666666666667), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::mul(s.ad_value(597), s.ad_value(2537)), s.ad_value(2540)), A::mul(s.ad_value(597), s.ad_value(2539))), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2669] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2669] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2669] != 0.0))) {
            let assign60450_ad_e78128: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign60450_ad_e78128);
        }

        s.v[2670] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        s.v[2671] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2670] != 0.0))) && (s.v[2671] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2670] != 0.0))) && (!(s.v[2671] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2672] = if (s.v[526] == 0.0) { 1.0 } else { 0.0 };

        s.v[2673] = if (s.v[506] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2673] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591)), s.ad_value(506)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) {
            s.store_mul_ad_rhs(2551, 573, A::div(A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(588)), s.ad_value(2526)));
        }

        s.v[2674] = if (((((-s.v[603]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2674] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(603)), s.ad_value(2551)));
        }

        s.v[2675] = if (((-s.v[603]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2674] != 0.0))) && (s.v[2675] != 0.0)) {
            let assign60640_ad_e78468: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(603)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(603)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(603)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign60640_ad_e78468);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2674] != 0.0))) && (!(s.v[2675] != 0.0))) {
            let assign60650_ad_e78519: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(603)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(603)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(603)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign60650_ad_e78519);
        }

        s.v[2676] = if (s.v[535] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2677] = if (s.v[2525] > ((-s.v[438]) * s.v[535])) { 1.0 } else { 0.0 };

        s.v[2678] = if (s.v[538] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2677] != 0.0)) && (s.v[2678] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(609)), A::mul(s.ad_value(2525), s.ad_value(609))), A::mul(s.ad_value(2525), s.ad_value(609))), A::mul(s.ad_value(2525), s.ad_value(609)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2677] != 0.0)) && (!(s.v[2678] != 0.0))) {
            s.store_ad(2526, &A::pow(A::abs(A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(538)));
        }

        s.v[2679] = if (s.v[570] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2679] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(567))));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2679] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(567))), s.ad_value(570)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_scale_ad(1910, A::add(A::mul(s.ad_value(579), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2518)))), p.p30);
        }

        s.v[2680] = if (s.v[669] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2680] != 0.0)) {
            s.store_scalar(1911, 0.0);
        }

        s.v[2681] = if ((s.v[518] == 0.0) && (s.v[521] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_sub(2529, 565, 2523);
        }

        s.v[2683] = if (s.v[507] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(s.ad_value(2529), s.ad_value(592)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) && (!(s.v[2683] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(s.ad_value(2529), s.ad_value(592)), s.ad_value(507)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_mul(2533, 586, 2526);
        }

        s.v[2684] = if (s.v[521] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul_ad_rhs(2536, 601, A::div(A::mul(s.ad_value(2533), s.ad_value(571)), s.ad_value(2529)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_div_ad_lhs(2537, A::scale(s.ad_value(598), 0.666666666666667), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::mul(s.ad_value(598), s.ad_value(2537)), s.ad_value(2540)), A::mul(s.ad_value(598), s.ad_value(2539))), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2687] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2687] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2687] != 0.0))) {
            let assign61200_ad_e79394: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign61200_ad_e79394);
        }

        s.v[2688] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        s.v[2689] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2688] != 0.0))) && (s.v[2689] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2688] != 0.0))) && (!(s.v[2689] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2690] = if (s.v[527] == 0.0) { 1.0 } else { 0.0 };

        s.v[2691] = if (s.v[507] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592)), s.ad_value(507)));
        }

    }
}
