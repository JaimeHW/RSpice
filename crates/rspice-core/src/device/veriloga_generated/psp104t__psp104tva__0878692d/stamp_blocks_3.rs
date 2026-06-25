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
        if (s.v[1016] != 0.0) {
            s.store_scalar(1004, 0.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(39, p.p812);
        }

        s.v[1126] = if (if self.param_given[813] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1126] != 0.0)) {
            s.store_scalar(39, p.p813);
        }

        s.v[1127] = if (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0)))) { 1.0 } else { 0.0 };

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (s.v[1] - 0.5);
            let assign9190_cond_e9118: f64 = if (((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) && (s.v[1004] < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1005, 1005, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1004), (s.v[7] + s.v[3])), (s.v[5] + (0.5 * s.v[3])))));
            }
            if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1006, 1006, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1004), (s.v[7] + s.v[3])), (s.v[6] + (0.5 * s.v[3])))));
            }
            if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_offset(1004, 1004, 1.0);
            }
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(989, 1005, 2);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(990, 1006, 2);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(991, (1.0 / (p.p808 + (0.5 * s.v[3]))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(992, (1.0 / (p.p809 + (0.5 * s.v[3]))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1002, &{
                if ((s.v[3] + s.v[310]) > 1e-9) {
                    A::offset(s.ad_value(310), s.v[3])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1003, &{
                if (((s.v[4] + s.v[311]) + p.p810) > 1e-9) {
                    A::offset(A::add(s.ad_value(4), s.ad_value(311)), p.p810)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1000, 1.0, A::powf(s.ad_value(1002), p.p818));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1001, 1.0, A::powf(s.ad_value(1003), p.p819));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scale_ad(993, A::add(A::add(A::offset(A::scale(s.ad_value(1000), p.p815), 1.0), A::scale(s.ad_value(1001), p.p816)), A::mul(A::scale(s.ad_value(1000), p.p817), s.ad_value(1001))), (1.0 + (p.p814 * (s.v[353] - 1.0))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(994, A::scale(A::add(s.ad_value(989), s.ad_value(990)), p.p811), 993);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(995, A::scale(A::add(s.ad_value(991), s.ad_value(992)), p.p811), 993);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1000, 1.0, A::powf(s.ad_value(1002), p.p824));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1001, 1.0, A::powf(s.ad_value(1003), p.p825));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add_ad(996, A::add(A::offset(A::scale(s.ad_value(1000), p.p821), 1.0), A::scale(s.ad_value(1001), p.p822)), A::mul(A::scale(s.ad_value(1000), p.p823), s.ad_value(1001)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_sub_ad_lhs(998, A::sub(A::add(s.ad_value(989), s.ad_value(990)), s.ad_value(991)), 992);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(999, A::offset(s.ad_value(994), 1.0), A::offset(s.ad_value(995), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(65, 65, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(82, A::mul(A::mul(s.ad_value(82), s.ad_value(999)), A::offset(A::scale(s.ad_value(995), p.p812), 1.0)), A::offset(A::scale(s.ad_value(994), p.p812), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(121, A::mul(A::mul(s.ad_value(121), s.ad_value(999)), A::offset(A::mul(s.ad_value(39), s.ad_value(995)), 1.0)), A::offset(A::mul(s.ad_value(39), s.ad_value(994)), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(150, 150, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(999, A::scale(s.ad_value(998), p.p820), 996);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(40, 40, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(145, 145, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(999, A::scale(s.ad_value(998), p.p826), A::powf(s.ad_value(996), p.p827));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(62, 62, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(155, 155, 999);
        }

        s.v[1128] = if ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1129] = if (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_offset(998, 4, s.v[8]);
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_scalar(999, (1.0 / p.p828));
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_from_scalar_ad(11, (p.p828 * p.p828), A::scale(s.ad_value(998), s.v[8]));
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(12, A::sub(A::scale(A::exp(A::scale(s.ad_value(999), ((-10.0) * s.v[8]))), ((0.1 * s.v[8]) + (0.01 * p.p828))), A::mul(A::offset(A::scale(s.ad_value(998), 0.1), (0.01 * p.p828)), A::exp(A::mul(A::scale(s.ad_value(998), (-10.0)), s.ad_value(999))))), 4);
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(13, A::sub(A::scale(A::exp(A::scale(s.ad_value(999), ((-20.0) * s.v[8]))), ((0.05 * s.v[8]) + (0.0025 * p.p828))), A::mul(A::offset(A::scale(s.ad_value(998), 0.05), (0.0025 * p.p828)), A::exp(A::mul(A::scale(s.ad_value(998), (-20.0)), s.ad_value(999))))), 4);
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad(998, A::add(s.ad_value(11), A::scale(s.ad_value(12), p.p829)), A::scale(s.ad_value(13), p.p830));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(40, 40, A::mul(s.ad_value(348), s.ad_value(998)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(65, 65, A::offset(A::mul(s.ad_value(349), s.ad_value(998)), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(145, 145, A::mul(s.ad_value(348), s.ad_value(998)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(150, 150, A::offset(A::mul(s.ad_value(349), s.ad_value(998)), 1.0));
        }

        s.copy_ad(175, 40);

        s.copy_ad(176, 41);

        s.copy_ad(177, 42);

        s.copy_ad(179, 43);

        s.copy_ad(180, 44);

        if (s.v[45] > 1e20) {
            s.store_ad(181, &{
                if (s.v[45] < 1e26) {
                    s.ad_value(45)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(181, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(182, 46);
        } else {
            s.store_scalar(182, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(183, 47);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(184, 48);

        s.copy_ad(185, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(186, 50);
        } else {
            s.store_scalar(186, 0.0);
        }

        s.copy_ad(190, 55);

        s.copy_ad(191, 56);

        if (s.v[57] > 1e23) {
            s.store_ad(192, &{
                if (s.v[57] < 1e27) {
                    s.ad_value(57)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(192, 1e23);
        }

        if (s.v[58] > 1e23) {
            s.store_ad(193, &{
                if (s.v[58] < 1e27) {
                    s.ad_value(58)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(193, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(187, 51);
        } else {
            s.store_scalar(187, 0.0);
        }

        if (s.v[53] > 0.0) {
            s.store_ad(189, &{
                if (s.v[53] < 0.5) {
                    s.ad_value(53)
                } else {
                    A::constant(0.5)
                }
            });
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[52] > 0.0) {
            s.store_ad(188, &{
                if (s.v[52] < 1.0) {
                    s.ad_value(52)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(178, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(194, 62);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.store_ad(196, &{
                if (s.v[64] < 1.0) {
                    s.ad_value(64)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(195, 63);
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(197, 59);
        } else {
            s.store_scalar(197, 0.0);
        }

        if (s.v[61] > 0.0) {
            s.store_ad(198, &{
                if (s.v[61] < 1.0) {
                    s.ad_value(61)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(199, 60);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(200, 65);
        } else {
            s.store_scalar(200, 0.0);
        }

        s.copy_ad(201, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(202, 67);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(204, 69);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(206, 71);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(208, 73);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(210, 75);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 76);

        s.copy_ad(212, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(213, 78);
        } else {
            s.store_scalar(213, 0.0);
        }

        s.copy_ad(214, 79);

        if (s.v[80] > (-0.5)) {
            s.store_ad(215, &{
                if (s.v[80] < 1.0) {
                    s.ad_value(80)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(215, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(216, 81);
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(217, 82);
        } else {
            s.store_scalar(217, 0.0);
        }

        s.copy_ad(218, 83);

        if (s.v[84] > (-0.5)) {
            s.store_ad(219, &{
                if (s.v[84] < 1.0) {
                    s.ad_value(84)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(219, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(220, 85);
        } else {
            s.store_scalar(220, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(221, 86);
        } else {
            s.store_scalar(221, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(222, 87);
        } else {
            s.store_scalar(222, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(223, 88);
        } else {
            s.store_scalar(223, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(224, 89);
        } else {
            s.store_scalar(224, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(225, 90);
        } else {
            s.store_scalar(225, 0.0);
        }

        s.copy_ad(226, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(227, 92);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 93);

        s.copy_ad(229, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(230, 95);
        } else {
            s.store_scalar(230, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(231, 96);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(232, 97);
        } else {
            s.store_scalar(232, 1e-12);
        }

        s.copy_ad(233, 98);

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
        if (s.v[99] > 0.0) {
            s.copy_ad(234, 99);
        } else {
            s.store_scalar(234, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(235, 100);
        } else {
            s.store_scalar(235, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(236, 101);
        } else {
            s.store_scalar(236, 0.0);
        }

        s.copy_ad(237, 102);

        s.copy_ad(238, 103);

        s.copy_ad(239, 104);

        s.copy_ad(240, 105);

        s.copy_ad(241, 106);

        s.copy_ad(242, 107);

        s.copy_ad(243, 108);

        s.copy_ad(244, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(245, 110);
        } else {
            s.store_scalar(245, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(246, 111);
        } else {
            s.store_scalar(246, 0.0);
        }

        s.copy_ad(247, 112);

        s.copy_ad(248, 113);

        s.copy_ad(249, 114);

        s.copy_ad(250, 115);

        s.copy_ad(251, 116);

        s.copy_ad(252, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(253, 118);
        } else {
            s.store_scalar(253, 0.0);
        }

        s.copy_ad(254, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(255, 120);
        } else {
            s.store_scalar(255, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(256, 121);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(257, 122);
        } else {
            s.store_scalar(257, 2.0);
        }

        s.copy_ad(258, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(259, 124);
        } else {
            s.store_scalar(259, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(260, 125);
        } else {
            s.store_scalar(260, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(261, 126);
        } else {
            s.store_scalar(261, 0.0);
        }

        s.copy_ad(262, 127);

        s.copy_ad(263, 128);

        s.copy_ad(264, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(265, 130);
        } else {
            s.store_scalar(265, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(266, 131);
        } else {
            s.store_scalar(266, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(267, 132);
        } else {
            s.store_scalar(267, 0.0);
        }

        s.copy_ad(268, 133);

        s.copy_ad(269, 134);

        s.copy_ad(270, 135);

        s.copy_ad(271, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(272, 137);
        } else {
            s.store_scalar(272, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(273, 138);
        } else {
            s.store_scalar(273, 0.0);
        }

        s.copy_ad(274, 139);

        s.copy_ad(280, 145);

        s.copy_ad(281, 146);

        s.copy_ad(282, 147);

        if (s.v[148] > 1e20) {
            s.store_ad(283, &{
                if (s.v[148] < 1e26) {
                    s.ad_value(148)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(283, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(284, 149);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(285, 150);
        } else {
            s.store_scalar(285, 0.0);
        }

        s.copy_ad(286, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(287, 152);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[153] > 0.0) {
            s.store_ad(288, &{
                if (s.v[153] < 1.0) {
                    s.ad_value(153)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(288, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(289, 154);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(290, 155);
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[157] > 0.0) {
            s.store_ad(292, &{
                if (s.v[157] < 1.0) {
                    s.ad_value(157)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(291, 156);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[173] > 0.0) {
            s.copy_ad(306, 173);
        } else {
            s.store_scalar(306, 0.0);
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
            s.copy_ad(191, 190);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(193, 192);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(246, 245);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(248, 247);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(250, 249);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(252, 251);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(236, 235);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(242, 240);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(243, 241);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(261, 260);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(263, 262);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(267, 266);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(273, 272);
        }

        s.store_scale(757, 180, 8.8541878176e-12);

        s.store_div(758, 757, 179);

        s.store_square(759, 179);

        s.store_scale(760, 758, 6.241449993689894e18);

        s.store_mul(761, 255, 181);

        if (s.v[761] > 1e20) {
            s.store_ad(761, &{
                if (s.v[761] < 1e26) {
                    s.ad_value(761)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(761, 1e20);
        }

        s.v[762] = 0.0;

        s.v[1131] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1131] != 0.0) {
            s.store_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.v[1132] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_scale(762, 762, (7.448711 / 5.951993));
        }

        s.store_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));

        s.store_scale(764, 212, 0.5);

        s.v[765] = 0.5;

        s.v[1133] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[1133] != 0.0) {
            s.store_scale(764, 212, 0.3333333333333333);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(765, 0.3333333333333333);
        }

        s.store_offset_ad(997, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0)), (-1.0));

        s.store_ad(766, &A::div(A::mul(A::offset(s.ad_value(997), (-1.0)), A::offset(s.ad_value(997), (-1.0))), {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_offset_ad(997, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(257)), 1.0)), (-1.0));

        s.store_ad(767, &A::div(A::mul(A::offset(s.ad_value(997), (-1.0)), A::offset(s.ad_value(997), (-1.0))), {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_div_from_scalar(768, 1.0, 226);

        s.store_div(769, 757, 190);

        s.store_div(770, 757, 191);

        s.store_div_ad_lhs(771, A::sqrt(A::scale(s.ad_value(192), ((2.0 * 1.6021918e-19) * (s.v[756] * s.v[356])))), 769);

        s.store_div_ad_lhs(772, A::sqrt(A::scale(s.ad_value(193), ((2.0 * 1.6021918e-19) * (s.v[756] * s.v[356])))), 770);

        s.store_square(773, 771);

        s.store_square(774, 772);

        s.store_offset_ad(775, A::div(A::ln(A::offset(A::exp(A::scale(s.ad_value(264), (0.005 * s.v[356]))), (-1.0))), s.ad_value(264)), (-((((((0.005 * s.v[356])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(776, A::ln(A::scale(s.ad_value(771), 0.5)), 775);

        s.store_add_ad_lhs(777, A::ln(A::scale(s.ad_value(772), 0.5)), 775);

        s.store_div_from_scalar(809, 1.0, 771);

        s.store_offset_scaled(810, 771, 3.1, 8.5);

        s.store_square(778, 810);

        s.store_scale(811, 810, 0.5);

        s.v[1134] = if (s.v[809] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1134] != 0.0) {
            s.store_scale(779, 809, 64.0);
        }

        s.v[1135] = if (s.v[809] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1134] != 0.0)) && (s.v[1135] != 0.0)) {
            s.store_offset_scaled(779, 809, 22.0, 3.0);
        }

        s.v[1136] = if (s.v[809] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (s.v[1136] != 0.0)) {
            s.store_offset_scaled(779, 809, (-7.2), 15.5);
        }

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (!(s.v[1136] != 0.0))) {
            s.copy_ad(779, 771);
        }

        s.store_sub_ad(780, A::add(s.ad_value(811), A::scale(s.ad_value(773), 0.5)), A::mul(s.ad_value(771), A::sqrt(A::add(A::add(s.ad_value(811), A::scale(s.ad_value(773), 0.25)), s.ad_value(779)))));

        s.store_div_from_scalar(809, 1.0, 772);

        s.store_offset_scaled(810, 772, 3.1, 8.5);

        s.store_square(781, 810);

        s.store_scale(811, 810, 0.5);

        s.v[1137] = if (s.v[809] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1137] != 0.0) {
            s.store_scale(782, 809, 64.0);
        }

        s.v[1138] = if (s.v[809] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1137] != 0.0)) && (s.v[1138] != 0.0)) {
            s.store_offset_scaled(782, 809, 22.0, 3.0);
        }

        s.v[1139] = if (s.v[809] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (s.v[1139] != 0.0)) {
            s.store_offset_scaled(782, 809, (-7.2), 15.5);
        }

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (!(s.v[1139] != 0.0))) {
            s.copy_ad(782, 772);
        }

        s.store_sub_ad(783, A::add(s.ad_value(811), A::scale(s.ad_value(774), 0.5)), A::mul(s.ad_value(772), A::sqrt(A::add(A::add(s.ad_value(811), A::scale(s.ad_value(774), 0.25)), s.ad_value(782)))));

        s.store_div_from_scalar(784, 1.0, 244);

        s.store_scale_ad(785, A::sqrt(A::scale(s.ad_value(244), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(786, 785, 179);

        s.store_mul(787, 785, 190);

        s.store_mul(788, 785, 191);

        s.v[789] = 0.0;

        s.v[1140] = if (s.v[239] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1140] != 0.0) {
            s.store_div_ad_lhs(789, A::scale(s.ad_value(238), (-0.495)), 239);
        }

        s.v[790] = 0.0;

        s.v[1141] = if (s.v[241] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1141] != 0.0) {
            s.store_div_ad_lhs(790, A::scale(s.ad_value(240), (-0.495)), 241);
        }

        s.v[1142] = if (s.v[243] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.store_div_ad_lhs(791, A::scale(s.ad_value(242), (-0.495)), 243);
        }

        s.store_ad(792, &A::pow_from_scalar(s.v[353], s.ad_value(237)));

        s.store_mul(234, 234, 792);

        s.store_mul(235, 235, 792);

        s.store_mul(236, 236, 792);

        if ((1.0 + (s.v[249] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 249, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(711, 247, 785);

        s.store_scaled_mul(795, 711, 190, 500000000.0);

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
        if ((1.0 + (s.v[250] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 250, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(712, 248, 785);

        s.store_scaled_mul(796, 712, 191, 500000000.0);

        s.v[797] = 0.0;

        s.v[1143] = if (s.v[270] > 1e-10) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_div_from_scalar(797, 0.75, 270);
        }

        s.store_square(798, 271);

        s.store_scale(20, 2, s.v[647]);

        s.store_scale(21, 2, s.v[648]);

        s.store_scale(22, 2, s.v[649]);

        s.store_scale(23, 2, s.v[674]);

        s.store_scale(24, 2, s.v[675]);

        s.store_scale(25, 2, s.v[676]);

        s.v[26] = 0.0;

        s.v[1151] = if (p.p43 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1151] != 0.0) {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 313);

        s.v[1152] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.v[1153] = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_scale(20, 2, s.v[650]);
        }

        if (s.v[1153] != 0.0) {
            s.store_sub_ad(21, A::scale(s.ad_value(2), s.v[651]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1153] != 0.0) {
            s.copy_ad(22, 27);
        }

        if (s.v[1153] != 0.0) {
            s.store_scale(23, 2, s.v[677]);
        }

        if (s.v[1153] != 0.0) {
            s.store_sub_ad(24, A::scale(s.ad_value(2), s.v[678]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1153] != 0.0) {
            s.copy_ad(25, 27);
        }

        s.v[1154] = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1154] != 0.0) {
            s.store_ad(647, &{
                if (s.v[20] > 0.0) {
                    s.ad_value(20)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(648, &{
                if (s.v[21] > 0.0) {
                    s.ad_value(21)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(649, &{
                if (s.v[22] > 0.0) {
                    s.ad_value(22)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(674, &{
                if (s.v[23] > 0.0) {
                    s.ad_value(23)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(675, &{
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(676, &{
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(649, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(674, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(675, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(676, 0.0);
        }

        s.v[657] = 0.0;

        s.v[684] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[660] = 0.0;

        s.v[687] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[652] = 1.0;

        s.v[679] = 1.0;

        s.v[653] = 1.0;

        s.v[680] = 1.0;

        s.v[654] = 1.0;

        s.v[681] = 1.0;

        s.v[502] = 0.0;

        s.v[1155] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[1156] = if ((s.v[388] * s.v[647]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1156] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(647), s.v[388])), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1156] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1157] = if ((s.v[389] * s.v[648]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1157] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(648), s.v[389])), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1157] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        s.v[1158] = if ((s.v[390] * s.v[649]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1158] != 0.0)) {
            s.store_scale_ad(457, A::ln(A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(649), s.v[390])), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1158] != 0.0))) {
            s.store_scalar(457, 100000000.0);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(655, &A::min(A::min(s.ad_value(455), s.ad_value(456)), s.ad_value(457)));
        }

        s.v[1159] = if ((((s.v[655] * s.v[372])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1159] != 0.0)) {
            s.store_exp_ad(656, A::scale(s.ad_value(655), s.v[372]));
        }

        s.v[1160] = if ((s.v[655] * s.v[372]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (!(s.v[1159] != 0.0))) && (s.v[1160] != 0.0)) {
            s.store_div_from_scalar_ad(656, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(655), s.v[372])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(655), s.v[372])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(655), s.v[372])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1155] != 0.0) && (!(s.v[1159] != 0.0))) && (!(s.v[1160] != 0.0))) {
            s.store_scale_ad(656, A::offset(A::mul(A::offset(A::scale(s.ad_value(655), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(655), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(655), s.v[372]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(397, s.v[394]);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(398, s.v[395]);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(399, s.v[396]);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(400, p.p848);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(401, p.p849);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(402, p.p850);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(403, p.p845);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(404, p.p846);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(405, p.p847);
        }

        s.v[1161] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scalar(397, (s.v[395] + s.v[396]));
        }

        if ((s.v[1155] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scalar(400, (0.9 * (p.p849).min(p.p850)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scalar(403, (p.p846 + p.p847));
        }

        s.v[1162] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scalar(398, (s.v[394] + s.v[396]));
        }

        if ((s.v[1155] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scalar(401, (0.9 * (p.p848).min(p.p850)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scalar(404, (p.p845 + p.p847));
        }

        s.v[1163] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scalar(399, (s.v[394] + s.v[395]));
        }

        if ((s.v[1155] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scalar(402, (0.9 * (p.p848).min(p.p849)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scalar(405, (p.p845 + p.p846));
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(657, &A::min(A::min(s.ad_value(397), s.ad_value(398)), s.ad_value(399)));
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(658, 657, 0.1);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(378, &A::max(A::max(s.ad_value(400), s.ad_value(401)), s.ad_value(402)));
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_rhs(659, 657, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378)))));
        }

        if (s.v[1155] != 0.0) {
            s.store_offset_ad(660, A::min(A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405)), (-0.05));
        }

        s.v[1164] = if ((s.v[564] * s.v[674]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1164] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(564), s.ad_value(674))), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1164] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1165] = if ((s.v[565] * s.v[675]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(565), s.ad_value(675))), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1165] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        s.v[1166] = if ((s.v[566] * s.v[676]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scale_ad(457, A::ln(A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(566), s.ad_value(676))), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1166] != 0.0))) {
            s.store_scalar(457, 100000000.0);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(682, &A::min(A::min(s.ad_value(455), s.ad_value(456)), s.ad_value(457)));
        }

        s.v[1167] = if ((((s.v[682] * s.v[372])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_exp_ad(683, A::scale(s.ad_value(682), s.v[372]));
        }

        s.v[1168] = if ((s.v[682] * s.v[372]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (!(s.v[1167] != 0.0))) && (s.v[1168] != 0.0)) {
            s.store_div_from_scalar_ad(683, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(682), s.v[372])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(682), s.v[372])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(682), s.v[372])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1155] != 0.0) && (!(s.v[1167] != 0.0))) && (!(s.v[1168] != 0.0))) {
            s.store_scale_ad(683, A::offset(A::mul(A::offset(A::scale(s.ad_value(682), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(682), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(682), s.v[372]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(397, 570);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(398, 571);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(399, 572);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(400, 512);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(401, 513);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(402, 514);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(403, 509);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(404, 510);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(405, 511);
        }

        s.v[1169] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_add(397, 571, 572);
        }

        if ((s.v[1155] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_scale_ad(400, A::min(s.ad_value(513), s.ad_value(514)), 0.9);
        }

        if ((s.v[1155] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_add(403, 510, 511);
        }

        s.v[1170] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_add(398, 570, 572);
        }

        if ((s.v[1155] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_scale_ad(401, A::min(s.ad_value(512), s.ad_value(514)), 0.9);
        }

        if ((s.v[1155] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_add(404, 509, 511);
        }

        s.v[1171] = if (s.v[676] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_add(399, 570, 571);
        }

        if ((s.v[1155] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_scale_ad(402, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
        }

        if ((s.v[1155] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_add(405, 509, 510);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(684, &A::min(A::min(s.ad_value(397), s.ad_value(398)), s.ad_value(399)));
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(685, 684, 0.1);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(378, &A::max(A::max(s.ad_value(400), s.ad_value(401)), s.ad_value(402)));
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_rhs(686, 684, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378)))));
        }

        if (s.v[1155] != 0.0) {
            s.store_offset_ad(687, A::min(A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405)), (-0.05));
        }

        s.v[1172] = if (s.v[475] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(502, A::add(A::add(A::scale(s.ad_value(647), s.v[415]), A::scale(s.ad_value(648), s.v[416])), A::scale(s.ad_value(649), s.v[417])), p.p946);
        }

        s.v[1507] = if ((s.v[647] * s.v[415]) <= s.v[502]) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1507] != 0.0)) {
            s.store_scalar(652, 0.0);
        }

        s.v[1508] = if ((s.v[648] * s.v[416]) <= s.v[502]) { 1.0 } else { 0.0 };

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
        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1508] != 0.0)) {
            s.store_scalar(653, 0.0);
        }

        s.v[1509] = if ((s.v[649] * s.v[417]) <= s.v[502]) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1509] != 0.0)) {
            s.store_scalar(654, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_mul_ad_rhs(502, 554, A::add(A::add(A::mul(s.ad_value(674), s.ad_value(582)), A::mul(s.ad_value(675), s.ad_value(583))), A::mul(s.ad_value(676), s.ad_value(584))));
        }

        s.v[1797] = if ((s.v[674] * s.v[582]) <= s.v[502]) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1797] != 0.0)) {
            s.store_scalar(679, 0.0);
        }

        s.v[1798] = if ((s.v[675] * s.v[583]) <= s.v[502]) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1798] != 0.0)) {
            s.store_scalar(680, 0.0);
        }

        s.v[1799] = if ((s.v[676] * s.v[584]) <= s.v[502]) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1799] != 0.0)) {
            s.store_scalar(681, 0.0);
        }

        s.v[1929] = 0.0;

        s.v[1930] = 0.0;

        s.v[1931] = 0.0;

        s.store_ad(357, &A::offset(A::voltage(ctx, &nodes, Some(4), None), s.v[352]));

        s.store_square(358, 357);

        s.store_offset(359, 357, (-s.v[351]));

        s.store_div_from_scalar(360, s.v[351], 357);

        s.store_ln(361, 360);

        s.store_scale(1916, 357, (1.3806505e-23 * 6.241449993689894e18));

        s.store_div_from_scalar(362, 1.0, 1916);

        s.store_sub_ad(363, A::sub_from_scalar(1.179, A::scale(s.ad_value(357), 9.025e-5)), A::scale(s.ad_value(358), 3.05e-7));

        s.store_scale_ad(364, A::mul(A::mul(A::offset(A::scale(s.ad_value(357), 0.00045), 1.045), A::sub(A::offset(A::scale(s.ad_value(357), 0.0014), 0.523), A::scale(s.ad_value(358), 1.48e-6))), s.ad_value(358)), 1.1111111111111112e-5);

        if !(s.v[364] > 0.001) {
            s.store_scalar(364, 0.001);
        }

        s.store_add_ad(717, A::add(s.ad_value(363), s.ad_value(185)), A::mul(A::scale(s.ad_value(1916), 2.0), A::ln(A::scale(A::mul(s.ad_value(181), A::powf(s.ad_value(364), (-0.75))), 4e-26))));

        if !(s.v[717] > 0.05) {
            s.store_scalar(717, 0.05);
        }

        s.store_div_ad_lhs(718, A::sqrt(A::mul(A::scale(s.ad_value(181), ((2.0 * 1.6021918e-19) * s.v[756])), s.ad_value(362))), 758);

        s.v[719] = 0.0;

        s.v[720] = 0.0;

        s.v[2004] = if (s.v[186] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2004] != 0.0) {
            s.store_div_from_scalar(721, 80000000.0, 759);
        }

        if (s.v[2004] != 0.0) {
            s.store_ad(720, &{
                if (s.v[186] > s.v[721]) {
                    s.ad_value(186)
                } else {
                    s.ad_value(721)
                }
            });
        }

        if (s.v[2004] != 0.0) {
            s.store_ad(720, &{
                if (5e24 > s.v[720]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(720)
                }
            });
        }

        if (s.v[2004] != 0.0) {
            s.store_div_ad(719, A::mul(A::mul(A::scale(s.ad_value(758), 2.0), s.ad_value(758)), s.ad_value(1916)), A::scale(s.ad_value(720), (1.6021918e-19 * s.v[756])));
        }

        s.store_mul_ad_lhs(722, A::scale(s.ad_value(1916), 100.0), 1916);

        s.v[2005] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2005] != 0.0) {
            s.store_sqrt_ad(723, A::mul(A::mul(A::mul(s.ad_value(1916), s.ad_value(718)), s.ad_value(718)), s.ad_value(717)));
        }

        if (s.v[2005] != 0.0) {
            s.store_mul_ad(724, A::scale(s.ad_value(762), 0.75), A::powf(s.ad_value(723), 0.6666666666666666));
        }

        if (s.v[2005] != 0.0) {
            s.store_add(717, 717, 724);
        }

        if (s.v[2005] != 0.0) {
            s.store_mul_ad_rhs(718, 718, A::offset(A::div(A::scale(s.ad_value(724), (2.0 * 0.6666666666666666)), s.ad_value(723)), 1.0));
        }

        s.store_sqrt(725, 717);

        s.store_scale(726, 717, 0.95);

        s.store_mul_ad_lhs(727, A::scale(s.ad_value(717), 0.0025), 717);

        s.copy_ad(728, 727);

        s.store_scaled_sqrt(729, 728, 0.5);

        s.store_scale_ad(730, A::sub(A::sub(s.ad_value(726), s.ad_value(729)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(726), s.ad_value(729)), A::sub(s.ad_value(726), s.ad_value(729))), s.ad_value(727)))), 0.5);

        s.store_scaled_add(731, 717, 363, 0.5);

        s.store_sub_ad_lhs(732, A::sqrt(A::add(s.ad_value(183), s.ad_value(717))), 725);

        s.store_sub_ad_lhs(733, A::sub(A::sqrt(A::add(A::add(s.ad_value(183), s.ad_value(184)), s.ad_value(717))), s.ad_value(725)), 732);

        s.store_add_ad(734, A::add(A::add(s.ad_value(363), s.ad_value(185)), s.ad_value(254)), A::mul(A::scale(s.ad_value(1916), 2.0), A::ln(A::scale(A::mul(s.ad_value(761), A::powf(s.ad_value(364), (-0.75))), 4e-26))));

        if !(s.v[734] > 0.05) {
            s.store_scalar(734, 0.05);
        }

        s.store_div_ad_lhs(735, A::sqrt(A::mul(A::scale(s.ad_value(761), ((2.0 * 1.6021918e-19) * s.v[756])), s.ad_value(362))), 758);

        s.v[2006] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2006] != 0.0) {
            s.store_sqrt_ad(723, A::mul(A::mul(A::mul(s.ad_value(1916), s.ad_value(735)), s.ad_value(735)), s.ad_value(734)));
        }

        if (s.v[2006] != 0.0) {
            s.store_mul_ad(724, A::scale(s.ad_value(762), 0.75), A::powf(s.ad_value(723), 0.6666666666666666));
        }

        if (s.v[2006] != 0.0) {
            s.store_add(734, 734, 724);
        }

        if (s.v[2006] != 0.0) {
            s.store_mul_ad_rhs(735, 735, A::offset(A::div(A::scale(s.ad_value(724), (2.0 * 0.6666666666666666)), s.ad_value(723)), 1.0));
        }

        s.store_scale(736, 734, 0.95);

        s.store_mul_ad_lhs(737, A::scale(s.ad_value(734), 0.0025), 734);

        s.copy_ad(738, 737);

        s.store_scaled_sqrt(729, 738, 0.5);

        s.store_scale_ad(739, A::sub(A::sub(s.ad_value(736), s.ad_value(729)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(736), s.ad_value(729)), A::sub(s.ad_value(736), s.ad_value(729))), s.ad_value(737)))), 0.5);

        s.store_offset_ad(701, A::add(s.ad_value(175), A::mul(A::mul(s.ad_value(176), s.ad_value(359)), A::offset(A::mul(s.ad_value(177), s.ad_value(359)), 1.0))), s.v[17]);

        s.store_exp_ad(740, A::mul(s.ad_value(178), s.ad_value(361)));

        s.store_mul(702, 187, 740);

        s.store_div(703, 188, 360);

        s.store_exp_ad(741, A::mul(s.ad_value(201), s.ad_value(361)));

        s.store_mul(704, 200, 741);

        s.store_mul_ad_lhs(1917, A::scale(s.ad_value(704), s.v[16]), 758);

        s.store_mul_ad_rhs(706, 204, A::exp(A::mul(s.ad_value(205), s.ad_value(361))));

        s.store_exp_ad(742, A::mul(s.ad_value(203), s.ad_value(361)));

        s.store_mul(705, 202, 742);

        s.store_mul_ad_rhs(708, 208, A::exp(A::mul(s.ad_value(209), s.ad_value(361))));

        s.store_exp_ad(743, A::mul(s.ad_value(207), s.ad_value(361)));

        s.store_mul(707, 206, 743);

        s.store_exp_ad(744, A::mul(s.ad_value(211), s.ad_value(361)));

        s.store_mul(709, 210, 744);

        s.store_exp_ad(745, A::mul(s.ad_value(214), s.ad_value(361)));

        s.store_mul(710, 213, 745);

        s.store_mul_ad_lhs(746, A::scale(s.ad_value(1917), 2.0), 710);

        s.store_exp_ad(747, A::mul(s.ad_value(218), s.ad_value(361)));

        s.store_mul(1921, 217, 747);

        s.store_mul(1922, 256, 747);

        s.store_mul_ad_rhs(713, 228, A::exp(A::mul(A::neg(s.ad_value(229)), s.ad_value(361))));

        s.store_mul_ad_lhs(1920, A::scale(s.ad_value(274), (4.0 * 1.3806505e-23)), 357);

        s.v[2007] = if ((p.p46 != 0.0) && (s.v[285] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2007] != 0.0) {
            s.store_offset_ad(714, A::add(s.ad_value(280), A::mul(s.ad_value(281), s.ad_value(359))), s.v[19]);
        }

        if (s.v[2007] != 0.0) {
            s.store_exp_ad(748, A::mul(s.ad_value(286), s.ad_value(361)));
        }

        if (s.v[2007] != 0.0) {
            s.store_mul(715, 285, 748);
        }

        if (s.v[2007] != 0.0) {
            s.store_mul_ad_lhs(1918, A::scale(s.ad_value(715), s.v[18]), 758);
        }

        if (s.v[2007] != 0.0) {
            s.store_mul_ad_rhs(1924, 1916, A::offset(A::mul(s.ad_value(284), s.ad_value(360)), 1.0));
        }

        if (s.v[2007] != 0.0) {
            s.store_add_ad(749, A::add(s.ad_value(363), s.ad_value(282)), A::mul(A::scale(s.ad_value(1924), 2.0), A::ln(A::scale(A::mul(s.ad_value(283), A::powf(s.ad_value(364), (-0.75))), 4e-26))));
        }

        if (s.v[2007] != 0.0) {
            s.store_ad(749, &{
                if (s.v[749] > 0.05) {
                    s.ad_value(749)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[2007] != 0.0) {
            s.store_div_ad_lhs(750, A::sqrt(A::mul(A::scale(s.ad_value(283), ((2.0 * 1.6021918e-19) * s.v[756])), s.ad_value(362))), 758);
        }

        if (s.v[2007] != 0.0) {
            s.store_square(1925, 750);
        }

        if (s.v[2007] != 0.0) {
            s.store_ln(1926, 1925);
        }

        if (s.v[2007] != 0.0) {
            s.store_scale(751, 749, 0.95);
        }

        if (s.v[2007] != 0.0) {
            s.store_mul_ad_lhs(752, A::scale(s.ad_value(749), 0.0025), 749);
        }

        if (s.v[2007] != 0.0) {
            s.copy_ad(753, 752);
        }

        if (s.v[2007] != 0.0) {
            s.store_scaled_sqrt(754, 753, 0.5);
        }

        if (s.v[2007] != 0.0) {
            s.store_scale_ad(755, A::sub(A::sub(s.ad_value(751), s.ad_value(754)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(751), s.ad_value(754)), A::sub(s.ad_value(751), s.ad_value(754))), s.ad_value(752)))), 0.5);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(714, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(748, 1.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(715, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(1918, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.copy_ad(1924, 1916);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(749, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(750, 1.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(1925, 1.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(1926, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(751, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(752, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(753, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(754, 0.0);
        }

        if (!(s.v[2007] != 0.0)) {
            s.store_scalar(755, 0.0);
        }

        s.v[2008] = if (s.v[0] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[2008] != 0.0) {
            s.store_ad(814, &A::voltage(ctx, &nodes, Some(6), Some(7)));
        }

        if (s.v[2008] != 0.0) {
            s.store_ad(815, &A::voltage(ctx, &nodes, Some(8), Some(7)));
        }

        if (s.v[2008] != 0.0) {
            s.store_ad(816, &A::voltage(ctx, &nodes, Some(7), Some(9)));
        }

        if (s.v[2008] != 0.0) {
            s.store_ad(821, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(11))));
        }

        if (s.v[2008] != 0.0) {
            s.store_ad(822, &A::neg(A::voltage(ctx, &nodes, Some(8), Some(12))));
        }

        if (!(s.v[2008] != 0.0)) {
            s.store_ad(814, &A::neg(A::voltage(ctx, &nodes, Some(6), Some(7))));
        }

        if (!(s.v[2008] != 0.0)) {
            s.store_ad(815, &A::neg(A::voltage(ctx, &nodes, Some(8), Some(7))));
        }

        if (!(s.v[2008] != 0.0)) {
            s.store_ad(816, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(9))));
        }

        if (!(s.v[2008] != 0.0)) {
            s.store_ad(821, &A::voltage(ctx, &nodes, Some(7), Some(11)));
        }

        if (!(s.v[2008] != 0.0)) {
            s.store_ad(822, &A::voltage(ctx, &nodes, Some(8), Some(12)));
        }

        s.store_add(818, 814, 816);

        s.copy_ad(823, 814);

        s.copy_ad(824, 816);

        s.store_add(825, 815, 816);

        s.store_sub(826, 814, 815);

        s.store_scale_ad(1801, A::neg(s.ad_value(823)), s.v[356]);

        s.store_scale_ad(1802, A::neg(s.ad_value(826)), s.v[356]);

        s.store_scale_ad(1803, A::neg(A::sub(s.ad_value(818), s.ad_value(701))), s.v[356]);

        s.v[820] = 1.0;

        s.v[2009] = if (s.v[815] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[2009] != 0.0) {
            s.store_scalar(820, (-1.0));
        }

        if (s.v[2009] != 0.0) {
            s.store_sub(814, 814, 815);
        }

        if (s.v[2009] != 0.0) {
            s.store_add(816, 816, 815);
        }

        if (s.v[2009] != 0.0) {
            s.store_neg(815, 815);
        }

        s.store_add(817, 815, 816);

        s.store_div_ad(819, A::square(s.ad_value(815)), A::offset(A::sqrt(A::offset(A::square(s.ad_value(815)), 0.01)), 0.1));

        s.store_add_ad_lhs(2013, A::scale(A::sub(A::add(s.ad_value(817), s.ad_value(816)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(728)))), 0.5), 726);

        s.copy_ad(1804, 2013);

        s.store_add_ad_lhs(1932, A::sub(s.ad_value(816), A::scale(A::sub(s.ad_value(2013), A::sqrt(A::add(A::mul(s.ad_value(2013), s.ad_value(2013)), s.ad_value(727)))), 0.5)), 730);

        s.copy_ad(1805, 1932);

        s.v[1933] = 0.0;

        s.v[2169] = if ((p.p45 != 0.0) && (s.v[182] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[2169] != 0.0) {
            s.store_add_ad_rhs(1934, 1932, A::scale(A::sub(s.ad_value(815), s.ad_value(819)), 0.5));
        }

        if (s.v[2169] != 0.0) {
            s.store_sub_ad_lhs(1935, A::sqrt(A::add(s.ad_value(1934), s.ad_value(717))), 725);
        }

        if (s.v[2169] != 0.0) {
            s.store_offset_ad(1929, A::div(A::scale(A::sub(s.ad_value(1935), s.ad_value(732)), 2.0), s.ad_value(733)), (-1.0));
        }

        if (s.v[2169] != 0.0) {
            s.store_sub_ad_rhs(1936, 1935, A::mul(A::mul(A::scale(A::sub_from_scalar(1.0, s.ad_value(182)), 0.25), s.ad_value(733)), A::add(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 0.4804530139182)))));
        }

        if (s.v[2169] != 0.0) {
            s.store_add_ad(1937, A::square(s.ad_value(1936)), A::mul(A::scale(s.ad_value(725), 2.0), s.ad_value(1936)));
        }

        if (s.v[2169] != 0.0) {
            s.store_sub_ad_rhs(1932, 1937, A::scale(A::sub(s.ad_value(815), s.ad_value(819)), 0.5));
        }

        if (s.v[2169] != 0.0) {
            s.store_sub(1933, 1805, 1932);
        }

        s.copy_ad(2010, 717);

        s.copy_ad(2011, 727);

        s.copy_ad(2012, 718);

        s.copy_ad(2014, 1932);

        s.copy_ad(2018, 1933);

        s.copy_ad(2015, 1921);

        s.copy_ad(2016, 766);

        s.store_sub_ad_lhs(2017, A::sub(s.ad_value(818), s.ad_value(2018)), 701);

        s.store_add_ad_rhs(2019, 2014, A::scale(A::sub(s.ad_value(815), s.ad_value(819)), 0.5));

        s.v[2031] = 1.0;

        s.v[2170] = if (s.v[188] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2170] != 0.0) {
            s.store_mul(2022, 2010, 362);
        }

        if (s.v[2170] != 0.0) {
            s.store_mul(2023, 2019, 362);
        }

        if (s.v[2170] != 0.0) {
            s.store_mul(2024, 2017, 362);
        }

        if (s.v[2170] != 0.0) {
            s.store_offset_ad(1930, A::div(A::scale(s.ad_value(2012), 0.5), A::sqrt(s.ad_value(2022))), 1.0);
        }

        if (s.v[2170] != 0.0) {
            s.store_add_ad_rhs(1931, 2022, A::mul(s.ad_value(2012), A::sqrt(s.ad_value(2022))));
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
        if (s.v[2170] != 0.0) {
            s.store_sub_ad(2025, A::add(A::div(A::sub(s.ad_value(2024), s.ad_value(1931)), s.ad_value(1930)), A::scale(s.ad_value(2022), 0.5)), A::mul(A::offset(s.ad_value(189), 1.0), s.ad_value(2023)));
        }

        if (s.v[2170] != 0.0) {
            s.store_offset_scaled(2026, 2022, 0.5, 2.0);
        }

        if (s.v[2170] != 0.0) {
            s.store_add(2027, 2022, 2023);
        }

        if (s.v[2170] != 0.0) {
            s.store_sub_ad(1930, A::sub(A::sub(s.ad_value(2024), s.ad_value(2027)), A::mul(s.ad_value(2012), A::sqrt(s.ad_value(2027)))), A::scale(A::ln(A::add(A::div(s.ad_value(2022), s.ad_value(2012)), A::sqrt(s.ad_value(2022)))), 2.0));
        }

        if (s.v[2170] != 0.0) {
            s.store_add_ad_lhs(2028, A::scale(s.ad_value(1930), 2.0), 2026);
        }

        if (s.v[2170] != 0.0) {
            s.store_scale_ad(1930, A::add(A::add(s.ad_value(2025), s.ad_value(2028)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2025), s.ad_value(2028)), A::sub(s.ad_value(2025), s.ad_value(2028))), 20.0))), 0.5);
        }

        if (s.v[2170] != 0.0) {
            s.store_sub_ad_lhs(1931, A::scale(A::sub(s.ad_value(2024), s.ad_value(2023)), 2.0), 2026);
        }

        if (s.v[2170] != 0.0) {
            s.store_scale_ad(2029, A::sub(A::add(s.ad_value(1930), s.ad_value(1931)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0))), 0.5);
        }

        if (s.v[2170] != 0.0) {
            s.store_scale_ad(1930, A::sub(A::add(s.ad_value(2029), s.ad_value(2026)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2029), s.ad_value(2026)), A::sub(s.ad_value(2029), s.ad_value(2026))), 5.0))), 0.5);
        }

        if (s.v[2170] != 0.0) {
            s.store_scale_ad(2030, A::add(A::sub(s.ad_value(1930), s.ad_value(2026)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), A::neg(s.ad_value(2026))), A::sub(s.ad_value(1930), A::neg(s.ad_value(2026)))), 20.0))), 0.5);
        }

        if (s.v[2170] != 0.0) {
            s.store_mul_ad_rhs(1931, 703, A::offset(A::div(s.ad_value(2030), s.ad_value(2026)), 1.0));
        }

        s.v[2171] = if (s.v[1931] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((s.v[2170] != 0.0) && (s.v[2171] != 0.0)) {
            s.store_exp(2031, 1931);
        }

        if ((s.v[2170] != 0.0) && (!(s.v[2171] != 0.0))) {
            s.store_div_from_scalar_ad(2031, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.store_offset_ad(2032, A::mul(s.ad_value(702), s.ad_value(2031)), 1.0);

        s.store_mul(2033, 1916, 2032);

        s.store_mul_ad(2034, A::mul(s.ad_value(197), A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0)), A::offset(A::mul(s.ad_value(198), s.ad_value(2019)), 1.0));

        s.store_mul_ad_rhs(2035, 2033, A::offset(s.ad_value(2034), 1.0));

        s.store_div_from_scalar(2036, 1.0, 2035);

        s.store_mul_ad_rhs(2020, 2012, A::sqrt(A::mul(s.ad_value(1916), s.ad_value(2036))));

        s.store_square(2021, 2020);

        s.store_div_from_scalar(2037, 1.0, 2021);

        s.store_mul(2038, 2014, 2036);

        s.store_mul(2039, 2017, 2036);

        s.store_div_ad(2040, A::scale(s.ad_value(819), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0));

        s.store_mul_ad(2041, A::mul(s.ad_value(194), s.ad_value(2040)), A::offset(A::mul(s.ad_value(196), s.ad_value(2019)), 1.0));

        s.store_mul(2042, 2010, 2036);

        s.store_sqrt_ad(1930, A::add(A::square(s.ad_value(2013)), s.ad_value(2011)));

        s.store_sqrt_ad(1931, A::add(A::mul(A::sub(s.ad_value(2013), s.ad_value(2041)), A::sub(s.ad_value(2013), s.ad_value(2041))), s.ad_value(2011)));

        s.store_mul_ad(2043, A::scale(s.ad_value(2036), 0.5), A::sub(A::add(s.ad_value(2041), s.ad_value(1930)), s.ad_value(1931)));

        s.store_add(2044, 2042, 2038);

        s.store_sub(2045, 2044, 2043);

        s.v[2172] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2173] = if (((s.v[2045]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2172] != 0.0) && (s.v[2173] != 0.0)) {
            s.store_offset_ad(2046, A::mul(s.ad_value(2020), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2045), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.3125))))), 1.0);
        }

        s.v[2174] = if (s.v[2045] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2172] != 0.0) && (!(s.v[2173] != 0.0))) && (s.v[2174] != 0.0)) {
            s.store_exp_ad(2060, A::neg(s.ad_value(2045)));
        }

        if (((s.v[2172] != 0.0) && (!(s.v[2173] != 0.0))) && (!(s.v[2174] != 0.0))) {
            s.store_div_from_scalar_ad(2060, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2045), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2045), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2045), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2172] != 0.0) && (!(s.v[2173] != 0.0))) {
            s.store_scalar(1929, (if (s.v[2045] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((s.v[2172] != 0.0) && (!(s.v[2173] != 0.0))) {
            s.store_offset_ad(2046, A::div(A::mul(A::mul(s.ad_value(1929), s.ad_value(2020)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2060), A::sub_from_scalar(1.0, s.ad_value(2045))))), A::scale(A::sqrt(A::mul(s.ad_value(2045), A::sub_from_scalar(1.0, s.ad_value(2060)))), 2.0)), 1.0);
        }

        if (!(s.v[2172] != 0.0)) {
            s.store_offset_ad(2046, A::div(A::scale(s.ad_value(2020), 0.5), A::sqrt(s.ad_value(2045))), 1.0);
        }

        s.store_sub_ad(2047, A::add(s.ad_value(2045), A::mul(s.ad_value(2020), A::sqrt(s.ad_value(2045)))), A::mul(s.ad_value(2046), A::ln(A::offset(s.ad_value(2046), (-1.0)))));

        s.store_div_ad_lhs(2048, A::sub(s.ad_value(2039), s.ad_value(2047)), 2046);

        s.store_mul_ad(2054, A::scale(s.ad_value(2021), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2021)), 1.0)), (-1.0)));

        s.v[2053] = 0.0;

        s.v[2055] = 1.0;

        s.v[2175] = if (s.v[2048] > (-30.0)) { 1.0 } else { 0.0 };

        if (s.v[2175] != 0.0) {
            s.store_offset_ad(2049, A::mul(s.ad_value(2046), s.ad_value(2048)), (-1.0));
        }

        if (s.v[2175] != 0.0) {
            s.store_scale_ad(1929, A::add(s.ad_value(2049), A::sqrt(A::offset(A::square(s.ad_value(2049)), 10.0))), 0.5);
        }

        if (s.v[2175] != 0.0) {
            s.store_sub_ad_rhs(2050, 2048, A::ln(s.ad_value(1929)));
        }

        if (s.v[2175] != 0.0) {
            s.store_scale_ad(2051, A::add(s.ad_value(2050), A::sqrt(A::offset(A::square(s.ad_value(2050)), 2.0))), 0.5);
        }

        s.v[2176] = if ((s.v[2048] - s.v[2051]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2175] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(2048), s.ad_value(2051)));
        }

        if ((s.v[2175] != 0.0) && (!(s.v[2176] != 0.0))) {
            s.store_scale_ad(1929, A::offset(A::mul(A::offset(A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[2175] != 0.0) {
            s.store_div(2052, 1929, 2046);
        }

        if (s.v[2175] != 0.0) {
            s.store_sub_ad_lhs(1929, A::scale(A::offset(s.ad_value(2051), 1.0), 2.0), 2052);
        }

        s.v[2177] = if (s.v[2052] > 1e-6) { 1.0 } else { 0.0 };

        if ((s.v[2175] != 0.0) && (s.v[2177] != 0.0)) {
            s.store_mul_ad_rhs(2053, 2046, A::offset(A::sub(s.ad_value(2051), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2052), s.ad_value(1929)), 1.0)), (-1.0)), s.ad_value(2052))), 1.0));
        }

        if ((s.v[2175] != 0.0) && (!(s.v[2177] != 0.0))) {
            s.store_mul_ad(2053, A::mul(A::scale(s.ad_value(2046), 0.5), s.ad_value(2052)), A::offset(A::mul(A::scale(s.ad_value(1929), 0.25), s.ad_value(1929)), 1.0));
        }

        if (s.v[2175] != 0.0) {
            s.store_scale_ad(1929, A::add(A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0)), A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0))), 1.0))), 0.5);
        }

        if (s.v[2175] != 0.0) {
            s.store_mul_ad(2054, A::scale(s.ad_value(2021), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2021)), s.ad_value(1929)), 1.0)), (-1.0)));
        }

        if (s.v[2175] != 0.0) {
            s.store_div_ad_rhs(2055, 2054, A::add(s.ad_value(2054), s.ad_value(2053)));
        }

        if (s.v[2175] != 0.0) {
            s.store_sub_ad_rhs(2045, 2044, A::mul(s.ad_value(2055), s.ad_value(2043)));
        }

        s.store_offset_scaled(2056, 2020, 0.7071067811865475, 1.0);

        s.store_scale(2057, 2056, 1e-5);

        s.store_div_from_scalar(2058, 1.0, 2056);

        s.v[2165] = 0.0;

        s.v[2059] = 0.0;

        s.v[2178] = if (s.v[2045] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[2178] != 0.0) {
            s.store_exp_ad(2060, A::neg(s.ad_value(2045)));
        }

        if (!(s.v[2178] != 0.0)) {
            s.store_div_from_scalar_ad(2060, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2045), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2045), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2045), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2179] = if (((s.v[2039]) as f64).abs() <= s.v[2057]) { 1.0 } else { 0.0 };

        if (s.v[2179] != 0.0) {
            s.store_scale_ad(2145, A::square(s.ad_value(2058)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (s.v[2179] != 0.0) {
            s.store_mul_ad(2059, A::mul(s.ad_value(2039), s.ad_value(2058)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2039), A::sub_from_scalar(1.0, s.ad_value(2060))), s.ad_value(2020)), s.ad_value(2145)), 1.0));
        }

        s.v[2180] = if (s.v[2039] < (-s.v[2057])) { 1.0 } else { 0.0 };

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_neg(2147, 2039);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_scaled_mul(2148, 2147, 2058, 1.25);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_scale_ad(2149, A::sub(A::offset(s.ad_value(2148), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2148), (-6.0)), A::offset(s.ad_value(2148), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub(2144, 2147, 2149);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_add_ad(2150, A::square(s.ad_value(2144)), A::mul(s.ad_value(2021), A::offset(s.ad_value(2149), 1.0)));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub_ad_lhs(2151, A::scale(s.ad_value(2144), 2.0), 2021);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub_ad_lhs(2152, A::ln(A::mul(s.ad_value(2150), s.ad_value(2037))), 2149);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_add(813, 2150, 2151);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2152), A::sub(A::scale(A::square(s.ad_value(2151)), 0.5), s.ad_value(2150))));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_add_ad_rhs(2153, 2149, A::div(A::mul(A::mul(s.ad_value(2150), s.ad_value(813)), s.ad_value(2152)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152)), s.ad_value(2152)), s.ad_value(2151)), A::sub(A::scale(A::square(s.ad_value(2151)), 0.3333333333333333), s.ad_value(2150))))));
        }

        s.v[2181] = if (s.v[2153] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) && (s.v[2181] != 0.0)) {
            s.store_exp(2154, 2153);
        }

        if (((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) && (!(s.v[2181] != 0.0))) {
            s.store_scale_ad(2154, A::offset(A::mul(A::offset(s.ad_value(2153), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2153), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2153), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_div_from_scalar(2155, 1.0, 2154);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_div_from_scalar_ad(2144, 1.0, A::offset(A::square(s.ad_value(2153)), 2.0));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_mul_ad_lhs(2156, A::square(s.ad_value(2153)), 2144);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_scale_ad(2157, A::mul(A::mul(s.ad_value(2153), s.ad_value(2144)), s.ad_value(2144)), 4.0);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_mul_ad_lhs(2158, A::mul(A::sub(A::scale(s.ad_value(2144), 8.0), A::scale(s.ad_value(2156), 12.0)), s.ad_value(2144)), 2144);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub(2144, 2147, 2153);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_mul(2145, 2060, 2155);
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_add_ad(2159, A::scale(s.ad_value(2144), 2.0), A::mul(s.ad_value(2021), A::add(A::sub(A::offset(s.ad_value(2154), (-1.0)), s.ad_value(2145)), A::mul(s.ad_value(2060), A::sub_from_scalar(1.0, s.ad_value(2157))))));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub_ad(2160, A::square(s.ad_value(2144)), A::mul(s.ad_value(2021), A::add(A::add(A::offset(A::sub(s.ad_value(2154), s.ad_value(2153)), (-1.0)), s.ad_value(2145)), A::mul(s.ad_value(2060), A::sub(A::offset(s.ad_value(2153), (-1.0)), s.ad_value(2156))))));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::sub(A::add(s.ad_value(2154), s.ad_value(2145)), A::mul(s.ad_value(2060), s.ad_value(2158)))));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub_ad(2144, A::square(s.ad_value(2159)), A::scale(A::mul(s.ad_value(2160), s.ad_value(2144)), 2.0));
        }

        if ((!(s.v[2179] != 0.0)) && (s.v[2180] != 0.0)) {
            s.store_sub_ad(2059, A::neg(s.ad_value(2153)), A::scale(A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_div_from_scalar_ad(2161, 1.0, A::offset(A::scale(s.ad_value(2020), 0.7324648775608221), 1.25));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_mul_ad_lhs(2162, A::offset(A::mul(A::scale(s.ad_value(2056), 1.25), s.ad_value(2161)), (-1.0)), 2161);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_mul_ad(2163, A::mul(s.ad_value(2039), s.ad_value(2058)), A::offset(A::mul(s.ad_value(2162), s.ad_value(2039)), 1.0));
        }

        s.v[2182] = if ((-s.v[2163]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (s.v[2182] != 0.0)) {
            s.store_exp_ad(2144, A::neg(s.ad_value(2163)));
        }

        if (((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (!(s.v[2182] != 0.0))) {
            s.store_div_from_scalar_ad(2144, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2163))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2163))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2163))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_from_scalar(2164, 1.0, 2144);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_ad(2165, A::add(s.ad_value(2039), A::scale(s.ad_value(2021), 0.5)), A::mul(s.ad_value(2020), A::sqrt(A::sub(A::add(s.ad_value(2039), A::scale(s.ad_value(2021), 0.25)), s.ad_value(2164)))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_offset(2166, 2045, 3.0);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_ad(2149, A::scale(A::sub(A::add(s.ad_value(2165), s.ad_value(2166)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2165), s.ad_value(2166)), A::sub(s.ad_value(2165), s.ad_value(2166))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2166), A::sqrt(A::offset(A::square(s.ad_value(2166)), 5.0))), 0.5));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub(2144, 2039, 2149);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_exp_ad(2145, A::neg(s.ad_value(2149)));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_div_from_scalar_ad(2146, 1.0, A::offset(A::square(s.ad_value(2149)), 2.0));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_mul_ad_lhs(2156, A::square(s.ad_value(2149)), 2146);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_scale_ad(2157, A::mul(A::mul(s.ad_value(2149), s.ad_value(2146)), s.ad_value(2146)), 4.0);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_mul_ad_lhs(2158, A::mul(A::sub(A::scale(s.ad_value(2146), 8.0), A::scale(s.ad_value(2156), 12.0)), s.ad_value(2146)), 2146);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            let assign42220_ad_e55425: A = {
                if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2060] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2144)), A::mul(s.ad_value(2021), A::sub(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), A::mul(s.ad_value(2060), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156))))))
                }
            };
            s.store_ad(2150, &assign42220_ad_e55425);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_from_scalar_ad(2167, 1.0, A::scale(A::mul(s.ad_value(2021), A::sub(s.ad_value(2145), A::mul(s.ad_value(2060), s.ad_value(2158)))), 0.5));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_add_ad(2151, A::scale(s.ad_value(2144), 2.0), A::mul(s.ad_value(2021), A::sub(A::sub_from_scalar(1.0, s.ad_value(2145)), A::mul(s.ad_value(2060), A::offset(s.ad_value(2157), 1.0)))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_add_ad(2152, A::sub(s.ad_value(2045), s.ad_value(2149)), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_add(813, 2150, 2151);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2152), A::sub(A::scale(A::square(s.ad_value(2151)), 0.5), A::mul(s.ad_value(2150), s.ad_value(2167)))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            let assign42280_ad_e55548: A = A::add(s.ad_value(2149), A::div(A::mul(A::mul(s.ad_value(2150), s.ad_value(813)), s.ad_value(2152)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152)), s.ad_value(2152)), s.ad_value(2151)), A::sub(A::scale(A::square(s.ad_value(2151)), 0.3333333333333333), A::mul(s.ad_value(2150), s.ad_value(2167)))))));
            s.store_ad(2168, &assign42280_ad_e55548);
        }

        s.v[2183] = if (s.v[2168] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (s.v[2183] != 0.0)) {
            s.store_exp(2154, 2168);
        }

        if (((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (s.v[2183] != 0.0)) {
            s.store_div_from_scalar(2155, 1.0, 2154);
        }

        if (((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (s.v[2183] != 0.0)) {
            s.store_mul(2154, 2060, 2154);
        }

        s.v[2184] = if (s.v[2168] > (s.v[2045] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (!(s.v[2183] != 0.0))) && (s.v[2184] != 0.0)) {
            s.store_exp_ad(2154, A::sub(s.ad_value(2168), s.ad_value(2045)));
        }

        if ((((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (!(s.v[2183] != 0.0))) && (s.v[2184] != 0.0)) {
            s.store_div(2155, 2060, 2154);
        }

        if ((((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (!(s.v[2183] != 0.0))) && (!(s.v[2184] != 0.0))) {
            s.store_div_from_scalar_ad(2154, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) && (!(s.v[2183] != 0.0))) && (!(s.v[2184] != 0.0))) {
            s.store_div_from_scalar_ad(2155, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2168), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2168), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2168), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_div_from_scalar_ad(2144, 1.0, A::offset(A::square(s.ad_value(2168)), 2.0));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_mul_ad_lhs(2156, A::square(s.ad_value(2168)), 2144);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_scale_ad(2157, A::mul(A::mul(s.ad_value(2168), s.ad_value(2144)), s.ad_value(2144)), 4.0);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_mul_ad_lhs(2158, A::mul(A::sub(A::scale(s.ad_value(2144), 8.0), A::scale(s.ad_value(2156), 12.0)), s.ad_value(2144)), 2144);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub(2144, 2039, 2168);
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_add_ad(2159, A::scale(s.ad_value(2144), 2.0), A::mul(s.ad_value(2021), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2155)), s.ad_value(2154)), A::mul(s.ad_value(2060), A::offset(s.ad_value(2157), 1.0)))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_ad(2160, A::square(s.ad_value(2144)), A::mul(s.ad_value(2021), A::sub(A::add(A::offset(A::add(s.ad_value(2155), s.ad_value(2168)), (-1.0)), s.ad_value(2154)), A::mul(s.ad_value(2060), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156))))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::sub(A::add(s.ad_value(2155), s.ad_value(2154)), A::mul(s.ad_value(2060), s.ad_value(2158)))));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_sub_ad(2144, A::square(s.ad_value(2159)), A::scale(A::mul(s.ad_value(2160), s.ad_value(2144)), 2.0));
        }

        if ((!(s.v[2179] != 0.0)) && (!(s.v[2180] != 0.0))) {
            s.store_add_ad_rhs(2059, 2168, A::scale(A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0));
        }

        s.v[2062] = 0.0;

        s.v[2063] = 0.0;

        s.v[2064] = 0.0;

        s.v[2065] = 0.0;

        s.v[2066] = 0.0;

        s.v[2067] = 0.0;

        s.v[2068] = 0.0;

        s.v[2069] = 1.0;

        s.v[2070] = 1.0;

        s.store_sub(2071, 2039, 2059);

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
        s.v[2072] = 0.0;

        s.store_mul(2073, 2035, 2071);

        s.v[2074] = 1.0;

        s.v[2075] = 1.0;

        s.v[2079] = 1.0;

        s.v[2080] = 1.0;

        s.v[2082] = 1.0;

        s.v[2185] = if (s.v[2039] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2185] != 0.0) {
            s.store_div_from_scalar_ad(1929, 1.0, A::offset(A::square(s.ad_value(2059)), 2.0));
        }

        if (s.v[2185] != 0.0) {
            s.store_mul_ad_lhs(2061, A::square(s.ad_value(2059)), 1929);
        }

        if (s.v[2185] != 0.0) {
            s.store_scale_ad(2062, A::mul(A::mul(s.ad_value(2059), s.ad_value(1929)), s.ad_value(1929)), 4.0);
        }

        if (s.v[2185] != 0.0) {
            s.store_mul_ad_lhs(2063, A::mul(A::sub(A::scale(s.ad_value(1929), 8.0), A::scale(s.ad_value(2061), 12.0)), s.ad_value(1929)), 1929);
        }

        if (s.v[2185] != 0.0) {
            s.store_scalar(2064, 0.0);
        }

        s.v[2186] = if (s.v[2059] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2185] != 0.0) && (s.v[2186] != 0.0)) {
            s.store_exp(2064, 2059);
        }

        if ((s.v[2185] != 0.0) && (s.v[2186] != 0.0)) {
            s.store_div_from_scalar(2065, 1.0, 2064);
        }

        if ((s.v[2185] != 0.0) && (s.v[2186] != 0.0)) {
            s.store_mul(2064, 2060, 2064);
        }

        s.v[2187] = if (s.v[2059] > (s.v[2045] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2185] != 0.0) && (!(s.v[2186] != 0.0))) && (s.v[2187] != 0.0)) {
            s.store_exp_ad(2064, A::sub(s.ad_value(2059), s.ad_value(2045)));
        }

        if (((s.v[2185] != 0.0) && (!(s.v[2186] != 0.0))) && (s.v[2187] != 0.0)) {
            s.store_div(2065, 2060, 2064);
        }

        if (((s.v[2185] != 0.0) && (!(s.v[2186] != 0.0))) && (!(s.v[2187] != 0.0))) {
            s.store_div_from_scalar_ad(2064, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2185] != 0.0) && (!(s.v[2186] != 0.0))) && (!(s.v[2187] != 0.0))) {
            s.store_div_from_scalar_ad(2065, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2059), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2059), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2059), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (s.v[2185] != 0.0) {
            s.store_sub_ad_rhs(2066, 2064, A::mul(s.ad_value(2060), A::add(A::offset(s.ad_value(2059), 1.0), s.ad_value(2061))));
        }

        s.v[2188] = if (s.v[2059] < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2185] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_scale_ad(2067, A::mul(A::square(s.ad_value(2059)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2059), A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((s.v[2185] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_scale_ad(2066, A::mul(A::mul(A::mul(A::mul(s.ad_value(2060), s.ad_value(2059)), s.ad_value(2059)), s.ad_value(2059)), A::offset(A::scale(s.ad_value(2059), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((s.v[2185] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2059), A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.25))), 0.3333333333333333)));
        }

        if ((s.v[2185] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_scaled_mul(2068, 2059, 1929, 0.7071067811865475);
        }

        if ((s.v[2185] != 0.0) && (s.v[2188] != 0.0)) {
            s.store_offset_ad(2069, A::scale(A::div(A::mul(s.ad_value(2020), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.5)), A::scale(A::square(s.ad_value(2059)), 0.16666666666666666))), s.ad_value(1929)), 0.7071067811865475), 1.0);
        }

        if ((s.v[2185] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_add_ad_lhs(2067, A::offset(s.ad_value(2059), (-1.0)), 2065);
        }

        if ((s.v[2185] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_sqrt(2068, 2067);
        }

        if ((s.v[2185] != 0.0) && (!(s.v[2188] != 0.0))) {
            s.store_offset_ad(2069, A::scale(A::div(A::mul(s.ad_value(2020), A::sub_from_scalar(1.0, s.ad_value(2065))), s.ad_value(2068)), 0.5), 1.0);
        }

        if (s.v[2185] != 0.0) {
            s.store_div_ad(2070, A::offset(A::mul(A::scale(s.ad_value(709), 0.2), s.ad_value(2019)), 1.0), A::offset(A::mul(s.ad_value(709), s.ad_value(2019)), 1.0));
        }

        s.v[2189] = if (s.v[2066] > 1e-100) { 1.0 } else { 0.0 };

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_mul_ad_rhs(2071, 2020, A::sqrt(A::add(s.ad_value(2067), s.ad_value(2066))));
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_div_ad(2072, A::mul(A::mul(s.ad_value(2021), s.ad_value(2066)), s.ad_value(2035)), A::add(s.ad_value(2071), A::mul(s.ad_value(2020), s.ad_value(2068))));
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_mul_ad_lhs(2073, A::mul(s.ad_value(2068), s.ad_value(2020)), 2035);
        }

        s.v[2190] = if (s.v[215] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (s.v[2190] != 0.0)) {
            s.store_div_from_scalar_ad(2074, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(215), s.ad_value(2019))));
        }

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (!(s.v[2190] != 0.0))) {
            s.store_offset_ad(2074, A::mul(s.ad_value(215), s.ad_value(2019)), 1.0);
        }

        s.v[2191] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (s.v[2191] != 0.0)) {
            s.store_sub_from_scalar_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2072)));
        }

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (!(s.v[2191] != 0.0))) {
            s.store_div_from_scalar_ad(2075, 1.0, A::offset(A::mul(s.ad_value(216), s.ad_value(2072)), 1.0));
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_mul_ad_lhs(2076, A::mul(A::mul(s.ad_value(746), s.ad_value(2074)), s.ad_value(2075)), 2072);
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_mul_ad_rhs(2077, 763, A::add(s.ad_value(2073), A::mul(s.ad_value(764), s.ad_value(2072))));
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_ln_ad(1930, A::div(s.ad_value(2067), A::offset(A::add(s.ad_value(2067), s.ad_value(2066)), 1e-14)));
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_add_ad(2078, A::pow(A::mul(s.ad_value(2077), s.ad_value(705)), s.ad_value(706)), A::mul(s.ad_value(707), A::exp(A::mul(A::scale(s.ad_value(708), 0.5), s.ad_value(1930)))));
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_mul_ad_lhs(2079, A::add(A::offset(s.ad_value(2078), 1.0), s.ad_value(2076)), 2070);
        }

        s.v[2192] = if (s.v[219] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (s.v[2192] != 0.0)) {
            s.store_div_from_scalar_ad(2080, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(219), s.ad_value(2019))));
        }

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (!(s.v[2192] != 0.0))) {
            s.store_offset_ad(2080, A::mul(s.ad_value(219), s.ad_value(2019)), 1.0);
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_mul(1931, 2072, 2080);
        }

        if ((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) {
            s.store_div_ad_rhs(2081, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.v[2193] = if (s.v[220] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (s.v[2193] != 0.0)) {
            s.store_div_from_scalar_ad(2082, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(220), s.ad_value(2081))));
        }

        if (((s.v[2185] != 0.0) && (s.v[2189] != 0.0)) && (!(s.v[2193] != 0.0))) {
            s.store_offset_ad(2082, A::mul(s.ad_value(220), s.ad_value(2081)), 1.0);
        }

        s.copy_ad(1806, 2017);

        s.copy_ad(1807, 2019);

        s.copy_ad(1808, 2035);

        s.copy_ad(1809, 2036);

        s.copy_ad(1810, 2020);

        s.copy_ad(1811, 2021);

        s.copy_ad(1812, 2037);

        s.copy_ad(1813, 2039);

        s.copy_ad(1814, 2044);

        s.copy_ad(1815, 2045);

        s.copy_ad(1816, 2056);

        s.copy_ad(1817, 2057);

        s.copy_ad(1818, 2058);

        s.copy_ad(1819, 2165);

        s.copy_ad(1820, 2060);

        s.copy_ad(1821, 2059);

        s.copy_ad(1822, 2062);

        s.copy_ad(1823, 2063);

        s.copy_ad(1824, 2064);

        s.copy_ad(1825, 2065);

        s.copy_ad(1826, 2067);

        s.copy_ad(1827, 2066);

        s.copy_ad(1828, 2068);

        s.copy_ad(1829, 2069);

        s.copy_ad(1830, 2070);

        s.copy_ad(1831, 2071);

        s.copy_ad(1832, 2072);

        s.copy_ad(1833, 2073);

        s.copy_ad(1834, 2074);

        s.copy_ad(1835, 2075);

        s.copy_ad(1836, 2079);

        s.copy_ad(1837, 2080);

        s.copy_ad(1838, 2082);

        s.v[2084] = 0.0;

        s.store_scale(2083, 2035, 4.60517018598809);

        s.copy_ad(2100, 2083);

        s.copy_ad(2101, 815);

        s.store_mul(2102, 815, 2036);

        s.copy_ad(2106, 2059);

        s.v[2107] = 0.0;

        s.v[2110] = 0.0;

        s.copy_ad(2112, 2065);

        s.copy_ad(2113, 2067);

        s.copy_ad(2115, 2066);

        s.copy_ad(2116, 2073);

        s.copy_ad(2117, 2059);

        s.copy_ad(2118, 2065);

        s.copy_ad(2120, 2066);

        s.copy_ad(2121, 2067);

        s.store_sub(2122, 2039, 2059);

        s.v[2123] = 1.0;

        s.v[2125] = 1.0;

        s.v[2124] = 0.0;

        s.copy_ad(2134, 2072);

        s.store_mul(2138, 2122, 2035);

        s.v[2135] = 0.0;

        s.copy_ad(2136, 2073);

        s.v[2141] = 0.0;

        s.v[2140] = 1.0;

        s.copy_ad(2143, 2015);

        s.copy_ad(2142, 2138);

        s.v[2194] = if (s.v[2039] > 0.0) { 1.0 } else { 0.0 };

        s.v[2195] = if (s.v[2066] > 1e-100) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_mul(2143, 2015, 2082);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_div(2084, 2143, 2079);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_add_ad_rhs(2085, 2071, A::scale(s.ad_value(2021), 0.5));
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_div_ad_lhs(1929, A::div(A::mul(s.ad_value(2021), s.ad_value(2064)), s.ad_value(2085)), 2085);
        }

        s.v[2196] = if (s.v[1929] > 0.0001) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2196] != 0.0)) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.v[2197] = if (s.v[1930] < 1e-10) { 1.0 } else { 0.0 };

        if ((((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2196] != 0.0)) && (s.v[2197] != 0.0)) {
            s.store_scalar(1931, 1.0);
        }

        if ((((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2196] != 0.0)) && (!(s.v[2197] != 0.0))) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (!(s.v[2196] != 0.0))) {
            s.store_scale(1931, 1929, 0.5);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_mul(2086, 1931, 2085);
        }

        s.v[2198] = if ((s.v[707] > 0.0) && (s.v[708] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_lhs(2087, A::scale(s.ad_value(2035), 0.475), 2086);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_sub_ad_rhs(1929, 2072, A::mul(s.ad_value(2069), s.ad_value(2087)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_scale_ad(2088, A::add(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1e-12))), 0.5);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_add_ad(2089, A::sub(A::mul(s.ad_value(2035), s.ad_value(2071)), s.ad_value(2072)), A::mul(A::offset(s.ad_value(2069), (-1.0)), s.ad_value(2087)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_offset_ad(2090, A::div(A::mul(A::scale(s.ad_value(2021), 0.5), s.ad_value(2035)), s.ad_value(2089)), 1.0);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_add_ad_rhs(1929, 2089, A::mul(s.ad_value(764), s.ad_value(2088)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_ad(2091, &A::pow(A::mul(A::mul(s.ad_value(763), s.ad_value(1929)), s.ad_value(705)), s.ad_value(706)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_lhs(1930, A::div(A::mul(s.ad_value(706), A::offset(A::mul(s.ad_value(2090), A::sub_from_scalar(1.0, s.ad_value(764))), (-1.0))), s.ad_value(1929)), 2091);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_div(1929, 2088, 2089);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_rhs(2092, 707, A::pow(A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708))));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_lhs(1931, A::div(A::mul(s.ad_value(708), A::add(A::offset(s.ad_value(2090), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1929), 1.0)))), s.ad_value(2089)), 2092);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_lhs(2093, A::mul(A::mul(s.ad_value(746), s.ad_value(2074)), s.ad_value(2075)), 2088);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_offset_ad(1929, A::div(A::sub(s.ad_value(1930), A::mul(A::mul(A::mul(s.ad_value(746), s.ad_value(2074)), s.ad_value(2075)), s.ad_value(2090))), s.ad_value(1931)), 1.0);
        }

        s.v[2199] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) && (s.v[2199] != 0.0)) {
            s.store_scale_ad(1930, A::ln(A::offset(A::exp(A::scale(s.ad_value(1929), 2.0)), 1.0)), 0.5);
        }

        if ((((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) && (!(s.v[2199] != 0.0))) {
            s.copy_ad(1930, 1929);
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_div_ad(2094, A::mul(A::mul(A::neg(s.ad_value(2087)), s.ad_value(1931)), s.ad_value(1930)), A::add(A::add(A::offset(s.ad_value(2091), 1.0), s.ad_value(2092)), s.ad_value(2093)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2198] != 0.0)) {
            s.store_mul_ad_rhs(2095, 2086, A::offset(A::div(s.ad_value(2094), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2094)), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (!(s.v[2198] != 0.0))) {
            s.copy_ad(2095, 2086);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_scale_ad(2096, A::mul(A::mul(s.ad_value(2035), s.ad_value(2084)), s.ad_value(2095)), 0.7071067811865475);
        }

        s.v[2200] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) && (s.v[2200] != 0.0)) {
            s.store_div_ad_rhs(2096, 2096, A::sqrt(A::offset(s.ad_value(2096), 1.0)));
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_div_from_scalar_ad(2097, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2096), 4.0), 1.0)), 1.0));
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_mul(1929, 2097, 2096);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_mul_ad(2098, A::mul(s.ad_value(2095), s.ad_value(2097)), A::offset(A::div(A::mul(A::scale(s.ad_value(1929), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(1929), s.ad_value(2097)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(1929), 4.0), s.ad_value(1929)), s.ad_value(2097)), 1.0)), 1.0));
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_scale(2099, 2098, 0.99);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_div_ad_lhs(1929, A::mul(A::mul(s.ad_value(2099), A::sub(s.ad_value(2099), A::scale(s.ad_value(2085), 2.0))), s.ad_value(2037)), 2066);
        }

        if ((s.v[2194] != 0.0) && (s.v[2195] != 0.0)) {
            s.store_mul_ad_rhs(2100, 2035, A::sub(s.ad_value(2099), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2195] != 0.0))) {
            s.copy_ad(2100, 2083);
        }

        if (s.v[2194] != 0.0) {
            s.store_offset(1929, 2016, 1.0);
        }

        if (s.v[2194] != 0.0) {
            s.store_div_ad_lhs(1930, A::mul(A::sqrt(s.ad_value(1929)), s.ad_value(815)), 2100);
        }

        if (s.v[2194] != 0.0) {
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
        }

        if (s.v[2194] != 0.0) {
            s.store_scale(1929, 1930, 2.0);
        }

        if (s.v[2194] != 0.0) {
            s.store_div_ad(2101, A::mul(s.ad_value(2100), s.ad_value(1929)), A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(2102, 2101, 2036);
        }

        if (s.v[2194] != 0.0) {
            s.store_add(2103, 2045, 2102);
        }

        s.v[2201] = if (s.v[2102] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2201] != 0.0)) {
            s.store_exp_ad(2104, A::neg(s.ad_value(2102)));
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
        if ((s.v[2194] != 0.0) && (!(s.v[2201] != 0.0))) {
            s.store_div_from_scalar_ad(2104, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2102), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2102), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2102), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(2105, 2060, 2104);
        }

        s.v[2202] = if (((s.v[2039]) as f64).abs() <= s.v[2057]) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2202] != 0.0)) {
            s.store_scale_ad(2145, A::square(s.ad_value(2058)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((s.v[2194] != 0.0) && (s.v[2202] != 0.0)) {
            s.store_mul_ad(2106, A::mul(s.ad_value(2039), s.ad_value(2058)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2039), A::sub_from_scalar(1.0, s.ad_value(2105))), s.ad_value(2020)), s.ad_value(2145)), 1.0));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_offset(2166, 2103, 3.0);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub_ad(2149, A::scale(A::sub(A::add(s.ad_value(2165), s.ad_value(2166)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2165), s.ad_value(2166)), A::sub(s.ad_value(2165), s.ad_value(2166))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2166), A::sqrt(A::offset(A::square(s.ad_value(2166)), 5.0))), 0.5));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub(2144, 2039, 2149);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_exp_ad(2145, A::neg(s.ad_value(2149)));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_div_from_scalar_ad(2146, 1.0, A::offset(A::square(s.ad_value(2149)), 2.0));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_mul_ad_lhs(2156, A::square(s.ad_value(2149)), 2146);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_scale_ad(2157, A::mul(A::mul(s.ad_value(2149), s.ad_value(2146)), s.ad_value(2146)), 4.0);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_mul_ad_lhs(2158, A::mul(A::sub(A::scale(s.ad_value(2146), 8.0), A::scale(s.ad_value(2156), 12.0)), s.ad_value(2146)), 2146);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            let assign44400_ad_e57369: A = {
                if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2105] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2144)), A::mul(s.ad_value(2021), A::sub(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), A::mul(s.ad_value(2105), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156))))))
                }
            };
            s.store_ad(2150, &assign44400_ad_e57369);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub_from_scalar_ad(2167, 1.0, A::scale(A::mul(s.ad_value(2021), A::sub(s.ad_value(2145), A::mul(s.ad_value(2105), s.ad_value(2158)))), 0.5));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_add_ad(2151, A::scale(s.ad_value(2144), 2.0), A::mul(s.ad_value(2021), A::sub(A::sub_from_scalar(1.0, s.ad_value(2145)), A::mul(s.ad_value(2105), A::offset(s.ad_value(2157), 1.0)))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_add_ad(2152, A::sub(s.ad_value(2103), s.ad_value(2149)), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_add(813, 2150, 2151);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2152), A::sub(A::scale(A::square(s.ad_value(2151)), 0.5), A::mul(s.ad_value(2150), s.ad_value(2167)))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            let assign44460_ad_e57486: A = A::add(s.ad_value(2149), A::div(A::mul(A::mul(s.ad_value(2150), s.ad_value(813)), s.ad_value(2152)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152)), s.ad_value(2152)), s.ad_value(2151)), A::sub(A::scale(A::square(s.ad_value(2151)), 0.3333333333333333), A::mul(s.ad_value(2150), s.ad_value(2167)))))));
            s.store_ad(2168, &assign44460_ad_e57486);
        }

        s.v[2203] = if (s.v[2168] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (s.v[2203] != 0.0)) {
            s.store_exp(2154, 2168);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (s.v[2203] != 0.0)) {
            s.store_div_from_scalar(2155, 1.0, 2154);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (s.v[2203] != 0.0)) {
            s.store_mul(2154, 2105, 2154);
        }

        s.v[2204] = if (s.v[2168] > (s.v[2103] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (!(s.v[2203] != 0.0))) && (s.v[2204] != 0.0)) {
            s.store_exp_ad(2154, A::sub(s.ad_value(2168), s.ad_value(2103)));
        }

        if ((((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (!(s.v[2203] != 0.0))) && (s.v[2204] != 0.0)) {
            s.store_div(2155, 2105, 2154);
        }

        if ((((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (!(s.v[2203] != 0.0))) && (!(s.v[2204] != 0.0))) {
            s.store_div_from_scalar_ad(2154, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) && (!(s.v[2203] != 0.0))) && (!(s.v[2204] != 0.0))) {
            s.store_div_from_scalar_ad(2155, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2168), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2168), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2168), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_div_from_scalar_ad(2144, 1.0, A::offset(A::square(s.ad_value(2168)), 2.0));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_mul_ad_lhs(2156, A::square(s.ad_value(2168)), 2144);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_scale_ad(2157, A::mul(A::mul(s.ad_value(2168), s.ad_value(2144)), s.ad_value(2144)), 4.0);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_mul_ad_lhs(2158, A::mul(A::sub(A::scale(s.ad_value(2144), 8.0), A::scale(s.ad_value(2156), 12.0)), s.ad_value(2144)), 2144);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub(2144, 2039, 2168);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_add_ad(2159, A::scale(s.ad_value(2144), 2.0), A::mul(s.ad_value(2021), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2155)), s.ad_value(2154)), A::mul(s.ad_value(2105), A::offset(s.ad_value(2157), 1.0)))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub_ad(2160, A::square(s.ad_value(2144)), A::mul(s.ad_value(2021), A::sub(A::add(A::offset(A::add(s.ad_value(2155), s.ad_value(2168)), (-1.0)), s.ad_value(2154)), A::mul(s.ad_value(2105), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156))))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::sub(A::add(s.ad_value(2155), s.ad_value(2154)), A::mul(s.ad_value(2105), s.ad_value(2158)))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_sub_ad(2144, A::square(s.ad_value(2159)), A::scale(A::mul(s.ad_value(2160), s.ad_value(2144)), 2.0));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2202] != 0.0))) {
            s.store_add_ad_rhs(2106, 2168, A::scale(A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0));
        }

        if (s.v[2194] != 0.0) {
            s.store_sub(2107, 2106, 2059);
        }

        s.v[2205] = if (s.v[2107] < 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_add_ad(2108, A::scale(A::sub(s.ad_value(2039), s.ad_value(2059)), 2.0), A::mul(s.ad_value(2021), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2065)), A::mul(s.ad_value(2064), s.ad_value(2104))), A::mul(s.ad_value(2105), A::offset(s.ad_value(2062), 1.0)))));
        }

        if ((s.v[2194] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_mul_ad_lhs(2109, A::mul(s.ad_value(2021), A::sub_from_scalar(1.0, s.ad_value(2104))), 2066);
        }

        if ((s.v[2194] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_sub_from_scalar_ad(1929, 2.0, A::mul(s.ad_value(2021), A::sub(A::add(s.ad_value(2065), A::mul(s.ad_value(2064), s.ad_value(2104))), A::mul(s.ad_value(2105), s.ad_value(2063)))));
        }

        if ((s.v[2194] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_sub_ad(1929, A::square(s.ad_value(2108)), A::scale(A::mul(s.ad_value(1929), s.ad_value(2109)), 2.0));
        }

        if ((s.v[2194] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_scale_ad(2107, A::div(s.ad_value(2109), A::add(s.ad_value(2108), A::sqrt(s.ad_value(1929)))), 2.0);
        }

        if ((s.v[2194] != 0.0) && (s.v[2205] != 0.0)) {
            s.store_add(2106, 2059, 2107);
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(2110, 2107, 2035);
        }

        if (s.v[2194] != 0.0) {
            s.store_div_ad(2111, A::square(s.ad_value(2106)), A::offset(A::square(s.ad_value(2106)), 2.0));
        }

        s.v[2206] = if (s.v[2106] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) {
            s.store_exp_ad(2112, A::neg(s.ad_value(2106)));
        }

        s.v[2207] = if (s.v[2106] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (s.v[2207] != 0.0)) {
            s.store_scale_ad(2113, A::mul(A::square(s.ad_value(2106)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2106), A::sub_from_scalar(1.0, A::scale(s.ad_value(2106), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (s.v[2207] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2106), A::sub_from_scalar(1.0, A::scale(s.ad_value(2106), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (s.v[2207] != 0.0)) {
            s.store_scaled_mul(2114, 2106, 1929, 0.7071067811865475);
        }

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (s.v[2207] != 0.0)) {
            s.store_mul_ad(2115, A::mul(A::mul(A::mul(A::scale(s.ad_value(2105), 0.16666666666666666), s.ad_value(2106)), s.ad_value(2106)), s.ad_value(2106)), A::offset(A::scale(s.ad_value(2106), 1.75), 1.0));
        }

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (!(s.v[2207] != 0.0))) {
            s.store_add_ad_lhs(2113, A::offset(s.ad_value(2106), (-1.0)), 2112);
        }

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (!(s.v[2207] != 0.0))) {
            s.store_sqrt(2114, 2113);
        }

        if (((s.v[2194] != 0.0) && (s.v[2206] != 0.0)) && (!(s.v[2207] != 0.0))) {
            s.store_mul_ad_rhs(2115, 2105, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2112)), s.ad_value(2106)), (-1.0)), s.ad_value(2111)));
        }

        s.v[2208] = if (s.v[2106] > (s.v[2103] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) && (s.v[2208] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(2106), s.ad_value(2103)));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) && (s.v[2208] != 0.0)) {
            s.store_div(2112, 2105, 1929);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) && (s.v[2208] != 0.0)) {
            s.store_sub_ad_rhs(2115, 1929, A::mul(s.ad_value(2105), A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111))));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) && (!(s.v[2208] != 0.0))) {
            s.store_div_from_scalar_ad(2112, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2106), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2106), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2106), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) && (!(s.v[2208] != 0.0))) {
            s.store_div_from_scalar_ad(1929, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) && (!(s.v[2208] != 0.0))) {
            s.store_sub_ad_rhs(2115, 1929, A::mul(s.ad_value(2105), A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) {
            s.store_add_ad_lhs(2113, A::offset(s.ad_value(2106), (-1.0)), 2112);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2206] != 0.0))) {
            s.store_sqrt(2114, 2113);
        }

        if (s.v[2194] != 0.0) {
            s.store_mul_ad_lhs(2116, A::mul(s.ad_value(2114), s.ad_value(2020)), 2035);
        }

        if (s.v[2194] != 0.0) {
            s.store_scaled_add(2117, 2059, 2106, 0.5);
        }

        if (s.v[2194] != 0.0) {
            s.store_scalar(2118, 0.0);
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(1929, 2112, 2065);
        }

        s.v[2209] = if (s.v[1929] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2209] != 0.0)) {
            s.store_sqrt(2118, 1929);
        }

        if (s.v[2194] != 0.0) {
            s.store_scaled_add(2119, 2066, 2115, 0.5);
        }

        if (s.v[2194] != 0.0) {
            s.store_add_ad_rhs(2120, 2119, A::scale(A::mul(A::square(s.ad_value(2107)), A::sub(s.ad_value(2118), A::scale(s.ad_value(2037), 2.0))), 0.125));
        }

        s.v[2210] = if (s.v[2117] < 1e-5) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2210] != 0.0)) {
            s.store_scale_ad(2121, A::mul(A::square(s.ad_value(2117)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2117), A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((s.v[2194] != 0.0) && (s.v[2210] != 0.0)) {
            s.store_mul_ad_rhs(2122, 2020, A::sqrt(A::add(s.ad_value(2120), s.ad_value(2121))));
        }

        s.v[2211] = if (s.v[719] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (s.v[2210] != 0.0)) && (s.v[2211] != 0.0)) {
            s.store_div_from_scalar_ad(2123, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0)));
        }

        if ((s.v[2194] != 0.0) && (s.v[2210] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2117), A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.25))), 0.3333333333333333)));
        }

        if ((s.v[2194] != 0.0) && (s.v[2210] != 0.0)) {
            s.store_scaled_mul(2124, 2117, 1929, 0.7071067811865475);
        }

        if ((s.v[2194] != 0.0) && (s.v[2210] != 0.0)) {
            s.store_add_ad_rhs(2125, 2123, A::scale(A::div(A::mul(s.ad_value(2020), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.5)), A::scale(A::square(s.ad_value(2117)), 0.16666666666666666))), s.ad_value(1929)), 0.7071067811865475));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) {
            s.store_add_ad_lhs(2121, A::offset(s.ad_value(2117), (-1.0)), 2118);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) {
            s.store_mul_ad_rhs(2122, 2020, A::sqrt(A::add(s.ad_value(2120), s.ad_value(2121))));
        }

        s.v[2212] = if (s.v[719] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_add_ad(2126, A::sub_from_scalar(1.0, s.ad_value(2118)), A::scale(A::mul(s.ad_value(2122), s.ad_value(2037)), 2.0));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_div_from_scalar_ad(2123, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0)));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_div_ad_rhs(1929, 2123, A::offset(s.ad_value(2123), 1.0));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_mul_ad_rhs(2127, 719, A::mul(A::mul(A::square(s.ad_value(1929)), s.ad_value(2021)), s.ad_value(2120)));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_add_ad(2128, A::scale(A::sub(s.ad_value(2122), s.ad_value(2127)), 2.0), A::mul(s.ad_value(2021), A::add(A::sub_from_scalar(1.0, s.ad_value(2118)), s.ad_value(2120))));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_mul_ad_rhs(2129, 2127, A::sub(s.ad_value(2127), A::scale(s.ad_value(2122), 2.0)));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_sub_from_scalar_ad(2130, 1.0, A::scale(A::mul(s.ad_value(2021), A::add(s.ad_value(2118), s.ad_value(2120))), 0.5));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_div_ad(2131, A::mul(s.ad_value(2129), s.ad_value(2128)), A::sub(A::square(s.ad_value(2128)), A::mul(s.ad_value(2130), s.ad_value(2129))));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_add(2117, 2117, 2131);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_exp(2132, 2131);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_div(2118, 2118, 2132);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_mul(2120, 2120, 2132);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_add_ad_lhs(2121, A::offset(s.ad_value(2117), (-1.0)), 2118);
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_mul_ad_rhs(2122, 2020, A::sqrt(A::add(s.ad_value(2120), s.ad_value(2121))));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_add_ad(2133, A::sub_from_scalar(1.0, s.ad_value(2118)), A::scale(A::mul(A::mul(s.ad_value(2122), s.ad_value(2123)), s.ad_value(2037)), 2.0));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_div_ad(2107, A::mul(A::mul(s.ad_value(2107), s.ad_value(2132)), A::add(s.ad_value(2126), s.ad_value(2119))), A::add(s.ad_value(2133), A::mul(s.ad_value(2132), s.ad_value(2119))));
        }

        if (((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) && (s.v[2212] != 0.0)) {
            s.store_mul(2110, 2107, 2035);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) {
            s.store_sqrt(2124, 2121);
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2210] != 0.0))) {
            s.store_add_ad_rhs(2125, 2123, A::scale(A::div(A::mul(s.ad_value(2020), A::sub_from_scalar(1.0, s.ad_value(2118))), s.ad_value(2124)), 0.5));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul_ad_rhs(2134, 2035, A::div(A::mul(s.ad_value(2021), s.ad_value(2120)), A::add(s.ad_value(2122), A::mul(s.ad_value(2020), s.ad_value(2124)))));
        }

        if (s.v[2194] != 0.0) {
            s.store_add_ad_rhs(2135, 2134, A::mul(s.ad_value(2035), s.ad_value(2125)));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul_ad_lhs(2136, A::mul(s.ad_value(2124), s.ad_value(2020)), 2035);
        }

        s.v[2213] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2213] != 0.0)) {
            s.store_sub_from_scalar_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2134)));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2213] != 0.0))) {
            s.store_div_from_scalar_ad(2075, 1.0, A::offset(A::mul(s.ad_value(216), s.ad_value(2134)), 1.0));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul_ad_lhs(2076, A::mul(A::mul(s.ad_value(746), s.ad_value(2074)), s.ad_value(2075)), 2134);
        }

        if (s.v[2194] != 0.0) {
            s.store_add_ad_rhs(2137, 2136, A::mul(s.ad_value(764), s.ad_value(2134)));
        }

        if (s.v[2194] != 0.0) {
            s.store_add_ad_rhs(2138, 2136, A::mul(s.ad_value(765), s.ad_value(2134)));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(2139, 763, 2137);
        }

        if (s.v[2194] != 0.0) {
            s.store_ln_ad(1930, A::div(s.ad_value(2121), A::offset(A::add(s.ad_value(2121), s.ad_value(2120)), 1e-14)));
        }

        if (s.v[2194] != 0.0) {
            s.store_add_ad(2078, A::pow(A::mul(s.ad_value(2139), s.ad_value(705)), s.ad_value(706)), A::mul(s.ad_value(707), A::exp(A::mul(A::scale(s.ad_value(708), 0.5), s.ad_value(1930)))));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul_ad_lhs(2140, A::add(A::offset(s.ad_value(2078), 1.0), s.ad_value(2076)), 2070);
        }

        if (s.v[2194] != 0.0) {
            s.store_ln_ad(2141, A::div(A::offset(A::mul(A::sub(s.ad_value(815), s.ad_value(2110)), s.ad_value(768)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2101), s.ad_value(2110)), s.ad_value(768)), 1.0)));
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(1931, 2134, 2080);
        }

        if (s.v[2194] != 0.0) {
            s.store_div_ad_rhs(2081, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.v[2214] = if (s.v[220] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2194] != 0.0) && (s.v[2214] != 0.0)) {
            s.store_div_from_scalar_ad(2082, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(220), s.ad_value(2081))));
        }

        if ((s.v[2194] != 0.0) && (!(s.v[2214] != 0.0))) {
            s.store_offset_ad(2082, A::mul(s.ad_value(220), s.ad_value(2081)), 1.0);
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(2143, 2015, 2082);
        }

        if (s.v[2194] != 0.0) {
            s.store_mul(2142, 2122, 2035);
        }

        s.copy_ad(1839, 2083);

        s.copy_ad(1841, 2101);

        s.copy_ad(1842, 2102);

        s.copy_ad(1843, 2107);

        s.copy_ad(1844, 2110);

        s.copy_ad(1846, 2117);

        s.copy_ad(1845, 2116);

        s.copy_ad(1847, 2123);

        s.copy_ad(1848, 2125);

        s.copy_ad(1849, 2134);

        s.copy_ad(1850, 2135);

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
        s.copy_ad(1851, 2136);

        s.copy_ad(1852, 2138);

        s.copy_ad(1853, 2140);

        s.copy_ad(1855, 2141);

        s.copy_ad(1854, 2143);

        s.copy_ad(1856, 2142);

        s.v[1857] = 1.0;

        s.v[1858] = 1.0;

        s.v[1860] = 1.0;

        s.v[1861] = 1.0;

        s.v[827] = 0.0;

        s.v[2215] = if (s.v[1813] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2215] != 0.0) {
            s.store_ln_ad(1939, A::offset(A::mul(s.ad_value(819), s.ad_value(768)), 1.0));
        }

        if (s.v[2215] != 0.0) {
            s.store_div_ad_lhs(1929, A::mul(s.ad_value(1808), s.ad_value(1848)), 1850);
        }

        if (s.v[2215] != 0.0) {
            s.store_add_ad(1938, A::mul(A::div(A::mul(A::add(s.ad_value(223), A::div(s.ad_value(224), s.ad_value(1850))), s.ad_value(1849)), s.ad_value(1850)), s.ad_value(1855)), A::mul(A::mul(A::mul(A::mul(s.ad_value(225), s.ad_value(1851)), s.ad_value(1929)), s.ad_value(1929)), s.ad_value(1939)));
        }

        if (s.v[2215] != 0.0) {
            s.store_div_from_scalar_ad(1857, 1.0, A::add(A::offset(s.ad_value(1938), 1.0), A::square(s.ad_value(1938))));
        }

        if (s.v[2215] != 0.0) {
            s.store_mul(1858, 1853, 1857);
        }

        if (s.v[2215] != 0.0) {
            s.store_div(1859, 1854, 1858);
        }

        if (s.v[2215] != 0.0) {
            s.store_mul_ad_lhs(1940, A::mul(A::square(s.ad_value(1859)), s.ad_value(1844)), 1844);
        }

        s.v[2216] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2215] != 0.0) && (s.v[2216] != 0.0)) {
            s.store_div_ad_rhs(1940, 1940, A::offset(A::mul(s.ad_value(1859), s.ad_value(1844)), 1.0));
        }

        if (s.v[2215] != 0.0) {
            s.store_scale_ad(1941, A::mul(s.ad_value(1858), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(1940), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2215] != 0.0) {
            s.store_div_from_scalar(1860, 1.0, 1941);
        }

        if (s.v[2215] != 0.0) {
            s.store_mul(1929, 1858, 1860);
        }

        if (s.v[2215] != 0.0) {
            s.store_mul_ad_rhs(1942, 1848, A::offset(A::scale(A::mul(A::mul(s.ad_value(1940), s.ad_value(1929)), s.ad_value(1929)), 0.5), 1.0));
        }

        if (s.v[2215] != 0.0) {
            s.store_div_ad_lhs(1861, A::mul(s.ad_value(1929), s.ad_value(1850)), 1942);
        }

        if (s.v[2215] != 0.0) {
            s.store_mul_ad_lhs(827, A::mul(A::mul(s.ad_value(1917), s.ad_value(1850)), s.ad_value(1844)), 1860);
        }

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1862] = 0.0;

        s.v[1863] = 0.0;

        s.v[2217] = if (((((p.p40 != 0.0) && ((s.v[235] > 0.0) || (s.v[236] > 0.0))) || ((p.p42 != 0.0) && ((s.v[245] > 0.0) || (s.v[246] > 0.0)))) || (s.v[260] > 0.0)) || (s.v[261] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2217] != 0.0) {
            s.store_scale_ad(1943, A::add(s.ad_value(1801), A::sqrt(A::add(A::square(s.ad_value(1801)), s.ad_value(778)))), 0.5);
        }

        if (s.v[2217] != 0.0) {
            s.store_add_ad_lhs(1944, A::add(A::sub(A::neg(s.ad_value(1943)), A::scale(s.ad_value(773), 0.5)), A::mul(s.ad_value(771), A::sqrt(A::add(A::add(s.ad_value(1943), A::scale(s.ad_value(773), 0.25)), s.ad_value(779))))), 780);
        }

        if (s.v[2217] != 0.0) {
            s.store_scale_ad(1943, A::add(s.ad_value(1802), A::sqrt(A::add(A::square(s.ad_value(1802)), s.ad_value(781)))), 0.5);
        }

        if (s.v[2217] != 0.0) {
            s.store_add_ad_lhs(1945, A::add(A::sub(A::neg(s.ad_value(1943)), A::scale(s.ad_value(774), 0.5)), A::mul(s.ad_value(772), A::sqrt(A::add(A::add(s.ad_value(1943), A::scale(s.ad_value(774), 0.25)), s.ad_value(782))))), 783);
        }

        if (s.v[2217] != 0.0) {
            s.store_scaled_add(1862, 1801, 1944, (-s.v[355]));
        }

        if (s.v[2217] != 0.0) {
            s.store_scaled_add(1863, 1802, 1945, (-s.v[355]));
        }

        s.v[2218] = if (p.p40 != 0.0) { 1.0 } else { 0.0 };

        s.v[2219] = if (s.v[235] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_mul_ad_lhs(1946, A::sqrt(A::offset(A::square(s.ad_value(1862)), 1e-6)), 784);
        }

        s.v[2220] = if (s.v[241] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) && (s.v[2220] != 0.0)) {
            s.store_scale_ad(1946, A::sub(A::add(s.ad_value(1946), s.ad_value(790)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(790)), A::sub(s.ad_value(1946), s.ad_value(790))), 1e-6))), 0.5);
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_mul_ad_rhs(1929, 787, A::offset(A::mul(s.ad_value(1946), A::add(s.ad_value(240), A::mul(s.ad_value(241), s.ad_value(1946)))), (-1.5)));
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_offset(1948, 1944, 3.0);
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_sub_from_scalar(1949, (-3.0), 233);
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_scale(1950, 823, 30.0);
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_scalar(807, (4.0 - 0.9));
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_add(808, 1948, 1950);
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_mul_ad(1929, A::div_from_scalar(2.0, s.ad_value(807)), A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul(A::mul(s.ad_value(807), s.ad_value(1948)), s.ad_value(1950))))));
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_scalar(807, (4.0 - 0.3));
        }

        if ((s.v[2218] != 0.0) && (s.v[2219] != 0.0)) {
            s.store_add(808, 1949, 1929);
        }

        s.v[2223] = if (s.v[236] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_mul_ad_lhs(1946, A::sqrt(A::offset(A::square(s.ad_value(1863)), 1e-6)), 784);
        }

        s.v[2224] = if (s.v[243] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) && (s.v[2224] != 0.0)) {
            s.store_scale_ad(1946, A::sub(A::add(s.ad_value(1946), s.ad_value(791)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(791)), A::sub(s.ad_value(1946), s.ad_value(791))), 1e-6))), 0.5);
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_mul_ad_rhs(1929, 788, A::offset(A::mul(s.ad_value(1946), A::add(s.ad_value(242), A::mul(s.ad_value(243), s.ad_value(1946)))), (-1.5)));
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_offset(1948, 1945, 3.0);
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_sub_from_scalar(1949, (-3.0), 233);
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_scale(1950, 826, 30.0);
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_scalar(807, (4.0 - 0.9));
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_add(808, 1948, 1950);
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_mul_ad(1929, A::div_from_scalar(2.0, s.ad_value(807)), A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul(A::mul(s.ad_value(807), s.ad_value(1948)), s.ad_value(1950))))));
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_scalar(807, (4.0 - 0.3));
        }

        if ((s.v[2218] != 0.0) && (s.v[2223] != 0.0)) {
            s.store_add(808, 1949, 1929);
        }

        s.v[2227] = if (s.v[234] > 0.0) { 1.0 } else { 0.0 };

        s.v[2228] = if (s.v[1813] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2228] != 0.0)) {
            s.store_offset(1929, 766, 1.0);
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2228] != 0.0)) {
            s.store_div_ad_lhs(1930, A::mul(A::sqrt(s.ad_value(1929)), s.ad_value(815)), 1839);
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2228] != 0.0)) {
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2228] != 0.0)) {
            s.store_scale(1929, 1930, 2.0);
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2228] != 0.0)) {
            s.store_div_ad(1842, A::mul(A::mul(s.ad_value(1839), s.ad_value(1809)), s.ad_value(1929)), A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))));
        }

        s.v[2229] = if ((s.v[1843] - s.v[1842]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2229] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(1843), s.ad_value(1842)));
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2229] != 0.0))) {
            let assign46700_ad_e59842: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1929, &assign46700_ad_e59842);
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_add_ad_rhs(1952, 1932, A::mul(s.ad_value(1808), A::sub(A::scale(s.ad_value(1843), 0.5), A::ln(A::scale(A::offset(s.ad_value(1929), 1.0), 0.5)))));
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_mul(1953, 233, 1808);
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_add(1954, 1856, 1953);
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_scale_ad(1955, A::sub(s.ad_value(1954), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(1954)), A::neg(s.ad_value(1954))), 0.01))), 0.5);
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_mul_ad_lhs(1946, A::sqrt(A::offset(A::square(s.ad_value(1856)), 1e-6)), 784);
        }

        s.v[2230] = if (s.v[239] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2230] != 0.0)) {
            s.store_scale_ad(1946, A::sub(A::add(s.ad_value(1946), s.ad_value(789)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(789)), A::sub(s.ad_value(1946), s.ad_value(789))), 1e-6))), 0.5);
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_add_ad_rhs(1956, 1846, A::mul(A::sub(A::sub(s.ad_value(1955), s.ad_value(731)), s.ad_value(1952)), s.ad_value(1809)));
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_mul_ad_lhs(1956, A::neg(A::sub(A::add(s.ad_value(814), s.ad_value(1932)), s.ad_value(1952))), 1809);
        }

        s.v[2233] = if (((s.v[1956]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (s.v[2233] != 0.0)) {
            s.store_exp(1929, 1956);
        }

        s.v[2234] = if (s.v[1956] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_div_from_scalar_ad(1929, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1956)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1956)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1956)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2233] != 0.0))) && (!(s.v[2234] != 0.0))) {
            s.store_scale_ad(1929, A::offset(A::mul(A::offset(s.ad_value(1956), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(1956), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(1956), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) {
            s.store_mul_ad_rhs(1929, 786, A::offset(A::mul(s.ad_value(1946), A::add(s.ad_value(238), A::mul(s.ad_value(239), s.ad_value(1946)))), (-1.5)));
        }

        s.v[2237] = if ((s.v[1813] <= 0.0) || ((s.v[238] == 0.0) && (s.v[239] == 0.0))) { 1.0 } else { 0.0 };

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) {
            s.store_add_ad_rhs(1929, 238, A::mul(A::scale(s.ad_value(239), 2.0), s.ad_value(1946)));
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) {
            s.store_div_ad_rhs(1960, 244, A::mul(s.ad_value(1929), s.ad_value(786)));
        }

        if (((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) {
            s.store_scaled_div(1961, 1844, 1960, 0.5);
        }

        s.v[2238] = if (s.v[1961] < 0.001) { 1.0 } else { 0.0 };

        s.v[2239] = if (((s.v[1961]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) && (!(s.v[2238] != 0.0))) && (s.v[2239] != 0.0)) {
            s.store_exp(1969, 1961);
        }

        s.v[2240] = if (s.v[1961] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) && (!(s.v[2238] != 0.0))) && (!(s.v[2239] != 0.0))) && (s.v[2240] != 0.0)) {
            s.store_div_from_scalar_ad(1969, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1961)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1961)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1961)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) && (!(s.v[2238] != 0.0))) && (!(s.v[2239] != 0.0))) && (!(s.v[2240] != 0.0))) {
            s.store_scale_ad(1969, A::offset(A::mul(A::offset(s.ad_value(1961), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(1961), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(1961), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) && (!(s.v[2238] != 0.0))) {
            s.store_div_from_scalar(1970, 1.0, 1969);
        }

        if ((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) && (!(s.v[2238] != 0.0))) {
            s.store_sub(1929, 1969, 1970);
        }

        if ((((s.v[2218] != 0.0) && (s.v[2227] != 0.0)) && (!(s.v[2237] != 0.0))) && (!(s.v[2238] != 0.0))) {
            s.store_add(1931, 1969, 1970);
        }

        s.v[2241] = if (p.p42 != 0.0) { 1.0 } else { 0.0 };

        s.v[2242] = if ((s.v[246] > 0.0) && (s.v[1863] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) {
            s.store_sqrt_ad(1973, A::offset(A::add(A::square(s.ad_value(1863)), A::mul(A::square(s.ad_value(252)), A::square(s.ad_value(825)))), 1e-6));
        }

        if ((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) {
            s.store_div_ad_lhs(1929, A::neg(s.ad_value(796)), 1973);
        }

        s.v[2243] = if (s.v[1929] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (s.v[2243] != 0.0)) {
            s.store_exp(1931, 1929);
        }

        if (((s.v[2241] != 0.0) && (s.v[2242] != 0.0)) && (!(s.v[2243] != 0.0))) {
            s.store_div_from_scalar_ad(1931, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2244] = if ((s.v[245] > 0.0) && (s.v[1862] < 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2241] != 0.0) && (s.v[2244] != 0.0)) {
            s.store_sqrt_ad(1974, A::offset(A::add(A::square(s.ad_value(1862)), A::mul(A::square(s.ad_value(251)), A::square(s.ad_value(824)))), 1e-6));
        }

        if ((s.v[2241] != 0.0) && (s.v[2244] != 0.0)) {
            s.store_div_ad_lhs(1929, A::neg(s.ad_value(795)), 1974);
        }

        s.v[2245] = if (s.v[1929] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2241] != 0.0) && (s.v[2244] != 0.0)) && (s.v[2245] != 0.0)) {
            s.store_exp(1931, 1929);
        }

        if (((s.v[2241] != 0.0) && (s.v[2244] != 0.0)) && (!(s.v[2245] != 0.0))) {
            s.store_div_from_scalar_ad(1931, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.copy_ad(1978, 1916);

        s.v[1864] = 0.0;

        s.v[1865] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 1e-40;

        s.v[1868] = 1.0;

        s.v[835] = 0.0;

        s.v[2246] = if ((p.p46 != 0.0) && (s.v[285] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2246] != 0.0) {
            s.store_add_ad_lhs(1929, A::scale(A::sub(A::add(s.ad_value(817), s.ad_value(816)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(753)))), 0.5), 751);
        }

        if (s.v[2246] != 0.0) {
            s.store_add_ad_lhs(1975, A::sub(s.ad_value(816), A::scale(A::sub(s.ad_value(1929), A::sqrt(A::add(A::mul(s.ad_value(1929), s.ad_value(1929)), s.ad_value(752)))), 0.5)), 755);
        }

        if (s.v[2246] != 0.0) {
            s.store_add_ad_rhs(1976, 1975, A::scale(A::sub(s.ad_value(815), s.ad_value(819)), 0.5));
        }

        if (s.v[2246] != 0.0) {
            s.store_mul_ad(1977, A::mul(s.ad_value(287), A::offset(A::mul(s.ad_value(289), s.ad_value(819)), 1.0)), A::offset(A::mul(s.ad_value(288), s.ad_value(1976)), 1.0));
        }

        if (s.v[2246] != 0.0) {
            s.store_mul_ad_rhs(1978, 1924, A::offset(s.ad_value(1977), 1.0));
        }

        if (s.v[2246] != 0.0) {
            s.store_div_from_scalar(1979, 1.0, 1978);
        }

        if (s.v[2246] != 0.0) {
            s.store_div_ad(1980, A::scale(s.ad_value(819), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(291), s.ad_value(819)), 1.0)), 1.0));
        }

        if (s.v[2246] != 0.0) {
            s.store_mul_ad(1981, A::mul(s.ad_value(290), s.ad_value(1980)), A::offset(A::mul(s.ad_value(292), s.ad_value(1976)), 1.0));
        }

        if (s.v[2246] != 0.0) {
            s.store_mul_ad_rhs(1864, 1979, A::sub(A::add(s.ad_value(818), s.ad_value(1981)), s.ad_value(714)));
        }

        if (s.v[2246] != 0.0) {
            s.store_mul(1982, 1979, 749);
        }

        if (s.v[2246] != 0.0) {
            s.store_scale_ad(1983, A::ln(A::add(A::div(s.ad_value(1982), s.ad_value(750)), A::sqrt(s.ad_value(1982)))), 2.0);
        }

        if (s.v[2246] != 0.0) {
            s.store_mul(1984, 1979, 1975);
        }

        if (s.v[2246] != 0.0) {
            s.store_add(1989, 1982, 1984);
        }

        if (s.v[2246] != 0.0) {
            s.store_add_ad_rhs(1990, 1989, A::mul(s.ad_value(750), A::sqrt(s.ad_value(1989))));
        }

        if (s.v[2246] != 0.0) {
            s.store_add(1991, 1990, 1983);
        }

        if (s.v[2246] != 0.0) {
            s.store_offset_ad(1992, A::div(s.ad_value(750), A::scale(A::sqrt(s.ad_value(1989)), 2.0)), 1.0);
        }

        if (s.v[2246] != 0.0) {
            s.store_div_from_scalar(1993, 1.0, 1992);
        }

        if (s.v[2246] != 0.0) {
            s.store_sub(1994, 1864, 1991);
        }

        s.v[2247] = if (s.v[1994] > (-12.0)) { 1.0 } else { 0.0 };

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_offset_ad(1995, A::add(s.ad_value(1994), s.ad_value(1926)), (-1.0));
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_scale_ad(1996, A::add(s.ad_value(1995), A::sqrt(A::offset(A::square(s.ad_value(1995)), 10.0))), 0.5);
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_add_ad_lhs(1997, A::sub(s.ad_value(1994), A::mul(s.ad_value(1992), A::ln(s.ad_value(1996)))), 1926);
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_scale_ad(1998, A::add(s.ad_value(1997), A::sqrt(A::offset(A::square(s.ad_value(1997)), 2.0))), 0.5);
        }

        s.v[2248] = if ((s.v[1994] - s.v[1998]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) && (s.v[2248] != 0.0)) {
            s.store_exp_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)));
        }

        if (((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) && (!(s.v[2248] != 0.0))) {
            s.store_scale_ad(1999, A::offset(A::mul(A::offset(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_mul(2000, 1925, 1999);
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
        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_ad(2001, &A::pow(s.ad_value(2000), s.ad_value(1993)));
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_add_ad(2002, A::square(s.ad_value(1992)), A::mul(A::sub(A::scale(A::add(s.ad_value(1998), s.ad_value(1992)), 2.0), s.ad_value(2001)), s.ad_value(2001)));
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_mul_ad_rhs(2003, 1992, A::offset(A::div(A::sub(A::sqrt(s.ad_value(2002)), s.ad_value(1992)), s.ad_value(2001)), (-1.0)));
        }

        if ((s.v[2246] != 0.0) && (s.v[2247] != 0.0)) {
            s.store_sub(1985, 1998, 2003);
        }

        s.v[2249] = if ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2246] != 0.0) && (!(s.v[2247] != 0.0))) && (s.v[2249] != 0.0)) {
            s.store_exp_ad(1985, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2247] != 0.0))) && (!(s.v[2249] != 0.0))) {
            let assign47850_ad_e61350: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            s.store_div_from_scalar_ad(1985, 1e-100, A::offset(assign47850_ad_e61350, 1.0));
        }

        if (s.v[2246] != 0.0) {
            s.store_mul_ad_rhs(1986, 1979, A::add(s.ad_value(1841), s.ad_value(1975)));
        }

        s.v[2250] = if ((s.v[1985] < 0.001) && (s.v[1841] < 1e-6)) { 1.0 } else { 0.0 };

        s.v[2251] = if (((-s.v[1986]) + s.v[1984]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2246] != 0.0) && (s.v[2250] != 0.0)) && (s.v[2251] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(1984), s.ad_value(1986)));
        }

        if (((s.v[2246] != 0.0) && (s.v[2250] != 0.0)) && (!(s.v[2251] != 0.0))) {
            let assign47900_ad_e61429: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1929, &assign47900_ad_e61429);
        }

        if ((s.v[2246] != 0.0) && (s.v[2250] != 0.0)) {
            s.store_mul_ad_rhs(1865, 1985, A::offset(s.ad_value(1929), (-1.0)));
        }

        if ((s.v[2246] != 0.0) && (s.v[2250] != 0.0)) {
            s.store_add(1987, 1865, 1985);
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_add(1989, 1982, 1986);
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_add_ad_rhs(1990, 1989, A::mul(s.ad_value(750), A::sqrt(s.ad_value(1989))));
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_add(1991, 1990, 1983);
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_offset_ad(1992, A::div(s.ad_value(750), A::scale(A::sqrt(s.ad_value(1989)), 2.0)), 1.0);
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_div_from_scalar(1993, 1.0, 1992);
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_sub(1994, 1864, 1991);
        }

        s.v[2252] = if (s.v[1994] > (-12.0)) { 1.0 } else { 0.0 };

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_offset_ad(1995, A::add(s.ad_value(1994), s.ad_value(1926)), (-1.0));
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_scale_ad(1996, A::add(s.ad_value(1995), A::sqrt(A::offset(A::square(s.ad_value(1995)), 10.0))), 0.5);
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_add_ad_lhs(1997, A::sub(s.ad_value(1994), A::mul(s.ad_value(1992), A::ln(s.ad_value(1996)))), 1926);
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_scale_ad(1998, A::add(s.ad_value(1997), A::sqrt(A::offset(A::square(s.ad_value(1997)), 2.0))), 0.5);
        }

        s.v[2253] = if ((s.v[1994] - s.v[1998]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) && (s.v[2253] != 0.0)) {
            s.store_exp_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)));
        }

        if ((((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) && (!(s.v[2253] != 0.0))) {
            s.store_scale_ad(1999, A::offset(A::mul(A::offset(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_mul(2000, 1925, 1999);
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_ad(2001, &A::pow(s.ad_value(2000), s.ad_value(1993)));
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_add_ad(2002, A::square(s.ad_value(1992)), A::mul(A::sub(A::scale(A::add(s.ad_value(1998), s.ad_value(1992)), 2.0), s.ad_value(2001)), s.ad_value(2001)));
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_mul_ad_rhs(2003, 1992, A::offset(A::div(A::sub(A::sqrt(s.ad_value(2002)), s.ad_value(1992)), s.ad_value(2001)), (-1.0)));
        }

        if (((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (s.v[2252] != 0.0)) {
            s.store_sub(1987, 1998, 2003);
        }

        s.v[2254] = if ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (!(s.v[2252] != 0.0))) && (s.v[2254] != 0.0)) {
            s.store_exp_ad(1987, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if ((((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) && (!(s.v[2252] != 0.0))) && (!(s.v[2254] != 0.0))) {
            let assign48140_ad_e61782: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            s.store_div_from_scalar_ad(1987, 1e-100, A::offset(assign48140_ad_e61782, 1.0));
        }

        if ((s.v[2246] != 0.0) && (!(s.v[2250] != 0.0))) {
            s.store_sub(1865, 1987, 1985);
        }

        if (s.v[2246] != 0.0) {
            s.store_scaled_add(1866, 1987, 1985, 0.5);
        }

        if (s.v[2246] != 0.0) {
            s.store_ad(1867, &{
                if ((s.v[1864] - s.v[1866]) > 1e-40) {
                    A::sub(s.ad_value(1864), s.ad_value(1866))
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.v[2246] != 0.0) {
            s.store_sub_from_scalar_ad(1868, 1.0, A::div(A::scale(s.ad_value(750), 0.5), A::sqrt(A::add(s.ad_value(1867), A::scale(s.ad_value(1925), 0.25)))));
        }

        if (s.v[2246] != 0.0) {
            s.store_div_ad_lhs(835, A::mul(A::mul(A::mul(A::mul(A::neg(s.ad_value(1918)), s.ad_value(1978)), s.ad_value(1978)), A::offset(A::mul(s.ad_value(1868), s.ad_value(1866)), 1.0)), s.ad_value(1865)), 1853);
        }

        s.v[1869] = 0.0;

        s.v[836] = 0.0;

        s.v[2255] = if ((s.v[1813] > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2255] != 0.0) {
            s.store_sub_ad_rhs(1988, 815, A::mul(s.ad_value(230), s.ad_value(1844)));
        }

        s.v[2256] = if (s.v[1988] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) {
            s.store_mul_ad_rhs(1931, 713, A::div(A::offset(A::mul(s.ad_value(231), A::sub(A::sqrt(A::add(s.ad_value(717), s.ad_value(1932))), s.ad_value(725))), 1.0), A::offset(s.ad_value(1988), 1e-30)));
        }

        s.v[2257] = if ((((-s.v[1931])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (s.v[2257] != 0.0)) {
            s.store_exp_ad(1929, A::neg(s.ad_value(1931)));
        }

        s.v[2258] = if ((-s.v[1931]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (!(s.v[2257] != 0.0))) && (s.v[2258] != 0.0)) {
            s.store_div_from_scalar_ad(1929, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1931))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1931))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1931))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (!(s.v[2257] != 0.0))) && (!(s.v[2258] != 0.0))) {
            s.store_scale_ad(1929, A::offset(A::mul(A::offset(A::neg(s.ad_value(1931)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1931)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1931)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) {
            s.store_mul_ad_rhs(1869, 227, A::mul(s.ad_value(1988), s.ad_value(1929)));
        }

        if ((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) {
            s.store_mul_ad_rhs(836, 1869, A::add(s.ad_value(827), s.ad_value(835)));
        }

        s.v[2259] = if (s.v[836] > (0.5 * s.v[232])) { 1.0 } else { 0.0 };

        if (((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (s.v[2259] != 0.0)) {
            s.store_offset_ad(1929, A::div(A::scale(s.ad_value(836), 2.0), s.ad_value(232)), (-1.0));
        }

        if (((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (s.v[2259] != 0.0)) {
            s.store_mul_ad(836, A::scale(s.ad_value(232), 0.5), A::offset(A::div(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1.0))), 1.0));
        }

        s.v[2453] = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        s.v[2454] = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2294, 717);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2295, 727);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2296, 718);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2297, 1804);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2298, 1805);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2302, 0.0);
        }

        s.v[2455] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.store_add_ad_lhs(2297, A::scale(A::sub(A::add(s.ad_value(817), s.ad_value(816)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(738)))), 0.5), 736);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.store_add_ad_lhs(1870, A::sub(s.ad_value(816), A::scale(A::sub(s.ad_value(2297), A::sqrt(A::add(A::mul(s.ad_value(2297), s.ad_value(2297)), s.ad_value(737)))), 0.5)), 739);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2298, 1870);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2294, 734);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2295, 737);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2296, 735);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub_ad_lhs(2301, A::sub(s.ad_value(818), s.ad_value(2302)), 701);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_add_ad_rhs(2303, 2298, A::scale(A::sub(s.ad_value(815), s.ad_value(819)), 0.5));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2315, 1.0);
        }

        s.v[2456] = if (s.v[188] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul(2306, 2294, 362);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul(2307, 2303, 362);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul(2308, 2301, 362);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_offset_ad(1930, A::div(A::scale(s.ad_value(2296), 0.5), A::sqrt(s.ad_value(2306))), 1.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_add_ad_rhs(1931, 2306, A::mul(s.ad_value(2296), A::sqrt(s.ad_value(2306))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_sub_ad(2309, A::add(A::div(A::sub(s.ad_value(2308), s.ad_value(1931)), s.ad_value(1930)), A::scale(s.ad_value(2306), 0.5)), A::mul(A::offset(s.ad_value(189), 1.0), s.ad_value(2307)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_offset_scaled(2310, 2306, 0.5, 2.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_add(2311, 2306, 2307);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_sub_ad(1930, A::sub(A::sub(s.ad_value(2308), s.ad_value(2311)), A::mul(s.ad_value(2296), A::sqrt(s.ad_value(2311)))), A::scale(A::ln(A::add(A::div(s.ad_value(2306), s.ad_value(2296)), A::sqrt(s.ad_value(2306)))), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_add_ad_lhs(2312, A::scale(s.ad_value(1930), 2.0), 2310);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(1930, A::add(A::add(s.ad_value(2309), s.ad_value(2312)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2309), s.ad_value(2312)), A::sub(s.ad_value(2309), s.ad_value(2312))), 20.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_sub_ad_lhs(1931, A::scale(A::sub(s.ad_value(2308), s.ad_value(2307)), 2.0), 2310);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(2313, A::sub(A::add(s.ad_value(1930), s.ad_value(1931)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(1930, A::sub(A::add(s.ad_value(2313), s.ad_value(2310)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2313), s.ad_value(2310)), A::sub(s.ad_value(2313), s.ad_value(2310))), 5.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(2314, A::add(A::sub(s.ad_value(1930), s.ad_value(2310)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), A::neg(s.ad_value(2310))), A::sub(s.ad_value(1930), A::neg(s.ad_value(2310)))), 20.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul_ad_rhs(1931, 703, A::offset(A::div(s.ad_value(2314), s.ad_value(2310)), 1.0));
        }

        s.v[2457] = if (s.v[1931] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_exp(2315, 1931);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) && (!(s.v[2457] != 0.0))) {
            s.store_div_from_scalar_ad(2315, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_offset_ad(2316, A::mul(s.ad_value(702), s.ad_value(2315)), 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2317, 1916, 2316);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2318, A::mul(s.ad_value(197), A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0)), A::offset(A::mul(s.ad_value(198), s.ad_value(2303)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad_rhs(2319, 2317, A::offset(s.ad_value(2318), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_from_scalar(2320, 1.0, 2319);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad_rhs(2304, 2296, A::sqrt(A::mul(s.ad_value(1916), s.ad_value(2320))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_square(2305, 2304);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_from_scalar(2321, 1.0, 2305);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2322, 2298, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2323, 2301, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_ad(2324, A::scale(s.ad_value(819), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2325, A::mul(s.ad_value(194), s.ad_value(2324)), A::offset(A::mul(s.ad_value(196), s.ad_value(2303)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2326, 2294, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sqrt_ad(1930, A::add(A::square(s.ad_value(2297)), s.ad_value(2295)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sqrt_ad(1931, A::add(A::mul(A::sub(s.ad_value(2297), s.ad_value(2325)), A::sub(s.ad_value(2297), s.ad_value(2325))), s.ad_value(2295)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2327, A::scale(s.ad_value(2320), 0.5), A::sub(A::add(s.ad_value(2325), s.ad_value(1930)), s.ad_value(1931)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_add(2328, 2326, 2322);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub(2329, 2328, 2327);
        }

        s.v[2458] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2459] = if (((s.v[2329]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (s.v[2459] != 0.0)) {
            s.store_offset_ad(2330, A::mul(s.ad_value(2304), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2329), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.3125))))), 1.0);
        }

        s.v[2460] = if (s.v[2329] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) && (s.v[2460] != 0.0)) {
            s.store_exp_ad(2344, A::neg(s.ad_value(2329)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) && (!(s.v[2460] != 0.0))) {
            s.store_div_from_scalar_ad(2344, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2329), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) {
            s.store_scalar(1929, (if (s.v[2329] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) {
            s.store_offset_ad(2330, A::div(A::mul(A::mul(s.ad_value(1929), s.ad_value(2304)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2344), A::sub_from_scalar(1.0, s.ad_value(2329))))), A::scale(A::sqrt(A::mul(s.ad_value(2329), A::sub_from_scalar(1.0, s.ad_value(2344)))), 2.0)), 1.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2458] != 0.0))) {
            s.store_offset_ad(2330, A::div(A::scale(s.ad_value(2304), 0.5), A::sqrt(s.ad_value(2329))), 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub_ad(2331, A::add(s.ad_value(2329), A::mul(s.ad_value(2304), A::sqrt(s.ad_value(2329)))), A::mul(s.ad_value(2330), A::ln(A::offset(s.ad_value(2330), (-1.0)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_ad_lhs(2332, A::sub(s.ad_value(2323), s.ad_value(2331)), 2330);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2338, A::scale(s.ad_value(2305), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2305)), 1.0)), (-1.0)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2337, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2339, 1.0);
        }

        s.v[2461] = if (s.v[2332] > (-30.0)) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_offset_ad(2333, A::mul(s.ad_value(2330), s.ad_value(2332)), (-1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_scale_ad(1929, A::add(s.ad_value(2333), A::sqrt(A::offset(A::square(s.ad_value(2333)), 10.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_sub_ad_rhs(2334, 2332, A::ln(s.ad_value(1929)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_scale_ad(2335, A::add(s.ad_value(2334), A::sqrt(A::offset(A::square(s.ad_value(2334)), 2.0))), 0.5);
        }

        s.v[2462] = if ((s.v[2332] - s.v[2335]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2462] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(2332), s.ad_value(2335)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2462] != 0.0))) {
            s.store_scale_ad(1929, A::offset(A::mul(A::offset(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div(2336, 1929, 2330);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_sub_ad_lhs(1929, A::scale(A::offset(s.ad_value(2335), 1.0), 2.0), 2336);
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
        s.v[2463] = if (s.v[2336] > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2463] != 0.0)) {
            s.store_mul_ad_rhs(2337, 2330, A::offset(A::sub(s.ad_value(2335), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2336), s.ad_value(1929)), 1.0)), (-1.0)), s.ad_value(2336))), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2463] != 0.0))) {
            s.store_mul_ad(2337, A::mul(A::scale(s.ad_value(2330), 0.5), s.ad_value(2336)), A::offset(A::mul(A::scale(s.ad_value(1929), 0.25), s.ad_value(1929)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_scale_ad(1929, A::add(A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0)), A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0))), 1.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad(2338, A::scale(s.ad_value(2305), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2305)), s.ad_value(1929)), 1.0)), (-1.0)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div_ad_rhs(2339, 2338, A::add(s.ad_value(2338), s.ad_value(2337)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_sub_ad_rhs(2329, 2328, A::mul(s.ad_value(2339), s.ad_value(2327)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_offset_scaled(2340, 2304, 0.7071067811865475, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scale(2341, 2340, 1e-5);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_from_scalar(2342, 1.0, 2340);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2449, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2343, 0.0);
        }

        s.v[2464] = if (s.v[2329] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2464] != 0.0)) {
            s.store_exp_ad(2344, A::neg(s.ad_value(2329)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2464] != 0.0))) {
            s.store_div_from_scalar_ad(2344, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2329), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2465] = if (((s.v[2323]) as f64).abs() <= s.v[2341]) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2465] != 0.0)) {
            s.store_scale_ad(2429, A::square(s.ad_value(2342)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2465] != 0.0)) {
            s.store_mul_ad(2343, A::mul(s.ad_value(2323), s.ad_value(2342)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2323), A::sub_from_scalar(1.0, s.ad_value(2344))), s.ad_value(2304)), s.ad_value(2429)), 1.0));
        }

        s.v[2466] = if (s.v[2323] < (-s.v[2341])) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_neg(2431, 2323);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_scaled_mul(2432, 2431, 2342, 1.25);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_scale_ad(2433, A::sub(A::offset(s.ad_value(2432), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2432), (-6.0)), A::offset(s.ad_value(2432), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub(2428, 2431, 2433);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad(2434, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::offset(s.ad_value(2433), 1.0)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad_lhs(2435, A::scale(s.ad_value(2428), 2.0), 2305);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad_lhs(2436, A::ln(A::mul(s.ad_value(2434), s.ad_value(2321))), 2433);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add(813, 2434, 2435);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2436), A::sub(A::scale(A::square(s.ad_value(2435)), 0.5), s.ad_value(2434))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad_rhs(2437, 2433, A::div(A::mul(A::mul(s.ad_value(2434), s.ad_value(813)), s.ad_value(2436)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436)), s.ad_value(2436)), s.ad_value(2435)), A::sub(A::scale(A::square(s.ad_value(2435)), 0.3333333333333333), s.ad_value(2434))))));
        }

        s.v[2467] = if (s.v[2437] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) && (s.v[2467] != 0.0)) {
            s.store_exp(2438, 2437);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) && (!(s.v[2467] != 0.0))) {
            s.store_scale_ad(2438, A::offset(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2437), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_div_from_scalar(2439, 1.0, 2438);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_div_from_scalar_ad(2428, 1.0, A::offset(A::square(s.ad_value(2437)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2437)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2437), s.ad_value(2428)), s.ad_value(2428)), 4.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2428), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2428)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub(2428, 2431, 2437);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_mul(2429, 2344, 2439);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad(2443, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::add(A::sub(A::offset(s.ad_value(2438), (-1.0)), s.ad_value(2429)), A::mul(s.ad_value(2344), A::sub_from_scalar(1.0, s.ad_value(2441))))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad(2444, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::add(A::add(A::offset(A::sub(s.ad_value(2438), s.ad_value(2437)), (-1.0)), s.ad_value(2429)), A::mul(s.ad_value(2344), A::sub(A::offset(s.ad_value(2437), (-1.0)), s.ad_value(2440))))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2438), s.ad_value(2429)), A::mul(s.ad_value(2344), s.ad_value(2442)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad(2428, A::square(s.ad_value(2443)), A::scale(A::mul(s.ad_value(2444), s.ad_value(2428)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad(2343, A::neg(s.ad_value(2437)), A::scale(A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_div_from_scalar_ad(2445, 1.0, A::offset(A::scale(s.ad_value(2304), 0.7324648775608221), 1.25));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2446, A::offset(A::mul(A::scale(s.ad_value(2340), 1.25), s.ad_value(2445)), (-1.0)), 2445);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad(2447, A::mul(s.ad_value(2323), s.ad_value(2342)), A::offset(A::mul(s.ad_value(2446), s.ad_value(2323)), 1.0));
        }

        s.v[2468] = if ((-s.v[2447]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2468] != 0.0)) {
            s.store_exp_ad(2428, A::neg(s.ad_value(2447)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2468] != 0.0))) {
            s.store_div_from_scalar_ad(2428, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2447))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2447))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2447))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_from_scalar(2448, 1.0, 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2449, A::add(s.ad_value(2323), A::scale(s.ad_value(2305), 0.5)), A::mul(s.ad_value(2304), A::sqrt(A::sub(A::add(s.ad_value(2323), A::scale(s.ad_value(2305), 0.25)), s.ad_value(2448)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_offset(2450, 2329, 3.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2433, A::scale(A::sub(A::add(s.ad_value(2449), s.ad_value(2450)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2450), A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0))), 0.5));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub(2428, 2323, 2433);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_exp_ad(2429, A::neg(s.ad_value(2433)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_div_from_scalar_ad(2430, 1.0, A::offset(A::square(s.ad_value(2433)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2433)), 2430);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2433), s.ad_value(2430)), s.ad_value(2430)), 4.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2430), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2430)), 2430);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            let assign49760_ad_e64162: A = {
                if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2344] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), A::mul(s.ad_value(2344), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440))))))
                }
            };
            s.store_ad(2434, &assign49760_ad_e64162);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_from_scalar_ad(2451, 1.0, A::scale(A::mul(s.ad_value(2305), A::sub(s.ad_value(2429), A::mul(s.ad_value(2344), s.ad_value(2442)))), 0.5));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(2435, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::sub_from_scalar(1.0, s.ad_value(2429)), A::mul(s.ad_value(2344), A::offset(s.ad_value(2441), 1.0)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(2436, A::sub(s.ad_value(2329), s.ad_value(2433)), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add(813, 2434, 2435);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2436), A::sub(A::scale(A::square(s.ad_value(2435)), 0.5), A::mul(s.ad_value(2434), s.ad_value(2451)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            let assign49820_ad_e64309: A = A::add(s.ad_value(2433), A::div(A::mul(A::mul(s.ad_value(2434), s.ad_value(813)), s.ad_value(2436)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436)), s.ad_value(2436)), s.ad_value(2435)), A::sub(A::scale(A::square(s.ad_value(2435)), 0.3333333333333333), A::mul(s.ad_value(2434), s.ad_value(2451)))))));
            s.store_ad(2452, &assign49820_ad_e64309);
        }

        s.v[2469] = if (s.v[2452] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2469] != 0.0)) {
            s.store_exp(2438, 2452);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2469] != 0.0)) {
            s.store_div_from_scalar(2439, 1.0, 2438);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2469] != 0.0)) {
            s.store_mul(2438, 2344, 2438);
        }

        s.v[2470] = if (s.v[2452] > (s.v[2329] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (s.v[2470] != 0.0)) {
            s.store_exp_ad(2438, A::sub(s.ad_value(2452), s.ad_value(2329)));
        }

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (s.v[2470] != 0.0)) {
            s.store_div(2439, 2344, 2438);
        }

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (!(s.v[2470] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (!(s.v[2470] != 0.0))) {
            s.store_div_from_scalar_ad(2439, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2452), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_div_from_scalar_ad(2428, 1.0, A::offset(A::square(s.ad_value(2452)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2452)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2452), s.ad_value(2428)), s.ad_value(2428)), 4.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2428), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2428)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub(2428, 2323, 2452);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(2443, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2439)), s.ad_value(2438)), A::mul(s.ad_value(2344), A::offset(s.ad_value(2441), 1.0)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2444, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::add(A::offset(A::add(s.ad_value(2439), s.ad_value(2452)), (-1.0)), s.ad_value(2438)), A::mul(s.ad_value(2344), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440))))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2439), s.ad_value(2438)), A::mul(s.ad_value(2344), s.ad_value(2442)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2428, A::square(s.ad_value(2443)), A::scale(A::mul(s.ad_value(2444), s.ad_value(2428)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad_rhs(2343, 2452, A::scale(A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2346, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2347, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2348, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2349, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2350, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2351, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2352, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2353, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2354, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub(2355, 2323, 2343);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2356, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2357, 2319, 2355);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2358, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2359, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2363, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2364, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2366, 1.0);
        }

        s.v[2471] = if (s.v[2323] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div_from_scalar_ad(1929, 1.0, A::offset(A::square(s.ad_value(2343)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2345, A::square(s.ad_value(2343)), 1929);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_scale_ad(2346, A::mul(A::mul(s.ad_value(2343), s.ad_value(1929)), s.ad_value(1929)), 4.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2347, A::mul(A::sub(A::scale(s.ad_value(1929), 8.0), A::scale(s.ad_value(2345), 12.0)), s.ad_value(1929)), 1929);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_scalar(2348, 0.0);
        }

        s.v[2472] = if (s.v[2343] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_exp(2348, 2343);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_div_from_scalar(2349, 1.0, 2348);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_mul(2348, 2344, 2348);
        }

        s.v[2473] = if (s.v[2343] > (s.v[2329] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (s.v[2473] != 0.0)) {
            s.store_exp_ad(2348, A::sub(s.ad_value(2343), s.ad_value(2329)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (s.v[2473] != 0.0)) {
            s.store_div(2349, 2344, 2348);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (!(s.v[2473] != 0.0))) {
            s.store_div_from_scalar_ad(2348, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (!(s.v[2473] != 0.0))) {
            s.store_div_from_scalar_ad(2349, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2343), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2343), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2343), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_sub_ad_rhs(2350, 2348, A::mul(s.ad_value(2344), A::add(A::offset(s.ad_value(2343), 1.0), s.ad_value(2345))));
        }

        s.v[2474] = if (s.v[2343] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_scale_ad(2351, A::mul(A::square(s.ad_value(2343)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2343), A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_scale_ad(2350, A::mul(A::mul(A::mul(A::mul(s.ad_value(2344), s.ad_value(2343)), s.ad_value(2343)), s.ad_value(2343)), A::offset(A::scale(s.ad_value(2343), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2343), A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_scaled_mul(2352, 2343, 1929, 0.7071067811865475);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_offset_ad(2353, A::scale(A::div(A::mul(s.ad_value(2304), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.5)), A::scale(A::square(s.ad_value(2343)), 0.16666666666666666))), s.ad_value(1929)), 0.7071067811865475), 1.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_add_ad_lhs(2351, A::offset(s.ad_value(2343), (-1.0)), 2349);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_sqrt(2352, 2351);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_offset_ad(2353, A::scale(A::div(A::mul(s.ad_value(2304), A::sub_from_scalar(1.0, s.ad_value(2349))), s.ad_value(2352)), 0.5), 1.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div_ad(2354, A::offset(A::mul(A::scale(s.ad_value(709), 0.2), s.ad_value(2303)), 1.0), A::offset(A::mul(s.ad_value(709), s.ad_value(2303)), 1.0));
        }

        s.v[2475] = if (s.v[2350] > 1e-100) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_rhs(2355, 2304, A::sqrt(A::add(s.ad_value(2351), s.ad_value(2350))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_div_ad(2356, A::mul(A::mul(s.ad_value(2305), s.ad_value(2350)), s.ad_value(2319)), A::add(s.ad_value(2355), A::mul(s.ad_value(2304), s.ad_value(2352))));
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
        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_lhs(2357, A::mul(s.ad_value(2352), s.ad_value(2304)), 2319);
        }

        s.v[2476] = if (s.v[215] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2476] != 0.0)) {
            s.store_div_from_scalar_ad(2358, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(215), s.ad_value(2303))));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2476] != 0.0))) {
            s.store_offset_ad(2358, A::mul(s.ad_value(215), s.ad_value(2303)), 1.0);
        }

        s.v[2477] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2477] != 0.0)) {
            s.store_sub_from_scalar_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2356)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2477] != 0.0))) {
            s.store_div_from_scalar_ad(2359, 1.0, A::offset(A::mul(s.ad_value(216), s.ad_value(2356)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_lhs(2360, A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), 2356);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_rhs(2361, 763, A::add(s.ad_value(2357), A::mul(s.ad_value(764), s.ad_value(2356))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_ln_ad(1930, A::div(s.ad_value(2351), A::offset(A::add(s.ad_value(2351), s.ad_value(2350)), 1e-14)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_add_ad(2362, A::pow(A::mul(s.ad_value(2361), s.ad_value(705)), s.ad_value(706)), A::mul(s.ad_value(707), A::exp(A::mul(A::scale(s.ad_value(708), 0.5), s.ad_value(1930)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_lhs(2363, A::add(A::offset(s.ad_value(2362), 1.0), s.ad_value(2360)), 2354);
        }

        s.v[2478] = if (s.v[219] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_div_from_scalar_ad(2364, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(219), s.ad_value(2303))));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2478] != 0.0))) {
            s.store_offset_ad(2364, A::mul(s.ad_value(219), s.ad_value(2303)), 1.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul(1931, 2356, 2364);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_div_ad_rhs(2365, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.v[2479] = if (s.v[220] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2479] != 0.0)) {
            s.store_div_from_scalar_ad(2366, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(220), s.ad_value(2365))));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2479] != 0.0))) {
            s.store_offset_ad(2366, A::mul(s.ad_value(220), s.ad_value(2365)), 1.0);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2301, 1806);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2303, 1807);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2319, 1808);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2320, 1809);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2304, 1810);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2305, 1811);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2321, 1812);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2323, 1813);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2328, 1814);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2329, 1815);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2340, 1816);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2341, 1817);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2342, 1818);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2449, 1819);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2344, 1820);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2343, 1821);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2346, 1822);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2347, 1823);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2348, 1824);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2349, 1825);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2351, 1826);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2350, 1827);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2352, 1828);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2353, 1829);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2354, 1830);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2355, 1831);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2356, 1832);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2357, 1833);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2358, 1834);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2359, 1835);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2363, 1836);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2364, 1837);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2366, 1838);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2299, 1921);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2300, 766);
        }

        s.v[2480] = if (p.p48 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2453] != 0.0) && (s.v[2480] != 0.0)) {
            s.copy_ad(2299, 1922);
        }

        if ((s.v[2453] != 0.0) && (s.v[2480] != 0.0)) {
            s.copy_ad(2300, 767);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2368, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scale(2367, 2319, 4.60517018598809);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2384, 2367);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2385, 815);
        }

        if (s.v[2453] != 0.0) {
            s.store_mul(2386, 815, 2320);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2390, 2343);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2391, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2394, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2396, 2349);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2397, 2351);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2399, 2350);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2400, 2357);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2401, 2343);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2402, 2349);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2404, 2350);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2405, 2351);
        }

        if (s.v[2453] != 0.0) {
            s.store_sub(2406, 2323, 2343);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2407, 1.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2409, 1.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2408, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2418, 2356);
        }

        if (s.v[2453] != 0.0) {
            s.store_mul(2422, 2406, 2319);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2419, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2420, 2357);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2425, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2424, 1.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2427, 2299);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2426, 2422);
        }

        s.v[2481] = if (s.v[2323] > 0.0) { 1.0 } else { 0.0 };

        s.v[2482] = if (s.v[2350] > 1e-100) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul(2427, 2299, 2366);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div(2368, 2427, 2363);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_add_ad_rhs(2369, 2355, A::scale(s.ad_value(2305), 0.5));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div_ad_lhs(1929, A::div(A::mul(s.ad_value(2305), s.ad_value(2348)), s.ad_value(2369)), 2369);
        }

        s.v[2483] = if (s.v[1929] > 0.0001) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.v[2484] = if (s.v[1930] < 1e-10) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2483] != 0.0)) && (s.v[2484] != 0.0)) {
            s.store_scalar(1931, 1.0);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2483] != 0.0)) && (!(s.v[2484] != 0.0))) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_scale(1931, 1929, 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul(2370, 1931, 2369);
        }

        s.v[2485] = if ((s.v[707] > 0.0) && (s.v[708] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(2371, A::scale(s.ad_value(2319), 0.475), 2370);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_sub_ad_rhs(1929, 2356, A::mul(s.ad_value(2353), s.ad_value(2371)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_scale_ad(2372, A::add(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1e-12))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2373, A::sub(A::mul(s.ad_value(2319), s.ad_value(2355)), s.ad_value(2356)), A::mul(A::offset(s.ad_value(2353), (-1.0)), s.ad_value(2371)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_offset_ad(2374, A::div(A::mul(A::scale(s.ad_value(2305), 0.5), s.ad_value(2319)), s.ad_value(2373)), 1.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_add_ad_rhs(1929, 2373, A::mul(s.ad_value(764), s.ad_value(2372)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_ad(2375, &A::pow(A::mul(A::mul(s.ad_value(763), s.ad_value(1929)), s.ad_value(705)), s.ad_value(706)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(1930, A::div(A::mul(s.ad_value(706), A::offset(A::mul(s.ad_value(2374), A::sub_from_scalar(1.0, s.ad_value(764))), (-1.0))), s.ad_value(1929)), 2375);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_div(1929, 2372, 2373);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2376, 707, A::pow(A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(1931, A::div(A::mul(s.ad_value(708), A::add(A::offset(s.ad_value(2374), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1929), 1.0)))), s.ad_value(2373)), 2376);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(2377, A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), 2372);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_offset_ad(1929, A::div(A::sub(s.ad_value(1930), A::mul(A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), s.ad_value(2374))), s.ad_value(1931)), 1.0);
        }

        s.v[2486] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) && (s.v[2486] != 0.0)) {
            s.store_scale_ad(1930, A::ln(A::offset(A::exp(A::scale(s.ad_value(1929), 2.0)), 1.0)), 0.5);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) && (!(s.v[2486] != 0.0))) {
            s.copy_ad(1930, 1929);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_div_ad(2378, A::mul(A::mul(A::neg(s.ad_value(2371)), s.ad_value(1931)), s.ad_value(1930)), A::add(A::add(A::offset(s.ad_value(2375), 1.0), s.ad_value(2376)), s.ad_value(2377)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2379, 2370, A::offset(A::div(s.ad_value(2378), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2378)), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (!(s.v[2485] != 0.0))) {
            s.copy_ad(2379, 2370);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_scale_ad(2380, A::mul(A::mul(s.ad_value(2319), s.ad_value(2368)), s.ad_value(2379)), 0.7071067811865475);
        }

        s.v[2487] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2487] != 0.0)) {
            s.store_div_ad_rhs(2380, 2380, A::sqrt(A::offset(s.ad_value(2380), 1.0)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div_from_scalar_ad(2381, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2380), 4.0), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul(1929, 2381, 2380);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul_ad(2382, A::mul(s.ad_value(2379), s.ad_value(2381)), A::offset(A::div(A::mul(A::scale(s.ad_value(1929), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(1929), s.ad_value(2381)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(1929), 4.0), s.ad_value(1929)), s.ad_value(2381)), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_scale(2383, 2382, 0.99);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div_ad_lhs(1929, A::mul(A::mul(s.ad_value(2383), A::sub(s.ad_value(2383), A::scale(s.ad_value(2369), 2.0))), s.ad_value(2321)), 2350);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul_ad_rhs(2384, 2319, A::sub(s.ad_value(2383), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2482] != 0.0))) {
            s.copy_ad(2384, 2367);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_offset(1929, 2300, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad_lhs(1930, A::mul(A::sqrt(s.ad_value(1929)), s.ad_value(815)), 2384);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
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
        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scale(1929, 1930, 2.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad(2385, A::mul(s.ad_value(2384), s.ad_value(1929)), A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2386, 2385, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add(2387, 2329, 2386);
        }

        s.v[2488] = if (s.v[2386] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2488] != 0.0)) {
            s.store_exp_ad(2388, A::neg(s.ad_value(2386)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2488] != 0.0))) {
            s.store_div_from_scalar_ad(2388, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2386), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2386), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2386), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2389, 2344, 2388);
        }

        s.v[2489] = if (((s.v[2323]) as f64).abs() <= s.v[2341]) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2489] != 0.0)) {
            s.store_scale_ad(2429, A::square(s.ad_value(2342)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2489] != 0.0)) {
            s.store_mul_ad(2390, A::mul(s.ad_value(2323), s.ad_value(2342)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2323), A::sub_from_scalar(1.0, s.ad_value(2389))), s.ad_value(2304)), s.ad_value(2429)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_offset(2450, 2387, 3.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_ad(2433, A::scale(A::sub(A::add(s.ad_value(2449), s.ad_value(2450)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2450), A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0))), 0.5));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub(2428, 2323, 2433);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_exp_ad(2429, A::neg(s.ad_value(2433)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_div_from_scalar_ad(2430, 1.0, A::offset(A::square(s.ad_value(2433)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2433)), 2430);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2433), s.ad_value(2430)), s.ad_value(2430)), 4.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2430), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2430)), 2430);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            let assign52090_ad_e66961: A = {
                if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2389] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440))))))
                }
            };
            s.store_ad(2434, &assign52090_ad_e66961);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_from_scalar_ad(2451, 1.0, A::scale(A::mul(s.ad_value(2305), A::sub(s.ad_value(2429), A::mul(s.ad_value(2389), s.ad_value(2442)))), 0.5));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(2435, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::sub_from_scalar(1.0, s.ad_value(2429)), A::mul(s.ad_value(2389), A::offset(s.ad_value(2441), 1.0)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(2436, A::sub(s.ad_value(2387), s.ad_value(2433)), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add(813, 2434, 2435);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2436), A::sub(A::scale(A::square(s.ad_value(2435)), 0.5), A::mul(s.ad_value(2434), s.ad_value(2451)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            let assign52150_ad_e67090: A = A::add(s.ad_value(2433), A::div(A::mul(A::mul(s.ad_value(2434), s.ad_value(813)), s.ad_value(2436)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436)), s.ad_value(2436)), s.ad_value(2435)), A::sub(A::scale(A::square(s.ad_value(2435)), 0.3333333333333333), A::mul(s.ad_value(2434), s.ad_value(2451)))))));
            s.store_ad(2452, &assign52150_ad_e67090);
        }

        s.v[2490] = if (s.v[2452] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (s.v[2490] != 0.0)) {
            s.store_exp(2438, 2452);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (s.v[2490] != 0.0)) {
            s.store_div_from_scalar(2439, 1.0, 2438);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (s.v[2490] != 0.0)) {
            s.store_mul(2438, 2389, 2438);
        }

        s.v[2491] = if (s.v[2452] > (s.v[2387] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (s.v[2491] != 0.0)) {
            s.store_exp_ad(2438, A::sub(s.ad_value(2452), s.ad_value(2387)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (s.v[2491] != 0.0)) {
            s.store_div(2439, 2389, 2438);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (!(s.v[2491] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (!(s.v[2491] != 0.0))) {
            s.store_div_from_scalar_ad(2439, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2452), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_div_from_scalar_ad(2428, 1.0, A::offset(A::square(s.ad_value(2452)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2452)), 2428);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2452), s.ad_value(2428)), s.ad_value(2428)), 4.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2428), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2428)), 2428);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub(2428, 2323, 2452);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(2443, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2439)), s.ad_value(2438)), A::mul(s.ad_value(2389), A::offset(s.ad_value(2441), 1.0)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_ad(2444, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::add(A::offset(A::add(s.ad_value(2439), s.ad_value(2452)), (-1.0)), s.ad_value(2438)), A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440))))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2439), s.ad_value(2438)), A::mul(s.ad_value(2389), s.ad_value(2442)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_ad(2428, A::square(s.ad_value(2443)), A::scale(A::mul(s.ad_value(2444), s.ad_value(2428)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad_rhs(2390, 2452, A::scale(A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_sub(2391, 2390, 2343);
        }

        s.v[2492] = if (s.v[2391] < 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_add_ad(2392, A::scale(A::sub(s.ad_value(2323), s.ad_value(2343)), 2.0), A::mul(s.ad_value(2305), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2349)), A::mul(s.ad_value(2348), s.ad_value(2388))), A::mul(s.ad_value(2389), A::offset(s.ad_value(2346), 1.0)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_mul_ad_lhs(2393, A::mul(s.ad_value(2305), A::sub_from_scalar(1.0, s.ad_value(2388))), 2350);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_sub_from_scalar_ad(1929, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2349), A::mul(s.ad_value(2348), s.ad_value(2388))), A::mul(s.ad_value(2389), s.ad_value(2347)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_sub_ad(1929, A::square(s.ad_value(2392)), A::scale(A::mul(s.ad_value(1929), s.ad_value(2393)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_scale_ad(2391, A::div(s.ad_value(2393), A::add(s.ad_value(2392), A::sqrt(s.ad_value(1929)))), 2.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_add(2390, 2343, 2391);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2394, 2391, 2319);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad(2395, A::square(s.ad_value(2390)), A::offset(A::square(s.ad_value(2390)), 2.0));
        }

        s.v[2493] = if (s.v[2390] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) {
            s.store_exp_ad(2396, A::neg(s.ad_value(2390)));
        }

        s.v[2494] = if (s.v[2390] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_scale_ad(2397, A::mul(A::square(s.ad_value(2390)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2390), A::sub_from_scalar(1.0, A::scale(s.ad_value(2390), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2390), A::sub_from_scalar(1.0, A::scale(s.ad_value(2390), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_scaled_mul(2398, 2390, 1929, 0.7071067811865475);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_mul_ad(2399, A::mul(A::mul(A::mul(A::scale(s.ad_value(2389), 0.16666666666666666), s.ad_value(2390)), s.ad_value(2390)), s.ad_value(2390)), A::offset(A::scale(s.ad_value(2390), 1.75), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (!(s.v[2494] != 0.0))) {
            s.store_add_ad_lhs(2397, A::offset(s.ad_value(2390), (-1.0)), 2396);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (!(s.v[2494] != 0.0))) {
            s.store_sqrt(2398, 2397);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (!(s.v[2494] != 0.0))) {
            s.store_mul_ad_rhs(2399, 2389, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2396)), s.ad_value(2390)), (-1.0)), s.ad_value(2395)));
        }

        s.v[2495] = if (s.v[2390] > (s.v[2387] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (s.v[2495] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(2390), s.ad_value(2387)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (s.v[2495] != 0.0)) {
            s.store_div(2396, 2389, 1929);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (s.v[2495] != 0.0)) {
            s.store_sub_ad_rhs(2399, 1929, A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (!(s.v[2495] != 0.0))) {
            s.store_div_from_scalar_ad(2396, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2390), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2390), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2390), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (!(s.v[2495] != 0.0))) {
            s.store_div_from_scalar_ad(1929, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (!(s.v[2495] != 0.0))) {
            s.store_sub_ad_rhs(2399, 1929, A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) {
            s.store_add_ad_lhs(2397, A::offset(s.ad_value(2390), (-1.0)), 2396);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) {
            s.store_sqrt(2398, 2397);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2400, A::mul(s.ad_value(2398), s.ad_value(2304)), 2319);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scaled_add(2401, 2343, 2390, 0.5);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scalar(2402, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(1929, 2396, 2349);
        }

        s.v[2496] = if (s.v[1929] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2496] != 0.0)) {
            s.store_sqrt(2402, 1929);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scaled_add(2403, 2350, 2399, 0.5);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2404, 2403, A::scale(A::mul(A::square(s.ad_value(2391)), A::sub(s.ad_value(2402), A::scale(s.ad_value(2321), 2.0))), 0.125));
        }

        s.v[2497] = if (s.v[2401] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_scale_ad(2405, A::mul(A::square(s.ad_value(2401)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2401), A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_mul_ad_rhs(2406, 2304, A::sqrt(A::add(s.ad_value(2404), s.ad_value(2405))));
        }

        s.v[2498] = if (s.v[719] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) && (s.v[2498] != 0.0)) {
            s.store_div_from_scalar_ad(2407, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2401), A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_scaled_mul(2408, 2401, 1929, 0.7071067811865475);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_add_ad_rhs(2409, 2407, A::scale(A::div(A::mul(s.ad_value(2304), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.5)), A::scale(A::square(s.ad_value(2401)), 0.16666666666666666))), s.ad_value(1929)), 0.7071067811865475));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_add_ad_lhs(2405, A::offset(s.ad_value(2401), (-1.0)), 2402);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_mul_ad_rhs(2406, 2304, A::sqrt(A::add(s.ad_value(2404), s.ad_value(2405))));
        }

        s.v[2499] = if (s.v[719] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad(2410, A::sub_from_scalar(1.0, s.ad_value(2402)), A::scale(A::mul(s.ad_value(2406), s.ad_value(2321)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_from_scalar_ad(2407, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_ad_rhs(1929, 2407, A::offset(s.ad_value(2407), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2411, 719, A::mul(A::mul(A::square(s.ad_value(1929)), s.ad_value(2305)), s.ad_value(2404)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad(2412, A::scale(A::sub(s.ad_value(2406), s.ad_value(2411)), 2.0), A::mul(s.ad_value(2305), A::add(A::sub_from_scalar(1.0, s.ad_value(2402)), s.ad_value(2404))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2413, 2411, A::sub(s.ad_value(2411), A::scale(s.ad_value(2406), 2.0)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_sub_from_scalar_ad(2414, 1.0, A::scale(A::mul(s.ad_value(2305), A::add(s.ad_value(2402), s.ad_value(2404))), 0.5));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_ad(2415, A::mul(s.ad_value(2413), s.ad_value(2412)), A::sub(A::square(s.ad_value(2412)), A::mul(s.ad_value(2414), s.ad_value(2413))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add(2401, 2401, 2415);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_exp(2416, 2415);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div(2402, 2402, 2416);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul(2404, 2404, 2416);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad_lhs(2405, A::offset(s.ad_value(2401), (-1.0)), 2402);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2406, 2304, A::sqrt(A::add(s.ad_value(2404), s.ad_value(2405))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad(2417, A::sub_from_scalar(1.0, s.ad_value(2402)), A::scale(A::mul(A::mul(s.ad_value(2406), s.ad_value(2407)), s.ad_value(2321)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_ad(2391, A::mul(A::mul(s.ad_value(2391), s.ad_value(2416)), A::add(s.ad_value(2410), s.ad_value(2403))), A::add(s.ad_value(2417), A::mul(s.ad_value(2416), s.ad_value(2403))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul(2394, 2391, 2319);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_sqrt(2408, 2405);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_add_ad_rhs(2409, 2407, A::scale(A::div(A::mul(s.ad_value(2304), A::sub_from_scalar(1.0, s.ad_value(2402))), s.ad_value(2408)), 0.5));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_rhs(2418, 2319, A::div(A::mul(s.ad_value(2305), s.ad_value(2404)), A::add(s.ad_value(2406), A::mul(s.ad_value(2304), s.ad_value(2408)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2419, 2418, A::mul(s.ad_value(2319), s.ad_value(2409)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2420, A::mul(s.ad_value(2408), s.ad_value(2304)), 2319);
        }

        s.v[2500] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2500] != 0.0)) {
            s.store_sub_from_scalar_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2418)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2500] != 0.0))) {
            s.store_div_from_scalar_ad(2359, 1.0, A::offset(A::mul(s.ad_value(216), s.ad_value(2418)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2360, A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), 2418);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2421, 2420, A::mul(s.ad_value(764), s.ad_value(2418)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2422, 2420, A::mul(s.ad_value(765), s.ad_value(2418)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2423, 763, 2421);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_ln_ad(1930, A::div(s.ad_value(2405), A::offset(A::add(s.ad_value(2405), s.ad_value(2404)), 1e-14)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad(2362, A::pow(A::mul(s.ad_value(2423), s.ad_value(705)), s.ad_value(706)), A::mul(s.ad_value(707), A::exp(A::mul(A::scale(s.ad_value(708), 0.5), s.ad_value(1930)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2424, A::add(A::offset(s.ad_value(2362), 1.0), s.ad_value(2360)), 2354);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_ln_ad(2425, A::div(A::offset(A::mul(A::sub(s.ad_value(815), s.ad_value(2394)), s.ad_value(768)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2385), s.ad_value(2394)), s.ad_value(768)), 1.0)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(1931, 2418, 2364);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad_rhs(2365, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.v[2501] = if (s.v[220] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2501] != 0.0)) {
            s.store_div_from_scalar_ad(2366, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(220), s.ad_value(2365))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2501] != 0.0))) {
            s.store_offset_ad(2366, A::mul(s.ad_value(220), s.ad_value(2365)), 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2427, 2299, 2366);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2426, 2406, 2319);
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
        if (s.v[2453] != 0.0) {
            s.copy_ad(1871, 2301);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1872, 2319);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1873, 2304);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1874, 2323);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1875, 2328);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1876, 2357);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1877, 2394);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1878, 2400);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1879, 2407);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1880, 2409);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1881, 2418);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1882, 2419);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1883, 2422);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1884, 2424);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1885, 2425);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1886, 2427);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1887, 2426);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(734, 717);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1871, 1806);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1872, 1808);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1873, 1810);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1874, 1813);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1875, 1814);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1876, 1833);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1877, 1844);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1878, 1845);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1879, 1847);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1880, 1848);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1881, 1849);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1882, 1850);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1883, 1852);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1884, 1853);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1885, 1855);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1886, 1854);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1887, 1856);
        }

        s.copy_ad(1888, 253);

        s.v[2502] = if (s.v[762] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2502] != 0.0) {
            s.store_div_ad_rhs(1888, 253, A::offset(A::mul(s.ad_value(762), A::powf(A::add(A::square(s.ad_value(1883)), s.ad_value(722)), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1889] = 1.0;

        s.v[1890] = 1.0;

        s.v[1891] = 0.0;

        s.v[1892] = 1.0;

        s.v[1893] = 1.0;

        s.copy_ad(2265, 1887);

        s.v[2268] = 0.0;

        s.v[2267] = 0.0;

        s.copy_ad(2269, 2265);

        s.v[2503] = if (s.v[1874] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2503] != 0.0) {
            s.store_mul_ad_lhs(2260, A::div(A::mul(A::add(s.ad_value(258), A::div(s.ad_value(259), s.ad_value(1882))), s.ad_value(1881)), s.ad_value(1882)), 1885);
        }

        s.v[2504] = if (s.v[2260] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2503] != 0.0) && (s.v[2504] != 0.0)) {
            s.store_div_from_scalar_ad(1889, 1.0, A::add(A::offset(s.ad_value(2260), 1.0), A::square(s.ad_value(2260))));
        }

        if ((s.v[2503] != 0.0) && (!(s.v[2504] != 0.0))) {
            s.store_sub_from_scalar(1889, 1.0, 2260);
        }

        if (s.v[2503] != 0.0) {
            s.store_mul(1890, 1884, 1889);
        }

        if (s.v[2503] != 0.0) {
            s.store_div(1891, 1886, 1890);
        }

        if (s.v[2503] != 0.0) {
            s.store_mul_ad_lhs(2261, A::mul(A::square(s.ad_value(1891)), s.ad_value(1877)), 1877);
        }

        s.v[2505] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2503] != 0.0) && (s.v[2505] != 0.0)) {
            s.store_div_ad_rhs(2261, 2261, A::offset(A::mul(s.ad_value(1891), s.ad_value(1877)), 1.0));
        }

        if (s.v[2503] != 0.0) {
            s.store_scale_ad(1892, A::mul(s.ad_value(1890), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2261), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2503] != 0.0) {
            s.store_div(1929, 1890, 1892);
        }

        if (s.v[2503] != 0.0) {
            s.store_mul_ad_rhs(2262, 1880, A::offset(A::scale(A::mul(A::mul(s.ad_value(2261), s.ad_value(1929)), s.ad_value(1929)), 0.5), 1.0));
        }

        if (s.v[2503] != 0.0) {
            s.store_div_ad_lhs(1893, A::mul(s.ad_value(1929), s.ad_value(1882)), 2262);
        }

        if (s.v[2503] != 0.0) {
            s.store_scaled_div(2263, 1877, 1893, 0.5);
        }

        if (s.v[2503] != 0.0) {
            s.store_square(2264, 2263);
        }

        if (s.v[2503] != 0.0) {
            s.store_add_ad_rhs(2265, 1887, A::scale(A::mul(A::mul(s.ad_value(1879), s.ad_value(1877)), A::add(A::offset(A::scale(A::mul(s.ad_value(2263), s.ad_value(1889)), 0.3333333333333333), (-1.0)), s.ad_value(1889))), 0.5));
        }

        if (s.v[2503] != 0.0) {
            s.store_scaled_mul(1929, 1880, 1877, 0.16666666666666666);
        }

        s.v[2506] = if (p.p49 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2503] != 0.0) && (s.v[2506] != 0.0)) {
            s.store_scalar(2266, 0.0);
        }

        if ((s.v[2503] != 0.0) && (s.v[2506] != 0.0)) {
            s.store_mul_ad(2267, A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub(s.ad_value(1881), A::mul(A::scale(s.ad_value(1929), 3.0), A::sub_from_scalar(2.0, s.ad_value(2263)))));
        }

        if ((s.v[2503] != 0.0) && (!(s.v[2506] != 0.0))) {
            s.store_mul_ad(2266, A::sub_from_scalar(1.0, s.ad_value(1889)), A::sub(s.ad_value(1881), A::scale(A::mul(s.ad_value(1880), s.ad_value(1877)), 0.5)));
        }

        if ((s.v[2503] != 0.0) && (!(s.v[2506] != 0.0))) {
            s.store_scale_ad(2267, A::add(A::mul(A::square(s.ad_value(1889)), A::sub(s.ad_value(1881), A::mul(s.ad_value(1929), A::sub(A::sub_from_scalar(1.0, s.ad_value(2263)), A::scale(s.ad_value(2264), 0.2))))), A::mul(s.ad_value(2266), A::offset(s.ad_value(1889), 1.0))), 0.5);
        }

        if (s.v[2503] != 0.0) {
            s.store_add_ad_lhs(2268, A::mul(s.ad_value(1889), A::add(s.ad_value(1881), A::mul(s.ad_value(1929), s.ad_value(2263)))), 2266);
        }

        if (s.v[2503] != 0.0) {
            s.store_sub(2269, 2265, 2268);
        }

        s.store_mul(840, 2265, 1888);

        s.store_mul_ad_lhs(842, A::neg(s.ad_value(2267)), 1888);

        s.store_mul_ad_lhs(841, A::neg(s.ad_value(2269)), 1888);

        s.v[2285] = 0.0;

        s.v[2286] = 0.0;

        s.v[2284] = 0.0;

        s.v[2507] = if ((s.v[266] > 0.0) || (s.v[267] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2507] != 0.0) {
            s.store_scalar(2274, 1.0);
        }

        if (s.v[2507] != 0.0) {
            s.copy_ad(2273, 1871);
        }

        s.v[2508] = if (s.v[270] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_add_ad_lhs(2270, A::sub(s.ad_value(1871), s.ad_value(268)), 797);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_scale_ad(1929, A::add(A::add(s.ad_value(2270), s.ad_value(797)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(2270), s.ad_value(797)), A::sub(s.ad_value(2270), s.ad_value(797))), s.ad_value(798)))), 0.5);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_mul_ad_rhs(1930, 1929, A::sub(A::sub(A::scale(s.ad_value(1929), 2.0), s.ad_value(797)), s.ad_value(2270)));
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_div(1931, 797, 1929);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_mul(2271, 2270, 1931);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_sqrt_ad(2272, A::sub_from_scalar(1.0, A::mul(s.ad_value(2271), s.ad_value(270))));
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_sub_ad_lhs(2273, A::add(A::div(A::sub_from_scalar(1.0, s.ad_value(2272)), s.ad_value(270)), s.ad_value(2270)), 2271);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_offset_ad(2274, A::div(A::mul(A::mul(A::offset(A::div_from_scalar(0.5, s.ad_value(2272)), (-1.0)), A::add(s.ad_value(1930), A::mul(s.ad_value(2270), A::sub(s.ad_value(797), s.ad_value(1929))))), s.ad_value(1931)), s.ad_value(1930)), 1.0);
        }

        if (s.v[2507] != 0.0) {
            s.store_scalar(2276, 1.0);
        }

        if (s.v[2507] != 0.0) {
            s.store_scalar(2277, 0.0);
        }

        s.v[2509] = if (s.v[269] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) {
            s.store_add_ad(1929, A::scale(s.ad_value(734), 0.5), A::mul(s.ad_value(1872), A::offset(A::scale(s.ad_value(1873), 0.7071067811865475), 1.0)));
        }

        if ((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) {
            s.store_div(2275, 1871, 1929);
        }

        s.v[2510] = if (((s.v[2275]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (s.v[2510] != 0.0)) {
            s.store_div_from_scalar_ad(2276, 1.0, A::offset(A::exp(A::neg(s.ad_value(2275))), 1.0));
        }

        s.v[2511] = if (s.v[2275] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (!(s.v[2510] != 0.0))) && (s.v[2511] != 0.0)) {
            s.store_div_from_scalar_ad(2276, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2275), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2275), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2275), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2512] = if (s.v[2275] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (s.v[2512] != 0.0)) {
            s.store_ln_ad(1930, A::offset(A::exp(s.ad_value(2275)), 1.0));
        }

        if (((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (!(s.v[2512] != 0.0))) {
            s.copy_ad(1930, 2275);
        }

        if ((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) {
            s.store_mul(2277, 1929, 1930);
        }

        if (s.v[2507] != 0.0) {
            s.store_add_ad_lhs(2278, A::mul(s.ad_value(269), A::sub(s.ad_value(2276), s.ad_value(2274))), 2274);
        }

        if (s.v[2507] != 0.0) {
            s.store_add_ad_lhs(2279, A::mul(s.ad_value(269), A::sub(s.ad_value(2277), s.ad_value(2273))), 2273);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad(2280, A::sub(A::sub(s.ad_value(1871), A::mul(s.ad_value(1872), s.ad_value(1875))), s.ad_value(1887)), A::scale(s.ad_value(1877), 0.5));
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(2281, A::sub(s.ad_value(1871), s.ad_value(2280)), 1876);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(2282, A::add(s.ad_value(1877), s.ad_value(2280)), 815);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(2283, A::sub(s.ad_value(1871), s.ad_value(2282)), 1878);
        }

        s.v[2513] = if (s.v[820] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2507] != 0.0) && (s.v[2513] != 0.0)) {
            s.store_mul_ad_rhs(2284, 2278, A::add(A::mul(s.ad_value(267), s.ad_value(2282)), A::mul(s.ad_value(266), s.ad_value(2280))));
        }

        if ((s.v[2507] != 0.0) && (s.v[2513] != 0.0)) {
            s.store_mul_ad_rhs(2285, 266, A::sub(s.ad_value(2281), s.ad_value(2279)));
        }

        if ((s.v[2507] != 0.0) && (s.v[2513] != 0.0)) {
            s.store_mul_ad_rhs(2286, 267, A::sub(s.ad_value(2283), s.ad_value(2279)));
        }

        if ((s.v[2507] != 0.0) && (!(s.v[2513] != 0.0))) {
            s.store_mul_ad_rhs(2284, 2278, A::add(A::mul(s.ad_value(266), s.ad_value(2282)), A::mul(s.ad_value(267), s.ad_value(2280))));
        }

        if ((s.v[2507] != 0.0) && (!(s.v[2513] != 0.0))) {
            s.store_mul_ad_rhs(2285, 267, A::sub(s.ad_value(2281), s.ad_value(2279)));
        }

        if ((s.v[2507] != 0.0) && (!(s.v[2513] != 0.0))) {
            s.store_mul_ad_rhs(2286, 266, A::sub(s.ad_value(2283), s.ad_value(2279)));
        }

        if (s.v[2507] != 0.0) {
            s.store_add(840, 840, 2284);
        }

        if (s.v[2507] != 0.0) {
            s.store_add(842, 842, 2286);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(841, A::sub(A::sub(s.ad_value(841), s.ad_value(2284)), s.ad_value(2286)), 2285);
        }

        s.store_mul(1894, 260, 1862);

        s.store_mul(1895, 261, 1863);

        s.v[2289] = 0.0;

        s.v[2287] = 0.0;

        s.v[2514] = if ((s.v[260] > 0.0) && (s.v[262] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2514] != 0.0) {
            s.store_mul_ad_rhs(1929, 264, A::add(A::scale(s.ad_value(1803), 0.5), s.ad_value(776)));
        }

        s.v[2515] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2516] = if (s.v[1929] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (s.v[2516] != 0.0)) {
            s.store_exp(2287, 1929);
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (!(s.v[2516] != 0.0))) {
            s.store_div_from_scalar_ad(2287, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2517] = if (s.v[2287] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (s.v[2517] != 0.0)) {
            s.store_ln_ad(2288, A::offset(s.ad_value(2287), 1.0));
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (s.v[2517] != 0.0)) {
            s.store_mul_ad_rhs(1930, 2288, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0))));
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (!(s.v[2517] != 0.0))) {
            s.copy_ad(2288, 2287);
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (!(s.v[2517] != 0.0))) {
            s.store_div_ad(1930, A::scale(s.ad_value(2288), 2.0), A::offset(s.ad_value(2288), 2.0));
        }

        if ((s.v[2514] != 0.0) && (!(s.v[2515] != 0.0))) {
            s.copy_ad(2288, 1929);
        }

        if ((s.v[2514] != 0.0) && (!(s.v[2515] != 0.0))) {
            s.store_mul_ad_rhs(1930, 2288, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0))));
        }

        if (s.v[2514] != 0.0) {
            s.store_mul_ad_lhs(2289, A::scale(A::mul(A::div(A::scale(s.ad_value(262), (-2.0)), s.ad_value(264)), s.ad_value(260)), s.v[355]), 1930);
        }

        s.v[2292] = 0.0;

        s.v[2290] = 0.0;

        s.v[2518] = if ((s.v[261] > 0.0) && (s.v[263] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2518] != 0.0) {
            s.store_mul_ad_rhs(1929, 264, A::add(A::scale(s.ad_value(1803), 0.5), s.ad_value(777)));
        }

        s.v[2519] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2520] = if (s.v[1929] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (s.v[2520] != 0.0)) {
            s.store_exp(2290, 1929);
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (!(s.v[2520] != 0.0))) {
            s.store_div_from_scalar_ad(2290, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2521] = if (s.v[2290] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (s.v[2521] != 0.0)) {
            s.store_ln_ad(2291, A::offset(s.ad_value(2290), 1.0));
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (s.v[2521] != 0.0)) {
            s.store_mul_ad_rhs(1930, 2291, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0))));
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (!(s.v[2521] != 0.0))) {
            s.copy_ad(2291, 2290);
        }

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
        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (!(s.v[2521] != 0.0))) {
            s.store_div_ad(1930, A::scale(s.ad_value(2291), 2.0), A::offset(s.ad_value(2291), 2.0));
        }

        if ((s.v[2518] != 0.0) && (!(s.v[2519] != 0.0))) {
            s.copy_ad(2291, 1929);
        }

        if ((s.v[2518] != 0.0) && (!(s.v[2519] != 0.0))) {
            s.store_mul_ad_rhs(1930, 2291, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0))));
        }

        if (s.v[2518] != 0.0) {
            s.store_mul_ad_lhs(2292, A::scale(A::mul(A::div(A::scale(s.ad_value(263), (-2.0)), s.ad_value(264)), s.ad_value(261)), s.v[355]), 1930);
        }

        s.store_add(2293, 2289, 2292);

        s.store_add_ad_lhs(845, A::mul(s.ad_value(265), s.ad_value(818)), 2293);

        s.store_mul(843, 272, 823);

        s.store_mul(844, 273, 826);

        s.v[2522] = 0.0;

        s.v[2525] = 0.0;

        s.v[2526] = 0.0;

        s.v[2527] = 0.0;

        s.v[2528] = 0.0;

        s.v[2529] = 0.0;

        s.v[2530] = 0.0;

        s.v[2531] = 0.0;

        s.v[2532] = 0.0;

        s.v[2533] = 0.0;

        s.v[2534] = 0.0;

        s.v[2535] = 0.0;

        s.v[2536] = 0.0;

        s.v[2537] = 0.0;

        s.v[2538] = 0.0;

        s.v[2539] = 0.0;

        s.v[2540] = 0.0;

        s.v[2543] = 0.0;

        s.v[2547] = 0.0;

        s.v[2550] = 0.0;

        s.v[2551] = 0.0;

        s.v[2552] = 0.0;

        s.v[2553] = 0.0;

        s.v[2554] = 0.0;

        s.v[2555] = 0.0;

        s.v[2558] = 0.0;

        s.v[2559] = 0.0;

        s.v[2560] = 0.0;

        s.v[2561] = 0.0;

        s.v[2565] = 0.0;

        s.v[2567] = 0.0;

        s.v[2568] = 0.0;

        s.v[846] = 0.0;

        s.v[1902] = 0.0;

        s.v[1903] = 0.0;

        s.v[1904] = 0.0;

        s.v[847] = 0.0;

        s.v[1905] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.v[2569] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[2570] = if (s.v[475] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2573, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2574, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add_ad_rhs(2527, 821, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scale_ad(2574, A::div(A::mul(s.ad_value(821), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2575] = if (s.v[652] > 0.5) { 1.0 } else { 0.0 };

        s.v[2576] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])), s.v[409]);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2575] != 0.0)) {
            s.store_add_ad(1902, A::scale(A::sub_from_scalar(1.0, s.ad_value(2573)), s.v[418]), A::scale(A::sub(s.ad_value(821), s.ad_value(2574)), s.v[421]));
        }

        s.v[2577] = if (s.v[653] > 0.5) { 1.0 } else { 0.0 };

        s.v[2578] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2577] != 0.0)) && (s.v[2578] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2577] != 0.0)) && (!(s.v[2578] != 0.0))) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])), s.v[410]);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2577] != 0.0)) {
            s.store_add_ad(1903, A::scale(A::sub_from_scalar(1.0, s.ad_value(2573)), s.v[419]), A::scale(A::sub(s.ad_value(821), s.ad_value(2574)), s.v[422]));
        }

        s.v[2579] = if (s.v[654] > 0.5) { 1.0 } else { 0.0 };

        s.v[2580] = if (s.v[411] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2579] != 0.0)) && (s.v[2580] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2579] != 0.0)) && (!(s.v[2580] != 0.0))) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])), s.v[411]);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_add_ad(1904, A::scale(A::sub_from_scalar(1.0, s.ad_value(2573)), s.v[420]), A::scale(A::sub(s.ad_value(821), s.ad_value(2574)), s.v[423]));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2573, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2574, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add_ad_rhs(2527, 822, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scale_ad(2574, A::div(A::mul(s.ad_value(822), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2581] = if (s.v[679] > 0.5) { 1.0 } else { 0.0 };

        s.v[2582] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2581] != 0.0)) && (s.v[2582] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(573))));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2581] != 0.0)) && (!(s.v[2582] != 0.0))) {
            s.store_ad(2573, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2581] != 0.0)) {
            s.store_add_ad(1905, A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2573))), A::mul(s.ad_value(588), A::sub(s.ad_value(822), s.ad_value(2574))));
        }

        s.v[2583] = if (s.v[680] > 0.5) { 1.0 } else { 0.0 };

        s.v[2584] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2583] != 0.0)) && (s.v[2584] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(574))));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2583] != 0.0)) && (!(s.v[2584] != 0.0))) {
            s.store_ad(2573, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(574))), s.ad_value(577)));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2583] != 0.0)) {
            s.store_add_ad(1906, A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2573))), A::mul(s.ad_value(589), A::sub(s.ad_value(822), s.ad_value(2574))));
        }

        s.v[2585] = if (s.v[681] > 0.5) { 1.0 } else { 0.0 };

        s.v[2586] = if (s.v[578] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2585] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(575))));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2585] != 0.0)) && (!(s.v[2586] != 0.0))) {
            s.store_ad(2573, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(575))), s.ad_value(578)));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2585] != 0.0)) {
            s.store_add_ad(1907, A::mul(s.ad_value(587), A::sub_from_scalar(1.0, s.ad_value(2573))), A::mul(s.ad_value(590), A::sub(s.ad_value(822), s.ad_value(2574))));
        }

        s.v[2587] = if (p.p889 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_scale_ad(643, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), p.p890), (-(((0.5 * 0.001)) as f64).powf(p.p890))), p.p889);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_offset(641, 643, p.p879);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_div_from_scalar(451, 1.0, 641);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2587] != 0.0))) {
            s.store_scalar(641, p.p879);
        }

        s.v[2588] = if (p.p891 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2588] != 0.0)) {
            s.store_scale_ad(645, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), p.p892), (-(((0.5 * 0.001)) as f64).powf(p.p892))), p.p891);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2588] != 0.0)) {
            s.store_mul_ad_rhs(444, 444, A::offset(s.ad_value(645), 1.0));
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2538, 0.0);
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2535, 0.0);
        }

        s.v[2589] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_add_ad_rhs(2527, 821, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2532, A::div(A::mul(s.ad_value(821), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2590] = if (s.v[821] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[2591] = if (((((-0.5) * (s.v[821] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_exp_ad(2533, A::scale(s.ad_value(821), (s.v[372] * (-0.5))));
        }

        s.v[2592] = if (((-0.5) * (s.v[821] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2592] != 0.0)) {
            let assign56430_ad_e71206: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2533, &assign56430_ad_e71206);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) && (!(s.v[2591] != 0.0))) && (!(s.v[2592] != 0.0))) {
            s.store_scale_ad(2533, A::offset(A::mul(A::offset(A::scale(s.ad_value(821), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(821), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(821), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) {
            s.store_div_from_scalar(2534, 1.0, 2533);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) {
            s.store_square(2531, 2534);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2590] != 0.0))) {
            s.store_mul_ad_lhs(2531, A::offset(A::scale(A::sub(s.ad_value(821), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2590] != 0.0))) {
            s.store_sqrt(2534, 2531);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2590] != 0.0))) {
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.v[2593] = if (s.v[821] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2593] != 0.0)) {
            s.store_scale_ad(2535, A::ln(A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2533), 1.0), A::offset(s.ad_value(2533), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2593] != 0.0))) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2534), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2534), 1.0), A::offset(A::scale(s.ad_value(2534), 3.0), 1.0))))), (s.v[371] * 2.0)), 821);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sub(2536, 657, 2535);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2537, A::sub(A::add(s.ad_value(821), s.ad_value(2536)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(2536)), A::sub(s.ad_value(821), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2538, A::sub(A::add(s.ad_value(821), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(660)), A::sub(s.ad_value(821), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2539, A::sub(s.ad_value(821), A::sqrt(A::offset(A::mul(s.ad_value(821), s.ad_value(821)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2594] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2594] != 0.0)) {
            s.store_scalar(1902, 0.0);
        }

        s.v[2595] = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_sub_from_scalar(2543, s.v[394], 2537);
        }

        s.v[2597] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) && (s.v[2597] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(s.ad_value(2543), s.v[430]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) && (!(s.v[2597] != 0.0))) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[430]), p.p848);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_scale(2547, 2540, s.v[424]);
        }

        s.v[2598] = if (p.p862 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale_ad(2550, A::div(A::scale(s.ad_value(2547), s.v[409]), s.ad_value(2543)), s.v[439]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[436]), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::scale(s.ad_value(2551), s.v[436]), s.ad_value(2554)), A::scale(s.ad_value(2553), s.v[436])), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2601] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2601] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2601] != 0.0))) {
            let assign56980_ad_e72156: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign56980_ad_e72156);
        }

        s.v[2602] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        s.v[2603] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2603] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2603] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2604] = if (p.p868 == 0.0) { 1.0 } else { 0.0 };

        s.v[2605] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

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
        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (s.v[2605] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (!(s.v[2605] != 0.0))) {
            s.store_powf_ad(2540, A::scale(A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]), p.p848);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) {
            s.store_scale_ad(2565, A::div(A::scale(A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[427]), s.ad_value(2540)), s.v[412]);
        }

        s.v[2606] = if (((((-s.v[442]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (s.v[2606] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(442)), s.ad_value(2565)));
        }

        s.v[2607] = if (((-s.v[442]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (!(s.v[2606] != 0.0))) && (s.v[2607] != 0.0)) {
            let assign57170_ad_e72496: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign57170_ad_e72496);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (!(s.v[2606] != 0.0))) && (!(s.v[2607] != 0.0))) {
            let assign57180_ad_e72547: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign57180_ad_e72547);
        }

        s.v[2608] = if (p.p877 > 1000.0) { 1.0 } else { 0.0 };

        s.v[2609] = if (s.v[2539] > ((-s.v[445]) * p.p877)) { 1.0 } else { 0.0 };

        s.v[2610] = if (p.p880 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) && (s.v[2610] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::scale(s.ad_value(2539), s.v[449]), A::scale(s.ad_value(2539), s.v[449])), A::scale(s.ad_value(2539), s.v[449])), A::scale(s.ad_value(2539), s.v[449]));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) && (!(s.v[2610] != 0.0))) {
            s.store_powf_ad(2540, A::abs(A::scale(s.ad_value(2539), s.v[449])), p.p880);
        }

        s.v[2611] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2611] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2611] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])), s.v[409]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) {
            s.store_scale_ad(1902, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[418]), A::scale(A::sub(s.ad_value(821), s.ad_value(2532)), s.v[421])), p.p30);
        }

        s.v[2612] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2612] != 0.0)) {
            s.store_scalar(1903, 0.0);
        }

        s.v[2613] = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_sub_from_scalar(2543, s.v[395], 2537);
        }

        s.v[2615] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) && (s.v[2615] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(s.ad_value(2543), s.v[431]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) && (!(s.v[2615] != 0.0))) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[431]), p.p849);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_scale(2547, 2540, s.v[425]);
        }

        s.v[2616] = if (p.p863 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_scale_ad(2550, A::div(A::scale(s.ad_value(2547), s.v[410]), s.ad_value(2543)), s.v[440]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[437]), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::scale(s.ad_value(2551), s.v[437]), s.ad_value(2554)), A::scale(s.ad_value(2553), s.v[437])), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2619] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2619] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2619] != 0.0))) {
            let assign57730_ad_e73422: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign57730_ad_e73422);
        }

        s.v[2620] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        s.v[2621] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2621] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2621] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2622] = if (p.p869 == 0.0) { 1.0 } else { 0.0 };

        s.v[2623] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (s.v[2623] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (!(s.v[2623] != 0.0))) {
            s.store_powf_ad(2540, A::scale(A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]), p.p849);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) {
            s.store_scale_ad(2565, A::div(A::scale(A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[428]), s.ad_value(2540)), s.v[413]);
        }

        s.v[2624] = if (((((-s.v[443]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (s.v[2624] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(443)), s.ad_value(2565)));
        }

        s.v[2625] = if (((-s.v[443]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (!(s.v[2624] != 0.0))) && (s.v[2625] != 0.0)) {
            let assign57920_ad_e73762: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign57920_ad_e73762);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (!(s.v[2624] != 0.0))) && (!(s.v[2625] != 0.0))) {
            let assign57930_ad_e73813: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign57930_ad_e73813);
        }

        s.v[2626] = if (p.p878 > 1000.0) { 1.0 } else { 0.0 };

        s.v[2627] = if (s.v[2539] > ((-s.v[445]) * p.p878)) { 1.0 } else { 0.0 };

        s.v[2628] = if (p.p881 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) && (s.v[2628] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::scale(s.ad_value(2539), s.v[450]), A::scale(s.ad_value(2539), s.v[450])), A::scale(s.ad_value(2539), s.v[450])), A::scale(s.ad_value(2539), s.v[450]));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) && (!(s.v[2628] != 0.0))) {
            s.store_powf_ad(2540, A::abs(A::scale(s.ad_value(2539), s.v[450])), p.p881);
        }

        s.v[2629] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2629] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2629] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])), s.v[410]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) {
            s.store_scale_ad(1903, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[419]), A::scale(A::sub(s.ad_value(821), s.ad_value(2532)), s.v[422])), p.p30);
        }

        s.v[2630] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2630] != 0.0)) {
            s.store_scalar(1904, 0.0);
        }

        s.v[2631] = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_sub_from_scalar(2543, s.v[396], 2537);
        }

        s.v[2633] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(s.ad_value(2543), s.v[432]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) && (!(s.v[2633] != 0.0))) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[432]), p.p850);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_scale(2547, 2540, s.v[426]);
        }

        s.v[2634] = if (p.p864 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_scale_ad(2550, A::div(A::scale(s.ad_value(2547), s.v[411]), s.ad_value(2543)), s.v[441]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[438]), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::scale(s.ad_value(2551), s.v[438]), s.ad_value(2554)), A::scale(s.ad_value(2553), s.v[438])), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2637] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2637] != 0.0))) {
            let assign58480_ad_e74688: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign58480_ad_e74688);
        }

        s.v[2638] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        s.v[2639] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2638] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2638] != 0.0))) && (!(s.v[2639] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2640] = if (p.p870 == 0.0) { 1.0 } else { 0.0 };

        s.v[2641] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (s.v[2641] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (!(s.v[2641] != 0.0))) {
            s.store_powf_ad(2540, A::scale(A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]), p.p850);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) {
            s.store_scale_ad(2565, A::div(A::scale(A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[429]), s.ad_value(2540)), s.v[414]);
        }

        s.v[2642] = if (((((-s.v[444]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (s.v[2642] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(444)), s.ad_value(2565)));
        }

        s.v[2643] = if (((-s.v[444]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (!(s.v[2642] != 0.0))) && (s.v[2643] != 0.0)) {
            let assign58670_ad_e75028: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign58670_ad_e75028);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (!(s.v[2642] != 0.0))) && (!(s.v[2643] != 0.0))) {
            let assign58680_ad_e75079: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign58680_ad_e75079);
        }

        s.v[2644] = if (s.v[641] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2645] = if (s.v[2539] > ((-s.v[445]) * s.v[641])) { 1.0 } else { 0.0 };

        s.v[2646] = if (p.p882 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2645] != 0.0)) && (s.v[2646] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(451)), A::mul(s.ad_value(2539), s.ad_value(451))), A::mul(s.ad_value(2539), s.ad_value(451))), A::mul(s.ad_value(2539), s.ad_value(451)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2645] != 0.0)) && (!(s.v[2646] != 0.0))) {
            s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);
        }

        s.v[2647] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            let assign58800_ad_e75304: A = {
                if (s.v[821] < p.p887) {
                    {
                        if (((s.v[821] - p.p887) / p.p888) < (-37.0)) {
                            A::constant(p.p887)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(821), (-p.p887)), 1.0 / (p.p888))), 1.0)), p.p888), p.p887)
                        }
                    }
                } else {
                    {
                        if (((s.v[821] - p.p887) / p.p888) > 37.0) {
                            s.ad_value(821)
                        } else {
                            A::add(s.ad_value(821), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p887, s.ad_value(821)), 1.0 / (p.p888))), 1.0)), p.p888))
                        }
                    }
                }
            };
            s.store_ad(2567, &assign58800_ad_e75304);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2648] = if (s.v[411] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (s.v[2648] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (!(s.v[2648] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])), s.v[411]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(1904, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[420]), A::scale(A::sub(s.ad_value(2567), s.ad_value(2568)), s.v[423])), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sub_ad_lhs(2567, A::offset(s.ad_value(821), p.p887), 2567);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2649] = if (s.v[468] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (s.v[2649] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(467))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (!(s.v[2649] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(467))), s.ad_value(468)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(473, A::add(A::mul(s.ad_value(471), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(472), A::sub(s.ad_value(2567), s.ad_value(2568)))), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add(1904, 1904, 473);
        }

        s.v[2650] = if (s.v[411] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2647] != 0.0))) && (s.v[2650] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2647] != 0.0))) && (!(s.v[2650] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2647] != 0.0))) {
            s.store_scale_ad(1904, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[420]), A::scale(A::sub(s.ad_value(821), s.ad_value(2532)), s.v[423])), p.p30);
        }

        s.v[2651] = if (s.v[637] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_mul_ad_rhs(644, 637, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638))));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_add(642, 543, 644);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_div_from_scalar(617, 1.0, 642);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2651] != 0.0))) {
            s.copy_ad(642, 543);
        }

        s.v[2652] = if (s.v[639] > 0.0) { 1.0 } else { 0.0 };

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
        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2652] != 0.0)) {
            s.store_mul_ad_rhs(646, 639, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), s.ad_value(640)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(640))));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2652] != 0.0)) {
            s.store_mul_ad_rhs(611, 611, A::offset(s.ad_value(646), 1.0));
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2538, 0.0);
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2535, 0.0);
        }

        s.v[2653] = if !(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_add_ad_rhs(2527, 822, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2532, A::div(A::mul(s.ad_value(822), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2654] = if (s.v[822] < s.v[682]) { 1.0 } else { 0.0 };

        s.v[2655] = if (((((-0.5) * (s.v[822] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) && (s.v[2655] != 0.0)) {
            s.store_exp_ad(2533, A::scale(s.ad_value(822), (s.v[372] * (-0.5))));
        }

        s.v[2656] = if (((-0.5) * (s.v[822] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) && (!(s.v[2655] != 0.0))) && (s.v[2656] != 0.0)) {
            let assign59330_ad_e76138: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2533, &assign59330_ad_e76138);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) && (!(s.v[2655] != 0.0))) && (!(s.v[2656] != 0.0))) {
            s.store_scale_ad(2533, A::offset(A::mul(A::offset(A::scale(s.ad_value(822), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(822), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(822), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) {
            s.store_div_from_scalar(2534, 1.0, 2533);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) {
            s.store_square(2531, 2534);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2654] != 0.0))) {
            s.store_mul_ad_lhs(2531, A::offset(A::scale(A::sub(s.ad_value(822), s.ad_value(682)), s.v[372]), 1.0), 683);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2654] != 0.0))) {
            s.store_sqrt(2534, 2531);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2654] != 0.0))) {
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.v[2657] = if (s.v[822] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2657] != 0.0)) {
            s.store_scale_ad(2535, A::ln(A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2533), 1.0), A::offset(s.ad_value(2533), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2657] != 0.0))) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2534), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2534), 1.0), A::offset(A::scale(s.ad_value(2534), 3.0), 1.0))))), (s.v[371] * 2.0)), 822);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_sub(2536, 684, 2535);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2537, A::sub(A::add(s.ad_value(822), s.ad_value(2536)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(2536)), A::sub(s.ad_value(822), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2538, A::sub(A::add(s.ad_value(822), s.ad_value(687)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(687)), A::sub(s.ad_value(822), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2539, A::sub(s.ad_value(822), A::sqrt(A::offset(A::mul(s.ad_value(822), s.ad_value(822)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2658] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2658] != 0.0)) {
            s.store_scalar(1905, 0.0);
        }

        s.v[2659] = if ((s.v[523] == 0.0) && (s.v[526] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_sub(2543, 570, 2537);
        }

        s.v[2661] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) && (s.v[2661] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(s.ad_value(2543), s.ad_value(597)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) && (!(s.v[2661] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(s.ad_value(2543), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_mul(2547, 591, 2540);
        }

        s.v[2662] = if (s.v[526] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_rhs(2550, 606, A::div(A::mul(s.ad_value(2547), s.ad_value(576)), s.ad_value(2543)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_div_ad_lhs(2551, A::scale(s.ad_value(603), 0.666666666666667), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::mul(s.ad_value(603), s.ad_value(2551)), s.ad_value(2554)), A::mul(s.ad_value(603), s.ad_value(2553))), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2665] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2665] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2665] != 0.0))) {
            let assign59880_ad_e77088: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign59880_ad_e77088);
        }

        s.v[2666] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        s.v[2667] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2667] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2667] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2668] = if (s.v[532] == 0.0) { 1.0 } else { 0.0 };

        s.v[2669] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (s.v[2669] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (!(s.v[2669] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) {
            s.store_mul_ad_rhs(2565, 579, A::div(A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(594)), s.ad_value(2540)));
        }

        s.v[2670] = if (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (s.v[2670] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(609)), s.ad_value(2565)));
        }

        s.v[2671] = if (((-s.v[609]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (!(s.v[2670] != 0.0))) && (s.v[2671] != 0.0)) {
            let assign60070_ad_e77428: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign60070_ad_e77428);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (!(s.v[2670] != 0.0))) && (!(s.v[2671] != 0.0))) {
            let assign60080_ad_e77479: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign60080_ad_e77479);
        }

        s.v[2672] = if (s.v[541] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2673] = if (s.v[2539] > ((-s.v[445]) * s.v[541])) { 1.0 } else { 0.0 };

        s.v[2674] = if (s.v[544] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) && (s.v[2674] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(615)), A::mul(s.ad_value(2539), s.ad_value(615))), A::mul(s.ad_value(2539), s.ad_value(615))), A::mul(s.ad_value(2539), s.ad_value(615)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) && (!(s.v[2674] != 0.0))) {
            s.store_ad(2540, &A::pow(A::abs(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(544)));
        }

        s.v[2675] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2675] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(573))));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2675] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) {
            s.store_scale_ad(1905, A::add(A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(588), A::sub(s.ad_value(822), s.ad_value(2532)))), p.p30);
        }

        s.v[2676] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2676] != 0.0)) {
            s.store_scalar(1906, 0.0);
        }

        s.v[2677] = if ((s.v[524] == 0.0) && (s.v[527] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_sub(2543, 571, 2537);
        }

        s.v[2679] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) && (s.v[2679] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(s.ad_value(2543), s.ad_value(598)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) && (!(s.v[2679] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(s.ad_value(2543), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_mul(2547, 592, 2540);
        }

        s.v[2680] = if (s.v[527] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_rhs(2550, 607, A::div(A::mul(s.ad_value(2547), s.ad_value(577)), s.ad_value(2543)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_div_ad_lhs(2551, A::scale(s.ad_value(604), 0.666666666666667), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::mul(s.ad_value(604), s.ad_value(2551)), s.ad_value(2554)), A::mul(s.ad_value(604), s.ad_value(2553))), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2683] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2683] != 0.0))) {
            let assign60630_ad_e78354: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign60630_ad_e78354);
        }

        s.v[2684] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        s.v[2685] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2685] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2686] = if (s.v[533] == 0.0) { 1.0 } else { 0.0 };

        s.v[2687] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (s.v[2687] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (!(s.v[2687] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) {
            s.store_mul_ad_rhs(2565, 580, A::div(A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(595)), s.ad_value(2540)));
        }

        s.v[2688] = if (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (s.v[2688] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(610)), s.ad_value(2565)));
        }

        s.v[2689] = if (((-s.v[610]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (!(s.v[2688] != 0.0))) && (s.v[2689] != 0.0)) {
            let assign60820_ad_e78694: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign60820_ad_e78694);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (!(s.v[2688] != 0.0))) && (!(s.v[2689] != 0.0))) {
            let assign60830_ad_e78745: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign60830_ad_e78745);
        }

        s.v[2690] = if (s.v[542] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2691] = if (s.v[2539] > ((-s.v[445]) * s.v[542])) { 1.0 } else { 0.0 };

        s.v[2692] = if (s.v[545] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) && (s.v[2692] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(616)), A::mul(s.ad_value(2539), s.ad_value(616))), A::mul(s.ad_value(2539), s.ad_value(616))), A::mul(s.ad_value(2539), s.ad_value(616)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) && (!(s.v[2692] != 0.0))) {
            s.store_ad(2540, &A::pow(A::abs(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(545)));
        }

        s.v[2693] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2693] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(574))));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2693] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(574))), s.ad_value(577)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) {
            s.store_scale_ad(1906, A::add(A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(589), A::sub(s.ad_value(822), s.ad_value(2532)))), p.p30);
        }

        s.v[2694] = if (s.v[676] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2694] != 0.0)) {
            s.store_scalar(1907, 0.0);
        }

        s.v[2695] = if ((s.v[525] == 0.0) && (s.v[528] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_sub(2543, 572, 2537);
        }

        s.v[2697] = if (s.v[514] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(s.ad_value(2543), s.ad_value(599)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) && (!(s.v[2697] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(s.ad_value(2543), s.ad_value(599)), s.ad_value(514)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_mul(2547, 593, 2540);
        }

        s.v[2698] = if (s.v[528] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul_ad_rhs(2550, 608, A::div(A::mul(s.ad_value(2547), s.ad_value(578)), s.ad_value(2543)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_div_ad_lhs(2551, A::scale(s.ad_value(605), 0.666666666666667), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::mul(s.ad_value(605), s.ad_value(2551)), s.ad_value(2554)), A::mul(s.ad_value(605), s.ad_value(2553))), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2701] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2701] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2701] != 0.0))) {
            let assign61380_ad_e79620: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign61380_ad_e79620);
        }

    }
}
