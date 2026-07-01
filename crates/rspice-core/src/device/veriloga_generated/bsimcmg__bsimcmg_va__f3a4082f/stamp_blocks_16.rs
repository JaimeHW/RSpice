#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_126(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign32180_e54498, assign32180_e54498_d_n3, assign32180_e54498_d_n4, assign32180_e54498_d_n5,) = {
    if (((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign32180_e54495: f64 = (-p.p1613);
        let assign32180_e54496: f64 = (locals.var_arg__blk603).powf(assign32180_e54495);
        (assign32180_e54496, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32180_e54495) as f64).is_finite() && ((assign32180_e54495) as f64).fract() == 0.0 { if assign32180_e54495 == 0.0 { 0.0 } else { (assign32180_e54495 * ((locals.var_arg__blk603).powf(assign32180_e54495 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32180_e54496 * (assign32180_e54495 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32180_e54498;
        locals.var_sarg__blk604_dn3 = assign32180_e54498_d_n3;
        locals.var_sarg__blk604_dn4 = assign32180_e54498_d_n4;
        locals.var_sarg__blk604_dn5 = assign32180_e54498_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32190_e54529, assign32190_e54529_d_n3, assign32190_e54529_d_n4, assign32190_e54529_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 != 0.0)) {
        let assign32190_e54514: f64 = (p.p1607 * locals.var_pb23d);
        let assign32190_e54516: f64 = (assign32190_e54514 * locals.var_czbdswg);
        let assign32190_e54520: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32190_e54521: f64 = (1.0 - assign32190_e54520);
        let assign32190_e54522: f64 = (assign32190_e54516 * assign32190_e54521);
        let assign32190_e54525: f64 = (1.0 - p.p1613);
        let assign32190_e54526: f64 = (assign32190_e54522 / assign32190_e54525);
        let assign32190_e54527: f64 = (locals.var_qec__blk605 + assign32190_e54526);
        (assign32190_e54527, (locals.var_qec__blk605_dn3 + ((assign32190_e54516 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32190_e54525)), (locals.var_qec__blk605_dn4 + ((((((p.p1607 * locals.var_pb23d_dn4) * locals.var_czbdswg) + (assign32190_e54514 * locals.var_czbdswg_dn4)) * assign32190_e54521) + (assign32190_e54516 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32190_e54525)), (locals.var_qec__blk605_dn5 + ((assign32190_e54516 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32190_e54525)),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32190_e54529;
        locals.var_qedj3_dn3 = assign32190_e54529_d_n3;
        locals.var_qedj3_dn4 = assign32190_e54529_d_n4;
        locals.var_qedj3_dn5 = assign32190_e54529_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32200_e54566, assign32200_e54566_d_n3, assign32200_e54566_d_n4, assign32200_e54566_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) && (locals.var_guard609 == 0.0)) && (locals.var_guard614 == 0.0)) {
        let assign32200_e54546: f64 = (p.p1607 * locals.var_pb23d);
        let assign32200_e54548: f64 = (assign32200_e54546 * locals.var_czbdswg);
        let (assign32200_e54562, assign32200_e54562_d_n3, assign32200_e54562_d_n4, assign32200_e54562_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32200_e54554: f64 = (-87.498233534);
                (assign32200_e54554, 0.0, 0.0, 0.0,)
            } else {
                let (assign32200_e54561, assign32200_e54561_d_n3, assign32200_e54561_d_n4, assign32200_e54561_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32200_e54559: f64 = (locals.var_arg__blk603).ln();
                        (assign32200_e54559, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32200_e54561, assign32200_e54561_d_n3, assign32200_e54561_d_n4, assign32200_e54561_d_n5,)
            }
        };
        let assign32200_e54563: f64 = (assign32200_e54548 * assign32200_e54562);
        let assign32200_e54564: f64 = (locals.var_qec__blk605 - assign32200_e54563);
        (assign32200_e54564, (locals.var_qec__blk605_dn3 - (assign32200_e54548 * assign32200_e54562_d_n3)), (locals.var_qec__blk605_dn4 - (((((p.p1607 * locals.var_pb23d_dn4) * locals.var_czbdswg) + (assign32200_e54546 * locals.var_czbdswg_dn4)) * assign32200_e54562) + (assign32200_e54548 * assign32200_e54562_d_n4))), (locals.var_qec__blk605_dn5 - (assign32200_e54548 * assign32200_e54562_d_n5)),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32200_e54566;
        locals.var_qedj3_dn3 = assign32200_e54566_d_n3;
        locals.var_qedj3_dn4 = assign32200_e54566_d_n4;
        locals.var_qedj3_dn5 = assign32200_e54566_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32210_e54579, assign32210_e54579_d_n3, assign32210_e54579_d_n4, assign32210_e54579_d_n5,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) {
        let assign32210_e54577: f64 = (1.0 - locals.var_t1__blk598);
        (assign32210_e54577, (-locals.var_t1__blk598_dn3), (-locals.var_t1__blk598_dn4), (-locals.var_t1__blk598_dn5),)
    } else {
        (locals.var_arg__blk603, locals.var_arg__blk603_dn3, locals.var_arg__blk603_dn4, locals.var_arg__blk603_dn5,)
    }
};
        locals.var_arg__blk603 = assign32210_e54579;
        locals.var_arg__blk603_dn3 = assign32210_e54579_d_n3;
        locals.var_arg__blk603_dn4 = assign32210_e54579_d_n4;
        locals.var_arg__blk603_dn5 = assign32210_e54579_d_n5;
        locals.var_arg__blk603_rv = 0.0;

        let assign32220_e54582: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign32220_e54582;
        locals.var_guard616_rv = 0.0;

        let assign32230_e54585: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard617 = assign32230_e54585;
        locals.var_guard617_rv = 0.0;

        let (assign32240_e54603, assign32240_e54603_d_n3, assign32240_e54603_d_n4, assign32240_e54603_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 != 0.0)) {
        let assign32240_e54600: f64 = (locals.var_arg__blk603).sqrt();
        let assign32240_e54601: f64 = (1.0 / assign32240_e54600);
        (assign32240_e54601, (-((locals.var_arg__blk603_dn3 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))), (-((locals.var_arg__blk603_dn4 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))), (-((locals.var_arg__blk603_dn5 / (2.0 * assign32240_e54600)) / (assign32240_e54600 * assign32240_e54600))),)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32240_e54603;
        locals.var_sarg__blk604_dn3 = assign32240_e54603_d_n3;
        locals.var_sarg__blk604_dn4 = assign32240_e54603_d_n4;
        locals.var_sarg__blk604_dn5 = assign32240_e54603_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32250_e54622, assign32250_e54622_d_n3, assign32250_e54622_d_n4, assign32250_e54622_d_n5,) = {
    if ((((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) && (locals.var_guard617 == 0.0)) {
        let assign32250_e54619: f64 = (-p.p1601);
        let assign32250_e54620: f64 = (locals.var_arg__blk603).powf(assign32250_e54619);
        (assign32250_e54620, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn3)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn3 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn4)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn4 / locals.var_arg__blk603))) }, if 0.0 == 0.0 && ((assign32250_e54619) as f64).is_finite() && ((assign32250_e54619) as f64).fract() == 0.0 { if assign32250_e54619 == 0.0 { 0.0 } else { (assign32250_e54619 * ((locals.var_arg__blk603).powf(assign32250_e54619 - 1.0) * locals.var_arg__blk603_dn5)) } } else { (assign32250_e54620 * (assign32250_e54619 * (locals.var_arg__blk603_dn5 / locals.var_arg__blk603))) },)
    } else {
        (locals.var_sarg__blk604, locals.var_sarg__blk604_dn3, locals.var_sarg__blk604_dn4, locals.var_sarg__blk604_dn5,)
    }
};
        locals.var_sarg__blk604 = assign32250_e54622;
        locals.var_sarg__blk604_dn3 = assign32250_e54622_d_n3;
        locals.var_sarg__blk604_dn4 = assign32250_e54622_d_n4;
        locals.var_sarg__blk604_dn5 = assign32250_e54622_d_n5;
        locals.var_sarg__blk604_rv = 0.0;

        let (assign32260_e54647, assign32260_e54647_d_n3, assign32260_e54647_d_n4, assign32260_e54647_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign32260_e54635: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32260_e54639: f64 = (locals.var_arg__blk603 * locals.var_sarg__blk604);
        let assign32260_e54640: f64 = (1.0 - assign32260_e54639);
        let assign32260_e54641: f64 = (assign32260_e54635 * assign32260_e54640);
        let assign32260_e54644: f64 = (1.0 - p.p1601);
        let assign32260_e54645: f64 = (assign32260_e54641 / assign32260_e54644);
        (assign32260_e54645, ((assign32260_e54635 * (-((locals.var_arg__blk603_dn3 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn3)))) / assign32260_e54644), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32260_e54640) + (assign32260_e54635 * (-((locals.var_arg__blk603_dn4 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn4))))) / assign32260_e54644), ((assign32260_e54635 * (-((locals.var_arg__blk603_dn5 * locals.var_sarg__blk604) + (locals.var_arg__blk603 * locals.var_sarg__blk604_dn5)))) / assign32260_e54644),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32260_e54647;
        locals.var_qedj3_dn3 = assign32260_e54647_d_n3;
        locals.var_qedj3_dn4 = assign32260_e54647_d_n4;
        locals.var_qedj3_dn5 = assign32260_e54647_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32270_e54679, assign32270_e54679_d_n3, assign32270_e54679_d_n4, assign32270_e54679_d_n5,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 == 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign32270_e54660: f64 = (-locals.var_pbswgd_t);
        let assign32270_e54662: f64 = (assign32270_e54660 * locals.var_czbdswg);
        let (assign32270_e54676, assign32270_e54676_d_n3, assign32270_e54676_d_n4, assign32270_e54676_d_n5,) = {
            if (!(locals.var_arg__blk603 > 1e-38)) {
                let assign32270_e54668: f64 = (-87.498233534);
                (assign32270_e54668, 0.0, 0.0, 0.0,)
            } else {
                let (assign32270_e54675, assign32270_e54675_d_n3, assign32270_e54675_d_n4, assign32270_e54675_d_n5,) = {
                    if (locals.var_arg__blk603 > 1e-38) {
                        let assign32270_e54673: f64 = (locals.var_arg__blk603).ln();
                        (assign32270_e54673, (locals.var_arg__blk603_dn3 / locals.var_arg__blk603), (locals.var_arg__blk603_dn4 / locals.var_arg__blk603), (locals.var_arg__blk603_dn5 / locals.var_arg__blk603),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32270_e54675, assign32270_e54675_d_n3, assign32270_e54675_d_n4, assign32270_e54675_d_n5,)
            }
        };
        let assign32270_e54677: f64 = (assign32270_e54662 * assign32270_e54676);
        (assign32270_e54677, (assign32270_e54662 * assign32270_e54676_d_n3), (((((-locals.var_pbswgd_t_dn4) * locals.var_czbdswg) + (assign32270_e54660 * locals.var_czbdswg_dn4)) * assign32270_e54676) + (assign32270_e54662 * assign32270_e54676_d_n4)), (assign32270_e54662 * assign32270_e54676_d_n5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32270_e54679;
        locals.var_qedj3_dn3 = assign32270_e54679_d_n3;
        locals.var_qedj3_dn4 = assign32270_e54679_d_n4;
        locals.var_qedj3_dn5 = assign32270_e54679_d_n5;
        locals.var_qedj3_rv = 0.0;

        let assign32280_e54682: f64 = if p.p1601 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign32280_e54682;
        locals.var_guard618_rv = 0.0;

        let assign32290_e54685: f64 = if p.p1601 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign32290_e54685;
        locals.var_guard619_rv = 0.0;

        let (assign32300_e54701,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign32300_e54698: f64 = (0.1_f64).sqrt();
        let assign32300_e54699: f64 = (1.0 / assign32300_e54698);
        (assign32300_e54699,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32300_e54701;
        locals.var_t2__blk599_rv = 0.0;

        let (assign32310_e54718,) = {
    if (((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) {
        let assign32310_e54715: f64 = (-p.p1601);
        let assign32310_e54716: f64 = (0.1_f64).powf(assign32310_e54715);
        (assign32310_e54716,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32310_e54718;
        locals.var_t2__blk599_rv = 0.0;

        let (assign32320_e54733,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) {
        let assign32320_e54730: f64 = (1.0 - p.p1601);
        let assign32320_e54731: f64 = (1.0 / assign32320_e54730);
        (assign32320_e54731,)
    } else {
        (locals.var_t3__blk600,)
    }
};
        locals.var_t3__blk600 = assign32320_e54733;
        locals.var_t3__blk600_rv = 0.0;

        let (assign32330_e54756,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 != 0.0)) {
        let assign32330_e54746: f64 = (0.05 * p.p1601);
        let assign32330_e54749: f64 = (1.0 + p.p1601);
        let assign32330_e54750: f64 = (assign32330_e54746 * assign32330_e54749);
        let assign32330_e54752: f64 = (assign32330_e54750 * locals.var_t2__blk599);
        let assign32330_e54753: f64 = (1.0 - assign32330_e54752);
        let assign32330_e54754: f64 = (locals.var_t3__blk600 * assign32330_e54753);
        (assign32330_e54754,)
    } else {
        (locals.var_t5__blk602,)
    }
};
        locals.var_t5__blk602 = assign32330_e54756;
        locals.var_t5__blk602_rv = 0.0;

        let (assign32340_e54768,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 == 0.0)) {
        (10.0,)
    } else {
        (locals.var_t2__blk599,)
    }
};
        locals.var_t2__blk599 = assign32340_e54768;
        locals.var_t2__blk599_rv = 0.0;

        let (assign32350_e54783,) = {
    if ((((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign32350_e54780: f64 = (0.1_f64).ln();
        let assign32350_e54781: f64 = (1.5 - assign32350_e54780);
        (assign32350_e54781,)
    } else {
        (locals.var_t5__blk602,)
    }
};
        locals.var_t5__blk602 = assign32350_e54783;
        locals.var_t5__blk602_rv = 0.0;

        let (assign32360_e54808, assign32360_e54808_d_n3, assign32360_e54808_d_n4, assign32360_e54808_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign32360_e54793: f64 = (locals.var_t1__blk598 - 1.0);
        let assign32360_e54794: f64 = (locals.var_t2__blk599 * assign32360_e54793);
        let assign32360_e54797: f64 = (5.0 * p.p1601);
        let assign32360_e54800: f64 = (locals.var_t1__blk598 - 1.0);
        let assign32360_e54801: f64 = (assign32360_e54797 * assign32360_e54800);
        let assign32360_e54804: f64 = (1.0 + p.p1601);
        let assign32360_e54805: f64 = (assign32360_e54801 + assign32360_e54804);
        let assign32360_e54806: f64 = (assign32360_e54794 * assign32360_e54805);
        (assign32360_e54806, (((locals.var_t2__blk599 * locals.var_t1__blk598_dn3) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn3))), (((locals.var_t2__blk599 * locals.var_t1__blk598_dn4) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn4))), (((locals.var_t2__blk599 * locals.var_t1__blk598_dn5) * assign32360_e54805) + (assign32360_e54794 * (assign32360_e54797 * locals.var_t1__blk598_dn5))),)
    } else {
        (locals.var_t4__blk601, locals.var_t4__blk601_dn3, locals.var_t4__blk601_dn4, locals.var_t4__blk601_dn5,)
    }
};
        locals.var_t4__blk601 = assign32360_e54808;
        locals.var_t4__blk601_dn3 = assign32360_e54808_d_n3;
        locals.var_t4__blk601_dn4 = assign32360_e54808_d_n4;
        locals.var_t4__blk601_dn5 = assign32360_e54808_d_n5;
        locals.var_t4__blk601_rv = 0.0;

        let (assign32370_e54823, assign32370_e54823_d_n3, assign32370_e54823_d_n4, assign32370_e54823_d_n5,) = {
    if (((locals.var_guard469 != 0.0) && (locals.var_guard606 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign32370_e54817: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign32370_e54820: f64 = (locals.var_t4__blk601 + locals.var_t5__blk602);
        let assign32370_e54821: f64 = (assign32370_e54817 * assign32370_e54820);
        (assign32370_e54821, (assign32370_e54817 * locals.var_t4__blk601_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign32370_e54820) + (assign32370_e54817 * locals.var_t4__blk601_dn4)), (assign32370_e54817 * locals.var_t4__blk601_dn5),)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32370_e54823;
        locals.var_qedj3_dn3 = assign32370_e54823_d_n3;
        locals.var_qedj3_dn4 = assign32370_e54823_d_n4;
        locals.var_qedj3_dn5 = assign32370_e54823_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32380_e54830, assign32380_e54830_d_n3, assign32380_e54830_d_n4, assign32380_e54830_d_n5,) = {
    if ((locals.var_guard469 != 0.0) && (locals.var_guard606 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qedj3, locals.var_qedj3_dn3, locals.var_qedj3_dn4, locals.var_qedj3_dn5,)
    }
};
        locals.var_qedj3 = assign32380_e54830;
        locals.var_qedj3_dn3 = assign32380_e54830_d_n3;
        locals.var_qedj3_dn4 = assign32380_e54830_d_n4;
        locals.var_qedj3_dn5 = assign32380_e54830_d_n5;
        locals.var_qedj3_rv = 0.0;

        let (assign32390_e54838, assign32390_e54838_d_n3, assign32390_e54838_d_n4, assign32390_e54838_d_n5,) = {
    if (locals.var_guard469 != 0.0) {
        let assign32390_e54834: f64 = (locals.var_qedj1 + locals.var_qedj2);
        let assign32390_e54836: f64 = (assign32390_e54834 + locals.var_qedj3);
        (assign32390_e54836, ((locals.var_qedj1_dn3 + locals.var_qedj2_dn3) + locals.var_qedj3_dn3), ((locals.var_qedj1_dn4 + locals.var_qedj2_dn4) + locals.var_qedj3_dn4), ((locals.var_qedj1_dn5 + locals.var_qedj2_dn5) + locals.var_qedj3_dn5),)
    } else {
        (locals.var_qedj, locals.var_qedj_dn3, locals.var_qedj_dn4, locals.var_qedj_dn5,)
    }
};
        locals.var_qedj = assign32390_e54838;
        locals.var_qedj_dn3 = assign32390_e54838_d_n3;
        locals.var_qedj_dn4 = assign32390_e54838_d_n4;
        locals.var_qedj_dn5 = assign32390_e54838_d_n5;
        locals.var_qedj_rv = 0.0;

        let assign32400_e54842: f64 = (locals.var_csbox * locals.var_ves_jct);
        let assign32400_e54843: f64 = (locals.var_qesj + assign32400_e54842);
        locals.var_qes = assign32400_e54843;
        locals.var_qes_dn0 = (locals.var_csbox_dn0 * locals.var_ves_jct);
        locals.var_qes_dn2 = (locals.var_csbox_dn2 * locals.var_ves_jct);
        locals.var_qes_dn3 = (locals.var_qesj_dn3 + ((locals.var_csbox_dn3 * locals.var_ves_jct) + (locals.var_csbox * locals.var_ves_jct_dn3)));
        locals.var_qes_dn4 = (locals.var_qesj_dn4 + (locals.var_csbox_dn4 * locals.var_ves_jct));
        locals.var_qes_dn5 = (locals.var_csbox_dn5 * locals.var_ves_jct);
        locals.var_qes_dn6 = (locals.var_qesj_dn6 + ((locals.var_csbox_dn6 * locals.var_ves_jct) + (locals.var_csbox * locals.var_ves_jct_dn6)));
        locals.var_qes_dn7 = (locals.var_csbox_dn7 * locals.var_ves_jct);
        locals.var_qes_dn8 = (locals.var_csbox_dn8 * locals.var_ves_jct);
        locals.var_qes_dn9 = (locals.var_csbox_dn9 * locals.var_ves_jct);
        locals.var_qes_dn10 = (locals.var_csbox_dn10 * locals.var_ves_jct);
        locals.var_qes_dn11 = (locals.var_csbox_dn11 * locals.var_ves_jct);
        locals.var_qes_dn13 = (locals.var_csbox_dn13 * locals.var_ves_jct);
        locals.var_qes_dn14 = (locals.var_csbox_dn14 * locals.var_ves_jct);
        locals.var_qes_rv = 0.0;

        let assign32410_e54847: f64 = (locals.var_cdbox * locals.var_ved_jct);
        let assign32410_e54848: f64 = (locals.var_qedj + assign32410_e54847);
        locals.var_qed = assign32410_e54848;
        locals.var_qed_dn0 = (locals.var_cdbox_dn0 * locals.var_ved_jct);
        locals.var_qed_dn2 = (locals.var_cdbox_dn2 * locals.var_ved_jct);
        locals.var_qed_dn3 = (locals.var_qedj_dn3 + ((locals.var_cdbox_dn3 * locals.var_ved_jct) + (locals.var_cdbox * locals.var_ved_jct_dn3)));
        locals.var_qed_dn4 = (locals.var_qedj_dn4 + (locals.var_cdbox_dn4 * locals.var_ved_jct));
        locals.var_qed_dn5 = (locals.var_qedj_dn5 + ((locals.var_cdbox_dn5 * locals.var_ved_jct) + (locals.var_cdbox * locals.var_ved_jct_dn5)));
        locals.var_qed_dn6 = (locals.var_cdbox_dn6 * locals.var_ved_jct);
        locals.var_qed_dn7 = (locals.var_cdbox_dn7 * locals.var_ved_jct);
        locals.var_qed_dn8 = (locals.var_cdbox_dn8 * locals.var_ved_jct);
        locals.var_qed_dn9 = (locals.var_cdbox_dn9 * locals.var_ved_jct);
        locals.var_qed_dn10 = (locals.var_cdbox_dn10 * locals.var_ved_jct);
        locals.var_qed_dn11 = (locals.var_cdbox_dn11 * locals.var_ved_jct);
        locals.var_qed_dn13 = (locals.var_cdbox_dn13 * locals.var_ved_jct);
        locals.var_qed_dn14 = (locals.var_cdbox_dn14 * locals.var_ved_jct);
        locals.var_qed_rv = 0.0;

        let assign32420_e54851: f64 = (locals.var_cgbox * locals.var_devsign);
        let assign32420_e54853: f64 = (assign32420_e54851 * (nv3 - nv10));
        locals.var_qeg = assign32420_e54853;
        locals.var_qeg_dn0 = 0.0;
        locals.var_qeg_dn2 = 0.0;
        locals.var_qeg_dn3 = assign32420_e54851;
        locals.var_qeg_dn4 = 0.0;
        locals.var_qeg_dn5 = 0.0;
        locals.var_qeg_dn6 = 0.0;
        locals.var_qeg_dn7 = 0.0;
        locals.var_qeg_dn8 = 0.0;
        locals.var_qeg_dn9 = 0.0;
        locals.var_qeg_dn10 = (-assign32420_e54851);
        locals.var_qeg_dn11 = 0.0;
        locals.var_qeg_dn13 = 0.0;
        locals.var_qeg_dn14 = 0.0;
        locals.var_qeg_rv = 0.0;

        let assign32430_e54856: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign32430_e54856;
        locals.var_guard620_rv = 0.0;

        let (assign32440_e54862, assign32440_e54862_d_n0, assign32440_e54862_d_n2, assign32440_e54862_d_n3, assign32440_e54862_d_n4, assign32440_e54862_d_n5, assign32440_e54862_d_n6, assign32440_e54862_d_n7, assign32440_e54862_d_n8, assign32440_e54862_d_n9, assign32440_e54862_d_n10, assign32440_e54862_d_n11, assign32440_e54862_d_n13, assign32440_e54862_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32440_e54860: f64 = (locals.var_devsign * (nv10 - nv3));
        (assign32440_e54860, 0.0, 0.0, (-locals.var_devsign), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_devsign, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32440_e54862;
        locals.var_t2_dn0 = assign32440_e54862_d_n0;
        locals.var_t2_dn2 = assign32440_e54862_d_n2;
        locals.var_t2_dn3 = assign32440_e54862_d_n3;
        locals.var_t2_dn4 = assign32440_e54862_d_n4;
        locals.var_t2_dn5 = assign32440_e54862_d_n5;
        locals.var_t2_dn6 = assign32440_e54862_d_n6;
        locals.var_t2_dn7 = assign32440_e54862_d_n7;
        locals.var_t2_dn8 = assign32440_e54862_d_n8;
        locals.var_t2_dn9 = assign32440_e54862_d_n9;
        locals.var_t2_dn10 = assign32440_e54862_d_n10;
        locals.var_t2_dn11 = assign32440_e54862_d_n11;
        locals.var_t2_dn13 = assign32440_e54862_d_n13;
        locals.var_t2_dn14 = assign32440_e54862_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32450_e54876, assign32450_e54876_d_n0, assign32450_e54876_d_n2, assign32450_e54876_d_n3, assign32450_e54876_d_n4, assign32450_e54876_d_n5, assign32450_e54876_d_n6, assign32450_e54876_d_n7, assign32450_e54876_d_n8, assign32450_e54876_d_n9, assign32450_e54876_d_n10, assign32450_e54876_d_n11, assign32450_e54876_d_n13, assign32450_e54876_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32450_e54866: f64 = (locals.var_t2 - locals.var_deltaphi);
        let assign32450_e54869: f64 = (locals.var_eg / 2.0);
        let assign32450_e54870: f64 = (assign32450_e54866 + assign32450_e54869);
        let assign32450_e54872: f64 = (assign32450_e54870 + locals.var_phib);
        let assign32450_e54874: f64 = (assign32450_e54872 - p.p1529);
        (assign32450_e54874, ((locals.var_t2_dn0 - locals.var_deltaphi_dn0) + locals.var_phib_dn0), ((locals.var_t2_dn2 - locals.var_deltaphi_dn2) + locals.var_phib_dn2), ((locals.var_t2_dn3 - locals.var_deltaphi_dn3) + locals.var_phib_dn3), (((locals.var_t2_dn4 - locals.var_deltaphi_dn4) + (locals.var_eg_dn4 / 2.0)) + locals.var_phib_dn4), ((locals.var_t2_dn5 - locals.var_deltaphi_dn5) + locals.var_phib_dn5), ((locals.var_t2_dn6 - locals.var_deltaphi_dn6) + locals.var_phib_dn6), ((locals.var_t2_dn7 - locals.var_deltaphi_dn7) + locals.var_phib_dn7), ((locals.var_t2_dn8 - locals.var_deltaphi_dn8) + locals.var_phib_dn8), ((locals.var_t2_dn9 - locals.var_deltaphi_dn9) + locals.var_phib_dn9), ((locals.var_t2_dn10 - locals.var_deltaphi_dn10) + locals.var_phib_dn10), ((locals.var_t2_dn11 - locals.var_deltaphi_dn11) + locals.var_phib_dn11), ((locals.var_t2_dn13 - locals.var_deltaphi_dn13) + locals.var_phib_dn13), ((locals.var_t2_dn14 - locals.var_deltaphi_dn14) + locals.var_phib_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32450_e54876;
        locals.var_t3_dn0 = assign32450_e54876_d_n0;
        locals.var_t3_dn2 = assign32450_e54876_d_n2;
        locals.var_t3_dn3 = assign32450_e54876_d_n3;
        locals.var_t3_dn4 = assign32450_e54876_d_n4;
        locals.var_t3_dn5 = assign32450_e54876_d_n5;
        locals.var_t3_dn6 = assign32450_e54876_d_n6;
        locals.var_t3_dn7 = assign32450_e54876_d_n7;
        locals.var_t3_dn8 = assign32450_e54876_d_n8;
        locals.var_t3_dn9 = assign32450_e54876_d_n9;
        locals.var_t3_dn10 = assign32450_e54876_d_n10;
        locals.var_t3_dn11 = assign32450_e54876_d_n11;
        locals.var_t3_dn13 = assign32450_e54876_d_n13;
        locals.var_t3_dn14 = assign32450_e54876_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32460_e54882, assign32460_e54882_d_n0, assign32460_e54882_d_n2, assign32460_e54882_d_n3, assign32460_e54882_d_n4, assign32460_e54882_d_n5, assign32460_e54882_d_n6, assign32460_e54882_d_n7, assign32460_e54882_d_n8, assign32460_e54882_d_n9, assign32460_e54882_d_n10, assign32460_e54882_d_n11, assign32460_e54882_d_n13, assign32460_e54882_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32460_e54880: f64 = (locals.var_t3 + 0.02);
        (assign32460_e54880, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32460_e54882;
        locals.var_t0_dn0 = assign32460_e54882_d_n0;
        locals.var_t0_dn2 = assign32460_e54882_d_n2;
        locals.var_t0_dn3 = assign32460_e54882_d_n3;
        locals.var_t0_dn4 = assign32460_e54882_d_n4;
        locals.var_t0_dn5 = assign32460_e54882_d_n5;
        locals.var_t0_dn6 = assign32460_e54882_d_n6;
        locals.var_t0_dn7 = assign32460_e54882_d_n7;
        locals.var_t0_dn8 = assign32460_e54882_d_n8;
        locals.var_t0_dn9 = assign32460_e54882_d_n9;
        locals.var_t0_dn10 = assign32460_e54882_d_n10;
        locals.var_t0_dn11 = assign32460_e54882_d_n11;
        locals.var_t0_dn13 = assign32460_e54882_d_n13;
        locals.var_t0_dn14 = assign32460_e54882_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32470_e54897, assign32470_e54897_d_n0, assign32470_e54897_d_n2, assign32470_e54897_d_n3, assign32470_e54897_d_n4, assign32470_e54897_d_n5, assign32470_e54897_d_n6, assign32470_e54897_d_n7, assign32470_e54897_d_n8, assign32470_e54897_d_n9, assign32470_e54897_d_n10, assign32470_e54897_d_n11, assign32470_e54897_d_n13, assign32470_e54897_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32470_e54888: f64 = (locals.var_t0 * locals.var_t0);
        let assign32470_e54891: f64 = (4.0 * 0.02);
        let assign32470_e54892: f64 = (assign32470_e54888 + assign32470_e54891);
        let assign32470_e54893: f64 = (assign32470_e54892).sqrt();
        let assign32470_e54894: f64 = (locals.var_t0 + assign32470_e54893);
        let assign32470_e54895: f64 = (0.5 * assign32470_e54894);
        (assign32470_e54895, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign32470_e54893)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign32470_e54893)))),)
    } else {
        (locals.var_vge_overlap, locals.var_vge_overlap_dn0, locals.var_vge_overlap_dn2, locals.var_vge_overlap_dn3, locals.var_vge_overlap_dn4, locals.var_vge_overlap_dn5, locals.var_vge_overlap_dn6, locals.var_vge_overlap_dn7, locals.var_vge_overlap_dn8, locals.var_vge_overlap_dn9, locals.var_vge_overlap_dn10, locals.var_vge_overlap_dn11, locals.var_vge_overlap_dn13, locals.var_vge_overlap_dn14,)
    }
};
        locals.var_vge_overlap = assign32470_e54897;
        locals.var_vge_overlap_dn0 = assign32470_e54897_d_n0;
        locals.var_vge_overlap_dn2 = assign32470_e54897_d_n2;
        locals.var_vge_overlap_dn3 = assign32470_e54897_d_n3;
        locals.var_vge_overlap_dn4 = assign32470_e54897_d_n4;
        locals.var_vge_overlap_dn5 = assign32470_e54897_d_n5;
        locals.var_vge_overlap_dn6 = assign32470_e54897_d_n6;
        locals.var_vge_overlap_dn7 = assign32470_e54897_d_n7;
        locals.var_vge_overlap_dn8 = assign32470_e54897_d_n8;
        locals.var_vge_overlap_dn9 = assign32470_e54897_d_n9;
        locals.var_vge_overlap_dn10 = assign32470_e54897_d_n10;
        locals.var_vge_overlap_dn11 = assign32470_e54897_d_n11;
        locals.var_vge_overlap_dn13 = assign32470_e54897_d_n13;
        locals.var_vge_overlap_dn14 = assign32470_e54897_d_n14;
        locals.var_vge_overlap_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_127(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32480_e54926, assign32480_e54926_d_n0, assign32480_e54926_d_n2, assign32480_e54926_d_n3, assign32480_e54926_d_n4, assign32480_e54926_d_n5, assign32480_e54926_d_n6, assign32480_e54926_d_n7, assign32480_e54926_d_n8, assign32480_e54926_d_n9, assign32480_e54926_d_n10, assign32480_e54926_d_n11, assign32480_e54926_d_n13, assign32480_e54926_d_n14,) = {
    if (locals.var_guard620 != 0.0) {
        let assign32480_e54902: f64 = (locals.var_nfintotal * locals.var_leffcv_1);
        let assign32480_e54906: f64 = (locals.var_t3 - locals.var_vge_overlap);
        let assign32480_e54909: f64 = (0.5 * locals.var_ckappab_i);
        let assign32480_e54913: f64 = (4.0 * locals.var_vge_overlap);
        let assign32480_e54915: f64 = (assign32480_e54913 / locals.var_ckappab_i);
        let assign32480_e54916: f64 = (1.0 + assign32480_e54915);
        let assign32480_e54917: f64 = (assign32480_e54916).sqrt();
        let assign32480_e54919: f64 = (assign32480_e54917 - 1.0);
        let assign32480_e54920: f64 = (assign32480_e54909 * assign32480_e54919);
        let assign32480_e54921: f64 = (assign32480_e54906 + assign32480_e54920);
        let assign32480_e54922: f64 = (locals.var_cgbl_i * assign32480_e54921);
        let assign32480_e54923: f64 = (assign32480_e54902 * assign32480_e54922);
        let assign32480_e54924: f64 = (locals.var_qeg - assign32480_e54923);
        (assign32480_e54924, (locals.var_qeg_dn0 - (((locals.var_nfintotal * locals.var_leffcv_1_dn0) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn0 - locals.var_vge_overlap_dn0) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn0) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn2 - (((locals.var_nfintotal * locals.var_leffcv_1_dn2) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn2 - locals.var_vge_overlap_dn2) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn2) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn3 - (((locals.var_nfintotal * locals.var_leffcv_1_dn3) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn3 - locals.var_vge_overlap_dn3) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn3) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn4 - (((locals.var_nfintotal * locals.var_leffcv_1_dn4) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn4 - locals.var_vge_overlap_dn4) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn4) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn5 - (((locals.var_nfintotal * locals.var_leffcv_1_dn5) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn5 - locals.var_vge_overlap_dn5) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn5) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn6 - (((locals.var_nfintotal * locals.var_leffcv_1_dn6) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn6 - locals.var_vge_overlap_dn6) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn6) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn7 - (((locals.var_nfintotal * locals.var_leffcv_1_dn7) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn7 - locals.var_vge_overlap_dn7) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn7) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn8 - (((locals.var_nfintotal * locals.var_leffcv_1_dn8) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn8 - locals.var_vge_overlap_dn8) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn8) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn9 - (((locals.var_nfintotal * locals.var_leffcv_1_dn9) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn9 - locals.var_vge_overlap_dn9) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn9) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn10 - (((locals.var_nfintotal * locals.var_leffcv_1_dn10) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn10 - locals.var_vge_overlap_dn10) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn10) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn11 - (((locals.var_nfintotal * locals.var_leffcv_1_dn11) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn11 - locals.var_vge_overlap_dn11) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn11) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn13 - (((locals.var_nfintotal * locals.var_leffcv_1_dn13) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn13 - locals.var_vge_overlap_dn13) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn13) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))), (locals.var_qeg_dn14 - (((locals.var_nfintotal * locals.var_leffcv_1_dn14) * assign32480_e54922) + (assign32480_e54902 * (locals.var_cgbl_i * ((locals.var_t3_dn14 - locals.var_vge_overlap_dn14) + (assign32480_e54909 * (((4.0 * locals.var_vge_overlap_dn14) / locals.var_ckappab_i) / (2.0 * assign32480_e54917)))))))),)
    } else {
        (locals.var_qeg, locals.var_qeg_dn0, locals.var_qeg_dn2, locals.var_qeg_dn3, locals.var_qeg_dn4, locals.var_qeg_dn5, locals.var_qeg_dn6, locals.var_qeg_dn7, locals.var_qeg_dn8, locals.var_qeg_dn9, locals.var_qeg_dn10, locals.var_qeg_dn11, locals.var_qeg_dn13, locals.var_qeg_dn14,)
    }
};
        locals.var_qeg = assign32480_e54926;
        locals.var_qeg_dn0 = assign32480_e54926_d_n0;
        locals.var_qeg_dn2 = assign32480_e54926_d_n2;
        locals.var_qeg_dn3 = assign32480_e54926_d_n3;
        locals.var_qeg_dn4 = assign32480_e54926_d_n4;
        locals.var_qeg_dn5 = assign32480_e54926_d_n5;
        locals.var_qeg_dn6 = assign32480_e54926_d_n6;
        locals.var_qeg_dn7 = assign32480_e54926_d_n7;
        locals.var_qeg_dn8 = assign32480_e54926_d_n8;
        locals.var_qeg_dn9 = assign32480_e54926_d_n9;
        locals.var_qeg_dn10 = assign32480_e54926_d_n10;
        locals.var_qeg_dn11 = assign32480_e54926_d_n11;
        locals.var_qeg_dn13 = assign32480_e54926_d_n13;
        locals.var_qeg_dn14 = assign32480_e54926_d_n14;
        locals.var_qeg_rv = 0.0;

        let assign32490_e54931: f64 = (locals.var_bigen_i * locals.var_vds);
        let assign32490_e54933: f64 = (assign32490_e54931 * locals.var_vds);
        let assign32490_e54934: f64 = (locals.var_aigen_i + assign32490_e54933);
        let assign32490_e54935: f64 = (locals.var_vds * assign32490_e54934);
        locals.var_t1 = assign32490_e54935;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = ((locals.var_vds_dn5 * assign32490_e54934) + (locals.var_vds * (((locals.var_bigen_i * locals.var_vds_dn5) * locals.var_vds) + (assign32490_e54931 * locals.var_vds_dn5))));
        locals.var_t1_dn6 = ((locals.var_vds_dn6 * assign32490_e54934) + (locals.var_vds * (((locals.var_bigen_i * locals.var_vds_dn6) * locals.var_vds) + (assign32490_e54931 * locals.var_vds_dn6))));
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign32510_e54951: f64 = (locals.var_ueff * locals.var_coxeff);
        let assign32510_e54953: f64 = (assign32510_e54951 * locals.var_weff0);
        let assign32510_e54955: f64 = (assign32510_e54953 / locals.var_leff_1);
        locals.var_t0 = assign32510_e54955;
        locals.var_t0_dn0 = ((((((locals.var_ueff_dn0 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn0)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn0)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn2 = ((((((locals.var_ueff_dn2 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn2)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn2)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn3 = ((((((locals.var_ueff_dn3 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn3)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn3)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn4 = ((((((locals.var_ueff_dn4 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn4)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn4)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn5 = ((((((locals.var_ueff_dn5 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn5)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn5)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn6 = ((((((locals.var_ueff_dn6 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn6)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn6)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn7 = ((((((locals.var_ueff_dn7 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn7)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn7)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn8 = ((((((locals.var_ueff_dn8 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn8)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn8)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn9 = ((((((locals.var_ueff_dn9 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn9)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn9)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn10 = ((((((locals.var_ueff_dn10 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn10)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn10)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn11 = ((((((locals.var_ueff_dn11 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn11)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn11)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn13 = ((((((locals.var_ueff_dn13 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn13)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn13)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_dn14 = ((((((locals.var_ueff_dn14 * locals.var_coxeff) + (locals.var_ueff * locals.var_coxeff_dn14)) * locals.var_weff0) * locals.var_leff_1) - (assign32510_e54953 * locals.var_leff_1_dn14)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_t0_rv = 0.0;

        let assign32730_e55066: f64 = (2.0 * locals.var_vsat_a);
        let assign32730_e55068: f64 = (assign32730_e55066 / locals.var_ueff);
        locals.var_esatnoi = assign32730_e55068;
        locals.var_esatnoi_dn0 = ((((2.0 * locals.var_vsat_a_dn0) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn0)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn2 = ((((2.0 * locals.var_vsat_a_dn2) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn2)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn3 = ((((2.0 * locals.var_vsat_a_dn3) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn4 = ((((2.0 * locals.var_vsat_a_dn4) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn5 = ((((2.0 * locals.var_vsat_a_dn5) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn6 = ((((2.0 * locals.var_vsat_a_dn6) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn7 = ((((2.0 * locals.var_vsat_a_dn7) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn8 = ((((2.0 * locals.var_vsat_a_dn8) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn9 = ((((2.0 * locals.var_vsat_a_dn9) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn10 = ((((2.0 * locals.var_vsat_a_dn10) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn11 = ((((2.0 * locals.var_vsat_a_dn11) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn13 = ((((2.0 * locals.var_vsat_a_dn13) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn13)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn14 = ((((2.0 * locals.var_vsat_a_dn14) * locals.var_ueff) - (assign32730_e55066 * locals.var_ueff_dn14)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_rv = 0.0;

        let assign32740_e55079: f64 = if (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard624 = assign32740_e55079;
        locals.var_guard624_rv = 0.0;

        let (assign32750_e55087, assign32750_e55087_d_n0, assign32750_e55087_d_n2, assign32750_e55087_d_n3, assign32750_e55087_d_n4, assign32750_e55087_d_n5, assign32750_e55087_d_n6, assign32750_e55087_d_n7, assign32750_e55087_d_n8, assign32750_e55087_d_n9, assign32750_e55087_d_n10, assign32750_e55087_d_n11, assign32750_e55087_d_n13, assign32750_e55087_d_n14,) = {
    if (locals.var_guard624 != 0.0) {
        let assign32750_e55084: f64 = (2.0 * p.p1687);
        let assign32750_e55085: f64 = (locals.var_leff_1 - assign32750_e55084);
        (assign32750_e55085, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    } else {
        (locals.var_leffnoi, locals.var_leffnoi_dn0, locals.var_leffnoi_dn2, locals.var_leffnoi_dn3, locals.var_leffnoi_dn4, locals.var_leffnoi_dn5, locals.var_leffnoi_dn6, locals.var_leffnoi_dn7, locals.var_leffnoi_dn8, locals.var_leffnoi_dn9, locals.var_leffnoi_dn10, locals.var_leffnoi_dn11, locals.var_leffnoi_dn13, locals.var_leffnoi_dn14,)
    }
};
        locals.var_leffnoi = assign32750_e55087;
        locals.var_leffnoi_dn0 = assign32750_e55087_d_n0;
        locals.var_leffnoi_dn2 = assign32750_e55087_d_n2;
        locals.var_leffnoi_dn3 = assign32750_e55087_d_n3;
        locals.var_leffnoi_dn4 = assign32750_e55087_d_n4;
        locals.var_leffnoi_dn5 = assign32750_e55087_d_n5;
        locals.var_leffnoi_dn6 = assign32750_e55087_d_n6;
        locals.var_leffnoi_dn7 = assign32750_e55087_d_n7;
        locals.var_leffnoi_dn8 = assign32750_e55087_d_n8;
        locals.var_leffnoi_dn9 = assign32750_e55087_d_n9;
        locals.var_leffnoi_dn10 = assign32750_e55087_d_n10;
        locals.var_leffnoi_dn11 = assign32750_e55087_d_n11;
        locals.var_leffnoi_dn13 = assign32750_e55087_d_n13;
        locals.var_leffnoi_dn14 = assign32750_e55087_d_n14;
        locals.var_leffnoi_rv = 0.0;

        let assign32760_e55090: f64 = if locals.var_leffnoi <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign32760_e55090;
        locals.var_guard625_rv = 0.0;

        let (assign32770_e55096, assign32770_e55096_d_n0, assign32770_e55096_d_n2, assign32770_e55096_d_n3, assign32770_e55096_d_n4, assign32770_e55096_d_n5, assign32770_e55096_d_n6, assign32770_e55096_d_n7, assign32770_e55096_d_n8, assign32770_e55096_d_n9, assign32770_e55096_d_n10, assign32770_e55096_d_n11, assign32770_e55096_d_n13, assign32770_e55096_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 != 0.0)) {
        (locals.var_leff_1, locals.var_leff_1_dn0, locals.var_leff_1_dn2, locals.var_leff_1_dn3, locals.var_leff_1_dn4, locals.var_leff_1_dn5, locals.var_leff_1_dn6, locals.var_leff_1_dn7, locals.var_leff_1_dn8, locals.var_leff_1_dn9, locals.var_leff_1_dn10, locals.var_leff_1_dn11, locals.var_leff_1_dn13, locals.var_leff_1_dn14,)
    } else {
        (locals.var_leffnoi, locals.var_leffnoi_dn0, locals.var_leffnoi_dn2, locals.var_leffnoi_dn3, locals.var_leffnoi_dn4, locals.var_leffnoi_dn5, locals.var_leffnoi_dn6, locals.var_leffnoi_dn7, locals.var_leffnoi_dn8, locals.var_leffnoi_dn9, locals.var_leffnoi_dn10, locals.var_leffnoi_dn11, locals.var_leffnoi_dn13, locals.var_leffnoi_dn14,)
    }
};
        locals.var_leffnoi = assign32770_e55096;
        locals.var_leffnoi_dn0 = assign32770_e55096_d_n0;
        locals.var_leffnoi_dn2 = assign32770_e55096_d_n2;
        locals.var_leffnoi_dn3 = assign32770_e55096_d_n3;
        locals.var_leffnoi_dn4 = assign32770_e55096_d_n4;
        locals.var_leffnoi_dn5 = assign32770_e55096_d_n5;
        locals.var_leffnoi_dn6 = assign32770_e55096_d_n6;
        locals.var_leffnoi_dn7 = assign32770_e55096_d_n7;
        locals.var_leffnoi_dn8 = assign32770_e55096_d_n8;
        locals.var_leffnoi_dn9 = assign32770_e55096_d_n9;
        locals.var_leffnoi_dn10 = assign32770_e55096_d_n10;
        locals.var_leffnoi_dn11 = assign32770_e55096_d_n11;
        locals.var_leffnoi_dn13 = assign32770_e55096_d_n13;
        locals.var_leffnoi_dn14 = assign32770_e55096_d_n14;
        locals.var_leffnoi_rv = 0.0;

        let assign32780_e55103: f64 = if ((p.p79 == 1.0) || (p.p79 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard626 = assign32780_e55103;
        locals.var_guard626_rv = 0.0;

        let (assign32790_e55111, assign32790_e55111_d_n0, assign32790_e55111_d_n2, assign32790_e55111_d_n3, assign32790_e55111_d_n4, assign32790_e55111_d_n5, assign32790_e55111_d_n6, assign32790_e55111_d_n7, assign32790_e55111_d_n8, assign32790_e55111_d_n9, assign32790_e55111_d_n10, assign32790_e55111_d_n11, assign32790_e55111_d_n13, assign32790_e55111_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32790_e55109: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign32790_e55109, ((locals.var_leffnoi_dn0 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn0)), ((locals.var_leffnoi_dn2 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn2)), ((locals.var_leffnoi_dn3 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn3)), ((locals.var_leffnoi_dn4 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn4)), ((locals.var_leffnoi_dn5 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn5)), ((locals.var_leffnoi_dn6 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn6)), ((locals.var_leffnoi_dn7 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn7)), ((locals.var_leffnoi_dn8 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn8)), ((locals.var_leffnoi_dn9 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn9)), ((locals.var_leffnoi_dn10 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn10)), ((locals.var_leffnoi_dn11 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn11)), ((locals.var_leffnoi_dn13 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn13)), ((locals.var_leffnoi_dn14 * locals.var_leffnoi) + (locals.var_leffnoi * locals.var_leffnoi_dn14)),)
    } else {
        (locals.var_leffnoisq, locals.var_leffnoisq_dn0, locals.var_leffnoisq_dn2, locals.var_leffnoisq_dn3, locals.var_leffnoisq_dn4, locals.var_leffnoisq_dn5, locals.var_leffnoisq_dn6, locals.var_leffnoisq_dn7, locals.var_leffnoisq_dn8, locals.var_leffnoisq_dn9, locals.var_leffnoisq_dn10, locals.var_leffnoisq_dn11, locals.var_leffnoisq_dn13, locals.var_leffnoisq_dn14,)
    }
};
        locals.var_leffnoisq = assign32790_e55111;
        locals.var_leffnoisq_dn0 = assign32790_e55111_d_n0;
        locals.var_leffnoisq_dn2 = assign32790_e55111_d_n2;
        locals.var_leffnoisq_dn3 = assign32790_e55111_d_n3;
        locals.var_leffnoisq_dn4 = assign32790_e55111_d_n4;
        locals.var_leffnoisq_dn5 = assign32790_e55111_d_n5;
        locals.var_leffnoisq_dn6 = assign32790_e55111_d_n6;
        locals.var_leffnoisq_dn7 = assign32790_e55111_d_n7;
        locals.var_leffnoisq_dn8 = assign32790_e55111_d_n8;
        locals.var_leffnoisq_dn9 = assign32790_e55111_d_n9;
        locals.var_leffnoisq_dn10 = assign32790_e55111_d_n10;
        locals.var_leffnoisq_dn11 = assign32790_e55111_d_n11;
        locals.var_leffnoisq_dn13 = assign32790_e55111_d_n13;
        locals.var_leffnoisq_dn14 = assign32790_e55111_d_n14;
        locals.var_leffnoisq_rv = 0.0;

        let assign32800_e55114: f64 = if p.p1681 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard627 = assign32800_e55114;
        locals.var_guard627_rv = 0.0;

        let (assign32810_e55128, assign32810_e55128_d_n0, assign32810_e55128_d_n2, assign32810_e55128_d_n3, assign32810_e55128_d_n4, assign32810_e55128_d_n5, assign32810_e55128_d_n6, assign32810_e55128_d_n7, assign32810_e55128_d_n8, assign32810_e55128_d_n9, assign32810_e55128_d_n10, assign32810_e55128_d_n11, assign32810_e55128_d_n13, assign32810_e55128_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let assign32810_e55122: f64 = (locals.var_diffvds / locals.var_litl);
        let assign32810_e55124: f64 = (assign32810_e55122 + p.p1681);
        let assign32810_e55126: f64 = (assign32810_e55124 / locals.var_esatnoi);
        (assign32810_e55126, ((((locals.var_diffvds_dn0 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn0)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn2 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn2)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn9 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn10 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn11 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn13 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn13)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn14 / locals.var_litl) * locals.var_esatnoi) - (assign32810_e55124 * locals.var_esatnoi_dn14)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32810_e55128;
        locals.var_t0_dn0 = assign32810_e55128_d_n0;
        locals.var_t0_dn2 = assign32810_e55128_d_n2;
        locals.var_t0_dn3 = assign32810_e55128_d_n3;
        locals.var_t0_dn4 = assign32810_e55128_d_n4;
        locals.var_t0_dn5 = assign32810_e55128_d_n5;
        locals.var_t0_dn6 = assign32810_e55128_d_n6;
        locals.var_t0_dn7 = assign32810_e55128_d_n7;
        locals.var_t0_dn8 = assign32810_e55128_d_n8;
        locals.var_t0_dn9 = assign32810_e55128_d_n9;
        locals.var_t0_dn10 = assign32810_e55128_d_n10;
        locals.var_t0_dn11 = assign32810_e55128_d_n11;
        locals.var_t0_dn13 = assign32810_e55128_d_n13;
        locals.var_t0_dn14 = assign32810_e55128_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32820_e55151, assign32820_e55151_d_n0, assign32820_e55151_d_n2, assign32820_e55151_d_n3, assign32820_e55151_d_n4, assign32820_e55151_d_n5, assign32820_e55151_d_n6, assign32820_e55151_d_n7, assign32820_e55151_d_n8, assign32820_e55151_d_n9, assign32820_e55151_d_n10, assign32820_e55151_d_n11, assign32820_e55151_d_n13, assign32820_e55151_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 != 0.0)) {
        let (assign32820_e55148, assign32820_e55148_d_n0, assign32820_e55148_d_n2, assign32820_e55148_d_n3, assign32820_e55148_d_n4, assign32820_e55148_d_n5, assign32820_e55148_d_n6, assign32820_e55148_d_n7, assign32820_e55148_d_n8, assign32820_e55148_d_n9, assign32820_e55148_d_n10, assign32820_e55148_d_n11, assign32820_e55148_d_n13, assign32820_e55148_d_n14,) = {
            if (!(locals.var_t0 > 1e-38)) {
                let assign32820_e55140: f64 = (-87.498233534);
                (assign32820_e55140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign32820_e55147, assign32820_e55147_d_n0, assign32820_e55147_d_n2, assign32820_e55147_d_n3, assign32820_e55147_d_n4, assign32820_e55147_d_n5, assign32820_e55147_d_n6, assign32820_e55147_d_n7, assign32820_e55147_d_n8, assign32820_e55147_d_n9, assign32820_e55147_d_n10, assign32820_e55147_d_n11, assign32820_e55147_d_n13, assign32820_e55147_d_n14,) = {
                    if (locals.var_t0 > 1e-38) {
                        let assign32820_e55145: f64 = (locals.var_t0).ln();
                        (assign32820_e55145, (locals.var_t0_dn0 / locals.var_t0), (locals.var_t0_dn2 / locals.var_t0), (locals.var_t0_dn3 / locals.var_t0), (locals.var_t0_dn4 / locals.var_t0), (locals.var_t0_dn5 / locals.var_t0), (locals.var_t0_dn6 / locals.var_t0), (locals.var_t0_dn7 / locals.var_t0), (locals.var_t0_dn8 / locals.var_t0), (locals.var_t0_dn9 / locals.var_t0), (locals.var_t0_dn10 / locals.var_t0), (locals.var_t0_dn11 / locals.var_t0), (locals.var_t0_dn13 / locals.var_t0), (locals.var_t0_dn14 / locals.var_t0),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32820_e55147, assign32820_e55147_d_n0, assign32820_e55147_d_n2, assign32820_e55147_d_n3, assign32820_e55147_d_n4, assign32820_e55147_d_n5, assign32820_e55147_d_n6, assign32820_e55147_d_n7, assign32820_e55147_d_n8, assign32820_e55147_d_n9, assign32820_e55147_d_n10, assign32820_e55147_d_n11, assign32820_e55147_d_n13, assign32820_e55147_d_n14,)
            }
        };
        let assign32820_e55149: f64 = (locals.var_litl * assign32820_e55148);
        (assign32820_e55149, (locals.var_litl * assign32820_e55148_d_n0), (locals.var_litl * assign32820_e55148_d_n2), (locals.var_litl * assign32820_e55148_d_n3), (locals.var_litl * assign32820_e55148_d_n4), (locals.var_litl * assign32820_e55148_d_n5), (locals.var_litl * assign32820_e55148_d_n6), (locals.var_litl * assign32820_e55148_d_n7), (locals.var_litl * assign32820_e55148_d_n8), (locals.var_litl * assign32820_e55148_d_n9), (locals.var_litl * assign32820_e55148_d_n10), (locals.var_litl * assign32820_e55148_d_n11), (locals.var_litl * assign32820_e55148_d_n13), (locals.var_litl * assign32820_e55148_d_n14),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign32820_e55151;
        locals.var_delclm_dn0 = assign32820_e55151_d_n0;
        locals.var_delclm_dn2 = assign32820_e55151_d_n2;
        locals.var_delclm_dn3 = assign32820_e55151_d_n3;
        locals.var_delclm_dn4 = assign32820_e55151_d_n4;
        locals.var_delclm_dn5 = assign32820_e55151_d_n5;
        locals.var_delclm_dn6 = assign32820_e55151_d_n6;
        locals.var_delclm_dn7 = assign32820_e55151_d_n7;
        locals.var_delclm_dn8 = assign32820_e55151_d_n8;
        locals.var_delclm_dn9 = assign32820_e55151_d_n9;
        locals.var_delclm_dn10 = assign32820_e55151_d_n10;
        locals.var_delclm_dn11 = assign32820_e55151_d_n11;
        locals.var_delclm_dn13 = assign32820_e55151_d_n13;
        locals.var_delclm_dn14 = assign32820_e55151_d_n14;
        locals.var_delclm_rv = 0.0;

        let (assign32830_e55160, assign32830_e55160_d_n0, assign32830_e55160_d_n2, assign32830_e55160_d_n3, assign32830_e55160_d_n4, assign32830_e55160_d_n5, assign32830_e55160_d_n6, assign32830_e55160_d_n7, assign32830_e55160_d_n8, assign32830_e55160_d_n9, assign32830_e55160_d_n10, assign32830_e55160_d_n11, assign32830_e55160_d_n13, assign32830_e55160_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard627 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign32830_e55160;
        locals.var_delclm_dn0 = assign32830_e55160_d_n0;
        locals.var_delclm_dn2 = assign32830_e55160_d_n2;
        locals.var_delclm_dn3 = assign32830_e55160_d_n3;
        locals.var_delclm_dn4 = assign32830_e55160_d_n4;
        locals.var_delclm_dn5 = assign32830_e55160_d_n5;
        locals.var_delclm_dn6 = assign32830_e55160_d_n6;
        locals.var_delclm_dn7 = assign32830_e55160_d_n7;
        locals.var_delclm_dn8 = assign32830_e55160_d_n8;
        locals.var_delclm_dn9 = assign32830_e55160_d_n9;
        locals.var_delclm_dn10 = assign32830_e55160_d_n10;
        locals.var_delclm_dn11 = assign32830_e55160_d_n11;
        locals.var_delclm_dn13 = assign32830_e55160_d_n13;
        locals.var_delclm_dn14 = assign32830_e55160_d_n14;
        locals.var_delclm_rv = 0.0;

        let assign32840_e55163: f64 = if p.p79 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign32840_e55163;
        locals.var_guard628_rv = 0.0;

        let (assign32850_e55173, assign32850_e55173_d_n0, assign32850_e55173_d_n2, assign32850_e55173_d_n3, assign32850_e55173_d_n4, assign32850_e55173_d_n5, assign32850_e55173_d_n6, assign32850_e55173_d_n7, assign32850_e55173_d_n8, assign32850_e55173_d_n9, assign32850_e55173_d_n10, assign32850_e55173_d_n11, assign32850_e55173_d_n13, assign32850_e55173_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32850_e55171: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign32850_e55171, (locals.var_qia2_dn0 / locals.var_qsref_i), (locals.var_qia2_dn2 / locals.var_qsref_i), (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i), (locals.var_qia2_dn9 / locals.var_qsref_i), (locals.var_qia2_dn10 / locals.var_qsref_i), (locals.var_qia2_dn11 / locals.var_qsref_i), (locals.var_qia2_dn13 / locals.var_qsref_i), (locals.var_qia2_dn14 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32850_e55173;
        locals.var_t1_dn0 = assign32850_e55173_d_n0;
        locals.var_t1_dn2 = assign32850_e55173_d_n2;
        locals.var_t1_dn3 = assign32850_e55173_d_n3;
        locals.var_t1_dn4 = assign32850_e55173_d_n4;
        locals.var_t1_dn5 = assign32850_e55173_d_n5;
        locals.var_t1_dn6 = assign32850_e55173_d_n6;
        locals.var_t1_dn7 = assign32850_e55173_d_n7;
        locals.var_t1_dn8 = assign32850_e55173_d_n8;
        locals.var_t1_dn9 = assign32850_e55173_d_n9;
        locals.var_t1_dn10 = assign32850_e55173_d_n10;
        locals.var_t1_dn11 = assign32850_e55173_d_n11;
        locals.var_t1_dn13 = assign32850_e55173_d_n13;
        locals.var_t1_dn14 = assign32850_e55173_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32860_e55185, assign32860_e55185_d_n0, assign32860_e55185_d_n2, assign32860_e55185_d_n3, assign32860_e55185_d_n4, assign32860_e55185_d_n5, assign32860_e55185_d_n6, assign32860_e55185_d_n7, assign32860_e55185_d_n8, assign32860_e55185_d_n9, assign32860_e55185_d_n10, assign32860_e55185_d_n11, assign32860_e55185_d_n13, assign32860_e55185_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32860_e55182: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign32860_e55183: f64 = (1.0 + assign32860_e55182);
        (assign32860_e55183, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn0)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn2)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn9)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn10)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn11)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn13)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn14)) } } else { (assign32860_e55182 * (locals.var_mpower_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32860_e55185;
        locals.var_t2_dn0 = assign32860_e55185_d_n0;
        locals.var_t2_dn2 = assign32860_e55185_d_n2;
        locals.var_t2_dn3 = assign32860_e55185_d_n3;
        locals.var_t2_dn4 = assign32860_e55185_d_n4;
        locals.var_t2_dn5 = assign32860_e55185_d_n5;
        locals.var_t2_dn6 = assign32860_e55185_d_n6;
        locals.var_t2_dn7 = assign32860_e55185_d_n7;
        locals.var_t2_dn8 = assign32860_e55185_d_n8;
        locals.var_t2_dn9 = assign32860_e55185_d_n9;
        locals.var_t2_dn10 = assign32860_e55185_d_n10;
        locals.var_t2_dn11 = assign32860_e55185_d_n11;
        locals.var_t2_dn13 = assign32860_e55185_d_n13;
        locals.var_t2_dn14 = assign32860_e55185_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32870_e55195, assign32870_e55195_d_n0, assign32870_e55195_d_n2, assign32870_e55195_d_n3, assign32870_e55195_d_n4, assign32870_e55195_d_n5, assign32870_e55195_d_n6, assign32870_e55195_d_n7, assign32870_e55195_d_n8, assign32870_e55195_d_n9, assign32870_e55195_d_n10, assign32870_e55195_d_n11, assign32870_e55195_d_n13, assign32870_e55195_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32870_e55193: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign32870_e55193, (-((locals.var_noia2_i * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32870_e55195;
        locals.var_t3_dn0 = assign32870_e55195_d_n0;
        locals.var_t3_dn2 = assign32870_e55195_d_n2;
        locals.var_t3_dn3 = assign32870_e55195_d_n3;
        locals.var_t3_dn4 = assign32870_e55195_d_n4;
        locals.var_t3_dn5 = assign32870_e55195_d_n5;
        locals.var_t3_dn6 = assign32870_e55195_d_n6;
        locals.var_t3_dn7 = assign32870_e55195_d_n7;
        locals.var_t3_dn8 = assign32870_e55195_d_n8;
        locals.var_t3_dn9 = assign32870_e55195_d_n9;
        locals.var_t3_dn10 = assign32870_e55195_d_n10;
        locals.var_t3_dn11 = assign32870_e55195_d_n11;
        locals.var_t3_dn13 = assign32870_e55195_d_n13;
        locals.var_t3_dn14 = assign32870_e55195_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32880_e55205, assign32880_e55205_d_n0, assign32880_e55205_d_n2, assign32880_e55205_d_n3, assign32880_e55205_d_n4, assign32880_e55205_d_n5, assign32880_e55205_d_n6, assign32880_e55205_d_n7, assign32880_e55205_d_n8, assign32880_e55205_d_n9, assign32880_e55205_d_n10, assign32880_e55205_d_n11, assign32880_e55205_d_n13, assign32880_e55205_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32880_e55203: f64 = (locals.var_t3 / p.p1682);
        (assign32880_e55203, (locals.var_t3_dn0 / p.p1682), (locals.var_t3_dn2 / p.p1682), (locals.var_t3_dn3 / p.p1682), (locals.var_t3_dn4 / p.p1682), (locals.var_t3_dn5 / p.p1682), (locals.var_t3_dn6 / p.p1682), (locals.var_t3_dn7 / p.p1682), (locals.var_t3_dn8 / p.p1682), (locals.var_t3_dn9 / p.p1682), (locals.var_t3_dn10 / p.p1682), (locals.var_t3_dn11 / p.p1682), (locals.var_t3_dn13 / p.p1682), (locals.var_t3_dn14 / p.p1682),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32880_e55205;
        locals.var_t4_dn0 = assign32880_e55205_d_n0;
        locals.var_t4_dn2 = assign32880_e55205_d_n2;
        locals.var_t4_dn3 = assign32880_e55205_d_n3;
        locals.var_t4_dn4 = assign32880_e55205_d_n4;
        locals.var_t4_dn5 = assign32880_e55205_d_n5;
        locals.var_t4_dn6 = assign32880_e55205_d_n6;
        locals.var_t4_dn7 = assign32880_e55205_d_n7;
        locals.var_t4_dn8 = assign32880_e55205_d_n8;
        locals.var_t4_dn9 = assign32880_e55205_d_n9;
        locals.var_t4_dn10 = assign32880_e55205_d_n10;
        locals.var_t4_dn11 = assign32880_e55205_d_n11;
        locals.var_t4_dn13 = assign32880_e55205_d_n13;
        locals.var_t4_dn14 = assign32880_e55205_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32890_e55232, assign32890_e55232_d_n0, assign32890_e55232_d_n2, assign32890_e55232_d_n3, assign32890_e55232_d_n4, assign32890_e55232_d_n5, assign32890_e55232_d_n6, assign32890_e55232_d_n7, assign32890_e55232_d_n8, assign32890_e55232_d_n9, assign32890_e55232_d_n10, assign32890_e55232_d_n11, assign32890_e55232_d_n13, assign32890_e55232_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32890_e55214: f64 = (locals.var_t4 + 1.0);
        let assign32890_e55217: f64 = (locals.var_t4 - 1.0);
        let assign32890_e55220: f64 = (locals.var_t4 - 1.0);
        let assign32890_e55221: f64 = (assign32890_e55217 * assign32890_e55220);
        let assign32890_e55224: f64 = (0.25 * p.p1688);
        let assign32890_e55226: f64 = (assign32890_e55224 * p.p1688);
        let assign32890_e55227: f64 = (assign32890_e55221 + assign32890_e55226);
        let assign32890_e55228: f64 = (assign32890_e55227).sqrt();
        let assign32890_e55229: f64 = (assign32890_e55214 + assign32890_e55228);
        let assign32890_e55230: f64 = (0.5 * assign32890_e55229);
        (assign32890_e55230, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn0)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn2)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn3)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn4)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn5)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn6)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn7)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn8)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn9)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn10)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn11)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn13)) / (2.0 * assign32890_e55228)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign32890_e55220) + (assign32890_e55217 * locals.var_t4_dn14)) / (2.0 * assign32890_e55228)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32890_e55232;
        locals.var_t5_dn0 = assign32890_e55232_d_n0;
        locals.var_t5_dn2 = assign32890_e55232_d_n2;
        locals.var_t5_dn3 = assign32890_e55232_d_n3;
        locals.var_t5_dn4 = assign32890_e55232_d_n4;
        locals.var_t5_dn5 = assign32890_e55232_d_n5;
        locals.var_t5_dn6 = assign32890_e55232_d_n6;
        locals.var_t5_dn7 = assign32890_e55232_d_n7;
        locals.var_t5_dn8 = assign32890_e55232_d_n8;
        locals.var_t5_dn9 = assign32890_e55232_d_n9;
        locals.var_t5_dn10 = assign32890_e55232_d_n10;
        locals.var_t5_dn11 = assign32890_e55232_d_n11;
        locals.var_t5_dn13 = assign32890_e55232_d_n13;
        locals.var_t5_dn14 = assign32890_e55232_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32900_e55242, assign32900_e55242_d_n0, assign32900_e55242_d_n2, assign32900_e55242_d_n3, assign32900_e55242_d_n4, assign32900_e55242_d_n5, assign32900_e55242_d_n6, assign32900_e55242_d_n7, assign32900_e55242_d_n8, assign32900_e55242_d_n9, assign32900_e55242_d_n10, assign32900_e55242_d_n11, assign32900_e55242_d_n13, assign32900_e55242_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign32900_e55240: f64 = (p.p1682 * locals.var_t5);
        (assign32900_e55240, (p.p1682 * locals.var_t5_dn0), (p.p1682 * locals.var_t5_dn2), (p.p1682 * locals.var_t5_dn3), (p.p1682 * locals.var_t5_dn4), (p.p1682 * locals.var_t5_dn5), (p.p1682 * locals.var_t5_dn6), (p.p1682 * locals.var_t5_dn7), (p.p1682 * locals.var_t5_dn8), (p.p1682 * locals.var_t5_dn9), (p.p1682 * locals.var_t5_dn10), (p.p1682 * locals.var_t5_dn11), (p.p1682 * locals.var_t5_dn13), (p.p1682 * locals.var_t5_dn14),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign32900_e55242;
        locals.var_noiaeff_dn0 = assign32900_e55242_d_n0;
        locals.var_noiaeff_dn2 = assign32900_e55242_d_n2;
        locals.var_noiaeff_dn3 = assign32900_e55242_d_n3;
        locals.var_noiaeff_dn4 = assign32900_e55242_d_n4;
        locals.var_noiaeff_dn5 = assign32900_e55242_d_n5;
        locals.var_noiaeff_dn6 = assign32900_e55242_d_n6;
        locals.var_noiaeff_dn7 = assign32900_e55242_d_n7;
        locals.var_noiaeff_dn8 = assign32900_e55242_d_n8;
        locals.var_noiaeff_dn9 = assign32900_e55242_d_n9;
        locals.var_noiaeff_dn10 = assign32900_e55242_d_n10;
        locals.var_noiaeff_dn11 = assign32900_e55242_d_n11;
        locals.var_noiaeff_dn13 = assign32900_e55242_d_n13;
        locals.var_noiaeff_dn14 = assign32900_e55242_d_n14;
        locals.var_noiaeff_rv = 0.0;

        let (assign32910_e55251, assign32910_e55251_d_n0, assign32910_e55251_d_n2, assign32910_e55251_d_n3, assign32910_e55251_d_n4, assign32910_e55251_d_n5, assign32910_e55251_d_n6, assign32910_e55251_d_n7, assign32910_e55251_d_n8, assign32910_e55251_d_n9, assign32910_e55251_d_n10, assign32910_e55251_d_n11, assign32910_e55251_d_n13, assign32910_e55251_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) && (locals.var_guard628 == 0.0)) {
        (p.p1682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign32910_e55251;
        locals.var_noiaeff_dn0 = assign32910_e55251_d_n0;
        locals.var_noiaeff_dn2 = assign32910_e55251_d_n2;
        locals.var_noiaeff_dn3 = assign32910_e55251_d_n3;
        locals.var_noiaeff_dn4 = assign32910_e55251_d_n4;
        locals.var_noiaeff_dn5 = assign32910_e55251_d_n5;
        locals.var_noiaeff_dn6 = assign32910_e55251_d_n6;
        locals.var_noiaeff_dn7 = assign32910_e55251_d_n7;
        locals.var_noiaeff_dn8 = assign32910_e55251_d_n8;
        locals.var_noiaeff_dn9 = assign32910_e55251_d_n9;
        locals.var_noiaeff_dn10 = assign32910_e55251_d_n10;
        locals.var_noiaeff_dn11 = assign32910_e55251_d_n11;
        locals.var_noiaeff_dn13 = assign32910_e55251_d_n13;
        locals.var_noiaeff_dn14 = assign32910_e55251_d_n14;
        locals.var_noiaeff_rv = 0.0;

        let (assign32920_e55268, assign32920_e55268_d_n0, assign32920_e55268_d_n2, assign32920_e55268_d_n3, assign32920_e55268_d_n4, assign32920_e55268_d_n5, assign32920_e55268_d_n6, assign32920_e55268_d_n7, assign32920_e55268_d_n8, assign32920_e55268_d_n9, assign32920_e55268_d_n10, assign32920_e55268_d_n11, assign32920_e55268_d_n13, assign32920_e55268_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32920_e55257: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32920_e55259: f64 = (assign32920_e55257 * 1.60219e-19);
        let assign32920_e55261: f64 = (assign32920_e55259 * locals.var_vtm);
        let assign32920_e55263: f64 = (locals.var_ids_v).abs();
        let assign32920_e55264: f64 = (assign32920_e55261 * assign32920_e55263);
        let assign32920_e55266: f64 = (assign32920_e55264 * locals.var_ueff);
        (assign32920_e55266, (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn0 } else { (-locals.var_ids_v_dn0) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn0)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn2 } else { (-locals.var_ids_v_dn2) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn2)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn3 } else { (-locals.var_ids_v_dn3) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn3)), (((((assign32920_e55259 * locals.var_vtm_dn4) * assign32920_e55263) + (assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn4 } else { (-locals.var_ids_v_dn4) })) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn4)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn5 } else { (-locals.var_ids_v_dn5) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn5)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn6 } else { (-locals.var_ids_v_dn6) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn6)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn7 } else { (-locals.var_ids_v_dn7) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn7)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn8 } else { (-locals.var_ids_v_dn8) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn8)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn9 } else { (-locals.var_ids_v_dn9) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn9)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn10 } else { (-locals.var_ids_v_dn10) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn10)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn11 } else { (-locals.var_ids_v_dn11) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn11)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn13 } else { (-locals.var_ids_v_dn13) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn13)), (((assign32920_e55261 * if locals.var_ids_v >= 0.0 { locals.var_ids_v_dn14 } else { (-locals.var_ids_v_dn14) }) * locals.var_ueff) + (assign32920_e55264 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e55268;
        locals.var_t1_dn0 = assign32920_e55268_d_n0;
        locals.var_t1_dn2 = assign32920_e55268_d_n2;
        locals.var_t1_dn3 = assign32920_e55268_d_n3;
        locals.var_t1_dn4 = assign32920_e55268_d_n4;
        locals.var_t1_dn5 = assign32920_e55268_d_n5;
        locals.var_t1_dn6 = assign32920_e55268_d_n6;
        locals.var_t1_dn7 = assign32920_e55268_d_n7;
        locals.var_t1_dn8 = assign32920_e55268_d_n8;
        locals.var_t1_dn9 = assign32920_e55268_d_n9;
        locals.var_t1_dn10 = assign32920_e55268_d_n10;
        locals.var_t1_dn11 = assign32920_e55268_d_n11;
        locals.var_t1_dn13 = assign32920_e55268_d_n13;
        locals.var_t1_dn14 = assign32920_e55268_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32930_e55278, assign32930_e55278_d_n0, assign32930_e55278_d_n2, assign32930_e55278_d_n3, assign32930_e55278_d_n4, assign32930_e55278_d_n5, assign32930_e55278_d_n6, assign32930_e55278_d_n7, assign32930_e55278_d_n8, assign32930_e55278_d_n9, assign32930_e55278_d_n10, assign32930_e55278_d_n11, assign32930_e55278_d_n13, assign32930_e55278_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32930_e55274: f64 = (10000000000.0 * locals.var_coxeff);
        let assign32930_e55276: f64 = (assign32930_e55274 * locals.var_leffnoisq);
        (assign32930_e55276, (((10000000000.0 * locals.var_coxeff_dn0) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn0)), (((10000000000.0 * locals.var_coxeff_dn2) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn2)), (((10000000000.0 * locals.var_coxeff_dn3) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn3)), (((10000000000.0 * locals.var_coxeff_dn4) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn4)), (((10000000000.0 * locals.var_coxeff_dn5) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn5)), (((10000000000.0 * locals.var_coxeff_dn6) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn6)), (((10000000000.0 * locals.var_coxeff_dn7) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn7)), (((10000000000.0 * locals.var_coxeff_dn8) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn8)), (((10000000000.0 * locals.var_coxeff_dn9) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn9)), (((10000000000.0 * locals.var_coxeff_dn10) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn10)), (((10000000000.0 * locals.var_coxeff_dn11) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn11)), (((10000000000.0 * locals.var_coxeff_dn13) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn13)), (((10000000000.0 * locals.var_coxeff_dn14) * locals.var_leffnoisq) + (assign32930_e55274 * locals.var_leffnoisq_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e55278;
        locals.var_t2_dn0 = assign32930_e55278_d_n0;
        locals.var_t2_dn2 = assign32930_e55278_d_n2;
        locals.var_t2_dn3 = assign32930_e55278_d_n3;
        locals.var_t2_dn4 = assign32930_e55278_d_n4;
        locals.var_t2_dn5 = assign32930_e55278_d_n5;
        locals.var_t2_dn6 = assign32930_e55278_d_n6;
        locals.var_t2_dn7 = assign32930_e55278_d_n7;
        locals.var_t2_dn8 = assign32930_e55278_d_n8;
        locals.var_t2_dn9 = assign32930_e55278_d_n9;
        locals.var_t2_dn10 = assign32930_e55278_d_n10;
        locals.var_t2_dn11 = assign32930_e55278_d_n11;
        locals.var_t2_dn13 = assign32930_e55278_d_n13;
        locals.var_t2_dn14 = assign32930_e55278_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_128(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32940_e55288, assign32940_e55288_d_n0, assign32940_e55288_d_n2, assign32940_e55288_d_n3, assign32940_e55288_d_n4, assign32940_e55288_d_n5, assign32940_e55288_d_n6, assign32940_e55288_d_n7, assign32940_e55288_d_n8, assign32940_e55288_d_n9, assign32940_e55288_d_n10, assign32940_e55288_d_n11, assign32940_e55288_d_n13, assign32940_e55288_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32940_e55284: f64 = (locals.var_coxeff * locals.var_qis);
        let assign32940_e55286: f64 = (assign32940_e55284 / 1.60219e-19);
        (assign32940_e55286, (((locals.var_coxeff_dn0 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn0)) / 1.60219e-19), (((locals.var_coxeff_dn2 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn2)) / 1.60219e-19), (((locals.var_coxeff_dn3 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn8)) / 1.60219e-19), (((locals.var_coxeff_dn9 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn9)) / 1.60219e-19), (((locals.var_coxeff_dn10 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn10)) / 1.60219e-19), (((locals.var_coxeff_dn11 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn11)) / 1.60219e-19), (((locals.var_coxeff_dn13 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn13)) / 1.60219e-19), (((locals.var_coxeff_dn14 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32940_e55288;
        locals.var_n0_dn0 = assign32940_e55288_d_n0;
        locals.var_n0_dn2 = assign32940_e55288_d_n2;
        locals.var_n0_dn3 = assign32940_e55288_d_n3;
        locals.var_n0_dn4 = assign32940_e55288_d_n4;
        locals.var_n0_dn5 = assign32940_e55288_d_n5;
        locals.var_n0_dn6 = assign32940_e55288_d_n6;
        locals.var_n0_dn7 = assign32940_e55288_d_n7;
        locals.var_n0_dn8 = assign32940_e55288_d_n8;
        locals.var_n0_dn9 = assign32940_e55288_d_n9;
        locals.var_n0_dn10 = assign32940_e55288_d_n10;
        locals.var_n0_dn11 = assign32940_e55288_d_n11;
        locals.var_n0_dn13 = assign32940_e55288_d_n13;
        locals.var_n0_dn14 = assign32940_e55288_d_n14;
        locals.var_n0_rv = 0.0;

        let (assign32950_e55298, assign32950_e55298_d_n0, assign32950_e55298_d_n2, assign32950_e55298_d_n3, assign32950_e55298_d_n4, assign32950_e55298_d_n5, assign32950_e55298_d_n6, assign32950_e55298_d_n7, assign32950_e55298_d_n8, assign32950_e55298_d_n9, assign32950_e55298_d_n10, assign32950_e55298_d_n11, assign32950_e55298_d_n13, assign32950_e55298_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32950_e55294: f64 = (locals.var_coxeff * locals.var_qid);
        let assign32950_e55296: f64 = (assign32950_e55294 / 1.60219e-19);
        (assign32950_e55296, (((locals.var_coxeff_dn0 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn0)) / 1.60219e-19), (((locals.var_coxeff_dn2 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn2)) / 1.60219e-19), (((locals.var_coxeff_dn3 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn8)) / 1.60219e-19), (((locals.var_coxeff_dn9 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn9)) / 1.60219e-19), (((locals.var_coxeff_dn10 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn10)) / 1.60219e-19), (((locals.var_coxeff_dn11 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn11)) / 1.60219e-19), (((locals.var_coxeff_dn13 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn13)) / 1.60219e-19), (((locals.var_coxeff_dn14 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32950_e55298;
        locals.var_nl_dn0 = assign32950_e55298_d_n0;
        locals.var_nl_dn2 = assign32950_e55298_d_n2;
        locals.var_nl_dn3 = assign32950_e55298_d_n3;
        locals.var_nl_dn4 = assign32950_e55298_d_n4;
        locals.var_nl_dn5 = assign32950_e55298_d_n5;
        locals.var_nl_dn6 = assign32950_e55298_d_n6;
        locals.var_nl_dn7 = assign32950_e55298_d_n7;
        locals.var_nl_dn8 = assign32950_e55298_d_n8;
        locals.var_nl_dn9 = assign32950_e55298_d_n9;
        locals.var_nl_dn10 = assign32950_e55298_d_n10;
        locals.var_nl_dn11 = assign32950_e55298_d_n11;
        locals.var_nl_dn13 = assign32950_e55298_d_n13;
        locals.var_nl_dn14 = assign32950_e55298_d_n14;
        locals.var_nl_rv = 0.0;

        let (assign32960_e55310, assign32960_e55310_d_n0, assign32960_e55310_d_n2, assign32960_e55310_d_n3, assign32960_e55310_d_n4, assign32960_e55310_d_n5, assign32960_e55310_d_n6, assign32960_e55310_d_n7, assign32960_e55310_d_n8, assign32960_e55310_d_n9, assign32960_e55310_d_n10, assign32960_e55310_d_n11, assign32960_e55310_d_n13, assign32960_e55310_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32960_e55304: f64 = (locals.var_vtm / 1.60219e-19);
        let assign32960_e55307: f64 = (locals.var_coxeff + locals.var_cit_a);
        let assign32960_e55308: f64 = (assign32960_e55304 * assign32960_e55307);
        (assign32960_e55308, (assign32960_e55304 * (locals.var_coxeff_dn0 + locals.var_cit_a_dn0)), (assign32960_e55304 * (locals.var_coxeff_dn2 + locals.var_cit_a_dn2)), (assign32960_e55304 * (locals.var_coxeff_dn3 + locals.var_cit_a_dn3)), (((locals.var_vtm_dn4 / 1.60219e-19) * assign32960_e55307) + (assign32960_e55304 * (locals.var_coxeff_dn4 + locals.var_cit_a_dn4))), (assign32960_e55304 * (locals.var_coxeff_dn5 + locals.var_cit_a_dn5)), (assign32960_e55304 * (locals.var_coxeff_dn6 + locals.var_cit_a_dn6)), (assign32960_e55304 * (locals.var_coxeff_dn7 + locals.var_cit_a_dn7)), (assign32960_e55304 * (locals.var_coxeff_dn8 + locals.var_cit_a_dn8)), (assign32960_e55304 * (locals.var_coxeff_dn9 + locals.var_cit_a_dn9)), (assign32960_e55304 * (locals.var_coxeff_dn10 + locals.var_cit_a_dn10)), (assign32960_e55304 * (locals.var_coxeff_dn11 + locals.var_cit_a_dn11)), (assign32960_e55304 * (locals.var_coxeff_dn13 + locals.var_cit_a_dn13)), (assign32960_e55304 * (locals.var_coxeff_dn14 + locals.var_cit_a_dn14)),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32960_e55310;
        locals.var_nstar_dn0 = assign32960_e55310_d_n0;
        locals.var_nstar_dn2 = assign32960_e55310_d_n2;
        locals.var_nstar_dn3 = assign32960_e55310_d_n3;
        locals.var_nstar_dn4 = assign32960_e55310_d_n4;
        locals.var_nstar_dn5 = assign32960_e55310_d_n5;
        locals.var_nstar_dn6 = assign32960_e55310_d_n6;
        locals.var_nstar_dn7 = assign32960_e55310_d_n7;
        locals.var_nstar_dn8 = assign32960_e55310_d_n8;
        locals.var_nstar_dn9 = assign32960_e55310_d_n9;
        locals.var_nstar_dn10 = assign32960_e55310_d_n10;
        locals.var_nstar_dn11 = assign32960_e55310_d_n11;
        locals.var_nstar_dn13 = assign32960_e55310_d_n13;
        locals.var_nstar_dn14 = assign32960_e55310_d_n14;
        locals.var_nstar_rv = 0.0;

        let (assign32970_e55349, assign32970_e55349_d_n0, assign32970_e55349_d_n2, assign32970_e55349_d_n3, assign32970_e55349_d_n4, assign32970_e55349_d_n5, assign32970_e55349_d_n6, assign32970_e55349_d_n7, assign32970_e55349_d_n8, assign32970_e55349_d_n9, assign32970_e55349_d_n10, assign32970_e55349_d_n11, assign32970_e55349_d_n13, assign32970_e55349_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32970_e55317: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32970_e55320: f64 = (locals.var_nl + locals.var_nstar);
        let assign32970_e55321: f64 = (assign32970_e55317 / assign32970_e55320);
        let (assign32970_e55346, assign32970_e55346_d_n0, assign32970_e55346_d_n2, assign32970_e55346_d_n3, assign32970_e55346_d_n4, assign32970_e55346_d_n5, assign32970_e55346_d_n6, assign32970_e55346_d_n7, assign32970_e55346_d_n8, assign32970_e55346_d_n9, assign32970_e55346_d_n10, assign32970_e55346_d_n11, assign32970_e55346_d_n13, assign32970_e55346_d_n14,) = {
            if (!(assign32970_e55321 > 1e-38)) {
                let assign32970_e55326: f64 = (-87.498233534);
                (assign32970_e55326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign32970_e55329: f64 = (locals.var_n0 + locals.var_nstar);
                let assign32970_e55332: f64 = (locals.var_nl + locals.var_nstar);
                let assign32970_e55333: f64 = (assign32970_e55329 / assign32970_e55332);
                let (assign32970_e55345, assign32970_e55345_d_n0, assign32970_e55345_d_n2, assign32970_e55345_d_n3, assign32970_e55345_d_n4, assign32970_e55345_d_n5, assign32970_e55345_d_n6, assign32970_e55345_d_n7, assign32970_e55345_d_n8, assign32970_e55345_d_n9, assign32970_e55345_d_n10, assign32970_e55345_d_n11, assign32970_e55345_d_n13, assign32970_e55345_d_n14,) = {
                    if (assign32970_e55333 > 1e-38) {
                        let assign32970_e55338: f64 = (locals.var_n0 + locals.var_nstar);
                        let assign32970_e55341: f64 = (locals.var_nl + locals.var_nstar);
                        let assign32970_e55342: f64 = (assign32970_e55338 / assign32970_e55341);
                        let assign32970_e55343: f64 = (assign32970_e55342).ln();
                        (assign32970_e55343, (((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342), (((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32970_e55341) - (assign32970_e55338 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32970_e55341 * assign32970_e55341)) / assign32970_e55342),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign32970_e55345, assign32970_e55345_d_n0, assign32970_e55345_d_n2, assign32970_e55345_d_n3, assign32970_e55345_d_n4, assign32970_e55345_d_n5, assign32970_e55345_d_n6, assign32970_e55345_d_n7, assign32970_e55345_d_n8, assign32970_e55345_d_n9, assign32970_e55345_d_n10, assign32970_e55345_d_n11, assign32970_e55345_d_n13, assign32970_e55345_d_n14,)
            }
        };
        let assign32970_e55347: f64 = (locals.var_noiaeff * assign32970_e55346);
        (assign32970_e55347, ((locals.var_noiaeff_dn0 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n0)), ((locals.var_noiaeff_dn2 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n2)), ((locals.var_noiaeff_dn3 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n3)), ((locals.var_noiaeff_dn4 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n4)), ((locals.var_noiaeff_dn5 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n5)), ((locals.var_noiaeff_dn6 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n6)), ((locals.var_noiaeff_dn7 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n7)), ((locals.var_noiaeff_dn8 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n8)), ((locals.var_noiaeff_dn9 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n9)), ((locals.var_noiaeff_dn10 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n10)), ((locals.var_noiaeff_dn11 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n11)), ((locals.var_noiaeff_dn13 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n13)), ((locals.var_noiaeff_dn14 * assign32970_e55346) + (locals.var_noiaeff * assign32970_e55346_d_n14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32970_e55349;
        locals.var_t3_dn0 = assign32970_e55349_d_n0;
        locals.var_t3_dn2 = assign32970_e55349_d_n2;
        locals.var_t3_dn3 = assign32970_e55349_d_n3;
        locals.var_t3_dn4 = assign32970_e55349_d_n4;
        locals.var_t3_dn5 = assign32970_e55349_d_n5;
        locals.var_t3_dn6 = assign32970_e55349_d_n6;
        locals.var_t3_dn7 = assign32970_e55349_d_n7;
        locals.var_t3_dn8 = assign32970_e55349_d_n8;
        locals.var_t3_dn9 = assign32970_e55349_d_n9;
        locals.var_t3_dn10 = assign32970_e55349_d_n10;
        locals.var_t3_dn11 = assign32970_e55349_d_n11;
        locals.var_t3_dn13 = assign32970_e55349_d_n13;
        locals.var_t3_dn14 = assign32970_e55349_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32980_e55359, assign32980_e55359_d_n0, assign32980_e55359_d_n2, assign32980_e55359_d_n3, assign32980_e55359_d_n4, assign32980_e55359_d_n5, assign32980_e55359_d_n6, assign32980_e55359_d_n7, assign32980_e55359_d_n8, assign32980_e55359_d_n9, assign32980_e55359_d_n10, assign32980_e55359_d_n11, assign32980_e55359_d_n13, assign32980_e55359_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32980_e55356: f64 = (locals.var_n0 - locals.var_nl);
        let assign32980_e55357: f64 = (p.p1683 * assign32980_e55356);
        (assign32980_e55357, (p.p1683 * (locals.var_n0_dn0 - locals.var_nl_dn0)), (p.p1683 * (locals.var_n0_dn2 - locals.var_nl_dn2)), (p.p1683 * (locals.var_n0_dn3 - locals.var_nl_dn3)), (p.p1683 * (locals.var_n0_dn4 - locals.var_nl_dn4)), (p.p1683 * (locals.var_n0_dn5 - locals.var_nl_dn5)), (p.p1683 * (locals.var_n0_dn6 - locals.var_nl_dn6)), (p.p1683 * (locals.var_n0_dn7 - locals.var_nl_dn7)), (p.p1683 * (locals.var_n0_dn8 - locals.var_nl_dn8)), (p.p1683 * (locals.var_n0_dn9 - locals.var_nl_dn9)), (p.p1683 * (locals.var_n0_dn10 - locals.var_nl_dn10)), (p.p1683 * (locals.var_n0_dn11 - locals.var_nl_dn11)), (p.p1683 * (locals.var_n0_dn13 - locals.var_nl_dn13)), (p.p1683 * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32980_e55359;
        locals.var_t4_dn0 = assign32980_e55359_d_n0;
        locals.var_t4_dn2 = assign32980_e55359_d_n2;
        locals.var_t4_dn3 = assign32980_e55359_d_n3;
        locals.var_t4_dn4 = assign32980_e55359_d_n4;
        locals.var_t4_dn5 = assign32980_e55359_d_n5;
        locals.var_t4_dn6 = assign32980_e55359_d_n6;
        locals.var_t4_dn7 = assign32980_e55359_d_n7;
        locals.var_t4_dn8 = assign32980_e55359_d_n8;
        locals.var_t4_dn9 = assign32980_e55359_d_n9;
        locals.var_t4_dn10 = assign32980_e55359_d_n10;
        locals.var_t4_dn11 = assign32980_e55359_d_n11;
        locals.var_t4_dn13 = assign32980_e55359_d_n13;
        locals.var_t4_dn14 = assign32980_e55359_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32990_e55375, assign32990_e55375_d_n0, assign32990_e55375_d_n2, assign32990_e55375_d_n3, assign32990_e55375_d_n4, assign32990_e55375_d_n5, assign32990_e55375_d_n6, assign32990_e55375_d_n7, assign32990_e55375_d_n8, assign32990_e55375_d_n9, assign32990_e55375_d_n10, assign32990_e55375_d_n11, assign32990_e55375_d_n13, assign32990_e55375_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign32990_e55365: f64 = (0.5 * p.p1684);
        let assign32990_e55368: f64 = (locals.var_n0 * locals.var_n0);
        let assign32990_e55371: f64 = (locals.var_nl * locals.var_nl);
        let assign32990_e55372: f64 = (assign32990_e55368 - assign32990_e55371);
        let assign32990_e55373: f64 = (assign32990_e55365 * assign32990_e55372);
        (assign32990_e55373, (assign32990_e55365 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32990_e55365 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32990_e55365 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32990_e55365 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32990_e55365 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32990_e55365 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32990_e55365 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32990_e55365 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32990_e55365 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32990_e55365 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32990_e55365 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32990_e55365 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32990_e55365 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32990_e55375;
        locals.var_t5_dn0 = assign32990_e55375_d_n0;
        locals.var_t5_dn2 = assign32990_e55375_d_n2;
        locals.var_t5_dn3 = assign32990_e55375_d_n3;
        locals.var_t5_dn4 = assign32990_e55375_d_n4;
        locals.var_t5_dn5 = assign32990_e55375_d_n5;
        locals.var_t5_dn6 = assign32990_e55375_d_n6;
        locals.var_t5_dn7 = assign32990_e55375_d_n7;
        locals.var_t5_dn8 = assign32990_e55375_d_n8;
        locals.var_t5_dn9 = assign32990_e55375_d_n9;
        locals.var_t5_dn10 = assign32990_e55375_d_n10;
        locals.var_t5_dn11 = assign32990_e55375_d_n11;
        locals.var_t5_dn13 = assign32990_e55375_d_n13;
        locals.var_t5_dn14 = assign32990_e55375_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33000_e55387, assign33000_e55387_d_n0, assign33000_e55387_d_n2, assign33000_e55387_d_n3, assign33000_e55387_d_n4, assign33000_e55387_d_n5, assign33000_e55387_d_n6, assign33000_e55387_d_n7, assign33000_e55387_d_n8, assign33000_e55387_d_n9, assign33000_e55387_d_n10, assign33000_e55387_d_n11, assign33000_e55387_d_n13, assign33000_e55387_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33000_e55381: f64 = (1.60219e-19 * locals.var_vtm);
        let assign33000_e55383: f64 = (assign33000_e55381 * locals.var_ids_v);
        let assign33000_e55385: f64 = (assign33000_e55383 * locals.var_ids_v);
        (assign33000_e55385, (((assign33000_e55381 * locals.var_ids_v_dn0) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn0)), (((assign33000_e55381 * locals.var_ids_v_dn2) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn2)), (((assign33000_e55381 * locals.var_ids_v_dn3) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn3)), (((((1.60219e-19 * locals.var_vtm_dn4) * locals.var_ids_v) + (assign33000_e55381 * locals.var_ids_v_dn4)) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn4)), (((assign33000_e55381 * locals.var_ids_v_dn5) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn5)), (((assign33000_e55381 * locals.var_ids_v_dn6) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn6)), (((assign33000_e55381 * locals.var_ids_v_dn7) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn7)), (((assign33000_e55381 * locals.var_ids_v_dn8) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn8)), (((assign33000_e55381 * locals.var_ids_v_dn9) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn9)), (((assign33000_e55381 * locals.var_ids_v_dn10) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn10)), (((assign33000_e55381 * locals.var_ids_v_dn11) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn11)), (((assign33000_e55381 * locals.var_ids_v_dn13) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn13)), (((assign33000_e55381 * locals.var_ids_v_dn14) * locals.var_ids_v) + (assign33000_e55383 * locals.var_ids_v_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33000_e55387;
        locals.var_t6_dn0 = assign33000_e55387_d_n0;
        locals.var_t6_dn2 = assign33000_e55387_d_n2;
        locals.var_t6_dn3 = assign33000_e55387_d_n3;
        locals.var_t6_dn4 = assign33000_e55387_d_n4;
        locals.var_t6_dn5 = assign33000_e55387_d_n5;
        locals.var_t6_dn6 = assign33000_e55387_d_n6;
        locals.var_t6_dn7 = assign33000_e55387_d_n7;
        locals.var_t6_dn8 = assign33000_e55387_d_n8;
        locals.var_t6_dn9 = assign33000_e55387_d_n9;
        locals.var_t6_dn10 = assign33000_e55387_d_n10;
        locals.var_t6_dn11 = assign33000_e55387_d_n11;
        locals.var_t6_dn13 = assign33000_e55387_d_n13;
        locals.var_t6_dn14 = assign33000_e55387_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign33010_e55399, assign33010_e55399_d_n0, assign33010_e55399_d_n2, assign33010_e55399_d_n3, assign33010_e55399_d_n4, assign33010_e55399_d_n5, assign33010_e55399_d_n6, assign33010_e55399_d_n7, assign33010_e55399_d_n8, assign33010_e55399_d_n9, assign33010_e55399_d_n10, assign33010_e55399_d_n11, assign33010_e55399_d_n13, assign33010_e55399_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33010_e55393: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign33010_e55395: f64 = (assign33010_e55393 * locals.var_weff0);
        let assign33010_e55397: f64 = (assign33010_e55395 * locals.var_nfintotal);
        (assign33010_e55397, (((10000000000.0 * locals.var_leffnoisq_dn0) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn2) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn3) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn4) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn5) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn6) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn7) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn8) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn9) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn10) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn11) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn13) * locals.var_weff0) * locals.var_nfintotal), (((10000000000.0 * locals.var_leffnoisq_dn14) * locals.var_weff0) * locals.var_nfintotal),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e55399;
        locals.var_t7_dn0 = assign33010_e55399_d_n0;
        locals.var_t7_dn2 = assign33010_e55399_d_n2;
        locals.var_t7_dn3 = assign33010_e55399_d_n3;
        locals.var_t7_dn4 = assign33010_e55399_d_n4;
        locals.var_t7_dn5 = assign33010_e55399_d_n5;
        locals.var_t7_dn6 = assign33010_e55399_d_n6;
        locals.var_t7_dn7 = assign33010_e55399_d_n7;
        locals.var_t7_dn8 = assign33010_e55399_d_n8;
        locals.var_t7_dn9 = assign33010_e55399_d_n9;
        locals.var_t7_dn10 = assign33010_e55399_d_n10;
        locals.var_t7_dn11 = assign33010_e55399_d_n11;
        locals.var_t7_dn13 = assign33010_e55399_d_n13;
        locals.var_t7_dn14 = assign33010_e55399_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign33020_e55415, assign33020_e55415_d_n0, assign33020_e55415_d_n2, assign33020_e55415_d_n3, assign33020_e55415_d_n4, assign33020_e55415_d_n5, assign33020_e55415_d_n6, assign33020_e55415_d_n7, assign33020_e55415_d_n8, assign33020_e55415_d_n9, assign33020_e55415_d_n10, assign33020_e55415_d_n11, assign33020_e55415_d_n13, assign33020_e55415_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33020_e55406: f64 = (p.p1683 * locals.var_nl);
        let assign33020_e55407: f64 = (locals.var_noiaeff + assign33020_e55406);
        let assign33020_e55410: f64 = (p.p1684 * locals.var_nl);
        let assign33020_e55412: f64 = (assign33020_e55410 * locals.var_nl);
        let assign33020_e55413: f64 = (assign33020_e55407 + assign33020_e55412);
        (assign33020_e55413, ((locals.var_noiaeff_dn0 + (p.p1683 * locals.var_nl_dn0)) + (((p.p1684 * locals.var_nl_dn0) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn0))), ((locals.var_noiaeff_dn2 + (p.p1683 * locals.var_nl_dn2)) + (((p.p1684 * locals.var_nl_dn2) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn2))), ((locals.var_noiaeff_dn3 + (p.p1683 * locals.var_nl_dn3)) + (((p.p1684 * locals.var_nl_dn3) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn3))), ((locals.var_noiaeff_dn4 + (p.p1683 * locals.var_nl_dn4)) + (((p.p1684 * locals.var_nl_dn4) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn4))), ((locals.var_noiaeff_dn5 + (p.p1683 * locals.var_nl_dn5)) + (((p.p1684 * locals.var_nl_dn5) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn5))), ((locals.var_noiaeff_dn6 + (p.p1683 * locals.var_nl_dn6)) + (((p.p1684 * locals.var_nl_dn6) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn6))), ((locals.var_noiaeff_dn7 + (p.p1683 * locals.var_nl_dn7)) + (((p.p1684 * locals.var_nl_dn7) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn7))), ((locals.var_noiaeff_dn8 + (p.p1683 * locals.var_nl_dn8)) + (((p.p1684 * locals.var_nl_dn8) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn8))), ((locals.var_noiaeff_dn9 + (p.p1683 * locals.var_nl_dn9)) + (((p.p1684 * locals.var_nl_dn9) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn9))), ((locals.var_noiaeff_dn10 + (p.p1683 * locals.var_nl_dn10)) + (((p.p1684 * locals.var_nl_dn10) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn10))), ((locals.var_noiaeff_dn11 + (p.p1683 * locals.var_nl_dn11)) + (((p.p1684 * locals.var_nl_dn11) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn11))), ((locals.var_noiaeff_dn13 + (p.p1683 * locals.var_nl_dn13)) + (((p.p1684 * locals.var_nl_dn13) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn13))), ((locals.var_noiaeff_dn14 + (p.p1683 * locals.var_nl_dn14)) + (((p.p1684 * locals.var_nl_dn14) * locals.var_nl) + (assign33020_e55410 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e55415;
        locals.var_t8_dn0 = assign33020_e55415_d_n0;
        locals.var_t8_dn2 = assign33020_e55415_d_n2;
        locals.var_t8_dn3 = assign33020_e55415_d_n3;
        locals.var_t8_dn4 = assign33020_e55415_d_n4;
        locals.var_t8_dn5 = assign33020_e55415_d_n5;
        locals.var_t8_dn6 = assign33020_e55415_d_n6;
        locals.var_t8_dn7 = assign33020_e55415_d_n7;
        locals.var_t8_dn8 = assign33020_e55415_d_n8;
        locals.var_t8_dn9 = assign33020_e55415_d_n9;
        locals.var_t8_dn10 = assign33020_e55415_d_n10;
        locals.var_t8_dn11 = assign33020_e55415_d_n11;
        locals.var_t8_dn13 = assign33020_e55415_d_n13;
        locals.var_t8_dn14 = assign33020_e55415_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign33030_e55427, assign33030_e55427_d_n0, assign33030_e55427_d_n2, assign33030_e55427_d_n3, assign33030_e55427_d_n4, assign33030_e55427_d_n5, assign33030_e55427_d_n6, assign33030_e55427_d_n7, assign33030_e55427_d_n8, assign33030_e55427_d_n9, assign33030_e55427_d_n10, assign33030_e55427_d_n11, assign33030_e55427_d_n13, assign33030_e55427_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33030_e55421: f64 = (locals.var_nl + locals.var_nstar);
        let assign33030_e55424: f64 = (locals.var_nl + locals.var_nstar);
        let assign33030_e55425: f64 = (assign33030_e55421 * assign33030_e55424);
        (assign33030_e55425, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign33030_e55424) + (assign33030_e55421 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn13, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign33030_e55427;
        locals.var_t9_dn0 = assign33030_e55427_d_n0;
        locals.var_t9_dn2 = assign33030_e55427_d_n2;
        locals.var_t9_dn3 = assign33030_e55427_d_n3;
        locals.var_t9_dn4 = assign33030_e55427_d_n4;
        locals.var_t9_dn5 = assign33030_e55427_d_n5;
        locals.var_t9_dn6 = assign33030_e55427_d_n6;
        locals.var_t9_dn7 = assign33030_e55427_d_n7;
        locals.var_t9_dn8 = assign33030_e55427_d_n8;
        locals.var_t9_dn9 = assign33030_e55427_d_n9;
        locals.var_t9_dn10 = assign33030_e55427_d_n10;
        locals.var_t9_dn11 = assign33030_e55427_d_n11;
        locals.var_t9_dn13 = assign33030_e55427_d_n13;
        locals.var_t9_dn14 = assign33030_e55427_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign33040_e55451, assign33040_e55451_d_n0, assign33040_e55451_d_n2, assign33040_e55451_d_n3, assign33040_e55451_d_n4, assign33040_e55451_d_n5, assign33040_e55451_d_n6, assign33040_e55451_d_n7, assign33040_e55451_d_n8, assign33040_e55451_d_n9, assign33040_e55451_d_n10, assign33040_e55451_d_n11, assign33040_e55451_d_n13, assign33040_e55451_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33040_e55433: f64 = (locals.var_t1 / locals.var_t2);
        let assign33040_e55436: f64 = (locals.var_t3 + locals.var_t4);
        let assign33040_e55438: f64 = (assign33040_e55436 + locals.var_t5);
        let assign33040_e55439: f64 = (assign33040_e55433 * assign33040_e55438);
        let assign33040_e55442: f64 = (locals.var_t6 / locals.var_t7);
        let assign33040_e55444: f64 = (assign33040_e55442 * locals.var_delclm);
        let assign33040_e55446: f64 = (assign33040_e55444 * locals.var_t8);
        let assign33040_e55448: f64 = (assign33040_e55446 / locals.var_t9);
        let assign33040_e55449: f64 = (assign33040_e55439 + assign33040_e55448);
        (assign33040_e55449, ((((((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn0 + locals.var_t4_dn0) + locals.var_t5_dn0))) + ((((((((((locals.var_t6_dn0 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn0)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn0)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn0)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn2 + locals.var_t4_dn2) + locals.var_t5_dn2))) + ((((((((((locals.var_t6_dn2 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn2)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn2)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn2)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn3 + locals.var_t4_dn3) + locals.var_t5_dn3))) + ((((((((((locals.var_t6_dn3 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn3)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn3)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn3)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn3)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn4 + locals.var_t4_dn4) + locals.var_t5_dn4))) + ((((((((((locals.var_t6_dn4 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn4)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn4)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn4)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn5 + locals.var_t4_dn5) + locals.var_t5_dn5))) + ((((((((((locals.var_t6_dn5 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn5)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn5)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn5)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn6 + locals.var_t4_dn6) + locals.var_t5_dn6))) + ((((((((((locals.var_t6_dn6 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn6)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn6)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn6)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn7 + locals.var_t4_dn7) + locals.var_t5_dn7))) + ((((((((((locals.var_t6_dn7 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn7)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn7)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn7)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn8 + locals.var_t4_dn8) + locals.var_t5_dn8))) + ((((((((((locals.var_t6_dn8 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn8)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn8)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn8)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn9 + locals.var_t4_dn9) + locals.var_t5_dn9))) + ((((((((((locals.var_t6_dn9 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn9)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn9)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn9)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn10 + locals.var_t4_dn10) + locals.var_t5_dn10))) + ((((((((((locals.var_t6_dn10 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn10)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn10)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn10)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn11 + locals.var_t4_dn11) + locals.var_t5_dn11))) + ((((((((((locals.var_t6_dn11 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn11)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn11)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn11)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn13 + locals.var_t4_dn13) + locals.var_t5_dn13))) + ((((((((((locals.var_t6_dn13 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn13)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn13)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn13)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) * assign33040_e55438) + (assign33040_e55433 * ((locals.var_t3_dn14 + locals.var_t4_dn14) + locals.var_t5_dn14))) + ((((((((((locals.var_t6_dn14 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn14)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign33040_e55442 * locals.var_delclm_dn14)) * locals.var_t8) + (assign33040_e55444 * locals.var_t8_dn14)) * locals.var_t9) - (assign33040_e55446 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign33040_e55451;
        locals.var_ssi_dn0 = assign33040_e55451_d_n0;
        locals.var_ssi_dn2 = assign33040_e55451_d_n2;
        locals.var_ssi_dn3 = assign33040_e55451_d_n3;
        locals.var_ssi_dn4 = assign33040_e55451_d_n4;
        locals.var_ssi_dn5 = assign33040_e55451_d_n5;
        locals.var_ssi_dn6 = assign33040_e55451_d_n6;
        locals.var_ssi_dn7 = assign33040_e55451_d_n7;
        locals.var_ssi_dn8 = assign33040_e55451_d_n8;
        locals.var_ssi_dn9 = assign33040_e55451_d_n9;
        locals.var_ssi_dn10 = assign33040_e55451_d_n10;
        locals.var_ssi_dn11 = assign33040_e55451_d_n11;
        locals.var_ssi_dn13 = assign33040_e55451_d_n13;
        locals.var_ssi_dn14 = assign33040_e55451_d_n14;
        locals.var_ssi_rv = 0.0;

        let (assign33050_e55461, assign33050_e55461_d_n0, assign33050_e55461_d_n2, assign33050_e55461_d_n3, assign33050_e55461_d_n4, assign33050_e55461_d_n5, assign33050_e55461_d_n6, assign33050_e55461_d_n7, assign33050_e55461_d_n8, assign33050_e55461_d_n9, assign33050_e55461_d_n10, assign33050_e55461_d_n11, assign33050_e55461_d_n13, assign33050_e55461_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33050_e55457: f64 = (locals.var_noiaeff * 1.60219e-19);
        let assign33050_e55459: f64 = (assign33050_e55457 * locals.var_vtm);
        (assign33050_e55459, ((locals.var_noiaeff_dn0 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn2 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn3 * 1.60219e-19) * locals.var_vtm), (((locals.var_noiaeff_dn4 * 1.60219e-19) * locals.var_vtm) + (assign33050_e55457 * locals.var_vtm_dn4)), ((locals.var_noiaeff_dn5 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn6 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn7 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn8 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn9 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn10 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn11 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn13 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn14 * 1.60219e-19) * locals.var_vtm),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn13, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign33050_e55461;
        locals.var_t10_dn0 = assign33050_e55461_d_n0;
        locals.var_t10_dn2 = assign33050_e55461_d_n2;
        locals.var_t10_dn3 = assign33050_e55461_d_n3;
        locals.var_t10_dn4 = assign33050_e55461_d_n4;
        locals.var_t10_dn5 = assign33050_e55461_d_n5;
        locals.var_t10_dn6 = assign33050_e55461_d_n6;
        locals.var_t10_dn7 = assign33050_e55461_d_n7;
        locals.var_t10_dn8 = assign33050_e55461_d_n8;
        locals.var_t10_dn9 = assign33050_e55461_d_n9;
        locals.var_t10_dn10 = assign33050_e55461_d_n10;
        locals.var_t10_dn11 = assign33050_e55461_d_n11;
        locals.var_t10_dn13 = assign33050_e55461_d_n13;
        locals.var_t10_dn14 = assign33050_e55461_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign33060_e55477, assign33060_e55477_d_n0, assign33060_e55477_d_n2, assign33060_e55477_d_n3, assign33060_e55477_d_n4, assign33060_e55477_d_n5, assign33060_e55477_d_n6, assign33060_e55477_d_n7, assign33060_e55477_d_n8, assign33060_e55477_d_n9, assign33060_e55477_d_n10, assign33060_e55477_d_n11, assign33060_e55477_d_n13, assign33060_e55477_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33060_e55467: f64 = (locals.var_weff0 * locals.var_nfintotal);
        let assign33060_e55469: f64 = (assign33060_e55467 * locals.var_leffnoi);
        let assign33060_e55471: f64 = (assign33060_e55469 * 10000000000.0);
        let assign33060_e55473: f64 = (assign33060_e55471 * locals.var_nstar);
        let assign33060_e55475: f64 = (assign33060_e55473 * locals.var_nstar);
        (assign33060_e55475, ((((((assign33060_e55467 * locals.var_leffnoi_dn0) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn0)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn0)), ((((((assign33060_e55467 * locals.var_leffnoi_dn2) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn2)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn2)), ((((((assign33060_e55467 * locals.var_leffnoi_dn3) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn3)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn3)), ((((((assign33060_e55467 * locals.var_leffnoi_dn4) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn4)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn4)), ((((((assign33060_e55467 * locals.var_leffnoi_dn5) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn5)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn5)), ((((((assign33060_e55467 * locals.var_leffnoi_dn6) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn6)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn6)), ((((((assign33060_e55467 * locals.var_leffnoi_dn7) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn7)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn7)), ((((((assign33060_e55467 * locals.var_leffnoi_dn8) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn8)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn8)), ((((((assign33060_e55467 * locals.var_leffnoi_dn9) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn9)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn9)), ((((((assign33060_e55467 * locals.var_leffnoi_dn10) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn10)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn10)), ((((((assign33060_e55467 * locals.var_leffnoi_dn11) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn11)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn11)), ((((((assign33060_e55467 * locals.var_leffnoi_dn13) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn13)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn13)), ((((((assign33060_e55467 * locals.var_leffnoi_dn14) * 10000000000.0) * locals.var_nstar) + (assign33060_e55471 * locals.var_nstar_dn14)) * locals.var_nstar) + (assign33060_e55473 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn13, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign33060_e55477;
        locals.var_t11_dn0 = assign33060_e55477_d_n0;
        locals.var_t11_dn2 = assign33060_e55477_d_n2;
        locals.var_t11_dn3 = assign33060_e55477_d_n3;
        locals.var_t11_dn4 = assign33060_e55477_d_n4;
        locals.var_t11_dn5 = assign33060_e55477_d_n5;
        locals.var_t11_dn6 = assign33060_e55477_d_n6;
        locals.var_t11_dn7 = assign33060_e55477_d_n7;
        locals.var_t11_dn8 = assign33060_e55477_d_n8;
        locals.var_t11_dn9 = assign33060_e55477_d_n9;
        locals.var_t11_dn10 = assign33060_e55477_d_n10;
        locals.var_t11_dn11 = assign33060_e55477_d_n11;
        locals.var_t11_dn13 = assign33060_e55477_d_n13;
        locals.var_t11_dn14 = assign33060_e55477_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign33070_e55489, assign33070_e55489_d_n0, assign33070_e55489_d_n2, assign33070_e55489_d_n3, assign33070_e55489_d_n4, assign33070_e55489_d_n5, assign33070_e55489_d_n6, assign33070_e55489_d_n7, assign33070_e55489_d_n8, assign33070_e55489_d_n9, assign33070_e55489_d_n10, assign33070_e55489_d_n11, assign33070_e55489_d_n13, assign33070_e55489_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33070_e55483: f64 = (locals.var_t10 / locals.var_t11);
        let assign33070_e55485: f64 = (assign33070_e55483 * locals.var_ids_v);
        let assign33070_e55487: f64 = (assign33070_e55485 * locals.var_ids_v);
        (assign33070_e55487, (((((((locals.var_t10_dn0 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn0)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn0)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn0)), (((((((locals.var_t10_dn2 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn2)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn2)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn2)), (((((((locals.var_t10_dn3 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn3)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn3)), (((((((locals.var_t10_dn4 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn4)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn4)), (((((((locals.var_t10_dn5 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn5)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn5)), (((((((locals.var_t10_dn6 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn6)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn6)), (((((((locals.var_t10_dn7 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn7)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn7)), (((((((locals.var_t10_dn8 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn8)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn8)), (((((((locals.var_t10_dn9 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn9)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn9)), (((((((locals.var_t10_dn10 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn10)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn10)), (((((((locals.var_t10_dn11 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn11)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn11)), (((((((locals.var_t10_dn13 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn13)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn13)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn13)), (((((((locals.var_t10_dn14 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn14)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids_v) + (assign33070_e55483 * locals.var_ids_v_dn14)) * locals.var_ids_v) + (assign33070_e55485 * locals.var_ids_v_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign33070_e55489;
        locals.var_swi_dn0 = assign33070_e55489_d_n0;
        locals.var_swi_dn2 = assign33070_e55489_d_n2;
        locals.var_swi_dn3 = assign33070_e55489_d_n3;
        locals.var_swi_dn4 = assign33070_e55489_d_n4;
        locals.var_swi_dn5 = assign33070_e55489_d_n5;
        locals.var_swi_dn6 = assign33070_e55489_d_n6;
        locals.var_swi_dn7 = assign33070_e55489_d_n7;
        locals.var_swi_dn8 = assign33070_e55489_d_n8;
        locals.var_swi_dn9 = assign33070_e55489_d_n9;
        locals.var_swi_dn10 = assign33070_e55489_d_n10;
        locals.var_swi_dn11 = assign33070_e55489_d_n11;
        locals.var_swi_dn13 = assign33070_e55489_d_n13;
        locals.var_swi_dn14 = assign33070_e55489_d_n14;
        locals.var_swi_rv = 0.0;

        let (assign33080_e55497, assign33080_e55497_d_n0, assign33080_e55497_d_n2, assign33080_e55497_d_n3, assign33080_e55497_d_n4, assign33080_e55497_d_n5, assign33080_e55497_d_n6, assign33080_e55497_d_n7, assign33080_e55497_d_n8, assign33080_e55497_d_n9, assign33080_e55497_d_n10, assign33080_e55497_d_n11, assign33080_e55497_d_n13, assign33080_e55497_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard626 != 0.0)) {
        let assign33080_e55495: f64 = (locals.var_swi + locals.var_ssi);
        (assign33080_e55495, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33080_e55497;
        locals.var_t1_dn0 = assign33080_e55497_d_n0;
        locals.var_t1_dn2 = assign33080_e55497_d_n2;
        locals.var_t1_dn3 = assign33080_e55497_d_n3;
        locals.var_t1_dn4 = assign33080_e55497_d_n4;
        locals.var_t1_dn5 = assign33080_e55497_d_n5;
        locals.var_t1_dn6 = assign33080_e55497_d_n6;
        locals.var_t1_dn7 = assign33080_e55497_d_n7;
        locals.var_t1_dn8 = assign33080_e55497_d_n8;
        locals.var_t1_dn9 = assign33080_e55497_d_n9;
        locals.var_t1_dn10 = assign33080_e55497_d_n10;
        locals.var_t1_dn11 = assign33080_e55497_d_n11;
        locals.var_t1_dn13 = assign33080_e55497_d_n13;
        locals.var_t1_dn14 = assign33080_e55497_d_n14;
        locals.var_t1_rv = 0.0;

        let assign33120_e55524: f64 = if p.p79 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign33120_e55524;
        locals.var_guard630_rv = 0.0;

        let (assign33130_e55535, assign33130_e55535_d_n0, assign33130_e55535_d_n2, assign33130_e55535_d_n3, assign33130_e55535_d_n4, assign33130_e55535_d_n5, assign33130_e55535_d_n6, assign33130_e55535_d_n7, assign33130_e55535_d_n8, assign33130_e55535_d_n9, assign33130_e55535_d_n10, assign33130_e55535_d_n11, assign33130_e55535_d_n13, assign33130_e55535_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33130_e55533: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign33130_e55533, (locals.var_qia2_dn0 / locals.var_qsref_i), (locals.var_qia2_dn2 / locals.var_qsref_i), (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i), (locals.var_qia2_dn9 / locals.var_qsref_i), (locals.var_qia2_dn10 / locals.var_qsref_i), (locals.var_qia2_dn11 / locals.var_qsref_i), (locals.var_qia2_dn13 / locals.var_qsref_i), (locals.var_qia2_dn14 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33130_e55535;
        locals.var_t1_dn0 = assign33130_e55535_d_n0;
        locals.var_t1_dn2 = assign33130_e55535_d_n2;
        locals.var_t1_dn3 = assign33130_e55535_d_n3;
        locals.var_t1_dn4 = assign33130_e55535_d_n4;
        locals.var_t1_dn5 = assign33130_e55535_d_n5;
        locals.var_t1_dn6 = assign33130_e55535_d_n6;
        locals.var_t1_dn7 = assign33130_e55535_d_n7;
        locals.var_t1_dn8 = assign33130_e55535_d_n8;
        locals.var_t1_dn9 = assign33130_e55535_d_n9;
        locals.var_t1_dn10 = assign33130_e55535_d_n10;
        locals.var_t1_dn11 = assign33130_e55535_d_n11;
        locals.var_t1_dn13 = assign33130_e55535_d_n13;
        locals.var_t1_dn14 = assign33130_e55535_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign33140_e55548, assign33140_e55548_d_n0, assign33140_e55548_d_n2, assign33140_e55548_d_n3, assign33140_e55548_d_n4, assign33140_e55548_d_n5, assign33140_e55548_d_n6, assign33140_e55548_d_n7, assign33140_e55548_d_n8, assign33140_e55548_d_n9, assign33140_e55548_d_n10, assign33140_e55548_d_n11, assign33140_e55548_d_n13, assign33140_e55548_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33140_e55545: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign33140_e55546: f64 = (1.0 + assign33140_e55545);
        (assign33140_e55546, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn0)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn2)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn9)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn10)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn11)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn13)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn13 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn14)) } } else { (assign33140_e55545 * (locals.var_mpower_i * (locals.var_t1_dn14 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33140_e55548;
        locals.var_t2_dn0 = assign33140_e55548_d_n0;
        locals.var_t2_dn2 = assign33140_e55548_d_n2;
        locals.var_t2_dn3 = assign33140_e55548_d_n3;
        locals.var_t2_dn4 = assign33140_e55548_d_n4;
        locals.var_t2_dn5 = assign33140_e55548_d_n5;
        locals.var_t2_dn6 = assign33140_e55548_d_n6;
        locals.var_t2_dn7 = assign33140_e55548_d_n7;
        locals.var_t2_dn8 = assign33140_e55548_d_n8;
        locals.var_t2_dn9 = assign33140_e55548_d_n9;
        locals.var_t2_dn10 = assign33140_e55548_d_n10;
        locals.var_t2_dn11 = assign33140_e55548_d_n11;
        locals.var_t2_dn13 = assign33140_e55548_d_n13;
        locals.var_t2_dn14 = assign33140_e55548_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33150_e55559, assign33150_e55559_d_n0, assign33150_e55559_d_n2, assign33150_e55559_d_n3, assign33150_e55559_d_n4, assign33150_e55559_d_n5, assign33150_e55559_d_n6, assign33150_e55559_d_n7, assign33150_e55559_d_n8, assign33150_e55559_d_n9, assign33150_e55559_d_n10, assign33150_e55559_d_n11, assign33150_e55559_d_n13, assign33150_e55559_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33150_e55557: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign33150_e55557, (-((locals.var_noia2_i * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33150_e55559;
        locals.var_t3_dn0 = assign33150_e55559_d_n0;
        locals.var_t3_dn2 = assign33150_e55559_d_n2;
        locals.var_t3_dn3 = assign33150_e55559_d_n3;
        locals.var_t3_dn4 = assign33150_e55559_d_n4;
        locals.var_t3_dn5 = assign33150_e55559_d_n5;
        locals.var_t3_dn6 = assign33150_e55559_d_n6;
        locals.var_t3_dn7 = assign33150_e55559_d_n7;
        locals.var_t3_dn8 = assign33150_e55559_d_n8;
        locals.var_t3_dn9 = assign33150_e55559_d_n9;
        locals.var_t3_dn10 = assign33150_e55559_d_n10;
        locals.var_t3_dn11 = assign33150_e55559_d_n11;
        locals.var_t3_dn13 = assign33150_e55559_d_n13;
        locals.var_t3_dn14 = assign33150_e55559_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33160_e55570, assign33160_e55570_d_n0, assign33160_e55570_d_n2, assign33160_e55570_d_n3, assign33160_e55570_d_n4, assign33160_e55570_d_n5, assign33160_e55570_d_n6, assign33160_e55570_d_n7, assign33160_e55570_d_n8, assign33160_e55570_d_n9, assign33160_e55570_d_n10, assign33160_e55570_d_n11, assign33160_e55570_d_n13, assign33160_e55570_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33160_e55568: f64 = (locals.var_t3 / p.p1682);
        (assign33160_e55568, (locals.var_t3_dn0 / p.p1682), (locals.var_t3_dn2 / p.p1682), (locals.var_t3_dn3 / p.p1682), (locals.var_t3_dn4 / p.p1682), (locals.var_t3_dn5 / p.p1682), (locals.var_t3_dn6 / p.p1682), (locals.var_t3_dn7 / p.p1682), (locals.var_t3_dn8 / p.p1682), (locals.var_t3_dn9 / p.p1682), (locals.var_t3_dn10 / p.p1682), (locals.var_t3_dn11 / p.p1682), (locals.var_t3_dn13 / p.p1682), (locals.var_t3_dn14 / p.p1682),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33160_e55570;
        locals.var_t4_dn0 = assign33160_e55570_d_n0;
        locals.var_t4_dn2 = assign33160_e55570_d_n2;
        locals.var_t4_dn3 = assign33160_e55570_d_n3;
        locals.var_t4_dn4 = assign33160_e55570_d_n4;
        locals.var_t4_dn5 = assign33160_e55570_d_n5;
        locals.var_t4_dn6 = assign33160_e55570_d_n6;
        locals.var_t4_dn7 = assign33160_e55570_d_n7;
        locals.var_t4_dn8 = assign33160_e55570_d_n8;
        locals.var_t4_dn9 = assign33160_e55570_d_n9;
        locals.var_t4_dn10 = assign33160_e55570_d_n10;
        locals.var_t4_dn11 = assign33160_e55570_d_n11;
        locals.var_t4_dn13 = assign33160_e55570_d_n13;
        locals.var_t4_dn14 = assign33160_e55570_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33170_e55598, assign33170_e55598_d_n0, assign33170_e55598_d_n2, assign33170_e55598_d_n3, assign33170_e55598_d_n4, assign33170_e55598_d_n5, assign33170_e55598_d_n6, assign33170_e55598_d_n7, assign33170_e55598_d_n8, assign33170_e55598_d_n9, assign33170_e55598_d_n10, assign33170_e55598_d_n11, assign33170_e55598_d_n13, assign33170_e55598_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33170_e55580: f64 = (locals.var_t4 + 1.0);
        let assign33170_e55583: f64 = (locals.var_t4 - 1.0);
        let assign33170_e55586: f64 = (locals.var_t4 - 1.0);
        let assign33170_e55587: f64 = (assign33170_e55583 * assign33170_e55586);
        let assign33170_e55590: f64 = (0.25 * p.p1688);
        let assign33170_e55592: f64 = (assign33170_e55590 * p.p1688);
        let assign33170_e55593: f64 = (assign33170_e55587 + assign33170_e55592);
        let assign33170_e55594: f64 = (assign33170_e55593).sqrt();
        let assign33170_e55595: f64 = (assign33170_e55580 + assign33170_e55594);
        let assign33170_e55596: f64 = (0.5 * assign33170_e55595);
        (assign33170_e55596, (0.5 * (locals.var_t4_dn0 + (((locals.var_t4_dn0 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn0)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn2 + (((locals.var_t4_dn2 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn2)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn3)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn4)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn5)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn6)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn7)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn8)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn9)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn10)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn11)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn13 + (((locals.var_t4_dn13 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn13)) / (2.0 * assign33170_e55594)))), (0.5 * (locals.var_t4_dn14 + (((locals.var_t4_dn14 * assign33170_e55586) + (assign33170_e55583 * locals.var_t4_dn14)) / (2.0 * assign33170_e55594)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33170_e55598;
        locals.var_t5_dn0 = assign33170_e55598_d_n0;
        locals.var_t5_dn2 = assign33170_e55598_d_n2;
        locals.var_t5_dn3 = assign33170_e55598_d_n3;
        locals.var_t5_dn4 = assign33170_e55598_d_n4;
        locals.var_t5_dn5 = assign33170_e55598_d_n5;
        locals.var_t5_dn6 = assign33170_e55598_d_n6;
        locals.var_t5_dn7 = assign33170_e55598_d_n7;
        locals.var_t5_dn8 = assign33170_e55598_d_n8;
        locals.var_t5_dn9 = assign33170_e55598_d_n9;
        locals.var_t5_dn10 = assign33170_e55598_d_n10;
        locals.var_t5_dn11 = assign33170_e55598_d_n11;
        locals.var_t5_dn13 = assign33170_e55598_d_n13;
        locals.var_t5_dn14 = assign33170_e55598_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33180_e55609, assign33180_e55609_d_n0, assign33180_e55609_d_n2, assign33180_e55609_d_n3, assign33180_e55609_d_n4, assign33180_e55609_d_n5, assign33180_e55609_d_n6, assign33180_e55609_d_n7, assign33180_e55609_d_n8, assign33180_e55609_d_n9, assign33180_e55609_d_n10, assign33180_e55609_d_n11, assign33180_e55609_d_n13, assign33180_e55609_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33180_e55607: f64 = (p.p1682 * locals.var_t5);
        (assign33180_e55607, (p.p1682 * locals.var_t5_dn0), (p.p1682 * locals.var_t5_dn2), (p.p1682 * locals.var_t5_dn3), (p.p1682 * locals.var_t5_dn4), (p.p1682 * locals.var_t5_dn5), (p.p1682 * locals.var_t5_dn6), (p.p1682 * locals.var_t5_dn7), (p.p1682 * locals.var_t5_dn8), (p.p1682 * locals.var_t5_dn9), (p.p1682 * locals.var_t5_dn10), (p.p1682 * locals.var_t5_dn11), (p.p1682 * locals.var_t5_dn13), (p.p1682 * locals.var_t5_dn14),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn0, locals.var_noiaeff_dn2, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8, locals.var_noiaeff_dn9, locals.var_noiaeff_dn10, locals.var_noiaeff_dn11, locals.var_noiaeff_dn13, locals.var_noiaeff_dn14,)
    }
};
        locals.var_noiaeff = assign33180_e55609;
        locals.var_noiaeff_dn0 = assign33180_e55609_d_n0;
        locals.var_noiaeff_dn2 = assign33180_e55609_d_n2;
        locals.var_noiaeff_dn3 = assign33180_e55609_d_n3;
        locals.var_noiaeff_dn4 = assign33180_e55609_d_n4;
        locals.var_noiaeff_dn5 = assign33180_e55609_d_n5;
        locals.var_noiaeff_dn6 = assign33180_e55609_d_n6;
        locals.var_noiaeff_dn7 = assign33180_e55609_d_n7;
        locals.var_noiaeff_dn8 = assign33180_e55609_d_n8;
        locals.var_noiaeff_dn9 = assign33180_e55609_d_n9;
        locals.var_noiaeff_dn10 = assign33180_e55609_d_n10;
        locals.var_noiaeff_dn11 = assign33180_e55609_d_n11;
        locals.var_noiaeff_dn13 = assign33180_e55609_d_n13;
        locals.var_noiaeff_dn14 = assign33180_e55609_d_n14;
        locals.var_noiaeff_rv = 0.0;

        let (assign33190_e55622, assign33190_e55622_d_n0, assign33190_e55622_d_n2, assign33190_e55622_d_n3, assign33190_e55622_d_n4, assign33190_e55622_d_n5, assign33190_e55622_d_n6, assign33190_e55622_d_n7, assign33190_e55622_d_n8, assign33190_e55622_d_n9, assign33190_e55622_d_n10, assign33190_e55622_d_n11, assign33190_e55622_d_n13, assign33190_e55622_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33190_e55618: f64 = (2.0 * locals.var_vtm);
        let assign33190_e55620: f64 = (assign33190_e55618 / locals.var_esatl);
        (assign33190_e55620, (-((assign33190_e55618 * locals.var_esatl_dn0) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn2) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn3) / (locals.var_esatl * locals.var_esatl))), ((((2.0 * locals.var_vtm_dn4) * locals.var_esatl) - (assign33190_e55618 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (-((assign33190_e55618 * locals.var_esatl_dn5) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn6) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn7) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn8) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn9) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn10) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn11) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn13) / (locals.var_esatl * locals.var_esatl))), (-((assign33190_e55618 * locals.var_esatl_dn14) / (locals.var_esatl * locals.var_esatl))),)
    } else {
        (locals.var_lambdac_fn2, locals.var_lambdac_fn2_dn0, locals.var_lambdac_fn2_dn2, locals.var_lambdac_fn2_dn3, locals.var_lambdac_fn2_dn4, locals.var_lambdac_fn2_dn5, locals.var_lambdac_fn2_dn6, locals.var_lambdac_fn2_dn7, locals.var_lambdac_fn2_dn8, locals.var_lambdac_fn2_dn9, locals.var_lambdac_fn2_dn10, locals.var_lambdac_fn2_dn11, locals.var_lambdac_fn2_dn13, locals.var_lambdac_fn2_dn14,)
    }
};
        locals.var_lambdac_fn2 = assign33190_e55622;
        locals.var_lambdac_fn2_dn0 = assign33190_e55622_d_n0;
        locals.var_lambdac_fn2_dn2 = assign33190_e55622_d_n2;
        locals.var_lambdac_fn2_dn3 = assign33190_e55622_d_n3;
        locals.var_lambdac_fn2_dn4 = assign33190_e55622_d_n4;
        locals.var_lambdac_fn2_dn5 = assign33190_e55622_d_n5;
        locals.var_lambdac_fn2_dn6 = assign33190_e55622_d_n6;
        locals.var_lambdac_fn2_dn7 = assign33190_e55622_d_n7;
        locals.var_lambdac_fn2_dn8 = assign33190_e55622_d_n8;
        locals.var_lambdac_fn2_dn9 = assign33190_e55622_d_n9;
        locals.var_lambdac_fn2_dn10 = assign33190_e55622_d_n10;
        locals.var_lambdac_fn2_dn11 = assign33190_e55622_d_n11;
        locals.var_lambdac_fn2_dn13 = assign33190_e55622_d_n13;
        locals.var_lambdac_fn2_dn14 = assign33190_e55622_d_n14;
        locals.var_lambdac_fn2_rv = 0.0;

        let (assign33200_e55635, assign33200_e55635_d_n0, assign33200_e55635_d_n2, assign33200_e55635_d_n3, assign33200_e55635_d_n4, assign33200_e55635_d_n5, assign33200_e55635_d_n6, assign33200_e55635_d_n7, assign33200_e55635_d_n8, assign33200_e55635_d_n9, assign33200_e55635_d_n10, assign33200_e55635_d_n11, assign33200_e55635_d_n13, assign33200_e55635_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33200_e55632: f64 = (locals.var_lambdac_fn2 * locals.var_dqi);
        let assign33200_e55633: f64 = (1.0 + assign33200_e55632);
        (assign33200_e55633, ((locals.var_lambdac_fn2_dn0 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn0)), ((locals.var_lambdac_fn2_dn2 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn2)), ((locals.var_lambdac_fn2_dn3 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn3)), ((locals.var_lambdac_fn2_dn4 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn4)), ((locals.var_lambdac_fn2_dn5 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn5)), ((locals.var_lambdac_fn2_dn6 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn6)), ((locals.var_lambdac_fn2_dn7 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn7)), ((locals.var_lambdac_fn2_dn8 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn8)), ((locals.var_lambdac_fn2_dn9 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn9)), ((locals.var_lambdac_fn2_dn10 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn10)), ((locals.var_lambdac_fn2_dn11 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn11)), ((locals.var_lambdac_fn2_dn13 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn13)), ((locals.var_lambdac_fn2_dn14 * locals.var_dqi) + (locals.var_lambdac_fn2 * locals.var_dqi_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33200_e55635;
        locals.var_t1_dn0 = assign33200_e55635_d_n0;
        locals.var_t1_dn2 = assign33200_e55635_d_n2;
        locals.var_t1_dn3 = assign33200_e55635_d_n3;
        locals.var_t1_dn4 = assign33200_e55635_d_n4;
        locals.var_t1_dn5 = assign33200_e55635_d_n5;
        locals.var_t1_dn6 = assign33200_e55635_d_n6;
        locals.var_t1_dn7 = assign33200_e55635_d_n7;
        locals.var_t1_dn8 = assign33200_e55635_d_n8;
        locals.var_t1_dn9 = assign33200_e55635_d_n9;
        locals.var_t1_dn10 = assign33200_e55635_d_n10;
        locals.var_t1_dn11 = assign33200_e55635_d_n11;
        locals.var_t1_dn13 = assign33200_e55635_d_n13;
        locals.var_t1_dn14 = assign33200_e55635_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign33210_e55648, assign33210_e55648_d_n0, assign33210_e55648_d_n2, assign33210_e55648_d_n3, assign33210_e55648_d_n4, assign33210_e55648_d_n5, assign33210_e55648_d_n6, assign33210_e55648_d_n7, assign33210_e55648_d_n8, assign33210_e55648_d_n9, assign33210_e55648_d_n10, assign33210_e55648_d_n11, assign33210_e55648_d_n13, assign33210_e55648_d_n14,) = {
    if (((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) {
        let assign33210_e55645: f64 = (p.p1685 * locals.var_dqi);
        let assign33210_e55646: f64 = (1.0 + assign33210_e55645);
        (assign33210_e55646, (p.p1685 * locals.var_dqi_dn0), (p.p1685 * locals.var_dqi_dn2), (p.p1685 * locals.var_dqi_dn3), (p.p1685 * locals.var_dqi_dn4), (p.p1685 * locals.var_dqi_dn5), (p.p1685 * locals.var_dqi_dn6), (p.p1685 * locals.var_dqi_dn7), (p.p1685 * locals.var_dqi_dn8), (p.p1685 * locals.var_dqi_dn9), (p.p1685 * locals.var_dqi_dn10), (p.p1685 * locals.var_dqi_dn11), (p.p1685 * locals.var_dqi_dn13), (p.p1685 * locals.var_dqi_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33210_e55648;
        locals.var_t2_dn0 = assign33210_e55648_d_n0;
        locals.var_t2_dn2 = assign33210_e55648_d_n2;
        locals.var_t2_dn3 = assign33210_e55648_d_n3;
        locals.var_t2_dn4 = assign33210_e55648_d_n4;
        locals.var_t2_dn5 = assign33210_e55648_d_n5;
        locals.var_t2_dn6 = assign33210_e55648_d_n6;
        locals.var_t2_dn7 = assign33210_e55648_d_n7;
        locals.var_t2_dn8 = assign33210_e55648_d_n8;
        locals.var_t2_dn9 = assign33210_e55648_d_n9;
        locals.var_t2_dn10 = assign33210_e55648_d_n10;
        locals.var_t2_dn11 = assign33210_e55648_d_n11;
        locals.var_t2_dn13 = assign33210_e55648_d_n13;
        locals.var_t2_dn14 = assign33210_e55648_d_n14;
        locals.var_t2_rv = 0.0;

        let assign33230_e55674: f64 = if ((locals.var_t1 > 0.0) && (locals.var_t2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard631 = assign33230_e55674;
        locals.var_guard631_rv = 0.0;

        let (assign33250_e55736, assign33250_e55736_d_n0, assign33250_e55736_d_n2, assign33250_e55736_d_n3, assign33250_e55736_d_n4, assign33250_e55736_d_n5, assign33250_e55736_d_n6, assign33250_e55736_d_n7, assign33250_e55736_d_n8, assign33250_e55736_d_n9, assign33250_e55736_d_n10, assign33250_e55736_d_n11, assign33250_e55736_d_n13, assign33250_e55736_d_n14,) = {
    if ((((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign33250_e55699: f64 = (locals.var_qis + 0.5);
        let assign33250_e55702: f64 = (locals.var_qid + 0.5);
        let assign33250_e55703: f64 = (assign33250_e55699 / assign33250_e55702);
        let (assign33250_e55728, assign33250_e55728_d_n0, assign33250_e55728_d_n2, assign33250_e55728_d_n3, assign33250_e55728_d_n4, assign33250_e55728_d_n5, assign33250_e55728_d_n6, assign33250_e55728_d_n7, assign33250_e55728_d_n8, assign33250_e55728_d_n9, assign33250_e55728_d_n10, assign33250_e55728_d_n11, assign33250_e55728_d_n13, assign33250_e55728_d_n14,) = {
            if (!(assign33250_e55703 > 1e-38)) {
                let assign33250_e55708: f64 = (-87.498233534);
                (assign33250_e55708, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33250_e55711: f64 = (locals.var_qis + 0.5);
                let assign33250_e55714: f64 = (locals.var_qid + 0.5);
                let assign33250_e55715: f64 = (assign33250_e55711 / assign33250_e55714);
                let (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,) = {
                    if (assign33250_e55715 > 1e-38) {
                        let assign33250_e55720: f64 = (locals.var_qis + 0.5);
                        let assign33250_e55723: f64 = (locals.var_qid + 0.5);
                        let assign33250_e55724: f64 = (assign33250_e55720 / assign33250_e55723);
                        let assign33250_e55725: f64 = (assign33250_e55724).ln();
                        (assign33250_e55725, ((((locals.var_qis_dn0 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn0)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn2 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn2)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn3 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn3)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn4 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn4)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn5 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn5)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn6 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn6)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn7 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn7)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn8 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn8)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn9 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn9)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn10 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn10)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn11 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn11)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn13 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn13)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((locals.var_qis_dn14 * assign33250_e55723) - (assign33250_e55720 * locals.var_qid_dn14)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,)
            }
        };
        let assign33250_e55731: f64 = (locals.var_qis + locals.var_qid);
        let assign33250_e55733: f64 = (assign33250_e55731 + 1.0);
        let assign33250_e55734: f64 = (assign33250_e55728 * assign33250_e55733);
        (assign33250_e55734, ((assign33250_e55728_d_n0 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn0 + locals.var_qid_dn0))), ((assign33250_e55728_d_n2 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn2 + locals.var_qid_dn2))), ((assign33250_e55728_d_n3 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn3 + locals.var_qid_dn3))), ((assign33250_e55728_d_n4 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn4 + locals.var_qid_dn4))), ((assign33250_e55728_d_n5 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn5 + locals.var_qid_dn5))), ((assign33250_e55728_d_n6 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn6 + locals.var_qid_dn6))), ((assign33250_e55728_d_n7 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn7 + locals.var_qid_dn7))), ((assign33250_e55728_d_n8 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn8 + locals.var_qid_dn8))), ((assign33250_e55728_d_n9 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn9 + locals.var_qid_dn9))), ((assign33250_e55728_d_n10 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn10 + locals.var_qid_dn10))), ((assign33250_e55728_d_n11 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn11 + locals.var_qid_dn11))), ((assign33250_e55728_d_n13 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn13 + locals.var_qid_dn13))), ((assign33250_e55728_d_n14 * assign33250_e55733) + (assign33250_e55728 * (locals.var_qis_dn14 + locals.var_qid_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33250_e55736;
        locals.var_t3_dn0 = assign33250_e55736_d_n0;
        locals.var_t3_dn2 = assign33250_e55736_d_n2;
        locals.var_t3_dn3 = assign33250_e55736_d_n3;
        locals.var_t3_dn4 = assign33250_e55736_d_n4;
        locals.var_t3_dn5 = assign33250_e55736_d_n5;
        locals.var_t3_dn6 = assign33250_e55736_d_n6;
        locals.var_t3_dn7 = assign33250_e55736_d_n7;
        locals.var_t3_dn8 = assign33250_e55736_d_n8;
        locals.var_t3_dn9 = assign33250_e55736_d_n9;
        locals.var_t3_dn10 = assign33250_e55736_d_n10;
        locals.var_t3_dn11 = assign33250_e55736_d_n11;
        locals.var_t3_dn13 = assign33250_e55736_d_n13;
        locals.var_t3_dn14 = assign33250_e55736_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33260_e55751, assign33260_e55751_d_n0, assign33260_e55751_d_n2, assign33260_e55751_d_n3, assign33260_e55751_d_n4, assign33260_e55751_d_n5, assign33260_e55751_d_n6, assign33260_e55751_d_n7, assign33260_e55751_d_n8, assign33260_e55751_d_n9, assign33260_e55751_d_n10, assign33260_e55751_d_n11, assign33260_e55751_d_n13, assign33260_e55751_d_n14,) = {
    if ((((locals.var_guard624 != 0.0) && (locals.var_guard626 == 0.0)) && (locals.var_guard630 != 0.0)) && (locals.var_guard631 != 0.0)) {
        let assign33260_e55748: f64 = (locals.var_qis - locals.var_qid);
        let assign33260_e55749: f64 = (2.0 * assign33260_e55748);
        (assign33260_e55749, (2.0 * (locals.var_qis_dn0 - locals.var_qid_dn0)), (2.0 * (locals.var_qis_dn2 - locals.var_qid_dn2)), (2.0 * (locals.var_qis_dn3 - locals.var_qid_dn3)), (2.0 * (locals.var_qis_dn4 - locals.var_qid_dn4)), (2.0 * (locals.var_qis_dn5 - locals.var_qid_dn5)), (2.0 * (locals.var_qis_dn6 - locals.var_qid_dn6)), (2.0 * (locals.var_qis_dn7 - locals.var_qid_dn7)), (2.0 * (locals.var_qis_dn8 - locals.var_qid_dn8)), (2.0 * (locals.var_qis_dn9 - locals.var_qid_dn9)), (2.0 * (locals.var_qis_dn10 - locals.var_qid_dn10)), (2.0 * (locals.var_qis_dn11 - locals.var_qid_dn11)), (2.0 * (locals.var_qis_dn13 - locals.var_qid_dn13)), (2.0 * (locals.var_qis_dn14 - locals.var_qid_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33260_e55751;
        locals.var_t4_dn0 = assign33260_e55751_d_n0;
        locals.var_t4_dn2 = assign33260_e55751_d_n2;
        locals.var_t4_dn3 = assign33260_e55751_d_n3;
        locals.var_t4_dn4 = assign33260_e55751_d_n4;
        locals.var_t4_dn5 = assign33260_e55751_d_n5;
        locals.var_t4_dn6 = assign33260_e55751_d_n6;
        locals.var_t4_dn7 = assign33260_e55751_d_n7;
        locals.var_t4_dn8 = assign33260_e55751_d_n8;
        locals.var_t4_dn9 = assign33260_e55751_d_n9;
        locals.var_t4_dn10 = assign33260_e55751_d_n10;
        locals.var_t4_dn11 = assign33260_e55751_d_n11;
        locals.var_t4_dn13 = assign33260_e55751_d_n13;
        locals.var_t4_dn14 = assign33260_e55751_d_n14;
        locals.var_t4_rv = 0.0;

        let assign33300_e55814: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign33300_e55814;
        locals.var_guard632_rv = 0.0;

        let assign33310_e55817: f64 = if p.p72 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign33310_e55817;
        locals.var_guard633_rv = 0.0;

        let (assign33320_e55823, assign33320_e55823_d_n0, assign33320_e55823_d_n2, assign33320_e55823_d_n3, assign33320_e55823_d_n4, assign33320_e55823_d_n5, assign33320_e55823_d_n6, assign33320_e55823_d_n7, assign33320_e55823_d_n8, assign33320_e55823_d_n9, assign33320_e55823_d_n10, assign33320_e55823_d_n11, assign33320_e55823_d_n13, assign33320_e55823_d_n14,) = {
    if (locals.var_guard632 != 0.0) {
        let assign33320_e55821: f64 = (locals.var_ueff * locals.var_qinv);
        (assign33320_e55821, ((locals.var_ueff_dn0 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn0)), ((locals.var_ueff_dn2 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn2)), ((locals.var_ueff_dn3 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn3)), ((locals.var_ueff_dn4 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn4)), ((locals.var_ueff_dn5 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn5)), ((locals.var_ueff_dn6 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn6)), ((locals.var_ueff_dn7 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn7)), ((locals.var_ueff_dn8 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn8)), ((locals.var_ueff_dn9 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn9)), ((locals.var_ueff_dn10 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn10)), ((locals.var_ueff_dn11 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn11)), ((locals.var_ueff_dn13 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn13)), ((locals.var_ueff_dn14 * locals.var_qinv) + (locals.var_ueff * locals.var_qinv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33320_e55823;
        locals.var_t0_dn0 = assign33320_e55823_d_n0;
        locals.var_t0_dn2 = assign33320_e55823_d_n2;
        locals.var_t0_dn3 = assign33320_e55823_d_n3;
        locals.var_t0_dn4 = assign33320_e55823_d_n4;
        locals.var_t0_dn5 = assign33320_e55823_d_n5;
        locals.var_t0_dn6 = assign33320_e55823_d_n6;
        locals.var_t0_dn7 = assign33320_e55823_d_n7;
        locals.var_t0_dn8 = assign33320_e55823_d_n8;
        locals.var_t0_dn9 = assign33320_e55823_d_n9;
        locals.var_t0_dn10 = assign33320_e55823_d_n10;
        locals.var_t0_dn11 = assign33320_e55823_d_n11;
        locals.var_t0_dn13 = assign33320_e55823_d_n13;
        locals.var_t0_dn14 = assign33320_e55823_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33330_e55833, assign33330_e55833_d_n0, assign33330_e55833_d_n2, assign33330_e55833_d_n3, assign33330_e55833_d_n4, assign33330_e55833_d_n5, assign33330_e55833_d_n6, assign33330_e55833_d_n7, assign33330_e55833_d_n8, assign33330_e55833_d_n9, assign33330_e55833_d_n10, assign33330_e55833_d_n11, assign33330_e55833_d_n13, assign33330_e55833_d_n14,) = {
    if (locals.var_guard632 != 0.0) {
        let assign33330_e55827: f64 = (locals.var_t0 * locals.var_rdsi);
        let assign33330_e55830: f64 = (locals.var_leff_1 * locals.var_leff_1);
        let assign33330_e55831: f64 = (assign33330_e55827 + assign33330_e55830);
        (assign33330_e55831, (((locals.var_t0_dn0 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn0)) + ((locals.var_leff_1_dn0 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn0))), (((locals.var_t0_dn2 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn2)) + ((locals.var_leff_1_dn2 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn2))), (((locals.var_t0_dn3 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn3)) + ((locals.var_leff_1_dn3 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn3))), (((locals.var_t0_dn4 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn4)) + ((locals.var_leff_1_dn4 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn4))), (((locals.var_t0_dn5 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn5)) + ((locals.var_leff_1_dn5 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn5))), (((locals.var_t0_dn6 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn6)) + ((locals.var_leff_1_dn6 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn6))), (((locals.var_t0_dn7 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn7)) + ((locals.var_leff_1_dn7 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn7))), (((locals.var_t0_dn8 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn8)) + ((locals.var_leff_1_dn8 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn8))), (((locals.var_t0_dn9 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn9)) + ((locals.var_leff_1_dn9 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn9))), (((locals.var_t0_dn10 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn10)) + ((locals.var_leff_1_dn10 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn10))), (((locals.var_t0_dn11 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn11)) + ((locals.var_leff_1_dn11 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn11))), (((locals.var_t0_dn13 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn13)) + ((locals.var_leff_1_dn13 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn13))), (((locals.var_t0_dn14 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn14)) + ((locals.var_leff_1_dn14 * locals.var_leff_1) + (locals.var_leff_1 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33330_e55833;
        locals.var_t1_dn0 = assign33330_e55833_d_n0;
        locals.var_t1_dn2 = assign33330_e55833_d_n2;
        locals.var_t1_dn3 = assign33330_e55833_d_n3;
        locals.var_t1_dn4 = assign33330_e55833_d_n4;
        locals.var_t1_dn5 = assign33330_e55833_d_n5;
        locals.var_t1_dn6 = assign33330_e55833_d_n6;
        locals.var_t1_dn7 = assign33330_e55833_d_n7;
        locals.var_t1_dn8 = assign33330_e55833_d_n8;
        locals.var_t1_dn9 = assign33330_e55833_d_n9;
        locals.var_t1_dn10 = assign33330_e55833_d_n10;
        locals.var_t1_dn11 = assign33330_e55833_d_n11;
        locals.var_t1_dn13 = assign33330_e55833_d_n13;
        locals.var_t1_dn14 = assign33330_e55833_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign33360_e55860, assign33360_e55860_d_n0, assign33360_e55860_d_n2, assign33360_e55860_d_n3, assign33360_e55860_d_n4, assign33360_e55860_d_n5, assign33360_e55860_d_n6, assign33360_e55860_d_n7, assign33360_e55860_d_n8, assign33360_e55860_d_n9, assign33360_e55860_d_n10, assign33360_e55860_d_n11, assign33360_e55860_d_n13, assign33360_e55860_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33360_e55858: f64 = (locals.var_qia / locals.var_esatl);
        (assign33360_e55858, (((locals.var_qia_dn0 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn0)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn2 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn2)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn3 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn4 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn5 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn6 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn7 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn8 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn9 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn10 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn11 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn13 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn13)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_qia_dn14 * locals.var_esatl) - (locals.var_qia * locals.var_esatl_dn14)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33360_e55860;
        locals.var_t0_dn0 = assign33360_e55860_d_n0;
        locals.var_t0_dn2 = assign33360_e55860_d_n2;
        locals.var_t0_dn3 = assign33360_e55860_d_n3;
        locals.var_t0_dn4 = assign33360_e55860_d_n4;
        locals.var_t0_dn5 = assign33360_e55860_d_n5;
        locals.var_t0_dn6 = assign33360_e55860_d_n6;
        locals.var_t0_dn7 = assign33360_e55860_d_n7;
        locals.var_t0_dn8 = assign33360_e55860_d_n8;
        locals.var_t0_dn9 = assign33360_e55860_d_n9;
        locals.var_t0_dn10 = assign33360_e55860_d_n10;
        locals.var_t0_dn11 = assign33360_e55860_d_n11;
        locals.var_t0_dn13 = assign33360_e55860_d_n13;
        locals.var_t0_dn14 = assign33360_e55860_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33370_e55869, assign33370_e55869_d_n0, assign33370_e55869_d_n2, assign33370_e55869_d_n3, assign33370_e55869_d_n4, assign33370_e55869_d_n5, assign33370_e55869_d_n6, assign33370_e55869_d_n7, assign33370_e55869_d_n8, assign33370_e55869_d_n9, assign33370_e55869_d_n10, assign33370_e55869_d_n11, assign33370_e55869_d_n13, assign33370_e55869_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33370_e55867: f64 = (locals.var_t0 * locals.var_t0);
        (assign33370_e55867, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33370_e55869;
        locals.var_t0_dn0 = assign33370_e55869_d_n0;
        locals.var_t0_dn2 = assign33370_e55869_d_n2;
        locals.var_t0_dn3 = assign33370_e55869_d_n3;
        locals.var_t0_dn4 = assign33370_e55869_d_n4;
        locals.var_t0_dn5 = assign33370_e55869_d_n5;
        locals.var_t0_dn6 = assign33370_e55869_d_n6;
        locals.var_t0_dn7 = assign33370_e55869_d_n7;
        locals.var_t0_dn8 = assign33370_e55869_d_n8;
        locals.var_t0_dn9 = assign33370_e55869_d_n9;
        locals.var_t0_dn10 = assign33370_e55869_d_n10;
        locals.var_t0_dn11 = assign33370_e55869_d_n11;
        locals.var_t0_dn13 = assign33370_e55869_d_n13;
        locals.var_t0_dn14 = assign33370_e55869_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33380_e55884, assign33380_e55884_d_n0, assign33380_e55884_d_n2, assign33380_e55884_d_n3, assign33380_e55884_d_n4, assign33380_e55884_d_n5, assign33380_e55884_d_n6, assign33380_e55884_d_n7, assign33380_e55884_d_n8, assign33380_e55884_d_n9, assign33380_e55884_d_n10, assign33380_e55884_d_n11, assign33380_e55884_d_n13, assign33380_e55884_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33380_e55878: f64 = (locals.var_t0 * p.p1709);
        let assign33380_e55880: f64 = (assign33380_e55878 * locals.var_leff_1);
        let assign33380_e55881: f64 = (1.0 + assign33380_e55880);
        let assign33380_e55882: f64 = (p.p1708 * assign33380_e55881);
        (assign33380_e55882, (p.p1708 * (((locals.var_t0_dn0 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn0))), (p.p1708 * (((locals.var_t0_dn2 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn2))), (p.p1708 * (((locals.var_t0_dn3 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn3))), (p.p1708 * (((locals.var_t0_dn4 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn4))), (p.p1708 * (((locals.var_t0_dn5 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn5))), (p.p1708 * (((locals.var_t0_dn6 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn6))), (p.p1708 * (((locals.var_t0_dn7 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn7))), (p.p1708 * (((locals.var_t0_dn8 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn8))), (p.p1708 * (((locals.var_t0_dn9 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn9))), (p.p1708 * (((locals.var_t0_dn10 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn10))), (p.p1708 * (((locals.var_t0_dn11 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn11))), (p.p1708 * (((locals.var_t0_dn13 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn13))), (p.p1708 * (((locals.var_t0_dn14 * p.p1709) * locals.var_leff_1) + (assign33380_e55878 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noibeta, locals.var_noibeta_dn0, locals.var_noibeta_dn2, locals.var_noibeta_dn3, locals.var_noibeta_dn4, locals.var_noibeta_dn5, locals.var_noibeta_dn6, locals.var_noibeta_dn7, locals.var_noibeta_dn8, locals.var_noibeta_dn9, locals.var_noibeta_dn10, locals.var_noibeta_dn11, locals.var_noibeta_dn13, locals.var_noibeta_dn14,)
    }
};
        locals.var_noibeta = assign33380_e55884;
        locals.var_noibeta_dn0 = assign33380_e55884_d_n0;
        locals.var_noibeta_dn2 = assign33380_e55884_d_n2;
        locals.var_noibeta_dn3 = assign33380_e55884_d_n3;
        locals.var_noibeta_dn4 = assign33380_e55884_d_n4;
        locals.var_noibeta_dn5 = assign33380_e55884_d_n5;
        locals.var_noibeta_dn6 = assign33380_e55884_d_n6;
        locals.var_noibeta_dn7 = assign33380_e55884_d_n7;
        locals.var_noibeta_dn8 = assign33380_e55884_d_n8;
        locals.var_noibeta_dn9 = assign33380_e55884_d_n9;
        locals.var_noibeta_dn10 = assign33380_e55884_d_n10;
        locals.var_noibeta_dn11 = assign33380_e55884_d_n11;
        locals.var_noibeta_dn13 = assign33380_e55884_d_n13;
        locals.var_noibeta_dn14 = assign33380_e55884_d_n14;
        locals.var_noibeta_rv = 0.0;

        let (assign33390_e55899, assign33390_e55899_d_n0, assign33390_e55899_d_n2, assign33390_e55899_d_n3, assign33390_e55899_d_n4, assign33390_e55899_d_n5, assign33390_e55899_d_n6, assign33390_e55899_d_n7, assign33390_e55899_d_n8, assign33390_e55899_d_n9, assign33390_e55899_d_n10, assign33390_e55899_d_n11, assign33390_e55899_d_n13, assign33390_e55899_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33390_e55893: f64 = (locals.var_t0 * p.p1711);
        let assign33390_e55895: f64 = (assign33390_e55893 * locals.var_leff_1);
        let assign33390_e55896: f64 = (1.0 + assign33390_e55895);
        let assign33390_e55897: f64 = (p.p1710 * assign33390_e55896);
        (assign33390_e55897, (p.p1710 * (((locals.var_t0_dn0 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn0))), (p.p1710 * (((locals.var_t0_dn2 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn2))), (p.p1710 * (((locals.var_t0_dn3 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn3))), (p.p1710 * (((locals.var_t0_dn4 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn4))), (p.p1710 * (((locals.var_t0_dn5 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn5))), (p.p1710 * (((locals.var_t0_dn6 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn6))), (p.p1710 * (((locals.var_t0_dn7 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn7))), (p.p1710 * (((locals.var_t0_dn8 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn8))), (p.p1710 * (((locals.var_t0_dn9 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn9))), (p.p1710 * (((locals.var_t0_dn10 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn10))), (p.p1710 * (((locals.var_t0_dn11 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn11))), (p.p1710 * (((locals.var_t0_dn13 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn13))), (p.p1710 * (((locals.var_t0_dn14 * p.p1711) * locals.var_leff_1) + (assign33390_e55893 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noitheta, locals.var_noitheta_dn0, locals.var_noitheta_dn2, locals.var_noitheta_dn3, locals.var_noitheta_dn4, locals.var_noitheta_dn5, locals.var_noitheta_dn6, locals.var_noitheta_dn7, locals.var_noitheta_dn8, locals.var_noitheta_dn9, locals.var_noitheta_dn10, locals.var_noitheta_dn11, locals.var_noitheta_dn13, locals.var_noitheta_dn14,)
    }
};
        locals.var_noitheta = assign33390_e55899;
        locals.var_noitheta_dn0 = assign33390_e55899_d_n0;
        locals.var_noitheta_dn2 = assign33390_e55899_d_n2;
        locals.var_noitheta_dn3 = assign33390_e55899_d_n3;
        locals.var_noitheta_dn4 = assign33390_e55899_d_n4;
        locals.var_noitheta_dn5 = assign33390_e55899_d_n5;
        locals.var_noitheta_dn6 = assign33390_e55899_d_n6;
        locals.var_noitheta_dn7 = assign33390_e55899_d_n7;
        locals.var_noitheta_dn8 = assign33390_e55899_d_n8;
        locals.var_noitheta_dn9 = assign33390_e55899_d_n9;
        locals.var_noitheta_dn10 = assign33390_e55899_d_n10;
        locals.var_noitheta_dn11 = assign33390_e55899_d_n11;
        locals.var_noitheta_dn13 = assign33390_e55899_d_n13;
        locals.var_noitheta_dn14 = assign33390_e55899_d_n14;
        locals.var_noitheta_rv = 0.0;

        let (assign33400_e55914, assign33400_e55914_d_n0, assign33400_e55914_d_n2, assign33400_e55914_d_n3, assign33400_e55914_d_n4, assign33400_e55914_d_n5, assign33400_e55914_d_n6, assign33400_e55914_d_n7, assign33400_e55914_d_n8, assign33400_e55914_d_n9, assign33400_e55914_d_n10, assign33400_e55914_d_n11, assign33400_e55914_d_n13, assign33400_e55914_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33400_e55908: f64 = (locals.var_t0 * p.p1713);
        let assign33400_e55910: f64 = (assign33400_e55908 * locals.var_leff_1);
        let assign33400_e55911: f64 = (1.0 + assign33400_e55910);
        let assign33400_e55912: f64 = (p.p1712 * assign33400_e55911);
        (assign33400_e55912, (p.p1712 * (((locals.var_t0_dn0 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn0))), (p.p1712 * (((locals.var_t0_dn2 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn2))), (p.p1712 * (((locals.var_t0_dn3 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn3))), (p.p1712 * (((locals.var_t0_dn4 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn4))), (p.p1712 * (((locals.var_t0_dn5 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn5))), (p.p1712 * (((locals.var_t0_dn6 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn6))), (p.p1712 * (((locals.var_t0_dn7 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn7))), (p.p1712 * (((locals.var_t0_dn8 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn8))), (p.p1712 * (((locals.var_t0_dn9 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn9))), (p.p1712 * (((locals.var_t0_dn10 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn10))), (p.p1712 * (((locals.var_t0_dn11 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn11))), (p.p1712 * (((locals.var_t0_dn13 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn13))), (p.p1712 * (((locals.var_t0_dn14 * p.p1713) * locals.var_leff_1) + (assign33400_e55908 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noicorr, locals.var_noicorr_dn0, locals.var_noicorr_dn2, locals.var_noicorr_dn3, locals.var_noicorr_dn4, locals.var_noicorr_dn5, locals.var_noicorr_dn6, locals.var_noicorr_dn7, locals.var_noicorr_dn8, locals.var_noicorr_dn9, locals.var_noicorr_dn10, locals.var_noicorr_dn11, locals.var_noicorr_dn13, locals.var_noicorr_dn14,)
    }
};
        locals.var_noicorr = assign33400_e55914;
        locals.var_noicorr_dn0 = assign33400_e55914_d_n0;
        locals.var_noicorr_dn2 = assign33400_e55914_d_n2;
        locals.var_noicorr_dn3 = assign33400_e55914_d_n3;
        locals.var_noicorr_dn4 = assign33400_e55914_d_n4;
        locals.var_noicorr_dn5 = assign33400_e55914_d_n5;
        locals.var_noicorr_dn6 = assign33400_e55914_d_n6;
        locals.var_noicorr_dn7 = assign33400_e55914_d_n7;
        locals.var_noicorr_dn8 = assign33400_e55914_d_n8;
        locals.var_noicorr_dn9 = assign33400_e55914_d_n9;
        locals.var_noicorr_dn10 = assign33400_e55914_d_n10;
        locals.var_noicorr_dn11 = assign33400_e55914_d_n11;
        locals.var_noicorr_dn13 = assign33400_e55914_d_n13;
        locals.var_noicorr_dn14 = assign33400_e55914_d_n14;
        locals.var_noicorr_rv = 0.0;

        let (assign33410_e55929, assign33410_e55929_d_n0, assign33410_e55929_d_n2, assign33410_e55929_d_n3, assign33410_e55929_d_n4, assign33410_e55929_d_n5, assign33410_e55929_d_n6, assign33410_e55929_d_n7, assign33410_e55929_d_n8, assign33410_e55929_d_n9, assign33410_e55929_d_n10, assign33410_e55929_d_n11, assign33410_e55929_d_n13, assign33410_e55929_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33410_e55923: f64 = (locals.var_t0 * p.p1715);
        let assign33410_e55925: f64 = (assign33410_e55923 * locals.var_leff_1);
        let assign33410_e55926: f64 = (1.0 + assign33410_e55925);
        let assign33410_e55927: f64 = (p.p1714 * assign33410_e55926);
        (assign33410_e55927, (p.p1714 * (((locals.var_t0_dn0 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn0))), (p.p1714 * (((locals.var_t0_dn2 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn2))), (p.p1714 * (((locals.var_t0_dn3 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn3))), (p.p1714 * (((locals.var_t0_dn4 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn4))), (p.p1714 * (((locals.var_t0_dn5 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn5))), (p.p1714 * (((locals.var_t0_dn6 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn6))), (p.p1714 * (((locals.var_t0_dn7 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn7))), (p.p1714 * (((locals.var_t0_dn8 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn8))), (p.p1714 * (((locals.var_t0_dn9 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn9))), (p.p1714 * (((locals.var_t0_dn10 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn10))), (p.p1714 * (((locals.var_t0_dn11 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn11))), (p.p1714 * (((locals.var_t0_dn13 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn13))), (p.p1714 * (((locals.var_t0_dn14 * p.p1715) * locals.var_leff_1) + (assign33410_e55923 * locals.var_leff_1_dn14))),)
    } else {
        (locals.var_noilowid, locals.var_noilowid_dn0, locals.var_noilowid_dn2, locals.var_noilowid_dn3, locals.var_noilowid_dn4, locals.var_noilowid_dn5, locals.var_noilowid_dn6, locals.var_noilowid_dn7, locals.var_noilowid_dn8, locals.var_noilowid_dn9, locals.var_noilowid_dn10, locals.var_noilowid_dn11, locals.var_noilowid_dn13, locals.var_noilowid_dn14,)
    }
};
        locals.var_noilowid = assign33410_e55929;
        locals.var_noilowid_dn0 = assign33410_e55929_d_n0;
        locals.var_noilowid_dn2 = assign33410_e55929_d_n2;
        locals.var_noilowid_dn3 = assign33410_e55929_d_n3;
        locals.var_noilowid_dn4 = assign33410_e55929_d_n4;
        locals.var_noilowid_dn5 = assign33410_e55929_d_n5;
        locals.var_noilowid_dn6 = assign33410_e55929_d_n6;
        locals.var_noilowid_dn7 = assign33410_e55929_d_n7;
        locals.var_noilowid_dn8 = assign33410_e55929_d_n8;
        locals.var_noilowid_dn9 = assign33410_e55929_d_n9;
        locals.var_noilowid_dn10 = assign33410_e55929_d_n10;
        locals.var_noilowid_dn11 = assign33410_e55929_d_n11;
        locals.var_noilowid_dn13 = assign33410_e55929_d_n13;
        locals.var_noilowid_dn14 = assign33410_e55929_d_n14;
        locals.var_noilowid_rv = 0.0;

        let (assign33420_e55940, assign33420_e55940_d_n0, assign33420_e55940_d_n2, assign33420_e55940_d_n3, assign33420_e55940_d_n4, assign33420_e55940_d_n5, assign33420_e55940_d_n6, assign33420_e55940_d_n7, assign33420_e55940_d_n8, assign33420_e55940_d_n9, assign33420_e55940_d_n10, assign33420_e55940_d_n11, assign33420_e55940_d_n13, assign33420_e55940_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33420_e55936: f64 = (3.0 * locals.var_noibeta);
        let assign33420_e55938: f64 = (assign33420_e55936 * locals.var_noibeta);
        (assign33420_e55938, (((3.0 * locals.var_noibeta_dn0) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn0)), (((3.0 * locals.var_noibeta_dn2) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn2)), (((3.0 * locals.var_noibeta_dn3) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn3)), (((3.0 * locals.var_noibeta_dn4) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn4)), (((3.0 * locals.var_noibeta_dn5) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn5)), (((3.0 * locals.var_noibeta_dn6) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn6)), (((3.0 * locals.var_noibeta_dn7) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn7)), (((3.0 * locals.var_noibeta_dn8) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn8)), (((3.0 * locals.var_noibeta_dn9) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn9)), (((3.0 * locals.var_noibeta_dn10) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn10)), (((3.0 * locals.var_noibeta_dn11) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn11)), (((3.0 * locals.var_noibeta_dn13) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn13)), (((3.0 * locals.var_noibeta_dn14) * locals.var_noibeta) + (assign33420_e55936 * locals.var_noibeta_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign33420_e55940;
        locals.var_t1_dn0 = assign33420_e55940_d_n0;
        locals.var_t1_dn2 = assign33420_e55940_d_n2;
        locals.var_t1_dn3 = assign33420_e55940_d_n3;
        locals.var_t1_dn4 = assign33420_e55940_d_n4;
        locals.var_t1_dn5 = assign33420_e55940_d_n5;
        locals.var_t1_dn6 = assign33420_e55940_d_n6;
        locals.var_t1_dn7 = assign33420_e55940_d_n7;
        locals.var_t1_dn8 = assign33420_e55940_d_n8;
        locals.var_t1_dn9 = assign33420_e55940_d_n9;
        locals.var_t1_dn10 = assign33420_e55940_d_n10;
        locals.var_t1_dn11 = assign33420_e55940_d_n11;
        locals.var_t1_dn13 = assign33420_e55940_d_n13;
        locals.var_t1_dn14 = assign33420_e55940_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign33430_e55951, assign33430_e55951_d_n0, assign33430_e55951_d_n2, assign33430_e55951_d_n3, assign33430_e55951_d_n4, assign33430_e55951_d_n5, assign33430_e55951_d_n6, assign33430_e55951_d_n7, assign33430_e55951_d_n8, assign33430_e55951_d_n9, assign33430_e55951_d_n10, assign33430_e55951_d_n11, assign33430_e55951_d_n13, assign33430_e55951_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33430_e55947: f64 = (7.5 * locals.var_noitheta);
        let assign33430_e55949: f64 = (assign33430_e55947 * locals.var_noitheta);
        (assign33430_e55949, (((7.5 * locals.var_noitheta_dn0) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn0)), (((7.5 * locals.var_noitheta_dn2) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn2)), (((7.5 * locals.var_noitheta_dn3) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn3)), (((7.5 * locals.var_noitheta_dn4) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn4)), (((7.5 * locals.var_noitheta_dn5) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn5)), (((7.5 * locals.var_noitheta_dn6) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn6)), (((7.5 * locals.var_noitheta_dn7) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn7)), (((7.5 * locals.var_noitheta_dn8) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn8)), (((7.5 * locals.var_noitheta_dn9) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn9)), (((7.5 * locals.var_noitheta_dn10) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn10)), (((7.5 * locals.var_noitheta_dn11) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn11)), (((7.5 * locals.var_noitheta_dn13) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn13)), (((7.5 * locals.var_noitheta_dn14) * locals.var_noitheta) + (assign33430_e55947 * locals.var_noitheta_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33430_e55951;
        locals.var_t2_dn0 = assign33430_e55951_d_n0;
        locals.var_t2_dn2 = assign33430_e55951_d_n2;
        locals.var_t2_dn3 = assign33430_e55951_d_n3;
        locals.var_t2_dn4 = assign33430_e55951_d_n4;
        locals.var_t2_dn5 = assign33430_e55951_d_n5;
        locals.var_t2_dn6 = assign33430_e55951_d_n6;
        locals.var_t2_dn7 = assign33430_e55951_d_n7;
        locals.var_t2_dn8 = assign33430_e55951_d_n8;
        locals.var_t2_dn9 = assign33430_e55951_d_n9;
        locals.var_t2_dn10 = assign33430_e55951_d_n10;
        locals.var_t2_dn11 = assign33430_e55951_d_n11;
        locals.var_t2_dn13 = assign33430_e55951_d_n13;
        locals.var_t2_dn14 = assign33430_e55951_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_130(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33440_e55960, assign33440_e55960_d_n0, assign33440_e55960_d_n2, assign33440_e55960_d_n3, assign33440_e55960_d_n4, assign33440_e55960_d_n5, assign33440_e55960_d_n6, assign33440_e55960_d_n7, assign33440_e55960_d_n8, assign33440_e55960_d_n9, assign33440_e55960_d_n10, assign33440_e55960_d_n11, assign33440_e55960_d_n13, assign33440_e55960_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33440_e55958: f64 = (2.5298 * locals.var_noicorr);
        (assign33440_e55958, (2.5298 * locals.var_noicorr_dn0), (2.5298 * locals.var_noicorr_dn2), (2.5298 * locals.var_noicorr_dn3), (2.5298 * locals.var_noicorr_dn4), (2.5298 * locals.var_noicorr_dn5), (2.5298 * locals.var_noicorr_dn6), (2.5298 * locals.var_noicorr_dn7), (2.5298 * locals.var_noicorr_dn8), (2.5298 * locals.var_noicorr_dn9), (2.5298 * locals.var_noicorr_dn10), (2.5298 * locals.var_noicorr_dn11), (2.5298 * locals.var_noicorr_dn13), (2.5298 * locals.var_noicorr_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33440_e55960;
        locals.var_t3_dn0 = assign33440_e55960_d_n0;
        locals.var_t3_dn2 = assign33440_e55960_d_n2;
        locals.var_t3_dn3 = assign33440_e55960_d_n3;
        locals.var_t3_dn4 = assign33440_e55960_d_n4;
        locals.var_t3_dn5 = assign33440_e55960_d_n5;
        locals.var_t3_dn6 = assign33440_e55960_d_n6;
        locals.var_t3_dn7 = assign33440_e55960_d_n7;
        locals.var_t3_dn8 = assign33440_e55960_d_n8;
        locals.var_t3_dn9 = assign33440_e55960_d_n9;
        locals.var_t3_dn10 = assign33440_e55960_d_n10;
        locals.var_t3_dn11 = assign33440_e55960_d_n11;
        locals.var_t3_dn13 = assign33440_e55960_d_n13;
        locals.var_t3_dn14 = assign33440_e55960_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33450_e55975, assign33450_e55975_d_n0, assign33450_e55975_d_n2, assign33450_e55975_d_n3, assign33450_e55975_d_n4, assign33450_e55975_d_n5, assign33450_e55975_d_n6, assign33450_e55975_d_n7, assign33450_e55975_d_n8, assign33450_e55975_d_n9, assign33450_e55975_d_n10, assign33450_e55975_d_n11, assign33450_e55975_d_n13, assign33450_e55975_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33450_e55967: f64 = (locals.var_qid / locals.var_qis);
        let assign33450_e55971: f64 = (locals.var_vdseff_1 / locals.var_vdsat);
        let assign33450_e55972: f64 = (1.0 - assign33450_e55971);
        let assign33450_e55973: f64 = (assign33450_e55967 * assign33450_e55972);
        (assign33450_e55973, (((((locals.var_qid_dn0 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn0)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn0 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn0)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn2 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn2)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn2 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn2)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn3 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn3)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn3 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn3)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn4 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn4 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn4)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn5 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn5)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn5 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn6 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn6 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn7 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn7 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn7)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn8 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn8 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn8)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn9 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn9 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn9)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn10 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn10)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn10 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn10)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn11 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn11)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn11 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn11)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn13 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn13)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn13 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn13)) / (locals.var_vdsat * locals.var_vdsat))))), (((((locals.var_qid_dn14 * locals.var_qis) - (locals.var_qid * locals.var_qis_dn14)) / (locals.var_qis * locals.var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((locals.var_vdseff_1_dn14 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn14)) / (locals.var_vdsat * locals.var_vdsat))))),)
    } else {
        (locals.var_noieta, locals.var_noieta_dn0, locals.var_noieta_dn2, locals.var_noieta_dn3, locals.var_noieta_dn4, locals.var_noieta_dn5, locals.var_noieta_dn6, locals.var_noieta_dn7, locals.var_noieta_dn8, locals.var_noieta_dn9, locals.var_noieta_dn10, locals.var_noieta_dn11, locals.var_noieta_dn13, locals.var_noieta_dn14,)
    }
};
        locals.var_noieta = assign33450_e55975;
        locals.var_noieta_dn0 = assign33450_e55975_d_n0;
        locals.var_noieta_dn2 = assign33450_e55975_d_n2;
        locals.var_noieta_dn3 = assign33450_e55975_d_n3;
        locals.var_noieta_dn4 = assign33450_e55975_d_n4;
        locals.var_noieta_dn5 = assign33450_e55975_d_n5;
        locals.var_noieta_dn6 = assign33450_e55975_d_n6;
        locals.var_noieta_dn7 = assign33450_e55975_d_n7;
        locals.var_noieta_dn8 = assign33450_e55975_d_n8;
        locals.var_noieta_dn9 = assign33450_e55975_d_n9;
        locals.var_noieta_dn10 = assign33450_e55975_d_n10;
        locals.var_noieta_dn11 = assign33450_e55975_d_n11;
        locals.var_noieta_dn13 = assign33450_e55975_d_n13;
        locals.var_noieta_dn14 = assign33450_e55975_d_n14;
        locals.var_noieta_rv = 0.0;

        let (assign33460_e55986, assign33460_e55986_d_n0, assign33460_e55986_d_n2, assign33460_e55986_d_n3, assign33460_e55986_d_n4, assign33460_e55986_d_n5, assign33460_e55986_d_n6, assign33460_e55986_d_n7, assign33460_e55986_d_n8, assign33460_e55986_d_n9, assign33460_e55986_d_n10, assign33460_e55986_d_n11, assign33460_e55986_d_n13, assign33460_e55986_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33460_e55982: f64 = (locals.var_dvsat * locals.var_dvsat);
        let assign33460_e55984: f64 = (assign33460_e55982 * locals.var_dvsat);
        (assign33460_e55984, ((((locals.var_dvsat_dn0 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn0)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn0)), ((((locals.var_dvsat_dn2 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn2)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn2)), ((((locals.var_dvsat_dn3 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn3)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn3)), ((((locals.var_dvsat_dn4 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn4)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn4)), ((((locals.var_dvsat_dn5 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn5)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn5)), ((((locals.var_dvsat_dn6 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn6)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn6)), ((((locals.var_dvsat_dn7 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn7)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn7)), ((((locals.var_dvsat_dn8 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn8)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn8)), ((((locals.var_dvsat_dn9 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn9)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn9)), ((((locals.var_dvsat_dn10 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn10)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn10)), ((((locals.var_dvsat_dn11 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn11)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn11)), ((((locals.var_dvsat_dn13 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn13)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn13)), ((((locals.var_dvsat_dn14 * locals.var_dvsat) + (locals.var_dvsat * locals.var_dvsat_dn14)) * locals.var_dvsat) + (assign33460_e55982 * locals.var_dvsat_dn14)),)
    } else {
        (locals.var_dvsat3, locals.var_dvsat3_dn0, locals.var_dvsat3_dn2, locals.var_dvsat3_dn3, locals.var_dvsat3_dn4, locals.var_dvsat3_dn5, locals.var_dvsat3_dn6, locals.var_dvsat3_dn7, locals.var_dvsat3_dn8, locals.var_dvsat3_dn9, locals.var_dvsat3_dn10, locals.var_dvsat3_dn11, locals.var_dvsat3_dn13, locals.var_dvsat3_dn14,)
    }
};
        locals.var_dvsat3 = assign33460_e55986;
        locals.var_dvsat3_dn0 = assign33460_e55986_d_n0;
        locals.var_dvsat3_dn2 = assign33460_e55986_d_n2;
        locals.var_dvsat3_dn3 = assign33460_e55986_d_n3;
        locals.var_dvsat3_dn4 = assign33460_e55986_d_n4;
        locals.var_dvsat3_dn5 = assign33460_e55986_d_n5;
        locals.var_dvsat3_dn6 = assign33460_e55986_d_n6;
        locals.var_dvsat3_dn7 = assign33460_e55986_d_n7;
        locals.var_dvsat3_dn8 = assign33460_e55986_d_n8;
        locals.var_dvsat3_dn9 = assign33460_e55986_d_n9;
        locals.var_dvsat3_dn10 = assign33460_e55986_d_n10;
        locals.var_dvsat3_dn11 = assign33460_e55986_d_n11;
        locals.var_dvsat3_dn13 = assign33460_e55986_d_n13;
        locals.var_dvsat3_dn14 = assign33460_e55986_d_n14;
        locals.var_dvsat3_rv = 0.0;

        let (assign33470_e55997, assign33470_e55997_d_n0, assign33470_e55997_d_n2, assign33470_e55997_d_n3, assign33470_e55997_d_n4, assign33470_e55997_d_n5, assign33470_e55997_d_n6, assign33470_e55997_d_n7, assign33470_e55997_d_n8, assign33470_e55997_d_n9, assign33470_e55997_d_n10, assign33470_e55997_d_n11, assign33470_e55997_d_n13, assign33470_e55997_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33470_e55994: f64 = (locals.var_q0 + locals.var_qia);
        let assign33470_e55995: f64 = (locals.var_q0 / assign33470_e55994);
        (assign33470_e55995, (((locals.var_q0_dn0 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn0 + locals.var_qia_dn0))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn2 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn2 + locals.var_qia_dn2))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn3 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn3 + locals.var_qia_dn3))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn4 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn4 + locals.var_qia_dn4))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn5 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn5 + locals.var_qia_dn5))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn6 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn6 + locals.var_qia_dn6))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn7 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn7 + locals.var_qia_dn7))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn8 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn8 + locals.var_qia_dn8))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn9 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn9 + locals.var_qia_dn9))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn10 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn10 + locals.var_qia_dn10))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn11 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn11 + locals.var_qia_dn11))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn13 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn13 + locals.var_qia_dn13))) / (assign33470_e55994 * assign33470_e55994)), (((locals.var_q0_dn14 * assign33470_e55994) - (locals.var_q0 * (locals.var_q0_dn14 + locals.var_qia_dn14))) / (assign33470_e55994 * assign33470_e55994)),)
    } else {
        (locals.var_noiwi, locals.var_noiwi_dn0, locals.var_noiwi_dn2, locals.var_noiwi_dn3, locals.var_noiwi_dn4, locals.var_noiwi_dn5, locals.var_noiwi_dn6, locals.var_noiwi_dn7, locals.var_noiwi_dn8, locals.var_noiwi_dn9, locals.var_noiwi_dn10, locals.var_noiwi_dn11, locals.var_noiwi_dn13, locals.var_noiwi_dn14,)
    }
};
        locals.var_noiwi = assign33470_e55997;
        locals.var_noiwi_dn0 = assign33470_e55997_d_n0;
        locals.var_noiwi_dn2 = assign33470_e55997_d_n2;
        locals.var_noiwi_dn3 = assign33470_e55997_d_n3;
        locals.var_noiwi_dn4 = assign33470_e55997_d_n4;
        locals.var_noiwi_dn5 = assign33470_e55997_d_n5;
        locals.var_noiwi_dn6 = assign33470_e55997_d_n6;
        locals.var_noiwi_dn7 = assign33470_e55997_d_n7;
        locals.var_noiwi_dn8 = assign33470_e55997_d_n8;
        locals.var_noiwi_dn9 = assign33470_e55997_d_n9;
        locals.var_noiwi_dn10 = assign33470_e55997_d_n10;
        locals.var_noiwi_dn11 = assign33470_e55997_d_n11;
        locals.var_noiwi_dn13 = assign33470_e55997_d_n13;
        locals.var_noiwi_dn14 = assign33470_e55997_d_n14;
        locals.var_noiwi_rv = 0.0;

        let (assign33480_e56014, assign33480_e56014_d_n0, assign33480_e56014_d_n2, assign33480_e56014_d_n3, assign33480_e56014_d_n4, assign33480_e56014_d_n5, assign33480_e56014_d_n6, assign33480_e56014_d_n7, assign33480_e56014_d_n8, assign33480_e56014_d_n9, assign33480_e56014_d_n10, assign33480_e56014_d_n11, assign33480_e56014_d_n13, assign33480_e56014_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33480_e56005: f64 = (0.0_f64).max(locals.var_k0si_t);
        let assign33480_e56007: f64 = (assign33480_e56005 * locals.var_qis);
        let assign33480_e56010: f64 = (2.0 * locals.var_nvtm);
        let assign33480_e56011: f64 = (assign33480_e56007 + assign33480_e56010);
        let assign33480_e56012: f64 = (locals.var_k0_t / assign33480_e56011);
        (assign33480_e56012, (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn0) + (2.0 * locals.var_nvtm_dn0))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn2) + (2.0 * locals.var_nvtm_dn2))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn3) + (2.0 * locals.var_nvtm_dn3))) / (assign33480_e56011 * assign33480_e56011))), (((locals.var_k0_t_dn4 * assign33480_e56011) - (locals.var_k0_t * (((if 0.0 >= locals.var_k0si_t { 0.0 } else { locals.var_k0si_t_dn4 } * locals.var_qis) + (assign33480_e56005 * locals.var_qis_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign33480_e56011 * assign33480_e56011)), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn5) + (2.0 * locals.var_nvtm_dn5))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn6) + (2.0 * locals.var_nvtm_dn6))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn7) + (2.0 * locals.var_nvtm_dn7))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn8) + (2.0 * locals.var_nvtm_dn8))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn9) + (2.0 * locals.var_nvtm_dn9))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn10) + (2.0 * locals.var_nvtm_dn10))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn11) + (2.0 * locals.var_nvtm_dn11))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn13) + (2.0 * locals.var_nvtm_dn13))) / (assign33480_e56011 * assign33480_e56011))), (-((locals.var_k0_t * ((assign33480_e56005 * locals.var_qis_dn14) + (2.0 * locals.var_nvtm_dn14))) / (assign33480_e56011 * assign33480_e56011))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33480_e56014;
        locals.var_t4_dn0 = assign33480_e56014_d_n0;
        locals.var_t4_dn2 = assign33480_e56014_d_n2;
        locals.var_t4_dn3 = assign33480_e56014_d_n3;
        locals.var_t4_dn4 = assign33480_e56014_d_n4;
        locals.var_t4_dn5 = assign33480_e56014_d_n5;
        locals.var_t4_dn6 = assign33480_e56014_d_n6;
        locals.var_t4_dn7 = assign33480_e56014_d_n7;
        locals.var_t4_dn8 = assign33480_e56014_d_n8;
        locals.var_t4_dn9 = assign33480_e56014_d_n9;
        locals.var_t4_dn10 = assign33480_e56014_d_n10;
        locals.var_t4_dn11 = assign33480_e56014_d_n11;
        locals.var_t4_dn13 = assign33480_e56014_d_n13;
        locals.var_t4_dn14 = assign33480_e56014_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33490_e56023, assign33490_e56023_d_n0, assign33490_e56023_d_n2, assign33490_e56023_d_n3, assign33490_e56023_d_n4, assign33490_e56023_d_n5, assign33490_e56023_d_n6, assign33490_e56023_d_n7, assign33490_e56023_d_n8, assign33490_e56023_d_n9, assign33490_e56023_d_n10, assign33490_e56023_d_n11, assign33490_e56023_d_n13, assign33490_e56023_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33490_e56020: f64 = (-locals.var_t4);
        let assign33490_e56021: f64 = { let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33490_e56021, ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn0)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn2)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn3)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn4)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn5)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn6)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn7)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn8)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn9)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn10)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn11)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn13)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t4_dn14)),)
    } else {
        (locals.var_mnud0, locals.var_mnud0_dn0, locals.var_mnud0_dn2, locals.var_mnud0_dn3, locals.var_mnud0_dn4, locals.var_mnud0_dn5, locals.var_mnud0_dn6, locals.var_mnud0_dn7, locals.var_mnud0_dn8, locals.var_mnud0_dn9, locals.var_mnud0_dn10, locals.var_mnud0_dn11, locals.var_mnud0_dn13, locals.var_mnud0_dn14,)
    }
};
        locals.var_mnud0 = assign33490_e56023;
        locals.var_mnud0_dn0 = assign33490_e56023_d_n0;
        locals.var_mnud0_dn2 = assign33490_e56023_d_n2;
        locals.var_mnud0_dn3 = assign33490_e56023_d_n3;
        locals.var_mnud0_dn4 = assign33490_e56023_d_n4;
        locals.var_mnud0_dn5 = assign33490_e56023_d_n5;
        locals.var_mnud0_dn6 = assign33490_e56023_d_n6;
        locals.var_mnud0_dn7 = assign33490_e56023_d_n7;
        locals.var_mnud0_dn8 = assign33490_e56023_d_n8;
        locals.var_mnud0_dn9 = assign33490_e56023_d_n9;
        locals.var_mnud0_dn10 = assign33490_e56023_d_n10;
        locals.var_mnud0_dn11 = assign33490_e56023_d_n11;
        locals.var_mnud0_dn13 = assign33490_e56023_d_n13;
        locals.var_mnud0_dn14 = assign33490_e56023_d_n14;
        locals.var_mnud0_rv = 0.0;

        let assign33500_e56026: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign33500_e56026;
        locals.var_guard634_rv = 0.0;

        let (assign33510_e56070, assign33510_e56070_d_n0, assign33510_e56070_d_n2, assign33510_e56070_d_n3, assign33510_e56070_d_n4, assign33510_e56070_d_n5, assign33510_e56070_d_n6, assign33510_e56070_d_n7, assign33510_e56070_d_n8, assign33510_e56070_d_n9, assign33510_e56070_d_n10, assign33510_e56070_d_n11, assign33510_e56070_d_n13, assign33510_e56070_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33510_e56035: f64 = (-10000.0);
        let assign33510_e56037: f64 = (assign33510_e56035 * 1e-6);
        let (assign33510_e56068, assign33510_e56068_d_n4,) = {
            if (!(locals.var_k2_t < assign33510_e56037)) {
                let assign33510_e56044: f64 = (locals.var_k2_t * locals.var_k2_t);
                let assign33510_e56047: f64 = (4.0 * 1e-6);
                let assign33510_e56049: f64 = (assign33510_e56047 * 1e-6);
                let assign33510_e56050: f64 = (assign33510_e56044 + assign33510_e56049);
                let assign33510_e56051: f64 = (assign33510_e56050).sqrt();
                let assign33510_e56052: f64 = (locals.var_k2_t + assign33510_e56051);
                let assign33510_e56053: f64 = (0.5 * assign33510_e56052);
                (assign33510_e56053, (0.5 * (locals.var_k2_t_dn4 + (((locals.var_k2_t_dn4 * locals.var_k2_t) + (locals.var_k2_t * locals.var_k2_t_dn4)) / (2.0 * assign33510_e56051)))),)
            } else {
                let assign33510_e56056: f64 = (-10000.0);
                let assign33510_e56058: f64 = (assign33510_e56056 * 1e-6);
                let (assign33510_e56067, assign33510_e56067_d_n4,) = {
                    if (locals.var_k2_t < assign33510_e56058) {
                        let assign33510_e56061: f64 = (-1e-6);
                        let assign33510_e56063: f64 = (assign33510_e56061 * 1e-6);
                        let assign33510_e56065: f64 = (assign33510_e56063 / locals.var_k2_t);
                        (assign33510_e56065, (-((assign33510_e56063 * locals.var_k2_t_dn4) / (locals.var_k2_t * locals.var_k2_t))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign33510_e56067, assign33510_e56067_d_n4,)
            }
        };
        (assign33510_e56068, 0.0, 0.0, 0.0, assign33510_e56068_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33510_e56070;
        locals.var_t4_dn0 = assign33510_e56070_d_n0;
        locals.var_t4_dn2 = assign33510_e56070_d_n2;
        locals.var_t4_dn3 = assign33510_e56070_d_n3;
        locals.var_t4_dn4 = assign33510_e56070_d_n4;
        locals.var_t4_dn5 = assign33510_e56070_d_n5;
        locals.var_t4_dn6 = assign33510_e56070_d_n6;
        locals.var_t4_dn7 = assign33510_e56070_d_n7;
        locals.var_t4_dn8 = assign33510_e56070_d_n8;
        locals.var_t4_dn9 = assign33510_e56070_d_n9;
        locals.var_t4_dn10 = assign33510_e56070_d_n10;
        locals.var_t4_dn11 = assign33510_e56070_d_n11;
        locals.var_t4_dn13 = assign33510_e56070_d_n13;
        locals.var_t4_dn14 = assign33510_e56070_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33520_e56089, assign33520_e56089_d_n0, assign33520_e56089_d_n2, assign33520_e56089_d_n3, assign33520_e56089_d_n4, assign33520_e56089_d_n5, assign33520_e56089_d_n6, assign33520_e56089_d_n7, assign33520_e56089_d_n8, assign33520_e56089_d_n9, assign33520_e56089_d_n10, assign33520_e56089_d_n11, assign33520_e56089_d_n13, assign33520_e56089_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33520_e56080: f64 = (0.0_f64).max(locals.var_k2si_t);
        let assign33520_e56082: f64 = (assign33520_e56080 * locals.var_qis);
        let assign33520_e56085: f64 = (2.0 * locals.var_nvtm);
        let assign33520_e56086: f64 = (assign33520_e56082 + assign33520_e56085);
        let assign33520_e56087: f64 = (locals.var_t4 / assign33520_e56086);
        (assign33520_e56087, (((locals.var_t4_dn0 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn0) + (2.0 * locals.var_nvtm_dn0)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn2 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn2) + (2.0 * locals.var_nvtm_dn2)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn3 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn3) + (2.0 * locals.var_nvtm_dn3)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn4 * assign33520_e56086) - (locals.var_t4 * (((if 0.0 >= locals.var_k2si_t { 0.0 } else { locals.var_k2si_t_dn4 } * locals.var_qis) + (assign33520_e56080 * locals.var_qis_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn5 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn5) + (2.0 * locals.var_nvtm_dn5)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn6 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn6) + (2.0 * locals.var_nvtm_dn6)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn7 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn7) + (2.0 * locals.var_nvtm_dn7)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn8 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn8) + (2.0 * locals.var_nvtm_dn8)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn9 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn9) + (2.0 * locals.var_nvtm_dn9)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn10 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn10) + (2.0 * locals.var_nvtm_dn10)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn11 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn11) + (2.0 * locals.var_nvtm_dn11)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn13 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn13) + (2.0 * locals.var_nvtm_dn13)))) / (assign33520_e56086 * assign33520_e56086)), (((locals.var_t4_dn14 * assign33520_e56086) - (locals.var_t4 * ((assign33520_e56080 * locals.var_qis_dn14) + (2.0 * locals.var_nvtm_dn14)))) / (assign33520_e56086 * assign33520_e56086)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33520_e56089;
        locals.var_t5_dn0 = assign33520_e56089_d_n0;
        locals.var_t5_dn2 = assign33520_e56089_d_n2;
        locals.var_t5_dn3 = assign33520_e56089_d_n3;
        locals.var_t5_dn4 = assign33520_e56089_d_n4;
        locals.var_t5_dn5 = assign33520_e56089_d_n5;
        locals.var_t5_dn6 = assign33520_e56089_d_n6;
        locals.var_t5_dn7 = assign33520_e56089_d_n7;
        locals.var_t5_dn8 = assign33520_e56089_d_n8;
        locals.var_t5_dn9 = assign33520_e56089_d_n9;
        locals.var_t5_dn10 = assign33520_e56089_d_n10;
        locals.var_t5_dn11 = assign33520_e56089_d_n11;
        locals.var_t5_dn13 = assign33520_e56089_d_n13;
        locals.var_t5_dn14 = assign33520_e56089_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33530_e56104, assign33530_e56104_d_n0, assign33530_e56104_d_n2, assign33530_e56104_d_n3, assign33530_e56104_d_n4, assign33530_e56104_d_n5, assign33530_e56104_d_n6, assign33530_e56104_d_n7, assign33530_e56104_d_n8, assign33530_e56104_d_n9, assign33530_e56104_d_n10, assign33530_e56104_d_n11, assign33530_e56104_d_n13, assign33530_e56104_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33530_e56098: f64 = (locals.var_phibe_i - locals.var_veseff);
        let assign33530_e56099: f64 = (assign33530_e56098).sqrt();
        let assign33530_e56101: f64 = (locals.var_phibe_i).sqrt();
        let assign33530_e56102: f64 = (assign33530_e56099 - assign33530_e56101);
        (assign33530_e56102, ((-locals.var_veseff_dn0) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn2) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn3) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn4) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn5) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn6) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn7) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn8) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn9) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn10) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn11) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn13) / (2.0 * assign33530_e56099)), ((-locals.var_veseff_dn14) / (2.0 * assign33530_e56099)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33530_e56104;
        locals.var_t6_dn0 = assign33530_e56104_d_n0;
        locals.var_t6_dn2 = assign33530_e56104_d_n2;
        locals.var_t6_dn3 = assign33530_e56104_d_n3;
        locals.var_t6_dn4 = assign33530_e56104_d_n4;
        locals.var_t6_dn5 = assign33530_e56104_d_n5;
        locals.var_t6_dn6 = assign33530_e56104_d_n6;
        locals.var_t6_dn7 = assign33530_e56104_d_n7;
        locals.var_t6_dn8 = assign33530_e56104_d_n8;
        locals.var_t6_dn9 = assign33530_e56104_d_n9;
        locals.var_t6_dn10 = assign33530_e56104_d_n10;
        locals.var_t6_dn11 = assign33530_e56104_d_n11;
        locals.var_t6_dn13 = assign33530_e56104_d_n13;
        locals.var_t6_dn14 = assign33530_e56104_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign33540_e56117, assign33540_e56117_d_n0, assign33540_e56117_d_n2, assign33540_e56117_d_n3, assign33540_e56117_d_n4, assign33540_e56117_d_n5, assign33540_e56117_d_n6, assign33540_e56117_d_n7, assign33540_e56117_d_n8, assign33540_e56117_d_n9, assign33540_e56117_d_n10, assign33540_e56117_d_n11, assign33540_e56117_d_n13, assign33540_e56117_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign33540_e56112: f64 = (-locals.var_t5);
        let assign33540_e56114: f64 = (assign33540_e56112 * locals.var_t6);
        let assign33540_e56115: f64 = { let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33540_e56115, ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn0) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn0))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn2) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn2))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn3) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn3))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn4) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn4))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn5) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn5))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn6) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn6))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn7) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn7))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn8) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn8))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn9) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn9))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn10) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn10))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn11) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn11))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn13) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn13))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-locals.var_t5_dn14) * locals.var_t6) + (assign33540_e56112 * locals.var_t6_dn14))),)
    } else {
        (locals.var_mob0, locals.var_mob0_dn0, locals.var_mob0_dn2, locals.var_mob0_dn3, locals.var_mob0_dn4, locals.var_mob0_dn5, locals.var_mob0_dn6, locals.var_mob0_dn7, locals.var_mob0_dn8, locals.var_mob0_dn9, locals.var_mob0_dn10, locals.var_mob0_dn11, locals.var_mob0_dn13, locals.var_mob0_dn14,)
    }
};
        locals.var_mob0 = assign33540_e56117;
        locals.var_mob0_dn0 = assign33540_e56117_d_n0;
        locals.var_mob0_dn2 = assign33540_e56117_d_n2;
        locals.var_mob0_dn3 = assign33540_e56117_d_n3;
        locals.var_mob0_dn4 = assign33540_e56117_d_n4;
        locals.var_mob0_dn5 = assign33540_e56117_d_n5;
        locals.var_mob0_dn6 = assign33540_e56117_d_n6;
        locals.var_mob0_dn7 = assign33540_e56117_d_n7;
        locals.var_mob0_dn8 = assign33540_e56117_d_n8;
        locals.var_mob0_dn9 = assign33540_e56117_d_n9;
        locals.var_mob0_dn10 = assign33540_e56117_d_n10;
        locals.var_mob0_dn11 = assign33540_e56117_d_n11;
        locals.var_mob0_dn13 = assign33540_e56117_d_n13;
        locals.var_mob0_dn14 = assign33540_e56117_d_n14;
        locals.var_mob0_rv = 0.0;

        let (assign33550_e56127, assign33550_e56127_d_n0, assign33550_e56127_d_n2, assign33550_e56127_d_n3, assign33550_e56127_d_n4, assign33550_e56127_d_n5, assign33550_e56127_d_n6, assign33550_e56127_d_n7, assign33550_e56127_d_n8, assign33550_e56127_d_n9, assign33550_e56127_d_n10, assign33550_e56127_d_n11, assign33550_e56127_d_n13, assign33550_e56127_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard634 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mob0, locals.var_mob0_dn0, locals.var_mob0_dn2, locals.var_mob0_dn3, locals.var_mob0_dn4, locals.var_mob0_dn5, locals.var_mob0_dn6, locals.var_mob0_dn7, locals.var_mob0_dn8, locals.var_mob0_dn9, locals.var_mob0_dn10, locals.var_mob0_dn11, locals.var_mob0_dn13, locals.var_mob0_dn14,)
    }
};
        locals.var_mob0 = assign33550_e56127;
        locals.var_mob0_dn0 = assign33550_e56127_d_n0;
        locals.var_mob0_dn2 = assign33550_e56127_d_n2;
        locals.var_mob0_dn3 = assign33550_e56127_d_n3;
        locals.var_mob0_dn4 = assign33550_e56127_d_n4;
        locals.var_mob0_dn5 = assign33550_e56127_d_n5;
        locals.var_mob0_dn6 = assign33550_e56127_d_n6;
        locals.var_mob0_dn7 = assign33550_e56127_d_n7;
        locals.var_mob0_dn8 = assign33550_e56127_d_n8;
        locals.var_mob0_dn9 = assign33550_e56127_d_n9;
        locals.var_mob0_dn10 = assign33550_e56127_d_n10;
        locals.var_mob0_dn11 = assign33550_e56127_d_n11;
        locals.var_mob0_dn13 = assign33550_e56127_d_n13;
        locals.var_mob0_dn14 = assign33550_e56127_d_n14;
        locals.var_mob0_rv = 0.0;

        let (assign33560_e56140, assign33560_e56140_d_n0, assign33560_e56140_d_n2, assign33560_e56140_d_n3, assign33560_e56140_d_n4, assign33560_e56140_d_n5, assign33560_e56140_d_n6, assign33560_e56140_d_n7, assign33560_e56140_d_n8, assign33560_e56140_d_n9, assign33560_e56140_d_n10, assign33560_e56140_d_n11, assign33560_e56140_d_n13, assign33560_e56140_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33560_e56136: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign33560_e56137: f64 = (locals.var_qba + assign33560_e56136);
        let assign33560_e56138: f64 = (locals.var_eefffactor * assign33560_e56137);
        (assign33560_e56138, (locals.var_eefffactor * (locals.var_qba_dn0 + (locals.var_eta_mu * locals.var_qis_dn0))), (locals.var_eefffactor * (locals.var_qba_dn2 + (locals.var_eta_mu * locals.var_qis_dn2))), (locals.var_eefffactor * (locals.var_qba_dn3 + (locals.var_eta_mu * locals.var_qis_dn3))), (locals.var_eefffactor * (locals.var_qba_dn4 + ((locals.var_eta_mu_dn4 * locals.var_qis) + (locals.var_eta_mu * locals.var_qis_dn4)))), (locals.var_eefffactor * (locals.var_qba_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_eefffactor * (locals.var_qba_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_eefffactor * (locals.var_qba_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_eefffactor * (locals.var_qba_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_eefffactor * (locals.var_qba_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))), (locals.var_eefffactor * (locals.var_qba_dn10 + (locals.var_eta_mu * locals.var_qis_dn10))), (locals.var_eefffactor * (locals.var_qba_dn11 + (locals.var_eta_mu * locals.var_qis_dn11))), (locals.var_eefffactor * (locals.var_qba_dn13 + (locals.var_eta_mu * locals.var_qis_dn13))), (locals.var_eefffactor * (locals.var_qba_dn14 + (locals.var_eta_mu * locals.var_qis_dn14))),)
    } else {
        (locals.var_eeffm0, locals.var_eeffm0_dn0, locals.var_eeffm0_dn2, locals.var_eeffm0_dn3, locals.var_eeffm0_dn4, locals.var_eeffm0_dn5, locals.var_eeffm0_dn6, locals.var_eeffm0_dn7, locals.var_eeffm0_dn8, locals.var_eeffm0_dn9, locals.var_eeffm0_dn10, locals.var_eeffm0_dn11, locals.var_eeffm0_dn13, locals.var_eeffm0_dn14,)
    }
};
        locals.var_eeffm0 = assign33560_e56140;
        locals.var_eeffm0_dn0 = assign33560_e56140_d_n0;
        locals.var_eeffm0_dn2 = assign33560_e56140_d_n2;
        locals.var_eeffm0_dn3 = assign33560_e56140_d_n3;
        locals.var_eeffm0_dn4 = assign33560_e56140_d_n4;
        locals.var_eeffm0_dn5 = assign33560_e56140_d_n5;
        locals.var_eeffm0_dn6 = assign33560_e56140_d_n6;
        locals.var_eeffm0_dn7 = assign33560_e56140_d_n7;
        locals.var_eeffm0_dn8 = assign33560_e56140_d_n8;
        locals.var_eeffm0_dn9 = assign33560_e56140_d_n9;
        locals.var_eeffm0_dn10 = assign33560_e56140_d_n10;
        locals.var_eeffm0_dn11 = assign33560_e56140_d_n11;
        locals.var_eeffm0_dn13 = assign33560_e56140_d_n13;
        locals.var_eeffm0_dn14 = assign33560_e56140_d_n14;
        locals.var_eeffm0_rv = 0.0;

        let (assign33570_e56156, assign33570_e56156_d_n0, assign33570_e56156_d_n2, assign33570_e56156_d_n3, assign33570_e56156_d_n4, assign33570_e56156_d_n5, assign33570_e56156_d_n6, assign33570_e56156_d_n7, assign33570_e56156_d_n8, assign33570_e56156_d_n9, assign33570_e56156_d_n10, assign33570_e56156_d_n11, assign33570_e56156_d_n13, assign33570_e56156_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33570_e56149: f64 = (locals.var_qis / locals.var_qb0);
        let assign33570_e56150: f64 = (assign33570_e56149).abs();
        let assign33570_e56151: f64 = (1.0 + assign33570_e56150);
        let assign33570_e56152: f64 = (0.5 * assign33570_e56151);
        let assign33570_e56154: f64 = (assign33570_e56152).powf(locals.var_ucs_t);
        (assign33570_e56154, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn0 / locals.var_qb0) } else { (-(locals.var_qis_dn0 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn0 / locals.var_qb0) } else { (-(locals.var_qis_dn0 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn2 / locals.var_qb0) } else { (-(locals.var_qis_dn2 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn2 / locals.var_qb0) } else { (-(locals.var_qis_dn2 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn3 / locals.var_qb0) } else { (-(locals.var_qis_dn3 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn3 / locals.var_qb0) } else { (-(locals.var_qis_dn3 / locals.var_qb0)) }) / assign33570_e56152))) }, if locals.var_ucs_t_dn4 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn4 / locals.var_qb0) } else { (-(locals.var_qis_dn4 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * ((locals.var_ucs_t_dn4 * (assign33570_e56152).ln()) + (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn4 / locals.var_qb0) } else { (-(locals.var_qis_dn4 / locals.var_qb0)) }) / assign33570_e56152)))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn5 / locals.var_qb0) } else { (-(locals.var_qis_dn5 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn5 / locals.var_qb0) } else { (-(locals.var_qis_dn5 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn6 / locals.var_qb0) } else { (-(locals.var_qis_dn6 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn6 / locals.var_qb0) } else { (-(locals.var_qis_dn6 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn7 / locals.var_qb0) } else { (-(locals.var_qis_dn7 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn7 / locals.var_qb0) } else { (-(locals.var_qis_dn7 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn8 / locals.var_qb0) } else { (-(locals.var_qis_dn8 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn8 / locals.var_qb0) } else { (-(locals.var_qis_dn8 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn9 / locals.var_qb0) } else { (-(locals.var_qis_dn9 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn9 / locals.var_qb0) } else { (-(locals.var_qis_dn9 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn10 / locals.var_qb0) } else { (-(locals.var_qis_dn10 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn10 / locals.var_qb0) } else { (-(locals.var_qis_dn10 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn11 / locals.var_qb0) } else { (-(locals.var_qis_dn11 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn11 / locals.var_qb0) } else { (-(locals.var_qis_dn11 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn13 / locals.var_qb0) } else { (-(locals.var_qis_dn13 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn13 / locals.var_qb0) } else { (-(locals.var_qis_dn13 / locals.var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign33570_e56152).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn14 / locals.var_qb0) } else { (-(locals.var_qis_dn14 / locals.var_qb0)) }))) } } else { (assign33570_e56154 * (locals.var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (locals.var_qis_dn14 / locals.var_qb0) } else { (-(locals.var_qis_dn14 / locals.var_qb0)) }) / assign33570_e56152))) },)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33570_e56156;
        locals.var_t4_dn0 = assign33570_e56156_d_n0;
        locals.var_t4_dn2 = assign33570_e56156_d_n2;
        locals.var_t4_dn3 = assign33570_e56156_d_n3;
        locals.var_t4_dn4 = assign33570_e56156_d_n4;
        locals.var_t4_dn5 = assign33570_e56156_d_n5;
        locals.var_t4_dn6 = assign33570_e56156_d_n6;
        locals.var_t4_dn7 = assign33570_e56156_d_n7;
        locals.var_t4_dn8 = assign33570_e56156_d_n8;
        locals.var_t4_dn9 = assign33570_e56156_d_n9;
        locals.var_t4_dn10 = assign33570_e56156_d_n10;
        locals.var_t4_dn11 = assign33570_e56156_d_n11;
        locals.var_t4_dn13 = assign33570_e56156_d_n13;
        locals.var_t4_dn14 = assign33570_e56156_d_n14;
        locals.var_t4_rv = 0.0;

        let assign33580_e56159: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign33580_e56159;
        locals.var_guard635_rv = 0.0;

        let (assign33590_e56181, assign33590_e56181_d_n0, assign33590_e56181_d_n2, assign33590_e56181_d_n3, assign33590_e56181_d_n4, assign33590_e56181_d_n5, assign33590_e56181_d_n6, assign33590_e56181_d_n7, assign33590_e56181_d_n8, assign33590_e56181_d_n9, assign33590_e56181_d_n10, assign33590_e56181_d_n11, assign33590_e56181_d_n13, assign33590_e56181_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard635 != 0.0)) {
        let assign33590_e56169: f64 = (locals.var_uc_a * locals.var_veseff);
        let assign33590_e56170: f64 = (locals.var_ua_a + assign33590_e56169);
        let assign33590_e56172: f64 = (locals.var_eeffm0).abs();
        let assign33590_e56174: f64 = (assign33590_e56172).powf(locals.var_eu_a);
        let assign33590_e56175: f64 = (assign33590_e56170 * assign33590_e56174);
        let assign33590_e56178: f64 = (locals.var_ud_a / locals.var_t4);
        let assign33590_e56179: f64 = (assign33590_e56175 + assign33590_e56178);
        (assign33590_e56179, ((((locals.var_ua_a_dn0 + ((locals.var_uc_a_dn0 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn0))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn0 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn0 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn2 + ((locals.var_uc_a_dn2 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn2))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn2 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn2 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn3))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn3 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn3 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn4))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn4 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn4 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn5))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn5 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn5 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn6))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn6 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn6 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn7))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn7 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn7 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn8))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn8 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn8 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn9))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn9 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn9 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn10))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn10 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn10 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn11))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn11 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn11 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn13 + ((locals.var_uc_a_dn13 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn13))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn13 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn13 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))), ((((locals.var_ua_a_dn14 + ((locals.var_uc_a_dn14 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn14))) * assign33590_e56174) + (assign33590_e56170 * if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33590_e56172).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) })) } } else { (assign33590_e56174 * ((locals.var_eu_a_dn14 * (assign33590_e56172).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) } / assign33590_e56172)))) })) + (((locals.var_ud_a_dn14 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33590_e56181;
        locals.var_t5_dn0 = assign33590_e56181_d_n0;
        locals.var_t5_dn2 = assign33590_e56181_d_n2;
        locals.var_t5_dn3 = assign33590_e56181_d_n3;
        locals.var_t5_dn4 = assign33590_e56181_d_n4;
        locals.var_t5_dn5 = assign33590_e56181_d_n5;
        locals.var_t5_dn6 = assign33590_e56181_d_n6;
        locals.var_t5_dn7 = assign33590_e56181_d_n7;
        locals.var_t5_dn8 = assign33590_e56181_d_n8;
        locals.var_t5_dn9 = assign33590_e56181_d_n9;
        locals.var_t5_dn10 = assign33590_e56181_d_n10;
        locals.var_t5_dn11 = assign33590_e56181_d_n11;
        locals.var_t5_dn13 = assign33590_e56181_d_n13;
        locals.var_t5_dn14 = assign33590_e56181_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33600_e56200, assign33600_e56200_d_n0, assign33600_e56200_d_n2, assign33600_e56200_d_n3, assign33600_e56200_d_n4, assign33600_e56200_d_n5, assign33600_e56200_d_n6, assign33600_e56200_d_n7, assign33600_e56200_d_n8, assign33600_e56200_d_n9, assign33600_e56200_d_n10, assign33600_e56200_d_n11, assign33600_e56200_d_n13, assign33600_e56200_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard635 == 0.0)) {
        let assign33600_e56191: f64 = (locals.var_eeffm0).abs();
        let assign33600_e56193: f64 = (assign33600_e56191).powf(locals.var_eu_a);
        let assign33600_e56194: f64 = (locals.var_ua_a * assign33600_e56193);
        let assign33600_e56197: f64 = (locals.var_ud_a / locals.var_t4);
        let assign33600_e56198: f64 = (assign33600_e56194 + assign33600_e56197);
        (assign33600_e56198, (((locals.var_ua_a_dn0 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn0 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn0 } else { (-locals.var_eeffm0_dn0) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn0 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn2 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn2 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn2 } else { (-locals.var_eeffm0_dn2) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn2 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn3 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn3 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn3 } else { (-locals.var_eeffm0_dn3) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn3 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn4 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn4 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn4 } else { (-locals.var_eeffm0_dn4) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn4 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn5 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn5 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn5 } else { (-locals.var_eeffm0_dn5) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn5 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn6 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn6 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn6 } else { (-locals.var_eeffm0_dn6) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn6 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn7 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn7 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn7 } else { (-locals.var_eeffm0_dn7) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn7 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn8 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn8 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn8 } else { (-locals.var_eeffm0_dn8) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn8 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn9 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn9 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn9 } else { (-locals.var_eeffm0_dn9) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn9 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn10 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn10 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn10 } else { (-locals.var_eeffm0_dn10) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn10 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn11 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn11 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn11 } else { (-locals.var_eeffm0_dn11) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn11 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn13 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn13 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn13 } else { (-locals.var_eeffm0_dn13) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn13 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))), (((locals.var_ua_a_dn14 * assign33600_e56193) + (locals.var_ua_a * if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((assign33600_e56191).powf(locals.var_eu_a - 1.0) * if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) })) } } else { (assign33600_e56193 * ((locals.var_eu_a_dn14 * (assign33600_e56191).ln()) + (locals.var_eu_a * (if locals.var_eeffm0 >= 0.0 { locals.var_eeffm0_dn14 } else { (-locals.var_eeffm0_dn14) } / assign33600_e56191)))) })) + (((locals.var_ud_a_dn14 * locals.var_t4) - (locals.var_ud_a * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33600_e56200;
        locals.var_t5_dn0 = assign33600_e56200_d_n0;
        locals.var_t5_dn2 = assign33600_e56200_d_n2;
        locals.var_t5_dn3 = assign33600_e56200_d_n3;
        locals.var_t5_dn4 = assign33600_e56200_d_n4;
        locals.var_t5_dn5 = assign33600_e56200_d_n5;
        locals.var_t5_dn6 = assign33600_e56200_d_n6;
        locals.var_t5_dn7 = assign33600_e56200_d_n7;
        locals.var_t5_dn8 = assign33600_e56200_d_n8;
        locals.var_t5_dn9 = assign33600_e56200_d_n9;
        locals.var_t5_dn10 = assign33600_e56200_d_n10;
        locals.var_t5_dn11 = assign33600_e56200_d_n11;
        locals.var_t5_dn13 = assign33600_e56200_d_n13;
        locals.var_t5_dn14 = assign33600_e56200_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33610_e56209, assign33610_e56209_d_n0, assign33610_e56209_d_n2, assign33610_e56209_d_n3, assign33610_e56209_d_n4, assign33610_e56209_d_n5, assign33610_e56209_d_n6, assign33610_e56209_d_n7, assign33610_e56209_d_n8, assign33610_e56209_d_n9, assign33610_e56209_d_n10, assign33610_e56209_d_n11, assign33610_e56209_d_n13, assign33610_e56209_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33610_e56207: f64 = (1.0 + locals.var_t5);
        (assign33610_e56207, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33610_e56209;
        locals.var_dmob0_dn0 = assign33610_e56209_d_n0;
        locals.var_dmob0_dn2 = assign33610_e56209_d_n2;
        locals.var_dmob0_dn3 = assign33610_e56209_d_n3;
        locals.var_dmob0_dn4 = assign33610_e56209_d_n4;
        locals.var_dmob0_dn5 = assign33610_e56209_d_n5;
        locals.var_dmob0_dn6 = assign33610_e56209_d_n6;
        locals.var_dmob0_dn7 = assign33610_e56209_d_n7;
        locals.var_dmob0_dn8 = assign33610_e56209_d_n8;
        locals.var_dmob0_dn9 = assign33610_e56209_d_n9;
        locals.var_dmob0_dn10 = assign33610_e56209_d_n10;
        locals.var_dmob0_dn11 = assign33610_e56209_d_n11;
        locals.var_dmob0_dn13 = assign33610_e56209_d_n13;
        locals.var_dmob0_dn14 = assign33610_e56209_d_n14;
        locals.var_dmob0_rv = 0.0;

        let (assign33620_e56235, assign33620_e56235_d_n0, assign33620_e56235_d_n2, assign33620_e56235_d_n3, assign33620_e56235_d_n4, assign33620_e56235_d_n5, assign33620_e56235_d_n6, assign33620_e56235_d_n7, assign33620_e56235_d_n8, assign33620_e56235_d_n9, assign33620_e56235_d_n10, assign33620_e56235_d_n11, assign33620_e56235_d_n13, assign33620_e56235_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33620_e56217: f64 = (locals.var_dmob0 + 1.0);
        let assign33620_e56220: f64 = (locals.var_dmob0 - 1.0);
        let assign33620_e56223: f64 = (locals.var_dmob0 - 1.0);
        let assign33620_e56224: f64 = (assign33620_e56220 * assign33620_e56223);
        let assign33620_e56227: f64 = (0.25 * p.p604);
        let assign33620_e56229: f64 = (assign33620_e56227 * p.p604);
        let assign33620_e56230: f64 = (assign33620_e56224 + assign33620_e56229);
        let assign33620_e56231: f64 = (assign33620_e56230).sqrt();
        let assign33620_e56232: f64 = (assign33620_e56217 + assign33620_e56231);
        let assign33620_e56233: f64 = (0.5 * assign33620_e56232);
        (assign33620_e56233, (0.5 * (locals.var_dmob0_dn0 + (((locals.var_dmob0_dn0 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn0)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn2 + (((locals.var_dmob0_dn2 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn2)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn3 + (((locals.var_dmob0_dn3 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn3)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn4 + (((locals.var_dmob0_dn4 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn4)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn5 + (((locals.var_dmob0_dn5 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn5)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn6 + (((locals.var_dmob0_dn6 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn6)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn7 + (((locals.var_dmob0_dn7 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn7)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn8 + (((locals.var_dmob0_dn8 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn8)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn9 + (((locals.var_dmob0_dn9 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn9)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn10 + (((locals.var_dmob0_dn10 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn10)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn11 + (((locals.var_dmob0_dn11 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn11)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn13 + (((locals.var_dmob0_dn13 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn13)) / (2.0 * assign33620_e56231)))), (0.5 * (locals.var_dmob0_dn14 + (((locals.var_dmob0_dn14 * assign33620_e56223) + (assign33620_e56220 * locals.var_dmob0_dn14)) / (2.0 * assign33620_e56231)))),)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33620_e56235;
        locals.var_dmob0_dn0 = assign33620_e56235_d_n0;
        locals.var_dmob0_dn2 = assign33620_e56235_d_n2;
        locals.var_dmob0_dn3 = assign33620_e56235_d_n3;
        locals.var_dmob0_dn4 = assign33620_e56235_d_n4;
        locals.var_dmob0_dn5 = assign33620_e56235_d_n5;
        locals.var_dmob0_dn6 = assign33620_e56235_d_n6;
        locals.var_dmob0_dn7 = assign33620_e56235_d_n7;
        locals.var_dmob0_dn8 = assign33620_e56235_d_n8;
        locals.var_dmob0_dn9 = assign33620_e56235_d_n9;
        locals.var_dmob0_dn10 = assign33620_e56235_d_n10;
        locals.var_dmob0_dn11 = assign33620_e56235_d_n11;
        locals.var_dmob0_dn13 = assign33620_e56235_d_n13;
        locals.var_dmob0_dn14 = assign33620_e56235_d_n14;
        locals.var_dmob0_rv = 0.0;

        let (assign33630_e56244, assign33630_e56244_d_n0, assign33630_e56244_d_n2, assign33630_e56244_d_n3, assign33630_e56244_d_n4, assign33630_e56244_d_n5, assign33630_e56244_d_n6, assign33630_e56244_d_n7, assign33630_e56244_d_n8, assign33630_e56244_d_n9, assign33630_e56244_d_n10, assign33630_e56244_d_n11, assign33630_e56244_d_n13, assign33630_e56244_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33630_e56242: f64 = (locals.var_dmob0 / p.p24);
        (assign33630_e56242, (locals.var_dmob0_dn0 / p.p24), (locals.var_dmob0_dn2 / p.p24), (locals.var_dmob0_dn3 / p.p24), (locals.var_dmob0_dn4 / p.p24), (locals.var_dmob0_dn5 / p.p24), (locals.var_dmob0_dn6 / p.p24), (locals.var_dmob0_dn7 / p.p24), (locals.var_dmob0_dn8 / p.p24), (locals.var_dmob0_dn9 / p.p24), (locals.var_dmob0_dn10 / p.p24), (locals.var_dmob0_dn11 / p.p24), (locals.var_dmob0_dn13 / p.p24), (locals.var_dmob0_dn14 / p.p24),)
    } else {
        (locals.var_dmob0, locals.var_dmob0_dn0, locals.var_dmob0_dn2, locals.var_dmob0_dn3, locals.var_dmob0_dn4, locals.var_dmob0_dn5, locals.var_dmob0_dn6, locals.var_dmob0_dn7, locals.var_dmob0_dn8, locals.var_dmob0_dn9, locals.var_dmob0_dn10, locals.var_dmob0_dn11, locals.var_dmob0_dn13, locals.var_dmob0_dn14,)
    }
};
        locals.var_dmob0 = assign33630_e56244;
        locals.var_dmob0_dn0 = assign33630_e56244_d_n0;
        locals.var_dmob0_dn2 = assign33630_e56244_d_n2;
        locals.var_dmob0_dn3 = assign33630_e56244_d_n3;
        locals.var_dmob0_dn4 = assign33630_e56244_d_n4;
        locals.var_dmob0_dn5 = assign33630_e56244_d_n5;
        locals.var_dmob0_dn6 = assign33630_e56244_d_n6;
        locals.var_dmob0_dn7 = assign33630_e56244_d_n7;
        locals.var_dmob0_dn8 = assign33630_e56244_d_n8;
        locals.var_dmob0_dn9 = assign33630_e56244_d_n9;
        locals.var_dmob0_dn10 = assign33630_e56244_d_n10;
        locals.var_dmob0_dn11 = assign33630_e56244_d_n11;
        locals.var_dmob0_dn13 = assign33630_e56244_d_n13;
        locals.var_dmob0_dn14 = assign33630_e56244_d_n14;
        locals.var_dmob0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_131(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33640_e56255,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33640_e56252: f64 = (0.25 * p.p453);
        let assign33640_e56253: f64 = (1.0 + assign33640_e56252);
        (assign33640_e56253,)
    } else {
        (locals.var_dvsat0,)
    }
};
        locals.var_dvsat0 = assign33640_e56255;
        locals.var_dvsat0_rv = 0.0;

        let (assign33650_e56266, assign33650_e56266_d_n0, assign33650_e56266_d_n2, assign33650_e56266_d_n3, assign33650_e56266_d_n4, assign33650_e56266_d_n5, assign33650_e56266_d_n6, assign33650_e56266_d_n7, assign33650_e56266_d_n8, assign33650_e56266_d_n9, assign33650_e56266_d_n10, assign33650_e56266_d_n11, assign33650_e56266_d_n13, assign33650_e56266_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33650_e56263: f64 = (locals.var_q0 + locals.var_qis);
        let assign33650_e56264: f64 = (locals.var_q0 / assign33650_e56263);
        (assign33650_e56264, (((locals.var_q0_dn0 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn0 + locals.var_qis_dn0))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn2 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn2 + locals.var_qis_dn2))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn3 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn3 + locals.var_qis_dn3))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn4 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn4 + locals.var_qis_dn4))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn5 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn5 + locals.var_qis_dn5))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn6 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn6 + locals.var_qis_dn6))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn7 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn7 + locals.var_qis_dn7))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn8 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn8 + locals.var_qis_dn8))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn9 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn9 + locals.var_qis_dn9))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn10 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn10 + locals.var_qis_dn10))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn11 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn11 + locals.var_qis_dn11))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn13 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn13 + locals.var_qis_dn13))) / (assign33650_e56263 * assign33650_e56263)), (((locals.var_q0_dn14 * assign33650_e56263) - (locals.var_q0 * (locals.var_q0_dn14 + locals.var_qis_dn14))) / (assign33650_e56263 * assign33650_e56263)),)
    } else {
        (locals.var_etaiv0, locals.var_etaiv0_dn0, locals.var_etaiv0_dn2, locals.var_etaiv0_dn3, locals.var_etaiv0_dn4, locals.var_etaiv0_dn5, locals.var_etaiv0_dn6, locals.var_etaiv0_dn7, locals.var_etaiv0_dn8, locals.var_etaiv0_dn9, locals.var_etaiv0_dn10, locals.var_etaiv0_dn11, locals.var_etaiv0_dn13, locals.var_etaiv0_dn14,)
    }
};
        locals.var_etaiv0 = assign33650_e56266;
        locals.var_etaiv0_dn0 = assign33650_e56266_d_n0;
        locals.var_etaiv0_dn2 = assign33650_e56266_d_n2;
        locals.var_etaiv0_dn3 = assign33650_e56266_d_n3;
        locals.var_etaiv0_dn4 = assign33650_e56266_d_n4;
        locals.var_etaiv0_dn5 = assign33650_e56266_d_n5;
        locals.var_etaiv0_dn6 = assign33650_e56266_d_n6;
        locals.var_etaiv0_dn7 = assign33650_e56266_d_n7;
        locals.var_etaiv0_dn8 = assign33650_e56266_d_n8;
        locals.var_etaiv0_dn9 = assign33650_e56266_d_n9;
        locals.var_etaiv0_dn10 = assign33650_e56266_d_n10;
        locals.var_etaiv0_dn11 = assign33650_e56266_d_n11;
        locals.var_etaiv0_dn13 = assign33650_e56266_d_n13;
        locals.var_etaiv0_dn14 = assign33650_e56266_d_n14;
        locals.var_etaiv0_rv = 0.0;

        let (assign33660_e56277, assign33660_e56277_d_n0, assign33660_e56277_d_n2, assign33660_e56277_d_n3, assign33660_e56277_d_n4, assign33660_e56277_d_n5, assign33660_e56277_d_n6, assign33660_e56277_d_n7, assign33660_e56277_d_n8, assign33660_e56277_d_n9, assign33660_e56277_d_n10, assign33660_e56277_d_n11, assign33660_e56277_d_n13, assign33660_e56277_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33660_e56273: f64 = (2.0 - locals.var_etaiv0);
        let assign33660_e56275: f64 = (assign33660_e56273 * locals.var_nvtm);
        (assign33660_e56275, (((-locals.var_etaiv0_dn0) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn0)), (((-locals.var_etaiv0_dn2) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn2)), (((-locals.var_etaiv0_dn3) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn3)), (((-locals.var_etaiv0_dn4) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn4)), (((-locals.var_etaiv0_dn5) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn5)), (((-locals.var_etaiv0_dn6) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn6)), (((-locals.var_etaiv0_dn7) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn7)), (((-locals.var_etaiv0_dn8) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn8)), (((-locals.var_etaiv0_dn9) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn9)), (((-locals.var_etaiv0_dn10) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn10)), (((-locals.var_etaiv0_dn11) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn11)), (((-locals.var_etaiv0_dn13) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn13)), (((-locals.var_etaiv0_dn14) * locals.var_nvtm) + (assign33660_e56273 * locals.var_nvtm_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33660_e56277;
        locals.var_t4_dn0 = assign33660_e56277_d_n0;
        locals.var_t4_dn2 = assign33660_e56277_d_n2;
        locals.var_t4_dn3 = assign33660_e56277_d_n3;
        locals.var_t4_dn4 = assign33660_e56277_d_n4;
        locals.var_t4_dn5 = assign33660_e56277_d_n5;
        locals.var_t4_dn6 = assign33660_e56277_d_n6;
        locals.var_t4_dn7 = assign33660_e56277_d_n7;
        locals.var_t4_dn8 = assign33660_e56277_d_n8;
        locals.var_t4_dn9 = assign33660_e56277_d_n9;
        locals.var_t4_dn10 = assign33660_e56277_d_n10;
        locals.var_t4_dn11 = assign33660_e56277_d_n11;
        locals.var_t4_dn13 = assign33660_e56277_d_n13;
        locals.var_t4_dn14 = assign33660_e56277_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33670_e56286, assign33670_e56286_d_n0, assign33670_e56286_d_n2, assign33670_e56286_d_n3, assign33670_e56286_d_n4, assign33670_e56286_d_n5, assign33670_e56286_d_n6, assign33670_e56286_d_n7, assign33670_e56286_d_n8, assign33670_e56286_d_n9, assign33670_e56286_d_n10, assign33670_e56286_d_n11, assign33670_e56286_d_n13, assign33670_e56286_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33670_e56284: f64 = (locals.var_qis + locals.var_t4);
        (assign33670_e56284, (locals.var_qis_dn0 + locals.var_t4_dn0), (locals.var_qis_dn2 + locals.var_t4_dn2), (locals.var_qis_dn3 + locals.var_t4_dn3), (locals.var_qis_dn4 + locals.var_t4_dn4), (locals.var_qis_dn5 + locals.var_t4_dn5), (locals.var_qis_dn6 + locals.var_t4_dn6), (locals.var_qis_dn7 + locals.var_t4_dn7), (locals.var_qis_dn8 + locals.var_t4_dn8), (locals.var_qis_dn9 + locals.var_t4_dn9), (locals.var_qis_dn10 + locals.var_t4_dn10), (locals.var_qis_dn11 + locals.var_t4_dn11), (locals.var_qis_dn13 + locals.var_t4_dn13), (locals.var_qis_dn14 + locals.var_t4_dn14),)
    } else {
        (locals.var_ids0_ov_dqi0, locals.var_ids0_ov_dqi0_dn0, locals.var_ids0_ov_dqi0_dn2, locals.var_ids0_ov_dqi0_dn3, locals.var_ids0_ov_dqi0_dn4, locals.var_ids0_ov_dqi0_dn5, locals.var_ids0_ov_dqi0_dn6, locals.var_ids0_ov_dqi0_dn7, locals.var_ids0_ov_dqi0_dn8, locals.var_ids0_ov_dqi0_dn9, locals.var_ids0_ov_dqi0_dn10, locals.var_ids0_ov_dqi0_dn11, locals.var_ids0_ov_dqi0_dn13, locals.var_ids0_ov_dqi0_dn14,)
    }
};
        locals.var_ids0_ov_dqi0 = assign33670_e56286;
        locals.var_ids0_ov_dqi0_dn0 = assign33670_e56286_d_n0;
        locals.var_ids0_ov_dqi0_dn2 = assign33670_e56286_d_n2;
        locals.var_ids0_ov_dqi0_dn3 = assign33670_e56286_d_n3;
        locals.var_ids0_ov_dqi0_dn4 = assign33670_e56286_d_n4;
        locals.var_ids0_ov_dqi0_dn5 = assign33670_e56286_d_n5;
        locals.var_ids0_ov_dqi0_dn6 = assign33670_e56286_d_n6;
        locals.var_ids0_ov_dqi0_dn7 = assign33670_e56286_d_n7;
        locals.var_ids0_ov_dqi0_dn8 = assign33670_e56286_d_n8;
        locals.var_ids0_ov_dqi0_dn9 = assign33670_e56286_d_n9;
        locals.var_ids0_ov_dqi0_dn10 = assign33670_e56286_d_n10;
        locals.var_ids0_ov_dqi0_dn11 = assign33670_e56286_d_n11;
        locals.var_ids0_ov_dqi0_dn13 = assign33670_e56286_d_n13;
        locals.var_ids0_ov_dqi0_dn14 = assign33670_e56286_d_n14;
        locals.var_ids0_ov_dqi0_rv = 0.0;

        let assign33680_e56289: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign33680_e56289;
        locals.var_guard636_rv = 0.0;

        let assign33690_e56292: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign33690_e56292;
        locals.var_guard637_rv = 0.0;

        let assign33700_e56295: f64 = if p.p64 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign33700_e56295;
        locals.var_guard638_rv = 0.0;

        let (assign33710_e56308, assign33710_e56308_d_n0, assign33710_e56308_d_n2, assign33710_e56308_d_n3, assign33710_e56308_d_n4, assign33710_e56308_d_n5, assign33710_e56308_d_n6, assign33710_e56308_d_n7, assign33710_e56308_d_n8, assign33710_e56308_d_n9, assign33710_e56308_d_n10, assign33710_e56308_d_n11, assign33710_e56308_d_n13, assign33710_e56308_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33710_e56305: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign33710_e56306: f64 = (1.0 + assign33710_e56305);
        (assign33710_e56306, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33710_e56308;
        locals.var_t4_dn0 = assign33710_e56308_d_n0;
        locals.var_t4_dn2 = assign33710_e56308_d_n2;
        locals.var_t4_dn3 = assign33710_e56308_d_n3;
        locals.var_t4_dn4 = assign33710_e56308_d_n4;
        locals.var_t4_dn5 = assign33710_e56308_d_n5;
        locals.var_t4_dn6 = assign33710_e56308_d_n6;
        locals.var_t4_dn7 = assign33710_e56308_d_n7;
        locals.var_t4_dn8 = assign33710_e56308_d_n8;
        locals.var_t4_dn9 = assign33710_e56308_d_n9;
        locals.var_t4_dn10 = assign33710_e56308_d_n10;
        locals.var_t4_dn11 = assign33710_e56308_d_n11;
        locals.var_t4_dn13 = assign33710_e56308_d_n13;
        locals.var_t4_dn14 = assign33710_e56308_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33720_e56319, assign33720_e56319_d_n0, assign33720_e56319_d_n2, assign33720_e56319_d_n3, assign33720_e56319_d_n4, assign33720_e56319_d_n5, assign33720_e56319_d_n6, assign33720_e56319_d_n7, assign33720_e56319_d_n8, assign33720_e56319_d_n9, assign33720_e56319_d_n10, assign33720_e56319_d_n11, assign33720_e56319_d_n13, assign33720_e56319_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33720_e56317: f64 = (1.0 / locals.var_t4);
        (assign33720_e56317, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33720_e56319;
        locals.var_t5_dn0 = assign33720_e56319_d_n0;
        locals.var_t5_dn2 = assign33720_e56319_d_n2;
        locals.var_t5_dn3 = assign33720_e56319_d_n3;
        locals.var_t5_dn4 = assign33720_e56319_d_n4;
        locals.var_t5_dn5 = assign33720_e56319_d_n5;
        locals.var_t5_dn6 = assign33720_e56319_d_n6;
        locals.var_t5_dn7 = assign33720_e56319_d_n7;
        locals.var_t5_dn8 = assign33720_e56319_d_n8;
        locals.var_t5_dn9 = assign33720_e56319_d_n9;
        locals.var_t5_dn10 = assign33720_e56319_d_n10;
        locals.var_t5_dn11 = assign33720_e56319_d_n11;
        locals.var_t5_dn13 = assign33720_e56319_d_n13;
        locals.var_t5_dn14 = assign33720_e56319_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33730_e56337, assign33730_e56337_d_n0, assign33730_e56337_d_n2, assign33730_e56337_d_n3, assign33730_e56337_d_n4, assign33730_e56337_d_n5, assign33730_e56337_d_n6, assign33730_e56337_d_n7, assign33730_e56337_d_n8, assign33730_e56337_d_n9, assign33730_e56337_d_n10, assign33730_e56337_d_n11, assign33730_e56337_d_n13, assign33730_e56337_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33730_e56330: f64 = (locals.var_t5 * locals.var_t5);
        let assign33730_e56332: f64 = (assign33730_e56330 + 0.01);
        let assign33730_e56333: f64 = (assign33730_e56332).sqrt();
        let assign33730_e56334: f64 = (locals.var_t5 + assign33730_e56333);
        let assign33730_e56335: f64 = (0.5 * assign33730_e56334);
        (assign33730_e56335, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign33730_e56333)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign33730_e56333)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33730_e56337;
        locals.var_t6_dn0 = assign33730_e56337_d_n0;
        locals.var_t6_dn2 = assign33730_e56337_d_n2;
        locals.var_t6_dn3 = assign33730_e56337_d_n3;
        locals.var_t6_dn4 = assign33730_e56337_d_n4;
        locals.var_t6_dn5 = assign33730_e56337_d_n5;
        locals.var_t6_dn6 = assign33730_e56337_d_n6;
        locals.var_t6_dn7 = assign33730_e56337_d_n7;
        locals.var_t6_dn8 = assign33730_e56337_d_n8;
        locals.var_t6_dn9 = assign33730_e56337_d_n9;
        locals.var_t6_dn10 = assign33730_e56337_d_n10;
        locals.var_t6_dn11 = assign33730_e56337_d_n11;
        locals.var_t6_dn13 = assign33730_e56337_d_n13;
        locals.var_t6_dn14 = assign33730_e56337_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign33740_e56354, assign33740_e56354_d_n0, assign33740_e56354_d_n2, assign33740_e56354_d_n3, assign33740_e56354_d_n4, assign33740_e56354_d_n5, assign33740_e56354_d_n6, assign33740_e56354_d_n7, assign33740_e56354_d_n8, assign33740_e56354_d_n9, assign33740_e56354_d_n10, assign33740_e56354_d_n11, assign33740_e56354_d_n13, assign33740_e56354_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33740_e56348: f64 = (locals.var_rdsw_i * locals.var_t6);
        let assign33740_e56349: f64 = (p.p908 + assign33740_e56348);
        let assign33740_e56350: f64 = (locals.var_rdstemp * assign33740_e56349);
        let assign33740_e56352: f64 = (assign33740_e56350 * locals.var_weffwrfactor);
        (assign33740_e56352, (((locals.var_rdstemp_dn0 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn0 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn0)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn2 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn2 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn2)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn3 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn3 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn3)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn4 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn4 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn4)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn5 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn5 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn5)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn6 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn6 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn6)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn7 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn7 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn7)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn8 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn8 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn8)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn9 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn9 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn9)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn10 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn10 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn10)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn11 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn11 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn11)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn13 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn13 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn13)))) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn14 * assign33740_e56349) + (locals.var_rdstemp * ((locals.var_rdsw_i_dn14 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn14)))) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33740_e56354;
        locals.var_rdsi0_dn0 = assign33740_e56354_d_n0;
        locals.var_rdsi0_dn2 = assign33740_e56354_d_n2;
        locals.var_rdsi0_dn3 = assign33740_e56354_d_n3;
        locals.var_rdsi0_dn4 = assign33740_e56354_d_n4;
        locals.var_rdsi0_dn5 = assign33740_e56354_d_n5;
        locals.var_rdsi0_dn6 = assign33740_e56354_d_n6;
        locals.var_rdsi0_dn7 = assign33740_e56354_d_n7;
        locals.var_rdsi0_dn8 = assign33740_e56354_d_n8;
        locals.var_rdsi0_dn9 = assign33740_e56354_d_n9;
        locals.var_rdsi0_dn10 = assign33740_e56354_d_n10;
        locals.var_rdsi0_dn11 = assign33740_e56354_d_n11;
        locals.var_rdsi0_dn13 = assign33740_e56354_d_n13;
        locals.var_rdsi0_dn14 = assign33740_e56354_d_n14;
        locals.var_rdsi0_rv = 0.0;

        let (assign33750_e56375, assign33750_e56375_d_n0, assign33750_e56375_d_n2, assign33750_e56375_d_n3, assign33750_e56375_d_n4, assign33750_e56375_d_n5, assign33750_e56375_d_n6, assign33750_e56375_d_n7, assign33750_e56375_d_n8, assign33750_e56375_d_n9, assign33750_e56375_d_n10, assign33750_e56375_d_n11, assign33750_e56375_d_n13, assign33750_e56375_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign33750_e56364: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33750_e56366: f64 = (assign33750_e56364 * locals.var_ids0_ov_dqi0);
        let assign33750_e56369: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33750_e56370: f64 = (assign33750_e56366 / assign33750_e56369);
        let assign33750_e56372: f64 = (assign33750_e56370 * locals.var_rdsi0);
        let assign33750_e56373: f64 = (1.0 + assign33750_e56372);
        (assign33750_e56373, ((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn0)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn0 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn0)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn2)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn2 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn2)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn3)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn3 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn3)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn4)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn4 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn4)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn5)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn5 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn5)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn6)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn6 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn6)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn7)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn7 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn7)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn8)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn8 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn8)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn9)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn9 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn9)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn10)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn10 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn10)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn11)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn11 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn11)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn13)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn13 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn13)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_ids0_ov_dqi0) + (assign33750_e56364 * locals.var_ids0_ov_dqi0_dn14)) * assign33750_e56369) - (assign33750_e56366 * (locals.var_dmob0_dn14 * locals.var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * locals.var_rdsi0) + (assign33750_e56370 * locals.var_rdsi0_dn14)),)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33750_e56375;
        locals.var_dr0_dn0 = assign33750_e56375_d_n0;
        locals.var_dr0_dn2 = assign33750_e56375_d_n2;
        locals.var_dr0_dn3 = assign33750_e56375_d_n3;
        locals.var_dr0_dn4 = assign33750_e56375_d_n4;
        locals.var_dr0_dn5 = assign33750_e56375_d_n5;
        locals.var_dr0_dn6 = assign33750_e56375_d_n6;
        locals.var_dr0_dn7 = assign33750_e56375_d_n7;
        locals.var_dr0_dn8 = assign33750_e56375_d_n8;
        locals.var_dr0_dn9 = assign33750_e56375_d_n9;
        locals.var_dr0_dn10 = assign33750_e56375_d_n10;
        locals.var_dr0_dn11 = assign33750_e56375_d_n11;
        locals.var_dr0_dn13 = assign33750_e56375_d_n13;
        locals.var_dr0_dn14 = assign33750_e56375_d_n14;
        locals.var_dr0_rv = 0.0;

        let (assign33760_e56387, assign33760_e56387_d_n0, assign33760_e56387_d_n2, assign33760_e56387_d_n3, assign33760_e56387_d_n4, assign33760_e56387_d_n5, assign33760_e56387_d_n6, assign33760_e56387_d_n7, assign33760_e56387_d_n8, assign33760_e56387_d_n9, assign33760_e56387_d_n10, assign33760_e56387_d_n11, assign33760_e56387_d_n13, assign33760_e56387_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard637 != 0.0) && (locals.var_guard636 == 0.0))) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33760_e56387;
        locals.var_dr0_dn0 = assign33760_e56387_d_n0;
        locals.var_dr0_dn2 = assign33760_e56387_d_n2;
        locals.var_dr0_dn3 = assign33760_e56387_d_n3;
        locals.var_dr0_dn4 = assign33760_e56387_d_n4;
        locals.var_dr0_dn5 = assign33760_e56387_d_n5;
        locals.var_dr0_dn6 = assign33760_e56387_d_n6;
        locals.var_dr0_dn7 = assign33760_e56387_d_n7;
        locals.var_dr0_dn8 = assign33760_e56387_d_n8;
        locals.var_dr0_dn9 = assign33760_e56387_d_n9;
        locals.var_dr0_dn10 = assign33760_e56387_d_n10;
        locals.var_dr0_dn11 = assign33760_e56387_d_n11;
        locals.var_dr0_dn13 = assign33760_e56387_d_n13;
        locals.var_dr0_dn14 = assign33760_e56387_d_n14;
        locals.var_dr0_rv = 0.0;

        let (assign33770_e56405, assign33770_e56405_d_n0, assign33770_e56405_d_n2, assign33770_e56405_d_n3, assign33770_e56405_d_n4, assign33770_e56405_d_n5, assign33770_e56405_d_n6, assign33770_e56405_d_n7, assign33770_e56405_d_n8, assign33770_e56405_d_n9, assign33770_e56405_d_n10, assign33770_e56405_d_n11, assign33770_e56405_d_n13, assign33770_e56405_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33770_e56402: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign33770_e56403: f64 = (1.0 + assign33770_e56402);
        (assign33770_e56403, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33770_e56405;
        locals.var_t4_dn0 = assign33770_e56405_d_n0;
        locals.var_t4_dn2 = assign33770_e56405_d_n2;
        locals.var_t4_dn3 = assign33770_e56405_d_n3;
        locals.var_t4_dn4 = assign33770_e56405_d_n4;
        locals.var_t4_dn5 = assign33770_e56405_d_n5;
        locals.var_t4_dn6 = assign33770_e56405_d_n6;
        locals.var_t4_dn7 = assign33770_e56405_d_n7;
        locals.var_t4_dn8 = assign33770_e56405_d_n8;
        locals.var_t4_dn9 = assign33770_e56405_d_n9;
        locals.var_t4_dn10 = assign33770_e56405_d_n10;
        locals.var_t4_dn11 = assign33770_e56405_d_n11;
        locals.var_t4_dn13 = assign33770_e56405_d_n13;
        locals.var_t4_dn14 = assign33770_e56405_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33780_e56421, assign33780_e56421_d_n0, assign33780_e56421_d_n2, assign33780_e56421_d_n3, assign33780_e56421_d_n4, assign33780_e56421_d_n5, assign33780_e56421_d_n6, assign33780_e56421_d_n7, assign33780_e56421_d_n8, assign33780_e56421_d_n9, assign33780_e56421_d_n10, assign33780_e56421_d_n11, assign33780_e56421_d_n13, assign33780_e56421_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33780_e56419: f64 = (1.0 / locals.var_t4);
        (assign33780_e56419, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33780_e56421;
        locals.var_t5_dn0 = assign33780_e56421_d_n0;
        locals.var_t5_dn2 = assign33780_e56421_d_n2;
        locals.var_t5_dn3 = assign33780_e56421_d_n3;
        locals.var_t5_dn4 = assign33780_e56421_d_n4;
        locals.var_t5_dn5 = assign33780_e56421_d_n5;
        locals.var_t5_dn6 = assign33780_e56421_d_n6;
        locals.var_t5_dn7 = assign33780_e56421_d_n7;
        locals.var_t5_dn8 = assign33780_e56421_d_n8;
        locals.var_t5_dn9 = assign33780_e56421_d_n9;
        locals.var_t5_dn10 = assign33780_e56421_d_n10;
        locals.var_t5_dn11 = assign33780_e56421_d_n11;
        locals.var_t5_dn13 = assign33780_e56421_d_n13;
        locals.var_t5_dn14 = assign33780_e56421_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign33790_e56444, assign33790_e56444_d_n0, assign33790_e56444_d_n2, assign33790_e56444_d_n3, assign33790_e56444_d_n4, assign33790_e56444_d_n5, assign33790_e56444_d_n6, assign33790_e56444_d_n7, assign33790_e56444_d_n8, assign33790_e56444_d_n9, assign33790_e56444_d_n10, assign33790_e56444_d_n11, assign33790_e56444_d_n13, assign33790_e56444_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33790_e56437: f64 = (locals.var_t5 * locals.var_t5);
        let assign33790_e56439: f64 = (assign33790_e56437 + 0.01);
        let assign33790_e56440: f64 = (assign33790_e56439).sqrt();
        let assign33790_e56441: f64 = (locals.var_t5 + assign33790_e56440);
        let assign33790_e56442: f64 = (0.5 * assign33790_e56441);
        (assign33790_e56442, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign33790_e56440)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign33790_e56440)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33790_e56444;
        locals.var_t6_dn0 = assign33790_e56444_d_n0;
        locals.var_t6_dn2 = assign33790_e56444_d_n2;
        locals.var_t6_dn3 = assign33790_e56444_d_n3;
        locals.var_t6_dn4 = assign33790_e56444_d_n4;
        locals.var_t6_dn5 = assign33790_e56444_d_n5;
        locals.var_t6_dn6 = assign33790_e56444_d_n6;
        locals.var_t6_dn7 = assign33790_e56444_d_n7;
        locals.var_t6_dn8 = assign33790_e56444_d_n8;
        locals.var_t6_dn9 = assign33790_e56444_d_n9;
        locals.var_t6_dn10 = assign33790_e56444_d_n10;
        locals.var_t6_dn11 = assign33790_e56444_d_n11;
        locals.var_t6_dn13 = assign33790_e56444_d_n13;
        locals.var_t6_dn14 = assign33790_e56444_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign33800_e56464, assign33800_e56464_d_n0, assign33800_e56464_d_n2, assign33800_e56464_d_n3, assign33800_e56464_d_n4, assign33800_e56464_d_n5, assign33800_e56464_d_n6, assign33800_e56464_d_n7, assign33800_e56464_d_n8, assign33800_e56464_d_n9, assign33800_e56464_d_n10, assign33800_e56464_d_n11, assign33800_e56464_d_n13, assign33800_e56464_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33800_e56459: f64 = (locals.var_rdsw_i * locals.var_t6);
        let assign33800_e56460: f64 = (p.p908 + assign33800_e56459);
        let assign33800_e56462: f64 = (assign33800_e56460 * locals.var_weffwrfactor);
        (assign33800_e56462, (((locals.var_rdsw_i_dn0 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn0)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn2 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn2)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn3 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn3)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn4 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn4)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn5 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn5)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn6 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn6)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn7 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn7)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn8 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn8)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn9 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn9)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn10 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn10)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn11 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn11)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn13 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn13)) * locals.var_weffwrfactor), (((locals.var_rdsw_i_dn14 * locals.var_t6) + (locals.var_rdsw_i * locals.var_t6_dn14)) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33800_e56464;
        locals.var_rdsi0_dn0 = assign33800_e56464_d_n0;
        locals.var_rdsi0_dn2 = assign33800_e56464_d_n2;
        locals.var_rdsi0_dn3 = assign33800_e56464_d_n3;
        locals.var_rdsi0_dn4 = assign33800_e56464_d_n4;
        locals.var_rdsi0_dn5 = assign33800_e56464_d_n5;
        locals.var_rdsi0_dn6 = assign33800_e56464_d_n6;
        locals.var_rdsi0_dn7 = assign33800_e56464_d_n7;
        locals.var_rdsi0_dn8 = assign33800_e56464_d_n8;
        locals.var_rdsi0_dn9 = assign33800_e56464_d_n9;
        locals.var_rdsi0_dn10 = assign33800_e56464_d_n10;
        locals.var_rdsi0_dn11 = assign33800_e56464_d_n11;
        locals.var_rdsi0_dn13 = assign33800_e56464_d_n13;
        locals.var_rdsi0_dn14 = assign33800_e56464_d_n14;
        locals.var_rdsi0_rv = 0.0;

        let (assign33810_e56484, assign33810_e56484_d_n0, assign33810_e56484_d_n2, assign33810_e56484_d_n3, assign33810_e56484_d_n4, assign33810_e56484_d_n5, assign33810_e56484_d_n6, assign33810_e56484_d_n7, assign33810_e56484_d_n8, assign33810_e56484_d_n9, assign33810_e56484_d_n10, assign33810_e56484_d_n11, assign33810_e56484_d_n13, assign33810_e56484_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33810_e56479: f64 = (locals.var_rsourcegeo + locals.var_rdraingeo);
        let assign33810_e56481: f64 = (assign33810_e56479 + locals.var_rdsi0);
        let assign33810_e56482: f64 = (locals.var_rdstemp * assign33810_e56481);
        (assign33810_e56482, ((locals.var_rdstemp_dn0 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn0 + locals.var_rdraingeo_dn0) + locals.var_rdsi0_dn0))), ((locals.var_rdstemp_dn2 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn2 + locals.var_rdraingeo_dn2) + locals.var_rdsi0_dn2))), ((locals.var_rdstemp_dn3 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn3 + locals.var_rdraingeo_dn3) + locals.var_rdsi0_dn3))), ((locals.var_rdstemp_dn4 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn4 + locals.var_rdraingeo_dn4) + locals.var_rdsi0_dn4))), ((locals.var_rdstemp_dn5 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn5 + locals.var_rdraingeo_dn5) + locals.var_rdsi0_dn5))), ((locals.var_rdstemp_dn6 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn6 + locals.var_rdraingeo_dn6) + locals.var_rdsi0_dn6))), ((locals.var_rdstemp_dn7 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn7 + locals.var_rdraingeo_dn7) + locals.var_rdsi0_dn7))), ((locals.var_rdstemp_dn8 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn8 + locals.var_rdraingeo_dn8) + locals.var_rdsi0_dn8))), ((locals.var_rdstemp_dn9 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn9 + locals.var_rdraingeo_dn9) + locals.var_rdsi0_dn9))), ((locals.var_rdstemp_dn10 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn10 + locals.var_rdraingeo_dn10) + locals.var_rdsi0_dn10))), ((locals.var_rdstemp_dn11 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn11 + locals.var_rdraingeo_dn11) + locals.var_rdsi0_dn11))), ((locals.var_rdstemp_dn13 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn13 + locals.var_rdraingeo_dn13) + locals.var_rdsi0_dn13))), ((locals.var_rdstemp_dn14 * assign33810_e56481) + (locals.var_rdstemp * ((locals.var_rsourcegeo_dn14 + locals.var_rdraingeo_dn14) + locals.var_rdsi0_dn14))),)
    } else {
        (locals.var_rdsi0, locals.var_rdsi0_dn0, locals.var_rdsi0_dn2, locals.var_rdsi0_dn3, locals.var_rdsi0_dn4, locals.var_rdsi0_dn5, locals.var_rdsi0_dn6, locals.var_rdsi0_dn7, locals.var_rdsi0_dn8, locals.var_rdsi0_dn9, locals.var_rdsi0_dn10, locals.var_rdsi0_dn11, locals.var_rdsi0_dn13, locals.var_rdsi0_dn14,)
    }
};
        locals.var_rdsi0 = assign33810_e56484;
        locals.var_rdsi0_dn0 = assign33810_e56484_d_n0;
        locals.var_rdsi0_dn2 = assign33810_e56484_d_n2;
        locals.var_rdsi0_dn3 = assign33810_e56484_d_n3;
        locals.var_rdsi0_dn4 = assign33810_e56484_d_n4;
        locals.var_rdsi0_dn5 = assign33810_e56484_d_n5;
        locals.var_rdsi0_dn6 = assign33810_e56484_d_n6;
        locals.var_rdsi0_dn7 = assign33810_e56484_d_n7;
        locals.var_rdsi0_dn8 = assign33810_e56484_d_n8;
        locals.var_rdsi0_dn9 = assign33810_e56484_d_n9;
        locals.var_rdsi0_dn10 = assign33810_e56484_d_n10;
        locals.var_rdsi0_dn11 = assign33810_e56484_d_n11;
        locals.var_rdsi0_dn13 = assign33810_e56484_d_n13;
        locals.var_rdsi0_dn14 = assign33810_e56484_d_n14;
        locals.var_rdsi0_rv = 0.0;

        let (assign33820_e56510, assign33820_e56510_d_n0, assign33820_e56510_d_n2, assign33820_e56510_d_n3, assign33820_e56510_d_n4, assign33820_e56510_d_n5, assign33820_e56510_d_n6, assign33820_e56510_d_n7, assign33820_e56510_d_n8, assign33820_e56510_d_n9, assign33820_e56510_d_n10, assign33820_e56510_d_n11, assign33820_e56510_d_n13, assign33820_e56510_d_n14,) = {
    if (((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) && ((locals.var_guard638 != 0.0) && (!((locals.var_guard636 != 0.0) || (locals.var_guard637 != 0.0))))) {
        let assign33820_e56499: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33820_e56501: f64 = (assign33820_e56499 * locals.var_ids0_ov_dqi0);
        let assign33820_e56504: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33820_e56505: f64 = (assign33820_e56501 / assign33820_e56504);
        let assign33820_e56507: f64 = (assign33820_e56505 * locals.var_rdsi0);
        let assign33820_e56508: f64 = (1.0 + assign33820_e56507);
        (assign33820_e56508, ((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn0)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn0 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn0)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn2)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn2 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn2)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn3)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn3 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn3)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn4)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn4 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn4)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn5)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn5 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn5)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn6)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn6 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn6)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn7)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn7 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn7)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn8)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn8 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn8)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn9)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn9 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn9)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn10)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn10 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn10)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn11)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn11 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn11)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn13)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn13 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn13)), ((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_ids0_ov_dqi0) + (assign33820_e56499 * locals.var_ids0_ov_dqi0_dn14)) * assign33820_e56504) - (assign33820_e56501 * (locals.var_dmob0_dn14 * locals.var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * locals.var_rdsi0) + (assign33820_e56505 * locals.var_rdsi0_dn14)),)
    } else {
        (locals.var_dr0, locals.var_dr0_dn0, locals.var_dr0_dn2, locals.var_dr0_dn3, locals.var_dr0_dn4, locals.var_dr0_dn5, locals.var_dr0_dn6, locals.var_dr0_dn7, locals.var_dr0_dn8, locals.var_dr0_dn9, locals.var_dr0_dn10, locals.var_dr0_dn11, locals.var_dr0_dn13, locals.var_dr0_dn14,)
    }
};
        locals.var_dr0 = assign33820_e56510;
        locals.var_dr0_dn0 = assign33820_e56510_d_n0;
        locals.var_dr0_dn2 = assign33820_e56510_d_n2;
        locals.var_dr0_dn3 = assign33820_e56510_d_n3;
        locals.var_dr0_dn4 = assign33820_e56510_d_n4;
        locals.var_dr0_dn5 = assign33820_e56510_d_n5;
        locals.var_dr0_dn6 = assign33820_e56510_d_n6;
        locals.var_dr0_dn7 = assign33820_e56510_d_n7;
        locals.var_dr0_dn8 = assign33820_e56510_d_n8;
        locals.var_dr0_dn9 = assign33820_e56510_d_n9;
        locals.var_dr0_dn10 = assign33820_e56510_d_n10;
        locals.var_dr0_dn11 = assign33820_e56510_d_n11;
        locals.var_dr0_dn13 = assign33820_e56510_d_n13;
        locals.var_dr0_dn14 = assign33820_e56510_d_n14;
        locals.var_dr0_rv = 0.0;

        let (assign33830_e56531, assign33830_e56531_d_n0, assign33830_e56531_d_n2, assign33830_e56531_d_n3, assign33830_e56531_d_n4, assign33830_e56531_d_n5, assign33830_e56531_d_n6, assign33830_e56531_d_n7, assign33830_e56531_d_n8, assign33830_e56531_d_n9, assign33830_e56531_d_n10, assign33830_e56531_d_n11, assign33830_e56531_d_n13, assign33830_e56531_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33830_e56517: f64 = (locals.var_nfintotal * locals.var_beta_v);
        let assign33830_e56519: f64 = (assign33830_e56517 * locals.var_qis);
        let assign33830_e56521: f64 = (assign33830_e56519 * locals.var_mnud0);
        let assign33830_e56523: f64 = (assign33830_e56521 * locals.var_mob0);
        let assign33830_e56526: f64 = (locals.var_dmob0 * locals.var_dvsat0);
        let assign33830_e56528: f64 = (assign33830_e56526 * locals.var_dr0);
        let assign33830_e56529: f64 = (assign33830_e56523 / assign33830_e56528);
        (assign33830_e56529, ((((((((((locals.var_nfintotal * locals.var_beta_v_dn0) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn0)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn0)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn0)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn0 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn0)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn2) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn2)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn2)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn2)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn2 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn2)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn3) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn3)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn3)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn3)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn3 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn3)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn4) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn4)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn4)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn4)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn4 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn4)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn5) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn5)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn5)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn5)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn5 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn5)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn6) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn6)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn6)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn6)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn6 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn6)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn7) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn7)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn7)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn7)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn7 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn7)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn8) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn8)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn8)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn8)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn8 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn8)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn9) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn9)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn9)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn9)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn9 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn9)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn10) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn10)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn10)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn10)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn10 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn10)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn11) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn11)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn11)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn11)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn11 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn11)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn13) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn13)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn13)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn13)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn13 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn13)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((locals.var_nfintotal * locals.var_beta_v_dn14) * locals.var_qis) + (assign33830_e56517 * locals.var_qis_dn14)) * locals.var_mnud0) + (assign33830_e56519 * locals.var_mnud0_dn14)) * locals.var_mob0) + (assign33830_e56521 * locals.var_mob0_dn14)) * assign33830_e56528) - (assign33830_e56523 * (((locals.var_dmob0_dn14 * locals.var_dvsat0) * locals.var_dr0) + (assign33830_e56526 * locals.var_dr0_dn14)))) / (assign33830_e56528 * assign33830_e56528)),)
    } else {
        (locals.var_noigd0, locals.var_noigd0_dn0, locals.var_noigd0_dn2, locals.var_noigd0_dn3, locals.var_noigd0_dn4, locals.var_noigd0_dn5, locals.var_noigd0_dn6, locals.var_noigd0_dn7, locals.var_noigd0_dn8, locals.var_noigd0_dn9, locals.var_noigd0_dn10, locals.var_noigd0_dn11, locals.var_noigd0_dn13, locals.var_noigd0_dn14,)
    }
};
        locals.var_noigd0 = assign33830_e56531;
        locals.var_noigd0_dn0 = assign33830_e56531_d_n0;
        locals.var_noigd0_dn2 = assign33830_e56531_d_n2;
        locals.var_noigd0_dn3 = assign33830_e56531_d_n3;
        locals.var_noigd0_dn4 = assign33830_e56531_d_n4;
        locals.var_noigd0_dn5 = assign33830_e56531_d_n5;
        locals.var_noigd0_dn6 = assign33830_e56531_d_n6;
        locals.var_noigd0_dn7 = assign33830_e56531_d_n7;
        locals.var_noigd0_dn8 = assign33830_e56531_d_n8;
        locals.var_noigd0_dn9 = assign33830_e56531_d_n9;
        locals.var_noigd0_dn10 = assign33830_e56531_d_n10;
        locals.var_noigd0_dn11 = assign33830_e56531_d_n11;
        locals.var_noigd0_dn13 = assign33830_e56531_d_n13;
        locals.var_noigd0_dn14 = assign33830_e56531_d_n14;
        locals.var_noigd0_rv = 0.0;

        let (assign33840_e56540, assign33840_e56540_d_n0, assign33840_e56540_d_n2, assign33840_e56540_d_n3, assign33840_e56540_d_n4, assign33840_e56540_d_n5, assign33840_e56540_d_n6, assign33840_e56540_d_n7, assign33840_e56540_d_n8, assign33840_e56540_d_n9, assign33840_e56540_d_n10, assign33840_e56540_d_n11, assign33840_e56540_d_n13, assign33840_e56540_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33840_e56538: f64 = (1.0 + locals.var_noieta);
        (assign33840_e56538, locals.var_noieta_dn0, locals.var_noieta_dn2, locals.var_noieta_dn3, locals.var_noieta_dn4, locals.var_noieta_dn5, locals.var_noieta_dn6, locals.var_noieta_dn7, locals.var_noieta_dn8, locals.var_noieta_dn9, locals.var_noieta_dn10, locals.var_noieta_dn11, locals.var_noieta_dn13, locals.var_noieta_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign33840_e56540;
        locals.var_t4_dn0 = assign33840_e56540_d_n0;
        locals.var_t4_dn2 = assign33840_e56540_d_n2;
        locals.var_t4_dn3 = assign33840_e56540_d_n3;
        locals.var_t4_dn4 = assign33840_e56540_d_n4;
        locals.var_t4_dn5 = assign33840_e56540_d_n5;
        locals.var_t4_dn6 = assign33840_e56540_d_n6;
        locals.var_t4_dn7 = assign33840_e56540_d_n7;
        locals.var_t4_dn8 = assign33840_e56540_d_n8;
        locals.var_t4_dn9 = assign33840_e56540_d_n9;
        locals.var_t4_dn10 = assign33840_e56540_d_n10;
        locals.var_t4_dn11 = assign33840_e56540_d_n11;
        locals.var_t4_dn13 = assign33840_e56540_d_n13;
        locals.var_t4_dn14 = assign33840_e56540_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign33850_e56549, assign33850_e56549_d_n0, assign33850_e56549_d_n2, assign33850_e56549_d_n3, assign33850_e56549_d_n4, assign33850_e56549_d_n5, assign33850_e56549_d_n6, assign33850_e56549_d_n7, assign33850_e56549_d_n8, assign33850_e56549_d_n9, assign33850_e56549_d_n10, assign33850_e56549_d_n11, assign33850_e56549_d_n13, assign33850_e56549_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33850_e56547: f64 = (1.0 - locals.var_noieta);
        (assign33850_e56547, (-locals.var_noieta_dn0), (-locals.var_noieta_dn2), (-locals.var_noieta_dn3), (-locals.var_noieta_dn4), (-locals.var_noieta_dn5), (-locals.var_noieta_dn6), (-locals.var_noieta_dn7), (-locals.var_noieta_dn8), (-locals.var_noieta_dn9), (-locals.var_noieta_dn10), (-locals.var_noieta_dn11), (-locals.var_noieta_dn13), (-locals.var_noieta_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign33850_e56549;
        locals.var_t5_dn0 = assign33850_e56549_d_n0;
        locals.var_t5_dn2 = assign33850_e56549_d_n2;
        locals.var_t5_dn3 = assign33850_e56549_d_n3;
        locals.var_t5_dn4 = assign33850_e56549_d_n4;
        locals.var_t5_dn5 = assign33850_e56549_d_n5;
        locals.var_t5_dn6 = assign33850_e56549_d_n6;
        locals.var_t5_dn7 = assign33850_e56549_d_n7;
        locals.var_t5_dn8 = assign33850_e56549_d_n8;
        locals.var_t5_dn9 = assign33850_e56549_d_n9;
        locals.var_t5_dn10 = assign33850_e56549_d_n10;
        locals.var_t5_dn11 = assign33850_e56549_d_n11;
        locals.var_t5_dn13 = assign33850_e56549_d_n13;
        locals.var_t5_dn14 = assign33850_e56549_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_132(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33860_e56562, assign33860_e56562_d_n0, assign33860_e56562_d_n2, assign33860_e56562_d_n3, assign33860_e56562_d_n4, assign33860_e56562_d_n5, assign33860_e56562_d_n6, assign33860_e56562_d_n7, assign33860_e56562_d_n8, assign33860_e56562_d_n9, assign33860_e56562_d_n10, assign33860_e56562_d_n11, assign33860_e56562_d_n13, assign33860_e56562_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33860_e56556: f64 = (2.0 * locals.var_noiwi);
        let assign33860_e56558: f64 = (assign33860_e56556 / locals.var_qis);
        let assign33860_e56560: f64 = (assign33860_e56558 * locals.var_nvtm);
        (assign33860_e56560, ((((((2.0 * locals.var_noiwi_dn0) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn0)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn0)), ((((((2.0 * locals.var_noiwi_dn2) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn2)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn2)), ((((((2.0 * locals.var_noiwi_dn3) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn3)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn3)), ((((((2.0 * locals.var_noiwi_dn4) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn4)), ((((((2.0 * locals.var_noiwi_dn5) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn5)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn5)), ((((((2.0 * locals.var_noiwi_dn6) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn6)), ((((((2.0 * locals.var_noiwi_dn7) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn7)), ((((((2.0 * locals.var_noiwi_dn8) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn8)), ((((((2.0 * locals.var_noiwi_dn9) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn9)), ((((((2.0 * locals.var_noiwi_dn10) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn10)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn10)), ((((((2.0 * locals.var_noiwi_dn11) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn11)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn11)), ((((((2.0 * locals.var_noiwi_dn13) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn13)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn13)), ((((((2.0 * locals.var_noiwi_dn14) * locals.var_qis) - (assign33860_e56556 * locals.var_qis_dn14)) / (locals.var_qis * locals.var_qis)) * locals.var_nvtm) + (assign33860_e56558 * locals.var_nvtm_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign33860_e56562;
        locals.var_t6_dn0 = assign33860_e56562_d_n0;
        locals.var_t6_dn2 = assign33860_e56562_d_n2;
        locals.var_t6_dn3 = assign33860_e56562_d_n3;
        locals.var_t6_dn4 = assign33860_e56562_d_n4;
        locals.var_t6_dn5 = assign33860_e56562_d_n5;
        locals.var_t6_dn6 = assign33860_e56562_d_n6;
        locals.var_t6_dn7 = assign33860_e56562_d_n7;
        locals.var_t6_dn8 = assign33860_e56562_d_n8;
        locals.var_t6_dn9 = assign33860_e56562_d_n9;
        locals.var_t6_dn10 = assign33860_e56562_d_n10;
        locals.var_t6_dn11 = assign33860_e56562_d_n11;
        locals.var_t6_dn13 = assign33860_e56562_d_n13;
        locals.var_t6_dn14 = assign33860_e56562_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign33870_e56571, assign33870_e56571_d_n0, assign33870_e56571_d_n2, assign33870_e56571_d_n3, assign33870_e56571_d_n4, assign33870_e56571_d_n5, assign33870_e56571_d_n6, assign33870_e56571_d_n7, assign33870_e56571_d_n8, assign33870_e56571_d_n9, assign33870_e56571_d_n10, assign33870_e56571_d_n11, assign33870_e56571_d_n13, assign33870_e56571_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33870_e56569: f64 = (locals.var_t4 + locals.var_t6);
        (assign33870_e56569, (locals.var_t4_dn0 + locals.var_t6_dn0), (locals.var_t4_dn2 + locals.var_t6_dn2), (locals.var_t4_dn3 + locals.var_t6_dn3), (locals.var_t4_dn4 + locals.var_t6_dn4), (locals.var_t4_dn5 + locals.var_t6_dn5), (locals.var_t4_dn6 + locals.var_t6_dn6), (locals.var_t4_dn7 + locals.var_t6_dn7), (locals.var_t4_dn8 + locals.var_t6_dn8), (locals.var_t4_dn9 + locals.var_t6_dn9), (locals.var_t4_dn10 + locals.var_t6_dn10), (locals.var_t4_dn11 + locals.var_t6_dn11), (locals.var_t4_dn13 + locals.var_t6_dn13), (locals.var_t4_dn14 + locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33870_e56571;
        locals.var_t7_dn0 = assign33870_e56571_d_n0;
        locals.var_t7_dn2 = assign33870_e56571_d_n2;
        locals.var_t7_dn3 = assign33870_e56571_d_n3;
        locals.var_t7_dn4 = assign33870_e56571_d_n4;
        locals.var_t7_dn5 = assign33870_e56571_d_n5;
        locals.var_t7_dn6 = assign33870_e56571_d_n6;
        locals.var_t7_dn7 = assign33870_e56571_d_n7;
        locals.var_t7_dn8 = assign33870_e56571_d_n8;
        locals.var_t7_dn9 = assign33870_e56571_d_n9;
        locals.var_t7_dn10 = assign33870_e56571_d_n10;
        locals.var_t7_dn11 = assign33870_e56571_d_n11;
        locals.var_t7_dn13 = assign33870_e56571_d_n13;
        locals.var_t7_dn14 = assign33870_e56571_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign33880_e56580, assign33880_e56580_d_n0, assign33880_e56580_d_n2, assign33880_e56580_d_n3, assign33880_e56580_d_n4, assign33880_e56580_d_n5, assign33880_e56580_d_n6, assign33880_e56580_d_n7, assign33880_e56580_d_n8, assign33880_e56580_d_n9, assign33880_e56580_d_n10, assign33880_e56580_d_n11, assign33880_e56580_d_n13, assign33880_e56580_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33880_e56578: f64 = (locals.var_t5 * locals.var_t5);
        (assign33880_e56578, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_2, locals.var_t5_2_dn0, locals.var_t5_2_dn2, locals.var_t5_2_dn3, locals.var_t5_2_dn4, locals.var_t5_2_dn5, locals.var_t5_2_dn6, locals.var_t5_2_dn7, locals.var_t5_2_dn8, locals.var_t5_2_dn9, locals.var_t5_2_dn10, locals.var_t5_2_dn11, locals.var_t5_2_dn13, locals.var_t5_2_dn14,)
    }
};
        locals.var_t5_2 = assign33880_e56580;
        locals.var_t5_2_dn0 = assign33880_e56580_d_n0;
        locals.var_t5_2_dn2 = assign33880_e56580_d_n2;
        locals.var_t5_2_dn3 = assign33880_e56580_d_n3;
        locals.var_t5_2_dn4 = assign33880_e56580_d_n4;
        locals.var_t5_2_dn5 = assign33880_e56580_d_n5;
        locals.var_t5_2_dn6 = assign33880_e56580_d_n6;
        locals.var_t5_2_dn7 = assign33880_e56580_d_n7;
        locals.var_t5_2_dn8 = assign33880_e56580_d_n8;
        locals.var_t5_2_dn9 = assign33880_e56580_d_n9;
        locals.var_t5_2_dn10 = assign33880_e56580_d_n10;
        locals.var_t5_2_dn11 = assign33880_e56580_d_n11;
        locals.var_t5_2_dn13 = assign33880_e56580_d_n13;
        locals.var_t5_2_dn14 = assign33880_e56580_d_n14;
        locals.var_t5_2_rv = 0.0;

        let (assign33890_e56589, assign33890_e56589_d_n0, assign33890_e56589_d_n2, assign33890_e56589_d_n3, assign33890_e56589_d_n4, assign33890_e56589_d_n5, assign33890_e56589_d_n6, assign33890_e56589_d_n7, assign33890_e56589_d_n8, assign33890_e56589_d_n9, assign33890_e56589_d_n10, assign33890_e56589_d_n11, assign33890_e56589_d_n13, assign33890_e56589_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33890_e56587: f64 = (locals.var_t5_2 * locals.var_t5);
        (assign33890_e56587, ((locals.var_t5_2_dn0 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn0)), ((locals.var_t5_2_dn2 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn2)), ((locals.var_t5_2_dn3 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn3)), ((locals.var_t5_2_dn4 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn4)), ((locals.var_t5_2_dn5 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn5)), ((locals.var_t5_2_dn6 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn6)), ((locals.var_t5_2_dn7 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn7)), ((locals.var_t5_2_dn8 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn8)), ((locals.var_t5_2_dn9 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn9)), ((locals.var_t5_2_dn10 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn10)), ((locals.var_t5_2_dn11 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn11)), ((locals.var_t5_2_dn13 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn13)), ((locals.var_t5_2_dn14 * locals.var_t5) + (locals.var_t5_2 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_3, locals.var_t5_3_dn0, locals.var_t5_3_dn2, locals.var_t5_3_dn3, locals.var_t5_3_dn4, locals.var_t5_3_dn5, locals.var_t5_3_dn6, locals.var_t5_3_dn7, locals.var_t5_3_dn8, locals.var_t5_3_dn9, locals.var_t5_3_dn10, locals.var_t5_3_dn11, locals.var_t5_3_dn13, locals.var_t5_3_dn14,)
    }
};
        locals.var_t5_3 = assign33890_e56589;
        locals.var_t5_3_dn0 = assign33890_e56589_d_n0;
        locals.var_t5_3_dn2 = assign33890_e56589_d_n2;
        locals.var_t5_3_dn3 = assign33890_e56589_d_n3;
        locals.var_t5_3_dn4 = assign33890_e56589_d_n4;
        locals.var_t5_3_dn5 = assign33890_e56589_d_n5;
        locals.var_t5_3_dn6 = assign33890_e56589_d_n6;
        locals.var_t5_3_dn7 = assign33890_e56589_d_n7;
        locals.var_t5_3_dn8 = assign33890_e56589_d_n8;
        locals.var_t5_3_dn9 = assign33890_e56589_d_n9;
        locals.var_t5_3_dn10 = assign33890_e56589_d_n10;
        locals.var_t5_3_dn11 = assign33890_e56589_d_n11;
        locals.var_t5_3_dn13 = assign33890_e56589_d_n13;
        locals.var_t5_3_dn14 = assign33890_e56589_d_n14;
        locals.var_t5_3_rv = 0.0;

        let (assign33900_e56598, assign33900_e56598_d_n0, assign33900_e56598_d_n2, assign33900_e56598_d_n3, assign33900_e56598_d_n4, assign33900_e56598_d_n5, assign33900_e56598_d_n6, assign33900_e56598_d_n7, assign33900_e56598_d_n8, assign33900_e56598_d_n9, assign33900_e56598_d_n10, assign33900_e56598_d_n11, assign33900_e56598_d_n13, assign33900_e56598_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33900_e56596: f64 = (locals.var_t5_3 * locals.var_t5);
        (assign33900_e56596, ((locals.var_t5_3_dn0 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn0)), ((locals.var_t5_3_dn2 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn2)), ((locals.var_t5_3_dn3 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn3)), ((locals.var_t5_3_dn4 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn4)), ((locals.var_t5_3_dn5 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn5)), ((locals.var_t5_3_dn6 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn6)), ((locals.var_t5_3_dn7 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn7)), ((locals.var_t5_3_dn8 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn8)), ((locals.var_t5_3_dn9 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn9)), ((locals.var_t5_3_dn10 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn10)), ((locals.var_t5_3_dn11 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn11)), ((locals.var_t5_3_dn13 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn13)), ((locals.var_t5_3_dn14 * locals.var_t5) + (locals.var_t5_3 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t5_4, locals.var_t5_4_dn0, locals.var_t5_4_dn2, locals.var_t5_4_dn3, locals.var_t5_4_dn4, locals.var_t5_4_dn5, locals.var_t5_4_dn6, locals.var_t5_4_dn7, locals.var_t5_4_dn8, locals.var_t5_4_dn9, locals.var_t5_4_dn10, locals.var_t5_4_dn11, locals.var_t5_4_dn13, locals.var_t5_4_dn14,)
    }
};
        locals.var_t5_4 = assign33900_e56598;
        locals.var_t5_4_dn0 = assign33900_e56598_d_n0;
        locals.var_t5_4_dn2 = assign33900_e56598_d_n2;
        locals.var_t5_4_dn3 = assign33900_e56598_d_n3;
        locals.var_t5_4_dn4 = assign33900_e56598_d_n4;
        locals.var_t5_4_dn5 = assign33900_e56598_d_n5;
        locals.var_t5_4_dn6 = assign33900_e56598_d_n6;
        locals.var_t5_4_dn7 = assign33900_e56598_d_n7;
        locals.var_t5_4_dn8 = assign33900_e56598_d_n8;
        locals.var_t5_4_dn9 = assign33900_e56598_d_n9;
        locals.var_t5_4_dn10 = assign33900_e56598_d_n10;
        locals.var_t5_4_dn11 = assign33900_e56598_d_n11;
        locals.var_t5_4_dn13 = assign33900_e56598_d_n13;
        locals.var_t5_4_dn14 = assign33900_e56598_d_n14;
        locals.var_t5_4_rv = 0.0;

        let (assign33910_e56607, assign33910_e56607_d_n0, assign33910_e56607_d_n2, assign33910_e56607_d_n3, assign33910_e56607_d_n4, assign33910_e56607_d_n5, assign33910_e56607_d_n6, assign33910_e56607_d_n7, assign33910_e56607_d_n8, assign33910_e56607_d_n9, assign33910_e56607_d_n10, assign33910_e56607_d_n11, assign33910_e56607_d_n13, assign33910_e56607_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33910_e56605: f64 = (locals.var_t7 * locals.var_t7);
        (assign33910_e56605, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_2, locals.var_t7_2_dn0, locals.var_t7_2_dn2, locals.var_t7_2_dn3, locals.var_t7_2_dn4, locals.var_t7_2_dn5, locals.var_t7_2_dn6, locals.var_t7_2_dn7, locals.var_t7_2_dn8, locals.var_t7_2_dn9, locals.var_t7_2_dn10, locals.var_t7_2_dn11, locals.var_t7_2_dn13, locals.var_t7_2_dn14,)
    }
};
        locals.var_t7_2 = assign33910_e56607;
        locals.var_t7_2_dn0 = assign33910_e56607_d_n0;
        locals.var_t7_2_dn2 = assign33910_e56607_d_n2;
        locals.var_t7_2_dn3 = assign33910_e56607_d_n3;
        locals.var_t7_2_dn4 = assign33910_e56607_d_n4;
        locals.var_t7_2_dn5 = assign33910_e56607_d_n5;
        locals.var_t7_2_dn6 = assign33910_e56607_d_n6;
        locals.var_t7_2_dn7 = assign33910_e56607_d_n7;
        locals.var_t7_2_dn8 = assign33910_e56607_d_n8;
        locals.var_t7_2_dn9 = assign33910_e56607_d_n9;
        locals.var_t7_2_dn10 = assign33910_e56607_d_n10;
        locals.var_t7_2_dn11 = assign33910_e56607_d_n11;
        locals.var_t7_2_dn13 = assign33910_e56607_d_n13;
        locals.var_t7_2_dn14 = assign33910_e56607_d_n14;
        locals.var_t7_2_rv = 0.0;

        let (assign33920_e56616, assign33920_e56616_d_n0, assign33920_e56616_d_n2, assign33920_e56616_d_n3, assign33920_e56616_d_n4, assign33920_e56616_d_n5, assign33920_e56616_d_n6, assign33920_e56616_d_n7, assign33920_e56616_d_n8, assign33920_e56616_d_n9, assign33920_e56616_d_n10, assign33920_e56616_d_n11, assign33920_e56616_d_n13, assign33920_e56616_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33920_e56614: f64 = (locals.var_t7_2 * locals.var_t7);
        (assign33920_e56614, ((locals.var_t7_2_dn0 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn0)), ((locals.var_t7_2_dn2 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn2)), ((locals.var_t7_2_dn3 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn3)), ((locals.var_t7_2_dn4 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn4)), ((locals.var_t7_2_dn5 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn5)), ((locals.var_t7_2_dn6 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn6)), ((locals.var_t7_2_dn7 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn7)), ((locals.var_t7_2_dn8 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn8)), ((locals.var_t7_2_dn9 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn9)), ((locals.var_t7_2_dn10 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn10)), ((locals.var_t7_2_dn11 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn11)), ((locals.var_t7_2_dn13 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn13)), ((locals.var_t7_2_dn14 * locals.var_t7) + (locals.var_t7_2 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_3, locals.var_t7_3_dn0, locals.var_t7_3_dn2, locals.var_t7_3_dn3, locals.var_t7_3_dn4, locals.var_t7_3_dn5, locals.var_t7_3_dn6, locals.var_t7_3_dn7, locals.var_t7_3_dn8, locals.var_t7_3_dn9, locals.var_t7_3_dn10, locals.var_t7_3_dn11, locals.var_t7_3_dn13, locals.var_t7_3_dn14,)
    }
};
        locals.var_t7_3 = assign33920_e56616;
        locals.var_t7_3_dn0 = assign33920_e56616_d_n0;
        locals.var_t7_3_dn2 = assign33920_e56616_d_n2;
        locals.var_t7_3_dn3 = assign33920_e56616_d_n3;
        locals.var_t7_3_dn4 = assign33920_e56616_d_n4;
        locals.var_t7_3_dn5 = assign33920_e56616_d_n5;
        locals.var_t7_3_dn6 = assign33920_e56616_d_n6;
        locals.var_t7_3_dn7 = assign33920_e56616_d_n7;
        locals.var_t7_3_dn8 = assign33920_e56616_d_n8;
        locals.var_t7_3_dn9 = assign33920_e56616_d_n9;
        locals.var_t7_3_dn10 = assign33920_e56616_d_n10;
        locals.var_t7_3_dn11 = assign33920_e56616_d_n11;
        locals.var_t7_3_dn13 = assign33920_e56616_d_n13;
        locals.var_t7_3_dn14 = assign33920_e56616_d_n14;
        locals.var_t7_3_rv = 0.0;

        let (assign33930_e56625, assign33930_e56625_d_n0, assign33930_e56625_d_n2, assign33930_e56625_d_n3, assign33930_e56625_d_n4, assign33930_e56625_d_n5, assign33930_e56625_d_n6, assign33930_e56625_d_n7, assign33930_e56625_d_n8, assign33930_e56625_d_n9, assign33930_e56625_d_n10, assign33930_e56625_d_n11, assign33930_e56625_d_n13, assign33930_e56625_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33930_e56623: f64 = (locals.var_t7_3 * locals.var_t7);
        (assign33930_e56623, ((locals.var_t7_3_dn0 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn0)), ((locals.var_t7_3_dn2 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn2)), ((locals.var_t7_3_dn3 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn3)), ((locals.var_t7_3_dn4 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn4)), ((locals.var_t7_3_dn5 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn5)), ((locals.var_t7_3_dn6 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn6)), ((locals.var_t7_3_dn7 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn7)), ((locals.var_t7_3_dn8 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn8)), ((locals.var_t7_3_dn9 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn9)), ((locals.var_t7_3_dn10 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn10)), ((locals.var_t7_3_dn11 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn11)), ((locals.var_t7_3_dn13 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn13)), ((locals.var_t7_3_dn14 * locals.var_t7) + (locals.var_t7_3 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_4, locals.var_t7_4_dn0, locals.var_t7_4_dn2, locals.var_t7_4_dn3, locals.var_t7_4_dn4, locals.var_t7_4_dn5, locals.var_t7_4_dn6, locals.var_t7_4_dn7, locals.var_t7_4_dn8, locals.var_t7_4_dn9, locals.var_t7_4_dn10, locals.var_t7_4_dn11, locals.var_t7_4_dn13, locals.var_t7_4_dn14,)
    }
};
        locals.var_t7_4 = assign33930_e56625;
        locals.var_t7_4_dn0 = assign33930_e56625_d_n0;
        locals.var_t7_4_dn2 = assign33930_e56625_d_n2;
        locals.var_t7_4_dn3 = assign33930_e56625_d_n3;
        locals.var_t7_4_dn4 = assign33930_e56625_d_n4;
        locals.var_t7_4_dn5 = assign33930_e56625_d_n5;
        locals.var_t7_4_dn6 = assign33930_e56625_d_n6;
        locals.var_t7_4_dn7 = assign33930_e56625_d_n7;
        locals.var_t7_4_dn8 = assign33930_e56625_d_n8;
        locals.var_t7_4_dn9 = assign33930_e56625_d_n9;
        locals.var_t7_4_dn10 = assign33930_e56625_d_n10;
        locals.var_t7_4_dn11 = assign33930_e56625_d_n11;
        locals.var_t7_4_dn13 = assign33930_e56625_d_n13;
        locals.var_t7_4_dn14 = assign33930_e56625_d_n14;
        locals.var_t7_4_rv = 0.0;

        let (assign33940_e56634, assign33940_e56634_d_n0, assign33940_e56634_d_n2, assign33940_e56634_d_n3, assign33940_e56634_d_n4, assign33940_e56634_d_n5, assign33940_e56634_d_n6, assign33940_e56634_d_n7, assign33940_e56634_d_n8, assign33940_e56634_d_n9, assign33940_e56634_d_n10, assign33940_e56634_d_n11, assign33940_e56634_d_n13, assign33940_e56634_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33940_e56632: f64 = (locals.var_t7_4 * locals.var_t7);
        (assign33940_e56632, ((locals.var_t7_4_dn0 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn0)), ((locals.var_t7_4_dn2 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn2)), ((locals.var_t7_4_dn3 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn3)), ((locals.var_t7_4_dn4 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn4)), ((locals.var_t7_4_dn5 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn5)), ((locals.var_t7_4_dn6 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn6)), ((locals.var_t7_4_dn7 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn7)), ((locals.var_t7_4_dn8 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn8)), ((locals.var_t7_4_dn9 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn9)), ((locals.var_t7_4_dn10 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn10)), ((locals.var_t7_4_dn11 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn11)), ((locals.var_t7_4_dn13 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn13)), ((locals.var_t7_4_dn14 * locals.var_t7) + (locals.var_t7_4 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t7_5, locals.var_t7_5_dn0, locals.var_t7_5_dn2, locals.var_t7_5_dn3, locals.var_t7_5_dn4, locals.var_t7_5_dn5, locals.var_t7_5_dn6, locals.var_t7_5_dn7, locals.var_t7_5_dn8, locals.var_t7_5_dn9, locals.var_t7_5_dn10, locals.var_t7_5_dn11, locals.var_t7_5_dn13, locals.var_t7_5_dn14,)
    }
};
        locals.var_t7_5 = assign33940_e56634;
        locals.var_t7_5_dn0 = assign33940_e56634_d_n0;
        locals.var_t7_5_dn2 = assign33940_e56634_d_n2;
        locals.var_t7_5_dn3 = assign33940_e56634_d_n3;
        locals.var_t7_5_dn4 = assign33940_e56634_d_n4;
        locals.var_t7_5_dn5 = assign33940_e56634_d_n5;
        locals.var_t7_5_dn6 = assign33940_e56634_d_n6;
        locals.var_t7_5_dn7 = assign33940_e56634_d_n7;
        locals.var_t7_5_dn8 = assign33940_e56634_d_n8;
        locals.var_t7_5_dn9 = assign33940_e56634_d_n9;
        locals.var_t7_5_dn10 = assign33940_e56634_d_n10;
        locals.var_t7_5_dn11 = assign33940_e56634_d_n11;
        locals.var_t7_5_dn13 = assign33940_e56634_d_n13;
        locals.var_t7_5_dn14 = assign33940_e56634_d_n14;
        locals.var_t7_5_rv = 0.0;

        let (assign33950_e56643, assign33950_e56643_d_n0, assign33950_e56643_d_n2, assign33950_e56643_d_n3, assign33950_e56643_d_n4, assign33950_e56643_d_n5, assign33950_e56643_d_n6, assign33950_e56643_d_n7, assign33950_e56643_d_n8, assign33950_e56643_d_n9, assign33950_e56643_d_n10, assign33950_e56643_d_n11, assign33950_e56643_d_n13, assign33950_e56643_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33950_e56641: f64 = (0.5 * locals.var_t4);
        (assign33950_e56641, (0.5 * locals.var_t4_dn0), (0.5 * locals.var_t4_dn2), (0.5 * locals.var_t4_dn3), (0.5 * locals.var_t4_dn4), (0.5 * locals.var_t4_dn5), (0.5 * locals.var_t4_dn6), (0.5 * locals.var_t4_dn7), (0.5 * locals.var_t4_dn8), (0.5 * locals.var_t4_dn9), (0.5 * locals.var_t4_dn10), (0.5 * locals.var_t4_dn11), (0.5 * locals.var_t4_dn13), (0.5 * locals.var_t4_dn14),)
    } else {
        (locals.var_gamma1, locals.var_gamma1_dn0, locals.var_gamma1_dn2, locals.var_gamma1_dn3, locals.var_gamma1_dn4, locals.var_gamma1_dn5, locals.var_gamma1_dn6, locals.var_gamma1_dn7, locals.var_gamma1_dn8, locals.var_gamma1_dn9, locals.var_gamma1_dn10, locals.var_gamma1_dn11, locals.var_gamma1_dn13, locals.var_gamma1_dn14,)
    }
};
        locals.var_gamma1 = assign33950_e56643;
        locals.var_gamma1_dn0 = assign33950_e56643_d_n0;
        locals.var_gamma1_dn2 = assign33950_e56643_d_n2;
        locals.var_gamma1_dn3 = assign33950_e56643_d_n3;
        locals.var_gamma1_dn4 = assign33950_e56643_d_n4;
        locals.var_gamma1_dn5 = assign33950_e56643_d_n5;
        locals.var_gamma1_dn6 = assign33950_e56643_d_n6;
        locals.var_gamma1_dn7 = assign33950_e56643_d_n7;
        locals.var_gamma1_dn8 = assign33950_e56643_d_n8;
        locals.var_gamma1_dn9 = assign33950_e56643_d_n9;
        locals.var_gamma1_dn10 = assign33950_e56643_d_n10;
        locals.var_gamma1_dn11 = assign33950_e56643_d_n11;
        locals.var_gamma1_dn13 = assign33950_e56643_d_n13;
        locals.var_gamma1_dn14 = assign33950_e56643_d_n14;
        locals.var_gamma1_rv = 0.0;

        let (assign33960_e56654, assign33960_e56654_d_n0, assign33960_e56654_d_n2, assign33960_e56654_d_n3, assign33960_e56654_d_n4, assign33960_e56654_d_n5, assign33960_e56654_d_n6, assign33960_e56654_d_n7, assign33960_e56654_d_n8, assign33960_e56654_d_n9, assign33960_e56654_d_n10, assign33960_e56654_d_n11, assign33960_e56654_d_n13, assign33960_e56654_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33960_e56651: f64 = (6.0 * locals.var_t7);
        let assign33960_e56652: f64 = (locals.var_t5_2 / assign33960_e56651);
        (assign33960_e56652, (((locals.var_t5_2_dn0 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn0))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn2 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn2))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn3 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn3))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn4 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn4))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn5 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn5))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn6 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn6))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn7 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn7))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn8 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn8))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn9 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn9))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn10 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn10))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn11 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn11))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn13 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn13))) / (assign33960_e56651 * assign33960_e56651)), (((locals.var_t5_2_dn14 * assign33960_e56651) - (locals.var_t5_2 * (6.0 * locals.var_t7_dn14))) / (assign33960_e56651 * assign33960_e56651)),)
    } else {
        (locals.var_gamma2, locals.var_gamma2_dn0, locals.var_gamma2_dn2, locals.var_gamma2_dn3, locals.var_gamma2_dn4, locals.var_gamma2_dn5, locals.var_gamma2_dn6, locals.var_gamma2_dn7, locals.var_gamma2_dn8, locals.var_gamma2_dn9, locals.var_gamma2_dn10, locals.var_gamma2_dn11, locals.var_gamma2_dn13, locals.var_gamma2_dn14,)
    }
};
        locals.var_gamma2 = assign33960_e56654;
        locals.var_gamma2_dn0 = assign33960_e56654_d_n0;
        locals.var_gamma2_dn2 = assign33960_e56654_d_n2;
        locals.var_gamma2_dn3 = assign33960_e56654_d_n3;
        locals.var_gamma2_dn4 = assign33960_e56654_d_n4;
        locals.var_gamma2_dn5 = assign33960_e56654_d_n5;
        locals.var_gamma2_dn6 = assign33960_e56654_d_n6;
        locals.var_gamma2_dn7 = assign33960_e56654_d_n7;
        locals.var_gamma2_dn8 = assign33960_e56654_d_n8;
        locals.var_gamma2_dn9 = assign33960_e56654_d_n9;
        locals.var_gamma2_dn10 = assign33960_e56654_d_n10;
        locals.var_gamma2_dn11 = assign33960_e56654_d_n11;
        locals.var_gamma2_dn13 = assign33960_e56654_d_n13;
        locals.var_gamma2_dn14 = assign33960_e56654_d_n14;
        locals.var_gamma2_rv = 0.0;

        let (assign33970_e56667, assign33970_e56667_d_n0, assign33970_e56667_d_n2, assign33970_e56667_d_n3, assign33970_e56667_d_n4, assign33970_e56667_d_n5, assign33970_e56667_d_n6, assign33970_e56667_d_n7, assign33970_e56667_d_n8, assign33970_e56667_d_n9, assign33970_e56667_d_n10, assign33970_e56667_d_n11, assign33970_e56667_d_n13, assign33970_e56667_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33970_e56661: f64 = (locals.var_moc / locals.var_dvsat);
        let assign33970_e56664: f64 = (locals.var_gamma1 + locals.var_gamma2);
        let assign33970_e56665: f64 = (assign33970_e56661 * assign33970_e56664);
        (assign33970_e56665, (((((locals.var_moc_dn0 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn0)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn0 + locals.var_gamma2_dn0))), (((((locals.var_moc_dn2 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn2)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn2 + locals.var_gamma2_dn2))), (((((locals.var_moc_dn3 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn3 + locals.var_gamma2_dn3))), (((((locals.var_moc_dn4 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn4 + locals.var_gamma2_dn4))), (((((locals.var_moc_dn5 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn5 + locals.var_gamma2_dn5))), (((((locals.var_moc_dn6 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn6 + locals.var_gamma2_dn6))), (((((locals.var_moc_dn7 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn7 + locals.var_gamma2_dn7))), (((((locals.var_moc_dn8 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn8 + locals.var_gamma2_dn8))), (((((locals.var_moc_dn9 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn9 + locals.var_gamma2_dn9))), (((((locals.var_moc_dn10 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn10 + locals.var_gamma2_dn10))), (((((locals.var_moc_dn11 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn11 + locals.var_gamma2_dn11))), (((((locals.var_moc_dn13 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn13)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn13 + locals.var_gamma2_dn13))), (((((locals.var_moc_dn14 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn14)) / (locals.var_dvsat * locals.var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (locals.var_gamma1_dn14 + locals.var_gamma2_dn14))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn3, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn13, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign33970_e56667;
        locals.var_gamma_dn0 = assign33970_e56667_d_n0;
        locals.var_gamma_dn2 = assign33970_e56667_d_n2;
        locals.var_gamma_dn3 = assign33970_e56667_d_n3;
        locals.var_gamma_dn4 = assign33970_e56667_d_n4;
        locals.var_gamma_dn5 = assign33970_e56667_d_n5;
        locals.var_gamma_dn6 = assign33970_e56667_d_n6;
        locals.var_gamma_dn7 = assign33970_e56667_d_n7;
        locals.var_gamma_dn8 = assign33970_e56667_d_n8;
        locals.var_gamma_dn9 = assign33970_e56667_d_n9;
        locals.var_gamma_dn10 = assign33970_e56667_d_n10;
        locals.var_gamma_dn11 = assign33970_e56667_d_n11;
        locals.var_gamma_dn13 = assign33970_e56667_d_n13;
        locals.var_gamma_dn14 = assign33970_e56667_d_n14;
        locals.var_gamma_rv = 0.0;

        let (assign33980_e56676, assign33980_e56676_d_n0, assign33980_e56676_d_n2, assign33980_e56676_d_n3, assign33980_e56676_d_n4, assign33980_e56676_d_n5, assign33980_e56676_d_n6, assign33980_e56676_d_n7, assign33980_e56676_d_n8, assign33980_e56676_d_n9, assign33980_e56676_d_n10, assign33980_e56676_d_n11, assign33980_e56676_d_n13, assign33980_e56676_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33980_e56674: f64 = (locals.var_t4 / locals.var_t7_2);
        (assign33980_e56674, (((locals.var_t4_dn0 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn0)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn2 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn2)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn3 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn3)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn4 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn4)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn5 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn5)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn6 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn6)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn7 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn7)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn8 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn8)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn9 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn9)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn10 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn10)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn11 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn11)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn13 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn13)) / (locals.var_t7_2 * locals.var_t7_2)), (((locals.var_t4_dn14 * locals.var_t7_2) - (locals.var_t4 * locals.var_t7_2_dn14)) / (locals.var_t7_2 * locals.var_t7_2)),)
    } else {
        (locals.var_delta1, locals.var_delta1_dn0, locals.var_delta1_dn2, locals.var_delta1_dn3, locals.var_delta1_dn4, locals.var_delta1_dn5, locals.var_delta1_dn6, locals.var_delta1_dn7, locals.var_delta1_dn8, locals.var_delta1_dn9, locals.var_delta1_dn10, locals.var_delta1_dn11, locals.var_delta1_dn13, locals.var_delta1_dn14,)
    }
};
        locals.var_delta1 = assign33980_e56676;
        locals.var_delta1_dn0 = assign33980_e56676_d_n0;
        locals.var_delta1_dn2 = assign33980_e56676_d_n2;
        locals.var_delta1_dn3 = assign33980_e56676_d_n3;
        locals.var_delta1_dn4 = assign33980_e56676_d_n4;
        locals.var_delta1_dn5 = assign33980_e56676_d_n5;
        locals.var_delta1_dn6 = assign33980_e56676_d_n6;
        locals.var_delta1_dn7 = assign33980_e56676_d_n7;
        locals.var_delta1_dn8 = assign33980_e56676_d_n8;
        locals.var_delta1_dn9 = assign33980_e56676_d_n9;
        locals.var_delta1_dn10 = assign33980_e56676_d_n10;
        locals.var_delta1_dn11 = assign33980_e56676_d_n11;
        locals.var_delta1_dn13 = assign33980_e56676_d_n13;
        locals.var_delta1_dn14 = assign33980_e56676_d_n14;
        locals.var_delta1_rv = 0.0;

        let (assign33990_e56693, assign33990_e56693_d_n0, assign33990_e56693_d_n2, assign33990_e56693_d_n3, assign33990_e56693_d_n4, assign33990_e56693_d_n5, assign33990_e56693_d_n6, assign33990_e56693_d_n7, assign33990_e56693_d_n8, assign33990_e56693_d_n9, assign33990_e56693_d_n10, assign33990_e56693_d_n11, assign33990_e56693_d_n13, assign33990_e56693_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign33990_e56683: f64 = (6.0 * locals.var_t4);
        let assign33990_e56685: f64 = (assign33990_e56683 + locals.var_t6);
        let assign33990_e56687: f64 = (assign33990_e56685 * locals.var_t5_2);
        let assign33990_e56690: f64 = (15.0 * locals.var_t7_4);
        let assign33990_e56691: f64 = (assign33990_e56687 / assign33990_e56690);
        (assign33990_e56691, (((((((6.0 * locals.var_t4_dn0) + locals.var_t6_dn0) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn0)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn0))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn2) + locals.var_t6_dn2) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn2)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn2))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn3) + locals.var_t6_dn3) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn3)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn3))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn4) + locals.var_t6_dn4) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn4)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn4))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn5) + locals.var_t6_dn5) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn5)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn5))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn6) + locals.var_t6_dn6) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn6)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn6))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn7) + locals.var_t6_dn7) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn7)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn7))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn8) + locals.var_t6_dn8) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn8)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn8))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn9) + locals.var_t6_dn9) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn9)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn9))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn10) + locals.var_t6_dn10) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn10)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn10))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn11) + locals.var_t6_dn11) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn11)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn11))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn13) + locals.var_t6_dn13) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn13)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn13))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * locals.var_t4_dn14) + locals.var_t6_dn14) * locals.var_t5_2) + (assign33990_e56685 * locals.var_t5_2_dn14)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * locals.var_t7_4_dn14))) / (assign33990_e56690 * assign33990_e56690)),)
    } else {
        (locals.var_delta2, locals.var_delta2_dn0, locals.var_delta2_dn2, locals.var_delta2_dn3, locals.var_delta2_dn4, locals.var_delta2_dn5, locals.var_delta2_dn6, locals.var_delta2_dn7, locals.var_delta2_dn8, locals.var_delta2_dn9, locals.var_delta2_dn10, locals.var_delta2_dn11, locals.var_delta2_dn13, locals.var_delta2_dn14,)
    }
};
        locals.var_delta2 = assign33990_e56693;
        locals.var_delta2_dn0 = assign33990_e56693_d_n0;
        locals.var_delta2_dn2 = assign33990_e56693_d_n2;
        locals.var_delta2_dn3 = assign33990_e56693_d_n3;
        locals.var_delta2_dn4 = assign33990_e56693_d_n4;
        locals.var_delta2_dn5 = assign33990_e56693_d_n5;
        locals.var_delta2_dn6 = assign33990_e56693_d_n6;
        locals.var_delta2_dn7 = assign33990_e56693_d_n7;
        locals.var_delta2_dn8 = assign33990_e56693_d_n8;
        locals.var_delta2_dn9 = assign33990_e56693_d_n9;
        locals.var_delta2_dn10 = assign33990_e56693_d_n10;
        locals.var_delta2_dn11 = assign33990_e56693_d_n11;
        locals.var_delta2_dn13 = assign33990_e56693_d_n13;
        locals.var_delta2_dn14 = assign33990_e56693_d_n14;
        locals.var_delta2_rv = 0.0;

        let (assign34000_e56704, assign34000_e56704_d_n0, assign34000_e56704_d_n2, assign34000_e56704_d_n3, assign34000_e56704_d_n4, assign34000_e56704_d_n5, assign34000_e56704_d_n6, assign34000_e56704_d_n7, assign34000_e56704_d_n8, assign34000_e56704_d_n9, assign34000_e56704_d_n10, assign34000_e56704_d_n11, assign34000_e56704_d_n13, assign34000_e56704_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34000_e56701: f64 = (9.0 * locals.var_t7_5);
        let assign34000_e56702: f64 = (locals.var_t5_4 / assign34000_e56701);
        (assign34000_e56702, (((locals.var_t5_4_dn0 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn0))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn2 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn2))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn3 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn3))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn4 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn4))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn5 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn5))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn6 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn6))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn7 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn7))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn8 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn8))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn9 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn9))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn10 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn10))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn11 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn11))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn13 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn13))) / (assign34000_e56701 * assign34000_e56701)), (((locals.var_t5_4_dn14 * assign34000_e56701) - (locals.var_t5_4 * (9.0 * locals.var_t7_5_dn14))) / (assign34000_e56701 * assign34000_e56701)),)
    } else {
        (locals.var_delta3, locals.var_delta3_dn0, locals.var_delta3_dn2, locals.var_delta3_dn3, locals.var_delta3_dn4, locals.var_delta3_dn5, locals.var_delta3_dn6, locals.var_delta3_dn7, locals.var_delta3_dn8, locals.var_delta3_dn9, locals.var_delta3_dn10, locals.var_delta3_dn11, locals.var_delta3_dn13, locals.var_delta3_dn14,)
    }
};
        locals.var_delta3 = assign34000_e56704;
        locals.var_delta3_dn0 = assign34000_e56704_d_n0;
        locals.var_delta3_dn2 = assign34000_e56704_d_n2;
        locals.var_delta3_dn3 = assign34000_e56704_d_n3;
        locals.var_delta3_dn4 = assign34000_e56704_d_n4;
        locals.var_delta3_dn5 = assign34000_e56704_d_n5;
        locals.var_delta3_dn6 = assign34000_e56704_d_n6;
        locals.var_delta3_dn7 = assign34000_e56704_d_n7;
        locals.var_delta3_dn8 = assign34000_e56704_d_n8;
        locals.var_delta3_dn9 = assign34000_e56704_d_n9;
        locals.var_delta3_dn10 = assign34000_e56704_d_n10;
        locals.var_delta3_dn11 = assign34000_e56704_d_n11;
        locals.var_delta3_dn13 = assign34000_e56704_d_n13;
        locals.var_delta3_dn14 = assign34000_e56704_d_n14;
        locals.var_delta3_rv = 0.0;

        let (assign34010_e56721, assign34010_e56721_d_n0, assign34010_e56721_d_n2, assign34010_e56721_d_n3, assign34010_e56721_d_n4, assign34010_e56721_d_n5, assign34010_e56721_d_n6, assign34010_e56721_d_n7, assign34010_e56721_d_n8, assign34010_e56721_d_n9, assign34010_e56721_d_n10, assign34010_e56721_d_n11, assign34010_e56721_d_n13, assign34010_e56721_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34010_e56711: f64 = (locals.var_moc / 6.0);
        let assign34010_e56713: f64 = (assign34010_e56711 * locals.var_dvsat3);
        let assign34010_e56716: f64 = (locals.var_delta1 - locals.var_delta2);
        let assign34010_e56718: f64 = (assign34010_e56716 + locals.var_delta3);
        let assign34010_e56719: f64 = (assign34010_e56713 * assign34010_e56718);
        (assign34010_e56719, (((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn0)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn0 - locals.var_delta2_dn0) + locals.var_delta3_dn0))), (((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn2)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn2 - locals.var_delta2_dn2) + locals.var_delta3_dn2))), (((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn3)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn3 - locals.var_delta2_dn3) + locals.var_delta3_dn3))), (((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn4)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn4 - locals.var_delta2_dn4) + locals.var_delta3_dn4))), (((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn5)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn5 - locals.var_delta2_dn5) + locals.var_delta3_dn5))), (((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn6)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn6 - locals.var_delta2_dn6) + locals.var_delta3_dn6))), (((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn7)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn7 - locals.var_delta2_dn7) + locals.var_delta3_dn7))), (((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn8)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn8 - locals.var_delta2_dn8) + locals.var_delta3_dn8))), (((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn9)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn9 - locals.var_delta2_dn9) + locals.var_delta3_dn9))), (((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn10)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn10 - locals.var_delta2_dn10) + locals.var_delta3_dn10))), (((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn11)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn11 - locals.var_delta2_dn11) + locals.var_delta3_dn11))), (((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn13)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn13 - locals.var_delta2_dn13) + locals.var_delta3_dn13))), (((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat3) + (assign34010_e56711 * locals.var_dvsat3_dn14)) * assign34010_e56718) + (assign34010_e56713 * ((locals.var_delta1_dn14 - locals.var_delta2_dn14) + locals.var_delta3_dn14))),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign34010_e56721;
        locals.var_delta_dn0 = assign34010_e56721_d_n0;
        locals.var_delta_dn2 = assign34010_e56721_d_n2;
        locals.var_delta_dn3 = assign34010_e56721_d_n3;
        locals.var_delta_dn4 = assign34010_e56721_d_n4;
        locals.var_delta_dn5 = assign34010_e56721_d_n5;
        locals.var_delta_dn6 = assign34010_e56721_d_n6;
        locals.var_delta_dn7 = assign34010_e56721_d_n7;
        locals.var_delta_dn8 = assign34010_e56721_d_n8;
        locals.var_delta_dn9 = assign34010_e56721_d_n9;
        locals.var_delta_dn10 = assign34010_e56721_d_n10;
        locals.var_delta_dn11 = assign34010_e56721_d_n11;
        locals.var_delta_dn13 = assign34010_e56721_d_n13;
        locals.var_delta_dn14 = assign34010_e56721_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign34100_e56816, assign34100_e56816_d_n0, assign34100_e56816_d_n2, assign34100_e56816_d_n3, assign34100_e56816_d_n4, assign34100_e56816_d_n5, assign34100_e56816_d_n6, assign34100_e56816_d_n7, assign34100_e56816_d_n8, assign34100_e56816_d_n9, assign34100_e56816_d_n10, assign34100_e56816_d_n11, assign34100_e56816_d_n13, assign34100_e56816_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34100_e56805: f64 = (locals.var_noilowid * locals.var_noilowid);
        let assign34100_e56808: f64 = (p.p1716 + locals.var_qia);
        let assign34100_e56809: f64 = (assign34100_e56805 / assign34100_e56808);
        let assign34100_e56812: f64 = (locals.var_vdseff_1 / locals.var_vdsat);
        let assign34100_e56813: f64 = (assign34100_e56809 * assign34100_e56812);
        let assign34100_e56814: f64 = (1.0 + assign34100_e56813);
        (assign34100_e56814, (((((((locals.var_noilowid_dn0 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn0)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn0)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn0 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn0)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn2 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn2)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn2)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn2 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn2)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn3 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn3)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn3)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn3 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn3)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn4 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn4)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn4)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn4 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn4)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn5 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn5)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn5)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn5 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn6 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn6)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn6)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn6 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn7 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn7)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn7)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn7 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn7)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn8 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn8)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn8)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn8 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn8)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn9 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn9)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn9)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn9 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn9)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn10 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn10)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn10)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn10 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn10)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn11 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn11)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn11)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn11 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn11)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn13 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn13)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn13)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn13 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn13)) / (locals.var_vdsat * locals.var_vdsat)))), (((((((locals.var_noilowid_dn14 * locals.var_noilowid) + (locals.var_noilowid * locals.var_noilowid_dn14)) * assign34100_e56808) - (assign34100_e56805 * locals.var_qia_dn14)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((locals.var_vdseff_1_dn14 * locals.var_vdsat) - (locals.var_vdseff_1 * locals.var_vdsat_dn14)) / (locals.var_vdsat * locals.var_vdsat)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign34100_e56816;
        locals.var_t8_dn0 = assign34100_e56816_d_n0;
        locals.var_t8_dn2 = assign34100_e56816_d_n2;
        locals.var_t8_dn3 = assign34100_e56816_d_n3;
        locals.var_t8_dn4 = assign34100_e56816_d_n4;
        locals.var_t8_dn5 = assign34100_e56816_d_n5;
        locals.var_t8_dn6 = assign34100_e56816_d_n6;
        locals.var_t8_dn7 = assign34100_e56816_d_n7;
        locals.var_t8_dn8 = assign34100_e56816_d_n8;
        locals.var_t8_dn9 = assign34100_e56816_d_n9;
        locals.var_t8_dn10 = assign34100_e56816_d_n10;
        locals.var_t8_dn11 = assign34100_e56816_d_n11;
        locals.var_t8_dn13 = assign34100_e56816_d_n13;
        locals.var_t8_dn14 = assign34100_e56816_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign34110_e56833, assign34110_e56833_d_n0, assign34110_e56833_d_n2, assign34110_e56833_d_n3, assign34110_e56833_d_n4, assign34110_e56833_d_n5, assign34110_e56833_d_n6, assign34110_e56833_d_n7, assign34110_e56833_d_n8, assign34110_e56833_d_n9, assign34110_e56833_d_n10, assign34110_e56833_d_n11, assign34110_e56833_d_n13, assign34110_e56833_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34110_e56823: f64 = (locals.var_moc / locals.var_dvsat);
        let assign34110_e56826: f64 = (locals.var_t8 * locals.var_gamma1);
        let assign34110_e56829: f64 = (locals.var_t1 * locals.var_gamma2);
        let assign34110_e56830: f64 = (assign34110_e56826 + assign34110_e56829);
        let assign34110_e56831: f64 = (assign34110_e56823 * assign34110_e56830);
        (assign34110_e56831, (((((locals.var_moc_dn0 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn0)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn0 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn0)) + ((locals.var_t1_dn0 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn0))))), (((((locals.var_moc_dn2 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn2)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn2 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn2)) + ((locals.var_t1_dn2 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn2))))), (((((locals.var_moc_dn3 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn3 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn3)) + ((locals.var_t1_dn3 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn3))))), (((((locals.var_moc_dn4 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn4 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn4)) + ((locals.var_t1_dn4 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn4))))), (((((locals.var_moc_dn5 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn5 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn5)) + ((locals.var_t1_dn5 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn5))))), (((((locals.var_moc_dn6 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn6 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn6)) + ((locals.var_t1_dn6 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn6))))), (((((locals.var_moc_dn7 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn7 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn7)) + ((locals.var_t1_dn7 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn7))))), (((((locals.var_moc_dn8 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn8 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn8)) + ((locals.var_t1_dn8 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn8))))), (((((locals.var_moc_dn9 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn9 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn9)) + ((locals.var_t1_dn9 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn9))))), (((((locals.var_moc_dn10 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn10 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn10)) + ((locals.var_t1_dn10 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn10))))), (((((locals.var_moc_dn11 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn11 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn11)) + ((locals.var_t1_dn11 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn11))))), (((((locals.var_moc_dn13 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn13)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn13 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn13)) + ((locals.var_t1_dn13 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn13))))), (((((locals.var_moc_dn14 * locals.var_dvsat) - (locals.var_moc * locals.var_dvsat_dn14)) / (locals.var_dvsat * locals.var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((locals.var_t8_dn14 * locals.var_gamma1) + (locals.var_t8 * locals.var_gamma1_dn14)) + ((locals.var_t1_dn14 * locals.var_gamma2) + (locals.var_t1 * locals.var_gamma2_dn14))))),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn3, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn13, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign34110_e56833;
        locals.var_gamma_dn0 = assign34110_e56833_d_n0;
        locals.var_gamma_dn2 = assign34110_e56833_d_n2;
        locals.var_gamma_dn3 = assign34110_e56833_d_n3;
        locals.var_gamma_dn4 = assign34110_e56833_d_n4;
        locals.var_gamma_dn5 = assign34110_e56833_d_n5;
        locals.var_gamma_dn6 = assign34110_e56833_d_n6;
        locals.var_gamma_dn7 = assign34110_e56833_d_n7;
        locals.var_gamma_dn8 = assign34110_e56833_d_n8;
        locals.var_gamma_dn9 = assign34110_e56833_d_n9;
        locals.var_gamma_dn10 = assign34110_e56833_d_n10;
        locals.var_gamma_dn11 = assign34110_e56833_d_n11;
        locals.var_gamma_dn13 = assign34110_e56833_d_n13;
        locals.var_gamma_dn14 = assign34110_e56833_d_n14;
        locals.var_gamma_rv = 0.0;

        let (assign34130_e56867, assign34130_e56867_d_n0, assign34130_e56867_d_n2, assign34130_e56867_d_n3, assign34130_e56867_d_n4, assign34130_e56867_d_n5, assign34130_e56867_d_n6, assign34130_e56867_d_n7, assign34130_e56867_d_n8, assign34130_e56867_d_n9, assign34130_e56867_d_n10, assign34130_e56867_d_n11, assign34130_e56867_d_n13, assign34130_e56867_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34130_e56855: f64 = (locals.var_moc / 6.0);
        let assign34130_e56857: f64 = (assign34130_e56855 * locals.var_dvsat3);
        let assign34130_e56859: f64 = (assign34130_e56857 * locals.var_t2);
        let assign34130_e56862: f64 = (locals.var_delta1 - locals.var_delta2);
        let assign34130_e56864: f64 = (assign34130_e56862 + locals.var_delta3);
        let assign34130_e56865: f64 = (assign34130_e56859 * assign34130_e56864);
        (assign34130_e56865, (((((((locals.var_moc_dn0 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn0)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn0)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn0 - locals.var_delta2_dn0) + locals.var_delta3_dn0))), (((((((locals.var_moc_dn2 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn2)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn2)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn2 - locals.var_delta2_dn2) + locals.var_delta3_dn2))), (((((((locals.var_moc_dn3 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn3)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn3)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn3 - locals.var_delta2_dn3) + locals.var_delta3_dn3))), (((((((locals.var_moc_dn4 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn4)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn4)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn4 - locals.var_delta2_dn4) + locals.var_delta3_dn4))), (((((((locals.var_moc_dn5 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn5)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn5)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn5 - locals.var_delta2_dn5) + locals.var_delta3_dn5))), (((((((locals.var_moc_dn6 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn6)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn6)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn6 - locals.var_delta2_dn6) + locals.var_delta3_dn6))), (((((((locals.var_moc_dn7 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn7)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn7)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn7 - locals.var_delta2_dn7) + locals.var_delta3_dn7))), (((((((locals.var_moc_dn8 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn8)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn8)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn8 - locals.var_delta2_dn8) + locals.var_delta3_dn8))), (((((((locals.var_moc_dn9 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn9)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn9)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn9 - locals.var_delta2_dn9) + locals.var_delta3_dn9))), (((((((locals.var_moc_dn10 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn10)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn10)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn10 - locals.var_delta2_dn10) + locals.var_delta3_dn10))), (((((((locals.var_moc_dn11 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn11)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn11)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn11 - locals.var_delta2_dn11) + locals.var_delta3_dn11))), (((((((locals.var_moc_dn13 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn13)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn13)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn13 - locals.var_delta2_dn13) + locals.var_delta3_dn13))), (((((((locals.var_moc_dn14 / 6.0) * locals.var_dvsat3) + (assign34130_e56855 * locals.var_dvsat3_dn14)) * locals.var_t2) + (assign34130_e56857 * locals.var_t2_dn14)) * assign34130_e56864) + (assign34130_e56859 * ((locals.var_delta1_dn14 - locals.var_delta2_dn14) + locals.var_delta3_dn14))),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign34130_e56867;
        locals.var_delta_dn0 = assign34130_e56867_d_n0;
        locals.var_delta_dn2 = assign34130_e56867_d_n2;
        locals.var_delta_dn3 = assign34130_e56867_d_n3;
        locals.var_delta_dn4 = assign34130_e56867_d_n4;
        locals.var_delta_dn5 = assign34130_e56867_d_n5;
        locals.var_delta_dn6 = assign34130_e56867_d_n6;
        locals.var_delta_dn7 = assign34130_e56867_d_n7;
        locals.var_delta_dn8 = assign34130_e56867_d_n8;
        locals.var_delta_dn9 = assign34130_e56867_d_n9;
        locals.var_delta_dn10 = assign34130_e56867_d_n10;
        locals.var_delta_dn11 = assign34130_e56867_d_n11;
        locals.var_delta_dn13 = assign34130_e56867_d_n13;
        locals.var_delta_dn14 = assign34130_e56867_d_n14;
        locals.var_delta_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_133(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign34140_e56887, assign34140_e56887_d_n0, assign34140_e56887_d_n2, assign34140_e56887_d_n3, assign34140_e56887_d_n4, assign34140_e56887_d_n5, assign34140_e56887_d_n6, assign34140_e56887_d_n7, assign34140_e56887_d_n8, assign34140_e56887_d_n9, assign34140_e56887_d_n10, assign34140_e56887_d_n11, assign34140_e56887_d_n13, assign34140_e56887_d_n14,) = {
    if ((locals.var_guard633 != 0.0) && (locals.var_guard632 == 0.0)) {
        let assign34140_e56874: f64 = (locals.var_delta / locals.var_gamma);
        let assign34140_e56875: f64 = (assign34140_e56874).sqrt();
        let assign34140_e56877: f64 = (assign34140_e56875 * locals.var_nfintotal);
        let assign34140_e56879: f64 = (assign34140_e56877 * locals.var_coxeff);
        let assign34140_e56881: f64 = (assign34140_e56879 * locals.var_weffcv0);
        let assign34140_e56883: f64 = (assign34140_e56881 * locals.var_leffcv_1);
        let assign34140_e56885: f64 = (assign34140_e56883 / locals.var_noigd0);
        (assign34140_e56885, (((((((((((((locals.var_delta_dn0 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn0)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn0)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn0)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn0)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn2 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn2)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn2)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn2)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn2)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn3 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn3)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn3)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn3)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn3)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn4 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn4)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn4)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn4)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn4)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn5 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn5)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn5)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn5)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn5)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn6 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn6)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn6)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn6)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn6)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn7 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn7)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn7)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn7)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn7)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn8 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn8)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn8)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn8)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn8)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn9 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn9)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn9)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn9)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn9)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn10 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn10)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn10)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn10)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn10)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn11 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn11)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn11)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn11)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn11)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn13 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn13)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn13)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn13)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn13)) / (locals.var_noigd0 * locals.var_noigd0)), (((((((((((((locals.var_delta_dn14 * locals.var_gamma) - (locals.var_delta * locals.var_gamma_dn14)) / (locals.var_gamma * locals.var_gamma)) / (2.0 * assign34140_e56875)) * locals.var_nfintotal) * locals.var_coxeff) + (assign34140_e56877 * locals.var_coxeff_dn14)) * locals.var_weffcv0) * locals.var_leffcv_1) + (assign34140_e56881 * locals.var_leffcv_1_dn14)) * locals.var_noigd0) - (assign34140_e56883 * locals.var_noigd0_dn14)) / (locals.var_noigd0 * locals.var_noigd0)),)
    } else {
        (locals.var_sigrat, locals.var_sigrat_dn0, locals.var_sigrat_dn2, locals.var_sigrat_dn3, locals.var_sigrat_dn4, locals.var_sigrat_dn5, locals.var_sigrat_dn6, locals.var_sigrat_dn7, locals.var_sigrat_dn8, locals.var_sigrat_dn9, locals.var_sigrat_dn10, locals.var_sigrat_dn11, locals.var_sigrat_dn13, locals.var_sigrat_dn14,)
    }
};
        locals.var_sigrat = assign34140_e56887;
        locals.var_sigrat_dn0 = assign34140_e56887_d_n0;
        locals.var_sigrat_dn2 = assign34140_e56887_d_n2;
        locals.var_sigrat_dn3 = assign34140_e56887_d_n3;
        locals.var_sigrat_dn4 = assign34140_e56887_d_n4;
        locals.var_sigrat_dn5 = assign34140_e56887_d_n5;
        locals.var_sigrat_dn6 = assign34140_e56887_d_n6;
        locals.var_sigrat_dn7 = assign34140_e56887_d_n7;
        locals.var_sigrat_dn8 = assign34140_e56887_d_n8;
        locals.var_sigrat_dn9 = assign34140_e56887_d_n9;
        locals.var_sigrat_dn10 = assign34140_e56887_d_n10;
        locals.var_sigrat_dn11 = assign34140_e56887_d_n11;
        locals.var_sigrat_dn13 = assign34140_e56887_d_n13;
        locals.var_sigrat_dn14 = assign34140_e56887_d_n14;
        locals.var_sigrat_rv = 0.0;

        let assign34160_e56893: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign34160_e56893;
        locals.var_guard642_rv = 0.0;

        let assign34250_e56944: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign34250_e56944;
        locals.var_guard651_rv = 0.0;

        let assign34260_e56947: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign34260_e56947;
        locals.var_guard652_rv = 0.0;

        let assign34270_e56950: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign34270_e56950;
        locals.var_guard653_rv = 0.0;

        let assign34280_e56953: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign34280_e56953;
        locals.var_guard654_rv = 0.0;

        let assign34290_e56956: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign34290_e56956;
        locals.var_guard655_rv = 0.0;

        let assign34300_e56959: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign34300_e56959;
        locals.var_guard656_rv = 0.0;

        let assign34310_e56962: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign34310_e56962;
        locals.var_guard657_rv = 0.0;

        let assign34320_e56965: f64 = if p.p1910 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign34320_e56965;
        locals.var_guard658_rv = 0.0;

        let (assign34330_e57042, assign34330_e57042_d_n4,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34330_e56972: f64 = (p.p1912 * locals.var_deltemp);
        let assign34330_e56973: f64 = (1.0 + assign34330_e56972);
        let assign34330_e56975: f64 = (assign34330_e56973 - 1e-6);
        let assign34330_e56977: f64 = (-10000.0);
        let assign34330_e56979: f64 = (assign34330_e56977 * 0.001);
        let (assign34330_e57040, assign34330_e57040_d_n4,) = {
            if (!(assign34330_e56975 < assign34330_e56979)) {
                let assign34330_e56986: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e56987: f64 = (1.0 + assign34330_e56986);
                let assign34330_e56989: f64 = (assign34330_e56987 - 1e-6);
                let assign34330_e56993: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e56994: f64 = (1.0 + assign34330_e56993);
                let assign34330_e56996: f64 = (assign34330_e56994 - 1e-6);
                let assign34330_e57000: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e57001: f64 = (1.0 + assign34330_e57000);
                let assign34330_e57003: f64 = (assign34330_e57001 - 1e-6);
                let assign34330_e57004: f64 = (assign34330_e56996 * assign34330_e57003);
                let assign34330_e57007: f64 = (4.0 * 0.001);
                let assign34330_e57009: f64 = (assign34330_e57007 * 0.001);
                let assign34330_e57010: f64 = (assign34330_e57004 + assign34330_e57009);
                let assign34330_e57011: f64 = (assign34330_e57010).sqrt();
                let assign34330_e57012: f64 = (assign34330_e56989 + assign34330_e57011);
                let assign34330_e57013: f64 = (0.5 * assign34330_e57012);
                (assign34330_e57013, (0.5 * ((p.p1912 * locals.var_deltemp_dn4) + ((((p.p1912 * locals.var_deltemp_dn4) * assign34330_e57003) + (assign34330_e56996 * (p.p1912 * locals.var_deltemp_dn4))) / (2.0 * assign34330_e57011)))),)
            } else {
                let assign34330_e57017: f64 = (p.p1912 * locals.var_deltemp);
                let assign34330_e57018: f64 = (1.0 + assign34330_e57017);
                let assign34330_e57020: f64 = (assign34330_e57018 - 1e-6);
                let assign34330_e57022: f64 = (-10000.0);
                let assign34330_e57024: f64 = (assign34330_e57022 * 0.001);
                let (assign34330_e57039, assign34330_e57039_d_n4,) = {
                    if (assign34330_e57020 < assign34330_e57024) {
                        let assign34330_e57027: f64 = (-0.001);
                        let assign34330_e57029: f64 = (assign34330_e57027 * 0.001);
                        let assign34330_e57033: f64 = (p.p1912 * locals.var_deltemp);
                        let assign34330_e57034: f64 = (1.0 + assign34330_e57033);
                        let assign34330_e57036: f64 = (assign34330_e57034 - 1e-6);
                        let assign34330_e57037: f64 = (assign34330_e57029 / assign34330_e57036);
                        (assign34330_e57037, (-((assign34330_e57029 * (p.p1912 * locals.var_deltemp_dn4)) / (assign34330_e57036 * assign34330_e57036))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34330_e57039, assign34330_e57039_d_n4,)
            }
        };
        (assign34330_e57040, assign34330_e57040_d_n4,)
    } else {
        (locals.var_rdstempvs, locals.var_rdstempvs_dn4,)
    }
};
        locals.var_rdstempvs = assign34330_e57042;
        locals.var_rdstempvs_dn4 = assign34330_e57042_d_n4;
        locals.var_rdstempvs_rv = 0.0;

        let assign34340_e57045: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign34340_e57045;
        locals.var_guard659_rv = 0.0;

        let (assign34350_e57096, assign34350_e57096_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 != 0.0)) {
        let assign34350_e57053: f64 = (-p.p1904);
        let assign34350_e57056: f64 = (-p.p1913);
        let assign34350_e57058: f64 = (assign34350_e57056 * locals.var_deltemp);
        let assign34350_e57060: f64 = (-p.p1904);
        let assign34350_e57061: f64 = (assign34350_e57058 - assign34350_e57060);
        let assign34350_e57063: f64 = (assign34350_e57061 - 1e-6);
        let assign34350_e57065: f64 = (-p.p1913);
        let assign34350_e57067: f64 = (assign34350_e57065 * locals.var_deltemp);
        let assign34350_e57069: f64 = (-p.p1904);
        let assign34350_e57070: f64 = (assign34350_e57067 - assign34350_e57069);
        let assign34350_e57072: f64 = (assign34350_e57070 - 1e-6);
        let assign34350_e57074: f64 = (-p.p1913);
        let assign34350_e57076: f64 = (assign34350_e57074 * locals.var_deltemp);
        let assign34350_e57078: f64 = (-p.p1904);
        let assign34350_e57079: f64 = (assign34350_e57076 - assign34350_e57078);
        let assign34350_e57081: f64 = (assign34350_e57079 - 1e-6);
        let assign34350_e57082: f64 = (assign34350_e57072 * assign34350_e57081);
        let assign34350_e57085: f64 = (-p.p1904);
        let assign34350_e57086: f64 = (4.0 * assign34350_e57085);
        let assign34350_e57088: f64 = (assign34350_e57086 * 1e-6);
        let assign34350_e57089: f64 = (assign34350_e57082 - assign34350_e57088);
        let assign34350_e57090: f64 = (assign34350_e57089).sqrt();
        let assign34350_e57091: f64 = (assign34350_e57063 + assign34350_e57090);
        let assign34350_e57092: f64 = (0.5 * assign34350_e57091);
        let assign34350_e57093: f64 = (assign34350_e57053 + assign34350_e57092);
        let assign34350_e57094: f64 = (p.p1904 + assign34350_e57093);
        (assign34350_e57094, (0.5 * ((assign34350_e57056 * locals.var_deltemp_dn4) + ((((assign34350_e57065 * locals.var_deltemp_dn4) * assign34350_e57081) + (assign34350_e57072 * (assign34350_e57074 * locals.var_deltemp_dn4))) / (2.0 * assign34350_e57090)))),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34350_e57096;
        locals.var_vsatrsd_t_dn4 = assign34350_e57096_d_n4;
        locals.var_vsatrsd_t_rv = 0.0;

        let (assign34360_e57184, assign34360_e57184_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard659 == 0.0)) {
        let assign34360_e57106: f64 = (-p.p1913);
        let assign34360_e57108: f64 = (assign34360_e57106 * locals.var_deltemp);
        let assign34360_e57109: f64 = (1.0 + assign34360_e57108);
        let assign34360_e57111: f64 = (assign34360_e57109 - 1e-6);
        let assign34360_e57113: f64 = (-10000.0);
        let assign34360_e57115: f64 = (assign34360_e57113 * 0.001);
        let (assign34360_e57181, assign34360_e57181_d_n4,) = {
            if (!(assign34360_e57111 < assign34360_e57115)) {
                let assign34360_e57121: f64 = (-p.p1913);
                let assign34360_e57123: f64 = (assign34360_e57121 * locals.var_deltemp);
                let assign34360_e57124: f64 = (1.0 + assign34360_e57123);
                let assign34360_e57126: f64 = (assign34360_e57124 - 1e-6);
                let assign34360_e57129: f64 = (-p.p1913);
                let assign34360_e57131: f64 = (assign34360_e57129 * locals.var_deltemp);
                let assign34360_e57132: f64 = (1.0 + assign34360_e57131);
                let assign34360_e57134: f64 = (assign34360_e57132 - 1e-6);
                let assign34360_e57137: f64 = (-p.p1913);
                let assign34360_e57139: f64 = (assign34360_e57137 * locals.var_deltemp);
                let assign34360_e57140: f64 = (1.0 + assign34360_e57139);
                let assign34360_e57142: f64 = (assign34360_e57140 - 1e-6);
                let assign34360_e57143: f64 = (assign34360_e57134 * assign34360_e57142);
                let assign34360_e57146: f64 = (4.0 * 0.001);
                let assign34360_e57148: f64 = (assign34360_e57146 * 0.001);
                let assign34360_e57149: f64 = (assign34360_e57143 + assign34360_e57148);
                let assign34360_e57150: f64 = (assign34360_e57149).sqrt();
                let assign34360_e57151: f64 = (assign34360_e57126 + assign34360_e57150);
                let assign34360_e57152: f64 = (0.5 * assign34360_e57151);
                (assign34360_e57152, (0.5 * ((assign34360_e57121 * locals.var_deltemp_dn4) + ((((assign34360_e57129 * locals.var_deltemp_dn4) * assign34360_e57142) + (assign34360_e57134 * (assign34360_e57137 * locals.var_deltemp_dn4))) / (2.0 * assign34360_e57150)))),)
            } else {
                let assign34360_e57155: f64 = (-p.p1913);
                let assign34360_e57157: f64 = (assign34360_e57155 * locals.var_deltemp);
                let assign34360_e57158: f64 = (1.0 + assign34360_e57157);
                let assign34360_e57160: f64 = (assign34360_e57158 - 1e-6);
                let assign34360_e57162: f64 = (-10000.0);
                let assign34360_e57164: f64 = (assign34360_e57162 * 0.001);
                let (assign34360_e57180, assign34360_e57180_d_n4,) = {
                    if (assign34360_e57160 < assign34360_e57164) {
                        let assign34360_e57167: f64 = (-0.001);
                        let assign34360_e57169: f64 = (assign34360_e57167 * 0.001);
                        let assign34360_e57172: f64 = (-p.p1913);
                        let assign34360_e57174: f64 = (assign34360_e57172 * locals.var_deltemp);
                        let assign34360_e57175: f64 = (1.0 + assign34360_e57174);
                        let assign34360_e57177: f64 = (assign34360_e57175 - 1e-6);
                        let assign34360_e57178: f64 = (assign34360_e57169 / assign34360_e57177);
                        (assign34360_e57178, (-((assign34360_e57169 * (assign34360_e57172 * locals.var_deltemp_dn4)) / (assign34360_e57177 * assign34360_e57177))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34360_e57180, assign34360_e57180_d_n4,)
            }
        };
        let assign34360_e57182: f64 = (p.p1904 * assign34360_e57181);
        (assign34360_e57182, (p.p1904 * assign34360_e57181_d_n4),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34360_e57184;
        locals.var_vsatrsd_t_dn4 = assign34360_e57184_d_n4;
        locals.var_vsatrsd_t_rv = 0.0;

        let (assign34370_e57192, assign34370_e57192_d_n0, assign34370_e57192_d_n2, assign34370_e57192_d_n3, assign34370_e57192_d_n4, assign34370_e57192_d_n5, assign34370_e57192_d_n6, assign34370_e57192_d_n7, assign34370_e57192_d_n8, assign34370_e57192_d_n9, assign34370_e57192_d_n10, assign34370_e57192_d_n11, assign34370_e57192_d_n13, assign34370_e57192_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34370_e57190: f64 = (locals.var_qis - p.p1906);
        (assign34370_e57190, locals.var_qis_dn0, locals.var_qis_dn2, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11, locals.var_qis_dn13, locals.var_qis_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34370_e57192;
        locals.var_t0_dn0 = assign34370_e57192_d_n0;
        locals.var_t0_dn2 = assign34370_e57192_d_n2;
        locals.var_t0_dn3 = assign34370_e57192_d_n3;
        locals.var_t0_dn4 = assign34370_e57192_d_n4;
        locals.var_t0_dn5 = assign34370_e57192_d_n5;
        locals.var_t0_dn6 = assign34370_e57192_d_n6;
        locals.var_t0_dn7 = assign34370_e57192_d_n7;
        locals.var_t0_dn8 = assign34370_e57192_d_n8;
        locals.var_t0_dn9 = assign34370_e57192_d_n9;
        locals.var_t0_dn10 = assign34370_e57192_d_n10;
        locals.var_t0_dn11 = assign34370_e57192_d_n11;
        locals.var_t0_dn13 = assign34370_e57192_d_n13;
        locals.var_t0_dn14 = assign34370_e57192_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34380_e57217, assign34380_e57217_d_n0, assign34380_e57217_d_n2, assign34380_e57217_d_n3, assign34380_e57217_d_n4, assign34380_e57217_d_n5, assign34380_e57217_d_n6, assign34380_e57217_d_n7, assign34380_e57217_d_n8, assign34380_e57217_d_n9, assign34380_e57217_d_n10, assign34380_e57217_d_n11, assign34380_e57217_d_n13, assign34380_e57217_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34380_e57199: f64 = (locals.var_t0 + 0.1);
        let assign34380_e57202: f64 = (locals.var_t0 - 0.1);
        let assign34380_e57205: f64 = (locals.var_t0 - 0.1);
        let assign34380_e57206: f64 = (assign34380_e57202 * assign34380_e57205);
        let assign34380_e57209: f64 = (0.25 * 2.0);
        let assign34380_e57211: f64 = (assign34380_e57209 * 2.0);
        let assign34380_e57212: f64 = (assign34380_e57206 + assign34380_e57211);
        let assign34380_e57213: f64 = (assign34380_e57212).sqrt();
        let assign34380_e57214: f64 = (assign34380_e57199 + assign34380_e57213);
        let assign34380_e57215: f64 = (0.5 * assign34380_e57214);
        (assign34380_e57215, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn0)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn2)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn3)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn4)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn5)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn6)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn7)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn8)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn9)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn10)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn11)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn13)) / (2.0 * assign34380_e57213)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign34380_e57205) + (assign34380_e57202 * locals.var_t0_dn14)) / (2.0 * assign34380_e57213)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34380_e57217;
        locals.var_t0_dn0 = assign34380_e57217_d_n0;
        locals.var_t0_dn2 = assign34380_e57217_d_n2;
        locals.var_t0_dn3 = assign34380_e57217_d_n3;
        locals.var_t0_dn4 = assign34380_e57217_d_n4;
        locals.var_t0_dn5 = assign34380_e57217_d_n5;
        locals.var_t0_dn6 = assign34380_e57217_d_n6;
        locals.var_t0_dn7 = assign34380_e57217_d_n7;
        locals.var_t0_dn8 = assign34380_e57217_d_n8;
        locals.var_t0_dn9 = assign34380_e57217_d_n9;
        locals.var_t0_dn10 = assign34380_e57217_d_n10;
        locals.var_t0_dn11 = assign34380_e57217_d_n11;
        locals.var_t0_dn13 = assign34380_e57217_d_n13;
        locals.var_t0_dn14 = assign34380_e57217_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34390_e57233, assign34390_e57233_d_n0, assign34390_e57233_d_n2, assign34390_e57233_d_n3, assign34390_e57233_d_n4, assign34390_e57233_d_n5, assign34390_e57233_d_n6, assign34390_e57233_d_n7, assign34390_e57233_d_n8, assign34390_e57233_d_n9, assign34390_e57233_d_n10, assign34390_e57233_d_n11, assign34390_e57233_d_n13, assign34390_e57233_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34390_e57223: f64 = (10.0 * p.p1907);
        let assign34390_e57225: f64 = (assign34390_e57223 * locals.var_t0);
        let assign34390_e57228: f64 = (10.0 * p.p1907);
        let assign34390_e57230: f64 = (assign34390_e57228 + locals.var_t0);
        let assign34390_e57231: f64 = (assign34390_e57225 / assign34390_e57230);
        (assign34390_e57231, ((((assign34390_e57223 * locals.var_t0_dn0) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn0)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn2) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn2)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn3) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn3)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn4) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn4)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn5) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn5)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn6) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn6)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn7) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn7)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn8) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn8)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn9) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn9)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn10) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn10)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn11) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn11)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn13) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn13)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * locals.var_t0_dn14) * assign34390_e57230) - (assign34390_e57225 * locals.var_t0_dn14)) / (assign34390_e57230 * assign34390_e57230)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34390_e57233;
        locals.var_t1_dn0 = assign34390_e57233_d_n0;
        locals.var_t1_dn2 = assign34390_e57233_d_n2;
        locals.var_t1_dn3 = assign34390_e57233_d_n3;
        locals.var_t1_dn4 = assign34390_e57233_d_n4;
        locals.var_t1_dn5 = assign34390_e57233_d_n5;
        locals.var_t1_dn6 = assign34390_e57233_d_n6;
        locals.var_t1_dn7 = assign34390_e57233_d_n7;
        locals.var_t1_dn8 = assign34390_e57233_d_n8;
        locals.var_t1_dn9 = assign34390_e57233_d_n9;
        locals.var_t1_dn10 = assign34390_e57233_d_n10;
        locals.var_t1_dn11 = assign34390_e57233_d_n11;
        locals.var_t1_dn13 = assign34390_e57233_d_n13;
        locals.var_t1_dn14 = assign34390_e57233_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34400_e57245, assign34400_e57245_d_n0, assign34400_e57245_d_n2, assign34400_e57245_d_n3, assign34400_e57245_d_n4, assign34400_e57245_d_n5, assign34400_e57245_d_n6, assign34400_e57245_d_n7, assign34400_e57245_d_n8, assign34400_e57245_d_n9, assign34400_e57245_d_n10, assign34400_e57245_d_n11, assign34400_e57245_d_n13, assign34400_e57245_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34400_e57241: f64 = (p.p1905 * locals.var_t1);
        let assign34400_e57242: f64 = (1.0 + assign34400_e57241);
        let assign34400_e57243: f64 = (locals.var_vsatrsd_t * assign34400_e57242);
        (assign34400_e57243, (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn0)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn2)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn3)), ((locals.var_vsatrsd_t_dn4 * assign34400_e57242) + (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn4))), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn5)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn6)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn7)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn8)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn9)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn10)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn11)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn13)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn14)),)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34400_e57245;
        locals.var_vsatrsd_eff_dn0 = assign34400_e57245_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34400_e57245_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34400_e57245_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34400_e57245_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34400_e57245_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34400_e57245_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34400_e57245_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34400_e57245_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34400_e57245_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34400_e57245_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34400_e57245_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34400_e57245_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34400_e57245_d_n14;
        locals.var_vsatrsd_eff_rv = 0.0;

        let (assign34410_e57286, assign34410_e57286_d_n0, assign34410_e57286_d_n2, assign34410_e57286_d_n3, assign34410_e57286_d_n4, assign34410_e57286_d_n5, assign34410_e57286_d_n6, assign34410_e57286_d_n7, assign34410_e57286_d_n8, assign34410_e57286_d_n9, assign34410_e57286_d_n10, assign34410_e57286_d_n11, assign34410_e57286_d_n13, assign34410_e57286_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34410_e57251: f64 = (-10000.0);
        let assign34410_e57253: f64 = (assign34410_e57251 * 10.0);
        let (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,) = {
            if (!(locals.var_vsatrsd_eff < assign34410_e57253)) {
                let assign34410_e57260: f64 = (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff);
                let assign34410_e57263: f64 = (4.0 * 10.0);
                let assign34410_e57265: f64 = (assign34410_e57263 * 10.0);
                let assign34410_e57266: f64 = (assign34410_e57260 + assign34410_e57265);
                let assign34410_e57267: f64 = (assign34410_e57266).sqrt();
                let assign34410_e57268: f64 = (locals.var_vsatrsd_eff + assign34410_e57267);
                let assign34410_e57269: f64 = (0.5 * assign34410_e57268);
                (assign34410_e57269, (0.5 * (locals.var_vsatrsd_eff_dn0 + (((locals.var_vsatrsd_eff_dn0 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn0)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn2 + (((locals.var_vsatrsd_eff_dn2 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn2)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn3 + (((locals.var_vsatrsd_eff_dn3 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn3)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn4 + (((locals.var_vsatrsd_eff_dn4 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn4)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn5 + (((locals.var_vsatrsd_eff_dn5 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn5)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn6 + (((locals.var_vsatrsd_eff_dn6 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn6)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn7 + (((locals.var_vsatrsd_eff_dn7 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn7)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn8 + (((locals.var_vsatrsd_eff_dn8 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn8)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn9 + (((locals.var_vsatrsd_eff_dn9 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn9)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn10 + (((locals.var_vsatrsd_eff_dn10 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn10)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn11 + (((locals.var_vsatrsd_eff_dn11 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn11)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn13 + (((locals.var_vsatrsd_eff_dn13 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn13)) / (2.0 * assign34410_e57267)))), (0.5 * (locals.var_vsatrsd_eff_dn14 + (((locals.var_vsatrsd_eff_dn14 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn14)) / (2.0 * assign34410_e57267)))),)
            } else {
                let assign34410_e57272: f64 = (-10000.0);
                let assign34410_e57274: f64 = (assign34410_e57272 * 10.0);
                let (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,) = {
                    if (locals.var_vsatrsd_eff < assign34410_e57274) {
                        let assign34410_e57277: f64 = (-10.0);
                        let assign34410_e57279: f64 = (assign34410_e57277 * 10.0);
                        let assign34410_e57281: f64 = (assign34410_e57279 / locals.var_vsatrsd_eff);
                        (assign34410_e57281, (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn0) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn2) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn3) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn4) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn5) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn6) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn7) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn8) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn9) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn10) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn11) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn13) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34410_e57279 * locals.var_vsatrsd_eff_dn14) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,)
            }
        };
        (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34410_e57286;
        locals.var_vsatrsd_eff_dn0 = assign34410_e57286_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34410_e57286_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34410_e57286_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34410_e57286_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34410_e57286_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34410_e57286_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34410_e57286_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34410_e57286_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34410_e57286_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34410_e57286_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34410_e57286_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34410_e57286_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34410_e57286_d_n14;
        locals.var_vsatrsd_eff_rv = 0.0;

        let (assign34420_e57298, assign34420_e57298_d_n0, assign34420_e57298_d_n2, assign34420_e57298_d_n3, assign34420_e57298_d_n4, assign34420_e57298_d_n5, assign34420_e57298_d_n6, assign34420_e57298_d_n7, assign34420_e57298_d_n8, assign34420_e57298_d_n9, assign34420_e57298_d_n10, assign34420_e57298_d_n11, assign34420_e57298_d_n13, assign34420_e57298_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34420_e57292: f64 = (locals.var_nfintotal * locals.var_weff0);
        let assign34420_e57294: f64 = (assign34420_e57292 * 1.60219e-19);
        let assign34420_e57296: f64 = (assign34420_e57294 * locals.var_vsatrsd_eff);
        (assign34420_e57296, (assign34420_e57294 * locals.var_vsatrsd_eff_dn0), (assign34420_e57294 * locals.var_vsatrsd_eff_dn2), (assign34420_e57294 * locals.var_vsatrsd_eff_dn3), (assign34420_e57294 * locals.var_vsatrsd_eff_dn4), (assign34420_e57294 * locals.var_vsatrsd_eff_dn5), (assign34420_e57294 * locals.var_vsatrsd_eff_dn6), (assign34420_e57294 * locals.var_vsatrsd_eff_dn7), (assign34420_e57294 * locals.var_vsatrsd_eff_dn8), (assign34420_e57294 * locals.var_vsatrsd_eff_dn9), (assign34420_e57294 * locals.var_vsatrsd_eff_dn10), (assign34420_e57294 * locals.var_vsatrsd_eff_dn11), (assign34420_e57294 * locals.var_vsatrsd_eff_dn13), (assign34420_e57294 * locals.var_vsatrsd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34420_e57298;
        locals.var_t2_dn0 = assign34420_e57298_d_n0;
        locals.var_t2_dn2 = assign34420_e57298_d_n2;
        locals.var_t2_dn3 = assign34420_e57298_d_n3;
        locals.var_t2_dn4 = assign34420_e57298_d_n4;
        locals.var_t2_dn5 = assign34420_e57298_d_n5;
        locals.var_t2_dn6 = assign34420_e57298_d_n6;
        locals.var_t2_dn7 = assign34420_e57298_d_n7;
        locals.var_t2_dn8 = assign34420_e57298_d_n8;
        locals.var_t2_dn9 = assign34420_e57298_d_n9;
        locals.var_t2_dn10 = assign34420_e57298_d_n10;
        locals.var_t2_dn11 = assign34420_e57298_d_n11;
        locals.var_t2_dn13 = assign34420_e57298_d_n13;
        locals.var_t2_dn14 = assign34420_e57298_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34430_e57305, assign34430_e57305_d_n0, assign34430_e57305_d_n2, assign34430_e57305_d_n3, assign34430_e57305_d_n4, assign34430_e57305_d_n5, assign34430_e57305_d_n6, assign34430_e57305_d_n7, assign34430_e57305_d_n8, assign34430_e57305_d_n9, assign34430_e57305_d_n10, assign34430_e57305_d_n11, assign34430_e57305_d_n13, assign34430_e57305_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34430_e57303: f64 = ((nv9 - nv7)).abs();
        (assign34430_e57303, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv9 - nv7) >= 0.0 { -1.0 } else { 1.0 }, 0.0, if (nv9 - nv7) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34430_e57305;
        locals.var_t5_dn0 = assign34430_e57305_d_n0;
        locals.var_t5_dn2 = assign34430_e57305_d_n2;
        locals.var_t5_dn3 = assign34430_e57305_d_n3;
        locals.var_t5_dn4 = assign34430_e57305_d_n4;
        locals.var_t5_dn5 = assign34430_e57305_d_n5;
        locals.var_t5_dn6 = assign34430_e57305_d_n6;
        locals.var_t5_dn7 = assign34430_e57305_d_n7;
        locals.var_t5_dn8 = assign34430_e57305_d_n8;
        locals.var_t5_dn9 = assign34430_e57305_d_n9;
        locals.var_t5_dn10 = assign34430_e57305_d_n10;
        locals.var_t5_dn11 = assign34430_e57305_d_n11;
        locals.var_t5_dn13 = assign34430_e57305_d_n13;
        locals.var_t5_dn14 = assign34430_e57305_d_n14;
        locals.var_t5_rv = 0.0;

        let assign34440_e57308: f64 = if p.p1917 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign34440_e57308;
        locals.var_guard660_rv = 0.0;

        let (assign34450_e57316, assign34450_e57316_d_n0, assign34450_e57316_d_n2, assign34450_e57316_d_n3, assign34450_e57316_d_n4, assign34450_e57316_d_n5, assign34450_e57316_d_n6, assign34450_e57316_d_n7, assign34450_e57316_d_n8, assign34450_e57316_d_n9, assign34450_e57316_d_n10, assign34450_e57316_d_n11, assign34450_e57316_d_n13, assign34450_e57316_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34450_e57316;
        locals.var_t3_dn0 = assign34450_e57316_d_n0;
        locals.var_t3_dn2 = assign34450_e57316_d_n2;
        locals.var_t3_dn3 = assign34450_e57316_d_n3;
        locals.var_t3_dn4 = assign34450_e57316_d_n4;
        locals.var_t3_dn5 = assign34450_e57316_d_n5;
        locals.var_t3_dn6 = assign34450_e57316_d_n6;
        locals.var_t3_dn7 = assign34450_e57316_d_n7;
        locals.var_t3_dn8 = assign34450_e57316_d_n8;
        locals.var_t3_dn9 = assign34450_e57316_d_n9;
        locals.var_t3_dn10 = assign34450_e57316_d_n10;
        locals.var_t3_dn11 = assign34450_e57316_d_n11;
        locals.var_t3_dn13 = assign34450_e57316_d_n13;
        locals.var_t3_dn14 = assign34450_e57316_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34460_e57350, assign34460_e57350_d_n0, assign34460_e57350_d_n2, assign34460_e57350_d_n3, assign34460_e57350_d_n4, assign34460_e57350_d_n5, assign34460_e57350_d_n6, assign34460_e57350_d_n7, assign34460_e57350_d_n8, assign34460_e57350_d_n9, assign34460_e57350_d_n10, assign34460_e57350_d_n11, assign34460_e57350_d_n13, assign34460_e57350_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign34460_e57326: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57328: f64 = assign34460_e57326;
        let assign34460_e57331: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57333: f64 = assign34460_e57331;
        let assign34460_e57336: f64 = (locals.var_t5 - p.p1916);
        let assign34460_e57338: f64 = assign34460_e57336;
        let assign34460_e57339: f64 = (assign34460_e57333 * assign34460_e57338);
        let assign34460_e57342: f64 = (0.25 * 0.5);
        let assign34460_e57344: f64 = (assign34460_e57342 * 0.5);
        let assign34460_e57345: f64 = (assign34460_e57339 + assign34460_e57344);
        let assign34460_e57346: f64 = (assign34460_e57345).sqrt();
        let assign34460_e57347: f64 = (assign34460_e57328 + assign34460_e57346);
        let assign34460_e57348: f64 = (0.5 * assign34460_e57347);
        (assign34460_e57348, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn0)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn2)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn3)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn4 + (((locals.var_t5_dn4 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn4)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn5)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn6)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn7)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn8)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn9)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn10)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn11)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn13)) / (2.0 * assign34460_e57346)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * assign34460_e57338) + (assign34460_e57333 * locals.var_t5_dn14)) / (2.0 * assign34460_e57346)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34460_e57350;
        locals.var_t3_dn0 = assign34460_e57350_d_n0;
        locals.var_t3_dn2 = assign34460_e57350_d_n2;
        locals.var_t3_dn3 = assign34460_e57350_d_n3;
        locals.var_t3_dn4 = assign34460_e57350_d_n4;
        locals.var_t3_dn5 = assign34460_e57350_d_n5;
        locals.var_t3_dn6 = assign34460_e57350_d_n6;
        locals.var_t3_dn7 = assign34460_e57350_d_n7;
        locals.var_t3_dn8 = assign34460_e57350_d_n8;
        locals.var_t3_dn9 = assign34460_e57350_d_n9;
        locals.var_t3_dn10 = assign34460_e57350_d_n10;
        locals.var_t3_dn11 = assign34460_e57350_d_n11;
        locals.var_t3_dn13 = assign34460_e57350_d_n13;
        locals.var_t3_dn14 = assign34460_e57350_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_134(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34470_e57363, assign34470_e57363_d_n0, assign34470_e57363_d_n2, assign34470_e57363_d_n3, assign34470_e57363_d_n4, assign34470_e57363_d_n5, assign34470_e57363_d_n6, assign34470_e57363_d_n7, assign34470_e57363_d_n8, assign34470_e57363_d_n9, assign34470_e57363_d_n10, assign34470_e57363_d_n11, assign34470_e57363_d_n13, assign34470_e57363_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign34470_e57360: f64 = (locals.var_t3 * p.p1917);
        let assign34470_e57361: f64 = (1.0 + assign34470_e57360);
        (assign34470_e57361, (locals.var_t3_dn0 * p.p1917), (locals.var_t3_dn2 * p.p1917), (locals.var_t3_dn3 * p.p1917), (locals.var_t3_dn4 * p.p1917), (locals.var_t3_dn5 * p.p1917), (locals.var_t3_dn6 * p.p1917), (locals.var_t3_dn7 * p.p1917), (locals.var_t3_dn8 * p.p1917), (locals.var_t3_dn9 * p.p1917), (locals.var_t3_dn10 * p.p1917), (locals.var_t3_dn11 * p.p1917), (locals.var_t3_dn13 * p.p1917), (locals.var_t3_dn14 * p.p1917),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34470_e57363;
        locals.var_t3_dn0 = assign34470_e57363_d_n0;
        locals.var_t3_dn2 = assign34470_e57363_d_n2;
        locals.var_t3_dn3 = assign34470_e57363_d_n3;
        locals.var_t3_dn4 = assign34470_e57363_d_n4;
        locals.var_t3_dn5 = assign34470_e57363_d_n5;
        locals.var_t3_dn6 = assign34470_e57363_d_n6;
        locals.var_t3_dn7 = assign34470_e57363_d_n7;
        locals.var_t3_dn8 = assign34470_e57363_d_n8;
        locals.var_t3_dn9 = assign34470_e57363_d_n9;
        locals.var_t3_dn10 = assign34470_e57363_d_n10;
        locals.var_t3_dn11 = assign34470_e57363_d_n11;
        locals.var_t3_dn13 = assign34470_e57363_d_n13;
        locals.var_t3_dn14 = assign34470_e57363_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34480_e57373, assign34480_e57373_d_n0, assign34480_e57373_d_n2, assign34480_e57373_d_n3, assign34480_e57373_d_n4, assign34480_e57373_d_n5, assign34480_e57373_d_n6, assign34480_e57373_d_n7, assign34480_e57373_d_n8, assign34480_e57373_d_n9, assign34480_e57373_d_n10, assign34480_e57373_d_n11, assign34480_e57373_d_n13, assign34480_e57373_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34480_e57369: f64 = (locals.var_t2 * p.p1903);
        let assign34480_e57371: f64 = (assign34480_e57369 * locals.var_t3);
        (assign34480_e57371, (((locals.var_t2_dn0 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn0)), (((locals.var_t2_dn2 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn2)), (((locals.var_t2_dn3 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn3)), (((locals.var_t2_dn4 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn4)), (((locals.var_t2_dn5 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn5)), (((locals.var_t2_dn6 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn6)), (((locals.var_t2_dn7 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn7)), (((locals.var_t2_dn8 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn8)), (((locals.var_t2_dn9 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn9)), (((locals.var_t2_dn10 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn10)), (((locals.var_t2_dn11 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn11)), (((locals.var_t2_dn13 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn13)), (((locals.var_t2_dn14 * p.p1903) * locals.var_t3) + (assign34480_e57369 * locals.var_t3_dn14)),)
    } else {
        (locals.var_isat_rd, locals.var_isat_rd_dn0, locals.var_isat_rd_dn2, locals.var_isat_rd_dn3, locals.var_isat_rd_dn4, locals.var_isat_rd_dn5, locals.var_isat_rd_dn6, locals.var_isat_rd_dn7, locals.var_isat_rd_dn8, locals.var_isat_rd_dn9, locals.var_isat_rd_dn10, locals.var_isat_rd_dn11, locals.var_isat_rd_dn13, locals.var_isat_rd_dn14,)
    }
};
        locals.var_isat_rd = assign34480_e57373;
        locals.var_isat_rd_dn0 = assign34480_e57373_d_n0;
        locals.var_isat_rd_dn2 = assign34480_e57373_d_n2;
        locals.var_isat_rd_dn3 = assign34480_e57373_d_n3;
        locals.var_isat_rd_dn4 = assign34480_e57373_d_n4;
        locals.var_isat_rd_dn5 = assign34480_e57373_d_n5;
        locals.var_isat_rd_dn6 = assign34480_e57373_d_n6;
        locals.var_isat_rd_dn7 = assign34480_e57373_d_n7;
        locals.var_isat_rd_dn8 = assign34480_e57373_d_n8;
        locals.var_isat_rd_dn9 = assign34480_e57373_d_n9;
        locals.var_isat_rd_dn10 = assign34480_e57373_d_n10;
        locals.var_isat_rd_dn11 = assign34480_e57373_d_n11;
        locals.var_isat_rd_dn13 = assign34480_e57373_d_n13;
        locals.var_isat_rd_dn14 = assign34480_e57373_d_n14;
        locals.var_isat_rd_rv = 0.0;

        let (assign34490_e57383, assign34490_e57383_d_n0, assign34490_e57383_d_n2, assign34490_e57383_d_n3, assign34490_e57383_d_n4, assign34490_e57383_d_n5, assign34490_e57383_d_n6, assign34490_e57383_d_n7, assign34490_e57383_d_n8, assign34490_e57383_d_n9, assign34490_e57383_d_n10, assign34490_e57383_d_n11, assign34490_e57383_d_n13, assign34490_e57383_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34490_e57379: f64 = (locals.var_rdstempvs * p.p1910);
        let assign34490_e57381: f64 = (assign34490_e57379 * locals.var_weffwrfactor);
        (assign34490_e57381, 0.0, 0.0, 0.0, ((locals.var_rdstempvs_dn4 * p.p1910) * locals.var_weffwrfactor), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34490_e57383;
        locals.var_t4_dn0 = assign34490_e57383_d_n0;
        locals.var_t4_dn2 = assign34490_e57383_d_n2;
        locals.var_t4_dn3 = assign34490_e57383_d_n3;
        locals.var_t4_dn4 = assign34490_e57383_d_n4;
        locals.var_t4_dn5 = assign34490_e57383_d_n5;
        locals.var_t4_dn6 = assign34490_e57383_d_n6;
        locals.var_t4_dn7 = assign34490_e57383_d_n7;
        locals.var_t4_dn8 = assign34490_e57383_d_n8;
        locals.var_t4_dn9 = assign34490_e57383_d_n9;
        locals.var_t4_dn10 = assign34490_e57383_d_n10;
        locals.var_t4_dn11 = assign34490_e57383_d_n11;
        locals.var_t4_dn13 = assign34490_e57383_d_n13;
        locals.var_t4_dn14 = assign34490_e57383_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign34500_e57391, assign34500_e57391_d_n0, assign34500_e57391_d_n2, assign34500_e57391_d_n3, assign34500_e57391_d_n4, assign34500_e57391_d_n5, assign34500_e57391_d_n6, assign34500_e57391_d_n7, assign34500_e57391_d_n8, assign34500_e57391_d_n9, assign34500_e57391_d_n10, assign34500_e57391_d_n11, assign34500_e57391_d_n13, assign34500_e57391_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34500_e57389: f64 = (locals.var_isat_rd * locals.var_t4);
        (assign34500_e57389, ((locals.var_isat_rd_dn0 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn0)), ((locals.var_isat_rd_dn2 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn2)), ((locals.var_isat_rd_dn3 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn3)), ((locals.var_isat_rd_dn4 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn4)), ((locals.var_isat_rd_dn5 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn5)), ((locals.var_isat_rd_dn6 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn6)), ((locals.var_isat_rd_dn7 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn7)), ((locals.var_isat_rd_dn8 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn8)), ((locals.var_isat_rd_dn9 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn9)), ((locals.var_isat_rd_dn10 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn10)), ((locals.var_isat_rd_dn11 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn11)), ((locals.var_isat_rd_dn13 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn13)), ((locals.var_isat_rd_dn14 * locals.var_t4) + (locals.var_isat_rd * locals.var_t4_dn14)),)
    } else {
        (locals.var_vsat_rd, locals.var_vsat_rd_dn0, locals.var_vsat_rd_dn2, locals.var_vsat_rd_dn3, locals.var_vsat_rd_dn4, locals.var_vsat_rd_dn5, locals.var_vsat_rd_dn6, locals.var_vsat_rd_dn7, locals.var_vsat_rd_dn8, locals.var_vsat_rd_dn9, locals.var_vsat_rd_dn10, locals.var_vsat_rd_dn11, locals.var_vsat_rd_dn13, locals.var_vsat_rd_dn14,)
    }
};
        locals.var_vsat_rd = assign34500_e57391;
        locals.var_vsat_rd_dn0 = assign34500_e57391_d_n0;
        locals.var_vsat_rd_dn2 = assign34500_e57391_d_n2;
        locals.var_vsat_rd_dn3 = assign34500_e57391_d_n3;
        locals.var_vsat_rd_dn4 = assign34500_e57391_d_n4;
        locals.var_vsat_rd_dn5 = assign34500_e57391_d_n5;
        locals.var_vsat_rd_dn6 = assign34500_e57391_d_n6;
        locals.var_vsat_rd_dn7 = assign34500_e57391_d_n7;
        locals.var_vsat_rd_dn8 = assign34500_e57391_d_n8;
        locals.var_vsat_rd_dn9 = assign34500_e57391_d_n9;
        locals.var_vsat_rd_dn10 = assign34500_e57391_d_n10;
        locals.var_vsat_rd_dn11 = assign34500_e57391_d_n11;
        locals.var_vsat_rd_dn13 = assign34500_e57391_d_n13;
        locals.var_vsat_rd_dn14 = assign34500_e57391_d_n14;
        locals.var_vsat_rd_rv = 0.0;

        let (assign34510_e57415, assign34510_e57415_d_n0, assign34510_e57415_d_n2, assign34510_e57415_d_n3, assign34510_e57415_d_n4, assign34510_e57415_d_n5, assign34510_e57415_d_n6, assign34510_e57415_d_n7, assign34510_e57415_d_n8, assign34510_e57415_d_n9, assign34510_e57415_d_n10, assign34510_e57415_d_n11, assign34510_e57415_d_n13, assign34510_e57415_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34510_e57398: f64 = (4.0 - p.p1908);
        let assign34510_e57399: f64 = (locals.var_t5).powf(assign34510_e57398);
        let assign34510_e57403: f64 = (4.0 - p.p1908);
        let assign34510_e57404: f64 = (locals.var_t5).powf(assign34510_e57403);
        let assign34510_e57409: f64 = (4.0 - p.p1908);
        let assign34510_e57410: f64 = (locals.var_vsat_rd).powf(assign34510_e57409);
        let assign34510_e57411: f64 = (p.p1914 * assign34510_e57410);
        let assign34510_e57412: f64 = (assign34510_e57404 + assign34510_e57411);
        let assign34510_e57413: f64 = (assign34510_e57399 / assign34510_e57412);
        (assign34510_e57413, (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn0)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn0 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn0)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn0 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn0)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn0 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn2)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn2 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn2)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn2 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn2)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn2 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn3)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn3 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn3)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn3 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn3)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn3 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn4)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn4 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn4)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn4 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn4)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn4 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn5)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn5 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn5)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn5 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn5)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn5 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn6)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn6 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn6)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn6 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn6)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn6 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn7)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn7 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn7)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn7 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn7)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn7 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn8)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn8 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn8)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn8 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn8)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn8 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn9)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn9 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn9)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn9 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn9)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn9 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn10)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn10 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn10)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn10 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn10)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn10 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn11)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn11 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn11)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn11 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn11)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn11 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn13)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn13 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn13)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn13 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn13)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn13 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((locals.var_t5).powf(assign34510_e57398 - 1.0) * locals.var_t5_dn14)) } } else { (assign34510_e57399 * (assign34510_e57398 * (locals.var_t5_dn14 / locals.var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((locals.var_t5).powf(assign34510_e57403 - 1.0) * locals.var_t5_dn14)) } } else { (assign34510_e57404 * (assign34510_e57403 * (locals.var_t5_dn14 / locals.var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((locals.var_vsat_rd).powf(assign34510_e57409 - 1.0) * locals.var_vsat_rd_dn14)) } } else { (assign34510_e57410 * (assign34510_e57409 * (locals.var_vsat_rd_dn14 / locals.var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)),)
    } else {
        (locals.var_delta_vsrd, locals.var_delta_vsrd_dn0, locals.var_delta_vsrd_dn2, locals.var_delta_vsrd_dn3, locals.var_delta_vsrd_dn4, locals.var_delta_vsrd_dn5, locals.var_delta_vsrd_dn6, locals.var_delta_vsrd_dn7, locals.var_delta_vsrd_dn8, locals.var_delta_vsrd_dn9, locals.var_delta_vsrd_dn10, locals.var_delta_vsrd_dn11, locals.var_delta_vsrd_dn13, locals.var_delta_vsrd_dn14,)
    }
};
        locals.var_delta_vsrd = assign34510_e57415;
        locals.var_delta_vsrd_dn0 = assign34510_e57415_d_n0;
        locals.var_delta_vsrd_dn2 = assign34510_e57415_d_n2;
        locals.var_delta_vsrd_dn3 = assign34510_e57415_d_n3;
        locals.var_delta_vsrd_dn4 = assign34510_e57415_d_n4;
        locals.var_delta_vsrd_dn5 = assign34510_e57415_d_n5;
        locals.var_delta_vsrd_dn6 = assign34510_e57415_d_n6;
        locals.var_delta_vsrd_dn7 = assign34510_e57415_d_n7;
        locals.var_delta_vsrd_dn8 = assign34510_e57415_d_n8;
        locals.var_delta_vsrd_dn9 = assign34510_e57415_d_n9;
        locals.var_delta_vsrd_dn10 = assign34510_e57415_d_n10;
        locals.var_delta_vsrd_dn11 = assign34510_e57415_d_n11;
        locals.var_delta_vsrd_dn13 = assign34510_e57415_d_n13;
        locals.var_delta_vsrd_dn14 = assign34510_e57415_d_n14;
        locals.var_delta_vsrd_rv = 0.0;

        let (assign34520_e57429, assign34520_e57429_d_n0, assign34520_e57429_d_n2, assign34520_e57429_d_n3, assign34520_e57429_d_n4, assign34520_e57429_d_n5, assign34520_e57429_d_n6, assign34520_e57429_d_n7, assign34520_e57429_d_n8, assign34520_e57429_d_n9, assign34520_e57429_d_n10, assign34520_e57429_d_n11, assign34520_e57429_d_n13, assign34520_e57429_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard658 != 0.0)) {
        let assign34520_e57422: f64 = (1.0 / p.p1908);
        let assign34520_e57423: f64 = (locals.var_delta_vsrd).powf(assign34520_e57422);
        let assign34520_e57425: f64 = (assign34520_e57423 * locals.var_t5);
        let assign34520_e57427: f64 = (assign34520_e57425 / locals.var_vsat_rd);
        (assign34520_e57427, (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn0)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn0 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn0)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn0)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn2)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn2 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn2)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn2)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn3)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn3 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn3)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn3)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn4)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn4 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn4)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn4)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn5)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn5 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn5)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn5)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn6)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn6 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn6)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn6)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn7)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn7 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn7)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn7)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn8)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn8 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn8)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn8)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn9)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn9 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn9)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn9)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn10)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn10 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn10)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn10)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn11)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn11 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn11)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn11)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn13)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn13 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn13)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn13)) / (locals.var_vsat_rd * locals.var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((locals.var_delta_vsrd).powf(assign34520_e57422 - 1.0) * locals.var_delta_vsrd_dn14)) } } else { (assign34520_e57423 * (assign34520_e57422 * (locals.var_delta_vsrd_dn14 / locals.var_delta_vsrd))) } * locals.var_t5) + (assign34520_e57423 * locals.var_t5_dn14)) * locals.var_vsat_rd) - (assign34520_e57425 * locals.var_vsat_rd_dn14)) / (locals.var_vsat_rd * locals.var_vsat_rd)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34520_e57429;
        locals.var_t6_dn0 = assign34520_e57429_d_n0;
        locals.var_t6_dn2 = assign34520_e57429_d_n2;
        locals.var_t6_dn3 = assign34520_e57429_d_n3;
        locals.var_t6_dn4 = assign34520_e57429_d_n4;
        locals.var_t6_dn5 = assign34520_e57429_d_n5;
        locals.var_t6_dn6 = assign34520_e57429_d_n6;
        locals.var_t6_dn7 = assign34520_e57429_d_n7;
        locals.var_t6_dn8 = assign34520_e57429_d_n8;
        locals.var_t6_dn9 = assign34520_e57429_d_n9;
        locals.var_t6_dn10 = assign34520_e57429_d_n10;
        locals.var_t6_dn11 = assign34520_e57429_d_n11;
        locals.var_t6_dn13 = assign34520_e57429_d_n13;
        locals.var_t6_dn14 = assign34520_e57429_d_n14;
        locals.var_t6_rv = 0.0;

        let assign34540_e57448: f64 = if p.p1911 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign34540_e57448;
        locals.var_guard661_rv = 0.0;

        let assign34550_e57451: f64 = if p.p1910 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign34550_e57451;
        locals.var_guard662_rv = 0.0;

        let (assign34560_e57530, assign34560_e57530_d_n4,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34560_e57460: f64 = (p.p1912 * locals.var_deltemp);
        let assign34560_e57461: f64 = (1.0 + assign34560_e57460);
        let assign34560_e57463: f64 = (assign34560_e57461 - 1e-6);
        let assign34560_e57465: f64 = (-10000.0);
        let assign34560_e57467: f64 = (assign34560_e57465 * 0.001);
        let (assign34560_e57528, assign34560_e57528_d_n4,) = {
            if (!(assign34560_e57463 < assign34560_e57467)) {
                let assign34560_e57474: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57475: f64 = (1.0 + assign34560_e57474);
                let assign34560_e57477: f64 = (assign34560_e57475 - 1e-6);
                let assign34560_e57481: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57482: f64 = (1.0 + assign34560_e57481);
                let assign34560_e57484: f64 = (assign34560_e57482 - 1e-6);
                let assign34560_e57488: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57489: f64 = (1.0 + assign34560_e57488);
                let assign34560_e57491: f64 = (assign34560_e57489 - 1e-6);
                let assign34560_e57492: f64 = (assign34560_e57484 * assign34560_e57491);
                let assign34560_e57495: f64 = (4.0 * 0.001);
                let assign34560_e57497: f64 = (assign34560_e57495 * 0.001);
                let assign34560_e57498: f64 = (assign34560_e57492 + assign34560_e57497);
                let assign34560_e57499: f64 = (assign34560_e57498).sqrt();
                let assign34560_e57500: f64 = (assign34560_e57477 + assign34560_e57499);
                let assign34560_e57501: f64 = (0.5 * assign34560_e57500);
                (assign34560_e57501, (0.5 * ((p.p1912 * locals.var_deltemp_dn4) + ((((p.p1912 * locals.var_deltemp_dn4) * assign34560_e57491) + (assign34560_e57484 * (p.p1912 * locals.var_deltemp_dn4))) / (2.0 * assign34560_e57499)))),)
            } else {
                let assign34560_e57505: f64 = (p.p1912 * locals.var_deltemp);
                let assign34560_e57506: f64 = (1.0 + assign34560_e57505);
                let assign34560_e57508: f64 = (assign34560_e57506 - 1e-6);
                let assign34560_e57510: f64 = (-10000.0);
                let assign34560_e57512: f64 = (assign34560_e57510 * 0.001);
                let (assign34560_e57527, assign34560_e57527_d_n4,) = {
                    if (assign34560_e57508 < assign34560_e57512) {
                        let assign34560_e57515: f64 = (-0.001);
                        let assign34560_e57517: f64 = (assign34560_e57515 * 0.001);
                        let assign34560_e57521: f64 = (p.p1912 * locals.var_deltemp);
                        let assign34560_e57522: f64 = (1.0 + assign34560_e57521);
                        let assign34560_e57524: f64 = (assign34560_e57522 - 1e-6);
                        let assign34560_e57525: f64 = (assign34560_e57517 / assign34560_e57524);
                        (assign34560_e57525, (-((assign34560_e57517 * (p.p1912 * locals.var_deltemp_dn4)) / (assign34560_e57524 * assign34560_e57524))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34560_e57527, assign34560_e57527_d_n4,)
            }
        };
        (assign34560_e57528, assign34560_e57528_d_n4,)
    } else {
        (locals.var_rdstempvs, locals.var_rdstempvs_dn4,)
    }
};
        locals.var_rdstempvs = assign34560_e57530;
        locals.var_rdstempvs_dn4 = assign34560_e57530_d_n4;
        locals.var_rdstempvs_rv = 0.0;

        let assign34570_e57533: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign34570_e57533;
        locals.var_guard663_rv = 0.0;

        let (assign34580_e57586, assign34580_e57586_d_n4,) = {
    if ((((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign34580_e57543: f64 = (-p.p1904);
        let assign34580_e57546: f64 = (-p.p1913);
        let assign34580_e57548: f64 = (assign34580_e57546 * locals.var_deltemp);
        let assign34580_e57550: f64 = (-p.p1904);
        let assign34580_e57551: f64 = (assign34580_e57548 - assign34580_e57550);
        let assign34580_e57553: f64 = (assign34580_e57551 - 1e-6);
        let assign34580_e57555: f64 = (-p.p1913);
        let assign34580_e57557: f64 = (assign34580_e57555 * locals.var_deltemp);
        let assign34580_e57559: f64 = (-p.p1904);
        let assign34580_e57560: f64 = (assign34580_e57557 - assign34580_e57559);
        let assign34580_e57562: f64 = (assign34580_e57560 - 1e-6);
        let assign34580_e57564: f64 = (-p.p1913);
        let assign34580_e57566: f64 = (assign34580_e57564 * locals.var_deltemp);
        let assign34580_e57568: f64 = (-p.p1904);
        let assign34580_e57569: f64 = (assign34580_e57566 - assign34580_e57568);
        let assign34580_e57571: f64 = (assign34580_e57569 - 1e-6);
        let assign34580_e57572: f64 = (assign34580_e57562 * assign34580_e57571);
        let assign34580_e57575: f64 = (-p.p1904);
        let assign34580_e57576: f64 = (4.0 * assign34580_e57575);
        let assign34580_e57578: f64 = (assign34580_e57576 * 1e-6);
        let assign34580_e57579: f64 = (assign34580_e57572 - assign34580_e57578);
        let assign34580_e57580: f64 = (assign34580_e57579).sqrt();
        let assign34580_e57581: f64 = (assign34580_e57553 + assign34580_e57580);
        let assign34580_e57582: f64 = (0.5 * assign34580_e57581);
        let assign34580_e57583: f64 = (assign34580_e57543 + assign34580_e57582);
        let assign34580_e57584: f64 = (p.p1904 + assign34580_e57583);
        (assign34580_e57584, (0.5 * ((assign34580_e57546 * locals.var_deltemp_dn4) + ((((assign34580_e57555 * locals.var_deltemp_dn4) * assign34580_e57571) + (assign34580_e57562 * (assign34580_e57564 * locals.var_deltemp_dn4))) / (2.0 * assign34580_e57580)))),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34580_e57586;
        locals.var_vsatrsd_t_dn4 = assign34580_e57586_d_n4;
        locals.var_vsatrsd_t_rv = 0.0;

        let (assign34590_e57676, assign34590_e57676_d_n4,) = {
    if ((((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) && (locals.var_guard663 == 0.0)) {
        let assign34590_e57598: f64 = (-p.p1913);
        let assign34590_e57600: f64 = (assign34590_e57598 * locals.var_deltemp);
        let assign34590_e57601: f64 = (1.0 + assign34590_e57600);
        let assign34590_e57603: f64 = (assign34590_e57601 - 1e-6);
        let assign34590_e57605: f64 = (-10000.0);
        let assign34590_e57607: f64 = (assign34590_e57605 * 0.001);
        let (assign34590_e57673, assign34590_e57673_d_n4,) = {
            if (!(assign34590_e57603 < assign34590_e57607)) {
                let assign34590_e57613: f64 = (-p.p1913);
                let assign34590_e57615: f64 = (assign34590_e57613 * locals.var_deltemp);
                let assign34590_e57616: f64 = (1.0 + assign34590_e57615);
                let assign34590_e57618: f64 = (assign34590_e57616 - 1e-6);
                let assign34590_e57621: f64 = (-p.p1913);
                let assign34590_e57623: f64 = (assign34590_e57621 * locals.var_deltemp);
                let assign34590_e57624: f64 = (1.0 + assign34590_e57623);
                let assign34590_e57626: f64 = (assign34590_e57624 - 1e-6);
                let assign34590_e57629: f64 = (-p.p1913);
                let assign34590_e57631: f64 = (assign34590_e57629 * locals.var_deltemp);
                let assign34590_e57632: f64 = (1.0 + assign34590_e57631);
                let assign34590_e57634: f64 = (assign34590_e57632 - 1e-6);
                let assign34590_e57635: f64 = (assign34590_e57626 * assign34590_e57634);
                let assign34590_e57638: f64 = (4.0 * 0.001);
                let assign34590_e57640: f64 = (assign34590_e57638 * 0.001);
                let assign34590_e57641: f64 = (assign34590_e57635 + assign34590_e57640);
                let assign34590_e57642: f64 = (assign34590_e57641).sqrt();
                let assign34590_e57643: f64 = (assign34590_e57618 + assign34590_e57642);
                let assign34590_e57644: f64 = (0.5 * assign34590_e57643);
                (assign34590_e57644, (0.5 * ((assign34590_e57613 * locals.var_deltemp_dn4) + ((((assign34590_e57621 * locals.var_deltemp_dn4) * assign34590_e57634) + (assign34590_e57626 * (assign34590_e57629 * locals.var_deltemp_dn4))) / (2.0 * assign34590_e57642)))),)
            } else {
                let assign34590_e57647: f64 = (-p.p1913);
                let assign34590_e57649: f64 = (assign34590_e57647 * locals.var_deltemp);
                let assign34590_e57650: f64 = (1.0 + assign34590_e57649);
                let assign34590_e57652: f64 = (assign34590_e57650 - 1e-6);
                let assign34590_e57654: f64 = (-10000.0);
                let assign34590_e57656: f64 = (assign34590_e57654 * 0.001);
                let (assign34590_e57672, assign34590_e57672_d_n4,) = {
                    if (assign34590_e57652 < assign34590_e57656) {
                        let assign34590_e57659: f64 = (-0.001);
                        let assign34590_e57661: f64 = (assign34590_e57659 * 0.001);
                        let assign34590_e57664: f64 = (-p.p1913);
                        let assign34590_e57666: f64 = (assign34590_e57664 * locals.var_deltemp);
                        let assign34590_e57667: f64 = (1.0 + assign34590_e57666);
                        let assign34590_e57669: f64 = (assign34590_e57667 - 1e-6);
                        let assign34590_e57670: f64 = (assign34590_e57661 / assign34590_e57669);
                        (assign34590_e57670, (-((assign34590_e57661 * (assign34590_e57664 * locals.var_deltemp_dn4)) / (assign34590_e57669 * assign34590_e57669))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34590_e57672, assign34590_e57672_d_n4,)
            }
        };
        let assign34590_e57674: f64 = (p.p1904 * assign34590_e57673);
        (assign34590_e57674, (p.p1904 * assign34590_e57673_d_n4),)
    } else {
        (locals.var_vsatrsd_t, locals.var_vsatrsd_t_dn4,)
    }
};
        locals.var_vsatrsd_t = assign34590_e57676;
        locals.var_vsatrsd_t_dn4 = assign34590_e57676_d_n4;
        locals.var_vsatrsd_t_rv = 0.0;

        let (assign34600_e57686, assign34600_e57686_d_n0, assign34600_e57686_d_n2, assign34600_e57686_d_n3, assign34600_e57686_d_n4, assign34600_e57686_d_n5, assign34600_e57686_d_n6, assign34600_e57686_d_n7, assign34600_e57686_d_n8, assign34600_e57686_d_n9, assign34600_e57686_d_n10, assign34600_e57686_d_n11, assign34600_e57686_d_n13, assign34600_e57686_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34600_e57684: f64 = (locals.var_qis - p.p1906);
        (assign34600_e57684, locals.var_qis_dn0, locals.var_qis_dn2, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11, locals.var_qis_dn13, locals.var_qis_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34600_e57686;
        locals.var_t0_dn0 = assign34600_e57686_d_n0;
        locals.var_t0_dn2 = assign34600_e57686_d_n2;
        locals.var_t0_dn3 = assign34600_e57686_d_n3;
        locals.var_t0_dn4 = assign34600_e57686_d_n4;
        locals.var_t0_dn5 = assign34600_e57686_d_n5;
        locals.var_t0_dn6 = assign34600_e57686_d_n6;
        locals.var_t0_dn7 = assign34600_e57686_d_n7;
        locals.var_t0_dn8 = assign34600_e57686_d_n8;
        locals.var_t0_dn9 = assign34600_e57686_d_n9;
        locals.var_t0_dn10 = assign34600_e57686_d_n10;
        locals.var_t0_dn11 = assign34600_e57686_d_n11;
        locals.var_t0_dn13 = assign34600_e57686_d_n13;
        locals.var_t0_dn14 = assign34600_e57686_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34610_e57713, assign34610_e57713_d_n0, assign34610_e57713_d_n2, assign34610_e57713_d_n3, assign34610_e57713_d_n4, assign34610_e57713_d_n5, assign34610_e57713_d_n6, assign34610_e57713_d_n7, assign34610_e57713_d_n8, assign34610_e57713_d_n9, assign34610_e57713_d_n10, assign34610_e57713_d_n11, assign34610_e57713_d_n13, assign34610_e57713_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34610_e57695: f64 = (locals.var_t0 + 0.1);
        let assign34610_e57698: f64 = (locals.var_t0 - 0.1);
        let assign34610_e57701: f64 = (locals.var_t0 - 0.1);
        let assign34610_e57702: f64 = (assign34610_e57698 * assign34610_e57701);
        let assign34610_e57705: f64 = (0.25 * 2.0);
        let assign34610_e57707: f64 = (assign34610_e57705 * 2.0);
        let assign34610_e57708: f64 = (assign34610_e57702 + assign34610_e57707);
        let assign34610_e57709: f64 = (assign34610_e57708).sqrt();
        let assign34610_e57710: f64 = (assign34610_e57695 + assign34610_e57709);
        let assign34610_e57711: f64 = (0.5 * assign34610_e57710);
        (assign34610_e57711, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn0)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn2)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn3)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn4)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn5)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn6)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn7)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn8)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn9)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn10)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn11)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn13)) / (2.0 * assign34610_e57709)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign34610_e57701) + (assign34610_e57698 * locals.var_t0_dn14)) / (2.0 * assign34610_e57709)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34610_e57713;
        locals.var_t0_dn0 = assign34610_e57713_d_n0;
        locals.var_t0_dn2 = assign34610_e57713_d_n2;
        locals.var_t0_dn3 = assign34610_e57713_d_n3;
        locals.var_t0_dn4 = assign34610_e57713_d_n4;
        locals.var_t0_dn5 = assign34610_e57713_d_n5;
        locals.var_t0_dn6 = assign34610_e57713_d_n6;
        locals.var_t0_dn7 = assign34610_e57713_d_n7;
        locals.var_t0_dn8 = assign34610_e57713_d_n8;
        locals.var_t0_dn9 = assign34610_e57713_d_n9;
        locals.var_t0_dn10 = assign34610_e57713_d_n10;
        locals.var_t0_dn11 = assign34610_e57713_d_n11;
        locals.var_t0_dn13 = assign34610_e57713_d_n13;
        locals.var_t0_dn14 = assign34610_e57713_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34620_e57731, assign34620_e57731_d_n0, assign34620_e57731_d_n2, assign34620_e57731_d_n3, assign34620_e57731_d_n4, assign34620_e57731_d_n5, assign34620_e57731_d_n6, assign34620_e57731_d_n7, assign34620_e57731_d_n8, assign34620_e57731_d_n9, assign34620_e57731_d_n10, assign34620_e57731_d_n11, assign34620_e57731_d_n13, assign34620_e57731_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34620_e57721: f64 = (10.0 * p.p1907);
        let assign34620_e57723: f64 = (assign34620_e57721 * locals.var_t0);
        let assign34620_e57726: f64 = (10.0 * p.p1907);
        let assign34620_e57728: f64 = (assign34620_e57726 + locals.var_t0);
        let assign34620_e57729: f64 = (assign34620_e57723 / assign34620_e57728);
        (assign34620_e57729, ((((assign34620_e57721 * locals.var_t0_dn0) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn0)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn2) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn2)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn3) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn3)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn4) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn4)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn5) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn5)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn6) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn6)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn7) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn7)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn8) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn8)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn9) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn9)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn10) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn10)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn11) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn11)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn13) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn13)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * locals.var_t0_dn14) * assign34620_e57728) - (assign34620_e57723 * locals.var_t0_dn14)) / (assign34620_e57728 * assign34620_e57728)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34620_e57731;
        locals.var_t1_dn0 = assign34620_e57731_d_n0;
        locals.var_t1_dn2 = assign34620_e57731_d_n2;
        locals.var_t1_dn3 = assign34620_e57731_d_n3;
        locals.var_t1_dn4 = assign34620_e57731_d_n4;
        locals.var_t1_dn5 = assign34620_e57731_d_n5;
        locals.var_t1_dn6 = assign34620_e57731_d_n6;
        locals.var_t1_dn7 = assign34620_e57731_d_n7;
        locals.var_t1_dn8 = assign34620_e57731_d_n8;
        locals.var_t1_dn9 = assign34620_e57731_d_n9;
        locals.var_t1_dn10 = assign34620_e57731_d_n10;
        locals.var_t1_dn11 = assign34620_e57731_d_n11;
        locals.var_t1_dn13 = assign34620_e57731_d_n13;
        locals.var_t1_dn14 = assign34620_e57731_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34630_e57745, assign34630_e57745_d_n0, assign34630_e57745_d_n2, assign34630_e57745_d_n3, assign34630_e57745_d_n4, assign34630_e57745_d_n5, assign34630_e57745_d_n6, assign34630_e57745_d_n7, assign34630_e57745_d_n8, assign34630_e57745_d_n9, assign34630_e57745_d_n10, assign34630_e57745_d_n11, assign34630_e57745_d_n13, assign34630_e57745_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34630_e57741: f64 = (p.p1905 * locals.var_t1);
        let assign34630_e57742: f64 = (1.0 + assign34630_e57741);
        let assign34630_e57743: f64 = (locals.var_vsatrsd_t * assign34630_e57742);
        (assign34630_e57743, (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn0)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn2)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn3)), ((locals.var_vsatrsd_t_dn4 * assign34630_e57742) + (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn4))), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn5)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn6)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn7)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn8)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn9)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn10)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn11)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn13)), (locals.var_vsatrsd_t * (p.p1905 * locals.var_t1_dn14)),)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34630_e57745;
        locals.var_vsatrsd_eff_dn0 = assign34630_e57745_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34630_e57745_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34630_e57745_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34630_e57745_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34630_e57745_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34630_e57745_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34630_e57745_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34630_e57745_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34630_e57745_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34630_e57745_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34630_e57745_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34630_e57745_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34630_e57745_d_n14;
        locals.var_vsatrsd_eff_rv = 0.0;

        let (assign34640_e57788, assign34640_e57788_d_n0, assign34640_e57788_d_n2, assign34640_e57788_d_n3, assign34640_e57788_d_n4, assign34640_e57788_d_n5, assign34640_e57788_d_n6, assign34640_e57788_d_n7, assign34640_e57788_d_n8, assign34640_e57788_d_n9, assign34640_e57788_d_n10, assign34640_e57788_d_n11, assign34640_e57788_d_n13, assign34640_e57788_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34640_e57753: f64 = (-10000.0);
        let assign34640_e57755: f64 = (assign34640_e57753 * 10.0);
        let (assign34640_e57786, assign34640_e57786_d_n0, assign34640_e57786_d_n2, assign34640_e57786_d_n3, assign34640_e57786_d_n4, assign34640_e57786_d_n5, assign34640_e57786_d_n6, assign34640_e57786_d_n7, assign34640_e57786_d_n8, assign34640_e57786_d_n9, assign34640_e57786_d_n10, assign34640_e57786_d_n11, assign34640_e57786_d_n13, assign34640_e57786_d_n14,) = {
            if (!(locals.var_vsatrsd_eff < assign34640_e57755)) {
                let assign34640_e57762: f64 = (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff);
                let assign34640_e57765: f64 = (4.0 * 10.0);
                let assign34640_e57767: f64 = (assign34640_e57765 * 10.0);
                let assign34640_e57768: f64 = (assign34640_e57762 + assign34640_e57767);
                let assign34640_e57769: f64 = (assign34640_e57768).sqrt();
                let assign34640_e57770: f64 = (locals.var_vsatrsd_eff + assign34640_e57769);
                let assign34640_e57771: f64 = (0.5 * assign34640_e57770);
                (assign34640_e57771, (0.5 * (locals.var_vsatrsd_eff_dn0 + (((locals.var_vsatrsd_eff_dn0 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn0)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn2 + (((locals.var_vsatrsd_eff_dn2 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn2)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn3 + (((locals.var_vsatrsd_eff_dn3 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn3)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn4 + (((locals.var_vsatrsd_eff_dn4 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn4)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn5 + (((locals.var_vsatrsd_eff_dn5 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn5)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn6 + (((locals.var_vsatrsd_eff_dn6 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn6)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn7 + (((locals.var_vsatrsd_eff_dn7 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn7)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn8 + (((locals.var_vsatrsd_eff_dn8 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn8)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn9 + (((locals.var_vsatrsd_eff_dn9 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn9)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn10 + (((locals.var_vsatrsd_eff_dn10 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn10)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn11 + (((locals.var_vsatrsd_eff_dn11 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn11)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn13 + (((locals.var_vsatrsd_eff_dn13 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn13)) / (2.0 * assign34640_e57769)))), (0.5 * (locals.var_vsatrsd_eff_dn14 + (((locals.var_vsatrsd_eff_dn14 * locals.var_vsatrsd_eff) + (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff_dn14)) / (2.0 * assign34640_e57769)))),)
            } else {
                let assign34640_e57774: f64 = (-10000.0);
                let assign34640_e57776: f64 = (assign34640_e57774 * 10.0);
                let (assign34640_e57785, assign34640_e57785_d_n0, assign34640_e57785_d_n2, assign34640_e57785_d_n3, assign34640_e57785_d_n4, assign34640_e57785_d_n5, assign34640_e57785_d_n6, assign34640_e57785_d_n7, assign34640_e57785_d_n8, assign34640_e57785_d_n9, assign34640_e57785_d_n10, assign34640_e57785_d_n11, assign34640_e57785_d_n13, assign34640_e57785_d_n14,) = {
                    if (locals.var_vsatrsd_eff < assign34640_e57776) {
                        let assign34640_e57779: f64 = (-10.0);
                        let assign34640_e57781: f64 = (assign34640_e57779 * 10.0);
                        let assign34640_e57783: f64 = (assign34640_e57781 / locals.var_vsatrsd_eff);
                        (assign34640_e57783, (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn0) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn2) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn3) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn4) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn5) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn6) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn7) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn8) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn9) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn10) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn11) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn13) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))), (-((assign34640_e57781 * locals.var_vsatrsd_eff_dn14) / (locals.var_vsatrsd_eff * locals.var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34640_e57785, assign34640_e57785_d_n0, assign34640_e57785_d_n2, assign34640_e57785_d_n3, assign34640_e57785_d_n4, assign34640_e57785_d_n5, assign34640_e57785_d_n6, assign34640_e57785_d_n7, assign34640_e57785_d_n8, assign34640_e57785_d_n9, assign34640_e57785_d_n10, assign34640_e57785_d_n11, assign34640_e57785_d_n13, assign34640_e57785_d_n14,)
            }
        };
        (assign34640_e57786, assign34640_e57786_d_n0, assign34640_e57786_d_n2, assign34640_e57786_d_n3, assign34640_e57786_d_n4, assign34640_e57786_d_n5, assign34640_e57786_d_n6, assign34640_e57786_d_n7, assign34640_e57786_d_n8, assign34640_e57786_d_n9, assign34640_e57786_d_n10, assign34640_e57786_d_n11, assign34640_e57786_d_n13, assign34640_e57786_d_n14,)
    } else {
        (locals.var_vsatrsd_eff, locals.var_vsatrsd_eff_dn0, locals.var_vsatrsd_eff_dn2, locals.var_vsatrsd_eff_dn3, locals.var_vsatrsd_eff_dn4, locals.var_vsatrsd_eff_dn5, locals.var_vsatrsd_eff_dn6, locals.var_vsatrsd_eff_dn7, locals.var_vsatrsd_eff_dn8, locals.var_vsatrsd_eff_dn9, locals.var_vsatrsd_eff_dn10, locals.var_vsatrsd_eff_dn11, locals.var_vsatrsd_eff_dn13, locals.var_vsatrsd_eff_dn14,)
    }
};
        locals.var_vsatrsd_eff = assign34640_e57788;
        locals.var_vsatrsd_eff_dn0 = assign34640_e57788_d_n0;
        locals.var_vsatrsd_eff_dn2 = assign34640_e57788_d_n2;
        locals.var_vsatrsd_eff_dn3 = assign34640_e57788_d_n3;
        locals.var_vsatrsd_eff_dn4 = assign34640_e57788_d_n4;
        locals.var_vsatrsd_eff_dn5 = assign34640_e57788_d_n5;
        locals.var_vsatrsd_eff_dn6 = assign34640_e57788_d_n6;
        locals.var_vsatrsd_eff_dn7 = assign34640_e57788_d_n7;
        locals.var_vsatrsd_eff_dn8 = assign34640_e57788_d_n8;
        locals.var_vsatrsd_eff_dn9 = assign34640_e57788_d_n9;
        locals.var_vsatrsd_eff_dn10 = assign34640_e57788_d_n10;
        locals.var_vsatrsd_eff_dn11 = assign34640_e57788_d_n11;
        locals.var_vsatrsd_eff_dn13 = assign34640_e57788_d_n13;
        locals.var_vsatrsd_eff_dn14 = assign34640_e57788_d_n14;
        locals.var_vsatrsd_eff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_135(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign34650_e57802, assign34650_e57802_d_n0, assign34650_e57802_d_n2, assign34650_e57802_d_n3, assign34650_e57802_d_n4, assign34650_e57802_d_n5, assign34650_e57802_d_n6, assign34650_e57802_d_n7, assign34650_e57802_d_n8, assign34650_e57802_d_n9, assign34650_e57802_d_n10, assign34650_e57802_d_n11, assign34650_e57802_d_n13, assign34650_e57802_d_n14,) = {
    if (((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign34650_e57796: f64 = (locals.var_nfintotal * locals.var_weff0);
        let assign34650_e57798: f64 = (assign34650_e57796 * 1.60219e-19);
        let assign34650_e57800: f64 = (assign34650_e57798 * locals.var_vsatrsd_eff);
        (assign34650_e57800, (assign34650_e57798 * locals.var_vsatrsd_eff_dn0), (assign34650_e57798 * locals.var_vsatrsd_eff_dn2), (assign34650_e57798 * locals.var_vsatrsd_eff_dn3), (assign34650_e57798 * locals.var_vsatrsd_eff_dn4), (assign34650_e57798 * locals.var_vsatrsd_eff_dn5), (assign34650_e57798 * locals.var_vsatrsd_eff_dn6), (assign34650_e57798 * locals.var_vsatrsd_eff_dn7), (assign34650_e57798 * locals.var_vsatrsd_eff_dn8), (assign34650_e57798 * locals.var_vsatrsd_eff_dn9), (assign34650_e57798 * locals.var_vsatrsd_eff_dn10), (assign34650_e57798 * locals.var_vsatrsd_eff_dn11), (assign34650_e57798 * locals.var_vsatrsd_eff_dn13), (assign34650_e57798 * locals.var_vsatrsd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34650_e57802;
        locals.var_t2_dn0 = assign34650_e57802_d_n0;
        locals.var_t2_dn2 = assign34650_e57802_d_n2;
        locals.var_t2_dn3 = assign34650_e57802_d_n3;
        locals.var_t2_dn4 = assign34650_e57802_d_n4;
        locals.var_t2_dn5 = assign34650_e57802_d_n5;
        locals.var_t2_dn6 = assign34650_e57802_d_n6;
        locals.var_t2_dn7 = assign34650_e57802_d_n7;
        locals.var_t2_dn8 = assign34650_e57802_d_n8;
        locals.var_t2_dn9 = assign34650_e57802_d_n9;
        locals.var_t2_dn10 = assign34650_e57802_d_n10;
        locals.var_t2_dn11 = assign34650_e57802_d_n11;
        locals.var_t2_dn13 = assign34650_e57802_d_n13;
        locals.var_t2_dn14 = assign34650_e57802_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34660_e57810, assign34660_e57810_d_n0, assign34660_e57810_d_n2, assign34660_e57810_d_n3, assign34660_e57810_d_n4, assign34660_e57810_d_n5, assign34660_e57810_d_n6, assign34660_e57810_d_n7, assign34660_e57810_d_n8, assign34660_e57810_d_n9, assign34660_e57810_d_n10, assign34660_e57810_d_n11, assign34660_e57810_d_n13, assign34660_e57810_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34660_e57808: f64 = (locals.var_t2 * p.p1909);
        (assign34660_e57808, (locals.var_t2_dn0 * p.p1909), (locals.var_t2_dn2 * p.p1909), (locals.var_t2_dn3 * p.p1909), (locals.var_t2_dn4 * p.p1909), (locals.var_t2_dn5 * p.p1909), (locals.var_t2_dn6 * p.p1909), (locals.var_t2_dn7 * p.p1909), (locals.var_t2_dn8 * p.p1909), (locals.var_t2_dn9 * p.p1909), (locals.var_t2_dn10 * p.p1909), (locals.var_t2_dn11 * p.p1909), (locals.var_t2_dn13 * p.p1909), (locals.var_t2_dn14 * p.p1909),)
    } else {
        (locals.var_isat_rs, locals.var_isat_rs_dn0, locals.var_isat_rs_dn2, locals.var_isat_rs_dn3, locals.var_isat_rs_dn4, locals.var_isat_rs_dn5, locals.var_isat_rs_dn6, locals.var_isat_rs_dn7, locals.var_isat_rs_dn8, locals.var_isat_rs_dn9, locals.var_isat_rs_dn10, locals.var_isat_rs_dn11, locals.var_isat_rs_dn13, locals.var_isat_rs_dn14,)
    }
};
        locals.var_isat_rs = assign34660_e57810;
        locals.var_isat_rs_dn0 = assign34660_e57810_d_n0;
        locals.var_isat_rs_dn2 = assign34660_e57810_d_n2;
        locals.var_isat_rs_dn3 = assign34660_e57810_d_n3;
        locals.var_isat_rs_dn4 = assign34660_e57810_d_n4;
        locals.var_isat_rs_dn5 = assign34660_e57810_d_n5;
        locals.var_isat_rs_dn6 = assign34660_e57810_d_n6;
        locals.var_isat_rs_dn7 = assign34660_e57810_d_n7;
        locals.var_isat_rs_dn8 = assign34660_e57810_d_n8;
        locals.var_isat_rs_dn9 = assign34660_e57810_d_n9;
        locals.var_isat_rs_dn10 = assign34660_e57810_d_n10;
        locals.var_isat_rs_dn11 = assign34660_e57810_d_n11;
        locals.var_isat_rs_dn13 = assign34660_e57810_d_n13;
        locals.var_isat_rs_dn14 = assign34660_e57810_d_n14;
        locals.var_isat_rs_rv = 0.0;

        let (assign34670_e57820, assign34670_e57820_d_n0, assign34670_e57820_d_n2, assign34670_e57820_d_n3, assign34670_e57820_d_n4, assign34670_e57820_d_n5, assign34670_e57820_d_n6, assign34670_e57820_d_n7, assign34670_e57820_d_n8, assign34670_e57820_d_n9, assign34670_e57820_d_n10, assign34670_e57820_d_n11, assign34670_e57820_d_n13, assign34670_e57820_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34670_e57816: f64 = (locals.var_rdstempvs * p.p1911);
        let assign34670_e57818: f64 = (assign34670_e57816 * locals.var_weffwrfactor);
        (assign34670_e57818, 0.0, 0.0, 0.0, ((locals.var_rdstempvs_dn4 * p.p1911) * locals.var_weffwrfactor), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34670_e57820;
        locals.var_t4_dn0 = assign34670_e57820_d_n0;
        locals.var_t4_dn2 = assign34670_e57820_d_n2;
        locals.var_t4_dn3 = assign34670_e57820_d_n3;
        locals.var_t4_dn4 = assign34670_e57820_d_n4;
        locals.var_t4_dn5 = assign34670_e57820_d_n5;
        locals.var_t4_dn6 = assign34670_e57820_d_n6;
        locals.var_t4_dn7 = assign34670_e57820_d_n7;
        locals.var_t4_dn8 = assign34670_e57820_d_n8;
        locals.var_t4_dn9 = assign34670_e57820_d_n9;
        locals.var_t4_dn10 = assign34670_e57820_d_n10;
        locals.var_t4_dn11 = assign34670_e57820_d_n11;
        locals.var_t4_dn13 = assign34670_e57820_d_n13;
        locals.var_t4_dn14 = assign34670_e57820_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign34680_e57828, assign34680_e57828_d_n0, assign34680_e57828_d_n2, assign34680_e57828_d_n3, assign34680_e57828_d_n4, assign34680_e57828_d_n5, assign34680_e57828_d_n6, assign34680_e57828_d_n7, assign34680_e57828_d_n8, assign34680_e57828_d_n9, assign34680_e57828_d_n10, assign34680_e57828_d_n11, assign34680_e57828_d_n13, assign34680_e57828_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34680_e57826: f64 = (locals.var_isat_rs * locals.var_t4);
        (assign34680_e57826, ((locals.var_isat_rs_dn0 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn0)), ((locals.var_isat_rs_dn2 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn2)), ((locals.var_isat_rs_dn3 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn3)), ((locals.var_isat_rs_dn4 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn4)), ((locals.var_isat_rs_dn5 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn5)), ((locals.var_isat_rs_dn6 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn6)), ((locals.var_isat_rs_dn7 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn7)), ((locals.var_isat_rs_dn8 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn8)), ((locals.var_isat_rs_dn9 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn9)), ((locals.var_isat_rs_dn10 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn10)), ((locals.var_isat_rs_dn11 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn11)), ((locals.var_isat_rs_dn13 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn13)), ((locals.var_isat_rs_dn14 * locals.var_t4) + (locals.var_isat_rs * locals.var_t4_dn14)),)
    } else {
        (locals.var_vsat_rs, locals.var_vsat_rs_dn0, locals.var_vsat_rs_dn2, locals.var_vsat_rs_dn3, locals.var_vsat_rs_dn4, locals.var_vsat_rs_dn5, locals.var_vsat_rs_dn6, locals.var_vsat_rs_dn7, locals.var_vsat_rs_dn8, locals.var_vsat_rs_dn9, locals.var_vsat_rs_dn10, locals.var_vsat_rs_dn11, locals.var_vsat_rs_dn13, locals.var_vsat_rs_dn14,)
    }
};
        locals.var_vsat_rs = assign34680_e57828;
        locals.var_vsat_rs_dn0 = assign34680_e57828_d_n0;
        locals.var_vsat_rs_dn2 = assign34680_e57828_d_n2;
        locals.var_vsat_rs_dn3 = assign34680_e57828_d_n3;
        locals.var_vsat_rs_dn4 = assign34680_e57828_d_n4;
        locals.var_vsat_rs_dn5 = assign34680_e57828_d_n5;
        locals.var_vsat_rs_dn6 = assign34680_e57828_d_n6;
        locals.var_vsat_rs_dn7 = assign34680_e57828_d_n7;
        locals.var_vsat_rs_dn8 = assign34680_e57828_d_n8;
        locals.var_vsat_rs_dn9 = assign34680_e57828_d_n9;
        locals.var_vsat_rs_dn10 = assign34680_e57828_d_n10;
        locals.var_vsat_rs_dn11 = assign34680_e57828_d_n11;
        locals.var_vsat_rs_dn13 = assign34680_e57828_d_n13;
        locals.var_vsat_rs_dn14 = assign34680_e57828_d_n14;
        locals.var_vsat_rs_rv = 0.0;

        let (assign34690_e57835, assign34690_e57835_d_n0, assign34690_e57835_d_n2, assign34690_e57835_d_n3, assign34690_e57835_d_n4, assign34690_e57835_d_n5, assign34690_e57835_d_n6, assign34690_e57835_d_n7, assign34690_e57835_d_n8, assign34690_e57835_d_n9, assign34690_e57835_d_n10, assign34690_e57835_d_n11, assign34690_e57835_d_n13, assign34690_e57835_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34690_e57833: f64 = ((nv6 - nv8)).abs();
        (assign34690_e57833, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv6 - nv8) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, if (nv6 - nv8) >= 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34690_e57835;
        locals.var_t5_dn0 = assign34690_e57835_d_n0;
        locals.var_t5_dn2 = assign34690_e57835_d_n2;
        locals.var_t5_dn3 = assign34690_e57835_d_n3;
        locals.var_t5_dn4 = assign34690_e57835_d_n4;
        locals.var_t5_dn5 = assign34690_e57835_d_n5;
        locals.var_t5_dn6 = assign34690_e57835_d_n6;
        locals.var_t5_dn7 = assign34690_e57835_d_n7;
        locals.var_t5_dn8 = assign34690_e57835_d_n8;
        locals.var_t5_dn9 = assign34690_e57835_d_n9;
        locals.var_t5_dn10 = assign34690_e57835_d_n10;
        locals.var_t5_dn11 = assign34690_e57835_d_n11;
        locals.var_t5_dn13 = assign34690_e57835_d_n13;
        locals.var_t5_dn14 = assign34690_e57835_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34700_e57859, assign34700_e57859_d_n0, assign34700_e57859_d_n2, assign34700_e57859_d_n3, assign34700_e57859_d_n4, assign34700_e57859_d_n5, assign34700_e57859_d_n6, assign34700_e57859_d_n7, assign34700_e57859_d_n8, assign34700_e57859_d_n9, assign34700_e57859_d_n10, assign34700_e57859_d_n11, assign34700_e57859_d_n13, assign34700_e57859_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34700_e57842: f64 = (4.0 - p.p1908);
        let assign34700_e57843: f64 = (locals.var_t5).powf(assign34700_e57842);
        let assign34700_e57847: f64 = (4.0 - p.p1908);
        let assign34700_e57848: f64 = (locals.var_t5).powf(assign34700_e57847);
        let assign34700_e57853: f64 = (4.0 - p.p1908);
        let assign34700_e57854: f64 = (locals.var_vsat_rs).powf(assign34700_e57853);
        let assign34700_e57855: f64 = (p.p1915 * assign34700_e57854);
        let assign34700_e57856: f64 = (assign34700_e57848 + assign34700_e57855);
        let assign34700_e57857: f64 = (assign34700_e57843 / assign34700_e57856);
        (assign34700_e57857, (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn0)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn0 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn0)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn0 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn0)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn0 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn2)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn2 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn2)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn2 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn2)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn2 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn3)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn3 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn3)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn3 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn3)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn3 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn4)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn4 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn4)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn4 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn4)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn4 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn5)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn5 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn5)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn5 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn5)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn5 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn6)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn6 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn6)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn6 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn6)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn6 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn7)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn7 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn7)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn7 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn7)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn7 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn8)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn8 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn8)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn8 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn8)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn8 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn9)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn9 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn9)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn9 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn9)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn9 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn10)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn10 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn10)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn10 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn10)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn10 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn11)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn11 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn11)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn11 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn11)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn11 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn13)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn13 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn13)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn13 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn13)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn13 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((locals.var_t5).powf(assign34700_e57842 - 1.0) * locals.var_t5_dn14)) } } else { (assign34700_e57843 * (assign34700_e57842 * (locals.var_t5_dn14 / locals.var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((locals.var_t5).powf(assign34700_e57847 - 1.0) * locals.var_t5_dn14)) } } else { (assign34700_e57848 * (assign34700_e57847 * (locals.var_t5_dn14 / locals.var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((locals.var_vsat_rs).powf(assign34700_e57853 - 1.0) * locals.var_vsat_rs_dn14)) } } else { (assign34700_e57854 * (assign34700_e57853 * (locals.var_vsat_rs_dn14 / locals.var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)),)
    } else {
        (locals.var_delta_vsrs, locals.var_delta_vsrs_dn0, locals.var_delta_vsrs_dn2, locals.var_delta_vsrs_dn3, locals.var_delta_vsrs_dn4, locals.var_delta_vsrs_dn5, locals.var_delta_vsrs_dn6, locals.var_delta_vsrs_dn7, locals.var_delta_vsrs_dn8, locals.var_delta_vsrs_dn9, locals.var_delta_vsrs_dn10, locals.var_delta_vsrs_dn11, locals.var_delta_vsrs_dn13, locals.var_delta_vsrs_dn14,)
    }
};
        locals.var_delta_vsrs = assign34700_e57859;
        locals.var_delta_vsrs_dn0 = assign34700_e57859_d_n0;
        locals.var_delta_vsrs_dn2 = assign34700_e57859_d_n2;
        locals.var_delta_vsrs_dn3 = assign34700_e57859_d_n3;
        locals.var_delta_vsrs_dn4 = assign34700_e57859_d_n4;
        locals.var_delta_vsrs_dn5 = assign34700_e57859_d_n5;
        locals.var_delta_vsrs_dn6 = assign34700_e57859_d_n6;
        locals.var_delta_vsrs_dn7 = assign34700_e57859_d_n7;
        locals.var_delta_vsrs_dn8 = assign34700_e57859_d_n8;
        locals.var_delta_vsrs_dn9 = assign34700_e57859_d_n9;
        locals.var_delta_vsrs_dn10 = assign34700_e57859_d_n10;
        locals.var_delta_vsrs_dn11 = assign34700_e57859_d_n11;
        locals.var_delta_vsrs_dn13 = assign34700_e57859_d_n13;
        locals.var_delta_vsrs_dn14 = assign34700_e57859_d_n14;
        locals.var_delta_vsrs_rv = 0.0;

        let (assign34710_e57873, assign34710_e57873_d_n0, assign34710_e57873_d_n2, assign34710_e57873_d_n3, assign34710_e57873_d_n4, assign34710_e57873_d_n5, assign34710_e57873_d_n6, assign34710_e57873_d_n7, assign34710_e57873_d_n8, assign34710_e57873_d_n9, assign34710_e57873_d_n10, assign34710_e57873_d_n11, assign34710_e57873_d_n13, assign34710_e57873_d_n14,) = {
    if ((locals.var_guard657 != 0.0) && (locals.var_guard661 != 0.0)) {
        let assign34710_e57866: f64 = (1.0 / p.p1908);
        let assign34710_e57867: f64 = (locals.var_delta_vsrs).powf(assign34710_e57866);
        let assign34710_e57869: f64 = (assign34710_e57867 * locals.var_t5);
        let assign34710_e57871: f64 = (assign34710_e57869 / locals.var_vsat_rs);
        (assign34710_e57871, (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn0)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn0 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn0)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn0)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn2)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn2 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn2)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn2)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn3)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn3 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn3)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn3)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn4)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn4 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn4)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn4)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn5)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn5 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn5)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn5)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn6)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn6 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn6)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn6)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn7)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn7 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn7)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn7)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn8)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn8 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn8)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn8)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn9)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn9 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn9)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn9)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn10)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn10 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn10)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn10)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn11)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn11 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn11)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn11)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn13)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn13 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn13)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn13)) / (locals.var_vsat_rs * locals.var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((locals.var_delta_vsrs).powf(assign34710_e57866 - 1.0) * locals.var_delta_vsrs_dn14)) } } else { (assign34710_e57867 * (assign34710_e57866 * (locals.var_delta_vsrs_dn14 / locals.var_delta_vsrs))) } * locals.var_t5) + (assign34710_e57867 * locals.var_t5_dn14)) * locals.var_vsat_rs) - (assign34710_e57869 * locals.var_vsat_rs_dn14)) / (locals.var_vsat_rs * locals.var_vsat_rs)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34710_e57873;
        locals.var_t6_dn0 = assign34710_e57873_d_n0;
        locals.var_t6_dn2 = assign34710_e57873_d_n2;
        locals.var_t6_dn3 = assign34710_e57873_d_n3;
        locals.var_t6_dn4 = assign34710_e57873_d_n4;
        locals.var_t6_dn5 = assign34710_e57873_d_n5;
        locals.var_t6_dn6 = assign34710_e57873_d_n6;
        locals.var_t6_dn7 = assign34710_e57873_d_n7;
        locals.var_t6_dn8 = assign34710_e57873_d_n8;
        locals.var_t6_dn9 = assign34710_e57873_d_n9;
        locals.var_t6_dn10 = assign34710_e57873_d_n10;
        locals.var_t6_dn11 = assign34710_e57873_d_n11;
        locals.var_t6_dn13 = assign34710_e57873_d_n13;
        locals.var_t6_dn14 = assign34710_e57873_d_n14;
        locals.var_t6_rv = 0.0;

        let assign34820_e57955: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign34820_e57955;
        locals.var_guard669_rv = 0.0;

        let assign34900_e57995: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign34900_e57995;
        locals.var_guard677_rv = 0.0;

        let assign34950_e58014: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard682 = assign34950_e58014;
        locals.var_guard682_rv = 0.0;

        let assign35580_e58822: f64 = (10.0 * locals.var_vtm);
        let assign35580_e58824: f64 = (assign35580_e58822 / locals.var_rc);
        let assign35580_e58827: f64 = (2.0 * locals.var_qbs);
        let assign35580_e58828: f64 = (assign35580_e58824 + assign35580_e58827);
        locals.var_q0 = assign35580_e58828;
        locals.var_q0_dn0 = 0.0;
        locals.var_q0_dn2 = 0.0;
        locals.var_q0_dn3 = 0.0;
        locals.var_q0_dn4 = ((10.0 * locals.var_vtm_dn4) / locals.var_rc);
        locals.var_q0_dn5 = 0.0;
        locals.var_q0_dn6 = 0.0;
        locals.var_q0_dn7 = 0.0;
        locals.var_q0_dn8 = 0.0;
        locals.var_q0_dn9 = 0.0;
        locals.var_q0_dn10 = 0.0;
        locals.var_q0_dn11 = 0.0;
        locals.var_q0_dn13 = 0.0;
        locals.var_q0_dn14 = 0.0;
        locals.var_q0_rv = 0.0;

        let assign35590_e58832: f64 = (locals.var_vtm + locals.var_q0);
        let assign35590_e58833: f64 = (locals.var_vtm * assign35590_e58832);
        locals.var_t1 = assign35590_e58833;
        locals.var_t1_dn0 = (locals.var_vtm * locals.var_q0_dn0);
        locals.var_t1_dn2 = (locals.var_vtm * locals.var_q0_dn2);
        locals.var_t1_dn3 = (locals.var_vtm * locals.var_q0_dn3);
        locals.var_t1_dn4 = ((locals.var_vtm_dn4 * assign35590_e58832) + (locals.var_vtm * (locals.var_vtm_dn4 + locals.var_q0_dn4)));
        locals.var_t1_dn5 = (locals.var_vtm * locals.var_q0_dn5);
        locals.var_t1_dn6 = (locals.var_vtm * locals.var_q0_dn6);
        locals.var_t1_dn7 = (locals.var_vtm * locals.var_q0_dn7);
        locals.var_t1_dn8 = (locals.var_vtm * locals.var_q0_dn8);
        locals.var_t1_dn9 = (locals.var_vtm * locals.var_q0_dn9);
        locals.var_t1_dn10 = (locals.var_vtm * locals.var_q0_dn10);
        locals.var_t1_dn11 = (locals.var_vtm * locals.var_q0_dn11);
        locals.var_t1_dn13 = (locals.var_vtm * locals.var_q0_dn13);
        locals.var_t1_dn14 = (locals.var_vtm * locals.var_q0_dn14);
        locals.var_t1_rv = 0.0;

        let assign35600_e58836: f64 = (locals.var_cox * locals.var_cox);
        let assign35600_e58838: f64 = (assign35600_e58836 * locals.var_t1);
        locals.var_t2 = assign35600_e58838;
        locals.var_t2_dn0 = (assign35600_e58836 * locals.var_t1_dn0);
        locals.var_t2_dn2 = (assign35600_e58836 * locals.var_t1_dn2);
        locals.var_t2_dn3 = (assign35600_e58836 * locals.var_t1_dn3);
        locals.var_t2_dn4 = (assign35600_e58836 * locals.var_t1_dn4);
        locals.var_t2_dn5 = (assign35600_e58836 * locals.var_t1_dn5);
        locals.var_t2_dn6 = (assign35600_e58836 * locals.var_t1_dn6);
        locals.var_t2_dn7 = (assign35600_e58836 * locals.var_t1_dn7);
        locals.var_t2_dn8 = (assign35600_e58836 * locals.var_t1_dn8);
        locals.var_t2_dn9 = (assign35600_e58836 * locals.var_t1_dn9);
        locals.var_t2_dn10 = (assign35600_e58836 * locals.var_t1_dn10);
        locals.var_t2_dn11 = (assign35600_e58836 * locals.var_t1_dn11);
        locals.var_t2_dn13 = (assign35600_e58836 * locals.var_t1_dn13);
        locals.var_t2_dn14 = (assign35600_e58836 * locals.var_t1_dn14);
        locals.var_t2_rv = 0.0;

        let assign35610_e58841: f64 = (2.0 * 1.60219e-19);
        let assign35610_e58843: f64 = (assign35610_e58841 * locals.var_ni);
        let assign35610_e58845: f64 = (assign35610_e58843 * locals.var_epssub);
        let assign35610_e58847: f64 = (assign35610_e58845 * locals.var_vtm);
        locals.var_t3 = assign35610_e58847;
        locals.var_t3_dn0 = (((assign35610_e58841 * locals.var_ni_dn0) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn2 = (((assign35610_e58841 * locals.var_ni_dn2) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn3 = (((assign35610_e58841 * locals.var_ni_dn3) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn4 = ((((assign35610_e58841 * locals.var_ni_dn4) * locals.var_epssub) * locals.var_vtm) + (assign35610_e58845 * locals.var_vtm_dn4));
        locals.var_t3_dn5 = (((assign35610_e58841 * locals.var_ni_dn5) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn6 = (((assign35610_e58841 * locals.var_ni_dn6) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn7 = (((assign35610_e58841 * locals.var_ni_dn7) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn8 = (((assign35610_e58841 * locals.var_ni_dn8) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn9 = (((assign35610_e58841 * locals.var_ni_dn9) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn10 = (((assign35610_e58841 * locals.var_ni_dn10) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn11 = (((assign35610_e58841 * locals.var_ni_dn11) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn13 = (((assign35610_e58841 * locals.var_ni_dn13) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_dn14 = (((assign35610_e58841 * locals.var_ni_dn14) * locals.var_epssub) * locals.var_vtm);
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14,) = {
    if (locals.var_guard642 == 0.0) {
        let eq4_e1976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qd_v);
        let eq4_e1977: f64 = (locals.var_devsign * eq4_e1976);
        let eq4_e1977_d_n0: f64 = (locals.var_devsign * (locals.var_qd_v_dn0 * ddt_scale));
        let eq4_e1977_d_n2: f64 = (locals.var_devsign * (locals.var_qd_v_dn2 * ddt_scale));
        let eq4_e1977_d_n3: f64 = (locals.var_devsign * (locals.var_qd_v_dn3 * ddt_scale));
        let eq4_e1977_d_n4: f64 = (locals.var_devsign * (locals.var_qd_v_dn4 * ddt_scale));
        let eq4_e1977_d_n5: f64 = (locals.var_devsign * (locals.var_qd_v_dn5 * ddt_scale));
        let eq4_e1977_d_n6: f64 = (locals.var_devsign * (locals.var_qd_v_dn6 * ddt_scale));
        let eq4_e1977_d_n7: f64 = (locals.var_devsign * (locals.var_qd_v_dn7 * ddt_scale));
        let eq4_e1977_d_n8: f64 = (locals.var_devsign * (locals.var_qd_v_dn8 * ddt_scale));
        let eq4_e1977_d_n9: f64 = (locals.var_devsign * (locals.var_qd_v_dn9 * ddt_scale));
        let eq4_e1977_d_n10: f64 = (locals.var_devsign * (locals.var_qd_v_dn10 * ddt_scale));
        let eq4_e1977_d_n11: f64 = (locals.var_devsign * (locals.var_qd_v_dn11 * ddt_scale));
        let eq4_e1977_d_n13: f64 = (locals.var_devsign * (locals.var_qd_v_dn13 * ddt_scale));
        let eq4_e1977_d_n14: f64 = (locals.var_devsign * (locals.var_qd_v_dn14 * ddt_scale));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n13, eq4_e1977_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1979;
        let eq4_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq4_node_derivatives: [f64; 13] = [eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14,) = {
    if (locals.var_guard642 == 0.0) {
        let eq5_e1984: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qg_v);
        let eq5_e1985: f64 = (locals.var_devsign * eq5_e1984);
        let eq5_e1985_d_n0: f64 = (locals.var_devsign * (locals.var_qg_v_dn0 * ddt_scale));
        let eq5_e1985_d_n2: f64 = (locals.var_devsign * (locals.var_qg_v_dn2 * ddt_scale));
        let eq5_e1985_d_n3: f64 = (locals.var_devsign * (locals.var_qg_v_dn3 * ddt_scale));
        let eq5_e1985_d_n4: f64 = (locals.var_devsign * (locals.var_qg_v_dn4 * ddt_scale));
        let eq5_e1985_d_n5: f64 = (locals.var_devsign * (locals.var_qg_v_dn5 * ddt_scale));
        let eq5_e1985_d_n6: f64 = (locals.var_devsign * (locals.var_qg_v_dn6 * ddt_scale));
        let eq5_e1985_d_n7: f64 = (locals.var_devsign * (locals.var_qg_v_dn7 * ddt_scale));
        let eq5_e1985_d_n8: f64 = (locals.var_devsign * (locals.var_qg_v_dn8 * ddt_scale));
        let eq5_e1985_d_n9: f64 = (locals.var_devsign * (locals.var_qg_v_dn9 * ddt_scale));
        let eq5_e1985_d_n10: f64 = (locals.var_devsign * (locals.var_qg_v_dn10 * ddt_scale));
        let eq5_e1985_d_n11: f64 = (locals.var_devsign * (locals.var_qg_v_dn11 * ddt_scale));
        let eq5_e1985_d_n13: f64 = (locals.var_devsign * (locals.var_qg_v_dn13 * ddt_scale));
        let eq5_e1985_d_n14: f64 = (locals.var_devsign * (locals.var_qg_v_dn14 * ddt_scale));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n13, eq5_e1985_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1987;
        let eq5_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq5_node_derivatives: [f64; 13] = [eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qes);
        let eq36_e2282: f64 = (locals.var_devsign * eq36_e2281);
        let eq36_e2282_d_n0: f64 = (locals.var_devsign * (locals.var_qes_dn0 * ddt_scale));
        let eq36_e2282_d_n2: f64 = (locals.var_devsign * (locals.var_qes_dn2 * ddt_scale));
        let eq36_e2282_d_n3: f64 = (locals.var_devsign * (locals.var_qes_dn3 * ddt_scale));
        let eq36_e2282_d_n4: f64 = (locals.var_devsign * (locals.var_qes_dn4 * ddt_scale));
        let eq36_e2282_d_n5: f64 = (locals.var_devsign * (locals.var_qes_dn5 * ddt_scale));
        let eq36_e2282_d_n6: f64 = (locals.var_devsign * (locals.var_qes_dn6 * ddt_scale));
        let eq36_e2282_d_n7: f64 = (locals.var_devsign * (locals.var_qes_dn7 * ddt_scale));
        let eq36_e2282_d_n8: f64 = (locals.var_devsign * (locals.var_qes_dn8 * ddt_scale));
        let eq36_e2282_d_n9: f64 = (locals.var_devsign * (locals.var_qes_dn9 * ddt_scale));
        let eq36_e2282_d_n10: f64 = (locals.var_devsign * (locals.var_qes_dn10 * ddt_scale));
        let eq36_e2282_d_n11: f64 = (locals.var_devsign * (locals.var_qes_dn11 * ddt_scale));
        let eq36_e2282_d_n13: f64 = (locals.var_devsign * (locals.var_qes_dn13 * ddt_scale));
        let eq36_e2282_d_n14: f64 = (locals.var_devsign * (locals.var_qes_dn14 * ddt_scale));
        let eq36_value: f64 = eq36_e2282;
        let eq36_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq36_node_derivatives: [f64; 13] = [eq36_e2282_d_n0, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, eq36_e2282_d_n13, eq36_e2282_d_n14];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let eq37_e2285: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qed);
        let eq37_e2286: f64 = (locals.var_devsign * eq37_e2285);
        let eq37_e2286_d_n0: f64 = (locals.var_devsign * (locals.var_qed_dn0 * ddt_scale));
        let eq37_e2286_d_n2: f64 = (locals.var_devsign * (locals.var_qed_dn2 * ddt_scale));
        let eq37_e2286_d_n3: f64 = (locals.var_devsign * (locals.var_qed_dn3 * ddt_scale));
        let eq37_e2286_d_n4: f64 = (locals.var_devsign * (locals.var_qed_dn4 * ddt_scale));
        let eq37_e2286_d_n5: f64 = (locals.var_devsign * (locals.var_qed_dn5 * ddt_scale));
        let eq37_e2286_d_n6: f64 = (locals.var_devsign * (locals.var_qed_dn6 * ddt_scale));
        let eq37_e2286_d_n7: f64 = (locals.var_devsign * (locals.var_qed_dn7 * ddt_scale));
        let eq37_e2286_d_n8: f64 = (locals.var_devsign * (locals.var_qed_dn8 * ddt_scale));
        let eq37_e2286_d_n9: f64 = (locals.var_devsign * (locals.var_qed_dn9 * ddt_scale));
        let eq37_e2286_d_n10: f64 = (locals.var_devsign * (locals.var_qed_dn10 * ddt_scale));
        let eq37_e2286_d_n11: f64 = (locals.var_devsign * (locals.var_qed_dn11 * ddt_scale));
        let eq37_e2286_d_n13: f64 = (locals.var_devsign * (locals.var_qed_dn13 * ddt_scale));
        let eq37_e2286_d_n14: f64 = (locals.var_devsign * (locals.var_qed_dn14 * ddt_scale));
        let eq37_value: f64 = eq37_e2286;
        let eq37_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq37_node_derivatives: [f64; 13] = [eq37_e2286_d_n0, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, eq37_e2286_d_n13, eq37_e2286_d_n14];
        let eq37_branch_derivative_indices: [usize; 0] = [];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq37_value),
            &eq37_node_derivative_indices,
            &eq37_node_derivatives,
            &eq37_branch_derivative_indices,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qeg);
        let eq38_e2290: f64 = (locals.var_devsign * eq38_e2289);
        let eq38_e2290_d_n0: f64 = (locals.var_devsign * (locals.var_qeg_dn0 * ddt_scale));
        let eq38_e2290_d_n2: f64 = (locals.var_devsign * (locals.var_qeg_dn2 * ddt_scale));
        let eq38_e2290_d_n3: f64 = (locals.var_devsign * (locals.var_qeg_dn3 * ddt_scale));
        let eq38_e2290_d_n4: f64 = (locals.var_devsign * (locals.var_qeg_dn4 * ddt_scale));
        let eq38_e2290_d_n5: f64 = (locals.var_devsign * (locals.var_qeg_dn5 * ddt_scale));
        let eq38_e2290_d_n6: f64 = (locals.var_devsign * (locals.var_qeg_dn6 * ddt_scale));
        let eq38_e2290_d_n7: f64 = (locals.var_devsign * (locals.var_qeg_dn7 * ddt_scale));
        let eq38_e2290_d_n8: f64 = (locals.var_devsign * (locals.var_qeg_dn8 * ddt_scale));
        let eq38_e2290_d_n9: f64 = (locals.var_devsign * (locals.var_qeg_dn9 * ddt_scale));
        let eq38_e2290_d_n10: f64 = (locals.var_devsign * (locals.var_qeg_dn10 * ddt_scale));
        let eq38_e2290_d_n11: f64 = (locals.var_devsign * (locals.var_qeg_dn11 * ddt_scale));
        let eq38_e2290_d_n13: f64 = (locals.var_devsign * (locals.var_qeg_dn13 * ddt_scale));
        let eq38_e2290_d_n14: f64 = (locals.var_devsign * (locals.var_qeg_dn14 * ddt_scale));
        let eq38_value: f64 = eq38_e2290;
        let eq38_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq38_node_derivatives: [f64; 13] = [eq38_e2290_d_n0, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, eq38_e2290_d_n13, eq38_e2290_d_n14];
        let eq38_branch_derivative_indices: [usize; 0] = [];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq38_value),
            &eq38_node_derivative_indices,
            &eq38_node_derivatives,
            &eq38_branch_derivative_indices,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14,) = {
    if (locals.var_guard651 != 0.0) {
        let eq39_e2293: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qgs_parasitic);
        (eq39_e2293, (locals.var_qgs_parasitic_dn0 * ddt_scale), (locals.var_qgs_parasitic_dn2 * ddt_scale), (locals.var_qgs_parasitic_dn3 * ddt_scale), (locals.var_qgs_parasitic_dn4 * ddt_scale), (locals.var_qgs_parasitic_dn5 * ddt_scale), (locals.var_qgs_parasitic_dn6 * ddt_scale), (locals.var_qgs_parasitic_dn7 * ddt_scale), (locals.var_qgs_parasitic_dn8 * ddt_scale), (locals.var_qgs_parasitic_dn9 * ddt_scale), (locals.var_qgs_parasitic_dn10 * ddt_scale), (locals.var_qgs_parasitic_dn11 * ddt_scale), (locals.var_qgs_parasitic_dn13 * ddt_scale), (locals.var_qgs_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e2295;
        let eq39_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq39_node_derivatives: [f64; 13] = [eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq39_value),
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 != 0.0)) {
        let eq40_e2300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qgd_parasitic);
        (eq40_e2300, (locals.var_qgd_parasitic_dn0 * ddt_scale), (locals.var_qgd_parasitic_dn2 * ddt_scale), (locals.var_qgd_parasitic_dn3 * ddt_scale), (locals.var_qgd_parasitic_dn4 * ddt_scale), (locals.var_qgd_parasitic_dn5 * ddt_scale), (locals.var_qgd_parasitic_dn6 * ddt_scale), (locals.var_qgd_parasitic_dn7 * ddt_scale), (locals.var_qgd_parasitic_dn8 * ddt_scale), (locals.var_qgd_parasitic_dn9 * ddt_scale), (locals.var_qgd_parasitic_dn10 * ddt_scale), (locals.var_qgd_parasitic_dn11 * ddt_scale), (locals.var_qgd_parasitic_dn13 * ddt_scale), (locals.var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e2302;
        let eq40_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq40_node_derivatives: [f64; 13] = [eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 != 0.0)) {
        let eq41_e2308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbov);
        let eq41_e2309: f64 = (locals.var_devsign * eq41_e2308);
        let eq41_e2309_d_n0: f64 = (locals.var_devsign * (locals.var_qbov_dn0 * ddt_scale));
        let eq41_e2309_d_n2: f64 = (locals.var_devsign * (locals.var_qbov_dn2 * ddt_scale));
        let eq41_e2309_d_n3: f64 = (locals.var_devsign * (locals.var_qbov_dn3 * ddt_scale));
        let eq41_e2309_d_n4: f64 = (locals.var_devsign * (locals.var_qbov_dn4 * ddt_scale));
        let eq41_e2309_d_n5: f64 = (locals.var_devsign * (locals.var_qbov_dn5 * ddt_scale));
        let eq41_e2309_d_n6: f64 = (locals.var_devsign * (locals.var_qbov_dn6 * ddt_scale));
        let eq41_e2309_d_n7: f64 = (locals.var_devsign * (locals.var_qbov_dn7 * ddt_scale));
        let eq41_e2309_d_n8: f64 = (locals.var_devsign * (locals.var_qbov_dn8 * ddt_scale));
        let eq41_e2309_d_n9: f64 = (locals.var_devsign * (locals.var_qbov_dn9 * ddt_scale));
        let eq41_e2309_d_n10: f64 = (locals.var_devsign * (locals.var_qbov_dn10 * ddt_scale));
        let eq41_e2309_d_n11: f64 = (locals.var_devsign * (locals.var_qbov_dn11 * ddt_scale));
        let eq41_e2309_d_n13: f64 = (locals.var_devsign * (locals.var_qbov_dn13 * ddt_scale));
        let eq41_e2309_d_n14: f64 = (locals.var_devsign * (locals.var_qbov_dn14 * ddt_scale));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n13, eq41_e2309_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e2311;
        let eq41_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq41_node_derivatives: [f64; 13] = [eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 != 0.0)) {
        let eq42_e2317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, locals.var_qbov_s);
        let eq42_e2318: f64 = (locals.var_devsign * eq42_e2317);
        let eq42_e2318_d_n0: f64 = (locals.var_devsign * (locals.var_qbov_s_dn0 * ddt_scale));
        let eq42_e2318_d_n2: f64 = (locals.var_devsign * (locals.var_qbov_s_dn2 * ddt_scale));
        let eq42_e2318_d_n3: f64 = (locals.var_devsign * (locals.var_qbov_s_dn3 * ddt_scale));
        let eq42_e2318_d_n4: f64 = (locals.var_devsign * (locals.var_qbov_s_dn4 * ddt_scale));
        let eq42_e2318_d_n5: f64 = (locals.var_devsign * (locals.var_qbov_s_dn5 * ddt_scale));
        let eq42_e2318_d_n6: f64 = (locals.var_devsign * (locals.var_qbov_s_dn6 * ddt_scale));
        let eq42_e2318_d_n7: f64 = (locals.var_devsign * (locals.var_qbov_s_dn7 * ddt_scale));
        let eq42_e2318_d_n8: f64 = (locals.var_devsign * (locals.var_qbov_s_dn8 * ddt_scale));
        let eq42_e2318_d_n9: f64 = (locals.var_devsign * (locals.var_qbov_s_dn9 * ddt_scale));
        let eq42_e2318_d_n10: f64 = (locals.var_devsign * (locals.var_qbov_s_dn10 * ddt_scale));
        let eq42_e2318_d_n11: f64 = (locals.var_devsign * (locals.var_qbov_s_dn11 * ddt_scale));
        let eq42_e2318_d_n13: f64 = (locals.var_devsign * (locals.var_qbov_s_dn13 * ddt_scale));
        let eq42_e2318_d_n14: f64 = (locals.var_devsign * (locals.var_qbov_s_dn14 * ddt_scale));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n13, eq42_e2318_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e2320;
        let eq42_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq42_node_derivatives: [f64; 13] = [eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 == 0.0)) {
        let eq43_e2326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, locals.var_qgd_parasitic);
        (eq43_e2326, (locals.var_qgd_parasitic_dn0 * ddt_scale), (locals.var_qgd_parasitic_dn2 * ddt_scale), (locals.var_qgd_parasitic_dn3 * ddt_scale), (locals.var_qgd_parasitic_dn4 * ddt_scale), (locals.var_qgd_parasitic_dn5 * ddt_scale), (locals.var_qgd_parasitic_dn6 * ddt_scale), (locals.var_qgd_parasitic_dn7 * ddt_scale), (locals.var_qgd_parasitic_dn8 * ddt_scale), (locals.var_qgd_parasitic_dn9 * ddt_scale), (locals.var_qgd_parasitic_dn10 * ddt_scale), (locals.var_qgd_parasitic_dn11 * ddt_scale), (locals.var_qgd_parasitic_dn13 * ddt_scale), (locals.var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e2328;
        let eq43_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq43_node_derivatives: [f64; 13] = [eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n2,) = {
    if (locals.var_guard651 != 0.0) {
        let eq44_e2331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qds_fr);
        (eq44_e2331, (locals.var_qds_fr_dn0 * ddt_scale), (locals.var_qds_fr_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e2333;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq44_value),
            0,
            multiplicity * (eq44_e2333_d_n0),
            2,
            multiplicity * (eq44_e2333_d_n2),
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard653 != 0.0)) {
        let eq45_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qgs_fr);
        (eq45_e2338, (locals.var_qgs_fr_dn0 * ddt_scale), (locals.var_qgs_fr_dn2 * ddt_scale), (locals.var_qgs_fr_dn3 * ddt_scale), (locals.var_qgs_fr_dn4 * ddt_scale), (locals.var_qgs_fr_dn5 * ddt_scale), (locals.var_qgs_fr_dn6 * ddt_scale), (locals.var_qgs_fr_dn7 * ddt_scale), (locals.var_qgs_fr_dn8 * ddt_scale), (locals.var_qgs_fr_dn9 * ddt_scale), (locals.var_qgs_fr_dn10 * ddt_scale), (locals.var_qgs_fr_dn11 * ddt_scale), (locals.var_qgs_fr_dn13 * ddt_scale), (locals.var_qgs_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e2340;
        let eq45_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq45_node_derivatives: [f64; 13] = [eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14];
        let eq45_branch_derivative_indices: [usize; 0] = [];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq45_value),
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard653 != 0.0)) {
        let eq46_e2345: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qgd_fr);
        (eq46_e2345, (locals.var_qgd_fr_dn0 * ddt_scale), (locals.var_qgd_fr_dn2 * ddt_scale), (locals.var_qgd_fr_dn3 * ddt_scale), (locals.var_qgd_fr_dn4 * ddt_scale), (locals.var_qgd_fr_dn5 * ddt_scale), (locals.var_qgd_fr_dn6 * ddt_scale), (locals.var_qgd_fr_dn7 * ddt_scale), (locals.var_qgd_fr_dn8 * ddt_scale), (locals.var_qgd_fr_dn9 * ddt_scale), (locals.var_qgd_fr_dn10 * ddt_scale), (locals.var_qgd_fr_dn11 * ddt_scale), (locals.var_qgd_fr_dn13 * ddt_scale), (locals.var_qgd_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e2347;
        let eq46_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq46_node_derivatives: [f64; 13] = [eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14];
        let eq46_branch_derivative_indices: [usize; 0] = [];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq46_value),
            &eq46_node_derivative_indices,
            &eq46_node_derivatives,
            &eq46_branch_derivative_indices,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14,) = {
    if (locals.var_guard651 == 0.0) {
        let eq47_e2351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qgs_parasitic);
        (eq47_e2351, (locals.var_qgs_parasitic_dn0 * ddt_scale), (locals.var_qgs_parasitic_dn2 * ddt_scale), (locals.var_qgs_parasitic_dn3 * ddt_scale), (locals.var_qgs_parasitic_dn4 * ddt_scale), (locals.var_qgs_parasitic_dn5 * ddt_scale), (locals.var_qgs_parasitic_dn6 * ddt_scale), (locals.var_qgs_parasitic_dn7 * ddt_scale), (locals.var_qgs_parasitic_dn8 * ddt_scale), (locals.var_qgs_parasitic_dn9 * ddt_scale), (locals.var_qgs_parasitic_dn10 * ddt_scale), (locals.var_qgs_parasitic_dn11 * ddt_scale), (locals.var_qgs_parasitic_dn13 * ddt_scale), (locals.var_qgs_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e2353;
        let eq47_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq47_node_derivatives: [f64; 13] = [eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14];
        let eq47_branch_derivative_indices: [usize; 0] = [];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(6),
            multiplicity * (eq47_value),
            &eq47_node_derivative_indices,
            &eq47_node_derivatives,
            &eq47_branch_derivative_indices,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 != 0.0)) {
        let eq48_e2359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qgd_parasitic);
        (eq48_e2359, (locals.var_qgd_parasitic_dn0 * ddt_scale), (locals.var_qgd_parasitic_dn2 * ddt_scale), (locals.var_qgd_parasitic_dn3 * ddt_scale), (locals.var_qgd_parasitic_dn4 * ddt_scale), (locals.var_qgd_parasitic_dn5 * ddt_scale), (locals.var_qgd_parasitic_dn6 * ddt_scale), (locals.var_qgd_parasitic_dn7 * ddt_scale), (locals.var_qgd_parasitic_dn8 * ddt_scale), (locals.var_qgd_parasitic_dn9 * ddt_scale), (locals.var_qgd_parasitic_dn10 * ddt_scale), (locals.var_qgd_parasitic_dn11 * ddt_scale), (locals.var_qgd_parasitic_dn13 * ddt_scale), (locals.var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e2361;
        let eq48_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq48_node_derivatives: [f64; 13] = [eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14];
        let eq48_branch_derivative_indices: [usize; 0] = [];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivative_indices,
            &eq48_node_derivatives,
            &eq48_branch_derivative_indices,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 != 0.0)) {
        let eq49_e2368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, locals.var_qbov);
        let eq49_e2369: f64 = (locals.var_devsign * eq49_e2368);
        let eq49_e2369_d_n0: f64 = (locals.var_devsign * (locals.var_qbov_dn0 * ddt_scale));
        let eq49_e2369_d_n2: f64 = (locals.var_devsign * (locals.var_qbov_dn2 * ddt_scale));
        let eq49_e2369_d_n3: f64 = (locals.var_devsign * (locals.var_qbov_dn3 * ddt_scale));
        let eq49_e2369_d_n4: f64 = (locals.var_devsign * (locals.var_qbov_dn4 * ddt_scale));
        let eq49_e2369_d_n5: f64 = (locals.var_devsign * (locals.var_qbov_dn5 * ddt_scale));
        let eq49_e2369_d_n6: f64 = (locals.var_devsign * (locals.var_qbov_dn6 * ddt_scale));
        let eq49_e2369_d_n7: f64 = (locals.var_devsign * (locals.var_qbov_dn7 * ddt_scale));
        let eq49_e2369_d_n8: f64 = (locals.var_devsign * (locals.var_qbov_dn8 * ddt_scale));
        let eq49_e2369_d_n9: f64 = (locals.var_devsign * (locals.var_qbov_dn9 * ddt_scale));
        let eq49_e2369_d_n10: f64 = (locals.var_devsign * (locals.var_qbov_dn10 * ddt_scale));
        let eq49_e2369_d_n11: f64 = (locals.var_devsign * (locals.var_qbov_dn11 * ddt_scale));
        let eq49_e2369_d_n13: f64 = (locals.var_devsign * (locals.var_qbov_dn13 * ddt_scale));
        let eq49_e2369_d_n14: f64 = (locals.var_devsign * (locals.var_qbov_dn14 * ddt_scale));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n13, eq49_e2369_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e2371;
        let eq49_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq49_node_derivatives: [f64; 13] = [eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14];
        let eq49_branch_derivative_indices: [usize; 0] = [];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(7),
            multiplicity * (eq49_value),
            &eq49_node_derivative_indices,
            &eq49_node_derivatives,
            &eq49_branch_derivative_indices,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 != 0.0)) {
        let eq50_e2378: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, locals.var_qbov_s);
        let eq50_e2379: f64 = (locals.var_devsign * eq50_e2378);
        let eq50_e2379_d_n0: f64 = (locals.var_devsign * (locals.var_qbov_s_dn0 * ddt_scale));
        let eq50_e2379_d_n2: f64 = (locals.var_devsign * (locals.var_qbov_s_dn2 * ddt_scale));
        let eq50_e2379_d_n3: f64 = (locals.var_devsign * (locals.var_qbov_s_dn3 * ddt_scale));
        let eq50_e2379_d_n4: f64 = (locals.var_devsign * (locals.var_qbov_s_dn4 * ddt_scale));
        let eq50_e2379_d_n5: f64 = (locals.var_devsign * (locals.var_qbov_s_dn5 * ddt_scale));
        let eq50_e2379_d_n6: f64 = (locals.var_devsign * (locals.var_qbov_s_dn6 * ddt_scale));
        let eq50_e2379_d_n7: f64 = (locals.var_devsign * (locals.var_qbov_s_dn7 * ddt_scale));
        let eq50_e2379_d_n8: f64 = (locals.var_devsign * (locals.var_qbov_s_dn8 * ddt_scale));
        let eq50_e2379_d_n9: f64 = (locals.var_devsign * (locals.var_qbov_s_dn9 * ddt_scale));
        let eq50_e2379_d_n10: f64 = (locals.var_devsign * (locals.var_qbov_s_dn10 * ddt_scale));
        let eq50_e2379_d_n11: f64 = (locals.var_devsign * (locals.var_qbov_s_dn11 * ddt_scale));
        let eq50_e2379_d_n13: f64 = (locals.var_devsign * (locals.var_qbov_s_dn13 * ddt_scale));
        let eq50_e2379_d_n14: f64 = (locals.var_devsign * (locals.var_qbov_s_dn14 * ddt_scale));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n13, eq50_e2379_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e2381;
        let eq50_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq50_node_derivatives: [f64; 13] = [eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14];
        let eq50_branch_derivative_indices: [usize; 0] = [];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq50_value),
            &eq50_node_derivative_indices,
            &eq50_node_derivatives,
            &eq50_branch_derivative_indices,
            &eq50_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 == 0.0)) {
        let eq51_e2388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, locals.var_qgd_parasitic);
        (eq51_e2388, (locals.var_qgd_parasitic_dn0 * ddt_scale), (locals.var_qgd_parasitic_dn2 * ddt_scale), (locals.var_qgd_parasitic_dn3 * ddt_scale), (locals.var_qgd_parasitic_dn4 * ddt_scale), (locals.var_qgd_parasitic_dn5 * ddt_scale), (locals.var_qgd_parasitic_dn6 * ddt_scale), (locals.var_qgd_parasitic_dn7 * ddt_scale), (locals.var_qgd_parasitic_dn8 * ddt_scale), (locals.var_qgd_parasitic_dn9 * ddt_scale), (locals.var_qgd_parasitic_dn10 * ddt_scale), (locals.var_qgd_parasitic_dn11 * ddt_scale), (locals.var_qgd_parasitic_dn13 * ddt_scale), (locals.var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2390;
        let eq51_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq51_node_derivatives: [f64; 13] = [eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq51_value),
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n2,) = {
    if (locals.var_guard651 == 0.0) {
        let eq52_e2394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, locals.var_qds_fr);
        (eq52_e2394, (locals.var_qds_fr_dn0 * ddt_scale), (locals.var_qds_fr_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2396;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq52_value),
            0,
            multiplicity * (eq52_e2396_d_n0),
            2,
            multiplicity * (eq52_e2396_d_n2),
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard655 != 0.0)) {
        let eq53_e2402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, locals.var_qgs_fr);
        (eq53_e2402, (locals.var_qgs_fr_dn0 * ddt_scale), (locals.var_qgs_fr_dn2 * ddt_scale), (locals.var_qgs_fr_dn3 * ddt_scale), (locals.var_qgs_fr_dn4 * ddt_scale), (locals.var_qgs_fr_dn5 * ddt_scale), (locals.var_qgs_fr_dn6 * ddt_scale), (locals.var_qgs_fr_dn7 * ddt_scale), (locals.var_qgs_fr_dn8 * ddt_scale), (locals.var_qgs_fr_dn9 * ddt_scale), (locals.var_qgs_fr_dn10 * ddt_scale), (locals.var_qgs_fr_dn11 * ddt_scale), (locals.var_qgs_fr_dn13 * ddt_scale), (locals.var_qgs_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2404;
        let eq53_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq53_node_derivatives: [f64; 13] = [eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(2),
            multiplicity * (eq53_value),
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard655 != 0.0)) {
        let eq54_e2410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, locals.var_qgd_fr);
        (eq54_e2410, (locals.var_qgd_fr_dn0 * ddt_scale), (locals.var_qgd_fr_dn2 * ddt_scale), (locals.var_qgd_fr_dn3 * ddt_scale), (locals.var_qgd_fr_dn4 * ddt_scale), (locals.var_qgd_fr_dn5 * ddt_scale), (locals.var_qgd_fr_dn6 * ddt_scale), (locals.var_qgd_fr_dn7 * ddt_scale), (locals.var_qgd_fr_dn8 * ddt_scale), (locals.var_qgd_fr_dn9 * ddt_scale), (locals.var_qgd_fr_dn10 * ddt_scale), (locals.var_qgd_fr_dn11 * ddt_scale), (locals.var_qgd_fr_dn13 * ddt_scale), (locals.var_qgd_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2412;
        let eq54_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq54_node_derivatives: [f64; 13] = [eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14];
        let eq54_branch_derivative_indices: [usize; 0] = [];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(0),
            multiplicity * (eq54_value),
            &eq54_node_derivative_indices,
            &eq54_node_derivatives,
            &eq54_branch_derivative_indices,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14,) = {
    if (locals.var_guard656 != 0.0) {
        let eq55_e2416: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, locals.var_qg_acc);
        let eq55_e2417: f64 = (locals.var_devsign * eq55_e2416);
        let eq55_e2417_d_n0: f64 = (locals.var_devsign * (locals.var_qg_acc_dn0 * ddt_scale));
        let eq55_e2417_d_n2: f64 = (locals.var_devsign * (locals.var_qg_acc_dn2 * ddt_scale));
        let eq55_e2417_d_n3: f64 = (locals.var_devsign * (locals.var_qg_acc_dn3 * ddt_scale));
        let eq55_e2417_d_n4: f64 = (locals.var_devsign * (locals.var_qg_acc_dn4 * ddt_scale));
        let eq55_e2417_d_n5: f64 = (locals.var_devsign * (locals.var_qg_acc_dn5 * ddt_scale));
        let eq55_e2417_d_n6: f64 = (locals.var_devsign * (locals.var_qg_acc_dn6 * ddt_scale));
        let eq55_e2417_d_n7: f64 = (locals.var_devsign * (locals.var_qg_acc_dn7 * ddt_scale));
        let eq55_e2417_d_n8: f64 = (locals.var_devsign * (locals.var_qg_acc_dn8 * ddt_scale));
        let eq55_e2417_d_n9: f64 = (locals.var_devsign * (locals.var_qg_acc_dn9 * ddt_scale));
        let eq55_e2417_d_n10: f64 = (locals.var_devsign * (locals.var_qg_acc_dn10 * ddt_scale));
        let eq55_e2417_d_n11: f64 = (locals.var_devsign * (locals.var_qg_acc_dn11 * ddt_scale));
        let eq55_e2417_d_n13: f64 = (locals.var_devsign * (locals.var_qg_acc_dn13 * ddt_scale));
        let eq55_e2417_d_n14: f64 = (locals.var_devsign * (locals.var_qg_acc_dn14 * ddt_scale));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n13, eq55_e2417_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2419;
        let eq55_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq55_node_derivatives: [f64; 13] = [eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14,) = {
    if (locals.var_guard656 != 0.0) {
        let eq56_e2423: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, locals.var_qb_acc);
        let eq56_e2424: f64 = (locals.var_devsign * eq56_e2423);
        let eq56_e2424_d_n0: f64 = (locals.var_devsign * (locals.var_qb_acc_dn0 * ddt_scale));
        let eq56_e2424_d_n2: f64 = (locals.var_devsign * (locals.var_qb_acc_dn2 * ddt_scale));
        let eq56_e2424_d_n3: f64 = (locals.var_devsign * (locals.var_qb_acc_dn3 * ddt_scale));
        let eq56_e2424_d_n4: f64 = (locals.var_devsign * (locals.var_qb_acc_dn4 * ddt_scale));
        let eq56_e2424_d_n5: f64 = (locals.var_devsign * (locals.var_qb_acc_dn5 * ddt_scale));
        let eq56_e2424_d_n6: f64 = (locals.var_devsign * (locals.var_qb_acc_dn6 * ddt_scale));
        let eq56_e2424_d_n7: f64 = (locals.var_devsign * (locals.var_qb_acc_dn7 * ddt_scale));
        let eq56_e2424_d_n8: f64 = (locals.var_devsign * (locals.var_qb_acc_dn8 * ddt_scale));
        let eq56_e2424_d_n9: f64 = (locals.var_devsign * (locals.var_qb_acc_dn9 * ddt_scale));
        let eq56_e2424_d_n10: f64 = (locals.var_devsign * (locals.var_qb_acc_dn10 * ddt_scale));
        let eq56_e2424_d_n11: f64 = (locals.var_devsign * (locals.var_qb_acc_dn11 * ddt_scale));
        let eq56_e2424_d_n13: f64 = (locals.var_devsign * (locals.var_qb_acc_dn13 * ddt_scale));
        let eq56_e2424_d_n14: f64 = (locals.var_devsign * (locals.var_qb_acc_dn14 * ddt_scale));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n13, eq56_e2424_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e2426;
        let eq56_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq56_node_derivatives: [f64; 13] = [eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14];
        let eq56_branch_derivative_indices: [usize; 0] = [];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivative_indices,
            &eq56_node_derivatives,
            &eq56_branch_derivative_indices,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14,) = {
    if (locals.var_guard669 != 0.0) {
        let eq69_e2503: f64 = (locals.var_qg_v - locals.var_qb_v);
        let eq69_e2503_d_n0: f64 = (locals.var_qg_v_dn0 - locals.var_qb_v_dn0);
        let eq69_e2503_d_n2: f64 = (locals.var_qg_v_dn2 - locals.var_qb_v_dn2);
        let eq69_e2503_d_n3: f64 = (locals.var_qg_v_dn3 - locals.var_qb_v_dn3);
        let eq69_e2503_d_n4: f64 = (locals.var_qg_v_dn4 - locals.var_qb_v_dn4);
        let eq69_e2503_d_n5: f64 = (locals.var_qg_v_dn5 - locals.var_qb_v_dn5);
        let eq69_e2503_d_n6: f64 = (locals.var_qg_v_dn6 - locals.var_qb_v_dn6);
        let eq69_e2503_d_n7: f64 = (locals.var_qg_v_dn7 - locals.var_qb_v_dn7);
        let eq69_e2503_d_n8: f64 = (locals.var_qg_v_dn8 - locals.var_qb_v_dn8);
        let eq69_e2503_d_n9: f64 = (locals.var_qg_v_dn9 - locals.var_qb_v_dn9);
        let eq69_e2503_d_n10: f64 = (locals.var_qg_v_dn10 - locals.var_qb_v_dn10);
        let eq69_e2503_d_n11: f64 = (locals.var_qg_v_dn11 - locals.var_qb_v_dn11);
        let eq69_e2503_d_n13: f64 = (locals.var_qg_v_dn13 - locals.var_qb_v_dn13);
        let eq69_e2503_d_n14: f64 = (locals.var_qg_v_dn14 - locals.var_qb_v_dn14);
        let eq69_e2504: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq69_e2503);
        (eq69_e2504, (eq69_e2503_d_n0 * ddt_scale), (eq69_e2503_d_n2 * ddt_scale), (eq69_e2503_d_n3 * ddt_scale), (eq69_e2503_d_n4 * ddt_scale), (eq69_e2503_d_n5 * ddt_scale), (eq69_e2503_d_n6 * ddt_scale), (eq69_e2503_d_n7 * ddt_scale), (eq69_e2503_d_n8 * ddt_scale), (eq69_e2503_d_n9 * ddt_scale), (eq69_e2503_d_n10 * ddt_scale), (eq69_e2503_d_n11 * ddt_scale), (eq69_e2503_d_n13 * ddt_scale), (eq69_e2503_d_n14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2506;
        let eq69_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq69_node_derivatives: [f64; 13] = [eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14];
        let eq69_branch_derivative_indices: [usize; 0] = [];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivative_indices,
            &eq69_node_derivatives,
            &eq69_branch_derivative_indices,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15,) = {
    if (locals.var_guard669 != 0.0) {
        let eq71_e2516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, (nv15 - 0.0));
        let eq71_e2517: f64 = (1e-9 * eq71_e2516);
        (eq71_e2517, (1e-9 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e2519;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq71_value),
            15,
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16,) = {
    if (locals.var_guard677 == 0.0) {
        let eq96_e2712: f64 = (0.7071 * locals.var_sigrat);
        let eq96_e2712_d_n0: f64 = (0.7071 * locals.var_sigrat_dn0);
        let eq96_e2712_d_n2: f64 = (0.7071 * locals.var_sigrat_dn2);
        let eq96_e2712_d_n3: f64 = (0.7071 * locals.var_sigrat_dn3);
        let eq96_e2712_d_n4: f64 = (0.7071 * locals.var_sigrat_dn4);
        let eq96_e2712_d_n5: f64 = (0.7071 * locals.var_sigrat_dn5);
        let eq96_e2712_d_n6: f64 = (0.7071 * locals.var_sigrat_dn6);
        let eq96_e2712_d_n7: f64 = (0.7071 * locals.var_sigrat_dn7);
        let eq96_e2712_d_n8: f64 = (0.7071 * locals.var_sigrat_dn8);
        let eq96_e2712_d_n9: f64 = (0.7071 * locals.var_sigrat_dn9);
        let eq96_e2712_d_n10: f64 = (0.7071 * locals.var_sigrat_dn10);
        let eq96_e2712_d_n11: f64 = (0.7071 * locals.var_sigrat_dn11);
        let eq96_e2712_d_n13: f64 = (0.7071 * locals.var_sigrat_dn13);
        let eq96_e2712_d_n14: f64 = (0.7071 * locals.var_sigrat_dn14);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2715: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, eq96_e2714);
        (eq96_e2715, (eq96_e2714_d_n0 * ddt_scale), (eq96_e2714_d_n2 * ddt_scale), (eq96_e2714_d_n3 * ddt_scale), (eq96_e2714_d_n4 * ddt_scale), (eq96_e2714_d_n5 * ddt_scale), (eq96_e2714_d_n6 * ddt_scale), (eq96_e2714_d_n7 * ddt_scale), (eq96_e2714_d_n8 * ddt_scale), (eq96_e2714_d_n9 * ddt_scale), (eq96_e2714_d_n10 * ddt_scale), (eq96_e2714_d_n11 * ddt_scale), (eq96_e2714_d_n13 * ddt_scale), (eq96_e2714_d_n14 * ddt_scale), (eq96_e2712 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e2717;
        let eq96_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq96_node_derivatives: [f64; 14] = [eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16];
        let eq96_branch_derivative_indices: [usize; 0] = [];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq96_value),
            &eq96_node_derivative_indices,
            &eq96_node_derivatives,
            &eq96_branch_derivative_indices,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16,) = {
    if (locals.var_guard677 == 0.0) {
        let eq97_e2722: f64 = (0.7071 * locals.var_sigrat);
        let eq97_e2722_d_n0: f64 = (0.7071 * locals.var_sigrat_dn0);
        let eq97_e2722_d_n2: f64 = (0.7071 * locals.var_sigrat_dn2);
        let eq97_e2722_d_n3: f64 = (0.7071 * locals.var_sigrat_dn3);
        let eq97_e2722_d_n4: f64 = (0.7071 * locals.var_sigrat_dn4);
        let eq97_e2722_d_n5: f64 = (0.7071 * locals.var_sigrat_dn5);
        let eq97_e2722_d_n6: f64 = (0.7071 * locals.var_sigrat_dn6);
        let eq97_e2722_d_n7: f64 = (0.7071 * locals.var_sigrat_dn7);
        let eq97_e2722_d_n8: f64 = (0.7071 * locals.var_sigrat_dn8);
        let eq97_e2722_d_n9: f64 = (0.7071 * locals.var_sigrat_dn9);
        let eq97_e2722_d_n10: f64 = (0.7071 * locals.var_sigrat_dn10);
        let eq97_e2722_d_n11: f64 = (0.7071 * locals.var_sigrat_dn11);
        let eq97_e2722_d_n13: f64 = (0.7071 * locals.var_sigrat_dn13);
        let eq97_e2722_d_n14: f64 = (0.7071 * locals.var_sigrat_dn14);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2725: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, eq97_e2724);
        (eq97_e2725, (eq97_e2724_d_n0 * ddt_scale), (eq97_e2724_d_n2 * ddt_scale), (eq97_e2724_d_n3 * ddt_scale), (eq97_e2724_d_n4 * ddt_scale), (eq97_e2724_d_n5 * ddt_scale), (eq97_e2724_d_n6 * ddt_scale), (eq97_e2724_d_n7 * ddt_scale), (eq97_e2724_d_n8 * ddt_scale), (eq97_e2724_d_n9 * ddt_scale), (eq97_e2724_d_n10 * ddt_scale), (eq97_e2724_d_n11 * ddt_scale), (eq97_e2724_d_n13 * ddt_scale), (eq97_e2724_d_n14 * ddt_scale), (eq97_e2722 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_value: f64 = eq97_e2727;
        let eq97_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq97_node_derivatives: [f64; 14] = [eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16];
        let eq97_branch_derivative_indices: [usize; 0] = [];
        let eq97_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq97_value),
            &eq97_node_derivative_indices,
            &eq97_node_derivatives,
            &eq97_branch_derivative_indices,
            &eq97_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14,) = {
    if (locals.var_guard682 != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * locals.var_cth);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * locals.var_cth_dn0);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * locals.var_cth_dn2);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * locals.var_cth_dn3);
        let eq111_e2901_d_n4: f64 = (locals.var_cth + ((nv4 - 0.0) * locals.var_cth_dn4));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * locals.var_cth_dn5);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * locals.var_cth_dn6);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * locals.var_cth_dn7);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * locals.var_cth_dn8);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * locals.var_cth_dn9);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * locals.var_cth_dn10);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * locals.var_cth_dn11);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * locals.var_cth_dn13);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * locals.var_cth_dn14);
        let eq111_e2902: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, eq111_e2901);
        (eq111_e2902, (eq111_e2901_d_n0 * ddt_scale), (eq111_e2901_d_n2 * ddt_scale), (eq111_e2901_d_n3 * ddt_scale), (eq111_e2901_d_n4 * ddt_scale), (eq111_e2901_d_n5 * ddt_scale), (eq111_e2901_d_n6 * ddt_scale), (eq111_e2901_d_n7 * ddt_scale), (eq111_e2901_d_n8 * ddt_scale), (eq111_e2901_d_n9 * ddt_scale), (eq111_e2901_d_n10 * ddt_scale), (eq111_e2901_d_n11 * ddt_scale), (eq111_e2901_d_n13 * ddt_scale), (eq111_e2901_d_n14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e2904;
        let eq111_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq111_node_derivatives: [f64; 13] = [eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14];
        let eq111_branch_derivative_indices: [usize; 0] = [];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq111_value),
            &eq111_node_derivative_indices,
            &eq111_node_derivatives,
            &eq111_branch_derivative_indices,
            &eq111_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_q,) = {
    if (locals.var_guard642 == 0.0) {
        let eq4_e1976_q: f64 = locals.var_qd_v;
        let eq4_e1977: f64 = (locals.var_devsign * locals.var_qd_v);
        let eq4_e1977_d_n0: f64 = (locals.var_devsign * locals.var_qd_v_dn0);
        let eq4_e1977_d_n2: f64 = (locals.var_devsign * locals.var_qd_v_dn2);
        let eq4_e1977_d_n3: f64 = (locals.var_devsign * locals.var_qd_v_dn3);
        let eq4_e1977_d_n4: f64 = (locals.var_devsign * locals.var_qd_v_dn4);
        let eq4_e1977_d_n5: f64 = (locals.var_devsign * locals.var_qd_v_dn5);
        let eq4_e1977_d_n6: f64 = (locals.var_devsign * locals.var_qd_v_dn6);
        let eq4_e1977_d_n7: f64 = (locals.var_devsign * locals.var_qd_v_dn7);
        let eq4_e1977_d_n8: f64 = (locals.var_devsign * locals.var_qd_v_dn8);
        let eq4_e1977_d_n9: f64 = (locals.var_devsign * locals.var_qd_v_dn9);
        let eq4_e1977_d_n10: f64 = (locals.var_devsign * locals.var_qd_v_dn10);
        let eq4_e1977_d_n11: f64 = (locals.var_devsign * locals.var_qd_v_dn11);
        let eq4_e1977_d_n13: f64 = (locals.var_devsign * locals.var_qd_v_dn13);
        let eq4_e1977_d_n14: f64 = (locals.var_devsign * locals.var_qd_v_dn14);
        let eq4_e1977_q: f64 = (locals.var_devsign * eq4_e1976_q);
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 17] = [eq4_e1979_d_n0, 0.0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, 0.0, eq4_e1979_d_n13, eq4_e1979_d_n14, 0.0, 0.0];
        let eq4_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_q,) = {
    if (locals.var_guard642 == 0.0) {
        let eq5_e1984_q: f64 = locals.var_qg_v;
        let eq5_e1985: f64 = (locals.var_devsign * locals.var_qg_v);
        let eq5_e1985_d_n0: f64 = (locals.var_devsign * locals.var_qg_v_dn0);
        let eq5_e1985_d_n2: f64 = (locals.var_devsign * locals.var_qg_v_dn2);
        let eq5_e1985_d_n3: f64 = (locals.var_devsign * locals.var_qg_v_dn3);
        let eq5_e1985_d_n4: f64 = (locals.var_devsign * locals.var_qg_v_dn4);
        let eq5_e1985_d_n5: f64 = (locals.var_devsign * locals.var_qg_v_dn5);
        let eq5_e1985_d_n6: f64 = (locals.var_devsign * locals.var_qg_v_dn6);
        let eq5_e1985_d_n7: f64 = (locals.var_devsign * locals.var_qg_v_dn7);
        let eq5_e1985_d_n8: f64 = (locals.var_devsign * locals.var_qg_v_dn8);
        let eq5_e1985_d_n9: f64 = (locals.var_devsign * locals.var_qg_v_dn9);
        let eq5_e1985_d_n10: f64 = (locals.var_devsign * locals.var_qg_v_dn10);
        let eq5_e1985_d_n11: f64 = (locals.var_devsign * locals.var_qg_v_dn11);
        let eq5_e1985_d_n13: f64 = (locals.var_devsign * locals.var_qg_v_dn13);
        let eq5_e1985_d_n14: f64 = (locals.var_devsign * locals.var_qg_v_dn14);
        let eq5_e1985_q: f64 = (locals.var_devsign * eq5_e1984_q);
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 17] = [eq5_e1987_d_n0, 0.0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, 0.0, eq5_e1987_d_n13, eq5_e1987_d_n14, 0.0, 0.0];
        let eq5_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281_q: f64 = locals.var_qes;
        let eq36_e2282: f64 = (locals.var_devsign * locals.var_qes);
        let eq36_e2282_d_n0: f64 = (locals.var_devsign * locals.var_qes_dn0);
        let eq36_e2282_d_n2: f64 = (locals.var_devsign * locals.var_qes_dn2);
        let eq36_e2282_d_n3: f64 = (locals.var_devsign * locals.var_qes_dn3);
        let eq36_e2282_d_n4: f64 = (locals.var_devsign * locals.var_qes_dn4);
        let eq36_e2282_d_n5: f64 = (locals.var_devsign * locals.var_qes_dn5);
        let eq36_e2282_d_n6: f64 = (locals.var_devsign * locals.var_qes_dn6);
        let eq36_e2282_d_n7: f64 = (locals.var_devsign * locals.var_qes_dn7);
        let eq36_e2282_d_n8: f64 = (locals.var_devsign * locals.var_qes_dn8);
        let eq36_e2282_d_n9: f64 = (locals.var_devsign * locals.var_qes_dn9);
        let eq36_e2282_d_n10: f64 = (locals.var_devsign * locals.var_qes_dn10);
        let eq36_e2282_d_n11: f64 = (locals.var_devsign * locals.var_qes_dn11);
        let eq36_e2282_d_n13: f64 = (locals.var_devsign * locals.var_qes_dn13);
        let eq36_e2282_d_n14: f64 = (locals.var_devsign * locals.var_qes_dn14);
        let eq36_e2282_q: f64 = (locals.var_devsign * eq36_e2281_q);
        let eq36_reactive_node_derivatives: [f64; 17] = [eq36_e2282_d_n0, 0.0, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, 0.0, eq36_e2282_d_n13, eq36_e2282_d_n14, 0.0, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e2285_q: f64 = locals.var_qed;
        let eq37_e2286: f64 = (locals.var_devsign * locals.var_qed);
        let eq37_e2286_d_n0: f64 = (locals.var_devsign * locals.var_qed_dn0);
        let eq37_e2286_d_n2: f64 = (locals.var_devsign * locals.var_qed_dn2);
        let eq37_e2286_d_n3: f64 = (locals.var_devsign * locals.var_qed_dn3);
        let eq37_e2286_d_n4: f64 = (locals.var_devsign * locals.var_qed_dn4);
        let eq37_e2286_d_n5: f64 = (locals.var_devsign * locals.var_qed_dn5);
        let eq37_e2286_d_n6: f64 = (locals.var_devsign * locals.var_qed_dn6);
        let eq37_e2286_d_n7: f64 = (locals.var_devsign * locals.var_qed_dn7);
        let eq37_e2286_d_n8: f64 = (locals.var_devsign * locals.var_qed_dn8);
        let eq37_e2286_d_n9: f64 = (locals.var_devsign * locals.var_qed_dn9);
        let eq37_e2286_d_n10: f64 = (locals.var_devsign * locals.var_qed_dn10);
        let eq37_e2286_d_n11: f64 = (locals.var_devsign * locals.var_qed_dn11);
        let eq37_e2286_d_n13: f64 = (locals.var_devsign * locals.var_qed_dn13);
        let eq37_e2286_d_n14: f64 = (locals.var_devsign * locals.var_qed_dn14);
        let eq37_e2286_q: f64 = (locals.var_devsign * eq37_e2285_q);
        let eq37_reactive_node_derivatives: [f64; 17] = [eq37_e2286_d_n0, 0.0, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, 0.0, eq37_e2286_d_n13, eq37_e2286_d_n14, 0.0, 0.0];
        let eq37_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289_q: f64 = locals.var_qeg;
        let eq38_e2290: f64 = (locals.var_devsign * locals.var_qeg);
        let eq38_e2290_d_n0: f64 = (locals.var_devsign * locals.var_qeg_dn0);
        let eq38_e2290_d_n2: f64 = (locals.var_devsign * locals.var_qeg_dn2);
        let eq38_e2290_d_n3: f64 = (locals.var_devsign * locals.var_qeg_dn3);
        let eq38_e2290_d_n4: f64 = (locals.var_devsign * locals.var_qeg_dn4);
        let eq38_e2290_d_n5: f64 = (locals.var_devsign * locals.var_qeg_dn5);
        let eq38_e2290_d_n6: f64 = (locals.var_devsign * locals.var_qeg_dn6);
        let eq38_e2290_d_n7: f64 = (locals.var_devsign * locals.var_qeg_dn7);
        let eq38_e2290_d_n8: f64 = (locals.var_devsign * locals.var_qeg_dn8);
        let eq38_e2290_d_n9: f64 = (locals.var_devsign * locals.var_qeg_dn9);
        let eq38_e2290_d_n10: f64 = (locals.var_devsign * locals.var_qeg_dn10);
        let eq38_e2290_d_n11: f64 = (locals.var_devsign * locals.var_qeg_dn11);
        let eq38_e2290_d_n13: f64 = (locals.var_devsign * locals.var_qeg_dn13);
        let eq38_e2290_d_n14: f64 = (locals.var_devsign * locals.var_qeg_dn14);
        let eq38_e2290_q: f64 = (locals.var_devsign * eq38_e2289_q);
        let eq38_reactive_node_derivatives: [f64; 17] = [eq38_e2290_d_n0, 0.0, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, 0.0, eq38_e2290_d_n13, eq38_e2290_d_n14, 0.0, 0.0];
        let eq38_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_q,) = {
    if (locals.var_guard651 != 0.0) {
        let eq39_e2293_q: f64 = locals.var_qgs_parasitic;
        (locals.var_qgs_parasitic, locals.var_qgs_parasitic_dn0, locals.var_qgs_parasitic_dn2, locals.var_qgs_parasitic_dn3, locals.var_qgs_parasitic_dn4, locals.var_qgs_parasitic_dn5, locals.var_qgs_parasitic_dn6, locals.var_qgs_parasitic_dn7, locals.var_qgs_parasitic_dn8, locals.var_qgs_parasitic_dn9, locals.var_qgs_parasitic_dn10, locals.var_qgs_parasitic_dn11, locals.var_qgs_parasitic_dn13, locals.var_qgs_parasitic_dn14, eq39_e2293_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 17] = [eq39_e2295_d_n0, 0.0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, 0.0, eq39_e2295_d_n13, eq39_e2295_d_n14, 0.0, 0.0];
        let eq39_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_q,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 != 0.0)) {
        let eq40_e2300_q: f64 = locals.var_qgd_parasitic;
        (locals.var_qgd_parasitic, locals.var_qgd_parasitic_dn0, locals.var_qgd_parasitic_dn2, locals.var_qgd_parasitic_dn3, locals.var_qgd_parasitic_dn4, locals.var_qgd_parasitic_dn5, locals.var_qgd_parasitic_dn6, locals.var_qgd_parasitic_dn7, locals.var_qgd_parasitic_dn8, locals.var_qgd_parasitic_dn9, locals.var_qgd_parasitic_dn10, locals.var_qgd_parasitic_dn11, locals.var_qgd_parasitic_dn13, locals.var_qgd_parasitic_dn14, eq40_e2300_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 17] = [eq40_e2302_d_n0, 0.0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, 0.0, eq40_e2302_d_n13, eq40_e2302_d_n14, 0.0, 0.0];
        let eq40_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_q,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 != 0.0)) {
        let eq41_e2308_q: f64 = locals.var_qbov;
        let eq41_e2309: f64 = (locals.var_devsign * locals.var_qbov);
        let eq41_e2309_d_n0: f64 = (locals.var_devsign * locals.var_qbov_dn0);
        let eq41_e2309_d_n2: f64 = (locals.var_devsign * locals.var_qbov_dn2);
        let eq41_e2309_d_n3: f64 = (locals.var_devsign * locals.var_qbov_dn3);
        let eq41_e2309_d_n4: f64 = (locals.var_devsign * locals.var_qbov_dn4);
        let eq41_e2309_d_n5: f64 = (locals.var_devsign * locals.var_qbov_dn5);
        let eq41_e2309_d_n6: f64 = (locals.var_devsign * locals.var_qbov_dn6);
        let eq41_e2309_d_n7: f64 = (locals.var_devsign * locals.var_qbov_dn7);
        let eq41_e2309_d_n8: f64 = (locals.var_devsign * locals.var_qbov_dn8);
        let eq41_e2309_d_n9: f64 = (locals.var_devsign * locals.var_qbov_dn9);
        let eq41_e2309_d_n10: f64 = (locals.var_devsign * locals.var_qbov_dn10);
        let eq41_e2309_d_n11: f64 = (locals.var_devsign * locals.var_qbov_dn11);
        let eq41_e2309_d_n13: f64 = (locals.var_devsign * locals.var_qbov_dn13);
        let eq41_e2309_d_n14: f64 = (locals.var_devsign * locals.var_qbov_dn14);
        let eq41_e2309_q: f64 = (locals.var_devsign * eq41_e2308_q);
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n13, eq41_e2309_d_n14, eq41_e2309_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 17] = [eq41_e2311_d_n0, 0.0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, 0.0, eq41_e2311_d_n13, eq41_e2311_d_n14, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_q,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 != 0.0)) {
        let eq42_e2317_q: f64 = locals.var_qbov_s;
        let eq42_e2318: f64 = (locals.var_devsign * locals.var_qbov_s);
        let eq42_e2318_d_n0: f64 = (locals.var_devsign * locals.var_qbov_s_dn0);
        let eq42_e2318_d_n2: f64 = (locals.var_devsign * locals.var_qbov_s_dn2);
        let eq42_e2318_d_n3: f64 = (locals.var_devsign * locals.var_qbov_s_dn3);
        let eq42_e2318_d_n4: f64 = (locals.var_devsign * locals.var_qbov_s_dn4);
        let eq42_e2318_d_n5: f64 = (locals.var_devsign * locals.var_qbov_s_dn5);
        let eq42_e2318_d_n6: f64 = (locals.var_devsign * locals.var_qbov_s_dn6);
        let eq42_e2318_d_n7: f64 = (locals.var_devsign * locals.var_qbov_s_dn7);
        let eq42_e2318_d_n8: f64 = (locals.var_devsign * locals.var_qbov_s_dn8);
        let eq42_e2318_d_n9: f64 = (locals.var_devsign * locals.var_qbov_s_dn9);
        let eq42_e2318_d_n10: f64 = (locals.var_devsign * locals.var_qbov_s_dn10);
        let eq42_e2318_d_n11: f64 = (locals.var_devsign * locals.var_qbov_s_dn11);
        let eq42_e2318_d_n13: f64 = (locals.var_devsign * locals.var_qbov_s_dn13);
        let eq42_e2318_d_n14: f64 = (locals.var_devsign * locals.var_qbov_s_dn14);
        let eq42_e2318_q: f64 = (locals.var_devsign * eq42_e2317_q);
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n13, eq42_e2318_d_n14, eq42_e2318_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_reactive_node_derivatives: [f64; 17] = [eq42_e2320_d_n0, 0.0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, 0.0, eq42_e2320_d_n13, eq42_e2320_d_n14, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_q,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard652 == 0.0)) {
        let eq43_e2326_q: f64 = locals.var_qgd_parasitic;
        (locals.var_qgd_parasitic, locals.var_qgd_parasitic_dn0, locals.var_qgd_parasitic_dn2, locals.var_qgd_parasitic_dn3, locals.var_qgd_parasitic_dn4, locals.var_qgd_parasitic_dn5, locals.var_qgd_parasitic_dn6, locals.var_qgd_parasitic_dn7, locals.var_qgd_parasitic_dn8, locals.var_qgd_parasitic_dn9, locals.var_qgd_parasitic_dn10, locals.var_qgd_parasitic_dn11, locals.var_qgd_parasitic_dn13, locals.var_qgd_parasitic_dn14, eq43_e2326_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_reactive_node_derivatives: [f64; 17] = [eq43_e2328_d_n0, 0.0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, 0.0, eq43_e2328_d_n13, eq43_e2328_d_n14, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n2, eq44_e2333_q,) = {
    if (locals.var_guard651 != 0.0) {
        let eq44_e2331_q: f64 = locals.var_qds_fr;
        (locals.var_qds_fr, locals.var_qds_fr_dn0, locals.var_qds_fr_dn2, eq44_e2331_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (eq44_e2333_d_n0),
            nodes[2],
            multiplicity * (eq44_e2333_d_n2),
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_q,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard653 != 0.0)) {
        let eq45_e2338_q: f64 = locals.var_qgs_fr;
        (locals.var_qgs_fr, locals.var_qgs_fr_dn0, locals.var_qgs_fr_dn2, locals.var_qgs_fr_dn3, locals.var_qgs_fr_dn4, locals.var_qgs_fr_dn5, locals.var_qgs_fr_dn6, locals.var_qgs_fr_dn7, locals.var_qgs_fr_dn8, locals.var_qgs_fr_dn9, locals.var_qgs_fr_dn10, locals.var_qgs_fr_dn11, locals.var_qgs_fr_dn13, locals.var_qgs_fr_dn14, eq45_e2338_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_reactive_node_derivatives: [f64; 17] = [eq45_e2340_d_n0, 0.0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, 0.0, eq45_e2340_d_n13, eq45_e2340_d_n14, 0.0, 0.0];
        let eq45_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_q,) = {
    if ((locals.var_guard651 != 0.0) && (locals.var_guard653 != 0.0)) {
        let eq46_e2345_q: f64 = locals.var_qgd_fr;
        (locals.var_qgd_fr, locals.var_qgd_fr_dn0, locals.var_qgd_fr_dn2, locals.var_qgd_fr_dn3, locals.var_qgd_fr_dn4, locals.var_qgd_fr_dn5, locals.var_qgd_fr_dn6, locals.var_qgd_fr_dn7, locals.var_qgd_fr_dn8, locals.var_qgd_fr_dn9, locals.var_qgd_fr_dn10, locals.var_qgd_fr_dn11, locals.var_qgd_fr_dn13, locals.var_qgd_fr_dn14, eq46_e2345_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 17] = [eq46_e2347_d_n0, 0.0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, 0.0, eq46_e2347_d_n13, eq46_e2347_d_n14, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_q,) = {
    if (locals.var_guard651 == 0.0) {
        let eq47_e2351_q: f64 = locals.var_qgs_parasitic;
        (locals.var_qgs_parasitic, locals.var_qgs_parasitic_dn0, locals.var_qgs_parasitic_dn2, locals.var_qgs_parasitic_dn3, locals.var_qgs_parasitic_dn4, locals.var_qgs_parasitic_dn5, locals.var_qgs_parasitic_dn6, locals.var_qgs_parasitic_dn7, locals.var_qgs_parasitic_dn8, locals.var_qgs_parasitic_dn9, locals.var_qgs_parasitic_dn10, locals.var_qgs_parasitic_dn11, locals.var_qgs_parasitic_dn13, locals.var_qgs_parasitic_dn14, eq47_e2351_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 17] = [eq47_e2353_d_n0, 0.0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, 0.0, eq47_e2353_d_n13, eq47_e2353_d_n14, 0.0, 0.0];
        let eq47_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_q,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 != 0.0)) {
        let eq48_e2359_q: f64 = locals.var_qgd_parasitic;
        (locals.var_qgd_parasitic, locals.var_qgd_parasitic_dn0, locals.var_qgd_parasitic_dn2, locals.var_qgd_parasitic_dn3, locals.var_qgd_parasitic_dn4, locals.var_qgd_parasitic_dn5, locals.var_qgd_parasitic_dn6, locals.var_qgd_parasitic_dn7, locals.var_qgd_parasitic_dn8, locals.var_qgd_parasitic_dn9, locals.var_qgd_parasitic_dn10, locals.var_qgd_parasitic_dn11, locals.var_qgd_parasitic_dn13, locals.var_qgd_parasitic_dn14, eq48_e2359_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 17] = [eq48_e2361_d_n0, 0.0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, 0.0, eq48_e2361_d_n13, eq48_e2361_d_n14, 0.0, 0.0];
        let eq48_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_q,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 != 0.0)) {
        let eq49_e2368_q: f64 = locals.var_qbov;
        let eq49_e2369: f64 = (locals.var_devsign * locals.var_qbov);
        let eq49_e2369_d_n0: f64 = (locals.var_devsign * locals.var_qbov_dn0);
        let eq49_e2369_d_n2: f64 = (locals.var_devsign * locals.var_qbov_dn2);
        let eq49_e2369_d_n3: f64 = (locals.var_devsign * locals.var_qbov_dn3);
        let eq49_e2369_d_n4: f64 = (locals.var_devsign * locals.var_qbov_dn4);
        let eq49_e2369_d_n5: f64 = (locals.var_devsign * locals.var_qbov_dn5);
        let eq49_e2369_d_n6: f64 = (locals.var_devsign * locals.var_qbov_dn6);
        let eq49_e2369_d_n7: f64 = (locals.var_devsign * locals.var_qbov_dn7);
        let eq49_e2369_d_n8: f64 = (locals.var_devsign * locals.var_qbov_dn8);
        let eq49_e2369_d_n9: f64 = (locals.var_devsign * locals.var_qbov_dn9);
        let eq49_e2369_d_n10: f64 = (locals.var_devsign * locals.var_qbov_dn10);
        let eq49_e2369_d_n11: f64 = (locals.var_devsign * locals.var_qbov_dn11);
        let eq49_e2369_d_n13: f64 = (locals.var_devsign * locals.var_qbov_dn13);
        let eq49_e2369_d_n14: f64 = (locals.var_devsign * locals.var_qbov_dn14);
        let eq49_e2369_q: f64 = (locals.var_devsign * eq49_e2368_q);
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n13, eq49_e2369_d_n14, eq49_e2369_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_reactive_node_derivatives: [f64; 17] = [eq49_e2371_d_n0, 0.0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, 0.0, eq49_e2371_d_n13, eq49_e2371_d_n14, 0.0, 0.0];
        let eq49_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_q,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 != 0.0)) {
        let eq50_e2378_q: f64 = locals.var_qbov_s;
        let eq50_e2379: f64 = (locals.var_devsign * locals.var_qbov_s);
        let eq50_e2379_d_n0: f64 = (locals.var_devsign * locals.var_qbov_s_dn0);
        let eq50_e2379_d_n2: f64 = (locals.var_devsign * locals.var_qbov_s_dn2);
        let eq50_e2379_d_n3: f64 = (locals.var_devsign * locals.var_qbov_s_dn3);
        let eq50_e2379_d_n4: f64 = (locals.var_devsign * locals.var_qbov_s_dn4);
        let eq50_e2379_d_n5: f64 = (locals.var_devsign * locals.var_qbov_s_dn5);
        let eq50_e2379_d_n6: f64 = (locals.var_devsign * locals.var_qbov_s_dn6);
        let eq50_e2379_d_n7: f64 = (locals.var_devsign * locals.var_qbov_s_dn7);
        let eq50_e2379_d_n8: f64 = (locals.var_devsign * locals.var_qbov_s_dn8);
        let eq50_e2379_d_n9: f64 = (locals.var_devsign * locals.var_qbov_s_dn9);
        let eq50_e2379_d_n10: f64 = (locals.var_devsign * locals.var_qbov_s_dn10);
        let eq50_e2379_d_n11: f64 = (locals.var_devsign * locals.var_qbov_s_dn11);
        let eq50_e2379_d_n13: f64 = (locals.var_devsign * locals.var_qbov_s_dn13);
        let eq50_e2379_d_n14: f64 = (locals.var_devsign * locals.var_qbov_s_dn14);
        let eq50_e2379_q: f64 = (locals.var_devsign * eq50_e2378_q);
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n13, eq50_e2379_d_n14, eq50_e2379_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 17] = [eq50_e2381_d_n0, 0.0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, 0.0, eq50_e2381_d_n13, eq50_e2381_d_n14, 0.0, 0.0];
        let eq50_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_q,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard654 == 0.0)) {
        let eq51_e2388_q: f64 = locals.var_qgd_parasitic;
        (locals.var_qgd_parasitic, locals.var_qgd_parasitic_dn0, locals.var_qgd_parasitic_dn2, locals.var_qgd_parasitic_dn3, locals.var_qgd_parasitic_dn4, locals.var_qgd_parasitic_dn5, locals.var_qgd_parasitic_dn6, locals.var_qgd_parasitic_dn7, locals.var_qgd_parasitic_dn8, locals.var_qgd_parasitic_dn9, locals.var_qgd_parasitic_dn10, locals.var_qgd_parasitic_dn11, locals.var_qgd_parasitic_dn13, locals.var_qgd_parasitic_dn14, eq51_e2388_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 17] = [eq51_e2390_d_n0, 0.0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, 0.0, eq51_e2390_d_n13, eq51_e2390_d_n14, 0.0, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n2, eq52_e2396_q,) = {
    if (locals.var_guard651 == 0.0) {
        let eq52_e2394_q: f64 = locals.var_qds_fr;
        (locals.var_qds_fr, locals.var_qds_fr_dn0, locals.var_qds_fr_dn2, eq52_e2394_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (eq52_e2396_d_n0),
            nodes[2],
            multiplicity * (eq52_e2396_d_n2),
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_q,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard655 != 0.0)) {
        let eq53_e2402_q: f64 = locals.var_qgs_fr;
        (locals.var_qgs_fr, locals.var_qgs_fr_dn0, locals.var_qgs_fr_dn2, locals.var_qgs_fr_dn3, locals.var_qgs_fr_dn4, locals.var_qgs_fr_dn5, locals.var_qgs_fr_dn6, locals.var_qgs_fr_dn7, locals.var_qgs_fr_dn8, locals.var_qgs_fr_dn9, locals.var_qgs_fr_dn10, locals.var_qgs_fr_dn11, locals.var_qgs_fr_dn13, locals.var_qgs_fr_dn14, eq53_e2402_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 17] = [eq53_e2404_d_n0, 0.0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, 0.0, eq53_e2404_d_n13, eq53_e2404_d_n14, 0.0, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[2]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_q,) = {
    if ((locals.var_guard651 == 0.0) && (locals.var_guard655 != 0.0)) {
        let eq54_e2410_q: f64 = locals.var_qgd_fr;
        (locals.var_qgd_fr, locals.var_qgd_fr_dn0, locals.var_qgd_fr_dn2, locals.var_qgd_fr_dn3, locals.var_qgd_fr_dn4, locals.var_qgd_fr_dn5, locals.var_qgd_fr_dn6, locals.var_qgd_fr_dn7, locals.var_qgd_fr_dn8, locals.var_qgd_fr_dn9, locals.var_qgd_fr_dn10, locals.var_qgd_fr_dn11, locals.var_qgd_fr_dn13, locals.var_qgd_fr_dn14, eq54_e2410_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 17] = [eq54_e2412_d_n0, 0.0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, 0.0, eq54_e2412_d_n13, eq54_e2412_d_n14, 0.0, 0.0];
        let eq54_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_q,) = {
    if (locals.var_guard656 != 0.0) {
        let eq55_e2416_q: f64 = locals.var_qg_acc;
        let eq55_e2417: f64 = (locals.var_devsign * locals.var_qg_acc);
        let eq55_e2417_d_n0: f64 = (locals.var_devsign * locals.var_qg_acc_dn0);
        let eq55_e2417_d_n2: f64 = (locals.var_devsign * locals.var_qg_acc_dn2);
        let eq55_e2417_d_n3: f64 = (locals.var_devsign * locals.var_qg_acc_dn3);
        let eq55_e2417_d_n4: f64 = (locals.var_devsign * locals.var_qg_acc_dn4);
        let eq55_e2417_d_n5: f64 = (locals.var_devsign * locals.var_qg_acc_dn5);
        let eq55_e2417_d_n6: f64 = (locals.var_devsign * locals.var_qg_acc_dn6);
        let eq55_e2417_d_n7: f64 = (locals.var_devsign * locals.var_qg_acc_dn7);
        let eq55_e2417_d_n8: f64 = (locals.var_devsign * locals.var_qg_acc_dn8);
        let eq55_e2417_d_n9: f64 = (locals.var_devsign * locals.var_qg_acc_dn9);
        let eq55_e2417_d_n10: f64 = (locals.var_devsign * locals.var_qg_acc_dn10);
        let eq55_e2417_d_n11: f64 = (locals.var_devsign * locals.var_qg_acc_dn11);
        let eq55_e2417_d_n13: f64 = (locals.var_devsign * locals.var_qg_acc_dn13);
        let eq55_e2417_d_n14: f64 = (locals.var_devsign * locals.var_qg_acc_dn14);
        let eq55_e2417_q: f64 = (locals.var_devsign * eq55_e2416_q);
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e2419_d_n0, 0.0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, 0.0, eq55_e2419_d_n13, eq55_e2419_d_n14, 0.0, 0.0];
        let eq55_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_q,) = {
    if (locals.var_guard656 != 0.0) {
        let eq56_e2423_q: f64 = locals.var_qb_acc;
        let eq56_e2424: f64 = (locals.var_devsign * locals.var_qb_acc);
        let eq56_e2424_d_n0: f64 = (locals.var_devsign * locals.var_qb_acc_dn0);
        let eq56_e2424_d_n2: f64 = (locals.var_devsign * locals.var_qb_acc_dn2);
        let eq56_e2424_d_n3: f64 = (locals.var_devsign * locals.var_qb_acc_dn3);
        let eq56_e2424_d_n4: f64 = (locals.var_devsign * locals.var_qb_acc_dn4);
        let eq56_e2424_d_n5: f64 = (locals.var_devsign * locals.var_qb_acc_dn5);
        let eq56_e2424_d_n6: f64 = (locals.var_devsign * locals.var_qb_acc_dn6);
        let eq56_e2424_d_n7: f64 = (locals.var_devsign * locals.var_qb_acc_dn7);
        let eq56_e2424_d_n8: f64 = (locals.var_devsign * locals.var_qb_acc_dn8);
        let eq56_e2424_d_n9: f64 = (locals.var_devsign * locals.var_qb_acc_dn9);
        let eq56_e2424_d_n10: f64 = (locals.var_devsign * locals.var_qb_acc_dn10);
        let eq56_e2424_d_n11: f64 = (locals.var_devsign * locals.var_qb_acc_dn11);
        let eq56_e2424_d_n13: f64 = (locals.var_devsign * locals.var_qb_acc_dn13);
        let eq56_e2424_d_n14: f64 = (locals.var_devsign * locals.var_qb_acc_dn14);
        let eq56_e2424_q: f64 = (locals.var_devsign * eq56_e2423_q);
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_reactive_node_derivatives: [f64; 17] = [eq56_e2426_d_n0, 0.0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, 0.0, eq56_e2426_d_n13, eq56_e2426_d_n14, 0.0, 0.0];
        let eq56_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_q,) = {
    if (locals.var_guard669 != 0.0) {
        let eq69_e2503: f64 = (locals.var_qg_v - locals.var_qb_v);
        let eq69_e2503_d_n0: f64 = (locals.var_qg_v_dn0 - locals.var_qb_v_dn0);
        let eq69_e2503_d_n2: f64 = (locals.var_qg_v_dn2 - locals.var_qb_v_dn2);
        let eq69_e2503_d_n3: f64 = (locals.var_qg_v_dn3 - locals.var_qb_v_dn3);
        let eq69_e2503_d_n4: f64 = (locals.var_qg_v_dn4 - locals.var_qb_v_dn4);
        let eq69_e2503_d_n5: f64 = (locals.var_qg_v_dn5 - locals.var_qb_v_dn5);
        let eq69_e2503_d_n6: f64 = (locals.var_qg_v_dn6 - locals.var_qb_v_dn6);
        let eq69_e2503_d_n7: f64 = (locals.var_qg_v_dn7 - locals.var_qb_v_dn7);
        let eq69_e2503_d_n8: f64 = (locals.var_qg_v_dn8 - locals.var_qb_v_dn8);
        let eq69_e2503_d_n9: f64 = (locals.var_qg_v_dn9 - locals.var_qb_v_dn9);
        let eq69_e2503_d_n10: f64 = (locals.var_qg_v_dn10 - locals.var_qb_v_dn10);
        let eq69_e2503_d_n11: f64 = (locals.var_qg_v_dn11 - locals.var_qb_v_dn11);
        let eq69_e2503_d_n13: f64 = (locals.var_qg_v_dn13 - locals.var_qb_v_dn13);
        let eq69_e2503_d_n14: f64 = (locals.var_qg_v_dn14 - locals.var_qb_v_dn14);
        let eq69_e2504_q: f64 = eq69_e2503;
        (eq69_e2503, eq69_e2503_d_n0, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2504_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 17] = [eq69_e2506_d_n0, 0.0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, 0.0, eq69_e2506_d_n13, eq69_e2506_d_n14, 0.0, 0.0];
        let eq69_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15, eq71_e2519_q,) = {
    if (locals.var_guard669 != 0.0) {
        let eq71_e2516_q: f64 = (nv15 - 0.0);
        let eq71_e2517: f64 = (1e-9 * (nv15 - 0.0));
        let eq71_e2517_q: f64 = (1e-9 * eq71_e2516_q);
        (eq71_e2517, 1e-9, eq71_e2517_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16, eq96_e2717_q,) = {
    if (locals.var_guard677 == 0.0) {
        let eq96_e2712: f64 = (0.7071 * locals.var_sigrat);
        let eq96_e2712_d_n0: f64 = (0.7071 * locals.var_sigrat_dn0);
        let eq96_e2712_d_n2: f64 = (0.7071 * locals.var_sigrat_dn2);
        let eq96_e2712_d_n3: f64 = (0.7071 * locals.var_sigrat_dn3);
        let eq96_e2712_d_n4: f64 = (0.7071 * locals.var_sigrat_dn4);
        let eq96_e2712_d_n5: f64 = (0.7071 * locals.var_sigrat_dn5);
        let eq96_e2712_d_n6: f64 = (0.7071 * locals.var_sigrat_dn6);
        let eq96_e2712_d_n7: f64 = (0.7071 * locals.var_sigrat_dn7);
        let eq96_e2712_d_n8: f64 = (0.7071 * locals.var_sigrat_dn8);
        let eq96_e2712_d_n9: f64 = (0.7071 * locals.var_sigrat_dn9);
        let eq96_e2712_d_n10: f64 = (0.7071 * locals.var_sigrat_dn10);
        let eq96_e2712_d_n11: f64 = (0.7071 * locals.var_sigrat_dn11);
        let eq96_e2712_d_n13: f64 = (0.7071 * locals.var_sigrat_dn13);
        let eq96_e2712_d_n14: f64 = (0.7071 * locals.var_sigrat_dn14);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2715_q: f64 = eq96_e2714;
        (eq96_e2714, eq96_e2714_d_n0, eq96_e2714_d_n2, eq96_e2714_d_n3, eq96_e2714_d_n4, eq96_e2714_d_n5, eq96_e2714_d_n6, eq96_e2714_d_n7, eq96_e2714_d_n8, eq96_e2714_d_n9, eq96_e2714_d_n10, eq96_e2714_d_n11, eq96_e2714_d_n13, eq96_e2714_d_n14, eq96_e2712, eq96_e2715_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_reactive_node_derivatives: [f64; 17] = [eq96_e2717_d_n0, 0.0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, 0.0, eq96_e2717_d_n13, eq96_e2717_d_n14, 0.0, eq96_e2717_d_n16];
        let eq96_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq96_reactive_node_derivatives,
            branches,
            &eq96_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16, eq97_e2727_q,) = {
    if (locals.var_guard677 == 0.0) {
        let eq97_e2722: f64 = (0.7071 * locals.var_sigrat);
        let eq97_e2722_d_n0: f64 = (0.7071 * locals.var_sigrat_dn0);
        let eq97_e2722_d_n2: f64 = (0.7071 * locals.var_sigrat_dn2);
        let eq97_e2722_d_n3: f64 = (0.7071 * locals.var_sigrat_dn3);
        let eq97_e2722_d_n4: f64 = (0.7071 * locals.var_sigrat_dn4);
        let eq97_e2722_d_n5: f64 = (0.7071 * locals.var_sigrat_dn5);
        let eq97_e2722_d_n6: f64 = (0.7071 * locals.var_sigrat_dn6);
        let eq97_e2722_d_n7: f64 = (0.7071 * locals.var_sigrat_dn7);
        let eq97_e2722_d_n8: f64 = (0.7071 * locals.var_sigrat_dn8);
        let eq97_e2722_d_n9: f64 = (0.7071 * locals.var_sigrat_dn9);
        let eq97_e2722_d_n10: f64 = (0.7071 * locals.var_sigrat_dn10);
        let eq97_e2722_d_n11: f64 = (0.7071 * locals.var_sigrat_dn11);
        let eq97_e2722_d_n13: f64 = (0.7071 * locals.var_sigrat_dn13);
        let eq97_e2722_d_n14: f64 = (0.7071 * locals.var_sigrat_dn14);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2725_q: f64 = eq97_e2724;
        (eq97_e2724, eq97_e2724_d_n0, eq97_e2724_d_n2, eq97_e2724_d_n3, eq97_e2724_d_n4, eq97_e2724_d_n5, eq97_e2724_d_n6, eq97_e2724_d_n7, eq97_e2724_d_n8, eq97_e2724_d_n9, eq97_e2724_d_n10, eq97_e2724_d_n11, eq97_e2724_d_n13, eq97_e2724_d_n14, eq97_e2722, eq97_e2725_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_reactive_node_derivatives: [f64; 17] = [eq97_e2727_d_n0, 0.0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, 0.0, eq97_e2727_d_n13, eq97_e2727_d_n14, 0.0, eq97_e2727_d_n16];
        let eq97_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq97_reactive_node_derivatives,
            branches,
            &eq97_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_q,) = {
    if (locals.var_guard682 != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * locals.var_cth);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * locals.var_cth_dn0);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * locals.var_cth_dn2);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * locals.var_cth_dn3);
        let eq111_e2901_d_n4: f64 = (locals.var_cth + ((nv4 - 0.0) * locals.var_cth_dn4));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * locals.var_cth_dn5);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * locals.var_cth_dn6);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * locals.var_cth_dn7);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * locals.var_cth_dn8);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * locals.var_cth_dn9);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * locals.var_cth_dn10);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * locals.var_cth_dn11);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * locals.var_cth_dn13);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * locals.var_cth_dn14);
        let eq111_e2902_q: f64 = eq111_e2901;
        (eq111_e2901, eq111_e2901_d_n0, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2902_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 17] = [eq111_e2904_d_n0, 0.0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, 0.0, eq111_e2904_d_n13, eq111_e2904_d_n14, 0.0, 0.0];
        let eq111_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
