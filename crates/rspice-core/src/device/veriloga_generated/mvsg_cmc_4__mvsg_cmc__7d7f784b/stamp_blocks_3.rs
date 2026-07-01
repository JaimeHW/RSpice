#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19180_e18585,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn241_calc_iq__vzeta,)
    }
};
        locals.var_fn241_calc_iq__vzeta = assign19180_e18585;

        let (assign19190_e18589,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn241_calc_iq__lambda,)
    }
};
        locals.var_fn241_calc_iq__lambda = assign19190_e18589;

        let (assign19200_e18593,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn241_calc_iq__ngf,)
    }
};
        locals.var_fn241_calc_iq__ngf = assign19200_e18593;

        let (assign19210_e18597,) = {
    if (locals.var_guard240 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn241_calc_iq__type,)
    }
};
        locals.var_fn241_calc_iq__type = assign19210_e18597;

        let (assign19220_e18601,) = {
    if (locals.var_guard240 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn241_calc_iq__trapfracdl,)
    }
};
        locals.var_fn241_calc_iq__trapfracdl = assign19220_e18601;

        let (assign19230_e18605, assign19230_e18605_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__alpha_phit, locals.var_fn241_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn241_calc_iq__alpha_phit = assign19230_e18605;
        locals.var_fn241_calc_iq__alpha_phit_dn4 = assign19230_e18605_d_n4;

        let (assign19240_e18609, assign19240_e18609_d_n11, assign19240_e18609_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__delta, locals.var_fn241_calc_iq__delta_dn11, locals.var_fn241_calc_iq__delta_dn12,)
    }
};
        locals.var_fn241_calc_iq__delta = assign19240_e18609;
        locals.var_fn241_calc_iq__delta_dn11 = assign19240_e18609_d_n11;
        locals.var_fn241_calc_iq__delta_dn12 = assign19240_e18609_d_n12;

        let (assign19250_e18613, assign19250_e18613_d_n4, assign19250_e18613_d_n11, assign19250_e18613_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__n, locals.var_fn241_calc_iq__n_dn4, locals.var_fn241_calc_iq__n_dn11, locals.var_fn241_calc_iq__n_dn12,)
    }
};
        locals.var_fn241_calc_iq__n = assign19250_e18613;
        locals.var_fn241_calc_iq__n_dn4 = assign19250_e18613_d_n4;
        locals.var_fn241_calc_iq__n_dn11 = assign19250_e18613_d_n11;
        locals.var_fn241_calc_iq__n_dn12 = assign19250_e18613_d_n12;

        let (assign19260_e18617, assign19260_e18617_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vtof, locals.var_fn241_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn241_calc_iq__vtof = assign19260_e18617;
        locals.var_fn241_calc_iq__vtof_dn4 = assign19260_e18617_d_n4;

        let (assign19270_e18621, assign19270_e18621_d_n11, assign19270_e18621_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsatdibl, locals.var_fn241_calc_iq__vsatdibl_dn11, locals.var_fn241_calc_iq__vsatdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsatdibl = assign19270_e18621;
        locals.var_fn241_calc_iq__vsatdibl_dn11 = assign19270_e18621_d_n11;
        locals.var_fn241_calc_iq__vsatdibl_dn12 = assign19270_e18621_d_n12;

        let (assign19280_e18625, assign19280_e18625_d_n2, assign19280_e18625_d_n3, assign19280_e18625_d_n4, assign19280_e18625_d_n7, assign19280_e18625_d_n11, assign19280_e18625_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign19280_e18625;
        locals.var_fn241_calc_iq__ffs_dn2 = assign19280_e18625_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign19280_e18625_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign19280_e18625_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign19280_e18625_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign19280_e18625_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign19280_e18625_d_n12;

        let (assign19290_e18629, assign19290_e18629_d_n4, assign19290_e18629_d_n11, assign19290_e18629_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit, locals.var_fn241_calc_iq__two_n_phit_dn4, locals.var_fn241_calc_iq__two_n_phit_dn11, locals.var_fn241_calc_iq__two_n_phit_dn12,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit = assign19290_e18629;
        locals.var_fn241_calc_iq__two_n_phit_dn4 = assign19290_e18629_d_n4;
        locals.var_fn241_calc_iq__two_n_phit_dn11 = assign19290_e18629_d_n11;
        locals.var_fn241_calc_iq__two_n_phit_dn12 = assign19290_e18629_d_n12;

        let (assign19300_e18633, assign19300_e18633_d_n4, assign19300_e18633_d_n11, assign19300_e18633_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qref, locals.var_fn241_calc_iq__qref_dn4, locals.var_fn241_calc_iq__qref_dn11, locals.var_fn241_calc_iq__qref_dn12,)
    }
};
        locals.var_fn241_calc_iq__qref = assign19300_e18633;
        locals.var_fn241_calc_iq__qref_dn4 = assign19300_e18633_d_n4;
        locals.var_fn241_calc_iq__qref_dn11 = assign19300_e18633_d_n11;
        locals.var_fn241_calc_iq__qref_dn12 = assign19300_e18633_d_n12;

        let (assign19310_e18637, assign19310_e18637_d_n2, assign19310_e18637_d_n3, assign19310_e18637_d_n4, assign19310_e18637_d_n7, assign19310_e18637_d_n11, assign19310_e18637_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etas, locals.var_fn241_calc_iq__etas_dn2, locals.var_fn241_calc_iq__etas_dn3, locals.var_fn241_calc_iq__etas_dn4, locals.var_fn241_calc_iq__etas_dn7, locals.var_fn241_calc_iq__etas_dn11, locals.var_fn241_calc_iq__etas_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas = assign19310_e18637;
        locals.var_fn241_calc_iq__etas_dn2 = assign19310_e18637_d_n2;
        locals.var_fn241_calc_iq__etas_dn3 = assign19310_e18637_d_n3;
        locals.var_fn241_calc_iq__etas_dn4 = assign19310_e18637_d_n4;
        locals.var_fn241_calc_iq__etas_dn7 = assign19310_e18637_d_n7;
        locals.var_fn241_calc_iq__etas_dn11 = assign19310_e18637_d_n11;
        locals.var_fn241_calc_iq__etas_dn12 = assign19310_e18637_d_n12;

        let (assign19320_e18641, assign19320_e18641_d_n2, assign19320_e18641_d_n3, assign19320_e18641_d_n4, assign19320_e18641_d_n7, assign19320_e18641_d_n11, assign19320_e18641_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign19320_e18641;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign19320_e18641_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign19320_e18641_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign19320_e18641_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign19320_e18641_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign19320_e18641_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign19320_e18641_d_n12;

        let (assign19330_e18645, assign19330_e18645_d_n2, assign19330_e18645_d_n3, assign19330_e18645_d_n4, assign19330_e18645_d_n7, assign19330_e18645_d_n11, assign19330_e18645_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__muf, locals.var_fn241_calc_iq__muf_dn2, locals.var_fn241_calc_iq__muf_dn3, locals.var_fn241_calc_iq__muf_dn4, locals.var_fn241_calc_iq__muf_dn7, locals.var_fn241_calc_iq__muf_dn11, locals.var_fn241_calc_iq__muf_dn12,)
    }
};
        locals.var_fn241_calc_iq__muf = assign19330_e18645;
        locals.var_fn241_calc_iq__muf_dn2 = assign19330_e18645_d_n2;
        locals.var_fn241_calc_iq__muf_dn3 = assign19330_e18645_d_n3;
        locals.var_fn241_calc_iq__muf_dn4 = assign19330_e18645_d_n4;
        locals.var_fn241_calc_iq__muf_dn7 = assign19330_e18645_d_n7;
        locals.var_fn241_calc_iq__muf_dn11 = assign19330_e18645_d_n11;
        locals.var_fn241_calc_iq__muf_dn12 = assign19330_e18645_d_n12;

        let (assign19340_e18649, assign19340_e18649_d_n2, assign19340_e18649_d_n3, assign19340_e18649_d_n4, assign19340_e18649_d_n7, assign19340_e18649_d_n11, assign19340_e18649_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vx, locals.var_fn241_calc_iq__vx_dn2, locals.var_fn241_calc_iq__vx_dn3, locals.var_fn241_calc_iq__vx_dn4, locals.var_fn241_calc_iq__vx_dn7, locals.var_fn241_calc_iq__vx_dn11, locals.var_fn241_calc_iq__vx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vx = assign19340_e18649;
        locals.var_fn241_calc_iq__vx_dn2 = assign19340_e18649_d_n2;
        locals.var_fn241_calc_iq__vx_dn3 = assign19340_e18649_d_n3;
        locals.var_fn241_calc_iq__vx_dn4 = assign19340_e18649_d_n4;
        locals.var_fn241_calc_iq__vx_dn7 = assign19340_e18649_d_n7;
        locals.var_fn241_calc_iq__vx_dn11 = assign19340_e18649_d_n11;
        locals.var_fn241_calc_iq__vx_dn12 = assign19340_e18649_d_n12;

        let (assign19360_e18657, assign19360_e18657_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__n0, locals.var_fn241_calc_iq__n0_dn4,)
    }
};
        locals.var_fn241_calc_iq__n0 = assign19360_e18657;
        locals.var_fn241_calc_iq__n0_dn4 = assign19360_e18657_d_n4;

        let (assign19370_e18661, assign19370_e18661_d_n2, assign19370_e18661_d_n4, assign19370_e18661_d_n7, assign19370_e18661_d_n11, assign19370_e18661_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign19370_e18661;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign19370_e18661_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign19370_e18661_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign19370_e18661_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign19370_e18661_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign19370_e18661_d_n12;

        let (assign19380_e18665, assign19380_e18665_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit0, locals.var_fn241_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit0 = assign19380_e18665;
        locals.var_fn241_calc_iq__two_n_phit0_dn4 = assign19380_e18665_d_n4;

        let (assign19390_e18669, assign19390_e18669_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qref0, locals.var_fn241_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn241_calc_iq__qref0 = assign19390_e18669;
        locals.var_fn241_calc_iq__qref0_dn4 = assign19390_e18669_d_n4;

        let (assign19400_e18673, assign19400_e18673_d_n2, assign19400_e18673_d_n4, assign19400_e18673_d_n7, assign19400_e18673_d_n11, assign19400_e18673_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etas0, locals.var_fn241_calc_iq__etas0_dn2, locals.var_fn241_calc_iq__etas0_dn4, locals.var_fn241_calc_iq__etas0_dn7, locals.var_fn241_calc_iq__etas0_dn11, locals.var_fn241_calc_iq__etas0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas0 = assign19400_e18673;
        locals.var_fn241_calc_iq__etas0_dn2 = assign19400_e18673_d_n2;
        locals.var_fn241_calc_iq__etas0_dn4 = assign19400_e18673_d_n4;
        locals.var_fn241_calc_iq__etas0_dn7 = assign19400_e18673_d_n7;
        locals.var_fn241_calc_iq__etas0_dn11 = assign19400_e18673_d_n11;
        locals.var_fn241_calc_iq__etas0_dn12 = assign19400_e18673_d_n12;

        let (assign19410_e18677, assign19410_e18677_d_n2, assign19410_e18677_d_n4, assign19410_e18677_d_n7, assign19410_e18677_d_n11, assign19410_e18677_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign19410_e18677;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign19410_e18677_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign19410_e18677_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign19410_e18677_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign19410_e18677_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign19410_e18677_d_n12;

        let (assign19420_e18681, assign19420_e18681_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__muf0, locals.var_fn241_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn241_calc_iq__muf0 = assign19420_e18681;
        locals.var_fn241_calc_iq__muf0_dn4 = assign19420_e18681_d_n4;

        let (assign19430_e18685, assign19430_e18685_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vx0, locals.var_fn241_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vx0 = assign19430_e18685;
        locals.var_fn241_calc_iq__vx0_dn4 = assign19430_e18685_d_n4;

        let (assign19440_e18689, assign19440_e18689_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__tfacmobin, locals.var_fn241_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn241_calc_iq__tfacmobin = assign19440_e18689;
        locals.var_fn241_calc_iq__tfacmobin_dn4 = assign19440_e18689_d_n4;

        let (assign19450_e18693, assign19450_e18693_d_n2, assign19450_e18693_d_n3, assign19450_e18693_d_n4, assign19450_e18693_d_n7, assign19450_e18693_d_n11, assign19450_e18693_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign19450_e18693;
        locals.var_fn241_calc_iq__ff_dn2 = assign19450_e18693_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign19450_e18693_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign19450_e18693_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign19450_e18693_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign19450_e18693_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign19450_e18693_d_n12;

        let (assign19460_e18697, assign19460_e18697_d_n2, assign19460_e18697_d_n3, assign19460_e18697_d_n4, assign19460_e18697_d_n7, assign19460_e18697_d_n11, assign19460_e18697_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__eta, locals.var_fn241_calc_iq__eta_dn2, locals.var_fn241_calc_iq__eta_dn3, locals.var_fn241_calc_iq__eta_dn4, locals.var_fn241_calc_iq__eta_dn7, locals.var_fn241_calc_iq__eta_dn11, locals.var_fn241_calc_iq__eta_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta = assign19460_e18697;
        locals.var_fn241_calc_iq__eta_dn2 = assign19460_e18697_d_n2;
        locals.var_fn241_calc_iq__eta_dn3 = assign19460_e18697_d_n3;
        locals.var_fn241_calc_iq__eta_dn4 = assign19460_e18697_d_n4;
        locals.var_fn241_calc_iq__eta_dn7 = assign19460_e18697_d_n7;
        locals.var_fn241_calc_iq__eta_dn11 = assign19460_e18697_d_n11;
        locals.var_fn241_calc_iq__eta_dn12 = assign19460_e18697_d_n12;

        let (assign19470_e18701, assign19470_e18701_d_n2, assign19470_e18701_d_n3, assign19470_e18701_d_n4, assign19470_e18701_d_n7, assign19470_e18701_d_n11, assign19470_e18701_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign19470_e18701;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign19470_e18701_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign19470_e18701_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign19470_e18701_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign19470_e18701_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign19470_e18701_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign19470_e18701_d_n12;

        let (assign19480_e18705, assign19480_e18705_d_n2, assign19480_e18705_d_n4, assign19480_e18705_d_n7, assign19480_e18705_d_n11, assign19480_e18705_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign19480_e18705;
        locals.var_fn241_calc_iq__ff0_dn2 = assign19480_e18705_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign19480_e18705_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign19480_e18705_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign19480_e18705_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign19480_e18705_d_n12;

        let (assign19490_e18709, assign19490_e18709_d_n2, assign19490_e18709_d_n4, assign19490_e18709_d_n7, assign19490_e18709_d_n11, assign19490_e18709_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__eta0, locals.var_fn241_calc_iq__eta0_dn2, locals.var_fn241_calc_iq__eta0_dn4, locals.var_fn241_calc_iq__eta0_dn7, locals.var_fn241_calc_iq__eta0_dn11, locals.var_fn241_calc_iq__eta0_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta0 = assign19490_e18709;
        locals.var_fn241_calc_iq__eta0_dn2 = assign19490_e18709_d_n2;
        locals.var_fn241_calc_iq__eta0_dn4 = assign19490_e18709_d_n4;
        locals.var_fn241_calc_iq__eta0_dn7 = assign19490_e18709_d_n7;
        locals.var_fn241_calc_iq__eta0_dn11 = assign19490_e18709_d_n11;
        locals.var_fn241_calc_iq__eta0_dn12 = assign19490_e18709_d_n12;

        let (assign19500_e18713, assign19500_e18713_d_n2, assign19500_e18713_d_n4, assign19500_e18713_d_n7, assign19500_e18713_d_n11, assign19500_e18713_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign19500_e18713;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign19500_e18713_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign19500_e18713_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign19500_e18713_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign19500_e18713_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign19500_e18713_d_n12;

        let (assign19510_e18717, assign19510_e18717_d_n2, assign19510_e18717_d_n3, assign19510_e18717_d_n4, assign19510_e18717_d_n7, assign19510_e18717_d_n11, assign19510_e18717_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats, locals.var_fn241_calc_iq__vdsats_dn2, locals.var_fn241_calc_iq__vdsats_dn3, locals.var_fn241_calc_iq__vdsats_dn4, locals.var_fn241_calc_iq__vdsats_dn7, locals.var_fn241_calc_iq__vdsats_dn11, locals.var_fn241_calc_iq__vdsats_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats = assign19510_e18717;
        locals.var_fn241_calc_iq__vdsats_dn2 = assign19510_e18717_d_n2;
        locals.var_fn241_calc_iq__vdsats_dn3 = assign19510_e18717_d_n3;
        locals.var_fn241_calc_iq__vdsats_dn4 = assign19510_e18717_d_n4;
        locals.var_fn241_calc_iq__vdsats_dn7 = assign19510_e18717_d_n7;
        locals.var_fn241_calc_iq__vdsats_dn11 = assign19510_e18717_d_n11;
        locals.var_fn241_calc_iq__vdsats_dn12 = assign19510_e18717_d_n12;

        let (assign19520_e18721, assign19520_e18721_d_n2, assign19520_e18721_d_n3, assign19520_e18721_d_n4, assign19520_e18721_d_n7, assign19520_e18721_d_n11, assign19520_e18721_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats1, locals.var_fn241_calc_iq__vdsats1_dn2, locals.var_fn241_calc_iq__vdsats1_dn3, locals.var_fn241_calc_iq__vdsats1_dn4, locals.var_fn241_calc_iq__vdsats1_dn7, locals.var_fn241_calc_iq__vdsats1_dn11, locals.var_fn241_calc_iq__vdsats1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats1 = assign19520_e18721;
        locals.var_fn241_calc_iq__vdsats1_dn2 = assign19520_e18721_d_n2;
        locals.var_fn241_calc_iq__vdsats1_dn3 = assign19520_e18721_d_n3;
        locals.var_fn241_calc_iq__vdsats1_dn4 = assign19520_e18721_d_n4;
        locals.var_fn241_calc_iq__vdsats1_dn7 = assign19520_e18721_d_n7;
        locals.var_fn241_calc_iq__vdsats1_dn11 = assign19520_e18721_d_n11;
        locals.var_fn241_calc_iq__vdsats1_dn12 = assign19520_e18721_d_n12;

        let (assign19530_e18725, assign19530_e18725_d_n2, assign19530_e18725_d_n3, assign19530_e18725_d_n4, assign19530_e18725_d_n7, assign19530_e18725_d_n11, assign19530_e18725_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsat, locals.var_fn241_calc_iq__vdsat_dn2, locals.var_fn241_calc_iq__vdsat_dn3, locals.var_fn241_calc_iq__vdsat_dn4, locals.var_fn241_calc_iq__vdsat_dn7, locals.var_fn241_calc_iq__vdsat_dn11, locals.var_fn241_calc_iq__vdsat_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat = assign19530_e18725;
        locals.var_fn241_calc_iq__vdsat_dn2 = assign19530_e18725_d_n2;
        locals.var_fn241_calc_iq__vdsat_dn3 = assign19530_e18725_d_n3;
        locals.var_fn241_calc_iq__vdsat_dn4 = assign19530_e18725_d_n4;
        locals.var_fn241_calc_iq__vdsat_dn7 = assign19530_e18725_d_n7;
        locals.var_fn241_calc_iq__vdsat_dn11 = assign19530_e18725_d_n11;
        locals.var_fn241_calc_iq__vdsat_dn12 = assign19530_e18725_d_n12;

        let (assign19540_e18729, assign19540_e18729_d_n2, assign19540_e18729_d_n3, assign19540_e18729_d_n4, assign19540_e18729_d_n7, assign19540_e18729_d_n11, assign19540_e18729_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fsd, locals.var_fn241_calc_iq__fsd_dn2, locals.var_fn241_calc_iq__fsd_dn3, locals.var_fn241_calc_iq__fsd_dn4, locals.var_fn241_calc_iq__fsd_dn7, locals.var_fn241_calc_iq__fsd_dn11, locals.var_fn241_calc_iq__fsd_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd = assign19540_e18729;
        locals.var_fn241_calc_iq__fsd_dn2 = assign19540_e18729_d_n2;
        locals.var_fn241_calc_iq__fsd_dn3 = assign19540_e18729_d_n3;
        locals.var_fn241_calc_iq__fsd_dn4 = assign19540_e18729_d_n4;
        locals.var_fn241_calc_iq__fsd_dn7 = assign19540_e18729_d_n7;
        locals.var_fn241_calc_iq__fsd_dn11 = assign19540_e18729_d_n11;
        locals.var_fn241_calc_iq__fsd_dn12 = assign19540_e18729_d_n12;

        let (assign19550_e18733, assign19550_e18733_d_n2, assign19550_e18733_d_n3, assign19550_e18733_d_n4, assign19550_e18733_d_n7, assign19550_e18733_d_n11, assign19550_e18733_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdx, locals.var_fn241_calc_iq__vdx_dn2, locals.var_fn241_calc_iq__vdx_dn3, locals.var_fn241_calc_iq__vdx_dn4, locals.var_fn241_calc_iq__vdx_dn7, locals.var_fn241_calc_iq__vdx_dn11, locals.var_fn241_calc_iq__vdx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx = assign19550_e18733;
        locals.var_fn241_calc_iq__vdx_dn2 = assign19550_e18733_d_n2;
        locals.var_fn241_calc_iq__vdx_dn3 = assign19550_e18733_d_n3;
        locals.var_fn241_calc_iq__vdx_dn4 = assign19550_e18733_d_n4;
        locals.var_fn241_calc_iq__vdx_dn7 = assign19550_e18733_d_n7;
        locals.var_fn241_calc_iq__vdx_dn11 = assign19550_e18733_d_n11;
        locals.var_fn241_calc_iq__vdx_dn12 = assign19550_e18733_d_n12;

        let (assign19560_e18737, assign19560_e18737_d_n2, assign19560_e18737_d_n3, assign19560_e18737_d_n4, assign19560_e18737_d_n7, assign19560_e18737_d_n11, assign19560_e18737_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fds, locals.var_fn241_calc_iq__fds_dn2, locals.var_fn241_calc_iq__fds_dn3, locals.var_fn241_calc_iq__fds_dn4, locals.var_fn241_calc_iq__fds_dn7, locals.var_fn241_calc_iq__fds_dn11, locals.var_fn241_calc_iq__fds_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds = assign19560_e18737;
        locals.var_fn241_calc_iq__fds_dn2 = assign19560_e18737_d_n2;
        locals.var_fn241_calc_iq__fds_dn3 = assign19560_e18737_d_n3;
        locals.var_fn241_calc_iq__fds_dn4 = assign19560_e18737_d_n4;
        locals.var_fn241_calc_iq__fds_dn7 = assign19560_e18737_d_n7;
        locals.var_fn241_calc_iq__fds_dn11 = assign19560_e18737_d_n11;
        locals.var_fn241_calc_iq__fds_dn12 = assign19560_e18737_d_n12;

        let (assign19570_e18741, assign19570_e18741_d_n2, assign19570_e18741_d_n3, assign19570_e18741_d_n4, assign19570_e18741_d_n7, assign19570_e18741_d_n11, assign19570_e18741_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsx, locals.var_fn241_calc_iq__vsx_dn2, locals.var_fn241_calc_iq__vsx_dn3, locals.var_fn241_calc_iq__vsx_dn4, locals.var_fn241_calc_iq__vsx_dn7, locals.var_fn241_calc_iq__vsx_dn11, locals.var_fn241_calc_iq__vsx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx = assign19570_e18741;
        locals.var_fn241_calc_iq__vsx_dn2 = assign19570_e18741_d_n2;
        locals.var_fn241_calc_iq__vsx_dn3 = assign19570_e18741_d_n3;
        locals.var_fn241_calc_iq__vsx_dn4 = assign19570_e18741_d_n4;
        locals.var_fn241_calc_iq__vsx_dn7 = assign19570_e18741_d_n7;
        locals.var_fn241_calc_iq__vsx_dn11 = assign19570_e18741_d_n11;
        locals.var_fn241_calc_iq__vsx_dn12 = assign19570_e18741_d_n12;

        let (assign19580_e18745, assign19580_e18745_d_n2, assign19580_e18745_d_n3, assign19580_e18745_d_n4, assign19580_e18745_d_n7, assign19580_e18745_d_n11, assign19580_e18745_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign19580_e18745;
        locals.var_fn241_calc_iq__ffd_dn2 = assign19580_e18745_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign19580_e18745_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign19580_e18745_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign19580_e18745_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign19580_e18745_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign19580_e18745_d_n12;

    }

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19590_e18749, assign19590_e18749_d_n2, assign19590_e18749_d_n3, assign19590_e18749_d_n4, assign19590_e18749_d_n7, assign19590_e18749_d_n11, assign19590_e18749_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etad, locals.var_fn241_calc_iq__etad_dn2, locals.var_fn241_calc_iq__etad_dn3, locals.var_fn241_calc_iq__etad_dn4, locals.var_fn241_calc_iq__etad_dn7, locals.var_fn241_calc_iq__etad_dn11, locals.var_fn241_calc_iq__etad_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad = assign19590_e18749;
        locals.var_fn241_calc_iq__etad_dn2 = assign19590_e18749_d_n2;
        locals.var_fn241_calc_iq__etad_dn3 = assign19590_e18749_d_n3;
        locals.var_fn241_calc_iq__etad_dn4 = assign19590_e18749_d_n4;
        locals.var_fn241_calc_iq__etad_dn7 = assign19590_e18749_d_n7;
        locals.var_fn241_calc_iq__etad_dn11 = assign19590_e18749_d_n11;
        locals.var_fn241_calc_iq__etad_dn12 = assign19590_e18749_d_n12;

        let (assign19600_e18753, assign19600_e18753_d_n2, assign19600_e18753_d_n3, assign19600_e18753_d_n4, assign19600_e18753_d_n7, assign19600_e18753_d_n11, assign19600_e18753_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign19600_e18753;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign19600_e18753_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign19600_e18753_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign19600_e18753_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign19600_e18753_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign19600_e18753_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign19600_e18753_d_n12;

        let (assign19610_e18757, assign19610_e18757_d_n2, assign19610_e18757_d_n3, assign19610_e18757_d_n4, assign19610_e18757_d_n7, assign19610_e18757_d_n11, assign19610_e18757_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsc, locals.var_fn241_calc_iq__vdsc_dn2, locals.var_fn241_calc_iq__vdsc_dn3, locals.var_fn241_calc_iq__vdsc_dn4, locals.var_fn241_calc_iq__vdsc_dn7, locals.var_fn241_calc_iq__vdsc_dn11, locals.var_fn241_calc_iq__vdsc_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsc = assign19610_e18757;
        locals.var_fn241_calc_iq__vdsc_dn2 = assign19610_e18757_d_n2;
        locals.var_fn241_calc_iq__vdsc_dn3 = assign19610_e18757_d_n3;
        locals.var_fn241_calc_iq__vdsc_dn4 = assign19610_e18757_d_n4;
        locals.var_fn241_calc_iq__vdsc_dn7 = assign19610_e18757_d_n7;
        locals.var_fn241_calc_iq__vdsc_dn11 = assign19610_e18757_d_n11;
        locals.var_fn241_calc_iq__vdsc_dn12 = assign19610_e18757_d_n12;

        let (assign19640_e18769, assign19640_e18769_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats0, locals.var_fn241_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vdsats0 = assign19640_e18769;
        locals.var_fn241_calc_iq__vdsats0_dn4 = assign19640_e18769_d_n4;

        let (assign19650_e18773, assign19650_e18773_d_n2, assign19650_e18773_d_n4, assign19650_e18773_d_n7, assign19650_e18773_d_n11, assign19650_e18773_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsats10, locals.var_fn241_calc_iq__vdsats10_dn2, locals.var_fn241_calc_iq__vdsats10_dn4, locals.var_fn241_calc_iq__vdsats10_dn7, locals.var_fn241_calc_iq__vdsats10_dn11, locals.var_fn241_calc_iq__vdsats10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats10 = assign19650_e18773;
        locals.var_fn241_calc_iq__vdsats10_dn2 = assign19650_e18773_d_n2;
        locals.var_fn241_calc_iq__vdsats10_dn4 = assign19650_e18773_d_n4;
        locals.var_fn241_calc_iq__vdsats10_dn7 = assign19650_e18773_d_n7;
        locals.var_fn241_calc_iq__vdsats10_dn11 = assign19650_e18773_d_n11;
        locals.var_fn241_calc_iq__vdsats10_dn12 = assign19650_e18773_d_n12;

        let (assign19660_e18777, assign19660_e18777_d_n2, assign19660_e18777_d_n4, assign19660_e18777_d_n7, assign19660_e18777_d_n11, assign19660_e18777_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdsat10, locals.var_fn241_calc_iq__vdsat10_dn2, locals.var_fn241_calc_iq__vdsat10_dn4, locals.var_fn241_calc_iq__vdsat10_dn7, locals.var_fn241_calc_iq__vdsat10_dn11, locals.var_fn241_calc_iq__vdsat10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat10 = assign19660_e18777;
        locals.var_fn241_calc_iq__vdsat10_dn2 = assign19660_e18777_d_n2;
        locals.var_fn241_calc_iq__vdsat10_dn4 = assign19660_e18777_d_n4;
        locals.var_fn241_calc_iq__vdsat10_dn7 = assign19660_e18777_d_n7;
        locals.var_fn241_calc_iq__vdsat10_dn11 = assign19660_e18777_d_n11;
        locals.var_fn241_calc_iq__vdsat10_dn12 = assign19660_e18777_d_n12;

        let (assign19670_e18781, assign19670_e18781_d_n2, assign19670_e18781_d_n4, assign19670_e18781_d_n7, assign19670_e18781_d_n11, assign19670_e18781_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fsd0, locals.var_fn241_calc_iq__fsd0_dn2, locals.var_fn241_calc_iq__fsd0_dn4, locals.var_fn241_calc_iq__fsd0_dn7, locals.var_fn241_calc_iq__fsd0_dn11, locals.var_fn241_calc_iq__fsd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd0 = assign19670_e18781;
        locals.var_fn241_calc_iq__fsd0_dn2 = assign19670_e18781_d_n2;
        locals.var_fn241_calc_iq__fsd0_dn4 = assign19670_e18781_d_n4;
        locals.var_fn241_calc_iq__fsd0_dn7 = assign19670_e18781_d_n7;
        locals.var_fn241_calc_iq__fsd0_dn11 = assign19670_e18781_d_n11;
        locals.var_fn241_calc_iq__fsd0_dn12 = assign19670_e18781_d_n12;

        let (assign19680_e18785, assign19680_e18785_d_n2, assign19680_e18785_d_n4, assign19680_e18785_d_n7, assign19680_e18785_d_n11, assign19680_e18785_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vdx0, locals.var_fn241_calc_iq__vdx0_dn2, locals.var_fn241_calc_iq__vdx0_dn4, locals.var_fn241_calc_iq__vdx0_dn7, locals.var_fn241_calc_iq__vdx0_dn11, locals.var_fn241_calc_iq__vdx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx0 = assign19680_e18785;
        locals.var_fn241_calc_iq__vdx0_dn2 = assign19680_e18785_d_n2;
        locals.var_fn241_calc_iq__vdx0_dn4 = assign19680_e18785_d_n4;
        locals.var_fn241_calc_iq__vdx0_dn7 = assign19680_e18785_d_n7;
        locals.var_fn241_calc_iq__vdx0_dn11 = assign19680_e18785_d_n11;
        locals.var_fn241_calc_iq__vdx0_dn12 = assign19680_e18785_d_n12;

        let (assign19690_e18789, assign19690_e18789_d_n2, assign19690_e18789_d_n4, assign19690_e18789_d_n7, assign19690_e18789_d_n11, assign19690_e18789_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__fds0, locals.var_fn241_calc_iq__fds0_dn2, locals.var_fn241_calc_iq__fds0_dn4, locals.var_fn241_calc_iq__fds0_dn7, locals.var_fn241_calc_iq__fds0_dn11, locals.var_fn241_calc_iq__fds0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds0 = assign19690_e18789;
        locals.var_fn241_calc_iq__fds0_dn2 = assign19690_e18789_d_n2;
        locals.var_fn241_calc_iq__fds0_dn4 = assign19690_e18789_d_n4;
        locals.var_fn241_calc_iq__fds0_dn7 = assign19690_e18789_d_n7;
        locals.var_fn241_calc_iq__fds0_dn11 = assign19690_e18789_d_n11;
        locals.var_fn241_calc_iq__fds0_dn12 = assign19690_e18789_d_n12;

        let (assign19700_e18793, assign19700_e18793_d_n2, assign19700_e18793_d_n4, assign19700_e18793_d_n7, assign19700_e18793_d_n11, assign19700_e18793_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsx0, locals.var_fn241_calc_iq__vsx0_dn2, locals.var_fn241_calc_iq__vsx0_dn4, locals.var_fn241_calc_iq__vsx0_dn7, locals.var_fn241_calc_iq__vsx0_dn11, locals.var_fn241_calc_iq__vsx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx0 = assign19700_e18793;
        locals.var_fn241_calc_iq__vsx0_dn2 = assign19700_e18793_d_n2;
        locals.var_fn241_calc_iq__vsx0_dn4 = assign19700_e18793_d_n4;
        locals.var_fn241_calc_iq__vsx0_dn7 = assign19700_e18793_d_n7;
        locals.var_fn241_calc_iq__vsx0_dn11 = assign19700_e18793_d_n11;
        locals.var_fn241_calc_iq__vsx0_dn12 = assign19700_e18793_d_n12;

        let (assign19710_e18797, assign19710_e18797_d_n2, assign19710_e18797_d_n4, assign19710_e18797_d_n7, assign19710_e18797_d_n11, assign19710_e18797_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign19710_e18797;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign19710_e18797_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign19710_e18797_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign19710_e18797_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign19710_e18797_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign19710_e18797_d_n12;

        let (assign19720_e18801, assign19720_e18801_d_n2, assign19720_e18801_d_n4, assign19720_e18801_d_n7, assign19720_e18801_d_n11, assign19720_e18801_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etad0, locals.var_fn241_calc_iq__etad0_dn2, locals.var_fn241_calc_iq__etad0_dn4, locals.var_fn241_calc_iq__etad0_dn7, locals.var_fn241_calc_iq__etad0_dn11, locals.var_fn241_calc_iq__etad0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad0 = assign19720_e18801;
        locals.var_fn241_calc_iq__etad0_dn2 = assign19720_e18801_d_n2;
        locals.var_fn241_calc_iq__etad0_dn4 = assign19720_e18801_d_n4;
        locals.var_fn241_calc_iq__etad0_dn7 = assign19720_e18801_d_n7;
        locals.var_fn241_calc_iq__etad0_dn11 = assign19720_e18801_d_n11;
        locals.var_fn241_calc_iq__etad0_dn12 = assign19720_e18801_d_n12;

        let (assign19730_e18805, assign19730_e18805_d_n2, assign19730_e18805_d_n4, assign19730_e18805_d_n7, assign19730_e18805_d_n11, assign19730_e18805_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign19730_e18805;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign19730_e18805_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign19730_e18805_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign19730_e18805_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign19730_e18805_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign19730_e18805_d_n12;

        let (assign19740_e18809, assign19740_e18809_d_n2, assign19740_e18809_d_n4, assign19740_e18809_d_n7, assign19740_e18809_d_n11, assign19740_e18809_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qs2, locals.var_fn241_calc_iq__qs2_dn2, locals.var_fn241_calc_iq__qs2_dn4, locals.var_fn241_calc_iq__qs2_dn7, locals.var_fn241_calc_iq__qs2_dn11, locals.var_fn241_calc_iq__qs2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs2 = assign19740_e18809;
        locals.var_fn241_calc_iq__qs2_dn2 = assign19740_e18809_d_n2;
        locals.var_fn241_calc_iq__qs2_dn4 = assign19740_e18809_d_n4;
        locals.var_fn241_calc_iq__qs2_dn7 = assign19740_e18809_d_n7;
        locals.var_fn241_calc_iq__qs2_dn11 = assign19740_e18809_d_n11;
        locals.var_fn241_calc_iq__qs2_dn12 = assign19740_e18809_d_n12;

        let (assign19750_e18813, assign19750_e18813_d_n2, assign19750_e18813_d_n4, assign19750_e18813_d_n7, assign19750_e18813_d_n11, assign19750_e18813_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qs3, locals.var_fn241_calc_iq__qs3_dn2, locals.var_fn241_calc_iq__qs3_dn4, locals.var_fn241_calc_iq__qs3_dn7, locals.var_fn241_calc_iq__qs3_dn11, locals.var_fn241_calc_iq__qs3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs3 = assign19750_e18813;
        locals.var_fn241_calc_iq__qs3_dn2 = assign19750_e18813_d_n2;
        locals.var_fn241_calc_iq__qs3_dn4 = assign19750_e18813_d_n4;
        locals.var_fn241_calc_iq__qs3_dn7 = assign19750_e18813_d_n7;
        locals.var_fn241_calc_iq__qs3_dn11 = assign19750_e18813_d_n11;
        locals.var_fn241_calc_iq__qs3_dn12 = assign19750_e18813_d_n12;

        let (assign19760_e18817, assign19760_e18817_d_n2, assign19760_e18817_d_n4, assign19760_e18817_d_n7, assign19760_e18817_d_n11, assign19760_e18817_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd2, locals.var_fn241_calc_iq__qd2_dn2, locals.var_fn241_calc_iq__qd2_dn4, locals.var_fn241_calc_iq__qd2_dn7, locals.var_fn241_calc_iq__qd2_dn11, locals.var_fn241_calc_iq__qd2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd2 = assign19760_e18817;
        locals.var_fn241_calc_iq__qd2_dn2 = assign19760_e18817_d_n2;
        locals.var_fn241_calc_iq__qd2_dn4 = assign19760_e18817_d_n4;
        locals.var_fn241_calc_iq__qd2_dn7 = assign19760_e18817_d_n7;
        locals.var_fn241_calc_iq__qd2_dn11 = assign19760_e18817_d_n11;
        locals.var_fn241_calc_iq__qd2_dn12 = assign19760_e18817_d_n12;

        let (assign19770_e18821, assign19770_e18821_d_n2, assign19770_e18821_d_n4, assign19770_e18821_d_n7, assign19770_e18821_d_n11, assign19770_e18821_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd3, locals.var_fn241_calc_iq__qd3_dn2, locals.var_fn241_calc_iq__qd3_dn4, locals.var_fn241_calc_iq__qd3_dn7, locals.var_fn241_calc_iq__qd3_dn11, locals.var_fn241_calc_iq__qd3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd3 = assign19770_e18821;
        locals.var_fn241_calc_iq__qd3_dn2 = assign19770_e18821_d_n2;
        locals.var_fn241_calc_iq__qd3_dn4 = assign19770_e18821_d_n4;
        locals.var_fn241_calc_iq__qd3_dn7 = assign19770_e18821_d_n7;
        locals.var_fn241_calc_iq__qd3_dn11 = assign19770_e18821_d_n11;
        locals.var_fn241_calc_iq__qd3_dn12 = assign19770_e18821_d_n12;

        let (assign19780_e18825, assign19780_e18825_d_n2, assign19780_e18825_d_n4, assign19780_e18825_d_n7, assign19780_e18825_d_n11, assign19780_e18825_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsqd, locals.var_fn241_calc_iq__qsqd_dn2, locals.var_fn241_calc_iq__qsqd_dn4, locals.var_fn241_calc_iq__qsqd_dn7, locals.var_fn241_calc_iq__qsqd_dn11, locals.var_fn241_calc_iq__qsqd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsqd = assign19780_e18825;
        locals.var_fn241_calc_iq__qsqd_dn2 = assign19780_e18825_d_n2;
        locals.var_fn241_calc_iq__qsqd_dn4 = assign19780_e18825_d_n4;
        locals.var_fn241_calc_iq__qsqd_dn7 = assign19780_e18825_d_n7;
        locals.var_fn241_calc_iq__qsqd_dn11 = assign19780_e18825_d_n11;
        locals.var_fn241_calc_iq__qsqd_dn12 = assign19780_e18825_d_n12;

        let (assign19790_e18829, assign19790_e18829_d_n2, assign19790_e18829_d_n4, assign19790_e18829_d_n7, assign19790_e18829_d_n11, assign19790_e18829_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qinvdd, locals.var_fn241_calc_iq__qinvdd_dn2, locals.var_fn241_calc_iq__qinvdd_dn4, locals.var_fn241_calc_iq__qinvdd_dn7, locals.var_fn241_calc_iq__qinvdd_dn11, locals.var_fn241_calc_iq__qinvdd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvdd = assign19790_e18829;
        locals.var_fn241_calc_iq__qinvdd_dn2 = assign19790_e18829_d_n2;
        locals.var_fn241_calc_iq__qinvdd_dn4 = assign19790_e18829_d_n4;
        locals.var_fn241_calc_iq__qinvdd_dn7 = assign19790_e18829_d_n7;
        locals.var_fn241_calc_iq__qinvdd_dn11 = assign19790_e18829_d_n11;
        locals.var_fn241_calc_iq__qinvdd_dn12 = assign19790_e18829_d_n12;

        let (assign19800_e18833, assign19800_e18833_d_n2, assign19800_e18833_d_n4, assign19800_e18833_d_n7, assign19800_e18833_d_n11, assign19800_e18833_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd1, locals.var_fn241_calc_iq__qd1_dn2, locals.var_fn241_calc_iq__qd1_dn4, locals.var_fn241_calc_iq__qd1_dn7, locals.var_fn241_calc_iq__qd1_dn11, locals.var_fn241_calc_iq__qd1_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd1 = assign19800_e18833;
        locals.var_fn241_calc_iq__qd1_dn2 = assign19800_e18833_d_n2;
        locals.var_fn241_calc_iq__qd1_dn4 = assign19800_e18833_d_n4;
        locals.var_fn241_calc_iq__qd1_dn7 = assign19800_e18833_d_n7;
        locals.var_fn241_calc_iq__qd1_dn11 = assign19800_e18833_d_n11;
        locals.var_fn241_calc_iq__qd1_dn12 = assign19800_e18833_d_n12;

        let (assign19810_e18837, assign19810_e18837_d_n2, assign19810_e18837_d_n4, assign19810_e18837_d_n7, assign19810_e18837_d_n11, assign19810_e18837_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qs, locals.var_fn241_calc_iq__qs_dn2, locals.var_fn241_calc_iq__qs_dn4, locals.var_fn241_calc_iq__qs_dn7, locals.var_fn241_calc_iq__qs_dn11, locals.var_fn241_calc_iq__qs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs = assign19810_e18837;
        locals.var_fn241_calc_iq__qs_dn2 = assign19810_e18837_d_n2;
        locals.var_fn241_calc_iq__qs_dn4 = assign19810_e18837_d_n4;
        locals.var_fn241_calc_iq__qs_dn7 = assign19810_e18837_d_n7;
        locals.var_fn241_calc_iq__qs_dn11 = assign19810_e18837_d_n11;
        locals.var_fn241_calc_iq__qs_dn12 = assign19810_e18837_d_n12;

        let (assign19820_e18841, assign19820_e18841_d_n2, assign19820_e18841_d_n4, assign19820_e18841_d_n7, assign19820_e18841_d_n11, assign19820_e18841_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qd, locals.var_fn241_calc_iq__qd_dn2, locals.var_fn241_calc_iq__qd_dn4, locals.var_fn241_calc_iq__qd_dn7, locals.var_fn241_calc_iq__qd_dn11, locals.var_fn241_calc_iq__qd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd = assign19820_e18841;
        locals.var_fn241_calc_iq__qd_dn2 = assign19820_e18841_d_n2;
        locals.var_fn241_calc_iq__qd_dn4 = assign19820_e18841_d_n4;
        locals.var_fn241_calc_iq__qd_dn7 = assign19820_e18841_d_n7;
        locals.var_fn241_calc_iq__qd_dn11 = assign19820_e18841_d_n11;
        locals.var_fn241_calc_iq__qd_dn12 = assign19820_e18841_d_n12;

        let (assign19830_e18845, assign19830_e18845_d_n2, assign19830_e18845_d_n4, assign19830_e18845_d_n7, assign19830_e18845_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etac, locals.var_fn241_calc_iq__etac_dn2, locals.var_fn241_calc_iq__etac_dn4, locals.var_fn241_calc_iq__etac_dn7, locals.var_fn241_calc_iq__etac_dn12,)
    }
};
        locals.var_fn241_calc_iq__etac = assign19830_e18845;
        locals.var_fn241_calc_iq__etac_dn2 = assign19830_e18845_d_n2;
        locals.var_fn241_calc_iq__etac_dn4 = assign19830_e18845_d_n4;
        locals.var_fn241_calc_iq__etac_dn7 = assign19830_e18845_d_n7;
        locals.var_fn241_calc_iq__etac_dn12 = assign19830_e18845_d_n12;

        let (assign19840_e18849, assign19840_e18849_d_n3, assign19840_e18849_d_n4, assign19840_e18849_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etab, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, locals.var_fn241_calc_iq__etab_dn12,)
    }
};
        locals.var_fn241_calc_iq__etab = assign19840_e18849;
        locals.var_fn241_calc_iq__etab_dn3 = assign19840_e18849_d_n3;
        locals.var_fn241_calc_iq__etab_dn4 = assign19840_e18849_d_n4;
        locals.var_fn241_calc_iq__etab_dn12 = assign19840_e18849_d_n12;

        let (assign19850_e18853, assign19850_e18853_d_n2, assign19850_e18853_d_n4, assign19850_e18853_d_n7, assign19850_e18853_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, locals.var_fn241_calc_iq__etags_dn12,)
    }
};
        locals.var_fn241_calc_iq__etags = assign19850_e18853;
        locals.var_fn241_calc_iq__etags_dn2 = assign19850_e18853_d_n2;
        locals.var_fn241_calc_iq__etags_dn4 = assign19850_e18853_d_n4;
        locals.var_fn241_calc_iq__etags_dn7 = assign19850_e18853_d_n7;
        locals.var_fn241_calc_iq__etags_dn12 = assign19850_e18853_d_n12;

        let (assign19860_e18857, assign19860_e18857_d_n2, assign19860_e18857_d_n3, assign19860_e18857_d_n4, assign19860_e18857_d_n7, assign19860_e18857_d_n11, assign19860_e18857_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign19860_e18857;
        locals.var_fn241_calc_iq__exparg_dn2 = assign19860_e18857_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign19860_e18857_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign19860_e18857_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign19860_e18857_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign19860_e18857_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign19860_e18857_d_n12;

        let (assign19870_e18861, assign19870_e18861_d_n2, assign19870_e18861_d_n3, assign19870_e18861_d_n4, assign19870_e18861_d_n7, assign19870_e18861_d_n11, assign19870_e18861_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__myarg, locals.var_fn241_calc_iq__myarg_dn2, locals.var_fn241_calc_iq__myarg_dn3, locals.var_fn241_calc_iq__myarg_dn4, locals.var_fn241_calc_iq__myarg_dn7, locals.var_fn241_calc_iq__myarg_dn11, locals.var_fn241_calc_iq__myarg_dn12,)
    }
};
        locals.var_fn241_calc_iq__myarg = assign19870_e18861;
        locals.var_fn241_calc_iq__myarg_dn2 = assign19870_e18861_d_n2;
        locals.var_fn241_calc_iq__myarg_dn3 = assign19870_e18861_d_n3;
        locals.var_fn241_calc_iq__myarg_dn4 = assign19870_e18861_d_n4;
        locals.var_fn241_calc_iq__myarg_dn7 = assign19870_e18861_d_n7;
        locals.var_fn241_calc_iq__myarg_dn11 = assign19870_e18861_d_n11;
        locals.var_fn241_calc_iq__myarg_dn12 = assign19870_e18861_d_n12;

        let (assign19880_e18865, assign19880_e18865_d_n11, assign19880_e18865_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__absvdsin, locals.var_fn241_calc_iq__absvdsin_dn11, locals.var_fn241_calc_iq__absvdsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__absvdsin = assign19880_e18865;
        locals.var_fn241_calc_iq__absvdsin_dn11 = assign19880_e18865_d_n11;
        locals.var_fn241_calc_iq__absvdsin_dn12 = assign19880_e18865_d_n12;

        let (assign19890_e18869, assign19890_e18869_d_n2, assign19890_e18869_d_n7, assign19890_e18869_d_n11, assign19890_e18869_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vgdin, locals.var_fn241_calc_iq__vgdin_dn2, locals.var_fn241_calc_iq__vgdin_dn7, locals.var_fn241_calc_iq__vgdin_dn11, locals.var_fn241_calc_iq__vgdin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vgdin = assign19890_e18869;
        locals.var_fn241_calc_iq__vgdin_dn2 = assign19890_e18869_d_n2;
        locals.var_fn241_calc_iq__vgdin_dn7 = assign19890_e18869_d_n7;
        locals.var_fn241_calc_iq__vgdin_dn11 = assign19890_e18869_d_n11;
        locals.var_fn241_calc_iq__vgdin_dn12 = assign19890_e18869_d_n12;

        let (assign19900_e18873, assign19900_e18873_d_n2, assign19900_e18873_d_n4, assign19900_e18873_d_n7, assign19900_e18873_d_n11, assign19900_e18873_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign19900_e18873;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign19900_e18873_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign19900_e18873_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign19900_e18873_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign19900_e18873_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign19900_e18873_d_n12;

        let (assign19910_e18877, assign19910_e18877_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__myarg0, locals.var_fn241_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn241_calc_iq__myarg0 = assign19910_e18877;
        locals.var_fn241_calc_iq__myarg0_dn4 = assign19910_e18877_d_n4;

        let (assign19920_e18904, assign19920_e18904_d_n11, assign19920_e18904_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign19920_e18902, assign19920_e18902_d_n11, assign19920_e18902_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign19920_e18886: f64 = (0.001 / p.p53);
                let assign19920_e18888: f64 = (assign19920_e18886 * locals.var_fn241_calc_iq__vdsin);
                let assign19920_e18889: f64 = (assign19920_e18888).tanh();
                let assign19920_e18890: f64 = (locals.var_fn241_calc_iq__vdsin * assign19920_e18889);
                (assign19920_e18890, ((locals.var_fn241_calc_iq__vdsin_dn11 * assign19920_e18889) + (locals.var_fn241_calc_iq__vdsin * ((assign19920_e18886 * locals.var_fn241_calc_iq__vdsin_dn11) / ((assign19920_e18888).cosh() * (assign19920_e18888).cosh())))), ((locals.var_fn241_calc_iq__vdsin_dn12 * assign19920_e18889) + (locals.var_fn241_calc_iq__vdsin * ((assign19920_e18886 * locals.var_fn241_calc_iq__vdsin_dn12) / ((assign19920_e18888).cosh() * (assign19920_e18888).cosh())))),)
            } else {
                let (assign19920_e18901, assign19920_e18901_d_n11, assign19920_e18901_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign19920_e18896: f64 = (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsin);
                        let assign19920_e18898: f64 = (assign19920_e18896 + p.p53);
                        let assign19920_e18899: f64 = (assign19920_e18898).sqrt();
                        (assign19920_e18899, (((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsin) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsin_dn11)) / (2.0 * assign19920_e18899)), (((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsin) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsin_dn12)) / (2.0 * assign19920_e18899)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign19920_e18901, assign19920_e18901_d_n11, assign19920_e18901_d_n12,)
            }
        };
        (assign19920_e18902, assign19920_e18902_d_n11, assign19920_e18902_d_n12,)
    } else {
        (locals.var_fn241_calc_iq__absvdsin, locals.var_fn241_calc_iq__absvdsin_dn11, locals.var_fn241_calc_iq__absvdsin_dn12,)
    }
};
        locals.var_fn241_calc_iq__absvdsin = assign19920_e18904;
        locals.var_fn241_calc_iq__absvdsin_dn11 = assign19920_e18904_d_n11;
        locals.var_fn241_calc_iq__absvdsin_dn12 = assign19920_e18904_d_n12;

        let (assign19930_e18910, assign19930_e18910_d_n2, assign19930_e18910_d_n7, assign19930_e18910_d_n11, assign19930_e18910_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19930_e18908: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vdsin);
        (assign19930_e18908, locals.var_fn241_calc_iq__vgsin_dn2, locals.var_fn241_calc_iq__vgsin_dn7, (-locals.var_fn241_calc_iq__vdsin_dn11), (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vdsin_dn12),)
    } else {
        (locals.var_fn241_calc_iq__vgdin, locals.var_fn241_calc_iq__vgdin_dn2, locals.var_fn241_calc_iq__vgdin_dn7, locals.var_fn241_calc_iq__vgdin_dn11, locals.var_fn241_calc_iq__vgdin_dn12,)
    }
};
        locals.var_fn241_calc_iq__vgdin = assign19930_e18910;
        locals.var_fn241_calc_iq__vgdin_dn2 = assign19930_e18910_d_n2;
        locals.var_fn241_calc_iq__vgdin_dn7 = assign19930_e18910_d_n7;
        locals.var_fn241_calc_iq__vgdin_dn11 = assign19930_e18910_d_n11;
        locals.var_fn241_calc_iq__vgdin_dn12 = assign19930_e18910_d_n12;

        let (assign19940_e18916, assign19940_e18916_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19940_e18914: f64 = (locals.var_fn241_calc_iq__alpha * locals.var_fn241_calc_iq__phitin);
        (assign19940_e18914, (locals.var_fn241_calc_iq__alpha * locals.var_fn241_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn241_calc_iq__alpha_phit, locals.var_fn241_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn241_calc_iq__alpha_phit = assign19940_e18916;
        locals.var_fn241_calc_iq__alpha_phit_dn4 = assign19940_e18916_d_n4;

        let (assign19950_e18928, assign19950_e18928_d_n4, assign19950_e18928_d_n11, assign19950_e18928_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19950_e18921: f64 = (2.302585092994046 * locals.var_fn241_calc_iq__phitin);
        let assign19950_e18922: f64 = (locals.var_fn241_calc_iq__ss / assign19950_e18921);
        let assign19950_e18925: f64 = (locals.var_fn241_calc_iq__nd * locals.var_fn241_calc_iq__absvdsin);
        let assign19950_e18926: f64 = (assign19950_e18922 + assign19950_e18925);
        (assign19950_e18926, (-((locals.var_fn241_calc_iq__ss * (2.302585092994046 * locals.var_fn241_calc_iq__phitin_dn4)) / (assign19950_e18921 * assign19950_e18921))), (locals.var_fn241_calc_iq__nd * locals.var_fn241_calc_iq__absvdsin_dn11), (locals.var_fn241_calc_iq__nd * locals.var_fn241_calc_iq__absvdsin_dn12),)
    } else {
        (locals.var_fn241_calc_iq__n, locals.var_fn241_calc_iq__n_dn4, locals.var_fn241_calc_iq__n_dn11, locals.var_fn241_calc_iq__n_dn12,)
    }
};
        locals.var_fn241_calc_iq__n = assign19950_e18928;
        locals.var_fn241_calc_iq__n_dn4 = assign19950_e18928_d_n4;
        locals.var_fn241_calc_iq__n_dn11 = assign19950_e18928_d_n11;
        locals.var_fn241_calc_iq__n_dn12 = assign19950_e18928_d_n12;

        let (assign19960_e18938, assign19960_e18938_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19960_e18934: f64 = (locals.var_fn241_calc_iq__tambin - locals.var_fn241_calc_iq__tnomin);
        let assign19960_e18935: f64 = (locals.var_fn241_calc_iq__vtzeta * assign19960_e18934);
        let assign19960_e18936: f64 = (locals.var_fn241_calc_iq__vto + assign19960_e18935);
        (assign19960_e18936, (locals.var_fn241_calc_iq__vtzeta * locals.var_fn241_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn241_calc_iq__vtof, locals.var_fn241_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn241_calc_iq__vtof = assign19960_e18938;
        locals.var_fn241_calc_iq__vtof_dn4 = assign19960_e18938_d_n4;

    }

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19970_e18946, assign19970_e18946_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign19970_e18942: f64 = (locals.var_fn241_calc_iq__tambin / locals.var_fn241_calc_iq__tnomin);
        let assign19970_e18944: f64 = (assign19970_e18942).powf(locals.var_fn241_calc_iq__epsilon);
        (assign19970_e18944, if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn241_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__epsilon * ((assign19970_e18942).powf(locals.var_fn241_calc_iq__epsilon - 1.0) * (locals.var_fn241_calc_iq__tambin_dn4 / locals.var_fn241_calc_iq__tnomin))) } } else { (assign19970_e18944 * (locals.var_fn241_calc_iq__epsilon * ((locals.var_fn241_calc_iq__tambin_dn4 / locals.var_fn241_calc_iq__tnomin) / assign19970_e18942))) },)
    } else {
        (locals.var_fn241_calc_iq__tfacmobin, locals.var_fn241_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn241_calc_iq__tfacmobin = assign19970_e18946;
        locals.var_fn241_calc_iq__tfacmobin_dn4 = assign19970_e18946_d_n4;

        let assign19980_e18949: f64 = if locals.var_fn241_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign19980_e18949;

        let (assign19990_e18967, assign19990_e18967_d_n11, assign19990_e18967_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard242 != 0.0)) {
        let assign19990_e18957: f64 = (locals.var_fn241_calc_iq__absvdsin / locals.var_fn241_calc_iq__dibsat);
        let assign19990_e18959: f64 = (assign19990_e18957).powf(locals.var_fn241_calc_iq__beta);
        let assign19990_e18960: f64 = (1.0 + assign19990_e18959);
        let assign19990_e18963: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign19990_e18964: f64 = (assign19990_e18960).powf(assign19990_e18963);
        let assign19990_e18965: f64 = (locals.var_fn241_calc_iq__absvdsin / assign19990_e18964);
        (assign19990_e18965, (((locals.var_fn241_calc_iq__absvdsin_dn11 * assign19990_e18964) - (locals.var_fn241_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign19990_e18963) as f64).is_finite() && ((assign19990_e18963) as f64).fract() == 0.0 { if assign19990_e18963 == 0.0 { 0.0 } else { (assign19990_e18963 * ((assign19990_e18960).powf(assign19990_e18963 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) })) } } else { (assign19990_e18964 * (assign19990_e18963 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn11 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) } / assign19990_e18960))) })) / (assign19990_e18964 * assign19990_e18964)), (((locals.var_fn241_calc_iq__absvdsin_dn12 * assign19990_e18964) - (locals.var_fn241_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign19990_e18963) as f64).is_finite() && ((assign19990_e18963) as f64).fract() == 0.0 { if assign19990_e18963 == 0.0 { 0.0 } else { (assign19990_e18963 * ((assign19990_e18960).powf(assign19990_e18963 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) })) } } else { (assign19990_e18964 * (assign19990_e18963 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign19990_e18957).powf(locals.var_fn241_calc_iq__beta - 1.0) * (locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat))) } } else { (assign19990_e18959 * (locals.var_fn241_calc_iq__beta * ((locals.var_fn241_calc_iq__absvdsin_dn12 / locals.var_fn241_calc_iq__dibsat) / assign19990_e18957))) } / assign19990_e18960))) })) / (assign19990_e18964 * assign19990_e18964)),)
    } else {
        (locals.var_fn241_calc_iq__vsatdibl, locals.var_fn241_calc_iq__vsatdibl_dn11, locals.var_fn241_calc_iq__vsatdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsatdibl = assign19990_e18967;
        locals.var_fn241_calc_iq__vsatdibl_dn11 = assign19990_e18967_d_n11;
        locals.var_fn241_calc_iq__vsatdibl_dn12 = assign19990_e18967_d_n12;

        let (assign20000_e18974, assign20000_e18974_d_n11, assign20000_e18974_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard242 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__vsatdibl, locals.var_fn241_calc_iq__vsatdibl_dn11, locals.var_fn241_calc_iq__vsatdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsatdibl = assign20000_e18974;
        locals.var_fn241_calc_iq__vsatdibl_dn11 = assign20000_e18974_d_n11;
        locals.var_fn241_calc_iq__vsatdibl_dn12 = assign20000_e18974_d_n12;

        let (assign20010_e18984, assign20010_e18984_d_n11, assign20010_e18984_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20010_e18979: f64 = (locals.var_fn241_calc_iq__vsatdibl * locals.var_fn241_calc_iq__delta2);
        let assign20010_e18980: f64 = (locals.var_fn241_calc_iq__delta1 - assign20010_e18979);
        let assign20010_e18982: f64 = (assign20010_e18980 * locals.var_fn241_calc_iq__absvdsin);
        (assign20010_e18982, (((-(locals.var_fn241_calc_iq__vsatdibl_dn11 * locals.var_fn241_calc_iq__delta2)) * locals.var_fn241_calc_iq__absvdsin) + (assign20010_e18980 * locals.var_fn241_calc_iq__absvdsin_dn11)), (((-(locals.var_fn241_calc_iq__vsatdibl_dn12 * locals.var_fn241_calc_iq__delta2)) * locals.var_fn241_calc_iq__absvdsin) + (assign20010_e18980 * locals.var_fn241_calc_iq__absvdsin_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__delta, locals.var_fn241_calc_iq__delta_dn11, locals.var_fn241_calc_iq__delta_dn12,)
    }
};
        locals.var_fn241_calc_iq__delta = assign20010_e18984;
        locals.var_fn241_calc_iq__delta_dn11 = assign20010_e18984_d_n11;
        locals.var_fn241_calc_iq__delta_dn12 = assign20010_e18984_d_n12;

        let (assign20020_e18990, assign20020_e18990_d_n4, assign20020_e18990_d_n11, assign20020_e18990_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20020_e18988: f64 = (locals.var_fn241_calc_iq__vtof - locals.var_fn241_calc_iq__delta);
        (assign20020_e18988, locals.var_fn241_calc_iq__vtof_dn4, (-locals.var_fn241_calc_iq__delta_dn11), (-locals.var_fn241_calc_iq__delta_dn12),)
    } else {
        (locals.var_fn241_calc_iq__vtdibl, locals.var_fn241_calc_iq__vtdibl_dn4, locals.var_fn241_calc_iq__vtdibl_dn11, locals.var_fn241_calc_iq__vtdibl_dn12,)
    }
};
        locals.var_fn241_calc_iq__vtdibl = assign20020_e18990;
        locals.var_fn241_calc_iq__vtdibl_dn4 = assign20020_e18990_d_n4;
        locals.var_fn241_calc_iq__vtdibl_dn11 = assign20020_e18990_d_n11;
        locals.var_fn241_calc_iq__vtdibl_dn12 = assign20020_e18990_d_n12;

        let (assign20030_e18998, assign20030_e18998_d_n4, assign20030_e18998_d_n11, assign20030_e18998_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20030_e18994: f64 = (2.0 * locals.var_fn241_calc_iq__n);
        let assign20030_e18996: f64 = (assign20030_e18994 * locals.var_fn241_calc_iq__phitin);
        (assign20030_e18996, (((2.0 * locals.var_fn241_calc_iq__n_dn4) * locals.var_fn241_calc_iq__phitin) + (assign20030_e18994 * locals.var_fn241_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn241_calc_iq__n_dn11) * locals.var_fn241_calc_iq__phitin), ((2.0 * locals.var_fn241_calc_iq__n_dn12) * locals.var_fn241_calc_iq__phitin),)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit, locals.var_fn241_calc_iq__two_n_phit_dn4, locals.var_fn241_calc_iq__two_n_phit_dn11, locals.var_fn241_calc_iq__two_n_phit_dn12,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit = assign20030_e18998;
        locals.var_fn241_calc_iq__two_n_phit_dn4 = assign20030_e18998_d_n4;
        locals.var_fn241_calc_iq__two_n_phit_dn11 = assign20030_e18998_d_n11;
        locals.var_fn241_calc_iq__two_n_phit_dn12 = assign20030_e18998_d_n12;

        let (assign20040_e19004, assign20040_e19004_d_n4, assign20040_e19004_d_n11, assign20040_e19004_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20040_e19002: f64 = (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit);
        (assign20040_e19002, ((locals.var_fn241_calc_iq__cgin_dn4 * locals.var_fn241_calc_iq__two_n_phit) + (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit_dn4)), (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit_dn11), (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qref, locals.var_fn241_calc_iq__qref_dn4, locals.var_fn241_calc_iq__qref_dn11, locals.var_fn241_calc_iq__qref_dn12,)
    }
};
        locals.var_fn241_calc_iq__qref = assign20040_e19004;
        locals.var_fn241_calc_iq__qref_dn4 = assign20040_e19004_d_n4;
        locals.var_fn241_calc_iq__qref_dn11 = assign20040_e19004_d_n11;
        locals.var_fn241_calc_iq__qref_dn12 = assign20040_e19004_d_n12;

        let (assign20050_e19014, assign20050_e19014_d_n2, assign20050_e19014_d_n3, assign20050_e19014_d_n4, assign20050_e19014_d_n7, assign20050_e19014_d_n11, assign20050_e19014_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20050_e19009: f64 = (p.p51 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20050_e19011: f64 = (assign20050_e19009 / 2.0);
        let assign20050_e19012: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20050_e19011);
        (assign20050_e19012, 0.0, 0.0, (locals.var_fn241_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn241_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn241_calc_iq__vtdibl_dn11, locals.var_fn241_calc_iq__vtdibl_dn12,)
    } else {
        (locals.var_fn241_calc_iq__myarg, locals.var_fn241_calc_iq__myarg_dn2, locals.var_fn241_calc_iq__myarg_dn3, locals.var_fn241_calc_iq__myarg_dn4, locals.var_fn241_calc_iq__myarg_dn7, locals.var_fn241_calc_iq__myarg_dn11, locals.var_fn241_calc_iq__myarg_dn12,)
    }
};
        locals.var_fn241_calc_iq__myarg = assign20050_e19014;
        locals.var_fn241_calc_iq__myarg_dn2 = assign20050_e19014_d_n2;
        locals.var_fn241_calc_iq__myarg_dn3 = assign20050_e19014_d_n3;
        locals.var_fn241_calc_iq__myarg_dn4 = assign20050_e19014_d_n4;
        locals.var_fn241_calc_iq__myarg_dn7 = assign20050_e19014_d_n7;
        locals.var_fn241_calc_iq__myarg_dn11 = assign20050_e19014_d_n11;
        locals.var_fn241_calc_iq__myarg_dn12 = assign20050_e19014_d_n12;

        let (assign20060_e19065, assign20060_e19065_d_n2, assign20060_e19065_d_n3, assign20060_e19065_d_n4, assign20060_e19065_d_n7, assign20060_e19065_d_n11, assign20060_e19065_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20060_e19059, assign20060_e19059_d_n2, assign20060_e19059_d_n7, assign20060_e19059_d_n11, assign20060_e19059_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20060_e19023: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20060_e19026: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20060_e19029: f64 = (0.001 / p.p53);
                let assign20060_e19032: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20060_e19033: f64 = (assign20060_e19029 * assign20060_e19032);
                let assign20060_e19034: f64 = (assign20060_e19033).tanh();
                let assign20060_e19035: f64 = (assign20060_e19026 * assign20060_e19034);
                let assign20060_e19036: f64 = (assign20060_e19023 + assign20060_e19035);
                let assign20060_e19037: f64 = (0.5 * assign20060_e19036);
                (assign20060_e19037, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20060_e19034) + (assign20060_e19026 * ((assign20060_e19029 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20060_e19033).cosh() * (assign20060_e19033).cosh())))))),)
            } else {
                let (assign20060_e19058, assign20060_e19058_d_n2, assign20060_e19058_d_n7, assign20060_e19058_d_n11, assign20060_e19058_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20060_e19044: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20060_e19047: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20060_e19050: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20060_e19051: f64 = (assign20060_e19047 * assign20060_e19050);
                        let assign20060_e19053: f64 = (assign20060_e19051 + p.p53);
                        let assign20060_e19054: f64 = (assign20060_e19053).sqrt();
                        let assign20060_e19055: f64 = (assign20060_e19044 + assign20060_e19054);
                        let assign20060_e19056: f64 = (0.5 * assign20060_e19055);
                        (assign20060_e19056, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20060_e19050) + (assign20060_e19047 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20060_e19054)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20060_e19050) + (assign20060_e19047 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20060_e19054)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20060_e19050) + (assign20060_e19047 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20060_e19054)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20060_e19050) + (assign20060_e19047 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20060_e19054)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20060_e19058, assign20060_e19058_d_n2, assign20060_e19058_d_n7, assign20060_e19058_d_n11, assign20060_e19058_d_n12,)
            }
        };
        let assign20060_e19061: f64 = (assign20060_e19059 - locals.var_fn241_calc_iq__myarg);
        let assign20060_e19063: f64 = (assign20060_e19061 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20060_e19063, ((assign20060_e19059_d_n2 - locals.var_fn241_calc_iq__myarg_dn2) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn3) / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20060_e19061 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), ((assign20060_e19059_d_n7 - locals.var_fn241_calc_iq__myarg_dn7) / locals.var_fn241_calc_iq__alpha_phit), ((assign20060_e19059_d_n11 - locals.var_fn241_calc_iq__myarg_dn11) / locals.var_fn241_calc_iq__alpha_phit), ((assign20060_e19059_d_n12 - locals.var_fn241_calc_iq__myarg_dn12) / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign20060_e19065;
        locals.var_fn241_calc_iq__exparg_dn2 = assign20060_e19065_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign20060_e19065_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign20060_e19065_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign20060_e19065_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign20060_e19065_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign20060_e19065_d_n12;

        let assign20070_e19068: f64 = if locals.var_fn241_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign20070_e19068;

        let (assign20080_e19074, assign20080_e19074_d_n2, assign20080_e19074_d_n3, assign20080_e19074_d_n4, assign20080_e19074_d_n7, assign20080_e19074_d_n11, assign20080_e19074_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard243 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign20080_e19074;
        locals.var_fn241_calc_iq__ff_dn2 = assign20080_e19074_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign20080_e19074_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign20080_e19074_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign20080_e19074_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign20080_e19074_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign20080_e19074_d_n12;

        let assign20090_e19077: f64 = (-50.0);
        let assign20090_e19078: f64 = if locals.var_fn241_calc_iq__exparg < assign20090_e19077 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign20090_e19078;

        let (assign20100_e19087, assign20100_e19087_d_n2, assign20100_e19087_d_n3, assign20100_e19087_d_n4, assign20100_e19087_d_n7, assign20100_e19087_d_n11, assign20100_e19087_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign20100_e19087;
        locals.var_fn241_calc_iq__ff_dn2 = assign20100_e19087_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign20100_e19087_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign20100_e19087_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign20100_e19087_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign20100_e19087_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign20100_e19087_d_n12;

        let (assign20110_e19102, assign20110_e19102_d_n2, assign20110_e19102_d_n3, assign20110_e19102_d_n4, assign20110_e19102_d_n7, assign20110_e19102_d_n11, assign20110_e19102_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard243 == 0.0)) && (locals.var_guard244 == 0.0)) {
        let assign20110_e19098: f64 = (locals.var_fn241_calc_iq__exparg).exp();
        let assign20110_e19099: f64 = (1.0 + assign20110_e19098);
        let assign20110_e19100: f64 = (1.0 / assign20110_e19099);
        (assign20110_e19100, (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn2) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn3) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn4) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn7) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn11) / (assign20110_e19099 * assign20110_e19099))), (-((assign20110_e19098 * locals.var_fn241_calc_iq__exparg_dn12) / (assign20110_e19099 * assign20110_e19099))),)
    } else {
        (locals.var_fn241_calc_iq__ff, locals.var_fn241_calc_iq__ff_dn2, locals.var_fn241_calc_iq__ff_dn3, locals.var_fn241_calc_iq__ff_dn4, locals.var_fn241_calc_iq__ff_dn7, locals.var_fn241_calc_iq__ff_dn11, locals.var_fn241_calc_iq__ff_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff = assign20110_e19102;
        locals.var_fn241_calc_iq__ff_dn2 = assign20110_e19102_d_n2;
        locals.var_fn241_calc_iq__ff_dn3 = assign20110_e19102_d_n3;
        locals.var_fn241_calc_iq__ff_dn4 = assign20110_e19102_d_n4;
        locals.var_fn241_calc_iq__ff_dn7 = assign20110_e19102_d_n7;
        locals.var_fn241_calc_iq__ff_dn11 = assign20110_e19102_d_n11;
        locals.var_fn241_calc_iq__ff_dn12 = assign20110_e19102_d_n12;

        let (assign20120_e19161, assign20120_e19161_d_n2, assign20120_e19161_d_n3, assign20120_e19161_d_n4, assign20120_e19161_d_n7, assign20120_e19161_d_n11, assign20120_e19161_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20120_e19147, assign20120_e19147_d_n2, assign20120_e19147_d_n7, assign20120_e19147_d_n11, assign20120_e19147_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20120_e19111: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20120_e19114: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20120_e19117: f64 = (0.001 / p.p53);
                let assign20120_e19120: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20120_e19121: f64 = (assign20120_e19117 * assign20120_e19120);
                let assign20120_e19122: f64 = (assign20120_e19121).tanh();
                let assign20120_e19123: f64 = (assign20120_e19114 * assign20120_e19122);
                let assign20120_e19124: f64 = (assign20120_e19111 + assign20120_e19123);
                let assign20120_e19125: f64 = (0.5 * assign20120_e19124);
                (assign20120_e19125, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20120_e19122) + (assign20120_e19114 * ((assign20120_e19117 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20120_e19121).cosh() * (assign20120_e19121).cosh())))))),)
            } else {
                let (assign20120_e19146, assign20120_e19146_d_n2, assign20120_e19146_d_n7, assign20120_e19146_d_n11, assign20120_e19146_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20120_e19132: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20120_e19135: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20120_e19138: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20120_e19139: f64 = (assign20120_e19135 * assign20120_e19138);
                        let assign20120_e19141: f64 = (assign20120_e19139 + p.p53);
                        let assign20120_e19142: f64 = (assign20120_e19141).sqrt();
                        let assign20120_e19143: f64 = (assign20120_e19132 + assign20120_e19142);
                        let assign20120_e19144: f64 = (0.5 * assign20120_e19143);
                        (assign20120_e19144, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20120_e19138) + (assign20120_e19135 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20120_e19142)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20120_e19138) + (assign20120_e19135 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20120_e19142)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20120_e19138) + (assign20120_e19135 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20120_e19142)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20120_e19138) + (assign20120_e19135 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20120_e19142)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20120_e19146, assign20120_e19146_d_n2, assign20120_e19146_d_n7, assign20120_e19146_d_n11, assign20120_e19146_d_n12,)
            }
        };
        let assign20120_e19151: f64 = (p.p51 * 0.1);
        let assign20120_e19153: f64 = (assign20120_e19151 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20120_e19155: f64 = (assign20120_e19153 * locals.var_fn241_calc_iq__ff);
        let assign20120_e19156: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20120_e19155);
        let assign20120_e19157: f64 = (assign20120_e19147 - assign20120_e19156);
        let assign20120_e19159: f64 = (assign20120_e19157 / locals.var_fn241_calc_iq__two_n_phit);
        (assign20120_e19159, ((assign20120_e19147_d_n2 - (-(assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn2))) / locals.var_fn241_calc_iq__two_n_phit), ((-(-(assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn3))) / locals.var_fn241_calc_iq__two_n_phit), ((((-(locals.var_fn241_calc_iq__vtdibl_dn4 - (((assign20120_e19151 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ff) + (assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn4)))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20120_e19157 * locals.var_fn241_calc_iq__two_n_phit_dn4)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), ((assign20120_e19147_d_n7 - (-(assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn7))) / locals.var_fn241_calc_iq__two_n_phit), ((((assign20120_e19147_d_n11 - (locals.var_fn241_calc_iq__vtdibl_dn11 - (assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn11))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20120_e19157 * locals.var_fn241_calc_iq__two_n_phit_dn11)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), ((((assign20120_e19147_d_n12 - (locals.var_fn241_calc_iq__vtdibl_dn12 - (assign20120_e19153 * locals.var_fn241_calc_iq__ff_dn12))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20120_e19157 * locals.var_fn241_calc_iq__two_n_phit_dn12)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn241_calc_iq__eta, locals.var_fn241_calc_iq__eta_dn2, locals.var_fn241_calc_iq__eta_dn3, locals.var_fn241_calc_iq__eta_dn4, locals.var_fn241_calc_iq__eta_dn7, locals.var_fn241_calc_iq__eta_dn11, locals.var_fn241_calc_iq__eta_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta = assign20120_e19161;
        locals.var_fn241_calc_iq__eta_dn2 = assign20120_e19161_d_n2;
        locals.var_fn241_calc_iq__eta_dn3 = assign20120_e19161_d_n3;
        locals.var_fn241_calc_iq__eta_dn4 = assign20120_e19161_d_n4;
        locals.var_fn241_calc_iq__eta_dn7 = assign20120_e19161_d_n7;
        locals.var_fn241_calc_iq__eta_dn11 = assign20120_e19161_d_n11;
        locals.var_fn241_calc_iq__eta_dn12 = assign20120_e19161_d_n12;

        let assign20130_e19164: f64 = if locals.var_fn241_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign20130_e19164;

        let (assign20140_e19172, assign20140_e19172_d_n2, assign20140_e19172_d_n3, assign20140_e19172_d_n4, assign20140_e19172_d_n7, assign20140_e19172_d_n11, assign20140_e19172_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign20140_e19170: f64 = (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta);
        (assign20140_e19170, (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn2), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn3), ((locals.var_fn241_calc_iq__qref_dn4 * locals.var_fn241_calc_iq__eta) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn4)), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn7), ((locals.var_fn241_calc_iq__qref_dn11 * locals.var_fn241_calc_iq__eta) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn11)), ((locals.var_fn241_calc_iq__qref_dn12 * locals.var_fn241_calc_iq__eta) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__eta_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign20140_e19172;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign20140_e19172_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign20140_e19172_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign20140_e19172_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign20140_e19172_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign20140_e19172_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign20140_e19172_d_n12;

        let assign20150_e19175: f64 = (-50.0);
        let assign20150_e19176: f64 = if locals.var_fn241_calc_iq__eta < assign20150_e19175 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign20150_e19176;

        let (assign20160_e19188, assign20160_e19188_d_n2, assign20160_e19188_d_n3, assign20160_e19188_d_n4, assign20160_e19188_d_n7, assign20160_e19188_d_n11, assign20160_e19188_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard245 == 0.0)) && (locals.var_guard246 != 0.0)) {
        let assign20160_e19185: f64 = (locals.var_fn241_calc_iq__eta).exp();
        let assign20160_e19186: f64 = (locals.var_fn241_calc_iq__qref * assign20160_e19185);
        (assign20160_e19186, (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn2)), (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn3)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20160_e19185) + (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn4))), (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn7)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20160_e19185) + (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn11))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20160_e19185) + (locals.var_fn241_calc_iq__qref * (assign20160_e19185 * locals.var_fn241_calc_iq__eta_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign20160_e19188;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign20160_e19188_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign20160_e19188_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign20160_e19188_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign20160_e19188_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign20160_e19188_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign20160_e19188_d_n12;

        let (assign20170_e19204, assign20170_e19204_d_n2, assign20170_e19204_d_n3, assign20170_e19204_d_n4, assign20170_e19204_d_n7, assign20170_e19204_d_n11, assign20170_e19204_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard245 == 0.0)) && (locals.var_guard246 == 0.0)) {
        let assign20170_e19199: f64 = (locals.var_fn241_calc_iq__eta).exp();
        let assign20170_e19200: f64 = (1.0 + assign20170_e19199);
        let assign20170_e19201: f64 = (assign20170_e19200).ln();
        let assign20170_e19202: f64 = (locals.var_fn241_calc_iq__qref * assign20170_e19201);
        (assign20170_e19202, (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn2) / assign20170_e19200)), (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn3) / assign20170_e19200)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20170_e19201) + (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn4) / assign20170_e19200))), (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn7) / assign20170_e19200)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20170_e19201) + (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn11) / assign20170_e19200))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20170_e19201) + (locals.var_fn241_calc_iq__qref * ((assign20170_e19199 * locals.var_fn241_calc_iq__eta_dn12) / assign20170_e19200))),)
    } else {
        (locals.var_fn241_calc_iq__qinvv, locals.var_fn241_calc_iq__qinvv_dn2, locals.var_fn241_calc_iq__qinvv_dn3, locals.var_fn241_calc_iq__qinvv_dn4, locals.var_fn241_calc_iq__qinvv_dn7, locals.var_fn241_calc_iq__qinvv_dn11, locals.var_fn241_calc_iq__qinvv_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv = assign20170_e19204;
        locals.var_fn241_calc_iq__qinvv_dn2 = assign20170_e19204_d_n2;
        locals.var_fn241_calc_iq__qinvv_dn3 = assign20170_e19204_d_n3;
        locals.var_fn241_calc_iq__qinvv_dn4 = assign20170_e19204_d_n4;
        locals.var_fn241_calc_iq__qinvv_dn7 = assign20170_e19204_d_n7;
        locals.var_fn241_calc_iq__qinvv_dn11 = assign20170_e19204_d_n11;
        locals.var_fn241_calc_iq__qinvv_dn12 = assign20170_e19204_d_n12;

        let (assign20180_e19218, assign20180_e19218_d_n2, assign20180_e19218_d_n3, assign20180_e19218_d_n4, assign20180_e19218_d_n7, assign20180_e19218_d_n11, assign20180_e19218_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20180_e19211: f64 = (locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv);
        let assign20180_e19213: f64 = (assign20180_e19211 / locals.var_fn241_calc_iq__cgin);
        let assign20180_e19214: f64 = (1.0 + assign20180_e19213);
        let assign20180_e19215: f64 = (locals.var_fn241_calc_iq__tfacmobin * assign20180_e19214);
        let assign20180_e19216: f64 = (locals.var_fn241_calc_iq__mu0 / assign20180_e19215);
        (assign20180_e19216, (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn2) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn3) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * ((locals.var_fn241_calc_iq__tfacmobin_dn4 * assign20180_e19214) + (locals.var_fn241_calc_iq__tfacmobin * ((((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20180_e19211 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin))))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn7) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn11) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))), (-((locals.var_fn241_calc_iq__mu0 * (locals.var_fn241_calc_iq__tfacmobin * ((locals.var_fn241_calc_iq__mtheta * locals.var_fn241_calc_iq__qinvv_dn12) / locals.var_fn241_calc_iq__cgin))) / (assign20180_e19215 * assign20180_e19215))),)
    } else {
        (locals.var_fn241_calc_iq__muf, locals.var_fn241_calc_iq__muf_dn2, locals.var_fn241_calc_iq__muf_dn3, locals.var_fn241_calc_iq__muf_dn4, locals.var_fn241_calc_iq__muf_dn7, locals.var_fn241_calc_iq__muf_dn11, locals.var_fn241_calc_iq__muf_dn12,)
    }
};
        locals.var_fn241_calc_iq__muf = assign20180_e19218;
        locals.var_fn241_calc_iq__muf_dn2 = assign20180_e19218_d_n2;
        locals.var_fn241_calc_iq__muf_dn3 = assign20180_e19218_d_n3;
        locals.var_fn241_calc_iq__muf_dn4 = assign20180_e19218_d_n4;
        locals.var_fn241_calc_iq__muf_dn7 = assign20180_e19218_d_n7;
        locals.var_fn241_calc_iq__muf_dn11 = assign20180_e19218_d_n11;
        locals.var_fn241_calc_iq__muf_dn12 = assign20180_e19218_d_n12;

        let (assign20190_e19250, assign20190_e19250_d_n2, assign20190_e19250_d_n3, assign20190_e19250_d_n4, assign20190_e19250_d_n7, assign20190_e19250_d_n11, assign20190_e19250_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20190_e19224: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tnomin);
        let assign20190_e19225: f64 = (1.0 + assign20190_e19224);
        let assign20190_e19229: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin);
        let assign20190_e19230: f64 = (1.0 + assign20190_e19229);
        let assign20190_e19231: f64 = (assign20190_e19225 / assign20190_e19230);
        let assign20190_e19232: f64 = (locals.var_fn241_calc_iq__vel0 * assign20190_e19231);
        let assign20190_e19236: f64 = (locals.var_fn241_calc_iq__lambda * locals.var_fn241_calc_iq__absvdsin);
        let assign20190_e19238: f64 = (assign20190_e19236 / locals.var_fn241_calc_iq__lin);
        let assign20190_e19239: f64 = (1.0 + assign20190_e19238);
        let assign20190_e19240: f64 = (assign20190_e19232 * assign20190_e19239);
        let assign20190_e19244: f64 = (locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv);
        let assign20190_e19246: f64 = (assign20190_e19244 / locals.var_fn241_calc_iq__cgin);
        let assign20190_e19247: f64 = (1.0 + assign20190_e19246);
        let assign20190_e19248: f64 = (assign20190_e19240 / assign20190_e19247);
        (assign20190_e19248, (-((assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn2) / locals.var_fn241_calc_iq__cgin)) / (assign20190_e19247 * assign20190_e19247))), (-((assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn3) / locals.var_fn241_calc_iq__cgin)) / (assign20190_e19247 * assign20190_e19247))), (((((locals.var_fn241_calc_iq__vel0 * (-((assign20190_e19225 * (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin_dn4)) / (assign20190_e19230 * assign20190_e19230)))) * assign20190_e19239) * assign20190_e19247) - (assign20190_e19240 * ((((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20190_e19244 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)))) / (assign20190_e19247 * assign20190_e19247)), (-((assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn7) / locals.var_fn241_calc_iq__cgin)) / (assign20190_e19247 * assign20190_e19247))), ((((assign20190_e19232 * ((locals.var_fn241_calc_iq__lambda * locals.var_fn241_calc_iq__absvdsin_dn11) / locals.var_fn241_calc_iq__lin)) * assign20190_e19247) - (assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn11) / locals.var_fn241_calc_iq__cgin))) / (assign20190_e19247 * assign20190_e19247)), ((((assign20190_e19232 * ((locals.var_fn241_calc_iq__lambda * locals.var_fn241_calc_iq__absvdsin_dn12) / locals.var_fn241_calc_iq__lin)) * assign20190_e19247) - (assign20190_e19240 * ((locals.var_fn241_calc_iq__vtheta * locals.var_fn241_calc_iq__qinvv_dn12) / locals.var_fn241_calc_iq__cgin))) / (assign20190_e19247 * assign20190_e19247)),)
    } else {
        (locals.var_fn241_calc_iq__vx, locals.var_fn241_calc_iq__vx_dn2, locals.var_fn241_calc_iq__vx_dn3, locals.var_fn241_calc_iq__vx_dn4, locals.var_fn241_calc_iq__vx_dn7, locals.var_fn241_calc_iq__vx_dn11, locals.var_fn241_calc_iq__vx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vx = assign20190_e19250;
        locals.var_fn241_calc_iq__vx_dn2 = assign20190_e19250_d_n2;
        locals.var_fn241_calc_iq__vx_dn3 = assign20190_e19250_d_n3;
        locals.var_fn241_calc_iq__vx_dn4 = assign20190_e19250_d_n4;
        locals.var_fn241_calc_iq__vx_dn7 = assign20190_e19250_d_n7;
        locals.var_fn241_calc_iq__vx_dn11 = assign20190_e19250_d_n11;
        locals.var_fn241_calc_iq__vx_dn12 = assign20190_e19250_d_n12;

        let (assign20210_e19276, assign20210_e19276_d_n2, assign20210_e19276_d_n3, assign20210_e19276_d_n4, assign20210_e19276_d_n7, assign20210_e19276_d_n11, assign20210_e19276_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20210_e19272: f64 = (locals.var_fn241_calc_iq__vx * locals.var_fn241_calc_iq__lin);
        let assign20210_e19274: f64 = (assign20210_e19272 / locals.var_fn241_calc_iq__muf);
        (assign20210_e19274, ((((locals.var_fn241_calc_iq__vx_dn2 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn2)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn3 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn3)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn4 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn4)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn7 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn7)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn11 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn11)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)), ((((locals.var_fn241_calc_iq__vx_dn12 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf) - (assign20210_e19272 * locals.var_fn241_calc_iq__muf_dn12)) / (locals.var_fn241_calc_iq__muf * locals.var_fn241_calc_iq__muf)),)
    } else {
        (locals.var_fn241_calc_iq__vdsats, locals.var_fn241_calc_iq__vdsats_dn2, locals.var_fn241_calc_iq__vdsats_dn3, locals.var_fn241_calc_iq__vdsats_dn4, locals.var_fn241_calc_iq__vdsats_dn7, locals.var_fn241_calc_iq__vdsats_dn11, locals.var_fn241_calc_iq__vdsats_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats = assign20210_e19276;
        locals.var_fn241_calc_iq__vdsats_dn2 = assign20210_e19276_d_n2;
        locals.var_fn241_calc_iq__vdsats_dn3 = assign20210_e19276_d_n3;
        locals.var_fn241_calc_iq__vdsats_dn4 = assign20210_e19276_d_n4;
        locals.var_fn241_calc_iq__vdsats_dn7 = assign20210_e19276_d_n7;
        locals.var_fn241_calc_iq__vdsats_dn11 = assign20210_e19276_d_n11;
        locals.var_fn241_calc_iq__vdsats_dn12 = assign20210_e19276_d_n12;

        let (assign20220_e19293, assign20220_e19293_d_n2, assign20220_e19293_d_n3, assign20220_e19293_d_n4, assign20220_e19293_d_n7, assign20220_e19293_d_n11, assign20220_e19293_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20220_e19282: f64 = (2.0 * locals.var_fn241_calc_iq__qinvv);
        let assign20220_e19284: f64 = (assign20220_e19282 / locals.var_fn241_calc_iq__cgin);
        let assign20220_e19286: f64 = (assign20220_e19284 / locals.var_fn241_calc_iq__vdsats);
        let assign20220_e19287: f64 = (1.0 + assign20220_e19286);
        let assign20220_e19288: f64 = (assign20220_e19287).sqrt();
        let assign20220_e19289: f64 = (locals.var_fn241_calc_iq__vdsats * assign20220_e19288);
        let assign20220_e19291: f64 = (assign20220_e19289 - locals.var_fn241_calc_iq__vdsats);
        (assign20220_e19291, (((locals.var_fn241_calc_iq__vdsats_dn2 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn2) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn2)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn2), (((locals.var_fn241_calc_iq__vdsats_dn3 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn3) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn3)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn3), (((locals.var_fn241_calc_iq__vdsats_dn4 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20220_e19282 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn4)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn4), (((locals.var_fn241_calc_iq__vdsats_dn7 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn7) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn7)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn7), (((locals.var_fn241_calc_iq__vdsats_dn11 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn11) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn11)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn11), (((locals.var_fn241_calc_iq__vdsats_dn12 * assign20220_e19288) + (locals.var_fn241_calc_iq__vdsats * ((((((2.0 * locals.var_fn241_calc_iq__qinvv_dn12) / locals.var_fn241_calc_iq__cgin) * locals.var_fn241_calc_iq__vdsats) - (assign20220_e19284 * locals.var_fn241_calc_iq__vdsats_dn12)) / (locals.var_fn241_calc_iq__vdsats * locals.var_fn241_calc_iq__vdsats)) / (2.0 * assign20220_e19288)))) - locals.var_fn241_calc_iq__vdsats_dn12),)
    } else {
        (locals.var_fn241_calc_iq__vdsats1, locals.var_fn241_calc_iq__vdsats1_dn2, locals.var_fn241_calc_iq__vdsats1_dn3, locals.var_fn241_calc_iq__vdsats1_dn4, locals.var_fn241_calc_iq__vdsats1_dn7, locals.var_fn241_calc_iq__vdsats1_dn11, locals.var_fn241_calc_iq__vdsats1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats1 = assign20220_e19293;
        locals.var_fn241_calc_iq__vdsats1_dn2 = assign20220_e19293_d_n2;
        locals.var_fn241_calc_iq__vdsats1_dn3 = assign20220_e19293_d_n3;
        locals.var_fn241_calc_iq__vdsats1_dn4 = assign20220_e19293_d_n4;
        locals.var_fn241_calc_iq__vdsats1_dn7 = assign20220_e19293_d_n7;
        locals.var_fn241_calc_iq__vdsats1_dn11 = assign20220_e19293_d_n11;
        locals.var_fn241_calc_iq__vdsats1_dn12 = assign20220_e19293_d_n12;

        let (assign20230_e19305, assign20230_e19305_d_n2, assign20230_e19305_d_n3, assign20230_e19305_d_n4, assign20230_e19305_d_n7, assign20230_e19305_d_n11, assign20230_e19305_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20230_e19298: f64 = (1.0 - locals.var_fn241_calc_iq__ff);
        let assign20230_e19299: f64 = (locals.var_fn241_calc_iq__vdsats * assign20230_e19298);
        let assign20230_e19302: f64 = (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff);
        let assign20230_e19303: f64 = (assign20230_e19299 + assign20230_e19302);
        (assign20230_e19303, (((locals.var_fn241_calc_iq__vdsats_dn2 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn2))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn2)), (((locals.var_fn241_calc_iq__vdsats_dn3 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn3))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn3)), (((locals.var_fn241_calc_iq__vdsats_dn4 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn4))) + ((locals.var_fn241_calc_iq__two_n_phit_dn4 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn4))), (((locals.var_fn241_calc_iq__vdsats_dn7 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn7))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn7)), (((locals.var_fn241_calc_iq__vdsats_dn11 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn11))) + ((locals.var_fn241_calc_iq__two_n_phit_dn11 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn11))), (((locals.var_fn241_calc_iq__vdsats_dn12 * assign20230_e19298) + (locals.var_fn241_calc_iq__vdsats * (-locals.var_fn241_calc_iq__ff_dn12))) + ((locals.var_fn241_calc_iq__two_n_phit_dn12 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__vdsat, locals.var_fn241_calc_iq__vdsat_dn2, locals.var_fn241_calc_iq__vdsat_dn3, locals.var_fn241_calc_iq__vdsat_dn4, locals.var_fn241_calc_iq__vdsat_dn7, locals.var_fn241_calc_iq__vdsat_dn11, locals.var_fn241_calc_iq__vdsat_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat = assign20230_e19305;
        locals.var_fn241_calc_iq__vdsat_dn2 = assign20230_e19305_d_n2;
        locals.var_fn241_calc_iq__vdsat_dn3 = assign20230_e19305_d_n3;
        locals.var_fn241_calc_iq__vdsat_dn4 = assign20230_e19305_d_n4;
        locals.var_fn241_calc_iq__vdsat_dn7 = assign20230_e19305_d_n7;
        locals.var_fn241_calc_iq__vdsat_dn11 = assign20230_e19305_d_n11;
        locals.var_fn241_calc_iq__vdsat_dn12 = assign20230_e19305_d_n12;

        let (assign20240_e19317, assign20240_e19317_d_n2, assign20240_e19317_d_n3, assign20240_e19317_d_n4, assign20240_e19317_d_n7, assign20240_e19317_d_n11, assign20240_e19317_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20240_e19310: f64 = (1.0 - locals.var_fn241_calc_iq__ff);
        let assign20240_e19311: f64 = (locals.var_fn241_calc_iq__vdsats1 * assign20240_e19310);
        let assign20240_e19314: f64 = (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff);
        let assign20240_e19315: f64 = (assign20240_e19311 + assign20240_e19314);
        (assign20240_e19315, (((locals.var_fn241_calc_iq__vdsats1_dn2 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn2))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn2)), (((locals.var_fn241_calc_iq__vdsats1_dn3 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn3))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn3)), (((locals.var_fn241_calc_iq__vdsats1_dn4 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn4))) + ((locals.var_fn241_calc_iq__two_n_phit_dn4 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn4))), (((locals.var_fn241_calc_iq__vdsats1_dn7 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn7))) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn7)), (((locals.var_fn241_calc_iq__vdsats1_dn11 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn11))) + ((locals.var_fn241_calc_iq__two_n_phit_dn11 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn11))), (((locals.var_fn241_calc_iq__vdsats1_dn12 * assign20240_e19310) + (locals.var_fn241_calc_iq__vdsats1 * (-locals.var_fn241_calc_iq__ff_dn12))) + ((locals.var_fn241_calc_iq__two_n_phit_dn12 * locals.var_fn241_calc_iq__ff) + (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__ff_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__vdsat1, locals.var_fn241_calc_iq__vdsat1_dn2, locals.var_fn241_calc_iq__vdsat1_dn3, locals.var_fn241_calc_iq__vdsat1_dn4, locals.var_fn241_calc_iq__vdsat1_dn7, locals.var_fn241_calc_iq__vdsat1_dn11, locals.var_fn241_calc_iq__vdsat1_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat1 = assign20240_e19317;
        locals.var_fn241_calc_iq__vdsat1_dn2 = assign20240_e19317_d_n2;
        locals.var_fn241_calc_iq__vdsat1_dn3 = assign20240_e19317_d_n3;
        locals.var_fn241_calc_iq__vdsat1_dn4 = assign20240_e19317_d_n4;
        locals.var_fn241_calc_iq__vdsat1_dn7 = assign20240_e19317_d_n7;
        locals.var_fn241_calc_iq__vdsat1_dn11 = assign20240_e19317_d_n11;
        locals.var_fn241_calc_iq__vdsat1_dn12 = assign20240_e19317_d_n12;

        let (assign20250_e19386, assign20250_e19386_d_n2, assign20250_e19386_d_n3, assign20250_e19386_d_n4, assign20250_e19386_d_n7, assign20250_e19386_d_n11, assign20250_e19386_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20250_e19376, assign20250_e19376_d_n2, assign20250_e19376_d_n3, assign20250_e19376_d_n4, assign20250_e19376_d_n7, assign20250_e19376_d_n11, assign20250_e19376_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20250_e19329: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                let assign20250_e19330: f64 = assign20250_e19329;
                let assign20250_e19334: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                let assign20250_e19335: f64 = (-assign20250_e19334);
                let assign20250_e19338: f64 = (0.001 / p.p53);
                let assign20250_e19342: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                let assign20250_e19343: f64 = (-assign20250_e19342);
                let assign20250_e19344: f64 = (assign20250_e19338 * assign20250_e19343);
                let assign20250_e19345: f64 = (assign20250_e19344).tanh();
                let assign20250_e19346: f64 = (assign20250_e19335 * assign20250_e19345);
                let assign20250_e19347: f64 = (assign20250_e19330 + assign20250_e19346);
                let assign20250_e19348: f64 = (0.5 * assign20250_e19347);
                (assign20250_e19348, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19345) + (assign20250_e19335 * ((assign20250_e19338 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20250_e19344).cosh() * (assign20250_e19344).cosh())))))),)
            } else {
                let (assign20250_e19375, assign20250_e19375_d_n2, assign20250_e19375_d_n3, assign20250_e19375_d_n4, assign20250_e19375_d_n7, assign20250_e19375_d_n11, assign20250_e19375_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20250_e19356: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                        let assign20250_e19357: f64 = assign20250_e19356;
                        let assign20250_e19361: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                        let assign20250_e19362: f64 = (-assign20250_e19361);
                        let assign20250_e19366: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat1);
                        let assign20250_e19367: f64 = (-assign20250_e19366);
                        let assign20250_e19368: f64 = (assign20250_e19362 * assign20250_e19367);
                        let assign20250_e19370: f64 = (assign20250_e19368 + p.p53);
                        let assign20250_e19371: f64 = (assign20250_e19370).sqrt();
                        let assign20250_e19372: f64 = (assign20250_e19357 + assign20250_e19371);
                        let assign20250_e19373: f64 = (0.5 * assign20250_e19372);
                        (assign20250_e19373, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20250_e19367) + (assign20250_e19362 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20250_e19371)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19367) + (assign20250_e19362 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20250_e19371)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20250_e19367) + (assign20250_e19362 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat1) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20250_e19371)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20250_e19375, assign20250_e19375_d_n2, assign20250_e19375_d_n3, assign20250_e19375_d_n4, assign20250_e19375_d_n7, assign20250_e19375_d_n11, assign20250_e19375_d_n12,)
            }
        };
        let assign20250_e19378: f64 = (assign20250_e19376).powf(locals.var_fn241_calc_iq__beta);
        let assign20250_e19379: f64 = (1.0 + assign20250_e19378);
        let assign20250_e19382: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20250_e19383: f64 = (assign20250_e19379).powf(assign20250_e19382);
        let assign20250_e19384: f64 = (1.0 / assign20250_e19383);
        (assign20250_e19384, (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n2)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n2 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n2)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n2 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n3)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n3 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n3)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n3 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n4)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n4 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n4)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n4 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n7)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n7 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n7)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n7 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n11)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n11 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n11)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n11 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))), (-(if 0.0 == 0.0 && ((assign20250_e19382) as f64).is_finite() && ((assign20250_e19382) as f64).fract() == 0.0 { if assign20250_e19382 == 0.0 { 0.0 } else { (assign20250_e19382 * ((assign20250_e19379).powf(assign20250_e19382 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n12)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n12 / assign20250_e19376))) })) } } else { (assign20250_e19383 * (assign20250_e19382 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20250_e19376).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20250_e19376_d_n12)) } } else { (assign20250_e19378 * (locals.var_fn241_calc_iq__beta * (assign20250_e19376_d_n12 / assign20250_e19376))) } / assign20250_e19379))) } / (assign20250_e19383 * assign20250_e19383))),)
    } else {
        (locals.var_fn241_calc_iq__fsd, locals.var_fn241_calc_iq__fsd_dn2, locals.var_fn241_calc_iq__fsd_dn3, locals.var_fn241_calc_iq__fsd_dn4, locals.var_fn241_calc_iq__fsd_dn7, locals.var_fn241_calc_iq__fsd_dn11, locals.var_fn241_calc_iq__fsd_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd = assign20250_e19386;
        locals.var_fn241_calc_iq__fsd_dn2 = assign20250_e19386_d_n2;
        locals.var_fn241_calc_iq__fsd_dn3 = assign20250_e19386_d_n3;
        locals.var_fn241_calc_iq__fsd_dn4 = assign20250_e19386_d_n4;
        locals.var_fn241_calc_iq__fsd_dn7 = assign20250_e19386_d_n7;
        locals.var_fn241_calc_iq__fsd_dn11 = assign20250_e19386_d_n11;
        locals.var_fn241_calc_iq__fsd_dn12 = assign20250_e19386_d_n12;

    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20260_e19392, assign20260_e19392_d_n2, assign20260_e19392_d_n3, assign20260_e19392_d_n4, assign20260_e19392_d_n7, assign20260_e19392_d_n11, assign20260_e19392_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20260_e19390: f64 = (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd);
        (assign20260_e19390, (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn2), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn3), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn4), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn7), ((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__fsd) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn11)), ((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__fsd) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vdx, locals.var_fn241_calc_iq__vdx_dn2, locals.var_fn241_calc_iq__vdx_dn3, locals.var_fn241_calc_iq__vdx_dn4, locals.var_fn241_calc_iq__vdx_dn7, locals.var_fn241_calc_iq__vdx_dn11, locals.var_fn241_calc_iq__vdx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx = assign20260_e19392;
        locals.var_fn241_calc_iq__vdx_dn2 = assign20260_e19392_d_n2;
        locals.var_fn241_calc_iq__vdx_dn3 = assign20260_e19392_d_n3;
        locals.var_fn241_calc_iq__vdx_dn4 = assign20260_e19392_d_n4;
        locals.var_fn241_calc_iq__vdx_dn7 = assign20260_e19392_d_n7;
        locals.var_fn241_calc_iq__vdx_dn11 = assign20260_e19392_d_n11;
        locals.var_fn241_calc_iq__vdx_dn12 = assign20260_e19392_d_n12;

        let (assign20270_e19467, assign20270_e19467_d_n2, assign20270_e19467_d_n3, assign20270_e19467_d_n4, assign20270_e19467_d_n7, assign20270_e19467_d_n11, assign20270_e19467_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20270_e19457, assign20270_e19457_d_n2, assign20270_e19457_d_n3, assign20270_e19457_d_n4, assign20270_e19457_d_n7, assign20270_e19457_d_n11, assign20270_e19457_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20270_e19403: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20270_e19405: f64 = (assign20270_e19403 / locals.var_fn241_calc_iq__vdsat1);
                let assign20270_e19406: f64 = assign20270_e19405;
                let assign20270_e19409: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20270_e19411: f64 = (assign20270_e19409 / locals.var_fn241_calc_iq__vdsat1);
                let assign20270_e19412: f64 = (-assign20270_e19411);
                let assign20270_e19415: f64 = (0.001 / p.p53);
                let assign20270_e19418: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20270_e19420: f64 = (assign20270_e19418 / locals.var_fn241_calc_iq__vdsat1);
                let assign20270_e19421: f64 = (-assign20270_e19420);
                let assign20270_e19422: f64 = (assign20270_e19415 * assign20270_e19421);
                let assign20270_e19423: f64 = (assign20270_e19422).tanh();
                let assign20270_e19424: f64 = (assign20270_e19412 * assign20270_e19423);
                let assign20270_e19425: f64 = (assign20270_e19406 + assign20270_e19424);
                let assign20270_e19426: f64 = (0.5 * assign20270_e19425);
                (assign20270_e19426, (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * ((-((assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + (((-(-((assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-(-((assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19403 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19409 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19423) + (assign20270_e19412 * ((assign20270_e19415 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19418 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) / ((assign20270_e19422).cosh() * (assign20270_e19422).cosh())))))),)
            } else {
                let (assign20270_e19456, assign20270_e19456_d_n2, assign20270_e19456_d_n3, assign20270_e19456_d_n4, assign20270_e19456_d_n7, assign20270_e19456_d_n11, assign20270_e19456_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20270_e19433: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20270_e19435: f64 = (assign20270_e19433 / locals.var_fn241_calc_iq__vdsat1);
                        let assign20270_e19436: f64 = assign20270_e19435;
                        let assign20270_e19439: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20270_e19441: f64 = (assign20270_e19439 / locals.var_fn241_calc_iq__vdsat1);
                        let assign20270_e19442: f64 = (-assign20270_e19441);
                        let assign20270_e19445: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20270_e19447: f64 = (assign20270_e19445 / locals.var_fn241_calc_iq__vdsat1);
                        let assign20270_e19448: f64 = (-assign20270_e19447);
                        let assign20270_e19449: f64 = (assign20270_e19442 * assign20270_e19448);
                        let assign20270_e19451: f64 = (assign20270_e19449 + p.p53);
                        let assign20270_e19452: f64 = (assign20270_e19451).sqrt();
                        let assign20270_e19453: f64 = (assign20270_e19436 + assign20270_e19452);
                        let assign20270_e19454: f64 = (0.5 * assign20270_e19453);
                        (assign20270_e19454, (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn2) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn3) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn4) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * ((-((assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) + ((((-(-((assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))) * assign20270_e19448) + (assign20270_e19442 * (-(-((assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn7) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)))))) / (2.0 * assign20270_e19452)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19448) + (assign20270_e19442 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn11)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20270_e19452)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19433 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19439 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))) * assign20270_e19448) + (assign20270_e19442 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat1) - (assign20270_e19445 * locals.var_fn241_calc_iq__vdsat1_dn12)) / (locals.var_fn241_calc_iq__vdsat1 * locals.var_fn241_calc_iq__vdsat1))))) / (2.0 * assign20270_e19452)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20270_e19456, assign20270_e19456_d_n2, assign20270_e19456_d_n3, assign20270_e19456_d_n4, assign20270_e19456_d_n7, assign20270_e19456_d_n11, assign20270_e19456_d_n12,)
            }
        };
        let assign20270_e19459: f64 = (assign20270_e19457).powf(locals.var_fn241_calc_iq__beta);
        let assign20270_e19460: f64 = (1.0 + assign20270_e19459);
        let assign20270_e19463: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20270_e19464: f64 = (assign20270_e19460).powf(assign20270_e19463);
        let assign20270_e19465: f64 = (1.0 / assign20270_e19464);
        (assign20270_e19465, (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n2)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n2 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n2)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n2 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n3)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n3 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n3)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n3 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n4)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n4 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n4)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n4 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n7)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n7 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n7)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n7 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n11)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n11 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n11)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n11 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))), (-(if 0.0 == 0.0 && ((assign20270_e19463) as f64).is_finite() && ((assign20270_e19463) as f64).fract() == 0.0 { if assign20270_e19463 == 0.0 { 0.0 } else { (assign20270_e19463 * ((assign20270_e19460).powf(assign20270_e19463 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n12)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n12 / assign20270_e19457))) })) } } else { (assign20270_e19464 * (assign20270_e19463 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20270_e19457).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20270_e19457_d_n12)) } } else { (assign20270_e19459 * (locals.var_fn241_calc_iq__beta * (assign20270_e19457_d_n12 / assign20270_e19457))) } / assign20270_e19460))) } / (assign20270_e19464 * assign20270_e19464))),)
    } else {
        (locals.var_fn241_calc_iq__fds, locals.var_fn241_calc_iq__fds_dn2, locals.var_fn241_calc_iq__fds_dn3, locals.var_fn241_calc_iq__fds_dn4, locals.var_fn241_calc_iq__fds_dn7, locals.var_fn241_calc_iq__fds_dn11, locals.var_fn241_calc_iq__fds_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds = assign20270_e19467;
        locals.var_fn241_calc_iq__fds_dn2 = assign20270_e19467_d_n2;
        locals.var_fn241_calc_iq__fds_dn3 = assign20270_e19467_d_n3;
        locals.var_fn241_calc_iq__fds_dn4 = assign20270_e19467_d_n4;
        locals.var_fn241_calc_iq__fds_dn7 = assign20270_e19467_d_n7;
        locals.var_fn241_calc_iq__fds_dn11 = assign20270_e19467_d_n11;
        locals.var_fn241_calc_iq__fds_dn12 = assign20270_e19467_d_n12;

        let (assign20280_e19474, assign20280_e19474_d_n2, assign20280_e19474_d_n3, assign20280_e19474_d_n4, assign20280_e19474_d_n7, assign20280_e19474_d_n11, assign20280_e19474_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20280_e19470: f64 = (-locals.var_fn241_calc_iq__vdsin);
        let assign20280_e19472: f64 = (assign20280_e19470 * locals.var_fn241_calc_iq__fds);
        (assign20280_e19472, (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn2), (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn3), (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn4), (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn7), (((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__fds) + (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn11)), (((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__fds) + (assign20280_e19470 * locals.var_fn241_calc_iq__fds_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vsx, locals.var_fn241_calc_iq__vsx_dn2, locals.var_fn241_calc_iq__vsx_dn3, locals.var_fn241_calc_iq__vsx_dn4, locals.var_fn241_calc_iq__vsx_dn7, locals.var_fn241_calc_iq__vsx_dn11, locals.var_fn241_calc_iq__vsx_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx = assign20280_e19474;
        locals.var_fn241_calc_iq__vsx_dn2 = assign20280_e19474_d_n2;
        locals.var_fn241_calc_iq__vsx_dn3 = assign20280_e19474_d_n3;
        locals.var_fn241_calc_iq__vsx_dn4 = assign20280_e19474_d_n4;
        locals.var_fn241_calc_iq__vsx_dn7 = assign20280_e19474_d_n7;
        locals.var_fn241_calc_iq__vsx_dn11 = assign20280_e19474_d_n11;
        locals.var_fn241_calc_iq__vsx_dn12 = assign20280_e19474_d_n12;

        let (assign20290_e19482, assign20290_e19482_d_n2, assign20290_e19482_d_n3, assign20290_e19482_d_n4, assign20290_e19482_d_n7, assign20290_e19482_d_n11, assign20290_e19482_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20290_e19478: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__myarg);
        let assign20290_e19480: f64 = (assign20290_e19478 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20290_e19480, ((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__myarg_dn2) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn3) / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20290_e19478 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), ((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__myarg_dn7) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn11) / locals.var_fn241_calc_iq__alpha_phit), ((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__myarg_dn12) / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign20290_e19482;
        locals.var_fn241_calc_iq__exparg_dn2 = assign20290_e19482_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign20290_e19482_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign20290_e19482_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign20290_e19482_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign20290_e19482_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign20290_e19482_d_n12;

        let assign20300_e19485: f64 = if locals.var_fn241_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign20300_e19485;

        let (assign20310_e19491, assign20310_e19491_d_n2, assign20310_e19491_d_n3, assign20310_e19491_d_n4, assign20310_e19491_d_n7, assign20310_e19491_d_n11, assign20310_e19491_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard247 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign20310_e19491;
        locals.var_fn241_calc_iq__ffs_dn2 = assign20310_e19491_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign20310_e19491_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign20310_e19491_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign20310_e19491_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign20310_e19491_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign20310_e19491_d_n12;

        let assign20320_e19494: f64 = (-50.0);
        let assign20320_e19495: f64 = if locals.var_fn241_calc_iq__exparg < assign20320_e19494 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign20320_e19495;

        let (assign20330_e19504, assign20330_e19504_d_n2, assign20330_e19504_d_n3, assign20330_e19504_d_n4, assign20330_e19504_d_n7, assign20330_e19504_d_n11, assign20330_e19504_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard247 == 0.0)) && (locals.var_guard248 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign20330_e19504;
        locals.var_fn241_calc_iq__ffs_dn2 = assign20330_e19504_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign20330_e19504_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign20330_e19504_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign20330_e19504_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign20330_e19504_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign20330_e19504_d_n12;

        let (assign20340_e19519, assign20340_e19519_d_n2, assign20340_e19519_d_n3, assign20340_e19519_d_n4, assign20340_e19519_d_n7, assign20340_e19519_d_n11, assign20340_e19519_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard247 == 0.0)) && (locals.var_guard248 == 0.0)) {
        let assign20340_e19515: f64 = (locals.var_fn241_calc_iq__exparg).exp();
        let assign20340_e19516: f64 = (1.0 + assign20340_e19515);
        let assign20340_e19517: f64 = (1.0 / assign20340_e19516);
        (assign20340_e19517, (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn2) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn3) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn4) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn7) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn11) / (assign20340_e19516 * assign20340_e19516))), (-((assign20340_e19515 * locals.var_fn241_calc_iq__exparg_dn12) / (assign20340_e19516 * assign20340_e19516))),)
    } else {
        (locals.var_fn241_calc_iq__ffs, locals.var_fn241_calc_iq__ffs_dn2, locals.var_fn241_calc_iq__ffs_dn3, locals.var_fn241_calc_iq__ffs_dn4, locals.var_fn241_calc_iq__ffs_dn7, locals.var_fn241_calc_iq__ffs_dn11, locals.var_fn241_calc_iq__ffs_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs = assign20340_e19519;
        locals.var_fn241_calc_iq__ffs_dn2 = assign20340_e19519_d_n2;
        locals.var_fn241_calc_iq__ffs_dn3 = assign20340_e19519_d_n3;
        locals.var_fn241_calc_iq__ffs_dn4 = assign20340_e19519_d_n4;
        locals.var_fn241_calc_iq__ffs_dn7 = assign20340_e19519_d_n7;
        locals.var_fn241_calc_iq__ffs_dn11 = assign20340_e19519_d_n11;
        locals.var_fn241_calc_iq__ffs_dn12 = assign20340_e19519_d_n12;

        let (assign20350_e19537, assign20350_e19537_d_n2, assign20350_e19537_d_n3, assign20350_e19537_d_n4, assign20350_e19537_d_n7, assign20350_e19537_d_n11, assign20350_e19537_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20350_e19523: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__vsx);
        let assign20350_e19527: f64 = (p.p51 * 0.1);
        let assign20350_e19529: f64 = (assign20350_e19527 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20350_e19531: f64 = (assign20350_e19529 * locals.var_fn241_calc_iq__ffs);
        let assign20350_e19532: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20350_e19531);
        let assign20350_e19533: f64 = (assign20350_e19523 - assign20350_e19532);
        let assign20350_e19535: f64 = (assign20350_e19533 / locals.var_fn241_calc_iq__two_n_phit);
        (assign20350_e19535, (((locals.var_fn241_calc_iq__vgdin_dn2 - locals.var_fn241_calc_iq__vsx_dn2) - (-(assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn2))) / locals.var_fn241_calc_iq__two_n_phit), (((-locals.var_fn241_calc_iq__vsx_dn3) - (-(assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn3))) / locals.var_fn241_calc_iq__two_n_phit), (((((-locals.var_fn241_calc_iq__vsx_dn4) - (locals.var_fn241_calc_iq__vtdibl_dn4 - (((assign20350_e19527 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffs) + (assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn4)))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20350_e19533 * locals.var_fn241_calc_iq__two_n_phit_dn4)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((locals.var_fn241_calc_iq__vgdin_dn7 - locals.var_fn241_calc_iq__vsx_dn7) - (-(assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn7))) / locals.var_fn241_calc_iq__two_n_phit), (((((locals.var_fn241_calc_iq__vgdin_dn11 - locals.var_fn241_calc_iq__vsx_dn11) - (locals.var_fn241_calc_iq__vtdibl_dn11 - (assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn11))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20350_e19533 * locals.var_fn241_calc_iq__two_n_phit_dn11)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((((locals.var_fn241_calc_iq__vgdin_dn12 - locals.var_fn241_calc_iq__vsx_dn12) - (locals.var_fn241_calc_iq__vtdibl_dn12 - (assign20350_e19529 * locals.var_fn241_calc_iq__ffs_dn12))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20350_e19533 * locals.var_fn241_calc_iq__two_n_phit_dn12)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn241_calc_iq__etas, locals.var_fn241_calc_iq__etas_dn2, locals.var_fn241_calc_iq__etas_dn3, locals.var_fn241_calc_iq__etas_dn4, locals.var_fn241_calc_iq__etas_dn7, locals.var_fn241_calc_iq__etas_dn11, locals.var_fn241_calc_iq__etas_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas = assign20350_e19537;
        locals.var_fn241_calc_iq__etas_dn2 = assign20350_e19537_d_n2;
        locals.var_fn241_calc_iq__etas_dn3 = assign20350_e19537_d_n3;
        locals.var_fn241_calc_iq__etas_dn4 = assign20350_e19537_d_n4;
        locals.var_fn241_calc_iq__etas_dn7 = assign20350_e19537_d_n7;
        locals.var_fn241_calc_iq__etas_dn11 = assign20350_e19537_d_n11;
        locals.var_fn241_calc_iq__etas_dn12 = assign20350_e19537_d_n12;

        let assign20360_e19540: f64 = if locals.var_fn241_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign20360_e19540;

        let (assign20370_e19548, assign20370_e19548_d_n2, assign20370_e19548_d_n3, assign20370_e19548_d_n4, assign20370_e19548_d_n7, assign20370_e19548_d_n11, assign20370_e19548_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard249 != 0.0)) {
        let assign20370_e19546: f64 = (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas);
        (assign20370_e19546, (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn2), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn3), ((locals.var_fn241_calc_iq__qref_dn4 * locals.var_fn241_calc_iq__etas) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn4)), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn7), ((locals.var_fn241_calc_iq__qref_dn11 * locals.var_fn241_calc_iq__etas) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn11)), ((locals.var_fn241_calc_iq__qref_dn12 * locals.var_fn241_calc_iq__etas) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etas_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign20370_e19548;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign20370_e19548_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign20370_e19548_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign20370_e19548_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign20370_e19548_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign20370_e19548_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign20370_e19548_d_n12;

        let assign20380_e19551: f64 = (-50.0);
        let assign20380_e19552: f64 = if locals.var_fn241_calc_iq__etas < assign20380_e19551 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign20380_e19552;

        let (assign20390_e19564, assign20390_e19564_d_n2, assign20390_e19564_d_n3, assign20390_e19564_d_n4, assign20390_e19564_d_n7, assign20390_e19564_d_n11, assign20390_e19564_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign20390_e19561: f64 = (locals.var_fn241_calc_iq__etas).exp();
        let assign20390_e19562: f64 = (locals.var_fn241_calc_iq__qref * assign20390_e19561);
        (assign20390_e19562, (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn2)), (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn3)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20390_e19561) + (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn4))), (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn7)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20390_e19561) + (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn11))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20390_e19561) + (locals.var_fn241_calc_iq__qref * (assign20390_e19561 * locals.var_fn241_calc_iq__etas_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign20390_e19564;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign20390_e19564_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign20390_e19564_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign20390_e19564_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign20390_e19564_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign20390_e19564_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign20390_e19564_d_n12;

        let (assign20400_e19580, assign20400_e19580_d_n2, assign20400_e19580_d_n3, assign20400_e19580_d_n4, assign20400_e19580_d_n7, assign20400_e19580_d_n11, assign20400_e19580_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard249 == 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign20400_e19575: f64 = (locals.var_fn241_calc_iq__etas).exp();
        let assign20400_e19576: f64 = (1.0 + assign20400_e19575);
        let assign20400_e19577: f64 = (assign20400_e19576).ln();
        let assign20400_e19578: f64 = (locals.var_fn241_calc_iq__qref * assign20400_e19577);
        (assign20400_e19578, (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn2) / assign20400_e19576)), (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn3) / assign20400_e19576)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20400_e19577) + (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn4) / assign20400_e19576))), (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn7) / assign20400_e19576)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20400_e19577) + (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn11) / assign20400_e19576))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20400_e19577) + (locals.var_fn241_calc_iq__qref * ((assign20400_e19575 * locals.var_fn241_calc_iq__etas_dn12) / assign20400_e19576))),)
    } else {
        (locals.var_fn241_calc_iq__qinvs, locals.var_fn241_calc_iq__qinvs_dn2, locals.var_fn241_calc_iq__qinvs_dn3, locals.var_fn241_calc_iq__qinvs_dn4, locals.var_fn241_calc_iq__qinvs_dn7, locals.var_fn241_calc_iq__qinvs_dn11, locals.var_fn241_calc_iq__qinvs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs = assign20400_e19580;
        locals.var_fn241_calc_iq__qinvs_dn2 = assign20400_e19580_d_n2;
        locals.var_fn241_calc_iq__qinvs_dn3 = assign20400_e19580_d_n3;
        locals.var_fn241_calc_iq__qinvs_dn4 = assign20400_e19580_d_n4;
        locals.var_fn241_calc_iq__qinvs_dn7 = assign20400_e19580_d_n7;
        locals.var_fn241_calc_iq__qinvs_dn11 = assign20400_e19580_d_n11;
        locals.var_fn241_calc_iq__qinvs_dn12 = assign20400_e19580_d_n12;

        let (assign20410_e19588, assign20410_e19588_d_n2, assign20410_e19588_d_n3, assign20410_e19588_d_n4, assign20410_e19588_d_n7, assign20410_e19588_d_n11, assign20410_e19588_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20410_e19584: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__myarg);
        let assign20410_e19586: f64 = (assign20410_e19584 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20410_e19586, ((locals.var_fn241_calc_iq__vgdin_dn2 - locals.var_fn241_calc_iq__myarg_dn2) / locals.var_fn241_calc_iq__alpha_phit), ((-locals.var_fn241_calc_iq__myarg_dn3) / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20410_e19584 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), ((locals.var_fn241_calc_iq__vgdin_dn7 - locals.var_fn241_calc_iq__myarg_dn7) / locals.var_fn241_calc_iq__alpha_phit), ((locals.var_fn241_calc_iq__vgdin_dn11 - locals.var_fn241_calc_iq__myarg_dn11) / locals.var_fn241_calc_iq__alpha_phit), ((locals.var_fn241_calc_iq__vgdin_dn12 - locals.var_fn241_calc_iq__myarg_dn12) / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign20410_e19588;
        locals.var_fn241_calc_iq__exparg_dn2 = assign20410_e19588_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign20410_e19588_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign20410_e19588_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign20410_e19588_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign20410_e19588_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign20410_e19588_d_n12;

        let assign20420_e19591: f64 = if locals.var_fn241_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign20420_e19591;

        let (assign20430_e19597, assign20430_e19597_d_n2, assign20430_e19597_d_n3, assign20430_e19597_d_n4, assign20430_e19597_d_n7, assign20430_e19597_d_n11, assign20430_e19597_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard251 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign20430_e19597;
        locals.var_fn241_calc_iq__ffd_dn2 = assign20430_e19597_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign20430_e19597_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign20430_e19597_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign20430_e19597_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign20430_e19597_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign20430_e19597_d_n12;

        let assign20440_e19600: f64 = (-50.0);
        let assign20440_e19601: f64 = if locals.var_fn241_calc_iq__exparg < assign20440_e19600 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign20440_e19601;

        let (assign20450_e19610, assign20450_e19610_d_n2, assign20450_e19610_d_n3, assign20450_e19610_d_n4, assign20450_e19610_d_n7, assign20450_e19610_d_n11, assign20450_e19610_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard251 == 0.0)) && (locals.var_guard252 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign20450_e19610;
        locals.var_fn241_calc_iq__ffd_dn2 = assign20450_e19610_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign20450_e19610_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign20450_e19610_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign20450_e19610_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign20450_e19610_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign20450_e19610_d_n12;

        let (assign20460_e19625, assign20460_e19625_d_n2, assign20460_e19625_d_n3, assign20460_e19625_d_n4, assign20460_e19625_d_n7, assign20460_e19625_d_n11, assign20460_e19625_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard251 == 0.0)) && (locals.var_guard252 == 0.0)) {
        let assign20460_e19621: f64 = (locals.var_fn241_calc_iq__exparg).exp();
        let assign20460_e19622: f64 = (1.0 + assign20460_e19621);
        let assign20460_e19623: f64 = (1.0 / assign20460_e19622);
        (assign20460_e19623, (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn2) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn3) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn4) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn7) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn11) / (assign20460_e19622 * assign20460_e19622))), (-((assign20460_e19621 * locals.var_fn241_calc_iq__exparg_dn12) / (assign20460_e19622 * assign20460_e19622))),)
    } else {
        (locals.var_fn241_calc_iq__ffd, locals.var_fn241_calc_iq__ffd_dn2, locals.var_fn241_calc_iq__ffd_dn3, locals.var_fn241_calc_iq__ffd_dn4, locals.var_fn241_calc_iq__ffd_dn7, locals.var_fn241_calc_iq__ffd_dn11, locals.var_fn241_calc_iq__ffd_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd = assign20460_e19625;
        locals.var_fn241_calc_iq__ffd_dn2 = assign20460_e19625_d_n2;
        locals.var_fn241_calc_iq__ffd_dn3 = assign20460_e19625_d_n3;
        locals.var_fn241_calc_iq__ffd_dn4 = assign20460_e19625_d_n4;
        locals.var_fn241_calc_iq__ffd_dn7 = assign20460_e19625_d_n7;
        locals.var_fn241_calc_iq__ffd_dn11 = assign20460_e19625_d_n11;
        locals.var_fn241_calc_iq__ffd_dn12 = assign20460_e19625_d_n12;

        let (assign20470_e19643, assign20470_e19643_d_n2, assign20470_e19643_d_n3, assign20470_e19643_d_n4, assign20470_e19643_d_n7, assign20470_e19643_d_n11, assign20470_e19643_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20470_e19629: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vdx);
        let assign20470_e19633: f64 = (p.p51 * 0.1);
        let assign20470_e19635: f64 = (assign20470_e19633 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20470_e19637: f64 = (assign20470_e19635 * locals.var_fn241_calc_iq__ffd);
        let assign20470_e19638: f64 = (locals.var_fn241_calc_iq__vtdibl - assign20470_e19637);
        let assign20470_e19639: f64 = (assign20470_e19629 - assign20470_e19638);
        let assign20470_e19641: f64 = (assign20470_e19639 / locals.var_fn241_calc_iq__two_n_phit);
        (assign20470_e19641, (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vdx_dn2) - (-(assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn2))) / locals.var_fn241_calc_iq__two_n_phit), (((-locals.var_fn241_calc_iq__vdx_dn3) - (-(assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn3))) / locals.var_fn241_calc_iq__two_n_phit), (((((-locals.var_fn241_calc_iq__vdx_dn4) - (locals.var_fn241_calc_iq__vtdibl_dn4 - (((assign20470_e19633 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffd) + (assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn4)))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20470_e19639 * locals.var_fn241_calc_iq__two_n_phit_dn4)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vdx_dn7) - (-(assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn7))) / locals.var_fn241_calc_iq__two_n_phit), (((((-locals.var_fn241_calc_iq__vdx_dn11) - (locals.var_fn241_calc_iq__vtdibl_dn11 - (assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn11))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20470_e19639 * locals.var_fn241_calc_iq__two_n_phit_dn11)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)), (((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vdx_dn12) - (locals.var_fn241_calc_iq__vtdibl_dn12 - (assign20470_e19635 * locals.var_fn241_calc_iq__ffd_dn12))) * locals.var_fn241_calc_iq__two_n_phit) - (assign20470_e19639 * locals.var_fn241_calc_iq__two_n_phit_dn12)) / (locals.var_fn241_calc_iq__two_n_phit * locals.var_fn241_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn241_calc_iq__etad, locals.var_fn241_calc_iq__etad_dn2, locals.var_fn241_calc_iq__etad_dn3, locals.var_fn241_calc_iq__etad_dn4, locals.var_fn241_calc_iq__etad_dn7, locals.var_fn241_calc_iq__etad_dn11, locals.var_fn241_calc_iq__etad_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad = assign20470_e19643;
        locals.var_fn241_calc_iq__etad_dn2 = assign20470_e19643_d_n2;
        locals.var_fn241_calc_iq__etad_dn3 = assign20470_e19643_d_n3;
        locals.var_fn241_calc_iq__etad_dn4 = assign20470_e19643_d_n4;
        locals.var_fn241_calc_iq__etad_dn7 = assign20470_e19643_d_n7;
        locals.var_fn241_calc_iq__etad_dn11 = assign20470_e19643_d_n11;
        locals.var_fn241_calc_iq__etad_dn12 = assign20470_e19643_d_n12;

        let assign20480_e19646: f64 = if locals.var_fn241_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign20480_e19646;

        let (assign20490_e19654, assign20490_e19654_d_n2, assign20490_e19654_d_n3, assign20490_e19654_d_n4, assign20490_e19654_d_n7, assign20490_e19654_d_n11, assign20490_e19654_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard253 != 0.0)) {
        let assign20490_e19652: f64 = (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad);
        (assign20490_e19652, (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn2), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn3), ((locals.var_fn241_calc_iq__qref_dn4 * locals.var_fn241_calc_iq__etad) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn4)), (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn7), ((locals.var_fn241_calc_iq__qref_dn11 * locals.var_fn241_calc_iq__etad) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn11)), ((locals.var_fn241_calc_iq__qref_dn12 * locals.var_fn241_calc_iq__etad) + (locals.var_fn241_calc_iq__qref * locals.var_fn241_calc_iq__etad_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign20490_e19654;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign20490_e19654_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign20490_e19654_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign20490_e19654_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign20490_e19654_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign20490_e19654_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign20490_e19654_d_n12;

        let assign20500_e19657: f64 = (-50.0);
        let assign20500_e19658: f64 = if locals.var_fn241_calc_iq__etad < assign20500_e19657 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign20500_e19658;

        let (assign20510_e19670, assign20510_e19670_d_n2, assign20510_e19670_d_n3, assign20510_e19670_d_n4, assign20510_e19670_d_n7, assign20510_e19670_d_n11, assign20510_e19670_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 != 0.0)) {
        let assign20510_e19667: f64 = (locals.var_fn241_calc_iq__etad).exp();
        let assign20510_e19668: f64 = (locals.var_fn241_calc_iq__qref * assign20510_e19667);
        (assign20510_e19668, (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn2)), (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn3)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20510_e19667) + (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn4))), (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn7)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20510_e19667) + (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn11))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20510_e19667) + (locals.var_fn241_calc_iq__qref * (assign20510_e19667 * locals.var_fn241_calc_iq__etad_dn12))),)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign20510_e19670;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign20510_e19670_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign20510_e19670_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign20510_e19670_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign20510_e19670_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign20510_e19670_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign20510_e19670_d_n12;

        let (assign20520_e19686, assign20520_e19686_d_n2, assign20520_e19686_d_n3, assign20520_e19686_d_n4, assign20520_e19686_d_n7, assign20520_e19686_d_n11, assign20520_e19686_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard253 == 0.0)) && (locals.var_guard254 == 0.0)) {
        let assign20520_e19681: f64 = (locals.var_fn241_calc_iq__etad).exp();
        let assign20520_e19682: f64 = (1.0 + assign20520_e19681);
        let assign20520_e19683: f64 = (assign20520_e19682).ln();
        let assign20520_e19684: f64 = (locals.var_fn241_calc_iq__qref * assign20520_e19683);
        (assign20520_e19684, (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn2) / assign20520_e19682)), (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn3) / assign20520_e19682)), ((locals.var_fn241_calc_iq__qref_dn4 * assign20520_e19683) + (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn4) / assign20520_e19682))), (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn7) / assign20520_e19682)), ((locals.var_fn241_calc_iq__qref_dn11 * assign20520_e19683) + (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn11) / assign20520_e19682))), ((locals.var_fn241_calc_iq__qref_dn12 * assign20520_e19683) + (locals.var_fn241_calc_iq__qref * ((assign20520_e19681 * locals.var_fn241_calc_iq__etad_dn12) / assign20520_e19682))),)
    } else {
        (locals.var_fn241_calc_iq__qinvd, locals.var_fn241_calc_iq__qinvd_dn2, locals.var_fn241_calc_iq__qinvd_dn3, locals.var_fn241_calc_iq__qinvd_dn4, locals.var_fn241_calc_iq__qinvd_dn7, locals.var_fn241_calc_iq__qinvd_dn11, locals.var_fn241_calc_iq__qinvd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd = assign20520_e19686;
        locals.var_fn241_calc_iq__qinvd_dn2 = assign20520_e19686_d_n2;
        locals.var_fn241_calc_iq__qinvd_dn3 = assign20520_e19686_d_n3;
        locals.var_fn241_calc_iq__qinvd_dn4 = assign20520_e19686_d_n4;
        locals.var_fn241_calc_iq__qinvd_dn7 = assign20520_e19686_d_n7;
        locals.var_fn241_calc_iq__qinvd_dn11 = assign20520_e19686_d_n11;
        locals.var_fn241_calc_iq__qinvd_dn12 = assign20520_e19686_d_n12;

        let (assign20530_e19694, assign20530_e19694_d_n2, assign20530_e19694_d_n3, assign20530_e19694_d_n4, assign20530_e19694_d_n7, assign20530_e19694_d_n11, assign20530_e19694_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20530_e19690: f64 = (locals.var_fn241_calc_iq__qinvs - locals.var_fn241_calc_iq__qinvd);
        let assign20530_e19692: f64 = (assign20530_e19690 / locals.var_fn241_calc_iq__cgin);
        (assign20530_e19692, ((locals.var_fn241_calc_iq__qinvs_dn2 - locals.var_fn241_calc_iq__qinvd_dn2) / locals.var_fn241_calc_iq__cgin), ((locals.var_fn241_calc_iq__qinvs_dn3 - locals.var_fn241_calc_iq__qinvd_dn3) / locals.var_fn241_calc_iq__cgin), ((((locals.var_fn241_calc_iq__qinvs_dn4 - locals.var_fn241_calc_iq__qinvd_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20530_e19690 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)), ((locals.var_fn241_calc_iq__qinvs_dn7 - locals.var_fn241_calc_iq__qinvd_dn7) / locals.var_fn241_calc_iq__cgin), ((locals.var_fn241_calc_iq__qinvs_dn11 - locals.var_fn241_calc_iq__qinvd_dn11) / locals.var_fn241_calc_iq__cgin), ((locals.var_fn241_calc_iq__qinvs_dn12 - locals.var_fn241_calc_iq__qinvd_dn12) / locals.var_fn241_calc_iq__cgin),)
    } else {
        (locals.var_fn241_calc_iq__vdsc, locals.var_fn241_calc_iq__vdsc_dn2, locals.var_fn241_calc_iq__vdsc_dn3, locals.var_fn241_calc_iq__vdsc_dn4, locals.var_fn241_calc_iq__vdsc_dn7, locals.var_fn241_calc_iq__vdsc_dn11, locals.var_fn241_calc_iq__vdsc_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsc = assign20530_e19694;
        locals.var_fn241_calc_iq__vdsc_dn2 = assign20530_e19694_d_n2;
        locals.var_fn241_calc_iq__vdsc_dn3 = assign20530_e19694_d_n3;
        locals.var_fn241_calc_iq__vdsc_dn4 = assign20530_e19694_d_n4;
        locals.var_fn241_calc_iq__vdsc_dn7 = assign20530_e19694_d_n7;
        locals.var_fn241_calc_iq__vdsc_dn11 = assign20530_e19694_d_n11;
        locals.var_fn241_calc_iq__vdsc_dn12 = assign20530_e19694_d_n12;

        let (assign20540_e19700, assign20540_e19700_d_n2, assign20540_e19700_d_n3, assign20540_e19700_d_n4, assign20540_e19700_d_n7, assign20540_e19700_d_n11, assign20540_e19700_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20540_e19698: f64 = (locals.var_fn241_calc_iq__vdsc / locals.var_fn241_calc_iq__vdsat);
        (assign20540_e19698, (((locals.var_fn241_calc_iq__vdsc_dn2 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn2)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn3 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn3)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn4 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn4)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn7 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn7)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn11 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn11)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)), (((locals.var_fn241_calc_iq__vdsc_dn12 * locals.var_fn241_calc_iq__vdsat) - (locals.var_fn241_calc_iq__vdsc * locals.var_fn241_calc_iq__vdsat_dn12)) / (locals.var_fn241_calc_iq__vdsat * locals.var_fn241_calc_iq__vdsat)),)
    } else {
        (locals.var_fn241_calc_iq__myarg, locals.var_fn241_calc_iq__myarg_dn2, locals.var_fn241_calc_iq__myarg_dn3, locals.var_fn241_calc_iq__myarg_dn4, locals.var_fn241_calc_iq__myarg_dn7, locals.var_fn241_calc_iq__myarg_dn11, locals.var_fn241_calc_iq__myarg_dn12,)
    }
};
        locals.var_fn241_calc_iq__myarg = assign20540_e19700;
        locals.var_fn241_calc_iq__myarg_dn2 = assign20540_e19700_d_n2;
        locals.var_fn241_calc_iq__myarg_dn3 = assign20540_e19700_d_n3;
        locals.var_fn241_calc_iq__myarg_dn4 = assign20540_e19700_d_n4;
        locals.var_fn241_calc_iq__myarg_dn7 = assign20540_e19700_d_n7;
        locals.var_fn241_calc_iq__myarg_dn11 = assign20540_e19700_d_n11;
        locals.var_fn241_calc_iq__myarg_dn12 = assign20540_e19700_d_n12;

        let (assign20580_e19769, assign20580_e19769_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20580_e19766: f64 = (2.302585092994046 * locals.var_fn241_calc_iq__phitin);
        let assign20580_e19767: f64 = (locals.var_fn241_calc_iq__ss / assign20580_e19766);
        (assign20580_e19767, (-((locals.var_fn241_calc_iq__ss * (2.302585092994046 * locals.var_fn241_calc_iq__phitin_dn4)) / (assign20580_e19766 * assign20580_e19766))),)
    } else {
        (locals.var_fn241_calc_iq__n0, locals.var_fn241_calc_iq__n0_dn4,)
    }
};
        locals.var_fn241_calc_iq__n0 = assign20580_e19769;
        locals.var_fn241_calc_iq__n0_dn4 = assign20580_e19769_d_n4;

        let (assign20590_e19777, assign20590_e19777_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20590_e19773: f64 = (2.0 * locals.var_fn241_calc_iq__n0);
        let assign20590_e19775: f64 = (assign20590_e19773 * locals.var_fn241_calc_iq__phitin);
        (assign20590_e19775, (((2.0 * locals.var_fn241_calc_iq__n0_dn4) * locals.var_fn241_calc_iq__phitin) + (assign20590_e19773 * locals.var_fn241_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn241_calc_iq__two_n_phit0, locals.var_fn241_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn241_calc_iq__two_n_phit0 = assign20590_e19777;
        locals.var_fn241_calc_iq__two_n_phit0_dn4 = assign20590_e19777_d_n4;

        let (assign20600_e19783, assign20600_e19783_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20600_e19781: f64 = (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit0);
        (assign20600_e19781, ((locals.var_fn241_calc_iq__cgin_dn4 * locals.var_fn241_calc_iq__two_n_phit0) + (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn241_calc_iq__qref0, locals.var_fn241_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn241_calc_iq__qref0 = assign20600_e19783;
        locals.var_fn241_calc_iq__qref0_dn4 = assign20600_e19783_d_n4;

        let (assign20610_e19793, assign20610_e19793_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20610_e19788: f64 = (p.p51 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20610_e19790: f64 = (assign20610_e19788 / 2.0);
        let assign20610_e19791: f64 = (locals.var_fn241_calc_iq__vtof - assign20610_e19790);
        (assign20610_e19791, (locals.var_fn241_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn241_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn241_calc_iq__myarg0, locals.var_fn241_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn241_calc_iq__myarg0 = assign20610_e19793;
        locals.var_fn241_calc_iq__myarg0_dn4 = assign20610_e19793_d_n4;

    }

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20620_e19844, assign20620_e19844_d_n2, assign20620_e19844_d_n4, assign20620_e19844_d_n7, assign20620_e19844_d_n11, assign20620_e19844_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20620_e19838, assign20620_e19838_d_n2, assign20620_e19838_d_n7, assign20620_e19838_d_n11, assign20620_e19838_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20620_e19802: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20620_e19805: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20620_e19808: f64 = (0.001 / p.p53);
                let assign20620_e19811: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20620_e19812: f64 = (assign20620_e19808 * assign20620_e19811);
                let assign20620_e19813: f64 = (assign20620_e19812).tanh();
                let assign20620_e19814: f64 = (assign20620_e19805 * assign20620_e19813);
                let assign20620_e19815: f64 = (assign20620_e19802 + assign20620_e19814);
                let assign20620_e19816: f64 = (0.5 * assign20620_e19815);
                (assign20620_e19816, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20620_e19813) + (assign20620_e19805 * ((assign20620_e19808 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20620_e19812).cosh() * (assign20620_e19812).cosh())))))),)
            } else {
                let (assign20620_e19837, assign20620_e19837_d_n2, assign20620_e19837_d_n7, assign20620_e19837_d_n11, assign20620_e19837_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20620_e19823: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20620_e19826: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20620_e19829: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20620_e19830: f64 = (assign20620_e19826 * assign20620_e19829);
                        let assign20620_e19832: f64 = (assign20620_e19830 + p.p53);
                        let assign20620_e19833: f64 = (assign20620_e19832).sqrt();
                        let assign20620_e19834: f64 = (assign20620_e19823 + assign20620_e19833);
                        let assign20620_e19835: f64 = (0.5 * assign20620_e19834);
                        (assign20620_e19835, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20620_e19829) + (assign20620_e19826 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20620_e19833)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20620_e19829) + (assign20620_e19826 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20620_e19833)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20620_e19829) + (assign20620_e19826 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20620_e19833)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20620_e19829) + (assign20620_e19826 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20620_e19833)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20620_e19837, assign20620_e19837_d_n2, assign20620_e19837_d_n7, assign20620_e19837_d_n11, assign20620_e19837_d_n12,)
            }
        };
        let assign20620_e19840: f64 = (assign20620_e19838 - locals.var_fn241_calc_iq__myarg0);
        let assign20620_e19842: f64 = (assign20620_e19840 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20620_e19842, (assign20620_e19838_d_n2 / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg0_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20620_e19840 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), (assign20620_e19838_d_n7 / locals.var_fn241_calc_iq__alpha_phit), (assign20620_e19838_d_n11 / locals.var_fn241_calc_iq__alpha_phit), (assign20620_e19838_d_n12 / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign20620_e19844;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign20620_e19844_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign20620_e19844_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign20620_e19844_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign20620_e19844_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign20620_e19844_d_n12;

        let assign20630_e19847: f64 = if locals.var_fn241_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign20630_e19847;

        let (assign20640_e19853, assign20640_e19853_d_n2, assign20640_e19853_d_n4, assign20640_e19853_d_n7, assign20640_e19853_d_n11, assign20640_e19853_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard255 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign20640_e19853;
        locals.var_fn241_calc_iq__ff0_dn2 = assign20640_e19853_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign20640_e19853_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign20640_e19853_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign20640_e19853_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign20640_e19853_d_n12;

        let assign20650_e19856: f64 = (-50.0);
        let assign20650_e19857: f64 = if locals.var_fn241_calc_iq__exparg0 < assign20650_e19856 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign20650_e19857;

        let (assign20660_e19866, assign20660_e19866_d_n2, assign20660_e19866_d_n4, assign20660_e19866_d_n7, assign20660_e19866_d_n11, assign20660_e19866_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard255 == 0.0)) && (locals.var_guard256 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign20660_e19866;
        locals.var_fn241_calc_iq__ff0_dn2 = assign20660_e19866_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign20660_e19866_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign20660_e19866_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign20660_e19866_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign20660_e19866_d_n12;

        let (assign20670_e19881, assign20670_e19881_d_n2, assign20670_e19881_d_n4, assign20670_e19881_d_n7, assign20670_e19881_d_n11, assign20670_e19881_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard255 == 0.0)) && (locals.var_guard256 == 0.0)) {
        let assign20670_e19877: f64 = (locals.var_fn241_calc_iq__exparg0).exp();
        let assign20670_e19878: f64 = (1.0 + assign20670_e19877);
        let assign20670_e19879: f64 = (1.0 / assign20670_e19878);
        (assign20670_e19879, (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn2) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn4) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn7) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn11) / (assign20670_e19878 * assign20670_e19878))), (-((assign20670_e19877 * locals.var_fn241_calc_iq__exparg0_dn12) / (assign20670_e19878 * assign20670_e19878))),)
    } else {
        (locals.var_fn241_calc_iq__ff0, locals.var_fn241_calc_iq__ff0_dn2, locals.var_fn241_calc_iq__ff0_dn4, locals.var_fn241_calc_iq__ff0_dn7, locals.var_fn241_calc_iq__ff0_dn11, locals.var_fn241_calc_iq__ff0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ff0 = assign20670_e19881;
        locals.var_fn241_calc_iq__ff0_dn2 = assign20670_e19881_d_n2;
        locals.var_fn241_calc_iq__ff0_dn4 = assign20670_e19881_d_n4;
        locals.var_fn241_calc_iq__ff0_dn7 = assign20670_e19881_d_n7;
        locals.var_fn241_calc_iq__ff0_dn11 = assign20670_e19881_d_n11;
        locals.var_fn241_calc_iq__ff0_dn12 = assign20670_e19881_d_n12;

        let (assign20680_e19940, assign20680_e19940_d_n2, assign20680_e19940_d_n4, assign20680_e19940_d_n7, assign20680_e19940_d_n11, assign20680_e19940_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20680_e19926, assign20680_e19926_d_n2, assign20680_e19926_d_n7, assign20680_e19926_d_n11, assign20680_e19926_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20680_e19890: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                let assign20680_e19893: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20680_e19896: f64 = (0.001 / p.p53);
                let assign20680_e19899: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                let assign20680_e19900: f64 = (assign20680_e19896 * assign20680_e19899);
                let assign20680_e19901: f64 = (assign20680_e19900).tanh();
                let assign20680_e19902: f64 = (assign20680_e19893 * assign20680_e19901);
                let assign20680_e19903: f64 = (assign20680_e19890 + assign20680_e19902);
                let assign20680_e19904: f64 = (0.5 * assign20680_e19903);
                (assign20680_e19904, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + (((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (-locals.var_fn241_calc_iq__vgdin_dn11)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20680_e19901) + (assign20680_e19893 * ((assign20680_e19896 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12)) / ((assign20680_e19900).cosh() * (assign20680_e19900).cosh())))))),)
            } else {
                let (assign20680_e19925, assign20680_e19925_d_n2, assign20680_e19925_d_n7, assign20680_e19925_d_n11, assign20680_e19925_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20680_e19911: f64 = (locals.var_fn241_calc_iq__vgsin + locals.var_fn241_calc_iq__vgdin);
                        let assign20680_e19914: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20680_e19917: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vgdin);
                        let assign20680_e19918: f64 = (assign20680_e19914 * assign20680_e19917);
                        let assign20680_e19920: f64 = (assign20680_e19918 + p.p53);
                        let assign20680_e19921: f64 = (assign20680_e19920).sqrt();
                        let assign20680_e19922: f64 = (assign20680_e19911 + assign20680_e19921);
                        let assign20680_e19923: f64 = (0.5 * assign20680_e19922);
                        (assign20680_e19923, (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn2 + locals.var_fn241_calc_iq__vgdin_dn2) + ((((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2) * assign20680_e19917) + (assign20680_e19914 * (locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vgdin_dn2))) / (2.0 * assign20680_e19921)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn7 + locals.var_fn241_calc_iq__vgdin_dn7) + ((((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7) * assign20680_e19917) + (assign20680_e19914 * (locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vgdin_dn7))) / (2.0 * assign20680_e19921)))), (0.5 * (locals.var_fn241_calc_iq__vgdin_dn11 + ((((-locals.var_fn241_calc_iq__vgdin_dn11) * assign20680_e19917) + (assign20680_e19914 * (-locals.var_fn241_calc_iq__vgdin_dn11))) / (2.0 * assign20680_e19921)))), (0.5 * ((locals.var_fn241_calc_iq__vgsin_dn12 + locals.var_fn241_calc_iq__vgdin_dn12) + ((((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12) * assign20680_e19917) + (assign20680_e19914 * (locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vgdin_dn12))) / (2.0 * assign20680_e19921)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20680_e19925, assign20680_e19925_d_n2, assign20680_e19925_d_n7, assign20680_e19925_d_n11, assign20680_e19925_d_n12,)
            }
        };
        let assign20680_e19930: f64 = (p.p51 * 0.1);
        let assign20680_e19932: f64 = (assign20680_e19930 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20680_e19934: f64 = (assign20680_e19932 * locals.var_fn241_calc_iq__ff0);
        let assign20680_e19935: f64 = (locals.var_fn241_calc_iq__vtof - assign20680_e19934);
        let assign20680_e19936: f64 = (assign20680_e19926 - assign20680_e19935);
        let assign20680_e19938: f64 = (assign20680_e19936 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign20680_e19938, ((assign20680_e19926_d_n2 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn2))) / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (((assign20680_e19930 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ff0) + (assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn4)))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign20680_e19936 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), ((assign20680_e19926_d_n7 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn7))) / locals.var_fn241_calc_iq__two_n_phit0), ((assign20680_e19926_d_n11 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn11))) / locals.var_fn241_calc_iq__two_n_phit0), ((assign20680_e19926_d_n12 - (-(assign20680_e19932 * locals.var_fn241_calc_iq__ff0_dn12))) / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__eta0, locals.var_fn241_calc_iq__eta0_dn2, locals.var_fn241_calc_iq__eta0_dn4, locals.var_fn241_calc_iq__eta0_dn7, locals.var_fn241_calc_iq__eta0_dn11, locals.var_fn241_calc_iq__eta0_dn12,)
    }
};
        locals.var_fn241_calc_iq__eta0 = assign20680_e19940;
        locals.var_fn241_calc_iq__eta0_dn2 = assign20680_e19940_d_n2;
        locals.var_fn241_calc_iq__eta0_dn4 = assign20680_e19940_d_n4;
        locals.var_fn241_calc_iq__eta0_dn7 = assign20680_e19940_d_n7;
        locals.var_fn241_calc_iq__eta0_dn11 = assign20680_e19940_d_n11;
        locals.var_fn241_calc_iq__eta0_dn12 = assign20680_e19940_d_n12;

        let assign20690_e19943: f64 = if locals.var_fn241_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign20690_e19943;

        let (assign20700_e19951, assign20700_e19951_d_n2, assign20700_e19951_d_n4, assign20700_e19951_d_n7, assign20700_e19951_d_n11, assign20700_e19951_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard257 != 0.0)) {
        let assign20700_e19949: f64 = (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0);
        (assign20700_e19949, (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn2), ((locals.var_fn241_calc_iq__qref0_dn4 * locals.var_fn241_calc_iq__eta0) + (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn4)), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn7), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn11), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__eta0_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign20700_e19951;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign20700_e19951_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign20700_e19951_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign20700_e19951_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign20700_e19951_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign20700_e19951_d_n12;

        let assign20710_e19954: f64 = (-50.0);
        let assign20710_e19955: f64 = if locals.var_fn241_calc_iq__eta0 < assign20710_e19954 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign20710_e19955;

        let (assign20720_e19967, assign20720_e19967_d_n2, assign20720_e19967_d_n4, assign20720_e19967_d_n7, assign20720_e19967_d_n11, assign20720_e19967_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard257 == 0.0)) && (locals.var_guard258 != 0.0)) {
        let assign20720_e19964: f64 = (locals.var_fn241_calc_iq__eta0).exp();
        let assign20720_e19965: f64 = (locals.var_fn241_calc_iq__qref0 * assign20720_e19964);
        (assign20720_e19965, (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn2)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20720_e19964) + (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn4))), (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn7)), (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn11)), (locals.var_fn241_calc_iq__qref0 * (assign20720_e19964 * locals.var_fn241_calc_iq__eta0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign20720_e19967;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign20720_e19967_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign20720_e19967_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign20720_e19967_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign20720_e19967_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign20720_e19967_d_n12;

        let (assign20730_e19983, assign20730_e19983_d_n2, assign20730_e19983_d_n4, assign20730_e19983_d_n7, assign20730_e19983_d_n11, assign20730_e19983_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard257 == 0.0)) && (locals.var_guard258 == 0.0)) {
        let assign20730_e19978: f64 = (locals.var_fn241_calc_iq__eta0).exp();
        let assign20730_e19979: f64 = (1.0 + assign20730_e19978);
        let assign20730_e19980: f64 = (assign20730_e19979).ln();
        let assign20730_e19981: f64 = (locals.var_fn241_calc_iq__qref0 * assign20730_e19980);
        (assign20730_e19981, (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn2) / assign20730_e19979)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20730_e19980) + (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn4) / assign20730_e19979))), (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn7) / assign20730_e19979)), (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn11) / assign20730_e19979)), (locals.var_fn241_calc_iq__qref0 * ((assign20730_e19978 * locals.var_fn241_calc_iq__eta0_dn12) / assign20730_e19979)),)
    } else {
        (locals.var_fn241_calc_iq__qinvv0, locals.var_fn241_calc_iq__qinvv0_dn2, locals.var_fn241_calc_iq__qinvv0_dn4, locals.var_fn241_calc_iq__qinvv0_dn7, locals.var_fn241_calc_iq__qinvv0_dn11, locals.var_fn241_calc_iq__qinvv0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvv0 = assign20730_e19983;
        locals.var_fn241_calc_iq__qinvv0_dn2 = assign20730_e19983_d_n2;
        locals.var_fn241_calc_iq__qinvv0_dn4 = assign20730_e19983_d_n4;
        locals.var_fn241_calc_iq__qinvv0_dn7 = assign20730_e19983_d_n7;
        locals.var_fn241_calc_iq__qinvv0_dn11 = assign20730_e19983_d_n11;
        locals.var_fn241_calc_iq__qinvv0_dn12 = assign20730_e19983_d_n12;

        let (assign20740_e19989, assign20740_e19989_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20740_e19987: f64 = (locals.var_fn241_calc_iq__mu0 / locals.var_fn241_calc_iq__tfacmobin);
        (assign20740_e19987, (-((locals.var_fn241_calc_iq__mu0 * locals.var_fn241_calc_iq__tfacmobin_dn4) / (locals.var_fn241_calc_iq__tfacmobin * locals.var_fn241_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn241_calc_iq__muf0, locals.var_fn241_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn241_calc_iq__muf0 = assign20740_e19989;
        locals.var_fn241_calc_iq__muf0_dn4 = assign20740_e19989_d_n4;

        let (assign20750_e20005, assign20750_e20005_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20750_e19995: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tnomin);
        let assign20750_e19996: f64 = (1.0 + assign20750_e19995);
        let assign20750_e20000: f64 = (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin);
        let assign20750_e20001: f64 = (1.0 + assign20750_e20000);
        let assign20750_e20002: f64 = (assign20750_e19996 / assign20750_e20001);
        let assign20750_e20003: f64 = (locals.var_fn241_calc_iq__vel0 * assign20750_e20002);
        (assign20750_e20003, (locals.var_fn241_calc_iq__vel0 * (-((assign20750_e19996 * (locals.var_fn241_calc_iq__vzeta * locals.var_fn241_calc_iq__tambin_dn4)) / (assign20750_e20001 * assign20750_e20001)))),)
    } else {
        (locals.var_fn241_calc_iq__vx0, locals.var_fn241_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vx0 = assign20750_e20005;
        locals.var_fn241_calc_iq__vx0_dn4 = assign20750_e20005_d_n4;

        let (assign20760_e20013, assign20760_e20013_d_n4,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20760_e20009: f64 = (locals.var_fn241_calc_iq__vx0 * locals.var_fn241_calc_iq__lin);
        let assign20760_e20011: f64 = (assign20760_e20009 / locals.var_fn241_calc_iq__muf0);
        (assign20760_e20011, ((((locals.var_fn241_calc_iq__vx0_dn4 * locals.var_fn241_calc_iq__lin) * locals.var_fn241_calc_iq__muf0) - (assign20760_e20009 * locals.var_fn241_calc_iq__muf0_dn4)) / (locals.var_fn241_calc_iq__muf0 * locals.var_fn241_calc_iq__muf0)),)
    } else {
        (locals.var_fn241_calc_iq__vdsats0, locals.var_fn241_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn241_calc_iq__vdsats0 = assign20760_e20013;
        locals.var_fn241_calc_iq__vdsats0_dn4 = assign20760_e20013_d_n4;

        let (assign20770_e20030, assign20770_e20030_d_n2, assign20770_e20030_d_n4, assign20770_e20030_d_n7, assign20770_e20030_d_n11, assign20770_e20030_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20770_e20019: f64 = (2.0 * locals.var_fn241_calc_iq__qinvv0);
        let assign20770_e20021: f64 = (assign20770_e20019 / locals.var_fn241_calc_iq__cgin);
        let assign20770_e20023: f64 = (assign20770_e20021 / locals.var_fn241_calc_iq__vdsats0);
        let assign20770_e20024: f64 = (1.0 + assign20770_e20023);
        let assign20770_e20025: f64 = (assign20770_e20024).sqrt();
        let assign20770_e20026: f64 = (locals.var_fn241_calc_iq__vdsats0 * assign20770_e20025);
        let assign20770_e20028: f64 = (assign20770_e20026 - locals.var_fn241_calc_iq__vdsats0);
        (assign20770_e20028, (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn2) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))), (((locals.var_fn241_calc_iq__vdsats0_dn4 * assign20770_e20025) + (locals.var_fn241_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn4) * locals.var_fn241_calc_iq__cgin) - (assign20770_e20019 * locals.var_fn241_calc_iq__cgin_dn4)) / (locals.var_fn241_calc_iq__cgin * locals.var_fn241_calc_iq__cgin)) * locals.var_fn241_calc_iq__vdsats0) - (assign20770_e20021 * locals.var_fn241_calc_iq__vdsats0_dn4)) / (locals.var_fn241_calc_iq__vdsats0 * locals.var_fn241_calc_iq__vdsats0)) / (2.0 * assign20770_e20025)))) - locals.var_fn241_calc_iq__vdsats0_dn4), (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn7) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))), (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn11) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))), (locals.var_fn241_calc_iq__vdsats0 * ((((2.0 * locals.var_fn241_calc_iq__qinvv0_dn12) / locals.var_fn241_calc_iq__cgin) / locals.var_fn241_calc_iq__vdsats0) / (2.0 * assign20770_e20025))),)
    } else {
        (locals.var_fn241_calc_iq__vdsats10, locals.var_fn241_calc_iq__vdsats10_dn2, locals.var_fn241_calc_iq__vdsats10_dn4, locals.var_fn241_calc_iq__vdsats10_dn7, locals.var_fn241_calc_iq__vdsats10_dn11, locals.var_fn241_calc_iq__vdsats10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsats10 = assign20770_e20030;
        locals.var_fn241_calc_iq__vdsats10_dn2 = assign20770_e20030_d_n2;
        locals.var_fn241_calc_iq__vdsats10_dn4 = assign20770_e20030_d_n4;
        locals.var_fn241_calc_iq__vdsats10_dn7 = assign20770_e20030_d_n7;
        locals.var_fn241_calc_iq__vdsats10_dn11 = assign20770_e20030_d_n11;
        locals.var_fn241_calc_iq__vdsats10_dn12 = assign20770_e20030_d_n12;

        let (assign20780_e20042, assign20780_e20042_d_n2, assign20780_e20042_d_n4, assign20780_e20042_d_n7, assign20780_e20042_d_n11, assign20780_e20042_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20780_e20035: f64 = (1.0 - locals.var_fn241_calc_iq__ff0);
        let assign20780_e20036: f64 = (locals.var_fn241_calc_iq__vdsats10 * assign20780_e20035);
        let assign20780_e20039: f64 = (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0);
        let assign20780_e20040: f64 = (assign20780_e20036 + assign20780_e20039);
        (assign20780_e20040, (((locals.var_fn241_calc_iq__vdsats10_dn2 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn2))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn2)), (((locals.var_fn241_calc_iq__vdsats10_dn4 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn4))) + ((locals.var_fn241_calc_iq__two_n_phit0_dn4 * locals.var_fn241_calc_iq__ff0) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn4))), (((locals.var_fn241_calc_iq__vdsats10_dn7 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn7))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn7)), (((locals.var_fn241_calc_iq__vdsats10_dn11 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn11))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn11)), (((locals.var_fn241_calc_iq__vdsats10_dn12 * assign20780_e20035) + (locals.var_fn241_calc_iq__vdsats10 * (-locals.var_fn241_calc_iq__ff0_dn12))) + (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__ff0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vdsat10, locals.var_fn241_calc_iq__vdsat10_dn2, locals.var_fn241_calc_iq__vdsat10_dn4, locals.var_fn241_calc_iq__vdsat10_dn7, locals.var_fn241_calc_iq__vdsat10_dn11, locals.var_fn241_calc_iq__vdsat10_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdsat10 = assign20780_e20042;
        locals.var_fn241_calc_iq__vdsat10_dn2 = assign20780_e20042_d_n2;
        locals.var_fn241_calc_iq__vdsat10_dn4 = assign20780_e20042_d_n4;
        locals.var_fn241_calc_iq__vdsat10_dn7 = assign20780_e20042_d_n7;
        locals.var_fn241_calc_iq__vdsat10_dn11 = assign20780_e20042_d_n11;
        locals.var_fn241_calc_iq__vdsat10_dn12 = assign20780_e20042_d_n12;

        let (assign20790_e20111, assign20790_e20111_d_n2, assign20790_e20111_d_n4, assign20790_e20111_d_n7, assign20790_e20111_d_n11, assign20790_e20111_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20790_e20101, assign20790_e20101_d_n2, assign20790_e20101_d_n4, assign20790_e20101_d_n7, assign20790_e20101_d_n11, assign20790_e20101_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20790_e20054: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                let assign20790_e20055: f64 = assign20790_e20054;
                let assign20790_e20059: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                let assign20790_e20060: f64 = (-assign20790_e20059);
                let assign20790_e20063: f64 = (0.001 / p.p53);
                let assign20790_e20067: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                let assign20790_e20068: f64 = (-assign20790_e20067);
                let assign20790_e20069: f64 = (assign20790_e20063 * assign20790_e20068);
                let assign20790_e20070: f64 = (assign20790_e20069).tanh();
                let assign20790_e20071: f64 = (assign20790_e20060 * assign20790_e20070);
                let assign20790_e20072: f64 = (assign20790_e20055 + assign20790_e20071);
                let assign20790_e20073: f64 = (0.5 * assign20790_e20072);
                (assign20790_e20073, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20070) + (assign20790_e20060 * ((assign20790_e20063 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20790_e20069).cosh() * (assign20790_e20069).cosh())))))),)
            } else {
                let (assign20790_e20100, assign20790_e20100_d_n2, assign20790_e20100_d_n4, assign20790_e20100_d_n7, assign20790_e20100_d_n11, assign20790_e20100_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20790_e20081: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                        let assign20790_e20082: f64 = assign20790_e20081;
                        let assign20790_e20086: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                        let assign20790_e20087: f64 = (-assign20790_e20086);
                        let assign20790_e20091: f64 = (locals.var_fn241_calc_iq__vdsin / locals.var_fn241_calc_iq__vdsat10);
                        let assign20790_e20092: f64 = (-assign20790_e20091);
                        let assign20790_e20093: f64 = (assign20790_e20087 * assign20790_e20092);
                        let assign20790_e20095: f64 = (assign20790_e20093 + p.p53);
                        let assign20790_e20096: f64 = (assign20790_e20095).sqrt();
                        let assign20790_e20097: f64 = (assign20790_e20082 + assign20790_e20096);
                        let assign20790_e20098: f64 = (0.5 * assign20790_e20097);
                        (assign20790_e20098, (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20092) + (assign20790_e20087 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20790_e20096)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20092) + (assign20790_e20087 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20790_e20096)))), (0.5 * ((-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20790_e20092) + (assign20790_e20087 * (-(-((locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20790_e20096)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20092) + (assign20790_e20087 * (-(((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20790_e20096)))), (0.5 * ((((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20790_e20092) + (assign20790_e20087 * (-(((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__vdsat10) - (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20790_e20096)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20790_e20100, assign20790_e20100_d_n2, assign20790_e20100_d_n4, assign20790_e20100_d_n7, assign20790_e20100_d_n11, assign20790_e20100_d_n12,)
            }
        };
        let assign20790_e20103: f64 = (assign20790_e20101).powf(locals.var_fn241_calc_iq__beta);
        let assign20790_e20104: f64 = (1.0 + assign20790_e20103);
        let assign20790_e20107: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20790_e20108: f64 = (assign20790_e20104).powf(assign20790_e20107);
        let assign20790_e20109: f64 = (1.0 / assign20790_e20108);
        (assign20790_e20109, (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n2)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n2 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n2)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n2 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n4)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n4 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n4)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n4 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n7)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n7 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n7)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n7 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n11)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n11 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n11)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n11 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))), (-(if 0.0 == 0.0 && ((assign20790_e20107) as f64).is_finite() && ((assign20790_e20107) as f64).fract() == 0.0 { if assign20790_e20107 == 0.0 { 0.0 } else { (assign20790_e20107 * ((assign20790_e20104).powf(assign20790_e20107 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n12)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n12 / assign20790_e20101))) })) } } else { (assign20790_e20108 * (assign20790_e20107 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20790_e20101).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20790_e20101_d_n12)) } } else { (assign20790_e20103 * (locals.var_fn241_calc_iq__beta * (assign20790_e20101_d_n12 / assign20790_e20101))) } / assign20790_e20104))) } / (assign20790_e20108 * assign20790_e20108))),)
    } else {
        (locals.var_fn241_calc_iq__fsd0, locals.var_fn241_calc_iq__fsd0_dn2, locals.var_fn241_calc_iq__fsd0_dn4, locals.var_fn241_calc_iq__fsd0_dn7, locals.var_fn241_calc_iq__fsd0_dn11, locals.var_fn241_calc_iq__fsd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fsd0 = assign20790_e20111;
        locals.var_fn241_calc_iq__fsd0_dn2 = assign20790_e20111_d_n2;
        locals.var_fn241_calc_iq__fsd0_dn4 = assign20790_e20111_d_n4;
        locals.var_fn241_calc_iq__fsd0_dn7 = assign20790_e20111_d_n7;
        locals.var_fn241_calc_iq__fsd0_dn11 = assign20790_e20111_d_n11;
        locals.var_fn241_calc_iq__fsd0_dn12 = assign20790_e20111_d_n12;

        let (assign20800_e20117, assign20800_e20117_d_n2, assign20800_e20117_d_n4, assign20800_e20117_d_n7, assign20800_e20117_d_n11, assign20800_e20117_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20800_e20115: f64 = (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0);
        (assign20800_e20115, (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn2), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn4), (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn7), ((locals.var_fn241_calc_iq__vdsin_dn11 * locals.var_fn241_calc_iq__fsd0) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn11)), ((locals.var_fn241_calc_iq__vdsin_dn12 * locals.var_fn241_calc_iq__fsd0) + (locals.var_fn241_calc_iq__vdsin * locals.var_fn241_calc_iq__fsd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vdx0, locals.var_fn241_calc_iq__vdx0_dn2, locals.var_fn241_calc_iq__vdx0_dn4, locals.var_fn241_calc_iq__vdx0_dn7, locals.var_fn241_calc_iq__vdx0_dn11, locals.var_fn241_calc_iq__vdx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vdx0 = assign20800_e20117;
        locals.var_fn241_calc_iq__vdx0_dn2 = assign20800_e20117_d_n2;
        locals.var_fn241_calc_iq__vdx0_dn4 = assign20800_e20117_d_n4;
        locals.var_fn241_calc_iq__vdx0_dn7 = assign20800_e20117_d_n7;
        locals.var_fn241_calc_iq__vdx0_dn11 = assign20800_e20117_d_n11;
        locals.var_fn241_calc_iq__vdx0_dn12 = assign20800_e20117_d_n12;

        let (assign20810_e20192, assign20810_e20192_d_n2, assign20810_e20192_d_n4, assign20810_e20192_d_n7, assign20810_e20192_d_n11, assign20810_e20192_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let (assign20810_e20182, assign20810_e20182_d_n2, assign20810_e20182_d_n4, assign20810_e20182_d_n7, assign20810_e20182_d_n11, assign20810_e20182_d_n12,) = {
            if (p.p52 != 0.0) {
                let assign20810_e20128: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20810_e20130: f64 = (assign20810_e20128 / locals.var_fn241_calc_iq__vdsat10);
                let assign20810_e20131: f64 = assign20810_e20130;
                let assign20810_e20134: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20810_e20136: f64 = (assign20810_e20134 / locals.var_fn241_calc_iq__vdsat10);
                let assign20810_e20137: f64 = (-assign20810_e20136);
                let assign20810_e20140: f64 = (0.001 / p.p53);
                let assign20810_e20143: f64 = (-locals.var_fn241_calc_iq__vdsin);
                let assign20810_e20145: f64 = (assign20810_e20143 / locals.var_fn241_calc_iq__vdsat10);
                let assign20810_e20146: f64 = (-assign20810_e20145);
                let assign20810_e20147: f64 = (assign20810_e20140 * assign20810_e20146);
                let assign20810_e20148: f64 = (assign20810_e20147).tanh();
                let assign20810_e20149: f64 = (assign20810_e20137 * assign20810_e20148);
                let assign20810_e20150: f64 = (assign20810_e20131 + assign20810_e20149);
                let assign20810_e20151: f64 = (0.5 * assign20810_e20150);
                (assign20810_e20151, (0.5 * ((-((assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-(-((assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * ((-((assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-(-((assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * ((-((assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + (((-(-((assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-(-((assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20128 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + (((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20134 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20148) + (assign20810_e20137 * ((assign20810_e20140 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20143 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) / ((assign20810_e20147).cosh() * (assign20810_e20147).cosh())))))),)
            } else {
                let (assign20810_e20181, assign20810_e20181_d_n2, assign20810_e20181_d_n4, assign20810_e20181_d_n7, assign20810_e20181_d_n11, assign20810_e20181_d_n12,) = {
                    if (p.p52 == 0.0) {
                        let assign20810_e20158: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20810_e20160: f64 = (assign20810_e20158 / locals.var_fn241_calc_iq__vdsat10);
                        let assign20810_e20161: f64 = assign20810_e20160;
                        let assign20810_e20164: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20810_e20166: f64 = (assign20810_e20164 / locals.var_fn241_calc_iq__vdsat10);
                        let assign20810_e20167: f64 = (-assign20810_e20166);
                        let assign20810_e20170: f64 = (-locals.var_fn241_calc_iq__vdsin);
                        let assign20810_e20172: f64 = (assign20810_e20170 / locals.var_fn241_calc_iq__vdsat10);
                        let assign20810_e20173: f64 = (-assign20810_e20172);
                        let assign20810_e20174: f64 = (assign20810_e20167 * assign20810_e20173);
                        let assign20810_e20176: f64 = (assign20810_e20174 + p.p53);
                        let assign20810_e20177: f64 = (assign20810_e20176).sqrt();
                        let assign20810_e20178: f64 = (assign20810_e20161 + assign20810_e20177);
                        let assign20810_e20179: f64 = (0.5 * assign20810_e20178);
                        (assign20810_e20179, (0.5 * ((-((assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20173) + (assign20810_e20167 * (-(-((assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn2) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20810_e20177)))), (0.5 * ((-((assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20173) + (assign20810_e20167 * (-(-((assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn4) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20810_e20177)))), (0.5 * ((-((assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) + ((((-(-((assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))) * assign20810_e20173) + (assign20810_e20167 * (-(-((assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn7) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)))))) / (2.0 * assign20810_e20177)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20173) + (assign20810_e20167 * (-((((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn11)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20810_e20177)))), (0.5 * (((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20158 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10)) + ((((-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20164 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))) * assign20810_e20173) + (assign20810_e20167 * (-((((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__vdsat10) - (assign20810_e20170 * locals.var_fn241_calc_iq__vdsat10_dn12)) / (locals.var_fn241_calc_iq__vdsat10 * locals.var_fn241_calc_iq__vdsat10))))) / (2.0 * assign20810_e20177)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20810_e20181, assign20810_e20181_d_n2, assign20810_e20181_d_n4, assign20810_e20181_d_n7, assign20810_e20181_d_n11, assign20810_e20181_d_n12,)
            }
        };
        let assign20810_e20184: f64 = (assign20810_e20182).powf(locals.var_fn241_calc_iq__beta);
        let assign20810_e20185: f64 = (1.0 + assign20810_e20184);
        let assign20810_e20188: f64 = (1.0 / locals.var_fn241_calc_iq__beta);
        let assign20810_e20189: f64 = (assign20810_e20185).powf(assign20810_e20188);
        let assign20810_e20190: f64 = (1.0 / assign20810_e20189);
        (assign20810_e20190, (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n2)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n2 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n2)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n2 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n4)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n4 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n4)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n4 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n7)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n7 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n7)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n7 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n11)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n11 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n11)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n11 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))), (-(if 0.0 == 0.0 && ((assign20810_e20188) as f64).is_finite() && ((assign20810_e20188) as f64).fract() == 0.0 { if assign20810_e20188 == 0.0 { 0.0 } else { (assign20810_e20188 * ((assign20810_e20185).powf(assign20810_e20188 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n12)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n12 / assign20810_e20182))) })) } } else { (assign20810_e20189 * (assign20810_e20188 * (if 0.0 == 0.0 && ((locals.var_fn241_calc_iq__beta) as f64).is_finite() && ((locals.var_fn241_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn241_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn241_calc_iq__beta * ((assign20810_e20182).powf(locals.var_fn241_calc_iq__beta - 1.0) * assign20810_e20182_d_n12)) } } else { (assign20810_e20184 * (locals.var_fn241_calc_iq__beta * (assign20810_e20182_d_n12 / assign20810_e20182))) } / assign20810_e20185))) } / (assign20810_e20189 * assign20810_e20189))),)
    } else {
        (locals.var_fn241_calc_iq__fds0, locals.var_fn241_calc_iq__fds0_dn2, locals.var_fn241_calc_iq__fds0_dn4, locals.var_fn241_calc_iq__fds0_dn7, locals.var_fn241_calc_iq__fds0_dn11, locals.var_fn241_calc_iq__fds0_dn12,)
    }
};
        locals.var_fn241_calc_iq__fds0 = assign20810_e20192;
        locals.var_fn241_calc_iq__fds0_dn2 = assign20810_e20192_d_n2;
        locals.var_fn241_calc_iq__fds0_dn4 = assign20810_e20192_d_n4;
        locals.var_fn241_calc_iq__fds0_dn7 = assign20810_e20192_d_n7;
        locals.var_fn241_calc_iq__fds0_dn11 = assign20810_e20192_d_n11;
        locals.var_fn241_calc_iq__fds0_dn12 = assign20810_e20192_d_n12;

        let (assign20820_e20199, assign20820_e20199_d_n2, assign20820_e20199_d_n4, assign20820_e20199_d_n7, assign20820_e20199_d_n11, assign20820_e20199_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20820_e20195: f64 = (-locals.var_fn241_calc_iq__vdsin);
        let assign20820_e20197: f64 = (assign20820_e20195 * locals.var_fn241_calc_iq__fds0);
        (assign20820_e20197, (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn2), (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn4), (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn7), (((-locals.var_fn241_calc_iq__vdsin_dn11) * locals.var_fn241_calc_iq__fds0) + (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn11)), (((-locals.var_fn241_calc_iq__vdsin_dn12) * locals.var_fn241_calc_iq__fds0) + (assign20820_e20195 * locals.var_fn241_calc_iq__fds0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__vsx0, locals.var_fn241_calc_iq__vsx0_dn2, locals.var_fn241_calc_iq__vsx0_dn4, locals.var_fn241_calc_iq__vsx0_dn7, locals.var_fn241_calc_iq__vsx0_dn11, locals.var_fn241_calc_iq__vsx0_dn12,)
    }
};
        locals.var_fn241_calc_iq__vsx0 = assign20820_e20199;
        locals.var_fn241_calc_iq__vsx0_dn2 = assign20820_e20199_d_n2;
        locals.var_fn241_calc_iq__vsx0_dn4 = assign20820_e20199_d_n4;
        locals.var_fn241_calc_iq__vsx0_dn7 = assign20820_e20199_d_n7;
        locals.var_fn241_calc_iq__vsx0_dn11 = assign20820_e20199_d_n11;
        locals.var_fn241_calc_iq__vsx0_dn12 = assign20820_e20199_d_n12;

        let (assign20830_e20207, assign20830_e20207_d_n2, assign20830_e20207_d_n4, assign20830_e20207_d_n7, assign20830_e20207_d_n11, assign20830_e20207_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20830_e20203: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__myarg0);
        let assign20830_e20205: f64 = (assign20830_e20203 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20830_e20205, (locals.var_fn241_calc_iq__vgsin_dn2 / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg0_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20830_e20203 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), (locals.var_fn241_calc_iq__vgsin_dn7 / locals.var_fn241_calc_iq__alpha_phit), 0.0, (locals.var_fn241_calc_iq__vgsin_dn12 / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign20830_e20207;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign20830_e20207_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign20830_e20207_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign20830_e20207_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign20830_e20207_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign20830_e20207_d_n12;

        let assign20840_e20210: f64 = if locals.var_fn241_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign20840_e20210;

        let (assign20850_e20216, assign20850_e20216_d_n2, assign20850_e20216_d_n4, assign20850_e20216_d_n7, assign20850_e20216_d_n11, assign20850_e20216_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard259 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign20850_e20216;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign20850_e20216_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign20850_e20216_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign20850_e20216_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign20850_e20216_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign20850_e20216_d_n12;

        let assign20860_e20219: f64 = (-50.0);
        let assign20860_e20220: f64 = if locals.var_fn241_calc_iq__exparg0 < assign20860_e20219 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign20860_e20220;

        let (assign20870_e20229, assign20870_e20229_d_n2, assign20870_e20229_d_n4, assign20870_e20229_d_n7, assign20870_e20229_d_n11, assign20870_e20229_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard260 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign20870_e20229;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign20870_e20229_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign20870_e20229_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign20870_e20229_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign20870_e20229_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign20870_e20229_d_n12;

        let (assign20880_e20244, assign20880_e20244_d_n2, assign20880_e20244_d_n4, assign20880_e20244_d_n7, assign20880_e20244_d_n11, assign20880_e20244_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard260 == 0.0)) {
        let assign20880_e20240: f64 = (locals.var_fn241_calc_iq__exparg0).exp();
        let assign20880_e20241: f64 = (1.0 + assign20880_e20240);
        let assign20880_e20242: f64 = (1.0 / assign20880_e20241);
        (assign20880_e20242, (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn2) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn4) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn7) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn11) / (assign20880_e20241 * assign20880_e20241))), (-((assign20880_e20240 * locals.var_fn241_calc_iq__exparg0_dn12) / (assign20880_e20241 * assign20880_e20241))),)
    } else {
        (locals.var_fn241_calc_iq__ffs0, locals.var_fn241_calc_iq__ffs0_dn2, locals.var_fn241_calc_iq__ffs0_dn4, locals.var_fn241_calc_iq__ffs0_dn7, locals.var_fn241_calc_iq__ffs0_dn11, locals.var_fn241_calc_iq__ffs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffs0 = assign20880_e20244;
        locals.var_fn241_calc_iq__ffs0_dn2 = assign20880_e20244_d_n2;
        locals.var_fn241_calc_iq__ffs0_dn4 = assign20880_e20244_d_n4;
        locals.var_fn241_calc_iq__ffs0_dn7 = assign20880_e20244_d_n7;
        locals.var_fn241_calc_iq__ffs0_dn11 = assign20880_e20244_d_n11;
        locals.var_fn241_calc_iq__ffs0_dn12 = assign20880_e20244_d_n12;

    }

    pub(super) fn stamp_transient_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20890_e20262, assign20890_e20262_d_n2, assign20890_e20262_d_n4, assign20890_e20262_d_n7, assign20890_e20262_d_n11, assign20890_e20262_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20890_e20248: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__vsx0);
        let assign20890_e20252: f64 = (p.p51 * 0.1);
        let assign20890_e20254: f64 = (assign20890_e20252 * locals.var_fn241_calc_iq__alpha_phit);
        let assign20890_e20256: f64 = (assign20890_e20254 * locals.var_fn241_calc_iq__ffs0);
        let assign20890_e20257: f64 = (locals.var_fn241_calc_iq__vtof - assign20890_e20256);
        let assign20890_e20258: f64 = (assign20890_e20248 - assign20890_e20257);
        let assign20890_e20260: f64 = (assign20890_e20258 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign20890_e20260, (((locals.var_fn241_calc_iq__vgdin_dn2 - locals.var_fn241_calc_iq__vsx0_dn2) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn2))) / locals.var_fn241_calc_iq__two_n_phit0), (((((-locals.var_fn241_calc_iq__vsx0_dn4) - (locals.var_fn241_calc_iq__vtof_dn4 - (((assign20890_e20252 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffs0) + (assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn4)))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign20890_e20258 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (((locals.var_fn241_calc_iq__vgdin_dn7 - locals.var_fn241_calc_iq__vsx0_dn7) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn7))) / locals.var_fn241_calc_iq__two_n_phit0), (((locals.var_fn241_calc_iq__vgdin_dn11 - locals.var_fn241_calc_iq__vsx0_dn11) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn11))) / locals.var_fn241_calc_iq__two_n_phit0), (((locals.var_fn241_calc_iq__vgdin_dn12 - locals.var_fn241_calc_iq__vsx0_dn12) - (-(assign20890_e20254 * locals.var_fn241_calc_iq__ffs0_dn12))) / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etas0, locals.var_fn241_calc_iq__etas0_dn2, locals.var_fn241_calc_iq__etas0_dn4, locals.var_fn241_calc_iq__etas0_dn7, locals.var_fn241_calc_iq__etas0_dn11, locals.var_fn241_calc_iq__etas0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etas0 = assign20890_e20262;
        locals.var_fn241_calc_iq__etas0_dn2 = assign20890_e20262_d_n2;
        locals.var_fn241_calc_iq__etas0_dn4 = assign20890_e20262_d_n4;
        locals.var_fn241_calc_iq__etas0_dn7 = assign20890_e20262_d_n7;
        locals.var_fn241_calc_iq__etas0_dn11 = assign20890_e20262_d_n11;
        locals.var_fn241_calc_iq__etas0_dn12 = assign20890_e20262_d_n12;

        let assign20900_e20265: f64 = if locals.var_fn241_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign20900_e20265;

        let (assign20910_e20273, assign20910_e20273_d_n2, assign20910_e20273_d_n4, assign20910_e20273_d_n7, assign20910_e20273_d_n11, assign20910_e20273_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard261 != 0.0)) {
        let assign20910_e20271: f64 = (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0);
        (assign20910_e20271, (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn2), ((locals.var_fn241_calc_iq__qref0_dn4 * locals.var_fn241_calc_iq__etas0) + (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn4)), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn7), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn11), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etas0_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign20910_e20273;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign20910_e20273_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign20910_e20273_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign20910_e20273_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign20910_e20273_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign20910_e20273_d_n12;

        let assign20920_e20276: f64 = (-50.0);
        let assign20920_e20277: f64 = if locals.var_fn241_calc_iq__etas0 < assign20920_e20276 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign20920_e20277;

        let (assign20930_e20289, assign20930_e20289_d_n2, assign20930_e20289_d_n4, assign20930_e20289_d_n7, assign20930_e20289_d_n11, assign20930_e20289_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign20930_e20286: f64 = (locals.var_fn241_calc_iq__etas0).exp();
        let assign20930_e20287: f64 = (locals.var_fn241_calc_iq__qref0 * assign20930_e20286);
        (assign20930_e20287, (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn2)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20930_e20286) + (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn4))), (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn7)), (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn11)), (locals.var_fn241_calc_iq__qref0 * (assign20930_e20286 * locals.var_fn241_calc_iq__etas0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign20930_e20289;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign20930_e20289_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign20930_e20289_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign20930_e20289_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign20930_e20289_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign20930_e20289_d_n12;

        let (assign20940_e20305, assign20940_e20305_d_n2, assign20940_e20305_d_n4, assign20940_e20305_d_n7, assign20940_e20305_d_n11, assign20940_e20305_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 == 0.0)) {
        let assign20940_e20300: f64 = (locals.var_fn241_calc_iq__etas0).exp();
        let assign20940_e20301: f64 = (1.0 + assign20940_e20300);
        let assign20940_e20302: f64 = (assign20940_e20301).ln();
        let assign20940_e20303: f64 = (locals.var_fn241_calc_iq__qref0 * assign20940_e20302);
        (assign20940_e20303, (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn2) / assign20940_e20301)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign20940_e20302) + (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn4) / assign20940_e20301))), (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn7) / assign20940_e20301)), (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn11) / assign20940_e20301)), (locals.var_fn241_calc_iq__qref0 * ((assign20940_e20300 * locals.var_fn241_calc_iq__etas0_dn12) / assign20940_e20301)),)
    } else {
        (locals.var_fn241_calc_iq__qinvs0, locals.var_fn241_calc_iq__qinvs0_dn2, locals.var_fn241_calc_iq__qinvs0_dn4, locals.var_fn241_calc_iq__qinvs0_dn7, locals.var_fn241_calc_iq__qinvs0_dn11, locals.var_fn241_calc_iq__qinvs0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvs0 = assign20940_e20305;
        locals.var_fn241_calc_iq__qinvs0_dn2 = assign20940_e20305_d_n2;
        locals.var_fn241_calc_iq__qinvs0_dn4 = assign20940_e20305_d_n4;
        locals.var_fn241_calc_iq__qinvs0_dn7 = assign20940_e20305_d_n7;
        locals.var_fn241_calc_iq__qinvs0_dn11 = assign20940_e20305_d_n11;
        locals.var_fn241_calc_iq__qinvs0_dn12 = assign20940_e20305_d_n12;

        let (assign20950_e20313, assign20950_e20313_d_n2, assign20950_e20313_d_n4, assign20950_e20313_d_n7, assign20950_e20313_d_n11, assign20950_e20313_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign20950_e20309: f64 = (locals.var_fn241_calc_iq__vgdin - locals.var_fn241_calc_iq__myarg0);
        let assign20950_e20311: f64 = (assign20950_e20309 / locals.var_fn241_calc_iq__alpha_phit);
        (assign20950_e20311, (locals.var_fn241_calc_iq__vgdin_dn2 / locals.var_fn241_calc_iq__alpha_phit), ((((-locals.var_fn241_calc_iq__myarg0_dn4) * locals.var_fn241_calc_iq__alpha_phit) - (assign20950_e20309 * locals.var_fn241_calc_iq__alpha_phit_dn4)) / (locals.var_fn241_calc_iq__alpha_phit * locals.var_fn241_calc_iq__alpha_phit)), (locals.var_fn241_calc_iq__vgdin_dn7 / locals.var_fn241_calc_iq__alpha_phit), (locals.var_fn241_calc_iq__vgdin_dn11 / locals.var_fn241_calc_iq__alpha_phit), (locals.var_fn241_calc_iq__vgdin_dn12 / locals.var_fn241_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn241_calc_iq__exparg0, locals.var_fn241_calc_iq__exparg0_dn2, locals.var_fn241_calc_iq__exparg0_dn4, locals.var_fn241_calc_iq__exparg0_dn7, locals.var_fn241_calc_iq__exparg0_dn11, locals.var_fn241_calc_iq__exparg0_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg0 = assign20950_e20313;
        locals.var_fn241_calc_iq__exparg0_dn2 = assign20950_e20313_d_n2;
        locals.var_fn241_calc_iq__exparg0_dn4 = assign20950_e20313_d_n4;
        locals.var_fn241_calc_iq__exparg0_dn7 = assign20950_e20313_d_n7;
        locals.var_fn241_calc_iq__exparg0_dn11 = assign20950_e20313_d_n11;
        locals.var_fn241_calc_iq__exparg0_dn12 = assign20950_e20313_d_n12;

        let assign20960_e20316: f64 = if locals.var_fn241_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign20960_e20316;

        let (assign20970_e20322, assign20970_e20322_d_n2, assign20970_e20322_d_n4, assign20970_e20322_d_n7, assign20970_e20322_d_n11, assign20970_e20322_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign20970_e20322;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign20970_e20322_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign20970_e20322_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign20970_e20322_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign20970_e20322_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign20970_e20322_d_n12;

        let assign20980_e20325: f64 = (-50.0);
        let assign20980_e20326: f64 = if locals.var_fn241_calc_iq__exparg0 < assign20980_e20325 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign20980_e20326;

        let (assign20990_e20335, assign20990_e20335_d_n2, assign20990_e20335_d_n4, assign20990_e20335_d_n7, assign20990_e20335_d_n11, assign20990_e20335_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard263 == 0.0)) && (locals.var_guard264 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign20990_e20335;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign20990_e20335_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign20990_e20335_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign20990_e20335_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign20990_e20335_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign20990_e20335_d_n12;

        let (assign21000_e20350, assign21000_e20350_d_n2, assign21000_e20350_d_n4, assign21000_e20350_d_n7, assign21000_e20350_d_n11, assign21000_e20350_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard263 == 0.0)) && (locals.var_guard264 == 0.0)) {
        let assign21000_e20346: f64 = (locals.var_fn241_calc_iq__exparg0).exp();
        let assign21000_e20347: f64 = (1.0 + assign21000_e20346);
        let assign21000_e20348: f64 = (1.0 / assign21000_e20347);
        (assign21000_e20348, (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn2) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn4) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn7) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn11) / (assign21000_e20347 * assign21000_e20347))), (-((assign21000_e20346 * locals.var_fn241_calc_iq__exparg0_dn12) / (assign21000_e20347 * assign21000_e20347))),)
    } else {
        (locals.var_fn241_calc_iq__ffd0, locals.var_fn241_calc_iq__ffd0_dn2, locals.var_fn241_calc_iq__ffd0_dn4, locals.var_fn241_calc_iq__ffd0_dn7, locals.var_fn241_calc_iq__ffd0_dn11, locals.var_fn241_calc_iq__ffd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__ffd0 = assign21000_e20350;
        locals.var_fn241_calc_iq__ffd0_dn2 = assign21000_e20350_d_n2;
        locals.var_fn241_calc_iq__ffd0_dn4 = assign21000_e20350_d_n4;
        locals.var_fn241_calc_iq__ffd0_dn7 = assign21000_e20350_d_n7;
        locals.var_fn241_calc_iq__ffd0_dn11 = assign21000_e20350_d_n11;
        locals.var_fn241_calc_iq__ffd0_dn12 = assign21000_e20350_d_n12;

        let (assign21010_e20368, assign21010_e20368_d_n2, assign21010_e20368_d_n4, assign21010_e20368_d_n7, assign21010_e20368_d_n11, assign21010_e20368_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21010_e20354: f64 = (locals.var_fn241_calc_iq__vgsin - locals.var_fn241_calc_iq__vdx0);
        let assign21010_e20358: f64 = (p.p51 * 0.1);
        let assign21010_e20360: f64 = (assign21010_e20358 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21010_e20362: f64 = (assign21010_e20360 * locals.var_fn241_calc_iq__ffd0);
        let assign21010_e20363: f64 = (locals.var_fn241_calc_iq__vtof - assign21010_e20362);
        let assign21010_e20364: f64 = (assign21010_e20354 - assign21010_e20363);
        let assign21010_e20366: f64 = (assign21010_e20364 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21010_e20366, (((locals.var_fn241_calc_iq__vgsin_dn2 - locals.var_fn241_calc_iq__vdx0_dn2) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn2))) / locals.var_fn241_calc_iq__two_n_phit0), (((((-locals.var_fn241_calc_iq__vdx0_dn4) - (locals.var_fn241_calc_iq__vtof_dn4 - (((assign21010_e20358 * locals.var_fn241_calc_iq__alpha_phit_dn4) * locals.var_fn241_calc_iq__ffd0) + (assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn4)))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21010_e20364 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (((locals.var_fn241_calc_iq__vgsin_dn7 - locals.var_fn241_calc_iq__vdx0_dn7) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn7))) / locals.var_fn241_calc_iq__two_n_phit0), (((-locals.var_fn241_calc_iq__vdx0_dn11) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn11))) / locals.var_fn241_calc_iq__two_n_phit0), (((locals.var_fn241_calc_iq__vgsin_dn12 - locals.var_fn241_calc_iq__vdx0_dn12) - (-(assign21010_e20360 * locals.var_fn241_calc_iq__ffd0_dn12))) / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etad0, locals.var_fn241_calc_iq__etad0_dn2, locals.var_fn241_calc_iq__etad0_dn4, locals.var_fn241_calc_iq__etad0_dn7, locals.var_fn241_calc_iq__etad0_dn11, locals.var_fn241_calc_iq__etad0_dn12,)
    }
};
        locals.var_fn241_calc_iq__etad0 = assign21010_e20368;
        locals.var_fn241_calc_iq__etad0_dn2 = assign21010_e20368_d_n2;
        locals.var_fn241_calc_iq__etad0_dn4 = assign21010_e20368_d_n4;
        locals.var_fn241_calc_iq__etad0_dn7 = assign21010_e20368_d_n7;
        locals.var_fn241_calc_iq__etad0_dn11 = assign21010_e20368_d_n11;
        locals.var_fn241_calc_iq__etad0_dn12 = assign21010_e20368_d_n12;

        let assign21020_e20371: f64 = if locals.var_fn241_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign21020_e20371;

        let (assign21030_e20379, assign21030_e20379_d_n2, assign21030_e20379_d_n4, assign21030_e20379_d_n7, assign21030_e20379_d_n11, assign21030_e20379_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard265 != 0.0)) {
        let assign21030_e20377: f64 = (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0);
        (assign21030_e20377, (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn2), ((locals.var_fn241_calc_iq__qref0_dn4 * locals.var_fn241_calc_iq__etad0) + (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn4)), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn7), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn11), (locals.var_fn241_calc_iq__qref0 * locals.var_fn241_calc_iq__etad0_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign21030_e20379;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign21030_e20379_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign21030_e20379_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign21030_e20379_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign21030_e20379_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign21030_e20379_d_n12;

        let assign21040_e20382: f64 = (-50.0);
        let assign21040_e20383: f64 = if locals.var_fn241_calc_iq__etad0 < assign21040_e20382 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign21040_e20383;

        let (assign21050_e20395, assign21050_e20395_d_n2, assign21050_e20395_d_n4, assign21050_e20395_d_n7, assign21050_e20395_d_n11, assign21050_e20395_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign21050_e20392: f64 = (locals.var_fn241_calc_iq__etad0).exp();
        let assign21050_e20393: f64 = (locals.var_fn241_calc_iq__qref0 * assign21050_e20392);
        (assign21050_e20393, (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn2)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign21050_e20392) + (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn4))), (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn7)), (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn11)), (locals.var_fn241_calc_iq__qref0 * (assign21050_e20392 * locals.var_fn241_calc_iq__etad0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign21050_e20395;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign21050_e20395_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign21050_e20395_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign21050_e20395_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign21050_e20395_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign21050_e20395_d_n12;

        let (assign21060_e20411, assign21060_e20411_d_n2, assign21060_e20411_d_n4, assign21060_e20411_d_n7, assign21060_e20411_d_n11, assign21060_e20411_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign21060_e20406: f64 = (locals.var_fn241_calc_iq__etad0).exp();
        let assign21060_e20407: f64 = (1.0 + assign21060_e20406);
        let assign21060_e20408: f64 = (assign21060_e20407).ln();
        let assign21060_e20409: f64 = (locals.var_fn241_calc_iq__qref0 * assign21060_e20408);
        (assign21060_e20409, (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn2) / assign21060_e20407)), ((locals.var_fn241_calc_iq__qref0_dn4 * assign21060_e20408) + (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn4) / assign21060_e20407))), (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn7) / assign21060_e20407)), (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn11) / assign21060_e20407)), (locals.var_fn241_calc_iq__qref0 * ((assign21060_e20406 * locals.var_fn241_calc_iq__etad0_dn12) / assign21060_e20407)),)
    } else {
        (locals.var_fn241_calc_iq__qinvd0, locals.var_fn241_calc_iq__qinvd0_dn2, locals.var_fn241_calc_iq__qinvd0_dn4, locals.var_fn241_calc_iq__qinvd0_dn7, locals.var_fn241_calc_iq__qinvd0_dn11, locals.var_fn241_calc_iq__qinvd0_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvd0 = assign21060_e20411;
        locals.var_fn241_calc_iq__qinvd0_dn2 = assign21060_e20411_d_n2;
        locals.var_fn241_calc_iq__qinvd0_dn4 = assign21060_e20411_d_n4;
        locals.var_fn241_calc_iq__qinvd0_dn7 = assign21060_e20411_d_n7;
        locals.var_fn241_calc_iq__qinvd0_dn11 = assign21060_e20411_d_n11;
        locals.var_fn241_calc_iq__qinvd0_dn12 = assign21060_e20411_d_n12;

        let (assign21070_e20419, assign21070_e20419_d_n2, assign21070_e20419_d_n4, assign21070_e20419_d_n7, assign21070_e20419_d_n11, assign21070_e20419_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21070_e20415: f64 = (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0);
        let assign21070_e20417: f64 = (assign21070_e20415 + 1e-38);
        (assign21070_e20417, ((locals.var_fn241_calc_iq__qinvs0_dn2 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn2)), ((locals.var_fn241_calc_iq__qinvs0_dn4 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn4)), ((locals.var_fn241_calc_iq__qinvs0_dn7 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn7)), ((locals.var_fn241_calc_iq__qinvs0_dn11 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn11)), ((locals.var_fn241_calc_iq__qinvs0_dn12 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvs0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qs2, locals.var_fn241_calc_iq__qs2_dn2, locals.var_fn241_calc_iq__qs2_dn4, locals.var_fn241_calc_iq__qs2_dn7, locals.var_fn241_calc_iq__qs2_dn11, locals.var_fn241_calc_iq__qs2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs2 = assign21070_e20419;
        locals.var_fn241_calc_iq__qs2_dn2 = assign21070_e20419_d_n2;
        locals.var_fn241_calc_iq__qs2_dn4 = assign21070_e20419_d_n4;
        locals.var_fn241_calc_iq__qs2_dn7 = assign21070_e20419_d_n7;
        locals.var_fn241_calc_iq__qs2_dn11 = assign21070_e20419_d_n11;
        locals.var_fn241_calc_iq__qs2_dn12 = assign21070_e20419_d_n12;

        let (assign21080_e20427, assign21080_e20427_d_n2, assign21080_e20427_d_n4, assign21080_e20427_d_n7, assign21080_e20427_d_n11, assign21080_e20427_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21080_e20423: f64 = (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0);
        let assign21080_e20425: f64 = (assign21080_e20423 + 1e-57);
        (assign21080_e20425, ((locals.var_fn241_calc_iq__qs2_dn2 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn2)), ((locals.var_fn241_calc_iq__qs2_dn4 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn4)), ((locals.var_fn241_calc_iq__qs2_dn7 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn7)), ((locals.var_fn241_calc_iq__qs2_dn11 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn11)), ((locals.var_fn241_calc_iq__qs2_dn12 * locals.var_fn241_calc_iq__qinvs0) + (locals.var_fn241_calc_iq__qs2 * locals.var_fn241_calc_iq__qinvs0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qs3, locals.var_fn241_calc_iq__qs3_dn2, locals.var_fn241_calc_iq__qs3_dn4, locals.var_fn241_calc_iq__qs3_dn7, locals.var_fn241_calc_iq__qs3_dn11, locals.var_fn241_calc_iq__qs3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs3 = assign21080_e20427;
        locals.var_fn241_calc_iq__qs3_dn2 = assign21080_e20427_d_n2;
        locals.var_fn241_calc_iq__qs3_dn4 = assign21080_e20427_d_n4;
        locals.var_fn241_calc_iq__qs3_dn7 = assign21080_e20427_d_n7;
        locals.var_fn241_calc_iq__qs3_dn11 = assign21080_e20427_d_n11;
        locals.var_fn241_calc_iq__qs3_dn12 = assign21080_e20427_d_n12;

        let (assign21090_e20435, assign21090_e20435_d_n2, assign21090_e20435_d_n4, assign21090_e20435_d_n7, assign21090_e20435_d_n11, assign21090_e20435_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21090_e20431: f64 = (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0);
        let assign21090_e20433: f64 = (assign21090_e20431 + 1e-38);
        (assign21090_e20433, ((locals.var_fn241_calc_iq__qinvd0_dn2 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn2)), ((locals.var_fn241_calc_iq__qinvd0_dn4 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn4)), ((locals.var_fn241_calc_iq__qinvd0_dn7 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn7)), ((locals.var_fn241_calc_iq__qinvd0_dn11 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn11)), ((locals.var_fn241_calc_iq__qinvd0_dn12 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvd0 * locals.var_fn241_calc_iq__qinvd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qd2, locals.var_fn241_calc_iq__qd2_dn2, locals.var_fn241_calc_iq__qd2_dn4, locals.var_fn241_calc_iq__qd2_dn7, locals.var_fn241_calc_iq__qd2_dn11, locals.var_fn241_calc_iq__qd2_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd2 = assign21090_e20435;
        locals.var_fn241_calc_iq__qd2_dn2 = assign21090_e20435_d_n2;
        locals.var_fn241_calc_iq__qd2_dn4 = assign21090_e20435_d_n4;
        locals.var_fn241_calc_iq__qd2_dn7 = assign21090_e20435_d_n7;
        locals.var_fn241_calc_iq__qd2_dn11 = assign21090_e20435_d_n11;
        locals.var_fn241_calc_iq__qd2_dn12 = assign21090_e20435_d_n12;

        let (assign21100_e20443, assign21100_e20443_d_n2, assign21100_e20443_d_n4, assign21100_e20443_d_n7, assign21100_e20443_d_n11, assign21100_e20443_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21100_e20439: f64 = (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0);
        let assign21100_e20441: f64 = (assign21100_e20439 + 1e-57);
        (assign21100_e20441, ((locals.var_fn241_calc_iq__qd2_dn2 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn2)), ((locals.var_fn241_calc_iq__qd2_dn4 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn4)), ((locals.var_fn241_calc_iq__qd2_dn7 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn7)), ((locals.var_fn241_calc_iq__qd2_dn11 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn11)), ((locals.var_fn241_calc_iq__qd2_dn12 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qd2 * locals.var_fn241_calc_iq__qinvd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qd3, locals.var_fn241_calc_iq__qd3_dn2, locals.var_fn241_calc_iq__qd3_dn4, locals.var_fn241_calc_iq__qd3_dn7, locals.var_fn241_calc_iq__qd3_dn11, locals.var_fn241_calc_iq__qd3_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd3 = assign21100_e20443;
        locals.var_fn241_calc_iq__qd3_dn2 = assign21100_e20443_d_n2;
        locals.var_fn241_calc_iq__qd3_dn4 = assign21100_e20443_d_n4;
        locals.var_fn241_calc_iq__qd3_dn7 = assign21100_e20443_d_n7;
        locals.var_fn241_calc_iq__qd3_dn11 = assign21100_e20443_d_n11;
        locals.var_fn241_calc_iq__qd3_dn12 = assign21100_e20443_d_n12;

        let (assign21110_e20451, assign21110_e20451_d_n2, assign21110_e20451_d_n4, assign21110_e20451_d_n7, assign21110_e20451_d_n11, assign21110_e20451_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21110_e20447: f64 = (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0);
        let assign21110_e20449: f64 = (assign21110_e20447 + 1e-38);
        (assign21110_e20449, ((locals.var_fn241_calc_iq__qinvs0_dn2 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn2)), ((locals.var_fn241_calc_iq__qinvs0_dn4 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn4)), ((locals.var_fn241_calc_iq__qinvs0_dn7 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn7)), ((locals.var_fn241_calc_iq__qinvs0_dn11 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn11)), ((locals.var_fn241_calc_iq__qinvs0_dn12 * locals.var_fn241_calc_iq__qinvd0) + (locals.var_fn241_calc_iq__qinvs0 * locals.var_fn241_calc_iq__qinvd0_dn12)),)
    } else {
        (locals.var_fn241_calc_iq__qsqd, locals.var_fn241_calc_iq__qsqd_dn2, locals.var_fn241_calc_iq__qsqd_dn4, locals.var_fn241_calc_iq__qsqd_dn7, locals.var_fn241_calc_iq__qsqd_dn11, locals.var_fn241_calc_iq__qsqd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsqd = assign21110_e20451;
        locals.var_fn241_calc_iq__qsqd_dn2 = assign21110_e20451_d_n2;
        locals.var_fn241_calc_iq__qsqd_dn4 = assign21110_e20451_d_n4;
        locals.var_fn241_calc_iq__qsqd_dn7 = assign21110_e20451_d_n7;
        locals.var_fn241_calc_iq__qsqd_dn11 = assign21110_e20451_d_n11;
        locals.var_fn241_calc_iq__qsqd_dn12 = assign21110_e20451_d_n12;

        let (assign21120_e20469, assign21120_e20469_d_n2, assign21120_e20469_d_n4, assign21120_e20469_d_n7, assign21120_e20469_d_n11, assign21120_e20469_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21120_e20455: f64 = (2.0 / 3.0);
        let assign21120_e20458: f64 = (locals.var_fn241_calc_iq__qs2 + locals.var_fn241_calc_iq__qd2);
        let assign21120_e20460: f64 = (assign21120_e20458 + locals.var_fn241_calc_iq__qsqd);
        let assign21120_e20461: f64 = (assign21120_e20455 * assign21120_e20460);
        let assign21120_e20464: f64 = (locals.var_fn241_calc_iq__qinvs0 + locals.var_fn241_calc_iq__qinvd0);
        let assign21120_e20466: f64 = (assign21120_e20464 + 2e-19);
        let assign21120_e20467: f64 = (assign21120_e20461 / assign21120_e20466);
        (assign21120_e20467, ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn2 + locals.var_fn241_calc_iq__qd2_dn2) + locals.var_fn241_calc_iq__qsqd_dn2)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn2 + locals.var_fn241_calc_iq__qinvd0_dn2))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn4 + locals.var_fn241_calc_iq__qd2_dn4) + locals.var_fn241_calc_iq__qsqd_dn4)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn4 + locals.var_fn241_calc_iq__qinvd0_dn4))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn7 + locals.var_fn241_calc_iq__qd2_dn7) + locals.var_fn241_calc_iq__qsqd_dn7)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn7 + locals.var_fn241_calc_iq__qinvd0_dn7))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn11 + locals.var_fn241_calc_iq__qd2_dn11) + locals.var_fn241_calc_iq__qsqd_dn11)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn11 + locals.var_fn241_calc_iq__qinvd0_dn11))) / (assign21120_e20466 * assign21120_e20466)), ((((assign21120_e20455 * ((locals.var_fn241_calc_iq__qs2_dn12 + locals.var_fn241_calc_iq__qd2_dn12) + locals.var_fn241_calc_iq__qsqd_dn12)) * assign21120_e20466) - (assign21120_e20461 * (locals.var_fn241_calc_iq__qinvs0_dn12 + locals.var_fn241_calc_iq__qinvd0_dn12))) / (assign21120_e20466 * assign21120_e20466)),)
    } else {
        (locals.var_fn241_calc_iq__qinvdd, locals.var_fn241_calc_iq__qinvdd_dn2, locals.var_fn241_calc_iq__qinvdd_dn4, locals.var_fn241_calc_iq__qinvdd_dn7, locals.var_fn241_calc_iq__qinvdd_dn11, locals.var_fn241_calc_iq__qinvdd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qinvdd = assign21120_e20469;
        locals.var_fn241_calc_iq__qinvdd_dn2 = assign21120_e20469_d_n2;
        locals.var_fn241_calc_iq__qinvdd_dn4 = assign21120_e20469_d_n4;
        locals.var_fn241_calc_iq__qinvdd_dn7 = assign21120_e20469_d_n7;
        locals.var_fn241_calc_iq__qinvdd_dn11 = assign21120_e20469_d_n11;
        locals.var_fn241_calc_iq__qinvdd_dn12 = assign21120_e20469_d_n12;

        let (assign21130_e20503, assign21130_e20503_d_n2, assign21130_e20503_d_n4, assign21130_e20503_d_n7, assign21130_e20503_d_n11, assign21130_e20503_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21130_e20474: f64 = (2.0 * locals.var_fn241_calc_iq__qs3);
        let assign21130_e20477: f64 = (3.0 * locals.var_fn241_calc_iq__qd3);
        let assign21130_e20478: f64 = (assign21130_e20474 + assign21130_e20477);
        let assign21130_e20481: f64 = (4.0 * locals.var_fn241_calc_iq__qs2);
        let assign21130_e20483: f64 = (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0);
        let assign21130_e20484: f64 = (assign21130_e20478 + assign21130_e20483);
        let assign21130_e20487: f64 = (6.0 * locals.var_fn241_calc_iq__qd2);
        let assign21130_e20489: f64 = (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0);
        let assign21130_e20490: f64 = (assign21130_e20484 + assign21130_e20489);
        let assign21130_e20491: f64 = (2.0 * assign21130_e20490);
        let assign21130_e20495: f64 = (locals.var_fn241_calc_iq__qs2 + locals.var_fn241_calc_iq__qd2);
        let assign21130_e20498: f64 = (2.0 * locals.var_fn241_calc_iq__qsqd);
        let assign21130_e20499: f64 = (assign21130_e20495 + assign21130_e20498);
        let assign21130_e20500: f64 = (15.0 * assign21130_e20499);
        let assign21130_e20501: f64 = (assign21130_e20491 / assign21130_e20500);
        (assign21130_e20501, ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn2) + (3.0 * locals.var_fn241_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn2) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn2) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn2)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn2 + locals.var_fn241_calc_iq__qd2_dn2) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn2))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn4) + (3.0 * locals.var_fn241_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn4) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn4) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn4)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn4 + locals.var_fn241_calc_iq__qd2_dn4) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn4))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn7) + (3.0 * locals.var_fn241_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn7) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn7) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn7)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn7 + locals.var_fn241_calc_iq__qd2_dn7) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn7))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn11) + (3.0 * locals.var_fn241_calc_iq__qd3_dn11)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn11) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn11))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn11) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn11)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn11 + locals.var_fn241_calc_iq__qd2_dn11) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn11))))) / (assign21130_e20500 * assign21130_e20500)), ((((2.0 * ((((2.0 * locals.var_fn241_calc_iq__qs3_dn12) + (3.0 * locals.var_fn241_calc_iq__qd3_dn12)) + (((4.0 * locals.var_fn241_calc_iq__qs2_dn12) * locals.var_fn241_calc_iq__qinvd0) + (assign21130_e20481 * locals.var_fn241_calc_iq__qinvd0_dn12))) + (((6.0 * locals.var_fn241_calc_iq__qd2_dn12) * locals.var_fn241_calc_iq__qinvs0) + (assign21130_e20487 * locals.var_fn241_calc_iq__qinvs0_dn12)))) * assign21130_e20500) - (assign21130_e20491 * (15.0 * ((locals.var_fn241_calc_iq__qs2_dn12 + locals.var_fn241_calc_iq__qd2_dn12) + (2.0 * locals.var_fn241_calc_iq__qsqd_dn12))))) / (assign21130_e20500 * assign21130_e20500)),)
    } else {
        (locals.var_fn241_calc_iq__qd1, locals.var_fn241_calc_iq__qd1_dn2, locals.var_fn241_calc_iq__qd1_dn4, locals.var_fn241_calc_iq__qd1_dn7, locals.var_fn241_calc_iq__qd1_dn11, locals.var_fn241_calc_iq__qd1_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd1 = assign21130_e20503;
        locals.var_fn241_calc_iq__qd1_dn2 = assign21130_e20503_d_n2;
        locals.var_fn241_calc_iq__qd1_dn4 = assign21130_e20503_d_n4;
        locals.var_fn241_calc_iq__qd1_dn7 = assign21130_e20503_d_n7;
        locals.var_fn241_calc_iq__qd1_dn11 = assign21130_e20503_d_n11;
        locals.var_fn241_calc_iq__qd1_dn12 = assign21130_e20503_d_n12;

        let (assign21140_e20509, assign21140_e20509_d_n2, assign21140_e20509_d_n4, assign21140_e20509_d_n7, assign21140_e20509_d_n11, assign21140_e20509_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21140_e20507: f64 = (locals.var_fn241_calc_iq__qinvdd - locals.var_fn241_calc_iq__qd1);
        (assign21140_e20507, (locals.var_fn241_calc_iq__qinvdd_dn2 - locals.var_fn241_calc_iq__qd1_dn2), (locals.var_fn241_calc_iq__qinvdd_dn4 - locals.var_fn241_calc_iq__qd1_dn4), (locals.var_fn241_calc_iq__qinvdd_dn7 - locals.var_fn241_calc_iq__qd1_dn7), (locals.var_fn241_calc_iq__qinvdd_dn11 - locals.var_fn241_calc_iq__qd1_dn11), (locals.var_fn241_calc_iq__qinvdd_dn12 - locals.var_fn241_calc_iq__qd1_dn12),)
    } else {
        (locals.var_fn241_calc_iq__qs, locals.var_fn241_calc_iq__qs_dn2, locals.var_fn241_calc_iq__qs_dn4, locals.var_fn241_calc_iq__qs_dn7, locals.var_fn241_calc_iq__qs_dn11, locals.var_fn241_calc_iq__qs_dn12,)
    }
};
        locals.var_fn241_calc_iq__qs = assign21140_e20509;
        locals.var_fn241_calc_iq__qs_dn2 = assign21140_e20509_d_n2;
        locals.var_fn241_calc_iq__qs_dn4 = assign21140_e20509_d_n4;
        locals.var_fn241_calc_iq__qs_dn7 = assign21140_e20509_d_n7;
        locals.var_fn241_calc_iq__qs_dn11 = assign21140_e20509_d_n11;
        locals.var_fn241_calc_iq__qs_dn12 = assign21140_e20509_d_n12;

        let (assign21150_e20513, assign21150_e20513_d_n2, assign21150_e20513_d_n4, assign21150_e20513_d_n7, assign21150_e20513_d_n11, assign21150_e20513_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qd1, locals.var_fn241_calc_iq__qd1_dn2, locals.var_fn241_calc_iq__qd1_dn4, locals.var_fn241_calc_iq__qd1_dn7, locals.var_fn241_calc_iq__qd1_dn11, locals.var_fn241_calc_iq__qd1_dn12,)
    } else {
        (locals.var_fn241_calc_iq__qd, locals.var_fn241_calc_iq__qd_dn2, locals.var_fn241_calc_iq__qd_dn4, locals.var_fn241_calc_iq__qd_dn7, locals.var_fn241_calc_iq__qd_dn11, locals.var_fn241_calc_iq__qd_dn12,)
    }
};
        locals.var_fn241_calc_iq__qd = assign21150_e20513;
        locals.var_fn241_calc_iq__qd_dn2 = assign21150_e20513_d_n2;
        locals.var_fn241_calc_iq__qd_dn4 = assign21150_e20513_d_n4;
        locals.var_fn241_calc_iq__qd_dn7 = assign21150_e20513_d_n7;
        locals.var_fn241_calc_iq__qd_dn11 = assign21150_e20513_d_n11;
        locals.var_fn241_calc_iq__qd_dn12 = assign21150_e20513_d_n12;

        let (assign21160_e20527, assign21160_e20527_d_n2, assign21160_e20527_d_n4, assign21160_e20527_d_n7, assign21160_e20527_d_n11, assign21160_e20527_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21160_e20517: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21160_e20519: f64 = (assign21160_e20517 * locals.var_fn241_calc_iq__lin);
        let assign21160_e20521: f64 = (assign21160_e20519 * locals.var_fn241_calc_iq__type);
        let assign21160_e20523: f64 = (assign21160_e20521 * locals.var_fn241_calc_iq__qs);
        let assign21160_e20525: f64 = (assign21160_e20523 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21160_e20525, ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn4) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21160_e20521 * locals.var_fn241_calc_iq__qs_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgsout = assign21160_e20527;
        locals.var_fn241_calc_iq__qgsout_dn2 = assign21160_e20527_d_n2;
        locals.var_fn241_calc_iq__qgsout_dn4 = assign21160_e20527_d_n4;
        locals.var_fn241_calc_iq__qgsout_dn7 = assign21160_e20527_d_n7;
        locals.var_fn241_calc_iq__qgsout_dn11 = assign21160_e20527_d_n11;
        locals.var_fn241_calc_iq__qgsout_dn12 = assign21160_e20527_d_n12;

        let (assign21170_e20541, assign21170_e20541_d_n2, assign21170_e20541_d_n4, assign21170_e20541_d_n7, assign21170_e20541_d_n11, assign21170_e20541_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        let assign21170_e20531: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21170_e20533: f64 = (assign21170_e20531 * locals.var_fn241_calc_iq__lin);
        let assign21170_e20535: f64 = (assign21170_e20533 * locals.var_fn241_calc_iq__type);
        let assign21170_e20537: f64 = (assign21170_e20535 * locals.var_fn241_calc_iq__qd);
        let assign21170_e20539: f64 = (assign21170_e20537 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21170_e20539, ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn4) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21170_e20535 * locals.var_fn241_calc_iq__qd_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qgdout = assign21170_e20541;
        locals.var_fn241_calc_iq__qgdout_dn2 = assign21170_e20541_d_n2;
        locals.var_fn241_calc_iq__qgdout_dn4 = assign21170_e20541_d_n4;
        locals.var_fn241_calc_iq__qgdout_dn7 = assign21170_e20541_d_n7;
        locals.var_fn241_calc_iq__qgdout_dn11 = assign21170_e20541_d_n11;
        locals.var_fn241_calc_iq__qgdout_dn12 = assign21170_e20541_d_n12;

        let assign21180_e20544: f64 = if locals.var_fn241_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign21180_e20544;

        let (assign21190_e20560, assign21190_e20560_d_n2, assign21190_e20560_d_n4, assign21190_e20560_d_n7, assign21190_e20560_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21190_e20552: f64 = (p.p51 * 0.5);
        let assign21190_e20554: f64 = (assign21190_e20552 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21190_e20555: f64 = (locals.var_fn241_calc_iq__vtof - assign21190_e20554);
        let assign21190_e20556: f64 = (locals.var_fn241_calc_iq__vcin - assign21190_e20555);
        let assign21190_e20558: f64 = (assign21190_e20556 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21190_e20558, (locals.var_fn241_calc_iq__vcin_dn2 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21190_e20552 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21190_e20556 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vcin_dn7 / locals.var_fn241_calc_iq__two_n_phit0), (locals.var_fn241_calc_iq__vcin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etac, locals.var_fn241_calc_iq__etac_dn2, locals.var_fn241_calc_iq__etac_dn4, locals.var_fn241_calc_iq__etac_dn7, locals.var_fn241_calc_iq__etac_dn12,)
    }
};
        locals.var_fn241_calc_iq__etac = assign21190_e20560;
        locals.var_fn241_calc_iq__etac_dn2 = assign21190_e20560_d_n2;
        locals.var_fn241_calc_iq__etac_dn4 = assign21190_e20560_d_n4;
        locals.var_fn241_calc_iq__etac_dn7 = assign21190_e20560_d_n7;
        locals.var_fn241_calc_iq__etac_dn12 = assign21190_e20560_d_n12;

        let assign21200_e20563: f64 = if locals.var_fn241_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign21200_e20563;

        let (assign21210_e20571, assign21210_e20571_d_n2, assign21210_e20571_d_n3, assign21210_e20571_d_n4, assign21210_e20571_d_n7, assign21210_e20571_d_n11, assign21210_e20571_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 != 0.0)) {
        (locals.var_fn241_calc_iq__etac, locals.var_fn241_calc_iq__etac_dn2, 0.0, locals.var_fn241_calc_iq__etac_dn4, locals.var_fn241_calc_iq__etac_dn7, 0.0, locals.var_fn241_calc_iq__etac_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21210_e20571;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21210_e20571_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21210_e20571_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21210_e20571_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21210_e20571_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21210_e20571_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21210_e20571_d_n12;

        let assign21220_e20574: f64 = (-50.0);
        let assign21220_e20575: f64 = if locals.var_fn241_calc_iq__etac < assign21220_e20574 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign21220_e20575;

        let (assign21230_e20587, assign21230_e20587_d_n2, assign21230_e20587_d_n3, assign21230_e20587_d_n4, assign21230_e20587_d_n7, assign21230_e20587_d_n11, assign21230_e20587_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 == 0.0)) && (locals.var_guard269 != 0.0)) {
        let assign21230_e20585: f64 = (locals.var_fn241_calc_iq__etac).exp();
        (assign21230_e20585, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn2), 0.0, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn4), (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn7), 0.0, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21230_e20587;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21230_e20587_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21230_e20587_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21230_e20587_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21230_e20587_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21230_e20587_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21230_e20587_d_n12;

        let (assign21240_e20603, assign21240_e20603_d_n2, assign21240_e20603_d_n3, assign21240_e20603_d_n4, assign21240_e20603_d_n7, assign21240_e20603_d_n11, assign21240_e20603_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 == 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign21240_e20599: f64 = (locals.var_fn241_calc_iq__etac).exp();
        let assign21240_e20600: f64 = (1.0 + assign21240_e20599);
        let assign21240_e20601: f64 = (assign21240_e20600).ln();
        (assign21240_e20601, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn2) / assign21240_e20600), 0.0, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn4) / assign21240_e20600), ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn7) / assign21240_e20600), 0.0, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn12) / assign21240_e20600),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21240_e20603;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21240_e20603_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21240_e20603_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21240_e20603_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21240_e20603_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21240_e20603_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21240_e20603_d_n12;

    }

    pub(super) fn stamp_transient_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21250_e20621, assign21250_e20621_d_n2, assign21250_e20621_d_n3, assign21250_e20621_d_n4, assign21250_e20621_d_n7, assign21250_e20621_d_n11, assign21250_e20621_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21250_e20609: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21250_e20611: f64 = (assign21250_e20609 * locals.var_fn241_calc_iq__type);
        let assign21250_e20613: f64 = (assign21250_e20611 * locals.var_fn241_calc_iq__cc);
        let assign21250_e20615: f64 = (assign21250_e20613 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21250_e20617: f64 = (assign21250_e20615 * locals.var_fn241_calc_iq__exparg);
        let assign21250_e20619: f64 = (assign21250_e20617 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21250_e20619, ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((((assign21250_e20611 * locals.var_fn241_calc_iq__cc_dn4) * locals.var_fn241_calc_iq__two_n_phit0) + (assign21250_e20613 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) * locals.var_fn241_calc_iq__exparg) + (assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign21250_e20621;
        locals.var_fn241_calc_iq__qcout_dn2 = assign21250_e20621_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign21250_e20621_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign21250_e20621_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign21250_e20621_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign21250_e20621_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign21250_e20621_d_n12;

        let (assign21260_e20637, assign21260_e20637_d_n3, assign21260_e20637_d_n4, assign21260_e20637_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21260_e20629: f64 = (p.p51 * 0.5);
        let assign21260_e20631: f64 = (assign21260_e20629 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21260_e20632: f64 = (locals.var_fn241_calc_iq__vtof - assign21260_e20631);
        let assign21260_e20633: f64 = (locals.var_fn241_calc_iq__vbin - assign21260_e20632);
        let assign21260_e20635: f64 = (assign21260_e20633 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21260_e20635, (locals.var_fn241_calc_iq__vbin_dn3 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21260_e20629 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21260_e20633 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vbin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etab, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, locals.var_fn241_calc_iq__etab_dn12,)
    }
};
        locals.var_fn241_calc_iq__etab = assign21260_e20637;
        locals.var_fn241_calc_iq__etab_dn3 = assign21260_e20637_d_n3;
        locals.var_fn241_calc_iq__etab_dn4 = assign21260_e20637_d_n4;
        locals.var_fn241_calc_iq__etab_dn12 = assign21260_e20637_d_n12;

        let assign21270_e20640: f64 = if locals.var_fn241_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign21270_e20640;

        let (assign21280_e20648, assign21280_e20648_d_n2, assign21280_e20648_d_n3, assign21280_e20648_d_n4, assign21280_e20648_d_n7, assign21280_e20648_d_n11, assign21280_e20648_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 != 0.0)) {
        (locals.var_fn241_calc_iq__etab, 0.0, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn241_calc_iq__etab_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21280_e20648;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21280_e20648_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21280_e20648_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21280_e20648_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21280_e20648_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21280_e20648_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21280_e20648_d_n12;

        let assign21290_e20651: f64 = (-50.0);
        let assign21290_e20652: f64 = if locals.var_fn241_calc_iq__etab < assign21290_e20651 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign21290_e20652;

        let (assign21300_e20664, assign21300_e20664_d_n2, assign21300_e20664_d_n3, assign21300_e20664_d_n4, assign21300_e20664_d_n7, assign21300_e20664_d_n11, assign21300_e20664_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 != 0.0)) {
        let assign21300_e20662: f64 = (locals.var_fn241_calc_iq__etab).exp();
        (assign21300_e20662, 0.0, (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn3), (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn4), 0.0, 0.0, (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21300_e20664;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21300_e20664_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21300_e20664_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21300_e20664_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21300_e20664_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21300_e20664_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21300_e20664_d_n12;

        let (assign21310_e20680, assign21310_e20680_d_n2, assign21310_e20680_d_n3, assign21310_e20680_d_n4, assign21310_e20680_d_n7, assign21310_e20680_d_n11, assign21310_e20680_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
        let assign21310_e20676: f64 = (locals.var_fn241_calc_iq__etab).exp();
        let assign21310_e20677: f64 = (1.0 + assign21310_e20676);
        let assign21310_e20678: f64 = (assign21310_e20677).ln();
        (assign21310_e20678, 0.0, ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn3) / assign21310_e20677), ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn4) / assign21310_e20677), 0.0, 0.0, ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn12) / assign21310_e20677),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21310_e20680;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21310_e20680_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21310_e20680_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21310_e20680_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21310_e20680_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21310_e20680_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21310_e20680_d_n12;

        let (assign21320_e20698, assign21320_e20698_d_n2, assign21320_e20698_d_n3, assign21320_e20698_d_n4, assign21320_e20698_d_n7, assign21320_e20698_d_n11, assign21320_e20698_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21320_e20686: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21320_e20688: f64 = (assign21320_e20686 * locals.var_fn241_calc_iq__type);
        let assign21320_e20690: f64 = (assign21320_e20688 * locals.var_fn241_calc_iq__cb);
        let assign21320_e20692: f64 = (assign21320_e20690 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21320_e20694: f64 = (assign21320_e20692 * locals.var_fn241_calc_iq__exparg);
        let assign21320_e20696: f64 = (assign21320_e20694 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21320_e20696, ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((((assign21320_e20688 * locals.var_fn241_calc_iq__cb_dn4) * locals.var_fn241_calc_iq__two_n_phit0) + (assign21320_e20690 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) * locals.var_fn241_calc_iq__exparg) + (assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign21320_e20698;
        locals.var_fn241_calc_iq__qbout_dn2 = assign21320_e20698_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign21320_e20698_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign21320_e20698_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign21320_e20698_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign21320_e20698_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign21320_e20698_d_n12;

        let (assign21330_e20705, assign21330_e20705_d_n2, assign21330_e20705_d_n3, assign21330_e20705_d_n4, assign21330_e20705_d_n7, assign21330_e20705_d_n11, assign21330_e20705_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign21330_e20705;
        locals.var_fn241_calc_iq__qcout_dn2 = assign21330_e20705_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign21330_e20705_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign21330_e20705_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign21330_e20705_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign21330_e20705_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign21330_e20705_d_n12;

        let (assign21340_e20712, assign21340_e20712_d_n2, assign21340_e20712_d_n3, assign21340_e20712_d_n4, assign21340_e20712_d_n7, assign21340_e20712_d_n11, assign21340_e20712_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign21340_e20712;
        locals.var_fn241_calc_iq__qbout_dn2 = assign21340_e20712_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign21340_e20712_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign21340_e20712_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign21340_e20712_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign21340_e20712_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign21340_e20712_d_n12;

        let assign21350_e20715: f64 = if locals.var_fn241_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign21350_e20715;

        let (assign21360_e20731, assign21360_e20731_d_n2, assign21360_e20731_d_n4, assign21360_e20731_d_n7, assign21360_e20731_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) {
        let assign21360_e20723: f64 = (p.p51 * 0.5);
        let assign21360_e20725: f64 = (assign21360_e20723 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21360_e20726: f64 = (locals.var_fn241_calc_iq__vtof - assign21360_e20725);
        let assign21360_e20727: f64 = (locals.var_fn241_calc_iq__vgsin - assign21360_e20726);
        let assign21360_e20729: f64 = (assign21360_e20727 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21360_e20729, (locals.var_fn241_calc_iq__vgsin_dn2 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21360_e20723 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21360_e20727 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vgsin_dn7 / locals.var_fn241_calc_iq__two_n_phit0), (locals.var_fn241_calc_iq__vgsin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, locals.var_fn241_calc_iq__etags_dn12,)
    }
};
        locals.var_fn241_calc_iq__etags = assign21360_e20731;
        locals.var_fn241_calc_iq__etags_dn2 = assign21360_e20731_d_n2;
        locals.var_fn241_calc_iq__etags_dn4 = assign21360_e20731_d_n4;
        locals.var_fn241_calc_iq__etags_dn7 = assign21360_e20731_d_n7;
        locals.var_fn241_calc_iq__etags_dn12 = assign21360_e20731_d_n12;

        let assign21370_e20734: f64 = if locals.var_fn241_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign21370_e20734;

        let (assign21380_e20742, assign21380_e20742_d_n2, assign21380_e20742_d_n3, assign21380_e20742_d_n4, assign21380_e20742_d_n7, assign21380_e20742_d_n11, assign21380_e20742_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 != 0.0)) {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, 0.0, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, 0.0, locals.var_fn241_calc_iq__etags_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21380_e20742;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21380_e20742_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21380_e20742_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21380_e20742_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21380_e20742_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21380_e20742_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21380_e20742_d_n12;

        let assign21390_e20745: f64 = (-50.0);
        let assign21390_e20746: f64 = if locals.var_fn241_calc_iq__etags < assign21390_e20745 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign21390_e20746;

        let (assign21400_e20758, assign21400_e20758_d_n2, assign21400_e20758_d_n3, assign21400_e20758_d_n4, assign21400_e20758_d_n7, assign21400_e20758_d_n11, assign21400_e20758_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 != 0.0)) {
        let assign21400_e20756: f64 = (locals.var_fn241_calc_iq__etags).exp();
        (assign21400_e20756, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn2), 0.0, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn4), (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn7), 0.0, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21400_e20758;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21400_e20758_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21400_e20758_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21400_e20758_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21400_e20758_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21400_e20758_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21400_e20758_d_n12;

        let (assign21410_e20774, assign21410_e20774_d_n2, assign21410_e20774_d_n3, assign21410_e20774_d_n4, assign21410_e20774_d_n7, assign21410_e20774_d_n11, assign21410_e20774_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 == 0.0)) {
        let assign21410_e20770: f64 = (locals.var_fn241_calc_iq__etags).exp();
        let assign21410_e20771: f64 = (1.0 + assign21410_e20770);
        let assign21410_e20772: f64 = (assign21410_e20771).ln();
        (assign21410_e20772, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn2) / assign21410_e20771), 0.0, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn4) / assign21410_e20771), ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn7) / assign21410_e20771), 0.0, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn12) / assign21410_e20771),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21410_e20774;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21410_e20774_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21410_e20774_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21410_e20774_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21410_e20774_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21410_e20774_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21410_e20774_d_n12;

        let (assign21420_e20792, assign21420_e20792_d_n2, assign21420_e20792_d_n3, assign21420_e20792_d_n4, assign21420_e20792_d_n7, assign21420_e20792_d_n11, assign21420_e20792_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) {
        let assign21420_e20780: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21420_e20782: f64 = (assign21420_e20780 * locals.var_fn241_calc_iq__type);
        let assign21420_e20784: f64 = (assign21420_e20782 * locals.var_fn241_calc_iq__cs);
        let assign21420_e20786: f64 = (assign21420_e20784 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21420_e20788: f64 = (assign21420_e20786 * locals.var_fn241_calc_iq__exparg);
        let assign21420_e20790: f64 = (assign21420_e20788 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21420_e20790, ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((assign21420_e20784 * locals.var_fn241_calc_iq__two_n_phit0_dn4) * locals.var_fn241_calc_iq__exparg) + (assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign21420_e20792;
        locals.var_fn241_calc_iq__qsout_dn2 = assign21420_e20792_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign21420_e20792_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign21420_e20792_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign21420_e20792_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign21420_e20792_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign21420_e20792_d_n12;

        let (assign21430_e20799, assign21430_e20799_d_n2, assign21430_e20799_d_n3, assign21430_e20799_d_n4, assign21430_e20799_d_n7, assign21430_e20799_d_n11, assign21430_e20799_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign21430_e20799;
        locals.var_fn241_calc_iq__qsout_dn2 = assign21430_e20799_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign21430_e20799_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign21430_e20799_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign21430_e20799_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign21430_e20799_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign21430_e20799_d_n12;

        let (assign21460_e20811, assign21460_e20811_d_n2, assign21460_e20811_d_n4, assign21460_e20811_d_n7, assign21460_e20811_d_n11, assign21460_e20811_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    } else {
        (locals.var_qgsfps3, locals.var_qgsfps3_dn2, locals.var_qgsfps3_dn4, locals.var_qgsfps3_dn7, locals.var_qgsfps3_dn11, locals.var_qgsfps3_dn12,)
    }
};
        locals.var_qgsfps3 = assign21460_e20811;
        locals.var_qgsfps3_dn2 = assign21460_e20811_d_n2;
        locals.var_qgsfps3_dn4 = assign21460_e20811_d_n4;
        locals.var_qgsfps3_dn7 = assign21460_e20811_d_n7;
        locals.var_qgsfps3_dn11 = assign21460_e20811_d_n11;
        locals.var_qgsfps3_dn12 = assign21460_e20811_d_n12;

        let (assign21470_e20815, assign21470_e20815_d_n2, assign21470_e20815_d_n4, assign21470_e20815_d_n7, assign21470_e20815_d_n11, assign21470_e20815_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    } else {
        (locals.var_qgdfps3, locals.var_qgdfps3_dn2, locals.var_qgdfps3_dn4, locals.var_qgdfps3_dn7, locals.var_qgdfps3_dn11, locals.var_qgdfps3_dn12,)
    }
};
        locals.var_qgdfps3 = assign21470_e20815;
        locals.var_qgdfps3_dn2 = assign21470_e20815_d_n2;
        locals.var_qgdfps3_dn4 = assign21470_e20815_d_n4;
        locals.var_qgdfps3_dn7 = assign21470_e20815_d_n7;
        locals.var_qgdfps3_dn11 = assign21470_e20815_d_n11;
        locals.var_qgdfps3_dn12 = assign21470_e20815_d_n12;

        let (assign21480_e20819, assign21480_e20819_d_n2, assign21480_e20819_d_n3, assign21480_e20819_d_n4, assign21480_e20819_d_n7, assign21480_e20819_d_n11, assign21480_e20819_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    } else {
        (locals.var_qcfps3, locals.var_qcfps3_dn2, locals.var_qcfps3_dn3, locals.var_qcfps3_dn4, locals.var_qcfps3_dn7, locals.var_qcfps3_dn11, locals.var_qcfps3_dn12,)
    }
};
        locals.var_qcfps3 = assign21480_e20819;
        locals.var_qcfps3_dn2 = assign21480_e20819_d_n2;
        locals.var_qcfps3_dn3 = assign21480_e20819_d_n3;
        locals.var_qcfps3_dn4 = assign21480_e20819_d_n4;
        locals.var_qcfps3_dn7 = assign21480_e20819_d_n7;
        locals.var_qcfps3_dn11 = assign21480_e20819_d_n11;
        locals.var_qcfps3_dn12 = assign21480_e20819_d_n12;

        let (assign21490_e20823, assign21490_e20823_d_n2, assign21490_e20823_d_n3, assign21490_e20823_d_n4, assign21490_e20823_d_n7, assign21490_e20823_d_n11, assign21490_e20823_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    } else {
        (locals.var_qbfps3, locals.var_qbfps3_dn2, locals.var_qbfps3_dn3, locals.var_qbfps3_dn4, locals.var_qbfps3_dn7, locals.var_qbfps3_dn11, locals.var_qbfps3_dn12,)
    }
};
        locals.var_qbfps3 = assign21490_e20823;
        locals.var_qbfps3_dn2 = assign21490_e20823_d_n2;
        locals.var_qbfps3_dn3 = assign21490_e20823_d_n3;
        locals.var_qbfps3_dn4 = assign21490_e20823_d_n4;
        locals.var_qbfps3_dn7 = assign21490_e20823_d_n7;
        locals.var_qbfps3_dn11 = assign21490_e20823_d_n11;
        locals.var_qbfps3_dn12 = assign21490_e20823_d_n12;

        let (assign21500_e20827, assign21500_e20827_d_n2, assign21500_e20827_d_n3, assign21500_e20827_d_n4, assign21500_e20827_d_n7, assign21500_e20827_d_n11, assign21500_e20827_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    } else {
        (locals.var_qsfps3, locals.var_qsfps3_dn2, locals.var_qsfps3_dn3, locals.var_qsfps3_dn4, locals.var_qsfps3_dn7, locals.var_qsfps3_dn11, locals.var_qsfps3_dn12,)
    }
};
        locals.var_qsfps3 = assign21500_e20827;
        locals.var_qsfps3_dn2 = assign21500_e20827_d_n2;
        locals.var_qsfps3_dn3 = assign21500_e20827_d_n3;
        locals.var_qsfps3_dn4 = assign21500_e20827_d_n4;
        locals.var_qsfps3_dn7 = assign21500_e20827_d_n7;
        locals.var_qsfps3_dn11 = assign21500_e20827_d_n11;
        locals.var_qsfps3_dn12 = assign21500_e20827_d_n12;

        let assign21540_e20842: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign21540_e20842;

        locals.var_qgsfps4 = 0.0;
        locals.var_qgsfps4_dn2 = 0.0;
        locals.var_qgsfps4_dn4 = 0.0;
        locals.var_qgsfps4_dn7 = 0.0;
        locals.var_qgsfps4_dn12 = 0.0;
        locals.var_qgsfps4_dn13 = 0.0;

        locals.var_qgdfps4 = 0.0;
        locals.var_qgdfps4_dn2 = 0.0;
        locals.var_qgdfps4_dn4 = 0.0;
        locals.var_qgdfps4_dn7 = 0.0;
        locals.var_qgdfps4_dn12 = 0.0;
        locals.var_qgdfps4_dn13 = 0.0;

        locals.var_qcfps4 = 0.0;
        locals.var_qcfps4_dn2 = 0.0;
        locals.var_qcfps4_dn3 = 0.0;
        locals.var_qcfps4_dn4 = 0.0;
        locals.var_qcfps4_dn7 = 0.0;
        locals.var_qcfps4_dn12 = 0.0;
        locals.var_qcfps4_dn13 = 0.0;

        locals.var_qbfps4 = 0.0;
        locals.var_qbfps4_dn2 = 0.0;
        locals.var_qbfps4_dn3 = 0.0;
        locals.var_qbfps4_dn4 = 0.0;
        locals.var_qbfps4_dn7 = 0.0;
        locals.var_qbfps4_dn12 = 0.0;
        locals.var_qbfps4_dn13 = 0.0;

        locals.var_qsfps4 = 0.0;
        locals.var_qsfps4_dn2 = 0.0;
        locals.var_qsfps4_dn3 = 0.0;
        locals.var_qsfps4_dn4 = 0.0;
        locals.var_qsfps4_dn7 = 0.0;
        locals.var_qsfps4_dn12 = 0.0;
        locals.var_qsfps4_dn13 = 0.0;

        let assign21630_e20853: f64 = if p.p145 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign21630_e20853;

        let (assign21660_e20865, assign21660_e20865_d_n2, assign21660_e20865_d_n4, assign21660_e20865_d_n7, assign21660_e20865_d_n12, assign21660_e20865_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgsout = assign21660_e20865;
        locals.var_fn277_calc_iq__qgsout_dn2 = assign21660_e20865_d_n2;
        locals.var_fn277_calc_iq__qgsout_dn4 = assign21660_e20865_d_n4;
        locals.var_fn277_calc_iq__qgsout_dn7 = assign21660_e20865_d_n7;
        locals.var_fn277_calc_iq__qgsout_dn12 = assign21660_e20865_d_n12;
        locals.var_fn277_calc_iq__qgsout_dn13 = assign21660_e20865_d_n13;

        let (assign21670_e20869, assign21670_e20869_d_n2, assign21670_e20869_d_n4, assign21670_e20869_d_n7, assign21670_e20869_d_n12, assign21670_e20869_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgdout = assign21670_e20869;
        locals.var_fn277_calc_iq__qgdout_dn2 = assign21670_e20869_d_n2;
        locals.var_fn277_calc_iq__qgdout_dn4 = assign21670_e20869_d_n4;
        locals.var_fn277_calc_iq__qgdout_dn7 = assign21670_e20869_d_n7;
        locals.var_fn277_calc_iq__qgdout_dn12 = assign21670_e20869_d_n12;
        locals.var_fn277_calc_iq__qgdout_dn13 = assign21670_e20869_d_n13;

        let (assign21680_e20873, assign21680_e20873_d_n2, assign21680_e20873_d_n3, assign21680_e20873_d_n4, assign21680_e20873_d_n7, assign21680_e20873_d_n12, assign21680_e20873_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign21680_e20873;
        locals.var_fn277_calc_iq__qcout_dn2 = assign21680_e20873_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign21680_e20873_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign21680_e20873_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign21680_e20873_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign21680_e20873_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign21680_e20873_d_n13;

        let (assign21690_e20877, assign21690_e20877_d_n2, assign21690_e20877_d_n3, assign21690_e20877_d_n4, assign21690_e20877_d_n7, assign21690_e20877_d_n12, assign21690_e20877_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign21690_e20877;
        locals.var_fn277_calc_iq__qbout_dn2 = assign21690_e20877_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign21690_e20877_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign21690_e20877_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign21690_e20877_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign21690_e20877_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign21690_e20877_d_n13;

        let (assign21700_e20881, assign21700_e20881_d_n2, assign21700_e20881_d_n3, assign21700_e20881_d_n4, assign21700_e20881_d_n7, assign21700_e20881_d_n12, assign21700_e20881_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign21700_e20881;
        locals.var_fn277_calc_iq__qsout_dn2 = assign21700_e20881_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign21700_e20881_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign21700_e20881_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign21700_e20881_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign21700_e20881_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign21700_e20881_d_n13;

        let (assign21710_e20885, assign21710_e20885_d_n4, assign21710_e20885_d_n12, assign21710_e20885_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vtdibl, locals.var_fn277_calc_iq__vtdibl_dn4, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vtdibl = assign21710_e20885;
        locals.var_fn277_calc_iq__vtdibl_dn4 = assign21710_e20885_d_n4;
        locals.var_fn277_calc_iq__vtdibl_dn12 = assign21710_e20885_d_n12;
        locals.var_fn277_calc_iq__vtdibl_dn13 = assign21710_e20885_d_n13;

        let (assign21720_e20889, assign21720_e20889_d_n2, assign21720_e20889_d_n3, assign21720_e20889_d_n4, assign21720_e20889_d_n7, assign21720_e20889_d_n12, assign21720_e20889_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat1, locals.var_fn277_calc_iq__vdsat1_dn2, locals.var_fn277_calc_iq__vdsat1_dn3, locals.var_fn277_calc_iq__vdsat1_dn4, locals.var_fn277_calc_iq__vdsat1_dn7, locals.var_fn277_calc_iq__vdsat1_dn12, locals.var_fn277_calc_iq__vdsat1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat1 = assign21720_e20889;
        locals.var_fn277_calc_iq__vdsat1_dn2 = assign21720_e20889_d_n2;
        locals.var_fn277_calc_iq__vdsat1_dn3 = assign21720_e20889_d_n3;
        locals.var_fn277_calc_iq__vdsat1_dn4 = assign21720_e20889_d_n4;
        locals.var_fn277_calc_iq__vdsat1_dn7 = assign21720_e20889_d_n7;
        locals.var_fn277_calc_iq__vdsat1_dn12 = assign21720_e20889_d_n12;
        locals.var_fn277_calc_iq__vdsat1_dn13 = assign21720_e20889_d_n13;

        let (assign21730_e20893, assign21730_e20893_d_n2, assign21730_e20893_d_n7, assign21730_e20893_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vgsin, locals.var_fn277_calc_iq__vgsin_dn2, locals.var_fn277_calc_iq__vgsin_dn7, locals.var_fn277_calc_iq__vgsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgsin = assign21730_e20893;
        locals.var_fn277_calc_iq__vgsin_dn2 = assign21730_e20893_d_n2;
        locals.var_fn277_calc_iq__vgsin_dn7 = assign21730_e20893_d_n7;
        locals.var_fn277_calc_iq__vgsin_dn13 = assign21730_e20893_d_n13;

        let (assign21740_e20897, assign21740_e20897_d_n12, assign21740_e20897_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vdsfps4, locals.var_vdsfps4_dn12, locals.var_vdsfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vdsin, locals.var_fn277_calc_iq__vdsin_dn12, locals.var_fn277_calc_iq__vdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsin = assign21740_e20897;
        locals.var_fn277_calc_iq__vdsin_dn12 = assign21740_e20897_d_n12;
        locals.var_fn277_calc_iq__vdsin_dn13 = assign21740_e20897_d_n13;

        let (assign21750_e20901,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p151,)
    } else {
        (locals.var_fn277_calc_iq__qcbflag,)
    }
};
        locals.var_fn277_calc_iq__qcbflag = assign21750_e20901;

    }

    pub(super) fn stamp_transient_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21760_e20905, assign21760_e20905_d_n2, assign21760_e20905_d_n7, assign21760_e20905_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vcin, locals.var_fn277_calc_iq__vcin_dn2, locals.var_fn277_calc_iq__vcin_dn7, locals.var_fn277_calc_iq__vcin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vcin = assign21760_e20905;
        locals.var_fn277_calc_iq__vcin_dn2 = assign21760_e20905_d_n2;
        locals.var_fn277_calc_iq__vcin_dn7 = assign21760_e20905_d_n7;
        locals.var_fn277_calc_iq__vcin_dn13 = assign21760_e20905_d_n13;

        let (assign21770_e20909, assign21770_e20909_d_n3, assign21770_e20909_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vbfps4, locals.var_vbfps4_dn3, locals.var_vbfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vbin, locals.var_fn277_calc_iq__vbin_dn3, locals.var_fn277_calc_iq__vbin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vbin = assign21770_e20909;
        locals.var_fn277_calc_iq__vbin_dn3 = assign21770_e20909_d_n3;
        locals.var_fn277_calc_iq__vbin_dn13 = assign21770_e20909_d_n13;

        let (assign21780_e20913,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p149,)
    } else {
        (locals.var_fn277_calc_iq__qgsflag,)
    }
};
        locals.var_fn277_calc_iq__qgsflag = assign21780_e20913;

        let (assign21790_e20917, assign21790_e20917_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn277_calc_iq__tambin, locals.var_fn277_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tambin = assign21790_e20917;
        locals.var_fn277_calc_iq__tambin_dn4 = assign21790_e20917_d_n4;

        let (assign21800_e20921,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn277_calc_iq__tnomin,)
    }
};
        locals.var_fn277_calc_iq__tnomin = assign21800_e20921;

        let (assign21810_e20925, assign21810_e20925_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn277_calc_iq__phitin, locals.var_fn277_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn277_calc_iq__phitin = assign21810_e20925;
        locals.var_fn277_calc_iq__phitin_dn4 = assign21810_e20925_d_n4;

        let (assign21820_e20929,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn277_calc_iq__w,)
    }
};
        locals.var_fn277_calc_iq__w = assign21820_e20929;

        let (assign21830_e20933,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p145,)
    } else {
        (locals.var_fn277_calc_iq__lin,)
    }
};
        locals.var_fn277_calc_iq__lin = assign21830_e20933;

        let (assign21840_e20937, assign21840_e20937_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_cgfps4t, locals.var_cgfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cgin, locals.var_fn277_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn277_calc_iq__cgin = assign21840_e20937;
        locals.var_fn277_calc_iq__cgin_dn4 = assign21840_e20937_d_n4;

        let (assign21850_e20941,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p150,)
    } else {
        (locals.var_fn277_calc_iq__cs,)
    }
};
        locals.var_fn277_calc_iq__cs = assign21850_e20941;

        let (assign21860_e20945, assign21860_e20945_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_ccfps4t, locals.var_ccfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cc, locals.var_fn277_calc_iq__cc_dn4,)
    }
};
        locals.var_fn277_calc_iq__cc = assign21860_e20945;
        locals.var_fn277_calc_iq__cc_dn4 = assign21860_e20945_d_n4;

        let (assign21870_e20949, assign21870_e20949_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_cbfps4t, locals.var_cbfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cb, locals.var_fn277_calc_iq__cb_dn4,)
    }
};
        locals.var_fn277_calc_iq__cb = assign21870_e20949;
        locals.var_fn277_calc_iq__cb_dn4 = assign21870_e20949_d_n4;

        let (assign21880_e20953,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p146,)
    } else {
        (locals.var_fn277_calc_iq__vto,)
    }
};
        locals.var_fn277_calc_iq__vto = assign21880_e20953;

        let (assign21890_e20957,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p160,)
    } else {
        (locals.var_fn277_calc_iq__ss,)
    }
};
        locals.var_fn277_calc_iq__ss = assign21890_e20957;

        let (assign21900_e20961,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p159,)
    } else {
        (locals.var_fn277_calc_iq__delta1,)
    }
};
        locals.var_fn277_calc_iq__delta1 = assign21900_e20961;

        let (assign21910_e20965,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn277_calc_iq__delta2,)
    }
};
        locals.var_fn277_calc_iq__delta2 = assign21910_e20965;

        let (assign21920_e20969,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p161,)
    } else {
        (locals.var_fn277_calc_iq__nd,)
    }
};
        locals.var_fn277_calc_iq__nd = assign21920_e20969;

        let (assign21930_e20973,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p165,)
    } else {
        (locals.var_fn277_calc_iq__alpha,)
    }
};
        locals.var_fn277_calc_iq__alpha = assign21930_e20973;

        let (assign21940_e20977,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p156,)
    } else {
        (locals.var_fn277_calc_iq__vel0,)
    }
};
        locals.var_fn277_calc_iq__vel0 = assign21940_e20977;

        let (assign21950_e20981,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p157,)
    } else {
        (locals.var_fn277_calc_iq__mu0,)
    }
};
        locals.var_fn277_calc_iq__mu0 = assign21950_e20981;

        let (assign21960_e20985,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p158,)
    } else {
        (locals.var_fn277_calc_iq__beta,)
    }
};
        locals.var_fn277_calc_iq__beta = assign21960_e20985;

        let (assign21970_e20989,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p164,)
    } else {
        (locals.var_fn277_calc_iq__mtheta,)
    }
};
        locals.var_fn277_calc_iq__mtheta = assign21970_e20989;

        let (assign21980_e20993,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p163,)
    } else {
        (locals.var_fn277_calc_iq__vtheta,)
    }
};
        locals.var_fn277_calc_iq__vtheta = assign21980_e20993;

        let (assign21990_e20997,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p162,)
    } else {
        (locals.var_fn277_calc_iq__vtzeta,)
    }
};
        locals.var_fn277_calc_iq__vtzeta = assign21990_e20997;

        let (assign22000_e21001,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn277_calc_iq__dibsat,)
    }
};
        locals.var_fn277_calc_iq__dibsat = assign22000_e21001;

        let (assign22010_e21005,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn277_calc_iq__epsilon,)
    }
};
        locals.var_fn277_calc_iq__epsilon = assign22010_e21005;

        let (assign22020_e21009,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn277_calc_iq__vzeta,)
    }
};
        locals.var_fn277_calc_iq__vzeta = assign22020_e21009;

        let (assign22030_e21013,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn277_calc_iq__lambda,)
    }
};
        locals.var_fn277_calc_iq__lambda = assign22030_e21013;

        let (assign22040_e21017,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn277_calc_iq__ngf,)
    }
};
        locals.var_fn277_calc_iq__ngf = assign22040_e21017;

        let (assign22050_e21021,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn277_calc_iq__type,)
    }
};
        locals.var_fn277_calc_iq__type = assign22050_e21021;

        let (assign22060_e21025,) = {
    if (locals.var_guard276 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn277_calc_iq__trapfracdl,)
    }
};
        locals.var_fn277_calc_iq__trapfracdl = assign22060_e21025;

        let (assign22070_e21029, assign22070_e21029_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__alpha_phit, locals.var_fn277_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn277_calc_iq__alpha_phit = assign22070_e21029;
        locals.var_fn277_calc_iq__alpha_phit_dn4 = assign22070_e21029_d_n4;

        let (assign22080_e21033, assign22080_e21033_d_n12, assign22080_e21033_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__delta, locals.var_fn277_calc_iq__delta_dn12, locals.var_fn277_calc_iq__delta_dn13,)
    }
};
        locals.var_fn277_calc_iq__delta = assign22080_e21033;
        locals.var_fn277_calc_iq__delta_dn12 = assign22080_e21033_d_n12;
        locals.var_fn277_calc_iq__delta_dn13 = assign22080_e21033_d_n13;

        let (assign22090_e21037, assign22090_e21037_d_n4, assign22090_e21037_d_n12, assign22090_e21037_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__n, locals.var_fn277_calc_iq__n_dn4, locals.var_fn277_calc_iq__n_dn12, locals.var_fn277_calc_iq__n_dn13,)
    }
};
        locals.var_fn277_calc_iq__n = assign22090_e21037;
        locals.var_fn277_calc_iq__n_dn4 = assign22090_e21037_d_n4;
        locals.var_fn277_calc_iq__n_dn12 = assign22090_e21037_d_n12;
        locals.var_fn277_calc_iq__n_dn13 = assign22090_e21037_d_n13;

        let (assign22100_e21041, assign22100_e21041_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vtof, locals.var_fn277_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn277_calc_iq__vtof = assign22100_e21041;
        locals.var_fn277_calc_iq__vtof_dn4 = assign22100_e21041_d_n4;

        let (assign22110_e21045, assign22110_e21045_d_n12, assign22110_e21045_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22110_e21045;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22110_e21045_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22110_e21045_d_n13;

        let (assign22120_e21049, assign22120_e21049_d_n2, assign22120_e21049_d_n3, assign22120_e21049_d_n4, assign22120_e21049_d_n7, assign22120_e21049_d_n12, assign22120_e21049_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign22120_e21049;
        locals.var_fn277_calc_iq__ffs_dn2 = assign22120_e21049_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign22120_e21049_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign22120_e21049_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign22120_e21049_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign22120_e21049_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign22120_e21049_d_n13;

        let (assign22130_e21053, assign22130_e21053_d_n4, assign22130_e21053_d_n12, assign22130_e21053_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit, locals.var_fn277_calc_iq__two_n_phit_dn4, locals.var_fn277_calc_iq__two_n_phit_dn12, locals.var_fn277_calc_iq__two_n_phit_dn13,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit = assign22130_e21053;
        locals.var_fn277_calc_iq__two_n_phit_dn4 = assign22130_e21053_d_n4;
        locals.var_fn277_calc_iq__two_n_phit_dn12 = assign22130_e21053_d_n12;
        locals.var_fn277_calc_iq__two_n_phit_dn13 = assign22130_e21053_d_n13;

        let (assign22140_e21057, assign22140_e21057_d_n4, assign22140_e21057_d_n12, assign22140_e21057_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qref, locals.var_fn277_calc_iq__qref_dn4, locals.var_fn277_calc_iq__qref_dn12, locals.var_fn277_calc_iq__qref_dn13,)
    }
};
        locals.var_fn277_calc_iq__qref = assign22140_e21057;
        locals.var_fn277_calc_iq__qref_dn4 = assign22140_e21057_d_n4;
        locals.var_fn277_calc_iq__qref_dn12 = assign22140_e21057_d_n12;
        locals.var_fn277_calc_iq__qref_dn13 = assign22140_e21057_d_n13;

        let (assign22150_e21061, assign22150_e21061_d_n2, assign22150_e21061_d_n3, assign22150_e21061_d_n4, assign22150_e21061_d_n7, assign22150_e21061_d_n12, assign22150_e21061_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etas, locals.var_fn277_calc_iq__etas_dn2, locals.var_fn277_calc_iq__etas_dn3, locals.var_fn277_calc_iq__etas_dn4, locals.var_fn277_calc_iq__etas_dn7, locals.var_fn277_calc_iq__etas_dn12, locals.var_fn277_calc_iq__etas_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas = assign22150_e21061;
        locals.var_fn277_calc_iq__etas_dn2 = assign22150_e21061_d_n2;
        locals.var_fn277_calc_iq__etas_dn3 = assign22150_e21061_d_n3;
        locals.var_fn277_calc_iq__etas_dn4 = assign22150_e21061_d_n4;
        locals.var_fn277_calc_iq__etas_dn7 = assign22150_e21061_d_n7;
        locals.var_fn277_calc_iq__etas_dn12 = assign22150_e21061_d_n12;
        locals.var_fn277_calc_iq__etas_dn13 = assign22150_e21061_d_n13;

        let (assign22160_e21065, assign22160_e21065_d_n2, assign22160_e21065_d_n3, assign22160_e21065_d_n4, assign22160_e21065_d_n7, assign22160_e21065_d_n12, assign22160_e21065_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign22160_e21065;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign22160_e21065_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign22160_e21065_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign22160_e21065_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign22160_e21065_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign22160_e21065_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign22160_e21065_d_n13;

        let (assign22170_e21069, assign22170_e21069_d_n2, assign22170_e21069_d_n3, assign22170_e21069_d_n4, assign22170_e21069_d_n7, assign22170_e21069_d_n12, assign22170_e21069_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__muf, locals.var_fn277_calc_iq__muf_dn2, locals.var_fn277_calc_iq__muf_dn3, locals.var_fn277_calc_iq__muf_dn4, locals.var_fn277_calc_iq__muf_dn7, locals.var_fn277_calc_iq__muf_dn12, locals.var_fn277_calc_iq__muf_dn13,)
    }
};
        locals.var_fn277_calc_iq__muf = assign22170_e21069;
        locals.var_fn277_calc_iq__muf_dn2 = assign22170_e21069_d_n2;
        locals.var_fn277_calc_iq__muf_dn3 = assign22170_e21069_d_n3;
        locals.var_fn277_calc_iq__muf_dn4 = assign22170_e21069_d_n4;
        locals.var_fn277_calc_iq__muf_dn7 = assign22170_e21069_d_n7;
        locals.var_fn277_calc_iq__muf_dn12 = assign22170_e21069_d_n12;
        locals.var_fn277_calc_iq__muf_dn13 = assign22170_e21069_d_n13;

        let (assign22180_e21073, assign22180_e21073_d_n2, assign22180_e21073_d_n3, assign22180_e21073_d_n4, assign22180_e21073_d_n7, assign22180_e21073_d_n12, assign22180_e21073_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vx, locals.var_fn277_calc_iq__vx_dn2, locals.var_fn277_calc_iq__vx_dn3, locals.var_fn277_calc_iq__vx_dn4, locals.var_fn277_calc_iq__vx_dn7, locals.var_fn277_calc_iq__vx_dn12, locals.var_fn277_calc_iq__vx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vx = assign22180_e21073;
        locals.var_fn277_calc_iq__vx_dn2 = assign22180_e21073_d_n2;
        locals.var_fn277_calc_iq__vx_dn3 = assign22180_e21073_d_n3;
        locals.var_fn277_calc_iq__vx_dn4 = assign22180_e21073_d_n4;
        locals.var_fn277_calc_iq__vx_dn7 = assign22180_e21073_d_n7;
        locals.var_fn277_calc_iq__vx_dn12 = assign22180_e21073_d_n12;
        locals.var_fn277_calc_iq__vx_dn13 = assign22180_e21073_d_n13;

        let (assign22200_e21081, assign22200_e21081_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__n0, locals.var_fn277_calc_iq__n0_dn4,)
    }
};
        locals.var_fn277_calc_iq__n0 = assign22200_e21081;
        locals.var_fn277_calc_iq__n0_dn4 = assign22200_e21081_d_n4;

        let (assign22210_e21085, assign22210_e21085_d_n2, assign22210_e21085_d_n4, assign22210_e21085_d_n7, assign22210_e21085_d_n12, assign22210_e21085_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign22210_e21085;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign22210_e21085_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign22210_e21085_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign22210_e21085_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign22210_e21085_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign22210_e21085_d_n13;

        let (assign22220_e21089, assign22220_e21089_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit0, locals.var_fn277_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit0 = assign22220_e21089;
        locals.var_fn277_calc_iq__two_n_phit0_dn4 = assign22220_e21089_d_n4;

        let (assign22230_e21093, assign22230_e21093_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qref0, locals.var_fn277_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn277_calc_iq__qref0 = assign22230_e21093;
        locals.var_fn277_calc_iq__qref0_dn4 = assign22230_e21093_d_n4;

        let (assign22240_e21097, assign22240_e21097_d_n2, assign22240_e21097_d_n4, assign22240_e21097_d_n7, assign22240_e21097_d_n12, assign22240_e21097_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etas0, locals.var_fn277_calc_iq__etas0_dn2, locals.var_fn277_calc_iq__etas0_dn4, locals.var_fn277_calc_iq__etas0_dn7, locals.var_fn277_calc_iq__etas0_dn12, locals.var_fn277_calc_iq__etas0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas0 = assign22240_e21097;
        locals.var_fn277_calc_iq__etas0_dn2 = assign22240_e21097_d_n2;
        locals.var_fn277_calc_iq__etas0_dn4 = assign22240_e21097_d_n4;
        locals.var_fn277_calc_iq__etas0_dn7 = assign22240_e21097_d_n7;
        locals.var_fn277_calc_iq__etas0_dn12 = assign22240_e21097_d_n12;
        locals.var_fn277_calc_iq__etas0_dn13 = assign22240_e21097_d_n13;

    }

    pub(super) fn stamp_transient_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign22250_e21101, assign22250_e21101_d_n2, assign22250_e21101_d_n4, assign22250_e21101_d_n7, assign22250_e21101_d_n12, assign22250_e21101_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign22250_e21101;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign22250_e21101_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign22250_e21101_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign22250_e21101_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign22250_e21101_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign22250_e21101_d_n13;

        let (assign22260_e21105, assign22260_e21105_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__muf0, locals.var_fn277_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn277_calc_iq__muf0 = assign22260_e21105;
        locals.var_fn277_calc_iq__muf0_dn4 = assign22260_e21105_d_n4;

        let (assign22270_e21109, assign22270_e21109_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vx0, locals.var_fn277_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vx0 = assign22270_e21109;
        locals.var_fn277_calc_iq__vx0_dn4 = assign22270_e21109_d_n4;

        let (assign22280_e21113, assign22280_e21113_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__tfacmobin, locals.var_fn277_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tfacmobin = assign22280_e21113;
        locals.var_fn277_calc_iq__tfacmobin_dn4 = assign22280_e21113_d_n4;

        let (assign22290_e21117, assign22290_e21117_d_n2, assign22290_e21117_d_n3, assign22290_e21117_d_n4, assign22290_e21117_d_n7, assign22290_e21117_d_n12, assign22290_e21117_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22290_e21117;
        locals.var_fn277_calc_iq__ff_dn2 = assign22290_e21117_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22290_e21117_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22290_e21117_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22290_e21117_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22290_e21117_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22290_e21117_d_n13;

        let (assign22300_e21121, assign22300_e21121_d_n2, assign22300_e21121_d_n3, assign22300_e21121_d_n4, assign22300_e21121_d_n7, assign22300_e21121_d_n12, assign22300_e21121_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__eta, locals.var_fn277_calc_iq__eta_dn2, locals.var_fn277_calc_iq__eta_dn3, locals.var_fn277_calc_iq__eta_dn4, locals.var_fn277_calc_iq__eta_dn7, locals.var_fn277_calc_iq__eta_dn12, locals.var_fn277_calc_iq__eta_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta = assign22300_e21121;
        locals.var_fn277_calc_iq__eta_dn2 = assign22300_e21121_d_n2;
        locals.var_fn277_calc_iq__eta_dn3 = assign22300_e21121_d_n3;
        locals.var_fn277_calc_iq__eta_dn4 = assign22300_e21121_d_n4;
        locals.var_fn277_calc_iq__eta_dn7 = assign22300_e21121_d_n7;
        locals.var_fn277_calc_iq__eta_dn12 = assign22300_e21121_d_n12;
        locals.var_fn277_calc_iq__eta_dn13 = assign22300_e21121_d_n13;

        let (assign22310_e21125, assign22310_e21125_d_n2, assign22310_e21125_d_n3, assign22310_e21125_d_n4, assign22310_e21125_d_n7, assign22310_e21125_d_n12, assign22310_e21125_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign22310_e21125;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign22310_e21125_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign22310_e21125_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign22310_e21125_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign22310_e21125_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign22310_e21125_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign22310_e21125_d_n13;

        let (assign22320_e21129, assign22320_e21129_d_n2, assign22320_e21129_d_n4, assign22320_e21129_d_n7, assign22320_e21129_d_n12, assign22320_e21129_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign22320_e21129;
        locals.var_fn277_calc_iq__ff0_dn2 = assign22320_e21129_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign22320_e21129_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign22320_e21129_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign22320_e21129_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign22320_e21129_d_n13;

        let (assign22330_e21133, assign22330_e21133_d_n2, assign22330_e21133_d_n4, assign22330_e21133_d_n7, assign22330_e21133_d_n12, assign22330_e21133_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__eta0, locals.var_fn277_calc_iq__eta0_dn2, locals.var_fn277_calc_iq__eta0_dn4, locals.var_fn277_calc_iq__eta0_dn7, locals.var_fn277_calc_iq__eta0_dn12, locals.var_fn277_calc_iq__eta0_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta0 = assign22330_e21133;
        locals.var_fn277_calc_iq__eta0_dn2 = assign22330_e21133_d_n2;
        locals.var_fn277_calc_iq__eta0_dn4 = assign22330_e21133_d_n4;
        locals.var_fn277_calc_iq__eta0_dn7 = assign22330_e21133_d_n7;
        locals.var_fn277_calc_iq__eta0_dn12 = assign22330_e21133_d_n12;
        locals.var_fn277_calc_iq__eta0_dn13 = assign22330_e21133_d_n13;

        let (assign22340_e21137, assign22340_e21137_d_n2, assign22340_e21137_d_n4, assign22340_e21137_d_n7, assign22340_e21137_d_n12, assign22340_e21137_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign22340_e21137;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign22340_e21137_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign22340_e21137_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign22340_e21137_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign22340_e21137_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign22340_e21137_d_n13;

        let (assign22350_e21141, assign22350_e21141_d_n2, assign22350_e21141_d_n3, assign22350_e21141_d_n4, assign22350_e21141_d_n7, assign22350_e21141_d_n12, assign22350_e21141_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats, locals.var_fn277_calc_iq__vdsats_dn2, locals.var_fn277_calc_iq__vdsats_dn3, locals.var_fn277_calc_iq__vdsats_dn4, locals.var_fn277_calc_iq__vdsats_dn7, locals.var_fn277_calc_iq__vdsats_dn12, locals.var_fn277_calc_iq__vdsats_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats = assign22350_e21141;
        locals.var_fn277_calc_iq__vdsats_dn2 = assign22350_e21141_d_n2;
        locals.var_fn277_calc_iq__vdsats_dn3 = assign22350_e21141_d_n3;
        locals.var_fn277_calc_iq__vdsats_dn4 = assign22350_e21141_d_n4;
        locals.var_fn277_calc_iq__vdsats_dn7 = assign22350_e21141_d_n7;
        locals.var_fn277_calc_iq__vdsats_dn12 = assign22350_e21141_d_n12;
        locals.var_fn277_calc_iq__vdsats_dn13 = assign22350_e21141_d_n13;

        let (assign22360_e21145, assign22360_e21145_d_n2, assign22360_e21145_d_n3, assign22360_e21145_d_n4, assign22360_e21145_d_n7, assign22360_e21145_d_n12, assign22360_e21145_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats1, locals.var_fn277_calc_iq__vdsats1_dn2, locals.var_fn277_calc_iq__vdsats1_dn3, locals.var_fn277_calc_iq__vdsats1_dn4, locals.var_fn277_calc_iq__vdsats1_dn7, locals.var_fn277_calc_iq__vdsats1_dn12, locals.var_fn277_calc_iq__vdsats1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats1 = assign22360_e21145;
        locals.var_fn277_calc_iq__vdsats1_dn2 = assign22360_e21145_d_n2;
        locals.var_fn277_calc_iq__vdsats1_dn3 = assign22360_e21145_d_n3;
        locals.var_fn277_calc_iq__vdsats1_dn4 = assign22360_e21145_d_n4;
        locals.var_fn277_calc_iq__vdsats1_dn7 = assign22360_e21145_d_n7;
        locals.var_fn277_calc_iq__vdsats1_dn12 = assign22360_e21145_d_n12;
        locals.var_fn277_calc_iq__vdsats1_dn13 = assign22360_e21145_d_n13;

        let (assign22370_e21149, assign22370_e21149_d_n2, assign22370_e21149_d_n3, assign22370_e21149_d_n4, assign22370_e21149_d_n7, assign22370_e21149_d_n12, assign22370_e21149_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat, locals.var_fn277_calc_iq__vdsat_dn2, locals.var_fn277_calc_iq__vdsat_dn3, locals.var_fn277_calc_iq__vdsat_dn4, locals.var_fn277_calc_iq__vdsat_dn7, locals.var_fn277_calc_iq__vdsat_dn12, locals.var_fn277_calc_iq__vdsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat = assign22370_e21149;
        locals.var_fn277_calc_iq__vdsat_dn2 = assign22370_e21149_d_n2;
        locals.var_fn277_calc_iq__vdsat_dn3 = assign22370_e21149_d_n3;
        locals.var_fn277_calc_iq__vdsat_dn4 = assign22370_e21149_d_n4;
        locals.var_fn277_calc_iq__vdsat_dn7 = assign22370_e21149_d_n7;
        locals.var_fn277_calc_iq__vdsat_dn12 = assign22370_e21149_d_n12;
        locals.var_fn277_calc_iq__vdsat_dn13 = assign22370_e21149_d_n13;

        let (assign22380_e21153, assign22380_e21153_d_n2, assign22380_e21153_d_n3, assign22380_e21153_d_n4, assign22380_e21153_d_n7, assign22380_e21153_d_n12, assign22380_e21153_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsd, locals.var_fn277_calc_iq__fsd_dn2, locals.var_fn277_calc_iq__fsd_dn3, locals.var_fn277_calc_iq__fsd_dn4, locals.var_fn277_calc_iq__fsd_dn7, locals.var_fn277_calc_iq__fsd_dn12, locals.var_fn277_calc_iq__fsd_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd = assign22380_e21153;
        locals.var_fn277_calc_iq__fsd_dn2 = assign22380_e21153_d_n2;
        locals.var_fn277_calc_iq__fsd_dn3 = assign22380_e21153_d_n3;
        locals.var_fn277_calc_iq__fsd_dn4 = assign22380_e21153_d_n4;
        locals.var_fn277_calc_iq__fsd_dn7 = assign22380_e21153_d_n7;
        locals.var_fn277_calc_iq__fsd_dn12 = assign22380_e21153_d_n12;
        locals.var_fn277_calc_iq__fsd_dn13 = assign22380_e21153_d_n13;

        let (assign22390_e21157, assign22390_e21157_d_n2, assign22390_e21157_d_n3, assign22390_e21157_d_n4, assign22390_e21157_d_n7, assign22390_e21157_d_n12, assign22390_e21157_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdx, locals.var_fn277_calc_iq__vdx_dn2, locals.var_fn277_calc_iq__vdx_dn3, locals.var_fn277_calc_iq__vdx_dn4, locals.var_fn277_calc_iq__vdx_dn7, locals.var_fn277_calc_iq__vdx_dn12, locals.var_fn277_calc_iq__vdx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx = assign22390_e21157;
        locals.var_fn277_calc_iq__vdx_dn2 = assign22390_e21157_d_n2;
        locals.var_fn277_calc_iq__vdx_dn3 = assign22390_e21157_d_n3;
        locals.var_fn277_calc_iq__vdx_dn4 = assign22390_e21157_d_n4;
        locals.var_fn277_calc_iq__vdx_dn7 = assign22390_e21157_d_n7;
        locals.var_fn277_calc_iq__vdx_dn12 = assign22390_e21157_d_n12;
        locals.var_fn277_calc_iq__vdx_dn13 = assign22390_e21157_d_n13;

        let (assign22400_e21161, assign22400_e21161_d_n2, assign22400_e21161_d_n3, assign22400_e21161_d_n4, assign22400_e21161_d_n7, assign22400_e21161_d_n12, assign22400_e21161_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fds, locals.var_fn277_calc_iq__fds_dn2, locals.var_fn277_calc_iq__fds_dn3, locals.var_fn277_calc_iq__fds_dn4, locals.var_fn277_calc_iq__fds_dn7, locals.var_fn277_calc_iq__fds_dn12, locals.var_fn277_calc_iq__fds_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds = assign22400_e21161;
        locals.var_fn277_calc_iq__fds_dn2 = assign22400_e21161_d_n2;
        locals.var_fn277_calc_iq__fds_dn3 = assign22400_e21161_d_n3;
        locals.var_fn277_calc_iq__fds_dn4 = assign22400_e21161_d_n4;
        locals.var_fn277_calc_iq__fds_dn7 = assign22400_e21161_d_n7;
        locals.var_fn277_calc_iq__fds_dn12 = assign22400_e21161_d_n12;
        locals.var_fn277_calc_iq__fds_dn13 = assign22400_e21161_d_n13;

        let (assign22410_e21165, assign22410_e21165_d_n2, assign22410_e21165_d_n3, assign22410_e21165_d_n4, assign22410_e21165_d_n7, assign22410_e21165_d_n12, assign22410_e21165_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsx, locals.var_fn277_calc_iq__vsx_dn2, locals.var_fn277_calc_iq__vsx_dn3, locals.var_fn277_calc_iq__vsx_dn4, locals.var_fn277_calc_iq__vsx_dn7, locals.var_fn277_calc_iq__vsx_dn12, locals.var_fn277_calc_iq__vsx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx = assign22410_e21165;
        locals.var_fn277_calc_iq__vsx_dn2 = assign22410_e21165_d_n2;
        locals.var_fn277_calc_iq__vsx_dn3 = assign22410_e21165_d_n3;
        locals.var_fn277_calc_iq__vsx_dn4 = assign22410_e21165_d_n4;
        locals.var_fn277_calc_iq__vsx_dn7 = assign22410_e21165_d_n7;
        locals.var_fn277_calc_iq__vsx_dn12 = assign22410_e21165_d_n12;
        locals.var_fn277_calc_iq__vsx_dn13 = assign22410_e21165_d_n13;

        let (assign22420_e21169, assign22420_e21169_d_n2, assign22420_e21169_d_n3, assign22420_e21169_d_n4, assign22420_e21169_d_n7, assign22420_e21169_d_n12, assign22420_e21169_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign22420_e21169;
        locals.var_fn277_calc_iq__ffd_dn2 = assign22420_e21169_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign22420_e21169_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign22420_e21169_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign22420_e21169_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign22420_e21169_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign22420_e21169_d_n13;

        let (assign22430_e21173, assign22430_e21173_d_n2, assign22430_e21173_d_n3, assign22430_e21173_d_n4, assign22430_e21173_d_n7, assign22430_e21173_d_n12, assign22430_e21173_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etad, locals.var_fn277_calc_iq__etad_dn2, locals.var_fn277_calc_iq__etad_dn3, locals.var_fn277_calc_iq__etad_dn4, locals.var_fn277_calc_iq__etad_dn7, locals.var_fn277_calc_iq__etad_dn12, locals.var_fn277_calc_iq__etad_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad = assign22430_e21173;
        locals.var_fn277_calc_iq__etad_dn2 = assign22430_e21173_d_n2;
        locals.var_fn277_calc_iq__etad_dn3 = assign22430_e21173_d_n3;
        locals.var_fn277_calc_iq__etad_dn4 = assign22430_e21173_d_n4;
        locals.var_fn277_calc_iq__etad_dn7 = assign22430_e21173_d_n7;
        locals.var_fn277_calc_iq__etad_dn12 = assign22430_e21173_d_n12;
        locals.var_fn277_calc_iq__etad_dn13 = assign22430_e21173_d_n13;

        let (assign22440_e21177, assign22440_e21177_d_n2, assign22440_e21177_d_n3, assign22440_e21177_d_n4, assign22440_e21177_d_n7, assign22440_e21177_d_n12, assign22440_e21177_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign22440_e21177;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign22440_e21177_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign22440_e21177_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign22440_e21177_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign22440_e21177_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign22440_e21177_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign22440_e21177_d_n13;

        let (assign22450_e21181, assign22450_e21181_d_n2, assign22450_e21181_d_n3, assign22450_e21181_d_n4, assign22450_e21181_d_n7, assign22450_e21181_d_n12, assign22450_e21181_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsc, locals.var_fn277_calc_iq__vdsc_dn2, locals.var_fn277_calc_iq__vdsc_dn3, locals.var_fn277_calc_iq__vdsc_dn4, locals.var_fn277_calc_iq__vdsc_dn7, locals.var_fn277_calc_iq__vdsc_dn12, locals.var_fn277_calc_iq__vdsc_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsc = assign22450_e21181;
        locals.var_fn277_calc_iq__vdsc_dn2 = assign22450_e21181_d_n2;
        locals.var_fn277_calc_iq__vdsc_dn3 = assign22450_e21181_d_n3;
        locals.var_fn277_calc_iq__vdsc_dn4 = assign22450_e21181_d_n4;
        locals.var_fn277_calc_iq__vdsc_dn7 = assign22450_e21181_d_n7;
        locals.var_fn277_calc_iq__vdsc_dn12 = assign22450_e21181_d_n12;
        locals.var_fn277_calc_iq__vdsc_dn13 = assign22450_e21181_d_n13;

        let (assign22480_e21193, assign22480_e21193_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats0, locals.var_fn277_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vdsats0 = assign22480_e21193;
        locals.var_fn277_calc_iq__vdsats0_dn4 = assign22480_e21193_d_n4;

        let (assign22490_e21197, assign22490_e21197_d_n2, assign22490_e21197_d_n4, assign22490_e21197_d_n7, assign22490_e21197_d_n12, assign22490_e21197_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats10, locals.var_fn277_calc_iq__vdsats10_dn2, locals.var_fn277_calc_iq__vdsats10_dn4, locals.var_fn277_calc_iq__vdsats10_dn7, locals.var_fn277_calc_iq__vdsats10_dn12, locals.var_fn277_calc_iq__vdsats10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats10 = assign22490_e21197;
        locals.var_fn277_calc_iq__vdsats10_dn2 = assign22490_e21197_d_n2;
        locals.var_fn277_calc_iq__vdsats10_dn4 = assign22490_e21197_d_n4;
        locals.var_fn277_calc_iq__vdsats10_dn7 = assign22490_e21197_d_n7;
        locals.var_fn277_calc_iq__vdsats10_dn12 = assign22490_e21197_d_n12;
        locals.var_fn277_calc_iq__vdsats10_dn13 = assign22490_e21197_d_n13;

        let (assign22500_e21201, assign22500_e21201_d_n2, assign22500_e21201_d_n4, assign22500_e21201_d_n7, assign22500_e21201_d_n12, assign22500_e21201_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat10, locals.var_fn277_calc_iq__vdsat10_dn2, locals.var_fn277_calc_iq__vdsat10_dn4, locals.var_fn277_calc_iq__vdsat10_dn7, locals.var_fn277_calc_iq__vdsat10_dn12, locals.var_fn277_calc_iq__vdsat10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat10 = assign22500_e21201;
        locals.var_fn277_calc_iq__vdsat10_dn2 = assign22500_e21201_d_n2;
        locals.var_fn277_calc_iq__vdsat10_dn4 = assign22500_e21201_d_n4;
        locals.var_fn277_calc_iq__vdsat10_dn7 = assign22500_e21201_d_n7;
        locals.var_fn277_calc_iq__vdsat10_dn12 = assign22500_e21201_d_n12;
        locals.var_fn277_calc_iq__vdsat10_dn13 = assign22500_e21201_d_n13;

        let (assign22510_e21205, assign22510_e21205_d_n2, assign22510_e21205_d_n4, assign22510_e21205_d_n7, assign22510_e21205_d_n12, assign22510_e21205_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsd0, locals.var_fn277_calc_iq__fsd0_dn2, locals.var_fn277_calc_iq__fsd0_dn4, locals.var_fn277_calc_iq__fsd0_dn7, locals.var_fn277_calc_iq__fsd0_dn12, locals.var_fn277_calc_iq__fsd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd0 = assign22510_e21205;
        locals.var_fn277_calc_iq__fsd0_dn2 = assign22510_e21205_d_n2;
        locals.var_fn277_calc_iq__fsd0_dn4 = assign22510_e21205_d_n4;
        locals.var_fn277_calc_iq__fsd0_dn7 = assign22510_e21205_d_n7;
        locals.var_fn277_calc_iq__fsd0_dn12 = assign22510_e21205_d_n12;
        locals.var_fn277_calc_iq__fsd0_dn13 = assign22510_e21205_d_n13;

        let (assign22520_e21209, assign22520_e21209_d_n2, assign22520_e21209_d_n4, assign22520_e21209_d_n7, assign22520_e21209_d_n12, assign22520_e21209_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdx0, locals.var_fn277_calc_iq__vdx0_dn2, locals.var_fn277_calc_iq__vdx0_dn4, locals.var_fn277_calc_iq__vdx0_dn7, locals.var_fn277_calc_iq__vdx0_dn12, locals.var_fn277_calc_iq__vdx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx0 = assign22520_e21209;
        locals.var_fn277_calc_iq__vdx0_dn2 = assign22520_e21209_d_n2;
        locals.var_fn277_calc_iq__vdx0_dn4 = assign22520_e21209_d_n4;
        locals.var_fn277_calc_iq__vdx0_dn7 = assign22520_e21209_d_n7;
        locals.var_fn277_calc_iq__vdx0_dn12 = assign22520_e21209_d_n12;
        locals.var_fn277_calc_iq__vdx0_dn13 = assign22520_e21209_d_n13;

        let (assign22530_e21213, assign22530_e21213_d_n2, assign22530_e21213_d_n4, assign22530_e21213_d_n7, assign22530_e21213_d_n12, assign22530_e21213_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fds0, locals.var_fn277_calc_iq__fds0_dn2, locals.var_fn277_calc_iq__fds0_dn4, locals.var_fn277_calc_iq__fds0_dn7, locals.var_fn277_calc_iq__fds0_dn12, locals.var_fn277_calc_iq__fds0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds0 = assign22530_e21213;
        locals.var_fn277_calc_iq__fds0_dn2 = assign22530_e21213_d_n2;
        locals.var_fn277_calc_iq__fds0_dn4 = assign22530_e21213_d_n4;
        locals.var_fn277_calc_iq__fds0_dn7 = assign22530_e21213_d_n7;
        locals.var_fn277_calc_iq__fds0_dn12 = assign22530_e21213_d_n12;
        locals.var_fn277_calc_iq__fds0_dn13 = assign22530_e21213_d_n13;

        let (assign22540_e21217, assign22540_e21217_d_n2, assign22540_e21217_d_n4, assign22540_e21217_d_n7, assign22540_e21217_d_n12, assign22540_e21217_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsx0, locals.var_fn277_calc_iq__vsx0_dn2, locals.var_fn277_calc_iq__vsx0_dn4, locals.var_fn277_calc_iq__vsx0_dn7, locals.var_fn277_calc_iq__vsx0_dn12, locals.var_fn277_calc_iq__vsx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx0 = assign22540_e21217;
        locals.var_fn277_calc_iq__vsx0_dn2 = assign22540_e21217_d_n2;
        locals.var_fn277_calc_iq__vsx0_dn4 = assign22540_e21217_d_n4;
        locals.var_fn277_calc_iq__vsx0_dn7 = assign22540_e21217_d_n7;
        locals.var_fn277_calc_iq__vsx0_dn12 = assign22540_e21217_d_n12;
        locals.var_fn277_calc_iq__vsx0_dn13 = assign22540_e21217_d_n13;

        let (assign22550_e21221, assign22550_e21221_d_n2, assign22550_e21221_d_n4, assign22550_e21221_d_n7, assign22550_e21221_d_n12, assign22550_e21221_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign22550_e21221;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign22550_e21221_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign22550_e21221_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign22550_e21221_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign22550_e21221_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign22550_e21221_d_n13;

        let (assign22560_e21225, assign22560_e21225_d_n2, assign22560_e21225_d_n4, assign22560_e21225_d_n7, assign22560_e21225_d_n12, assign22560_e21225_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etad0, locals.var_fn277_calc_iq__etad0_dn2, locals.var_fn277_calc_iq__etad0_dn4, locals.var_fn277_calc_iq__etad0_dn7, locals.var_fn277_calc_iq__etad0_dn12, locals.var_fn277_calc_iq__etad0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad0 = assign22560_e21225;
        locals.var_fn277_calc_iq__etad0_dn2 = assign22560_e21225_d_n2;
        locals.var_fn277_calc_iq__etad0_dn4 = assign22560_e21225_d_n4;
        locals.var_fn277_calc_iq__etad0_dn7 = assign22560_e21225_d_n7;
        locals.var_fn277_calc_iq__etad0_dn12 = assign22560_e21225_d_n12;
        locals.var_fn277_calc_iq__etad0_dn13 = assign22560_e21225_d_n13;

        let (assign22570_e21229, assign22570_e21229_d_n2, assign22570_e21229_d_n4, assign22570_e21229_d_n7, assign22570_e21229_d_n12, assign22570_e21229_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign22570_e21229;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign22570_e21229_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign22570_e21229_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign22570_e21229_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign22570_e21229_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign22570_e21229_d_n13;

        let (assign22580_e21233, assign22580_e21233_d_n2, assign22580_e21233_d_n4, assign22580_e21233_d_n7, assign22580_e21233_d_n12, assign22580_e21233_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs2, locals.var_fn277_calc_iq__qs2_dn2, locals.var_fn277_calc_iq__qs2_dn4, locals.var_fn277_calc_iq__qs2_dn7, locals.var_fn277_calc_iq__qs2_dn12, locals.var_fn277_calc_iq__qs2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs2 = assign22580_e21233;
        locals.var_fn277_calc_iq__qs2_dn2 = assign22580_e21233_d_n2;
        locals.var_fn277_calc_iq__qs2_dn4 = assign22580_e21233_d_n4;
        locals.var_fn277_calc_iq__qs2_dn7 = assign22580_e21233_d_n7;
        locals.var_fn277_calc_iq__qs2_dn12 = assign22580_e21233_d_n12;
        locals.var_fn277_calc_iq__qs2_dn13 = assign22580_e21233_d_n13;

        let (assign22590_e21237, assign22590_e21237_d_n2, assign22590_e21237_d_n4, assign22590_e21237_d_n7, assign22590_e21237_d_n12, assign22590_e21237_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs3, locals.var_fn277_calc_iq__qs3_dn2, locals.var_fn277_calc_iq__qs3_dn4, locals.var_fn277_calc_iq__qs3_dn7, locals.var_fn277_calc_iq__qs3_dn12, locals.var_fn277_calc_iq__qs3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs3 = assign22590_e21237;
        locals.var_fn277_calc_iq__qs3_dn2 = assign22590_e21237_d_n2;
        locals.var_fn277_calc_iq__qs3_dn4 = assign22590_e21237_d_n4;
        locals.var_fn277_calc_iq__qs3_dn7 = assign22590_e21237_d_n7;
        locals.var_fn277_calc_iq__qs3_dn12 = assign22590_e21237_d_n12;
        locals.var_fn277_calc_iq__qs3_dn13 = assign22590_e21237_d_n13;

        let (assign22600_e21241, assign22600_e21241_d_n2, assign22600_e21241_d_n4, assign22600_e21241_d_n7, assign22600_e21241_d_n12, assign22600_e21241_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd2, locals.var_fn277_calc_iq__qd2_dn2, locals.var_fn277_calc_iq__qd2_dn4, locals.var_fn277_calc_iq__qd2_dn7, locals.var_fn277_calc_iq__qd2_dn12, locals.var_fn277_calc_iq__qd2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd2 = assign22600_e21241;
        locals.var_fn277_calc_iq__qd2_dn2 = assign22600_e21241_d_n2;
        locals.var_fn277_calc_iq__qd2_dn4 = assign22600_e21241_d_n4;
        locals.var_fn277_calc_iq__qd2_dn7 = assign22600_e21241_d_n7;
        locals.var_fn277_calc_iq__qd2_dn12 = assign22600_e21241_d_n12;
        locals.var_fn277_calc_iq__qd2_dn13 = assign22600_e21241_d_n13;

        let (assign22610_e21245, assign22610_e21245_d_n2, assign22610_e21245_d_n4, assign22610_e21245_d_n7, assign22610_e21245_d_n12, assign22610_e21245_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd3, locals.var_fn277_calc_iq__qd3_dn2, locals.var_fn277_calc_iq__qd3_dn4, locals.var_fn277_calc_iq__qd3_dn7, locals.var_fn277_calc_iq__qd3_dn12, locals.var_fn277_calc_iq__qd3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd3 = assign22610_e21245;
        locals.var_fn277_calc_iq__qd3_dn2 = assign22610_e21245_d_n2;
        locals.var_fn277_calc_iq__qd3_dn4 = assign22610_e21245_d_n4;
        locals.var_fn277_calc_iq__qd3_dn7 = assign22610_e21245_d_n7;
        locals.var_fn277_calc_iq__qd3_dn12 = assign22610_e21245_d_n12;
        locals.var_fn277_calc_iq__qd3_dn13 = assign22610_e21245_d_n13;

        let (assign22620_e21249, assign22620_e21249_d_n2, assign22620_e21249_d_n4, assign22620_e21249_d_n7, assign22620_e21249_d_n12, assign22620_e21249_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsqd, locals.var_fn277_calc_iq__qsqd_dn2, locals.var_fn277_calc_iq__qsqd_dn4, locals.var_fn277_calc_iq__qsqd_dn7, locals.var_fn277_calc_iq__qsqd_dn12, locals.var_fn277_calc_iq__qsqd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsqd = assign22620_e21249;
        locals.var_fn277_calc_iq__qsqd_dn2 = assign22620_e21249_d_n2;
        locals.var_fn277_calc_iq__qsqd_dn4 = assign22620_e21249_d_n4;
        locals.var_fn277_calc_iq__qsqd_dn7 = assign22620_e21249_d_n7;
        locals.var_fn277_calc_iq__qsqd_dn12 = assign22620_e21249_d_n12;
        locals.var_fn277_calc_iq__qsqd_dn13 = assign22620_e21249_d_n13;

    }

    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22630_e21253, assign22630_e21253_d_n2, assign22630_e21253_d_n4, assign22630_e21253_d_n7, assign22630_e21253_d_n12, assign22630_e21253_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvdd, locals.var_fn277_calc_iq__qinvdd_dn2, locals.var_fn277_calc_iq__qinvdd_dn4, locals.var_fn277_calc_iq__qinvdd_dn7, locals.var_fn277_calc_iq__qinvdd_dn12, locals.var_fn277_calc_iq__qinvdd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvdd = assign22630_e21253;
        locals.var_fn277_calc_iq__qinvdd_dn2 = assign22630_e21253_d_n2;
        locals.var_fn277_calc_iq__qinvdd_dn4 = assign22630_e21253_d_n4;
        locals.var_fn277_calc_iq__qinvdd_dn7 = assign22630_e21253_d_n7;
        locals.var_fn277_calc_iq__qinvdd_dn12 = assign22630_e21253_d_n12;
        locals.var_fn277_calc_iq__qinvdd_dn13 = assign22630_e21253_d_n13;

        let (assign22640_e21257, assign22640_e21257_d_n2, assign22640_e21257_d_n4, assign22640_e21257_d_n7, assign22640_e21257_d_n12, assign22640_e21257_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd1 = assign22640_e21257;
        locals.var_fn277_calc_iq__qd1_dn2 = assign22640_e21257_d_n2;
        locals.var_fn277_calc_iq__qd1_dn4 = assign22640_e21257_d_n4;
        locals.var_fn277_calc_iq__qd1_dn7 = assign22640_e21257_d_n7;
        locals.var_fn277_calc_iq__qd1_dn12 = assign22640_e21257_d_n12;
        locals.var_fn277_calc_iq__qd1_dn13 = assign22640_e21257_d_n13;

        let (assign22650_e21261, assign22650_e21261_d_n2, assign22650_e21261_d_n4, assign22650_e21261_d_n7, assign22650_e21261_d_n12, assign22650_e21261_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs, locals.var_fn277_calc_iq__qs_dn2, locals.var_fn277_calc_iq__qs_dn4, locals.var_fn277_calc_iq__qs_dn7, locals.var_fn277_calc_iq__qs_dn12, locals.var_fn277_calc_iq__qs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs = assign22650_e21261;
        locals.var_fn277_calc_iq__qs_dn2 = assign22650_e21261_d_n2;
        locals.var_fn277_calc_iq__qs_dn4 = assign22650_e21261_d_n4;
        locals.var_fn277_calc_iq__qs_dn7 = assign22650_e21261_d_n7;
        locals.var_fn277_calc_iq__qs_dn12 = assign22650_e21261_d_n12;
        locals.var_fn277_calc_iq__qs_dn13 = assign22650_e21261_d_n13;

        let (assign22660_e21265, assign22660_e21265_d_n2, assign22660_e21265_d_n4, assign22660_e21265_d_n7, assign22660_e21265_d_n12, assign22660_e21265_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd, locals.var_fn277_calc_iq__qd_dn2, locals.var_fn277_calc_iq__qd_dn4, locals.var_fn277_calc_iq__qd_dn7, locals.var_fn277_calc_iq__qd_dn12, locals.var_fn277_calc_iq__qd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd = assign22660_e21265;
        locals.var_fn277_calc_iq__qd_dn2 = assign22660_e21265_d_n2;
        locals.var_fn277_calc_iq__qd_dn4 = assign22660_e21265_d_n4;
        locals.var_fn277_calc_iq__qd_dn7 = assign22660_e21265_d_n7;
        locals.var_fn277_calc_iq__qd_dn12 = assign22660_e21265_d_n12;
        locals.var_fn277_calc_iq__qd_dn13 = assign22660_e21265_d_n13;

        let (assign22670_e21269, assign22670_e21269_d_n2, assign22670_e21269_d_n4, assign22670_e21269_d_n7, assign22670_e21269_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, locals.var_fn277_calc_iq__etac_dn13,)
    }
};
        locals.var_fn277_calc_iq__etac = assign22670_e21269;
        locals.var_fn277_calc_iq__etac_dn2 = assign22670_e21269_d_n2;
        locals.var_fn277_calc_iq__etac_dn4 = assign22670_e21269_d_n4;
        locals.var_fn277_calc_iq__etac_dn7 = assign22670_e21269_d_n7;
        locals.var_fn277_calc_iq__etac_dn13 = assign22670_e21269_d_n13;

        let (assign22680_e21273, assign22680_e21273_d_n3, assign22680_e21273_d_n4, assign22680_e21273_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etab, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, locals.var_fn277_calc_iq__etab_dn13,)
    }
};
        locals.var_fn277_calc_iq__etab = assign22680_e21273;
        locals.var_fn277_calc_iq__etab_dn3 = assign22680_e21273_d_n3;
        locals.var_fn277_calc_iq__etab_dn4 = assign22680_e21273_d_n4;
        locals.var_fn277_calc_iq__etab_dn13 = assign22680_e21273_d_n13;

        let (assign22690_e21277, assign22690_e21277_d_n2, assign22690_e21277_d_n4, assign22690_e21277_d_n7, assign22690_e21277_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, locals.var_fn277_calc_iq__etags_dn13,)
    }
};
        locals.var_fn277_calc_iq__etags = assign22690_e21277;
        locals.var_fn277_calc_iq__etags_dn2 = assign22690_e21277_d_n2;
        locals.var_fn277_calc_iq__etags_dn4 = assign22690_e21277_d_n4;
        locals.var_fn277_calc_iq__etags_dn7 = assign22690_e21277_d_n7;
        locals.var_fn277_calc_iq__etags_dn13 = assign22690_e21277_d_n13;

        let (assign22700_e21281, assign22700_e21281_d_n2, assign22700_e21281_d_n3, assign22700_e21281_d_n4, assign22700_e21281_d_n7, assign22700_e21281_d_n12, assign22700_e21281_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign22700_e21281;
        locals.var_fn277_calc_iq__exparg_dn2 = assign22700_e21281_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign22700_e21281_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign22700_e21281_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign22700_e21281_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign22700_e21281_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign22700_e21281_d_n13;

        let (assign22710_e21285, assign22710_e21285_d_n2, assign22710_e21285_d_n3, assign22710_e21285_d_n4, assign22710_e21285_d_n7, assign22710_e21285_d_n12, assign22710_e21285_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign22710_e21285;
        locals.var_fn277_calc_iq__myarg_dn2 = assign22710_e21285_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign22710_e21285_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign22710_e21285_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign22710_e21285_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign22710_e21285_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign22710_e21285_d_n13;

        let (assign22720_e21289, assign22720_e21289_d_n12, assign22720_e21289_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__absvdsin, locals.var_fn277_calc_iq__absvdsin_dn12, locals.var_fn277_calc_iq__absvdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__absvdsin = assign22720_e21289;
        locals.var_fn277_calc_iq__absvdsin_dn12 = assign22720_e21289_d_n12;
        locals.var_fn277_calc_iq__absvdsin_dn13 = assign22720_e21289_d_n13;

        let (assign22730_e21293, assign22730_e21293_d_n2, assign22730_e21293_d_n7, assign22730_e21293_d_n12, assign22730_e21293_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vgdin, locals.var_fn277_calc_iq__vgdin_dn2, locals.var_fn277_calc_iq__vgdin_dn7, locals.var_fn277_calc_iq__vgdin_dn12, locals.var_fn277_calc_iq__vgdin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgdin = assign22730_e21293;
        locals.var_fn277_calc_iq__vgdin_dn2 = assign22730_e21293_d_n2;
        locals.var_fn277_calc_iq__vgdin_dn7 = assign22730_e21293_d_n7;
        locals.var_fn277_calc_iq__vgdin_dn12 = assign22730_e21293_d_n12;
        locals.var_fn277_calc_iq__vgdin_dn13 = assign22730_e21293_d_n13;

        let (assign22740_e21297, assign22740_e21297_d_n2, assign22740_e21297_d_n4, assign22740_e21297_d_n7, assign22740_e21297_d_n12, assign22740_e21297_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign22740_e21297;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign22740_e21297_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign22740_e21297_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign22740_e21297_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign22740_e21297_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign22740_e21297_d_n13;

        let (assign22750_e21301, assign22750_e21301_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__myarg0, locals.var_fn277_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn277_calc_iq__myarg0 = assign22750_e21301;
        locals.var_fn277_calc_iq__myarg0_dn4 = assign22750_e21301_d_n4;

        let (assign22760_e21328, assign22760_e21328_d_n12, assign22760_e21328_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22760_e21326, assign22760_e21326_d_n12, assign22760_e21326_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22760_e21310: f64 = (0.001 / p.p53);
                let assign22760_e21312: f64 = (assign22760_e21310 * locals.var_fn277_calc_iq__vdsin);
                let assign22760_e21313: f64 = (assign22760_e21312).tanh();
                let assign22760_e21314: f64 = (locals.var_fn277_calc_iq__vdsin * assign22760_e21313);
                (assign22760_e21314, ((locals.var_fn277_calc_iq__vdsin_dn12 * assign22760_e21313) + (locals.var_fn277_calc_iq__vdsin * ((assign22760_e21310 * locals.var_fn277_calc_iq__vdsin_dn12) / ((assign22760_e21312).cosh() * (assign22760_e21312).cosh())))), ((locals.var_fn277_calc_iq__vdsin_dn13 * assign22760_e21313) + (locals.var_fn277_calc_iq__vdsin * ((assign22760_e21310 * locals.var_fn277_calc_iq__vdsin_dn13) / ((assign22760_e21312).cosh() * (assign22760_e21312).cosh())))),)
            } else {
                let (assign22760_e21325, assign22760_e21325_d_n12, assign22760_e21325_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22760_e21320: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin);
                        let assign22760_e21322: f64 = (assign22760_e21320 + p.p53);
                        let assign22760_e21323: f64 = (assign22760_e21322).sqrt();
                        (assign22760_e21323, (((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsin) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin_dn12)) / (2.0 * assign22760_e21323)), (((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsin) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin_dn13)) / (2.0 * assign22760_e21323)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign22760_e21325, assign22760_e21325_d_n12, assign22760_e21325_d_n13,)
            }
        };
        (assign22760_e21326, assign22760_e21326_d_n12, assign22760_e21326_d_n13,)
    } else {
        (locals.var_fn277_calc_iq__absvdsin, locals.var_fn277_calc_iq__absvdsin_dn12, locals.var_fn277_calc_iq__absvdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__absvdsin = assign22760_e21328;
        locals.var_fn277_calc_iq__absvdsin_dn12 = assign22760_e21328_d_n12;
        locals.var_fn277_calc_iq__absvdsin_dn13 = assign22760_e21328_d_n13;

        let (assign22770_e21334, assign22770_e21334_d_n2, assign22770_e21334_d_n7, assign22770_e21334_d_n12, assign22770_e21334_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22770_e21332: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdsin);
        (assign22770_e21332, locals.var_fn277_calc_iq__vgsin_dn2, locals.var_fn277_calc_iq__vgsin_dn7, (-locals.var_fn277_calc_iq__vdsin_dn12), (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdsin_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vgdin, locals.var_fn277_calc_iq__vgdin_dn2, locals.var_fn277_calc_iq__vgdin_dn7, locals.var_fn277_calc_iq__vgdin_dn12, locals.var_fn277_calc_iq__vgdin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgdin = assign22770_e21334;
        locals.var_fn277_calc_iq__vgdin_dn2 = assign22770_e21334_d_n2;
        locals.var_fn277_calc_iq__vgdin_dn7 = assign22770_e21334_d_n7;
        locals.var_fn277_calc_iq__vgdin_dn12 = assign22770_e21334_d_n12;
        locals.var_fn277_calc_iq__vgdin_dn13 = assign22770_e21334_d_n13;

        let (assign22780_e21340, assign22780_e21340_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22780_e21338: f64 = (locals.var_fn277_calc_iq__alpha * locals.var_fn277_calc_iq__phitin);
        (assign22780_e21338, (locals.var_fn277_calc_iq__alpha * locals.var_fn277_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn277_calc_iq__alpha_phit, locals.var_fn277_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn277_calc_iq__alpha_phit = assign22780_e21340;
        locals.var_fn277_calc_iq__alpha_phit_dn4 = assign22780_e21340_d_n4;

        let (assign22790_e21352, assign22790_e21352_d_n4, assign22790_e21352_d_n12, assign22790_e21352_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22790_e21345: f64 = (2.302585092994046 * locals.var_fn277_calc_iq__phitin);
        let assign22790_e21346: f64 = (locals.var_fn277_calc_iq__ss / assign22790_e21345);
        let assign22790_e21349: f64 = (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin);
        let assign22790_e21350: f64 = (assign22790_e21346 + assign22790_e21349);
        (assign22790_e21350, (-((locals.var_fn277_calc_iq__ss * (2.302585092994046 * locals.var_fn277_calc_iq__phitin_dn4)) / (assign22790_e21345 * assign22790_e21345))), (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin_dn12), (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin_dn13),)
    } else {
        (locals.var_fn277_calc_iq__n, locals.var_fn277_calc_iq__n_dn4, locals.var_fn277_calc_iq__n_dn12, locals.var_fn277_calc_iq__n_dn13,)
    }
};
        locals.var_fn277_calc_iq__n = assign22790_e21352;
        locals.var_fn277_calc_iq__n_dn4 = assign22790_e21352_d_n4;
        locals.var_fn277_calc_iq__n_dn12 = assign22790_e21352_d_n12;
        locals.var_fn277_calc_iq__n_dn13 = assign22790_e21352_d_n13;

        let (assign22800_e21362, assign22800_e21362_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22800_e21358: f64 = (locals.var_fn277_calc_iq__tambin - locals.var_fn277_calc_iq__tnomin);
        let assign22800_e21359: f64 = (locals.var_fn277_calc_iq__vtzeta * assign22800_e21358);
        let assign22800_e21360: f64 = (locals.var_fn277_calc_iq__vto + assign22800_e21359);
        (assign22800_e21360, (locals.var_fn277_calc_iq__vtzeta * locals.var_fn277_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn277_calc_iq__vtof, locals.var_fn277_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn277_calc_iq__vtof = assign22800_e21362;
        locals.var_fn277_calc_iq__vtof_dn4 = assign22800_e21362_d_n4;

        let (assign22810_e21370, assign22810_e21370_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22810_e21366: f64 = (locals.var_fn277_calc_iq__tambin / locals.var_fn277_calc_iq__tnomin);
        let assign22810_e21368: f64 = (assign22810_e21366).powf(locals.var_fn277_calc_iq__epsilon);
        (assign22810_e21368, if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn277_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__epsilon * ((assign22810_e21366).powf(locals.var_fn277_calc_iq__epsilon - 1.0) * (locals.var_fn277_calc_iq__tambin_dn4 / locals.var_fn277_calc_iq__tnomin))) } } else { (assign22810_e21368 * (locals.var_fn277_calc_iq__epsilon * ((locals.var_fn277_calc_iq__tambin_dn4 / locals.var_fn277_calc_iq__tnomin) / assign22810_e21366))) },)
    } else {
        (locals.var_fn277_calc_iq__tfacmobin, locals.var_fn277_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tfacmobin = assign22810_e21370;
        locals.var_fn277_calc_iq__tfacmobin_dn4 = assign22810_e21370_d_n4;

        let assign22820_e21373: f64 = if locals.var_fn277_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign22820_e21373;

        let (assign22830_e21391, assign22830_e21391_d_n12, assign22830_e21391_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard278 != 0.0)) {
        let assign22830_e21381: f64 = (locals.var_fn277_calc_iq__absvdsin / locals.var_fn277_calc_iq__dibsat);
        let assign22830_e21383: f64 = (assign22830_e21381).powf(locals.var_fn277_calc_iq__beta);
        let assign22830_e21384: f64 = (1.0 + assign22830_e21383);
        let assign22830_e21387: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign22830_e21388: f64 = (assign22830_e21384).powf(assign22830_e21387);
        let assign22830_e21389: f64 = (locals.var_fn277_calc_iq__absvdsin / assign22830_e21388);
        (assign22830_e21389, (((locals.var_fn277_calc_iq__absvdsin_dn12 * assign22830_e21388) - (locals.var_fn277_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign22830_e21387) as f64).is_finite() && ((assign22830_e21387) as f64).fract() == 0.0 { if assign22830_e21387 == 0.0 { 0.0 } else { (assign22830_e21387 * ((assign22830_e21384).powf(assign22830_e21387 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) })) } } else { (assign22830_e21388 * (assign22830_e21387 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) } / assign22830_e21384))) })) / (assign22830_e21388 * assign22830_e21388)), (((locals.var_fn277_calc_iq__absvdsin_dn13 * assign22830_e21388) - (locals.var_fn277_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign22830_e21387) as f64).is_finite() && ((assign22830_e21387) as f64).fract() == 0.0 { if assign22830_e21387 == 0.0 { 0.0 } else { (assign22830_e21387 * ((assign22830_e21384).powf(assign22830_e21387 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) })) } } else { (assign22830_e21388 * (assign22830_e21387 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) } / assign22830_e21384))) })) / (assign22830_e21388 * assign22830_e21388)),)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22830_e21391;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22830_e21391_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22830_e21391_d_n13;

        let (assign22840_e21398, assign22840_e21398_d_n12, assign22840_e21398_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard278 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22840_e21398;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22840_e21398_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22840_e21398_d_n13;

        let (assign22850_e21408, assign22850_e21408_d_n12, assign22850_e21408_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22850_e21403: f64 = (locals.var_fn277_calc_iq__vsatdibl * locals.var_fn277_calc_iq__delta2);
        let assign22850_e21404: f64 = (locals.var_fn277_calc_iq__delta1 - assign22850_e21403);
        let assign22850_e21406: f64 = (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin);
        (assign22850_e21406, (((-(locals.var_fn277_calc_iq__vsatdibl_dn12 * locals.var_fn277_calc_iq__delta2)) * locals.var_fn277_calc_iq__absvdsin) + (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin_dn12)), (((-(locals.var_fn277_calc_iq__vsatdibl_dn13 * locals.var_fn277_calc_iq__delta2)) * locals.var_fn277_calc_iq__absvdsin) + (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__delta, locals.var_fn277_calc_iq__delta_dn12, locals.var_fn277_calc_iq__delta_dn13,)
    }
};
        locals.var_fn277_calc_iq__delta = assign22850_e21408;
        locals.var_fn277_calc_iq__delta_dn12 = assign22850_e21408_d_n12;
        locals.var_fn277_calc_iq__delta_dn13 = assign22850_e21408_d_n13;

        let (assign22860_e21414, assign22860_e21414_d_n4, assign22860_e21414_d_n12, assign22860_e21414_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22860_e21412: f64 = (locals.var_fn277_calc_iq__vtof - locals.var_fn277_calc_iq__delta);
        (assign22860_e21412, locals.var_fn277_calc_iq__vtof_dn4, (-locals.var_fn277_calc_iq__delta_dn12), (-locals.var_fn277_calc_iq__delta_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vtdibl, locals.var_fn277_calc_iq__vtdibl_dn4, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vtdibl = assign22860_e21414;
        locals.var_fn277_calc_iq__vtdibl_dn4 = assign22860_e21414_d_n4;
        locals.var_fn277_calc_iq__vtdibl_dn12 = assign22860_e21414_d_n12;
        locals.var_fn277_calc_iq__vtdibl_dn13 = assign22860_e21414_d_n13;

        let (assign22870_e21422, assign22870_e21422_d_n4, assign22870_e21422_d_n12, assign22870_e21422_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22870_e21418: f64 = (2.0 * locals.var_fn277_calc_iq__n);
        let assign22870_e21420: f64 = (assign22870_e21418 * locals.var_fn277_calc_iq__phitin);
        (assign22870_e21420, (((2.0 * locals.var_fn277_calc_iq__n_dn4) * locals.var_fn277_calc_iq__phitin) + (assign22870_e21418 * locals.var_fn277_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn277_calc_iq__n_dn12) * locals.var_fn277_calc_iq__phitin), ((2.0 * locals.var_fn277_calc_iq__n_dn13) * locals.var_fn277_calc_iq__phitin),)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit, locals.var_fn277_calc_iq__two_n_phit_dn4, locals.var_fn277_calc_iq__two_n_phit_dn12, locals.var_fn277_calc_iq__two_n_phit_dn13,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit = assign22870_e21422;
        locals.var_fn277_calc_iq__two_n_phit_dn4 = assign22870_e21422_d_n4;
        locals.var_fn277_calc_iq__two_n_phit_dn12 = assign22870_e21422_d_n12;
        locals.var_fn277_calc_iq__two_n_phit_dn13 = assign22870_e21422_d_n13;

        let (assign22880_e21428, assign22880_e21428_d_n4, assign22880_e21428_d_n12, assign22880_e21428_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22880_e21426: f64 = (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit);
        (assign22880_e21426, ((locals.var_fn277_calc_iq__cgin_dn4 * locals.var_fn277_calc_iq__two_n_phit) + (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn4)), (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn12), (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qref, locals.var_fn277_calc_iq__qref_dn4, locals.var_fn277_calc_iq__qref_dn12, locals.var_fn277_calc_iq__qref_dn13,)
    }
};
        locals.var_fn277_calc_iq__qref = assign22880_e21428;
        locals.var_fn277_calc_iq__qref_dn4 = assign22880_e21428_d_n4;
        locals.var_fn277_calc_iq__qref_dn12 = assign22880_e21428_d_n12;
        locals.var_fn277_calc_iq__qref_dn13 = assign22880_e21428_d_n13;

        let (assign22890_e21438, assign22890_e21438_d_n2, assign22890_e21438_d_n3, assign22890_e21438_d_n4, assign22890_e21438_d_n7, assign22890_e21438_d_n12, assign22890_e21438_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22890_e21433: f64 = (p.p51 * locals.var_fn277_calc_iq__alpha_phit);
        let assign22890_e21435: f64 = (assign22890_e21433 / 2.0);
        let assign22890_e21436: f64 = (locals.var_fn277_calc_iq__vtdibl - assign22890_e21435);
        (assign22890_e21436, 0.0, 0.0, (locals.var_fn277_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn277_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign22890_e21438;
        locals.var_fn277_calc_iq__myarg_dn2 = assign22890_e21438_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign22890_e21438_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign22890_e21438_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign22890_e21438_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign22890_e21438_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign22890_e21438_d_n13;

        let (assign22900_e21489, assign22900_e21489_d_n2, assign22900_e21489_d_n3, assign22900_e21489_d_n4, assign22900_e21489_d_n7, assign22900_e21489_d_n12, assign22900_e21489_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22900_e21483, assign22900_e21483_d_n2, assign22900_e21483_d_n7, assign22900_e21483_d_n12, assign22900_e21483_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22900_e21447: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21450: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21453: f64 = (0.001 / p.p53);
                let assign22900_e21456: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21457: f64 = (assign22900_e21453 * assign22900_e21456);
                let assign22900_e21458: f64 = (assign22900_e21457).tanh();
                let assign22900_e21459: f64 = (assign22900_e21450 * assign22900_e21458);
                let assign22900_e21460: f64 = (assign22900_e21447 + assign22900_e21459);
                let assign22900_e21461: f64 = (0.5 * assign22900_e21460);
                (assign22900_e21461, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))),)
            } else {
                let (assign22900_e21482, assign22900_e21482_d_n2, assign22900_e21482_d_n7, assign22900_e21482_d_n12, assign22900_e21482_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22900_e21468: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21471: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21474: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21475: f64 = (assign22900_e21471 * assign22900_e21474);
                        let assign22900_e21477: f64 = (assign22900_e21475 + p.p53);
                        let assign22900_e21478: f64 = (assign22900_e21477).sqrt();
                        let assign22900_e21479: f64 = (assign22900_e21468 + assign22900_e21478);
                        let assign22900_e21480: f64 = (0.5 * assign22900_e21479);
                        (assign22900_e21480, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign22900_e21478)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign22900_e21478)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22900_e21474) + (assign22900_e21471 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign22900_e21478)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign22900_e21478)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign22900_e21482, assign22900_e21482_d_n2, assign22900_e21482_d_n7, assign22900_e21482_d_n12, assign22900_e21482_d_n13,)
            }
        };
        let assign22900_e21485: f64 = (assign22900_e21483 - locals.var_fn277_calc_iq__myarg);
        let assign22900_e21487: f64 = (assign22900_e21485 / locals.var_fn277_calc_iq__alpha_phit);
        (assign22900_e21487, ((assign22900_e21483_d_n2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign22900_e21485 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((assign22900_e21483_d_n7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((assign22900_e21483_d_n12 - locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((assign22900_e21483_d_n13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign22900_e21489;
        locals.var_fn277_calc_iq__exparg_dn2 = assign22900_e21489_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign22900_e21489_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign22900_e21489_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign22900_e21489_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign22900_e21489_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign22900_e21489_d_n13;

        let assign22910_e21492: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign22910_e21492;

        let (assign22920_e21498, assign22920_e21498_d_n2, assign22920_e21498_d_n3, assign22920_e21498_d_n4, assign22920_e21498_d_n7, assign22920_e21498_d_n12, assign22920_e21498_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard279 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22920_e21498;
        locals.var_fn277_calc_iq__ff_dn2 = assign22920_e21498_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22920_e21498_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22920_e21498_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22920_e21498_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22920_e21498_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22920_e21498_d_n13;

        let assign22930_e21501: f64 = (-50.0);
        let assign22930_e21502: f64 = if locals.var_fn277_calc_iq__exparg < assign22930_e21501 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign22930_e21502;

        let (assign22940_e21511, assign22940_e21511_d_n2, assign22940_e21511_d_n3, assign22940_e21511_d_n4, assign22940_e21511_d_n7, assign22940_e21511_d_n12, assign22940_e21511_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard279 == 0.0)) && (locals.var_guard280 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22940_e21511;
        locals.var_fn277_calc_iq__ff_dn2 = assign22940_e21511_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22940_e21511_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22940_e21511_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22940_e21511_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22940_e21511_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22940_e21511_d_n13;

        let (assign22950_e21526, assign22950_e21526_d_n2, assign22950_e21526_d_n3, assign22950_e21526_d_n4, assign22950_e21526_d_n7, assign22950_e21526_d_n12, assign22950_e21526_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard279 == 0.0)) && (locals.var_guard280 == 0.0)) {
        let assign22950_e21522: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign22950_e21523: f64 = (1.0 + assign22950_e21522);
        let assign22950_e21524: f64 = (1.0 / assign22950_e21523);
        (assign22950_e21524, (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn2) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn3) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn4) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn7) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn12) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn13) / (assign22950_e21523 * assign22950_e21523))),)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22950_e21526;
        locals.var_fn277_calc_iq__ff_dn2 = assign22950_e21526_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22950_e21526_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22950_e21526_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22950_e21526_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22950_e21526_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22950_e21526_d_n13;

    }

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22960_e21585, assign22960_e21585_d_n2, assign22960_e21585_d_n3, assign22960_e21585_d_n4, assign22960_e21585_d_n7, assign22960_e21585_d_n12, assign22960_e21585_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22960_e21571, assign22960_e21571_d_n2, assign22960_e21571_d_n7, assign22960_e21571_d_n12, assign22960_e21571_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22960_e21535: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21538: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21541: f64 = (0.001 / p.p53);
                let assign22960_e21544: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21545: f64 = (assign22960_e21541 * assign22960_e21544);
                let assign22960_e21546: f64 = (assign22960_e21545).tanh();
                let assign22960_e21547: f64 = (assign22960_e21538 * assign22960_e21546);
                let assign22960_e21548: f64 = (assign22960_e21535 + assign22960_e21547);
                let assign22960_e21549: f64 = (0.5 * assign22960_e21548);
                (assign22960_e21549, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))),)
            } else {
                let (assign22960_e21570, assign22960_e21570_d_n2, assign22960_e21570_d_n7, assign22960_e21570_d_n12, assign22960_e21570_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22960_e21556: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21559: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21562: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21563: f64 = (assign22960_e21559 * assign22960_e21562);
                        let assign22960_e21565: f64 = (assign22960_e21563 + p.p53);
                        let assign22960_e21566: f64 = (assign22960_e21565).sqrt();
                        let assign22960_e21567: f64 = (assign22960_e21556 + assign22960_e21566);
                        let assign22960_e21568: f64 = (0.5 * assign22960_e21567);
                        (assign22960_e21568, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign22960_e21566)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign22960_e21566)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22960_e21562) + (assign22960_e21559 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign22960_e21566)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign22960_e21566)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign22960_e21570, assign22960_e21570_d_n2, assign22960_e21570_d_n7, assign22960_e21570_d_n12, assign22960_e21570_d_n13,)
            }
        };
        let assign22960_e21575: f64 = (p.p51 * 0.1);
        let assign22960_e21577: f64 = (assign22960_e21575 * locals.var_fn277_calc_iq__alpha_phit);
        let assign22960_e21579: f64 = (assign22960_e21577 * locals.var_fn277_calc_iq__ff);
        let assign22960_e21580: f64 = (locals.var_fn277_calc_iq__vtdibl - assign22960_e21579);
        let assign22960_e21581: f64 = (assign22960_e21571 - assign22960_e21580);
        let assign22960_e21583: f64 = (assign22960_e21581 / locals.var_fn277_calc_iq__two_n_phit);
        (assign22960_e21583, ((assign22960_e21571_d_n2 - (-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn2))) / locals.var_fn277_calc_iq__two_n_phit), ((-(-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn3))) / locals.var_fn277_calc_iq__two_n_phit), ((((-(locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign22960_e21575 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ff) + (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), ((assign22960_e21571_d_n7 - (-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn7))) / locals.var_fn277_calc_iq__two_n_phit), ((((assign22960_e21571_d_n12 - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), ((((assign22960_e21571_d_n13 - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__eta, locals.var_fn277_calc_iq__eta_dn2, locals.var_fn277_calc_iq__eta_dn3, locals.var_fn277_calc_iq__eta_dn4, locals.var_fn277_calc_iq__eta_dn7, locals.var_fn277_calc_iq__eta_dn12, locals.var_fn277_calc_iq__eta_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta = assign22960_e21585;
        locals.var_fn277_calc_iq__eta_dn2 = assign22960_e21585_d_n2;
        locals.var_fn277_calc_iq__eta_dn3 = assign22960_e21585_d_n3;
        locals.var_fn277_calc_iq__eta_dn4 = assign22960_e21585_d_n4;
        locals.var_fn277_calc_iq__eta_dn7 = assign22960_e21585_d_n7;
        locals.var_fn277_calc_iq__eta_dn12 = assign22960_e21585_d_n12;
        locals.var_fn277_calc_iq__eta_dn13 = assign22960_e21585_d_n13;

        let assign22970_e21588: f64 = if locals.var_fn277_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign22970_e21588;

        let (assign22980_e21596, assign22980_e21596_d_n2, assign22980_e21596_d_n3, assign22980_e21596_d_n4, assign22980_e21596_d_n7, assign22980_e21596_d_n12, assign22980_e21596_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign22980_e21594: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta);
        (assign22980_e21594, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign22980_e21596;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign22980_e21596_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign22980_e21596_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign22980_e21596_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign22980_e21596_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign22980_e21596_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign22980_e21596_d_n13;

        let assign22990_e21599: f64 = (-50.0);
        let assign22990_e21600: f64 = if locals.var_fn277_calc_iq__eta < assign22990_e21599 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign22990_e21600;

        let (assign23000_e21612, assign23000_e21612_d_n2, assign23000_e21612_d_n3, assign23000_e21612_d_n4, assign23000_e21612_d_n7, assign23000_e21612_d_n12, assign23000_e21612_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard281 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign23000_e21609: f64 = (locals.var_fn277_calc_iq__eta).exp();
        let assign23000_e21610: f64 = (locals.var_fn277_calc_iq__qref * assign23000_e21609);
        (assign23000_e21610, (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn2)), (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn4))), (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign23000_e21612;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign23000_e21612_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign23000_e21612_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign23000_e21612_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign23000_e21612_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign23000_e21612_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign23000_e21612_d_n13;

        let (assign23010_e21628, assign23010_e21628_d_n2, assign23010_e21628_d_n3, assign23010_e21628_d_n4, assign23010_e21628_d_n7, assign23010_e21628_d_n12, assign23010_e21628_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard281 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let assign23010_e21623: f64 = (locals.var_fn277_calc_iq__eta).exp();
        let assign23010_e21624: f64 = (1.0 + assign23010_e21623);
        let assign23010_e21625: f64 = (assign23010_e21624).ln();
        let assign23010_e21626: f64 = (locals.var_fn277_calc_iq__qref * assign23010_e21625);
        (assign23010_e21626, (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn2) / assign23010_e21624)), (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn3) / assign23010_e21624)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn4) / assign23010_e21624))), (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn7) / assign23010_e21624)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn12) / assign23010_e21624))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn13) / assign23010_e21624))),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign23010_e21628;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign23010_e21628_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign23010_e21628_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign23010_e21628_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign23010_e21628_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign23010_e21628_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign23010_e21628_d_n13;

        let (assign23020_e21642, assign23020_e21642_d_n2, assign23020_e21642_d_n3, assign23020_e21642_d_n4, assign23020_e21642_d_n7, assign23020_e21642_d_n12, assign23020_e21642_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23020_e21635: f64 = (locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv);
        let assign23020_e21637: f64 = (assign23020_e21635 / locals.var_fn277_calc_iq__cgin);
        let assign23020_e21638: f64 = (1.0 + assign23020_e21637);
        let assign23020_e21639: f64 = (locals.var_fn277_calc_iq__tfacmobin * assign23020_e21638);
        let assign23020_e21640: f64 = (locals.var_fn277_calc_iq__mu0 / assign23020_e21639);
        (assign23020_e21640, (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * ((locals.var_fn277_calc_iq__tfacmobin_dn4 * assign23020_e21638) + (locals.var_fn277_calc_iq__tfacmobin * ((((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23020_e21635 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin))))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))),)
    } else {
        (locals.var_fn277_calc_iq__muf, locals.var_fn277_calc_iq__muf_dn2, locals.var_fn277_calc_iq__muf_dn3, locals.var_fn277_calc_iq__muf_dn4, locals.var_fn277_calc_iq__muf_dn7, locals.var_fn277_calc_iq__muf_dn12, locals.var_fn277_calc_iq__muf_dn13,)
    }
};
        locals.var_fn277_calc_iq__muf = assign23020_e21642;
        locals.var_fn277_calc_iq__muf_dn2 = assign23020_e21642_d_n2;
        locals.var_fn277_calc_iq__muf_dn3 = assign23020_e21642_d_n3;
        locals.var_fn277_calc_iq__muf_dn4 = assign23020_e21642_d_n4;
        locals.var_fn277_calc_iq__muf_dn7 = assign23020_e21642_d_n7;
        locals.var_fn277_calc_iq__muf_dn12 = assign23020_e21642_d_n12;
        locals.var_fn277_calc_iq__muf_dn13 = assign23020_e21642_d_n13;

        let (assign23030_e21674, assign23030_e21674_d_n2, assign23030_e21674_d_n3, assign23030_e21674_d_n4, assign23030_e21674_d_n7, assign23030_e21674_d_n12, assign23030_e21674_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23030_e21648: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tnomin);
        let assign23030_e21649: f64 = (1.0 + assign23030_e21648);
        let assign23030_e21653: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin);
        let assign23030_e21654: f64 = (1.0 + assign23030_e21653);
        let assign23030_e21655: f64 = (assign23030_e21649 / assign23030_e21654);
        let assign23030_e21656: f64 = (locals.var_fn277_calc_iq__vel0 * assign23030_e21655);
        let assign23030_e21660: f64 = (locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin);
        let assign23030_e21662: f64 = (assign23030_e21660 / locals.var_fn277_calc_iq__lin);
        let assign23030_e21663: f64 = (1.0 + assign23030_e21662);
        let assign23030_e21664: f64 = (assign23030_e21656 * assign23030_e21663);
        let assign23030_e21668: f64 = (locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv);
        let assign23030_e21670: f64 = (assign23030_e21668 / locals.var_fn277_calc_iq__cgin);
        let assign23030_e21671: f64 = (1.0 + assign23030_e21670);
        let assign23030_e21672: f64 = (assign23030_e21664 / assign23030_e21671);
        (assign23030_e21672, (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), (((((locals.var_fn277_calc_iq__vel0 * (-((assign23030_e21649 * (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin_dn4)) / (assign23030_e21654 * assign23030_e21654)))) * assign23030_e21663) * assign23030_e21671) - (assign23030_e21664 * ((((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23030_e21668 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)))) / (assign23030_e21671 * assign23030_e21671)), (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), ((((assign23030_e21656 * ((locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin_dn12) / locals.var_fn277_calc_iq__lin)) * assign23030_e21671) - (assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin))) / (assign23030_e21671 * assign23030_e21671)), ((((assign23030_e21656 * ((locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin_dn13) / locals.var_fn277_calc_iq__lin)) * assign23030_e21671) - (assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin))) / (assign23030_e21671 * assign23030_e21671)),)
    } else {
        (locals.var_fn277_calc_iq__vx, locals.var_fn277_calc_iq__vx_dn2, locals.var_fn277_calc_iq__vx_dn3, locals.var_fn277_calc_iq__vx_dn4, locals.var_fn277_calc_iq__vx_dn7, locals.var_fn277_calc_iq__vx_dn12, locals.var_fn277_calc_iq__vx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vx = assign23030_e21674;
        locals.var_fn277_calc_iq__vx_dn2 = assign23030_e21674_d_n2;
        locals.var_fn277_calc_iq__vx_dn3 = assign23030_e21674_d_n3;
        locals.var_fn277_calc_iq__vx_dn4 = assign23030_e21674_d_n4;
        locals.var_fn277_calc_iq__vx_dn7 = assign23030_e21674_d_n7;
        locals.var_fn277_calc_iq__vx_dn12 = assign23030_e21674_d_n12;
        locals.var_fn277_calc_iq__vx_dn13 = assign23030_e21674_d_n13;

        let (assign23050_e21700, assign23050_e21700_d_n2, assign23050_e21700_d_n3, assign23050_e21700_d_n4, assign23050_e21700_d_n7, assign23050_e21700_d_n12, assign23050_e21700_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23050_e21696: f64 = (locals.var_fn277_calc_iq__vx * locals.var_fn277_calc_iq__lin);
        let assign23050_e21698: f64 = (assign23050_e21696 / locals.var_fn277_calc_iq__muf);
        (assign23050_e21698, ((((locals.var_fn277_calc_iq__vx_dn2 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn2)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn3 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn3)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn4 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn4)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn7 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn7)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn12 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn12)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn13 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn13)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)),)
    } else {
        (locals.var_fn277_calc_iq__vdsats, locals.var_fn277_calc_iq__vdsats_dn2, locals.var_fn277_calc_iq__vdsats_dn3, locals.var_fn277_calc_iq__vdsats_dn4, locals.var_fn277_calc_iq__vdsats_dn7, locals.var_fn277_calc_iq__vdsats_dn12, locals.var_fn277_calc_iq__vdsats_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats = assign23050_e21700;
        locals.var_fn277_calc_iq__vdsats_dn2 = assign23050_e21700_d_n2;
        locals.var_fn277_calc_iq__vdsats_dn3 = assign23050_e21700_d_n3;
        locals.var_fn277_calc_iq__vdsats_dn4 = assign23050_e21700_d_n4;
        locals.var_fn277_calc_iq__vdsats_dn7 = assign23050_e21700_d_n7;
        locals.var_fn277_calc_iq__vdsats_dn12 = assign23050_e21700_d_n12;
        locals.var_fn277_calc_iq__vdsats_dn13 = assign23050_e21700_d_n13;

        let (assign23060_e21717, assign23060_e21717_d_n2, assign23060_e21717_d_n3, assign23060_e21717_d_n4, assign23060_e21717_d_n7, assign23060_e21717_d_n12, assign23060_e21717_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23060_e21706: f64 = (2.0 * locals.var_fn277_calc_iq__qinvv);
        let assign23060_e21708: f64 = (assign23060_e21706 / locals.var_fn277_calc_iq__cgin);
        let assign23060_e21710: f64 = (assign23060_e21708 / locals.var_fn277_calc_iq__vdsats);
        let assign23060_e21711: f64 = (1.0 + assign23060_e21710);
        let assign23060_e21712: f64 = (assign23060_e21711).sqrt();
        let assign23060_e21713: f64 = (locals.var_fn277_calc_iq__vdsats * assign23060_e21712);
        let assign23060_e21715: f64 = (assign23060_e21713 - locals.var_fn277_calc_iq__vdsats);
        (assign23060_e21715, (((locals.var_fn277_calc_iq__vdsats_dn2 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn2)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn2), (((locals.var_fn277_calc_iq__vdsats_dn3 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn3)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn3), (((locals.var_fn277_calc_iq__vdsats_dn4 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23060_e21706 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn4)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn4), (((locals.var_fn277_calc_iq__vdsats_dn7 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn7)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn7), (((locals.var_fn277_calc_iq__vdsats_dn12 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn12)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn12), (((locals.var_fn277_calc_iq__vdsats_dn13 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn13)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vdsats1, locals.var_fn277_calc_iq__vdsats1_dn2, locals.var_fn277_calc_iq__vdsats1_dn3, locals.var_fn277_calc_iq__vdsats1_dn4, locals.var_fn277_calc_iq__vdsats1_dn7, locals.var_fn277_calc_iq__vdsats1_dn12, locals.var_fn277_calc_iq__vdsats1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats1 = assign23060_e21717;
        locals.var_fn277_calc_iq__vdsats1_dn2 = assign23060_e21717_d_n2;
        locals.var_fn277_calc_iq__vdsats1_dn3 = assign23060_e21717_d_n3;
        locals.var_fn277_calc_iq__vdsats1_dn4 = assign23060_e21717_d_n4;
        locals.var_fn277_calc_iq__vdsats1_dn7 = assign23060_e21717_d_n7;
        locals.var_fn277_calc_iq__vdsats1_dn12 = assign23060_e21717_d_n12;
        locals.var_fn277_calc_iq__vdsats1_dn13 = assign23060_e21717_d_n13;

        let (assign23070_e21729, assign23070_e21729_d_n2, assign23070_e21729_d_n3, assign23070_e21729_d_n4, assign23070_e21729_d_n7, assign23070_e21729_d_n12, assign23070_e21729_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23070_e21722: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23070_e21723: f64 = (locals.var_fn277_calc_iq__vdsats * assign23070_e21722);
        let assign23070_e21726: f64 = (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff);
        let assign23070_e21727: f64 = (assign23070_e21723 + assign23070_e21726);
        (assign23070_e21727, (((locals.var_fn277_calc_iq__vdsats_dn2 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn2))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn2)), (((locals.var_fn277_calc_iq__vdsats_dn3 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn3))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn3)), (((locals.var_fn277_calc_iq__vdsats_dn4 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit_dn4 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn4))), (((locals.var_fn277_calc_iq__vdsats_dn7 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn7))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn7)), (((locals.var_fn277_calc_iq__vdsats_dn12 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn12))) + ((locals.var_fn277_calc_iq__two_n_phit_dn12 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn12))), (((locals.var_fn277_calc_iq__vdsats_dn13 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn13))) + ((locals.var_fn277_calc_iq__two_n_phit_dn13 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vdsat, locals.var_fn277_calc_iq__vdsat_dn2, locals.var_fn277_calc_iq__vdsat_dn3, locals.var_fn277_calc_iq__vdsat_dn4, locals.var_fn277_calc_iq__vdsat_dn7, locals.var_fn277_calc_iq__vdsat_dn12, locals.var_fn277_calc_iq__vdsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat = assign23070_e21729;
        locals.var_fn277_calc_iq__vdsat_dn2 = assign23070_e21729_d_n2;
        locals.var_fn277_calc_iq__vdsat_dn3 = assign23070_e21729_d_n3;
        locals.var_fn277_calc_iq__vdsat_dn4 = assign23070_e21729_d_n4;
        locals.var_fn277_calc_iq__vdsat_dn7 = assign23070_e21729_d_n7;
        locals.var_fn277_calc_iq__vdsat_dn12 = assign23070_e21729_d_n12;
        locals.var_fn277_calc_iq__vdsat_dn13 = assign23070_e21729_d_n13;

        let (assign23080_e21741, assign23080_e21741_d_n2, assign23080_e21741_d_n3, assign23080_e21741_d_n4, assign23080_e21741_d_n7, assign23080_e21741_d_n12, assign23080_e21741_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23080_e21734: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23080_e21735: f64 = (locals.var_fn277_calc_iq__vdsats1 * assign23080_e21734);
        let assign23080_e21738: f64 = (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff);
        let assign23080_e21739: f64 = (assign23080_e21735 + assign23080_e21738);
        (assign23080_e21739, (((locals.var_fn277_calc_iq__vdsats1_dn2 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn2))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn2)), (((locals.var_fn277_calc_iq__vdsats1_dn3 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn3))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn3)), (((locals.var_fn277_calc_iq__vdsats1_dn4 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit_dn4 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn4))), (((locals.var_fn277_calc_iq__vdsats1_dn7 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn7))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn7)), (((locals.var_fn277_calc_iq__vdsats1_dn12 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn12))) + ((locals.var_fn277_calc_iq__two_n_phit_dn12 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn12))), (((locals.var_fn277_calc_iq__vdsats1_dn13 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn13))) + ((locals.var_fn277_calc_iq__two_n_phit_dn13 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vdsat1, locals.var_fn277_calc_iq__vdsat1_dn2, locals.var_fn277_calc_iq__vdsat1_dn3, locals.var_fn277_calc_iq__vdsat1_dn4, locals.var_fn277_calc_iq__vdsat1_dn7, locals.var_fn277_calc_iq__vdsat1_dn12, locals.var_fn277_calc_iq__vdsat1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat1 = assign23080_e21741;
        locals.var_fn277_calc_iq__vdsat1_dn2 = assign23080_e21741_d_n2;
        locals.var_fn277_calc_iq__vdsat1_dn3 = assign23080_e21741_d_n3;
        locals.var_fn277_calc_iq__vdsat1_dn4 = assign23080_e21741_d_n4;
        locals.var_fn277_calc_iq__vdsat1_dn7 = assign23080_e21741_d_n7;
        locals.var_fn277_calc_iq__vdsat1_dn12 = assign23080_e21741_d_n12;
        locals.var_fn277_calc_iq__vdsat1_dn13 = assign23080_e21741_d_n13;

        let (assign23090_e21810, assign23090_e21810_d_n2, assign23090_e21810_d_n3, assign23090_e21810_d_n4, assign23090_e21810_d_n7, assign23090_e21810_d_n12, assign23090_e21810_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23090_e21800, assign23090_e21800_d_n2, assign23090_e21800_d_n3, assign23090_e21800_d_n4, assign23090_e21800_d_n7, assign23090_e21800_d_n12, assign23090_e21800_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23090_e21753: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21754: f64 = assign23090_e21753;
                let assign23090_e21758: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21759: f64 = (-assign23090_e21758);
                let assign23090_e21762: f64 = (0.001 / p.p53);
                let assign23090_e21766: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21767: f64 = (-assign23090_e21766);
                let assign23090_e21768: f64 = (assign23090_e21762 * assign23090_e21767);
                let assign23090_e21769: f64 = (assign23090_e21768).tanh();
                let assign23090_e21770: f64 = (assign23090_e21759 * assign23090_e21769);
                let assign23090_e21771: f64 = (assign23090_e21754 + assign23090_e21770);
                let assign23090_e21772: f64 = (0.5 * assign23090_e21771);
                (assign23090_e21772, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))),)
            } else {
                let (assign23090_e21799, assign23090_e21799_d_n2, assign23090_e21799_d_n3, assign23090_e21799_d_n4, assign23090_e21799_d_n7, assign23090_e21799_d_n12, assign23090_e21799_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23090_e21780: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21781: f64 = assign23090_e21780;
                        let assign23090_e21785: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21786: f64 = (-assign23090_e21785);
                        let assign23090_e21790: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21791: f64 = (-assign23090_e21790);
                        let assign23090_e21792: f64 = (assign23090_e21786 * assign23090_e21791);
                        let assign23090_e21794: f64 = (assign23090_e21792 + p.p53);
                        let assign23090_e21795: f64 = (assign23090_e21794).sqrt();
                        let assign23090_e21796: f64 = (assign23090_e21781 + assign23090_e21795);
                        let assign23090_e21797: f64 = (0.5 * assign23090_e21796);
                        (assign23090_e21797, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21791) + (assign23090_e21786 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23090_e21795)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21791) + (assign23090_e21786 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23090_e21795)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23090_e21799, assign23090_e21799_d_n2, assign23090_e21799_d_n3, assign23090_e21799_d_n4, assign23090_e21799_d_n7, assign23090_e21799_d_n12, assign23090_e21799_d_n13,)
            }
        };
        let assign23090_e21802: f64 = (assign23090_e21800).powf(locals.var_fn277_calc_iq__beta);
        let assign23090_e21803: f64 = (1.0 + assign23090_e21802);
        let assign23090_e21806: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23090_e21807: f64 = (assign23090_e21803).powf(assign23090_e21806);
        let assign23090_e21808: f64 = (1.0 / assign23090_e21807);
        (assign23090_e21808, (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n2)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n2 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n2)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n2 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n3)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n3 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n3)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n3 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n4)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n4 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n4)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n4 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n7)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n7 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n7)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n7 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n12)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n12 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n12)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n12 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n13)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n13 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n13)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n13 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))),)
    } else {
        (locals.var_fn277_calc_iq__fsd, locals.var_fn277_calc_iq__fsd_dn2, locals.var_fn277_calc_iq__fsd_dn3, locals.var_fn277_calc_iq__fsd_dn4, locals.var_fn277_calc_iq__fsd_dn7, locals.var_fn277_calc_iq__fsd_dn12, locals.var_fn277_calc_iq__fsd_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd = assign23090_e21810;
        locals.var_fn277_calc_iq__fsd_dn2 = assign23090_e21810_d_n2;
        locals.var_fn277_calc_iq__fsd_dn3 = assign23090_e21810_d_n3;
        locals.var_fn277_calc_iq__fsd_dn4 = assign23090_e21810_d_n4;
        locals.var_fn277_calc_iq__fsd_dn7 = assign23090_e21810_d_n7;
        locals.var_fn277_calc_iq__fsd_dn12 = assign23090_e21810_d_n12;
        locals.var_fn277_calc_iq__fsd_dn13 = assign23090_e21810_d_n13;

        let (assign23100_e21816, assign23100_e21816_d_n2, assign23100_e21816_d_n3, assign23100_e21816_d_n4, assign23100_e21816_d_n7, assign23100_e21816_d_n12, assign23100_e21816_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23100_e21814: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd);
        (assign23100_e21814, (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn2), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn3), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn4), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn7), ((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__fsd) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn12)), ((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__fsd) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdx, locals.var_fn277_calc_iq__vdx_dn2, locals.var_fn277_calc_iq__vdx_dn3, locals.var_fn277_calc_iq__vdx_dn4, locals.var_fn277_calc_iq__vdx_dn7, locals.var_fn277_calc_iq__vdx_dn12, locals.var_fn277_calc_iq__vdx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx = assign23100_e21816;
        locals.var_fn277_calc_iq__vdx_dn2 = assign23100_e21816_d_n2;
        locals.var_fn277_calc_iq__vdx_dn3 = assign23100_e21816_d_n3;
        locals.var_fn277_calc_iq__vdx_dn4 = assign23100_e21816_d_n4;
        locals.var_fn277_calc_iq__vdx_dn7 = assign23100_e21816_d_n7;
        locals.var_fn277_calc_iq__vdx_dn12 = assign23100_e21816_d_n12;
        locals.var_fn277_calc_iq__vdx_dn13 = assign23100_e21816_d_n13;

        let (assign23110_e21891, assign23110_e21891_d_n2, assign23110_e21891_d_n3, assign23110_e21891_d_n4, assign23110_e21891_d_n7, assign23110_e21891_d_n12, assign23110_e21891_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23110_e21881, assign23110_e21881_d_n2, assign23110_e21881_d_n3, assign23110_e21881_d_n4, assign23110_e21881_d_n7, assign23110_e21881_d_n12, assign23110_e21881_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23110_e21827: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21829: f64 = (assign23110_e21827 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21830: f64 = assign23110_e21829;
                let assign23110_e21833: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21835: f64 = (assign23110_e21833 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21836: f64 = (-assign23110_e21835);
                let assign23110_e21839: f64 = (0.001 / p.p53);
                let assign23110_e21842: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21844: f64 = (assign23110_e21842 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21845: f64 = (-assign23110_e21844);
                let assign23110_e21846: f64 = (assign23110_e21839 * assign23110_e21845);
                let assign23110_e21847: f64 = (assign23110_e21846).tanh();
                let assign23110_e21848: f64 = (assign23110_e21836 * assign23110_e21847);
                let assign23110_e21849: f64 = (assign23110_e21830 + assign23110_e21848);
                let assign23110_e21850: f64 = (0.5 * assign23110_e21849);
                (assign23110_e21850, (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))),)
            } else {
                let (assign23110_e21880, assign23110_e21880_d_n2, assign23110_e21880_d_n3, assign23110_e21880_d_n4, assign23110_e21880_d_n7, assign23110_e21880_d_n12, assign23110_e21880_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23110_e21857: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21859: f64 = (assign23110_e21857 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21860: f64 = assign23110_e21859;
                        let assign23110_e21863: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21865: f64 = (assign23110_e21863 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21866: f64 = (-assign23110_e21865);
                        let assign23110_e21869: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21871: f64 = (assign23110_e21869 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21872: f64 = (-assign23110_e21871);
                        let assign23110_e21873: f64 = (assign23110_e21866 * assign23110_e21872);
                        let assign23110_e21875: f64 = (assign23110_e21873 + p.p53);
                        let assign23110_e21876: f64 = (assign23110_e21875).sqrt();
                        let assign23110_e21877: f64 = (assign23110_e21860 + assign23110_e21876);
                        let assign23110_e21878: f64 = (0.5 * assign23110_e21877);
                        (assign23110_e21878, (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21872) + (assign23110_e21866 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23110_e21876)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21872) + (assign23110_e21866 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23110_e21876)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23110_e21880, assign23110_e21880_d_n2, assign23110_e21880_d_n3, assign23110_e21880_d_n4, assign23110_e21880_d_n7, assign23110_e21880_d_n12, assign23110_e21880_d_n13,)
            }
        };
        let assign23110_e21883: f64 = (assign23110_e21881).powf(locals.var_fn277_calc_iq__beta);
        let assign23110_e21884: f64 = (1.0 + assign23110_e21883);
        let assign23110_e21887: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23110_e21888: f64 = (assign23110_e21884).powf(assign23110_e21887);
        let assign23110_e21889: f64 = (1.0 / assign23110_e21888);
        (assign23110_e21889, (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n2)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n2 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n2)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n2 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n3)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n3 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n3)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n3 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n4)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n4 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n4)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n4 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n7)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n7 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n7)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n7 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n12)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n12 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n12)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n12 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n13)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n13 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n13)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n13 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))),)
    } else {
        (locals.var_fn277_calc_iq__fds, locals.var_fn277_calc_iq__fds_dn2, locals.var_fn277_calc_iq__fds_dn3, locals.var_fn277_calc_iq__fds_dn4, locals.var_fn277_calc_iq__fds_dn7, locals.var_fn277_calc_iq__fds_dn12, locals.var_fn277_calc_iq__fds_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds = assign23110_e21891;
        locals.var_fn277_calc_iq__fds_dn2 = assign23110_e21891_d_n2;
        locals.var_fn277_calc_iq__fds_dn3 = assign23110_e21891_d_n3;
        locals.var_fn277_calc_iq__fds_dn4 = assign23110_e21891_d_n4;
        locals.var_fn277_calc_iq__fds_dn7 = assign23110_e21891_d_n7;
        locals.var_fn277_calc_iq__fds_dn12 = assign23110_e21891_d_n12;
        locals.var_fn277_calc_iq__fds_dn13 = assign23110_e21891_d_n13;

        let (assign23120_e21898, assign23120_e21898_d_n2, assign23120_e21898_d_n3, assign23120_e21898_d_n4, assign23120_e21898_d_n7, assign23120_e21898_d_n12, assign23120_e21898_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23120_e21894: f64 = (-locals.var_fn277_calc_iq__vdsin);
        let assign23120_e21896: f64 = (assign23120_e21894 * locals.var_fn277_calc_iq__fds);
        (assign23120_e21896, (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn2), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn3), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn4), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn7), (((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__fds) + (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn12)), (((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__fds) + (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vsx, locals.var_fn277_calc_iq__vsx_dn2, locals.var_fn277_calc_iq__vsx_dn3, locals.var_fn277_calc_iq__vsx_dn4, locals.var_fn277_calc_iq__vsx_dn7, locals.var_fn277_calc_iq__vsx_dn12, locals.var_fn277_calc_iq__vsx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx = assign23120_e21898;
        locals.var_fn277_calc_iq__vsx_dn2 = assign23120_e21898_d_n2;
        locals.var_fn277_calc_iq__vsx_dn3 = assign23120_e21898_d_n3;
        locals.var_fn277_calc_iq__vsx_dn4 = assign23120_e21898_d_n4;
        locals.var_fn277_calc_iq__vsx_dn7 = assign23120_e21898_d_n7;
        locals.var_fn277_calc_iq__vsx_dn12 = assign23120_e21898_d_n12;
        locals.var_fn277_calc_iq__vsx_dn13 = assign23120_e21898_d_n13;

        let (assign23130_e21906, assign23130_e21906_d_n2, assign23130_e21906_d_n3, assign23130_e21906_d_n4, assign23130_e21906_d_n7, assign23130_e21906_d_n12, assign23130_e21906_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23130_e21902: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__myarg);
        let assign23130_e21904: f64 = (assign23130_e21902 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23130_e21904, ((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23130_e21902 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign23130_e21906;
        locals.var_fn277_calc_iq__exparg_dn2 = assign23130_e21906_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign23130_e21906_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign23130_e21906_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign23130_e21906_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign23130_e21906_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign23130_e21906_d_n13;

        let assign23140_e21909: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign23140_e21909;

        let (assign23150_e21915, assign23150_e21915_d_n2, assign23150_e21915_d_n3, assign23150_e21915_d_n4, assign23150_e21915_d_n7, assign23150_e21915_d_n12, assign23150_e21915_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard283 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23150_e21915;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23150_e21915_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23150_e21915_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23150_e21915_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23150_e21915_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23150_e21915_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23150_e21915_d_n13;

        let assign23160_e21918: f64 = (-50.0);
        let assign23160_e21919: f64 = if locals.var_fn277_calc_iq__exparg < assign23160_e21918 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign23160_e21919;

        let (assign23170_e21928, assign23170_e21928_d_n2, assign23170_e21928_d_n3, assign23170_e21928_d_n4, assign23170_e21928_d_n7, assign23170_e21928_d_n12, assign23170_e21928_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard283 == 0.0)) && (locals.var_guard284 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23170_e21928;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23170_e21928_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23170_e21928_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23170_e21928_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23170_e21928_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23170_e21928_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23170_e21928_d_n13;

        let (assign23180_e21943, assign23180_e21943_d_n2, assign23180_e21943_d_n3, assign23180_e21943_d_n4, assign23180_e21943_d_n7, assign23180_e21943_d_n12, assign23180_e21943_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard283 == 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign23180_e21939: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign23180_e21940: f64 = (1.0 + assign23180_e21939);
        let assign23180_e21941: f64 = (1.0 / assign23180_e21940);
        (assign23180_e21941, (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn2) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn3) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn4) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn7) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn12) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn13) / (assign23180_e21940 * assign23180_e21940))),)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23180_e21943;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23180_e21943_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23180_e21943_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23180_e21943_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23180_e21943_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23180_e21943_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23180_e21943_d_n13;

        let (assign23190_e21961, assign23190_e21961_d_n2, assign23190_e21961_d_n3, assign23190_e21961_d_n4, assign23190_e21961_d_n7, assign23190_e21961_d_n12, assign23190_e21961_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23190_e21947: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__vsx);
        let assign23190_e21951: f64 = (p.p51 * 0.1);
        let assign23190_e21953: f64 = (assign23190_e21951 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23190_e21955: f64 = (assign23190_e21953 * locals.var_fn277_calc_iq__ffs);
        let assign23190_e21956: f64 = (locals.var_fn277_calc_iq__vtdibl - assign23190_e21955);
        let assign23190_e21957: f64 = (assign23190_e21947 - assign23190_e21956);
        let assign23190_e21959: f64 = (assign23190_e21957 / locals.var_fn277_calc_iq__two_n_phit);
        (assign23190_e21959, (((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__vsx_dn2) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn2))) / locals.var_fn277_calc_iq__two_n_phit), (((-locals.var_fn277_calc_iq__vsx_dn3) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn3))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vsx_dn4) - (locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign23190_e21951 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffs) + (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__vsx_dn7) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn7))) / locals.var_fn277_calc_iq__two_n_phit), (((((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__vsx_dn12) - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__vsx_dn13) - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__etas, locals.var_fn277_calc_iq__etas_dn2, locals.var_fn277_calc_iq__etas_dn3, locals.var_fn277_calc_iq__etas_dn4, locals.var_fn277_calc_iq__etas_dn7, locals.var_fn277_calc_iq__etas_dn12, locals.var_fn277_calc_iq__etas_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas = assign23190_e21961;
        locals.var_fn277_calc_iq__etas_dn2 = assign23190_e21961_d_n2;
        locals.var_fn277_calc_iq__etas_dn3 = assign23190_e21961_d_n3;
        locals.var_fn277_calc_iq__etas_dn4 = assign23190_e21961_d_n4;
        locals.var_fn277_calc_iq__etas_dn7 = assign23190_e21961_d_n7;
        locals.var_fn277_calc_iq__etas_dn12 = assign23190_e21961_d_n12;
        locals.var_fn277_calc_iq__etas_dn13 = assign23190_e21961_d_n13;

        let assign23200_e21964: f64 = if locals.var_fn277_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign23200_e21964;

        let (assign23210_e21972, assign23210_e21972_d_n2, assign23210_e21972_d_n3, assign23210_e21972_d_n4, assign23210_e21972_d_n7, assign23210_e21972_d_n12, assign23210_e21972_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard285 != 0.0)) {
        let assign23210_e21970: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas);
        (assign23210_e21970, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23210_e21972;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23210_e21972_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23210_e21972_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23210_e21972_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23210_e21972_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23210_e21972_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23210_e21972_d_n13;

        let assign23220_e21975: f64 = (-50.0);
        let assign23220_e21976: f64 = if locals.var_fn277_calc_iq__etas < assign23220_e21975 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign23220_e21976;

    }

    pub(super) fn stamp_transient_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23230_e21988, assign23230_e21988_d_n2, assign23230_e21988_d_n3, assign23230_e21988_d_n4, assign23230_e21988_d_n7, assign23230_e21988_d_n12, assign23230_e21988_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard285 == 0.0)) && (locals.var_guard286 != 0.0)) {
        let assign23230_e21985: f64 = (locals.var_fn277_calc_iq__etas).exp();
        let assign23230_e21986: f64 = (locals.var_fn277_calc_iq__qref * assign23230_e21985);
        (assign23230_e21986, (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn2)), (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn4))), (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23230_e21988;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23230_e21988_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23230_e21988_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23230_e21988_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23230_e21988_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23230_e21988_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23230_e21988_d_n13;

        let (assign23240_e22004, assign23240_e22004_d_n2, assign23240_e22004_d_n3, assign23240_e22004_d_n4, assign23240_e22004_d_n7, assign23240_e22004_d_n12, assign23240_e22004_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard285 == 0.0)) && (locals.var_guard286 == 0.0)) {
        let assign23240_e21999: f64 = (locals.var_fn277_calc_iq__etas).exp();
        let assign23240_e22000: f64 = (1.0 + assign23240_e21999);
        let assign23240_e22001: f64 = (assign23240_e22000).ln();
        let assign23240_e22002: f64 = (locals.var_fn277_calc_iq__qref * assign23240_e22001);
        (assign23240_e22002, (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn2) / assign23240_e22000)), (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn3) / assign23240_e22000)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn4) / assign23240_e22000))), (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn7) / assign23240_e22000)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn12) / assign23240_e22000))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn13) / assign23240_e22000))),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23240_e22004;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23240_e22004_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23240_e22004_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23240_e22004_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23240_e22004_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23240_e22004_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23240_e22004_d_n13;

        let (assign23250_e22012, assign23250_e22012_d_n2, assign23250_e22012_d_n3, assign23250_e22012_d_n4, assign23250_e22012_d_n7, assign23250_e22012_d_n12, assign23250_e22012_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23250_e22008: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__myarg);
        let assign23250_e22010: f64 = (assign23250_e22008 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23250_e22010, ((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23250_e22008 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign23250_e22012;
        locals.var_fn277_calc_iq__exparg_dn2 = assign23250_e22012_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign23250_e22012_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign23250_e22012_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign23250_e22012_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign23250_e22012_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign23250_e22012_d_n13;

        let assign23260_e22015: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign23260_e22015;

        let (assign23270_e22021, assign23270_e22021_d_n2, assign23270_e22021_d_n3, assign23270_e22021_d_n4, assign23270_e22021_d_n7, assign23270_e22021_d_n12, assign23270_e22021_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23270_e22021;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23270_e22021_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23270_e22021_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23270_e22021_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23270_e22021_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23270_e22021_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23270_e22021_d_n13;

        let assign23280_e22024: f64 = (-50.0);
        let assign23280_e22025: f64 = if locals.var_fn277_calc_iq__exparg < assign23280_e22024 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign23280_e22025;

        let (assign23290_e22034, assign23290_e22034_d_n2, assign23290_e22034_d_n3, assign23290_e22034_d_n4, assign23290_e22034_d_n7, assign23290_e22034_d_n12, assign23290_e22034_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23290_e22034;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23290_e22034_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23290_e22034_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23290_e22034_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23290_e22034_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23290_e22034_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23290_e22034_d_n13;

        let (assign23300_e22049, assign23300_e22049_d_n2, assign23300_e22049_d_n3, assign23300_e22049_d_n4, assign23300_e22049_d_n7, assign23300_e22049_d_n12, assign23300_e22049_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign23300_e22045: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign23300_e22046: f64 = (1.0 + assign23300_e22045);
        let assign23300_e22047: f64 = (1.0 / assign23300_e22046);
        (assign23300_e22047, (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn2) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn3) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn4) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn7) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn12) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn13) / (assign23300_e22046 * assign23300_e22046))),)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23300_e22049;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23300_e22049_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23300_e22049_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23300_e22049_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23300_e22049_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23300_e22049_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23300_e22049_d_n13;

        let (assign23310_e22067, assign23310_e22067_d_n2, assign23310_e22067_d_n3, assign23310_e22067_d_n4, assign23310_e22067_d_n7, assign23310_e22067_d_n12, assign23310_e22067_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23310_e22053: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdx);
        let assign23310_e22057: f64 = (p.p51 * 0.1);
        let assign23310_e22059: f64 = (assign23310_e22057 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23310_e22061: f64 = (assign23310_e22059 * locals.var_fn277_calc_iq__ffd);
        let assign23310_e22062: f64 = (locals.var_fn277_calc_iq__vtdibl - assign23310_e22061);
        let assign23310_e22063: f64 = (assign23310_e22053 - assign23310_e22062);
        let assign23310_e22065: f64 = (assign23310_e22063 / locals.var_fn277_calc_iq__two_n_phit);
        (assign23310_e22065, (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vdx_dn2) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn2))) / locals.var_fn277_calc_iq__two_n_phit), (((-locals.var_fn277_calc_iq__vdx_dn3) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn3))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vdx_dn4) - (locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign23310_e22057 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffd) + (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vdx_dn7) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn7))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vdx_dn12) - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdx_dn13) - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__etad, locals.var_fn277_calc_iq__etad_dn2, locals.var_fn277_calc_iq__etad_dn3, locals.var_fn277_calc_iq__etad_dn4, locals.var_fn277_calc_iq__etad_dn7, locals.var_fn277_calc_iq__etad_dn12, locals.var_fn277_calc_iq__etad_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad = assign23310_e22067;
        locals.var_fn277_calc_iq__etad_dn2 = assign23310_e22067_d_n2;
        locals.var_fn277_calc_iq__etad_dn3 = assign23310_e22067_d_n3;
        locals.var_fn277_calc_iq__etad_dn4 = assign23310_e22067_d_n4;
        locals.var_fn277_calc_iq__etad_dn7 = assign23310_e22067_d_n7;
        locals.var_fn277_calc_iq__etad_dn12 = assign23310_e22067_d_n12;
        locals.var_fn277_calc_iq__etad_dn13 = assign23310_e22067_d_n13;

        let assign23320_e22070: f64 = if locals.var_fn277_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign23320_e22070;

        let (assign23330_e22078, assign23330_e22078_d_n2, assign23330_e22078_d_n3, assign23330_e22078_d_n4, assign23330_e22078_d_n7, assign23330_e22078_d_n12, assign23330_e22078_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard289 != 0.0)) {
        let assign23330_e22076: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad);
        (assign23330_e22076, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23330_e22078;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23330_e22078_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23330_e22078_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23330_e22078_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23330_e22078_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23330_e22078_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23330_e22078_d_n13;

        let assign23340_e22081: f64 = (-50.0);
        let assign23340_e22082: f64 = if locals.var_fn277_calc_iq__etad < assign23340_e22081 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign23340_e22082;

        let (assign23350_e22094, assign23350_e22094_d_n2, assign23350_e22094_d_n3, assign23350_e22094_d_n4, assign23350_e22094_d_n7, assign23350_e22094_d_n12, assign23350_e22094_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard289 == 0.0)) && (locals.var_guard290 != 0.0)) {
        let assign23350_e22091: f64 = (locals.var_fn277_calc_iq__etad).exp();
        let assign23350_e22092: f64 = (locals.var_fn277_calc_iq__qref * assign23350_e22091);
        (assign23350_e22092, (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn2)), (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn4))), (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23350_e22094;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23350_e22094_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23350_e22094_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23350_e22094_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23350_e22094_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23350_e22094_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23350_e22094_d_n13;

        let (assign23360_e22110, assign23360_e22110_d_n2, assign23360_e22110_d_n3, assign23360_e22110_d_n4, assign23360_e22110_d_n7, assign23360_e22110_d_n12, assign23360_e22110_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard289 == 0.0)) && (locals.var_guard290 == 0.0)) {
        let assign23360_e22105: f64 = (locals.var_fn277_calc_iq__etad).exp();
        let assign23360_e22106: f64 = (1.0 + assign23360_e22105);
        let assign23360_e22107: f64 = (assign23360_e22106).ln();
        let assign23360_e22108: f64 = (locals.var_fn277_calc_iq__qref * assign23360_e22107);
        (assign23360_e22108, (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn2) / assign23360_e22106)), (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn3) / assign23360_e22106)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23360_e22107) + (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn4) / assign23360_e22106))), (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn7) / assign23360_e22106)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23360_e22107) + (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn12) / assign23360_e22106))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23360_e22107) + (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn13) / assign23360_e22106))),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23360_e22110;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23360_e22110_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23360_e22110_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23360_e22110_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23360_e22110_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23360_e22110_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23360_e22110_d_n13;

        let (assign23370_e22118, assign23370_e22118_d_n2, assign23370_e22118_d_n3, assign23370_e22118_d_n4, assign23370_e22118_d_n7, assign23370_e22118_d_n12, assign23370_e22118_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23370_e22114: f64 = (locals.var_fn277_calc_iq__qinvs - locals.var_fn277_calc_iq__qinvd);
        let assign23370_e22116: f64 = (assign23370_e22114 / locals.var_fn277_calc_iq__cgin);
        (assign23370_e22116, ((locals.var_fn277_calc_iq__qinvs_dn2 - locals.var_fn277_calc_iq__qinvd_dn2) / locals.var_fn277_calc_iq__cgin), ((locals.var_fn277_calc_iq__qinvs_dn3 - locals.var_fn277_calc_iq__qinvd_dn3) / locals.var_fn277_calc_iq__cgin), ((((locals.var_fn277_calc_iq__qinvs_dn4 - locals.var_fn277_calc_iq__qinvd_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23370_e22114 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)), ((locals.var_fn277_calc_iq__qinvs_dn7 - locals.var_fn277_calc_iq__qinvd_dn7) / locals.var_fn277_calc_iq__cgin), ((locals.var_fn277_calc_iq__qinvs_dn12 - locals.var_fn277_calc_iq__qinvd_dn12) / locals.var_fn277_calc_iq__cgin), ((locals.var_fn277_calc_iq__qinvs_dn13 - locals.var_fn277_calc_iq__qinvd_dn13) / locals.var_fn277_calc_iq__cgin),)
    } else {
        (locals.var_fn277_calc_iq__vdsc, locals.var_fn277_calc_iq__vdsc_dn2, locals.var_fn277_calc_iq__vdsc_dn3, locals.var_fn277_calc_iq__vdsc_dn4, locals.var_fn277_calc_iq__vdsc_dn7, locals.var_fn277_calc_iq__vdsc_dn12, locals.var_fn277_calc_iq__vdsc_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsc = assign23370_e22118;
        locals.var_fn277_calc_iq__vdsc_dn2 = assign23370_e22118_d_n2;
        locals.var_fn277_calc_iq__vdsc_dn3 = assign23370_e22118_d_n3;
        locals.var_fn277_calc_iq__vdsc_dn4 = assign23370_e22118_d_n4;
        locals.var_fn277_calc_iq__vdsc_dn7 = assign23370_e22118_d_n7;
        locals.var_fn277_calc_iq__vdsc_dn12 = assign23370_e22118_d_n12;
        locals.var_fn277_calc_iq__vdsc_dn13 = assign23370_e22118_d_n13;

        let (assign23380_e22124, assign23380_e22124_d_n2, assign23380_e22124_d_n3, assign23380_e22124_d_n4, assign23380_e22124_d_n7, assign23380_e22124_d_n12, assign23380_e22124_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23380_e22122: f64 = (locals.var_fn277_calc_iq__vdsc / locals.var_fn277_calc_iq__vdsat);
        (assign23380_e22122, (((locals.var_fn277_calc_iq__vdsc_dn2 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn2)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn3 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn3)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn4 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn4)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn7 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn7)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn12 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn12)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn13 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn13)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)),)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign23380_e22124;
        locals.var_fn277_calc_iq__myarg_dn2 = assign23380_e22124_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign23380_e22124_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign23380_e22124_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign23380_e22124_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign23380_e22124_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign23380_e22124_d_n13;

        let (assign23420_e22193, assign23420_e22193_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23420_e22190: f64 = (2.302585092994046 * locals.var_fn277_calc_iq__phitin);
        let assign23420_e22191: f64 = (locals.var_fn277_calc_iq__ss / assign23420_e22190);
        (assign23420_e22191, (-((locals.var_fn277_calc_iq__ss * (2.302585092994046 * locals.var_fn277_calc_iq__phitin_dn4)) / (assign23420_e22190 * assign23420_e22190))),)
    } else {
        (locals.var_fn277_calc_iq__n0, locals.var_fn277_calc_iq__n0_dn4,)
    }
};
        locals.var_fn277_calc_iq__n0 = assign23420_e22193;
        locals.var_fn277_calc_iq__n0_dn4 = assign23420_e22193_d_n4;

        let (assign23430_e22201, assign23430_e22201_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23430_e22197: f64 = (2.0 * locals.var_fn277_calc_iq__n0);
        let assign23430_e22199: f64 = (assign23430_e22197 * locals.var_fn277_calc_iq__phitin);
        (assign23430_e22199, (((2.0 * locals.var_fn277_calc_iq__n0_dn4) * locals.var_fn277_calc_iq__phitin) + (assign23430_e22197 * locals.var_fn277_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit0, locals.var_fn277_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit0 = assign23430_e22201;
        locals.var_fn277_calc_iq__two_n_phit0_dn4 = assign23430_e22201_d_n4;

        let (assign23440_e22207, assign23440_e22207_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23440_e22205: f64 = (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit0);
        (assign23440_e22205, ((locals.var_fn277_calc_iq__cgin_dn4 * locals.var_fn277_calc_iq__two_n_phit0) + (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn277_calc_iq__qref0, locals.var_fn277_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn277_calc_iq__qref0 = assign23440_e22207;
        locals.var_fn277_calc_iq__qref0_dn4 = assign23440_e22207_d_n4;

        let (assign23450_e22217, assign23450_e22217_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23450_e22212: f64 = (p.p51 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23450_e22214: f64 = (assign23450_e22212 / 2.0);
        let assign23450_e22215: f64 = (locals.var_fn277_calc_iq__vtof - assign23450_e22214);
        (assign23450_e22215, (locals.var_fn277_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn277_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn277_calc_iq__myarg0, locals.var_fn277_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn277_calc_iq__myarg0 = assign23450_e22217;
        locals.var_fn277_calc_iq__myarg0_dn4 = assign23450_e22217_d_n4;

        let (assign23460_e22268, assign23460_e22268_d_n2, assign23460_e22268_d_n4, assign23460_e22268_d_n7, assign23460_e22268_d_n12, assign23460_e22268_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23460_e22262, assign23460_e22262_d_n2, assign23460_e22262_d_n7, assign23460_e22262_d_n12, assign23460_e22262_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23460_e22226: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign23460_e22229: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23460_e22232: f64 = (0.001 / p.p53);
                let assign23460_e22235: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23460_e22236: f64 = (assign23460_e22232 * assign23460_e22235);
                let assign23460_e22237: f64 = (assign23460_e22236).tanh();
                let assign23460_e22238: f64 = (assign23460_e22229 * assign23460_e22237);
                let assign23460_e22239: f64 = (assign23460_e22226 + assign23460_e22238);
                let assign23460_e22240: f64 = (0.5 * assign23460_e22239);
                (assign23460_e22240, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))),)
            } else {
                let (assign23460_e22261, assign23460_e22261_d_n2, assign23460_e22261_d_n7, assign23460_e22261_d_n12, assign23460_e22261_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23460_e22247: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign23460_e22250: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23460_e22253: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23460_e22254: f64 = (assign23460_e22250 * assign23460_e22253);
                        let assign23460_e22256: f64 = (assign23460_e22254 + p.p53);
                        let assign23460_e22257: f64 = (assign23460_e22256).sqrt();
                        let assign23460_e22258: f64 = (assign23460_e22247 + assign23460_e22257);
                        let assign23460_e22259: f64 = (0.5 * assign23460_e22258);
                        (assign23460_e22259, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23460_e22253) + (assign23460_e22250 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign23460_e22257)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23460_e22253) + (assign23460_e22250 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign23460_e22257)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23460_e22253) + (assign23460_e22250 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign23460_e22257)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23460_e22253) + (assign23460_e22250 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign23460_e22257)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23460_e22261, assign23460_e22261_d_n2, assign23460_e22261_d_n7, assign23460_e22261_d_n12, assign23460_e22261_d_n13,)
            }
        };
        let assign23460_e22264: f64 = (assign23460_e22262 - locals.var_fn277_calc_iq__myarg0);
        let assign23460_e22266: f64 = (assign23460_e22264 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23460_e22266, (assign23460_e22262_d_n2 / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg0_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23460_e22264 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), (assign23460_e22262_d_n7 / locals.var_fn277_calc_iq__alpha_phit), (assign23460_e22262_d_n12 / locals.var_fn277_calc_iq__alpha_phit), (assign23460_e22262_d_n13 / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign23460_e22268;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign23460_e22268_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign23460_e22268_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign23460_e22268_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign23460_e22268_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign23460_e22268_d_n13;

        let assign23470_e22271: f64 = if locals.var_fn277_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign23470_e22271;

        let (assign23480_e22277, assign23480_e22277_d_n2, assign23480_e22277_d_n4, assign23480_e22277_d_n7, assign23480_e22277_d_n12, assign23480_e22277_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard291 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign23480_e22277;
        locals.var_fn277_calc_iq__ff0_dn2 = assign23480_e22277_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign23480_e22277_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign23480_e22277_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign23480_e22277_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign23480_e22277_d_n13;

        let assign23490_e22280: f64 = (-50.0);
        let assign23490_e22281: f64 = if locals.var_fn277_calc_iq__exparg0 < assign23490_e22280 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign23490_e22281;

        let (assign23500_e22290, assign23500_e22290_d_n2, assign23500_e22290_d_n4, assign23500_e22290_d_n7, assign23500_e22290_d_n12, assign23500_e22290_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard291 == 0.0)) && (locals.var_guard292 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign23500_e22290;
        locals.var_fn277_calc_iq__ff0_dn2 = assign23500_e22290_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign23500_e22290_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign23500_e22290_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign23500_e22290_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign23500_e22290_d_n13;

        let (assign23510_e22305, assign23510_e22305_d_n2, assign23510_e22305_d_n4, assign23510_e22305_d_n7, assign23510_e22305_d_n12, assign23510_e22305_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard291 == 0.0)) && (locals.var_guard292 == 0.0)) {
        let assign23510_e22301: f64 = (locals.var_fn277_calc_iq__exparg0).exp();
        let assign23510_e22302: f64 = (1.0 + assign23510_e22301);
        let assign23510_e22303: f64 = (1.0 / assign23510_e22302);
        (assign23510_e22303, (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn2) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn4) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn7) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn12) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn13) / (assign23510_e22302 * assign23510_e22302))),)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign23510_e22305;
        locals.var_fn277_calc_iq__ff0_dn2 = assign23510_e22305_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign23510_e22305_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign23510_e22305_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign23510_e22305_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign23510_e22305_d_n13;

        let (assign23520_e22364, assign23520_e22364_d_n2, assign23520_e22364_d_n4, assign23520_e22364_d_n7, assign23520_e22364_d_n12, assign23520_e22364_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23520_e22350, assign23520_e22350_d_n2, assign23520_e22350_d_n7, assign23520_e22350_d_n12, assign23520_e22350_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23520_e22314: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign23520_e22317: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23520_e22320: f64 = (0.001 / p.p53);
                let assign23520_e22323: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23520_e22324: f64 = (assign23520_e22320 * assign23520_e22323);
                let assign23520_e22325: f64 = (assign23520_e22324).tanh();
                let assign23520_e22326: f64 = (assign23520_e22317 * assign23520_e22325);
                let assign23520_e22327: f64 = (assign23520_e22314 + assign23520_e22326);
                let assign23520_e22328: f64 = (0.5 * assign23520_e22327);
                (assign23520_e22328, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))),)
            } else {
                let (assign23520_e22349, assign23520_e22349_d_n2, assign23520_e22349_d_n7, assign23520_e22349_d_n12, assign23520_e22349_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23520_e22335: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign23520_e22338: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23520_e22341: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23520_e22342: f64 = (assign23520_e22338 * assign23520_e22341);
                        let assign23520_e22344: f64 = (assign23520_e22342 + p.p53);
                        let assign23520_e22345: f64 = (assign23520_e22344).sqrt();
                        let assign23520_e22346: f64 = (assign23520_e22335 + assign23520_e22345);
                        let assign23520_e22347: f64 = (0.5 * assign23520_e22346);
                        (assign23520_e22347, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23520_e22341) + (assign23520_e22338 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign23520_e22345)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23520_e22341) + (assign23520_e22338 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign23520_e22345)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23520_e22341) + (assign23520_e22338 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign23520_e22345)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23520_e22341) + (assign23520_e22338 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign23520_e22345)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23520_e22349, assign23520_e22349_d_n2, assign23520_e22349_d_n7, assign23520_e22349_d_n12, assign23520_e22349_d_n13,)
            }
        };
        let assign23520_e22354: f64 = (p.p51 * 0.1);
        let assign23520_e22356: f64 = (assign23520_e22354 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23520_e22358: f64 = (assign23520_e22356 * locals.var_fn277_calc_iq__ff0);
        let assign23520_e22359: f64 = (locals.var_fn277_calc_iq__vtof - assign23520_e22358);
        let assign23520_e22360: f64 = (assign23520_e22350 - assign23520_e22359);
        let assign23520_e22362: f64 = (assign23520_e22360 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign23520_e22362, ((assign23520_e22350_d_n2 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn2))) / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (((assign23520_e22354 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ff0) + (assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn4)))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign23520_e22360 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), ((assign23520_e22350_d_n7 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn7))) / locals.var_fn277_calc_iq__two_n_phit0), ((assign23520_e22350_d_n12 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn12))) / locals.var_fn277_calc_iq__two_n_phit0), ((assign23520_e22350_d_n13 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn13))) / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__eta0, locals.var_fn277_calc_iq__eta0_dn2, locals.var_fn277_calc_iq__eta0_dn4, locals.var_fn277_calc_iq__eta0_dn7, locals.var_fn277_calc_iq__eta0_dn12, locals.var_fn277_calc_iq__eta0_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta0 = assign23520_e22364;
        locals.var_fn277_calc_iq__eta0_dn2 = assign23520_e22364_d_n2;
        locals.var_fn277_calc_iq__eta0_dn4 = assign23520_e22364_d_n4;
        locals.var_fn277_calc_iq__eta0_dn7 = assign23520_e22364_d_n7;
        locals.var_fn277_calc_iq__eta0_dn12 = assign23520_e22364_d_n12;
        locals.var_fn277_calc_iq__eta0_dn13 = assign23520_e22364_d_n13;

        let assign23530_e22367: f64 = if locals.var_fn277_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign23530_e22367;

        let (assign23540_e22375, assign23540_e22375_d_n2, assign23540_e22375_d_n4, assign23540_e22375_d_n7, assign23540_e22375_d_n12, assign23540_e22375_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard293 != 0.0)) {
        let assign23540_e22373: f64 = (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0);
        (assign23540_e22373, (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn2), ((locals.var_fn277_calc_iq__qref0_dn4 * locals.var_fn277_calc_iq__eta0) + (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn4)), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn7), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn12), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign23540_e22375;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign23540_e22375_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign23540_e22375_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign23540_e22375_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign23540_e22375_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign23540_e22375_d_n13;

        let assign23550_e22378: f64 = (-50.0);
        let assign23550_e22379: f64 = if locals.var_fn277_calc_iq__eta0 < assign23550_e22378 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign23550_e22379;

        let (assign23560_e22391, assign23560_e22391_d_n2, assign23560_e22391_d_n4, assign23560_e22391_d_n7, assign23560_e22391_d_n12, assign23560_e22391_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard293 == 0.0)) && (locals.var_guard294 != 0.0)) {
        let assign23560_e22388: f64 = (locals.var_fn277_calc_iq__eta0).exp();
        let assign23560_e22389: f64 = (locals.var_fn277_calc_iq__qref0 * assign23560_e22388);
        (assign23560_e22389, (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn2)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23560_e22388) + (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn4))), (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn7)), (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn12)), (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign23560_e22391;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign23560_e22391_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign23560_e22391_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign23560_e22391_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign23560_e22391_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign23560_e22391_d_n13;

        let (assign23570_e22407, assign23570_e22407_d_n2, assign23570_e22407_d_n4, assign23570_e22407_d_n7, assign23570_e22407_d_n12, assign23570_e22407_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard293 == 0.0)) && (locals.var_guard294 == 0.0)) {
        let assign23570_e22402: f64 = (locals.var_fn277_calc_iq__eta0).exp();
        let assign23570_e22403: f64 = (1.0 + assign23570_e22402);
        let assign23570_e22404: f64 = (assign23570_e22403).ln();
        let assign23570_e22405: f64 = (locals.var_fn277_calc_iq__qref0 * assign23570_e22404);
        (assign23570_e22405, (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn2) / assign23570_e22403)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23570_e22404) + (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn4) / assign23570_e22403))), (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn7) / assign23570_e22403)), (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn12) / assign23570_e22403)), (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn13) / assign23570_e22403)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign23570_e22407;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign23570_e22407_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign23570_e22407_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign23570_e22407_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign23570_e22407_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign23570_e22407_d_n13;

        let (assign23580_e22413, assign23580_e22413_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23580_e22411: f64 = (locals.var_fn277_calc_iq__mu0 / locals.var_fn277_calc_iq__tfacmobin);
        (assign23580_e22411, (-((locals.var_fn277_calc_iq__mu0 * locals.var_fn277_calc_iq__tfacmobin_dn4) / (locals.var_fn277_calc_iq__tfacmobin * locals.var_fn277_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn277_calc_iq__muf0, locals.var_fn277_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn277_calc_iq__muf0 = assign23580_e22413;
        locals.var_fn277_calc_iq__muf0_dn4 = assign23580_e22413_d_n4;

        let (assign23590_e22429, assign23590_e22429_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23590_e22419: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tnomin);
        let assign23590_e22420: f64 = (1.0 + assign23590_e22419);
        let assign23590_e22424: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin);
        let assign23590_e22425: f64 = (1.0 + assign23590_e22424);
        let assign23590_e22426: f64 = (assign23590_e22420 / assign23590_e22425);
        let assign23590_e22427: f64 = (locals.var_fn277_calc_iq__vel0 * assign23590_e22426);
        (assign23590_e22427, (locals.var_fn277_calc_iq__vel0 * (-((assign23590_e22420 * (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin_dn4)) / (assign23590_e22425 * assign23590_e22425)))),)
    } else {
        (locals.var_fn277_calc_iq__vx0, locals.var_fn277_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vx0 = assign23590_e22429;
        locals.var_fn277_calc_iq__vx0_dn4 = assign23590_e22429_d_n4;

    }

    pub(super) fn stamp_transient_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23600_e22437, assign23600_e22437_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23600_e22433: f64 = (locals.var_fn277_calc_iq__vx0 * locals.var_fn277_calc_iq__lin);
        let assign23600_e22435: f64 = (assign23600_e22433 / locals.var_fn277_calc_iq__muf0);
        (assign23600_e22435, ((((locals.var_fn277_calc_iq__vx0_dn4 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf0) - (assign23600_e22433 * locals.var_fn277_calc_iq__muf0_dn4)) / (locals.var_fn277_calc_iq__muf0 * locals.var_fn277_calc_iq__muf0)),)
    } else {
        (locals.var_fn277_calc_iq__vdsats0, locals.var_fn277_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vdsats0 = assign23600_e22437;
        locals.var_fn277_calc_iq__vdsats0_dn4 = assign23600_e22437_d_n4;

        let (assign23610_e22454, assign23610_e22454_d_n2, assign23610_e22454_d_n4, assign23610_e22454_d_n7, assign23610_e22454_d_n12, assign23610_e22454_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23610_e22443: f64 = (2.0 * locals.var_fn277_calc_iq__qinvv0);
        let assign23610_e22445: f64 = (assign23610_e22443 / locals.var_fn277_calc_iq__cgin);
        let assign23610_e22447: f64 = (assign23610_e22445 / locals.var_fn277_calc_iq__vdsats0);
        let assign23610_e22448: f64 = (1.0 + assign23610_e22447);
        let assign23610_e22449: f64 = (assign23610_e22448).sqrt();
        let assign23610_e22450: f64 = (locals.var_fn277_calc_iq__vdsats0 * assign23610_e22449);
        let assign23610_e22452: f64 = (assign23610_e22450 - locals.var_fn277_calc_iq__vdsats0);
        (assign23610_e22452, (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn2) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))), (((locals.var_fn277_calc_iq__vdsats0_dn4 * assign23610_e22449) + (locals.var_fn277_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23610_e22443 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)) * locals.var_fn277_calc_iq__vdsats0) - (assign23610_e22445 * locals.var_fn277_calc_iq__vdsats0_dn4)) / (locals.var_fn277_calc_iq__vdsats0 * locals.var_fn277_calc_iq__vdsats0)) / (2.0 * assign23610_e22449)))) - locals.var_fn277_calc_iq__vdsats0_dn4), (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn7) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))), (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn12) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))), (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn13) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))),)
    } else {
        (locals.var_fn277_calc_iq__vdsats10, locals.var_fn277_calc_iq__vdsats10_dn2, locals.var_fn277_calc_iq__vdsats10_dn4, locals.var_fn277_calc_iq__vdsats10_dn7, locals.var_fn277_calc_iq__vdsats10_dn12, locals.var_fn277_calc_iq__vdsats10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats10 = assign23610_e22454;
        locals.var_fn277_calc_iq__vdsats10_dn2 = assign23610_e22454_d_n2;
        locals.var_fn277_calc_iq__vdsats10_dn4 = assign23610_e22454_d_n4;
        locals.var_fn277_calc_iq__vdsats10_dn7 = assign23610_e22454_d_n7;
        locals.var_fn277_calc_iq__vdsats10_dn12 = assign23610_e22454_d_n12;
        locals.var_fn277_calc_iq__vdsats10_dn13 = assign23610_e22454_d_n13;

        let (assign23620_e22466, assign23620_e22466_d_n2, assign23620_e22466_d_n4, assign23620_e22466_d_n7, assign23620_e22466_d_n12, assign23620_e22466_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23620_e22459: f64 = (1.0 - locals.var_fn277_calc_iq__ff0);
        let assign23620_e22460: f64 = (locals.var_fn277_calc_iq__vdsats10 * assign23620_e22459);
        let assign23620_e22463: f64 = (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0);
        let assign23620_e22464: f64 = (assign23620_e22460 + assign23620_e22463);
        (assign23620_e22464, (((locals.var_fn277_calc_iq__vdsats10_dn2 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn2))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn2)), (((locals.var_fn277_calc_iq__vdsats10_dn4 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit0_dn4 * locals.var_fn277_calc_iq__ff0) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn4))), (((locals.var_fn277_calc_iq__vdsats10_dn7 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn7))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn7)), (((locals.var_fn277_calc_iq__vdsats10_dn12 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn12))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn12)), (((locals.var_fn277_calc_iq__vdsats10_dn13 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn13))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdsat10, locals.var_fn277_calc_iq__vdsat10_dn2, locals.var_fn277_calc_iq__vdsat10_dn4, locals.var_fn277_calc_iq__vdsat10_dn7, locals.var_fn277_calc_iq__vdsat10_dn12, locals.var_fn277_calc_iq__vdsat10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat10 = assign23620_e22466;
        locals.var_fn277_calc_iq__vdsat10_dn2 = assign23620_e22466_d_n2;
        locals.var_fn277_calc_iq__vdsat10_dn4 = assign23620_e22466_d_n4;
        locals.var_fn277_calc_iq__vdsat10_dn7 = assign23620_e22466_d_n7;
        locals.var_fn277_calc_iq__vdsat10_dn12 = assign23620_e22466_d_n12;
        locals.var_fn277_calc_iq__vdsat10_dn13 = assign23620_e22466_d_n13;

        let (assign23630_e22535, assign23630_e22535_d_n2, assign23630_e22535_d_n4, assign23630_e22535_d_n7, assign23630_e22535_d_n12, assign23630_e22535_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23630_e22525, assign23630_e22525_d_n2, assign23630_e22525_d_n4, assign23630_e22525_d_n7, assign23630_e22525_d_n12, assign23630_e22525_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23630_e22478: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                let assign23630_e22479: f64 = assign23630_e22478;
                let assign23630_e22483: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                let assign23630_e22484: f64 = (-assign23630_e22483);
                let assign23630_e22487: f64 = (0.001 / p.p53);
                let assign23630_e22491: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                let assign23630_e22492: f64 = (-assign23630_e22491);
                let assign23630_e22493: f64 = (assign23630_e22487 * assign23630_e22492);
                let assign23630_e22494: f64 = (assign23630_e22493).tanh();
                let assign23630_e22495: f64 = (assign23630_e22484 * assign23630_e22494);
                let assign23630_e22496: f64 = (assign23630_e22479 + assign23630_e22495);
                let assign23630_e22497: f64 = (0.5 * assign23630_e22496);
                (assign23630_e22497, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))),)
            } else {
                let (assign23630_e22524, assign23630_e22524_d_n2, assign23630_e22524_d_n4, assign23630_e22524_d_n7, assign23630_e22524_d_n12, assign23630_e22524_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23630_e22505: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                        let assign23630_e22506: f64 = assign23630_e22505;
                        let assign23630_e22510: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                        let assign23630_e22511: f64 = (-assign23630_e22510);
                        let assign23630_e22515: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                        let assign23630_e22516: f64 = (-assign23630_e22515);
                        let assign23630_e22517: f64 = (assign23630_e22511 * assign23630_e22516);
                        let assign23630_e22519: f64 = (assign23630_e22517 + p.p53);
                        let assign23630_e22520: f64 = (assign23630_e22519).sqrt();
                        let assign23630_e22521: f64 = (assign23630_e22506 + assign23630_e22520);
                        let assign23630_e22522: f64 = (0.5 * assign23630_e22521);
                        (assign23630_e22522, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22516) + (assign23630_e22511 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23630_e22520)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22516) + (assign23630_e22511 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23630_e22520)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22516) + (assign23630_e22511 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23630_e22520)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22516) + (assign23630_e22511 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23630_e22520)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22516) + (assign23630_e22511 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23630_e22520)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23630_e22524, assign23630_e22524_d_n2, assign23630_e22524_d_n4, assign23630_e22524_d_n7, assign23630_e22524_d_n12, assign23630_e22524_d_n13,)
            }
        };
        let assign23630_e22527: f64 = (assign23630_e22525).powf(locals.var_fn277_calc_iq__beta);
        let assign23630_e22528: f64 = (1.0 + assign23630_e22527);
        let assign23630_e22531: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23630_e22532: f64 = (assign23630_e22528).powf(assign23630_e22531);
        let assign23630_e22533: f64 = (1.0 / assign23630_e22532);
        (assign23630_e22533, (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n2)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n2 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n2)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n2 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n4)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n4 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n4)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n4 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n7)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n7 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n7)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n7 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n12)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n12 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n12)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n12 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n13)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n13 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n13)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n13 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))),)
    } else {
        (locals.var_fn277_calc_iq__fsd0, locals.var_fn277_calc_iq__fsd0_dn2, locals.var_fn277_calc_iq__fsd0_dn4, locals.var_fn277_calc_iq__fsd0_dn7, locals.var_fn277_calc_iq__fsd0_dn12, locals.var_fn277_calc_iq__fsd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd0 = assign23630_e22535;
        locals.var_fn277_calc_iq__fsd0_dn2 = assign23630_e22535_d_n2;
        locals.var_fn277_calc_iq__fsd0_dn4 = assign23630_e22535_d_n4;
        locals.var_fn277_calc_iq__fsd0_dn7 = assign23630_e22535_d_n7;
        locals.var_fn277_calc_iq__fsd0_dn12 = assign23630_e22535_d_n12;
        locals.var_fn277_calc_iq__fsd0_dn13 = assign23630_e22535_d_n13;

        let (assign23640_e22541, assign23640_e22541_d_n2, assign23640_e22541_d_n4, assign23640_e22541_d_n7, assign23640_e22541_d_n12, assign23640_e22541_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23640_e22539: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0);
        (assign23640_e22539, (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn2), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn4), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn7), ((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__fsd0) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn12)), ((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__fsd0) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdx0, locals.var_fn277_calc_iq__vdx0_dn2, locals.var_fn277_calc_iq__vdx0_dn4, locals.var_fn277_calc_iq__vdx0_dn7, locals.var_fn277_calc_iq__vdx0_dn12, locals.var_fn277_calc_iq__vdx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx0 = assign23640_e22541;
        locals.var_fn277_calc_iq__vdx0_dn2 = assign23640_e22541_d_n2;
        locals.var_fn277_calc_iq__vdx0_dn4 = assign23640_e22541_d_n4;
        locals.var_fn277_calc_iq__vdx0_dn7 = assign23640_e22541_d_n7;
        locals.var_fn277_calc_iq__vdx0_dn12 = assign23640_e22541_d_n12;
        locals.var_fn277_calc_iq__vdx0_dn13 = assign23640_e22541_d_n13;

        let (assign23650_e22616, assign23650_e22616_d_n2, assign23650_e22616_d_n4, assign23650_e22616_d_n7, assign23650_e22616_d_n12, assign23650_e22616_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23650_e22606, assign23650_e22606_d_n2, assign23650_e22606_d_n4, assign23650_e22606_d_n7, assign23650_e22606_d_n12, assign23650_e22606_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23650_e22552: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23650_e22554: f64 = (assign23650_e22552 / locals.var_fn277_calc_iq__vdsat10);
                let assign23650_e22555: f64 = assign23650_e22554;
                let assign23650_e22558: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23650_e22560: f64 = (assign23650_e22558 / locals.var_fn277_calc_iq__vdsat10);
                let assign23650_e22561: f64 = (-assign23650_e22560);
                let assign23650_e22564: f64 = (0.001 / p.p53);
                let assign23650_e22567: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23650_e22569: f64 = (assign23650_e22567 / locals.var_fn277_calc_iq__vdsat10);
                let assign23650_e22570: f64 = (-assign23650_e22569);
                let assign23650_e22571: f64 = (assign23650_e22564 * assign23650_e22570);
                let assign23650_e22572: f64 = (assign23650_e22571).tanh();
                let assign23650_e22573: f64 = (assign23650_e22561 * assign23650_e22572);
                let assign23650_e22574: f64 = (assign23650_e22555 + assign23650_e22573);
                let assign23650_e22575: f64 = (0.5 * assign23650_e22574);
                (assign23650_e22575, (0.5 * ((-((assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-(-((assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * ((-((assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-(-((assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * ((-((assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-(-((assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))),)
            } else {
                let (assign23650_e22605, assign23650_e22605_d_n2, assign23650_e22605_d_n4, assign23650_e22605_d_n7, assign23650_e22605_d_n12, assign23650_e22605_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23650_e22582: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23650_e22584: f64 = (assign23650_e22582 / locals.var_fn277_calc_iq__vdsat10);
                        let assign23650_e22585: f64 = assign23650_e22584;
                        let assign23650_e22588: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23650_e22590: f64 = (assign23650_e22588 / locals.var_fn277_calc_iq__vdsat10);
                        let assign23650_e22591: f64 = (-assign23650_e22590);
                        let assign23650_e22594: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23650_e22596: f64 = (assign23650_e22594 / locals.var_fn277_calc_iq__vdsat10);
                        let assign23650_e22597: f64 = (-assign23650_e22596);
                        let assign23650_e22598: f64 = (assign23650_e22591 * assign23650_e22597);
                        let assign23650_e22600: f64 = (assign23650_e22598 + p.p53);
                        let assign23650_e22601: f64 = (assign23650_e22600).sqrt();
                        let assign23650_e22602: f64 = (assign23650_e22585 + assign23650_e22601);
                        let assign23650_e22603: f64 = (0.5 * assign23650_e22602);
                        (assign23650_e22603, (0.5 * ((-((assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22597) + (assign23650_e22591 * (-(-((assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23650_e22601)))), (0.5 * ((-((assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22597) + (assign23650_e22591 * (-(-((assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23650_e22601)))), (0.5 * ((-((assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22597) + (assign23650_e22591 * (-(-((assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23650_e22601)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22597) + (assign23650_e22591 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23650_e22601)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22597) + (assign23650_e22591 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23650_e22601)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23650_e22605, assign23650_e22605_d_n2, assign23650_e22605_d_n4, assign23650_e22605_d_n7, assign23650_e22605_d_n12, assign23650_e22605_d_n13,)
            }
        };
        let assign23650_e22608: f64 = (assign23650_e22606).powf(locals.var_fn277_calc_iq__beta);
        let assign23650_e22609: f64 = (1.0 + assign23650_e22608);
        let assign23650_e22612: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23650_e22613: f64 = (assign23650_e22609).powf(assign23650_e22612);
        let assign23650_e22614: f64 = (1.0 / assign23650_e22613);
        (assign23650_e22614, (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n2)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n2 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n2)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n2 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n4)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n4 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n4)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n4 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n7)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n7 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n7)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n7 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n12)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n12 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n12)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n12 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n13)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n13 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n13)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n13 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))),)
    } else {
        (locals.var_fn277_calc_iq__fds0, locals.var_fn277_calc_iq__fds0_dn2, locals.var_fn277_calc_iq__fds0_dn4, locals.var_fn277_calc_iq__fds0_dn7, locals.var_fn277_calc_iq__fds0_dn12, locals.var_fn277_calc_iq__fds0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds0 = assign23650_e22616;
        locals.var_fn277_calc_iq__fds0_dn2 = assign23650_e22616_d_n2;
        locals.var_fn277_calc_iq__fds0_dn4 = assign23650_e22616_d_n4;
        locals.var_fn277_calc_iq__fds0_dn7 = assign23650_e22616_d_n7;
        locals.var_fn277_calc_iq__fds0_dn12 = assign23650_e22616_d_n12;
        locals.var_fn277_calc_iq__fds0_dn13 = assign23650_e22616_d_n13;

        let (assign23660_e22623, assign23660_e22623_d_n2, assign23660_e22623_d_n4, assign23660_e22623_d_n7, assign23660_e22623_d_n12, assign23660_e22623_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23660_e22619: f64 = (-locals.var_fn277_calc_iq__vdsin);
        let assign23660_e22621: f64 = (assign23660_e22619 * locals.var_fn277_calc_iq__fds0);
        (assign23660_e22621, (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn2), (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn4), (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn7), (((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__fds0) + (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn12)), (((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__fds0) + (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vsx0, locals.var_fn277_calc_iq__vsx0_dn2, locals.var_fn277_calc_iq__vsx0_dn4, locals.var_fn277_calc_iq__vsx0_dn7, locals.var_fn277_calc_iq__vsx0_dn12, locals.var_fn277_calc_iq__vsx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx0 = assign23660_e22623;
        locals.var_fn277_calc_iq__vsx0_dn2 = assign23660_e22623_d_n2;
        locals.var_fn277_calc_iq__vsx0_dn4 = assign23660_e22623_d_n4;
        locals.var_fn277_calc_iq__vsx0_dn7 = assign23660_e22623_d_n7;
        locals.var_fn277_calc_iq__vsx0_dn12 = assign23660_e22623_d_n12;
        locals.var_fn277_calc_iq__vsx0_dn13 = assign23660_e22623_d_n13;

        let (assign23670_e22631, assign23670_e22631_d_n2, assign23670_e22631_d_n4, assign23670_e22631_d_n7, assign23670_e22631_d_n12, assign23670_e22631_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23670_e22627: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__myarg0);
        let assign23670_e22629: f64 = (assign23670_e22627 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23670_e22629, (locals.var_fn277_calc_iq__vgsin_dn2 / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg0_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23670_e22627 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), (locals.var_fn277_calc_iq__vgsin_dn7 / locals.var_fn277_calc_iq__alpha_phit), 0.0, (locals.var_fn277_calc_iq__vgsin_dn13 / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign23670_e22631;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign23670_e22631_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign23670_e22631_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign23670_e22631_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign23670_e22631_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign23670_e22631_d_n13;

        let assign23680_e22634: f64 = if locals.var_fn277_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign23680_e22634;

        let (assign23690_e22640, assign23690_e22640_d_n2, assign23690_e22640_d_n4, assign23690_e22640_d_n7, assign23690_e22640_d_n12, assign23690_e22640_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard295 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign23690_e22640;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign23690_e22640_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign23690_e22640_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign23690_e22640_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign23690_e22640_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign23690_e22640_d_n13;

        let assign23700_e22643: f64 = (-50.0);
        let assign23700_e22644: f64 = if locals.var_fn277_calc_iq__exparg0 < assign23700_e22643 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign23700_e22644;

        let (assign23710_e22653, assign23710_e22653_d_n2, assign23710_e22653_d_n4, assign23710_e22653_d_n7, assign23710_e22653_d_n12, assign23710_e22653_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard295 == 0.0)) && (locals.var_guard296 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign23710_e22653;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign23710_e22653_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign23710_e22653_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign23710_e22653_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign23710_e22653_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign23710_e22653_d_n13;

        let (assign23720_e22668, assign23720_e22668_d_n2, assign23720_e22668_d_n4, assign23720_e22668_d_n7, assign23720_e22668_d_n12, assign23720_e22668_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard295 == 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign23720_e22664: f64 = (locals.var_fn277_calc_iq__exparg0).exp();
        let assign23720_e22665: f64 = (1.0 + assign23720_e22664);
        let assign23720_e22666: f64 = (1.0 / assign23720_e22665);
        (assign23720_e22666, (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn2) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn4) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn7) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn12) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn13) / (assign23720_e22665 * assign23720_e22665))),)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign23720_e22668;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign23720_e22668_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign23720_e22668_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign23720_e22668_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign23720_e22668_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign23720_e22668_d_n13;

        let (assign23730_e22686, assign23730_e22686_d_n2, assign23730_e22686_d_n4, assign23730_e22686_d_n7, assign23730_e22686_d_n12, assign23730_e22686_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23730_e22672: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__vsx0);
        let assign23730_e22676: f64 = (p.p51 * 0.1);
        let assign23730_e22678: f64 = (assign23730_e22676 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23730_e22680: f64 = (assign23730_e22678 * locals.var_fn277_calc_iq__ffs0);
        let assign23730_e22681: f64 = (locals.var_fn277_calc_iq__vtof - assign23730_e22680);
        let assign23730_e22682: f64 = (assign23730_e22672 - assign23730_e22681);
        let assign23730_e22684: f64 = (assign23730_e22682 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign23730_e22684, (((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__vsx0_dn2) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn2))) / locals.var_fn277_calc_iq__two_n_phit0), (((((-locals.var_fn277_calc_iq__vsx0_dn4) - (locals.var_fn277_calc_iq__vtof_dn4 - (((assign23730_e22676 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffs0) + (assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn4)))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign23730_e22682 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__vsx0_dn7) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn7))) / locals.var_fn277_calc_iq__two_n_phit0), (((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__vsx0_dn12) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn12))) / locals.var_fn277_calc_iq__two_n_phit0), (((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__vsx0_dn13) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn13))) / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etas0, locals.var_fn277_calc_iq__etas0_dn2, locals.var_fn277_calc_iq__etas0_dn4, locals.var_fn277_calc_iq__etas0_dn7, locals.var_fn277_calc_iq__etas0_dn12, locals.var_fn277_calc_iq__etas0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas0 = assign23730_e22686;
        locals.var_fn277_calc_iq__etas0_dn2 = assign23730_e22686_d_n2;
        locals.var_fn277_calc_iq__etas0_dn4 = assign23730_e22686_d_n4;
        locals.var_fn277_calc_iq__etas0_dn7 = assign23730_e22686_d_n7;
        locals.var_fn277_calc_iq__etas0_dn12 = assign23730_e22686_d_n12;
        locals.var_fn277_calc_iq__etas0_dn13 = assign23730_e22686_d_n13;

        let assign23740_e22689: f64 = if locals.var_fn277_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign23740_e22689;

        let (assign23750_e22697, assign23750_e22697_d_n2, assign23750_e22697_d_n4, assign23750_e22697_d_n7, assign23750_e22697_d_n12, assign23750_e22697_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard297 != 0.0)) {
        let assign23750_e22695: f64 = (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0);
        (assign23750_e22695, (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn2), ((locals.var_fn277_calc_iq__qref0_dn4 * locals.var_fn277_calc_iq__etas0) + (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn4)), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn7), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn12), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign23750_e22697;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign23750_e22697_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign23750_e22697_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign23750_e22697_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign23750_e22697_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign23750_e22697_d_n13;

        let assign23760_e22700: f64 = (-50.0);
        let assign23760_e22701: f64 = if locals.var_fn277_calc_iq__etas0 < assign23760_e22700 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign23760_e22701;

        let (assign23770_e22713, assign23770_e22713_d_n2, assign23770_e22713_d_n4, assign23770_e22713_d_n7, assign23770_e22713_d_n12, assign23770_e22713_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard297 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign23770_e22710: f64 = (locals.var_fn277_calc_iq__etas0).exp();
        let assign23770_e22711: f64 = (locals.var_fn277_calc_iq__qref0 * assign23770_e22710);
        (assign23770_e22711, (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn2)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23770_e22710) + (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn4))), (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn7)), (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn12)), (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign23770_e22713;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign23770_e22713_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign23770_e22713_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign23770_e22713_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign23770_e22713_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign23770_e22713_d_n13;

        let (assign23780_e22729, assign23780_e22729_d_n2, assign23780_e22729_d_n4, assign23780_e22729_d_n7, assign23780_e22729_d_n12, assign23780_e22729_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard297 == 0.0)) && (locals.var_guard298 == 0.0)) {
        let assign23780_e22724: f64 = (locals.var_fn277_calc_iq__etas0).exp();
        let assign23780_e22725: f64 = (1.0 + assign23780_e22724);
        let assign23780_e22726: f64 = (assign23780_e22725).ln();
        let assign23780_e22727: f64 = (locals.var_fn277_calc_iq__qref0 * assign23780_e22726);
        (assign23780_e22727, (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn2) / assign23780_e22725)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23780_e22726) + (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn4) / assign23780_e22725))), (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn7) / assign23780_e22725)), (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn12) / assign23780_e22725)), (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn13) / assign23780_e22725)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign23780_e22729;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign23780_e22729_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign23780_e22729_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign23780_e22729_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign23780_e22729_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign23780_e22729_d_n13;

        let (assign23790_e22737, assign23790_e22737_d_n2, assign23790_e22737_d_n4, assign23790_e22737_d_n7, assign23790_e22737_d_n12, assign23790_e22737_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23790_e22733: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__myarg0);
        let assign23790_e22735: f64 = (assign23790_e22733 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23790_e22735, (locals.var_fn277_calc_iq__vgdin_dn2 / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg0_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23790_e22733 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), (locals.var_fn277_calc_iq__vgdin_dn7 / locals.var_fn277_calc_iq__alpha_phit), (locals.var_fn277_calc_iq__vgdin_dn12 / locals.var_fn277_calc_iq__alpha_phit), (locals.var_fn277_calc_iq__vgdin_dn13 / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign23790_e22737;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign23790_e22737_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign23790_e22737_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign23790_e22737_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign23790_e22737_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign23790_e22737_d_n13;

        let assign23800_e22740: f64 = if locals.var_fn277_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign23800_e22740;

        let (assign23810_e22746, assign23810_e22746_d_n2, assign23810_e22746_d_n4, assign23810_e22746_d_n7, assign23810_e22746_d_n12, assign23810_e22746_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard299 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign23810_e22746;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign23810_e22746_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign23810_e22746_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign23810_e22746_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign23810_e22746_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign23810_e22746_d_n13;

        let assign23820_e22749: f64 = (-50.0);
        let assign23820_e22750: f64 = if locals.var_fn277_calc_iq__exparg0 < assign23820_e22749 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign23820_e22750;

        let (assign23830_e22759, assign23830_e22759_d_n2, assign23830_e22759_d_n4, assign23830_e22759_d_n7, assign23830_e22759_d_n12, assign23830_e22759_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard299 == 0.0)) && (locals.var_guard300 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign23830_e22759;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign23830_e22759_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign23830_e22759_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign23830_e22759_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign23830_e22759_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign23830_e22759_d_n13;

        let (assign23840_e22774, assign23840_e22774_d_n2, assign23840_e22774_d_n4, assign23840_e22774_d_n7, assign23840_e22774_d_n12, assign23840_e22774_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard299 == 0.0)) && (locals.var_guard300 == 0.0)) {
        let assign23840_e22770: f64 = (locals.var_fn277_calc_iq__exparg0).exp();
        let assign23840_e22771: f64 = (1.0 + assign23840_e22770);
        let assign23840_e22772: f64 = (1.0 / assign23840_e22771);
        (assign23840_e22772, (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn2) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn4) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn7) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn12) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn13) / (assign23840_e22771 * assign23840_e22771))),)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign23840_e22774;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign23840_e22774_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign23840_e22774_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign23840_e22774_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign23840_e22774_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign23840_e22774_d_n13;

        let (assign23850_e22792, assign23850_e22792_d_n2, assign23850_e22792_d_n4, assign23850_e22792_d_n7, assign23850_e22792_d_n12, assign23850_e22792_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23850_e22778: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdx0);
        let assign23850_e22782: f64 = (p.p51 * 0.1);
        let assign23850_e22784: f64 = (assign23850_e22782 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23850_e22786: f64 = (assign23850_e22784 * locals.var_fn277_calc_iq__ffd0);
        let assign23850_e22787: f64 = (locals.var_fn277_calc_iq__vtof - assign23850_e22786);
        let assign23850_e22788: f64 = (assign23850_e22778 - assign23850_e22787);
        let assign23850_e22790: f64 = (assign23850_e22788 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign23850_e22790, (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vdx0_dn2) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn2))) / locals.var_fn277_calc_iq__two_n_phit0), (((((-locals.var_fn277_calc_iq__vdx0_dn4) - (locals.var_fn277_calc_iq__vtof_dn4 - (((assign23850_e22782 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffd0) + (assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn4)))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign23850_e22788 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vdx0_dn7) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn7))) / locals.var_fn277_calc_iq__two_n_phit0), (((-locals.var_fn277_calc_iq__vdx0_dn12) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn12))) / locals.var_fn277_calc_iq__two_n_phit0), (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdx0_dn13) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn13))) / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etad0, locals.var_fn277_calc_iq__etad0_dn2, locals.var_fn277_calc_iq__etad0_dn4, locals.var_fn277_calc_iq__etad0_dn7, locals.var_fn277_calc_iq__etad0_dn12, locals.var_fn277_calc_iq__etad0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad0 = assign23850_e22792;
        locals.var_fn277_calc_iq__etad0_dn2 = assign23850_e22792_d_n2;
        locals.var_fn277_calc_iq__etad0_dn4 = assign23850_e22792_d_n4;
        locals.var_fn277_calc_iq__etad0_dn7 = assign23850_e22792_d_n7;
        locals.var_fn277_calc_iq__etad0_dn12 = assign23850_e22792_d_n12;
        locals.var_fn277_calc_iq__etad0_dn13 = assign23850_e22792_d_n13;

        let assign23860_e22795: f64 = if locals.var_fn277_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard301 = assign23860_e22795;

        let (assign23870_e22803, assign23870_e22803_d_n2, assign23870_e22803_d_n4, assign23870_e22803_d_n7, assign23870_e22803_d_n12, assign23870_e22803_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign23870_e22801: f64 = (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0);
        (assign23870_e22801, (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn2), ((locals.var_fn277_calc_iq__qref0_dn4 * locals.var_fn277_calc_iq__etad0) + (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn4)), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn7), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn12), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign23870_e22803;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign23870_e22803_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign23870_e22803_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign23870_e22803_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign23870_e22803_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign23870_e22803_d_n13;

        let assign23880_e22806: f64 = (-50.0);
        let assign23880_e22807: f64 = if locals.var_fn277_calc_iq__etad0 < assign23880_e22806 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign23880_e22807;

        let (assign23890_e22819, assign23890_e22819_d_n2, assign23890_e22819_d_n4, assign23890_e22819_d_n7, assign23890_e22819_d_n12, assign23890_e22819_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 != 0.0)) {
        let assign23890_e22816: f64 = (locals.var_fn277_calc_iq__etad0).exp();
        let assign23890_e22817: f64 = (locals.var_fn277_calc_iq__qref0 * assign23890_e22816);
        (assign23890_e22817, (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn2)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23890_e22816) + (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn4))), (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn7)), (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn12)), (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign23890_e22819;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign23890_e22819_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign23890_e22819_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign23890_e22819_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign23890_e22819_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign23890_e22819_d_n13;

        let (assign23900_e22835, assign23900_e22835_d_n2, assign23900_e22835_d_n4, assign23900_e22835_d_n7, assign23900_e22835_d_n12, assign23900_e22835_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 == 0.0)) {
        let assign23900_e22830: f64 = (locals.var_fn277_calc_iq__etad0).exp();
        let assign23900_e22831: f64 = (1.0 + assign23900_e22830);
        let assign23900_e22832: f64 = (assign23900_e22831).ln();
        let assign23900_e22833: f64 = (locals.var_fn277_calc_iq__qref0 * assign23900_e22832);
        (assign23900_e22833, (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn2) / assign23900_e22831)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23900_e22832) + (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn4) / assign23900_e22831))), (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn7) / assign23900_e22831)), (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn12) / assign23900_e22831)), (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn13) / assign23900_e22831)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign23900_e22835;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign23900_e22835_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign23900_e22835_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign23900_e22835_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign23900_e22835_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign23900_e22835_d_n13;

        let (assign23910_e22843, assign23910_e22843_d_n2, assign23910_e22843_d_n4, assign23910_e22843_d_n7, assign23910_e22843_d_n12, assign23910_e22843_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23910_e22839: f64 = (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0);
        let assign23910_e22841: f64 = (assign23910_e22839 + 1e-38);
        (assign23910_e22841, ((locals.var_fn277_calc_iq__qinvs0_dn2 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn2)), ((locals.var_fn277_calc_iq__qinvs0_dn4 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn4)), ((locals.var_fn277_calc_iq__qinvs0_dn7 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn7)), ((locals.var_fn277_calc_iq__qinvs0_dn12 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn12)), ((locals.var_fn277_calc_iq__qinvs0_dn13 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qs2, locals.var_fn277_calc_iq__qs2_dn2, locals.var_fn277_calc_iq__qs2_dn4, locals.var_fn277_calc_iq__qs2_dn7, locals.var_fn277_calc_iq__qs2_dn12, locals.var_fn277_calc_iq__qs2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs2 = assign23910_e22843;
        locals.var_fn277_calc_iq__qs2_dn2 = assign23910_e22843_d_n2;
        locals.var_fn277_calc_iq__qs2_dn4 = assign23910_e22843_d_n4;
        locals.var_fn277_calc_iq__qs2_dn7 = assign23910_e22843_d_n7;
        locals.var_fn277_calc_iq__qs2_dn12 = assign23910_e22843_d_n12;
        locals.var_fn277_calc_iq__qs2_dn13 = assign23910_e22843_d_n13;

    }

    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23920_e22851, assign23920_e22851_d_n2, assign23920_e22851_d_n4, assign23920_e22851_d_n7, assign23920_e22851_d_n12, assign23920_e22851_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23920_e22847: f64 = (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0);
        let assign23920_e22849: f64 = (assign23920_e22847 + 1e-57);
        (assign23920_e22849, ((locals.var_fn277_calc_iq__qs2_dn2 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn2)), ((locals.var_fn277_calc_iq__qs2_dn4 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn4)), ((locals.var_fn277_calc_iq__qs2_dn7 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn7)), ((locals.var_fn277_calc_iq__qs2_dn12 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn12)), ((locals.var_fn277_calc_iq__qs2_dn13 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qs3, locals.var_fn277_calc_iq__qs3_dn2, locals.var_fn277_calc_iq__qs3_dn4, locals.var_fn277_calc_iq__qs3_dn7, locals.var_fn277_calc_iq__qs3_dn12, locals.var_fn277_calc_iq__qs3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs3 = assign23920_e22851;
        locals.var_fn277_calc_iq__qs3_dn2 = assign23920_e22851_d_n2;
        locals.var_fn277_calc_iq__qs3_dn4 = assign23920_e22851_d_n4;
        locals.var_fn277_calc_iq__qs3_dn7 = assign23920_e22851_d_n7;
        locals.var_fn277_calc_iq__qs3_dn12 = assign23920_e22851_d_n12;
        locals.var_fn277_calc_iq__qs3_dn13 = assign23920_e22851_d_n13;

        let (assign23930_e22859, assign23930_e22859_d_n2, assign23930_e22859_d_n4, assign23930_e22859_d_n7, assign23930_e22859_d_n12, assign23930_e22859_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23930_e22855: f64 = (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0);
        let assign23930_e22857: f64 = (assign23930_e22855 + 1e-38);
        (assign23930_e22857, ((locals.var_fn277_calc_iq__qinvd0_dn2 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn2)), ((locals.var_fn277_calc_iq__qinvd0_dn4 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn4)), ((locals.var_fn277_calc_iq__qinvd0_dn7 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn7)), ((locals.var_fn277_calc_iq__qinvd0_dn12 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn12)), ((locals.var_fn277_calc_iq__qinvd0_dn13 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qd2, locals.var_fn277_calc_iq__qd2_dn2, locals.var_fn277_calc_iq__qd2_dn4, locals.var_fn277_calc_iq__qd2_dn7, locals.var_fn277_calc_iq__qd2_dn12, locals.var_fn277_calc_iq__qd2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd2 = assign23930_e22859;
        locals.var_fn277_calc_iq__qd2_dn2 = assign23930_e22859_d_n2;
        locals.var_fn277_calc_iq__qd2_dn4 = assign23930_e22859_d_n4;
        locals.var_fn277_calc_iq__qd2_dn7 = assign23930_e22859_d_n7;
        locals.var_fn277_calc_iq__qd2_dn12 = assign23930_e22859_d_n12;
        locals.var_fn277_calc_iq__qd2_dn13 = assign23930_e22859_d_n13;

        let (assign23940_e22867, assign23940_e22867_d_n2, assign23940_e22867_d_n4, assign23940_e22867_d_n7, assign23940_e22867_d_n12, assign23940_e22867_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23940_e22863: f64 = (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0);
        let assign23940_e22865: f64 = (assign23940_e22863 + 1e-57);
        (assign23940_e22865, ((locals.var_fn277_calc_iq__qd2_dn2 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn2)), ((locals.var_fn277_calc_iq__qd2_dn4 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn4)), ((locals.var_fn277_calc_iq__qd2_dn7 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn7)), ((locals.var_fn277_calc_iq__qd2_dn12 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn12)), ((locals.var_fn277_calc_iq__qd2_dn13 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qd3, locals.var_fn277_calc_iq__qd3_dn2, locals.var_fn277_calc_iq__qd3_dn4, locals.var_fn277_calc_iq__qd3_dn7, locals.var_fn277_calc_iq__qd3_dn12, locals.var_fn277_calc_iq__qd3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd3 = assign23940_e22867;
        locals.var_fn277_calc_iq__qd3_dn2 = assign23940_e22867_d_n2;
        locals.var_fn277_calc_iq__qd3_dn4 = assign23940_e22867_d_n4;
        locals.var_fn277_calc_iq__qd3_dn7 = assign23940_e22867_d_n7;
        locals.var_fn277_calc_iq__qd3_dn12 = assign23940_e22867_d_n12;
        locals.var_fn277_calc_iq__qd3_dn13 = assign23940_e22867_d_n13;

        let (assign23950_e22875, assign23950_e22875_d_n2, assign23950_e22875_d_n4, assign23950_e22875_d_n7, assign23950_e22875_d_n12, assign23950_e22875_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23950_e22871: f64 = (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0);
        let assign23950_e22873: f64 = (assign23950_e22871 + 1e-38);
        (assign23950_e22873, ((locals.var_fn277_calc_iq__qinvs0_dn2 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn2)), ((locals.var_fn277_calc_iq__qinvs0_dn4 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn4)), ((locals.var_fn277_calc_iq__qinvs0_dn7 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn7)), ((locals.var_fn277_calc_iq__qinvs0_dn12 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn12)), ((locals.var_fn277_calc_iq__qinvs0_dn13 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qsqd, locals.var_fn277_calc_iq__qsqd_dn2, locals.var_fn277_calc_iq__qsqd_dn4, locals.var_fn277_calc_iq__qsqd_dn7, locals.var_fn277_calc_iq__qsqd_dn12, locals.var_fn277_calc_iq__qsqd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsqd = assign23950_e22875;
        locals.var_fn277_calc_iq__qsqd_dn2 = assign23950_e22875_d_n2;
        locals.var_fn277_calc_iq__qsqd_dn4 = assign23950_e22875_d_n4;
        locals.var_fn277_calc_iq__qsqd_dn7 = assign23950_e22875_d_n7;
        locals.var_fn277_calc_iq__qsqd_dn12 = assign23950_e22875_d_n12;
        locals.var_fn277_calc_iq__qsqd_dn13 = assign23950_e22875_d_n13;

        let (assign23960_e22893, assign23960_e22893_d_n2, assign23960_e22893_d_n4, assign23960_e22893_d_n7, assign23960_e22893_d_n12, assign23960_e22893_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23960_e22879: f64 = (2.0 / 3.0);
        let assign23960_e22882: f64 = (locals.var_fn277_calc_iq__qs2 + locals.var_fn277_calc_iq__qd2);
        let assign23960_e22884: f64 = (assign23960_e22882 + locals.var_fn277_calc_iq__qsqd);
        let assign23960_e22885: f64 = (assign23960_e22879 * assign23960_e22884);
        let assign23960_e22888: f64 = (locals.var_fn277_calc_iq__qinvs0 + locals.var_fn277_calc_iq__qinvd0);
        let assign23960_e22890: f64 = (assign23960_e22888 + 2e-19);
        let assign23960_e22891: f64 = (assign23960_e22885 / assign23960_e22890);
        (assign23960_e22891, ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn2 + locals.var_fn277_calc_iq__qd2_dn2) + locals.var_fn277_calc_iq__qsqd_dn2)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn2 + locals.var_fn277_calc_iq__qinvd0_dn2))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn4 + locals.var_fn277_calc_iq__qd2_dn4) + locals.var_fn277_calc_iq__qsqd_dn4)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn4 + locals.var_fn277_calc_iq__qinvd0_dn4))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn7 + locals.var_fn277_calc_iq__qd2_dn7) + locals.var_fn277_calc_iq__qsqd_dn7)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn7 + locals.var_fn277_calc_iq__qinvd0_dn7))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn12 + locals.var_fn277_calc_iq__qd2_dn12) + locals.var_fn277_calc_iq__qsqd_dn12)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn12 + locals.var_fn277_calc_iq__qinvd0_dn12))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn13 + locals.var_fn277_calc_iq__qd2_dn13) + locals.var_fn277_calc_iq__qsqd_dn13)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn13 + locals.var_fn277_calc_iq__qinvd0_dn13))) / (assign23960_e22890 * assign23960_e22890)),)
    } else {
        (locals.var_fn277_calc_iq__qinvdd, locals.var_fn277_calc_iq__qinvdd_dn2, locals.var_fn277_calc_iq__qinvdd_dn4, locals.var_fn277_calc_iq__qinvdd_dn7, locals.var_fn277_calc_iq__qinvdd_dn12, locals.var_fn277_calc_iq__qinvdd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvdd = assign23960_e22893;
        locals.var_fn277_calc_iq__qinvdd_dn2 = assign23960_e22893_d_n2;
        locals.var_fn277_calc_iq__qinvdd_dn4 = assign23960_e22893_d_n4;
        locals.var_fn277_calc_iq__qinvdd_dn7 = assign23960_e22893_d_n7;
        locals.var_fn277_calc_iq__qinvdd_dn12 = assign23960_e22893_d_n12;
        locals.var_fn277_calc_iq__qinvdd_dn13 = assign23960_e22893_d_n13;

        let (assign23970_e22927, assign23970_e22927_d_n2, assign23970_e22927_d_n4, assign23970_e22927_d_n7, assign23970_e22927_d_n12, assign23970_e22927_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23970_e22898: f64 = (2.0 * locals.var_fn277_calc_iq__qs3);
        let assign23970_e22901: f64 = (3.0 * locals.var_fn277_calc_iq__qd3);
        let assign23970_e22902: f64 = (assign23970_e22898 + assign23970_e22901);
        let assign23970_e22905: f64 = (4.0 * locals.var_fn277_calc_iq__qs2);
        let assign23970_e22907: f64 = (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0);
        let assign23970_e22908: f64 = (assign23970_e22902 + assign23970_e22907);
        let assign23970_e22911: f64 = (6.0 * locals.var_fn277_calc_iq__qd2);
        let assign23970_e22913: f64 = (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0);
        let assign23970_e22914: f64 = (assign23970_e22908 + assign23970_e22913);
        let assign23970_e22915: f64 = (2.0 * assign23970_e22914);
        let assign23970_e22919: f64 = (locals.var_fn277_calc_iq__qs2 + locals.var_fn277_calc_iq__qd2);
        let assign23970_e22922: f64 = (2.0 * locals.var_fn277_calc_iq__qsqd);
        let assign23970_e22923: f64 = (assign23970_e22919 + assign23970_e22922);
        let assign23970_e22924: f64 = (15.0 * assign23970_e22923);
        let assign23970_e22925: f64 = (assign23970_e22915 / assign23970_e22924);
        (assign23970_e22925, ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn2) + (3.0 * locals.var_fn277_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn2) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn2) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn2)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn2 + locals.var_fn277_calc_iq__qd2_dn2) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn2))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn4) + (3.0 * locals.var_fn277_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn4) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn4) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn4)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn4 + locals.var_fn277_calc_iq__qd2_dn4) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn4))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn7) + (3.0 * locals.var_fn277_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn7) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn7) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn7)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn7 + locals.var_fn277_calc_iq__qd2_dn7) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn7))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn12) + (3.0 * locals.var_fn277_calc_iq__qd3_dn12)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn12) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn12))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn12) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn12)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn12 + locals.var_fn277_calc_iq__qd2_dn12) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn12))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn13) + (3.0 * locals.var_fn277_calc_iq__qd3_dn13)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn13) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn13))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn13) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn13)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn13 + locals.var_fn277_calc_iq__qd2_dn13) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn13))))) / (assign23970_e22924 * assign23970_e22924)),)
    } else {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd1 = assign23970_e22927;
        locals.var_fn277_calc_iq__qd1_dn2 = assign23970_e22927_d_n2;
        locals.var_fn277_calc_iq__qd1_dn4 = assign23970_e22927_d_n4;
        locals.var_fn277_calc_iq__qd1_dn7 = assign23970_e22927_d_n7;
        locals.var_fn277_calc_iq__qd1_dn12 = assign23970_e22927_d_n12;
        locals.var_fn277_calc_iq__qd1_dn13 = assign23970_e22927_d_n13;

        let (assign23980_e22933, assign23980_e22933_d_n2, assign23980_e22933_d_n4, assign23980_e22933_d_n7, assign23980_e22933_d_n12, assign23980_e22933_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23980_e22931: f64 = (locals.var_fn277_calc_iq__qinvdd - locals.var_fn277_calc_iq__qd1);
        (assign23980_e22931, (locals.var_fn277_calc_iq__qinvdd_dn2 - locals.var_fn277_calc_iq__qd1_dn2), (locals.var_fn277_calc_iq__qinvdd_dn4 - locals.var_fn277_calc_iq__qd1_dn4), (locals.var_fn277_calc_iq__qinvdd_dn7 - locals.var_fn277_calc_iq__qd1_dn7), (locals.var_fn277_calc_iq__qinvdd_dn12 - locals.var_fn277_calc_iq__qd1_dn12), (locals.var_fn277_calc_iq__qinvdd_dn13 - locals.var_fn277_calc_iq__qd1_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qs, locals.var_fn277_calc_iq__qs_dn2, locals.var_fn277_calc_iq__qs_dn4, locals.var_fn277_calc_iq__qs_dn7, locals.var_fn277_calc_iq__qs_dn12, locals.var_fn277_calc_iq__qs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs = assign23980_e22933;
        locals.var_fn277_calc_iq__qs_dn2 = assign23980_e22933_d_n2;
        locals.var_fn277_calc_iq__qs_dn4 = assign23980_e22933_d_n4;
        locals.var_fn277_calc_iq__qs_dn7 = assign23980_e22933_d_n7;
        locals.var_fn277_calc_iq__qs_dn12 = assign23980_e22933_d_n12;
        locals.var_fn277_calc_iq__qs_dn13 = assign23980_e22933_d_n13;

        let (assign23990_e22937, assign23990_e22937_d_n2, assign23990_e22937_d_n4, assign23990_e22937_d_n7, assign23990_e22937_d_n12, assign23990_e22937_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    } else {
        (locals.var_fn277_calc_iq__qd, locals.var_fn277_calc_iq__qd_dn2, locals.var_fn277_calc_iq__qd_dn4, locals.var_fn277_calc_iq__qd_dn7, locals.var_fn277_calc_iq__qd_dn12, locals.var_fn277_calc_iq__qd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd = assign23990_e22937;
        locals.var_fn277_calc_iq__qd_dn2 = assign23990_e22937_d_n2;
        locals.var_fn277_calc_iq__qd_dn4 = assign23990_e22937_d_n4;
        locals.var_fn277_calc_iq__qd_dn7 = assign23990_e22937_d_n7;
        locals.var_fn277_calc_iq__qd_dn12 = assign23990_e22937_d_n12;
        locals.var_fn277_calc_iq__qd_dn13 = assign23990_e22937_d_n13;

        let (assign24000_e22951, assign24000_e22951_d_n2, assign24000_e22951_d_n4, assign24000_e22951_d_n7, assign24000_e22951_d_n12, assign24000_e22951_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign24000_e22941: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24000_e22943: f64 = (assign24000_e22941 * locals.var_fn277_calc_iq__lin);
        let assign24000_e22945: f64 = (assign24000_e22943 * locals.var_fn277_calc_iq__type);
        let assign24000_e22947: f64 = (assign24000_e22945 * locals.var_fn277_calc_iq__qs);
        let assign24000_e22949: f64 = (assign24000_e22947 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24000_e22949, ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn4) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgsout = assign24000_e22951;
        locals.var_fn277_calc_iq__qgsout_dn2 = assign24000_e22951_d_n2;
        locals.var_fn277_calc_iq__qgsout_dn4 = assign24000_e22951_d_n4;
        locals.var_fn277_calc_iq__qgsout_dn7 = assign24000_e22951_d_n7;
        locals.var_fn277_calc_iq__qgsout_dn12 = assign24000_e22951_d_n12;
        locals.var_fn277_calc_iq__qgsout_dn13 = assign24000_e22951_d_n13;

        let (assign24010_e22965, assign24010_e22965_d_n2, assign24010_e22965_d_n4, assign24010_e22965_d_n7, assign24010_e22965_d_n12, assign24010_e22965_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign24010_e22955: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24010_e22957: f64 = (assign24010_e22955 * locals.var_fn277_calc_iq__lin);
        let assign24010_e22959: f64 = (assign24010_e22957 * locals.var_fn277_calc_iq__type);
        let assign24010_e22961: f64 = (assign24010_e22959 * locals.var_fn277_calc_iq__qd);
        let assign24010_e22963: f64 = (assign24010_e22961 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24010_e22963, ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn4) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgdout = assign24010_e22965;
        locals.var_fn277_calc_iq__qgdout_dn2 = assign24010_e22965_d_n2;
        locals.var_fn277_calc_iq__qgdout_dn4 = assign24010_e22965_d_n4;
        locals.var_fn277_calc_iq__qgdout_dn7 = assign24010_e22965_d_n7;
        locals.var_fn277_calc_iq__qgdout_dn12 = assign24010_e22965_d_n12;
        locals.var_fn277_calc_iq__qgdout_dn13 = assign24010_e22965_d_n13;

        let assign24020_e22968: f64 = if locals.var_fn277_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign24020_e22968;

        let (assign24030_e22984, assign24030_e22984_d_n2, assign24030_e22984_d_n4, assign24030_e22984_d_n7, assign24030_e22984_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24030_e22976: f64 = (p.p51 * 0.5);
        let assign24030_e22978: f64 = (assign24030_e22976 * locals.var_fn277_calc_iq__alpha_phit);
        let assign24030_e22979: f64 = (locals.var_fn277_calc_iq__vtof - assign24030_e22978);
        let assign24030_e22980: f64 = (locals.var_fn277_calc_iq__vcin - assign24030_e22979);
        let assign24030_e22982: f64 = (assign24030_e22980 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign24030_e22982, (locals.var_fn277_calc_iq__vcin_dn2 / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (assign24030_e22976 * locals.var_fn277_calc_iq__alpha_phit_dn4))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign24030_e22980 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (locals.var_fn277_calc_iq__vcin_dn7 / locals.var_fn277_calc_iq__two_n_phit0), (locals.var_fn277_calc_iq__vcin_dn13 / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, locals.var_fn277_calc_iq__etac_dn13,)
    }
};
        locals.var_fn277_calc_iq__etac = assign24030_e22984;
        locals.var_fn277_calc_iq__etac_dn2 = assign24030_e22984_d_n2;
        locals.var_fn277_calc_iq__etac_dn4 = assign24030_e22984_d_n4;
        locals.var_fn277_calc_iq__etac_dn7 = assign24030_e22984_d_n7;
        locals.var_fn277_calc_iq__etac_dn13 = assign24030_e22984_d_n13;

        let assign24040_e22987: f64 = if locals.var_fn277_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign24040_e22987;

        let (assign24050_e22995, assign24050_e22995_d_n2, assign24050_e22995_d_n3, assign24050_e22995_d_n4, assign24050_e22995_d_n7, assign24050_e22995_d_n12, assign24050_e22995_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard304 != 0.0)) {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, 0.0, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, 0.0, locals.var_fn277_calc_iq__etac_dn13,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24050_e22995;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24050_e22995_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24050_e22995_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24050_e22995_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24050_e22995_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24050_e22995_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24050_e22995_d_n13;

        let assign24060_e22998: f64 = (-50.0);
        let assign24060_e22999: f64 = if locals.var_fn277_calc_iq__etac < assign24060_e22998 { 1.0 } else { 0.0 };
        locals.var_guard305 = assign24060_e22999;

        let (assign24070_e23011, assign24070_e23011_d_n2, assign24070_e23011_d_n3, assign24070_e23011_d_n4, assign24070_e23011_d_n7, assign24070_e23011_d_n12, assign24070_e23011_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard305 != 0.0)) {
        let assign24070_e23009: f64 = (locals.var_fn277_calc_iq__etac).exp();
        (assign24070_e23009, (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn2), 0.0, (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn4), (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn7), 0.0, (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn13),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24070_e23011;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24070_e23011_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24070_e23011_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24070_e23011_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24070_e23011_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24070_e23011_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24070_e23011_d_n13;

        let (assign24080_e23027, assign24080_e23027_d_n2, assign24080_e23027_d_n3, assign24080_e23027_d_n4, assign24080_e23027_d_n7, assign24080_e23027_d_n12, assign24080_e23027_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard305 == 0.0)) {
        let assign24080_e23023: f64 = (locals.var_fn277_calc_iq__etac).exp();
        let assign24080_e23024: f64 = (1.0 + assign24080_e23023);
        let assign24080_e23025: f64 = (assign24080_e23024).ln();
        (assign24080_e23025, ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn2) / assign24080_e23024), 0.0, ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn4) / assign24080_e23024), ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn7) / assign24080_e23024), 0.0, ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn13) / assign24080_e23024),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24080_e23027;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24080_e23027_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24080_e23027_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24080_e23027_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24080_e23027_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24080_e23027_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24080_e23027_d_n13;

        let (assign24090_e23045, assign24090_e23045_d_n2, assign24090_e23045_d_n3, assign24090_e23045_d_n4, assign24090_e23045_d_n7, assign24090_e23045_d_n12, assign24090_e23045_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24090_e23033: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24090_e23035: f64 = (assign24090_e23033 * locals.var_fn277_calc_iq__type);
        let assign24090_e23037: f64 = (assign24090_e23035 * locals.var_fn277_calc_iq__cc);
        let assign24090_e23039: f64 = (assign24090_e23037 * locals.var_fn277_calc_iq__two_n_phit0);
        let assign24090_e23041: f64 = (assign24090_e23039 * locals.var_fn277_calc_iq__exparg);
        let assign24090_e23043: f64 = (assign24090_e23041 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24090_e23043, ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn3) * locals.var_fn277_calc_iq__trapfracdl), ((((((assign24090_e23035 * locals.var_fn277_calc_iq__cc_dn4) * locals.var_fn277_calc_iq__two_n_phit0) + (assign24090_e23037 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) * locals.var_fn277_calc_iq__exparg) + (assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn4)) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign24090_e23045;
        locals.var_fn277_calc_iq__qcout_dn2 = assign24090_e23045_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign24090_e23045_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign24090_e23045_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign24090_e23045_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign24090_e23045_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign24090_e23045_d_n13;

        let (assign24100_e23061, assign24100_e23061_d_n3, assign24100_e23061_d_n4, assign24100_e23061_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24100_e23053: f64 = (p.p51 * 0.5);
        let assign24100_e23055: f64 = (assign24100_e23053 * locals.var_fn277_calc_iq__alpha_phit);
        let assign24100_e23056: f64 = (locals.var_fn277_calc_iq__vtof - assign24100_e23055);
        let assign24100_e23057: f64 = (locals.var_fn277_calc_iq__vbin - assign24100_e23056);
        let assign24100_e23059: f64 = (assign24100_e23057 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign24100_e23059, (locals.var_fn277_calc_iq__vbin_dn3 / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (assign24100_e23053 * locals.var_fn277_calc_iq__alpha_phit_dn4))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign24100_e23057 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (locals.var_fn277_calc_iq__vbin_dn13 / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etab, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, locals.var_fn277_calc_iq__etab_dn13,)
    }
};
        locals.var_fn277_calc_iq__etab = assign24100_e23061;
        locals.var_fn277_calc_iq__etab_dn3 = assign24100_e23061_d_n3;
        locals.var_fn277_calc_iq__etab_dn4 = assign24100_e23061_d_n4;
        locals.var_fn277_calc_iq__etab_dn13 = assign24100_e23061_d_n13;

        let assign24110_e23064: f64 = if locals.var_fn277_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign24110_e23064;

        let (assign24120_e23072, assign24120_e23072_d_n2, assign24120_e23072_d_n3, assign24120_e23072_d_n4, assign24120_e23072_d_n7, assign24120_e23072_d_n12, assign24120_e23072_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard306 != 0.0)) {
        (locals.var_fn277_calc_iq__etab, 0.0, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn277_calc_iq__etab_dn13,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24120_e23072;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24120_e23072_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24120_e23072_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24120_e23072_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24120_e23072_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24120_e23072_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24120_e23072_d_n13;

        let assign24130_e23075: f64 = (-50.0);
        let assign24130_e23076: f64 = if locals.var_fn277_calc_iq__etab < assign24130_e23075 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign24130_e23076;

        let (assign24140_e23088, assign24140_e23088_d_n2, assign24140_e23088_d_n3, assign24140_e23088_d_n4, assign24140_e23088_d_n7, assign24140_e23088_d_n12, assign24140_e23088_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard306 == 0.0)) && (locals.var_guard307 != 0.0)) {
        let assign24140_e23086: f64 = (locals.var_fn277_calc_iq__etab).exp();
        (assign24140_e23086, 0.0, (assign24140_e23086 * locals.var_fn277_calc_iq__etab_dn3), (assign24140_e23086 * locals.var_fn277_calc_iq__etab_dn4), 0.0, 0.0, (assign24140_e23086 * locals.var_fn277_calc_iq__etab_dn13),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24140_e23088;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24140_e23088_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24140_e23088_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24140_e23088_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24140_e23088_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24140_e23088_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24140_e23088_d_n13;

        let (assign24150_e23104, assign24150_e23104_d_n2, assign24150_e23104_d_n3, assign24150_e23104_d_n4, assign24150_e23104_d_n7, assign24150_e23104_d_n12, assign24150_e23104_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard306 == 0.0)) && (locals.var_guard307 == 0.0)) {
        let assign24150_e23100: f64 = (locals.var_fn277_calc_iq__etab).exp();
        let assign24150_e23101: f64 = (1.0 + assign24150_e23100);
        let assign24150_e23102: f64 = (assign24150_e23101).ln();
        (assign24150_e23102, 0.0, ((assign24150_e23100 * locals.var_fn277_calc_iq__etab_dn3) / assign24150_e23101), ((assign24150_e23100 * locals.var_fn277_calc_iq__etab_dn4) / assign24150_e23101), 0.0, 0.0, ((assign24150_e23100 * locals.var_fn277_calc_iq__etab_dn13) / assign24150_e23101),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24150_e23104;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24150_e23104_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24150_e23104_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24150_e23104_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24150_e23104_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24150_e23104_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24150_e23104_d_n13;

        let (assign24160_e23122, assign24160_e23122_d_n2, assign24160_e23122_d_n3, assign24160_e23122_d_n4, assign24160_e23122_d_n7, assign24160_e23122_d_n12, assign24160_e23122_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24160_e23110: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24160_e23112: f64 = (assign24160_e23110 * locals.var_fn277_calc_iq__type);
        let assign24160_e23114: f64 = (assign24160_e23112 * locals.var_fn277_calc_iq__cb);
        let assign24160_e23116: f64 = (assign24160_e23114 * locals.var_fn277_calc_iq__two_n_phit0);
        let assign24160_e23118: f64 = (assign24160_e23116 * locals.var_fn277_calc_iq__exparg);
        let assign24160_e23120: f64 = (assign24160_e23118 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24160_e23120, ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn3) * locals.var_fn277_calc_iq__trapfracdl), ((((((assign24160_e23112 * locals.var_fn277_calc_iq__cb_dn4) * locals.var_fn277_calc_iq__two_n_phit0) + (assign24160_e23114 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) * locals.var_fn277_calc_iq__exparg) + (assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn4)) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign24160_e23122;
        locals.var_fn277_calc_iq__qbout_dn2 = assign24160_e23122_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign24160_e23122_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign24160_e23122_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign24160_e23122_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign24160_e23122_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign24160_e23122_d_n13;

        let (assign24170_e23129, assign24170_e23129_d_n2, assign24170_e23129_d_n3, assign24170_e23129_d_n4, assign24170_e23129_d_n7, assign24170_e23129_d_n12, assign24170_e23129_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign24170_e23129;
        locals.var_fn277_calc_iq__qcout_dn2 = assign24170_e23129_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign24170_e23129_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign24170_e23129_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign24170_e23129_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign24170_e23129_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign24170_e23129_d_n13;

        let (assign24180_e23136, assign24180_e23136_d_n2, assign24180_e23136_d_n3, assign24180_e23136_d_n4, assign24180_e23136_d_n7, assign24180_e23136_d_n12, assign24180_e23136_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign24180_e23136;
        locals.var_fn277_calc_iq__qbout_dn2 = assign24180_e23136_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign24180_e23136_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign24180_e23136_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign24180_e23136_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign24180_e23136_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign24180_e23136_d_n13;

        let assign24190_e23139: f64 = if locals.var_fn277_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign24190_e23139;

        let (assign24200_e23155, assign24200_e23155_d_n2, assign24200_e23155_d_n4, assign24200_e23155_d_n7, assign24200_e23155_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) {
        let assign24200_e23147: f64 = (p.p51 * 0.5);
        let assign24200_e23149: f64 = (assign24200_e23147 * locals.var_fn277_calc_iq__alpha_phit);
        let assign24200_e23150: f64 = (locals.var_fn277_calc_iq__vtof - assign24200_e23149);
        let assign24200_e23151: f64 = (locals.var_fn277_calc_iq__vgsin - assign24200_e23150);
        let assign24200_e23153: f64 = (assign24200_e23151 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign24200_e23153, (locals.var_fn277_calc_iq__vgsin_dn2 / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (assign24200_e23147 * locals.var_fn277_calc_iq__alpha_phit_dn4))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign24200_e23151 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (locals.var_fn277_calc_iq__vgsin_dn7 / locals.var_fn277_calc_iq__two_n_phit0), (locals.var_fn277_calc_iq__vgsin_dn13 / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, locals.var_fn277_calc_iq__etags_dn13,)
    }
};
        locals.var_fn277_calc_iq__etags = assign24200_e23155;
        locals.var_fn277_calc_iq__etags_dn2 = assign24200_e23155_d_n2;
        locals.var_fn277_calc_iq__etags_dn4 = assign24200_e23155_d_n4;
        locals.var_fn277_calc_iq__etags_dn7 = assign24200_e23155_d_n7;
        locals.var_fn277_calc_iq__etags_dn13 = assign24200_e23155_d_n13;

        let assign24210_e23158: f64 = if locals.var_fn277_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign24210_e23158;

        let (assign24220_e23166, assign24220_e23166_d_n2, assign24220_e23166_d_n3, assign24220_e23166_d_n4, assign24220_e23166_d_n7, assign24220_e23166_d_n12, assign24220_e23166_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 != 0.0)) {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, 0.0, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, 0.0, locals.var_fn277_calc_iq__etags_dn13,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24220_e23166;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24220_e23166_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24220_e23166_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24220_e23166_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24220_e23166_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24220_e23166_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24220_e23166_d_n13;

        let assign24230_e23169: f64 = (-50.0);
        let assign24230_e23170: f64 = if locals.var_fn277_calc_iq__etags < assign24230_e23169 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign24230_e23170;

        let (assign24240_e23182, assign24240_e23182_d_n2, assign24240_e23182_d_n3, assign24240_e23182_d_n4, assign24240_e23182_d_n7, assign24240_e23182_d_n12, assign24240_e23182_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign24240_e23180: f64 = (locals.var_fn277_calc_iq__etags).exp();
        (assign24240_e23180, (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn2), 0.0, (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn4), (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn7), 0.0, (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn13),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24240_e23182;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24240_e23182_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24240_e23182_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24240_e23182_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24240_e23182_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24240_e23182_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24240_e23182_d_n13;

        let (assign24250_e23198, assign24250_e23198_d_n2, assign24250_e23198_d_n3, assign24250_e23198_d_n4, assign24250_e23198_d_n7, assign24250_e23198_d_n12, assign24250_e23198_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
        let assign24250_e23194: f64 = (locals.var_fn277_calc_iq__etags).exp();
        let assign24250_e23195: f64 = (1.0 + assign24250_e23194);
        let assign24250_e23196: f64 = (assign24250_e23195).ln();
        (assign24250_e23196, ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn2) / assign24250_e23195), 0.0, ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn4) / assign24250_e23195), ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn7) / assign24250_e23195), 0.0, ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn13) / assign24250_e23195),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24250_e23198;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24250_e23198_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24250_e23198_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24250_e23198_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24250_e23198_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24250_e23198_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24250_e23198_d_n13;

        let (assign24260_e23216, assign24260_e23216_d_n2, assign24260_e23216_d_n3, assign24260_e23216_d_n4, assign24260_e23216_d_n7, assign24260_e23216_d_n12, assign24260_e23216_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) {
        let assign24260_e23204: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24260_e23206: f64 = (assign24260_e23204 * locals.var_fn277_calc_iq__type);
        let assign24260_e23208: f64 = (assign24260_e23206 * locals.var_fn277_calc_iq__cs);
        let assign24260_e23210: f64 = (assign24260_e23208 * locals.var_fn277_calc_iq__two_n_phit0);
        let assign24260_e23212: f64 = (assign24260_e23210 * locals.var_fn277_calc_iq__exparg);
        let assign24260_e23214: f64 = (assign24260_e23212 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24260_e23214, ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn3) * locals.var_fn277_calc_iq__trapfracdl), ((((assign24260_e23208 * locals.var_fn277_calc_iq__two_n_phit0_dn4) * locals.var_fn277_calc_iq__exparg) + (assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn4)) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign24260_e23216;
        locals.var_fn277_calc_iq__qsout_dn2 = assign24260_e23216_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign24260_e23216_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign24260_e23216_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign24260_e23216_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign24260_e23216_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign24260_e23216_d_n13;

    }

    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24270_e23223, assign24270_e23223_d_n2, assign24270_e23223_d_n3, assign24270_e23223_d_n4, assign24270_e23223_d_n7, assign24270_e23223_d_n12, assign24270_e23223_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard308 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign24270_e23223;
        locals.var_fn277_calc_iq__qsout_dn2 = assign24270_e23223_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign24270_e23223_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign24270_e23223_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign24270_e23223_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign24270_e23223_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign24270_e23223_d_n13;

        let (assign24300_e23235, assign24300_e23235_d_n2, assign24300_e23235_d_n4, assign24300_e23235_d_n7, assign24300_e23235_d_n12, assign24300_e23235_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    } else {
        (locals.var_qgsfps4, locals.var_qgsfps4_dn2, locals.var_qgsfps4_dn4, locals.var_qgsfps4_dn7, locals.var_qgsfps4_dn12, locals.var_qgsfps4_dn13,)
    }
};
        locals.var_qgsfps4 = assign24300_e23235;
        locals.var_qgsfps4_dn2 = assign24300_e23235_d_n2;
        locals.var_qgsfps4_dn4 = assign24300_e23235_d_n4;
        locals.var_qgsfps4_dn7 = assign24300_e23235_d_n7;
        locals.var_qgsfps4_dn12 = assign24300_e23235_d_n12;
        locals.var_qgsfps4_dn13 = assign24300_e23235_d_n13;

        let (assign24310_e23239, assign24310_e23239_d_n2, assign24310_e23239_d_n4, assign24310_e23239_d_n7, assign24310_e23239_d_n12, assign24310_e23239_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    } else {
        (locals.var_qgdfps4, locals.var_qgdfps4_dn2, locals.var_qgdfps4_dn4, locals.var_qgdfps4_dn7, locals.var_qgdfps4_dn12, locals.var_qgdfps4_dn13,)
    }
};
        locals.var_qgdfps4 = assign24310_e23239;
        locals.var_qgdfps4_dn2 = assign24310_e23239_d_n2;
        locals.var_qgdfps4_dn4 = assign24310_e23239_d_n4;
        locals.var_qgdfps4_dn7 = assign24310_e23239_d_n7;
        locals.var_qgdfps4_dn12 = assign24310_e23239_d_n12;
        locals.var_qgdfps4_dn13 = assign24310_e23239_d_n13;

        let (assign24320_e23243, assign24320_e23243_d_n2, assign24320_e23243_d_n3, assign24320_e23243_d_n4, assign24320_e23243_d_n7, assign24320_e23243_d_n12, assign24320_e23243_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    } else {
        (locals.var_qcfps4, locals.var_qcfps4_dn2, locals.var_qcfps4_dn3, locals.var_qcfps4_dn4, locals.var_qcfps4_dn7, locals.var_qcfps4_dn12, locals.var_qcfps4_dn13,)
    }
};
        locals.var_qcfps4 = assign24320_e23243;
        locals.var_qcfps4_dn2 = assign24320_e23243_d_n2;
        locals.var_qcfps4_dn3 = assign24320_e23243_d_n3;
        locals.var_qcfps4_dn4 = assign24320_e23243_d_n4;
        locals.var_qcfps4_dn7 = assign24320_e23243_d_n7;
        locals.var_qcfps4_dn12 = assign24320_e23243_d_n12;
        locals.var_qcfps4_dn13 = assign24320_e23243_d_n13;

        let (assign24330_e23247, assign24330_e23247_d_n2, assign24330_e23247_d_n3, assign24330_e23247_d_n4, assign24330_e23247_d_n7, assign24330_e23247_d_n12, assign24330_e23247_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    } else {
        (locals.var_qbfps4, locals.var_qbfps4_dn2, locals.var_qbfps4_dn3, locals.var_qbfps4_dn4, locals.var_qbfps4_dn7, locals.var_qbfps4_dn12, locals.var_qbfps4_dn13,)
    }
};
        locals.var_qbfps4 = assign24330_e23247;
        locals.var_qbfps4_dn2 = assign24330_e23247_d_n2;
        locals.var_qbfps4_dn3 = assign24330_e23247_d_n3;
        locals.var_qbfps4_dn4 = assign24330_e23247_d_n4;
        locals.var_qbfps4_dn7 = assign24330_e23247_d_n7;
        locals.var_qbfps4_dn12 = assign24330_e23247_d_n12;
        locals.var_qbfps4_dn13 = assign24330_e23247_d_n13;

        let (assign24340_e23251, assign24340_e23251_d_n2, assign24340_e23251_d_n3, assign24340_e23251_d_n4, assign24340_e23251_d_n7, assign24340_e23251_d_n12, assign24340_e23251_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    } else {
        (locals.var_qsfps4, locals.var_qsfps4_dn2, locals.var_qsfps4_dn3, locals.var_qsfps4_dn4, locals.var_qsfps4_dn7, locals.var_qsfps4_dn12, locals.var_qsfps4_dn13,)
    }
};
        locals.var_qsfps4 = assign24340_e23251;
        locals.var_qsfps4_dn2 = assign24340_e23251_d_n2;
        locals.var_qsfps4_dn3 = assign24340_e23251_d_n3;
        locals.var_qsfps4_dn4 = assign24340_e23251_d_n4;
        locals.var_qsfps4_dn7 = assign24340_e23251_d_n7;
        locals.var_qsfps4_dn12 = assign24340_e23251_d_n12;
        locals.var_qsfps4_dn13 = assign24340_e23251_d_n13;

        let assign24380_e23266: f64 = if p.p144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign24380_e23266;

        locals.var_fn382_calc_iq__return = 0.0;
        locals.var_fn382_calc_iq__return_dn4 = 0.0;
        locals.var_fn382_calc_iq__return_dn5 = 0.0;
        locals.var_fn382_calc_iq__return_dn8 = 0.0;
        locals.var_fn382_calc_iq__return_dn9 = 0.0;
        locals.var_fn382_calc_iq__return_dn22 = 0.0;
        locals.var_fn382_calc_iq__return_dn23 = 0.0;
        locals.var_fn382_calc_iq__return_dn25 = 0.0;
        locals.var_fn382_calc_iq__return_dn26 = 0.0;

        locals.var_fn382_calc_iq__idsout = 0.0;
        locals.var_fn382_calc_iq__idsout_dn4 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn5 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn8 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn9 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn22 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn23 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn25 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn26 = 0.0;

        locals.var_fn382_calc_iq__qgsout = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn4 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn5 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn8 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn9 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn22 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn23 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn25 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn26 = 0.0;

        locals.var_fn382_calc_iq__qgdout = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn4 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn5 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn8 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn9 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn22 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn23 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn25 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn26 = 0.0;

        locals.var_fn382_calc_iq__vtdibl = 0.0;
        locals.var_fn382_calc_iq__vtdibl_dn4 = 0.0;
        locals.var_fn382_calc_iq__vtdibl_dn5 = 0.0;
        locals.var_fn382_calc_iq__vtdibl_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsat1 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn9 = 0.0;

        locals.var_fn382_calc_iq__vgsin = locals.var_vgsi;
        locals.var_fn382_calc_iq__vgsin_dn8 = locals.var_vgsi_dn8;
        locals.var_fn382_calc_iq__vgsin_dn9 = locals.var_vgsi_dn9;

        locals.var_fn382_calc_iq__vdsin = locals.var_vdsi;
        locals.var_fn382_calc_iq__vdsin_dn5 = locals.var_vdsi_dn5;
        locals.var_fn382_calc_iq__vdsin_dn9 = locals.var_vdsi_dn9;

        locals.var_fn382_calc_iq__qcbflag = 0.0;

        locals.var_fn382_calc_iq__vcin = 0.0;

        locals.var_fn382_calc_iq__vbin = 0.0;

        locals.var_fn382_calc_iq__qgsflag = 0.0;

        locals.var_fn382_calc_iq__tambin = locals.var_tdut;
        locals.var_fn382_calc_iq__tambin_dn4 = locals.var_tdut_dn4;

        locals.var_fn382_calc_iq__tnomin = locals.var_tnomk;

        locals.var_fn382_calc_iq__phitin = locals.var_phit;
        locals.var_fn382_calc_iq__phitin_dn4 = locals.var_phit_dn4;

        locals.var_fn382_calc_iq__w = p.p0;

        locals.var_fn382_calc_iq__lin = p.p1;

        locals.var_fn382_calc_iq__cgin = locals.var_cgt;
        locals.var_fn382_calc_iq__cgin_dn4 = locals.var_cgt_dn4;

        locals.var_fn382_calc_iq__vto = p.p35;

        locals.var_fn382_calc_iq__ss = p.p36;

        locals.var_fn382_calc_iq__delta1 = p.p37;

        locals.var_fn382_calc_iq__delta2 = p.p38;

        locals.var_fn382_calc_iq__nd = p.p40;

        locals.var_fn382_calc_iq__alpha = p.p41;

        locals.var_fn382_calc_iq__vel0 = p.p32;

        locals.var_fn382_calc_iq__mu0 = p.p33;

        locals.var_fn382_calc_iq__beta = p.p34;

        locals.var_fn382_calc_iq__mtheta = p.p44;

        locals.var_fn382_calc_iq__vtheta = p.p43;

        locals.var_fn382_calc_iq__vtzeta = p.p46;

        locals.var_fn382_calc_iq__dibsat = p.p39;

        locals.var_fn382_calc_iq__epsilon = p.p47;

        locals.var_fn382_calc_iq__vzeta = p.p45;

        locals.var_fn382_calc_iq__lambda = p.p42;

        locals.var_fn382_calc_iq__ngf = p.p2;

        locals.var_fn382_calc_iq__type = p.p6;

        locals.var_fn382_calc_iq__trapfracdl = locals.var_chargefrac;
        locals.var_fn382_calc_iq__trapfracdl_dn22 = locals.var_chargefrac_dn22;
        locals.var_fn382_calc_iq__trapfracdl_dn23 = locals.var_chargefrac_dn23;
        locals.var_fn382_calc_iq__trapfracdl_dn25 = locals.var_chargefrac_dn25;
        locals.var_fn382_calc_iq__trapfracdl_dn26 = locals.var_chargefrac_dn26;

        locals.var_fn382_calc_iq__alpha_phit = 0.0;
        locals.var_fn382_calc_iq__alpha_phit_dn4 = 0.0;

        locals.var_fn382_calc_iq__delta = 0.0;
        locals.var_fn382_calc_iq__delta_dn5 = 0.0;
        locals.var_fn382_calc_iq__delta_dn9 = 0.0;

        locals.var_fn382_calc_iq__n = 0.0;
        locals.var_fn382_calc_iq__n_dn4 = 0.0;
        locals.var_fn382_calc_iq__n_dn5 = 0.0;
        locals.var_fn382_calc_iq__n_dn9 = 0.0;

        locals.var_fn382_calc_iq__vtof = 0.0;
        locals.var_fn382_calc_iq__vtof_dn4 = 0.0;

        locals.var_fn382_calc_iq__vsatdibl = 0.0;
        locals.var_fn382_calc_iq__vsatdibl_dn5 = 0.0;
        locals.var_fn382_calc_iq__vsatdibl_dn9 = 0.0;

        locals.var_fn382_calc_iq__ffs = 0.0;
        locals.var_fn382_calc_iq__ffs_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffs_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffs_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffs_dn9 = 0.0;

        locals.var_fn382_calc_iq__two_n_phit = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_dn4 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_dn5 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_dn9 = 0.0;

        locals.var_fn382_calc_iq__qref = 0.0;
        locals.var_fn382_calc_iq__qref_dn4 = 0.0;
        locals.var_fn382_calc_iq__qref_dn5 = 0.0;
        locals.var_fn382_calc_iq__qref_dn9 = 0.0;

        locals.var_fn382_calc_iq__etas = 0.0;
        locals.var_fn382_calc_iq__etas_dn4 = 0.0;
        locals.var_fn382_calc_iq__etas_dn5 = 0.0;
        locals.var_fn382_calc_iq__etas_dn8 = 0.0;
        locals.var_fn382_calc_iq__etas_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvs = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn9 = 0.0;

        locals.var_fn382_calc_iq__muf = 0.0;
        locals.var_fn382_calc_iq__muf_dn4 = 0.0;
        locals.var_fn382_calc_iq__muf_dn5 = 0.0;
        locals.var_fn382_calc_iq__muf_dn8 = 0.0;
        locals.var_fn382_calc_iq__muf_dn9 = 0.0;

        locals.var_fn382_calc_iq__vx = 0.0;
        locals.var_fn382_calc_iq__vx_dn4 = 0.0;
        locals.var_fn382_calc_iq__vx_dn5 = 0.0;
        locals.var_fn382_calc_iq__vx_dn8 = 0.0;
        locals.var_fn382_calc_iq__vx_dn9 = 0.0;

        locals.var_fn382_calc_iq__vxf = 0.0;
        locals.var_fn382_calc_iq__vxf_dn4 = 0.0;
        locals.var_fn382_calc_iq__vxf_dn5 = 0.0;
        locals.var_fn382_calc_iq__vxf_dn8 = 0.0;
        locals.var_fn382_calc_iq__vxf_dn9 = 0.0;

        locals.var_fn382_calc_iq__n0 = 0.0;
        locals.var_fn382_calc_iq__n0_dn4 = 0.0;

        locals.var_fn382_calc_iq__ffs0 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn9 = 0.0;

        locals.var_fn382_calc_iq__two_n_phit0 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit0_dn4 = 0.0;

        locals.var_fn382_calc_iq__qref0 = 0.0;
        locals.var_fn382_calc_iq__qref0_dn4 = 0.0;

        locals.var_fn382_calc_iq__etas0 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn4 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn5 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn8 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvs0 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn9 = 0.0;

        locals.var_fn382_calc_iq__muf0 = 0.0;
        locals.var_fn382_calc_iq__muf0_dn4 = 0.0;

        locals.var_fn382_calc_iq__vx0 = 0.0;
        locals.var_fn382_calc_iq__vx0_dn4 = 0.0;

        locals.var_fn382_calc_iq__tfacmobin = 0.0;
        locals.var_fn382_calc_iq__tfacmobin_dn4 = 0.0;

        locals.var_fn382_calc_iq__ff = 0.0;
        locals.var_fn382_calc_iq__ff_dn4 = 0.0;
        locals.var_fn382_calc_iq__ff_dn5 = 0.0;
        locals.var_fn382_calc_iq__ff_dn8 = 0.0;
        locals.var_fn382_calc_iq__ff_dn9 = 0.0;

        locals.var_fn382_calc_iq__eta = 0.0;
        locals.var_fn382_calc_iq__eta_dn4 = 0.0;
        locals.var_fn382_calc_iq__eta_dn5 = 0.0;
        locals.var_fn382_calc_iq__eta_dn8 = 0.0;
        locals.var_fn382_calc_iq__eta_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvv = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn9 = 0.0;

        locals.var_fn382_calc_iq__ff0 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn4 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn5 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn8 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn9 = 0.0;

        locals.var_fn382_calc_iq__eta0 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn4 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn5 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn8 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvv0 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsats = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsats1 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsat = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn9 = 0.0;

        locals.var_fn382_calc_iq__fsd = 0.0;
        locals.var_fn382_calc_iq__fsd_dn4 = 0.0;
        locals.var_fn382_calc_iq__fsd_dn5 = 0.0;
        locals.var_fn382_calc_iq__fsd_dn8 = 0.0;
        locals.var_fn382_calc_iq__fsd_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdx = 0.0;
        locals.var_fn382_calc_iq__vdx_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdx_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdx_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdx_dn9 = 0.0;

        locals.var_fn382_calc_iq__fds = 0.0;
        locals.var_fn382_calc_iq__fds_dn4 = 0.0;
        locals.var_fn382_calc_iq__fds_dn5 = 0.0;
        locals.var_fn382_calc_iq__fds_dn8 = 0.0;
        locals.var_fn382_calc_iq__fds_dn9 = 0.0;

        locals.var_fn382_calc_iq__vsx = 0.0;
        locals.var_fn382_calc_iq__vsx_dn4 = 0.0;
        locals.var_fn382_calc_iq__vsx_dn5 = 0.0;
        locals.var_fn382_calc_iq__vsx_dn8 = 0.0;
        locals.var_fn382_calc_iq__vsx_dn9 = 0.0;

        locals.var_fn382_calc_iq__ffd = 0.0;
        locals.var_fn382_calc_iq__ffd_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffd_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffd_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffd_dn9 = 0.0;

        locals.var_fn382_calc_iq__etad = 0.0;
        locals.var_fn382_calc_iq__etad_dn4 = 0.0;
        locals.var_fn382_calc_iq__etad_dn5 = 0.0;
        locals.var_fn382_calc_iq__etad_dn8 = 0.0;
        locals.var_fn382_calc_iq__etad_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvd = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsc = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn9 = 0.0;

        locals.var_fn382_calc_iq__fsat = 0.0;
        locals.var_fn382_calc_iq__fsat_dn4 = 0.0;
        locals.var_fn382_calc_iq__fsat_dn5 = 0.0;
        locals.var_fn382_calc_iq__fsat_dn8 = 0.0;
        locals.var_fn382_calc_iq__fsat_dn9 = 0.0;

        locals.var_fn382_calc_iq__vel = 0.0;
        locals.var_fn382_calc_iq__vel_dn4 = 0.0;
        locals.var_fn382_calc_iq__vel_dn5 = 0.0;
        locals.var_fn382_calc_iq__vel_dn8 = 0.0;
        locals.var_fn382_calc_iq__vel_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsats0 = 0.0;
        locals.var_fn382_calc_iq__vdsats0_dn4 = 0.0;

        locals.var_fn382_calc_iq__vdsats10 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdsat10 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn9 = 0.0;

        locals.var_fn382_calc_iq__fsd0 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn4 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn5 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn8 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn9 = 0.0;

        locals.var_fn382_calc_iq__vdx0 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn9 = 0.0;

        locals.var_fn382_calc_iq__fds0 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn4 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn5 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn8 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn9 = 0.0;

        locals.var_fn382_calc_iq__vsx0 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn4 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn5 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn8 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn9 = 0.0;

        locals.var_fn382_calc_iq__ffd0 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn9 = 0.0;

        locals.var_fn382_calc_iq__etad0 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn4 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn5 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn8 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvd0 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn9 = 0.0;

        locals.var_fn382_calc_iq__qs2 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn4 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn5 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn8 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn9 = 0.0;

        locals.var_fn382_calc_iq__qs3 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn4 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn5 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn8 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn9 = 0.0;

        locals.var_fn382_calc_iq__qd2 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn9 = 0.0;

        locals.var_fn382_calc_iq__qd3 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn9 = 0.0;

    }

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_fn382_calc_iq__qsqd = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn9 = 0.0;

        locals.var_fn382_calc_iq__qinvdd = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn9 = 0.0;

        locals.var_fn382_calc_iq__qd1 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn9 = 0.0;

        locals.var_fn382_calc_iq__qs = 0.0;
        locals.var_fn382_calc_iq__qs_dn4 = 0.0;
        locals.var_fn382_calc_iq__qs_dn5 = 0.0;
        locals.var_fn382_calc_iq__qs_dn8 = 0.0;
        locals.var_fn382_calc_iq__qs_dn9 = 0.0;

        locals.var_fn382_calc_iq__qd = 0.0;
        locals.var_fn382_calc_iq__qd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd_dn9 = 0.0;

        locals.var_fn382_calc_iq__etac = 0.0;
        locals.var_fn382_calc_iq__etac_dn4 = 0.0;

        locals.var_fn382_calc_iq__etab = 0.0;
        locals.var_fn382_calc_iq__etab_dn4 = 0.0;

        locals.var_fn382_calc_iq__etags = 0.0;
        locals.var_fn382_calc_iq__etags_dn4 = 0.0;
        locals.var_fn382_calc_iq__etags_dn8 = 0.0;
        locals.var_fn382_calc_iq__etags_dn9 = 0.0;

        locals.var_fn382_calc_iq__exparg = 0.0;
        locals.var_fn382_calc_iq__exparg_dn4 = 0.0;
        locals.var_fn382_calc_iq__exparg_dn5 = 0.0;
        locals.var_fn382_calc_iq__exparg_dn8 = 0.0;
        locals.var_fn382_calc_iq__exparg_dn9 = 0.0;

        locals.var_fn382_calc_iq__myarg = 0.0;
        locals.var_fn382_calc_iq__myarg_dn4 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn5 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn8 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn9 = 0.0;

        locals.var_fn382_calc_iq__absvdsin = 0.0;
        locals.var_fn382_calc_iq__absvdsin_dn5 = 0.0;
        locals.var_fn382_calc_iq__absvdsin_dn9 = 0.0;

        locals.var_fn382_calc_iq__vgdin = 0.0;
        locals.var_fn382_calc_iq__vgdin_dn5 = 0.0;
        locals.var_fn382_calc_iq__vgdin_dn8 = 0.0;
        locals.var_fn382_calc_iq__vgdin_dn9 = 0.0;

        locals.var_fn382_calc_iq__exparg0 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn4 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn5 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn8 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn9 = 0.0;

        locals.var_fn382_calc_iq__myarg0 = 0.0;
        locals.var_fn382_calc_iq__myarg0_dn4 = 0.0;

        let (assign31170_e28252, assign31170_e28252_d_n5, assign31170_e28252_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31170_e28236: f64 = (0.001 / p.p53);
        let assign31170_e28238: f64 = (assign31170_e28236 * locals.var_fn382_calc_iq__vdsin);
        let assign31170_e28239: f64 = (assign31170_e28238).tanh();
        let assign31170_e28240: f64 = (locals.var_fn382_calc_iq__vdsin * assign31170_e28239);
        (assign31170_e28240, ((locals.var_fn382_calc_iq__vdsin_dn5 * assign31170_e28239) + (locals.var_fn382_calc_iq__vdsin * ((assign31170_e28236 * locals.var_fn382_calc_iq__vdsin_dn5) / ((assign31170_e28238).cosh() * (assign31170_e28238).cosh())))), ((locals.var_fn382_calc_iq__vdsin_dn9 * assign31170_e28239) + (locals.var_fn382_calc_iq__vdsin * ((assign31170_e28236 * locals.var_fn382_calc_iq__vdsin_dn9) / ((assign31170_e28238).cosh() * (assign31170_e28238).cosh())))),)
    } else {
        let (assign31170_e28251, assign31170_e28251_d_n5, assign31170_e28251_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31170_e28246: f64 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsin);
                let assign31170_e28248: f64 = (assign31170_e28246 + p.p53);
                let assign31170_e28249: f64 = (assign31170_e28248).sqrt();
                (assign31170_e28249, (((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsin) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsin_dn5)) / (2.0 * assign31170_e28249)), (((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsin) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsin_dn9)) / (2.0 * assign31170_e28249)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (assign31170_e28251, assign31170_e28251_d_n5, assign31170_e28251_d_n9,)
    }
};
        locals.var_fn382_calc_iq__absvdsin = assign31170_e28252;
        locals.var_fn382_calc_iq__absvdsin_dn5 = assign31170_e28252_d_n5;
        locals.var_fn382_calc_iq__absvdsin_dn9 = assign31170_e28252_d_n9;

        let assign31180_e28255: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vdsin);
        locals.var_fn382_calc_iq__vgdin = assign31180_e28255;
        locals.var_fn382_calc_iq__vgdin_dn5 = (-locals.var_fn382_calc_iq__vdsin_dn5);
        locals.var_fn382_calc_iq__vgdin_dn8 = locals.var_fn382_calc_iq__vgsin_dn8;
        locals.var_fn382_calc_iq__vgdin_dn9 = (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vdsin_dn9);

        let assign31190_e28258: f64 = (locals.var_fn382_calc_iq__alpha * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__alpha_phit = assign31190_e28258;
        locals.var_fn382_calc_iq__alpha_phit_dn4 = (locals.var_fn382_calc_iq__alpha * locals.var_fn382_calc_iq__phitin_dn4);

        let assign31200_e28262: f64 = (2.302585092994046 * locals.var_fn382_calc_iq__phitin);
        let assign31200_e28263: f64 = (locals.var_fn382_calc_iq__ss / assign31200_e28262);
        let assign31200_e28266: f64 = (locals.var_fn382_calc_iq__nd * locals.var_fn382_calc_iq__absvdsin);
        let assign31200_e28267: f64 = (assign31200_e28263 + assign31200_e28266);
        locals.var_fn382_calc_iq__n = assign31200_e28267;
        locals.var_fn382_calc_iq__n_dn4 = (-((locals.var_fn382_calc_iq__ss * (2.302585092994046 * locals.var_fn382_calc_iq__phitin_dn4)) / (assign31200_e28262 * assign31200_e28262)));
        locals.var_fn382_calc_iq__n_dn5 = (locals.var_fn382_calc_iq__nd * locals.var_fn382_calc_iq__absvdsin_dn5);
        locals.var_fn382_calc_iq__n_dn9 = (locals.var_fn382_calc_iq__nd * locals.var_fn382_calc_iq__absvdsin_dn9);

        let assign31210_e28272: f64 = (locals.var_fn382_calc_iq__tambin - locals.var_fn382_calc_iq__tnomin);
        let assign31210_e28273: f64 = (locals.var_fn382_calc_iq__vtzeta * assign31210_e28272);
        let assign31210_e28274: f64 = (locals.var_fn382_calc_iq__vto + assign31210_e28273);
        locals.var_fn382_calc_iq__vtof = assign31210_e28274;
        locals.var_fn382_calc_iq__vtof_dn4 = (locals.var_fn382_calc_iq__vtzeta * locals.var_fn382_calc_iq__tambin_dn4);

        let assign31220_e28277: f64 = (locals.var_fn382_calc_iq__tambin / locals.var_fn382_calc_iq__tnomin);
        let assign31220_e28279: f64 = (assign31220_e28277).powf(locals.var_fn382_calc_iq__epsilon);
        locals.var_fn382_calc_iq__tfacmobin = assign31220_e28279;
        locals.var_fn382_calc_iq__tfacmobin_dn4 = if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn382_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__epsilon * ((assign31220_e28277).powf(locals.var_fn382_calc_iq__epsilon - 1.0) * (locals.var_fn382_calc_iq__tambin_dn4 / locals.var_fn382_calc_iq__tnomin))) } } else { (assign31220_e28279 * (locals.var_fn382_calc_iq__epsilon * ((locals.var_fn382_calc_iq__tambin_dn4 / locals.var_fn382_calc_iq__tnomin) / assign31220_e28277))) };

        let assign31230_e28282: f64 = if locals.var_fn382_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign31230_e28282;

        let (assign31240_e28298, assign31240_e28298_d_n5, assign31240_e28298_d_n9,) = {
    if (locals.var_guard383 != 0.0) {
        let assign31240_e28288: f64 = (locals.var_fn382_calc_iq__absvdsin / locals.var_fn382_calc_iq__dibsat);
        let assign31240_e28290: f64 = (assign31240_e28288).powf(locals.var_fn382_calc_iq__beta);
        let assign31240_e28291: f64 = (1.0 + assign31240_e28290);
        let assign31240_e28294: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31240_e28295: f64 = (assign31240_e28291).powf(assign31240_e28294);
        let assign31240_e28296: f64 = (locals.var_fn382_calc_iq__absvdsin / assign31240_e28295);
        (assign31240_e28296, (((locals.var_fn382_calc_iq__absvdsin_dn5 * assign31240_e28295) - (locals.var_fn382_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign31240_e28294) as f64).is_finite() && ((assign31240_e28294) as f64).fract() == 0.0 { if assign31240_e28294 == 0.0 { 0.0 } else { (assign31240_e28294 * ((assign31240_e28291).powf(assign31240_e28294 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) })) } } else { (assign31240_e28295 * (assign31240_e28294 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) } / assign31240_e28291))) })) / (assign31240_e28295 * assign31240_e28295)), (((locals.var_fn382_calc_iq__absvdsin_dn9 * assign31240_e28295) - (locals.var_fn382_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign31240_e28294) as f64).is_finite() && ((assign31240_e28294) as f64).fract() == 0.0 { if assign31240_e28294 == 0.0 { 0.0 } else { (assign31240_e28294 * ((assign31240_e28291).powf(assign31240_e28294 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) })) } } else { (assign31240_e28295 * (assign31240_e28294 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) } / assign31240_e28291))) })) / (assign31240_e28295 * assign31240_e28295)),)
    } else {
        (locals.var_fn382_calc_iq__vsatdibl, locals.var_fn382_calc_iq__vsatdibl_dn5, locals.var_fn382_calc_iq__vsatdibl_dn9,)
    }
};
        locals.var_fn382_calc_iq__vsatdibl = assign31240_e28298;
        locals.var_fn382_calc_iq__vsatdibl_dn5 = assign31240_e28298_d_n5;
        locals.var_fn382_calc_iq__vsatdibl_dn9 = assign31240_e28298_d_n9;

        let (assign31250_e28303, assign31250_e28303_d_n5, assign31250_e28303_d_n9,) = {
    if (locals.var_guard383 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__vsatdibl, locals.var_fn382_calc_iq__vsatdibl_dn5, locals.var_fn382_calc_iq__vsatdibl_dn9,)
    }
};
        locals.var_fn382_calc_iq__vsatdibl = assign31250_e28303;
        locals.var_fn382_calc_iq__vsatdibl_dn5 = assign31250_e28303_d_n5;
        locals.var_fn382_calc_iq__vsatdibl_dn9 = assign31250_e28303_d_n9;

        let assign31260_e28307: f64 = (locals.var_fn382_calc_iq__vsatdibl * locals.var_fn382_calc_iq__delta2);
        let assign31260_e28308: f64 = (locals.var_fn382_calc_iq__delta1 - assign31260_e28307);
        let assign31260_e28310: f64 = (assign31260_e28308 * locals.var_fn382_calc_iq__absvdsin);
        locals.var_fn382_calc_iq__delta = assign31260_e28310;
        locals.var_fn382_calc_iq__delta_dn5 = (((-(locals.var_fn382_calc_iq__vsatdibl_dn5 * locals.var_fn382_calc_iq__delta2)) * locals.var_fn382_calc_iq__absvdsin) + (assign31260_e28308 * locals.var_fn382_calc_iq__absvdsin_dn5));
        locals.var_fn382_calc_iq__delta_dn9 = (((-(locals.var_fn382_calc_iq__vsatdibl_dn9 * locals.var_fn382_calc_iq__delta2)) * locals.var_fn382_calc_iq__absvdsin) + (assign31260_e28308 * locals.var_fn382_calc_iq__absvdsin_dn9));

        let assign31270_e28313: f64 = (locals.var_fn382_calc_iq__vtof - locals.var_fn382_calc_iq__delta);
        locals.var_fn382_calc_iq__vtdibl = assign31270_e28313;
        locals.var_fn382_calc_iq__vtdibl_dn4 = locals.var_fn382_calc_iq__vtof_dn4;
        locals.var_fn382_calc_iq__vtdibl_dn5 = (-locals.var_fn382_calc_iq__delta_dn5);
        locals.var_fn382_calc_iq__vtdibl_dn9 = (-locals.var_fn382_calc_iq__delta_dn9);

        let assign31280_e28316: f64 = (2.0 * locals.var_fn382_calc_iq__n);
        let assign31280_e28318: f64 = (assign31280_e28316 * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit = assign31280_e28318;
        locals.var_fn382_calc_iq__two_n_phit_dn4 = (((2.0 * locals.var_fn382_calc_iq__n_dn4) * locals.var_fn382_calc_iq__phitin) + (assign31280_e28316 * locals.var_fn382_calc_iq__phitin_dn4));
        locals.var_fn382_calc_iq__two_n_phit_dn5 = ((2.0 * locals.var_fn382_calc_iq__n_dn5) * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit_dn9 = ((2.0 * locals.var_fn382_calc_iq__n_dn9) * locals.var_fn382_calc_iq__phitin);

        let assign31290_e28321: f64 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__qref = assign31290_e28321;
        locals.var_fn382_calc_iq__qref_dn4 = ((locals.var_fn382_calc_iq__cgin_dn4 * locals.var_fn382_calc_iq__two_n_phit) + (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit_dn4));
        locals.var_fn382_calc_iq__qref_dn5 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit_dn5);
        locals.var_fn382_calc_iq__qref_dn9 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit_dn9);

        let assign31300_e28325: f64 = (p.p51 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31300_e28327: f64 = (assign31300_e28325 / 2.0);
        let assign31300_e28328: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31300_e28327);
        locals.var_fn382_calc_iq__myarg = assign31300_e28328;
        locals.var_fn382_calc_iq__myarg_dn4 = (locals.var_fn382_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn382_calc_iq__alpha_phit_dn4) / 2.0));
        locals.var_fn382_calc_iq__myarg_dn5 = locals.var_fn382_calc_iq__vtdibl_dn5;
        locals.var_fn382_calc_iq__myarg_dn8 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn9 = locals.var_fn382_calc_iq__vtdibl_dn9;

        let (assign31310_e28372, assign31310_e28372_d_n5, assign31310_e28372_d_n8, assign31310_e28372_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31310_e28336: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31310_e28339: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31310_e28342: f64 = (0.001 / p.p53);
        let assign31310_e28345: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31310_e28346: f64 = (assign31310_e28342 * assign31310_e28345);
        let assign31310_e28347: f64 = (assign31310_e28346).tanh();
        let assign31310_e28348: f64 = (assign31310_e28339 * assign31310_e28347);
        let assign31310_e28349: f64 = (assign31310_e28336 + assign31310_e28348);
        let assign31310_e28350: f64 = (0.5 * assign31310_e28349);
        (assign31310_e28350, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31310_e28347) + (assign31310_e28339 * ((assign31310_e28342 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31310_e28346).cosh() * (assign31310_e28346).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31310_e28347) + (assign31310_e28339 * ((assign31310_e28342 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31310_e28346).cosh() * (assign31310_e28346).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31310_e28347) + (assign31310_e28339 * ((assign31310_e28342 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31310_e28346).cosh() * (assign31310_e28346).cosh())))))),)
    } else {
        let (assign31310_e28371, assign31310_e28371_d_n5, assign31310_e28371_d_n8, assign31310_e28371_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31310_e28357: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31310_e28360: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31310_e28363: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31310_e28364: f64 = (assign31310_e28360 * assign31310_e28363);
                let assign31310_e28366: f64 = (assign31310_e28364 + p.p53);
                let assign31310_e28367: f64 = (assign31310_e28366).sqrt();
                let assign31310_e28368: f64 = (assign31310_e28357 + assign31310_e28367);
                let assign31310_e28369: f64 = (0.5 * assign31310_e28368);
                (assign31310_e28369, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31310_e28363) + (assign31310_e28360 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31310_e28367)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31310_e28363) + (assign31310_e28360 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31310_e28367)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31310_e28363) + (assign31310_e28360 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31310_e28367)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31310_e28371, assign31310_e28371_d_n5, assign31310_e28371_d_n8, assign31310_e28371_d_n9,)
    }
};
        let assign31310_e28374: f64 = (assign31310_e28372 - locals.var_fn382_calc_iq__myarg);
        let assign31310_e28376: f64 = (assign31310_e28374 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg = assign31310_e28376;
        locals.var_fn382_calc_iq__exparg_dn4 = ((((-locals.var_fn382_calc_iq__myarg_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31310_e28374 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg_dn5 = ((assign31310_e28372_d_n5 - locals.var_fn382_calc_iq__myarg_dn5) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn8 = ((assign31310_e28372_d_n8 - locals.var_fn382_calc_iq__myarg_dn8) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn9 = ((assign31310_e28372_d_n9 - locals.var_fn382_calc_iq__myarg_dn9) / locals.var_fn382_calc_iq__alpha_phit);

        let assign31320_e28379: f64 = if locals.var_fn382_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign31320_e28379;

        let (assign31330_e28383, assign31330_e28383_d_n4, assign31330_e28383_d_n5, assign31330_e28383_d_n8, assign31330_e28383_d_n9,) = {
    if (locals.var_guard384 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff, locals.var_fn382_calc_iq__ff_dn4, locals.var_fn382_calc_iq__ff_dn5, locals.var_fn382_calc_iq__ff_dn8, locals.var_fn382_calc_iq__ff_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff = assign31330_e28383;
        locals.var_fn382_calc_iq__ff_dn4 = assign31330_e28383_d_n4;
        locals.var_fn382_calc_iq__ff_dn5 = assign31330_e28383_d_n5;
        locals.var_fn382_calc_iq__ff_dn8 = assign31330_e28383_d_n8;
        locals.var_fn382_calc_iq__ff_dn9 = assign31330_e28383_d_n9;

        let assign31340_e28386: f64 = (-50.0);
        let assign31340_e28387: f64 = if locals.var_fn382_calc_iq__exparg < assign31340_e28386 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign31340_e28387;

        let (assign31350_e28394, assign31350_e28394_d_n4, assign31350_e28394_d_n5, assign31350_e28394_d_n8, assign31350_e28394_d_n9,) = {
    if ((locals.var_guard384 == 0.0) && (locals.var_guard385 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff, locals.var_fn382_calc_iq__ff_dn4, locals.var_fn382_calc_iq__ff_dn5, locals.var_fn382_calc_iq__ff_dn8, locals.var_fn382_calc_iq__ff_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff = assign31350_e28394;
        locals.var_fn382_calc_iq__ff_dn4 = assign31350_e28394_d_n4;
        locals.var_fn382_calc_iq__ff_dn5 = assign31350_e28394_d_n5;
        locals.var_fn382_calc_iq__ff_dn8 = assign31350_e28394_d_n8;
        locals.var_fn382_calc_iq__ff_dn9 = assign31350_e28394_d_n9;

        let (assign31360_e28407, assign31360_e28407_d_n4, assign31360_e28407_d_n5, assign31360_e28407_d_n8, assign31360_e28407_d_n9,) = {
    if ((locals.var_guard384 == 0.0) && (locals.var_guard385 == 0.0)) {
        let assign31360_e28403: f64 = (locals.var_fn382_calc_iq__exparg).exp();
        let assign31360_e28404: f64 = (1.0 + assign31360_e28403);
        let assign31360_e28405: f64 = (1.0 / assign31360_e28404);
        (assign31360_e28405, (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn4) / (assign31360_e28404 * assign31360_e28404))), (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn5) / (assign31360_e28404 * assign31360_e28404))), (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn8) / (assign31360_e28404 * assign31360_e28404))), (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn9) / (assign31360_e28404 * assign31360_e28404))),)
    } else {
        (locals.var_fn382_calc_iq__ff, locals.var_fn382_calc_iq__ff_dn4, locals.var_fn382_calc_iq__ff_dn5, locals.var_fn382_calc_iq__ff_dn8, locals.var_fn382_calc_iq__ff_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff = assign31360_e28407;
        locals.var_fn382_calc_iq__ff_dn4 = assign31360_e28407_d_n4;
        locals.var_fn382_calc_iq__ff_dn5 = assign31360_e28407_d_n5;
        locals.var_fn382_calc_iq__ff_dn8 = assign31360_e28407_d_n8;
        locals.var_fn382_calc_iq__ff_dn9 = assign31360_e28407_d_n9;

        let (assign31370_e28451, assign31370_e28451_d_n5, assign31370_e28451_d_n8, assign31370_e28451_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31370_e28415: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31370_e28418: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31370_e28421: f64 = (0.001 / p.p53);
        let assign31370_e28424: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31370_e28425: f64 = (assign31370_e28421 * assign31370_e28424);
        let assign31370_e28426: f64 = (assign31370_e28425).tanh();
        let assign31370_e28427: f64 = (assign31370_e28418 * assign31370_e28426);
        let assign31370_e28428: f64 = (assign31370_e28415 + assign31370_e28427);
        let assign31370_e28429: f64 = (0.5 * assign31370_e28428);
        (assign31370_e28429, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31370_e28426) + (assign31370_e28418 * ((assign31370_e28421 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31370_e28425).cosh() * (assign31370_e28425).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31370_e28426) + (assign31370_e28418 * ((assign31370_e28421 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31370_e28425).cosh() * (assign31370_e28425).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31370_e28426) + (assign31370_e28418 * ((assign31370_e28421 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31370_e28425).cosh() * (assign31370_e28425).cosh())))))),)
    } else {
        let (assign31370_e28450, assign31370_e28450_d_n5, assign31370_e28450_d_n8, assign31370_e28450_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31370_e28436: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31370_e28439: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31370_e28442: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31370_e28443: f64 = (assign31370_e28439 * assign31370_e28442);
                let assign31370_e28445: f64 = (assign31370_e28443 + p.p53);
                let assign31370_e28446: f64 = (assign31370_e28445).sqrt();
                let assign31370_e28447: f64 = (assign31370_e28436 + assign31370_e28446);
                let assign31370_e28448: f64 = (0.5 * assign31370_e28447);
                (assign31370_e28448, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31370_e28442) + (assign31370_e28439 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31370_e28446)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31370_e28442) + (assign31370_e28439 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31370_e28446)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31370_e28442) + (assign31370_e28439 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31370_e28446)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31370_e28450, assign31370_e28450_d_n5, assign31370_e28450_d_n8, assign31370_e28450_d_n9,)
    }
};
        let assign31370_e28455: f64 = (p.p51 * 0.1);
        let assign31370_e28457: f64 = (assign31370_e28455 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31370_e28459: f64 = (assign31370_e28457 * locals.var_fn382_calc_iq__ff);
        let assign31370_e28460: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31370_e28459);
        let assign31370_e28461: f64 = (assign31370_e28451 - assign31370_e28460);
        let assign31370_e28463: f64 = (assign31370_e28461 / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__eta = assign31370_e28463;
        locals.var_fn382_calc_iq__eta_dn4 = ((((-(locals.var_fn382_calc_iq__vtdibl_dn4 - (((assign31370_e28455 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ff) + (assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn4)))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31370_e28461 * locals.var_fn382_calc_iq__two_n_phit_dn4)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__eta_dn5 = ((((assign31370_e28451_d_n5 - (locals.var_fn382_calc_iq__vtdibl_dn5 - (assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn5))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31370_e28461 * locals.var_fn382_calc_iq__two_n_phit_dn5)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__eta_dn8 = ((assign31370_e28451_d_n8 - (-(assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn8))) / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__eta_dn9 = ((((assign31370_e28451_d_n9 - (locals.var_fn382_calc_iq__vtdibl_dn9 - (assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn9))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31370_e28461 * locals.var_fn382_calc_iq__two_n_phit_dn9)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));

        let assign31380_e28466: f64 = if locals.var_fn382_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign31380_e28466;

        let (assign31390_e28472, assign31390_e28472_d_n4, assign31390_e28472_d_n5, assign31390_e28472_d_n8, assign31390_e28472_d_n9,) = {
    if (locals.var_guard386 != 0.0) {
        let assign31390_e28470: f64 = (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta);
        (assign31390_e28470, ((locals.var_fn382_calc_iq__qref_dn4 * locals.var_fn382_calc_iq__eta) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn4)), ((locals.var_fn382_calc_iq__qref_dn5 * locals.var_fn382_calc_iq__eta) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn5)), (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn8), ((locals.var_fn382_calc_iq__qref_dn9 * locals.var_fn382_calc_iq__eta) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvv, locals.var_fn382_calc_iq__qinvv_dn4, locals.var_fn382_calc_iq__qinvv_dn5, locals.var_fn382_calc_iq__qinvv_dn8, locals.var_fn382_calc_iq__qinvv_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv = assign31390_e28472;
        locals.var_fn382_calc_iq__qinvv_dn4 = assign31390_e28472_d_n4;
        locals.var_fn382_calc_iq__qinvv_dn5 = assign31390_e28472_d_n5;
        locals.var_fn382_calc_iq__qinvv_dn8 = assign31390_e28472_d_n8;
        locals.var_fn382_calc_iq__qinvv_dn9 = assign31390_e28472_d_n9;

        let assign31400_e28475: f64 = (-50.0);
        let assign31400_e28476: f64 = if locals.var_fn382_calc_iq__eta < assign31400_e28475 { 1.0 } else { 0.0 };
        locals.var_guard387 = assign31400_e28476;

        let (assign31410_e28486, assign31410_e28486_d_n4, assign31410_e28486_d_n5, assign31410_e28486_d_n8, assign31410_e28486_d_n9,) = {
    if ((locals.var_guard386 == 0.0) && (locals.var_guard387 != 0.0)) {
        let assign31410_e28483: f64 = (locals.var_fn382_calc_iq__eta).exp();
        let assign31410_e28484: f64 = (locals.var_fn382_calc_iq__qref * assign31410_e28483);
        (assign31410_e28484, ((locals.var_fn382_calc_iq__qref_dn4 * assign31410_e28483) + (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn4))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31410_e28483) + (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn5))), (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn8)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31410_e28483) + (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn9))),)
    } else {
        (locals.var_fn382_calc_iq__qinvv, locals.var_fn382_calc_iq__qinvv_dn4, locals.var_fn382_calc_iq__qinvv_dn5, locals.var_fn382_calc_iq__qinvv_dn8, locals.var_fn382_calc_iq__qinvv_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv = assign31410_e28486;
        locals.var_fn382_calc_iq__qinvv_dn4 = assign31410_e28486_d_n4;
        locals.var_fn382_calc_iq__qinvv_dn5 = assign31410_e28486_d_n5;
        locals.var_fn382_calc_iq__qinvv_dn8 = assign31410_e28486_d_n8;
        locals.var_fn382_calc_iq__qinvv_dn9 = assign31410_e28486_d_n9;

        let (assign31420_e28500, assign31420_e28500_d_n4, assign31420_e28500_d_n5, assign31420_e28500_d_n8, assign31420_e28500_d_n9,) = {
    if ((locals.var_guard386 == 0.0) && (locals.var_guard387 == 0.0)) {
        let assign31420_e28495: f64 = (locals.var_fn382_calc_iq__eta).exp();
        let assign31420_e28496: f64 = (1.0 + assign31420_e28495);
        let assign31420_e28497: f64 = (assign31420_e28496).ln();
        let assign31420_e28498: f64 = (locals.var_fn382_calc_iq__qref * assign31420_e28497);
        (assign31420_e28498, ((locals.var_fn382_calc_iq__qref_dn4 * assign31420_e28497) + (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn4) / assign31420_e28496))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31420_e28497) + (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn5) / assign31420_e28496))), (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn8) / assign31420_e28496)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31420_e28497) + (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn9) / assign31420_e28496))),)
    } else {
        (locals.var_fn382_calc_iq__qinvv, locals.var_fn382_calc_iq__qinvv_dn4, locals.var_fn382_calc_iq__qinvv_dn5, locals.var_fn382_calc_iq__qinvv_dn8, locals.var_fn382_calc_iq__qinvv_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv = assign31420_e28500;
        locals.var_fn382_calc_iq__qinvv_dn4 = assign31420_e28500_d_n4;
        locals.var_fn382_calc_iq__qinvv_dn5 = assign31420_e28500_d_n5;
        locals.var_fn382_calc_iq__qinvv_dn8 = assign31420_e28500_d_n8;
        locals.var_fn382_calc_iq__qinvv_dn9 = assign31420_e28500_d_n9;

        let assign31430_e28506: f64 = (locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv);
        let assign31430_e28508: f64 = (assign31430_e28506 / locals.var_fn382_calc_iq__cgin);
        let assign31430_e28509: f64 = (1.0 + assign31430_e28508);
        let assign31430_e28510: f64 = (locals.var_fn382_calc_iq__tfacmobin * assign31430_e28509);
        let assign31430_e28511: f64 = (locals.var_fn382_calc_iq__mu0 / assign31430_e28510);
        locals.var_fn382_calc_iq__muf = assign31430_e28511;
        locals.var_fn382_calc_iq__muf_dn4 = (-((locals.var_fn382_calc_iq__mu0 * ((locals.var_fn382_calc_iq__tfacmobin_dn4 * assign31430_e28509) + (locals.var_fn382_calc_iq__tfacmobin * ((((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31430_e28506 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin))))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_dn5 = (-((locals.var_fn382_calc_iq__mu0 * (locals.var_fn382_calc_iq__tfacmobin * ((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn5) / locals.var_fn382_calc_iq__cgin))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_dn8 = (-((locals.var_fn382_calc_iq__mu0 * (locals.var_fn382_calc_iq__tfacmobin * ((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn8) / locals.var_fn382_calc_iq__cgin))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_dn9 = (-((locals.var_fn382_calc_iq__mu0 * (locals.var_fn382_calc_iq__tfacmobin * ((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn9) / locals.var_fn382_calc_iq__cgin))) / (assign31430_e28510 * assign31430_e28510)));

        let assign31440_e28516: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tnomin);
        let assign31440_e28517: f64 = (1.0 + assign31440_e28516);
        let assign31440_e28521: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin);
        let assign31440_e28522: f64 = (1.0 + assign31440_e28521);
        let assign31440_e28523: f64 = (assign31440_e28517 / assign31440_e28522);
        let assign31440_e28524: f64 = (locals.var_fn382_calc_iq__vel0 * assign31440_e28523);
        let assign31440_e28528: f64 = (locals.var_fn382_calc_iq__lambda * locals.var_fn382_calc_iq__absvdsin);
        let assign31440_e28530: f64 = (assign31440_e28528 / locals.var_fn382_calc_iq__lin);
        let assign31440_e28531: f64 = (1.0 + assign31440_e28530);
        let assign31440_e28532: f64 = (assign31440_e28524 * assign31440_e28531);
        let assign31440_e28536: f64 = (locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv);
        let assign31440_e28538: f64 = (assign31440_e28536 / locals.var_fn382_calc_iq__cgin);
        let assign31440_e28539: f64 = (1.0 + assign31440_e28538);
        let assign31440_e28540: f64 = (assign31440_e28532 / assign31440_e28539);
        locals.var_fn382_calc_iq__vx = assign31440_e28540;
        locals.var_fn382_calc_iq__vx_dn4 = (((((locals.var_fn382_calc_iq__vel0 * (-((assign31440_e28517 * (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin_dn4)) / (assign31440_e28522 * assign31440_e28522)))) * assign31440_e28531) * assign31440_e28539) - (assign31440_e28532 * ((((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31440_e28536 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin)))) / (assign31440_e28539 * assign31440_e28539));
        locals.var_fn382_calc_iq__vx_dn5 = ((((assign31440_e28524 * ((locals.var_fn382_calc_iq__lambda * locals.var_fn382_calc_iq__absvdsin_dn5) / locals.var_fn382_calc_iq__lin)) * assign31440_e28539) - (assign31440_e28532 * ((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn5) / locals.var_fn382_calc_iq__cgin))) / (assign31440_e28539 * assign31440_e28539));
        locals.var_fn382_calc_iq__vx_dn8 = (-((assign31440_e28532 * ((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn8) / locals.var_fn382_calc_iq__cgin)) / (assign31440_e28539 * assign31440_e28539)));
        locals.var_fn382_calc_iq__vx_dn9 = ((((assign31440_e28524 * ((locals.var_fn382_calc_iq__lambda * locals.var_fn382_calc_iq__absvdsin_dn9) / locals.var_fn382_calc_iq__lin)) * assign31440_e28539) - (assign31440_e28532 * ((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn9) / locals.var_fn382_calc_iq__cgin))) / (assign31440_e28539 * assign31440_e28539));

        let assign31450_e28543: f64 = (2.0 * locals.var_fn382_calc_iq__ff);
        let assign31450_e28545: f64 = (assign31450_e28543 * locals.var_fn382_calc_iq__phitin);
        let assign31450_e28547: f64 = (assign31450_e28545 * locals.var_fn382_calc_iq__muf);
        let assign31450_e28549: f64 = (assign31450_e28547 / locals.var_fn382_calc_iq__lin);
        let assign31450_e28552: f64 = (1.0 - locals.var_fn382_calc_iq__ff);
        let assign31450_e28554: f64 = (assign31450_e28552 * locals.var_fn382_calc_iq__vx);
        let assign31450_e28555: f64 = (assign31450_e28549 + assign31450_e28554);
        locals.var_fn382_calc_iq__vxf = assign31450_e28555;
        locals.var_fn382_calc_iq__vxf_dn4 = (((((((2.0 * locals.var_fn382_calc_iq__ff_dn4) * locals.var_fn382_calc_iq__phitin) + (assign31450_e28543 * locals.var_fn382_calc_iq__phitin_dn4)) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn4)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn4) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn4)));
        locals.var_fn382_calc_iq__vxf_dn5 = ((((((2.0 * locals.var_fn382_calc_iq__ff_dn5) * locals.var_fn382_calc_iq__phitin) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn5)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn5) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn5)));
        locals.var_fn382_calc_iq__vxf_dn8 = ((((((2.0 * locals.var_fn382_calc_iq__ff_dn8) * locals.var_fn382_calc_iq__phitin) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn8)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn8) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn8)));
        locals.var_fn382_calc_iq__vxf_dn9 = ((((((2.0 * locals.var_fn382_calc_iq__ff_dn9) * locals.var_fn382_calc_iq__phitin) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn9)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn9) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn9)));

        let assign31460_e28558: f64 = (locals.var_fn382_calc_iq__vx * locals.var_fn382_calc_iq__lin);
        let assign31460_e28560: f64 = (assign31460_e28558 / locals.var_fn382_calc_iq__muf);
        locals.var_fn382_calc_iq__vdsats = assign31460_e28560;
        locals.var_fn382_calc_iq__vdsats_dn4 = ((((locals.var_fn382_calc_iq__vx_dn4 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn4)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_dn5 = ((((locals.var_fn382_calc_iq__vx_dn5 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn5)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_dn8 = ((((locals.var_fn382_calc_iq__vx_dn8 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn8)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_dn9 = ((((locals.var_fn382_calc_iq__vx_dn9 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn9)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));

        let assign31470_e28565: f64 = (2.0 * locals.var_fn382_calc_iq__qinvv);
        let assign31470_e28567: f64 = (assign31470_e28565 / locals.var_fn382_calc_iq__cgin);
        let assign31470_e28569: f64 = (assign31470_e28567 / locals.var_fn382_calc_iq__vdsats);
        let assign31470_e28570: f64 = (1.0 + assign31470_e28569);
        let assign31470_e28571: f64 = (assign31470_e28570).sqrt();
        let assign31470_e28572: f64 = (locals.var_fn382_calc_iq__vdsats * assign31470_e28571);
        let assign31470_e28574: f64 = (assign31470_e28572 - locals.var_fn382_calc_iq__vdsats);
        locals.var_fn382_calc_iq__vdsats1 = assign31470_e28574;
        locals.var_fn382_calc_iq__vdsats1_dn4 = (((locals.var_fn382_calc_iq__vdsats_dn4 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31470_e28565 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin)) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn4)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn4);
        locals.var_fn382_calc_iq__vdsats1_dn5 = (((locals.var_fn382_calc_iq__vdsats_dn5 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn5) / locals.var_fn382_calc_iq__cgin) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn5)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn5);
        locals.var_fn382_calc_iq__vdsats1_dn8 = (((locals.var_fn382_calc_iq__vdsats_dn8 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn8) / locals.var_fn382_calc_iq__cgin) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn8)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn8);
        locals.var_fn382_calc_iq__vdsats1_dn9 = (((locals.var_fn382_calc_iq__vdsats_dn9 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn9) / locals.var_fn382_calc_iq__cgin) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn9)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn9);

        let assign31480_e28578: f64 = (1.0 - locals.var_fn382_calc_iq__ff);
        let assign31480_e28579: f64 = (locals.var_fn382_calc_iq__vdsats * assign31480_e28578);
        let assign31480_e28582: f64 = (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff);
        let assign31480_e28583: f64 = (assign31480_e28579 + assign31480_e28582);
        locals.var_fn382_calc_iq__vdsat = assign31480_e28583;
        locals.var_fn382_calc_iq__vdsat_dn4 = (((locals.var_fn382_calc_iq__vdsats_dn4 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn4))) + ((locals.var_fn382_calc_iq__two_n_phit_dn4 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn4)));
        locals.var_fn382_calc_iq__vdsat_dn5 = (((locals.var_fn382_calc_iq__vdsats_dn5 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn5))) + ((locals.var_fn382_calc_iq__two_n_phit_dn5 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn5)));
        locals.var_fn382_calc_iq__vdsat_dn8 = (((locals.var_fn382_calc_iq__vdsats_dn8 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn8))) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn8));
        locals.var_fn382_calc_iq__vdsat_dn9 = (((locals.var_fn382_calc_iq__vdsats_dn9 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn9))) + ((locals.var_fn382_calc_iq__two_n_phit_dn9 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn9)));

        let assign31490_e28587: f64 = (1.0 - locals.var_fn382_calc_iq__ff);
        let assign31490_e28588: f64 = (locals.var_fn382_calc_iq__vdsats1 * assign31490_e28587);
        let assign31490_e28591: f64 = (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff);
        let assign31490_e28592: f64 = (assign31490_e28588 + assign31490_e28591);
        locals.var_fn382_calc_iq__vdsat1 = assign31490_e28592;
        locals.var_fn382_calc_iq__vdsat1_dn4 = (((locals.var_fn382_calc_iq__vdsats1_dn4 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn4))) + ((locals.var_fn382_calc_iq__two_n_phit_dn4 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn4)));
        locals.var_fn382_calc_iq__vdsat1_dn5 = (((locals.var_fn382_calc_iq__vdsats1_dn5 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn5))) + ((locals.var_fn382_calc_iq__two_n_phit_dn5 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn5)));
        locals.var_fn382_calc_iq__vdsat1_dn8 = (((locals.var_fn382_calc_iq__vdsats1_dn8 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn8))) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn8));
        locals.var_fn382_calc_iq__vdsat1_dn9 = (((locals.var_fn382_calc_iq__vdsats1_dn9 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn9))) + ((locals.var_fn382_calc_iq__two_n_phit_dn9 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn9)));

        let (assign31500_e28650, assign31500_e28650_d_n4, assign31500_e28650_d_n5, assign31500_e28650_d_n8, assign31500_e28650_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31500_e28603: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
        let assign31500_e28604: f64 = assign31500_e28603;
        let assign31500_e28608: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
        let assign31500_e28609: f64 = (-assign31500_e28608);
        let assign31500_e28612: f64 = (0.001 / p.p53);
        let assign31500_e28616: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
        let assign31500_e28617: f64 = (-assign31500_e28616);
        let assign31500_e28618: f64 = (assign31500_e28612 * assign31500_e28617);
        let assign31500_e28619: f64 = (assign31500_e28618).tanh();
        let assign31500_e28620: f64 = (assign31500_e28609 * assign31500_e28619);
        let assign31500_e28621: f64 = (assign31500_e28604 + assign31500_e28620);
        let assign31500_e28622: f64 = (0.5 * assign31500_e28621);
        (assign31500_e28622, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))),)
    } else {
        let (assign31500_e28649, assign31500_e28649_d_n4, assign31500_e28649_d_n5, assign31500_e28649_d_n8, assign31500_e28649_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31500_e28630: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
                let assign31500_e28631: f64 = assign31500_e28630;
                let assign31500_e28635: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
                let assign31500_e28636: f64 = (-assign31500_e28635);
                let assign31500_e28640: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
                let assign31500_e28641: f64 = (-assign31500_e28640);
                let assign31500_e28642: f64 = (assign31500_e28636 * assign31500_e28641);
                let assign31500_e28644: f64 = (assign31500_e28642 + p.p53);
                let assign31500_e28645: f64 = (assign31500_e28644).sqrt();
                let assign31500_e28646: f64 = (assign31500_e28631 + assign31500_e28645);
                let assign31500_e28647: f64 = (0.5 * assign31500_e28646);
                (assign31500_e28647, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28641) + (assign31500_e28636 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31500_e28645)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28641) + (assign31500_e28636 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31500_e28645)))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28641) + (assign31500_e28636 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31500_e28645)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28641) + (assign31500_e28636 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31500_e28645)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31500_e28649, assign31500_e28649_d_n4, assign31500_e28649_d_n5, assign31500_e28649_d_n8, assign31500_e28649_d_n9,)
    }
};
        let assign31500_e28652: f64 = (assign31500_e28650).powf(locals.var_fn382_calc_iq__beta);
        let assign31500_e28653: f64 = (1.0 + assign31500_e28652);
        let assign31500_e28656: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31500_e28657: f64 = (assign31500_e28653).powf(assign31500_e28656);
        let assign31500_e28658: f64 = (1.0 / assign31500_e28657);
        locals.var_fn382_calc_iq__fsd = assign31500_e28658;
        locals.var_fn382_calc_iq__fsd_dn4 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n4)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n4 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n4)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n4 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_dn5 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n5)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n5 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n5)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n5 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_dn8 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n8)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n8 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n8)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n8 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_dn9 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n9)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n9 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n9)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n9 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));

    }
}
