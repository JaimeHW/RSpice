#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        locals: &mut StampLocals,
    ) {
        let (assign5270_e6581, assign5270_e6581_d_n2, assign5270_e6581_d_n3, assign5270_e6581_d_n4, assign5270_e6581_d_n7, assign5270_e6581_d_n15, assign5270_e6581_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5270_e6581;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5270_e6581_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5270_e6581_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5270_e6581_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5270_e6581_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5270_e6581_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5270_e6581_d_n16;
        locals.var_fn61_calc_iq__qinvv_rv = 0.0;

        let (assign5280_e6585, assign5280_e6585_d_n2, assign5280_e6585_d_n4, assign5280_e6585_d_n7, assign5280_e6585_d_n15, assign5280_e6585_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff0, locals.var_fn61_calc_iq__ff0_dn2, locals.var_fn61_calc_iq__ff0_dn4, locals.var_fn61_calc_iq__ff0_dn7, locals.var_fn61_calc_iq__ff0_dn15, locals.var_fn61_calc_iq__ff0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff0 = assign5280_e6585;
        locals.var_fn61_calc_iq__ff0_dn2 = assign5280_e6585_d_n2;
        locals.var_fn61_calc_iq__ff0_dn4 = assign5280_e6585_d_n4;
        locals.var_fn61_calc_iq__ff0_dn7 = assign5280_e6585_d_n7;
        locals.var_fn61_calc_iq__ff0_dn15 = assign5280_e6585_d_n15;
        locals.var_fn61_calc_iq__ff0_dn16 = assign5280_e6585_d_n16;
        locals.var_fn61_calc_iq__ff0_rv = 0.0;

        let (assign5290_e6589, assign5290_e6589_d_n2, assign5290_e6589_d_n4, assign5290_e6589_d_n7, assign5290_e6589_d_n15, assign5290_e6589_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__eta0, locals.var_fn61_calc_iq__eta0_dn2, locals.var_fn61_calc_iq__eta0_dn4, locals.var_fn61_calc_iq__eta0_dn7, locals.var_fn61_calc_iq__eta0_dn15, locals.var_fn61_calc_iq__eta0_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta0 = assign5290_e6589;
        locals.var_fn61_calc_iq__eta0_dn2 = assign5290_e6589_d_n2;
        locals.var_fn61_calc_iq__eta0_dn4 = assign5290_e6589_d_n4;
        locals.var_fn61_calc_iq__eta0_dn7 = assign5290_e6589_d_n7;
        locals.var_fn61_calc_iq__eta0_dn15 = assign5290_e6589_d_n15;
        locals.var_fn61_calc_iq__eta0_dn16 = assign5290_e6589_d_n16;
        locals.var_fn61_calc_iq__eta0_rv = 0.0;

        let (assign5300_e6593, assign5300_e6593_d_n2, assign5300_e6593_d_n4, assign5300_e6593_d_n7, assign5300_e6593_d_n15, assign5300_e6593_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvv0, locals.var_fn61_calc_iq__qinvv0_dn2, locals.var_fn61_calc_iq__qinvv0_dn4, locals.var_fn61_calc_iq__qinvv0_dn7, locals.var_fn61_calc_iq__qinvv0_dn15, locals.var_fn61_calc_iq__qinvv0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv0 = assign5300_e6593;
        locals.var_fn61_calc_iq__qinvv0_dn2 = assign5300_e6593_d_n2;
        locals.var_fn61_calc_iq__qinvv0_dn4 = assign5300_e6593_d_n4;
        locals.var_fn61_calc_iq__qinvv0_dn7 = assign5300_e6593_d_n7;
        locals.var_fn61_calc_iq__qinvv0_dn15 = assign5300_e6593_d_n15;
        locals.var_fn61_calc_iq__qinvv0_dn16 = assign5300_e6593_d_n16;
        locals.var_fn61_calc_iq__qinvv0_rv = 0.0;

        let (assign5310_e6597, assign5310_e6597_d_n2, assign5310_e6597_d_n3, assign5310_e6597_d_n4, assign5310_e6597_d_n7, assign5310_e6597_d_n15, assign5310_e6597_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats, locals.var_fn61_calc_iq__vdsats_dn2, locals.var_fn61_calc_iq__vdsats_dn3, locals.var_fn61_calc_iq__vdsats_dn4, locals.var_fn61_calc_iq__vdsats_dn7, locals.var_fn61_calc_iq__vdsats_dn15, locals.var_fn61_calc_iq__vdsats_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats = assign5310_e6597;
        locals.var_fn61_calc_iq__vdsats_dn2 = assign5310_e6597_d_n2;
        locals.var_fn61_calc_iq__vdsats_dn3 = assign5310_e6597_d_n3;
        locals.var_fn61_calc_iq__vdsats_dn4 = assign5310_e6597_d_n4;
        locals.var_fn61_calc_iq__vdsats_dn7 = assign5310_e6597_d_n7;
        locals.var_fn61_calc_iq__vdsats_dn15 = assign5310_e6597_d_n15;
        locals.var_fn61_calc_iq__vdsats_dn16 = assign5310_e6597_d_n16;
        locals.var_fn61_calc_iq__vdsats_rv = 0.0;

        let (assign5320_e6601, assign5320_e6601_d_n2, assign5320_e6601_d_n3, assign5320_e6601_d_n4, assign5320_e6601_d_n7, assign5320_e6601_d_n15, assign5320_e6601_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats1, locals.var_fn61_calc_iq__vdsats1_dn2, locals.var_fn61_calc_iq__vdsats1_dn3, locals.var_fn61_calc_iq__vdsats1_dn4, locals.var_fn61_calc_iq__vdsats1_dn7, locals.var_fn61_calc_iq__vdsats1_dn15, locals.var_fn61_calc_iq__vdsats1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats1 = assign5320_e6601;
        locals.var_fn61_calc_iq__vdsats1_dn2 = assign5320_e6601_d_n2;
        locals.var_fn61_calc_iq__vdsats1_dn3 = assign5320_e6601_d_n3;
        locals.var_fn61_calc_iq__vdsats1_dn4 = assign5320_e6601_d_n4;
        locals.var_fn61_calc_iq__vdsats1_dn7 = assign5320_e6601_d_n7;
        locals.var_fn61_calc_iq__vdsats1_dn15 = assign5320_e6601_d_n15;
        locals.var_fn61_calc_iq__vdsats1_dn16 = assign5320_e6601_d_n16;
        locals.var_fn61_calc_iq__vdsats1_rv = 0.0;

        let (assign5330_e6605, assign5330_e6605_d_n2, assign5330_e6605_d_n3, assign5330_e6605_d_n4, assign5330_e6605_d_n7, assign5330_e6605_d_n15, assign5330_e6605_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsat, locals.var_fn61_calc_iq__vdsat_dn2, locals.var_fn61_calc_iq__vdsat_dn3, locals.var_fn61_calc_iq__vdsat_dn4, locals.var_fn61_calc_iq__vdsat_dn7, locals.var_fn61_calc_iq__vdsat_dn15, locals.var_fn61_calc_iq__vdsat_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat = assign5330_e6605;
        locals.var_fn61_calc_iq__vdsat_dn2 = assign5330_e6605_d_n2;
        locals.var_fn61_calc_iq__vdsat_dn3 = assign5330_e6605_d_n3;
        locals.var_fn61_calc_iq__vdsat_dn4 = assign5330_e6605_d_n4;
        locals.var_fn61_calc_iq__vdsat_dn7 = assign5330_e6605_d_n7;
        locals.var_fn61_calc_iq__vdsat_dn15 = assign5330_e6605_d_n15;
        locals.var_fn61_calc_iq__vdsat_dn16 = assign5330_e6605_d_n16;
        locals.var_fn61_calc_iq__vdsat_rv = 0.0;

        let (assign5340_e6609, assign5340_e6609_d_n2, assign5340_e6609_d_n3, assign5340_e6609_d_n4, assign5340_e6609_d_n7, assign5340_e6609_d_n15, assign5340_e6609_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fsd, locals.var_fn61_calc_iq__fsd_dn2, locals.var_fn61_calc_iq__fsd_dn3, locals.var_fn61_calc_iq__fsd_dn4, locals.var_fn61_calc_iq__fsd_dn7, locals.var_fn61_calc_iq__fsd_dn15, locals.var_fn61_calc_iq__fsd_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd = assign5340_e6609;
        locals.var_fn61_calc_iq__fsd_dn2 = assign5340_e6609_d_n2;
        locals.var_fn61_calc_iq__fsd_dn3 = assign5340_e6609_d_n3;
        locals.var_fn61_calc_iq__fsd_dn4 = assign5340_e6609_d_n4;
        locals.var_fn61_calc_iq__fsd_dn7 = assign5340_e6609_d_n7;
        locals.var_fn61_calc_iq__fsd_dn15 = assign5340_e6609_d_n15;
        locals.var_fn61_calc_iq__fsd_dn16 = assign5340_e6609_d_n16;
        locals.var_fn61_calc_iq__fsd_rv = 0.0;

        let (assign5350_e6613, assign5350_e6613_d_n2, assign5350_e6613_d_n3, assign5350_e6613_d_n4, assign5350_e6613_d_n7, assign5350_e6613_d_n15, assign5350_e6613_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdx, locals.var_fn61_calc_iq__vdx_dn2, locals.var_fn61_calc_iq__vdx_dn3, locals.var_fn61_calc_iq__vdx_dn4, locals.var_fn61_calc_iq__vdx_dn7, locals.var_fn61_calc_iq__vdx_dn15, locals.var_fn61_calc_iq__vdx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx = assign5350_e6613;
        locals.var_fn61_calc_iq__vdx_dn2 = assign5350_e6613_d_n2;
        locals.var_fn61_calc_iq__vdx_dn3 = assign5350_e6613_d_n3;
        locals.var_fn61_calc_iq__vdx_dn4 = assign5350_e6613_d_n4;
        locals.var_fn61_calc_iq__vdx_dn7 = assign5350_e6613_d_n7;
        locals.var_fn61_calc_iq__vdx_dn15 = assign5350_e6613_d_n15;
        locals.var_fn61_calc_iq__vdx_dn16 = assign5350_e6613_d_n16;
        locals.var_fn61_calc_iq__vdx_rv = 0.0;

        let (assign5360_e6617, assign5360_e6617_d_n2, assign5360_e6617_d_n3, assign5360_e6617_d_n4, assign5360_e6617_d_n7, assign5360_e6617_d_n15, assign5360_e6617_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fds, locals.var_fn61_calc_iq__fds_dn2, locals.var_fn61_calc_iq__fds_dn3, locals.var_fn61_calc_iq__fds_dn4, locals.var_fn61_calc_iq__fds_dn7, locals.var_fn61_calc_iq__fds_dn15, locals.var_fn61_calc_iq__fds_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds = assign5360_e6617;
        locals.var_fn61_calc_iq__fds_dn2 = assign5360_e6617_d_n2;
        locals.var_fn61_calc_iq__fds_dn3 = assign5360_e6617_d_n3;
        locals.var_fn61_calc_iq__fds_dn4 = assign5360_e6617_d_n4;
        locals.var_fn61_calc_iq__fds_dn7 = assign5360_e6617_d_n7;
        locals.var_fn61_calc_iq__fds_dn15 = assign5360_e6617_d_n15;
        locals.var_fn61_calc_iq__fds_dn16 = assign5360_e6617_d_n16;
        locals.var_fn61_calc_iq__fds_rv = 0.0;

        let (assign5370_e6621, assign5370_e6621_d_n2, assign5370_e6621_d_n3, assign5370_e6621_d_n4, assign5370_e6621_d_n7, assign5370_e6621_d_n15, assign5370_e6621_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsx, locals.var_fn61_calc_iq__vsx_dn2, locals.var_fn61_calc_iq__vsx_dn3, locals.var_fn61_calc_iq__vsx_dn4, locals.var_fn61_calc_iq__vsx_dn7, locals.var_fn61_calc_iq__vsx_dn15, locals.var_fn61_calc_iq__vsx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx = assign5370_e6621;
        locals.var_fn61_calc_iq__vsx_dn2 = assign5370_e6621_d_n2;
        locals.var_fn61_calc_iq__vsx_dn3 = assign5370_e6621_d_n3;
        locals.var_fn61_calc_iq__vsx_dn4 = assign5370_e6621_d_n4;
        locals.var_fn61_calc_iq__vsx_dn7 = assign5370_e6621_d_n7;
        locals.var_fn61_calc_iq__vsx_dn15 = assign5370_e6621_d_n15;
        locals.var_fn61_calc_iq__vsx_dn16 = assign5370_e6621_d_n16;
        locals.var_fn61_calc_iq__vsx_rv = 0.0;

        let (assign5380_e6625, assign5380_e6625_d_n2, assign5380_e6625_d_n3, assign5380_e6625_d_n4, assign5380_e6625_d_n7, assign5380_e6625_d_n15, assign5380_e6625_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign5380_e6625;
        locals.var_fn61_calc_iq__ffd_dn2 = assign5380_e6625_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign5380_e6625_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign5380_e6625_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign5380_e6625_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign5380_e6625_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign5380_e6625_d_n16;
        locals.var_fn61_calc_iq__ffd_rv = 0.0;

        let (assign5390_e6629, assign5390_e6629_d_n2, assign5390_e6629_d_n3, assign5390_e6629_d_n4, assign5390_e6629_d_n7, assign5390_e6629_d_n15, assign5390_e6629_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etad, locals.var_fn61_calc_iq__etad_dn2, locals.var_fn61_calc_iq__etad_dn3, locals.var_fn61_calc_iq__etad_dn4, locals.var_fn61_calc_iq__etad_dn7, locals.var_fn61_calc_iq__etad_dn15, locals.var_fn61_calc_iq__etad_dn16,)
    }
};
        locals.var_fn61_calc_iq__etad = assign5390_e6629;
        locals.var_fn61_calc_iq__etad_dn2 = assign5390_e6629_d_n2;
        locals.var_fn61_calc_iq__etad_dn3 = assign5390_e6629_d_n3;
        locals.var_fn61_calc_iq__etad_dn4 = assign5390_e6629_d_n4;
        locals.var_fn61_calc_iq__etad_dn7 = assign5390_e6629_d_n7;
        locals.var_fn61_calc_iq__etad_dn15 = assign5390_e6629_d_n15;
        locals.var_fn61_calc_iq__etad_dn16 = assign5390_e6629_d_n16;
        locals.var_fn61_calc_iq__etad_rv = 0.0;

        let (assign5400_e6633, assign5400_e6633_d_n2, assign5400_e6633_d_n3, assign5400_e6633_d_n4, assign5400_e6633_d_n7, assign5400_e6633_d_n15, assign5400_e6633_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvd, locals.var_fn61_calc_iq__qinvd_dn2, locals.var_fn61_calc_iq__qinvd_dn3, locals.var_fn61_calc_iq__qinvd_dn4, locals.var_fn61_calc_iq__qinvd_dn7, locals.var_fn61_calc_iq__qinvd_dn15, locals.var_fn61_calc_iq__qinvd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd = assign5400_e6633;
        locals.var_fn61_calc_iq__qinvd_dn2 = assign5400_e6633_d_n2;
        locals.var_fn61_calc_iq__qinvd_dn3 = assign5400_e6633_d_n3;
        locals.var_fn61_calc_iq__qinvd_dn4 = assign5400_e6633_d_n4;
        locals.var_fn61_calc_iq__qinvd_dn7 = assign5400_e6633_d_n7;
        locals.var_fn61_calc_iq__qinvd_dn15 = assign5400_e6633_d_n15;
        locals.var_fn61_calc_iq__qinvd_dn16 = assign5400_e6633_d_n16;
        locals.var_fn61_calc_iq__qinvd_rv = 0.0;

        let (assign5410_e6637, assign5410_e6637_d_n2, assign5410_e6637_d_n3, assign5410_e6637_d_n4, assign5410_e6637_d_n7, assign5410_e6637_d_n15, assign5410_e6637_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsc, locals.var_fn61_calc_iq__vdsc_dn2, locals.var_fn61_calc_iq__vdsc_dn3, locals.var_fn61_calc_iq__vdsc_dn4, locals.var_fn61_calc_iq__vdsc_dn7, locals.var_fn61_calc_iq__vdsc_dn15, locals.var_fn61_calc_iq__vdsc_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsc = assign5410_e6637;
        locals.var_fn61_calc_iq__vdsc_dn2 = assign5410_e6637_d_n2;
        locals.var_fn61_calc_iq__vdsc_dn3 = assign5410_e6637_d_n3;
        locals.var_fn61_calc_iq__vdsc_dn4 = assign5410_e6637_d_n4;
        locals.var_fn61_calc_iq__vdsc_dn7 = assign5410_e6637_d_n7;
        locals.var_fn61_calc_iq__vdsc_dn15 = assign5410_e6637_d_n15;
        locals.var_fn61_calc_iq__vdsc_dn16 = assign5410_e6637_d_n16;
        locals.var_fn61_calc_iq__vdsc_rv = 0.0;

        let (assign5440_e6649, assign5440_e6649_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats0, locals.var_fn61_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn61_calc_iq__vdsats0 = assign5440_e6649;
        locals.var_fn61_calc_iq__vdsats0_dn4 = assign5440_e6649_d_n4;
        locals.var_fn61_calc_iq__vdsats0_rv = 0.0;

        let (assign5450_e6653, assign5450_e6653_d_n2, assign5450_e6653_d_n4, assign5450_e6653_d_n7, assign5450_e6653_d_n15, assign5450_e6653_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsats10, locals.var_fn61_calc_iq__vdsats10_dn2, locals.var_fn61_calc_iq__vdsats10_dn4, locals.var_fn61_calc_iq__vdsats10_dn7, locals.var_fn61_calc_iq__vdsats10_dn15, locals.var_fn61_calc_iq__vdsats10_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats10 = assign5450_e6653;
        locals.var_fn61_calc_iq__vdsats10_dn2 = assign5450_e6653_d_n2;
        locals.var_fn61_calc_iq__vdsats10_dn4 = assign5450_e6653_d_n4;
        locals.var_fn61_calc_iq__vdsats10_dn7 = assign5450_e6653_d_n7;
        locals.var_fn61_calc_iq__vdsats10_dn15 = assign5450_e6653_d_n15;
        locals.var_fn61_calc_iq__vdsats10_dn16 = assign5450_e6653_d_n16;
        locals.var_fn61_calc_iq__vdsats10_rv = 0.0;

        let (assign5460_e6657, assign5460_e6657_d_n2, assign5460_e6657_d_n4, assign5460_e6657_d_n7, assign5460_e6657_d_n15, assign5460_e6657_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdsat10, locals.var_fn61_calc_iq__vdsat10_dn2, locals.var_fn61_calc_iq__vdsat10_dn4, locals.var_fn61_calc_iq__vdsat10_dn7, locals.var_fn61_calc_iq__vdsat10_dn15, locals.var_fn61_calc_iq__vdsat10_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat10 = assign5460_e6657;
        locals.var_fn61_calc_iq__vdsat10_dn2 = assign5460_e6657_d_n2;
        locals.var_fn61_calc_iq__vdsat10_dn4 = assign5460_e6657_d_n4;
        locals.var_fn61_calc_iq__vdsat10_dn7 = assign5460_e6657_d_n7;
        locals.var_fn61_calc_iq__vdsat10_dn15 = assign5460_e6657_d_n15;
        locals.var_fn61_calc_iq__vdsat10_dn16 = assign5460_e6657_d_n16;
        locals.var_fn61_calc_iq__vdsat10_rv = 0.0;

        let (assign5470_e6661, assign5470_e6661_d_n2, assign5470_e6661_d_n4, assign5470_e6661_d_n7, assign5470_e6661_d_n15, assign5470_e6661_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fsd0, locals.var_fn61_calc_iq__fsd0_dn2, locals.var_fn61_calc_iq__fsd0_dn4, locals.var_fn61_calc_iq__fsd0_dn7, locals.var_fn61_calc_iq__fsd0_dn15, locals.var_fn61_calc_iq__fsd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd0 = assign5470_e6661;
        locals.var_fn61_calc_iq__fsd0_dn2 = assign5470_e6661_d_n2;
        locals.var_fn61_calc_iq__fsd0_dn4 = assign5470_e6661_d_n4;
        locals.var_fn61_calc_iq__fsd0_dn7 = assign5470_e6661_d_n7;
        locals.var_fn61_calc_iq__fsd0_dn15 = assign5470_e6661_d_n15;
        locals.var_fn61_calc_iq__fsd0_dn16 = assign5470_e6661_d_n16;
        locals.var_fn61_calc_iq__fsd0_rv = 0.0;

        let (assign5480_e6665, assign5480_e6665_d_n2, assign5480_e6665_d_n4, assign5480_e6665_d_n7, assign5480_e6665_d_n15, assign5480_e6665_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vdx0, locals.var_fn61_calc_iq__vdx0_dn2, locals.var_fn61_calc_iq__vdx0_dn4, locals.var_fn61_calc_iq__vdx0_dn7, locals.var_fn61_calc_iq__vdx0_dn15, locals.var_fn61_calc_iq__vdx0_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx0 = assign5480_e6665;
        locals.var_fn61_calc_iq__vdx0_dn2 = assign5480_e6665_d_n2;
        locals.var_fn61_calc_iq__vdx0_dn4 = assign5480_e6665_d_n4;
        locals.var_fn61_calc_iq__vdx0_dn7 = assign5480_e6665_d_n7;
        locals.var_fn61_calc_iq__vdx0_dn15 = assign5480_e6665_d_n15;
        locals.var_fn61_calc_iq__vdx0_dn16 = assign5480_e6665_d_n16;
        locals.var_fn61_calc_iq__vdx0_rv = 0.0;

        let (assign5490_e6669, assign5490_e6669_d_n2, assign5490_e6669_d_n4, assign5490_e6669_d_n7, assign5490_e6669_d_n15, assign5490_e6669_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__fds0, locals.var_fn61_calc_iq__fds0_dn2, locals.var_fn61_calc_iq__fds0_dn4, locals.var_fn61_calc_iq__fds0_dn7, locals.var_fn61_calc_iq__fds0_dn15, locals.var_fn61_calc_iq__fds0_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds0 = assign5490_e6669;
        locals.var_fn61_calc_iq__fds0_dn2 = assign5490_e6669_d_n2;
        locals.var_fn61_calc_iq__fds0_dn4 = assign5490_e6669_d_n4;
        locals.var_fn61_calc_iq__fds0_dn7 = assign5490_e6669_d_n7;
        locals.var_fn61_calc_iq__fds0_dn15 = assign5490_e6669_d_n15;
        locals.var_fn61_calc_iq__fds0_dn16 = assign5490_e6669_d_n16;
        locals.var_fn61_calc_iq__fds0_rv = 0.0;

        let (assign5500_e6673, assign5500_e6673_d_n2, assign5500_e6673_d_n4, assign5500_e6673_d_n7, assign5500_e6673_d_n15, assign5500_e6673_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsx0, locals.var_fn61_calc_iq__vsx0_dn2, locals.var_fn61_calc_iq__vsx0_dn4, locals.var_fn61_calc_iq__vsx0_dn7, locals.var_fn61_calc_iq__vsx0_dn15, locals.var_fn61_calc_iq__vsx0_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx0 = assign5500_e6673;
        locals.var_fn61_calc_iq__vsx0_dn2 = assign5500_e6673_d_n2;
        locals.var_fn61_calc_iq__vsx0_dn4 = assign5500_e6673_d_n4;
        locals.var_fn61_calc_iq__vsx0_dn7 = assign5500_e6673_d_n7;
        locals.var_fn61_calc_iq__vsx0_dn15 = assign5500_e6673_d_n15;
        locals.var_fn61_calc_iq__vsx0_dn16 = assign5500_e6673_d_n16;
        locals.var_fn61_calc_iq__vsx0_rv = 0.0;

        let (assign5510_e6677, assign5510_e6677_d_n2, assign5510_e6677_d_n4, assign5510_e6677_d_n7, assign5510_e6677_d_n15, assign5510_e6677_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd0, locals.var_fn61_calc_iq__ffd0_dn2, locals.var_fn61_calc_iq__ffd0_dn4, locals.var_fn61_calc_iq__ffd0_dn7, locals.var_fn61_calc_iq__ffd0_dn15, locals.var_fn61_calc_iq__ffd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd0 = assign5510_e6677;
        locals.var_fn61_calc_iq__ffd0_dn2 = assign5510_e6677_d_n2;
        locals.var_fn61_calc_iq__ffd0_dn4 = assign5510_e6677_d_n4;
        locals.var_fn61_calc_iq__ffd0_dn7 = assign5510_e6677_d_n7;
        locals.var_fn61_calc_iq__ffd0_dn15 = assign5510_e6677_d_n15;
        locals.var_fn61_calc_iq__ffd0_dn16 = assign5510_e6677_d_n16;
        locals.var_fn61_calc_iq__ffd0_rv = 0.0;

        let (assign5520_e6681, assign5520_e6681_d_n2, assign5520_e6681_d_n4, assign5520_e6681_d_n7, assign5520_e6681_d_n15, assign5520_e6681_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etad0, locals.var_fn61_calc_iq__etad0_dn2, locals.var_fn61_calc_iq__etad0_dn4, locals.var_fn61_calc_iq__etad0_dn7, locals.var_fn61_calc_iq__etad0_dn15, locals.var_fn61_calc_iq__etad0_dn16,)
    }
};
        locals.var_fn61_calc_iq__etad0 = assign5520_e6681;
        locals.var_fn61_calc_iq__etad0_dn2 = assign5520_e6681_d_n2;
        locals.var_fn61_calc_iq__etad0_dn4 = assign5520_e6681_d_n4;
        locals.var_fn61_calc_iq__etad0_dn7 = assign5520_e6681_d_n7;
        locals.var_fn61_calc_iq__etad0_dn15 = assign5520_e6681_d_n15;
        locals.var_fn61_calc_iq__etad0_dn16 = assign5520_e6681_d_n16;
        locals.var_fn61_calc_iq__etad0_rv = 0.0;

        let (assign5530_e6685, assign5530_e6685_d_n2, assign5530_e6685_d_n4, assign5530_e6685_d_n7, assign5530_e6685_d_n15, assign5530_e6685_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvd0, locals.var_fn61_calc_iq__qinvd0_dn2, locals.var_fn61_calc_iq__qinvd0_dn4, locals.var_fn61_calc_iq__qinvd0_dn7, locals.var_fn61_calc_iq__qinvd0_dn15, locals.var_fn61_calc_iq__qinvd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd0 = assign5530_e6685;
        locals.var_fn61_calc_iq__qinvd0_dn2 = assign5530_e6685_d_n2;
        locals.var_fn61_calc_iq__qinvd0_dn4 = assign5530_e6685_d_n4;
        locals.var_fn61_calc_iq__qinvd0_dn7 = assign5530_e6685_d_n7;
        locals.var_fn61_calc_iq__qinvd0_dn15 = assign5530_e6685_d_n15;
        locals.var_fn61_calc_iq__qinvd0_dn16 = assign5530_e6685_d_n16;
        locals.var_fn61_calc_iq__qinvd0_rv = 0.0;

        let (assign5540_e6689, assign5540_e6689_d_n2, assign5540_e6689_d_n4, assign5540_e6689_d_n7, assign5540_e6689_d_n15, assign5540_e6689_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qs2, locals.var_fn61_calc_iq__qs2_dn2, locals.var_fn61_calc_iq__qs2_dn4, locals.var_fn61_calc_iq__qs2_dn7, locals.var_fn61_calc_iq__qs2_dn15, locals.var_fn61_calc_iq__qs2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs2 = assign5540_e6689;
        locals.var_fn61_calc_iq__qs2_dn2 = assign5540_e6689_d_n2;
        locals.var_fn61_calc_iq__qs2_dn4 = assign5540_e6689_d_n4;
        locals.var_fn61_calc_iq__qs2_dn7 = assign5540_e6689_d_n7;
        locals.var_fn61_calc_iq__qs2_dn15 = assign5540_e6689_d_n15;
        locals.var_fn61_calc_iq__qs2_dn16 = assign5540_e6689_d_n16;
        locals.var_fn61_calc_iq__qs2_rv = 0.0;

        let (assign5550_e6693, assign5550_e6693_d_n2, assign5550_e6693_d_n4, assign5550_e6693_d_n7, assign5550_e6693_d_n15, assign5550_e6693_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qs3, locals.var_fn61_calc_iq__qs3_dn2, locals.var_fn61_calc_iq__qs3_dn4, locals.var_fn61_calc_iq__qs3_dn7, locals.var_fn61_calc_iq__qs3_dn15, locals.var_fn61_calc_iq__qs3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs3 = assign5550_e6693;
        locals.var_fn61_calc_iq__qs3_dn2 = assign5550_e6693_d_n2;
        locals.var_fn61_calc_iq__qs3_dn4 = assign5550_e6693_d_n4;
        locals.var_fn61_calc_iq__qs3_dn7 = assign5550_e6693_d_n7;
        locals.var_fn61_calc_iq__qs3_dn15 = assign5550_e6693_d_n15;
        locals.var_fn61_calc_iq__qs3_dn16 = assign5550_e6693_d_n16;
        locals.var_fn61_calc_iq__qs3_rv = 0.0;

        let (assign5560_e6697, assign5560_e6697_d_n2, assign5560_e6697_d_n4, assign5560_e6697_d_n7, assign5560_e6697_d_n15, assign5560_e6697_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd2, locals.var_fn61_calc_iq__qd2_dn2, locals.var_fn61_calc_iq__qd2_dn4, locals.var_fn61_calc_iq__qd2_dn7, locals.var_fn61_calc_iq__qd2_dn15, locals.var_fn61_calc_iq__qd2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd2 = assign5560_e6697;
        locals.var_fn61_calc_iq__qd2_dn2 = assign5560_e6697_d_n2;
        locals.var_fn61_calc_iq__qd2_dn4 = assign5560_e6697_d_n4;
        locals.var_fn61_calc_iq__qd2_dn7 = assign5560_e6697_d_n7;
        locals.var_fn61_calc_iq__qd2_dn15 = assign5560_e6697_d_n15;
        locals.var_fn61_calc_iq__qd2_dn16 = assign5560_e6697_d_n16;
        locals.var_fn61_calc_iq__qd2_rv = 0.0;

        let (assign5570_e6701, assign5570_e6701_d_n2, assign5570_e6701_d_n4, assign5570_e6701_d_n7, assign5570_e6701_d_n15, assign5570_e6701_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd3, locals.var_fn61_calc_iq__qd3_dn2, locals.var_fn61_calc_iq__qd3_dn4, locals.var_fn61_calc_iq__qd3_dn7, locals.var_fn61_calc_iq__qd3_dn15, locals.var_fn61_calc_iq__qd3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd3 = assign5570_e6701;
        locals.var_fn61_calc_iq__qd3_dn2 = assign5570_e6701_d_n2;
        locals.var_fn61_calc_iq__qd3_dn4 = assign5570_e6701_d_n4;
        locals.var_fn61_calc_iq__qd3_dn7 = assign5570_e6701_d_n7;
        locals.var_fn61_calc_iq__qd3_dn15 = assign5570_e6701_d_n15;
        locals.var_fn61_calc_iq__qd3_dn16 = assign5570_e6701_d_n16;
        locals.var_fn61_calc_iq__qd3_rv = 0.0;

        let (assign5580_e6705, assign5580_e6705_d_n2, assign5580_e6705_d_n4, assign5580_e6705_d_n7, assign5580_e6705_d_n15, assign5580_e6705_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qsqd, locals.var_fn61_calc_iq__qsqd_dn2, locals.var_fn61_calc_iq__qsqd_dn4, locals.var_fn61_calc_iq__qsqd_dn7, locals.var_fn61_calc_iq__qsqd_dn15, locals.var_fn61_calc_iq__qsqd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsqd = assign5580_e6705;
        locals.var_fn61_calc_iq__qsqd_dn2 = assign5580_e6705_d_n2;
        locals.var_fn61_calc_iq__qsqd_dn4 = assign5580_e6705_d_n4;
        locals.var_fn61_calc_iq__qsqd_dn7 = assign5580_e6705_d_n7;
        locals.var_fn61_calc_iq__qsqd_dn15 = assign5580_e6705_d_n15;
        locals.var_fn61_calc_iq__qsqd_dn16 = assign5580_e6705_d_n16;
        locals.var_fn61_calc_iq__qsqd_rv = 0.0;

        let (assign5590_e6709, assign5590_e6709_d_n2, assign5590_e6709_d_n4, assign5590_e6709_d_n7, assign5590_e6709_d_n15, assign5590_e6709_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qinvdd, locals.var_fn61_calc_iq__qinvdd_dn2, locals.var_fn61_calc_iq__qinvdd_dn4, locals.var_fn61_calc_iq__qinvdd_dn7, locals.var_fn61_calc_iq__qinvdd_dn15, locals.var_fn61_calc_iq__qinvdd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvdd = assign5590_e6709;
        locals.var_fn61_calc_iq__qinvdd_dn2 = assign5590_e6709_d_n2;
        locals.var_fn61_calc_iq__qinvdd_dn4 = assign5590_e6709_d_n4;
        locals.var_fn61_calc_iq__qinvdd_dn7 = assign5590_e6709_d_n7;
        locals.var_fn61_calc_iq__qinvdd_dn15 = assign5590_e6709_d_n15;
        locals.var_fn61_calc_iq__qinvdd_dn16 = assign5590_e6709_d_n16;
        locals.var_fn61_calc_iq__qinvdd_rv = 0.0;

        let (assign5600_e6713, assign5600_e6713_d_n2, assign5600_e6713_d_n4, assign5600_e6713_d_n7, assign5600_e6713_d_n15, assign5600_e6713_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd1, locals.var_fn61_calc_iq__qd1_dn2, locals.var_fn61_calc_iq__qd1_dn4, locals.var_fn61_calc_iq__qd1_dn7, locals.var_fn61_calc_iq__qd1_dn15, locals.var_fn61_calc_iq__qd1_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd1 = assign5600_e6713;
        locals.var_fn61_calc_iq__qd1_dn2 = assign5600_e6713_d_n2;
        locals.var_fn61_calc_iq__qd1_dn4 = assign5600_e6713_d_n4;
        locals.var_fn61_calc_iq__qd1_dn7 = assign5600_e6713_d_n7;
        locals.var_fn61_calc_iq__qd1_dn15 = assign5600_e6713_d_n15;
        locals.var_fn61_calc_iq__qd1_dn16 = assign5600_e6713_d_n16;
        locals.var_fn61_calc_iq__qd1_rv = 0.0;

        let (assign5610_e6717, assign5610_e6717_d_n2, assign5610_e6717_d_n4, assign5610_e6717_d_n7, assign5610_e6717_d_n15, assign5610_e6717_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qs, locals.var_fn61_calc_iq__qs_dn2, locals.var_fn61_calc_iq__qs_dn4, locals.var_fn61_calc_iq__qs_dn7, locals.var_fn61_calc_iq__qs_dn15, locals.var_fn61_calc_iq__qs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs = assign5610_e6717;
        locals.var_fn61_calc_iq__qs_dn2 = assign5610_e6717_d_n2;
        locals.var_fn61_calc_iq__qs_dn4 = assign5610_e6717_d_n4;
        locals.var_fn61_calc_iq__qs_dn7 = assign5610_e6717_d_n7;
        locals.var_fn61_calc_iq__qs_dn15 = assign5610_e6717_d_n15;
        locals.var_fn61_calc_iq__qs_dn16 = assign5610_e6717_d_n16;
        locals.var_fn61_calc_iq__qs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5620_e6721, assign5620_e6721_d_n2, assign5620_e6721_d_n4, assign5620_e6721_d_n7, assign5620_e6721_d_n15, assign5620_e6721_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qd, locals.var_fn61_calc_iq__qd_dn2, locals.var_fn61_calc_iq__qd_dn4, locals.var_fn61_calc_iq__qd_dn7, locals.var_fn61_calc_iq__qd_dn15, locals.var_fn61_calc_iq__qd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd = assign5620_e6721;
        locals.var_fn61_calc_iq__qd_dn2 = assign5620_e6721_d_n2;
        locals.var_fn61_calc_iq__qd_dn4 = assign5620_e6721_d_n4;
        locals.var_fn61_calc_iq__qd_dn7 = assign5620_e6721_d_n7;
        locals.var_fn61_calc_iq__qd_dn15 = assign5620_e6721_d_n15;
        locals.var_fn61_calc_iq__qd_dn16 = assign5620_e6721_d_n16;
        locals.var_fn61_calc_iq__qd_rv = 0.0;

        let (assign5630_e6725, assign5630_e6725_d_n2, assign5630_e6725_d_n4, assign5630_e6725_d_n7, assign5630_e6725_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etac, locals.var_fn61_calc_iq__etac_dn2, locals.var_fn61_calc_iq__etac_dn4, locals.var_fn61_calc_iq__etac_dn7, locals.var_fn61_calc_iq__etac_dn15,)
    }
};
        locals.var_fn61_calc_iq__etac = assign5630_e6725;
        locals.var_fn61_calc_iq__etac_dn2 = assign5630_e6725_d_n2;
        locals.var_fn61_calc_iq__etac_dn4 = assign5630_e6725_d_n4;
        locals.var_fn61_calc_iq__etac_dn7 = assign5630_e6725_d_n7;
        locals.var_fn61_calc_iq__etac_dn15 = assign5630_e6725_d_n15;
        locals.var_fn61_calc_iq__etac_rv = 0.0;

        let (assign5640_e6729, assign5640_e6729_d_n3, assign5640_e6729_d_n4, assign5640_e6729_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etab, locals.var_fn61_calc_iq__etab_dn3, locals.var_fn61_calc_iq__etab_dn4, locals.var_fn61_calc_iq__etab_dn15,)
    }
};
        locals.var_fn61_calc_iq__etab = assign5640_e6729;
        locals.var_fn61_calc_iq__etab_dn3 = assign5640_e6729_d_n3;
        locals.var_fn61_calc_iq__etab_dn4 = assign5640_e6729_d_n4;
        locals.var_fn61_calc_iq__etab_dn15 = assign5640_e6729_d_n15;
        locals.var_fn61_calc_iq__etab_rv = 0.0;

        let (assign5650_e6733, assign5650_e6733_d_n2, assign5650_e6733_d_n4, assign5650_e6733_d_n7, assign5650_e6733_d_n15,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__etags, locals.var_fn61_calc_iq__etags_dn2, locals.var_fn61_calc_iq__etags_dn4, locals.var_fn61_calc_iq__etags_dn7, locals.var_fn61_calc_iq__etags_dn15,)
    }
};
        locals.var_fn61_calc_iq__etags = assign5650_e6733;
        locals.var_fn61_calc_iq__etags_dn2 = assign5650_e6733_d_n2;
        locals.var_fn61_calc_iq__etags_dn4 = assign5650_e6733_d_n4;
        locals.var_fn61_calc_iq__etags_dn7 = assign5650_e6733_d_n7;
        locals.var_fn61_calc_iq__etags_dn15 = assign5650_e6733_d_n15;
        locals.var_fn61_calc_iq__etags_rv = 0.0;

        let (assign5660_e6737, assign5660_e6737_d_n2, assign5660_e6737_d_n3, assign5660_e6737_d_n4, assign5660_e6737_d_n7, assign5660_e6737_d_n15, assign5660_e6737_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign5660_e6737;
        locals.var_fn61_calc_iq__exparg_dn2 = assign5660_e6737_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign5660_e6737_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign5660_e6737_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign5660_e6737_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign5660_e6737_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign5660_e6737_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign5670_e6741, assign5670_e6741_d_n2, assign5670_e6741_d_n3, assign5670_e6741_d_n4, assign5670_e6741_d_n7, assign5670_e6741_d_n15, assign5670_e6741_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__myarg, locals.var_fn61_calc_iq__myarg_dn2, locals.var_fn61_calc_iq__myarg_dn3, locals.var_fn61_calc_iq__myarg_dn4, locals.var_fn61_calc_iq__myarg_dn7, locals.var_fn61_calc_iq__myarg_dn15, locals.var_fn61_calc_iq__myarg_dn16,)
    }
};
        locals.var_fn61_calc_iq__myarg = assign5670_e6741;
        locals.var_fn61_calc_iq__myarg_dn2 = assign5670_e6741_d_n2;
        locals.var_fn61_calc_iq__myarg_dn3 = assign5670_e6741_d_n3;
        locals.var_fn61_calc_iq__myarg_dn4 = assign5670_e6741_d_n4;
        locals.var_fn61_calc_iq__myarg_dn7 = assign5670_e6741_d_n7;
        locals.var_fn61_calc_iq__myarg_dn15 = assign5670_e6741_d_n15;
        locals.var_fn61_calc_iq__myarg_dn16 = assign5670_e6741_d_n16;
        locals.var_fn61_calc_iq__myarg_rv = 0.0;

        let (assign5680_e6745, assign5680_e6745_d_n15, assign5680_e6745_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__absvdsin, locals.var_fn61_calc_iq__absvdsin_dn15, locals.var_fn61_calc_iq__absvdsin_dn16,)
    }
};
        locals.var_fn61_calc_iq__absvdsin = assign5680_e6745;
        locals.var_fn61_calc_iq__absvdsin_dn15 = assign5680_e6745_d_n15;
        locals.var_fn61_calc_iq__absvdsin_dn16 = assign5680_e6745_d_n16;
        locals.var_fn61_calc_iq__absvdsin_rv = 0.0;

        let (assign5690_e6749, assign5690_e6749_d_n2, assign5690_e6749_d_n7, assign5690_e6749_d_n15, assign5690_e6749_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vgdin, locals.var_fn61_calc_iq__vgdin_dn2, locals.var_fn61_calc_iq__vgdin_dn7, locals.var_fn61_calc_iq__vgdin_dn15, locals.var_fn61_calc_iq__vgdin_dn16,)
    }
};
        locals.var_fn61_calc_iq__vgdin = assign5690_e6749;
        locals.var_fn61_calc_iq__vgdin_dn2 = assign5690_e6749_d_n2;
        locals.var_fn61_calc_iq__vgdin_dn7 = assign5690_e6749_d_n7;
        locals.var_fn61_calc_iq__vgdin_dn15 = assign5690_e6749_d_n15;
        locals.var_fn61_calc_iq__vgdin_dn16 = assign5690_e6749_d_n16;
        locals.var_fn61_calc_iq__vgdin_rv = 0.0;

        let (assign5700_e6753, assign5700_e6753_d_n2, assign5700_e6753_d_n4, assign5700_e6753_d_n7, assign5700_e6753_d_n15, assign5700_e6753_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg0, locals.var_fn61_calc_iq__exparg0_dn2, locals.var_fn61_calc_iq__exparg0_dn4, locals.var_fn61_calc_iq__exparg0_dn7, locals.var_fn61_calc_iq__exparg0_dn15, locals.var_fn61_calc_iq__exparg0_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg0 = assign5700_e6753;
        locals.var_fn61_calc_iq__exparg0_dn2 = assign5700_e6753_d_n2;
        locals.var_fn61_calc_iq__exparg0_dn4 = assign5700_e6753_d_n4;
        locals.var_fn61_calc_iq__exparg0_dn7 = assign5700_e6753_d_n7;
        locals.var_fn61_calc_iq__exparg0_dn15 = assign5700_e6753_d_n15;
        locals.var_fn61_calc_iq__exparg0_dn16 = assign5700_e6753_d_n16;
        locals.var_fn61_calc_iq__exparg0_rv = 0.0;

        let (assign5710_e6757, assign5710_e6757_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__myarg0, locals.var_fn61_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn61_calc_iq__myarg0 = assign5710_e6757;
        locals.var_fn61_calc_iq__myarg0_dn4 = assign5710_e6757_d_n4;
        locals.var_fn61_calc_iq__myarg0_rv = 0.0;

        let (assign5720_e6784, assign5720_e6784_d_n15, assign5720_e6784_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign5720_e6782, assign5720_e6782_d_n15, assign5720_e6782_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign5720_e6766: f64 = (0.001 / p.p53);
                let assign5720_e6768: f64 = (assign5720_e6766 * locals.var_fn61_calc_iq__vdsin);
                let assign5720_e6769: f64 = (assign5720_e6768).tanh();
                let assign5720_e6770: f64 = (locals.var_fn61_calc_iq__vdsin * assign5720_e6769);
                (assign5720_e6770, ((locals.var_fn61_calc_iq__vdsin_dn15 * assign5720_e6769) + (locals.var_fn61_calc_iq__vdsin * ((assign5720_e6766 * locals.var_fn61_calc_iq__vdsin_dn15) / ((assign5720_e6768).cosh() * (assign5720_e6768).cosh())))), ((locals.var_fn61_calc_iq__vdsin_dn16 * assign5720_e6769) + (locals.var_fn61_calc_iq__vdsin * ((assign5720_e6766 * locals.var_fn61_calc_iq__vdsin_dn16) / ((assign5720_e6768).cosh() * (assign5720_e6768).cosh())))),)
            } else {
                let (assign5720_e6781, assign5720_e6781_d_n15, assign5720_e6781_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign5720_e6776: f64 = (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsin);
                        let assign5720_e6778: f64 = (assign5720_e6776 + p.p53);
                        let assign5720_e6779: f64 = (assign5720_e6778).sqrt();
                        (assign5720_e6779, (((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsin) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsin_dn15)) / (2.0 * assign5720_e6779)), (((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsin) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsin_dn16)) / (2.0 * assign5720_e6779)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign5720_e6781, assign5720_e6781_d_n15, assign5720_e6781_d_n16,)
            }
        };
        (assign5720_e6782, assign5720_e6782_d_n15, assign5720_e6782_d_n16,)
    } else {
        (locals.var_fn61_calc_iq__absvdsin, locals.var_fn61_calc_iq__absvdsin_dn15, locals.var_fn61_calc_iq__absvdsin_dn16,)
    }
};
        locals.var_fn61_calc_iq__absvdsin = assign5720_e6784;
        locals.var_fn61_calc_iq__absvdsin_dn15 = assign5720_e6784_d_n15;
        locals.var_fn61_calc_iq__absvdsin_dn16 = assign5720_e6784_d_n16;
        locals.var_fn61_calc_iq__absvdsin_rv = 0.0;

        let (assign5730_e6790, assign5730_e6790_d_n2, assign5730_e6790_d_n7, assign5730_e6790_d_n15, assign5730_e6790_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5730_e6788: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vdsin);
        (assign5730_e6788, locals.var_fn61_calc_iq__vgsin_dn2, locals.var_fn61_calc_iq__vgsin_dn7, (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vdsin_dn15), (-locals.var_fn61_calc_iq__vdsin_dn16),)
    } else {
        (locals.var_fn61_calc_iq__vgdin, locals.var_fn61_calc_iq__vgdin_dn2, locals.var_fn61_calc_iq__vgdin_dn7, locals.var_fn61_calc_iq__vgdin_dn15, locals.var_fn61_calc_iq__vgdin_dn16,)
    }
};
        locals.var_fn61_calc_iq__vgdin = assign5730_e6790;
        locals.var_fn61_calc_iq__vgdin_dn2 = assign5730_e6790_d_n2;
        locals.var_fn61_calc_iq__vgdin_dn7 = assign5730_e6790_d_n7;
        locals.var_fn61_calc_iq__vgdin_dn15 = assign5730_e6790_d_n15;
        locals.var_fn61_calc_iq__vgdin_dn16 = assign5730_e6790_d_n16;
        locals.var_fn61_calc_iq__vgdin_rv = 0.0;

        let (assign5740_e6796, assign5740_e6796_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5740_e6794: f64 = (locals.var_fn61_calc_iq__alpha * locals.var_fn61_calc_iq__phitin);
        (assign5740_e6794, (locals.var_fn61_calc_iq__alpha * locals.var_fn61_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn61_calc_iq__alpha_phit, locals.var_fn61_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn61_calc_iq__alpha_phit = assign5740_e6796;
        locals.var_fn61_calc_iq__alpha_phit_dn4 = assign5740_e6796_d_n4;
        locals.var_fn61_calc_iq__alpha_phit_rv = 0.0;

        let (assign5750_e6808, assign5750_e6808_d_n4, assign5750_e6808_d_n15, assign5750_e6808_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5750_e6801: f64 = (2.302585092994046 * locals.var_fn61_calc_iq__phitin);
        let assign5750_e6802: f64 = (locals.var_fn61_calc_iq__ss / assign5750_e6801);
        let assign5750_e6805: f64 = (locals.var_fn61_calc_iq__nd * locals.var_fn61_calc_iq__absvdsin);
        let assign5750_e6806: f64 = (assign5750_e6802 + assign5750_e6805);
        (assign5750_e6806, (-((locals.var_fn61_calc_iq__ss * (2.302585092994046 * locals.var_fn61_calc_iq__phitin_dn4)) / (assign5750_e6801 * assign5750_e6801))), (locals.var_fn61_calc_iq__nd * locals.var_fn61_calc_iq__absvdsin_dn15), (locals.var_fn61_calc_iq__nd * locals.var_fn61_calc_iq__absvdsin_dn16),)
    } else {
        (locals.var_fn61_calc_iq__n, locals.var_fn61_calc_iq__n_dn4, locals.var_fn61_calc_iq__n_dn15, locals.var_fn61_calc_iq__n_dn16,)
    }
};
        locals.var_fn61_calc_iq__n = assign5750_e6808;
        locals.var_fn61_calc_iq__n_dn4 = assign5750_e6808_d_n4;
        locals.var_fn61_calc_iq__n_dn15 = assign5750_e6808_d_n15;
        locals.var_fn61_calc_iq__n_dn16 = assign5750_e6808_d_n16;
        locals.var_fn61_calc_iq__n_rv = 0.0;

        let (assign5760_e6818, assign5760_e6818_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5760_e6814: f64 = (locals.var_fn61_calc_iq__tambin - locals.var_fn61_calc_iq__tnomin);
        let assign5760_e6815: f64 = (locals.var_fn61_calc_iq__vtzeta * assign5760_e6814);
        let assign5760_e6816: f64 = (locals.var_fn61_calc_iq__vto + assign5760_e6815);
        (assign5760_e6816, (locals.var_fn61_calc_iq__vtzeta * locals.var_fn61_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn61_calc_iq__vtof, locals.var_fn61_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn61_calc_iq__vtof = assign5760_e6818;
        locals.var_fn61_calc_iq__vtof_dn4 = assign5760_e6818_d_n4;
        locals.var_fn61_calc_iq__vtof_rv = 0.0;

        let (assign5770_e6826, assign5770_e6826_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5770_e6822: f64 = (locals.var_fn61_calc_iq__tambin / locals.var_fn61_calc_iq__tnomin);
        let assign5770_e6824: f64 = (assign5770_e6822).powf(locals.var_fn61_calc_iq__epsilon);
        (assign5770_e6824, if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn61_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__epsilon * ((assign5770_e6822).powf(locals.var_fn61_calc_iq__epsilon - 1.0) * (locals.var_fn61_calc_iq__tambin_dn4 / locals.var_fn61_calc_iq__tnomin))) } } else { (assign5770_e6824 * (locals.var_fn61_calc_iq__epsilon * ((locals.var_fn61_calc_iq__tambin_dn4 / locals.var_fn61_calc_iq__tnomin) / assign5770_e6822))) },)
    } else {
        (locals.var_fn61_calc_iq__tfacmobin, locals.var_fn61_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn61_calc_iq__tfacmobin = assign5770_e6826;
        locals.var_fn61_calc_iq__tfacmobin_dn4 = assign5770_e6826_d_n4;
        locals.var_fn61_calc_iq__tfacmobin_rv = 0.0;

        let assign5780_e6829: f64 = if locals.var_fn61_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign5780_e6829;
        locals.var_guard62_rv = 0.0;

        let (assign5790_e6847, assign5790_e6847_d_n15, assign5790_e6847_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign5790_e6837: f64 = (locals.var_fn61_calc_iq__absvdsin / locals.var_fn61_calc_iq__dibsat);
        let assign5790_e6839: f64 = (assign5790_e6837).powf(locals.var_fn61_calc_iq__beta);
        let assign5790_e6840: f64 = (1.0 + assign5790_e6839);
        let assign5790_e6843: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign5790_e6844: f64 = (assign5790_e6840).powf(assign5790_e6843);
        let assign5790_e6845: f64 = (locals.var_fn61_calc_iq__absvdsin / assign5790_e6844);
        (assign5790_e6845, (((locals.var_fn61_calc_iq__absvdsin_dn15 * assign5790_e6844) - (locals.var_fn61_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign5790_e6843) as f64).is_finite() && ((assign5790_e6843) as f64).fract() == 0.0 { if assign5790_e6843 == 0.0 { 0.0 } else { (assign5790_e6843 * ((assign5790_e6840).powf(assign5790_e6843 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) })) } } else { (assign5790_e6844 * (assign5790_e6843 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn15 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) } / assign5790_e6840))) })) / (assign5790_e6844 * assign5790_e6844)), (((locals.var_fn61_calc_iq__absvdsin_dn16 * assign5790_e6844) - (locals.var_fn61_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign5790_e6843) as f64).is_finite() && ((assign5790_e6843) as f64).fract() == 0.0 { if assign5790_e6843 == 0.0 { 0.0 } else { (assign5790_e6843 * ((assign5790_e6840).powf(assign5790_e6843 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) })) } } else { (assign5790_e6844 * (assign5790_e6843 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign5790_e6837).powf(locals.var_fn61_calc_iq__beta - 1.0) * (locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat))) } } else { (assign5790_e6839 * (locals.var_fn61_calc_iq__beta * ((locals.var_fn61_calc_iq__absvdsin_dn16 / locals.var_fn61_calc_iq__dibsat) / assign5790_e6837))) } / assign5790_e6840))) })) / (assign5790_e6844 * assign5790_e6844)),)
    } else {
        (locals.var_fn61_calc_iq__vsatdibl, locals.var_fn61_calc_iq__vsatdibl_dn15, locals.var_fn61_calc_iq__vsatdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsatdibl = assign5790_e6847;
        locals.var_fn61_calc_iq__vsatdibl_dn15 = assign5790_e6847_d_n15;
        locals.var_fn61_calc_iq__vsatdibl_dn16 = assign5790_e6847_d_n16;
        locals.var_fn61_calc_iq__vsatdibl_rv = 0.0;

        let (assign5800_e6854, assign5800_e6854_d_n15, assign5800_e6854_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard62 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__vsatdibl, locals.var_fn61_calc_iq__vsatdibl_dn15, locals.var_fn61_calc_iq__vsatdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsatdibl = assign5800_e6854;
        locals.var_fn61_calc_iq__vsatdibl_dn15 = assign5800_e6854_d_n15;
        locals.var_fn61_calc_iq__vsatdibl_dn16 = assign5800_e6854_d_n16;
        locals.var_fn61_calc_iq__vsatdibl_rv = 0.0;

        let (assign5810_e6864, assign5810_e6864_d_n15, assign5810_e6864_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5810_e6859: f64 = (locals.var_fn61_calc_iq__vsatdibl * locals.var_fn61_calc_iq__delta2);
        let assign5810_e6860: f64 = (locals.var_fn61_calc_iq__delta1 - assign5810_e6859);
        let assign5810_e6862: f64 = (assign5810_e6860 * locals.var_fn61_calc_iq__absvdsin);
        (assign5810_e6862, (((-(locals.var_fn61_calc_iq__vsatdibl_dn15 * locals.var_fn61_calc_iq__delta2)) * locals.var_fn61_calc_iq__absvdsin) + (assign5810_e6860 * locals.var_fn61_calc_iq__absvdsin_dn15)), (((-(locals.var_fn61_calc_iq__vsatdibl_dn16 * locals.var_fn61_calc_iq__delta2)) * locals.var_fn61_calc_iq__absvdsin) + (assign5810_e6860 * locals.var_fn61_calc_iq__absvdsin_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__delta, locals.var_fn61_calc_iq__delta_dn15, locals.var_fn61_calc_iq__delta_dn16,)
    }
};
        locals.var_fn61_calc_iq__delta = assign5810_e6864;
        locals.var_fn61_calc_iq__delta_dn15 = assign5810_e6864_d_n15;
        locals.var_fn61_calc_iq__delta_dn16 = assign5810_e6864_d_n16;
        locals.var_fn61_calc_iq__delta_rv = 0.0;

        let (assign5820_e6870, assign5820_e6870_d_n4, assign5820_e6870_d_n15, assign5820_e6870_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5820_e6868: f64 = (locals.var_fn61_calc_iq__vtof - locals.var_fn61_calc_iq__delta);
        (assign5820_e6868, locals.var_fn61_calc_iq__vtof_dn4, (-locals.var_fn61_calc_iq__delta_dn15), (-locals.var_fn61_calc_iq__delta_dn16),)
    } else {
        (locals.var_fn61_calc_iq__vtdibl, locals.var_fn61_calc_iq__vtdibl_dn4, locals.var_fn61_calc_iq__vtdibl_dn15, locals.var_fn61_calc_iq__vtdibl_dn16,)
    }
};
        locals.var_fn61_calc_iq__vtdibl = assign5820_e6870;
        locals.var_fn61_calc_iq__vtdibl_dn4 = assign5820_e6870_d_n4;
        locals.var_fn61_calc_iq__vtdibl_dn15 = assign5820_e6870_d_n15;
        locals.var_fn61_calc_iq__vtdibl_dn16 = assign5820_e6870_d_n16;
        locals.var_fn61_calc_iq__vtdibl_rv = 0.0;

        let (assign5830_e6878, assign5830_e6878_d_n4, assign5830_e6878_d_n15, assign5830_e6878_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5830_e6874: f64 = (2.0 * locals.var_fn61_calc_iq__n);
        let assign5830_e6876: f64 = (assign5830_e6874 * locals.var_fn61_calc_iq__phitin);
        (assign5830_e6876, (((2.0 * locals.var_fn61_calc_iq__n_dn4) * locals.var_fn61_calc_iq__phitin) + (assign5830_e6874 * locals.var_fn61_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn61_calc_iq__n_dn15) * locals.var_fn61_calc_iq__phitin), ((2.0 * locals.var_fn61_calc_iq__n_dn16) * locals.var_fn61_calc_iq__phitin),)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit, locals.var_fn61_calc_iq__two_n_phit_dn4, locals.var_fn61_calc_iq__two_n_phit_dn15, locals.var_fn61_calc_iq__two_n_phit_dn16,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit = assign5830_e6878;
        locals.var_fn61_calc_iq__two_n_phit_dn4 = assign5830_e6878_d_n4;
        locals.var_fn61_calc_iq__two_n_phit_dn15 = assign5830_e6878_d_n15;
        locals.var_fn61_calc_iq__two_n_phit_dn16 = assign5830_e6878_d_n16;
        locals.var_fn61_calc_iq__two_n_phit_rv = 0.0;

        let (assign5840_e6884, assign5840_e6884_d_n4, assign5840_e6884_d_n15, assign5840_e6884_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5840_e6882: f64 = (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit);
        (assign5840_e6882, ((locals.var_fn61_calc_iq__cgin_dn4 * locals.var_fn61_calc_iq__two_n_phit) + (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit_dn4)), (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit_dn15), (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qref, locals.var_fn61_calc_iq__qref_dn4, locals.var_fn61_calc_iq__qref_dn15, locals.var_fn61_calc_iq__qref_dn16,)
    }
};
        locals.var_fn61_calc_iq__qref = assign5840_e6884;
        locals.var_fn61_calc_iq__qref_dn4 = assign5840_e6884_d_n4;
        locals.var_fn61_calc_iq__qref_dn15 = assign5840_e6884_d_n15;
        locals.var_fn61_calc_iq__qref_dn16 = assign5840_e6884_d_n16;
        locals.var_fn61_calc_iq__qref_rv = 0.0;

        let (assign5850_e6894, assign5850_e6894_d_n2, assign5850_e6894_d_n3, assign5850_e6894_d_n4, assign5850_e6894_d_n7, assign5850_e6894_d_n15, assign5850_e6894_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5850_e6889: f64 = (p.p51 * locals.var_fn61_calc_iq__alpha_phit);
        let assign5850_e6891: f64 = (assign5850_e6889 / 2.0);
        let assign5850_e6892: f64 = (locals.var_fn61_calc_iq__vtdibl - assign5850_e6891);
        (assign5850_e6892, 0.0, 0.0, (locals.var_fn61_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn61_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn61_calc_iq__vtdibl_dn15, locals.var_fn61_calc_iq__vtdibl_dn16,)
    } else {
        (locals.var_fn61_calc_iq__myarg, locals.var_fn61_calc_iq__myarg_dn2, locals.var_fn61_calc_iq__myarg_dn3, locals.var_fn61_calc_iq__myarg_dn4, locals.var_fn61_calc_iq__myarg_dn7, locals.var_fn61_calc_iq__myarg_dn15, locals.var_fn61_calc_iq__myarg_dn16,)
    }
};
        locals.var_fn61_calc_iq__myarg = assign5850_e6894;
        locals.var_fn61_calc_iq__myarg_dn2 = assign5850_e6894_d_n2;
        locals.var_fn61_calc_iq__myarg_dn3 = assign5850_e6894_d_n3;
        locals.var_fn61_calc_iq__myarg_dn4 = assign5850_e6894_d_n4;
        locals.var_fn61_calc_iq__myarg_dn7 = assign5850_e6894_d_n7;
        locals.var_fn61_calc_iq__myarg_dn15 = assign5850_e6894_d_n15;
        locals.var_fn61_calc_iq__myarg_dn16 = assign5850_e6894_d_n16;
        locals.var_fn61_calc_iq__myarg_rv = 0.0;

        let (assign5860_e6945, assign5860_e6945_d_n2, assign5860_e6945_d_n3, assign5860_e6945_d_n4, assign5860_e6945_d_n7, assign5860_e6945_d_n15, assign5860_e6945_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign5860_e6939, assign5860_e6939_d_n2, assign5860_e6939_d_n7, assign5860_e6939_d_n15, assign5860_e6939_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign5860_e6903: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                let assign5860_e6906: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5860_e6909: f64 = (0.001 / p.p53);
                let assign5860_e6912: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5860_e6913: f64 = (assign5860_e6909 * assign5860_e6912);
                let assign5860_e6914: f64 = (assign5860_e6913).tanh();
                let assign5860_e6915: f64 = (assign5860_e6906 * assign5860_e6914);
                let assign5860_e6916: f64 = (assign5860_e6903 + assign5860_e6915);
                let assign5860_e6917: f64 = (0.5 * assign5860_e6916);
                (assign5860_e6917, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + (((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5860_e6914) + (assign5860_e6906 * ((assign5860_e6909 * (-locals.var_fn61_calc_iq__vgdin_dn16)) / ((assign5860_e6913).cosh() * (assign5860_e6913).cosh())))))),)
            } else {
                let (assign5860_e6938, assign5860_e6938_d_n2, assign5860_e6938_d_n7, assign5860_e6938_d_n15, assign5860_e6938_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign5860_e6924: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                        let assign5860_e6927: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5860_e6930: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5860_e6931: f64 = (assign5860_e6927 * assign5860_e6930);
                        let assign5860_e6933: f64 = (assign5860_e6931 + p.p53);
                        let assign5860_e6934: f64 = (assign5860_e6933).sqrt();
                        let assign5860_e6935: f64 = (assign5860_e6924 + assign5860_e6934);
                        let assign5860_e6936: f64 = (0.5 * assign5860_e6935);
                        (assign5860_e6936, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + ((((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5860_e6930) + (assign5860_e6927 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2))) / (2.0 * assign5860_e6934)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + ((((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5860_e6930) + (assign5860_e6927 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7))) / (2.0 * assign5860_e6934)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + ((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5860_e6930) + (assign5860_e6927 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15))) / (2.0 * assign5860_e6934)))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + ((((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5860_e6930) + (assign5860_e6927 * (-locals.var_fn61_calc_iq__vgdin_dn16))) / (2.0 * assign5860_e6934)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign5860_e6938, assign5860_e6938_d_n2, assign5860_e6938_d_n7, assign5860_e6938_d_n15, assign5860_e6938_d_n16,)
            }
        };
        let assign5860_e6941: f64 = (assign5860_e6939 - locals.var_fn61_calc_iq__myarg);
        let assign5860_e6943: f64 = (assign5860_e6941 / locals.var_fn61_calc_iq__alpha_phit);
        (assign5860_e6943, ((assign5860_e6939_d_n2 - locals.var_fn61_calc_iq__myarg_dn2) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn3) / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign5860_e6941 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), ((assign5860_e6939_d_n7 - locals.var_fn61_calc_iq__myarg_dn7) / locals.var_fn61_calc_iq__alpha_phit), ((assign5860_e6939_d_n15 - locals.var_fn61_calc_iq__myarg_dn15) / locals.var_fn61_calc_iq__alpha_phit), ((assign5860_e6939_d_n16 - locals.var_fn61_calc_iq__myarg_dn16) / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign5860_e6945;
        locals.var_fn61_calc_iq__exparg_dn2 = assign5860_e6945_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign5860_e6945_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign5860_e6945_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign5860_e6945_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign5860_e6945_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign5860_e6945_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let assign5870_e6948: f64 = if locals.var_fn61_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign5870_e6948;
        locals.var_guard63_rv = 0.0;

        let (assign5880_e6954, assign5880_e6954_d_n2, assign5880_e6954_d_n3, assign5880_e6954_d_n4, assign5880_e6954_d_n7, assign5880_e6954_d_n15, assign5880_e6954_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard63 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5880_e6954;
        locals.var_fn61_calc_iq__ff_dn2 = assign5880_e6954_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5880_e6954_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5880_e6954_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5880_e6954_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5880_e6954_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5880_e6954_d_n16;
        locals.var_fn61_calc_iq__ff_rv = 0.0;

        let assign5890_e6957: f64 = (-50.0);
        let assign5890_e6958: f64 = if locals.var_fn61_calc_iq__exparg < assign5890_e6957 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign5890_e6958;
        locals.var_guard64_rv = 0.0;

        let (assign5900_e6967, assign5900_e6967_d_n2, assign5900_e6967_d_n3, assign5900_e6967_d_n4, assign5900_e6967_d_n7, assign5900_e6967_d_n15, assign5900_e6967_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard64 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5900_e6967;
        locals.var_fn61_calc_iq__ff_dn2 = assign5900_e6967_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5900_e6967_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5900_e6967_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5900_e6967_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5900_e6967_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5900_e6967_d_n16;
        locals.var_fn61_calc_iq__ff_rv = 0.0;

        let (assign5910_e6982, assign5910_e6982_d_n2, assign5910_e6982_d_n3, assign5910_e6982_d_n4, assign5910_e6982_d_n7, assign5910_e6982_d_n15, assign5910_e6982_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard63 == 0.0)) && (locals.var_guard64 == 0.0)) {
        let assign5910_e6978: f64 = (locals.var_fn61_calc_iq__exparg).exp();
        let assign5910_e6979: f64 = (1.0 + assign5910_e6978);
        let assign5910_e6980: f64 = (1.0 / assign5910_e6979);
        (assign5910_e6980, (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn2) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn3) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn4) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn7) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn15) / (assign5910_e6979 * assign5910_e6979))), (-((assign5910_e6978 * locals.var_fn61_calc_iq__exparg_dn16) / (assign5910_e6979 * assign5910_e6979))),)
    } else {
        (locals.var_fn61_calc_iq__ff, locals.var_fn61_calc_iq__ff_dn2, locals.var_fn61_calc_iq__ff_dn3, locals.var_fn61_calc_iq__ff_dn4, locals.var_fn61_calc_iq__ff_dn7, locals.var_fn61_calc_iq__ff_dn15, locals.var_fn61_calc_iq__ff_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff = assign5910_e6982;
        locals.var_fn61_calc_iq__ff_dn2 = assign5910_e6982_d_n2;
        locals.var_fn61_calc_iq__ff_dn3 = assign5910_e6982_d_n3;
        locals.var_fn61_calc_iq__ff_dn4 = assign5910_e6982_d_n4;
        locals.var_fn61_calc_iq__ff_dn7 = assign5910_e6982_d_n7;
        locals.var_fn61_calc_iq__ff_dn15 = assign5910_e6982_d_n15;
        locals.var_fn61_calc_iq__ff_dn16 = assign5910_e6982_d_n16;
        locals.var_fn61_calc_iq__ff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5920_e7041, assign5920_e7041_d_n2, assign5920_e7041_d_n3, assign5920_e7041_d_n4, assign5920_e7041_d_n7, assign5920_e7041_d_n15, assign5920_e7041_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign5920_e7027, assign5920_e7027_d_n2, assign5920_e7027_d_n7, assign5920_e7027_d_n15, assign5920_e7027_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign5920_e6991: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                let assign5920_e6994: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5920_e6997: f64 = (0.001 / p.p53);
                let assign5920_e7000: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign5920_e7001: f64 = (assign5920_e6997 * assign5920_e7000);
                let assign5920_e7002: f64 = (assign5920_e7001).tanh();
                let assign5920_e7003: f64 = (assign5920_e6994 * assign5920_e7002);
                let assign5920_e7004: f64 = (assign5920_e6991 + assign5920_e7003);
                let assign5920_e7005: f64 = (0.5 * assign5920_e7004);
                (assign5920_e7005, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + (((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5920_e7002) + (assign5920_e6994 * ((assign5920_e6997 * (-locals.var_fn61_calc_iq__vgdin_dn16)) / ((assign5920_e7001).cosh() * (assign5920_e7001).cosh())))))),)
            } else {
                let (assign5920_e7026, assign5920_e7026_d_n2, assign5920_e7026_d_n7, assign5920_e7026_d_n15, assign5920_e7026_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign5920_e7012: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                        let assign5920_e7015: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5920_e7018: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign5920_e7019: f64 = (assign5920_e7015 * assign5920_e7018);
                        let assign5920_e7021: f64 = (assign5920_e7019 + p.p53);
                        let assign5920_e7022: f64 = (assign5920_e7021).sqrt();
                        let assign5920_e7023: f64 = (assign5920_e7012 + assign5920_e7022);
                        let assign5920_e7024: f64 = (0.5 * assign5920_e7023);
                        (assign5920_e7024, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + ((((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign5920_e7018) + (assign5920_e7015 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2))) / (2.0 * assign5920_e7022)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + ((((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign5920_e7018) + (assign5920_e7015 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7))) / (2.0 * assign5920_e7022)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + ((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign5920_e7018) + (assign5920_e7015 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15))) / (2.0 * assign5920_e7022)))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + ((((-locals.var_fn61_calc_iq__vgdin_dn16) * assign5920_e7018) + (assign5920_e7015 * (-locals.var_fn61_calc_iq__vgdin_dn16))) / (2.0 * assign5920_e7022)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign5920_e7026, assign5920_e7026_d_n2, assign5920_e7026_d_n7, assign5920_e7026_d_n15, assign5920_e7026_d_n16,)
            }
        };
        let assign5920_e7031: f64 = (p.p51 * 0.1);
        let assign5920_e7033: f64 = (assign5920_e7031 * locals.var_fn61_calc_iq__alpha_phit);
        let assign5920_e7035: f64 = (assign5920_e7033 * locals.var_fn61_calc_iq__ff);
        let assign5920_e7036: f64 = (locals.var_fn61_calc_iq__vtdibl - assign5920_e7035);
        let assign5920_e7037: f64 = (assign5920_e7027 - assign5920_e7036);
        let assign5920_e7039: f64 = (assign5920_e7037 / locals.var_fn61_calc_iq__two_n_phit);
        (assign5920_e7039, ((assign5920_e7027_d_n2 - (-(assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn2))) / locals.var_fn61_calc_iq__two_n_phit), ((-(-(assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn3))) / locals.var_fn61_calc_iq__two_n_phit), ((((-(locals.var_fn61_calc_iq__vtdibl_dn4 - (((assign5920_e7031 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ff) + (assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn4)))) * locals.var_fn61_calc_iq__two_n_phit) - (assign5920_e7037 * locals.var_fn61_calc_iq__two_n_phit_dn4)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), ((assign5920_e7027_d_n7 - (-(assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn7))) / locals.var_fn61_calc_iq__two_n_phit), ((((assign5920_e7027_d_n15 - (locals.var_fn61_calc_iq__vtdibl_dn15 - (assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn15))) * locals.var_fn61_calc_iq__two_n_phit) - (assign5920_e7037 * locals.var_fn61_calc_iq__two_n_phit_dn15)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), ((((assign5920_e7027_d_n16 - (locals.var_fn61_calc_iq__vtdibl_dn16 - (assign5920_e7033 * locals.var_fn61_calc_iq__ff_dn16))) * locals.var_fn61_calc_iq__two_n_phit) - (assign5920_e7037 * locals.var_fn61_calc_iq__two_n_phit_dn16)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn61_calc_iq__eta, locals.var_fn61_calc_iq__eta_dn2, locals.var_fn61_calc_iq__eta_dn3, locals.var_fn61_calc_iq__eta_dn4, locals.var_fn61_calc_iq__eta_dn7, locals.var_fn61_calc_iq__eta_dn15, locals.var_fn61_calc_iq__eta_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta = assign5920_e7041;
        locals.var_fn61_calc_iq__eta_dn2 = assign5920_e7041_d_n2;
        locals.var_fn61_calc_iq__eta_dn3 = assign5920_e7041_d_n3;
        locals.var_fn61_calc_iq__eta_dn4 = assign5920_e7041_d_n4;
        locals.var_fn61_calc_iq__eta_dn7 = assign5920_e7041_d_n7;
        locals.var_fn61_calc_iq__eta_dn15 = assign5920_e7041_d_n15;
        locals.var_fn61_calc_iq__eta_dn16 = assign5920_e7041_d_n16;
        locals.var_fn61_calc_iq__eta_rv = 0.0;

        let assign5930_e7044: f64 = if locals.var_fn61_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign5930_e7044;
        locals.var_guard65_rv = 0.0;

        let (assign5940_e7052, assign5940_e7052_d_n2, assign5940_e7052_d_n3, assign5940_e7052_d_n4, assign5940_e7052_d_n7, assign5940_e7052_d_n15, assign5940_e7052_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign5940_e7050: f64 = (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta);
        (assign5940_e7050, (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn2), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn3), ((locals.var_fn61_calc_iq__qref_dn4 * locals.var_fn61_calc_iq__eta) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn4)), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn7), ((locals.var_fn61_calc_iq__qref_dn15 * locals.var_fn61_calc_iq__eta) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn15)), ((locals.var_fn61_calc_iq__qref_dn16 * locals.var_fn61_calc_iq__eta) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__eta_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5940_e7052;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5940_e7052_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5940_e7052_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5940_e7052_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5940_e7052_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5940_e7052_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5940_e7052_d_n16;
        locals.var_fn61_calc_iq__qinvv_rv = 0.0;

        let assign5950_e7055: f64 = (-50.0);
        let assign5950_e7056: f64 = if locals.var_fn61_calc_iq__eta < assign5950_e7055 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign5950_e7056;
        locals.var_guard66_rv = 0.0;

        let (assign5960_e7068, assign5960_e7068_d_n2, assign5960_e7068_d_n3, assign5960_e7068_d_n4, assign5960_e7068_d_n7, assign5960_e7068_d_n15, assign5960_e7068_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard65 == 0.0)) && (locals.var_guard66 != 0.0)) {
        let assign5960_e7065: f64 = (locals.var_fn61_calc_iq__eta).exp();
        let assign5960_e7066: f64 = (locals.var_fn61_calc_iq__qref * assign5960_e7065);
        (assign5960_e7066, (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn2)), (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn3)), ((locals.var_fn61_calc_iq__qref_dn4 * assign5960_e7065) + (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn4))), (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn7)), ((locals.var_fn61_calc_iq__qref_dn15 * assign5960_e7065) + (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn15))), ((locals.var_fn61_calc_iq__qref_dn16 * assign5960_e7065) + (locals.var_fn61_calc_iq__qref * (assign5960_e7065 * locals.var_fn61_calc_iq__eta_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5960_e7068;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5960_e7068_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5960_e7068_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5960_e7068_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5960_e7068_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5960_e7068_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5960_e7068_d_n16;
        locals.var_fn61_calc_iq__qinvv_rv = 0.0;

        let (assign5970_e7084, assign5970_e7084_d_n2, assign5970_e7084_d_n3, assign5970_e7084_d_n4, assign5970_e7084_d_n7, assign5970_e7084_d_n15, assign5970_e7084_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard65 == 0.0)) && (locals.var_guard66 == 0.0)) {
        let assign5970_e7079: f64 = (locals.var_fn61_calc_iq__eta).exp();
        let assign5970_e7080: f64 = (1.0 + assign5970_e7079);
        let assign5970_e7081: f64 = (assign5970_e7080).ln();
        let assign5970_e7082: f64 = (locals.var_fn61_calc_iq__qref * assign5970_e7081);
        (assign5970_e7082, (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn2) / assign5970_e7080)), (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn3) / assign5970_e7080)), ((locals.var_fn61_calc_iq__qref_dn4 * assign5970_e7081) + (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn4) / assign5970_e7080))), (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn7) / assign5970_e7080)), ((locals.var_fn61_calc_iq__qref_dn15 * assign5970_e7081) + (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn15) / assign5970_e7080))), ((locals.var_fn61_calc_iq__qref_dn16 * assign5970_e7081) + (locals.var_fn61_calc_iq__qref * ((assign5970_e7079 * locals.var_fn61_calc_iq__eta_dn16) / assign5970_e7080))),)
    } else {
        (locals.var_fn61_calc_iq__qinvv, locals.var_fn61_calc_iq__qinvv_dn2, locals.var_fn61_calc_iq__qinvv_dn3, locals.var_fn61_calc_iq__qinvv_dn4, locals.var_fn61_calc_iq__qinvv_dn7, locals.var_fn61_calc_iq__qinvv_dn15, locals.var_fn61_calc_iq__qinvv_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv = assign5970_e7084;
        locals.var_fn61_calc_iq__qinvv_dn2 = assign5970_e7084_d_n2;
        locals.var_fn61_calc_iq__qinvv_dn3 = assign5970_e7084_d_n3;
        locals.var_fn61_calc_iq__qinvv_dn4 = assign5970_e7084_d_n4;
        locals.var_fn61_calc_iq__qinvv_dn7 = assign5970_e7084_d_n7;
        locals.var_fn61_calc_iq__qinvv_dn15 = assign5970_e7084_d_n15;
        locals.var_fn61_calc_iq__qinvv_dn16 = assign5970_e7084_d_n16;
        locals.var_fn61_calc_iq__qinvv_rv = 0.0;

        let (assign5980_e7098, assign5980_e7098_d_n2, assign5980_e7098_d_n3, assign5980_e7098_d_n4, assign5980_e7098_d_n7, assign5980_e7098_d_n15, assign5980_e7098_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5980_e7091: f64 = (locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv);
        let assign5980_e7093: f64 = (assign5980_e7091 / locals.var_fn61_calc_iq__cgin);
        let assign5980_e7094: f64 = (1.0 + assign5980_e7093);
        let assign5980_e7095: f64 = (locals.var_fn61_calc_iq__tfacmobin * assign5980_e7094);
        let assign5980_e7096: f64 = (locals.var_fn61_calc_iq__mu0 / assign5980_e7095);
        (assign5980_e7096, (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn2) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn3) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * ((locals.var_fn61_calc_iq__tfacmobin_dn4 * assign5980_e7094) + (locals.var_fn61_calc_iq__tfacmobin * ((((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn4) * locals.var_fn61_calc_iq__cgin) - (assign5980_e7091 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin))))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn7) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn15) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))), (-((locals.var_fn61_calc_iq__mu0 * (locals.var_fn61_calc_iq__tfacmobin * ((locals.var_fn61_calc_iq__mtheta * locals.var_fn61_calc_iq__qinvv_dn16) / locals.var_fn61_calc_iq__cgin))) / (assign5980_e7095 * assign5980_e7095))),)
    } else {
        (locals.var_fn61_calc_iq__muf, locals.var_fn61_calc_iq__muf_dn2, locals.var_fn61_calc_iq__muf_dn3, locals.var_fn61_calc_iq__muf_dn4, locals.var_fn61_calc_iq__muf_dn7, locals.var_fn61_calc_iq__muf_dn15, locals.var_fn61_calc_iq__muf_dn16,)
    }
};
        locals.var_fn61_calc_iq__muf = assign5980_e7098;
        locals.var_fn61_calc_iq__muf_dn2 = assign5980_e7098_d_n2;
        locals.var_fn61_calc_iq__muf_dn3 = assign5980_e7098_d_n3;
        locals.var_fn61_calc_iq__muf_dn4 = assign5980_e7098_d_n4;
        locals.var_fn61_calc_iq__muf_dn7 = assign5980_e7098_d_n7;
        locals.var_fn61_calc_iq__muf_dn15 = assign5980_e7098_d_n15;
        locals.var_fn61_calc_iq__muf_dn16 = assign5980_e7098_d_n16;
        locals.var_fn61_calc_iq__muf_rv = 0.0;

        let (assign5990_e7130, assign5990_e7130_d_n2, assign5990_e7130_d_n3, assign5990_e7130_d_n4, assign5990_e7130_d_n7, assign5990_e7130_d_n15, assign5990_e7130_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign5990_e7104: f64 = (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tnomin);
        let assign5990_e7105: f64 = (1.0 + assign5990_e7104);
        let assign5990_e7109: f64 = (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tambin);
        let assign5990_e7110: f64 = (1.0 + assign5990_e7109);
        let assign5990_e7111: f64 = (assign5990_e7105 / assign5990_e7110);
        let assign5990_e7112: f64 = (locals.var_fn61_calc_iq__vel0 * assign5990_e7111);
        let assign5990_e7116: f64 = (locals.var_fn61_calc_iq__lambda * locals.var_fn61_calc_iq__absvdsin);
        let assign5990_e7118: f64 = (assign5990_e7116 / locals.var_fn61_calc_iq__lin);
        let assign5990_e7119: f64 = (1.0 + assign5990_e7118);
        let assign5990_e7120: f64 = (assign5990_e7112 * assign5990_e7119);
        let assign5990_e7124: f64 = (locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv);
        let assign5990_e7126: f64 = (assign5990_e7124 / locals.var_fn61_calc_iq__cgin);
        let assign5990_e7127: f64 = (1.0 + assign5990_e7126);
        let assign5990_e7128: f64 = (assign5990_e7120 / assign5990_e7127);
        (assign5990_e7128, (-((assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn2) / locals.var_fn61_calc_iq__cgin)) / (assign5990_e7127 * assign5990_e7127))), (-((assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn3) / locals.var_fn61_calc_iq__cgin)) / (assign5990_e7127 * assign5990_e7127))), (((((locals.var_fn61_calc_iq__vel0 * (-((assign5990_e7105 * (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tambin_dn4)) / (assign5990_e7110 * assign5990_e7110)))) * assign5990_e7119) * assign5990_e7127) - (assign5990_e7120 * ((((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn4) * locals.var_fn61_calc_iq__cgin) - (assign5990_e7124 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin)))) / (assign5990_e7127 * assign5990_e7127)), (-((assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn7) / locals.var_fn61_calc_iq__cgin)) / (assign5990_e7127 * assign5990_e7127))), ((((assign5990_e7112 * ((locals.var_fn61_calc_iq__lambda * locals.var_fn61_calc_iq__absvdsin_dn15) / locals.var_fn61_calc_iq__lin)) * assign5990_e7127) - (assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn15) / locals.var_fn61_calc_iq__cgin))) / (assign5990_e7127 * assign5990_e7127)), ((((assign5990_e7112 * ((locals.var_fn61_calc_iq__lambda * locals.var_fn61_calc_iq__absvdsin_dn16) / locals.var_fn61_calc_iq__lin)) * assign5990_e7127) - (assign5990_e7120 * ((locals.var_fn61_calc_iq__vtheta * locals.var_fn61_calc_iq__qinvv_dn16) / locals.var_fn61_calc_iq__cgin))) / (assign5990_e7127 * assign5990_e7127)),)
    } else {
        (locals.var_fn61_calc_iq__vx, locals.var_fn61_calc_iq__vx_dn2, locals.var_fn61_calc_iq__vx_dn3, locals.var_fn61_calc_iq__vx_dn4, locals.var_fn61_calc_iq__vx_dn7, locals.var_fn61_calc_iq__vx_dn15, locals.var_fn61_calc_iq__vx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vx = assign5990_e7130;
        locals.var_fn61_calc_iq__vx_dn2 = assign5990_e7130_d_n2;
        locals.var_fn61_calc_iq__vx_dn3 = assign5990_e7130_d_n3;
        locals.var_fn61_calc_iq__vx_dn4 = assign5990_e7130_d_n4;
        locals.var_fn61_calc_iq__vx_dn7 = assign5990_e7130_d_n7;
        locals.var_fn61_calc_iq__vx_dn15 = assign5990_e7130_d_n15;
        locals.var_fn61_calc_iq__vx_dn16 = assign5990_e7130_d_n16;
        locals.var_fn61_calc_iq__vx_rv = 0.0;

        let (assign6010_e7156, assign6010_e7156_d_n2, assign6010_e7156_d_n3, assign6010_e7156_d_n4, assign6010_e7156_d_n7, assign6010_e7156_d_n15, assign6010_e7156_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6010_e7152: f64 = (locals.var_fn61_calc_iq__vx * locals.var_fn61_calc_iq__lin);
        let assign6010_e7154: f64 = (assign6010_e7152 / locals.var_fn61_calc_iq__muf);
        (assign6010_e7154, ((((locals.var_fn61_calc_iq__vx_dn2 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn2)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn3 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn3)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn4 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn4)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn7 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn7)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn15 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn15)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)), ((((locals.var_fn61_calc_iq__vx_dn16 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf) - (assign6010_e7152 * locals.var_fn61_calc_iq__muf_dn16)) / (locals.var_fn61_calc_iq__muf * locals.var_fn61_calc_iq__muf)),)
    } else {
        (locals.var_fn61_calc_iq__vdsats, locals.var_fn61_calc_iq__vdsats_dn2, locals.var_fn61_calc_iq__vdsats_dn3, locals.var_fn61_calc_iq__vdsats_dn4, locals.var_fn61_calc_iq__vdsats_dn7, locals.var_fn61_calc_iq__vdsats_dn15, locals.var_fn61_calc_iq__vdsats_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats = assign6010_e7156;
        locals.var_fn61_calc_iq__vdsats_dn2 = assign6010_e7156_d_n2;
        locals.var_fn61_calc_iq__vdsats_dn3 = assign6010_e7156_d_n3;
        locals.var_fn61_calc_iq__vdsats_dn4 = assign6010_e7156_d_n4;
        locals.var_fn61_calc_iq__vdsats_dn7 = assign6010_e7156_d_n7;
        locals.var_fn61_calc_iq__vdsats_dn15 = assign6010_e7156_d_n15;
        locals.var_fn61_calc_iq__vdsats_dn16 = assign6010_e7156_d_n16;
        locals.var_fn61_calc_iq__vdsats_rv = 0.0;

        let (assign6020_e7173, assign6020_e7173_d_n2, assign6020_e7173_d_n3, assign6020_e7173_d_n4, assign6020_e7173_d_n7, assign6020_e7173_d_n15, assign6020_e7173_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6020_e7162: f64 = (2.0 * locals.var_fn61_calc_iq__qinvv);
        let assign6020_e7164: f64 = (assign6020_e7162 / locals.var_fn61_calc_iq__cgin);
        let assign6020_e7166: f64 = (assign6020_e7164 / locals.var_fn61_calc_iq__vdsats);
        let assign6020_e7167: f64 = (1.0 + assign6020_e7166);
        let assign6020_e7168: f64 = (assign6020_e7167).sqrt();
        let assign6020_e7169: f64 = (locals.var_fn61_calc_iq__vdsats * assign6020_e7168);
        let assign6020_e7171: f64 = (assign6020_e7169 - locals.var_fn61_calc_iq__vdsats);
        (assign6020_e7171, (((locals.var_fn61_calc_iq__vdsats_dn2 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn2) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn2)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn2), (((locals.var_fn61_calc_iq__vdsats_dn3 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn3) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn3)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn3), (((locals.var_fn61_calc_iq__vdsats_dn4 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn4) * locals.var_fn61_calc_iq__cgin) - (assign6020_e7162 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin)) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn4)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn4), (((locals.var_fn61_calc_iq__vdsats_dn7 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn7) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn7)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn7), (((locals.var_fn61_calc_iq__vdsats_dn15 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn15) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn15)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn15), (((locals.var_fn61_calc_iq__vdsats_dn16 * assign6020_e7168) + (locals.var_fn61_calc_iq__vdsats * ((((((2.0 * locals.var_fn61_calc_iq__qinvv_dn16) / locals.var_fn61_calc_iq__cgin) * locals.var_fn61_calc_iq__vdsats) - (assign6020_e7164 * locals.var_fn61_calc_iq__vdsats_dn16)) / (locals.var_fn61_calc_iq__vdsats * locals.var_fn61_calc_iq__vdsats)) / (2.0 * assign6020_e7168)))) - locals.var_fn61_calc_iq__vdsats_dn16),)
    } else {
        (locals.var_fn61_calc_iq__vdsats1, locals.var_fn61_calc_iq__vdsats1_dn2, locals.var_fn61_calc_iq__vdsats1_dn3, locals.var_fn61_calc_iq__vdsats1_dn4, locals.var_fn61_calc_iq__vdsats1_dn7, locals.var_fn61_calc_iq__vdsats1_dn15, locals.var_fn61_calc_iq__vdsats1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats1 = assign6020_e7173;
        locals.var_fn61_calc_iq__vdsats1_dn2 = assign6020_e7173_d_n2;
        locals.var_fn61_calc_iq__vdsats1_dn3 = assign6020_e7173_d_n3;
        locals.var_fn61_calc_iq__vdsats1_dn4 = assign6020_e7173_d_n4;
        locals.var_fn61_calc_iq__vdsats1_dn7 = assign6020_e7173_d_n7;
        locals.var_fn61_calc_iq__vdsats1_dn15 = assign6020_e7173_d_n15;
        locals.var_fn61_calc_iq__vdsats1_dn16 = assign6020_e7173_d_n16;
        locals.var_fn61_calc_iq__vdsats1_rv = 0.0;

        let (assign6030_e7185, assign6030_e7185_d_n2, assign6030_e7185_d_n3, assign6030_e7185_d_n4, assign6030_e7185_d_n7, assign6030_e7185_d_n15, assign6030_e7185_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6030_e7178: f64 = (1.0 - locals.var_fn61_calc_iq__ff);
        let assign6030_e7179: f64 = (locals.var_fn61_calc_iq__vdsats * assign6030_e7178);
        let assign6030_e7182: f64 = (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff);
        let assign6030_e7183: f64 = (assign6030_e7179 + assign6030_e7182);
        (assign6030_e7183, (((locals.var_fn61_calc_iq__vdsats_dn2 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn2))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn2)), (((locals.var_fn61_calc_iq__vdsats_dn3 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn3))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn3)), (((locals.var_fn61_calc_iq__vdsats_dn4 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn4))) + ((locals.var_fn61_calc_iq__two_n_phit_dn4 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn4))), (((locals.var_fn61_calc_iq__vdsats_dn7 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn7))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn7)), (((locals.var_fn61_calc_iq__vdsats_dn15 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn15))) + ((locals.var_fn61_calc_iq__two_n_phit_dn15 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn15))), (((locals.var_fn61_calc_iq__vdsats_dn16 * assign6030_e7178) + (locals.var_fn61_calc_iq__vdsats * (-locals.var_fn61_calc_iq__ff_dn16))) + ((locals.var_fn61_calc_iq__two_n_phit_dn16 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__vdsat, locals.var_fn61_calc_iq__vdsat_dn2, locals.var_fn61_calc_iq__vdsat_dn3, locals.var_fn61_calc_iq__vdsat_dn4, locals.var_fn61_calc_iq__vdsat_dn7, locals.var_fn61_calc_iq__vdsat_dn15, locals.var_fn61_calc_iq__vdsat_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat = assign6030_e7185;
        locals.var_fn61_calc_iq__vdsat_dn2 = assign6030_e7185_d_n2;
        locals.var_fn61_calc_iq__vdsat_dn3 = assign6030_e7185_d_n3;
        locals.var_fn61_calc_iq__vdsat_dn4 = assign6030_e7185_d_n4;
        locals.var_fn61_calc_iq__vdsat_dn7 = assign6030_e7185_d_n7;
        locals.var_fn61_calc_iq__vdsat_dn15 = assign6030_e7185_d_n15;
        locals.var_fn61_calc_iq__vdsat_dn16 = assign6030_e7185_d_n16;
        locals.var_fn61_calc_iq__vdsat_rv = 0.0;

        let (assign6040_e7197, assign6040_e7197_d_n2, assign6040_e7197_d_n3, assign6040_e7197_d_n4, assign6040_e7197_d_n7, assign6040_e7197_d_n15, assign6040_e7197_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6040_e7190: f64 = (1.0 - locals.var_fn61_calc_iq__ff);
        let assign6040_e7191: f64 = (locals.var_fn61_calc_iq__vdsats1 * assign6040_e7190);
        let assign6040_e7194: f64 = (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff);
        let assign6040_e7195: f64 = (assign6040_e7191 + assign6040_e7194);
        (assign6040_e7195, (((locals.var_fn61_calc_iq__vdsats1_dn2 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn2))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn2)), (((locals.var_fn61_calc_iq__vdsats1_dn3 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn3))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn3)), (((locals.var_fn61_calc_iq__vdsats1_dn4 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn4))) + ((locals.var_fn61_calc_iq__two_n_phit_dn4 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn4))), (((locals.var_fn61_calc_iq__vdsats1_dn7 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn7))) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn7)), (((locals.var_fn61_calc_iq__vdsats1_dn15 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn15))) + ((locals.var_fn61_calc_iq__two_n_phit_dn15 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn15))), (((locals.var_fn61_calc_iq__vdsats1_dn16 * assign6040_e7190) + (locals.var_fn61_calc_iq__vdsats1 * (-locals.var_fn61_calc_iq__ff_dn16))) + ((locals.var_fn61_calc_iq__two_n_phit_dn16 * locals.var_fn61_calc_iq__ff) + (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__ff_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__vdsat1, locals.var_fn61_calc_iq__vdsat1_dn2, locals.var_fn61_calc_iq__vdsat1_dn3, locals.var_fn61_calc_iq__vdsat1_dn4, locals.var_fn61_calc_iq__vdsat1_dn7, locals.var_fn61_calc_iq__vdsat1_dn15, locals.var_fn61_calc_iq__vdsat1_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat1 = assign6040_e7197;
        locals.var_fn61_calc_iq__vdsat1_dn2 = assign6040_e7197_d_n2;
        locals.var_fn61_calc_iq__vdsat1_dn3 = assign6040_e7197_d_n3;
        locals.var_fn61_calc_iq__vdsat1_dn4 = assign6040_e7197_d_n4;
        locals.var_fn61_calc_iq__vdsat1_dn7 = assign6040_e7197_d_n7;
        locals.var_fn61_calc_iq__vdsat1_dn15 = assign6040_e7197_d_n15;
        locals.var_fn61_calc_iq__vdsat1_dn16 = assign6040_e7197_d_n16;
        locals.var_fn61_calc_iq__vdsat1_rv = 0.0;

        let (assign6050_e7266, assign6050_e7266_d_n2, assign6050_e7266_d_n3, assign6050_e7266_d_n4, assign6050_e7266_d_n7, assign6050_e7266_d_n15, assign6050_e7266_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6050_e7256, assign6050_e7256_d_n2, assign6050_e7256_d_n3, assign6050_e7256_d_n4, assign6050_e7256_d_n7, assign6050_e7256_d_n15, assign6050_e7256_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6050_e7209: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                let assign6050_e7210: f64 = assign6050_e7209;
                let assign6050_e7214: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                let assign6050_e7215: f64 = (-assign6050_e7214);
                let assign6050_e7218: f64 = (0.001 / p.p53);
                let assign6050_e7222: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                let assign6050_e7223: f64 = (-assign6050_e7222);
                let assign6050_e7224: f64 = (assign6050_e7218 * assign6050_e7223);
                let assign6050_e7225: f64 = (assign6050_e7224).tanh();
                let assign6050_e7226: f64 = (assign6050_e7215 * assign6050_e7225);
                let assign6050_e7227: f64 = (assign6050_e7210 + assign6050_e7226);
                let assign6050_e7228: f64 = (0.5 * assign6050_e7227);
                (assign6050_e7228, (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7225) + (assign6050_e7215 * ((assign6050_e7218 * (-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6050_e7224).cosh() * (assign6050_e7224).cosh())))))),)
            } else {
                let (assign6050_e7255, assign6050_e7255_d_n2, assign6050_e7255_d_n3, assign6050_e7255_d_n4, assign6050_e7255_d_n7, assign6050_e7255_d_n15, assign6050_e7255_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6050_e7236: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                        let assign6050_e7237: f64 = assign6050_e7236;
                        let assign6050_e7241: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                        let assign6050_e7242: f64 = (-assign6050_e7241);
                        let assign6050_e7246: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat1);
                        let assign6050_e7247: f64 = (-assign6050_e7246);
                        let assign6050_e7248: f64 = (assign6050_e7242 * assign6050_e7247);
                        let assign6050_e7250: f64 = (assign6050_e7248 + p.p53);
                        let assign6050_e7251: f64 = (assign6050_e7250).sqrt();
                        let assign6050_e7252: f64 = (assign6050_e7237 + assign6050_e7251);
                        let assign6050_e7253: f64 = (0.5 * assign6050_e7252);
                        (assign6050_e7253, (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6050_e7247) + (assign6050_e7242 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6050_e7251)))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7247) + (assign6050_e7242 * (-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6050_e7251)))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6050_e7247) + (assign6050_e7242 * (-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat1) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6050_e7251)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6050_e7255, assign6050_e7255_d_n2, assign6050_e7255_d_n3, assign6050_e7255_d_n4, assign6050_e7255_d_n7, assign6050_e7255_d_n15, assign6050_e7255_d_n16,)
            }
        };
        let assign6050_e7258: f64 = (assign6050_e7256).powf(locals.var_fn61_calc_iq__beta);
        let assign6050_e7259: f64 = (1.0 + assign6050_e7258);
        let assign6050_e7262: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign6050_e7263: f64 = (assign6050_e7259).powf(assign6050_e7262);
        let assign6050_e7264: f64 = (1.0 / assign6050_e7263);
        (assign6050_e7264, (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n2)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n2 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n2)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n2 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n3)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n3 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n3)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n3 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n4)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n4 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n4)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n4 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n7)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n7 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n7)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n7 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n15)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n15 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n15)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n15 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))), (-(if 0.0 == 0.0 && ((assign6050_e7262) as f64).is_finite() && ((assign6050_e7262) as f64).fract() == 0.0 { if assign6050_e7262 == 0.0 { 0.0 } else { (assign6050_e7262 * ((assign6050_e7259).powf(assign6050_e7262 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n16)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n16 / assign6050_e7256))) })) } } else { (assign6050_e7263 * (assign6050_e7262 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6050_e7256).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6050_e7256_d_n16)) } } else { (assign6050_e7258 * (locals.var_fn61_calc_iq__beta * (assign6050_e7256_d_n16 / assign6050_e7256))) } / assign6050_e7259))) } / (assign6050_e7263 * assign6050_e7263))),)
    } else {
        (locals.var_fn61_calc_iq__fsd, locals.var_fn61_calc_iq__fsd_dn2, locals.var_fn61_calc_iq__fsd_dn3, locals.var_fn61_calc_iq__fsd_dn4, locals.var_fn61_calc_iq__fsd_dn7, locals.var_fn61_calc_iq__fsd_dn15, locals.var_fn61_calc_iq__fsd_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd = assign6050_e7266;
        locals.var_fn61_calc_iq__fsd_dn2 = assign6050_e7266_d_n2;
        locals.var_fn61_calc_iq__fsd_dn3 = assign6050_e7266_d_n3;
        locals.var_fn61_calc_iq__fsd_dn4 = assign6050_e7266_d_n4;
        locals.var_fn61_calc_iq__fsd_dn7 = assign6050_e7266_d_n7;
        locals.var_fn61_calc_iq__fsd_dn15 = assign6050_e7266_d_n15;
        locals.var_fn61_calc_iq__fsd_dn16 = assign6050_e7266_d_n16;
        locals.var_fn61_calc_iq__fsd_rv = 0.0;

        let (assign6060_e7272, assign6060_e7272_d_n2, assign6060_e7272_d_n3, assign6060_e7272_d_n4, assign6060_e7272_d_n7, assign6060_e7272_d_n15, assign6060_e7272_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6060_e7270: f64 = (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd);
        (assign6060_e7270, (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn2), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn3), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn4), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn7), ((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__fsd) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn15)), ((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__fsd) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vdx, locals.var_fn61_calc_iq__vdx_dn2, locals.var_fn61_calc_iq__vdx_dn3, locals.var_fn61_calc_iq__vdx_dn4, locals.var_fn61_calc_iq__vdx_dn7, locals.var_fn61_calc_iq__vdx_dn15, locals.var_fn61_calc_iq__vdx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx = assign6060_e7272;
        locals.var_fn61_calc_iq__vdx_dn2 = assign6060_e7272_d_n2;
        locals.var_fn61_calc_iq__vdx_dn3 = assign6060_e7272_d_n3;
        locals.var_fn61_calc_iq__vdx_dn4 = assign6060_e7272_d_n4;
        locals.var_fn61_calc_iq__vdx_dn7 = assign6060_e7272_d_n7;
        locals.var_fn61_calc_iq__vdx_dn15 = assign6060_e7272_d_n15;
        locals.var_fn61_calc_iq__vdx_dn16 = assign6060_e7272_d_n16;
        locals.var_fn61_calc_iq__vdx_rv = 0.0;

        let (assign6070_e7347, assign6070_e7347_d_n2, assign6070_e7347_d_n3, assign6070_e7347_d_n4, assign6070_e7347_d_n7, assign6070_e7347_d_n15, assign6070_e7347_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6070_e7337, assign6070_e7337_d_n2, assign6070_e7337_d_n3, assign6070_e7337_d_n4, assign6070_e7337_d_n7, assign6070_e7337_d_n15, assign6070_e7337_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6070_e7283: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6070_e7285: f64 = (assign6070_e7283 / locals.var_fn61_calc_iq__vdsat1);
                let assign6070_e7286: f64 = assign6070_e7285;
                let assign6070_e7289: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6070_e7291: f64 = (assign6070_e7289 / locals.var_fn61_calc_iq__vdsat1);
                let assign6070_e7292: f64 = (-assign6070_e7291);
                let assign6070_e7295: f64 = (0.001 / p.p53);
                let assign6070_e7298: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6070_e7300: f64 = (assign6070_e7298 / locals.var_fn61_calc_iq__vdsat1);
                let assign6070_e7301: f64 = (-assign6070_e7300);
                let assign6070_e7302: f64 = (assign6070_e7295 * assign6070_e7301);
                let assign6070_e7303: f64 = (assign6070_e7302).tanh();
                let assign6070_e7304: f64 = (assign6070_e7292 * assign6070_e7303);
                let assign6070_e7305: f64 = (assign6070_e7286 + assign6070_e7304);
                let assign6070_e7306: f64 = (0.5 * assign6070_e7305);
                (assign6070_e7306, (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * ((-((assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + (((-(-((assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-(-((assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7283 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + (((-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7289 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7303) + (assign6070_e7292 * ((assign6070_e7295 * (-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7298 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) / ((assign6070_e7302).cosh() * (assign6070_e7302).cosh())))))),)
            } else {
                let (assign6070_e7336, assign6070_e7336_d_n2, assign6070_e7336_d_n3, assign6070_e7336_d_n4, assign6070_e7336_d_n7, assign6070_e7336_d_n15, assign6070_e7336_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6070_e7313: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6070_e7315: f64 = (assign6070_e7313 / locals.var_fn61_calc_iq__vdsat1);
                        let assign6070_e7316: f64 = assign6070_e7315;
                        let assign6070_e7319: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6070_e7321: f64 = (assign6070_e7319 / locals.var_fn61_calc_iq__vdsat1);
                        let assign6070_e7322: f64 = (-assign6070_e7321);
                        let assign6070_e7325: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6070_e7327: f64 = (assign6070_e7325 / locals.var_fn61_calc_iq__vdsat1);
                        let assign6070_e7328: f64 = (-assign6070_e7327);
                        let assign6070_e7329: f64 = (assign6070_e7322 * assign6070_e7328);
                        let assign6070_e7331: f64 = (assign6070_e7329 + p.p53);
                        let assign6070_e7332: f64 = (assign6070_e7331).sqrt();
                        let assign6070_e7333: f64 = (assign6070_e7316 + assign6070_e7332);
                        let assign6070_e7334: f64 = (0.5 * assign6070_e7333);
                        (assign6070_e7334, (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn2) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn3) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn4) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * ((-((assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) + ((((-(-((assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))) * assign6070_e7328) + (assign6070_e7322 * (-(-((assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn7) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)))))) / (2.0 * assign6070_e7332)))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7328) + (assign6070_e7322 * (-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn15)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6070_e7332)))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7313 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1)) + ((((-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7319 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))) * assign6070_e7328) + (assign6070_e7322 * (-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat1) - (assign6070_e7325 * locals.var_fn61_calc_iq__vdsat1_dn16)) / (locals.var_fn61_calc_iq__vdsat1 * locals.var_fn61_calc_iq__vdsat1))))) / (2.0 * assign6070_e7332)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6070_e7336, assign6070_e7336_d_n2, assign6070_e7336_d_n3, assign6070_e7336_d_n4, assign6070_e7336_d_n7, assign6070_e7336_d_n15, assign6070_e7336_d_n16,)
            }
        };
        let assign6070_e7339: f64 = (assign6070_e7337).powf(locals.var_fn61_calc_iq__beta);
        let assign6070_e7340: f64 = (1.0 + assign6070_e7339);
        let assign6070_e7343: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign6070_e7344: f64 = (assign6070_e7340).powf(assign6070_e7343);
        let assign6070_e7345: f64 = (1.0 / assign6070_e7344);
        (assign6070_e7345, (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n2)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n2 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n2)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n2 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n3)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n3 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n3)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n3 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n4)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n4 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n4)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n4 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n7)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n7 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n7)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n7 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n15)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n15 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n15)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n15 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))), (-(if 0.0 == 0.0 && ((assign6070_e7343) as f64).is_finite() && ((assign6070_e7343) as f64).fract() == 0.0 { if assign6070_e7343 == 0.0 { 0.0 } else { (assign6070_e7343 * ((assign6070_e7340).powf(assign6070_e7343 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n16)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n16 / assign6070_e7337))) })) } } else { (assign6070_e7344 * (assign6070_e7343 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6070_e7337).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6070_e7337_d_n16)) } } else { (assign6070_e7339 * (locals.var_fn61_calc_iq__beta * (assign6070_e7337_d_n16 / assign6070_e7337))) } / assign6070_e7340))) } / (assign6070_e7344 * assign6070_e7344))),)
    } else {
        (locals.var_fn61_calc_iq__fds, locals.var_fn61_calc_iq__fds_dn2, locals.var_fn61_calc_iq__fds_dn3, locals.var_fn61_calc_iq__fds_dn4, locals.var_fn61_calc_iq__fds_dn7, locals.var_fn61_calc_iq__fds_dn15, locals.var_fn61_calc_iq__fds_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds = assign6070_e7347;
        locals.var_fn61_calc_iq__fds_dn2 = assign6070_e7347_d_n2;
        locals.var_fn61_calc_iq__fds_dn3 = assign6070_e7347_d_n3;
        locals.var_fn61_calc_iq__fds_dn4 = assign6070_e7347_d_n4;
        locals.var_fn61_calc_iq__fds_dn7 = assign6070_e7347_d_n7;
        locals.var_fn61_calc_iq__fds_dn15 = assign6070_e7347_d_n15;
        locals.var_fn61_calc_iq__fds_dn16 = assign6070_e7347_d_n16;
        locals.var_fn61_calc_iq__fds_rv = 0.0;

        let (assign6080_e7354, assign6080_e7354_d_n2, assign6080_e7354_d_n3, assign6080_e7354_d_n4, assign6080_e7354_d_n7, assign6080_e7354_d_n15, assign6080_e7354_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6080_e7350: f64 = (-locals.var_fn61_calc_iq__vdsin);
        let assign6080_e7352: f64 = (assign6080_e7350 * locals.var_fn61_calc_iq__fds);
        (assign6080_e7352, (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn2), (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn3), (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn4), (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn7), (((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__fds) + (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn15)), (((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__fds) + (assign6080_e7350 * locals.var_fn61_calc_iq__fds_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vsx, locals.var_fn61_calc_iq__vsx_dn2, locals.var_fn61_calc_iq__vsx_dn3, locals.var_fn61_calc_iq__vsx_dn4, locals.var_fn61_calc_iq__vsx_dn7, locals.var_fn61_calc_iq__vsx_dn15, locals.var_fn61_calc_iq__vsx_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx = assign6080_e7354;
        locals.var_fn61_calc_iq__vsx_dn2 = assign6080_e7354_d_n2;
        locals.var_fn61_calc_iq__vsx_dn3 = assign6080_e7354_d_n3;
        locals.var_fn61_calc_iq__vsx_dn4 = assign6080_e7354_d_n4;
        locals.var_fn61_calc_iq__vsx_dn7 = assign6080_e7354_d_n7;
        locals.var_fn61_calc_iq__vsx_dn15 = assign6080_e7354_d_n15;
        locals.var_fn61_calc_iq__vsx_dn16 = assign6080_e7354_d_n16;
        locals.var_fn61_calc_iq__vsx_rv = 0.0;

        let (assign6090_e7362, assign6090_e7362_d_n2, assign6090_e7362_d_n3, assign6090_e7362_d_n4, assign6090_e7362_d_n7, assign6090_e7362_d_n15, assign6090_e7362_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6090_e7358: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__myarg);
        let assign6090_e7360: f64 = (assign6090_e7358 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6090_e7360, ((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__myarg_dn2) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn3) / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6090_e7358 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), ((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__myarg_dn7) / locals.var_fn61_calc_iq__alpha_phit), ((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__myarg_dn15) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn16) / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign6090_e7362;
        locals.var_fn61_calc_iq__exparg_dn2 = assign6090_e7362_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign6090_e7362_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign6090_e7362_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign6090_e7362_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign6090_e7362_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign6090_e7362_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let assign6100_e7365: f64 = if locals.var_fn61_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign6100_e7365;
        locals.var_guard67_rv = 0.0;

        let (assign6110_e7371, assign6110_e7371_d_n2, assign6110_e7371_d_n3, assign6110_e7371_d_n4, assign6110_e7371_d_n7, assign6110_e7371_d_n15, assign6110_e7371_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard67 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign6110_e7371;
        locals.var_fn61_calc_iq__ffs_dn2 = assign6110_e7371_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign6110_e7371_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign6110_e7371_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign6110_e7371_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign6110_e7371_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign6110_e7371_d_n16;
        locals.var_fn61_calc_iq__ffs_rv = 0.0;

        let assign6120_e7374: f64 = (-50.0);
        let assign6120_e7375: f64 = if locals.var_fn61_calc_iq__exparg < assign6120_e7374 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign6120_e7375;
        locals.var_guard68_rv = 0.0;

        let (assign6130_e7384, assign6130_e7384_d_n2, assign6130_e7384_d_n3, assign6130_e7384_d_n4, assign6130_e7384_d_n7, assign6130_e7384_d_n15, assign6130_e7384_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard67 == 0.0)) && (locals.var_guard68 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign6130_e7384;
        locals.var_fn61_calc_iq__ffs_dn2 = assign6130_e7384_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign6130_e7384_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign6130_e7384_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign6130_e7384_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign6130_e7384_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign6130_e7384_d_n16;
        locals.var_fn61_calc_iq__ffs_rv = 0.0;

        let (assign6140_e7399, assign6140_e7399_d_n2, assign6140_e7399_d_n3, assign6140_e7399_d_n4, assign6140_e7399_d_n7, assign6140_e7399_d_n15, assign6140_e7399_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard67 == 0.0)) && (locals.var_guard68 == 0.0)) {
        let assign6140_e7395: f64 = (locals.var_fn61_calc_iq__exparg).exp();
        let assign6140_e7396: f64 = (1.0 + assign6140_e7395);
        let assign6140_e7397: f64 = (1.0 / assign6140_e7396);
        (assign6140_e7397, (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn2) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn3) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn4) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn7) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn15) / (assign6140_e7396 * assign6140_e7396))), (-((assign6140_e7395 * locals.var_fn61_calc_iq__exparg_dn16) / (assign6140_e7396 * assign6140_e7396))),)
    } else {
        (locals.var_fn61_calc_iq__ffs, locals.var_fn61_calc_iq__ffs_dn2, locals.var_fn61_calc_iq__ffs_dn3, locals.var_fn61_calc_iq__ffs_dn4, locals.var_fn61_calc_iq__ffs_dn7, locals.var_fn61_calc_iq__ffs_dn15, locals.var_fn61_calc_iq__ffs_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs = assign6140_e7399;
        locals.var_fn61_calc_iq__ffs_dn2 = assign6140_e7399_d_n2;
        locals.var_fn61_calc_iq__ffs_dn3 = assign6140_e7399_d_n3;
        locals.var_fn61_calc_iq__ffs_dn4 = assign6140_e7399_d_n4;
        locals.var_fn61_calc_iq__ffs_dn7 = assign6140_e7399_d_n7;
        locals.var_fn61_calc_iq__ffs_dn15 = assign6140_e7399_d_n15;
        locals.var_fn61_calc_iq__ffs_dn16 = assign6140_e7399_d_n16;
        locals.var_fn61_calc_iq__ffs_rv = 0.0;

        let (assign6150_e7417, assign6150_e7417_d_n2, assign6150_e7417_d_n3, assign6150_e7417_d_n4, assign6150_e7417_d_n7, assign6150_e7417_d_n15, assign6150_e7417_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6150_e7403: f64 = (locals.var_fn61_calc_iq__vgdin - locals.var_fn61_calc_iq__vsx);
        let assign6150_e7407: f64 = (p.p51 * 0.1);
        let assign6150_e7409: f64 = (assign6150_e7407 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6150_e7411: f64 = (assign6150_e7409 * locals.var_fn61_calc_iq__ffs);
        let assign6150_e7412: f64 = (locals.var_fn61_calc_iq__vtdibl - assign6150_e7411);
        let assign6150_e7413: f64 = (assign6150_e7403 - assign6150_e7412);
        let assign6150_e7415: f64 = (assign6150_e7413 / locals.var_fn61_calc_iq__two_n_phit);
        (assign6150_e7415, (((locals.var_fn61_calc_iq__vgdin_dn2 - locals.var_fn61_calc_iq__vsx_dn2) - (-(assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn2))) / locals.var_fn61_calc_iq__two_n_phit), (((-locals.var_fn61_calc_iq__vsx_dn3) - (-(assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn3))) / locals.var_fn61_calc_iq__two_n_phit), (((((-locals.var_fn61_calc_iq__vsx_dn4) - (locals.var_fn61_calc_iq__vtdibl_dn4 - (((assign6150_e7407 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ffs) + (assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn4)))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6150_e7413 * locals.var_fn61_calc_iq__two_n_phit_dn4)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), (((locals.var_fn61_calc_iq__vgdin_dn7 - locals.var_fn61_calc_iq__vsx_dn7) - (-(assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn7))) / locals.var_fn61_calc_iq__two_n_phit), (((((locals.var_fn61_calc_iq__vgdin_dn15 - locals.var_fn61_calc_iq__vsx_dn15) - (locals.var_fn61_calc_iq__vtdibl_dn15 - (assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn15))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6150_e7413 * locals.var_fn61_calc_iq__two_n_phit_dn15)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), (((((locals.var_fn61_calc_iq__vgdin_dn16 - locals.var_fn61_calc_iq__vsx_dn16) - (locals.var_fn61_calc_iq__vtdibl_dn16 - (assign6150_e7409 * locals.var_fn61_calc_iq__ffs_dn16))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6150_e7413 * locals.var_fn61_calc_iq__two_n_phit_dn16)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn61_calc_iq__etas, locals.var_fn61_calc_iq__etas_dn2, locals.var_fn61_calc_iq__etas_dn3, locals.var_fn61_calc_iq__etas_dn4, locals.var_fn61_calc_iq__etas_dn7, locals.var_fn61_calc_iq__etas_dn15, locals.var_fn61_calc_iq__etas_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas = assign6150_e7417;
        locals.var_fn61_calc_iq__etas_dn2 = assign6150_e7417_d_n2;
        locals.var_fn61_calc_iq__etas_dn3 = assign6150_e7417_d_n3;
        locals.var_fn61_calc_iq__etas_dn4 = assign6150_e7417_d_n4;
        locals.var_fn61_calc_iq__etas_dn7 = assign6150_e7417_d_n7;
        locals.var_fn61_calc_iq__etas_dn15 = assign6150_e7417_d_n15;
        locals.var_fn61_calc_iq__etas_dn16 = assign6150_e7417_d_n16;
        locals.var_fn61_calc_iq__etas_rv = 0.0;

        let assign6160_e7420: f64 = if locals.var_fn61_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign6160_e7420;
        locals.var_guard69_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6170_e7428, assign6170_e7428_d_n2, assign6170_e7428_d_n3, assign6170_e7428_d_n4, assign6170_e7428_d_n7, assign6170_e7428_d_n15, assign6170_e7428_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign6170_e7426: f64 = (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas);
        (assign6170_e7426, (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn2), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn3), ((locals.var_fn61_calc_iq__qref_dn4 * locals.var_fn61_calc_iq__etas) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn4)), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn7), ((locals.var_fn61_calc_iq__qref_dn15 * locals.var_fn61_calc_iq__etas) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn15)), ((locals.var_fn61_calc_iq__qref_dn16 * locals.var_fn61_calc_iq__etas) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etas_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign6170_e7428;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign6170_e7428_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign6170_e7428_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign6170_e7428_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign6170_e7428_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign6170_e7428_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign6170_e7428_d_n16;
        locals.var_fn61_calc_iq__qinvs_rv = 0.0;

        let assign6180_e7431: f64 = (-50.0);
        let assign6180_e7432: f64 = if locals.var_fn61_calc_iq__etas < assign6180_e7431 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign6180_e7432;
        locals.var_guard70_rv = 0.0;

        let (assign6190_e7444, assign6190_e7444_d_n2, assign6190_e7444_d_n3, assign6190_e7444_d_n4, assign6190_e7444_d_n7, assign6190_e7444_d_n15, assign6190_e7444_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard69 == 0.0)) && (locals.var_guard70 != 0.0)) {
        let assign6190_e7441: f64 = (locals.var_fn61_calc_iq__etas).exp();
        let assign6190_e7442: f64 = (locals.var_fn61_calc_iq__qref * assign6190_e7441);
        (assign6190_e7442, (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn2)), (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn3)), ((locals.var_fn61_calc_iq__qref_dn4 * assign6190_e7441) + (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn4))), (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn7)), ((locals.var_fn61_calc_iq__qref_dn15 * assign6190_e7441) + (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn15))), ((locals.var_fn61_calc_iq__qref_dn16 * assign6190_e7441) + (locals.var_fn61_calc_iq__qref * (assign6190_e7441 * locals.var_fn61_calc_iq__etas_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign6190_e7444;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign6190_e7444_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign6190_e7444_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign6190_e7444_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign6190_e7444_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign6190_e7444_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign6190_e7444_d_n16;
        locals.var_fn61_calc_iq__qinvs_rv = 0.0;

        let (assign6200_e7460, assign6200_e7460_d_n2, assign6200_e7460_d_n3, assign6200_e7460_d_n4, assign6200_e7460_d_n7, assign6200_e7460_d_n15, assign6200_e7460_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard69 == 0.0)) && (locals.var_guard70 == 0.0)) {
        let assign6200_e7455: f64 = (locals.var_fn61_calc_iq__etas).exp();
        let assign6200_e7456: f64 = (1.0 + assign6200_e7455);
        let assign6200_e7457: f64 = (assign6200_e7456).ln();
        let assign6200_e7458: f64 = (locals.var_fn61_calc_iq__qref * assign6200_e7457);
        (assign6200_e7458, (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn2) / assign6200_e7456)), (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn3) / assign6200_e7456)), ((locals.var_fn61_calc_iq__qref_dn4 * assign6200_e7457) + (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn4) / assign6200_e7456))), (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn7) / assign6200_e7456)), ((locals.var_fn61_calc_iq__qref_dn15 * assign6200_e7457) + (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn15) / assign6200_e7456))), ((locals.var_fn61_calc_iq__qref_dn16 * assign6200_e7457) + (locals.var_fn61_calc_iq__qref * ((assign6200_e7455 * locals.var_fn61_calc_iq__etas_dn16) / assign6200_e7456))),)
    } else {
        (locals.var_fn61_calc_iq__qinvs, locals.var_fn61_calc_iq__qinvs_dn2, locals.var_fn61_calc_iq__qinvs_dn3, locals.var_fn61_calc_iq__qinvs_dn4, locals.var_fn61_calc_iq__qinvs_dn7, locals.var_fn61_calc_iq__qinvs_dn15, locals.var_fn61_calc_iq__qinvs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs = assign6200_e7460;
        locals.var_fn61_calc_iq__qinvs_dn2 = assign6200_e7460_d_n2;
        locals.var_fn61_calc_iq__qinvs_dn3 = assign6200_e7460_d_n3;
        locals.var_fn61_calc_iq__qinvs_dn4 = assign6200_e7460_d_n4;
        locals.var_fn61_calc_iq__qinvs_dn7 = assign6200_e7460_d_n7;
        locals.var_fn61_calc_iq__qinvs_dn15 = assign6200_e7460_d_n15;
        locals.var_fn61_calc_iq__qinvs_dn16 = assign6200_e7460_d_n16;
        locals.var_fn61_calc_iq__qinvs_rv = 0.0;

        let (assign6210_e7468, assign6210_e7468_d_n2, assign6210_e7468_d_n3, assign6210_e7468_d_n4, assign6210_e7468_d_n7, assign6210_e7468_d_n15, assign6210_e7468_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6210_e7464: f64 = (locals.var_fn61_calc_iq__vgdin - locals.var_fn61_calc_iq__myarg);
        let assign6210_e7466: f64 = (assign6210_e7464 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6210_e7466, ((locals.var_fn61_calc_iq__vgdin_dn2 - locals.var_fn61_calc_iq__myarg_dn2) / locals.var_fn61_calc_iq__alpha_phit), ((-locals.var_fn61_calc_iq__myarg_dn3) / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6210_e7464 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), ((locals.var_fn61_calc_iq__vgdin_dn7 - locals.var_fn61_calc_iq__myarg_dn7) / locals.var_fn61_calc_iq__alpha_phit), ((locals.var_fn61_calc_iq__vgdin_dn15 - locals.var_fn61_calc_iq__myarg_dn15) / locals.var_fn61_calc_iq__alpha_phit), ((locals.var_fn61_calc_iq__vgdin_dn16 - locals.var_fn61_calc_iq__myarg_dn16) / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign6210_e7468;
        locals.var_fn61_calc_iq__exparg_dn2 = assign6210_e7468_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign6210_e7468_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign6210_e7468_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign6210_e7468_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign6210_e7468_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign6210_e7468_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let assign6220_e7471: f64 = if locals.var_fn61_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign6220_e7471;
        locals.var_guard71_rv = 0.0;

        let (assign6230_e7477, assign6230_e7477_d_n2, assign6230_e7477_d_n3, assign6230_e7477_d_n4, assign6230_e7477_d_n7, assign6230_e7477_d_n15, assign6230_e7477_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard71 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign6230_e7477;
        locals.var_fn61_calc_iq__ffd_dn2 = assign6230_e7477_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign6230_e7477_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign6230_e7477_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign6230_e7477_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign6230_e7477_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign6230_e7477_d_n16;
        locals.var_fn61_calc_iq__ffd_rv = 0.0;

        let assign6240_e7480: f64 = (-50.0);
        let assign6240_e7481: f64 = if locals.var_fn61_calc_iq__exparg < assign6240_e7480 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign6240_e7481;
        locals.var_guard72_rv = 0.0;

        let (assign6250_e7490, assign6250_e7490_d_n2, assign6250_e7490_d_n3, assign6250_e7490_d_n4, assign6250_e7490_d_n7, assign6250_e7490_d_n15, assign6250_e7490_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard71 == 0.0)) && (locals.var_guard72 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign6250_e7490;
        locals.var_fn61_calc_iq__ffd_dn2 = assign6250_e7490_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign6250_e7490_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign6250_e7490_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign6250_e7490_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign6250_e7490_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign6250_e7490_d_n16;
        locals.var_fn61_calc_iq__ffd_rv = 0.0;

        let (assign6260_e7505, assign6260_e7505_d_n2, assign6260_e7505_d_n3, assign6260_e7505_d_n4, assign6260_e7505_d_n7, assign6260_e7505_d_n15, assign6260_e7505_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard71 == 0.0)) && (locals.var_guard72 == 0.0)) {
        let assign6260_e7501: f64 = (locals.var_fn61_calc_iq__exparg).exp();
        let assign6260_e7502: f64 = (1.0 + assign6260_e7501);
        let assign6260_e7503: f64 = (1.0 / assign6260_e7502);
        (assign6260_e7503, (-((assign6260_e7501 * locals.var_fn61_calc_iq__exparg_dn2) / (assign6260_e7502 * assign6260_e7502))), (-((assign6260_e7501 * locals.var_fn61_calc_iq__exparg_dn3) / (assign6260_e7502 * assign6260_e7502))), (-((assign6260_e7501 * locals.var_fn61_calc_iq__exparg_dn4) / (assign6260_e7502 * assign6260_e7502))), (-((assign6260_e7501 * locals.var_fn61_calc_iq__exparg_dn7) / (assign6260_e7502 * assign6260_e7502))), (-((assign6260_e7501 * locals.var_fn61_calc_iq__exparg_dn15) / (assign6260_e7502 * assign6260_e7502))), (-((assign6260_e7501 * locals.var_fn61_calc_iq__exparg_dn16) / (assign6260_e7502 * assign6260_e7502))),)
    } else {
        (locals.var_fn61_calc_iq__ffd, locals.var_fn61_calc_iq__ffd_dn2, locals.var_fn61_calc_iq__ffd_dn3, locals.var_fn61_calc_iq__ffd_dn4, locals.var_fn61_calc_iq__ffd_dn7, locals.var_fn61_calc_iq__ffd_dn15, locals.var_fn61_calc_iq__ffd_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd = assign6260_e7505;
        locals.var_fn61_calc_iq__ffd_dn2 = assign6260_e7505_d_n2;
        locals.var_fn61_calc_iq__ffd_dn3 = assign6260_e7505_d_n3;
        locals.var_fn61_calc_iq__ffd_dn4 = assign6260_e7505_d_n4;
        locals.var_fn61_calc_iq__ffd_dn7 = assign6260_e7505_d_n7;
        locals.var_fn61_calc_iq__ffd_dn15 = assign6260_e7505_d_n15;
        locals.var_fn61_calc_iq__ffd_dn16 = assign6260_e7505_d_n16;
        locals.var_fn61_calc_iq__ffd_rv = 0.0;

        let (assign6270_e7523, assign6270_e7523_d_n2, assign6270_e7523_d_n3, assign6270_e7523_d_n4, assign6270_e7523_d_n7, assign6270_e7523_d_n15, assign6270_e7523_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6270_e7509: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vdx);
        let assign6270_e7513: f64 = (p.p51 * 0.1);
        let assign6270_e7515: f64 = (assign6270_e7513 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6270_e7517: f64 = (assign6270_e7515 * locals.var_fn61_calc_iq__ffd);
        let assign6270_e7518: f64 = (locals.var_fn61_calc_iq__vtdibl - assign6270_e7517);
        let assign6270_e7519: f64 = (assign6270_e7509 - assign6270_e7518);
        let assign6270_e7521: f64 = (assign6270_e7519 / locals.var_fn61_calc_iq__two_n_phit);
        (assign6270_e7521, (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vdx_dn2) - (-(assign6270_e7515 * locals.var_fn61_calc_iq__ffd_dn2))) / locals.var_fn61_calc_iq__two_n_phit), (((-locals.var_fn61_calc_iq__vdx_dn3) - (-(assign6270_e7515 * locals.var_fn61_calc_iq__ffd_dn3))) / locals.var_fn61_calc_iq__two_n_phit), (((((-locals.var_fn61_calc_iq__vdx_dn4) - (locals.var_fn61_calc_iq__vtdibl_dn4 - (((assign6270_e7513 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ffd) + (assign6270_e7515 * locals.var_fn61_calc_iq__ffd_dn4)))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6270_e7519 * locals.var_fn61_calc_iq__two_n_phit_dn4)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vdx_dn7) - (-(assign6270_e7515 * locals.var_fn61_calc_iq__ffd_dn7))) / locals.var_fn61_calc_iq__two_n_phit), (((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vdx_dn15) - (locals.var_fn61_calc_iq__vtdibl_dn15 - (assign6270_e7515 * locals.var_fn61_calc_iq__ffd_dn15))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6270_e7519 * locals.var_fn61_calc_iq__two_n_phit_dn15)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)), (((((-locals.var_fn61_calc_iq__vdx_dn16) - (locals.var_fn61_calc_iq__vtdibl_dn16 - (assign6270_e7515 * locals.var_fn61_calc_iq__ffd_dn16))) * locals.var_fn61_calc_iq__two_n_phit) - (assign6270_e7519 * locals.var_fn61_calc_iq__two_n_phit_dn16)) / (locals.var_fn61_calc_iq__two_n_phit * locals.var_fn61_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn61_calc_iq__etad, locals.var_fn61_calc_iq__etad_dn2, locals.var_fn61_calc_iq__etad_dn3, locals.var_fn61_calc_iq__etad_dn4, locals.var_fn61_calc_iq__etad_dn7, locals.var_fn61_calc_iq__etad_dn15, locals.var_fn61_calc_iq__etad_dn16,)
    }
};
        locals.var_fn61_calc_iq__etad = assign6270_e7523;
        locals.var_fn61_calc_iq__etad_dn2 = assign6270_e7523_d_n2;
        locals.var_fn61_calc_iq__etad_dn3 = assign6270_e7523_d_n3;
        locals.var_fn61_calc_iq__etad_dn4 = assign6270_e7523_d_n4;
        locals.var_fn61_calc_iq__etad_dn7 = assign6270_e7523_d_n7;
        locals.var_fn61_calc_iq__etad_dn15 = assign6270_e7523_d_n15;
        locals.var_fn61_calc_iq__etad_dn16 = assign6270_e7523_d_n16;
        locals.var_fn61_calc_iq__etad_rv = 0.0;

        let assign6280_e7526: f64 = if locals.var_fn61_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign6280_e7526;
        locals.var_guard73_rv = 0.0;

        let (assign6290_e7534, assign6290_e7534_d_n2, assign6290_e7534_d_n3, assign6290_e7534_d_n4, assign6290_e7534_d_n7, assign6290_e7534_d_n15, assign6290_e7534_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign6290_e7532: f64 = (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad);
        (assign6290_e7532, (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad_dn2), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad_dn3), ((locals.var_fn61_calc_iq__qref_dn4 * locals.var_fn61_calc_iq__etad) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad_dn4)), (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad_dn7), ((locals.var_fn61_calc_iq__qref_dn15 * locals.var_fn61_calc_iq__etad) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad_dn15)), ((locals.var_fn61_calc_iq__qref_dn16 * locals.var_fn61_calc_iq__etad) + (locals.var_fn61_calc_iq__qref * locals.var_fn61_calc_iq__etad_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvd, locals.var_fn61_calc_iq__qinvd_dn2, locals.var_fn61_calc_iq__qinvd_dn3, locals.var_fn61_calc_iq__qinvd_dn4, locals.var_fn61_calc_iq__qinvd_dn7, locals.var_fn61_calc_iq__qinvd_dn15, locals.var_fn61_calc_iq__qinvd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd = assign6290_e7534;
        locals.var_fn61_calc_iq__qinvd_dn2 = assign6290_e7534_d_n2;
        locals.var_fn61_calc_iq__qinvd_dn3 = assign6290_e7534_d_n3;
        locals.var_fn61_calc_iq__qinvd_dn4 = assign6290_e7534_d_n4;
        locals.var_fn61_calc_iq__qinvd_dn7 = assign6290_e7534_d_n7;
        locals.var_fn61_calc_iq__qinvd_dn15 = assign6290_e7534_d_n15;
        locals.var_fn61_calc_iq__qinvd_dn16 = assign6290_e7534_d_n16;
        locals.var_fn61_calc_iq__qinvd_rv = 0.0;

        let assign6300_e7537: f64 = (-50.0);
        let assign6300_e7538: f64 = if locals.var_fn61_calc_iq__etad < assign6300_e7537 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign6300_e7538;
        locals.var_guard74_rv = 0.0;

        let (assign6310_e7550, assign6310_e7550_d_n2, assign6310_e7550_d_n3, assign6310_e7550_d_n4, assign6310_e7550_d_n7, assign6310_e7550_d_n15, assign6310_e7550_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard73 == 0.0)) && (locals.var_guard74 != 0.0)) {
        let assign6310_e7547: f64 = (locals.var_fn61_calc_iq__etad).exp();
        let assign6310_e7548: f64 = (locals.var_fn61_calc_iq__qref * assign6310_e7547);
        (assign6310_e7548, (locals.var_fn61_calc_iq__qref * (assign6310_e7547 * locals.var_fn61_calc_iq__etad_dn2)), (locals.var_fn61_calc_iq__qref * (assign6310_e7547 * locals.var_fn61_calc_iq__etad_dn3)), ((locals.var_fn61_calc_iq__qref_dn4 * assign6310_e7547) + (locals.var_fn61_calc_iq__qref * (assign6310_e7547 * locals.var_fn61_calc_iq__etad_dn4))), (locals.var_fn61_calc_iq__qref * (assign6310_e7547 * locals.var_fn61_calc_iq__etad_dn7)), ((locals.var_fn61_calc_iq__qref_dn15 * assign6310_e7547) + (locals.var_fn61_calc_iq__qref * (assign6310_e7547 * locals.var_fn61_calc_iq__etad_dn15))), ((locals.var_fn61_calc_iq__qref_dn16 * assign6310_e7547) + (locals.var_fn61_calc_iq__qref * (assign6310_e7547 * locals.var_fn61_calc_iq__etad_dn16))),)
    } else {
        (locals.var_fn61_calc_iq__qinvd, locals.var_fn61_calc_iq__qinvd_dn2, locals.var_fn61_calc_iq__qinvd_dn3, locals.var_fn61_calc_iq__qinvd_dn4, locals.var_fn61_calc_iq__qinvd_dn7, locals.var_fn61_calc_iq__qinvd_dn15, locals.var_fn61_calc_iq__qinvd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd = assign6310_e7550;
        locals.var_fn61_calc_iq__qinvd_dn2 = assign6310_e7550_d_n2;
        locals.var_fn61_calc_iq__qinvd_dn3 = assign6310_e7550_d_n3;
        locals.var_fn61_calc_iq__qinvd_dn4 = assign6310_e7550_d_n4;
        locals.var_fn61_calc_iq__qinvd_dn7 = assign6310_e7550_d_n7;
        locals.var_fn61_calc_iq__qinvd_dn15 = assign6310_e7550_d_n15;
        locals.var_fn61_calc_iq__qinvd_dn16 = assign6310_e7550_d_n16;
        locals.var_fn61_calc_iq__qinvd_rv = 0.0;

        let (assign6320_e7566, assign6320_e7566_d_n2, assign6320_e7566_d_n3, assign6320_e7566_d_n4, assign6320_e7566_d_n7, assign6320_e7566_d_n15, assign6320_e7566_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard73 == 0.0)) && (locals.var_guard74 == 0.0)) {
        let assign6320_e7561: f64 = (locals.var_fn61_calc_iq__etad).exp();
        let assign6320_e7562: f64 = (1.0 + assign6320_e7561);
        let assign6320_e7563: f64 = (assign6320_e7562).ln();
        let assign6320_e7564: f64 = (locals.var_fn61_calc_iq__qref * assign6320_e7563);
        (assign6320_e7564, (locals.var_fn61_calc_iq__qref * ((assign6320_e7561 * locals.var_fn61_calc_iq__etad_dn2) / assign6320_e7562)), (locals.var_fn61_calc_iq__qref * ((assign6320_e7561 * locals.var_fn61_calc_iq__etad_dn3) / assign6320_e7562)), ((locals.var_fn61_calc_iq__qref_dn4 * assign6320_e7563) + (locals.var_fn61_calc_iq__qref * ((assign6320_e7561 * locals.var_fn61_calc_iq__etad_dn4) / assign6320_e7562))), (locals.var_fn61_calc_iq__qref * ((assign6320_e7561 * locals.var_fn61_calc_iq__etad_dn7) / assign6320_e7562)), ((locals.var_fn61_calc_iq__qref_dn15 * assign6320_e7563) + (locals.var_fn61_calc_iq__qref * ((assign6320_e7561 * locals.var_fn61_calc_iq__etad_dn15) / assign6320_e7562))), ((locals.var_fn61_calc_iq__qref_dn16 * assign6320_e7563) + (locals.var_fn61_calc_iq__qref * ((assign6320_e7561 * locals.var_fn61_calc_iq__etad_dn16) / assign6320_e7562))),)
    } else {
        (locals.var_fn61_calc_iq__qinvd, locals.var_fn61_calc_iq__qinvd_dn2, locals.var_fn61_calc_iq__qinvd_dn3, locals.var_fn61_calc_iq__qinvd_dn4, locals.var_fn61_calc_iq__qinvd_dn7, locals.var_fn61_calc_iq__qinvd_dn15, locals.var_fn61_calc_iq__qinvd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd = assign6320_e7566;
        locals.var_fn61_calc_iq__qinvd_dn2 = assign6320_e7566_d_n2;
        locals.var_fn61_calc_iq__qinvd_dn3 = assign6320_e7566_d_n3;
        locals.var_fn61_calc_iq__qinvd_dn4 = assign6320_e7566_d_n4;
        locals.var_fn61_calc_iq__qinvd_dn7 = assign6320_e7566_d_n7;
        locals.var_fn61_calc_iq__qinvd_dn15 = assign6320_e7566_d_n15;
        locals.var_fn61_calc_iq__qinvd_dn16 = assign6320_e7566_d_n16;
        locals.var_fn61_calc_iq__qinvd_rv = 0.0;

        let (assign6330_e7574, assign6330_e7574_d_n2, assign6330_e7574_d_n3, assign6330_e7574_d_n4, assign6330_e7574_d_n7, assign6330_e7574_d_n15, assign6330_e7574_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6330_e7570: f64 = (locals.var_fn61_calc_iq__qinvs - locals.var_fn61_calc_iq__qinvd);
        let assign6330_e7572: f64 = (assign6330_e7570 / locals.var_fn61_calc_iq__cgin);
        (assign6330_e7572, ((locals.var_fn61_calc_iq__qinvs_dn2 - locals.var_fn61_calc_iq__qinvd_dn2) / locals.var_fn61_calc_iq__cgin), ((locals.var_fn61_calc_iq__qinvs_dn3 - locals.var_fn61_calc_iq__qinvd_dn3) / locals.var_fn61_calc_iq__cgin), ((((locals.var_fn61_calc_iq__qinvs_dn4 - locals.var_fn61_calc_iq__qinvd_dn4) * locals.var_fn61_calc_iq__cgin) - (assign6330_e7570 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin)), ((locals.var_fn61_calc_iq__qinvs_dn7 - locals.var_fn61_calc_iq__qinvd_dn7) / locals.var_fn61_calc_iq__cgin), ((locals.var_fn61_calc_iq__qinvs_dn15 - locals.var_fn61_calc_iq__qinvd_dn15) / locals.var_fn61_calc_iq__cgin), ((locals.var_fn61_calc_iq__qinvs_dn16 - locals.var_fn61_calc_iq__qinvd_dn16) / locals.var_fn61_calc_iq__cgin),)
    } else {
        (locals.var_fn61_calc_iq__vdsc, locals.var_fn61_calc_iq__vdsc_dn2, locals.var_fn61_calc_iq__vdsc_dn3, locals.var_fn61_calc_iq__vdsc_dn4, locals.var_fn61_calc_iq__vdsc_dn7, locals.var_fn61_calc_iq__vdsc_dn15, locals.var_fn61_calc_iq__vdsc_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsc = assign6330_e7574;
        locals.var_fn61_calc_iq__vdsc_dn2 = assign6330_e7574_d_n2;
        locals.var_fn61_calc_iq__vdsc_dn3 = assign6330_e7574_d_n3;
        locals.var_fn61_calc_iq__vdsc_dn4 = assign6330_e7574_d_n4;
        locals.var_fn61_calc_iq__vdsc_dn7 = assign6330_e7574_d_n7;
        locals.var_fn61_calc_iq__vdsc_dn15 = assign6330_e7574_d_n15;
        locals.var_fn61_calc_iq__vdsc_dn16 = assign6330_e7574_d_n16;
        locals.var_fn61_calc_iq__vdsc_rv = 0.0;

        let (assign6340_e7580, assign6340_e7580_d_n2, assign6340_e7580_d_n3, assign6340_e7580_d_n4, assign6340_e7580_d_n7, assign6340_e7580_d_n15, assign6340_e7580_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6340_e7578: f64 = (locals.var_fn61_calc_iq__vdsc / locals.var_fn61_calc_iq__vdsat);
        (assign6340_e7578, (((locals.var_fn61_calc_iq__vdsc_dn2 * locals.var_fn61_calc_iq__vdsat) - (locals.var_fn61_calc_iq__vdsc * locals.var_fn61_calc_iq__vdsat_dn2)) / (locals.var_fn61_calc_iq__vdsat * locals.var_fn61_calc_iq__vdsat)), (((locals.var_fn61_calc_iq__vdsc_dn3 * locals.var_fn61_calc_iq__vdsat) - (locals.var_fn61_calc_iq__vdsc * locals.var_fn61_calc_iq__vdsat_dn3)) / (locals.var_fn61_calc_iq__vdsat * locals.var_fn61_calc_iq__vdsat)), (((locals.var_fn61_calc_iq__vdsc_dn4 * locals.var_fn61_calc_iq__vdsat) - (locals.var_fn61_calc_iq__vdsc * locals.var_fn61_calc_iq__vdsat_dn4)) / (locals.var_fn61_calc_iq__vdsat * locals.var_fn61_calc_iq__vdsat)), (((locals.var_fn61_calc_iq__vdsc_dn7 * locals.var_fn61_calc_iq__vdsat) - (locals.var_fn61_calc_iq__vdsc * locals.var_fn61_calc_iq__vdsat_dn7)) / (locals.var_fn61_calc_iq__vdsat * locals.var_fn61_calc_iq__vdsat)), (((locals.var_fn61_calc_iq__vdsc_dn15 * locals.var_fn61_calc_iq__vdsat) - (locals.var_fn61_calc_iq__vdsc * locals.var_fn61_calc_iq__vdsat_dn15)) / (locals.var_fn61_calc_iq__vdsat * locals.var_fn61_calc_iq__vdsat)), (((locals.var_fn61_calc_iq__vdsc_dn16 * locals.var_fn61_calc_iq__vdsat) - (locals.var_fn61_calc_iq__vdsc * locals.var_fn61_calc_iq__vdsat_dn16)) / (locals.var_fn61_calc_iq__vdsat * locals.var_fn61_calc_iq__vdsat)),)
    } else {
        (locals.var_fn61_calc_iq__myarg, locals.var_fn61_calc_iq__myarg_dn2, locals.var_fn61_calc_iq__myarg_dn3, locals.var_fn61_calc_iq__myarg_dn4, locals.var_fn61_calc_iq__myarg_dn7, locals.var_fn61_calc_iq__myarg_dn15, locals.var_fn61_calc_iq__myarg_dn16,)
    }
};
        locals.var_fn61_calc_iq__myarg = assign6340_e7580;
        locals.var_fn61_calc_iq__myarg_dn2 = assign6340_e7580_d_n2;
        locals.var_fn61_calc_iq__myarg_dn3 = assign6340_e7580_d_n3;
        locals.var_fn61_calc_iq__myarg_dn4 = assign6340_e7580_d_n4;
        locals.var_fn61_calc_iq__myarg_dn7 = assign6340_e7580_d_n7;
        locals.var_fn61_calc_iq__myarg_dn15 = assign6340_e7580_d_n15;
        locals.var_fn61_calc_iq__myarg_dn16 = assign6340_e7580_d_n16;
        locals.var_fn61_calc_iq__myarg_rv = 0.0;

        let (assign6380_e7649, assign6380_e7649_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6380_e7646: f64 = (2.302585092994046 * locals.var_fn61_calc_iq__phitin);
        let assign6380_e7647: f64 = (locals.var_fn61_calc_iq__ss / assign6380_e7646);
        (assign6380_e7647, (-((locals.var_fn61_calc_iq__ss * (2.302585092994046 * locals.var_fn61_calc_iq__phitin_dn4)) / (assign6380_e7646 * assign6380_e7646))),)
    } else {
        (locals.var_fn61_calc_iq__n0, locals.var_fn61_calc_iq__n0_dn4,)
    }
};
        locals.var_fn61_calc_iq__n0 = assign6380_e7649;
        locals.var_fn61_calc_iq__n0_dn4 = assign6380_e7649_d_n4;
        locals.var_fn61_calc_iq__n0_rv = 0.0;

        let (assign6390_e7657, assign6390_e7657_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6390_e7653: f64 = (2.0 * locals.var_fn61_calc_iq__n0);
        let assign6390_e7655: f64 = (assign6390_e7653 * locals.var_fn61_calc_iq__phitin);
        (assign6390_e7655, (((2.0 * locals.var_fn61_calc_iq__n0_dn4) * locals.var_fn61_calc_iq__phitin) + (assign6390_e7653 * locals.var_fn61_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn61_calc_iq__two_n_phit0, locals.var_fn61_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn61_calc_iq__two_n_phit0 = assign6390_e7657;
        locals.var_fn61_calc_iq__two_n_phit0_dn4 = assign6390_e7657_d_n4;
        locals.var_fn61_calc_iq__two_n_phit0_rv = 0.0;

        let (assign6400_e7663, assign6400_e7663_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6400_e7661: f64 = (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit0);
        (assign6400_e7661, ((locals.var_fn61_calc_iq__cgin_dn4 * locals.var_fn61_calc_iq__two_n_phit0) + (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn61_calc_iq__qref0, locals.var_fn61_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn61_calc_iq__qref0 = assign6400_e7663;
        locals.var_fn61_calc_iq__qref0_dn4 = assign6400_e7663_d_n4;
        locals.var_fn61_calc_iq__qref0_rv = 0.0;

        let (assign6410_e7673, assign6410_e7673_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6410_e7668: f64 = (p.p51 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6410_e7670: f64 = (assign6410_e7668 / 2.0);
        let assign6410_e7671: f64 = (locals.var_fn61_calc_iq__vtof - assign6410_e7670);
        (assign6410_e7671, (locals.var_fn61_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn61_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn61_calc_iq__myarg0, locals.var_fn61_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn61_calc_iq__myarg0 = assign6410_e7673;
        locals.var_fn61_calc_iq__myarg0_dn4 = assign6410_e7673_d_n4;
        locals.var_fn61_calc_iq__myarg0_rv = 0.0;

        let (assign6420_e7724, assign6420_e7724_d_n2, assign6420_e7724_d_n4, assign6420_e7724_d_n7, assign6420_e7724_d_n15, assign6420_e7724_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6420_e7718, assign6420_e7718_d_n2, assign6420_e7718_d_n7, assign6420_e7718_d_n15, assign6420_e7718_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6420_e7682: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                let assign6420_e7685: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign6420_e7688: f64 = (0.001 / p.p53);
                let assign6420_e7691: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign6420_e7692: f64 = (assign6420_e7688 * assign6420_e7691);
                let assign6420_e7693: f64 = (assign6420_e7692).tanh();
                let assign6420_e7694: f64 = (assign6420_e7685 * assign6420_e7693);
                let assign6420_e7695: f64 = (assign6420_e7682 + assign6420_e7694);
                let assign6420_e7696: f64 = (0.5 * assign6420_e7695);
                (assign6420_e7696, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign6420_e7693) + (assign6420_e7685 * ((assign6420_e7688 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2)) / ((assign6420_e7692).cosh() * (assign6420_e7692).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign6420_e7693) + (assign6420_e7685 * ((assign6420_e7688 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7)) / ((assign6420_e7692).cosh() * (assign6420_e7692).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign6420_e7693) + (assign6420_e7685 * ((assign6420_e7688 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15)) / ((assign6420_e7692).cosh() * (assign6420_e7692).cosh())))))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + (((-locals.var_fn61_calc_iq__vgdin_dn16) * assign6420_e7693) + (assign6420_e7685 * ((assign6420_e7688 * (-locals.var_fn61_calc_iq__vgdin_dn16)) / ((assign6420_e7692).cosh() * (assign6420_e7692).cosh())))))),)
            } else {
                let (assign6420_e7717, assign6420_e7717_d_n2, assign6420_e7717_d_n7, assign6420_e7717_d_n15, assign6420_e7717_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6420_e7703: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                        let assign6420_e7706: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign6420_e7709: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign6420_e7710: f64 = (assign6420_e7706 * assign6420_e7709);
                        let assign6420_e7712: f64 = (assign6420_e7710 + p.p53);
                        let assign6420_e7713: f64 = (assign6420_e7712).sqrt();
                        let assign6420_e7714: f64 = (assign6420_e7703 + assign6420_e7713);
                        let assign6420_e7715: f64 = (0.5 * assign6420_e7714);
                        (assign6420_e7715, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + ((((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign6420_e7709) + (assign6420_e7706 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2))) / (2.0 * assign6420_e7713)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + ((((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign6420_e7709) + (assign6420_e7706 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7))) / (2.0 * assign6420_e7713)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + ((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign6420_e7709) + (assign6420_e7706 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15))) / (2.0 * assign6420_e7713)))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + ((((-locals.var_fn61_calc_iq__vgdin_dn16) * assign6420_e7709) + (assign6420_e7706 * (-locals.var_fn61_calc_iq__vgdin_dn16))) / (2.0 * assign6420_e7713)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6420_e7717, assign6420_e7717_d_n2, assign6420_e7717_d_n7, assign6420_e7717_d_n15, assign6420_e7717_d_n16,)
            }
        };
        let assign6420_e7720: f64 = (assign6420_e7718 - locals.var_fn61_calc_iq__myarg0);
        let assign6420_e7722: f64 = (assign6420_e7720 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6420_e7722, (assign6420_e7718_d_n2 / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg0_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6420_e7720 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), (assign6420_e7718_d_n7 / locals.var_fn61_calc_iq__alpha_phit), (assign6420_e7718_d_n15 / locals.var_fn61_calc_iq__alpha_phit), (assign6420_e7718_d_n16 / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg0, locals.var_fn61_calc_iq__exparg0_dn2, locals.var_fn61_calc_iq__exparg0_dn4, locals.var_fn61_calc_iq__exparg0_dn7, locals.var_fn61_calc_iq__exparg0_dn15, locals.var_fn61_calc_iq__exparg0_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg0 = assign6420_e7724;
        locals.var_fn61_calc_iq__exparg0_dn2 = assign6420_e7724_d_n2;
        locals.var_fn61_calc_iq__exparg0_dn4 = assign6420_e7724_d_n4;
        locals.var_fn61_calc_iq__exparg0_dn7 = assign6420_e7724_d_n7;
        locals.var_fn61_calc_iq__exparg0_dn15 = assign6420_e7724_d_n15;
        locals.var_fn61_calc_iq__exparg0_dn16 = assign6420_e7724_d_n16;
        locals.var_fn61_calc_iq__exparg0_rv = 0.0;

        let assign6430_e7727: f64 = if locals.var_fn61_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign6430_e7727;
        locals.var_guard75_rv = 0.0;

        let (assign6440_e7733, assign6440_e7733_d_n2, assign6440_e7733_d_n4, assign6440_e7733_d_n7, assign6440_e7733_d_n15, assign6440_e7733_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard75 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff0, locals.var_fn61_calc_iq__ff0_dn2, locals.var_fn61_calc_iq__ff0_dn4, locals.var_fn61_calc_iq__ff0_dn7, locals.var_fn61_calc_iq__ff0_dn15, locals.var_fn61_calc_iq__ff0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff0 = assign6440_e7733;
        locals.var_fn61_calc_iq__ff0_dn2 = assign6440_e7733_d_n2;
        locals.var_fn61_calc_iq__ff0_dn4 = assign6440_e7733_d_n4;
        locals.var_fn61_calc_iq__ff0_dn7 = assign6440_e7733_d_n7;
        locals.var_fn61_calc_iq__ff0_dn15 = assign6440_e7733_d_n15;
        locals.var_fn61_calc_iq__ff0_dn16 = assign6440_e7733_d_n16;
        locals.var_fn61_calc_iq__ff0_rv = 0.0;

        let assign6450_e7736: f64 = (-50.0);
        let assign6450_e7737: f64 = if locals.var_fn61_calc_iq__exparg0 < assign6450_e7736 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign6450_e7737;
        locals.var_guard76_rv = 0.0;

        let (assign6460_e7746, assign6460_e7746_d_n2, assign6460_e7746_d_n4, assign6460_e7746_d_n7, assign6460_e7746_d_n15, assign6460_e7746_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard75 == 0.0)) && (locals.var_guard76 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ff0, locals.var_fn61_calc_iq__ff0_dn2, locals.var_fn61_calc_iq__ff0_dn4, locals.var_fn61_calc_iq__ff0_dn7, locals.var_fn61_calc_iq__ff0_dn15, locals.var_fn61_calc_iq__ff0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff0 = assign6460_e7746;
        locals.var_fn61_calc_iq__ff0_dn2 = assign6460_e7746_d_n2;
        locals.var_fn61_calc_iq__ff0_dn4 = assign6460_e7746_d_n4;
        locals.var_fn61_calc_iq__ff0_dn7 = assign6460_e7746_d_n7;
        locals.var_fn61_calc_iq__ff0_dn15 = assign6460_e7746_d_n15;
        locals.var_fn61_calc_iq__ff0_dn16 = assign6460_e7746_d_n16;
        locals.var_fn61_calc_iq__ff0_rv = 0.0;

        let (assign6470_e7761, assign6470_e7761_d_n2, assign6470_e7761_d_n4, assign6470_e7761_d_n7, assign6470_e7761_d_n15, assign6470_e7761_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard75 == 0.0)) && (locals.var_guard76 == 0.0)) {
        let assign6470_e7757: f64 = (locals.var_fn61_calc_iq__exparg0).exp();
        let assign6470_e7758: f64 = (1.0 + assign6470_e7757);
        let assign6470_e7759: f64 = (1.0 / assign6470_e7758);
        (assign6470_e7759, (-((assign6470_e7757 * locals.var_fn61_calc_iq__exparg0_dn2) / (assign6470_e7758 * assign6470_e7758))), (-((assign6470_e7757 * locals.var_fn61_calc_iq__exparg0_dn4) / (assign6470_e7758 * assign6470_e7758))), (-((assign6470_e7757 * locals.var_fn61_calc_iq__exparg0_dn7) / (assign6470_e7758 * assign6470_e7758))), (-((assign6470_e7757 * locals.var_fn61_calc_iq__exparg0_dn15) / (assign6470_e7758 * assign6470_e7758))), (-((assign6470_e7757 * locals.var_fn61_calc_iq__exparg0_dn16) / (assign6470_e7758 * assign6470_e7758))),)
    } else {
        (locals.var_fn61_calc_iq__ff0, locals.var_fn61_calc_iq__ff0_dn2, locals.var_fn61_calc_iq__ff0_dn4, locals.var_fn61_calc_iq__ff0_dn7, locals.var_fn61_calc_iq__ff0_dn15, locals.var_fn61_calc_iq__ff0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ff0 = assign6470_e7761;
        locals.var_fn61_calc_iq__ff0_dn2 = assign6470_e7761_d_n2;
        locals.var_fn61_calc_iq__ff0_dn4 = assign6470_e7761_d_n4;
        locals.var_fn61_calc_iq__ff0_dn7 = assign6470_e7761_d_n7;
        locals.var_fn61_calc_iq__ff0_dn15 = assign6470_e7761_d_n15;
        locals.var_fn61_calc_iq__ff0_dn16 = assign6470_e7761_d_n16;
        locals.var_fn61_calc_iq__ff0_rv = 0.0;

        let (assign6480_e7820, assign6480_e7820_d_n2, assign6480_e7820_d_n4, assign6480_e7820_d_n7, assign6480_e7820_d_n15, assign6480_e7820_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6480_e7806, assign6480_e7806_d_n2, assign6480_e7806_d_n7, assign6480_e7806_d_n15, assign6480_e7806_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6480_e7770: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                let assign6480_e7773: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign6480_e7776: f64 = (0.001 / p.p53);
                let assign6480_e7779: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                let assign6480_e7780: f64 = (assign6480_e7776 * assign6480_e7779);
                let assign6480_e7781: f64 = (assign6480_e7780).tanh();
                let assign6480_e7782: f64 = (assign6480_e7773 * assign6480_e7781);
                let assign6480_e7783: f64 = (assign6480_e7770 + assign6480_e7782);
                let assign6480_e7784: f64 = (0.5 * assign6480_e7783);
                (assign6480_e7784, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign6480_e7781) + (assign6480_e7773 * ((assign6480_e7776 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2)) / ((assign6480_e7780).cosh() * (assign6480_e7780).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign6480_e7781) + (assign6480_e7773 * ((assign6480_e7776 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7)) / ((assign6480_e7780).cosh() * (assign6480_e7780).cosh())))))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign6480_e7781) + (assign6480_e7773 * ((assign6480_e7776 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15)) / ((assign6480_e7780).cosh() * (assign6480_e7780).cosh())))))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + (((-locals.var_fn61_calc_iq__vgdin_dn16) * assign6480_e7781) + (assign6480_e7773 * ((assign6480_e7776 * (-locals.var_fn61_calc_iq__vgdin_dn16)) / ((assign6480_e7780).cosh() * (assign6480_e7780).cosh())))))),)
            } else {
                let (assign6480_e7805, assign6480_e7805_d_n2, assign6480_e7805_d_n7, assign6480_e7805_d_n15, assign6480_e7805_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6480_e7791: f64 = (locals.var_fn61_calc_iq__vgsin + locals.var_fn61_calc_iq__vgdin);
                        let assign6480_e7794: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign6480_e7797: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vgdin);
                        let assign6480_e7798: f64 = (assign6480_e7794 * assign6480_e7797);
                        let assign6480_e7800: f64 = (assign6480_e7798 + p.p53);
                        let assign6480_e7801: f64 = (assign6480_e7800).sqrt();
                        let assign6480_e7802: f64 = (assign6480_e7791 + assign6480_e7801);
                        let assign6480_e7803: f64 = (0.5 * assign6480_e7802);
                        (assign6480_e7803, (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn2 + locals.var_fn61_calc_iq__vgdin_dn2) + ((((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2) * assign6480_e7797) + (assign6480_e7794 * (locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vgdin_dn2))) / (2.0 * assign6480_e7801)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn7 + locals.var_fn61_calc_iq__vgdin_dn7) + ((((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7) * assign6480_e7797) + (assign6480_e7794 * (locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vgdin_dn7))) / (2.0 * assign6480_e7801)))), (0.5 * ((locals.var_fn61_calc_iq__vgsin_dn15 + locals.var_fn61_calc_iq__vgdin_dn15) + ((((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15) * assign6480_e7797) + (assign6480_e7794 * (locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vgdin_dn15))) / (2.0 * assign6480_e7801)))), (0.5 * (locals.var_fn61_calc_iq__vgdin_dn16 + ((((-locals.var_fn61_calc_iq__vgdin_dn16) * assign6480_e7797) + (assign6480_e7794 * (-locals.var_fn61_calc_iq__vgdin_dn16))) / (2.0 * assign6480_e7801)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6480_e7805, assign6480_e7805_d_n2, assign6480_e7805_d_n7, assign6480_e7805_d_n15, assign6480_e7805_d_n16,)
            }
        };
        let assign6480_e7810: f64 = (p.p51 * 0.1);
        let assign6480_e7812: f64 = (assign6480_e7810 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6480_e7814: f64 = (assign6480_e7812 * locals.var_fn61_calc_iq__ff0);
        let assign6480_e7815: f64 = (locals.var_fn61_calc_iq__vtof - assign6480_e7814);
        let assign6480_e7816: f64 = (assign6480_e7806 - assign6480_e7815);
        let assign6480_e7818: f64 = (assign6480_e7816 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign6480_e7818, ((assign6480_e7806_d_n2 - (-(assign6480_e7812 * locals.var_fn61_calc_iq__ff0_dn2))) / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (((assign6480_e7810 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ff0) + (assign6480_e7812 * locals.var_fn61_calc_iq__ff0_dn4)))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign6480_e7816 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), ((assign6480_e7806_d_n7 - (-(assign6480_e7812 * locals.var_fn61_calc_iq__ff0_dn7))) / locals.var_fn61_calc_iq__two_n_phit0), ((assign6480_e7806_d_n15 - (-(assign6480_e7812 * locals.var_fn61_calc_iq__ff0_dn15))) / locals.var_fn61_calc_iq__two_n_phit0), ((assign6480_e7806_d_n16 - (-(assign6480_e7812 * locals.var_fn61_calc_iq__ff0_dn16))) / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__eta0, locals.var_fn61_calc_iq__eta0_dn2, locals.var_fn61_calc_iq__eta0_dn4, locals.var_fn61_calc_iq__eta0_dn7, locals.var_fn61_calc_iq__eta0_dn15, locals.var_fn61_calc_iq__eta0_dn16,)
    }
};
        locals.var_fn61_calc_iq__eta0 = assign6480_e7820;
        locals.var_fn61_calc_iq__eta0_dn2 = assign6480_e7820_d_n2;
        locals.var_fn61_calc_iq__eta0_dn4 = assign6480_e7820_d_n4;
        locals.var_fn61_calc_iq__eta0_dn7 = assign6480_e7820_d_n7;
        locals.var_fn61_calc_iq__eta0_dn15 = assign6480_e7820_d_n15;
        locals.var_fn61_calc_iq__eta0_dn16 = assign6480_e7820_d_n16;
        locals.var_fn61_calc_iq__eta0_rv = 0.0;

        let assign6490_e7823: f64 = if locals.var_fn61_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard77 = assign6490_e7823;
        locals.var_guard77_rv = 0.0;

        let (assign6500_e7831, assign6500_e7831_d_n2, assign6500_e7831_d_n4, assign6500_e7831_d_n7, assign6500_e7831_d_n15, assign6500_e7831_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign6500_e7829: f64 = (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__eta0);
        (assign6500_e7829, (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__eta0_dn2), ((locals.var_fn61_calc_iq__qref0_dn4 * locals.var_fn61_calc_iq__eta0) + (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__eta0_dn4)), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__eta0_dn7), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__eta0_dn15), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__eta0_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qinvv0, locals.var_fn61_calc_iq__qinvv0_dn2, locals.var_fn61_calc_iq__qinvv0_dn4, locals.var_fn61_calc_iq__qinvv0_dn7, locals.var_fn61_calc_iq__qinvv0_dn15, locals.var_fn61_calc_iq__qinvv0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv0 = assign6500_e7831;
        locals.var_fn61_calc_iq__qinvv0_dn2 = assign6500_e7831_d_n2;
        locals.var_fn61_calc_iq__qinvv0_dn4 = assign6500_e7831_d_n4;
        locals.var_fn61_calc_iq__qinvv0_dn7 = assign6500_e7831_d_n7;
        locals.var_fn61_calc_iq__qinvv0_dn15 = assign6500_e7831_d_n15;
        locals.var_fn61_calc_iq__qinvv0_dn16 = assign6500_e7831_d_n16;
        locals.var_fn61_calc_iq__qinvv0_rv = 0.0;

        let assign6510_e7834: f64 = (-50.0);
        let assign6510_e7835: f64 = if locals.var_fn61_calc_iq__eta0 < assign6510_e7834 { 1.0 } else { 0.0 };
        locals.var_guard78 = assign6510_e7835;
        locals.var_guard78_rv = 0.0;

        let (assign6520_e7847, assign6520_e7847_d_n2, assign6520_e7847_d_n4, assign6520_e7847_d_n7, assign6520_e7847_d_n15, assign6520_e7847_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard77 == 0.0)) && (locals.var_guard78 != 0.0)) {
        let assign6520_e7844: f64 = (locals.var_fn61_calc_iq__eta0).exp();
        let assign6520_e7845: f64 = (locals.var_fn61_calc_iq__qref0 * assign6520_e7844);
        (assign6520_e7845, (locals.var_fn61_calc_iq__qref0 * (assign6520_e7844 * locals.var_fn61_calc_iq__eta0_dn2)), ((locals.var_fn61_calc_iq__qref0_dn4 * assign6520_e7844) + (locals.var_fn61_calc_iq__qref0 * (assign6520_e7844 * locals.var_fn61_calc_iq__eta0_dn4))), (locals.var_fn61_calc_iq__qref0 * (assign6520_e7844 * locals.var_fn61_calc_iq__eta0_dn7)), (locals.var_fn61_calc_iq__qref0 * (assign6520_e7844 * locals.var_fn61_calc_iq__eta0_dn15)), (locals.var_fn61_calc_iq__qref0 * (assign6520_e7844 * locals.var_fn61_calc_iq__eta0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvv0, locals.var_fn61_calc_iq__qinvv0_dn2, locals.var_fn61_calc_iq__qinvv0_dn4, locals.var_fn61_calc_iq__qinvv0_dn7, locals.var_fn61_calc_iq__qinvv0_dn15, locals.var_fn61_calc_iq__qinvv0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv0 = assign6520_e7847;
        locals.var_fn61_calc_iq__qinvv0_dn2 = assign6520_e7847_d_n2;
        locals.var_fn61_calc_iq__qinvv0_dn4 = assign6520_e7847_d_n4;
        locals.var_fn61_calc_iq__qinvv0_dn7 = assign6520_e7847_d_n7;
        locals.var_fn61_calc_iq__qinvv0_dn15 = assign6520_e7847_d_n15;
        locals.var_fn61_calc_iq__qinvv0_dn16 = assign6520_e7847_d_n16;
        locals.var_fn61_calc_iq__qinvv0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6530_e7863, assign6530_e7863_d_n2, assign6530_e7863_d_n4, assign6530_e7863_d_n7, assign6530_e7863_d_n15, assign6530_e7863_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard77 == 0.0)) && (locals.var_guard78 == 0.0)) {
        let assign6530_e7858: f64 = (locals.var_fn61_calc_iq__eta0).exp();
        let assign6530_e7859: f64 = (1.0 + assign6530_e7858);
        let assign6530_e7860: f64 = (assign6530_e7859).ln();
        let assign6530_e7861: f64 = (locals.var_fn61_calc_iq__qref0 * assign6530_e7860);
        (assign6530_e7861, (locals.var_fn61_calc_iq__qref0 * ((assign6530_e7858 * locals.var_fn61_calc_iq__eta0_dn2) / assign6530_e7859)), ((locals.var_fn61_calc_iq__qref0_dn4 * assign6530_e7860) + (locals.var_fn61_calc_iq__qref0 * ((assign6530_e7858 * locals.var_fn61_calc_iq__eta0_dn4) / assign6530_e7859))), (locals.var_fn61_calc_iq__qref0 * ((assign6530_e7858 * locals.var_fn61_calc_iq__eta0_dn7) / assign6530_e7859)), (locals.var_fn61_calc_iq__qref0 * ((assign6530_e7858 * locals.var_fn61_calc_iq__eta0_dn15) / assign6530_e7859)), (locals.var_fn61_calc_iq__qref0 * ((assign6530_e7858 * locals.var_fn61_calc_iq__eta0_dn16) / assign6530_e7859)),)
    } else {
        (locals.var_fn61_calc_iq__qinvv0, locals.var_fn61_calc_iq__qinvv0_dn2, locals.var_fn61_calc_iq__qinvv0_dn4, locals.var_fn61_calc_iq__qinvv0_dn7, locals.var_fn61_calc_iq__qinvv0_dn15, locals.var_fn61_calc_iq__qinvv0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvv0 = assign6530_e7863;
        locals.var_fn61_calc_iq__qinvv0_dn2 = assign6530_e7863_d_n2;
        locals.var_fn61_calc_iq__qinvv0_dn4 = assign6530_e7863_d_n4;
        locals.var_fn61_calc_iq__qinvv0_dn7 = assign6530_e7863_d_n7;
        locals.var_fn61_calc_iq__qinvv0_dn15 = assign6530_e7863_d_n15;
        locals.var_fn61_calc_iq__qinvv0_dn16 = assign6530_e7863_d_n16;
        locals.var_fn61_calc_iq__qinvv0_rv = 0.0;

        let (assign6540_e7869, assign6540_e7869_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6540_e7867: f64 = (locals.var_fn61_calc_iq__mu0 / locals.var_fn61_calc_iq__tfacmobin);
        (assign6540_e7867, (-((locals.var_fn61_calc_iq__mu0 * locals.var_fn61_calc_iq__tfacmobin_dn4) / (locals.var_fn61_calc_iq__tfacmobin * locals.var_fn61_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn61_calc_iq__muf0, locals.var_fn61_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn61_calc_iq__muf0 = assign6540_e7869;
        locals.var_fn61_calc_iq__muf0_dn4 = assign6540_e7869_d_n4;
        locals.var_fn61_calc_iq__muf0_rv = 0.0;

        let (assign6550_e7885, assign6550_e7885_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6550_e7875: f64 = (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tnomin);
        let assign6550_e7876: f64 = (1.0 + assign6550_e7875);
        let assign6550_e7880: f64 = (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tambin);
        let assign6550_e7881: f64 = (1.0 + assign6550_e7880);
        let assign6550_e7882: f64 = (assign6550_e7876 / assign6550_e7881);
        let assign6550_e7883: f64 = (locals.var_fn61_calc_iq__vel0 * assign6550_e7882);
        (assign6550_e7883, (locals.var_fn61_calc_iq__vel0 * (-((assign6550_e7876 * (locals.var_fn61_calc_iq__vzeta * locals.var_fn61_calc_iq__tambin_dn4)) / (assign6550_e7881 * assign6550_e7881)))),)
    } else {
        (locals.var_fn61_calc_iq__vx0, locals.var_fn61_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn61_calc_iq__vx0 = assign6550_e7885;
        locals.var_fn61_calc_iq__vx0_dn4 = assign6550_e7885_d_n4;
        locals.var_fn61_calc_iq__vx0_rv = 0.0;

        let (assign6560_e7893, assign6560_e7893_d_n4,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6560_e7889: f64 = (locals.var_fn61_calc_iq__vx0 * locals.var_fn61_calc_iq__lin);
        let assign6560_e7891: f64 = (assign6560_e7889 / locals.var_fn61_calc_iq__muf0);
        (assign6560_e7891, ((((locals.var_fn61_calc_iq__vx0_dn4 * locals.var_fn61_calc_iq__lin) * locals.var_fn61_calc_iq__muf0) - (assign6560_e7889 * locals.var_fn61_calc_iq__muf0_dn4)) / (locals.var_fn61_calc_iq__muf0 * locals.var_fn61_calc_iq__muf0)),)
    } else {
        (locals.var_fn61_calc_iq__vdsats0, locals.var_fn61_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn61_calc_iq__vdsats0 = assign6560_e7893;
        locals.var_fn61_calc_iq__vdsats0_dn4 = assign6560_e7893_d_n4;
        locals.var_fn61_calc_iq__vdsats0_rv = 0.0;

        let (assign6570_e7910, assign6570_e7910_d_n2, assign6570_e7910_d_n4, assign6570_e7910_d_n7, assign6570_e7910_d_n15, assign6570_e7910_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6570_e7899: f64 = (2.0 * locals.var_fn61_calc_iq__qinvv0);
        let assign6570_e7901: f64 = (assign6570_e7899 / locals.var_fn61_calc_iq__cgin);
        let assign6570_e7903: f64 = (assign6570_e7901 / locals.var_fn61_calc_iq__vdsats0);
        let assign6570_e7904: f64 = (1.0 + assign6570_e7903);
        let assign6570_e7905: f64 = (assign6570_e7904).sqrt();
        let assign6570_e7906: f64 = (locals.var_fn61_calc_iq__vdsats0 * assign6570_e7905);
        let assign6570_e7908: f64 = (assign6570_e7906 - locals.var_fn61_calc_iq__vdsats0);
        (assign6570_e7908, (locals.var_fn61_calc_iq__vdsats0 * ((((2.0 * locals.var_fn61_calc_iq__qinvv0_dn2) / locals.var_fn61_calc_iq__cgin) / locals.var_fn61_calc_iq__vdsats0) / (2.0 * assign6570_e7905))), (((locals.var_fn61_calc_iq__vdsats0_dn4 * assign6570_e7905) + (locals.var_fn61_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn61_calc_iq__qinvv0_dn4) * locals.var_fn61_calc_iq__cgin) - (assign6570_e7899 * locals.var_fn61_calc_iq__cgin_dn4)) / (locals.var_fn61_calc_iq__cgin * locals.var_fn61_calc_iq__cgin)) * locals.var_fn61_calc_iq__vdsats0) - (assign6570_e7901 * locals.var_fn61_calc_iq__vdsats0_dn4)) / (locals.var_fn61_calc_iq__vdsats0 * locals.var_fn61_calc_iq__vdsats0)) / (2.0 * assign6570_e7905)))) - locals.var_fn61_calc_iq__vdsats0_dn4), (locals.var_fn61_calc_iq__vdsats0 * ((((2.0 * locals.var_fn61_calc_iq__qinvv0_dn7) / locals.var_fn61_calc_iq__cgin) / locals.var_fn61_calc_iq__vdsats0) / (2.0 * assign6570_e7905))), (locals.var_fn61_calc_iq__vdsats0 * ((((2.0 * locals.var_fn61_calc_iq__qinvv0_dn15) / locals.var_fn61_calc_iq__cgin) / locals.var_fn61_calc_iq__vdsats0) / (2.0 * assign6570_e7905))), (locals.var_fn61_calc_iq__vdsats0 * ((((2.0 * locals.var_fn61_calc_iq__qinvv0_dn16) / locals.var_fn61_calc_iq__cgin) / locals.var_fn61_calc_iq__vdsats0) / (2.0 * assign6570_e7905))),)
    } else {
        (locals.var_fn61_calc_iq__vdsats10, locals.var_fn61_calc_iq__vdsats10_dn2, locals.var_fn61_calc_iq__vdsats10_dn4, locals.var_fn61_calc_iq__vdsats10_dn7, locals.var_fn61_calc_iq__vdsats10_dn15, locals.var_fn61_calc_iq__vdsats10_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsats10 = assign6570_e7910;
        locals.var_fn61_calc_iq__vdsats10_dn2 = assign6570_e7910_d_n2;
        locals.var_fn61_calc_iq__vdsats10_dn4 = assign6570_e7910_d_n4;
        locals.var_fn61_calc_iq__vdsats10_dn7 = assign6570_e7910_d_n7;
        locals.var_fn61_calc_iq__vdsats10_dn15 = assign6570_e7910_d_n15;
        locals.var_fn61_calc_iq__vdsats10_dn16 = assign6570_e7910_d_n16;
        locals.var_fn61_calc_iq__vdsats10_rv = 0.0;

        let (assign6580_e7922, assign6580_e7922_d_n2, assign6580_e7922_d_n4, assign6580_e7922_d_n7, assign6580_e7922_d_n15, assign6580_e7922_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6580_e7915: f64 = (1.0 - locals.var_fn61_calc_iq__ff0);
        let assign6580_e7916: f64 = (locals.var_fn61_calc_iq__vdsats10 * assign6580_e7915);
        let assign6580_e7919: f64 = (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__ff0);
        let assign6580_e7920: f64 = (assign6580_e7916 + assign6580_e7919);
        (assign6580_e7920, (((locals.var_fn61_calc_iq__vdsats10_dn2 * assign6580_e7915) + (locals.var_fn61_calc_iq__vdsats10 * (-locals.var_fn61_calc_iq__ff0_dn2))) + (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__ff0_dn2)), (((locals.var_fn61_calc_iq__vdsats10_dn4 * assign6580_e7915) + (locals.var_fn61_calc_iq__vdsats10 * (-locals.var_fn61_calc_iq__ff0_dn4))) + ((locals.var_fn61_calc_iq__two_n_phit0_dn4 * locals.var_fn61_calc_iq__ff0) + (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__ff0_dn4))), (((locals.var_fn61_calc_iq__vdsats10_dn7 * assign6580_e7915) + (locals.var_fn61_calc_iq__vdsats10 * (-locals.var_fn61_calc_iq__ff0_dn7))) + (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__ff0_dn7)), (((locals.var_fn61_calc_iq__vdsats10_dn15 * assign6580_e7915) + (locals.var_fn61_calc_iq__vdsats10 * (-locals.var_fn61_calc_iq__ff0_dn15))) + (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__ff0_dn15)), (((locals.var_fn61_calc_iq__vdsats10_dn16 * assign6580_e7915) + (locals.var_fn61_calc_iq__vdsats10 * (-locals.var_fn61_calc_iq__ff0_dn16))) + (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__ff0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vdsat10, locals.var_fn61_calc_iq__vdsat10_dn2, locals.var_fn61_calc_iq__vdsat10_dn4, locals.var_fn61_calc_iq__vdsat10_dn7, locals.var_fn61_calc_iq__vdsat10_dn15, locals.var_fn61_calc_iq__vdsat10_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdsat10 = assign6580_e7922;
        locals.var_fn61_calc_iq__vdsat10_dn2 = assign6580_e7922_d_n2;
        locals.var_fn61_calc_iq__vdsat10_dn4 = assign6580_e7922_d_n4;
        locals.var_fn61_calc_iq__vdsat10_dn7 = assign6580_e7922_d_n7;
        locals.var_fn61_calc_iq__vdsat10_dn15 = assign6580_e7922_d_n15;
        locals.var_fn61_calc_iq__vdsat10_dn16 = assign6580_e7922_d_n16;
        locals.var_fn61_calc_iq__vdsat10_rv = 0.0;

        let (assign6590_e7991, assign6590_e7991_d_n2, assign6590_e7991_d_n4, assign6590_e7991_d_n7, assign6590_e7991_d_n15, assign6590_e7991_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6590_e7981, assign6590_e7981_d_n2, assign6590_e7981_d_n4, assign6590_e7981_d_n7, assign6590_e7981_d_n15, assign6590_e7981_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6590_e7934: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat10);
                let assign6590_e7935: f64 = assign6590_e7934;
                let assign6590_e7939: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat10);
                let assign6590_e7940: f64 = (-assign6590_e7939);
                let assign6590_e7943: f64 = (0.001 / p.p53);
                let assign6590_e7947: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat10);
                let assign6590_e7948: f64 = (-assign6590_e7947);
                let assign6590_e7949: f64 = (assign6590_e7943 * assign6590_e7948);
                let assign6590_e7950: f64 = (assign6590_e7949).tanh();
                let assign6590_e7951: f64 = (assign6590_e7940 * assign6590_e7950);
                let assign6590_e7952: f64 = (assign6590_e7935 + assign6590_e7951);
                let assign6590_e7953: f64 = (0.5 * assign6590_e7952);
                (assign6590_e7953, (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6590_e7950) + (assign6590_e7940 * ((assign6590_e7943 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / ((assign6590_e7949).cosh() * (assign6590_e7949).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6590_e7950) + (assign6590_e7940 * ((assign6590_e7943 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / ((assign6590_e7949).cosh() * (assign6590_e7949).cosh())))))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + (((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6590_e7950) + (assign6590_e7940 * ((assign6590_e7943 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / ((assign6590_e7949).cosh() * (assign6590_e7949).cosh())))))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + (((-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6590_e7950) + (assign6590_e7940 * ((assign6590_e7943 * (-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) / ((assign6590_e7949).cosh() * (assign6590_e7949).cosh())))))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + (((-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6590_e7950) + (assign6590_e7940 * ((assign6590_e7943 * (-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) / ((assign6590_e7949).cosh() * (assign6590_e7949).cosh())))))),)
            } else {
                let (assign6590_e7980, assign6590_e7980_d_n2, assign6590_e7980_d_n4, assign6590_e7980_d_n7, assign6590_e7980_d_n15, assign6590_e7980_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6590_e7961: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat10);
                        let assign6590_e7962: f64 = assign6590_e7961;
                        let assign6590_e7966: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat10);
                        let assign6590_e7967: f64 = (-assign6590_e7966);
                        let assign6590_e7971: f64 = (locals.var_fn61_calc_iq__vdsin / locals.var_fn61_calc_iq__vdsat10);
                        let assign6590_e7972: f64 = (-assign6590_e7971);
                        let assign6590_e7973: f64 = (assign6590_e7967 * assign6590_e7972);
                        let assign6590_e7975: f64 = (assign6590_e7973 + p.p53);
                        let assign6590_e7976: f64 = (assign6590_e7975).sqrt();
                        let assign6590_e7977: f64 = (assign6590_e7962 + assign6590_e7976);
                        let assign6590_e7978: f64 = (0.5 * assign6590_e7977);
                        (assign6590_e7978, (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6590_e7972) + (assign6590_e7967 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))))) / (2.0 * assign6590_e7976)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6590_e7972) + (assign6590_e7967 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))))) / (2.0 * assign6590_e7976)))), (0.5 * ((-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + ((((-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6590_e7972) + (assign6590_e7967 * (-(-((locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))))) / (2.0 * assign6590_e7976)))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + ((((-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6590_e7972) + (assign6590_e7967 * (-(((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / (2.0 * assign6590_e7976)))), (0.5 * ((((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + ((((-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6590_e7972) + (assign6590_e7967 * (-(((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__vdsat10) - (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / (2.0 * assign6590_e7976)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6590_e7980, assign6590_e7980_d_n2, assign6590_e7980_d_n4, assign6590_e7980_d_n7, assign6590_e7980_d_n15, assign6590_e7980_d_n16,)
            }
        };
        let assign6590_e7983: f64 = (assign6590_e7981).powf(locals.var_fn61_calc_iq__beta);
        let assign6590_e7984: f64 = (1.0 + assign6590_e7983);
        let assign6590_e7987: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign6590_e7988: f64 = (assign6590_e7984).powf(assign6590_e7987);
        let assign6590_e7989: f64 = (1.0 / assign6590_e7988);
        (assign6590_e7989, (-(if 0.0 == 0.0 && ((assign6590_e7987) as f64).is_finite() && ((assign6590_e7987) as f64).fract() == 0.0 { if assign6590_e7987 == 0.0 { 0.0 } else { (assign6590_e7987 * ((assign6590_e7984).powf(assign6590_e7987 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n2)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n2 / assign6590_e7981))) })) } } else { (assign6590_e7988 * (assign6590_e7987 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n2)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n2 / assign6590_e7981))) } / assign6590_e7984))) } / (assign6590_e7988 * assign6590_e7988))), (-(if 0.0 == 0.0 && ((assign6590_e7987) as f64).is_finite() && ((assign6590_e7987) as f64).fract() == 0.0 { if assign6590_e7987 == 0.0 { 0.0 } else { (assign6590_e7987 * ((assign6590_e7984).powf(assign6590_e7987 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n4)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n4 / assign6590_e7981))) })) } } else { (assign6590_e7988 * (assign6590_e7987 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n4)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n4 / assign6590_e7981))) } / assign6590_e7984))) } / (assign6590_e7988 * assign6590_e7988))), (-(if 0.0 == 0.0 && ((assign6590_e7987) as f64).is_finite() && ((assign6590_e7987) as f64).fract() == 0.0 { if assign6590_e7987 == 0.0 { 0.0 } else { (assign6590_e7987 * ((assign6590_e7984).powf(assign6590_e7987 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n7)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n7 / assign6590_e7981))) })) } } else { (assign6590_e7988 * (assign6590_e7987 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n7)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n7 / assign6590_e7981))) } / assign6590_e7984))) } / (assign6590_e7988 * assign6590_e7988))), (-(if 0.0 == 0.0 && ((assign6590_e7987) as f64).is_finite() && ((assign6590_e7987) as f64).fract() == 0.0 { if assign6590_e7987 == 0.0 { 0.0 } else { (assign6590_e7987 * ((assign6590_e7984).powf(assign6590_e7987 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n15)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n15 / assign6590_e7981))) })) } } else { (assign6590_e7988 * (assign6590_e7987 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n15)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n15 / assign6590_e7981))) } / assign6590_e7984))) } / (assign6590_e7988 * assign6590_e7988))), (-(if 0.0 == 0.0 && ((assign6590_e7987) as f64).is_finite() && ((assign6590_e7987) as f64).fract() == 0.0 { if assign6590_e7987 == 0.0 { 0.0 } else { (assign6590_e7987 * ((assign6590_e7984).powf(assign6590_e7987 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n16)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n16 / assign6590_e7981))) })) } } else { (assign6590_e7988 * (assign6590_e7987 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6590_e7981).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6590_e7981_d_n16)) } } else { (assign6590_e7983 * (locals.var_fn61_calc_iq__beta * (assign6590_e7981_d_n16 / assign6590_e7981))) } / assign6590_e7984))) } / (assign6590_e7988 * assign6590_e7988))),)
    } else {
        (locals.var_fn61_calc_iq__fsd0, locals.var_fn61_calc_iq__fsd0_dn2, locals.var_fn61_calc_iq__fsd0_dn4, locals.var_fn61_calc_iq__fsd0_dn7, locals.var_fn61_calc_iq__fsd0_dn15, locals.var_fn61_calc_iq__fsd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__fsd0 = assign6590_e7991;
        locals.var_fn61_calc_iq__fsd0_dn2 = assign6590_e7991_d_n2;
        locals.var_fn61_calc_iq__fsd0_dn4 = assign6590_e7991_d_n4;
        locals.var_fn61_calc_iq__fsd0_dn7 = assign6590_e7991_d_n7;
        locals.var_fn61_calc_iq__fsd0_dn15 = assign6590_e7991_d_n15;
        locals.var_fn61_calc_iq__fsd0_dn16 = assign6590_e7991_d_n16;
        locals.var_fn61_calc_iq__fsd0_rv = 0.0;

        let (assign6600_e7997, assign6600_e7997_d_n2, assign6600_e7997_d_n4, assign6600_e7997_d_n7, assign6600_e7997_d_n15, assign6600_e7997_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6600_e7995: f64 = (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd0);
        (assign6600_e7995, (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd0_dn2), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd0_dn4), (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd0_dn7), ((locals.var_fn61_calc_iq__vdsin_dn15 * locals.var_fn61_calc_iq__fsd0) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd0_dn15)), ((locals.var_fn61_calc_iq__vdsin_dn16 * locals.var_fn61_calc_iq__fsd0) + (locals.var_fn61_calc_iq__vdsin * locals.var_fn61_calc_iq__fsd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vdx0, locals.var_fn61_calc_iq__vdx0_dn2, locals.var_fn61_calc_iq__vdx0_dn4, locals.var_fn61_calc_iq__vdx0_dn7, locals.var_fn61_calc_iq__vdx0_dn15, locals.var_fn61_calc_iq__vdx0_dn16,)
    }
};
        locals.var_fn61_calc_iq__vdx0 = assign6600_e7997;
        locals.var_fn61_calc_iq__vdx0_dn2 = assign6600_e7997_d_n2;
        locals.var_fn61_calc_iq__vdx0_dn4 = assign6600_e7997_d_n4;
        locals.var_fn61_calc_iq__vdx0_dn7 = assign6600_e7997_d_n7;
        locals.var_fn61_calc_iq__vdx0_dn15 = assign6600_e7997_d_n15;
        locals.var_fn61_calc_iq__vdx0_dn16 = assign6600_e7997_d_n16;
        locals.var_fn61_calc_iq__vdx0_rv = 0.0;

        let (assign6610_e8072, assign6610_e8072_d_n2, assign6610_e8072_d_n4, assign6610_e8072_d_n7, assign6610_e8072_d_n15, assign6610_e8072_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let (assign6610_e8062, assign6610_e8062_d_n2, assign6610_e8062_d_n4, assign6610_e8062_d_n7, assign6610_e8062_d_n15, assign6610_e8062_d_n16,) = {
            if (p.p52 != 0.0) {
                let assign6610_e8008: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6610_e8010: f64 = (assign6610_e8008 / locals.var_fn61_calc_iq__vdsat10);
                let assign6610_e8011: f64 = assign6610_e8010;
                let assign6610_e8014: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6610_e8016: f64 = (assign6610_e8014 / locals.var_fn61_calc_iq__vdsat10);
                let assign6610_e8017: f64 = (-assign6610_e8016);
                let assign6610_e8020: f64 = (0.001 / p.p53);
                let assign6610_e8023: f64 = (-locals.var_fn61_calc_iq__vdsin);
                let assign6610_e8025: f64 = (assign6610_e8023 / locals.var_fn61_calc_iq__vdsat10);
                let assign6610_e8026: f64 = (-assign6610_e8025);
                let assign6610_e8027: f64 = (assign6610_e8020 * assign6610_e8026);
                let assign6610_e8028: f64 = (assign6610_e8027).tanh();
                let assign6610_e8029: f64 = (assign6610_e8017 * assign6610_e8028);
                let assign6610_e8030: f64 = (assign6610_e8011 + assign6610_e8029);
                let assign6610_e8031: f64 = (0.5 * assign6610_e8030);
                (assign6610_e8031, (0.5 * ((-((assign6610_e8008 * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + (((-(-((assign6610_e8014 * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6610_e8028) + (assign6610_e8017 * ((assign6610_e8020 * (-(-((assign6610_e8023 * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / ((assign6610_e8027).cosh() * (assign6610_e8027).cosh())))))), (0.5 * ((-((assign6610_e8008 * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + (((-(-((assign6610_e8014 * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6610_e8028) + (assign6610_e8017 * ((assign6610_e8020 * (-(-((assign6610_e8023 * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / ((assign6610_e8027).cosh() * (assign6610_e8027).cosh())))))), (0.5 * ((-((assign6610_e8008 * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + (((-(-((assign6610_e8014 * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6610_e8028) + (assign6610_e8017 * ((assign6610_e8020 * (-(-((assign6610_e8023 * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / ((assign6610_e8027).cosh() * (assign6610_e8027).cosh())))))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8008 * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + (((-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8014 * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6610_e8028) + (assign6610_e8017 * ((assign6610_e8020 * (-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8023 * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) / ((assign6610_e8027).cosh() * (assign6610_e8027).cosh())))))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8008 * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + (((-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8014 * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6610_e8028) + (assign6610_e8017 * ((assign6610_e8020 * (-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8023 * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) / ((assign6610_e8027).cosh() * (assign6610_e8027).cosh())))))),)
            } else {
                let (assign6610_e8061, assign6610_e8061_d_n2, assign6610_e8061_d_n4, assign6610_e8061_d_n7, assign6610_e8061_d_n15, assign6610_e8061_d_n16,) = {
                    if (p.p52 == 0.0) {
                        let assign6610_e8038: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6610_e8040: f64 = (assign6610_e8038 / locals.var_fn61_calc_iq__vdsat10);
                        let assign6610_e8041: f64 = assign6610_e8040;
                        let assign6610_e8044: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6610_e8046: f64 = (assign6610_e8044 / locals.var_fn61_calc_iq__vdsat10);
                        let assign6610_e8047: f64 = (-assign6610_e8046);
                        let assign6610_e8050: f64 = (-locals.var_fn61_calc_iq__vdsin);
                        let assign6610_e8052: f64 = (assign6610_e8050 / locals.var_fn61_calc_iq__vdsat10);
                        let assign6610_e8053: f64 = (-assign6610_e8052);
                        let assign6610_e8054: f64 = (assign6610_e8047 * assign6610_e8053);
                        let assign6610_e8056: f64 = (assign6610_e8054 + p.p53);
                        let assign6610_e8057: f64 = (assign6610_e8056).sqrt();
                        let assign6610_e8058: f64 = (assign6610_e8041 + assign6610_e8057);
                        let assign6610_e8059: f64 = (0.5 * assign6610_e8058);
                        (assign6610_e8059, (0.5 * ((-((assign6610_e8038 * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + ((((-(-((assign6610_e8044 * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6610_e8053) + (assign6610_e8047 * (-(-((assign6610_e8050 * locals.var_fn61_calc_iq__vdsat10_dn2) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))))) / (2.0 * assign6610_e8057)))), (0.5 * ((-((assign6610_e8038 * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + ((((-(-((assign6610_e8044 * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6610_e8053) + (assign6610_e8047 * (-(-((assign6610_e8050 * locals.var_fn61_calc_iq__vdsat10_dn4) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))))) / (2.0 * assign6610_e8057)))), (0.5 * ((-((assign6610_e8038 * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) + ((((-(-((assign6610_e8044 * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))) * assign6610_e8053) + (assign6610_e8047 * (-(-((assign6610_e8050 * locals.var_fn61_calc_iq__vdsat10_dn7) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)))))) / (2.0 * assign6610_e8057)))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8038 * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + ((((-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8044 * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6610_e8053) + (assign6610_e8047 * (-((((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8050 * locals.var_fn61_calc_iq__vdsat10_dn15)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / (2.0 * assign6610_e8057)))), (0.5 * (((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8038 * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10)) + ((((-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8044 * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))) * assign6610_e8053) + (assign6610_e8047 * (-((((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__vdsat10) - (assign6610_e8050 * locals.var_fn61_calc_iq__vdsat10_dn16)) / (locals.var_fn61_calc_iq__vdsat10 * locals.var_fn61_calc_iq__vdsat10))))) / (2.0 * assign6610_e8057)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign6610_e8061, assign6610_e8061_d_n2, assign6610_e8061_d_n4, assign6610_e8061_d_n7, assign6610_e8061_d_n15, assign6610_e8061_d_n16,)
            }
        };
        let assign6610_e8064: f64 = (assign6610_e8062).powf(locals.var_fn61_calc_iq__beta);
        let assign6610_e8065: f64 = (1.0 + assign6610_e8064);
        let assign6610_e8068: f64 = (1.0 / locals.var_fn61_calc_iq__beta);
        let assign6610_e8069: f64 = (assign6610_e8065).powf(assign6610_e8068);
        let assign6610_e8070: f64 = (1.0 / assign6610_e8069);
        (assign6610_e8070, (-(if 0.0 == 0.0 && ((assign6610_e8068) as f64).is_finite() && ((assign6610_e8068) as f64).fract() == 0.0 { if assign6610_e8068 == 0.0 { 0.0 } else { (assign6610_e8068 * ((assign6610_e8065).powf(assign6610_e8068 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n2)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n2 / assign6610_e8062))) })) } } else { (assign6610_e8069 * (assign6610_e8068 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n2)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n2 / assign6610_e8062))) } / assign6610_e8065))) } / (assign6610_e8069 * assign6610_e8069))), (-(if 0.0 == 0.0 && ((assign6610_e8068) as f64).is_finite() && ((assign6610_e8068) as f64).fract() == 0.0 { if assign6610_e8068 == 0.0 { 0.0 } else { (assign6610_e8068 * ((assign6610_e8065).powf(assign6610_e8068 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n4)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n4 / assign6610_e8062))) })) } } else { (assign6610_e8069 * (assign6610_e8068 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n4)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n4 / assign6610_e8062))) } / assign6610_e8065))) } / (assign6610_e8069 * assign6610_e8069))), (-(if 0.0 == 0.0 && ((assign6610_e8068) as f64).is_finite() && ((assign6610_e8068) as f64).fract() == 0.0 { if assign6610_e8068 == 0.0 { 0.0 } else { (assign6610_e8068 * ((assign6610_e8065).powf(assign6610_e8068 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n7)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n7 / assign6610_e8062))) })) } } else { (assign6610_e8069 * (assign6610_e8068 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n7)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n7 / assign6610_e8062))) } / assign6610_e8065))) } / (assign6610_e8069 * assign6610_e8069))), (-(if 0.0 == 0.0 && ((assign6610_e8068) as f64).is_finite() && ((assign6610_e8068) as f64).fract() == 0.0 { if assign6610_e8068 == 0.0 { 0.0 } else { (assign6610_e8068 * ((assign6610_e8065).powf(assign6610_e8068 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n15)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n15 / assign6610_e8062))) })) } } else { (assign6610_e8069 * (assign6610_e8068 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n15)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n15 / assign6610_e8062))) } / assign6610_e8065))) } / (assign6610_e8069 * assign6610_e8069))), (-(if 0.0 == 0.0 && ((assign6610_e8068) as f64).is_finite() && ((assign6610_e8068) as f64).fract() == 0.0 { if assign6610_e8068 == 0.0 { 0.0 } else { (assign6610_e8068 * ((assign6610_e8065).powf(assign6610_e8068 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n16)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n16 / assign6610_e8062))) })) } } else { (assign6610_e8069 * (assign6610_e8068 * (if 0.0 == 0.0 && ((locals.var_fn61_calc_iq__beta) as f64).is_finite() && ((locals.var_fn61_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn61_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn61_calc_iq__beta * ((assign6610_e8062).powf(locals.var_fn61_calc_iq__beta - 1.0) * assign6610_e8062_d_n16)) } } else { (assign6610_e8064 * (locals.var_fn61_calc_iq__beta * (assign6610_e8062_d_n16 / assign6610_e8062))) } / assign6610_e8065))) } / (assign6610_e8069 * assign6610_e8069))),)
    } else {
        (locals.var_fn61_calc_iq__fds0, locals.var_fn61_calc_iq__fds0_dn2, locals.var_fn61_calc_iq__fds0_dn4, locals.var_fn61_calc_iq__fds0_dn7, locals.var_fn61_calc_iq__fds0_dn15, locals.var_fn61_calc_iq__fds0_dn16,)
    }
};
        locals.var_fn61_calc_iq__fds0 = assign6610_e8072;
        locals.var_fn61_calc_iq__fds0_dn2 = assign6610_e8072_d_n2;
        locals.var_fn61_calc_iq__fds0_dn4 = assign6610_e8072_d_n4;
        locals.var_fn61_calc_iq__fds0_dn7 = assign6610_e8072_d_n7;
        locals.var_fn61_calc_iq__fds0_dn15 = assign6610_e8072_d_n15;
        locals.var_fn61_calc_iq__fds0_dn16 = assign6610_e8072_d_n16;
        locals.var_fn61_calc_iq__fds0_rv = 0.0;

        let (assign6620_e8079, assign6620_e8079_d_n2, assign6620_e8079_d_n4, assign6620_e8079_d_n7, assign6620_e8079_d_n15, assign6620_e8079_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6620_e8075: f64 = (-locals.var_fn61_calc_iq__vdsin);
        let assign6620_e8077: f64 = (assign6620_e8075 * locals.var_fn61_calc_iq__fds0);
        (assign6620_e8077, (assign6620_e8075 * locals.var_fn61_calc_iq__fds0_dn2), (assign6620_e8075 * locals.var_fn61_calc_iq__fds0_dn4), (assign6620_e8075 * locals.var_fn61_calc_iq__fds0_dn7), (((-locals.var_fn61_calc_iq__vdsin_dn15) * locals.var_fn61_calc_iq__fds0) + (assign6620_e8075 * locals.var_fn61_calc_iq__fds0_dn15)), (((-locals.var_fn61_calc_iq__vdsin_dn16) * locals.var_fn61_calc_iq__fds0) + (assign6620_e8075 * locals.var_fn61_calc_iq__fds0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__vsx0, locals.var_fn61_calc_iq__vsx0_dn2, locals.var_fn61_calc_iq__vsx0_dn4, locals.var_fn61_calc_iq__vsx0_dn7, locals.var_fn61_calc_iq__vsx0_dn15, locals.var_fn61_calc_iq__vsx0_dn16,)
    }
};
        locals.var_fn61_calc_iq__vsx0 = assign6620_e8079;
        locals.var_fn61_calc_iq__vsx0_dn2 = assign6620_e8079_d_n2;
        locals.var_fn61_calc_iq__vsx0_dn4 = assign6620_e8079_d_n4;
        locals.var_fn61_calc_iq__vsx0_dn7 = assign6620_e8079_d_n7;
        locals.var_fn61_calc_iq__vsx0_dn15 = assign6620_e8079_d_n15;
        locals.var_fn61_calc_iq__vsx0_dn16 = assign6620_e8079_d_n16;
        locals.var_fn61_calc_iq__vsx0_rv = 0.0;

        let (assign6630_e8087, assign6630_e8087_d_n2, assign6630_e8087_d_n4, assign6630_e8087_d_n7, assign6630_e8087_d_n15, assign6630_e8087_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6630_e8083: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__myarg0);
        let assign6630_e8085: f64 = (assign6630_e8083 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6630_e8085, (locals.var_fn61_calc_iq__vgsin_dn2 / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg0_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6630_e8083 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), (locals.var_fn61_calc_iq__vgsin_dn7 / locals.var_fn61_calc_iq__alpha_phit), (locals.var_fn61_calc_iq__vgsin_dn15 / locals.var_fn61_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg0, locals.var_fn61_calc_iq__exparg0_dn2, locals.var_fn61_calc_iq__exparg0_dn4, locals.var_fn61_calc_iq__exparg0_dn7, locals.var_fn61_calc_iq__exparg0_dn15, locals.var_fn61_calc_iq__exparg0_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg0 = assign6630_e8087;
        locals.var_fn61_calc_iq__exparg0_dn2 = assign6630_e8087_d_n2;
        locals.var_fn61_calc_iq__exparg0_dn4 = assign6630_e8087_d_n4;
        locals.var_fn61_calc_iq__exparg0_dn7 = assign6630_e8087_d_n7;
        locals.var_fn61_calc_iq__exparg0_dn15 = assign6630_e8087_d_n15;
        locals.var_fn61_calc_iq__exparg0_dn16 = assign6630_e8087_d_n16;
        locals.var_fn61_calc_iq__exparg0_rv = 0.0;

        let assign6640_e8090: f64 = if locals.var_fn61_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign6640_e8090;
        locals.var_guard79_rv = 0.0;

        let (assign6650_e8096, assign6650_e8096_d_n2, assign6650_e8096_d_n4, assign6650_e8096_d_n7, assign6650_e8096_d_n15, assign6650_e8096_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard79 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs0, locals.var_fn61_calc_iq__ffs0_dn2, locals.var_fn61_calc_iq__ffs0_dn4, locals.var_fn61_calc_iq__ffs0_dn7, locals.var_fn61_calc_iq__ffs0_dn15, locals.var_fn61_calc_iq__ffs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs0 = assign6650_e8096;
        locals.var_fn61_calc_iq__ffs0_dn2 = assign6650_e8096_d_n2;
        locals.var_fn61_calc_iq__ffs0_dn4 = assign6650_e8096_d_n4;
        locals.var_fn61_calc_iq__ffs0_dn7 = assign6650_e8096_d_n7;
        locals.var_fn61_calc_iq__ffs0_dn15 = assign6650_e8096_d_n15;
        locals.var_fn61_calc_iq__ffs0_dn16 = assign6650_e8096_d_n16;
        locals.var_fn61_calc_iq__ffs0_rv = 0.0;

        let assign6660_e8099: f64 = (-50.0);
        let assign6660_e8100: f64 = if locals.var_fn61_calc_iq__exparg0 < assign6660_e8099 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign6660_e8100;
        locals.var_guard80_rv = 0.0;

        let (assign6670_e8109, assign6670_e8109_d_n2, assign6670_e8109_d_n4, assign6670_e8109_d_n7, assign6670_e8109_d_n15, assign6670_e8109_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard79 == 0.0)) && (locals.var_guard80 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffs0, locals.var_fn61_calc_iq__ffs0_dn2, locals.var_fn61_calc_iq__ffs0_dn4, locals.var_fn61_calc_iq__ffs0_dn7, locals.var_fn61_calc_iq__ffs0_dn15, locals.var_fn61_calc_iq__ffs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs0 = assign6670_e8109;
        locals.var_fn61_calc_iq__ffs0_dn2 = assign6670_e8109_d_n2;
        locals.var_fn61_calc_iq__ffs0_dn4 = assign6670_e8109_d_n4;
        locals.var_fn61_calc_iq__ffs0_dn7 = assign6670_e8109_d_n7;
        locals.var_fn61_calc_iq__ffs0_dn15 = assign6670_e8109_d_n15;
        locals.var_fn61_calc_iq__ffs0_dn16 = assign6670_e8109_d_n16;
        locals.var_fn61_calc_iq__ffs0_rv = 0.0;

        let (assign6680_e8124, assign6680_e8124_d_n2, assign6680_e8124_d_n4, assign6680_e8124_d_n7, assign6680_e8124_d_n15, assign6680_e8124_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard79 == 0.0)) && (locals.var_guard80 == 0.0)) {
        let assign6680_e8120: f64 = (locals.var_fn61_calc_iq__exparg0).exp();
        let assign6680_e8121: f64 = (1.0 + assign6680_e8120);
        let assign6680_e8122: f64 = (1.0 / assign6680_e8121);
        (assign6680_e8122, (-((assign6680_e8120 * locals.var_fn61_calc_iq__exparg0_dn2) / (assign6680_e8121 * assign6680_e8121))), (-((assign6680_e8120 * locals.var_fn61_calc_iq__exparg0_dn4) / (assign6680_e8121 * assign6680_e8121))), (-((assign6680_e8120 * locals.var_fn61_calc_iq__exparg0_dn7) / (assign6680_e8121 * assign6680_e8121))), (-((assign6680_e8120 * locals.var_fn61_calc_iq__exparg0_dn15) / (assign6680_e8121 * assign6680_e8121))), (-((assign6680_e8120 * locals.var_fn61_calc_iq__exparg0_dn16) / (assign6680_e8121 * assign6680_e8121))),)
    } else {
        (locals.var_fn61_calc_iq__ffs0, locals.var_fn61_calc_iq__ffs0_dn2, locals.var_fn61_calc_iq__ffs0_dn4, locals.var_fn61_calc_iq__ffs0_dn7, locals.var_fn61_calc_iq__ffs0_dn15, locals.var_fn61_calc_iq__ffs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffs0 = assign6680_e8124;
        locals.var_fn61_calc_iq__ffs0_dn2 = assign6680_e8124_d_n2;
        locals.var_fn61_calc_iq__ffs0_dn4 = assign6680_e8124_d_n4;
        locals.var_fn61_calc_iq__ffs0_dn7 = assign6680_e8124_d_n7;
        locals.var_fn61_calc_iq__ffs0_dn15 = assign6680_e8124_d_n15;
        locals.var_fn61_calc_iq__ffs0_dn16 = assign6680_e8124_d_n16;
        locals.var_fn61_calc_iq__ffs0_rv = 0.0;

        let (assign6690_e8142, assign6690_e8142_d_n2, assign6690_e8142_d_n4, assign6690_e8142_d_n7, assign6690_e8142_d_n15, assign6690_e8142_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6690_e8128: f64 = (locals.var_fn61_calc_iq__vgdin - locals.var_fn61_calc_iq__vsx0);
        let assign6690_e8132: f64 = (p.p51 * 0.1);
        let assign6690_e8134: f64 = (assign6690_e8132 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6690_e8136: f64 = (assign6690_e8134 * locals.var_fn61_calc_iq__ffs0);
        let assign6690_e8137: f64 = (locals.var_fn61_calc_iq__vtof - assign6690_e8136);
        let assign6690_e8138: f64 = (assign6690_e8128 - assign6690_e8137);
        let assign6690_e8140: f64 = (assign6690_e8138 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign6690_e8140, (((locals.var_fn61_calc_iq__vgdin_dn2 - locals.var_fn61_calc_iq__vsx0_dn2) - (-(assign6690_e8134 * locals.var_fn61_calc_iq__ffs0_dn2))) / locals.var_fn61_calc_iq__two_n_phit0), (((((-locals.var_fn61_calc_iq__vsx0_dn4) - (locals.var_fn61_calc_iq__vtof_dn4 - (((assign6690_e8132 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ffs0) + (assign6690_e8134 * locals.var_fn61_calc_iq__ffs0_dn4)))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign6690_e8138 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (((locals.var_fn61_calc_iq__vgdin_dn7 - locals.var_fn61_calc_iq__vsx0_dn7) - (-(assign6690_e8134 * locals.var_fn61_calc_iq__ffs0_dn7))) / locals.var_fn61_calc_iq__two_n_phit0), (((locals.var_fn61_calc_iq__vgdin_dn15 - locals.var_fn61_calc_iq__vsx0_dn15) - (-(assign6690_e8134 * locals.var_fn61_calc_iq__ffs0_dn15))) / locals.var_fn61_calc_iq__two_n_phit0), (((locals.var_fn61_calc_iq__vgdin_dn16 - locals.var_fn61_calc_iq__vsx0_dn16) - (-(assign6690_e8134 * locals.var_fn61_calc_iq__ffs0_dn16))) / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etas0, locals.var_fn61_calc_iq__etas0_dn2, locals.var_fn61_calc_iq__etas0_dn4, locals.var_fn61_calc_iq__etas0_dn7, locals.var_fn61_calc_iq__etas0_dn15, locals.var_fn61_calc_iq__etas0_dn16,)
    }
};
        locals.var_fn61_calc_iq__etas0 = assign6690_e8142;
        locals.var_fn61_calc_iq__etas0_dn2 = assign6690_e8142_d_n2;
        locals.var_fn61_calc_iq__etas0_dn4 = assign6690_e8142_d_n4;
        locals.var_fn61_calc_iq__etas0_dn7 = assign6690_e8142_d_n7;
        locals.var_fn61_calc_iq__etas0_dn15 = assign6690_e8142_d_n15;
        locals.var_fn61_calc_iq__etas0_dn16 = assign6690_e8142_d_n16;
        locals.var_fn61_calc_iq__etas0_rv = 0.0;

        let assign6700_e8145: f64 = if locals.var_fn61_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign6700_e8145;
        locals.var_guard81_rv = 0.0;

        let (assign6710_e8153, assign6710_e8153_d_n2, assign6710_e8153_d_n4, assign6710_e8153_d_n7, assign6710_e8153_d_n15, assign6710_e8153_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign6710_e8151: f64 = (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etas0);
        (assign6710_e8151, (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etas0_dn2), ((locals.var_fn61_calc_iq__qref0_dn4 * locals.var_fn61_calc_iq__etas0) + (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etas0_dn4)), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etas0_dn7), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etas0_dn15), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etas0_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qinvs0, locals.var_fn61_calc_iq__qinvs0_dn2, locals.var_fn61_calc_iq__qinvs0_dn4, locals.var_fn61_calc_iq__qinvs0_dn7, locals.var_fn61_calc_iq__qinvs0_dn15, locals.var_fn61_calc_iq__qinvs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs0 = assign6710_e8153;
        locals.var_fn61_calc_iq__qinvs0_dn2 = assign6710_e8153_d_n2;
        locals.var_fn61_calc_iq__qinvs0_dn4 = assign6710_e8153_d_n4;
        locals.var_fn61_calc_iq__qinvs0_dn7 = assign6710_e8153_d_n7;
        locals.var_fn61_calc_iq__qinvs0_dn15 = assign6710_e8153_d_n15;
        locals.var_fn61_calc_iq__qinvs0_dn16 = assign6710_e8153_d_n16;
        locals.var_fn61_calc_iq__qinvs0_rv = 0.0;

        let assign6720_e8156: f64 = (-50.0);
        let assign6720_e8157: f64 = if locals.var_fn61_calc_iq__etas0 < assign6720_e8156 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign6720_e8157;
        locals.var_guard82_rv = 0.0;

        let (assign6730_e8169, assign6730_e8169_d_n2, assign6730_e8169_d_n4, assign6730_e8169_d_n7, assign6730_e8169_d_n15, assign6730_e8169_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 != 0.0)) {
        let assign6730_e8166: f64 = (locals.var_fn61_calc_iq__etas0).exp();
        let assign6730_e8167: f64 = (locals.var_fn61_calc_iq__qref0 * assign6730_e8166);
        (assign6730_e8167, (locals.var_fn61_calc_iq__qref0 * (assign6730_e8166 * locals.var_fn61_calc_iq__etas0_dn2)), ((locals.var_fn61_calc_iq__qref0_dn4 * assign6730_e8166) + (locals.var_fn61_calc_iq__qref0 * (assign6730_e8166 * locals.var_fn61_calc_iq__etas0_dn4))), (locals.var_fn61_calc_iq__qref0 * (assign6730_e8166 * locals.var_fn61_calc_iq__etas0_dn7)), (locals.var_fn61_calc_iq__qref0 * (assign6730_e8166 * locals.var_fn61_calc_iq__etas0_dn15)), (locals.var_fn61_calc_iq__qref0 * (assign6730_e8166 * locals.var_fn61_calc_iq__etas0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvs0, locals.var_fn61_calc_iq__qinvs0_dn2, locals.var_fn61_calc_iq__qinvs0_dn4, locals.var_fn61_calc_iq__qinvs0_dn7, locals.var_fn61_calc_iq__qinvs0_dn15, locals.var_fn61_calc_iq__qinvs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs0 = assign6730_e8169;
        locals.var_fn61_calc_iq__qinvs0_dn2 = assign6730_e8169_d_n2;
        locals.var_fn61_calc_iq__qinvs0_dn4 = assign6730_e8169_d_n4;
        locals.var_fn61_calc_iq__qinvs0_dn7 = assign6730_e8169_d_n7;
        locals.var_fn61_calc_iq__qinvs0_dn15 = assign6730_e8169_d_n15;
        locals.var_fn61_calc_iq__qinvs0_dn16 = assign6730_e8169_d_n16;
        locals.var_fn61_calc_iq__qinvs0_rv = 0.0;

        let (assign6740_e8185, assign6740_e8185_d_n2, assign6740_e8185_d_n4, assign6740_e8185_d_n7, assign6740_e8185_d_n15, assign6740_e8185_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard81 == 0.0)) && (locals.var_guard82 == 0.0)) {
        let assign6740_e8180: f64 = (locals.var_fn61_calc_iq__etas0).exp();
        let assign6740_e8181: f64 = (1.0 + assign6740_e8180);
        let assign6740_e8182: f64 = (assign6740_e8181).ln();
        let assign6740_e8183: f64 = (locals.var_fn61_calc_iq__qref0 * assign6740_e8182);
        (assign6740_e8183, (locals.var_fn61_calc_iq__qref0 * ((assign6740_e8180 * locals.var_fn61_calc_iq__etas0_dn2) / assign6740_e8181)), ((locals.var_fn61_calc_iq__qref0_dn4 * assign6740_e8182) + (locals.var_fn61_calc_iq__qref0 * ((assign6740_e8180 * locals.var_fn61_calc_iq__etas0_dn4) / assign6740_e8181))), (locals.var_fn61_calc_iq__qref0 * ((assign6740_e8180 * locals.var_fn61_calc_iq__etas0_dn7) / assign6740_e8181)), (locals.var_fn61_calc_iq__qref0 * ((assign6740_e8180 * locals.var_fn61_calc_iq__etas0_dn15) / assign6740_e8181)), (locals.var_fn61_calc_iq__qref0 * ((assign6740_e8180 * locals.var_fn61_calc_iq__etas0_dn16) / assign6740_e8181)),)
    } else {
        (locals.var_fn61_calc_iq__qinvs0, locals.var_fn61_calc_iq__qinvs0_dn2, locals.var_fn61_calc_iq__qinvs0_dn4, locals.var_fn61_calc_iq__qinvs0_dn7, locals.var_fn61_calc_iq__qinvs0_dn15, locals.var_fn61_calc_iq__qinvs0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvs0 = assign6740_e8185;
        locals.var_fn61_calc_iq__qinvs0_dn2 = assign6740_e8185_d_n2;
        locals.var_fn61_calc_iq__qinvs0_dn4 = assign6740_e8185_d_n4;
        locals.var_fn61_calc_iq__qinvs0_dn7 = assign6740_e8185_d_n7;
        locals.var_fn61_calc_iq__qinvs0_dn15 = assign6740_e8185_d_n15;
        locals.var_fn61_calc_iq__qinvs0_dn16 = assign6740_e8185_d_n16;
        locals.var_fn61_calc_iq__qinvs0_rv = 0.0;

        let (assign6750_e8193, assign6750_e8193_d_n2, assign6750_e8193_d_n4, assign6750_e8193_d_n7, assign6750_e8193_d_n15, assign6750_e8193_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6750_e8189: f64 = (locals.var_fn61_calc_iq__vgdin - locals.var_fn61_calc_iq__myarg0);
        let assign6750_e8191: f64 = (assign6750_e8189 / locals.var_fn61_calc_iq__alpha_phit);
        (assign6750_e8191, (locals.var_fn61_calc_iq__vgdin_dn2 / locals.var_fn61_calc_iq__alpha_phit), ((((-locals.var_fn61_calc_iq__myarg0_dn4) * locals.var_fn61_calc_iq__alpha_phit) - (assign6750_e8189 * locals.var_fn61_calc_iq__alpha_phit_dn4)) / (locals.var_fn61_calc_iq__alpha_phit * locals.var_fn61_calc_iq__alpha_phit)), (locals.var_fn61_calc_iq__vgdin_dn7 / locals.var_fn61_calc_iq__alpha_phit), (locals.var_fn61_calc_iq__vgdin_dn15 / locals.var_fn61_calc_iq__alpha_phit), (locals.var_fn61_calc_iq__vgdin_dn16 / locals.var_fn61_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn61_calc_iq__exparg0, locals.var_fn61_calc_iq__exparg0_dn2, locals.var_fn61_calc_iq__exparg0_dn4, locals.var_fn61_calc_iq__exparg0_dn7, locals.var_fn61_calc_iq__exparg0_dn15, locals.var_fn61_calc_iq__exparg0_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg0 = assign6750_e8193;
        locals.var_fn61_calc_iq__exparg0_dn2 = assign6750_e8193_d_n2;
        locals.var_fn61_calc_iq__exparg0_dn4 = assign6750_e8193_d_n4;
        locals.var_fn61_calc_iq__exparg0_dn7 = assign6750_e8193_d_n7;
        locals.var_fn61_calc_iq__exparg0_dn15 = assign6750_e8193_d_n15;
        locals.var_fn61_calc_iq__exparg0_dn16 = assign6750_e8193_d_n16;
        locals.var_fn61_calc_iq__exparg0_rv = 0.0;

        let assign6760_e8196: f64 = if locals.var_fn61_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign6760_e8196;
        locals.var_guard83_rv = 0.0;

        let (assign6770_e8202, assign6770_e8202_d_n2, assign6770_e8202_d_n4, assign6770_e8202_d_n7, assign6770_e8202_d_n15, assign6770_e8202_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard83 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd0, locals.var_fn61_calc_iq__ffd0_dn2, locals.var_fn61_calc_iq__ffd0_dn4, locals.var_fn61_calc_iq__ffd0_dn7, locals.var_fn61_calc_iq__ffd0_dn15, locals.var_fn61_calc_iq__ffd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd0 = assign6770_e8202;
        locals.var_fn61_calc_iq__ffd0_dn2 = assign6770_e8202_d_n2;
        locals.var_fn61_calc_iq__ffd0_dn4 = assign6770_e8202_d_n4;
        locals.var_fn61_calc_iq__ffd0_dn7 = assign6770_e8202_d_n7;
        locals.var_fn61_calc_iq__ffd0_dn15 = assign6770_e8202_d_n15;
        locals.var_fn61_calc_iq__ffd0_dn16 = assign6770_e8202_d_n16;
        locals.var_fn61_calc_iq__ffd0_rv = 0.0;

        let assign6780_e8205: f64 = (-50.0);
        let assign6780_e8206: f64 = if locals.var_fn61_calc_iq__exparg0 < assign6780_e8205 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign6780_e8206;
        locals.var_guard84_rv = 0.0;

        let (assign6790_e8215, assign6790_e8215_d_n2, assign6790_e8215_d_n4, assign6790_e8215_d_n7, assign6790_e8215_d_n15, assign6790_e8215_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard83 == 0.0)) && (locals.var_guard84 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__ffd0, locals.var_fn61_calc_iq__ffd0_dn2, locals.var_fn61_calc_iq__ffd0_dn4, locals.var_fn61_calc_iq__ffd0_dn7, locals.var_fn61_calc_iq__ffd0_dn15, locals.var_fn61_calc_iq__ffd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd0 = assign6790_e8215;
        locals.var_fn61_calc_iq__ffd0_dn2 = assign6790_e8215_d_n2;
        locals.var_fn61_calc_iq__ffd0_dn4 = assign6790_e8215_d_n4;
        locals.var_fn61_calc_iq__ffd0_dn7 = assign6790_e8215_d_n7;
        locals.var_fn61_calc_iq__ffd0_dn15 = assign6790_e8215_d_n15;
        locals.var_fn61_calc_iq__ffd0_dn16 = assign6790_e8215_d_n16;
        locals.var_fn61_calc_iq__ffd0_rv = 0.0;

        let (assign6800_e8230, assign6800_e8230_d_n2, assign6800_e8230_d_n4, assign6800_e8230_d_n7, assign6800_e8230_d_n15, assign6800_e8230_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard83 == 0.0)) && (locals.var_guard84 == 0.0)) {
        let assign6800_e8226: f64 = (locals.var_fn61_calc_iq__exparg0).exp();
        let assign6800_e8227: f64 = (1.0 + assign6800_e8226);
        let assign6800_e8228: f64 = (1.0 / assign6800_e8227);
        (assign6800_e8228, (-((assign6800_e8226 * locals.var_fn61_calc_iq__exparg0_dn2) / (assign6800_e8227 * assign6800_e8227))), (-((assign6800_e8226 * locals.var_fn61_calc_iq__exparg0_dn4) / (assign6800_e8227 * assign6800_e8227))), (-((assign6800_e8226 * locals.var_fn61_calc_iq__exparg0_dn7) / (assign6800_e8227 * assign6800_e8227))), (-((assign6800_e8226 * locals.var_fn61_calc_iq__exparg0_dn15) / (assign6800_e8227 * assign6800_e8227))), (-((assign6800_e8226 * locals.var_fn61_calc_iq__exparg0_dn16) / (assign6800_e8227 * assign6800_e8227))),)
    } else {
        (locals.var_fn61_calc_iq__ffd0, locals.var_fn61_calc_iq__ffd0_dn2, locals.var_fn61_calc_iq__ffd0_dn4, locals.var_fn61_calc_iq__ffd0_dn7, locals.var_fn61_calc_iq__ffd0_dn15, locals.var_fn61_calc_iq__ffd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__ffd0 = assign6800_e8230;
        locals.var_fn61_calc_iq__ffd0_dn2 = assign6800_e8230_d_n2;
        locals.var_fn61_calc_iq__ffd0_dn4 = assign6800_e8230_d_n4;
        locals.var_fn61_calc_iq__ffd0_dn7 = assign6800_e8230_d_n7;
        locals.var_fn61_calc_iq__ffd0_dn15 = assign6800_e8230_d_n15;
        locals.var_fn61_calc_iq__ffd0_dn16 = assign6800_e8230_d_n16;
        locals.var_fn61_calc_iq__ffd0_rv = 0.0;

        let (assign6810_e8248, assign6810_e8248_d_n2, assign6810_e8248_d_n4, assign6810_e8248_d_n7, assign6810_e8248_d_n15, assign6810_e8248_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6810_e8234: f64 = (locals.var_fn61_calc_iq__vgsin - locals.var_fn61_calc_iq__vdx0);
        let assign6810_e8238: f64 = (p.p51 * 0.1);
        let assign6810_e8240: f64 = (assign6810_e8238 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6810_e8242: f64 = (assign6810_e8240 * locals.var_fn61_calc_iq__ffd0);
        let assign6810_e8243: f64 = (locals.var_fn61_calc_iq__vtof - assign6810_e8242);
        let assign6810_e8244: f64 = (assign6810_e8234 - assign6810_e8243);
        let assign6810_e8246: f64 = (assign6810_e8244 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign6810_e8246, (((locals.var_fn61_calc_iq__vgsin_dn2 - locals.var_fn61_calc_iq__vdx0_dn2) - (-(assign6810_e8240 * locals.var_fn61_calc_iq__ffd0_dn2))) / locals.var_fn61_calc_iq__two_n_phit0), (((((-locals.var_fn61_calc_iq__vdx0_dn4) - (locals.var_fn61_calc_iq__vtof_dn4 - (((assign6810_e8238 * locals.var_fn61_calc_iq__alpha_phit_dn4) * locals.var_fn61_calc_iq__ffd0) + (assign6810_e8240 * locals.var_fn61_calc_iq__ffd0_dn4)))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign6810_e8244 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (((locals.var_fn61_calc_iq__vgsin_dn7 - locals.var_fn61_calc_iq__vdx0_dn7) - (-(assign6810_e8240 * locals.var_fn61_calc_iq__ffd0_dn7))) / locals.var_fn61_calc_iq__two_n_phit0), (((locals.var_fn61_calc_iq__vgsin_dn15 - locals.var_fn61_calc_iq__vdx0_dn15) - (-(assign6810_e8240 * locals.var_fn61_calc_iq__ffd0_dn15))) / locals.var_fn61_calc_iq__two_n_phit0), (((-locals.var_fn61_calc_iq__vdx0_dn16) - (-(assign6810_e8240 * locals.var_fn61_calc_iq__ffd0_dn16))) / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etad0, locals.var_fn61_calc_iq__etad0_dn2, locals.var_fn61_calc_iq__etad0_dn4, locals.var_fn61_calc_iq__etad0_dn7, locals.var_fn61_calc_iq__etad0_dn15, locals.var_fn61_calc_iq__etad0_dn16,)
    }
};
        locals.var_fn61_calc_iq__etad0 = assign6810_e8248;
        locals.var_fn61_calc_iq__etad0_dn2 = assign6810_e8248_d_n2;
        locals.var_fn61_calc_iq__etad0_dn4 = assign6810_e8248_d_n4;
        locals.var_fn61_calc_iq__etad0_dn7 = assign6810_e8248_d_n7;
        locals.var_fn61_calc_iq__etad0_dn15 = assign6810_e8248_d_n15;
        locals.var_fn61_calc_iq__etad0_dn16 = assign6810_e8248_d_n16;
        locals.var_fn61_calc_iq__etad0_rv = 0.0;

        let assign6820_e8251: f64 = if locals.var_fn61_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign6820_e8251;
        locals.var_guard85_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6830_e8259, assign6830_e8259_d_n2, assign6830_e8259_d_n4, assign6830_e8259_d_n7, assign6830_e8259_d_n15, assign6830_e8259_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign6830_e8257: f64 = (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etad0);
        (assign6830_e8257, (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etad0_dn2), ((locals.var_fn61_calc_iq__qref0_dn4 * locals.var_fn61_calc_iq__etad0) + (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etad0_dn4)), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etad0_dn7), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etad0_dn15), (locals.var_fn61_calc_iq__qref0 * locals.var_fn61_calc_iq__etad0_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qinvd0, locals.var_fn61_calc_iq__qinvd0_dn2, locals.var_fn61_calc_iq__qinvd0_dn4, locals.var_fn61_calc_iq__qinvd0_dn7, locals.var_fn61_calc_iq__qinvd0_dn15, locals.var_fn61_calc_iq__qinvd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd0 = assign6830_e8259;
        locals.var_fn61_calc_iq__qinvd0_dn2 = assign6830_e8259_d_n2;
        locals.var_fn61_calc_iq__qinvd0_dn4 = assign6830_e8259_d_n4;
        locals.var_fn61_calc_iq__qinvd0_dn7 = assign6830_e8259_d_n7;
        locals.var_fn61_calc_iq__qinvd0_dn15 = assign6830_e8259_d_n15;
        locals.var_fn61_calc_iq__qinvd0_dn16 = assign6830_e8259_d_n16;
        locals.var_fn61_calc_iq__qinvd0_rv = 0.0;

        let assign6840_e8262: f64 = (-50.0);
        let assign6840_e8263: f64 = if locals.var_fn61_calc_iq__etad0 < assign6840_e8262 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign6840_e8263;
        locals.var_guard86_rv = 0.0;

        let (assign6850_e8275, assign6850_e8275_d_n2, assign6850_e8275_d_n4, assign6850_e8275_d_n7, assign6850_e8275_d_n15, assign6850_e8275_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard85 == 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign6850_e8272: f64 = (locals.var_fn61_calc_iq__etad0).exp();
        let assign6850_e8273: f64 = (locals.var_fn61_calc_iq__qref0 * assign6850_e8272);
        (assign6850_e8273, (locals.var_fn61_calc_iq__qref0 * (assign6850_e8272 * locals.var_fn61_calc_iq__etad0_dn2)), ((locals.var_fn61_calc_iq__qref0_dn4 * assign6850_e8272) + (locals.var_fn61_calc_iq__qref0 * (assign6850_e8272 * locals.var_fn61_calc_iq__etad0_dn4))), (locals.var_fn61_calc_iq__qref0 * (assign6850_e8272 * locals.var_fn61_calc_iq__etad0_dn7)), (locals.var_fn61_calc_iq__qref0 * (assign6850_e8272 * locals.var_fn61_calc_iq__etad0_dn15)), (locals.var_fn61_calc_iq__qref0 * (assign6850_e8272 * locals.var_fn61_calc_iq__etad0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qinvd0, locals.var_fn61_calc_iq__qinvd0_dn2, locals.var_fn61_calc_iq__qinvd0_dn4, locals.var_fn61_calc_iq__qinvd0_dn7, locals.var_fn61_calc_iq__qinvd0_dn15, locals.var_fn61_calc_iq__qinvd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd0 = assign6850_e8275;
        locals.var_fn61_calc_iq__qinvd0_dn2 = assign6850_e8275_d_n2;
        locals.var_fn61_calc_iq__qinvd0_dn4 = assign6850_e8275_d_n4;
        locals.var_fn61_calc_iq__qinvd0_dn7 = assign6850_e8275_d_n7;
        locals.var_fn61_calc_iq__qinvd0_dn15 = assign6850_e8275_d_n15;
        locals.var_fn61_calc_iq__qinvd0_dn16 = assign6850_e8275_d_n16;
        locals.var_fn61_calc_iq__qinvd0_rv = 0.0;

        let (assign6860_e8291, assign6860_e8291_d_n2, assign6860_e8291_d_n4, assign6860_e8291_d_n7, assign6860_e8291_d_n15, assign6860_e8291_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard85 == 0.0)) && (locals.var_guard86 == 0.0)) {
        let assign6860_e8286: f64 = (locals.var_fn61_calc_iq__etad0).exp();
        let assign6860_e8287: f64 = (1.0 + assign6860_e8286);
        let assign6860_e8288: f64 = (assign6860_e8287).ln();
        let assign6860_e8289: f64 = (locals.var_fn61_calc_iq__qref0 * assign6860_e8288);
        (assign6860_e8289, (locals.var_fn61_calc_iq__qref0 * ((assign6860_e8286 * locals.var_fn61_calc_iq__etad0_dn2) / assign6860_e8287)), ((locals.var_fn61_calc_iq__qref0_dn4 * assign6860_e8288) + (locals.var_fn61_calc_iq__qref0 * ((assign6860_e8286 * locals.var_fn61_calc_iq__etad0_dn4) / assign6860_e8287))), (locals.var_fn61_calc_iq__qref0 * ((assign6860_e8286 * locals.var_fn61_calc_iq__etad0_dn7) / assign6860_e8287)), (locals.var_fn61_calc_iq__qref0 * ((assign6860_e8286 * locals.var_fn61_calc_iq__etad0_dn15) / assign6860_e8287)), (locals.var_fn61_calc_iq__qref0 * ((assign6860_e8286 * locals.var_fn61_calc_iq__etad0_dn16) / assign6860_e8287)),)
    } else {
        (locals.var_fn61_calc_iq__qinvd0, locals.var_fn61_calc_iq__qinvd0_dn2, locals.var_fn61_calc_iq__qinvd0_dn4, locals.var_fn61_calc_iq__qinvd0_dn7, locals.var_fn61_calc_iq__qinvd0_dn15, locals.var_fn61_calc_iq__qinvd0_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvd0 = assign6860_e8291;
        locals.var_fn61_calc_iq__qinvd0_dn2 = assign6860_e8291_d_n2;
        locals.var_fn61_calc_iq__qinvd0_dn4 = assign6860_e8291_d_n4;
        locals.var_fn61_calc_iq__qinvd0_dn7 = assign6860_e8291_d_n7;
        locals.var_fn61_calc_iq__qinvd0_dn15 = assign6860_e8291_d_n15;
        locals.var_fn61_calc_iq__qinvd0_dn16 = assign6860_e8291_d_n16;
        locals.var_fn61_calc_iq__qinvd0_rv = 0.0;

        let (assign6870_e8299, assign6870_e8299_d_n2, assign6870_e8299_d_n4, assign6870_e8299_d_n7, assign6870_e8299_d_n15, assign6870_e8299_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6870_e8295: f64 = (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvs0);
        let assign6870_e8297: f64 = (assign6870_e8295 + 1e-38);
        (assign6870_e8297, ((locals.var_fn61_calc_iq__qinvs0_dn2 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvs0_dn2)), ((locals.var_fn61_calc_iq__qinvs0_dn4 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvs0_dn4)), ((locals.var_fn61_calc_iq__qinvs0_dn7 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvs0_dn7)), ((locals.var_fn61_calc_iq__qinvs0_dn15 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvs0_dn15)), ((locals.var_fn61_calc_iq__qinvs0_dn16 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvs0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qs2, locals.var_fn61_calc_iq__qs2_dn2, locals.var_fn61_calc_iq__qs2_dn4, locals.var_fn61_calc_iq__qs2_dn7, locals.var_fn61_calc_iq__qs2_dn15, locals.var_fn61_calc_iq__qs2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs2 = assign6870_e8299;
        locals.var_fn61_calc_iq__qs2_dn2 = assign6870_e8299_d_n2;
        locals.var_fn61_calc_iq__qs2_dn4 = assign6870_e8299_d_n4;
        locals.var_fn61_calc_iq__qs2_dn7 = assign6870_e8299_d_n7;
        locals.var_fn61_calc_iq__qs2_dn15 = assign6870_e8299_d_n15;
        locals.var_fn61_calc_iq__qs2_dn16 = assign6870_e8299_d_n16;
        locals.var_fn61_calc_iq__qs2_rv = 0.0;

        let (assign6880_e8307, assign6880_e8307_d_n2, assign6880_e8307_d_n4, assign6880_e8307_d_n7, assign6880_e8307_d_n15, assign6880_e8307_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6880_e8303: f64 = (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0);
        let assign6880_e8305: f64 = (assign6880_e8303 + 1e-57);
        (assign6880_e8305, ((locals.var_fn61_calc_iq__qs2_dn2 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn2)), ((locals.var_fn61_calc_iq__qs2_dn4 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn4)), ((locals.var_fn61_calc_iq__qs2_dn7 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn7)), ((locals.var_fn61_calc_iq__qs2_dn15 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn15)), ((locals.var_fn61_calc_iq__qs2_dn16 * locals.var_fn61_calc_iq__qinvs0) + (locals.var_fn61_calc_iq__qs2 * locals.var_fn61_calc_iq__qinvs0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qs3, locals.var_fn61_calc_iq__qs3_dn2, locals.var_fn61_calc_iq__qs3_dn4, locals.var_fn61_calc_iq__qs3_dn7, locals.var_fn61_calc_iq__qs3_dn15, locals.var_fn61_calc_iq__qs3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs3 = assign6880_e8307;
        locals.var_fn61_calc_iq__qs3_dn2 = assign6880_e8307_d_n2;
        locals.var_fn61_calc_iq__qs3_dn4 = assign6880_e8307_d_n4;
        locals.var_fn61_calc_iq__qs3_dn7 = assign6880_e8307_d_n7;
        locals.var_fn61_calc_iq__qs3_dn15 = assign6880_e8307_d_n15;
        locals.var_fn61_calc_iq__qs3_dn16 = assign6880_e8307_d_n16;
        locals.var_fn61_calc_iq__qs3_rv = 0.0;

        let (assign6890_e8315, assign6890_e8315_d_n2, assign6890_e8315_d_n4, assign6890_e8315_d_n7, assign6890_e8315_d_n15, assign6890_e8315_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6890_e8311: f64 = (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0);
        let assign6890_e8313: f64 = (assign6890_e8311 + 1e-38);
        (assign6890_e8313, ((locals.var_fn61_calc_iq__qinvd0_dn2 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn2)), ((locals.var_fn61_calc_iq__qinvd0_dn4 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn4)), ((locals.var_fn61_calc_iq__qinvd0_dn7 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn7)), ((locals.var_fn61_calc_iq__qinvd0_dn15 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn15)), ((locals.var_fn61_calc_iq__qinvd0_dn16 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvd0 * locals.var_fn61_calc_iq__qinvd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qd2, locals.var_fn61_calc_iq__qd2_dn2, locals.var_fn61_calc_iq__qd2_dn4, locals.var_fn61_calc_iq__qd2_dn7, locals.var_fn61_calc_iq__qd2_dn15, locals.var_fn61_calc_iq__qd2_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd2 = assign6890_e8315;
        locals.var_fn61_calc_iq__qd2_dn2 = assign6890_e8315_d_n2;
        locals.var_fn61_calc_iq__qd2_dn4 = assign6890_e8315_d_n4;
        locals.var_fn61_calc_iq__qd2_dn7 = assign6890_e8315_d_n7;
        locals.var_fn61_calc_iq__qd2_dn15 = assign6890_e8315_d_n15;
        locals.var_fn61_calc_iq__qd2_dn16 = assign6890_e8315_d_n16;
        locals.var_fn61_calc_iq__qd2_rv = 0.0;

        let (assign6900_e8323, assign6900_e8323_d_n2, assign6900_e8323_d_n4, assign6900_e8323_d_n7, assign6900_e8323_d_n15, assign6900_e8323_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6900_e8319: f64 = (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0);
        let assign6900_e8321: f64 = (assign6900_e8319 + 1e-57);
        (assign6900_e8321, ((locals.var_fn61_calc_iq__qd2_dn2 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn2)), ((locals.var_fn61_calc_iq__qd2_dn4 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn4)), ((locals.var_fn61_calc_iq__qd2_dn7 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn7)), ((locals.var_fn61_calc_iq__qd2_dn15 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn15)), ((locals.var_fn61_calc_iq__qd2_dn16 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qd2 * locals.var_fn61_calc_iq__qinvd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qd3, locals.var_fn61_calc_iq__qd3_dn2, locals.var_fn61_calc_iq__qd3_dn4, locals.var_fn61_calc_iq__qd3_dn7, locals.var_fn61_calc_iq__qd3_dn15, locals.var_fn61_calc_iq__qd3_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd3 = assign6900_e8323;
        locals.var_fn61_calc_iq__qd3_dn2 = assign6900_e8323_d_n2;
        locals.var_fn61_calc_iq__qd3_dn4 = assign6900_e8323_d_n4;
        locals.var_fn61_calc_iq__qd3_dn7 = assign6900_e8323_d_n7;
        locals.var_fn61_calc_iq__qd3_dn15 = assign6900_e8323_d_n15;
        locals.var_fn61_calc_iq__qd3_dn16 = assign6900_e8323_d_n16;
        locals.var_fn61_calc_iq__qd3_rv = 0.0;

        let (assign6910_e8331, assign6910_e8331_d_n2, assign6910_e8331_d_n4, assign6910_e8331_d_n7, assign6910_e8331_d_n15, assign6910_e8331_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6910_e8327: f64 = (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0);
        let assign6910_e8329: f64 = (assign6910_e8327 + 1e-38);
        (assign6910_e8329, ((locals.var_fn61_calc_iq__qinvs0_dn2 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn2)), ((locals.var_fn61_calc_iq__qinvs0_dn4 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn4)), ((locals.var_fn61_calc_iq__qinvs0_dn7 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn7)), ((locals.var_fn61_calc_iq__qinvs0_dn15 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn15)), ((locals.var_fn61_calc_iq__qinvs0_dn16 * locals.var_fn61_calc_iq__qinvd0) + (locals.var_fn61_calc_iq__qinvs0 * locals.var_fn61_calc_iq__qinvd0_dn16)),)
    } else {
        (locals.var_fn61_calc_iq__qsqd, locals.var_fn61_calc_iq__qsqd_dn2, locals.var_fn61_calc_iq__qsqd_dn4, locals.var_fn61_calc_iq__qsqd_dn7, locals.var_fn61_calc_iq__qsqd_dn15, locals.var_fn61_calc_iq__qsqd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsqd = assign6910_e8331;
        locals.var_fn61_calc_iq__qsqd_dn2 = assign6910_e8331_d_n2;
        locals.var_fn61_calc_iq__qsqd_dn4 = assign6910_e8331_d_n4;
        locals.var_fn61_calc_iq__qsqd_dn7 = assign6910_e8331_d_n7;
        locals.var_fn61_calc_iq__qsqd_dn15 = assign6910_e8331_d_n15;
        locals.var_fn61_calc_iq__qsqd_dn16 = assign6910_e8331_d_n16;
        locals.var_fn61_calc_iq__qsqd_rv = 0.0;

        let (assign6920_e8349, assign6920_e8349_d_n2, assign6920_e8349_d_n4, assign6920_e8349_d_n7, assign6920_e8349_d_n15, assign6920_e8349_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6920_e8335: f64 = (2.0 / 3.0);
        let assign6920_e8338: f64 = (locals.var_fn61_calc_iq__qs2 + locals.var_fn61_calc_iq__qd2);
        let assign6920_e8340: f64 = (assign6920_e8338 + locals.var_fn61_calc_iq__qsqd);
        let assign6920_e8341: f64 = (assign6920_e8335 * assign6920_e8340);
        let assign6920_e8344: f64 = (locals.var_fn61_calc_iq__qinvs0 + locals.var_fn61_calc_iq__qinvd0);
        let assign6920_e8346: f64 = (assign6920_e8344 + 2e-19);
        let assign6920_e8347: f64 = (assign6920_e8341 / assign6920_e8346);
        (assign6920_e8347, ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn2 + locals.var_fn61_calc_iq__qd2_dn2) + locals.var_fn61_calc_iq__qsqd_dn2)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn2 + locals.var_fn61_calc_iq__qinvd0_dn2))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn4 + locals.var_fn61_calc_iq__qd2_dn4) + locals.var_fn61_calc_iq__qsqd_dn4)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn4 + locals.var_fn61_calc_iq__qinvd0_dn4))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn7 + locals.var_fn61_calc_iq__qd2_dn7) + locals.var_fn61_calc_iq__qsqd_dn7)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn7 + locals.var_fn61_calc_iq__qinvd0_dn7))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn15 + locals.var_fn61_calc_iq__qd2_dn15) + locals.var_fn61_calc_iq__qsqd_dn15)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn15 + locals.var_fn61_calc_iq__qinvd0_dn15))) / (assign6920_e8346 * assign6920_e8346)), ((((assign6920_e8335 * ((locals.var_fn61_calc_iq__qs2_dn16 + locals.var_fn61_calc_iq__qd2_dn16) + locals.var_fn61_calc_iq__qsqd_dn16)) * assign6920_e8346) - (assign6920_e8341 * (locals.var_fn61_calc_iq__qinvs0_dn16 + locals.var_fn61_calc_iq__qinvd0_dn16))) / (assign6920_e8346 * assign6920_e8346)),)
    } else {
        (locals.var_fn61_calc_iq__qinvdd, locals.var_fn61_calc_iq__qinvdd_dn2, locals.var_fn61_calc_iq__qinvdd_dn4, locals.var_fn61_calc_iq__qinvdd_dn7, locals.var_fn61_calc_iq__qinvdd_dn15, locals.var_fn61_calc_iq__qinvdd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qinvdd = assign6920_e8349;
        locals.var_fn61_calc_iq__qinvdd_dn2 = assign6920_e8349_d_n2;
        locals.var_fn61_calc_iq__qinvdd_dn4 = assign6920_e8349_d_n4;
        locals.var_fn61_calc_iq__qinvdd_dn7 = assign6920_e8349_d_n7;
        locals.var_fn61_calc_iq__qinvdd_dn15 = assign6920_e8349_d_n15;
        locals.var_fn61_calc_iq__qinvdd_dn16 = assign6920_e8349_d_n16;
        locals.var_fn61_calc_iq__qinvdd_rv = 0.0;

        let (assign6930_e8383, assign6930_e8383_d_n2, assign6930_e8383_d_n4, assign6930_e8383_d_n7, assign6930_e8383_d_n15, assign6930_e8383_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6930_e8354: f64 = (2.0 * locals.var_fn61_calc_iq__qs3);
        let assign6930_e8357: f64 = (3.0 * locals.var_fn61_calc_iq__qd3);
        let assign6930_e8358: f64 = (assign6930_e8354 + assign6930_e8357);
        let assign6930_e8361: f64 = (4.0 * locals.var_fn61_calc_iq__qs2);
        let assign6930_e8363: f64 = (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0);
        let assign6930_e8364: f64 = (assign6930_e8358 + assign6930_e8363);
        let assign6930_e8367: f64 = (6.0 * locals.var_fn61_calc_iq__qd2);
        let assign6930_e8369: f64 = (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0);
        let assign6930_e8370: f64 = (assign6930_e8364 + assign6930_e8369);
        let assign6930_e8371: f64 = (2.0 * assign6930_e8370);
        let assign6930_e8375: f64 = (locals.var_fn61_calc_iq__qs2 + locals.var_fn61_calc_iq__qd2);
        let assign6930_e8378: f64 = (2.0 * locals.var_fn61_calc_iq__qsqd);
        let assign6930_e8379: f64 = (assign6930_e8375 + assign6930_e8378);
        let assign6930_e8380: f64 = (15.0 * assign6930_e8379);
        let assign6930_e8381: f64 = (assign6930_e8371 / assign6930_e8380);
        (assign6930_e8381, ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn2) + (3.0 * locals.var_fn61_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn2) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn2) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn2)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn2 + locals.var_fn61_calc_iq__qd2_dn2) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn2))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn4) + (3.0 * locals.var_fn61_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn4) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn4) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn4)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn4 + locals.var_fn61_calc_iq__qd2_dn4) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn4))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn7) + (3.0 * locals.var_fn61_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn7) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn7) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn7)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn7 + locals.var_fn61_calc_iq__qd2_dn7) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn7))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn15) + (3.0 * locals.var_fn61_calc_iq__qd3_dn15)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn15) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn15))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn15) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn15)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn15 + locals.var_fn61_calc_iq__qd2_dn15) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn15))))) / (assign6930_e8380 * assign6930_e8380)), ((((2.0 * ((((2.0 * locals.var_fn61_calc_iq__qs3_dn16) + (3.0 * locals.var_fn61_calc_iq__qd3_dn16)) + (((4.0 * locals.var_fn61_calc_iq__qs2_dn16) * locals.var_fn61_calc_iq__qinvd0) + (assign6930_e8361 * locals.var_fn61_calc_iq__qinvd0_dn16))) + (((6.0 * locals.var_fn61_calc_iq__qd2_dn16) * locals.var_fn61_calc_iq__qinvs0) + (assign6930_e8367 * locals.var_fn61_calc_iq__qinvs0_dn16)))) * assign6930_e8380) - (assign6930_e8371 * (15.0 * ((locals.var_fn61_calc_iq__qs2_dn16 + locals.var_fn61_calc_iq__qd2_dn16) + (2.0 * locals.var_fn61_calc_iq__qsqd_dn16))))) / (assign6930_e8380 * assign6930_e8380)),)
    } else {
        (locals.var_fn61_calc_iq__qd1, locals.var_fn61_calc_iq__qd1_dn2, locals.var_fn61_calc_iq__qd1_dn4, locals.var_fn61_calc_iq__qd1_dn7, locals.var_fn61_calc_iq__qd1_dn15, locals.var_fn61_calc_iq__qd1_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd1 = assign6930_e8383;
        locals.var_fn61_calc_iq__qd1_dn2 = assign6930_e8383_d_n2;
        locals.var_fn61_calc_iq__qd1_dn4 = assign6930_e8383_d_n4;
        locals.var_fn61_calc_iq__qd1_dn7 = assign6930_e8383_d_n7;
        locals.var_fn61_calc_iq__qd1_dn15 = assign6930_e8383_d_n15;
        locals.var_fn61_calc_iq__qd1_dn16 = assign6930_e8383_d_n16;
        locals.var_fn61_calc_iq__qd1_rv = 0.0;

        let (assign6940_e8389, assign6940_e8389_d_n2, assign6940_e8389_d_n4, assign6940_e8389_d_n7, assign6940_e8389_d_n15, assign6940_e8389_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6940_e8387: f64 = (locals.var_fn61_calc_iq__qinvdd - locals.var_fn61_calc_iq__qd1);
        (assign6940_e8387, (locals.var_fn61_calc_iq__qinvdd_dn2 - locals.var_fn61_calc_iq__qd1_dn2), (locals.var_fn61_calc_iq__qinvdd_dn4 - locals.var_fn61_calc_iq__qd1_dn4), (locals.var_fn61_calc_iq__qinvdd_dn7 - locals.var_fn61_calc_iq__qd1_dn7), (locals.var_fn61_calc_iq__qinvdd_dn15 - locals.var_fn61_calc_iq__qd1_dn15), (locals.var_fn61_calc_iq__qinvdd_dn16 - locals.var_fn61_calc_iq__qd1_dn16),)
    } else {
        (locals.var_fn61_calc_iq__qs, locals.var_fn61_calc_iq__qs_dn2, locals.var_fn61_calc_iq__qs_dn4, locals.var_fn61_calc_iq__qs_dn7, locals.var_fn61_calc_iq__qs_dn15, locals.var_fn61_calc_iq__qs_dn16,)
    }
};
        locals.var_fn61_calc_iq__qs = assign6940_e8389;
        locals.var_fn61_calc_iq__qs_dn2 = assign6940_e8389_d_n2;
        locals.var_fn61_calc_iq__qs_dn4 = assign6940_e8389_d_n4;
        locals.var_fn61_calc_iq__qs_dn7 = assign6940_e8389_d_n7;
        locals.var_fn61_calc_iq__qs_dn15 = assign6940_e8389_d_n15;
        locals.var_fn61_calc_iq__qs_dn16 = assign6940_e8389_d_n16;
        locals.var_fn61_calc_iq__qs_rv = 0.0;

        let (assign6950_e8393, assign6950_e8393_d_n2, assign6950_e8393_d_n4, assign6950_e8393_d_n7, assign6950_e8393_d_n15, assign6950_e8393_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qd1, locals.var_fn61_calc_iq__qd1_dn2, locals.var_fn61_calc_iq__qd1_dn4, locals.var_fn61_calc_iq__qd1_dn7, locals.var_fn61_calc_iq__qd1_dn15, locals.var_fn61_calc_iq__qd1_dn16,)
    } else {
        (locals.var_fn61_calc_iq__qd, locals.var_fn61_calc_iq__qd_dn2, locals.var_fn61_calc_iq__qd_dn4, locals.var_fn61_calc_iq__qd_dn7, locals.var_fn61_calc_iq__qd_dn15, locals.var_fn61_calc_iq__qd_dn16,)
    }
};
        locals.var_fn61_calc_iq__qd = assign6950_e8393;
        locals.var_fn61_calc_iq__qd_dn2 = assign6950_e8393_d_n2;
        locals.var_fn61_calc_iq__qd_dn4 = assign6950_e8393_d_n4;
        locals.var_fn61_calc_iq__qd_dn7 = assign6950_e8393_d_n7;
        locals.var_fn61_calc_iq__qd_dn15 = assign6950_e8393_d_n15;
        locals.var_fn61_calc_iq__qd_dn16 = assign6950_e8393_d_n16;
        locals.var_fn61_calc_iq__qd_rv = 0.0;

        let (assign6960_e8407, assign6960_e8407_d_n2, assign6960_e8407_d_n4, assign6960_e8407_d_n7, assign6960_e8407_d_n15, assign6960_e8407_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6960_e8397: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign6960_e8399: f64 = (assign6960_e8397 * locals.var_fn61_calc_iq__lin);
        let assign6960_e8401: f64 = (assign6960_e8399 * locals.var_fn61_calc_iq__type);
        let assign6960_e8403: f64 = (assign6960_e8401 * locals.var_fn61_calc_iq__qs);
        let assign6960_e8405: f64 = (assign6960_e8403 * locals.var_fn61_calc_iq__trapfracdl);
        (assign6960_e8405, ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn4) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign6960_e8401 * locals.var_fn61_calc_iq__qs_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qgsout, locals.var_fn61_calc_iq__qgsout_dn2, locals.var_fn61_calc_iq__qgsout_dn4, locals.var_fn61_calc_iq__qgsout_dn7, locals.var_fn61_calc_iq__qgsout_dn15, locals.var_fn61_calc_iq__qgsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgsout = assign6960_e8407;
        locals.var_fn61_calc_iq__qgsout_dn2 = assign6960_e8407_d_n2;
        locals.var_fn61_calc_iq__qgsout_dn4 = assign6960_e8407_d_n4;
        locals.var_fn61_calc_iq__qgsout_dn7 = assign6960_e8407_d_n7;
        locals.var_fn61_calc_iq__qgsout_dn15 = assign6960_e8407_d_n15;
        locals.var_fn61_calc_iq__qgsout_dn16 = assign6960_e8407_d_n16;
        locals.var_fn61_calc_iq__qgsout_rv = 0.0;

        let (assign6970_e8421, assign6970_e8421_d_n2, assign6970_e8421_d_n4, assign6970_e8421_d_n7, assign6970_e8421_d_n15, assign6970_e8421_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        let assign6970_e8411: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign6970_e8413: f64 = (assign6970_e8411 * locals.var_fn61_calc_iq__lin);
        let assign6970_e8415: f64 = (assign6970_e8413 * locals.var_fn61_calc_iq__type);
        let assign6970_e8417: f64 = (assign6970_e8415 * locals.var_fn61_calc_iq__qd);
        let assign6970_e8419: f64 = (assign6970_e8417 * locals.var_fn61_calc_iq__trapfracdl);
        (assign6970_e8419, ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn4) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign6970_e8415 * locals.var_fn61_calc_iq__qd_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qgdout, locals.var_fn61_calc_iq__qgdout_dn2, locals.var_fn61_calc_iq__qgdout_dn4, locals.var_fn61_calc_iq__qgdout_dn7, locals.var_fn61_calc_iq__qgdout_dn15, locals.var_fn61_calc_iq__qgdout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qgdout = assign6970_e8421;
        locals.var_fn61_calc_iq__qgdout_dn2 = assign6970_e8421_d_n2;
        locals.var_fn61_calc_iq__qgdout_dn4 = assign6970_e8421_d_n4;
        locals.var_fn61_calc_iq__qgdout_dn7 = assign6970_e8421_d_n7;
        locals.var_fn61_calc_iq__qgdout_dn15 = assign6970_e8421_d_n15;
        locals.var_fn61_calc_iq__qgdout_dn16 = assign6970_e8421_d_n16;
        locals.var_fn61_calc_iq__qgdout_rv = 0.0;

        let assign6980_e8424: f64 = if locals.var_fn61_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign6980_e8424;
        locals.var_guard87_rv = 0.0;

        let (assign6990_e8440, assign6990_e8440_d_n2, assign6990_e8440_d_n4, assign6990_e8440_d_n7, assign6990_e8440_d_n15,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign6990_e8432: f64 = (p.p51 * 0.5);
        let assign6990_e8434: f64 = (assign6990_e8432 * locals.var_fn61_calc_iq__alpha_phit);
        let assign6990_e8435: f64 = (locals.var_fn61_calc_iq__vtof - assign6990_e8434);
        let assign6990_e8436: f64 = (locals.var_fn61_calc_iq__vcin - assign6990_e8435);
        let assign6990_e8438: f64 = (assign6990_e8436 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign6990_e8438, (locals.var_fn61_calc_iq__vcin_dn2 / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (assign6990_e8432 * locals.var_fn61_calc_iq__alpha_phit_dn4))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign6990_e8436 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (locals.var_fn61_calc_iq__vcin_dn7 / locals.var_fn61_calc_iq__two_n_phit0), (locals.var_fn61_calc_iq__vcin_dn15 / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etac, locals.var_fn61_calc_iq__etac_dn2, locals.var_fn61_calc_iq__etac_dn4, locals.var_fn61_calc_iq__etac_dn7, locals.var_fn61_calc_iq__etac_dn15,)
    }
};
        locals.var_fn61_calc_iq__etac = assign6990_e8440;
        locals.var_fn61_calc_iq__etac_dn2 = assign6990_e8440_d_n2;
        locals.var_fn61_calc_iq__etac_dn4 = assign6990_e8440_d_n4;
        locals.var_fn61_calc_iq__etac_dn7 = assign6990_e8440_d_n7;
        locals.var_fn61_calc_iq__etac_dn15 = assign6990_e8440_d_n15;
        locals.var_fn61_calc_iq__etac_rv = 0.0;

        let assign7000_e8443: f64 = if locals.var_fn61_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7000_e8443;
        locals.var_guard88_rv = 0.0;

        let (assign7010_e8451, assign7010_e8451_d_n2, assign7010_e8451_d_n3, assign7010_e8451_d_n4, assign7010_e8451_d_n7, assign7010_e8451_d_n15, assign7010_e8451_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 != 0.0)) {
        (locals.var_fn61_calc_iq__etac, locals.var_fn61_calc_iq__etac_dn2, 0.0, locals.var_fn61_calc_iq__etac_dn4, locals.var_fn61_calc_iq__etac_dn7, locals.var_fn61_calc_iq__etac_dn15, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7010_e8451;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7010_e8451_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7010_e8451_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7010_e8451_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7010_e8451_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7010_e8451_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7010_e8451_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let assign7020_e8454: f64 = (-50.0);
        let assign7020_e8455: f64 = if locals.var_fn61_calc_iq__etac < assign7020_e8454 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7020_e8455;
        locals.var_guard89_rv = 0.0;

        let (assign7030_e8467, assign7030_e8467_d_n2, assign7030_e8467_d_n3, assign7030_e8467_d_n4, assign7030_e8467_d_n7, assign7030_e8467_d_n15, assign7030_e8467_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard89 != 0.0)) {
        let assign7030_e8465: f64 = (locals.var_fn61_calc_iq__etac).exp();
        (assign7030_e8465, (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn2), 0.0, (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn4), (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn7), (assign7030_e8465 * locals.var_fn61_calc_iq__etac_dn15), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7030_e8467;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7030_e8467_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7030_e8467_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7030_e8467_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7030_e8467_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7030_e8467_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7030_e8467_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign7040_e8483, assign7040_e8483_d_n2, assign7040_e8483_d_n3, assign7040_e8483_d_n4, assign7040_e8483_d_n7, assign7040_e8483_d_n15, assign7040_e8483_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard88 == 0.0)) && (locals.var_guard89 == 0.0)) {
        let assign7040_e8479: f64 = (locals.var_fn61_calc_iq__etac).exp();
        let assign7040_e8480: f64 = (1.0 + assign7040_e8479);
        let assign7040_e8481: f64 = (assign7040_e8480).ln();
        (assign7040_e8481, ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn2) / assign7040_e8480), 0.0, ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn4) / assign7040_e8480), ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn7) / assign7040_e8480), ((assign7040_e8479 * locals.var_fn61_calc_iq__etac_dn15) / assign7040_e8480), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7040_e8483;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7040_e8483_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7040_e8483_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7040_e8483_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7040_e8483_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7040_e8483_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7040_e8483_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign7050_e8501, assign7050_e8501_d_n2, assign7050_e8501_d_n3, assign7050_e8501_d_n4, assign7050_e8501_d_n7, assign7050_e8501_d_n15, assign7050_e8501_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7050_e8489: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign7050_e8491: f64 = (assign7050_e8489 * locals.var_fn61_calc_iq__type);
        let assign7050_e8493: f64 = (assign7050_e8491 * locals.var_fn61_calc_iq__cc);
        let assign7050_e8495: f64 = (assign7050_e8493 * locals.var_fn61_calc_iq__two_n_phit0);
        let assign7050_e8497: f64 = (assign7050_e8495 * locals.var_fn61_calc_iq__exparg);
        let assign7050_e8499: f64 = (assign7050_e8497 * locals.var_fn61_calc_iq__trapfracdl);
        (assign7050_e8499, ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn3) * locals.var_fn61_calc_iq__trapfracdl), ((((((assign7050_e8491 * locals.var_fn61_calc_iq__cc_dn4) * locals.var_fn61_calc_iq__two_n_phit0) + (assign7050_e8493 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) * locals.var_fn61_calc_iq__exparg) + (assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn4)) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign7050_e8495 * locals.var_fn61_calc_iq__exparg_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qcout = assign7050_e8501;
        locals.var_fn61_calc_iq__qcout_dn2 = assign7050_e8501_d_n2;
        locals.var_fn61_calc_iq__qcout_dn3 = assign7050_e8501_d_n3;
        locals.var_fn61_calc_iq__qcout_dn4 = assign7050_e8501_d_n4;
        locals.var_fn61_calc_iq__qcout_dn7 = assign7050_e8501_d_n7;
        locals.var_fn61_calc_iq__qcout_dn15 = assign7050_e8501_d_n15;
        locals.var_fn61_calc_iq__qcout_dn16 = assign7050_e8501_d_n16;
        locals.var_fn61_calc_iq__qcout_rv = 0.0;

        let (assign7060_e8517, assign7060_e8517_d_n3, assign7060_e8517_d_n4, assign7060_e8517_d_n15,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7060_e8509: f64 = (p.p51 * 0.5);
        let assign7060_e8511: f64 = (assign7060_e8509 * locals.var_fn61_calc_iq__alpha_phit);
        let assign7060_e8512: f64 = (locals.var_fn61_calc_iq__vtof - assign7060_e8511);
        let assign7060_e8513: f64 = (locals.var_fn61_calc_iq__vbin - assign7060_e8512);
        let assign7060_e8515: f64 = (assign7060_e8513 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign7060_e8515, (locals.var_fn61_calc_iq__vbin_dn3 / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (assign7060_e8509 * locals.var_fn61_calc_iq__alpha_phit_dn4))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign7060_e8513 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (locals.var_fn61_calc_iq__vbin_dn15 / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etab, locals.var_fn61_calc_iq__etab_dn3, locals.var_fn61_calc_iq__etab_dn4, locals.var_fn61_calc_iq__etab_dn15,)
    }
};
        locals.var_fn61_calc_iq__etab = assign7060_e8517;
        locals.var_fn61_calc_iq__etab_dn3 = assign7060_e8517_d_n3;
        locals.var_fn61_calc_iq__etab_dn4 = assign7060_e8517_d_n4;
        locals.var_fn61_calc_iq__etab_dn15 = assign7060_e8517_d_n15;
        locals.var_fn61_calc_iq__etab_rv = 0.0;

        let assign7070_e8520: f64 = if locals.var_fn61_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7070_e8520;
        locals.var_guard90_rv = 0.0;

        let (assign7080_e8528, assign7080_e8528_d_n2, assign7080_e8528_d_n3, assign7080_e8528_d_n4, assign7080_e8528_d_n7, assign7080_e8528_d_n15, assign7080_e8528_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard90 != 0.0)) {
        (locals.var_fn61_calc_iq__etab, 0.0, locals.var_fn61_calc_iq__etab_dn3, locals.var_fn61_calc_iq__etab_dn4, 0.0, locals.var_fn61_calc_iq__etab_dn15, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7080_e8528;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7080_e8528_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7080_e8528_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7080_e8528_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7080_e8528_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7080_e8528_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7080_e8528_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let assign7090_e8531: f64 = (-50.0);
        let assign7090_e8532: f64 = if locals.var_fn61_calc_iq__etab < assign7090_e8531 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7090_e8532;
        locals.var_guard91_rv = 0.0;

        let (assign7100_e8544, assign7100_e8544_d_n2, assign7100_e8544_d_n3, assign7100_e8544_d_n4, assign7100_e8544_d_n7, assign7100_e8544_d_n15, assign7100_e8544_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard91 != 0.0)) {
        let assign7100_e8542: f64 = (locals.var_fn61_calc_iq__etab).exp();
        (assign7100_e8542, 0.0, (assign7100_e8542 * locals.var_fn61_calc_iq__etab_dn3), (assign7100_e8542 * locals.var_fn61_calc_iq__etab_dn4), 0.0, (assign7100_e8542 * locals.var_fn61_calc_iq__etab_dn15), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7100_e8544;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7100_e8544_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7100_e8544_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7100_e8544_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7100_e8544_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7100_e8544_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7100_e8544_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign7110_e8560, assign7110_e8560_d_n2, assign7110_e8560_d_n3, assign7110_e8560_d_n4, assign7110_e8560_d_n7, assign7110_e8560_d_n15, assign7110_e8560_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) && (locals.var_guard90 == 0.0)) && (locals.var_guard91 == 0.0)) {
        let assign7110_e8556: f64 = (locals.var_fn61_calc_iq__etab).exp();
        let assign7110_e8557: f64 = (1.0 + assign7110_e8556);
        let assign7110_e8558: f64 = (assign7110_e8557).ln();
        (assign7110_e8558, 0.0, ((assign7110_e8556 * locals.var_fn61_calc_iq__etab_dn3) / assign7110_e8557), ((assign7110_e8556 * locals.var_fn61_calc_iq__etab_dn4) / assign7110_e8557), 0.0, ((assign7110_e8556 * locals.var_fn61_calc_iq__etab_dn15) / assign7110_e8557), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7110_e8560;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7110_e8560_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7110_e8560_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7110_e8560_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7110_e8560_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7110_e8560_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7110_e8560_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign7120_e8578, assign7120_e8578_d_n2, assign7120_e8578_d_n3, assign7120_e8578_d_n4, assign7120_e8578_d_n7, assign7120_e8578_d_n15, assign7120_e8578_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7120_e8566: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign7120_e8568: f64 = (assign7120_e8566 * locals.var_fn61_calc_iq__type);
        let assign7120_e8570: f64 = (assign7120_e8568 * locals.var_fn61_calc_iq__cb);
        let assign7120_e8572: f64 = (assign7120_e8570 * locals.var_fn61_calc_iq__two_n_phit0);
        let assign7120_e8574: f64 = (assign7120_e8572 * locals.var_fn61_calc_iq__exparg);
        let assign7120_e8576: f64 = (assign7120_e8574 * locals.var_fn61_calc_iq__trapfracdl);
        (assign7120_e8576, ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn3) * locals.var_fn61_calc_iq__trapfracdl), ((((((assign7120_e8568 * locals.var_fn61_calc_iq__cb_dn4) * locals.var_fn61_calc_iq__two_n_phit0) + (assign7120_e8570 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) * locals.var_fn61_calc_iq__exparg) + (assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn4)) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign7120_e8572 * locals.var_fn61_calc_iq__exparg_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qbout = assign7120_e8578;
        locals.var_fn61_calc_iq__qbout_dn2 = assign7120_e8578_d_n2;
        locals.var_fn61_calc_iq__qbout_dn3 = assign7120_e8578_d_n3;
        locals.var_fn61_calc_iq__qbout_dn4 = assign7120_e8578_d_n4;
        locals.var_fn61_calc_iq__qbout_dn7 = assign7120_e8578_d_n7;
        locals.var_fn61_calc_iq__qbout_dn15 = assign7120_e8578_d_n15;
        locals.var_fn61_calc_iq__qbout_dn16 = assign7120_e8578_d_n16;
        locals.var_fn61_calc_iq__qbout_rv = 0.0;

        let (assign7130_e8585, assign7130_e8585_d_n2, assign7130_e8585_d_n3, assign7130_e8585_d_n4, assign7130_e8585_d_n7, assign7130_e8585_d_n15, assign7130_e8585_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qcout = assign7130_e8585;
        locals.var_fn61_calc_iq__qcout_dn2 = assign7130_e8585_d_n2;
        locals.var_fn61_calc_iq__qcout_dn3 = assign7130_e8585_d_n3;
        locals.var_fn61_calc_iq__qcout_dn4 = assign7130_e8585_d_n4;
        locals.var_fn61_calc_iq__qcout_dn7 = assign7130_e8585_d_n7;
        locals.var_fn61_calc_iq__qcout_dn15 = assign7130_e8585_d_n15;
        locals.var_fn61_calc_iq__qcout_dn16 = assign7130_e8585_d_n16;
        locals.var_fn61_calc_iq__qcout_rv = 0.0;

        let (assign7140_e8592, assign7140_e8592_d_n2, assign7140_e8592_d_n3, assign7140_e8592_d_n4, assign7140_e8592_d_n7, assign7140_e8592_d_n15, assign7140_e8592_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard87 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qbout = assign7140_e8592;
        locals.var_fn61_calc_iq__qbout_dn2 = assign7140_e8592_d_n2;
        locals.var_fn61_calc_iq__qbout_dn3 = assign7140_e8592_d_n3;
        locals.var_fn61_calc_iq__qbout_dn4 = assign7140_e8592_d_n4;
        locals.var_fn61_calc_iq__qbout_dn7 = assign7140_e8592_d_n7;
        locals.var_fn61_calc_iq__qbout_dn15 = assign7140_e8592_d_n15;
        locals.var_fn61_calc_iq__qbout_dn16 = assign7140_e8592_d_n16;
        locals.var_fn61_calc_iq__qbout_rv = 0.0;

        let assign7150_e8595: f64 = if locals.var_fn61_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7150_e8595;
        locals.var_guard92_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7160_e8611, assign7160_e8611_d_n2, assign7160_e8611_d_n4, assign7160_e8611_d_n7, assign7160_e8611_d_n15,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign7160_e8603: f64 = (p.p51 * 0.5);
        let assign7160_e8605: f64 = (assign7160_e8603 * locals.var_fn61_calc_iq__alpha_phit);
        let assign7160_e8606: f64 = (locals.var_fn61_calc_iq__vtof - assign7160_e8605);
        let assign7160_e8607: f64 = (locals.var_fn61_calc_iq__vgsin - assign7160_e8606);
        let assign7160_e8609: f64 = (assign7160_e8607 / locals.var_fn61_calc_iq__two_n_phit0);
        (assign7160_e8609, (locals.var_fn61_calc_iq__vgsin_dn2 / locals.var_fn61_calc_iq__two_n_phit0), ((((-(locals.var_fn61_calc_iq__vtof_dn4 - (assign7160_e8603 * locals.var_fn61_calc_iq__alpha_phit_dn4))) * locals.var_fn61_calc_iq__two_n_phit0) - (assign7160_e8607 * locals.var_fn61_calc_iq__two_n_phit0_dn4)) / (locals.var_fn61_calc_iq__two_n_phit0 * locals.var_fn61_calc_iq__two_n_phit0)), (locals.var_fn61_calc_iq__vgsin_dn7 / locals.var_fn61_calc_iq__two_n_phit0), (locals.var_fn61_calc_iq__vgsin_dn15 / locals.var_fn61_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn61_calc_iq__etags, locals.var_fn61_calc_iq__etags_dn2, locals.var_fn61_calc_iq__etags_dn4, locals.var_fn61_calc_iq__etags_dn7, locals.var_fn61_calc_iq__etags_dn15,)
    }
};
        locals.var_fn61_calc_iq__etags = assign7160_e8611;
        locals.var_fn61_calc_iq__etags_dn2 = assign7160_e8611_d_n2;
        locals.var_fn61_calc_iq__etags_dn4 = assign7160_e8611_d_n4;
        locals.var_fn61_calc_iq__etags_dn7 = assign7160_e8611_d_n7;
        locals.var_fn61_calc_iq__etags_dn15 = assign7160_e8611_d_n15;
        locals.var_fn61_calc_iq__etags_rv = 0.0;

        let assign7170_e8614: f64 = if locals.var_fn61_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign7170_e8614;
        locals.var_guard93_rv = 0.0;

        let (assign7180_e8622, assign7180_e8622_d_n2, assign7180_e8622_d_n3, assign7180_e8622_d_n4, assign7180_e8622_d_n7, assign7180_e8622_d_n15, assign7180_e8622_d_n16,) = {
    if (((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) && (locals.var_guard93 != 0.0)) {
        (locals.var_fn61_calc_iq__etags, locals.var_fn61_calc_iq__etags_dn2, 0.0, locals.var_fn61_calc_iq__etags_dn4, locals.var_fn61_calc_iq__etags_dn7, locals.var_fn61_calc_iq__etags_dn15, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7180_e8622;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7180_e8622_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7180_e8622_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7180_e8622_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7180_e8622_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7180_e8622_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7180_e8622_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let assign7190_e8625: f64 = (-50.0);
        let assign7190_e8626: f64 = if locals.var_fn61_calc_iq__etags < assign7190_e8625 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign7190_e8626;
        locals.var_guard94_rv = 0.0;

        let (assign7200_e8638, assign7200_e8638_d_n2, assign7200_e8638_d_n3, assign7200_e8638_d_n4, assign7200_e8638_d_n7, assign7200_e8638_d_n15, assign7200_e8638_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) && (locals.var_guard93 == 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign7200_e8636: f64 = (locals.var_fn61_calc_iq__etags).exp();
        (assign7200_e8636, (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn2), 0.0, (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn4), (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn7), (assign7200_e8636 * locals.var_fn61_calc_iq__etags_dn15), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7200_e8638;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7200_e8638_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7200_e8638_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7200_e8638_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7200_e8638_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7200_e8638_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7200_e8638_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign7210_e8654, assign7210_e8654_d_n2, assign7210_e8654_d_n3, assign7210_e8654_d_n4, assign7210_e8654_d_n7, assign7210_e8654_d_n15, assign7210_e8654_d_n16,) = {
    if ((((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) && (locals.var_guard93 == 0.0)) && (locals.var_guard94 == 0.0)) {
        let assign7210_e8650: f64 = (locals.var_fn61_calc_iq__etags).exp();
        let assign7210_e8651: f64 = (1.0 + assign7210_e8650);
        let assign7210_e8652: f64 = (assign7210_e8651).ln();
        (assign7210_e8652, ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn2) / assign7210_e8651), 0.0, ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn4) / assign7210_e8651), ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn7) / assign7210_e8651), ((assign7210_e8650 * locals.var_fn61_calc_iq__etags_dn15) / assign7210_e8651), 0.0,)
    } else {
        (locals.var_fn61_calc_iq__exparg, locals.var_fn61_calc_iq__exparg_dn2, locals.var_fn61_calc_iq__exparg_dn3, locals.var_fn61_calc_iq__exparg_dn4, locals.var_fn61_calc_iq__exparg_dn7, locals.var_fn61_calc_iq__exparg_dn15, locals.var_fn61_calc_iq__exparg_dn16,)
    }
};
        locals.var_fn61_calc_iq__exparg = assign7210_e8654;
        locals.var_fn61_calc_iq__exparg_dn2 = assign7210_e8654_d_n2;
        locals.var_fn61_calc_iq__exparg_dn3 = assign7210_e8654_d_n3;
        locals.var_fn61_calc_iq__exparg_dn4 = assign7210_e8654_d_n4;
        locals.var_fn61_calc_iq__exparg_dn7 = assign7210_e8654_d_n7;
        locals.var_fn61_calc_iq__exparg_dn15 = assign7210_e8654_d_n15;
        locals.var_fn61_calc_iq__exparg_dn16 = assign7210_e8654_d_n16;
        locals.var_fn61_calc_iq__exparg_rv = 0.0;

        let (assign7220_e8672, assign7220_e8672_d_n2, assign7220_e8672_d_n3, assign7220_e8672_d_n4, assign7220_e8672_d_n7, assign7220_e8672_d_n15, assign7220_e8672_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign7220_e8660: f64 = (locals.var_fn61_calc_iq__w * locals.var_fn61_calc_iq__ngf);
        let assign7220_e8662: f64 = (assign7220_e8660 * locals.var_fn61_calc_iq__type);
        let assign7220_e8664: f64 = (assign7220_e8662 * locals.var_fn61_calc_iq__cs);
        let assign7220_e8666: f64 = (assign7220_e8664 * locals.var_fn61_calc_iq__two_n_phit0);
        let assign7220_e8668: f64 = (assign7220_e8666 * locals.var_fn61_calc_iq__exparg);
        let assign7220_e8670: f64 = (assign7220_e8668 * locals.var_fn61_calc_iq__trapfracdl);
        (assign7220_e8670, ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn2) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn3) * locals.var_fn61_calc_iq__trapfracdl), ((((assign7220_e8664 * locals.var_fn61_calc_iq__two_n_phit0_dn4) * locals.var_fn61_calc_iq__exparg) + (assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn4)) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn7) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn15) * locals.var_fn61_calc_iq__trapfracdl), ((assign7220_e8666 * locals.var_fn61_calc_iq__exparg_dn16) * locals.var_fn61_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsout = assign7220_e8672;
        locals.var_fn61_calc_iq__qsout_dn2 = assign7220_e8672_d_n2;
        locals.var_fn61_calc_iq__qsout_dn3 = assign7220_e8672_d_n3;
        locals.var_fn61_calc_iq__qsout_dn4 = assign7220_e8672_d_n4;
        locals.var_fn61_calc_iq__qsout_dn7 = assign7220_e8672_d_n7;
        locals.var_fn61_calc_iq__qsout_dn15 = assign7220_e8672_d_n15;
        locals.var_fn61_calc_iq__qsout_dn16 = assign7220_e8672_d_n16;
        locals.var_fn61_calc_iq__qsout_rv = 0.0;

        let (assign7230_e8679, assign7230_e8679_d_n2, assign7230_e8679_d_n3, assign7230_e8679_d_n4, assign7230_e8679_d_n7, assign7230_e8679_d_n15, assign7230_e8679_d_n16,) = {
    if ((locals.var_guard60 != 0.0) && (locals.var_guard92 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    }
};
        locals.var_fn61_calc_iq__qsout = assign7230_e8679;
        locals.var_fn61_calc_iq__qsout_dn2 = assign7230_e8679_d_n2;
        locals.var_fn61_calc_iq__qsout_dn3 = assign7230_e8679_d_n3;
        locals.var_fn61_calc_iq__qsout_dn4 = assign7230_e8679_d_n4;
        locals.var_fn61_calc_iq__qsout_dn7 = assign7230_e8679_d_n7;
        locals.var_fn61_calc_iq__qsout_dn15 = assign7230_e8679_d_n15;
        locals.var_fn61_calc_iq__qsout_dn16 = assign7230_e8679_d_n16;
        locals.var_fn61_calc_iq__qsout_rv = 0.0;

        let (assign7260_e8691, assign7260_e8691_d_n2, assign7260_e8691_d_n4, assign7260_e8691_d_n7, assign7260_e8691_d_n15, assign7260_e8691_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qgsout, locals.var_fn61_calc_iq__qgsout_dn2, locals.var_fn61_calc_iq__qgsout_dn4, locals.var_fn61_calc_iq__qgsout_dn7, locals.var_fn61_calc_iq__qgsout_dn15, locals.var_fn61_calc_iq__qgsout_dn16,)
    } else {
        (locals.var_qgsfp3, locals.var_qgsfp3_dn2, locals.var_qgsfp3_dn4, locals.var_qgsfp3_dn7, locals.var_qgsfp3_dn15, locals.var_qgsfp3_dn16,)
    }
};
        locals.var_qgsfp3 = assign7260_e8691;
        locals.var_qgsfp3_dn2 = assign7260_e8691_d_n2;
        locals.var_qgsfp3_dn4 = assign7260_e8691_d_n4;
        locals.var_qgsfp3_dn7 = assign7260_e8691_d_n7;
        locals.var_qgsfp3_dn15 = assign7260_e8691_d_n15;
        locals.var_qgsfp3_dn16 = assign7260_e8691_d_n16;
        locals.var_qgsfp3_rv = 0.0;

        let (assign7270_e8695, assign7270_e8695_d_n2, assign7270_e8695_d_n4, assign7270_e8695_d_n7, assign7270_e8695_d_n15, assign7270_e8695_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qgdout, locals.var_fn61_calc_iq__qgdout_dn2, locals.var_fn61_calc_iq__qgdout_dn4, locals.var_fn61_calc_iq__qgdout_dn7, locals.var_fn61_calc_iq__qgdout_dn15, locals.var_fn61_calc_iq__qgdout_dn16,)
    } else {
        (locals.var_qgdfp3, locals.var_qgdfp3_dn2, locals.var_qgdfp3_dn4, locals.var_qgdfp3_dn7, locals.var_qgdfp3_dn15, locals.var_qgdfp3_dn16,)
    }
};
        locals.var_qgdfp3 = assign7270_e8695;
        locals.var_qgdfp3_dn2 = assign7270_e8695_d_n2;
        locals.var_qgdfp3_dn4 = assign7270_e8695_d_n4;
        locals.var_qgdfp3_dn7 = assign7270_e8695_d_n7;
        locals.var_qgdfp3_dn15 = assign7270_e8695_d_n15;
        locals.var_qgdfp3_dn16 = assign7270_e8695_d_n16;
        locals.var_qgdfp3_rv = 0.0;

        let (assign7280_e8699, assign7280_e8699_d_n2, assign7280_e8699_d_n3, assign7280_e8699_d_n4, assign7280_e8699_d_n7, assign7280_e8699_d_n15, assign7280_e8699_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qcout, locals.var_fn61_calc_iq__qcout_dn2, locals.var_fn61_calc_iq__qcout_dn3, locals.var_fn61_calc_iq__qcout_dn4, locals.var_fn61_calc_iq__qcout_dn7, locals.var_fn61_calc_iq__qcout_dn15, locals.var_fn61_calc_iq__qcout_dn16,)
    } else {
        (locals.var_qcfp3, locals.var_qcfp3_dn2, locals.var_qcfp3_dn3, locals.var_qcfp3_dn4, locals.var_qcfp3_dn7, locals.var_qcfp3_dn15, locals.var_qcfp3_dn16,)
    }
};
        locals.var_qcfp3 = assign7280_e8699;
        locals.var_qcfp3_dn2 = assign7280_e8699_d_n2;
        locals.var_qcfp3_dn3 = assign7280_e8699_d_n3;
        locals.var_qcfp3_dn4 = assign7280_e8699_d_n4;
        locals.var_qcfp3_dn7 = assign7280_e8699_d_n7;
        locals.var_qcfp3_dn15 = assign7280_e8699_d_n15;
        locals.var_qcfp3_dn16 = assign7280_e8699_d_n16;
        locals.var_qcfp3_rv = 0.0;

        let (assign7290_e8703, assign7290_e8703_d_n2, assign7290_e8703_d_n3, assign7290_e8703_d_n4, assign7290_e8703_d_n7, assign7290_e8703_d_n15, assign7290_e8703_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qbout, locals.var_fn61_calc_iq__qbout_dn2, locals.var_fn61_calc_iq__qbout_dn3, locals.var_fn61_calc_iq__qbout_dn4, locals.var_fn61_calc_iq__qbout_dn7, locals.var_fn61_calc_iq__qbout_dn15, locals.var_fn61_calc_iq__qbout_dn16,)
    } else {
        (locals.var_qbfp3, locals.var_qbfp3_dn2, locals.var_qbfp3_dn3, locals.var_qbfp3_dn4, locals.var_qbfp3_dn7, locals.var_qbfp3_dn15, locals.var_qbfp3_dn16,)
    }
};
        locals.var_qbfp3 = assign7290_e8703;
        locals.var_qbfp3_dn2 = assign7290_e8703_d_n2;
        locals.var_qbfp3_dn3 = assign7290_e8703_d_n3;
        locals.var_qbfp3_dn4 = assign7290_e8703_d_n4;
        locals.var_qbfp3_dn7 = assign7290_e8703_d_n7;
        locals.var_qbfp3_dn15 = assign7290_e8703_d_n15;
        locals.var_qbfp3_dn16 = assign7290_e8703_d_n16;
        locals.var_qbfp3_rv = 0.0;

        let (assign7300_e8707, assign7300_e8707_d_n2, assign7300_e8707_d_n3, assign7300_e8707_d_n4, assign7300_e8707_d_n7, assign7300_e8707_d_n15, assign7300_e8707_d_n16,) = {
    if (locals.var_guard60 != 0.0) {
        (locals.var_fn61_calc_iq__qsout, locals.var_fn61_calc_iq__qsout_dn2, locals.var_fn61_calc_iq__qsout_dn3, locals.var_fn61_calc_iq__qsout_dn4, locals.var_fn61_calc_iq__qsout_dn7, locals.var_fn61_calc_iq__qsout_dn15, locals.var_fn61_calc_iq__qsout_dn16,)
    } else {
        (locals.var_qsfp3, locals.var_qsfp3_dn2, locals.var_qsfp3_dn3, locals.var_qsfp3_dn4, locals.var_qsfp3_dn7, locals.var_qsfp3_dn15, locals.var_qsfp3_dn16,)
    }
};
        locals.var_qsfp3 = assign7300_e8707;
        locals.var_qsfp3_dn2 = assign7300_e8707_d_n2;
        locals.var_qsfp3_dn3 = assign7300_e8707_d_n3;
        locals.var_qsfp3_dn4 = assign7300_e8707_d_n4;
        locals.var_qsfp3_dn7 = assign7300_e8707_d_n7;
        locals.var_qsfp3_dn15 = assign7300_e8707_d_n15;
        locals.var_qsfp3_dn16 = assign7300_e8707_d_n16;
        locals.var_qsfp3_rv = 0.0;

        let assign7340_e8722: f64 = if p.p210 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign7340_e8722;
        locals.var_guard95_rv = 0.0;

        locals.var_qgsfp2 = 0.0;
        locals.var_qgsfp2_dn2 = 0.0;
        locals.var_qgsfp2_dn4 = 0.0;
        locals.var_qgsfp2_dn7 = 0.0;
        locals.var_qgsfp2_dn14 = 0.0;
        locals.var_qgsfp2_dn15 = 0.0;
        locals.var_qgsfp2_rv = 0.0;

        locals.var_qgdfp2 = 0.0;
        locals.var_qgdfp2_dn2 = 0.0;
        locals.var_qgdfp2_dn4 = 0.0;
        locals.var_qgdfp2_dn7 = 0.0;
        locals.var_qgdfp2_dn14 = 0.0;
        locals.var_qgdfp2_dn15 = 0.0;
        locals.var_qgdfp2_rv = 0.0;

        locals.var_qcfp2 = 0.0;
        locals.var_qcfp2_dn2 = 0.0;
        locals.var_qcfp2_dn3 = 0.0;
        locals.var_qcfp2_dn4 = 0.0;
        locals.var_qcfp2_dn7 = 0.0;
        locals.var_qcfp2_dn14 = 0.0;
        locals.var_qcfp2_dn15 = 0.0;
        locals.var_qcfp2_rv = 0.0;

        locals.var_qbfp2 = 0.0;
        locals.var_qbfp2_dn2 = 0.0;
        locals.var_qbfp2_dn3 = 0.0;
        locals.var_qbfp2_dn4 = 0.0;
        locals.var_qbfp2_dn7 = 0.0;
        locals.var_qbfp2_dn14 = 0.0;
        locals.var_qbfp2_dn15 = 0.0;
        locals.var_qbfp2_rv = 0.0;

        locals.var_qsfp2 = 0.0;
        locals.var_qsfp2_dn2 = 0.0;
        locals.var_qsfp2_dn3 = 0.0;
        locals.var_qsfp2_dn4 = 0.0;
        locals.var_qsfp2_dn7 = 0.0;
        locals.var_qsfp2_dn14 = 0.0;
        locals.var_qsfp2_dn15 = 0.0;
        locals.var_qsfp2_rv = 0.0;

        let assign7430_e8733: f64 = if p.p189 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign7430_e8733;
        locals.var_guard96_rv = 0.0;

        let (assign7460_e8745, assign7460_e8745_d_n2, assign7460_e8745_d_n4, assign7460_e8745_d_n7, assign7460_e8745_d_n14, assign7460_e8745_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qgsout, locals.var_fn97_calc_iq__qgsout_dn2, locals.var_fn97_calc_iq__qgsout_dn4, locals.var_fn97_calc_iq__qgsout_dn7, locals.var_fn97_calc_iq__qgsout_dn14, locals.var_fn97_calc_iq__qgsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgsout = assign7460_e8745;
        locals.var_fn97_calc_iq__qgsout_dn2 = assign7460_e8745_d_n2;
        locals.var_fn97_calc_iq__qgsout_dn4 = assign7460_e8745_d_n4;
        locals.var_fn97_calc_iq__qgsout_dn7 = assign7460_e8745_d_n7;
        locals.var_fn97_calc_iq__qgsout_dn14 = assign7460_e8745_d_n14;
        locals.var_fn97_calc_iq__qgsout_dn15 = assign7460_e8745_d_n15;
        locals.var_fn97_calc_iq__qgsout_rv = 0.0;

        let (assign7470_e8749, assign7470_e8749_d_n2, assign7470_e8749_d_n4, assign7470_e8749_d_n7, assign7470_e8749_d_n14, assign7470_e8749_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qgdout, locals.var_fn97_calc_iq__qgdout_dn2, locals.var_fn97_calc_iq__qgdout_dn4, locals.var_fn97_calc_iq__qgdout_dn7, locals.var_fn97_calc_iq__qgdout_dn14, locals.var_fn97_calc_iq__qgdout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgdout = assign7470_e8749;
        locals.var_fn97_calc_iq__qgdout_dn2 = assign7470_e8749_d_n2;
        locals.var_fn97_calc_iq__qgdout_dn4 = assign7470_e8749_d_n4;
        locals.var_fn97_calc_iq__qgdout_dn7 = assign7470_e8749_d_n7;
        locals.var_fn97_calc_iq__qgdout_dn14 = assign7470_e8749_d_n14;
        locals.var_fn97_calc_iq__qgdout_dn15 = assign7470_e8749_d_n15;
        locals.var_fn97_calc_iq__qgdout_rv = 0.0;

        let (assign7480_e8753, assign7480_e8753_d_n2, assign7480_e8753_d_n3, assign7480_e8753_d_n4, assign7480_e8753_d_n7, assign7480_e8753_d_n14, assign7480_e8753_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qcout = assign7480_e8753;
        locals.var_fn97_calc_iq__qcout_dn2 = assign7480_e8753_d_n2;
        locals.var_fn97_calc_iq__qcout_dn3 = assign7480_e8753_d_n3;
        locals.var_fn97_calc_iq__qcout_dn4 = assign7480_e8753_d_n4;
        locals.var_fn97_calc_iq__qcout_dn7 = assign7480_e8753_d_n7;
        locals.var_fn97_calc_iq__qcout_dn14 = assign7480_e8753_d_n14;
        locals.var_fn97_calc_iq__qcout_dn15 = assign7480_e8753_d_n15;
        locals.var_fn97_calc_iq__qcout_rv = 0.0;

        let (assign7490_e8757, assign7490_e8757_d_n2, assign7490_e8757_d_n3, assign7490_e8757_d_n4, assign7490_e8757_d_n7, assign7490_e8757_d_n14, assign7490_e8757_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qbout = assign7490_e8757;
        locals.var_fn97_calc_iq__qbout_dn2 = assign7490_e8757_d_n2;
        locals.var_fn97_calc_iq__qbout_dn3 = assign7490_e8757_d_n3;
        locals.var_fn97_calc_iq__qbout_dn4 = assign7490_e8757_d_n4;
        locals.var_fn97_calc_iq__qbout_dn7 = assign7490_e8757_d_n7;
        locals.var_fn97_calc_iq__qbout_dn14 = assign7490_e8757_d_n14;
        locals.var_fn97_calc_iq__qbout_dn15 = assign7490_e8757_d_n15;
        locals.var_fn97_calc_iq__qbout_rv = 0.0;

        let (assign7500_e8761, assign7500_e8761_d_n2, assign7500_e8761_d_n3, assign7500_e8761_d_n4, assign7500_e8761_d_n7, assign7500_e8761_d_n14, assign7500_e8761_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsout = assign7500_e8761;
        locals.var_fn97_calc_iq__qsout_dn2 = assign7500_e8761_d_n2;
        locals.var_fn97_calc_iq__qsout_dn3 = assign7500_e8761_d_n3;
        locals.var_fn97_calc_iq__qsout_dn4 = assign7500_e8761_d_n4;
        locals.var_fn97_calc_iq__qsout_dn7 = assign7500_e8761_d_n7;
        locals.var_fn97_calc_iq__qsout_dn14 = assign7500_e8761_d_n14;
        locals.var_fn97_calc_iq__qsout_dn15 = assign7500_e8761_d_n15;
        locals.var_fn97_calc_iq__qsout_rv = 0.0;

        let (assign7510_e8765, assign7510_e8765_d_n4, assign7510_e8765_d_n14, assign7510_e8765_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vtdibl, locals.var_fn97_calc_iq__vtdibl_dn4, locals.var_fn97_calc_iq__vtdibl_dn14, locals.var_fn97_calc_iq__vtdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vtdibl = assign7510_e8765;
        locals.var_fn97_calc_iq__vtdibl_dn4 = assign7510_e8765_d_n4;
        locals.var_fn97_calc_iq__vtdibl_dn14 = assign7510_e8765_d_n14;
        locals.var_fn97_calc_iq__vtdibl_dn15 = assign7510_e8765_d_n15;
        locals.var_fn97_calc_iq__vtdibl_rv = 0.0;

        let (assign7520_e8769, assign7520_e8769_d_n2, assign7520_e8769_d_n3, assign7520_e8769_d_n4, assign7520_e8769_d_n7, assign7520_e8769_d_n14, assign7520_e8769_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsat1, locals.var_fn97_calc_iq__vdsat1_dn2, locals.var_fn97_calc_iq__vdsat1_dn3, locals.var_fn97_calc_iq__vdsat1_dn4, locals.var_fn97_calc_iq__vdsat1_dn7, locals.var_fn97_calc_iq__vdsat1_dn14, locals.var_fn97_calc_iq__vdsat1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat1 = assign7520_e8769;
        locals.var_fn97_calc_iq__vdsat1_dn2 = assign7520_e8769_d_n2;
        locals.var_fn97_calc_iq__vdsat1_dn3 = assign7520_e8769_d_n3;
        locals.var_fn97_calc_iq__vdsat1_dn4 = assign7520_e8769_d_n4;
        locals.var_fn97_calc_iq__vdsat1_dn7 = assign7520_e8769_d_n7;
        locals.var_fn97_calc_iq__vdsat1_dn14 = assign7520_e8769_d_n14;
        locals.var_fn97_calc_iq__vdsat1_dn15 = assign7520_e8769_d_n15;
        locals.var_fn97_calc_iq__vdsat1_rv = 0.0;

        let (assign7530_e8773, assign7530_e8773_d_n2, assign7530_e8773_d_n7, assign7530_e8773_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vgsfp2, locals.var_vgsfp2_dn2, locals.var_vgsfp2_dn7, locals.var_vgsfp2_dn14,)
    } else {
        (locals.var_fn97_calc_iq__vgsin, locals.var_fn97_calc_iq__vgsin_dn2, locals.var_fn97_calc_iq__vgsin_dn7, locals.var_fn97_calc_iq__vgsin_dn14,)
    }
};
        locals.var_fn97_calc_iq__vgsin = assign7530_e8773;
        locals.var_fn97_calc_iq__vgsin_dn2 = assign7530_e8773_d_n2;
        locals.var_fn97_calc_iq__vgsin_dn7 = assign7530_e8773_d_n7;
        locals.var_fn97_calc_iq__vgsin_dn14 = assign7530_e8773_d_n14;
        locals.var_fn97_calc_iq__vgsin_rv = 0.0;

        let (assign7540_e8777, assign7540_e8777_d_n14, assign7540_e8777_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vdsfp2, locals.var_vdsfp2_dn14, locals.var_vdsfp2_dn15,)
    } else {
        (locals.var_fn97_calc_iq__vdsin, locals.var_fn97_calc_iq__vdsin_dn14, locals.var_fn97_calc_iq__vdsin_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsin = assign7540_e8777;
        locals.var_fn97_calc_iq__vdsin_dn14 = assign7540_e8777_d_n14;
        locals.var_fn97_calc_iq__vdsin_dn15 = assign7540_e8777_d_n15;
        locals.var_fn97_calc_iq__vdsin_rv = 0.0;

        let (assign7550_e8781,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p195,)
    } else {
        (locals.var_fn97_calc_iq__qcbflag,)
    }
};
        locals.var_fn97_calc_iq__qcbflag = assign7550_e8781;
        locals.var_fn97_calc_iq__qcbflag_rv = 0.0;

        let (assign7560_e8785, assign7560_e8785_d_n2, assign7560_e8785_d_n7, assign7560_e8785_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vcfp2, locals.var_vcfp2_dn2, locals.var_vcfp2_dn7, locals.var_vcfp2_dn14,)
    } else {
        (locals.var_fn97_calc_iq__vcin, locals.var_fn97_calc_iq__vcin_dn2, locals.var_fn97_calc_iq__vcin_dn7, locals.var_fn97_calc_iq__vcin_dn14,)
    }
};
        locals.var_fn97_calc_iq__vcin = assign7560_e8785;
        locals.var_fn97_calc_iq__vcin_dn2 = assign7560_e8785_d_n2;
        locals.var_fn97_calc_iq__vcin_dn7 = assign7560_e8785_d_n7;
        locals.var_fn97_calc_iq__vcin_dn14 = assign7560_e8785_d_n14;
        locals.var_fn97_calc_iq__vcin_rv = 0.0;

        let (assign7570_e8789, assign7570_e8789_d_n3, assign7570_e8789_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_vbfp2, locals.var_vbfp2_dn3, locals.var_vbfp2_dn14,)
    } else {
        (locals.var_fn97_calc_iq__vbin, locals.var_fn97_calc_iq__vbin_dn3, locals.var_fn97_calc_iq__vbin_dn14,)
    }
};
        locals.var_fn97_calc_iq__vbin = assign7570_e8789;
        locals.var_fn97_calc_iq__vbin_dn3 = assign7570_e8789_d_n3;
        locals.var_fn97_calc_iq__vbin_dn14 = assign7570_e8789_d_n14;
        locals.var_fn97_calc_iq__vbin_rv = 0.0;

        let (assign7580_e8793,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_fn97_calc_iq__qgsflag,)
    }
};
        locals.var_fn97_calc_iq__qgsflag = assign7580_e8793;
        locals.var_fn97_calc_iq__qgsflag_rv = 0.0;

        let (assign7590_e8797, assign7590_e8797_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn97_calc_iq__tambin, locals.var_fn97_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn97_calc_iq__tambin = assign7590_e8797;
        locals.var_fn97_calc_iq__tambin_dn4 = assign7590_e8797_d_n4;
        locals.var_fn97_calc_iq__tambin_rv = 0.0;

        let (assign7600_e8801,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn97_calc_iq__tnomin,)
    }
};
        locals.var_fn97_calc_iq__tnomin = assign7600_e8801;
        locals.var_fn97_calc_iq__tnomin_rv = 0.0;

        let (assign7610_e8805, assign7610_e8805_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn97_calc_iq__phitin, locals.var_fn97_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn97_calc_iq__phitin = assign7610_e8805;
        locals.var_fn97_calc_iq__phitin_dn4 = assign7610_e8805_d_n4;
        locals.var_fn97_calc_iq__phitin_rv = 0.0;

        let (assign7620_e8809,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn97_calc_iq__w,)
    }
};
        locals.var_fn97_calc_iq__w = assign7620_e8809;
        locals.var_fn97_calc_iq__w_rv = 0.0;

        let (assign7630_e8813,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p189,)
    } else {
        (locals.var_fn97_calc_iq__lin,)
    }
};
        locals.var_fn97_calc_iq__lin = assign7630_e8813;
        locals.var_fn97_calc_iq__lin_rv = 0.0;

        let (assign7640_e8817, assign7640_e8817_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_cgfp2t, locals.var_cgfp2t_dn4,)
    } else {
        (locals.var_fn97_calc_iq__cgin, locals.var_fn97_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn97_calc_iq__cgin = assign7640_e8817;
        locals.var_fn97_calc_iq__cgin_dn4 = assign7640_e8817_d_n4;
        locals.var_fn97_calc_iq__cgin_rv = 0.0;

        let (assign7650_e8821,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p194,)
    } else {
        (locals.var_fn97_calc_iq__cs,)
    }
};
        locals.var_fn97_calc_iq__cs = assign7650_e8821;
        locals.var_fn97_calc_iq__cs_rv = 0.0;

        let (assign7660_e8825, assign7660_e8825_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_ccfp2t, locals.var_ccfp2t_dn4,)
    } else {
        (locals.var_fn97_calc_iq__cc, locals.var_fn97_calc_iq__cc_dn4,)
    }
};
        locals.var_fn97_calc_iq__cc = assign7660_e8825;
        locals.var_fn97_calc_iq__cc_dn4 = assign7660_e8825_d_n4;
        locals.var_fn97_calc_iq__cc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7670_e8829, assign7670_e8829_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_cbfp2t, locals.var_cbfp2t_dn4,)
    } else {
        (locals.var_fn97_calc_iq__cb, locals.var_fn97_calc_iq__cb_dn4,)
    }
};
        locals.var_fn97_calc_iq__cb = assign7670_e8829;
        locals.var_fn97_calc_iq__cb_dn4 = assign7670_e8829_d_n4;
        locals.var_fn97_calc_iq__cb_rv = 0.0;

        let (assign7680_e8833,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p190,)
    } else {
        (locals.var_fn97_calc_iq__vto,)
    }
};
        locals.var_fn97_calc_iq__vto = assign7680_e8833;
        locals.var_fn97_calc_iq__vto_rv = 0.0;

        let (assign7690_e8837,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p204,)
    } else {
        (locals.var_fn97_calc_iq__ss,)
    }
};
        locals.var_fn97_calc_iq__ss = assign7690_e8837;
        locals.var_fn97_calc_iq__ss_rv = 0.0;

        let (assign7700_e8841,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p203,)
    } else {
        (locals.var_fn97_calc_iq__delta1,)
    }
};
        locals.var_fn97_calc_iq__delta1 = assign7700_e8841;
        locals.var_fn97_calc_iq__delta1_rv = 0.0;

        let (assign7710_e8845,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn97_calc_iq__delta2,)
    }
};
        locals.var_fn97_calc_iq__delta2 = assign7710_e8845;
        locals.var_fn97_calc_iq__delta2_rv = 0.0;

        let (assign7720_e8849,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p205,)
    } else {
        (locals.var_fn97_calc_iq__nd,)
    }
};
        locals.var_fn97_calc_iq__nd = assign7720_e8849;
        locals.var_fn97_calc_iq__nd_rv = 0.0;

        let (assign7730_e8853,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p209,)
    } else {
        (locals.var_fn97_calc_iq__alpha,)
    }
};
        locals.var_fn97_calc_iq__alpha = assign7730_e8853;
        locals.var_fn97_calc_iq__alpha_rv = 0.0;

        let (assign7740_e8857,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p200,)
    } else {
        (locals.var_fn97_calc_iq__vel0,)
    }
};
        locals.var_fn97_calc_iq__vel0 = assign7740_e8857;
        locals.var_fn97_calc_iq__vel0_rv = 0.0;

        let (assign7750_e8861,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p201,)
    } else {
        (locals.var_fn97_calc_iq__mu0,)
    }
};
        locals.var_fn97_calc_iq__mu0 = assign7750_e8861;
        locals.var_fn97_calc_iq__mu0_rv = 0.0;

        let (assign7760_e8865,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p202,)
    } else {
        (locals.var_fn97_calc_iq__beta,)
    }
};
        locals.var_fn97_calc_iq__beta = assign7760_e8865;
        locals.var_fn97_calc_iq__beta_rv = 0.0;

        let (assign7770_e8869,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p208,)
    } else {
        (locals.var_fn97_calc_iq__mtheta,)
    }
};
        locals.var_fn97_calc_iq__mtheta = assign7770_e8869;
        locals.var_fn97_calc_iq__mtheta_rv = 0.0;

        let (assign7780_e8873,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p207,)
    } else {
        (locals.var_fn97_calc_iq__vtheta,)
    }
};
        locals.var_fn97_calc_iq__vtheta = assign7780_e8873;
        locals.var_fn97_calc_iq__vtheta_rv = 0.0;

        let (assign7790_e8877,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p206,)
    } else {
        (locals.var_fn97_calc_iq__vtzeta,)
    }
};
        locals.var_fn97_calc_iq__vtzeta = assign7790_e8877;
        locals.var_fn97_calc_iq__vtzeta_rv = 0.0;

        let (assign7800_e8881,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn97_calc_iq__dibsat,)
    }
};
        locals.var_fn97_calc_iq__dibsat = assign7800_e8881;
        locals.var_fn97_calc_iq__dibsat_rv = 0.0;

        let (assign7810_e8885,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn97_calc_iq__epsilon,)
    }
};
        locals.var_fn97_calc_iq__epsilon = assign7810_e8885;
        locals.var_fn97_calc_iq__epsilon_rv = 0.0;

        let (assign7820_e8889,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn97_calc_iq__vzeta,)
    }
};
        locals.var_fn97_calc_iq__vzeta = assign7820_e8889;
        locals.var_fn97_calc_iq__vzeta_rv = 0.0;

        let (assign7830_e8893,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn97_calc_iq__lambda,)
    }
};
        locals.var_fn97_calc_iq__lambda = assign7830_e8893;
        locals.var_fn97_calc_iq__lambda_rv = 0.0;

        let (assign7840_e8897,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn97_calc_iq__ngf,)
    }
};
        locals.var_fn97_calc_iq__ngf = assign7840_e8897;
        locals.var_fn97_calc_iq__ngf_rv = 0.0;

        let (assign7850_e8901,) = {
    if (locals.var_guard96 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn97_calc_iq__type,)
    }
};
        locals.var_fn97_calc_iq__type = assign7850_e8901;
        locals.var_fn97_calc_iq__type_rv = 0.0;

        let (assign7860_e8905,) = {
    if (locals.var_guard96 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn97_calc_iq__trapfracdl,)
    }
};
        locals.var_fn97_calc_iq__trapfracdl = assign7860_e8905;
        locals.var_fn97_calc_iq__trapfracdl_rv = 0.0;

        let (assign7870_e8909, assign7870_e8909_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__alpha_phit, locals.var_fn97_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn97_calc_iq__alpha_phit = assign7870_e8909;
        locals.var_fn97_calc_iq__alpha_phit_dn4 = assign7870_e8909_d_n4;
        locals.var_fn97_calc_iq__alpha_phit_rv = 0.0;

        let (assign7880_e8913, assign7880_e8913_d_n14, assign7880_e8913_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__delta, locals.var_fn97_calc_iq__delta_dn14, locals.var_fn97_calc_iq__delta_dn15,)
    }
};
        locals.var_fn97_calc_iq__delta = assign7880_e8913;
        locals.var_fn97_calc_iq__delta_dn14 = assign7880_e8913_d_n14;
        locals.var_fn97_calc_iq__delta_dn15 = assign7880_e8913_d_n15;
        locals.var_fn97_calc_iq__delta_rv = 0.0;

        let (assign7890_e8917, assign7890_e8917_d_n4, assign7890_e8917_d_n14, assign7890_e8917_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__n, locals.var_fn97_calc_iq__n_dn4, locals.var_fn97_calc_iq__n_dn14, locals.var_fn97_calc_iq__n_dn15,)
    }
};
        locals.var_fn97_calc_iq__n = assign7890_e8917;
        locals.var_fn97_calc_iq__n_dn4 = assign7890_e8917_d_n4;
        locals.var_fn97_calc_iq__n_dn14 = assign7890_e8917_d_n14;
        locals.var_fn97_calc_iq__n_dn15 = assign7890_e8917_d_n15;
        locals.var_fn97_calc_iq__n_rv = 0.0;

        let (assign7900_e8921, assign7900_e8921_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vtof, locals.var_fn97_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn97_calc_iq__vtof = assign7900_e8921;
        locals.var_fn97_calc_iq__vtof_dn4 = assign7900_e8921_d_n4;
        locals.var_fn97_calc_iq__vtof_rv = 0.0;

        let (assign7910_e8925, assign7910_e8925_d_n14, assign7910_e8925_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsatdibl, locals.var_fn97_calc_iq__vsatdibl_dn14, locals.var_fn97_calc_iq__vsatdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsatdibl = assign7910_e8925;
        locals.var_fn97_calc_iq__vsatdibl_dn14 = assign7910_e8925_d_n14;
        locals.var_fn97_calc_iq__vsatdibl_dn15 = assign7910_e8925_d_n15;
        locals.var_fn97_calc_iq__vsatdibl_rv = 0.0;

        let (assign7920_e8929, assign7920_e8929_d_n2, assign7920_e8929_d_n3, assign7920_e8929_d_n4, assign7920_e8929_d_n7, assign7920_e8929_d_n14, assign7920_e8929_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign7920_e8929;
        locals.var_fn97_calc_iq__ffs_dn2 = assign7920_e8929_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign7920_e8929_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign7920_e8929_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign7920_e8929_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign7920_e8929_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign7920_e8929_d_n15;
        locals.var_fn97_calc_iq__ffs_rv = 0.0;

        let (assign7930_e8933, assign7930_e8933_d_n4, assign7930_e8933_d_n14, assign7930_e8933_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit, locals.var_fn97_calc_iq__two_n_phit_dn4, locals.var_fn97_calc_iq__two_n_phit_dn14, locals.var_fn97_calc_iq__two_n_phit_dn15,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit = assign7930_e8933;
        locals.var_fn97_calc_iq__two_n_phit_dn4 = assign7930_e8933_d_n4;
        locals.var_fn97_calc_iq__two_n_phit_dn14 = assign7930_e8933_d_n14;
        locals.var_fn97_calc_iq__two_n_phit_dn15 = assign7930_e8933_d_n15;
        locals.var_fn97_calc_iq__two_n_phit_rv = 0.0;

        let (assign7940_e8937, assign7940_e8937_d_n4, assign7940_e8937_d_n14, assign7940_e8937_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qref, locals.var_fn97_calc_iq__qref_dn4, locals.var_fn97_calc_iq__qref_dn14, locals.var_fn97_calc_iq__qref_dn15,)
    }
};
        locals.var_fn97_calc_iq__qref = assign7940_e8937;
        locals.var_fn97_calc_iq__qref_dn4 = assign7940_e8937_d_n4;
        locals.var_fn97_calc_iq__qref_dn14 = assign7940_e8937_d_n14;
        locals.var_fn97_calc_iq__qref_dn15 = assign7940_e8937_d_n15;
        locals.var_fn97_calc_iq__qref_rv = 0.0;

        let (assign7950_e8941, assign7950_e8941_d_n2, assign7950_e8941_d_n3, assign7950_e8941_d_n4, assign7950_e8941_d_n7, assign7950_e8941_d_n14, assign7950_e8941_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etas, locals.var_fn97_calc_iq__etas_dn2, locals.var_fn97_calc_iq__etas_dn3, locals.var_fn97_calc_iq__etas_dn4, locals.var_fn97_calc_iq__etas_dn7, locals.var_fn97_calc_iq__etas_dn14, locals.var_fn97_calc_iq__etas_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas = assign7950_e8941;
        locals.var_fn97_calc_iq__etas_dn2 = assign7950_e8941_d_n2;
        locals.var_fn97_calc_iq__etas_dn3 = assign7950_e8941_d_n3;
        locals.var_fn97_calc_iq__etas_dn4 = assign7950_e8941_d_n4;
        locals.var_fn97_calc_iq__etas_dn7 = assign7950_e8941_d_n7;
        locals.var_fn97_calc_iq__etas_dn14 = assign7950_e8941_d_n14;
        locals.var_fn97_calc_iq__etas_dn15 = assign7950_e8941_d_n15;
        locals.var_fn97_calc_iq__etas_rv = 0.0;

        let (assign7960_e8945, assign7960_e8945_d_n2, assign7960_e8945_d_n3, assign7960_e8945_d_n4, assign7960_e8945_d_n7, assign7960_e8945_d_n14, assign7960_e8945_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign7960_e8945;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign7960_e8945_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign7960_e8945_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign7960_e8945_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign7960_e8945_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign7960_e8945_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign7960_e8945_d_n15;
        locals.var_fn97_calc_iq__qinvs_rv = 0.0;

        let (assign7970_e8949, assign7970_e8949_d_n2, assign7970_e8949_d_n3, assign7970_e8949_d_n4, assign7970_e8949_d_n7, assign7970_e8949_d_n14, assign7970_e8949_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__muf, locals.var_fn97_calc_iq__muf_dn2, locals.var_fn97_calc_iq__muf_dn3, locals.var_fn97_calc_iq__muf_dn4, locals.var_fn97_calc_iq__muf_dn7, locals.var_fn97_calc_iq__muf_dn14, locals.var_fn97_calc_iq__muf_dn15,)
    }
};
        locals.var_fn97_calc_iq__muf = assign7970_e8949;
        locals.var_fn97_calc_iq__muf_dn2 = assign7970_e8949_d_n2;
        locals.var_fn97_calc_iq__muf_dn3 = assign7970_e8949_d_n3;
        locals.var_fn97_calc_iq__muf_dn4 = assign7970_e8949_d_n4;
        locals.var_fn97_calc_iq__muf_dn7 = assign7970_e8949_d_n7;
        locals.var_fn97_calc_iq__muf_dn14 = assign7970_e8949_d_n14;
        locals.var_fn97_calc_iq__muf_dn15 = assign7970_e8949_d_n15;
        locals.var_fn97_calc_iq__muf_rv = 0.0;

        let (assign7980_e8953, assign7980_e8953_d_n2, assign7980_e8953_d_n3, assign7980_e8953_d_n4, assign7980_e8953_d_n7, assign7980_e8953_d_n14, assign7980_e8953_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vx, locals.var_fn97_calc_iq__vx_dn2, locals.var_fn97_calc_iq__vx_dn3, locals.var_fn97_calc_iq__vx_dn4, locals.var_fn97_calc_iq__vx_dn7, locals.var_fn97_calc_iq__vx_dn14, locals.var_fn97_calc_iq__vx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vx = assign7980_e8953;
        locals.var_fn97_calc_iq__vx_dn2 = assign7980_e8953_d_n2;
        locals.var_fn97_calc_iq__vx_dn3 = assign7980_e8953_d_n3;
        locals.var_fn97_calc_iq__vx_dn4 = assign7980_e8953_d_n4;
        locals.var_fn97_calc_iq__vx_dn7 = assign7980_e8953_d_n7;
        locals.var_fn97_calc_iq__vx_dn14 = assign7980_e8953_d_n14;
        locals.var_fn97_calc_iq__vx_dn15 = assign7980_e8953_d_n15;
        locals.var_fn97_calc_iq__vx_rv = 0.0;

        let (assign8000_e8961, assign8000_e8961_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__n0, locals.var_fn97_calc_iq__n0_dn4,)
    }
};
        locals.var_fn97_calc_iq__n0 = assign8000_e8961;
        locals.var_fn97_calc_iq__n0_dn4 = assign8000_e8961_d_n4;
        locals.var_fn97_calc_iq__n0_rv = 0.0;

        let (assign8010_e8965, assign8010_e8965_d_n2, assign8010_e8965_d_n4, assign8010_e8965_d_n7, assign8010_e8965_d_n14, assign8010_e8965_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign8010_e8965;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign8010_e8965_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign8010_e8965_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign8010_e8965_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign8010_e8965_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign8010_e8965_d_n15;
        locals.var_fn97_calc_iq__ffs0_rv = 0.0;

        let (assign8020_e8969, assign8020_e8969_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit0, locals.var_fn97_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit0 = assign8020_e8969;
        locals.var_fn97_calc_iq__two_n_phit0_dn4 = assign8020_e8969_d_n4;
        locals.var_fn97_calc_iq__two_n_phit0_rv = 0.0;

        let (assign8030_e8973, assign8030_e8973_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qref0, locals.var_fn97_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn97_calc_iq__qref0 = assign8030_e8973;
        locals.var_fn97_calc_iq__qref0_dn4 = assign8030_e8973_d_n4;
        locals.var_fn97_calc_iq__qref0_rv = 0.0;

        let (assign8040_e8977, assign8040_e8977_d_n2, assign8040_e8977_d_n4, assign8040_e8977_d_n7, assign8040_e8977_d_n14, assign8040_e8977_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etas0, locals.var_fn97_calc_iq__etas0_dn2, locals.var_fn97_calc_iq__etas0_dn4, locals.var_fn97_calc_iq__etas0_dn7, locals.var_fn97_calc_iq__etas0_dn14, locals.var_fn97_calc_iq__etas0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas0 = assign8040_e8977;
        locals.var_fn97_calc_iq__etas0_dn2 = assign8040_e8977_d_n2;
        locals.var_fn97_calc_iq__etas0_dn4 = assign8040_e8977_d_n4;
        locals.var_fn97_calc_iq__etas0_dn7 = assign8040_e8977_d_n7;
        locals.var_fn97_calc_iq__etas0_dn14 = assign8040_e8977_d_n14;
        locals.var_fn97_calc_iq__etas0_dn15 = assign8040_e8977_d_n15;
        locals.var_fn97_calc_iq__etas0_rv = 0.0;

        let (assign8050_e8981, assign8050_e8981_d_n2, assign8050_e8981_d_n4, assign8050_e8981_d_n7, assign8050_e8981_d_n14, assign8050_e8981_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign8050_e8981;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign8050_e8981_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign8050_e8981_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign8050_e8981_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign8050_e8981_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign8050_e8981_d_n15;
        locals.var_fn97_calc_iq__qinvs0_rv = 0.0;

        let (assign8060_e8985, assign8060_e8985_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__muf0, locals.var_fn97_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn97_calc_iq__muf0 = assign8060_e8985;
        locals.var_fn97_calc_iq__muf0_dn4 = assign8060_e8985_d_n4;
        locals.var_fn97_calc_iq__muf0_rv = 0.0;

        let (assign8070_e8989, assign8070_e8989_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vx0, locals.var_fn97_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vx0 = assign8070_e8989;
        locals.var_fn97_calc_iq__vx0_dn4 = assign8070_e8989_d_n4;
        locals.var_fn97_calc_iq__vx0_rv = 0.0;

        let (assign8080_e8993, assign8080_e8993_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__tfacmobin, locals.var_fn97_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn97_calc_iq__tfacmobin = assign8080_e8993;
        locals.var_fn97_calc_iq__tfacmobin_dn4 = assign8080_e8993_d_n4;
        locals.var_fn97_calc_iq__tfacmobin_rv = 0.0;

        let (assign8090_e8997, assign8090_e8997_d_n2, assign8090_e8997_d_n3, assign8090_e8997_d_n4, assign8090_e8997_d_n7, assign8090_e8997_d_n14, assign8090_e8997_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8090_e8997;
        locals.var_fn97_calc_iq__ff_dn2 = assign8090_e8997_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8090_e8997_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8090_e8997_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8090_e8997_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8090_e8997_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8090_e8997_d_n15;
        locals.var_fn97_calc_iq__ff_rv = 0.0;

        let (assign8100_e9001, assign8100_e9001_d_n2, assign8100_e9001_d_n3, assign8100_e9001_d_n4, assign8100_e9001_d_n7, assign8100_e9001_d_n14, assign8100_e9001_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__eta, locals.var_fn97_calc_iq__eta_dn2, locals.var_fn97_calc_iq__eta_dn3, locals.var_fn97_calc_iq__eta_dn4, locals.var_fn97_calc_iq__eta_dn7, locals.var_fn97_calc_iq__eta_dn14, locals.var_fn97_calc_iq__eta_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta = assign8100_e9001;
        locals.var_fn97_calc_iq__eta_dn2 = assign8100_e9001_d_n2;
        locals.var_fn97_calc_iq__eta_dn3 = assign8100_e9001_d_n3;
        locals.var_fn97_calc_iq__eta_dn4 = assign8100_e9001_d_n4;
        locals.var_fn97_calc_iq__eta_dn7 = assign8100_e9001_d_n7;
        locals.var_fn97_calc_iq__eta_dn14 = assign8100_e9001_d_n14;
        locals.var_fn97_calc_iq__eta_dn15 = assign8100_e9001_d_n15;
        locals.var_fn97_calc_iq__eta_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign8110_e9005, assign8110_e9005_d_n2, assign8110_e9005_d_n3, assign8110_e9005_d_n4, assign8110_e9005_d_n7, assign8110_e9005_d_n14, assign8110_e9005_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8110_e9005;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8110_e9005_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8110_e9005_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8110_e9005_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8110_e9005_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8110_e9005_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8110_e9005_d_n15;
        locals.var_fn97_calc_iq__qinvv_rv = 0.0;

        let (assign8120_e9009, assign8120_e9009_d_n2, assign8120_e9009_d_n4, assign8120_e9009_d_n7, assign8120_e9009_d_n14, assign8120_e9009_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign8120_e9009;
        locals.var_fn97_calc_iq__ff0_dn2 = assign8120_e9009_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign8120_e9009_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign8120_e9009_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign8120_e9009_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign8120_e9009_d_n15;
        locals.var_fn97_calc_iq__ff0_rv = 0.0;

        let (assign8130_e9013, assign8130_e9013_d_n2, assign8130_e9013_d_n4, assign8130_e9013_d_n7, assign8130_e9013_d_n14, assign8130_e9013_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__eta0, locals.var_fn97_calc_iq__eta0_dn2, locals.var_fn97_calc_iq__eta0_dn4, locals.var_fn97_calc_iq__eta0_dn7, locals.var_fn97_calc_iq__eta0_dn14, locals.var_fn97_calc_iq__eta0_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta0 = assign8130_e9013;
        locals.var_fn97_calc_iq__eta0_dn2 = assign8130_e9013_d_n2;
        locals.var_fn97_calc_iq__eta0_dn4 = assign8130_e9013_d_n4;
        locals.var_fn97_calc_iq__eta0_dn7 = assign8130_e9013_d_n7;
        locals.var_fn97_calc_iq__eta0_dn14 = assign8130_e9013_d_n14;
        locals.var_fn97_calc_iq__eta0_dn15 = assign8130_e9013_d_n15;
        locals.var_fn97_calc_iq__eta0_rv = 0.0;

        let (assign8140_e9017, assign8140_e9017_d_n2, assign8140_e9017_d_n4, assign8140_e9017_d_n7, assign8140_e9017_d_n14, assign8140_e9017_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign8140_e9017;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign8140_e9017_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign8140_e9017_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign8140_e9017_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign8140_e9017_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign8140_e9017_d_n15;
        locals.var_fn97_calc_iq__qinvv0_rv = 0.0;

        let (assign8150_e9021, assign8150_e9021_d_n2, assign8150_e9021_d_n3, assign8150_e9021_d_n4, assign8150_e9021_d_n7, assign8150_e9021_d_n14, assign8150_e9021_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats, locals.var_fn97_calc_iq__vdsats_dn2, locals.var_fn97_calc_iq__vdsats_dn3, locals.var_fn97_calc_iq__vdsats_dn4, locals.var_fn97_calc_iq__vdsats_dn7, locals.var_fn97_calc_iq__vdsats_dn14, locals.var_fn97_calc_iq__vdsats_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats = assign8150_e9021;
        locals.var_fn97_calc_iq__vdsats_dn2 = assign8150_e9021_d_n2;
        locals.var_fn97_calc_iq__vdsats_dn3 = assign8150_e9021_d_n3;
        locals.var_fn97_calc_iq__vdsats_dn4 = assign8150_e9021_d_n4;
        locals.var_fn97_calc_iq__vdsats_dn7 = assign8150_e9021_d_n7;
        locals.var_fn97_calc_iq__vdsats_dn14 = assign8150_e9021_d_n14;
        locals.var_fn97_calc_iq__vdsats_dn15 = assign8150_e9021_d_n15;
        locals.var_fn97_calc_iq__vdsats_rv = 0.0;

        let (assign8160_e9025, assign8160_e9025_d_n2, assign8160_e9025_d_n3, assign8160_e9025_d_n4, assign8160_e9025_d_n7, assign8160_e9025_d_n14, assign8160_e9025_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats1, locals.var_fn97_calc_iq__vdsats1_dn2, locals.var_fn97_calc_iq__vdsats1_dn3, locals.var_fn97_calc_iq__vdsats1_dn4, locals.var_fn97_calc_iq__vdsats1_dn7, locals.var_fn97_calc_iq__vdsats1_dn14, locals.var_fn97_calc_iq__vdsats1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats1 = assign8160_e9025;
        locals.var_fn97_calc_iq__vdsats1_dn2 = assign8160_e9025_d_n2;
        locals.var_fn97_calc_iq__vdsats1_dn3 = assign8160_e9025_d_n3;
        locals.var_fn97_calc_iq__vdsats1_dn4 = assign8160_e9025_d_n4;
        locals.var_fn97_calc_iq__vdsats1_dn7 = assign8160_e9025_d_n7;
        locals.var_fn97_calc_iq__vdsats1_dn14 = assign8160_e9025_d_n14;
        locals.var_fn97_calc_iq__vdsats1_dn15 = assign8160_e9025_d_n15;
        locals.var_fn97_calc_iq__vdsats1_rv = 0.0;

        let (assign8170_e9029, assign8170_e9029_d_n2, assign8170_e9029_d_n3, assign8170_e9029_d_n4, assign8170_e9029_d_n7, assign8170_e9029_d_n14, assign8170_e9029_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsat, locals.var_fn97_calc_iq__vdsat_dn2, locals.var_fn97_calc_iq__vdsat_dn3, locals.var_fn97_calc_iq__vdsat_dn4, locals.var_fn97_calc_iq__vdsat_dn7, locals.var_fn97_calc_iq__vdsat_dn14, locals.var_fn97_calc_iq__vdsat_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat = assign8170_e9029;
        locals.var_fn97_calc_iq__vdsat_dn2 = assign8170_e9029_d_n2;
        locals.var_fn97_calc_iq__vdsat_dn3 = assign8170_e9029_d_n3;
        locals.var_fn97_calc_iq__vdsat_dn4 = assign8170_e9029_d_n4;
        locals.var_fn97_calc_iq__vdsat_dn7 = assign8170_e9029_d_n7;
        locals.var_fn97_calc_iq__vdsat_dn14 = assign8170_e9029_d_n14;
        locals.var_fn97_calc_iq__vdsat_dn15 = assign8170_e9029_d_n15;
        locals.var_fn97_calc_iq__vdsat_rv = 0.0;

        let (assign8180_e9033, assign8180_e9033_d_n2, assign8180_e9033_d_n3, assign8180_e9033_d_n4, assign8180_e9033_d_n7, assign8180_e9033_d_n14, assign8180_e9033_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fsd, locals.var_fn97_calc_iq__fsd_dn2, locals.var_fn97_calc_iq__fsd_dn3, locals.var_fn97_calc_iq__fsd_dn4, locals.var_fn97_calc_iq__fsd_dn7, locals.var_fn97_calc_iq__fsd_dn14, locals.var_fn97_calc_iq__fsd_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd = assign8180_e9033;
        locals.var_fn97_calc_iq__fsd_dn2 = assign8180_e9033_d_n2;
        locals.var_fn97_calc_iq__fsd_dn3 = assign8180_e9033_d_n3;
        locals.var_fn97_calc_iq__fsd_dn4 = assign8180_e9033_d_n4;
        locals.var_fn97_calc_iq__fsd_dn7 = assign8180_e9033_d_n7;
        locals.var_fn97_calc_iq__fsd_dn14 = assign8180_e9033_d_n14;
        locals.var_fn97_calc_iq__fsd_dn15 = assign8180_e9033_d_n15;
        locals.var_fn97_calc_iq__fsd_rv = 0.0;

        let (assign8190_e9037, assign8190_e9037_d_n2, assign8190_e9037_d_n3, assign8190_e9037_d_n4, assign8190_e9037_d_n7, assign8190_e9037_d_n14, assign8190_e9037_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdx, locals.var_fn97_calc_iq__vdx_dn2, locals.var_fn97_calc_iq__vdx_dn3, locals.var_fn97_calc_iq__vdx_dn4, locals.var_fn97_calc_iq__vdx_dn7, locals.var_fn97_calc_iq__vdx_dn14, locals.var_fn97_calc_iq__vdx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx = assign8190_e9037;
        locals.var_fn97_calc_iq__vdx_dn2 = assign8190_e9037_d_n2;
        locals.var_fn97_calc_iq__vdx_dn3 = assign8190_e9037_d_n3;
        locals.var_fn97_calc_iq__vdx_dn4 = assign8190_e9037_d_n4;
        locals.var_fn97_calc_iq__vdx_dn7 = assign8190_e9037_d_n7;
        locals.var_fn97_calc_iq__vdx_dn14 = assign8190_e9037_d_n14;
        locals.var_fn97_calc_iq__vdx_dn15 = assign8190_e9037_d_n15;
        locals.var_fn97_calc_iq__vdx_rv = 0.0;

        let (assign8200_e9041, assign8200_e9041_d_n2, assign8200_e9041_d_n3, assign8200_e9041_d_n4, assign8200_e9041_d_n7, assign8200_e9041_d_n14, assign8200_e9041_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fds, locals.var_fn97_calc_iq__fds_dn2, locals.var_fn97_calc_iq__fds_dn3, locals.var_fn97_calc_iq__fds_dn4, locals.var_fn97_calc_iq__fds_dn7, locals.var_fn97_calc_iq__fds_dn14, locals.var_fn97_calc_iq__fds_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds = assign8200_e9041;
        locals.var_fn97_calc_iq__fds_dn2 = assign8200_e9041_d_n2;
        locals.var_fn97_calc_iq__fds_dn3 = assign8200_e9041_d_n3;
        locals.var_fn97_calc_iq__fds_dn4 = assign8200_e9041_d_n4;
        locals.var_fn97_calc_iq__fds_dn7 = assign8200_e9041_d_n7;
        locals.var_fn97_calc_iq__fds_dn14 = assign8200_e9041_d_n14;
        locals.var_fn97_calc_iq__fds_dn15 = assign8200_e9041_d_n15;
        locals.var_fn97_calc_iq__fds_rv = 0.0;

        let (assign8210_e9045, assign8210_e9045_d_n2, assign8210_e9045_d_n3, assign8210_e9045_d_n4, assign8210_e9045_d_n7, assign8210_e9045_d_n14, assign8210_e9045_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsx, locals.var_fn97_calc_iq__vsx_dn2, locals.var_fn97_calc_iq__vsx_dn3, locals.var_fn97_calc_iq__vsx_dn4, locals.var_fn97_calc_iq__vsx_dn7, locals.var_fn97_calc_iq__vsx_dn14, locals.var_fn97_calc_iq__vsx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx = assign8210_e9045;
        locals.var_fn97_calc_iq__vsx_dn2 = assign8210_e9045_d_n2;
        locals.var_fn97_calc_iq__vsx_dn3 = assign8210_e9045_d_n3;
        locals.var_fn97_calc_iq__vsx_dn4 = assign8210_e9045_d_n4;
        locals.var_fn97_calc_iq__vsx_dn7 = assign8210_e9045_d_n7;
        locals.var_fn97_calc_iq__vsx_dn14 = assign8210_e9045_d_n14;
        locals.var_fn97_calc_iq__vsx_dn15 = assign8210_e9045_d_n15;
        locals.var_fn97_calc_iq__vsx_rv = 0.0;

        let (assign8220_e9049, assign8220_e9049_d_n2, assign8220_e9049_d_n3, assign8220_e9049_d_n4, assign8220_e9049_d_n7, assign8220_e9049_d_n14, assign8220_e9049_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign8220_e9049;
        locals.var_fn97_calc_iq__ffd_dn2 = assign8220_e9049_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign8220_e9049_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign8220_e9049_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign8220_e9049_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign8220_e9049_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign8220_e9049_d_n15;
        locals.var_fn97_calc_iq__ffd_rv = 0.0;

        let (assign8230_e9053, assign8230_e9053_d_n2, assign8230_e9053_d_n3, assign8230_e9053_d_n4, assign8230_e9053_d_n7, assign8230_e9053_d_n14, assign8230_e9053_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etad, locals.var_fn97_calc_iq__etad_dn2, locals.var_fn97_calc_iq__etad_dn3, locals.var_fn97_calc_iq__etad_dn4, locals.var_fn97_calc_iq__etad_dn7, locals.var_fn97_calc_iq__etad_dn14, locals.var_fn97_calc_iq__etad_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad = assign8230_e9053;
        locals.var_fn97_calc_iq__etad_dn2 = assign8230_e9053_d_n2;
        locals.var_fn97_calc_iq__etad_dn3 = assign8230_e9053_d_n3;
        locals.var_fn97_calc_iq__etad_dn4 = assign8230_e9053_d_n4;
        locals.var_fn97_calc_iq__etad_dn7 = assign8230_e9053_d_n7;
        locals.var_fn97_calc_iq__etad_dn14 = assign8230_e9053_d_n14;
        locals.var_fn97_calc_iq__etad_dn15 = assign8230_e9053_d_n15;
        locals.var_fn97_calc_iq__etad_rv = 0.0;

        let (assign8240_e9057, assign8240_e9057_d_n2, assign8240_e9057_d_n3, assign8240_e9057_d_n4, assign8240_e9057_d_n7, assign8240_e9057_d_n14, assign8240_e9057_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign8240_e9057;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign8240_e9057_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign8240_e9057_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign8240_e9057_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign8240_e9057_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign8240_e9057_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign8240_e9057_d_n15;
        locals.var_fn97_calc_iq__qinvd_rv = 0.0;

        let (assign8250_e9061, assign8250_e9061_d_n2, assign8250_e9061_d_n3, assign8250_e9061_d_n4, assign8250_e9061_d_n7, assign8250_e9061_d_n14, assign8250_e9061_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsc, locals.var_fn97_calc_iq__vdsc_dn2, locals.var_fn97_calc_iq__vdsc_dn3, locals.var_fn97_calc_iq__vdsc_dn4, locals.var_fn97_calc_iq__vdsc_dn7, locals.var_fn97_calc_iq__vdsc_dn14, locals.var_fn97_calc_iq__vdsc_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsc = assign8250_e9061;
        locals.var_fn97_calc_iq__vdsc_dn2 = assign8250_e9061_d_n2;
        locals.var_fn97_calc_iq__vdsc_dn3 = assign8250_e9061_d_n3;
        locals.var_fn97_calc_iq__vdsc_dn4 = assign8250_e9061_d_n4;
        locals.var_fn97_calc_iq__vdsc_dn7 = assign8250_e9061_d_n7;
        locals.var_fn97_calc_iq__vdsc_dn14 = assign8250_e9061_d_n14;
        locals.var_fn97_calc_iq__vdsc_dn15 = assign8250_e9061_d_n15;
        locals.var_fn97_calc_iq__vdsc_rv = 0.0;

        let (assign8280_e9073, assign8280_e9073_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats0, locals.var_fn97_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vdsats0 = assign8280_e9073;
        locals.var_fn97_calc_iq__vdsats0_dn4 = assign8280_e9073_d_n4;
        locals.var_fn97_calc_iq__vdsats0_rv = 0.0;

        let (assign8290_e9077, assign8290_e9077_d_n2, assign8290_e9077_d_n4, assign8290_e9077_d_n7, assign8290_e9077_d_n14, assign8290_e9077_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsats10, locals.var_fn97_calc_iq__vdsats10_dn2, locals.var_fn97_calc_iq__vdsats10_dn4, locals.var_fn97_calc_iq__vdsats10_dn7, locals.var_fn97_calc_iq__vdsats10_dn14, locals.var_fn97_calc_iq__vdsats10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats10 = assign8290_e9077;
        locals.var_fn97_calc_iq__vdsats10_dn2 = assign8290_e9077_d_n2;
        locals.var_fn97_calc_iq__vdsats10_dn4 = assign8290_e9077_d_n4;
        locals.var_fn97_calc_iq__vdsats10_dn7 = assign8290_e9077_d_n7;
        locals.var_fn97_calc_iq__vdsats10_dn14 = assign8290_e9077_d_n14;
        locals.var_fn97_calc_iq__vdsats10_dn15 = assign8290_e9077_d_n15;
        locals.var_fn97_calc_iq__vdsats10_rv = 0.0;

        let (assign8300_e9081, assign8300_e9081_d_n2, assign8300_e9081_d_n4, assign8300_e9081_d_n7, assign8300_e9081_d_n14, assign8300_e9081_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdsat10, locals.var_fn97_calc_iq__vdsat10_dn2, locals.var_fn97_calc_iq__vdsat10_dn4, locals.var_fn97_calc_iq__vdsat10_dn7, locals.var_fn97_calc_iq__vdsat10_dn14, locals.var_fn97_calc_iq__vdsat10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat10 = assign8300_e9081;
        locals.var_fn97_calc_iq__vdsat10_dn2 = assign8300_e9081_d_n2;
        locals.var_fn97_calc_iq__vdsat10_dn4 = assign8300_e9081_d_n4;
        locals.var_fn97_calc_iq__vdsat10_dn7 = assign8300_e9081_d_n7;
        locals.var_fn97_calc_iq__vdsat10_dn14 = assign8300_e9081_d_n14;
        locals.var_fn97_calc_iq__vdsat10_dn15 = assign8300_e9081_d_n15;
        locals.var_fn97_calc_iq__vdsat10_rv = 0.0;

        let (assign8310_e9085, assign8310_e9085_d_n2, assign8310_e9085_d_n4, assign8310_e9085_d_n7, assign8310_e9085_d_n14, assign8310_e9085_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fsd0, locals.var_fn97_calc_iq__fsd0_dn2, locals.var_fn97_calc_iq__fsd0_dn4, locals.var_fn97_calc_iq__fsd0_dn7, locals.var_fn97_calc_iq__fsd0_dn14, locals.var_fn97_calc_iq__fsd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd0 = assign8310_e9085;
        locals.var_fn97_calc_iq__fsd0_dn2 = assign8310_e9085_d_n2;
        locals.var_fn97_calc_iq__fsd0_dn4 = assign8310_e9085_d_n4;
        locals.var_fn97_calc_iq__fsd0_dn7 = assign8310_e9085_d_n7;
        locals.var_fn97_calc_iq__fsd0_dn14 = assign8310_e9085_d_n14;
        locals.var_fn97_calc_iq__fsd0_dn15 = assign8310_e9085_d_n15;
        locals.var_fn97_calc_iq__fsd0_rv = 0.0;

        let (assign8320_e9089, assign8320_e9089_d_n2, assign8320_e9089_d_n4, assign8320_e9089_d_n7, assign8320_e9089_d_n14, assign8320_e9089_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vdx0, locals.var_fn97_calc_iq__vdx0_dn2, locals.var_fn97_calc_iq__vdx0_dn4, locals.var_fn97_calc_iq__vdx0_dn7, locals.var_fn97_calc_iq__vdx0_dn14, locals.var_fn97_calc_iq__vdx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx0 = assign8320_e9089;
        locals.var_fn97_calc_iq__vdx0_dn2 = assign8320_e9089_d_n2;
        locals.var_fn97_calc_iq__vdx0_dn4 = assign8320_e9089_d_n4;
        locals.var_fn97_calc_iq__vdx0_dn7 = assign8320_e9089_d_n7;
        locals.var_fn97_calc_iq__vdx0_dn14 = assign8320_e9089_d_n14;
        locals.var_fn97_calc_iq__vdx0_dn15 = assign8320_e9089_d_n15;
        locals.var_fn97_calc_iq__vdx0_rv = 0.0;

        let (assign8330_e9093, assign8330_e9093_d_n2, assign8330_e9093_d_n4, assign8330_e9093_d_n7, assign8330_e9093_d_n14, assign8330_e9093_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__fds0, locals.var_fn97_calc_iq__fds0_dn2, locals.var_fn97_calc_iq__fds0_dn4, locals.var_fn97_calc_iq__fds0_dn7, locals.var_fn97_calc_iq__fds0_dn14, locals.var_fn97_calc_iq__fds0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds0 = assign8330_e9093;
        locals.var_fn97_calc_iq__fds0_dn2 = assign8330_e9093_d_n2;
        locals.var_fn97_calc_iq__fds0_dn4 = assign8330_e9093_d_n4;
        locals.var_fn97_calc_iq__fds0_dn7 = assign8330_e9093_d_n7;
        locals.var_fn97_calc_iq__fds0_dn14 = assign8330_e9093_d_n14;
        locals.var_fn97_calc_iq__fds0_dn15 = assign8330_e9093_d_n15;
        locals.var_fn97_calc_iq__fds0_rv = 0.0;

        let (assign8340_e9097, assign8340_e9097_d_n2, assign8340_e9097_d_n4, assign8340_e9097_d_n7, assign8340_e9097_d_n14, assign8340_e9097_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsx0, locals.var_fn97_calc_iq__vsx0_dn2, locals.var_fn97_calc_iq__vsx0_dn4, locals.var_fn97_calc_iq__vsx0_dn7, locals.var_fn97_calc_iq__vsx0_dn14, locals.var_fn97_calc_iq__vsx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx0 = assign8340_e9097;
        locals.var_fn97_calc_iq__vsx0_dn2 = assign8340_e9097_d_n2;
        locals.var_fn97_calc_iq__vsx0_dn4 = assign8340_e9097_d_n4;
        locals.var_fn97_calc_iq__vsx0_dn7 = assign8340_e9097_d_n7;
        locals.var_fn97_calc_iq__vsx0_dn14 = assign8340_e9097_d_n14;
        locals.var_fn97_calc_iq__vsx0_dn15 = assign8340_e9097_d_n15;
        locals.var_fn97_calc_iq__vsx0_rv = 0.0;

        let (assign8350_e9101, assign8350_e9101_d_n2, assign8350_e9101_d_n4, assign8350_e9101_d_n7, assign8350_e9101_d_n14, assign8350_e9101_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign8350_e9101;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign8350_e9101_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign8350_e9101_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign8350_e9101_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign8350_e9101_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign8350_e9101_d_n15;
        locals.var_fn97_calc_iq__ffd0_rv = 0.0;

        let (assign8360_e9105, assign8360_e9105_d_n2, assign8360_e9105_d_n4, assign8360_e9105_d_n7, assign8360_e9105_d_n14, assign8360_e9105_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etad0, locals.var_fn97_calc_iq__etad0_dn2, locals.var_fn97_calc_iq__etad0_dn4, locals.var_fn97_calc_iq__etad0_dn7, locals.var_fn97_calc_iq__etad0_dn14, locals.var_fn97_calc_iq__etad0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad0 = assign8360_e9105;
        locals.var_fn97_calc_iq__etad0_dn2 = assign8360_e9105_d_n2;
        locals.var_fn97_calc_iq__etad0_dn4 = assign8360_e9105_d_n4;
        locals.var_fn97_calc_iq__etad0_dn7 = assign8360_e9105_d_n7;
        locals.var_fn97_calc_iq__etad0_dn14 = assign8360_e9105_d_n14;
        locals.var_fn97_calc_iq__etad0_dn15 = assign8360_e9105_d_n15;
        locals.var_fn97_calc_iq__etad0_rv = 0.0;

        let (assign8370_e9109, assign8370_e9109_d_n2, assign8370_e9109_d_n4, assign8370_e9109_d_n7, assign8370_e9109_d_n14, assign8370_e9109_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign8370_e9109;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign8370_e9109_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign8370_e9109_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign8370_e9109_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign8370_e9109_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign8370_e9109_d_n15;
        locals.var_fn97_calc_iq__qinvd0_rv = 0.0;

        let (assign8380_e9113, assign8380_e9113_d_n2, assign8380_e9113_d_n4, assign8380_e9113_d_n7, assign8380_e9113_d_n14, assign8380_e9113_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qs2, locals.var_fn97_calc_iq__qs2_dn2, locals.var_fn97_calc_iq__qs2_dn4, locals.var_fn97_calc_iq__qs2_dn7, locals.var_fn97_calc_iq__qs2_dn14, locals.var_fn97_calc_iq__qs2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs2 = assign8380_e9113;
        locals.var_fn97_calc_iq__qs2_dn2 = assign8380_e9113_d_n2;
        locals.var_fn97_calc_iq__qs2_dn4 = assign8380_e9113_d_n4;
        locals.var_fn97_calc_iq__qs2_dn7 = assign8380_e9113_d_n7;
        locals.var_fn97_calc_iq__qs2_dn14 = assign8380_e9113_d_n14;
        locals.var_fn97_calc_iq__qs2_dn15 = assign8380_e9113_d_n15;
        locals.var_fn97_calc_iq__qs2_rv = 0.0;

        let (assign8390_e9117, assign8390_e9117_d_n2, assign8390_e9117_d_n4, assign8390_e9117_d_n7, assign8390_e9117_d_n14, assign8390_e9117_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qs3, locals.var_fn97_calc_iq__qs3_dn2, locals.var_fn97_calc_iq__qs3_dn4, locals.var_fn97_calc_iq__qs3_dn7, locals.var_fn97_calc_iq__qs3_dn14, locals.var_fn97_calc_iq__qs3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs3 = assign8390_e9117;
        locals.var_fn97_calc_iq__qs3_dn2 = assign8390_e9117_d_n2;
        locals.var_fn97_calc_iq__qs3_dn4 = assign8390_e9117_d_n4;
        locals.var_fn97_calc_iq__qs3_dn7 = assign8390_e9117_d_n7;
        locals.var_fn97_calc_iq__qs3_dn14 = assign8390_e9117_d_n14;
        locals.var_fn97_calc_iq__qs3_dn15 = assign8390_e9117_d_n15;
        locals.var_fn97_calc_iq__qs3_rv = 0.0;

        let (assign8400_e9121, assign8400_e9121_d_n2, assign8400_e9121_d_n4, assign8400_e9121_d_n7, assign8400_e9121_d_n14, assign8400_e9121_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd2, locals.var_fn97_calc_iq__qd2_dn2, locals.var_fn97_calc_iq__qd2_dn4, locals.var_fn97_calc_iq__qd2_dn7, locals.var_fn97_calc_iq__qd2_dn14, locals.var_fn97_calc_iq__qd2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd2 = assign8400_e9121;
        locals.var_fn97_calc_iq__qd2_dn2 = assign8400_e9121_d_n2;
        locals.var_fn97_calc_iq__qd2_dn4 = assign8400_e9121_d_n4;
        locals.var_fn97_calc_iq__qd2_dn7 = assign8400_e9121_d_n7;
        locals.var_fn97_calc_iq__qd2_dn14 = assign8400_e9121_d_n14;
        locals.var_fn97_calc_iq__qd2_dn15 = assign8400_e9121_d_n15;
        locals.var_fn97_calc_iq__qd2_rv = 0.0;

        let (assign8410_e9125, assign8410_e9125_d_n2, assign8410_e9125_d_n4, assign8410_e9125_d_n7, assign8410_e9125_d_n14, assign8410_e9125_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd3, locals.var_fn97_calc_iq__qd3_dn2, locals.var_fn97_calc_iq__qd3_dn4, locals.var_fn97_calc_iq__qd3_dn7, locals.var_fn97_calc_iq__qd3_dn14, locals.var_fn97_calc_iq__qd3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd3 = assign8410_e9125;
        locals.var_fn97_calc_iq__qd3_dn2 = assign8410_e9125_d_n2;
        locals.var_fn97_calc_iq__qd3_dn4 = assign8410_e9125_d_n4;
        locals.var_fn97_calc_iq__qd3_dn7 = assign8410_e9125_d_n7;
        locals.var_fn97_calc_iq__qd3_dn14 = assign8410_e9125_d_n14;
        locals.var_fn97_calc_iq__qd3_dn15 = assign8410_e9125_d_n15;
        locals.var_fn97_calc_iq__qd3_rv = 0.0;

        let (assign8420_e9129, assign8420_e9129_d_n2, assign8420_e9129_d_n4, assign8420_e9129_d_n7, assign8420_e9129_d_n14, assign8420_e9129_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qsqd, locals.var_fn97_calc_iq__qsqd_dn2, locals.var_fn97_calc_iq__qsqd_dn4, locals.var_fn97_calc_iq__qsqd_dn7, locals.var_fn97_calc_iq__qsqd_dn14, locals.var_fn97_calc_iq__qsqd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsqd = assign8420_e9129;
        locals.var_fn97_calc_iq__qsqd_dn2 = assign8420_e9129_d_n2;
        locals.var_fn97_calc_iq__qsqd_dn4 = assign8420_e9129_d_n4;
        locals.var_fn97_calc_iq__qsqd_dn7 = assign8420_e9129_d_n7;
        locals.var_fn97_calc_iq__qsqd_dn14 = assign8420_e9129_d_n14;
        locals.var_fn97_calc_iq__qsqd_dn15 = assign8420_e9129_d_n15;
        locals.var_fn97_calc_iq__qsqd_rv = 0.0;

        let (assign8430_e9133, assign8430_e9133_d_n2, assign8430_e9133_d_n4, assign8430_e9133_d_n7, assign8430_e9133_d_n14, assign8430_e9133_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qinvdd, locals.var_fn97_calc_iq__qinvdd_dn2, locals.var_fn97_calc_iq__qinvdd_dn4, locals.var_fn97_calc_iq__qinvdd_dn7, locals.var_fn97_calc_iq__qinvdd_dn14, locals.var_fn97_calc_iq__qinvdd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvdd = assign8430_e9133;
        locals.var_fn97_calc_iq__qinvdd_dn2 = assign8430_e9133_d_n2;
        locals.var_fn97_calc_iq__qinvdd_dn4 = assign8430_e9133_d_n4;
        locals.var_fn97_calc_iq__qinvdd_dn7 = assign8430_e9133_d_n7;
        locals.var_fn97_calc_iq__qinvdd_dn14 = assign8430_e9133_d_n14;
        locals.var_fn97_calc_iq__qinvdd_dn15 = assign8430_e9133_d_n15;
        locals.var_fn97_calc_iq__qinvdd_rv = 0.0;

        let (assign8440_e9137, assign8440_e9137_d_n2, assign8440_e9137_d_n4, assign8440_e9137_d_n7, assign8440_e9137_d_n14, assign8440_e9137_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd1, locals.var_fn97_calc_iq__qd1_dn2, locals.var_fn97_calc_iq__qd1_dn4, locals.var_fn97_calc_iq__qd1_dn7, locals.var_fn97_calc_iq__qd1_dn14, locals.var_fn97_calc_iq__qd1_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd1 = assign8440_e9137;
        locals.var_fn97_calc_iq__qd1_dn2 = assign8440_e9137_d_n2;
        locals.var_fn97_calc_iq__qd1_dn4 = assign8440_e9137_d_n4;
        locals.var_fn97_calc_iq__qd1_dn7 = assign8440_e9137_d_n7;
        locals.var_fn97_calc_iq__qd1_dn14 = assign8440_e9137_d_n14;
        locals.var_fn97_calc_iq__qd1_dn15 = assign8440_e9137_d_n15;
        locals.var_fn97_calc_iq__qd1_rv = 0.0;

        let (assign8450_e9141, assign8450_e9141_d_n2, assign8450_e9141_d_n4, assign8450_e9141_d_n7, assign8450_e9141_d_n14, assign8450_e9141_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qs, locals.var_fn97_calc_iq__qs_dn2, locals.var_fn97_calc_iq__qs_dn4, locals.var_fn97_calc_iq__qs_dn7, locals.var_fn97_calc_iq__qs_dn14, locals.var_fn97_calc_iq__qs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs = assign8450_e9141;
        locals.var_fn97_calc_iq__qs_dn2 = assign8450_e9141_d_n2;
        locals.var_fn97_calc_iq__qs_dn4 = assign8450_e9141_d_n4;
        locals.var_fn97_calc_iq__qs_dn7 = assign8450_e9141_d_n7;
        locals.var_fn97_calc_iq__qs_dn14 = assign8450_e9141_d_n14;
        locals.var_fn97_calc_iq__qs_dn15 = assign8450_e9141_d_n15;
        locals.var_fn97_calc_iq__qs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8460_e9145, assign8460_e9145_d_n2, assign8460_e9145_d_n4, assign8460_e9145_d_n7, assign8460_e9145_d_n14, assign8460_e9145_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qd, locals.var_fn97_calc_iq__qd_dn2, locals.var_fn97_calc_iq__qd_dn4, locals.var_fn97_calc_iq__qd_dn7, locals.var_fn97_calc_iq__qd_dn14, locals.var_fn97_calc_iq__qd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd = assign8460_e9145;
        locals.var_fn97_calc_iq__qd_dn2 = assign8460_e9145_d_n2;
        locals.var_fn97_calc_iq__qd_dn4 = assign8460_e9145_d_n4;
        locals.var_fn97_calc_iq__qd_dn7 = assign8460_e9145_d_n7;
        locals.var_fn97_calc_iq__qd_dn14 = assign8460_e9145_d_n14;
        locals.var_fn97_calc_iq__qd_dn15 = assign8460_e9145_d_n15;
        locals.var_fn97_calc_iq__qd_rv = 0.0;

        let (assign8470_e9149, assign8470_e9149_d_n2, assign8470_e9149_d_n4, assign8470_e9149_d_n7, assign8470_e9149_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etac, locals.var_fn97_calc_iq__etac_dn2, locals.var_fn97_calc_iq__etac_dn4, locals.var_fn97_calc_iq__etac_dn7, locals.var_fn97_calc_iq__etac_dn14,)
    }
};
        locals.var_fn97_calc_iq__etac = assign8470_e9149;
        locals.var_fn97_calc_iq__etac_dn2 = assign8470_e9149_d_n2;
        locals.var_fn97_calc_iq__etac_dn4 = assign8470_e9149_d_n4;
        locals.var_fn97_calc_iq__etac_dn7 = assign8470_e9149_d_n7;
        locals.var_fn97_calc_iq__etac_dn14 = assign8470_e9149_d_n14;
        locals.var_fn97_calc_iq__etac_rv = 0.0;

        let (assign8480_e9153, assign8480_e9153_d_n3, assign8480_e9153_d_n4, assign8480_e9153_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etab, locals.var_fn97_calc_iq__etab_dn3, locals.var_fn97_calc_iq__etab_dn4, locals.var_fn97_calc_iq__etab_dn14,)
    }
};
        locals.var_fn97_calc_iq__etab = assign8480_e9153;
        locals.var_fn97_calc_iq__etab_dn3 = assign8480_e9153_d_n3;
        locals.var_fn97_calc_iq__etab_dn4 = assign8480_e9153_d_n4;
        locals.var_fn97_calc_iq__etab_dn14 = assign8480_e9153_d_n14;
        locals.var_fn97_calc_iq__etab_rv = 0.0;

        let (assign8490_e9157, assign8490_e9157_d_n2, assign8490_e9157_d_n4, assign8490_e9157_d_n7, assign8490_e9157_d_n14,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__etags, locals.var_fn97_calc_iq__etags_dn2, locals.var_fn97_calc_iq__etags_dn4, locals.var_fn97_calc_iq__etags_dn7, locals.var_fn97_calc_iq__etags_dn14,)
    }
};
        locals.var_fn97_calc_iq__etags = assign8490_e9157;
        locals.var_fn97_calc_iq__etags_dn2 = assign8490_e9157_d_n2;
        locals.var_fn97_calc_iq__etags_dn4 = assign8490_e9157_d_n4;
        locals.var_fn97_calc_iq__etags_dn7 = assign8490_e9157_d_n7;
        locals.var_fn97_calc_iq__etags_dn14 = assign8490_e9157_d_n14;
        locals.var_fn97_calc_iq__etags_rv = 0.0;

        let (assign8500_e9161, assign8500_e9161_d_n2, assign8500_e9161_d_n3, assign8500_e9161_d_n4, assign8500_e9161_d_n7, assign8500_e9161_d_n14, assign8500_e9161_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign8500_e9161;
        locals.var_fn97_calc_iq__exparg_dn2 = assign8500_e9161_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign8500_e9161_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign8500_e9161_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign8500_e9161_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign8500_e9161_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign8500_e9161_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign8510_e9165, assign8510_e9165_d_n2, assign8510_e9165_d_n3, assign8510_e9165_d_n4, assign8510_e9165_d_n7, assign8510_e9165_d_n14, assign8510_e9165_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__myarg, locals.var_fn97_calc_iq__myarg_dn2, locals.var_fn97_calc_iq__myarg_dn3, locals.var_fn97_calc_iq__myarg_dn4, locals.var_fn97_calc_iq__myarg_dn7, locals.var_fn97_calc_iq__myarg_dn14, locals.var_fn97_calc_iq__myarg_dn15,)
    }
};
        locals.var_fn97_calc_iq__myarg = assign8510_e9165;
        locals.var_fn97_calc_iq__myarg_dn2 = assign8510_e9165_d_n2;
        locals.var_fn97_calc_iq__myarg_dn3 = assign8510_e9165_d_n3;
        locals.var_fn97_calc_iq__myarg_dn4 = assign8510_e9165_d_n4;
        locals.var_fn97_calc_iq__myarg_dn7 = assign8510_e9165_d_n7;
        locals.var_fn97_calc_iq__myarg_dn14 = assign8510_e9165_d_n14;
        locals.var_fn97_calc_iq__myarg_dn15 = assign8510_e9165_d_n15;
        locals.var_fn97_calc_iq__myarg_rv = 0.0;

        let (assign8520_e9169, assign8520_e9169_d_n14, assign8520_e9169_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__absvdsin, locals.var_fn97_calc_iq__absvdsin_dn14, locals.var_fn97_calc_iq__absvdsin_dn15,)
    }
};
        locals.var_fn97_calc_iq__absvdsin = assign8520_e9169;
        locals.var_fn97_calc_iq__absvdsin_dn14 = assign8520_e9169_d_n14;
        locals.var_fn97_calc_iq__absvdsin_dn15 = assign8520_e9169_d_n15;
        locals.var_fn97_calc_iq__absvdsin_rv = 0.0;

        let (assign8530_e9173, assign8530_e9173_d_n2, assign8530_e9173_d_n7, assign8530_e9173_d_n14, assign8530_e9173_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vgdin, locals.var_fn97_calc_iq__vgdin_dn2, locals.var_fn97_calc_iq__vgdin_dn7, locals.var_fn97_calc_iq__vgdin_dn14, locals.var_fn97_calc_iq__vgdin_dn15,)
    }
};
        locals.var_fn97_calc_iq__vgdin = assign8530_e9173;
        locals.var_fn97_calc_iq__vgdin_dn2 = assign8530_e9173_d_n2;
        locals.var_fn97_calc_iq__vgdin_dn7 = assign8530_e9173_d_n7;
        locals.var_fn97_calc_iq__vgdin_dn14 = assign8530_e9173_d_n14;
        locals.var_fn97_calc_iq__vgdin_dn15 = assign8530_e9173_d_n15;
        locals.var_fn97_calc_iq__vgdin_rv = 0.0;

        let (assign8540_e9177, assign8540_e9177_d_n2, assign8540_e9177_d_n4, assign8540_e9177_d_n7, assign8540_e9177_d_n14, assign8540_e9177_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign8540_e9177;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign8540_e9177_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign8540_e9177_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign8540_e9177_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign8540_e9177_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign8540_e9177_d_n15;
        locals.var_fn97_calc_iq__exparg0_rv = 0.0;

        let (assign8550_e9181, assign8550_e9181_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__myarg0, locals.var_fn97_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn97_calc_iq__myarg0 = assign8550_e9181;
        locals.var_fn97_calc_iq__myarg0_dn4 = assign8550_e9181_d_n4;
        locals.var_fn97_calc_iq__myarg0_rv = 0.0;

        let (assign8560_e9208, assign8560_e9208_d_n14, assign8560_e9208_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8560_e9206, assign8560_e9206_d_n14, assign8560_e9206_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8560_e9190: f64 = (0.001 / p.p53);
                let assign8560_e9192: f64 = (assign8560_e9190 * locals.var_fn97_calc_iq__vdsin);
                let assign8560_e9193: f64 = (assign8560_e9192).tanh();
                let assign8560_e9194: f64 = (locals.var_fn97_calc_iq__vdsin * assign8560_e9193);
                (assign8560_e9194, ((locals.var_fn97_calc_iq__vdsin_dn14 * assign8560_e9193) + (locals.var_fn97_calc_iq__vdsin * ((assign8560_e9190 * locals.var_fn97_calc_iq__vdsin_dn14) / ((assign8560_e9192).cosh() * (assign8560_e9192).cosh())))), ((locals.var_fn97_calc_iq__vdsin_dn15 * assign8560_e9193) + (locals.var_fn97_calc_iq__vdsin * ((assign8560_e9190 * locals.var_fn97_calc_iq__vdsin_dn15) / ((assign8560_e9192).cosh() * (assign8560_e9192).cosh())))),)
            } else {
                let (assign8560_e9205, assign8560_e9205_d_n14, assign8560_e9205_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8560_e9200: f64 = (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsin);
                        let assign8560_e9202: f64 = (assign8560_e9200 + p.p53);
                        let assign8560_e9203: f64 = (assign8560_e9202).sqrt();
                        (assign8560_e9203, (((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsin) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsin_dn14)) / (2.0 * assign8560_e9203)), (((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsin) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsin_dn15)) / (2.0 * assign8560_e9203)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign8560_e9205, assign8560_e9205_d_n14, assign8560_e9205_d_n15,)
            }
        };
        (assign8560_e9206, assign8560_e9206_d_n14, assign8560_e9206_d_n15,)
    } else {
        (locals.var_fn97_calc_iq__absvdsin, locals.var_fn97_calc_iq__absvdsin_dn14, locals.var_fn97_calc_iq__absvdsin_dn15,)
    }
};
        locals.var_fn97_calc_iq__absvdsin = assign8560_e9208;
        locals.var_fn97_calc_iq__absvdsin_dn14 = assign8560_e9208_d_n14;
        locals.var_fn97_calc_iq__absvdsin_dn15 = assign8560_e9208_d_n15;
        locals.var_fn97_calc_iq__absvdsin_rv = 0.0;

        let (assign8570_e9214, assign8570_e9214_d_n2, assign8570_e9214_d_n7, assign8570_e9214_d_n14, assign8570_e9214_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8570_e9212: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vdsin);
        (assign8570_e9212, locals.var_fn97_calc_iq__vgsin_dn2, locals.var_fn97_calc_iq__vgsin_dn7, (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vdsin_dn14), (-locals.var_fn97_calc_iq__vdsin_dn15),)
    } else {
        (locals.var_fn97_calc_iq__vgdin, locals.var_fn97_calc_iq__vgdin_dn2, locals.var_fn97_calc_iq__vgdin_dn7, locals.var_fn97_calc_iq__vgdin_dn14, locals.var_fn97_calc_iq__vgdin_dn15,)
    }
};
        locals.var_fn97_calc_iq__vgdin = assign8570_e9214;
        locals.var_fn97_calc_iq__vgdin_dn2 = assign8570_e9214_d_n2;
        locals.var_fn97_calc_iq__vgdin_dn7 = assign8570_e9214_d_n7;
        locals.var_fn97_calc_iq__vgdin_dn14 = assign8570_e9214_d_n14;
        locals.var_fn97_calc_iq__vgdin_dn15 = assign8570_e9214_d_n15;
        locals.var_fn97_calc_iq__vgdin_rv = 0.0;

        let (assign8580_e9220, assign8580_e9220_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8580_e9218: f64 = (locals.var_fn97_calc_iq__alpha * locals.var_fn97_calc_iq__phitin);
        (assign8580_e9218, (locals.var_fn97_calc_iq__alpha * locals.var_fn97_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn97_calc_iq__alpha_phit, locals.var_fn97_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn97_calc_iq__alpha_phit = assign8580_e9220;
        locals.var_fn97_calc_iq__alpha_phit_dn4 = assign8580_e9220_d_n4;
        locals.var_fn97_calc_iq__alpha_phit_rv = 0.0;

        let (assign8590_e9232, assign8590_e9232_d_n4, assign8590_e9232_d_n14, assign8590_e9232_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8590_e9225: f64 = (2.302585092994046 * locals.var_fn97_calc_iq__phitin);
        let assign8590_e9226: f64 = (locals.var_fn97_calc_iq__ss / assign8590_e9225);
        let assign8590_e9229: f64 = (locals.var_fn97_calc_iq__nd * locals.var_fn97_calc_iq__absvdsin);
        let assign8590_e9230: f64 = (assign8590_e9226 + assign8590_e9229);
        (assign8590_e9230, (-((locals.var_fn97_calc_iq__ss * (2.302585092994046 * locals.var_fn97_calc_iq__phitin_dn4)) / (assign8590_e9225 * assign8590_e9225))), (locals.var_fn97_calc_iq__nd * locals.var_fn97_calc_iq__absvdsin_dn14), (locals.var_fn97_calc_iq__nd * locals.var_fn97_calc_iq__absvdsin_dn15),)
    } else {
        (locals.var_fn97_calc_iq__n, locals.var_fn97_calc_iq__n_dn4, locals.var_fn97_calc_iq__n_dn14, locals.var_fn97_calc_iq__n_dn15,)
    }
};
        locals.var_fn97_calc_iq__n = assign8590_e9232;
        locals.var_fn97_calc_iq__n_dn4 = assign8590_e9232_d_n4;
        locals.var_fn97_calc_iq__n_dn14 = assign8590_e9232_d_n14;
        locals.var_fn97_calc_iq__n_dn15 = assign8590_e9232_d_n15;
        locals.var_fn97_calc_iq__n_rv = 0.0;

        let (assign8600_e9242, assign8600_e9242_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8600_e9238: f64 = (locals.var_fn97_calc_iq__tambin - locals.var_fn97_calc_iq__tnomin);
        let assign8600_e9239: f64 = (locals.var_fn97_calc_iq__vtzeta * assign8600_e9238);
        let assign8600_e9240: f64 = (locals.var_fn97_calc_iq__vto + assign8600_e9239);
        (assign8600_e9240, (locals.var_fn97_calc_iq__vtzeta * locals.var_fn97_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn97_calc_iq__vtof, locals.var_fn97_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn97_calc_iq__vtof = assign8600_e9242;
        locals.var_fn97_calc_iq__vtof_dn4 = assign8600_e9242_d_n4;
        locals.var_fn97_calc_iq__vtof_rv = 0.0;

        let (assign8610_e9250, assign8610_e9250_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8610_e9246: f64 = (locals.var_fn97_calc_iq__tambin / locals.var_fn97_calc_iq__tnomin);
        let assign8610_e9248: f64 = (assign8610_e9246).powf(locals.var_fn97_calc_iq__epsilon);
        (assign8610_e9248, if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn97_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__epsilon * ((assign8610_e9246).powf(locals.var_fn97_calc_iq__epsilon - 1.0) * (locals.var_fn97_calc_iq__tambin_dn4 / locals.var_fn97_calc_iq__tnomin))) } } else { (assign8610_e9248 * (locals.var_fn97_calc_iq__epsilon * ((locals.var_fn97_calc_iq__tambin_dn4 / locals.var_fn97_calc_iq__tnomin) / assign8610_e9246))) },)
    } else {
        (locals.var_fn97_calc_iq__tfacmobin, locals.var_fn97_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn97_calc_iq__tfacmobin = assign8610_e9250;
        locals.var_fn97_calc_iq__tfacmobin_dn4 = assign8610_e9250_d_n4;
        locals.var_fn97_calc_iq__tfacmobin_rv = 0.0;

        let assign8620_e9253: f64 = if locals.var_fn97_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8620_e9253;
        locals.var_guard98_rv = 0.0;

        let (assign8630_e9271, assign8630_e9271_d_n14, assign8630_e9271_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8630_e9261: f64 = (locals.var_fn97_calc_iq__absvdsin / locals.var_fn97_calc_iq__dibsat);
        let assign8630_e9263: f64 = (assign8630_e9261).powf(locals.var_fn97_calc_iq__beta);
        let assign8630_e9264: f64 = (1.0 + assign8630_e9263);
        let assign8630_e9267: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign8630_e9268: f64 = (assign8630_e9264).powf(assign8630_e9267);
        let assign8630_e9269: f64 = (locals.var_fn97_calc_iq__absvdsin / assign8630_e9268);
        (assign8630_e9269, (((locals.var_fn97_calc_iq__absvdsin_dn14 * assign8630_e9268) - (locals.var_fn97_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign8630_e9267) as f64).is_finite() && ((assign8630_e9267) as f64).fract() == 0.0 { if assign8630_e9267 == 0.0 { 0.0 } else { (assign8630_e9267 * ((assign8630_e9264).powf(assign8630_e9267 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) })) } } else { (assign8630_e9268 * (assign8630_e9267 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn14 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) } / assign8630_e9264))) })) / (assign8630_e9268 * assign8630_e9268)), (((locals.var_fn97_calc_iq__absvdsin_dn15 * assign8630_e9268) - (locals.var_fn97_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign8630_e9267) as f64).is_finite() && ((assign8630_e9267) as f64).fract() == 0.0 { if assign8630_e9267 == 0.0 { 0.0 } else { (assign8630_e9267 * ((assign8630_e9264).powf(assign8630_e9267 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) })) } } else { (assign8630_e9268 * (assign8630_e9267 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8630_e9261).powf(locals.var_fn97_calc_iq__beta - 1.0) * (locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat))) } } else { (assign8630_e9263 * (locals.var_fn97_calc_iq__beta * ((locals.var_fn97_calc_iq__absvdsin_dn15 / locals.var_fn97_calc_iq__dibsat) / assign8630_e9261))) } / assign8630_e9264))) })) / (assign8630_e9268 * assign8630_e9268)),)
    } else {
        (locals.var_fn97_calc_iq__vsatdibl, locals.var_fn97_calc_iq__vsatdibl_dn14, locals.var_fn97_calc_iq__vsatdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsatdibl = assign8630_e9271;
        locals.var_fn97_calc_iq__vsatdibl_dn14 = assign8630_e9271_d_n14;
        locals.var_fn97_calc_iq__vsatdibl_dn15 = assign8630_e9271_d_n15;
        locals.var_fn97_calc_iq__vsatdibl_rv = 0.0;

        let (assign8640_e9278, assign8640_e9278_d_n14, assign8640_e9278_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard98 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__vsatdibl, locals.var_fn97_calc_iq__vsatdibl_dn14, locals.var_fn97_calc_iq__vsatdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsatdibl = assign8640_e9278;
        locals.var_fn97_calc_iq__vsatdibl_dn14 = assign8640_e9278_d_n14;
        locals.var_fn97_calc_iq__vsatdibl_dn15 = assign8640_e9278_d_n15;
        locals.var_fn97_calc_iq__vsatdibl_rv = 0.0;

        let (assign8650_e9288, assign8650_e9288_d_n14, assign8650_e9288_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8650_e9283: f64 = (locals.var_fn97_calc_iq__vsatdibl * locals.var_fn97_calc_iq__delta2);
        let assign8650_e9284: f64 = (locals.var_fn97_calc_iq__delta1 - assign8650_e9283);
        let assign8650_e9286: f64 = (assign8650_e9284 * locals.var_fn97_calc_iq__absvdsin);
        (assign8650_e9286, (((-(locals.var_fn97_calc_iq__vsatdibl_dn14 * locals.var_fn97_calc_iq__delta2)) * locals.var_fn97_calc_iq__absvdsin) + (assign8650_e9284 * locals.var_fn97_calc_iq__absvdsin_dn14)), (((-(locals.var_fn97_calc_iq__vsatdibl_dn15 * locals.var_fn97_calc_iq__delta2)) * locals.var_fn97_calc_iq__absvdsin) + (assign8650_e9284 * locals.var_fn97_calc_iq__absvdsin_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__delta, locals.var_fn97_calc_iq__delta_dn14, locals.var_fn97_calc_iq__delta_dn15,)
    }
};
        locals.var_fn97_calc_iq__delta = assign8650_e9288;
        locals.var_fn97_calc_iq__delta_dn14 = assign8650_e9288_d_n14;
        locals.var_fn97_calc_iq__delta_dn15 = assign8650_e9288_d_n15;
        locals.var_fn97_calc_iq__delta_rv = 0.0;

        let (assign8660_e9294, assign8660_e9294_d_n4, assign8660_e9294_d_n14, assign8660_e9294_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8660_e9292: f64 = (locals.var_fn97_calc_iq__vtof - locals.var_fn97_calc_iq__delta);
        (assign8660_e9292, locals.var_fn97_calc_iq__vtof_dn4, (-locals.var_fn97_calc_iq__delta_dn14), (-locals.var_fn97_calc_iq__delta_dn15),)
    } else {
        (locals.var_fn97_calc_iq__vtdibl, locals.var_fn97_calc_iq__vtdibl_dn4, locals.var_fn97_calc_iq__vtdibl_dn14, locals.var_fn97_calc_iq__vtdibl_dn15,)
    }
};
        locals.var_fn97_calc_iq__vtdibl = assign8660_e9294;
        locals.var_fn97_calc_iq__vtdibl_dn4 = assign8660_e9294_d_n4;
        locals.var_fn97_calc_iq__vtdibl_dn14 = assign8660_e9294_d_n14;
        locals.var_fn97_calc_iq__vtdibl_dn15 = assign8660_e9294_d_n15;
        locals.var_fn97_calc_iq__vtdibl_rv = 0.0;

        let (assign8670_e9302, assign8670_e9302_d_n4, assign8670_e9302_d_n14, assign8670_e9302_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8670_e9298: f64 = (2.0 * locals.var_fn97_calc_iq__n);
        let assign8670_e9300: f64 = (assign8670_e9298 * locals.var_fn97_calc_iq__phitin);
        (assign8670_e9300, (((2.0 * locals.var_fn97_calc_iq__n_dn4) * locals.var_fn97_calc_iq__phitin) + (assign8670_e9298 * locals.var_fn97_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn97_calc_iq__n_dn14) * locals.var_fn97_calc_iq__phitin), ((2.0 * locals.var_fn97_calc_iq__n_dn15) * locals.var_fn97_calc_iq__phitin),)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit, locals.var_fn97_calc_iq__two_n_phit_dn4, locals.var_fn97_calc_iq__two_n_phit_dn14, locals.var_fn97_calc_iq__two_n_phit_dn15,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit = assign8670_e9302;
        locals.var_fn97_calc_iq__two_n_phit_dn4 = assign8670_e9302_d_n4;
        locals.var_fn97_calc_iq__two_n_phit_dn14 = assign8670_e9302_d_n14;
        locals.var_fn97_calc_iq__two_n_phit_dn15 = assign8670_e9302_d_n15;
        locals.var_fn97_calc_iq__two_n_phit_rv = 0.0;

        let (assign8680_e9308, assign8680_e9308_d_n4, assign8680_e9308_d_n14, assign8680_e9308_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8680_e9306: f64 = (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit);
        (assign8680_e9306, ((locals.var_fn97_calc_iq__cgin_dn4 * locals.var_fn97_calc_iq__two_n_phit) + (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit_dn4)), (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit_dn14), (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qref, locals.var_fn97_calc_iq__qref_dn4, locals.var_fn97_calc_iq__qref_dn14, locals.var_fn97_calc_iq__qref_dn15,)
    }
};
        locals.var_fn97_calc_iq__qref = assign8680_e9308;
        locals.var_fn97_calc_iq__qref_dn4 = assign8680_e9308_d_n4;
        locals.var_fn97_calc_iq__qref_dn14 = assign8680_e9308_d_n14;
        locals.var_fn97_calc_iq__qref_dn15 = assign8680_e9308_d_n15;
        locals.var_fn97_calc_iq__qref_rv = 0.0;

        let (assign8690_e9318, assign8690_e9318_d_n2, assign8690_e9318_d_n3, assign8690_e9318_d_n4, assign8690_e9318_d_n7, assign8690_e9318_d_n14, assign8690_e9318_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8690_e9313: f64 = (p.p51 * locals.var_fn97_calc_iq__alpha_phit);
        let assign8690_e9315: f64 = (assign8690_e9313 / 2.0);
        let assign8690_e9316: f64 = (locals.var_fn97_calc_iq__vtdibl - assign8690_e9315);
        (assign8690_e9316, 0.0, 0.0, (locals.var_fn97_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn97_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn97_calc_iq__vtdibl_dn14, locals.var_fn97_calc_iq__vtdibl_dn15,)
    } else {
        (locals.var_fn97_calc_iq__myarg, locals.var_fn97_calc_iq__myarg_dn2, locals.var_fn97_calc_iq__myarg_dn3, locals.var_fn97_calc_iq__myarg_dn4, locals.var_fn97_calc_iq__myarg_dn7, locals.var_fn97_calc_iq__myarg_dn14, locals.var_fn97_calc_iq__myarg_dn15,)
    }
};
        locals.var_fn97_calc_iq__myarg = assign8690_e9318;
        locals.var_fn97_calc_iq__myarg_dn2 = assign8690_e9318_d_n2;
        locals.var_fn97_calc_iq__myarg_dn3 = assign8690_e9318_d_n3;
        locals.var_fn97_calc_iq__myarg_dn4 = assign8690_e9318_d_n4;
        locals.var_fn97_calc_iq__myarg_dn7 = assign8690_e9318_d_n7;
        locals.var_fn97_calc_iq__myarg_dn14 = assign8690_e9318_d_n14;
        locals.var_fn97_calc_iq__myarg_dn15 = assign8690_e9318_d_n15;
        locals.var_fn97_calc_iq__myarg_rv = 0.0;

        let (assign8700_e9369, assign8700_e9369_d_n2, assign8700_e9369_d_n3, assign8700_e9369_d_n4, assign8700_e9369_d_n7, assign8700_e9369_d_n14, assign8700_e9369_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8700_e9363, assign8700_e9363_d_n2, assign8700_e9363_d_n7, assign8700_e9363_d_n14, assign8700_e9363_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8700_e9327: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign8700_e9330: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8700_e9333: f64 = (0.001 / p.p53);
                let assign8700_e9336: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8700_e9337: f64 = (assign8700_e9333 * assign8700_e9336);
                let assign8700_e9338: f64 = (assign8700_e9337).tanh();
                let assign8700_e9339: f64 = (assign8700_e9330 * assign8700_e9338);
                let assign8700_e9340: f64 = (assign8700_e9327 + assign8700_e9339);
                let assign8700_e9341: f64 = (0.5 * assign8700_e9340);
                (assign8700_e9341, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8700_e9338) + (assign8700_e9330 * ((assign8700_e9333 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign8700_e9337).cosh() * (assign8700_e9337).cosh())))))),)
            } else {
                let (assign8700_e9362, assign8700_e9362_d_n2, assign8700_e9362_d_n7, assign8700_e9362_d_n14, assign8700_e9362_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8700_e9348: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign8700_e9351: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8700_e9354: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8700_e9355: f64 = (assign8700_e9351 * assign8700_e9354);
                        let assign8700_e9357: f64 = (assign8700_e9355 + p.p53);
                        let assign8700_e9358: f64 = (assign8700_e9357).sqrt();
                        let assign8700_e9359: f64 = (assign8700_e9348 + assign8700_e9358);
                        let assign8700_e9360: f64 = (0.5 * assign8700_e9359);
                        (assign8700_e9360, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8700_e9354) + (assign8700_e9351 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign8700_e9358)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8700_e9354) + (assign8700_e9351 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign8700_e9358)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8700_e9354) + (assign8700_e9351 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign8700_e9358)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8700_e9354) + (assign8700_e9351 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign8700_e9358)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8700_e9362, assign8700_e9362_d_n2, assign8700_e9362_d_n7, assign8700_e9362_d_n14, assign8700_e9362_d_n15,)
            }
        };
        let assign8700_e9365: f64 = (assign8700_e9363 - locals.var_fn97_calc_iq__myarg);
        let assign8700_e9367: f64 = (assign8700_e9365 / locals.var_fn97_calc_iq__alpha_phit);
        (assign8700_e9367, ((assign8700_e9363_d_n2 - locals.var_fn97_calc_iq__myarg_dn2) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn3) / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign8700_e9365 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), ((assign8700_e9363_d_n7 - locals.var_fn97_calc_iq__myarg_dn7) / locals.var_fn97_calc_iq__alpha_phit), ((assign8700_e9363_d_n14 - locals.var_fn97_calc_iq__myarg_dn14) / locals.var_fn97_calc_iq__alpha_phit), ((assign8700_e9363_d_n15 - locals.var_fn97_calc_iq__myarg_dn15) / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign8700_e9369;
        locals.var_fn97_calc_iq__exparg_dn2 = assign8700_e9369_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign8700_e9369_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign8700_e9369_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign8700_e9369_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign8700_e9369_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign8700_e9369_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let assign8710_e9372: f64 = if locals.var_fn97_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8710_e9372;
        locals.var_guard99_rv = 0.0;

        let (assign8720_e9378, assign8720_e9378_d_n2, assign8720_e9378_d_n3, assign8720_e9378_d_n4, assign8720_e9378_d_n7, assign8720_e9378_d_n14, assign8720_e9378_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard99 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8720_e9378;
        locals.var_fn97_calc_iq__ff_dn2 = assign8720_e9378_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8720_e9378_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8720_e9378_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8720_e9378_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8720_e9378_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8720_e9378_d_n15;
        locals.var_fn97_calc_iq__ff_rv = 0.0;

        let assign8730_e9381: f64 = (-50.0);
        let assign8730_e9382: f64 = if locals.var_fn97_calc_iq__exparg < assign8730_e9381 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8730_e9382;
        locals.var_guard100_rv = 0.0;

        let (assign8740_e9391, assign8740_e9391_d_n2, assign8740_e9391_d_n3, assign8740_e9391_d_n4, assign8740_e9391_d_n7, assign8740_e9391_d_n14, assign8740_e9391_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard99 == 0.0)) && (locals.var_guard100 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8740_e9391;
        locals.var_fn97_calc_iq__ff_dn2 = assign8740_e9391_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8740_e9391_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8740_e9391_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8740_e9391_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8740_e9391_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8740_e9391_d_n15;
        locals.var_fn97_calc_iq__ff_rv = 0.0;

        let (assign8750_e9406, assign8750_e9406_d_n2, assign8750_e9406_d_n3, assign8750_e9406_d_n4, assign8750_e9406_d_n7, assign8750_e9406_d_n14, assign8750_e9406_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard99 == 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign8750_e9402: f64 = (locals.var_fn97_calc_iq__exparg).exp();
        let assign8750_e9403: f64 = (1.0 + assign8750_e9402);
        let assign8750_e9404: f64 = (1.0 / assign8750_e9403);
        (assign8750_e9404, (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn2) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn3) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn4) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn7) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn14) / (assign8750_e9403 * assign8750_e9403))), (-((assign8750_e9402 * locals.var_fn97_calc_iq__exparg_dn15) / (assign8750_e9403 * assign8750_e9403))),)
    } else {
        (locals.var_fn97_calc_iq__ff, locals.var_fn97_calc_iq__ff_dn2, locals.var_fn97_calc_iq__ff_dn3, locals.var_fn97_calc_iq__ff_dn4, locals.var_fn97_calc_iq__ff_dn7, locals.var_fn97_calc_iq__ff_dn14, locals.var_fn97_calc_iq__ff_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff = assign8750_e9406;
        locals.var_fn97_calc_iq__ff_dn2 = assign8750_e9406_d_n2;
        locals.var_fn97_calc_iq__ff_dn3 = assign8750_e9406_d_n3;
        locals.var_fn97_calc_iq__ff_dn4 = assign8750_e9406_d_n4;
        locals.var_fn97_calc_iq__ff_dn7 = assign8750_e9406_d_n7;
        locals.var_fn97_calc_iq__ff_dn14 = assign8750_e9406_d_n14;
        locals.var_fn97_calc_iq__ff_dn15 = assign8750_e9406_d_n15;
        locals.var_fn97_calc_iq__ff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8760_e9465, assign8760_e9465_d_n2, assign8760_e9465_d_n3, assign8760_e9465_d_n4, assign8760_e9465_d_n7, assign8760_e9465_d_n14, assign8760_e9465_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8760_e9451, assign8760_e9451_d_n2, assign8760_e9451_d_n7, assign8760_e9451_d_n14, assign8760_e9451_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8760_e9415: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign8760_e9418: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8760_e9421: f64 = (0.001 / p.p53);
                let assign8760_e9424: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign8760_e9425: f64 = (assign8760_e9421 * assign8760_e9424);
                let assign8760_e9426: f64 = (assign8760_e9425).tanh();
                let assign8760_e9427: f64 = (assign8760_e9418 * assign8760_e9426);
                let assign8760_e9428: f64 = (assign8760_e9415 + assign8760_e9427);
                let assign8760_e9429: f64 = (0.5 * assign8760_e9428);
                (assign8760_e9429, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8760_e9426) + (assign8760_e9418 * ((assign8760_e9421 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign8760_e9425).cosh() * (assign8760_e9425).cosh())))))),)
            } else {
                let (assign8760_e9450, assign8760_e9450_d_n2, assign8760_e9450_d_n7, assign8760_e9450_d_n14, assign8760_e9450_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8760_e9436: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign8760_e9439: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8760_e9442: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign8760_e9443: f64 = (assign8760_e9439 * assign8760_e9442);
                        let assign8760_e9445: f64 = (assign8760_e9443 + p.p53);
                        let assign8760_e9446: f64 = (assign8760_e9445).sqrt();
                        let assign8760_e9447: f64 = (assign8760_e9436 + assign8760_e9446);
                        let assign8760_e9448: f64 = (0.5 * assign8760_e9447);
                        (assign8760_e9448, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign8760_e9442) + (assign8760_e9439 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign8760_e9446)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign8760_e9442) + (assign8760_e9439 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign8760_e9446)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign8760_e9442) + (assign8760_e9439 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign8760_e9446)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign8760_e9442) + (assign8760_e9439 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign8760_e9446)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8760_e9450, assign8760_e9450_d_n2, assign8760_e9450_d_n7, assign8760_e9450_d_n14, assign8760_e9450_d_n15,)
            }
        };
        let assign8760_e9455: f64 = (p.p51 * 0.1);
        let assign8760_e9457: f64 = (assign8760_e9455 * locals.var_fn97_calc_iq__alpha_phit);
        let assign8760_e9459: f64 = (assign8760_e9457 * locals.var_fn97_calc_iq__ff);
        let assign8760_e9460: f64 = (locals.var_fn97_calc_iq__vtdibl - assign8760_e9459);
        let assign8760_e9461: f64 = (assign8760_e9451 - assign8760_e9460);
        let assign8760_e9463: f64 = (assign8760_e9461 / locals.var_fn97_calc_iq__two_n_phit);
        (assign8760_e9463, ((assign8760_e9451_d_n2 - (-(assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn2))) / locals.var_fn97_calc_iq__two_n_phit), ((-(-(assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn3))) / locals.var_fn97_calc_iq__two_n_phit), ((((-(locals.var_fn97_calc_iq__vtdibl_dn4 - (((assign8760_e9455 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ff) + (assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn4)))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8760_e9461 * locals.var_fn97_calc_iq__two_n_phit_dn4)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), ((assign8760_e9451_d_n7 - (-(assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn7))) / locals.var_fn97_calc_iq__two_n_phit), ((((assign8760_e9451_d_n14 - (locals.var_fn97_calc_iq__vtdibl_dn14 - (assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn14))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8760_e9461 * locals.var_fn97_calc_iq__two_n_phit_dn14)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), ((((assign8760_e9451_d_n15 - (locals.var_fn97_calc_iq__vtdibl_dn15 - (assign8760_e9457 * locals.var_fn97_calc_iq__ff_dn15))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8760_e9461 * locals.var_fn97_calc_iq__two_n_phit_dn15)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn97_calc_iq__eta, locals.var_fn97_calc_iq__eta_dn2, locals.var_fn97_calc_iq__eta_dn3, locals.var_fn97_calc_iq__eta_dn4, locals.var_fn97_calc_iq__eta_dn7, locals.var_fn97_calc_iq__eta_dn14, locals.var_fn97_calc_iq__eta_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta = assign8760_e9465;
        locals.var_fn97_calc_iq__eta_dn2 = assign8760_e9465_d_n2;
        locals.var_fn97_calc_iq__eta_dn3 = assign8760_e9465_d_n3;
        locals.var_fn97_calc_iq__eta_dn4 = assign8760_e9465_d_n4;
        locals.var_fn97_calc_iq__eta_dn7 = assign8760_e9465_d_n7;
        locals.var_fn97_calc_iq__eta_dn14 = assign8760_e9465_d_n14;
        locals.var_fn97_calc_iq__eta_dn15 = assign8760_e9465_d_n15;
        locals.var_fn97_calc_iq__eta_rv = 0.0;

        let assign8770_e9468: f64 = if locals.var_fn97_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8770_e9468;
        locals.var_guard101_rv = 0.0;

        let (assign8780_e9476, assign8780_e9476_d_n2, assign8780_e9476_d_n3, assign8780_e9476_d_n4, assign8780_e9476_d_n7, assign8780_e9476_d_n14, assign8780_e9476_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8780_e9474: f64 = (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta);
        (assign8780_e9474, (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn2), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn3), ((locals.var_fn97_calc_iq__qref_dn4 * locals.var_fn97_calc_iq__eta) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn4)), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn7), ((locals.var_fn97_calc_iq__qref_dn14 * locals.var_fn97_calc_iq__eta) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn14)), ((locals.var_fn97_calc_iq__qref_dn15 * locals.var_fn97_calc_iq__eta) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__eta_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8780_e9476;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8780_e9476_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8780_e9476_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8780_e9476_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8780_e9476_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8780_e9476_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8780_e9476_d_n15;
        locals.var_fn97_calc_iq__qinvv_rv = 0.0;

        let assign8790_e9479: f64 = (-50.0);
        let assign8790_e9480: f64 = if locals.var_fn97_calc_iq__eta < assign8790_e9479 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8790_e9480;
        locals.var_guard102_rv = 0.0;

        let (assign8800_e9492, assign8800_e9492_d_n2, assign8800_e9492_d_n3, assign8800_e9492_d_n4, assign8800_e9492_d_n7, assign8800_e9492_d_n14, assign8800_e9492_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard101 == 0.0)) && (locals.var_guard102 != 0.0)) {
        let assign8800_e9489: f64 = (locals.var_fn97_calc_iq__eta).exp();
        let assign8800_e9490: f64 = (locals.var_fn97_calc_iq__qref * assign8800_e9489);
        (assign8800_e9490, (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn2)), (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn3)), ((locals.var_fn97_calc_iq__qref_dn4 * assign8800_e9489) + (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn4))), (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn7)), ((locals.var_fn97_calc_iq__qref_dn14 * assign8800_e9489) + (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn14))), ((locals.var_fn97_calc_iq__qref_dn15 * assign8800_e9489) + (locals.var_fn97_calc_iq__qref * (assign8800_e9489 * locals.var_fn97_calc_iq__eta_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8800_e9492;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8800_e9492_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8800_e9492_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8800_e9492_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8800_e9492_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8800_e9492_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8800_e9492_d_n15;
        locals.var_fn97_calc_iq__qinvv_rv = 0.0;

        let (assign8810_e9508, assign8810_e9508_d_n2, assign8810_e9508_d_n3, assign8810_e9508_d_n4, assign8810_e9508_d_n7, assign8810_e9508_d_n14, assign8810_e9508_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard101 == 0.0)) && (locals.var_guard102 == 0.0)) {
        let assign8810_e9503: f64 = (locals.var_fn97_calc_iq__eta).exp();
        let assign8810_e9504: f64 = (1.0 + assign8810_e9503);
        let assign8810_e9505: f64 = (assign8810_e9504).ln();
        let assign8810_e9506: f64 = (locals.var_fn97_calc_iq__qref * assign8810_e9505);
        (assign8810_e9506, (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn2) / assign8810_e9504)), (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn3) / assign8810_e9504)), ((locals.var_fn97_calc_iq__qref_dn4 * assign8810_e9505) + (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn4) / assign8810_e9504))), (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn7) / assign8810_e9504)), ((locals.var_fn97_calc_iq__qref_dn14 * assign8810_e9505) + (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn14) / assign8810_e9504))), ((locals.var_fn97_calc_iq__qref_dn15 * assign8810_e9505) + (locals.var_fn97_calc_iq__qref * ((assign8810_e9503 * locals.var_fn97_calc_iq__eta_dn15) / assign8810_e9504))),)
    } else {
        (locals.var_fn97_calc_iq__qinvv, locals.var_fn97_calc_iq__qinvv_dn2, locals.var_fn97_calc_iq__qinvv_dn3, locals.var_fn97_calc_iq__qinvv_dn4, locals.var_fn97_calc_iq__qinvv_dn7, locals.var_fn97_calc_iq__qinvv_dn14, locals.var_fn97_calc_iq__qinvv_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv = assign8810_e9508;
        locals.var_fn97_calc_iq__qinvv_dn2 = assign8810_e9508_d_n2;
        locals.var_fn97_calc_iq__qinvv_dn3 = assign8810_e9508_d_n3;
        locals.var_fn97_calc_iq__qinvv_dn4 = assign8810_e9508_d_n4;
        locals.var_fn97_calc_iq__qinvv_dn7 = assign8810_e9508_d_n7;
        locals.var_fn97_calc_iq__qinvv_dn14 = assign8810_e9508_d_n14;
        locals.var_fn97_calc_iq__qinvv_dn15 = assign8810_e9508_d_n15;
        locals.var_fn97_calc_iq__qinvv_rv = 0.0;

        let (assign8820_e9522, assign8820_e9522_d_n2, assign8820_e9522_d_n3, assign8820_e9522_d_n4, assign8820_e9522_d_n7, assign8820_e9522_d_n14, assign8820_e9522_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8820_e9515: f64 = (locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv);
        let assign8820_e9517: f64 = (assign8820_e9515 / locals.var_fn97_calc_iq__cgin);
        let assign8820_e9518: f64 = (1.0 + assign8820_e9517);
        let assign8820_e9519: f64 = (locals.var_fn97_calc_iq__tfacmobin * assign8820_e9518);
        let assign8820_e9520: f64 = (locals.var_fn97_calc_iq__mu0 / assign8820_e9519);
        (assign8820_e9520, (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn2) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn3) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * ((locals.var_fn97_calc_iq__tfacmobin_dn4 * assign8820_e9518) + (locals.var_fn97_calc_iq__tfacmobin * ((((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn4) * locals.var_fn97_calc_iq__cgin) - (assign8820_e9515 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin))))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn7) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn14) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))), (-((locals.var_fn97_calc_iq__mu0 * (locals.var_fn97_calc_iq__tfacmobin * ((locals.var_fn97_calc_iq__mtheta * locals.var_fn97_calc_iq__qinvv_dn15) / locals.var_fn97_calc_iq__cgin))) / (assign8820_e9519 * assign8820_e9519))),)
    } else {
        (locals.var_fn97_calc_iq__muf, locals.var_fn97_calc_iq__muf_dn2, locals.var_fn97_calc_iq__muf_dn3, locals.var_fn97_calc_iq__muf_dn4, locals.var_fn97_calc_iq__muf_dn7, locals.var_fn97_calc_iq__muf_dn14, locals.var_fn97_calc_iq__muf_dn15,)
    }
};
        locals.var_fn97_calc_iq__muf = assign8820_e9522;
        locals.var_fn97_calc_iq__muf_dn2 = assign8820_e9522_d_n2;
        locals.var_fn97_calc_iq__muf_dn3 = assign8820_e9522_d_n3;
        locals.var_fn97_calc_iq__muf_dn4 = assign8820_e9522_d_n4;
        locals.var_fn97_calc_iq__muf_dn7 = assign8820_e9522_d_n7;
        locals.var_fn97_calc_iq__muf_dn14 = assign8820_e9522_d_n14;
        locals.var_fn97_calc_iq__muf_dn15 = assign8820_e9522_d_n15;
        locals.var_fn97_calc_iq__muf_rv = 0.0;

        let (assign8830_e9554, assign8830_e9554_d_n2, assign8830_e9554_d_n3, assign8830_e9554_d_n4, assign8830_e9554_d_n7, assign8830_e9554_d_n14, assign8830_e9554_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8830_e9528: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tnomin);
        let assign8830_e9529: f64 = (1.0 + assign8830_e9528);
        let assign8830_e9533: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin);
        let assign8830_e9534: f64 = (1.0 + assign8830_e9533);
        let assign8830_e9535: f64 = (assign8830_e9529 / assign8830_e9534);
        let assign8830_e9536: f64 = (locals.var_fn97_calc_iq__vel0 * assign8830_e9535);
        let assign8830_e9540: f64 = (locals.var_fn97_calc_iq__lambda * locals.var_fn97_calc_iq__absvdsin);
        let assign8830_e9542: f64 = (assign8830_e9540 / locals.var_fn97_calc_iq__lin);
        let assign8830_e9543: f64 = (1.0 + assign8830_e9542);
        let assign8830_e9544: f64 = (assign8830_e9536 * assign8830_e9543);
        let assign8830_e9548: f64 = (locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv);
        let assign8830_e9550: f64 = (assign8830_e9548 / locals.var_fn97_calc_iq__cgin);
        let assign8830_e9551: f64 = (1.0 + assign8830_e9550);
        let assign8830_e9552: f64 = (assign8830_e9544 / assign8830_e9551);
        (assign8830_e9552, (-((assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn2) / locals.var_fn97_calc_iq__cgin)) / (assign8830_e9551 * assign8830_e9551))), (-((assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn3) / locals.var_fn97_calc_iq__cgin)) / (assign8830_e9551 * assign8830_e9551))), (((((locals.var_fn97_calc_iq__vel0 * (-((assign8830_e9529 * (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin_dn4)) / (assign8830_e9534 * assign8830_e9534)))) * assign8830_e9543) * assign8830_e9551) - (assign8830_e9544 * ((((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn4) * locals.var_fn97_calc_iq__cgin) - (assign8830_e9548 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)))) / (assign8830_e9551 * assign8830_e9551)), (-((assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn7) / locals.var_fn97_calc_iq__cgin)) / (assign8830_e9551 * assign8830_e9551))), ((((assign8830_e9536 * ((locals.var_fn97_calc_iq__lambda * locals.var_fn97_calc_iq__absvdsin_dn14) / locals.var_fn97_calc_iq__lin)) * assign8830_e9551) - (assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn14) / locals.var_fn97_calc_iq__cgin))) / (assign8830_e9551 * assign8830_e9551)), ((((assign8830_e9536 * ((locals.var_fn97_calc_iq__lambda * locals.var_fn97_calc_iq__absvdsin_dn15) / locals.var_fn97_calc_iq__lin)) * assign8830_e9551) - (assign8830_e9544 * ((locals.var_fn97_calc_iq__vtheta * locals.var_fn97_calc_iq__qinvv_dn15) / locals.var_fn97_calc_iq__cgin))) / (assign8830_e9551 * assign8830_e9551)),)
    } else {
        (locals.var_fn97_calc_iq__vx, locals.var_fn97_calc_iq__vx_dn2, locals.var_fn97_calc_iq__vx_dn3, locals.var_fn97_calc_iq__vx_dn4, locals.var_fn97_calc_iq__vx_dn7, locals.var_fn97_calc_iq__vx_dn14, locals.var_fn97_calc_iq__vx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vx = assign8830_e9554;
        locals.var_fn97_calc_iq__vx_dn2 = assign8830_e9554_d_n2;
        locals.var_fn97_calc_iq__vx_dn3 = assign8830_e9554_d_n3;
        locals.var_fn97_calc_iq__vx_dn4 = assign8830_e9554_d_n4;
        locals.var_fn97_calc_iq__vx_dn7 = assign8830_e9554_d_n7;
        locals.var_fn97_calc_iq__vx_dn14 = assign8830_e9554_d_n14;
        locals.var_fn97_calc_iq__vx_dn15 = assign8830_e9554_d_n15;
        locals.var_fn97_calc_iq__vx_rv = 0.0;

        let (assign8850_e9580, assign8850_e9580_d_n2, assign8850_e9580_d_n3, assign8850_e9580_d_n4, assign8850_e9580_d_n7, assign8850_e9580_d_n14, assign8850_e9580_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8850_e9576: f64 = (locals.var_fn97_calc_iq__vx * locals.var_fn97_calc_iq__lin);
        let assign8850_e9578: f64 = (assign8850_e9576 / locals.var_fn97_calc_iq__muf);
        (assign8850_e9578, ((((locals.var_fn97_calc_iq__vx_dn2 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn2)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn3 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn3)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn4 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn4)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn7 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn7)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn14 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn14)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)), ((((locals.var_fn97_calc_iq__vx_dn15 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf) - (assign8850_e9576 * locals.var_fn97_calc_iq__muf_dn15)) / (locals.var_fn97_calc_iq__muf * locals.var_fn97_calc_iq__muf)),)
    } else {
        (locals.var_fn97_calc_iq__vdsats, locals.var_fn97_calc_iq__vdsats_dn2, locals.var_fn97_calc_iq__vdsats_dn3, locals.var_fn97_calc_iq__vdsats_dn4, locals.var_fn97_calc_iq__vdsats_dn7, locals.var_fn97_calc_iq__vdsats_dn14, locals.var_fn97_calc_iq__vdsats_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats = assign8850_e9580;
        locals.var_fn97_calc_iq__vdsats_dn2 = assign8850_e9580_d_n2;
        locals.var_fn97_calc_iq__vdsats_dn3 = assign8850_e9580_d_n3;
        locals.var_fn97_calc_iq__vdsats_dn4 = assign8850_e9580_d_n4;
        locals.var_fn97_calc_iq__vdsats_dn7 = assign8850_e9580_d_n7;
        locals.var_fn97_calc_iq__vdsats_dn14 = assign8850_e9580_d_n14;
        locals.var_fn97_calc_iq__vdsats_dn15 = assign8850_e9580_d_n15;
        locals.var_fn97_calc_iq__vdsats_rv = 0.0;

        let (assign8860_e9597, assign8860_e9597_d_n2, assign8860_e9597_d_n3, assign8860_e9597_d_n4, assign8860_e9597_d_n7, assign8860_e9597_d_n14, assign8860_e9597_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8860_e9586: f64 = (2.0 * locals.var_fn97_calc_iq__qinvv);
        let assign8860_e9588: f64 = (assign8860_e9586 / locals.var_fn97_calc_iq__cgin);
        let assign8860_e9590: f64 = (assign8860_e9588 / locals.var_fn97_calc_iq__vdsats);
        let assign8860_e9591: f64 = (1.0 + assign8860_e9590);
        let assign8860_e9592: f64 = (assign8860_e9591).sqrt();
        let assign8860_e9593: f64 = (locals.var_fn97_calc_iq__vdsats * assign8860_e9592);
        let assign8860_e9595: f64 = (assign8860_e9593 - locals.var_fn97_calc_iq__vdsats);
        (assign8860_e9595, (((locals.var_fn97_calc_iq__vdsats_dn2 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn2) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn2)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn2), (((locals.var_fn97_calc_iq__vdsats_dn3 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn3) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn3)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn3), (((locals.var_fn97_calc_iq__vdsats_dn4 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn4) * locals.var_fn97_calc_iq__cgin) - (assign8860_e9586 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn4)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn4), (((locals.var_fn97_calc_iq__vdsats_dn7 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn7) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn7)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn7), (((locals.var_fn97_calc_iq__vdsats_dn14 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn14) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn14)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn14), (((locals.var_fn97_calc_iq__vdsats_dn15 * assign8860_e9592) + (locals.var_fn97_calc_iq__vdsats * ((((((2.0 * locals.var_fn97_calc_iq__qinvv_dn15) / locals.var_fn97_calc_iq__cgin) * locals.var_fn97_calc_iq__vdsats) - (assign8860_e9588 * locals.var_fn97_calc_iq__vdsats_dn15)) / (locals.var_fn97_calc_iq__vdsats * locals.var_fn97_calc_iq__vdsats)) / (2.0 * assign8860_e9592)))) - locals.var_fn97_calc_iq__vdsats_dn15),)
    } else {
        (locals.var_fn97_calc_iq__vdsats1, locals.var_fn97_calc_iq__vdsats1_dn2, locals.var_fn97_calc_iq__vdsats1_dn3, locals.var_fn97_calc_iq__vdsats1_dn4, locals.var_fn97_calc_iq__vdsats1_dn7, locals.var_fn97_calc_iq__vdsats1_dn14, locals.var_fn97_calc_iq__vdsats1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats1 = assign8860_e9597;
        locals.var_fn97_calc_iq__vdsats1_dn2 = assign8860_e9597_d_n2;
        locals.var_fn97_calc_iq__vdsats1_dn3 = assign8860_e9597_d_n3;
        locals.var_fn97_calc_iq__vdsats1_dn4 = assign8860_e9597_d_n4;
        locals.var_fn97_calc_iq__vdsats1_dn7 = assign8860_e9597_d_n7;
        locals.var_fn97_calc_iq__vdsats1_dn14 = assign8860_e9597_d_n14;
        locals.var_fn97_calc_iq__vdsats1_dn15 = assign8860_e9597_d_n15;
        locals.var_fn97_calc_iq__vdsats1_rv = 0.0;

        let (assign8870_e9609, assign8870_e9609_d_n2, assign8870_e9609_d_n3, assign8870_e9609_d_n4, assign8870_e9609_d_n7, assign8870_e9609_d_n14, assign8870_e9609_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8870_e9602: f64 = (1.0 - locals.var_fn97_calc_iq__ff);
        let assign8870_e9603: f64 = (locals.var_fn97_calc_iq__vdsats * assign8870_e9602);
        let assign8870_e9606: f64 = (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff);
        let assign8870_e9607: f64 = (assign8870_e9603 + assign8870_e9606);
        (assign8870_e9607, (((locals.var_fn97_calc_iq__vdsats_dn2 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn2))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn2)), (((locals.var_fn97_calc_iq__vdsats_dn3 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn3))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn3)), (((locals.var_fn97_calc_iq__vdsats_dn4 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn4))) + ((locals.var_fn97_calc_iq__two_n_phit_dn4 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn4))), (((locals.var_fn97_calc_iq__vdsats_dn7 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn7))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn7)), (((locals.var_fn97_calc_iq__vdsats_dn14 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn14))) + ((locals.var_fn97_calc_iq__two_n_phit_dn14 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn14))), (((locals.var_fn97_calc_iq__vdsats_dn15 * assign8870_e9602) + (locals.var_fn97_calc_iq__vdsats * (-locals.var_fn97_calc_iq__ff_dn15))) + ((locals.var_fn97_calc_iq__two_n_phit_dn15 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__vdsat, locals.var_fn97_calc_iq__vdsat_dn2, locals.var_fn97_calc_iq__vdsat_dn3, locals.var_fn97_calc_iq__vdsat_dn4, locals.var_fn97_calc_iq__vdsat_dn7, locals.var_fn97_calc_iq__vdsat_dn14, locals.var_fn97_calc_iq__vdsat_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat = assign8870_e9609;
        locals.var_fn97_calc_iq__vdsat_dn2 = assign8870_e9609_d_n2;
        locals.var_fn97_calc_iq__vdsat_dn3 = assign8870_e9609_d_n3;
        locals.var_fn97_calc_iq__vdsat_dn4 = assign8870_e9609_d_n4;
        locals.var_fn97_calc_iq__vdsat_dn7 = assign8870_e9609_d_n7;
        locals.var_fn97_calc_iq__vdsat_dn14 = assign8870_e9609_d_n14;
        locals.var_fn97_calc_iq__vdsat_dn15 = assign8870_e9609_d_n15;
        locals.var_fn97_calc_iq__vdsat_rv = 0.0;

        let (assign8880_e9621, assign8880_e9621_d_n2, assign8880_e9621_d_n3, assign8880_e9621_d_n4, assign8880_e9621_d_n7, assign8880_e9621_d_n14, assign8880_e9621_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8880_e9614: f64 = (1.0 - locals.var_fn97_calc_iq__ff);
        let assign8880_e9615: f64 = (locals.var_fn97_calc_iq__vdsats1 * assign8880_e9614);
        let assign8880_e9618: f64 = (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff);
        let assign8880_e9619: f64 = (assign8880_e9615 + assign8880_e9618);
        (assign8880_e9619, (((locals.var_fn97_calc_iq__vdsats1_dn2 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn2))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn2)), (((locals.var_fn97_calc_iq__vdsats1_dn3 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn3))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn3)), (((locals.var_fn97_calc_iq__vdsats1_dn4 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn4))) + ((locals.var_fn97_calc_iq__two_n_phit_dn4 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn4))), (((locals.var_fn97_calc_iq__vdsats1_dn7 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn7))) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn7)), (((locals.var_fn97_calc_iq__vdsats1_dn14 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn14))) + ((locals.var_fn97_calc_iq__two_n_phit_dn14 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn14))), (((locals.var_fn97_calc_iq__vdsats1_dn15 * assign8880_e9614) + (locals.var_fn97_calc_iq__vdsats1 * (-locals.var_fn97_calc_iq__ff_dn15))) + ((locals.var_fn97_calc_iq__two_n_phit_dn15 * locals.var_fn97_calc_iq__ff) + (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__ff_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__vdsat1, locals.var_fn97_calc_iq__vdsat1_dn2, locals.var_fn97_calc_iq__vdsat1_dn3, locals.var_fn97_calc_iq__vdsat1_dn4, locals.var_fn97_calc_iq__vdsat1_dn7, locals.var_fn97_calc_iq__vdsat1_dn14, locals.var_fn97_calc_iq__vdsat1_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat1 = assign8880_e9621;
        locals.var_fn97_calc_iq__vdsat1_dn2 = assign8880_e9621_d_n2;
        locals.var_fn97_calc_iq__vdsat1_dn3 = assign8880_e9621_d_n3;
        locals.var_fn97_calc_iq__vdsat1_dn4 = assign8880_e9621_d_n4;
        locals.var_fn97_calc_iq__vdsat1_dn7 = assign8880_e9621_d_n7;
        locals.var_fn97_calc_iq__vdsat1_dn14 = assign8880_e9621_d_n14;
        locals.var_fn97_calc_iq__vdsat1_dn15 = assign8880_e9621_d_n15;
        locals.var_fn97_calc_iq__vdsat1_rv = 0.0;

        let (assign8890_e9690, assign8890_e9690_d_n2, assign8890_e9690_d_n3, assign8890_e9690_d_n4, assign8890_e9690_d_n7, assign8890_e9690_d_n14, assign8890_e9690_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8890_e9680, assign8890_e9680_d_n2, assign8890_e9680_d_n3, assign8890_e9680_d_n4, assign8890_e9680_d_n7, assign8890_e9680_d_n14, assign8890_e9680_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8890_e9633: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                let assign8890_e9634: f64 = assign8890_e9633;
                let assign8890_e9638: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                let assign8890_e9639: f64 = (-assign8890_e9638);
                let assign8890_e9642: f64 = (0.001 / p.p53);
                let assign8890_e9646: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                let assign8890_e9647: f64 = (-assign8890_e9646);
                let assign8890_e9648: f64 = (assign8890_e9642 * assign8890_e9647);
                let assign8890_e9649: f64 = (assign8890_e9648).tanh();
                let assign8890_e9650: f64 = (assign8890_e9639 * assign8890_e9649);
                let assign8890_e9651: f64 = (assign8890_e9634 + assign8890_e9650);
                let assign8890_e9652: f64 = (0.5 * assign8890_e9651);
                (assign8890_e9652, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9649) + (assign8890_e9639 * ((assign8890_e9642 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8890_e9648).cosh() * (assign8890_e9648).cosh())))))),)
            } else {
                let (assign8890_e9679, assign8890_e9679_d_n2, assign8890_e9679_d_n3, assign8890_e9679_d_n4, assign8890_e9679_d_n7, assign8890_e9679_d_n14, assign8890_e9679_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8890_e9660: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                        let assign8890_e9661: f64 = assign8890_e9660;
                        let assign8890_e9665: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                        let assign8890_e9666: f64 = (-assign8890_e9665);
                        let assign8890_e9670: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat1);
                        let assign8890_e9671: f64 = (-assign8890_e9670);
                        let assign8890_e9672: f64 = (assign8890_e9666 * assign8890_e9671);
                        let assign8890_e9674: f64 = (assign8890_e9672 + p.p53);
                        let assign8890_e9675: f64 = (assign8890_e9674).sqrt();
                        let assign8890_e9676: f64 = (assign8890_e9661 + assign8890_e9675);
                        let assign8890_e9677: f64 = (0.5 * assign8890_e9676);
                        (assign8890_e9677, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8890_e9671) + (assign8890_e9666 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8890_e9675)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9671) + (assign8890_e9666 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8890_e9675)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8890_e9671) + (assign8890_e9666 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat1) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8890_e9675)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8890_e9679, assign8890_e9679_d_n2, assign8890_e9679_d_n3, assign8890_e9679_d_n4, assign8890_e9679_d_n7, assign8890_e9679_d_n14, assign8890_e9679_d_n15,)
            }
        };
        let assign8890_e9682: f64 = (assign8890_e9680).powf(locals.var_fn97_calc_iq__beta);
        let assign8890_e9683: f64 = (1.0 + assign8890_e9682);
        let assign8890_e9686: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign8890_e9687: f64 = (assign8890_e9683).powf(assign8890_e9686);
        let assign8890_e9688: f64 = (1.0 / assign8890_e9687);
        (assign8890_e9688, (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n2)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n2 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n2)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n2 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n3)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n3 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n3)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n3 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n4)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n4 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n4)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n4 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n7)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n7 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n7)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n7 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n14)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n14 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n14)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n14 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))), (-(if 0.0 == 0.0 && ((assign8890_e9686) as f64).is_finite() && ((assign8890_e9686) as f64).fract() == 0.0 { if assign8890_e9686 == 0.0 { 0.0 } else { (assign8890_e9686 * ((assign8890_e9683).powf(assign8890_e9686 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n15)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n15 / assign8890_e9680))) })) } } else { (assign8890_e9687 * (assign8890_e9686 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8890_e9680).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8890_e9680_d_n15)) } } else { (assign8890_e9682 * (locals.var_fn97_calc_iq__beta * (assign8890_e9680_d_n15 / assign8890_e9680))) } / assign8890_e9683))) } / (assign8890_e9687 * assign8890_e9687))),)
    } else {
        (locals.var_fn97_calc_iq__fsd, locals.var_fn97_calc_iq__fsd_dn2, locals.var_fn97_calc_iq__fsd_dn3, locals.var_fn97_calc_iq__fsd_dn4, locals.var_fn97_calc_iq__fsd_dn7, locals.var_fn97_calc_iq__fsd_dn14, locals.var_fn97_calc_iq__fsd_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd = assign8890_e9690;
        locals.var_fn97_calc_iq__fsd_dn2 = assign8890_e9690_d_n2;
        locals.var_fn97_calc_iq__fsd_dn3 = assign8890_e9690_d_n3;
        locals.var_fn97_calc_iq__fsd_dn4 = assign8890_e9690_d_n4;
        locals.var_fn97_calc_iq__fsd_dn7 = assign8890_e9690_d_n7;
        locals.var_fn97_calc_iq__fsd_dn14 = assign8890_e9690_d_n14;
        locals.var_fn97_calc_iq__fsd_dn15 = assign8890_e9690_d_n15;
        locals.var_fn97_calc_iq__fsd_rv = 0.0;

        let (assign8900_e9696, assign8900_e9696_d_n2, assign8900_e9696_d_n3, assign8900_e9696_d_n4, assign8900_e9696_d_n7, assign8900_e9696_d_n14, assign8900_e9696_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8900_e9694: f64 = (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd);
        (assign8900_e9694, (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn2), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn3), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn4), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn7), ((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__fsd) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn14)), ((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__fsd) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vdx, locals.var_fn97_calc_iq__vdx_dn2, locals.var_fn97_calc_iq__vdx_dn3, locals.var_fn97_calc_iq__vdx_dn4, locals.var_fn97_calc_iq__vdx_dn7, locals.var_fn97_calc_iq__vdx_dn14, locals.var_fn97_calc_iq__vdx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx = assign8900_e9696;
        locals.var_fn97_calc_iq__vdx_dn2 = assign8900_e9696_d_n2;
        locals.var_fn97_calc_iq__vdx_dn3 = assign8900_e9696_d_n3;
        locals.var_fn97_calc_iq__vdx_dn4 = assign8900_e9696_d_n4;
        locals.var_fn97_calc_iq__vdx_dn7 = assign8900_e9696_d_n7;
        locals.var_fn97_calc_iq__vdx_dn14 = assign8900_e9696_d_n14;
        locals.var_fn97_calc_iq__vdx_dn15 = assign8900_e9696_d_n15;
        locals.var_fn97_calc_iq__vdx_rv = 0.0;

        let (assign8910_e9771, assign8910_e9771_d_n2, assign8910_e9771_d_n3, assign8910_e9771_d_n4, assign8910_e9771_d_n7, assign8910_e9771_d_n14, assign8910_e9771_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign8910_e9761, assign8910_e9761_d_n2, assign8910_e9761_d_n3, assign8910_e9761_d_n4, assign8910_e9761_d_n7, assign8910_e9761_d_n14, assign8910_e9761_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign8910_e9707: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign8910_e9709: f64 = (assign8910_e9707 / locals.var_fn97_calc_iq__vdsat1);
                let assign8910_e9710: f64 = assign8910_e9709;
                let assign8910_e9713: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign8910_e9715: f64 = (assign8910_e9713 / locals.var_fn97_calc_iq__vdsat1);
                let assign8910_e9716: f64 = (-assign8910_e9715);
                let assign8910_e9719: f64 = (0.001 / p.p53);
                let assign8910_e9722: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign8910_e9724: f64 = (assign8910_e9722 / locals.var_fn97_calc_iq__vdsat1);
                let assign8910_e9725: f64 = (-assign8910_e9724);
                let assign8910_e9726: f64 = (assign8910_e9719 * assign8910_e9725);
                let assign8910_e9727: f64 = (assign8910_e9726).tanh();
                let assign8910_e9728: f64 = (assign8910_e9716 * assign8910_e9727);
                let assign8910_e9729: f64 = (assign8910_e9710 + assign8910_e9728);
                let assign8910_e9730: f64 = (0.5 * assign8910_e9729);
                (assign8910_e9730, (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * ((-((assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + (((-(-((assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-(-((assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9707 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9713 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9727) + (assign8910_e9716 * ((assign8910_e9719 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9722 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) / ((assign8910_e9726).cosh() * (assign8910_e9726).cosh())))))),)
            } else {
                let (assign8910_e9760, assign8910_e9760_d_n2, assign8910_e9760_d_n3, assign8910_e9760_d_n4, assign8910_e9760_d_n7, assign8910_e9760_d_n14, assign8910_e9760_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign8910_e9737: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign8910_e9739: f64 = (assign8910_e9737 / locals.var_fn97_calc_iq__vdsat1);
                        let assign8910_e9740: f64 = assign8910_e9739;
                        let assign8910_e9743: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign8910_e9745: f64 = (assign8910_e9743 / locals.var_fn97_calc_iq__vdsat1);
                        let assign8910_e9746: f64 = (-assign8910_e9745);
                        let assign8910_e9749: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign8910_e9751: f64 = (assign8910_e9749 / locals.var_fn97_calc_iq__vdsat1);
                        let assign8910_e9752: f64 = (-assign8910_e9751);
                        let assign8910_e9753: f64 = (assign8910_e9746 * assign8910_e9752);
                        let assign8910_e9755: f64 = (assign8910_e9753 + p.p53);
                        let assign8910_e9756: f64 = (assign8910_e9755).sqrt();
                        let assign8910_e9757: f64 = (assign8910_e9740 + assign8910_e9756);
                        let assign8910_e9758: f64 = (0.5 * assign8910_e9757);
                        (assign8910_e9758, (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn2) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn3) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn4) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * ((-((assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) + ((((-(-((assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))) * assign8910_e9752) + (assign8910_e9746 * (-(-((assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn7) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)))))) / (2.0 * assign8910_e9756)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9752) + (assign8910_e9746 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn14)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8910_e9756)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9737 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9743 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))) * assign8910_e9752) + (assign8910_e9746 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat1) - (assign8910_e9749 * locals.var_fn97_calc_iq__vdsat1_dn15)) / (locals.var_fn97_calc_iq__vdsat1 * locals.var_fn97_calc_iq__vdsat1))))) / (2.0 * assign8910_e9756)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign8910_e9760, assign8910_e9760_d_n2, assign8910_e9760_d_n3, assign8910_e9760_d_n4, assign8910_e9760_d_n7, assign8910_e9760_d_n14, assign8910_e9760_d_n15,)
            }
        };
        let assign8910_e9763: f64 = (assign8910_e9761).powf(locals.var_fn97_calc_iq__beta);
        let assign8910_e9764: f64 = (1.0 + assign8910_e9763);
        let assign8910_e9767: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign8910_e9768: f64 = (assign8910_e9764).powf(assign8910_e9767);
        let assign8910_e9769: f64 = (1.0 / assign8910_e9768);
        (assign8910_e9769, (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n2)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n2 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n2)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n2 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n3)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n3 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n3)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n3 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n4)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n4 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n4)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n4 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n7)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n7 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n7)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n7 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n14)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n14 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n14)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n14 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))), (-(if 0.0 == 0.0 && ((assign8910_e9767) as f64).is_finite() && ((assign8910_e9767) as f64).fract() == 0.0 { if assign8910_e9767 == 0.0 { 0.0 } else { (assign8910_e9767 * ((assign8910_e9764).powf(assign8910_e9767 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n15)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n15 / assign8910_e9761))) })) } } else { (assign8910_e9768 * (assign8910_e9767 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign8910_e9761).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign8910_e9761_d_n15)) } } else { (assign8910_e9763 * (locals.var_fn97_calc_iq__beta * (assign8910_e9761_d_n15 / assign8910_e9761))) } / assign8910_e9764))) } / (assign8910_e9768 * assign8910_e9768))),)
    } else {
        (locals.var_fn97_calc_iq__fds, locals.var_fn97_calc_iq__fds_dn2, locals.var_fn97_calc_iq__fds_dn3, locals.var_fn97_calc_iq__fds_dn4, locals.var_fn97_calc_iq__fds_dn7, locals.var_fn97_calc_iq__fds_dn14, locals.var_fn97_calc_iq__fds_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds = assign8910_e9771;
        locals.var_fn97_calc_iq__fds_dn2 = assign8910_e9771_d_n2;
        locals.var_fn97_calc_iq__fds_dn3 = assign8910_e9771_d_n3;
        locals.var_fn97_calc_iq__fds_dn4 = assign8910_e9771_d_n4;
        locals.var_fn97_calc_iq__fds_dn7 = assign8910_e9771_d_n7;
        locals.var_fn97_calc_iq__fds_dn14 = assign8910_e9771_d_n14;
        locals.var_fn97_calc_iq__fds_dn15 = assign8910_e9771_d_n15;
        locals.var_fn97_calc_iq__fds_rv = 0.0;

        let (assign8920_e9778, assign8920_e9778_d_n2, assign8920_e9778_d_n3, assign8920_e9778_d_n4, assign8920_e9778_d_n7, assign8920_e9778_d_n14, assign8920_e9778_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8920_e9774: f64 = (-locals.var_fn97_calc_iq__vdsin);
        let assign8920_e9776: f64 = (assign8920_e9774 * locals.var_fn97_calc_iq__fds);
        (assign8920_e9776, (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn2), (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn3), (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn4), (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn7), (((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__fds) + (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn14)), (((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__fds) + (assign8920_e9774 * locals.var_fn97_calc_iq__fds_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vsx, locals.var_fn97_calc_iq__vsx_dn2, locals.var_fn97_calc_iq__vsx_dn3, locals.var_fn97_calc_iq__vsx_dn4, locals.var_fn97_calc_iq__vsx_dn7, locals.var_fn97_calc_iq__vsx_dn14, locals.var_fn97_calc_iq__vsx_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx = assign8920_e9778;
        locals.var_fn97_calc_iq__vsx_dn2 = assign8920_e9778_d_n2;
        locals.var_fn97_calc_iq__vsx_dn3 = assign8920_e9778_d_n3;
        locals.var_fn97_calc_iq__vsx_dn4 = assign8920_e9778_d_n4;
        locals.var_fn97_calc_iq__vsx_dn7 = assign8920_e9778_d_n7;
        locals.var_fn97_calc_iq__vsx_dn14 = assign8920_e9778_d_n14;
        locals.var_fn97_calc_iq__vsx_dn15 = assign8920_e9778_d_n15;
        locals.var_fn97_calc_iq__vsx_rv = 0.0;

        let (assign8930_e9786, assign8930_e9786_d_n2, assign8930_e9786_d_n3, assign8930_e9786_d_n4, assign8930_e9786_d_n7, assign8930_e9786_d_n14, assign8930_e9786_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8930_e9782: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__myarg);
        let assign8930_e9784: f64 = (assign8930_e9782 / locals.var_fn97_calc_iq__alpha_phit);
        (assign8930_e9784, ((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__myarg_dn2) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn3) / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign8930_e9782 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), ((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__myarg_dn7) / locals.var_fn97_calc_iq__alpha_phit), ((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__myarg_dn14) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn15) / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign8930_e9786;
        locals.var_fn97_calc_iq__exparg_dn2 = assign8930_e9786_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign8930_e9786_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign8930_e9786_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign8930_e9786_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign8930_e9786_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign8930_e9786_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let assign8940_e9789: f64 = if locals.var_fn97_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8940_e9789;
        locals.var_guard103_rv = 0.0;

        let (assign8950_e9795, assign8950_e9795_d_n2, assign8950_e9795_d_n3, assign8950_e9795_d_n4, assign8950_e9795_d_n7, assign8950_e9795_d_n14, assign8950_e9795_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard103 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign8950_e9795;
        locals.var_fn97_calc_iq__ffs_dn2 = assign8950_e9795_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign8950_e9795_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign8950_e9795_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign8950_e9795_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign8950_e9795_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign8950_e9795_d_n15;
        locals.var_fn97_calc_iq__ffs_rv = 0.0;

        let assign8960_e9798: f64 = (-50.0);
        let assign8960_e9799: f64 = if locals.var_fn97_calc_iq__exparg < assign8960_e9798 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8960_e9799;
        locals.var_guard104_rv = 0.0;

        let (assign8970_e9808, assign8970_e9808_d_n2, assign8970_e9808_d_n3, assign8970_e9808_d_n4, assign8970_e9808_d_n7, assign8970_e9808_d_n14, assign8970_e9808_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard103 == 0.0)) && (locals.var_guard104 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign8970_e9808;
        locals.var_fn97_calc_iq__ffs_dn2 = assign8970_e9808_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign8970_e9808_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign8970_e9808_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign8970_e9808_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign8970_e9808_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign8970_e9808_d_n15;
        locals.var_fn97_calc_iq__ffs_rv = 0.0;

        let (assign8980_e9823, assign8980_e9823_d_n2, assign8980_e9823_d_n3, assign8980_e9823_d_n4, assign8980_e9823_d_n7, assign8980_e9823_d_n14, assign8980_e9823_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard103 == 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign8980_e9819: f64 = (locals.var_fn97_calc_iq__exparg).exp();
        let assign8980_e9820: f64 = (1.0 + assign8980_e9819);
        let assign8980_e9821: f64 = (1.0 / assign8980_e9820);
        (assign8980_e9821, (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn2) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn3) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn4) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn7) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn14) / (assign8980_e9820 * assign8980_e9820))), (-((assign8980_e9819 * locals.var_fn97_calc_iq__exparg_dn15) / (assign8980_e9820 * assign8980_e9820))),)
    } else {
        (locals.var_fn97_calc_iq__ffs, locals.var_fn97_calc_iq__ffs_dn2, locals.var_fn97_calc_iq__ffs_dn3, locals.var_fn97_calc_iq__ffs_dn4, locals.var_fn97_calc_iq__ffs_dn7, locals.var_fn97_calc_iq__ffs_dn14, locals.var_fn97_calc_iq__ffs_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs = assign8980_e9823;
        locals.var_fn97_calc_iq__ffs_dn2 = assign8980_e9823_d_n2;
        locals.var_fn97_calc_iq__ffs_dn3 = assign8980_e9823_d_n3;
        locals.var_fn97_calc_iq__ffs_dn4 = assign8980_e9823_d_n4;
        locals.var_fn97_calc_iq__ffs_dn7 = assign8980_e9823_d_n7;
        locals.var_fn97_calc_iq__ffs_dn14 = assign8980_e9823_d_n14;
        locals.var_fn97_calc_iq__ffs_dn15 = assign8980_e9823_d_n15;
        locals.var_fn97_calc_iq__ffs_rv = 0.0;

        let (assign8990_e9841, assign8990_e9841_d_n2, assign8990_e9841_d_n3, assign8990_e9841_d_n4, assign8990_e9841_d_n7, assign8990_e9841_d_n14, assign8990_e9841_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign8990_e9827: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__vsx);
        let assign8990_e9831: f64 = (p.p51 * 0.1);
        let assign8990_e9833: f64 = (assign8990_e9831 * locals.var_fn97_calc_iq__alpha_phit);
        let assign8990_e9835: f64 = (assign8990_e9833 * locals.var_fn97_calc_iq__ffs);
        let assign8990_e9836: f64 = (locals.var_fn97_calc_iq__vtdibl - assign8990_e9835);
        let assign8990_e9837: f64 = (assign8990_e9827 - assign8990_e9836);
        let assign8990_e9839: f64 = (assign8990_e9837 / locals.var_fn97_calc_iq__two_n_phit);
        (assign8990_e9839, (((locals.var_fn97_calc_iq__vgdin_dn2 - locals.var_fn97_calc_iq__vsx_dn2) - (-(assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn2))) / locals.var_fn97_calc_iq__two_n_phit), (((-locals.var_fn97_calc_iq__vsx_dn3) - (-(assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn3))) / locals.var_fn97_calc_iq__two_n_phit), (((((-locals.var_fn97_calc_iq__vsx_dn4) - (locals.var_fn97_calc_iq__vtdibl_dn4 - (((assign8990_e9831 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffs) + (assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn4)))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8990_e9837 * locals.var_fn97_calc_iq__two_n_phit_dn4)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((locals.var_fn97_calc_iq__vgdin_dn7 - locals.var_fn97_calc_iq__vsx_dn7) - (-(assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn7))) / locals.var_fn97_calc_iq__two_n_phit), (((((locals.var_fn97_calc_iq__vgdin_dn14 - locals.var_fn97_calc_iq__vsx_dn14) - (locals.var_fn97_calc_iq__vtdibl_dn14 - (assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn14))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8990_e9837 * locals.var_fn97_calc_iq__two_n_phit_dn14)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((((locals.var_fn97_calc_iq__vgdin_dn15 - locals.var_fn97_calc_iq__vsx_dn15) - (locals.var_fn97_calc_iq__vtdibl_dn15 - (assign8990_e9833 * locals.var_fn97_calc_iq__ffs_dn15))) * locals.var_fn97_calc_iq__two_n_phit) - (assign8990_e9837 * locals.var_fn97_calc_iq__two_n_phit_dn15)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn97_calc_iq__etas, locals.var_fn97_calc_iq__etas_dn2, locals.var_fn97_calc_iq__etas_dn3, locals.var_fn97_calc_iq__etas_dn4, locals.var_fn97_calc_iq__etas_dn7, locals.var_fn97_calc_iq__etas_dn14, locals.var_fn97_calc_iq__etas_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas = assign8990_e9841;
        locals.var_fn97_calc_iq__etas_dn2 = assign8990_e9841_d_n2;
        locals.var_fn97_calc_iq__etas_dn3 = assign8990_e9841_d_n3;
        locals.var_fn97_calc_iq__etas_dn4 = assign8990_e9841_d_n4;
        locals.var_fn97_calc_iq__etas_dn7 = assign8990_e9841_d_n7;
        locals.var_fn97_calc_iq__etas_dn14 = assign8990_e9841_d_n14;
        locals.var_fn97_calc_iq__etas_dn15 = assign8990_e9841_d_n15;
        locals.var_fn97_calc_iq__etas_rv = 0.0;

        let assign9000_e9844: f64 = if locals.var_fn97_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign9000_e9844;
        locals.var_guard105_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9010_e9852, assign9010_e9852_d_n2, assign9010_e9852_d_n3, assign9010_e9852_d_n4, assign9010_e9852_d_n7, assign9010_e9852_d_n14, assign9010_e9852_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard105 != 0.0)) {
        let assign9010_e9850: f64 = (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas);
        (assign9010_e9850, (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn2), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn3), ((locals.var_fn97_calc_iq__qref_dn4 * locals.var_fn97_calc_iq__etas) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn4)), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn7), ((locals.var_fn97_calc_iq__qref_dn14 * locals.var_fn97_calc_iq__etas) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn14)), ((locals.var_fn97_calc_iq__qref_dn15 * locals.var_fn97_calc_iq__etas) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etas_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign9010_e9852;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign9010_e9852_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign9010_e9852_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign9010_e9852_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign9010_e9852_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign9010_e9852_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign9010_e9852_d_n15;
        locals.var_fn97_calc_iq__qinvs_rv = 0.0;

        let assign9020_e9855: f64 = (-50.0);
        let assign9020_e9856: f64 = if locals.var_fn97_calc_iq__etas < assign9020_e9855 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign9020_e9856;
        locals.var_guard106_rv = 0.0;

        let (assign9030_e9868, assign9030_e9868_d_n2, assign9030_e9868_d_n3, assign9030_e9868_d_n4, assign9030_e9868_d_n7, assign9030_e9868_d_n14, assign9030_e9868_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard105 == 0.0)) && (locals.var_guard106 != 0.0)) {
        let assign9030_e9865: f64 = (locals.var_fn97_calc_iq__etas).exp();
        let assign9030_e9866: f64 = (locals.var_fn97_calc_iq__qref * assign9030_e9865);
        (assign9030_e9866, (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn2)), (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn3)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9030_e9865) + (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn4))), (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn7)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9030_e9865) + (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn14))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9030_e9865) + (locals.var_fn97_calc_iq__qref * (assign9030_e9865 * locals.var_fn97_calc_iq__etas_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign9030_e9868;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign9030_e9868_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign9030_e9868_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign9030_e9868_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign9030_e9868_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign9030_e9868_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign9030_e9868_d_n15;
        locals.var_fn97_calc_iq__qinvs_rv = 0.0;

        let (assign9040_e9884, assign9040_e9884_d_n2, assign9040_e9884_d_n3, assign9040_e9884_d_n4, assign9040_e9884_d_n7, assign9040_e9884_d_n14, assign9040_e9884_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard105 == 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign9040_e9879: f64 = (locals.var_fn97_calc_iq__etas).exp();
        let assign9040_e9880: f64 = (1.0 + assign9040_e9879);
        let assign9040_e9881: f64 = (assign9040_e9880).ln();
        let assign9040_e9882: f64 = (locals.var_fn97_calc_iq__qref * assign9040_e9881);
        (assign9040_e9882, (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn2) / assign9040_e9880)), (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn3) / assign9040_e9880)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9040_e9881) + (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn4) / assign9040_e9880))), (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn7) / assign9040_e9880)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9040_e9881) + (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn14) / assign9040_e9880))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9040_e9881) + (locals.var_fn97_calc_iq__qref * ((assign9040_e9879 * locals.var_fn97_calc_iq__etas_dn15) / assign9040_e9880))),)
    } else {
        (locals.var_fn97_calc_iq__qinvs, locals.var_fn97_calc_iq__qinvs_dn2, locals.var_fn97_calc_iq__qinvs_dn3, locals.var_fn97_calc_iq__qinvs_dn4, locals.var_fn97_calc_iq__qinvs_dn7, locals.var_fn97_calc_iq__qinvs_dn14, locals.var_fn97_calc_iq__qinvs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs = assign9040_e9884;
        locals.var_fn97_calc_iq__qinvs_dn2 = assign9040_e9884_d_n2;
        locals.var_fn97_calc_iq__qinvs_dn3 = assign9040_e9884_d_n3;
        locals.var_fn97_calc_iq__qinvs_dn4 = assign9040_e9884_d_n4;
        locals.var_fn97_calc_iq__qinvs_dn7 = assign9040_e9884_d_n7;
        locals.var_fn97_calc_iq__qinvs_dn14 = assign9040_e9884_d_n14;
        locals.var_fn97_calc_iq__qinvs_dn15 = assign9040_e9884_d_n15;
        locals.var_fn97_calc_iq__qinvs_rv = 0.0;

        let (assign9050_e9892, assign9050_e9892_d_n2, assign9050_e9892_d_n3, assign9050_e9892_d_n4, assign9050_e9892_d_n7, assign9050_e9892_d_n14, assign9050_e9892_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9050_e9888: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__myarg);
        let assign9050_e9890: f64 = (assign9050_e9888 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9050_e9890, ((locals.var_fn97_calc_iq__vgdin_dn2 - locals.var_fn97_calc_iq__myarg_dn2) / locals.var_fn97_calc_iq__alpha_phit), ((-locals.var_fn97_calc_iq__myarg_dn3) / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9050_e9888 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), ((locals.var_fn97_calc_iq__vgdin_dn7 - locals.var_fn97_calc_iq__myarg_dn7) / locals.var_fn97_calc_iq__alpha_phit), ((locals.var_fn97_calc_iq__vgdin_dn14 - locals.var_fn97_calc_iq__myarg_dn14) / locals.var_fn97_calc_iq__alpha_phit), ((locals.var_fn97_calc_iq__vgdin_dn15 - locals.var_fn97_calc_iq__myarg_dn15) / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9050_e9892;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9050_e9892_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9050_e9892_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9050_e9892_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9050_e9892_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9050_e9892_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9050_e9892_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let assign9060_e9895: f64 = if locals.var_fn97_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign9060_e9895;
        locals.var_guard107_rv = 0.0;

        let (assign9070_e9901, assign9070_e9901_d_n2, assign9070_e9901_d_n3, assign9070_e9901_d_n4, assign9070_e9901_d_n7, assign9070_e9901_d_n14, assign9070_e9901_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard107 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign9070_e9901;
        locals.var_fn97_calc_iq__ffd_dn2 = assign9070_e9901_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign9070_e9901_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign9070_e9901_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign9070_e9901_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign9070_e9901_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign9070_e9901_d_n15;
        locals.var_fn97_calc_iq__ffd_rv = 0.0;

        let assign9080_e9904: f64 = (-50.0);
        let assign9080_e9905: f64 = if locals.var_fn97_calc_iq__exparg < assign9080_e9904 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign9080_e9905;
        locals.var_guard108_rv = 0.0;

        let (assign9090_e9914, assign9090_e9914_d_n2, assign9090_e9914_d_n3, assign9090_e9914_d_n4, assign9090_e9914_d_n7, assign9090_e9914_d_n14, assign9090_e9914_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard107 == 0.0)) && (locals.var_guard108 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign9090_e9914;
        locals.var_fn97_calc_iq__ffd_dn2 = assign9090_e9914_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign9090_e9914_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign9090_e9914_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign9090_e9914_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign9090_e9914_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign9090_e9914_d_n15;
        locals.var_fn97_calc_iq__ffd_rv = 0.0;

        let (assign9100_e9929, assign9100_e9929_d_n2, assign9100_e9929_d_n3, assign9100_e9929_d_n4, assign9100_e9929_d_n7, assign9100_e9929_d_n14, assign9100_e9929_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard107 == 0.0)) && (locals.var_guard108 == 0.0)) {
        let assign9100_e9925: f64 = (locals.var_fn97_calc_iq__exparg).exp();
        let assign9100_e9926: f64 = (1.0 + assign9100_e9925);
        let assign9100_e9927: f64 = (1.0 / assign9100_e9926);
        (assign9100_e9927, (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn2) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn3) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn4) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn7) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn14) / (assign9100_e9926 * assign9100_e9926))), (-((assign9100_e9925 * locals.var_fn97_calc_iq__exparg_dn15) / (assign9100_e9926 * assign9100_e9926))),)
    } else {
        (locals.var_fn97_calc_iq__ffd, locals.var_fn97_calc_iq__ffd_dn2, locals.var_fn97_calc_iq__ffd_dn3, locals.var_fn97_calc_iq__ffd_dn4, locals.var_fn97_calc_iq__ffd_dn7, locals.var_fn97_calc_iq__ffd_dn14, locals.var_fn97_calc_iq__ffd_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd = assign9100_e9929;
        locals.var_fn97_calc_iq__ffd_dn2 = assign9100_e9929_d_n2;
        locals.var_fn97_calc_iq__ffd_dn3 = assign9100_e9929_d_n3;
        locals.var_fn97_calc_iq__ffd_dn4 = assign9100_e9929_d_n4;
        locals.var_fn97_calc_iq__ffd_dn7 = assign9100_e9929_d_n7;
        locals.var_fn97_calc_iq__ffd_dn14 = assign9100_e9929_d_n14;
        locals.var_fn97_calc_iq__ffd_dn15 = assign9100_e9929_d_n15;
        locals.var_fn97_calc_iq__ffd_rv = 0.0;

        let (assign9110_e9947, assign9110_e9947_d_n2, assign9110_e9947_d_n3, assign9110_e9947_d_n4, assign9110_e9947_d_n7, assign9110_e9947_d_n14, assign9110_e9947_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9110_e9933: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vdx);
        let assign9110_e9937: f64 = (p.p51 * 0.1);
        let assign9110_e9939: f64 = (assign9110_e9937 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9110_e9941: f64 = (assign9110_e9939 * locals.var_fn97_calc_iq__ffd);
        let assign9110_e9942: f64 = (locals.var_fn97_calc_iq__vtdibl - assign9110_e9941);
        let assign9110_e9943: f64 = (assign9110_e9933 - assign9110_e9942);
        let assign9110_e9945: f64 = (assign9110_e9943 / locals.var_fn97_calc_iq__two_n_phit);
        (assign9110_e9945, (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vdx_dn2) - (-(assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn2))) / locals.var_fn97_calc_iq__two_n_phit), (((-locals.var_fn97_calc_iq__vdx_dn3) - (-(assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn3))) / locals.var_fn97_calc_iq__two_n_phit), (((((-locals.var_fn97_calc_iq__vdx_dn4) - (locals.var_fn97_calc_iq__vtdibl_dn4 - (((assign9110_e9937 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffd) + (assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn4)))) * locals.var_fn97_calc_iq__two_n_phit) - (assign9110_e9943 * locals.var_fn97_calc_iq__two_n_phit_dn4)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vdx_dn7) - (-(assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn7))) / locals.var_fn97_calc_iq__two_n_phit), (((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vdx_dn14) - (locals.var_fn97_calc_iq__vtdibl_dn14 - (assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn14))) * locals.var_fn97_calc_iq__two_n_phit) - (assign9110_e9943 * locals.var_fn97_calc_iq__two_n_phit_dn14)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)), (((((-locals.var_fn97_calc_iq__vdx_dn15) - (locals.var_fn97_calc_iq__vtdibl_dn15 - (assign9110_e9939 * locals.var_fn97_calc_iq__ffd_dn15))) * locals.var_fn97_calc_iq__two_n_phit) - (assign9110_e9943 * locals.var_fn97_calc_iq__two_n_phit_dn15)) / (locals.var_fn97_calc_iq__two_n_phit * locals.var_fn97_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn97_calc_iq__etad, locals.var_fn97_calc_iq__etad_dn2, locals.var_fn97_calc_iq__etad_dn3, locals.var_fn97_calc_iq__etad_dn4, locals.var_fn97_calc_iq__etad_dn7, locals.var_fn97_calc_iq__etad_dn14, locals.var_fn97_calc_iq__etad_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad = assign9110_e9947;
        locals.var_fn97_calc_iq__etad_dn2 = assign9110_e9947_d_n2;
        locals.var_fn97_calc_iq__etad_dn3 = assign9110_e9947_d_n3;
        locals.var_fn97_calc_iq__etad_dn4 = assign9110_e9947_d_n4;
        locals.var_fn97_calc_iq__etad_dn7 = assign9110_e9947_d_n7;
        locals.var_fn97_calc_iq__etad_dn14 = assign9110_e9947_d_n14;
        locals.var_fn97_calc_iq__etad_dn15 = assign9110_e9947_d_n15;
        locals.var_fn97_calc_iq__etad_rv = 0.0;

        let assign9120_e9950: f64 = if locals.var_fn97_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign9120_e9950;
        locals.var_guard109_rv = 0.0;

        let (assign9130_e9958, assign9130_e9958_d_n2, assign9130_e9958_d_n3, assign9130_e9958_d_n4, assign9130_e9958_d_n7, assign9130_e9958_d_n14, assign9130_e9958_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard109 != 0.0)) {
        let assign9130_e9956: f64 = (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad);
        (assign9130_e9956, (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn2), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn3), ((locals.var_fn97_calc_iq__qref_dn4 * locals.var_fn97_calc_iq__etad) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn4)), (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn7), ((locals.var_fn97_calc_iq__qref_dn14 * locals.var_fn97_calc_iq__etad) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn14)), ((locals.var_fn97_calc_iq__qref_dn15 * locals.var_fn97_calc_iq__etad) + (locals.var_fn97_calc_iq__qref * locals.var_fn97_calc_iq__etad_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign9130_e9958;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign9130_e9958_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign9130_e9958_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign9130_e9958_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign9130_e9958_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign9130_e9958_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign9130_e9958_d_n15;
        locals.var_fn97_calc_iq__qinvd_rv = 0.0;

        let assign9140_e9961: f64 = (-50.0);
        let assign9140_e9962: f64 = if locals.var_fn97_calc_iq__etad < assign9140_e9961 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign9140_e9962;
        locals.var_guard110_rv = 0.0;

        let (assign9150_e9974, assign9150_e9974_d_n2, assign9150_e9974_d_n3, assign9150_e9974_d_n4, assign9150_e9974_d_n7, assign9150_e9974_d_n14, assign9150_e9974_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard109 == 0.0)) && (locals.var_guard110 != 0.0)) {
        let assign9150_e9971: f64 = (locals.var_fn97_calc_iq__etad).exp();
        let assign9150_e9972: f64 = (locals.var_fn97_calc_iq__qref * assign9150_e9971);
        (assign9150_e9972, (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn2)), (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn3)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9150_e9971) + (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn4))), (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn7)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9150_e9971) + (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn14))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9150_e9971) + (locals.var_fn97_calc_iq__qref * (assign9150_e9971 * locals.var_fn97_calc_iq__etad_dn15))),)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign9150_e9974;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign9150_e9974_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign9150_e9974_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign9150_e9974_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign9150_e9974_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign9150_e9974_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign9150_e9974_d_n15;
        locals.var_fn97_calc_iq__qinvd_rv = 0.0;

        let (assign9160_e9990, assign9160_e9990_d_n2, assign9160_e9990_d_n3, assign9160_e9990_d_n4, assign9160_e9990_d_n7, assign9160_e9990_d_n14, assign9160_e9990_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard109 == 0.0)) && (locals.var_guard110 == 0.0)) {
        let assign9160_e9985: f64 = (locals.var_fn97_calc_iq__etad).exp();
        let assign9160_e9986: f64 = (1.0 + assign9160_e9985);
        let assign9160_e9987: f64 = (assign9160_e9986).ln();
        let assign9160_e9988: f64 = (locals.var_fn97_calc_iq__qref * assign9160_e9987);
        (assign9160_e9988, (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn2) / assign9160_e9986)), (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn3) / assign9160_e9986)), ((locals.var_fn97_calc_iq__qref_dn4 * assign9160_e9987) + (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn4) / assign9160_e9986))), (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn7) / assign9160_e9986)), ((locals.var_fn97_calc_iq__qref_dn14 * assign9160_e9987) + (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn14) / assign9160_e9986))), ((locals.var_fn97_calc_iq__qref_dn15 * assign9160_e9987) + (locals.var_fn97_calc_iq__qref * ((assign9160_e9985 * locals.var_fn97_calc_iq__etad_dn15) / assign9160_e9986))),)
    } else {
        (locals.var_fn97_calc_iq__qinvd, locals.var_fn97_calc_iq__qinvd_dn2, locals.var_fn97_calc_iq__qinvd_dn3, locals.var_fn97_calc_iq__qinvd_dn4, locals.var_fn97_calc_iq__qinvd_dn7, locals.var_fn97_calc_iq__qinvd_dn14, locals.var_fn97_calc_iq__qinvd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd = assign9160_e9990;
        locals.var_fn97_calc_iq__qinvd_dn2 = assign9160_e9990_d_n2;
        locals.var_fn97_calc_iq__qinvd_dn3 = assign9160_e9990_d_n3;
        locals.var_fn97_calc_iq__qinvd_dn4 = assign9160_e9990_d_n4;
        locals.var_fn97_calc_iq__qinvd_dn7 = assign9160_e9990_d_n7;
        locals.var_fn97_calc_iq__qinvd_dn14 = assign9160_e9990_d_n14;
        locals.var_fn97_calc_iq__qinvd_dn15 = assign9160_e9990_d_n15;
        locals.var_fn97_calc_iq__qinvd_rv = 0.0;

        let (assign9170_e9998, assign9170_e9998_d_n2, assign9170_e9998_d_n3, assign9170_e9998_d_n4, assign9170_e9998_d_n7, assign9170_e9998_d_n14, assign9170_e9998_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9170_e9994: f64 = (locals.var_fn97_calc_iq__qinvs - locals.var_fn97_calc_iq__qinvd);
        let assign9170_e9996: f64 = (assign9170_e9994 / locals.var_fn97_calc_iq__cgin);
        (assign9170_e9996, ((locals.var_fn97_calc_iq__qinvs_dn2 - locals.var_fn97_calc_iq__qinvd_dn2) / locals.var_fn97_calc_iq__cgin), ((locals.var_fn97_calc_iq__qinvs_dn3 - locals.var_fn97_calc_iq__qinvd_dn3) / locals.var_fn97_calc_iq__cgin), ((((locals.var_fn97_calc_iq__qinvs_dn4 - locals.var_fn97_calc_iq__qinvd_dn4) * locals.var_fn97_calc_iq__cgin) - (assign9170_e9994 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)), ((locals.var_fn97_calc_iq__qinvs_dn7 - locals.var_fn97_calc_iq__qinvd_dn7) / locals.var_fn97_calc_iq__cgin), ((locals.var_fn97_calc_iq__qinvs_dn14 - locals.var_fn97_calc_iq__qinvd_dn14) / locals.var_fn97_calc_iq__cgin), ((locals.var_fn97_calc_iq__qinvs_dn15 - locals.var_fn97_calc_iq__qinvd_dn15) / locals.var_fn97_calc_iq__cgin),)
    } else {
        (locals.var_fn97_calc_iq__vdsc, locals.var_fn97_calc_iq__vdsc_dn2, locals.var_fn97_calc_iq__vdsc_dn3, locals.var_fn97_calc_iq__vdsc_dn4, locals.var_fn97_calc_iq__vdsc_dn7, locals.var_fn97_calc_iq__vdsc_dn14, locals.var_fn97_calc_iq__vdsc_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsc = assign9170_e9998;
        locals.var_fn97_calc_iq__vdsc_dn2 = assign9170_e9998_d_n2;
        locals.var_fn97_calc_iq__vdsc_dn3 = assign9170_e9998_d_n3;
        locals.var_fn97_calc_iq__vdsc_dn4 = assign9170_e9998_d_n4;
        locals.var_fn97_calc_iq__vdsc_dn7 = assign9170_e9998_d_n7;
        locals.var_fn97_calc_iq__vdsc_dn14 = assign9170_e9998_d_n14;
        locals.var_fn97_calc_iq__vdsc_dn15 = assign9170_e9998_d_n15;
        locals.var_fn97_calc_iq__vdsc_rv = 0.0;

        let (assign9180_e10004, assign9180_e10004_d_n2, assign9180_e10004_d_n3, assign9180_e10004_d_n4, assign9180_e10004_d_n7, assign9180_e10004_d_n14, assign9180_e10004_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9180_e10002: f64 = (locals.var_fn97_calc_iq__vdsc / locals.var_fn97_calc_iq__vdsat);
        (assign9180_e10002, (((locals.var_fn97_calc_iq__vdsc_dn2 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn2)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn3 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn3)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn4 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn4)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn7 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn7)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn14 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn14)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)), (((locals.var_fn97_calc_iq__vdsc_dn15 * locals.var_fn97_calc_iq__vdsat) - (locals.var_fn97_calc_iq__vdsc * locals.var_fn97_calc_iq__vdsat_dn15)) / (locals.var_fn97_calc_iq__vdsat * locals.var_fn97_calc_iq__vdsat)),)
    } else {
        (locals.var_fn97_calc_iq__myarg, locals.var_fn97_calc_iq__myarg_dn2, locals.var_fn97_calc_iq__myarg_dn3, locals.var_fn97_calc_iq__myarg_dn4, locals.var_fn97_calc_iq__myarg_dn7, locals.var_fn97_calc_iq__myarg_dn14, locals.var_fn97_calc_iq__myarg_dn15,)
    }
};
        locals.var_fn97_calc_iq__myarg = assign9180_e10004;
        locals.var_fn97_calc_iq__myarg_dn2 = assign9180_e10004_d_n2;
        locals.var_fn97_calc_iq__myarg_dn3 = assign9180_e10004_d_n3;
        locals.var_fn97_calc_iq__myarg_dn4 = assign9180_e10004_d_n4;
        locals.var_fn97_calc_iq__myarg_dn7 = assign9180_e10004_d_n7;
        locals.var_fn97_calc_iq__myarg_dn14 = assign9180_e10004_d_n14;
        locals.var_fn97_calc_iq__myarg_dn15 = assign9180_e10004_d_n15;
        locals.var_fn97_calc_iq__myarg_rv = 0.0;

        let (assign9220_e10073, assign9220_e10073_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9220_e10070: f64 = (2.302585092994046 * locals.var_fn97_calc_iq__phitin);
        let assign9220_e10071: f64 = (locals.var_fn97_calc_iq__ss / assign9220_e10070);
        (assign9220_e10071, (-((locals.var_fn97_calc_iq__ss * (2.302585092994046 * locals.var_fn97_calc_iq__phitin_dn4)) / (assign9220_e10070 * assign9220_e10070))),)
    } else {
        (locals.var_fn97_calc_iq__n0, locals.var_fn97_calc_iq__n0_dn4,)
    }
};
        locals.var_fn97_calc_iq__n0 = assign9220_e10073;
        locals.var_fn97_calc_iq__n0_dn4 = assign9220_e10073_d_n4;
        locals.var_fn97_calc_iq__n0_rv = 0.0;

        let (assign9230_e10081, assign9230_e10081_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9230_e10077: f64 = (2.0 * locals.var_fn97_calc_iq__n0);
        let assign9230_e10079: f64 = (assign9230_e10077 * locals.var_fn97_calc_iq__phitin);
        (assign9230_e10079, (((2.0 * locals.var_fn97_calc_iq__n0_dn4) * locals.var_fn97_calc_iq__phitin) + (assign9230_e10077 * locals.var_fn97_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn97_calc_iq__two_n_phit0, locals.var_fn97_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn97_calc_iq__two_n_phit0 = assign9230_e10081;
        locals.var_fn97_calc_iq__two_n_phit0_dn4 = assign9230_e10081_d_n4;
        locals.var_fn97_calc_iq__two_n_phit0_rv = 0.0;

        let (assign9240_e10087, assign9240_e10087_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9240_e10085: f64 = (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit0);
        (assign9240_e10085, ((locals.var_fn97_calc_iq__cgin_dn4 * locals.var_fn97_calc_iq__two_n_phit0) + (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn97_calc_iq__qref0, locals.var_fn97_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn97_calc_iq__qref0 = assign9240_e10087;
        locals.var_fn97_calc_iq__qref0_dn4 = assign9240_e10087_d_n4;
        locals.var_fn97_calc_iq__qref0_rv = 0.0;

        let (assign9250_e10097, assign9250_e10097_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9250_e10092: f64 = (p.p51 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9250_e10094: f64 = (assign9250_e10092 / 2.0);
        let assign9250_e10095: f64 = (locals.var_fn97_calc_iq__vtof - assign9250_e10094);
        (assign9250_e10095, (locals.var_fn97_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn97_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn97_calc_iq__myarg0, locals.var_fn97_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn97_calc_iq__myarg0 = assign9250_e10097;
        locals.var_fn97_calc_iq__myarg0_dn4 = assign9250_e10097_d_n4;
        locals.var_fn97_calc_iq__myarg0_rv = 0.0;

        let (assign9260_e10148, assign9260_e10148_d_n2, assign9260_e10148_d_n4, assign9260_e10148_d_n7, assign9260_e10148_d_n14, assign9260_e10148_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9260_e10142, assign9260_e10142_d_n2, assign9260_e10142_d_n7, assign9260_e10142_d_n14, assign9260_e10142_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9260_e10106: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign9260_e10109: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9260_e10112: f64 = (0.001 / p.p53);
                let assign9260_e10115: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9260_e10116: f64 = (assign9260_e10112 * assign9260_e10115);
                let assign9260_e10117: f64 = (assign9260_e10116).tanh();
                let assign9260_e10118: f64 = (assign9260_e10109 * assign9260_e10117);
                let assign9260_e10119: f64 = (assign9260_e10106 + assign9260_e10118);
                let assign9260_e10120: f64 = (0.5 * assign9260_e10119);
                (assign9260_e10120, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9260_e10117) + (assign9260_e10109 * ((assign9260_e10112 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign9260_e10116).cosh() * (assign9260_e10116).cosh())))))),)
            } else {
                let (assign9260_e10141, assign9260_e10141_d_n2, assign9260_e10141_d_n7, assign9260_e10141_d_n14, assign9260_e10141_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9260_e10127: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign9260_e10130: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9260_e10133: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9260_e10134: f64 = (assign9260_e10130 * assign9260_e10133);
                        let assign9260_e10136: f64 = (assign9260_e10134 + p.p53);
                        let assign9260_e10137: f64 = (assign9260_e10136).sqrt();
                        let assign9260_e10138: f64 = (assign9260_e10127 + assign9260_e10137);
                        let assign9260_e10139: f64 = (0.5 * assign9260_e10138);
                        (assign9260_e10139, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9260_e10133) + (assign9260_e10130 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign9260_e10137)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9260_e10133) + (assign9260_e10130 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign9260_e10137)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9260_e10133) + (assign9260_e10130 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign9260_e10137)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9260_e10133) + (assign9260_e10130 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign9260_e10137)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9260_e10141, assign9260_e10141_d_n2, assign9260_e10141_d_n7, assign9260_e10141_d_n14, assign9260_e10141_d_n15,)
            }
        };
        let assign9260_e10144: f64 = (assign9260_e10142 - locals.var_fn97_calc_iq__myarg0);
        let assign9260_e10146: f64 = (assign9260_e10144 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9260_e10146, (assign9260_e10142_d_n2 / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg0_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9260_e10144 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), (assign9260_e10142_d_n7 / locals.var_fn97_calc_iq__alpha_phit), (assign9260_e10142_d_n14 / locals.var_fn97_calc_iq__alpha_phit), (assign9260_e10142_d_n15 / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign9260_e10148;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign9260_e10148_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign9260_e10148_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign9260_e10148_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign9260_e10148_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign9260_e10148_d_n15;
        locals.var_fn97_calc_iq__exparg0_rv = 0.0;

        let assign9270_e10151: f64 = if locals.var_fn97_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign9270_e10151;
        locals.var_guard111_rv = 0.0;

        let (assign9280_e10157, assign9280_e10157_d_n2, assign9280_e10157_d_n4, assign9280_e10157_d_n7, assign9280_e10157_d_n14, assign9280_e10157_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard111 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign9280_e10157;
        locals.var_fn97_calc_iq__ff0_dn2 = assign9280_e10157_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign9280_e10157_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign9280_e10157_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign9280_e10157_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign9280_e10157_d_n15;
        locals.var_fn97_calc_iq__ff0_rv = 0.0;

        let assign9290_e10160: f64 = (-50.0);
        let assign9290_e10161: f64 = if locals.var_fn97_calc_iq__exparg0 < assign9290_e10160 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign9290_e10161;
        locals.var_guard112_rv = 0.0;

        let (assign9300_e10170, assign9300_e10170_d_n2, assign9300_e10170_d_n4, assign9300_e10170_d_n7, assign9300_e10170_d_n14, assign9300_e10170_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard111 == 0.0)) && (locals.var_guard112 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign9300_e10170;
        locals.var_fn97_calc_iq__ff0_dn2 = assign9300_e10170_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign9300_e10170_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign9300_e10170_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign9300_e10170_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign9300_e10170_d_n15;
        locals.var_fn97_calc_iq__ff0_rv = 0.0;

        let (assign9310_e10185, assign9310_e10185_d_n2, assign9310_e10185_d_n4, assign9310_e10185_d_n7, assign9310_e10185_d_n14, assign9310_e10185_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard111 == 0.0)) && (locals.var_guard112 == 0.0)) {
        let assign9310_e10181: f64 = (locals.var_fn97_calc_iq__exparg0).exp();
        let assign9310_e10182: f64 = (1.0 + assign9310_e10181);
        let assign9310_e10183: f64 = (1.0 / assign9310_e10182);
        (assign9310_e10183, (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn2) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn4) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn7) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn14) / (assign9310_e10182 * assign9310_e10182))), (-((assign9310_e10181 * locals.var_fn97_calc_iq__exparg0_dn15) / (assign9310_e10182 * assign9310_e10182))),)
    } else {
        (locals.var_fn97_calc_iq__ff0, locals.var_fn97_calc_iq__ff0_dn2, locals.var_fn97_calc_iq__ff0_dn4, locals.var_fn97_calc_iq__ff0_dn7, locals.var_fn97_calc_iq__ff0_dn14, locals.var_fn97_calc_iq__ff0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ff0 = assign9310_e10185;
        locals.var_fn97_calc_iq__ff0_dn2 = assign9310_e10185_d_n2;
        locals.var_fn97_calc_iq__ff0_dn4 = assign9310_e10185_d_n4;
        locals.var_fn97_calc_iq__ff0_dn7 = assign9310_e10185_d_n7;
        locals.var_fn97_calc_iq__ff0_dn14 = assign9310_e10185_d_n14;
        locals.var_fn97_calc_iq__ff0_dn15 = assign9310_e10185_d_n15;
        locals.var_fn97_calc_iq__ff0_rv = 0.0;

        let (assign9320_e10244, assign9320_e10244_d_n2, assign9320_e10244_d_n4, assign9320_e10244_d_n7, assign9320_e10244_d_n14, assign9320_e10244_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9320_e10230, assign9320_e10230_d_n2, assign9320_e10230_d_n7, assign9320_e10230_d_n14, assign9320_e10230_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9320_e10194: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                let assign9320_e10197: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9320_e10200: f64 = (0.001 / p.p53);
                let assign9320_e10203: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                let assign9320_e10204: f64 = (assign9320_e10200 * assign9320_e10203);
                let assign9320_e10205: f64 = (assign9320_e10204).tanh();
                let assign9320_e10206: f64 = (assign9320_e10197 * assign9320_e10205);
                let assign9320_e10207: f64 = (assign9320_e10194 + assign9320_e10206);
                let assign9320_e10208: f64 = (0.5 * assign9320_e10207);
                (assign9320_e10208, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + (((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9320_e10205) + (assign9320_e10197 * ((assign9320_e10200 * (-locals.var_fn97_calc_iq__vgdin_dn15)) / ((assign9320_e10204).cosh() * (assign9320_e10204).cosh())))))),)
            } else {
                let (assign9320_e10229, assign9320_e10229_d_n2, assign9320_e10229_d_n7, assign9320_e10229_d_n14, assign9320_e10229_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9320_e10215: f64 = (locals.var_fn97_calc_iq__vgsin + locals.var_fn97_calc_iq__vgdin);
                        let assign9320_e10218: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9320_e10221: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vgdin);
                        let assign9320_e10222: f64 = (assign9320_e10218 * assign9320_e10221);
                        let assign9320_e10224: f64 = (assign9320_e10222 + p.p53);
                        let assign9320_e10225: f64 = (assign9320_e10224).sqrt();
                        let assign9320_e10226: f64 = (assign9320_e10215 + assign9320_e10225);
                        let assign9320_e10227: f64 = (0.5 * assign9320_e10226);
                        (assign9320_e10227, (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn2 + locals.var_fn97_calc_iq__vgdin_dn2) + ((((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2) * assign9320_e10221) + (assign9320_e10218 * (locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vgdin_dn2))) / (2.0 * assign9320_e10225)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn7 + locals.var_fn97_calc_iq__vgdin_dn7) + ((((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7) * assign9320_e10221) + (assign9320_e10218 * (locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vgdin_dn7))) / (2.0 * assign9320_e10225)))), (0.5 * ((locals.var_fn97_calc_iq__vgsin_dn14 + locals.var_fn97_calc_iq__vgdin_dn14) + ((((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14) * assign9320_e10221) + (assign9320_e10218 * (locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vgdin_dn14))) / (2.0 * assign9320_e10225)))), (0.5 * (locals.var_fn97_calc_iq__vgdin_dn15 + ((((-locals.var_fn97_calc_iq__vgdin_dn15) * assign9320_e10221) + (assign9320_e10218 * (-locals.var_fn97_calc_iq__vgdin_dn15))) / (2.0 * assign9320_e10225)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9320_e10229, assign9320_e10229_d_n2, assign9320_e10229_d_n7, assign9320_e10229_d_n14, assign9320_e10229_d_n15,)
            }
        };
        let assign9320_e10234: f64 = (p.p51 * 0.1);
        let assign9320_e10236: f64 = (assign9320_e10234 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9320_e10238: f64 = (assign9320_e10236 * locals.var_fn97_calc_iq__ff0);
        let assign9320_e10239: f64 = (locals.var_fn97_calc_iq__vtof - assign9320_e10238);
        let assign9320_e10240: f64 = (assign9320_e10230 - assign9320_e10239);
        let assign9320_e10242: f64 = (assign9320_e10240 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9320_e10242, ((assign9320_e10230_d_n2 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn2))) / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (((assign9320_e10234 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ff0) + (assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn4)))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9320_e10240 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), ((assign9320_e10230_d_n7 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn7))) / locals.var_fn97_calc_iq__two_n_phit0), ((assign9320_e10230_d_n14 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn14))) / locals.var_fn97_calc_iq__two_n_phit0), ((assign9320_e10230_d_n15 - (-(assign9320_e10236 * locals.var_fn97_calc_iq__ff0_dn15))) / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__eta0, locals.var_fn97_calc_iq__eta0_dn2, locals.var_fn97_calc_iq__eta0_dn4, locals.var_fn97_calc_iq__eta0_dn7, locals.var_fn97_calc_iq__eta0_dn14, locals.var_fn97_calc_iq__eta0_dn15,)
    }
};
        locals.var_fn97_calc_iq__eta0 = assign9320_e10244;
        locals.var_fn97_calc_iq__eta0_dn2 = assign9320_e10244_d_n2;
        locals.var_fn97_calc_iq__eta0_dn4 = assign9320_e10244_d_n4;
        locals.var_fn97_calc_iq__eta0_dn7 = assign9320_e10244_d_n7;
        locals.var_fn97_calc_iq__eta0_dn14 = assign9320_e10244_d_n14;
        locals.var_fn97_calc_iq__eta0_dn15 = assign9320_e10244_d_n15;
        locals.var_fn97_calc_iq__eta0_rv = 0.0;

        let assign9330_e10247: f64 = if locals.var_fn97_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign9330_e10247;
        locals.var_guard113_rv = 0.0;

        let (assign9340_e10255, assign9340_e10255_d_n2, assign9340_e10255_d_n4, assign9340_e10255_d_n7, assign9340_e10255_d_n14, assign9340_e10255_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign9340_e10253: f64 = (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0);
        (assign9340_e10253, (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn2), ((locals.var_fn97_calc_iq__qref0_dn4 * locals.var_fn97_calc_iq__eta0) + (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn4)), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn7), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn14), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__eta0_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign9340_e10255;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign9340_e10255_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign9340_e10255_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign9340_e10255_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign9340_e10255_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign9340_e10255_d_n15;
        locals.var_fn97_calc_iq__qinvv0_rv = 0.0;

        let assign9350_e10258: f64 = (-50.0);
        let assign9350_e10259: f64 = if locals.var_fn97_calc_iq__eta0 < assign9350_e10258 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign9350_e10259;
        locals.var_guard114_rv = 0.0;

        let (assign9360_e10271, assign9360_e10271_d_n2, assign9360_e10271_d_n4, assign9360_e10271_d_n7, assign9360_e10271_d_n14, assign9360_e10271_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 != 0.0)) {
        let assign9360_e10268: f64 = (locals.var_fn97_calc_iq__eta0).exp();
        let assign9360_e10269: f64 = (locals.var_fn97_calc_iq__qref0 * assign9360_e10268);
        (assign9360_e10269, (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn2)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9360_e10268) + (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn4))), (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn7)), (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn14)), (locals.var_fn97_calc_iq__qref0 * (assign9360_e10268 * locals.var_fn97_calc_iq__eta0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign9360_e10271;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign9360_e10271_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign9360_e10271_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign9360_e10271_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign9360_e10271_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign9360_e10271_d_n15;
        locals.var_fn97_calc_iq__qinvv0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9370_e10287, assign9370_e10287_d_n2, assign9370_e10287_d_n4, assign9370_e10287_d_n7, assign9370_e10287_d_n14, assign9370_e10287_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard113 == 0.0)) && (locals.var_guard114 == 0.0)) {
        let assign9370_e10282: f64 = (locals.var_fn97_calc_iq__eta0).exp();
        let assign9370_e10283: f64 = (1.0 + assign9370_e10282);
        let assign9370_e10284: f64 = (assign9370_e10283).ln();
        let assign9370_e10285: f64 = (locals.var_fn97_calc_iq__qref0 * assign9370_e10284);
        (assign9370_e10285, (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn2) / assign9370_e10283)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9370_e10284) + (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn4) / assign9370_e10283))), (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn7) / assign9370_e10283)), (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn14) / assign9370_e10283)), (locals.var_fn97_calc_iq__qref0 * ((assign9370_e10282 * locals.var_fn97_calc_iq__eta0_dn15) / assign9370_e10283)),)
    } else {
        (locals.var_fn97_calc_iq__qinvv0, locals.var_fn97_calc_iq__qinvv0_dn2, locals.var_fn97_calc_iq__qinvv0_dn4, locals.var_fn97_calc_iq__qinvv0_dn7, locals.var_fn97_calc_iq__qinvv0_dn14, locals.var_fn97_calc_iq__qinvv0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvv0 = assign9370_e10287;
        locals.var_fn97_calc_iq__qinvv0_dn2 = assign9370_e10287_d_n2;
        locals.var_fn97_calc_iq__qinvv0_dn4 = assign9370_e10287_d_n4;
        locals.var_fn97_calc_iq__qinvv0_dn7 = assign9370_e10287_d_n7;
        locals.var_fn97_calc_iq__qinvv0_dn14 = assign9370_e10287_d_n14;
        locals.var_fn97_calc_iq__qinvv0_dn15 = assign9370_e10287_d_n15;
        locals.var_fn97_calc_iq__qinvv0_rv = 0.0;

        let (assign9380_e10293, assign9380_e10293_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9380_e10291: f64 = (locals.var_fn97_calc_iq__mu0 / locals.var_fn97_calc_iq__tfacmobin);
        (assign9380_e10291, (-((locals.var_fn97_calc_iq__mu0 * locals.var_fn97_calc_iq__tfacmobin_dn4) / (locals.var_fn97_calc_iq__tfacmobin * locals.var_fn97_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn97_calc_iq__muf0, locals.var_fn97_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn97_calc_iq__muf0 = assign9380_e10293;
        locals.var_fn97_calc_iq__muf0_dn4 = assign9380_e10293_d_n4;
        locals.var_fn97_calc_iq__muf0_rv = 0.0;

        let (assign9390_e10309, assign9390_e10309_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9390_e10299: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tnomin);
        let assign9390_e10300: f64 = (1.0 + assign9390_e10299);
        let assign9390_e10304: f64 = (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin);
        let assign9390_e10305: f64 = (1.0 + assign9390_e10304);
        let assign9390_e10306: f64 = (assign9390_e10300 / assign9390_e10305);
        let assign9390_e10307: f64 = (locals.var_fn97_calc_iq__vel0 * assign9390_e10306);
        (assign9390_e10307, (locals.var_fn97_calc_iq__vel0 * (-((assign9390_e10300 * (locals.var_fn97_calc_iq__vzeta * locals.var_fn97_calc_iq__tambin_dn4)) / (assign9390_e10305 * assign9390_e10305)))),)
    } else {
        (locals.var_fn97_calc_iq__vx0, locals.var_fn97_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vx0 = assign9390_e10309;
        locals.var_fn97_calc_iq__vx0_dn4 = assign9390_e10309_d_n4;
        locals.var_fn97_calc_iq__vx0_rv = 0.0;

        let (assign9400_e10317, assign9400_e10317_d_n4,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9400_e10313: f64 = (locals.var_fn97_calc_iq__vx0 * locals.var_fn97_calc_iq__lin);
        let assign9400_e10315: f64 = (assign9400_e10313 / locals.var_fn97_calc_iq__muf0);
        (assign9400_e10315, ((((locals.var_fn97_calc_iq__vx0_dn4 * locals.var_fn97_calc_iq__lin) * locals.var_fn97_calc_iq__muf0) - (assign9400_e10313 * locals.var_fn97_calc_iq__muf0_dn4)) / (locals.var_fn97_calc_iq__muf0 * locals.var_fn97_calc_iq__muf0)),)
    } else {
        (locals.var_fn97_calc_iq__vdsats0, locals.var_fn97_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn97_calc_iq__vdsats0 = assign9400_e10317;
        locals.var_fn97_calc_iq__vdsats0_dn4 = assign9400_e10317_d_n4;
        locals.var_fn97_calc_iq__vdsats0_rv = 0.0;

        let (assign9410_e10334, assign9410_e10334_d_n2, assign9410_e10334_d_n4, assign9410_e10334_d_n7, assign9410_e10334_d_n14, assign9410_e10334_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9410_e10323: f64 = (2.0 * locals.var_fn97_calc_iq__qinvv0);
        let assign9410_e10325: f64 = (assign9410_e10323 / locals.var_fn97_calc_iq__cgin);
        let assign9410_e10327: f64 = (assign9410_e10325 / locals.var_fn97_calc_iq__vdsats0);
        let assign9410_e10328: f64 = (1.0 + assign9410_e10327);
        let assign9410_e10329: f64 = (assign9410_e10328).sqrt();
        let assign9410_e10330: f64 = (locals.var_fn97_calc_iq__vdsats0 * assign9410_e10329);
        let assign9410_e10332: f64 = (assign9410_e10330 - locals.var_fn97_calc_iq__vdsats0);
        (assign9410_e10332, (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn2) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))), (((locals.var_fn97_calc_iq__vdsats0_dn4 * assign9410_e10329) + (locals.var_fn97_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn4) * locals.var_fn97_calc_iq__cgin) - (assign9410_e10323 * locals.var_fn97_calc_iq__cgin_dn4)) / (locals.var_fn97_calc_iq__cgin * locals.var_fn97_calc_iq__cgin)) * locals.var_fn97_calc_iq__vdsats0) - (assign9410_e10325 * locals.var_fn97_calc_iq__vdsats0_dn4)) / (locals.var_fn97_calc_iq__vdsats0 * locals.var_fn97_calc_iq__vdsats0)) / (2.0 * assign9410_e10329)))) - locals.var_fn97_calc_iq__vdsats0_dn4), (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn7) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))), (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn14) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))), (locals.var_fn97_calc_iq__vdsats0 * ((((2.0 * locals.var_fn97_calc_iq__qinvv0_dn15) / locals.var_fn97_calc_iq__cgin) / locals.var_fn97_calc_iq__vdsats0) / (2.0 * assign9410_e10329))),)
    } else {
        (locals.var_fn97_calc_iq__vdsats10, locals.var_fn97_calc_iq__vdsats10_dn2, locals.var_fn97_calc_iq__vdsats10_dn4, locals.var_fn97_calc_iq__vdsats10_dn7, locals.var_fn97_calc_iq__vdsats10_dn14, locals.var_fn97_calc_iq__vdsats10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsats10 = assign9410_e10334;
        locals.var_fn97_calc_iq__vdsats10_dn2 = assign9410_e10334_d_n2;
        locals.var_fn97_calc_iq__vdsats10_dn4 = assign9410_e10334_d_n4;
        locals.var_fn97_calc_iq__vdsats10_dn7 = assign9410_e10334_d_n7;
        locals.var_fn97_calc_iq__vdsats10_dn14 = assign9410_e10334_d_n14;
        locals.var_fn97_calc_iq__vdsats10_dn15 = assign9410_e10334_d_n15;
        locals.var_fn97_calc_iq__vdsats10_rv = 0.0;

        let (assign9420_e10346, assign9420_e10346_d_n2, assign9420_e10346_d_n4, assign9420_e10346_d_n7, assign9420_e10346_d_n14, assign9420_e10346_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9420_e10339: f64 = (1.0 - locals.var_fn97_calc_iq__ff0);
        let assign9420_e10340: f64 = (locals.var_fn97_calc_iq__vdsats10 * assign9420_e10339);
        let assign9420_e10343: f64 = (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0);
        let assign9420_e10344: f64 = (assign9420_e10340 + assign9420_e10343);
        (assign9420_e10344, (((locals.var_fn97_calc_iq__vdsats10_dn2 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn2))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn2)), (((locals.var_fn97_calc_iq__vdsats10_dn4 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn4))) + ((locals.var_fn97_calc_iq__two_n_phit0_dn4 * locals.var_fn97_calc_iq__ff0) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn4))), (((locals.var_fn97_calc_iq__vdsats10_dn7 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn7))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn7)), (((locals.var_fn97_calc_iq__vdsats10_dn14 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn14))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn14)), (((locals.var_fn97_calc_iq__vdsats10_dn15 * assign9420_e10339) + (locals.var_fn97_calc_iq__vdsats10 * (-locals.var_fn97_calc_iq__ff0_dn15))) + (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__ff0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vdsat10, locals.var_fn97_calc_iq__vdsat10_dn2, locals.var_fn97_calc_iq__vdsat10_dn4, locals.var_fn97_calc_iq__vdsat10_dn7, locals.var_fn97_calc_iq__vdsat10_dn14, locals.var_fn97_calc_iq__vdsat10_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdsat10 = assign9420_e10346;
        locals.var_fn97_calc_iq__vdsat10_dn2 = assign9420_e10346_d_n2;
        locals.var_fn97_calc_iq__vdsat10_dn4 = assign9420_e10346_d_n4;
        locals.var_fn97_calc_iq__vdsat10_dn7 = assign9420_e10346_d_n7;
        locals.var_fn97_calc_iq__vdsat10_dn14 = assign9420_e10346_d_n14;
        locals.var_fn97_calc_iq__vdsat10_dn15 = assign9420_e10346_d_n15;
        locals.var_fn97_calc_iq__vdsat10_rv = 0.0;

        let (assign9430_e10415, assign9430_e10415_d_n2, assign9430_e10415_d_n4, assign9430_e10415_d_n7, assign9430_e10415_d_n14, assign9430_e10415_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9430_e10405, assign9430_e10405_d_n2, assign9430_e10405_d_n4, assign9430_e10405_d_n7, assign9430_e10405_d_n14, assign9430_e10405_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9430_e10358: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                let assign9430_e10359: f64 = assign9430_e10358;
                let assign9430_e10363: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                let assign9430_e10364: f64 = (-assign9430_e10363);
                let assign9430_e10367: f64 = (0.001 / p.p53);
                let assign9430_e10371: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                let assign9430_e10372: f64 = (-assign9430_e10371);
                let assign9430_e10373: f64 = (assign9430_e10367 * assign9430_e10372);
                let assign9430_e10374: f64 = (assign9430_e10373).tanh();
                let assign9430_e10375: f64 = (assign9430_e10364 * assign9430_e10374);
                let assign9430_e10376: f64 = (assign9430_e10359 + assign9430_e10375);
                let assign9430_e10377: f64 = (0.5 * assign9430_e10376);
                (assign9430_e10377, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10374) + (assign9430_e10364 * ((assign9430_e10367 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9430_e10373).cosh() * (assign9430_e10373).cosh())))))),)
            } else {
                let (assign9430_e10404, assign9430_e10404_d_n2, assign9430_e10404_d_n4, assign9430_e10404_d_n7, assign9430_e10404_d_n14, assign9430_e10404_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9430_e10385: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                        let assign9430_e10386: f64 = assign9430_e10385;
                        let assign9430_e10390: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                        let assign9430_e10391: f64 = (-assign9430_e10390);
                        let assign9430_e10395: f64 = (locals.var_fn97_calc_iq__vdsin / locals.var_fn97_calc_iq__vdsat10);
                        let assign9430_e10396: f64 = (-assign9430_e10395);
                        let assign9430_e10397: f64 = (assign9430_e10391 * assign9430_e10396);
                        let assign9430_e10399: f64 = (assign9430_e10397 + p.p53);
                        let assign9430_e10400: f64 = (assign9430_e10399).sqrt();
                        let assign9430_e10401: f64 = (assign9430_e10386 + assign9430_e10400);
                        let assign9430_e10402: f64 = (0.5 * assign9430_e10401);
                        (assign9430_e10402, (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10396) + (assign9430_e10391 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9430_e10400)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10396) + (assign9430_e10391 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9430_e10400)))), (0.5 * ((-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9430_e10396) + (assign9430_e10391 * (-(-((locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9430_e10400)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10396) + (assign9430_e10391 * (-(((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9430_e10400)))), (0.5 * ((((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9430_e10396) + (assign9430_e10391 * (-(((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__vdsat10) - (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9430_e10400)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9430_e10404, assign9430_e10404_d_n2, assign9430_e10404_d_n4, assign9430_e10404_d_n7, assign9430_e10404_d_n14, assign9430_e10404_d_n15,)
            }
        };
        let assign9430_e10407: f64 = (assign9430_e10405).powf(locals.var_fn97_calc_iq__beta);
        let assign9430_e10408: f64 = (1.0 + assign9430_e10407);
        let assign9430_e10411: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign9430_e10412: f64 = (assign9430_e10408).powf(assign9430_e10411);
        let assign9430_e10413: f64 = (1.0 / assign9430_e10412);
        (assign9430_e10413, (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n2)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n2 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n2)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n2 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n4)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n4 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n4)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n4 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n7)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n7 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n7)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n7 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n14)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n14 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n14)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n14 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))), (-(if 0.0 == 0.0 && ((assign9430_e10411) as f64).is_finite() && ((assign9430_e10411) as f64).fract() == 0.0 { if assign9430_e10411 == 0.0 { 0.0 } else { (assign9430_e10411 * ((assign9430_e10408).powf(assign9430_e10411 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n15)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n15 / assign9430_e10405))) })) } } else { (assign9430_e10412 * (assign9430_e10411 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9430_e10405).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9430_e10405_d_n15)) } } else { (assign9430_e10407 * (locals.var_fn97_calc_iq__beta * (assign9430_e10405_d_n15 / assign9430_e10405))) } / assign9430_e10408))) } / (assign9430_e10412 * assign9430_e10412))),)
    } else {
        (locals.var_fn97_calc_iq__fsd0, locals.var_fn97_calc_iq__fsd0_dn2, locals.var_fn97_calc_iq__fsd0_dn4, locals.var_fn97_calc_iq__fsd0_dn7, locals.var_fn97_calc_iq__fsd0_dn14, locals.var_fn97_calc_iq__fsd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fsd0 = assign9430_e10415;
        locals.var_fn97_calc_iq__fsd0_dn2 = assign9430_e10415_d_n2;
        locals.var_fn97_calc_iq__fsd0_dn4 = assign9430_e10415_d_n4;
        locals.var_fn97_calc_iq__fsd0_dn7 = assign9430_e10415_d_n7;
        locals.var_fn97_calc_iq__fsd0_dn14 = assign9430_e10415_d_n14;
        locals.var_fn97_calc_iq__fsd0_dn15 = assign9430_e10415_d_n15;
        locals.var_fn97_calc_iq__fsd0_rv = 0.0;

        let (assign9440_e10421, assign9440_e10421_d_n2, assign9440_e10421_d_n4, assign9440_e10421_d_n7, assign9440_e10421_d_n14, assign9440_e10421_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9440_e10419: f64 = (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0);
        (assign9440_e10419, (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn2), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn4), (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn7), ((locals.var_fn97_calc_iq__vdsin_dn14 * locals.var_fn97_calc_iq__fsd0) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn14)), ((locals.var_fn97_calc_iq__vdsin_dn15 * locals.var_fn97_calc_iq__fsd0) + (locals.var_fn97_calc_iq__vdsin * locals.var_fn97_calc_iq__fsd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vdx0, locals.var_fn97_calc_iq__vdx0_dn2, locals.var_fn97_calc_iq__vdx0_dn4, locals.var_fn97_calc_iq__vdx0_dn7, locals.var_fn97_calc_iq__vdx0_dn14, locals.var_fn97_calc_iq__vdx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vdx0 = assign9440_e10421;
        locals.var_fn97_calc_iq__vdx0_dn2 = assign9440_e10421_d_n2;
        locals.var_fn97_calc_iq__vdx0_dn4 = assign9440_e10421_d_n4;
        locals.var_fn97_calc_iq__vdx0_dn7 = assign9440_e10421_d_n7;
        locals.var_fn97_calc_iq__vdx0_dn14 = assign9440_e10421_d_n14;
        locals.var_fn97_calc_iq__vdx0_dn15 = assign9440_e10421_d_n15;
        locals.var_fn97_calc_iq__vdx0_rv = 0.0;

        let (assign9450_e10496, assign9450_e10496_d_n2, assign9450_e10496_d_n4, assign9450_e10496_d_n7, assign9450_e10496_d_n14, assign9450_e10496_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let (assign9450_e10486, assign9450_e10486_d_n2, assign9450_e10486_d_n4, assign9450_e10486_d_n7, assign9450_e10486_d_n14, assign9450_e10486_d_n15,) = {
            if (p.p52 != 0.0) {
                let assign9450_e10432: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign9450_e10434: f64 = (assign9450_e10432 / locals.var_fn97_calc_iq__vdsat10);
                let assign9450_e10435: f64 = assign9450_e10434;
                let assign9450_e10438: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign9450_e10440: f64 = (assign9450_e10438 / locals.var_fn97_calc_iq__vdsat10);
                let assign9450_e10441: f64 = (-assign9450_e10440);
                let assign9450_e10444: f64 = (0.001 / p.p53);
                let assign9450_e10447: f64 = (-locals.var_fn97_calc_iq__vdsin);
                let assign9450_e10449: f64 = (assign9450_e10447 / locals.var_fn97_calc_iq__vdsat10);
                let assign9450_e10450: f64 = (-assign9450_e10449);
                let assign9450_e10451: f64 = (assign9450_e10444 * assign9450_e10450);
                let assign9450_e10452: f64 = (assign9450_e10451).tanh();
                let assign9450_e10453: f64 = (assign9450_e10441 * assign9450_e10452);
                let assign9450_e10454: f64 = (assign9450_e10435 + assign9450_e10453);
                let assign9450_e10455: f64 = (0.5 * assign9450_e10454);
                (assign9450_e10455, (0.5 * ((-((assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-(-((assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * ((-((assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-(-((assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * ((-((assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + (((-(-((assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-(-((assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10432 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + (((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10438 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10452) + (assign9450_e10441 * ((assign9450_e10444 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10447 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) / ((assign9450_e10451).cosh() * (assign9450_e10451).cosh())))))),)
            } else {
                let (assign9450_e10485, assign9450_e10485_d_n2, assign9450_e10485_d_n4, assign9450_e10485_d_n7, assign9450_e10485_d_n14, assign9450_e10485_d_n15,) = {
                    if (p.p52 == 0.0) {
                        let assign9450_e10462: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign9450_e10464: f64 = (assign9450_e10462 / locals.var_fn97_calc_iq__vdsat10);
                        let assign9450_e10465: f64 = assign9450_e10464;
                        let assign9450_e10468: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign9450_e10470: f64 = (assign9450_e10468 / locals.var_fn97_calc_iq__vdsat10);
                        let assign9450_e10471: f64 = (-assign9450_e10470);
                        let assign9450_e10474: f64 = (-locals.var_fn97_calc_iq__vdsin);
                        let assign9450_e10476: f64 = (assign9450_e10474 / locals.var_fn97_calc_iq__vdsat10);
                        let assign9450_e10477: f64 = (-assign9450_e10476);
                        let assign9450_e10478: f64 = (assign9450_e10471 * assign9450_e10477);
                        let assign9450_e10480: f64 = (assign9450_e10478 + p.p53);
                        let assign9450_e10481: f64 = (assign9450_e10480).sqrt();
                        let assign9450_e10482: f64 = (assign9450_e10465 + assign9450_e10481);
                        let assign9450_e10483: f64 = (0.5 * assign9450_e10482);
                        (assign9450_e10483, (0.5 * ((-((assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10477) + (assign9450_e10471 * (-(-((assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn2) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9450_e10481)))), (0.5 * ((-((assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10477) + (assign9450_e10471 * (-(-((assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn4) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9450_e10481)))), (0.5 * ((-((assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) + ((((-(-((assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))) * assign9450_e10477) + (assign9450_e10471 * (-(-((assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn7) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)))))) / (2.0 * assign9450_e10481)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10477) + (assign9450_e10471 * (-((((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn14)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9450_e10481)))), (0.5 * (((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10462 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10)) + ((((-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10468 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))) * assign9450_e10477) + (assign9450_e10471 * (-((((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__vdsat10) - (assign9450_e10474 * locals.var_fn97_calc_iq__vdsat10_dn15)) / (locals.var_fn97_calc_iq__vdsat10 * locals.var_fn97_calc_iq__vdsat10))))) / (2.0 * assign9450_e10481)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign9450_e10485, assign9450_e10485_d_n2, assign9450_e10485_d_n4, assign9450_e10485_d_n7, assign9450_e10485_d_n14, assign9450_e10485_d_n15,)
            }
        };
        let assign9450_e10488: f64 = (assign9450_e10486).powf(locals.var_fn97_calc_iq__beta);
        let assign9450_e10489: f64 = (1.0 + assign9450_e10488);
        let assign9450_e10492: f64 = (1.0 / locals.var_fn97_calc_iq__beta);
        let assign9450_e10493: f64 = (assign9450_e10489).powf(assign9450_e10492);
        let assign9450_e10494: f64 = (1.0 / assign9450_e10493);
        (assign9450_e10494, (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n2)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n2 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n2)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n2 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n4)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n4 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n4)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n4 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n7)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n7 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n7)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n7 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n14)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n14 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n14)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n14 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))), (-(if 0.0 == 0.0 && ((assign9450_e10492) as f64).is_finite() && ((assign9450_e10492) as f64).fract() == 0.0 { if assign9450_e10492 == 0.0 { 0.0 } else { (assign9450_e10492 * ((assign9450_e10489).powf(assign9450_e10492 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n15)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n15 / assign9450_e10486))) })) } } else { (assign9450_e10493 * (assign9450_e10492 * (if 0.0 == 0.0 && ((locals.var_fn97_calc_iq__beta) as f64).is_finite() && ((locals.var_fn97_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn97_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn97_calc_iq__beta * ((assign9450_e10486).powf(locals.var_fn97_calc_iq__beta - 1.0) * assign9450_e10486_d_n15)) } } else { (assign9450_e10488 * (locals.var_fn97_calc_iq__beta * (assign9450_e10486_d_n15 / assign9450_e10486))) } / assign9450_e10489))) } / (assign9450_e10493 * assign9450_e10493))),)
    } else {
        (locals.var_fn97_calc_iq__fds0, locals.var_fn97_calc_iq__fds0_dn2, locals.var_fn97_calc_iq__fds0_dn4, locals.var_fn97_calc_iq__fds0_dn7, locals.var_fn97_calc_iq__fds0_dn14, locals.var_fn97_calc_iq__fds0_dn15,)
    }
};
        locals.var_fn97_calc_iq__fds0 = assign9450_e10496;
        locals.var_fn97_calc_iq__fds0_dn2 = assign9450_e10496_d_n2;
        locals.var_fn97_calc_iq__fds0_dn4 = assign9450_e10496_d_n4;
        locals.var_fn97_calc_iq__fds0_dn7 = assign9450_e10496_d_n7;
        locals.var_fn97_calc_iq__fds0_dn14 = assign9450_e10496_d_n14;
        locals.var_fn97_calc_iq__fds0_dn15 = assign9450_e10496_d_n15;
        locals.var_fn97_calc_iq__fds0_rv = 0.0;

        let (assign9460_e10503, assign9460_e10503_d_n2, assign9460_e10503_d_n4, assign9460_e10503_d_n7, assign9460_e10503_d_n14, assign9460_e10503_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9460_e10499: f64 = (-locals.var_fn97_calc_iq__vdsin);
        let assign9460_e10501: f64 = (assign9460_e10499 * locals.var_fn97_calc_iq__fds0);
        (assign9460_e10501, (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn2), (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn4), (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn7), (((-locals.var_fn97_calc_iq__vdsin_dn14) * locals.var_fn97_calc_iq__fds0) + (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn14)), (((-locals.var_fn97_calc_iq__vdsin_dn15) * locals.var_fn97_calc_iq__fds0) + (assign9460_e10499 * locals.var_fn97_calc_iq__fds0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__vsx0, locals.var_fn97_calc_iq__vsx0_dn2, locals.var_fn97_calc_iq__vsx0_dn4, locals.var_fn97_calc_iq__vsx0_dn7, locals.var_fn97_calc_iq__vsx0_dn14, locals.var_fn97_calc_iq__vsx0_dn15,)
    }
};
        locals.var_fn97_calc_iq__vsx0 = assign9460_e10503;
        locals.var_fn97_calc_iq__vsx0_dn2 = assign9460_e10503_d_n2;
        locals.var_fn97_calc_iq__vsx0_dn4 = assign9460_e10503_d_n4;
        locals.var_fn97_calc_iq__vsx0_dn7 = assign9460_e10503_d_n7;
        locals.var_fn97_calc_iq__vsx0_dn14 = assign9460_e10503_d_n14;
        locals.var_fn97_calc_iq__vsx0_dn15 = assign9460_e10503_d_n15;
        locals.var_fn97_calc_iq__vsx0_rv = 0.0;

        let (assign9470_e10511, assign9470_e10511_d_n2, assign9470_e10511_d_n4, assign9470_e10511_d_n7, assign9470_e10511_d_n14, assign9470_e10511_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9470_e10507: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__myarg0);
        let assign9470_e10509: f64 = (assign9470_e10507 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9470_e10509, (locals.var_fn97_calc_iq__vgsin_dn2 / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg0_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9470_e10507 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), (locals.var_fn97_calc_iq__vgsin_dn7 / locals.var_fn97_calc_iq__alpha_phit), (locals.var_fn97_calc_iq__vgsin_dn14 / locals.var_fn97_calc_iq__alpha_phit), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign9470_e10511;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign9470_e10511_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign9470_e10511_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign9470_e10511_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign9470_e10511_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign9470_e10511_d_n15;
        locals.var_fn97_calc_iq__exparg0_rv = 0.0;

        let assign9480_e10514: f64 = if locals.var_fn97_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign9480_e10514;
        locals.var_guard115_rv = 0.0;

        let (assign9490_e10520, assign9490_e10520_d_n2, assign9490_e10520_d_n4, assign9490_e10520_d_n7, assign9490_e10520_d_n14, assign9490_e10520_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard115 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign9490_e10520;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign9490_e10520_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign9490_e10520_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign9490_e10520_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign9490_e10520_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign9490_e10520_d_n15;
        locals.var_fn97_calc_iq__ffs0_rv = 0.0;

        let assign9500_e10523: f64 = (-50.0);
        let assign9500_e10524: f64 = if locals.var_fn97_calc_iq__exparg0 < assign9500_e10523 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign9500_e10524;
        locals.var_guard116_rv = 0.0;

        let (assign9510_e10533, assign9510_e10533_d_n2, assign9510_e10533_d_n4, assign9510_e10533_d_n7, assign9510_e10533_d_n14, assign9510_e10533_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard115 == 0.0)) && (locals.var_guard116 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign9510_e10533;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign9510_e10533_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign9510_e10533_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign9510_e10533_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign9510_e10533_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign9510_e10533_d_n15;
        locals.var_fn97_calc_iq__ffs0_rv = 0.0;

        let (assign9520_e10548, assign9520_e10548_d_n2, assign9520_e10548_d_n4, assign9520_e10548_d_n7, assign9520_e10548_d_n14, assign9520_e10548_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard115 == 0.0)) && (locals.var_guard116 == 0.0)) {
        let assign9520_e10544: f64 = (locals.var_fn97_calc_iq__exparg0).exp();
        let assign9520_e10545: f64 = (1.0 + assign9520_e10544);
        let assign9520_e10546: f64 = (1.0 / assign9520_e10545);
        (assign9520_e10546, (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn2) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn4) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn7) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn14) / (assign9520_e10545 * assign9520_e10545))), (-((assign9520_e10544 * locals.var_fn97_calc_iq__exparg0_dn15) / (assign9520_e10545 * assign9520_e10545))),)
    } else {
        (locals.var_fn97_calc_iq__ffs0, locals.var_fn97_calc_iq__ffs0_dn2, locals.var_fn97_calc_iq__ffs0_dn4, locals.var_fn97_calc_iq__ffs0_dn7, locals.var_fn97_calc_iq__ffs0_dn14, locals.var_fn97_calc_iq__ffs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffs0 = assign9520_e10548;
        locals.var_fn97_calc_iq__ffs0_dn2 = assign9520_e10548_d_n2;
        locals.var_fn97_calc_iq__ffs0_dn4 = assign9520_e10548_d_n4;
        locals.var_fn97_calc_iq__ffs0_dn7 = assign9520_e10548_d_n7;
        locals.var_fn97_calc_iq__ffs0_dn14 = assign9520_e10548_d_n14;
        locals.var_fn97_calc_iq__ffs0_dn15 = assign9520_e10548_d_n15;
        locals.var_fn97_calc_iq__ffs0_rv = 0.0;

        let (assign9530_e10566, assign9530_e10566_d_n2, assign9530_e10566_d_n4, assign9530_e10566_d_n7, assign9530_e10566_d_n14, assign9530_e10566_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9530_e10552: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__vsx0);
        let assign9530_e10556: f64 = (p.p51 * 0.1);
        let assign9530_e10558: f64 = (assign9530_e10556 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9530_e10560: f64 = (assign9530_e10558 * locals.var_fn97_calc_iq__ffs0);
        let assign9530_e10561: f64 = (locals.var_fn97_calc_iq__vtof - assign9530_e10560);
        let assign9530_e10562: f64 = (assign9530_e10552 - assign9530_e10561);
        let assign9530_e10564: f64 = (assign9530_e10562 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9530_e10564, (((locals.var_fn97_calc_iq__vgdin_dn2 - locals.var_fn97_calc_iq__vsx0_dn2) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn2))) / locals.var_fn97_calc_iq__two_n_phit0), (((((-locals.var_fn97_calc_iq__vsx0_dn4) - (locals.var_fn97_calc_iq__vtof_dn4 - (((assign9530_e10556 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffs0) + (assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn4)))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9530_e10562 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (((locals.var_fn97_calc_iq__vgdin_dn7 - locals.var_fn97_calc_iq__vsx0_dn7) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn7))) / locals.var_fn97_calc_iq__two_n_phit0), (((locals.var_fn97_calc_iq__vgdin_dn14 - locals.var_fn97_calc_iq__vsx0_dn14) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn14))) / locals.var_fn97_calc_iq__two_n_phit0), (((locals.var_fn97_calc_iq__vgdin_dn15 - locals.var_fn97_calc_iq__vsx0_dn15) - (-(assign9530_e10558 * locals.var_fn97_calc_iq__ffs0_dn15))) / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etas0, locals.var_fn97_calc_iq__etas0_dn2, locals.var_fn97_calc_iq__etas0_dn4, locals.var_fn97_calc_iq__etas0_dn7, locals.var_fn97_calc_iq__etas0_dn14, locals.var_fn97_calc_iq__etas0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etas0 = assign9530_e10566;
        locals.var_fn97_calc_iq__etas0_dn2 = assign9530_e10566_d_n2;
        locals.var_fn97_calc_iq__etas0_dn4 = assign9530_e10566_d_n4;
        locals.var_fn97_calc_iq__etas0_dn7 = assign9530_e10566_d_n7;
        locals.var_fn97_calc_iq__etas0_dn14 = assign9530_e10566_d_n14;
        locals.var_fn97_calc_iq__etas0_dn15 = assign9530_e10566_d_n15;
        locals.var_fn97_calc_iq__etas0_rv = 0.0;

        let assign9540_e10569: f64 = if locals.var_fn97_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign9540_e10569;
        locals.var_guard117_rv = 0.0;

        let (assign9550_e10577, assign9550_e10577_d_n2, assign9550_e10577_d_n4, assign9550_e10577_d_n7, assign9550_e10577_d_n14, assign9550_e10577_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard117 != 0.0)) {
        let assign9550_e10575: f64 = (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0);
        (assign9550_e10575, (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn2), ((locals.var_fn97_calc_iq__qref0_dn4 * locals.var_fn97_calc_iq__etas0) + (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn4)), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn7), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn14), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etas0_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign9550_e10577;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign9550_e10577_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign9550_e10577_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign9550_e10577_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign9550_e10577_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign9550_e10577_d_n15;
        locals.var_fn97_calc_iq__qinvs0_rv = 0.0;

        let assign9560_e10580: f64 = (-50.0);
        let assign9560_e10581: f64 = if locals.var_fn97_calc_iq__etas0 < assign9560_e10580 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign9560_e10581;
        locals.var_guard118_rv = 0.0;

        let (assign9570_e10593, assign9570_e10593_d_n2, assign9570_e10593_d_n4, assign9570_e10593_d_n7, assign9570_e10593_d_n14, assign9570_e10593_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard117 == 0.0)) && (locals.var_guard118 != 0.0)) {
        let assign9570_e10590: f64 = (locals.var_fn97_calc_iq__etas0).exp();
        let assign9570_e10591: f64 = (locals.var_fn97_calc_iq__qref0 * assign9570_e10590);
        (assign9570_e10591, (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn2)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9570_e10590) + (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn4))), (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn7)), (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn14)), (locals.var_fn97_calc_iq__qref0 * (assign9570_e10590 * locals.var_fn97_calc_iq__etas0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign9570_e10593;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign9570_e10593_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign9570_e10593_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign9570_e10593_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign9570_e10593_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign9570_e10593_d_n15;
        locals.var_fn97_calc_iq__qinvs0_rv = 0.0;

        let (assign9580_e10609, assign9580_e10609_d_n2, assign9580_e10609_d_n4, assign9580_e10609_d_n7, assign9580_e10609_d_n14, assign9580_e10609_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard117 == 0.0)) && (locals.var_guard118 == 0.0)) {
        let assign9580_e10604: f64 = (locals.var_fn97_calc_iq__etas0).exp();
        let assign9580_e10605: f64 = (1.0 + assign9580_e10604);
        let assign9580_e10606: f64 = (assign9580_e10605).ln();
        let assign9580_e10607: f64 = (locals.var_fn97_calc_iq__qref0 * assign9580_e10606);
        (assign9580_e10607, (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn2) / assign9580_e10605)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9580_e10606) + (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn4) / assign9580_e10605))), (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn7) / assign9580_e10605)), (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn14) / assign9580_e10605)), (locals.var_fn97_calc_iq__qref0 * ((assign9580_e10604 * locals.var_fn97_calc_iq__etas0_dn15) / assign9580_e10605)),)
    } else {
        (locals.var_fn97_calc_iq__qinvs0, locals.var_fn97_calc_iq__qinvs0_dn2, locals.var_fn97_calc_iq__qinvs0_dn4, locals.var_fn97_calc_iq__qinvs0_dn7, locals.var_fn97_calc_iq__qinvs0_dn14, locals.var_fn97_calc_iq__qinvs0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvs0 = assign9580_e10609;
        locals.var_fn97_calc_iq__qinvs0_dn2 = assign9580_e10609_d_n2;
        locals.var_fn97_calc_iq__qinvs0_dn4 = assign9580_e10609_d_n4;
        locals.var_fn97_calc_iq__qinvs0_dn7 = assign9580_e10609_d_n7;
        locals.var_fn97_calc_iq__qinvs0_dn14 = assign9580_e10609_d_n14;
        locals.var_fn97_calc_iq__qinvs0_dn15 = assign9580_e10609_d_n15;
        locals.var_fn97_calc_iq__qinvs0_rv = 0.0;

        let (assign9590_e10617, assign9590_e10617_d_n2, assign9590_e10617_d_n4, assign9590_e10617_d_n7, assign9590_e10617_d_n14, assign9590_e10617_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9590_e10613: f64 = (locals.var_fn97_calc_iq__vgdin - locals.var_fn97_calc_iq__myarg0);
        let assign9590_e10615: f64 = (assign9590_e10613 / locals.var_fn97_calc_iq__alpha_phit);
        (assign9590_e10615, (locals.var_fn97_calc_iq__vgdin_dn2 / locals.var_fn97_calc_iq__alpha_phit), ((((-locals.var_fn97_calc_iq__myarg0_dn4) * locals.var_fn97_calc_iq__alpha_phit) - (assign9590_e10613 * locals.var_fn97_calc_iq__alpha_phit_dn4)) / (locals.var_fn97_calc_iq__alpha_phit * locals.var_fn97_calc_iq__alpha_phit)), (locals.var_fn97_calc_iq__vgdin_dn7 / locals.var_fn97_calc_iq__alpha_phit), (locals.var_fn97_calc_iq__vgdin_dn14 / locals.var_fn97_calc_iq__alpha_phit), (locals.var_fn97_calc_iq__vgdin_dn15 / locals.var_fn97_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn97_calc_iq__exparg0, locals.var_fn97_calc_iq__exparg0_dn2, locals.var_fn97_calc_iq__exparg0_dn4, locals.var_fn97_calc_iq__exparg0_dn7, locals.var_fn97_calc_iq__exparg0_dn14, locals.var_fn97_calc_iq__exparg0_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg0 = assign9590_e10617;
        locals.var_fn97_calc_iq__exparg0_dn2 = assign9590_e10617_d_n2;
        locals.var_fn97_calc_iq__exparg0_dn4 = assign9590_e10617_d_n4;
        locals.var_fn97_calc_iq__exparg0_dn7 = assign9590_e10617_d_n7;
        locals.var_fn97_calc_iq__exparg0_dn14 = assign9590_e10617_d_n14;
        locals.var_fn97_calc_iq__exparg0_dn15 = assign9590_e10617_d_n15;
        locals.var_fn97_calc_iq__exparg0_rv = 0.0;

        let assign9600_e10620: f64 = if locals.var_fn97_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign9600_e10620;
        locals.var_guard119_rv = 0.0;

        let (assign9610_e10626, assign9610_e10626_d_n2, assign9610_e10626_d_n4, assign9610_e10626_d_n7, assign9610_e10626_d_n14, assign9610_e10626_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard119 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign9610_e10626;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign9610_e10626_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign9610_e10626_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign9610_e10626_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign9610_e10626_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign9610_e10626_d_n15;
        locals.var_fn97_calc_iq__ffd0_rv = 0.0;

        let assign9620_e10629: f64 = (-50.0);
        let assign9620_e10630: f64 = if locals.var_fn97_calc_iq__exparg0 < assign9620_e10629 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign9620_e10630;
        locals.var_guard120_rv = 0.0;

        let (assign9630_e10639, assign9630_e10639_d_n2, assign9630_e10639_d_n4, assign9630_e10639_d_n7, assign9630_e10639_d_n14, assign9630_e10639_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard119 == 0.0)) && (locals.var_guard120 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign9630_e10639;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign9630_e10639_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign9630_e10639_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign9630_e10639_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign9630_e10639_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign9630_e10639_d_n15;
        locals.var_fn97_calc_iq__ffd0_rv = 0.0;

        let (assign9640_e10654, assign9640_e10654_d_n2, assign9640_e10654_d_n4, assign9640_e10654_d_n7, assign9640_e10654_d_n14, assign9640_e10654_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard119 == 0.0)) && (locals.var_guard120 == 0.0)) {
        let assign9640_e10650: f64 = (locals.var_fn97_calc_iq__exparg0).exp();
        let assign9640_e10651: f64 = (1.0 + assign9640_e10650);
        let assign9640_e10652: f64 = (1.0 / assign9640_e10651);
        (assign9640_e10652, (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn2) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn4) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn7) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn14) / (assign9640_e10651 * assign9640_e10651))), (-((assign9640_e10650 * locals.var_fn97_calc_iq__exparg0_dn15) / (assign9640_e10651 * assign9640_e10651))),)
    } else {
        (locals.var_fn97_calc_iq__ffd0, locals.var_fn97_calc_iq__ffd0_dn2, locals.var_fn97_calc_iq__ffd0_dn4, locals.var_fn97_calc_iq__ffd0_dn7, locals.var_fn97_calc_iq__ffd0_dn14, locals.var_fn97_calc_iq__ffd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__ffd0 = assign9640_e10654;
        locals.var_fn97_calc_iq__ffd0_dn2 = assign9640_e10654_d_n2;
        locals.var_fn97_calc_iq__ffd0_dn4 = assign9640_e10654_d_n4;
        locals.var_fn97_calc_iq__ffd0_dn7 = assign9640_e10654_d_n7;
        locals.var_fn97_calc_iq__ffd0_dn14 = assign9640_e10654_d_n14;
        locals.var_fn97_calc_iq__ffd0_dn15 = assign9640_e10654_d_n15;
        locals.var_fn97_calc_iq__ffd0_rv = 0.0;

        let (assign9650_e10672, assign9650_e10672_d_n2, assign9650_e10672_d_n4, assign9650_e10672_d_n7, assign9650_e10672_d_n14, assign9650_e10672_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9650_e10658: f64 = (locals.var_fn97_calc_iq__vgsin - locals.var_fn97_calc_iq__vdx0);
        let assign9650_e10662: f64 = (p.p51 * 0.1);
        let assign9650_e10664: f64 = (assign9650_e10662 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9650_e10666: f64 = (assign9650_e10664 * locals.var_fn97_calc_iq__ffd0);
        let assign9650_e10667: f64 = (locals.var_fn97_calc_iq__vtof - assign9650_e10666);
        let assign9650_e10668: f64 = (assign9650_e10658 - assign9650_e10667);
        let assign9650_e10670: f64 = (assign9650_e10668 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9650_e10670, (((locals.var_fn97_calc_iq__vgsin_dn2 - locals.var_fn97_calc_iq__vdx0_dn2) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn2))) / locals.var_fn97_calc_iq__two_n_phit0), (((((-locals.var_fn97_calc_iq__vdx0_dn4) - (locals.var_fn97_calc_iq__vtof_dn4 - (((assign9650_e10662 * locals.var_fn97_calc_iq__alpha_phit_dn4) * locals.var_fn97_calc_iq__ffd0) + (assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn4)))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9650_e10668 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (((locals.var_fn97_calc_iq__vgsin_dn7 - locals.var_fn97_calc_iq__vdx0_dn7) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn7))) / locals.var_fn97_calc_iq__two_n_phit0), (((locals.var_fn97_calc_iq__vgsin_dn14 - locals.var_fn97_calc_iq__vdx0_dn14) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn14))) / locals.var_fn97_calc_iq__two_n_phit0), (((-locals.var_fn97_calc_iq__vdx0_dn15) - (-(assign9650_e10664 * locals.var_fn97_calc_iq__ffd0_dn15))) / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etad0, locals.var_fn97_calc_iq__etad0_dn2, locals.var_fn97_calc_iq__etad0_dn4, locals.var_fn97_calc_iq__etad0_dn7, locals.var_fn97_calc_iq__etad0_dn14, locals.var_fn97_calc_iq__etad0_dn15,)
    }
};
        locals.var_fn97_calc_iq__etad0 = assign9650_e10672;
        locals.var_fn97_calc_iq__etad0_dn2 = assign9650_e10672_d_n2;
        locals.var_fn97_calc_iq__etad0_dn4 = assign9650_e10672_d_n4;
        locals.var_fn97_calc_iq__etad0_dn7 = assign9650_e10672_d_n7;
        locals.var_fn97_calc_iq__etad0_dn14 = assign9650_e10672_d_n14;
        locals.var_fn97_calc_iq__etad0_dn15 = assign9650_e10672_d_n15;
        locals.var_fn97_calc_iq__etad0_rv = 0.0;

        let assign9660_e10675: f64 = if locals.var_fn97_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign9660_e10675;
        locals.var_guard121_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9670_e10683, assign9670_e10683_d_n2, assign9670_e10683_d_n4, assign9670_e10683_d_n7, assign9670_e10683_d_n14, assign9670_e10683_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard121 != 0.0)) {
        let assign9670_e10681: f64 = (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0);
        (assign9670_e10681, (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn2), ((locals.var_fn97_calc_iq__qref0_dn4 * locals.var_fn97_calc_iq__etad0) + (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn4)), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn7), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn14), (locals.var_fn97_calc_iq__qref0 * locals.var_fn97_calc_iq__etad0_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign9670_e10683;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign9670_e10683_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign9670_e10683_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign9670_e10683_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign9670_e10683_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign9670_e10683_d_n15;
        locals.var_fn97_calc_iq__qinvd0_rv = 0.0;

        let assign9680_e10686: f64 = (-50.0);
        let assign9680_e10687: f64 = if locals.var_fn97_calc_iq__etad0 < assign9680_e10686 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign9680_e10687;
        locals.var_guard122_rv = 0.0;

        let (assign9690_e10699, assign9690_e10699_d_n2, assign9690_e10699_d_n4, assign9690_e10699_d_n7, assign9690_e10699_d_n14, assign9690_e10699_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 != 0.0)) {
        let assign9690_e10696: f64 = (locals.var_fn97_calc_iq__etad0).exp();
        let assign9690_e10697: f64 = (locals.var_fn97_calc_iq__qref0 * assign9690_e10696);
        (assign9690_e10697, (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn2)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9690_e10696) + (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn4))), (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn7)), (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn14)), (locals.var_fn97_calc_iq__qref0 * (assign9690_e10696 * locals.var_fn97_calc_iq__etad0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign9690_e10699;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign9690_e10699_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign9690_e10699_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign9690_e10699_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign9690_e10699_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign9690_e10699_d_n15;
        locals.var_fn97_calc_iq__qinvd0_rv = 0.0;

        let (assign9700_e10715, assign9700_e10715_d_n2, assign9700_e10715_d_n4, assign9700_e10715_d_n7, assign9700_e10715_d_n14, assign9700_e10715_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard121 == 0.0)) && (locals.var_guard122 == 0.0)) {
        let assign9700_e10710: f64 = (locals.var_fn97_calc_iq__etad0).exp();
        let assign9700_e10711: f64 = (1.0 + assign9700_e10710);
        let assign9700_e10712: f64 = (assign9700_e10711).ln();
        let assign9700_e10713: f64 = (locals.var_fn97_calc_iq__qref0 * assign9700_e10712);
        (assign9700_e10713, (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn2) / assign9700_e10711)), ((locals.var_fn97_calc_iq__qref0_dn4 * assign9700_e10712) + (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn4) / assign9700_e10711))), (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn7) / assign9700_e10711)), (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn14) / assign9700_e10711)), (locals.var_fn97_calc_iq__qref0 * ((assign9700_e10710 * locals.var_fn97_calc_iq__etad0_dn15) / assign9700_e10711)),)
    } else {
        (locals.var_fn97_calc_iq__qinvd0, locals.var_fn97_calc_iq__qinvd0_dn2, locals.var_fn97_calc_iq__qinvd0_dn4, locals.var_fn97_calc_iq__qinvd0_dn7, locals.var_fn97_calc_iq__qinvd0_dn14, locals.var_fn97_calc_iq__qinvd0_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvd0 = assign9700_e10715;
        locals.var_fn97_calc_iq__qinvd0_dn2 = assign9700_e10715_d_n2;
        locals.var_fn97_calc_iq__qinvd0_dn4 = assign9700_e10715_d_n4;
        locals.var_fn97_calc_iq__qinvd0_dn7 = assign9700_e10715_d_n7;
        locals.var_fn97_calc_iq__qinvd0_dn14 = assign9700_e10715_d_n14;
        locals.var_fn97_calc_iq__qinvd0_dn15 = assign9700_e10715_d_n15;
        locals.var_fn97_calc_iq__qinvd0_rv = 0.0;

        let (assign9710_e10723, assign9710_e10723_d_n2, assign9710_e10723_d_n4, assign9710_e10723_d_n7, assign9710_e10723_d_n14, assign9710_e10723_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9710_e10719: f64 = (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0);
        let assign9710_e10721: f64 = (assign9710_e10719 + 1e-38);
        (assign9710_e10721, ((locals.var_fn97_calc_iq__qinvs0_dn2 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn2)), ((locals.var_fn97_calc_iq__qinvs0_dn4 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn4)), ((locals.var_fn97_calc_iq__qinvs0_dn7 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn7)), ((locals.var_fn97_calc_iq__qinvs0_dn14 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn14)), ((locals.var_fn97_calc_iq__qinvs0_dn15 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvs0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qs2, locals.var_fn97_calc_iq__qs2_dn2, locals.var_fn97_calc_iq__qs2_dn4, locals.var_fn97_calc_iq__qs2_dn7, locals.var_fn97_calc_iq__qs2_dn14, locals.var_fn97_calc_iq__qs2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs2 = assign9710_e10723;
        locals.var_fn97_calc_iq__qs2_dn2 = assign9710_e10723_d_n2;
        locals.var_fn97_calc_iq__qs2_dn4 = assign9710_e10723_d_n4;
        locals.var_fn97_calc_iq__qs2_dn7 = assign9710_e10723_d_n7;
        locals.var_fn97_calc_iq__qs2_dn14 = assign9710_e10723_d_n14;
        locals.var_fn97_calc_iq__qs2_dn15 = assign9710_e10723_d_n15;
        locals.var_fn97_calc_iq__qs2_rv = 0.0;

        let (assign9720_e10731, assign9720_e10731_d_n2, assign9720_e10731_d_n4, assign9720_e10731_d_n7, assign9720_e10731_d_n14, assign9720_e10731_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9720_e10727: f64 = (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0);
        let assign9720_e10729: f64 = (assign9720_e10727 + 1e-57);
        (assign9720_e10729, ((locals.var_fn97_calc_iq__qs2_dn2 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn2)), ((locals.var_fn97_calc_iq__qs2_dn4 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn4)), ((locals.var_fn97_calc_iq__qs2_dn7 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn7)), ((locals.var_fn97_calc_iq__qs2_dn14 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn14)), ((locals.var_fn97_calc_iq__qs2_dn15 * locals.var_fn97_calc_iq__qinvs0) + (locals.var_fn97_calc_iq__qs2 * locals.var_fn97_calc_iq__qinvs0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qs3, locals.var_fn97_calc_iq__qs3_dn2, locals.var_fn97_calc_iq__qs3_dn4, locals.var_fn97_calc_iq__qs3_dn7, locals.var_fn97_calc_iq__qs3_dn14, locals.var_fn97_calc_iq__qs3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs3 = assign9720_e10731;
        locals.var_fn97_calc_iq__qs3_dn2 = assign9720_e10731_d_n2;
        locals.var_fn97_calc_iq__qs3_dn4 = assign9720_e10731_d_n4;
        locals.var_fn97_calc_iq__qs3_dn7 = assign9720_e10731_d_n7;
        locals.var_fn97_calc_iq__qs3_dn14 = assign9720_e10731_d_n14;
        locals.var_fn97_calc_iq__qs3_dn15 = assign9720_e10731_d_n15;
        locals.var_fn97_calc_iq__qs3_rv = 0.0;

        let (assign9730_e10739, assign9730_e10739_d_n2, assign9730_e10739_d_n4, assign9730_e10739_d_n7, assign9730_e10739_d_n14, assign9730_e10739_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9730_e10735: f64 = (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0);
        let assign9730_e10737: f64 = (assign9730_e10735 + 1e-38);
        (assign9730_e10737, ((locals.var_fn97_calc_iq__qinvd0_dn2 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn2)), ((locals.var_fn97_calc_iq__qinvd0_dn4 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn4)), ((locals.var_fn97_calc_iq__qinvd0_dn7 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn7)), ((locals.var_fn97_calc_iq__qinvd0_dn14 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn14)), ((locals.var_fn97_calc_iq__qinvd0_dn15 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvd0 * locals.var_fn97_calc_iq__qinvd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qd2, locals.var_fn97_calc_iq__qd2_dn2, locals.var_fn97_calc_iq__qd2_dn4, locals.var_fn97_calc_iq__qd2_dn7, locals.var_fn97_calc_iq__qd2_dn14, locals.var_fn97_calc_iq__qd2_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd2 = assign9730_e10739;
        locals.var_fn97_calc_iq__qd2_dn2 = assign9730_e10739_d_n2;
        locals.var_fn97_calc_iq__qd2_dn4 = assign9730_e10739_d_n4;
        locals.var_fn97_calc_iq__qd2_dn7 = assign9730_e10739_d_n7;
        locals.var_fn97_calc_iq__qd2_dn14 = assign9730_e10739_d_n14;
        locals.var_fn97_calc_iq__qd2_dn15 = assign9730_e10739_d_n15;
        locals.var_fn97_calc_iq__qd2_rv = 0.0;

        let (assign9740_e10747, assign9740_e10747_d_n2, assign9740_e10747_d_n4, assign9740_e10747_d_n7, assign9740_e10747_d_n14, assign9740_e10747_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9740_e10743: f64 = (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0);
        let assign9740_e10745: f64 = (assign9740_e10743 + 1e-57);
        (assign9740_e10745, ((locals.var_fn97_calc_iq__qd2_dn2 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn2)), ((locals.var_fn97_calc_iq__qd2_dn4 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn4)), ((locals.var_fn97_calc_iq__qd2_dn7 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn7)), ((locals.var_fn97_calc_iq__qd2_dn14 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn14)), ((locals.var_fn97_calc_iq__qd2_dn15 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qd2 * locals.var_fn97_calc_iq__qinvd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qd3, locals.var_fn97_calc_iq__qd3_dn2, locals.var_fn97_calc_iq__qd3_dn4, locals.var_fn97_calc_iq__qd3_dn7, locals.var_fn97_calc_iq__qd3_dn14, locals.var_fn97_calc_iq__qd3_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd3 = assign9740_e10747;
        locals.var_fn97_calc_iq__qd3_dn2 = assign9740_e10747_d_n2;
        locals.var_fn97_calc_iq__qd3_dn4 = assign9740_e10747_d_n4;
        locals.var_fn97_calc_iq__qd3_dn7 = assign9740_e10747_d_n7;
        locals.var_fn97_calc_iq__qd3_dn14 = assign9740_e10747_d_n14;
        locals.var_fn97_calc_iq__qd3_dn15 = assign9740_e10747_d_n15;
        locals.var_fn97_calc_iq__qd3_rv = 0.0;

        let (assign9750_e10755, assign9750_e10755_d_n2, assign9750_e10755_d_n4, assign9750_e10755_d_n7, assign9750_e10755_d_n14, assign9750_e10755_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9750_e10751: f64 = (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0);
        let assign9750_e10753: f64 = (assign9750_e10751 + 1e-38);
        (assign9750_e10753, ((locals.var_fn97_calc_iq__qinvs0_dn2 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn2)), ((locals.var_fn97_calc_iq__qinvs0_dn4 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn4)), ((locals.var_fn97_calc_iq__qinvs0_dn7 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn7)), ((locals.var_fn97_calc_iq__qinvs0_dn14 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn14)), ((locals.var_fn97_calc_iq__qinvs0_dn15 * locals.var_fn97_calc_iq__qinvd0) + (locals.var_fn97_calc_iq__qinvs0 * locals.var_fn97_calc_iq__qinvd0_dn15)),)
    } else {
        (locals.var_fn97_calc_iq__qsqd, locals.var_fn97_calc_iq__qsqd_dn2, locals.var_fn97_calc_iq__qsqd_dn4, locals.var_fn97_calc_iq__qsqd_dn7, locals.var_fn97_calc_iq__qsqd_dn14, locals.var_fn97_calc_iq__qsqd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsqd = assign9750_e10755;
        locals.var_fn97_calc_iq__qsqd_dn2 = assign9750_e10755_d_n2;
        locals.var_fn97_calc_iq__qsqd_dn4 = assign9750_e10755_d_n4;
        locals.var_fn97_calc_iq__qsqd_dn7 = assign9750_e10755_d_n7;
        locals.var_fn97_calc_iq__qsqd_dn14 = assign9750_e10755_d_n14;
        locals.var_fn97_calc_iq__qsqd_dn15 = assign9750_e10755_d_n15;
        locals.var_fn97_calc_iq__qsqd_rv = 0.0;

        let (assign9760_e10773, assign9760_e10773_d_n2, assign9760_e10773_d_n4, assign9760_e10773_d_n7, assign9760_e10773_d_n14, assign9760_e10773_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9760_e10759: f64 = (2.0 / 3.0);
        let assign9760_e10762: f64 = (locals.var_fn97_calc_iq__qs2 + locals.var_fn97_calc_iq__qd2);
        let assign9760_e10764: f64 = (assign9760_e10762 + locals.var_fn97_calc_iq__qsqd);
        let assign9760_e10765: f64 = (assign9760_e10759 * assign9760_e10764);
        let assign9760_e10768: f64 = (locals.var_fn97_calc_iq__qinvs0 + locals.var_fn97_calc_iq__qinvd0);
        let assign9760_e10770: f64 = (assign9760_e10768 + 2e-19);
        let assign9760_e10771: f64 = (assign9760_e10765 / assign9760_e10770);
        (assign9760_e10771, ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn2 + locals.var_fn97_calc_iq__qd2_dn2) + locals.var_fn97_calc_iq__qsqd_dn2)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn2 + locals.var_fn97_calc_iq__qinvd0_dn2))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn4 + locals.var_fn97_calc_iq__qd2_dn4) + locals.var_fn97_calc_iq__qsqd_dn4)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn4 + locals.var_fn97_calc_iq__qinvd0_dn4))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn7 + locals.var_fn97_calc_iq__qd2_dn7) + locals.var_fn97_calc_iq__qsqd_dn7)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn7 + locals.var_fn97_calc_iq__qinvd0_dn7))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn14 + locals.var_fn97_calc_iq__qd2_dn14) + locals.var_fn97_calc_iq__qsqd_dn14)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn14 + locals.var_fn97_calc_iq__qinvd0_dn14))) / (assign9760_e10770 * assign9760_e10770)), ((((assign9760_e10759 * ((locals.var_fn97_calc_iq__qs2_dn15 + locals.var_fn97_calc_iq__qd2_dn15) + locals.var_fn97_calc_iq__qsqd_dn15)) * assign9760_e10770) - (assign9760_e10765 * (locals.var_fn97_calc_iq__qinvs0_dn15 + locals.var_fn97_calc_iq__qinvd0_dn15))) / (assign9760_e10770 * assign9760_e10770)),)
    } else {
        (locals.var_fn97_calc_iq__qinvdd, locals.var_fn97_calc_iq__qinvdd_dn2, locals.var_fn97_calc_iq__qinvdd_dn4, locals.var_fn97_calc_iq__qinvdd_dn7, locals.var_fn97_calc_iq__qinvdd_dn14, locals.var_fn97_calc_iq__qinvdd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qinvdd = assign9760_e10773;
        locals.var_fn97_calc_iq__qinvdd_dn2 = assign9760_e10773_d_n2;
        locals.var_fn97_calc_iq__qinvdd_dn4 = assign9760_e10773_d_n4;
        locals.var_fn97_calc_iq__qinvdd_dn7 = assign9760_e10773_d_n7;
        locals.var_fn97_calc_iq__qinvdd_dn14 = assign9760_e10773_d_n14;
        locals.var_fn97_calc_iq__qinvdd_dn15 = assign9760_e10773_d_n15;
        locals.var_fn97_calc_iq__qinvdd_rv = 0.0;

        let (assign9770_e10807, assign9770_e10807_d_n2, assign9770_e10807_d_n4, assign9770_e10807_d_n7, assign9770_e10807_d_n14, assign9770_e10807_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9770_e10778: f64 = (2.0 * locals.var_fn97_calc_iq__qs3);
        let assign9770_e10781: f64 = (3.0 * locals.var_fn97_calc_iq__qd3);
        let assign9770_e10782: f64 = (assign9770_e10778 + assign9770_e10781);
        let assign9770_e10785: f64 = (4.0 * locals.var_fn97_calc_iq__qs2);
        let assign9770_e10787: f64 = (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0);
        let assign9770_e10788: f64 = (assign9770_e10782 + assign9770_e10787);
        let assign9770_e10791: f64 = (6.0 * locals.var_fn97_calc_iq__qd2);
        let assign9770_e10793: f64 = (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0);
        let assign9770_e10794: f64 = (assign9770_e10788 + assign9770_e10793);
        let assign9770_e10795: f64 = (2.0 * assign9770_e10794);
        let assign9770_e10799: f64 = (locals.var_fn97_calc_iq__qs2 + locals.var_fn97_calc_iq__qd2);
        let assign9770_e10802: f64 = (2.0 * locals.var_fn97_calc_iq__qsqd);
        let assign9770_e10803: f64 = (assign9770_e10799 + assign9770_e10802);
        let assign9770_e10804: f64 = (15.0 * assign9770_e10803);
        let assign9770_e10805: f64 = (assign9770_e10795 / assign9770_e10804);
        (assign9770_e10805, ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn2) + (3.0 * locals.var_fn97_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn2) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn2) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn2)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn2 + locals.var_fn97_calc_iq__qd2_dn2) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn2))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn4) + (3.0 * locals.var_fn97_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn4) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn4) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn4)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn4 + locals.var_fn97_calc_iq__qd2_dn4) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn4))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn7) + (3.0 * locals.var_fn97_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn7) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn7) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn7)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn7 + locals.var_fn97_calc_iq__qd2_dn7) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn7))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn14) + (3.0 * locals.var_fn97_calc_iq__qd3_dn14)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn14) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn14))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn14) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn14)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn14 + locals.var_fn97_calc_iq__qd2_dn14) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn14))))) / (assign9770_e10804 * assign9770_e10804)), ((((2.0 * ((((2.0 * locals.var_fn97_calc_iq__qs3_dn15) + (3.0 * locals.var_fn97_calc_iq__qd3_dn15)) + (((4.0 * locals.var_fn97_calc_iq__qs2_dn15) * locals.var_fn97_calc_iq__qinvd0) + (assign9770_e10785 * locals.var_fn97_calc_iq__qinvd0_dn15))) + (((6.0 * locals.var_fn97_calc_iq__qd2_dn15) * locals.var_fn97_calc_iq__qinvs0) + (assign9770_e10791 * locals.var_fn97_calc_iq__qinvs0_dn15)))) * assign9770_e10804) - (assign9770_e10795 * (15.0 * ((locals.var_fn97_calc_iq__qs2_dn15 + locals.var_fn97_calc_iq__qd2_dn15) + (2.0 * locals.var_fn97_calc_iq__qsqd_dn15))))) / (assign9770_e10804 * assign9770_e10804)),)
    } else {
        (locals.var_fn97_calc_iq__qd1, locals.var_fn97_calc_iq__qd1_dn2, locals.var_fn97_calc_iq__qd1_dn4, locals.var_fn97_calc_iq__qd1_dn7, locals.var_fn97_calc_iq__qd1_dn14, locals.var_fn97_calc_iq__qd1_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd1 = assign9770_e10807;
        locals.var_fn97_calc_iq__qd1_dn2 = assign9770_e10807_d_n2;
        locals.var_fn97_calc_iq__qd1_dn4 = assign9770_e10807_d_n4;
        locals.var_fn97_calc_iq__qd1_dn7 = assign9770_e10807_d_n7;
        locals.var_fn97_calc_iq__qd1_dn14 = assign9770_e10807_d_n14;
        locals.var_fn97_calc_iq__qd1_dn15 = assign9770_e10807_d_n15;
        locals.var_fn97_calc_iq__qd1_rv = 0.0;

        let (assign9780_e10813, assign9780_e10813_d_n2, assign9780_e10813_d_n4, assign9780_e10813_d_n7, assign9780_e10813_d_n14, assign9780_e10813_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9780_e10811: f64 = (locals.var_fn97_calc_iq__qinvdd - locals.var_fn97_calc_iq__qd1);
        (assign9780_e10811, (locals.var_fn97_calc_iq__qinvdd_dn2 - locals.var_fn97_calc_iq__qd1_dn2), (locals.var_fn97_calc_iq__qinvdd_dn4 - locals.var_fn97_calc_iq__qd1_dn4), (locals.var_fn97_calc_iq__qinvdd_dn7 - locals.var_fn97_calc_iq__qd1_dn7), (locals.var_fn97_calc_iq__qinvdd_dn14 - locals.var_fn97_calc_iq__qd1_dn14), (locals.var_fn97_calc_iq__qinvdd_dn15 - locals.var_fn97_calc_iq__qd1_dn15),)
    } else {
        (locals.var_fn97_calc_iq__qs, locals.var_fn97_calc_iq__qs_dn2, locals.var_fn97_calc_iq__qs_dn4, locals.var_fn97_calc_iq__qs_dn7, locals.var_fn97_calc_iq__qs_dn14, locals.var_fn97_calc_iq__qs_dn15,)
    }
};
        locals.var_fn97_calc_iq__qs = assign9780_e10813;
        locals.var_fn97_calc_iq__qs_dn2 = assign9780_e10813_d_n2;
        locals.var_fn97_calc_iq__qs_dn4 = assign9780_e10813_d_n4;
        locals.var_fn97_calc_iq__qs_dn7 = assign9780_e10813_d_n7;
        locals.var_fn97_calc_iq__qs_dn14 = assign9780_e10813_d_n14;
        locals.var_fn97_calc_iq__qs_dn15 = assign9780_e10813_d_n15;
        locals.var_fn97_calc_iq__qs_rv = 0.0;

        let (assign9790_e10817, assign9790_e10817_d_n2, assign9790_e10817_d_n4, assign9790_e10817_d_n7, assign9790_e10817_d_n14, assign9790_e10817_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qd1, locals.var_fn97_calc_iq__qd1_dn2, locals.var_fn97_calc_iq__qd1_dn4, locals.var_fn97_calc_iq__qd1_dn7, locals.var_fn97_calc_iq__qd1_dn14, locals.var_fn97_calc_iq__qd1_dn15,)
    } else {
        (locals.var_fn97_calc_iq__qd, locals.var_fn97_calc_iq__qd_dn2, locals.var_fn97_calc_iq__qd_dn4, locals.var_fn97_calc_iq__qd_dn7, locals.var_fn97_calc_iq__qd_dn14, locals.var_fn97_calc_iq__qd_dn15,)
    }
};
        locals.var_fn97_calc_iq__qd = assign9790_e10817;
        locals.var_fn97_calc_iq__qd_dn2 = assign9790_e10817_d_n2;
        locals.var_fn97_calc_iq__qd_dn4 = assign9790_e10817_d_n4;
        locals.var_fn97_calc_iq__qd_dn7 = assign9790_e10817_d_n7;
        locals.var_fn97_calc_iq__qd_dn14 = assign9790_e10817_d_n14;
        locals.var_fn97_calc_iq__qd_dn15 = assign9790_e10817_d_n15;
        locals.var_fn97_calc_iq__qd_rv = 0.0;

        let (assign9800_e10831, assign9800_e10831_d_n2, assign9800_e10831_d_n4, assign9800_e10831_d_n7, assign9800_e10831_d_n14, assign9800_e10831_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9800_e10821: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9800_e10823: f64 = (assign9800_e10821 * locals.var_fn97_calc_iq__lin);
        let assign9800_e10825: f64 = (assign9800_e10823 * locals.var_fn97_calc_iq__type);
        let assign9800_e10827: f64 = (assign9800_e10825 * locals.var_fn97_calc_iq__qs);
        let assign9800_e10829: f64 = (assign9800_e10827 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9800_e10829, ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn4) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9800_e10825 * locals.var_fn97_calc_iq__qs_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qgsout, locals.var_fn97_calc_iq__qgsout_dn2, locals.var_fn97_calc_iq__qgsout_dn4, locals.var_fn97_calc_iq__qgsout_dn7, locals.var_fn97_calc_iq__qgsout_dn14, locals.var_fn97_calc_iq__qgsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgsout = assign9800_e10831;
        locals.var_fn97_calc_iq__qgsout_dn2 = assign9800_e10831_d_n2;
        locals.var_fn97_calc_iq__qgsout_dn4 = assign9800_e10831_d_n4;
        locals.var_fn97_calc_iq__qgsout_dn7 = assign9800_e10831_d_n7;
        locals.var_fn97_calc_iq__qgsout_dn14 = assign9800_e10831_d_n14;
        locals.var_fn97_calc_iq__qgsout_dn15 = assign9800_e10831_d_n15;
        locals.var_fn97_calc_iq__qgsout_rv = 0.0;

        let (assign9810_e10845, assign9810_e10845_d_n2, assign9810_e10845_d_n4, assign9810_e10845_d_n7, assign9810_e10845_d_n14, assign9810_e10845_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9810_e10835: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9810_e10837: f64 = (assign9810_e10835 * locals.var_fn97_calc_iq__lin);
        let assign9810_e10839: f64 = (assign9810_e10837 * locals.var_fn97_calc_iq__type);
        let assign9810_e10841: f64 = (assign9810_e10839 * locals.var_fn97_calc_iq__qd);
        let assign9810_e10843: f64 = (assign9810_e10841 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9810_e10843, ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn4) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9810_e10839 * locals.var_fn97_calc_iq__qd_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qgdout, locals.var_fn97_calc_iq__qgdout_dn2, locals.var_fn97_calc_iq__qgdout_dn4, locals.var_fn97_calc_iq__qgdout_dn7, locals.var_fn97_calc_iq__qgdout_dn14, locals.var_fn97_calc_iq__qgdout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qgdout = assign9810_e10845;
        locals.var_fn97_calc_iq__qgdout_dn2 = assign9810_e10845_d_n2;
        locals.var_fn97_calc_iq__qgdout_dn4 = assign9810_e10845_d_n4;
        locals.var_fn97_calc_iq__qgdout_dn7 = assign9810_e10845_d_n7;
        locals.var_fn97_calc_iq__qgdout_dn14 = assign9810_e10845_d_n14;
        locals.var_fn97_calc_iq__qgdout_dn15 = assign9810_e10845_d_n15;
        locals.var_fn97_calc_iq__qgdout_rv = 0.0;

        let assign9820_e10848: f64 = if locals.var_fn97_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign9820_e10848;
        locals.var_guard123_rv = 0.0;

        let (assign9830_e10864, assign9830_e10864_d_n2, assign9830_e10864_d_n4, assign9830_e10864_d_n7, assign9830_e10864_d_n14,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9830_e10856: f64 = (p.p51 * 0.5);
        let assign9830_e10858: f64 = (assign9830_e10856 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9830_e10859: f64 = (locals.var_fn97_calc_iq__vtof - assign9830_e10858);
        let assign9830_e10860: f64 = (locals.var_fn97_calc_iq__vcin - assign9830_e10859);
        let assign9830_e10862: f64 = (assign9830_e10860 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9830_e10862, (locals.var_fn97_calc_iq__vcin_dn2 / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (assign9830_e10856 * locals.var_fn97_calc_iq__alpha_phit_dn4))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9830_e10860 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (locals.var_fn97_calc_iq__vcin_dn7 / locals.var_fn97_calc_iq__two_n_phit0), (locals.var_fn97_calc_iq__vcin_dn14 / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etac, locals.var_fn97_calc_iq__etac_dn2, locals.var_fn97_calc_iq__etac_dn4, locals.var_fn97_calc_iq__etac_dn7, locals.var_fn97_calc_iq__etac_dn14,)
    }
};
        locals.var_fn97_calc_iq__etac = assign9830_e10864;
        locals.var_fn97_calc_iq__etac_dn2 = assign9830_e10864_d_n2;
        locals.var_fn97_calc_iq__etac_dn4 = assign9830_e10864_d_n4;
        locals.var_fn97_calc_iq__etac_dn7 = assign9830_e10864_d_n7;
        locals.var_fn97_calc_iq__etac_dn14 = assign9830_e10864_d_n14;
        locals.var_fn97_calc_iq__etac_rv = 0.0;

        let assign9840_e10867: f64 = if locals.var_fn97_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign9840_e10867;
        locals.var_guard124_rv = 0.0;

        let (assign9850_e10875, assign9850_e10875_d_n2, assign9850_e10875_d_n3, assign9850_e10875_d_n4, assign9850_e10875_d_n7, assign9850_e10875_d_n14, assign9850_e10875_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard124 != 0.0)) {
        (locals.var_fn97_calc_iq__etac, locals.var_fn97_calc_iq__etac_dn2, 0.0, locals.var_fn97_calc_iq__etac_dn4, locals.var_fn97_calc_iq__etac_dn7, locals.var_fn97_calc_iq__etac_dn14, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9850_e10875;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9850_e10875_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9850_e10875_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9850_e10875_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9850_e10875_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9850_e10875_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9850_e10875_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let assign9860_e10878: f64 = (-50.0);
        let assign9860_e10879: f64 = if locals.var_fn97_calc_iq__etac < assign9860_e10878 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign9860_e10879;
        locals.var_guard125_rv = 0.0;

        let (assign9870_e10891, assign9870_e10891_d_n2, assign9870_e10891_d_n3, assign9870_e10891_d_n4, assign9870_e10891_d_n7, assign9870_e10891_d_n14, assign9870_e10891_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard124 == 0.0)) && (locals.var_guard125 != 0.0)) {
        let assign9870_e10889: f64 = (locals.var_fn97_calc_iq__etac).exp();
        (assign9870_e10889, (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn2), 0.0, (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn4), (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn7), (assign9870_e10889 * locals.var_fn97_calc_iq__etac_dn14), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9870_e10891;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9870_e10891_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9870_e10891_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9870_e10891_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9870_e10891_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9870_e10891_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9870_e10891_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign9880_e10907, assign9880_e10907_d_n2, assign9880_e10907_d_n3, assign9880_e10907_d_n4, assign9880_e10907_d_n7, assign9880_e10907_d_n14, assign9880_e10907_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard124 == 0.0)) && (locals.var_guard125 == 0.0)) {
        let assign9880_e10903: f64 = (locals.var_fn97_calc_iq__etac).exp();
        let assign9880_e10904: f64 = (1.0 + assign9880_e10903);
        let assign9880_e10905: f64 = (assign9880_e10904).ln();
        (assign9880_e10905, ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn2) / assign9880_e10904), 0.0, ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn4) / assign9880_e10904), ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn7) / assign9880_e10904), ((assign9880_e10903 * locals.var_fn97_calc_iq__etac_dn14) / assign9880_e10904), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9880_e10907;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9880_e10907_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9880_e10907_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9880_e10907_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9880_e10907_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9880_e10907_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9880_e10907_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign9890_e10925, assign9890_e10925_d_n2, assign9890_e10925_d_n3, assign9890_e10925_d_n4, assign9890_e10925_d_n7, assign9890_e10925_d_n14, assign9890_e10925_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9890_e10913: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9890_e10915: f64 = (assign9890_e10913 * locals.var_fn97_calc_iq__type);
        let assign9890_e10917: f64 = (assign9890_e10915 * locals.var_fn97_calc_iq__cc);
        let assign9890_e10919: f64 = (assign9890_e10917 * locals.var_fn97_calc_iq__two_n_phit0);
        let assign9890_e10921: f64 = (assign9890_e10919 * locals.var_fn97_calc_iq__exparg);
        let assign9890_e10923: f64 = (assign9890_e10921 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9890_e10923, ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn3) * locals.var_fn97_calc_iq__trapfracdl), ((((((assign9890_e10915 * locals.var_fn97_calc_iq__cc_dn4) * locals.var_fn97_calc_iq__two_n_phit0) + (assign9890_e10917 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) * locals.var_fn97_calc_iq__exparg) + (assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn4)) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9890_e10919 * locals.var_fn97_calc_iq__exparg_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qcout = assign9890_e10925;
        locals.var_fn97_calc_iq__qcout_dn2 = assign9890_e10925_d_n2;
        locals.var_fn97_calc_iq__qcout_dn3 = assign9890_e10925_d_n3;
        locals.var_fn97_calc_iq__qcout_dn4 = assign9890_e10925_d_n4;
        locals.var_fn97_calc_iq__qcout_dn7 = assign9890_e10925_d_n7;
        locals.var_fn97_calc_iq__qcout_dn14 = assign9890_e10925_d_n14;
        locals.var_fn97_calc_iq__qcout_dn15 = assign9890_e10925_d_n15;
        locals.var_fn97_calc_iq__qcout_rv = 0.0;

        let (assign9900_e10941, assign9900_e10941_d_n3, assign9900_e10941_d_n4, assign9900_e10941_d_n14,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9900_e10933: f64 = (p.p51 * 0.5);
        let assign9900_e10935: f64 = (assign9900_e10933 * locals.var_fn97_calc_iq__alpha_phit);
        let assign9900_e10936: f64 = (locals.var_fn97_calc_iq__vtof - assign9900_e10935);
        let assign9900_e10937: f64 = (locals.var_fn97_calc_iq__vbin - assign9900_e10936);
        let assign9900_e10939: f64 = (assign9900_e10937 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign9900_e10939, (locals.var_fn97_calc_iq__vbin_dn3 / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (assign9900_e10933 * locals.var_fn97_calc_iq__alpha_phit_dn4))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign9900_e10937 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (locals.var_fn97_calc_iq__vbin_dn14 / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etab, locals.var_fn97_calc_iq__etab_dn3, locals.var_fn97_calc_iq__etab_dn4, locals.var_fn97_calc_iq__etab_dn14,)
    }
};
        locals.var_fn97_calc_iq__etab = assign9900_e10941;
        locals.var_fn97_calc_iq__etab_dn3 = assign9900_e10941_d_n3;
        locals.var_fn97_calc_iq__etab_dn4 = assign9900_e10941_d_n4;
        locals.var_fn97_calc_iq__etab_dn14 = assign9900_e10941_d_n14;
        locals.var_fn97_calc_iq__etab_rv = 0.0;

        let assign9910_e10944: f64 = if locals.var_fn97_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign9910_e10944;
        locals.var_guard126_rv = 0.0;

        let (assign9920_e10952, assign9920_e10952_d_n2, assign9920_e10952_d_n3, assign9920_e10952_d_n4, assign9920_e10952_d_n7, assign9920_e10952_d_n14, assign9920_e10952_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard126 != 0.0)) {
        (locals.var_fn97_calc_iq__etab, 0.0, locals.var_fn97_calc_iq__etab_dn3, locals.var_fn97_calc_iq__etab_dn4, 0.0, locals.var_fn97_calc_iq__etab_dn14, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9920_e10952;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9920_e10952_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9920_e10952_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9920_e10952_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9920_e10952_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9920_e10952_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9920_e10952_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let assign9930_e10955: f64 = (-50.0);
        let assign9930_e10956: f64 = if locals.var_fn97_calc_iq__etab < assign9930_e10955 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign9930_e10956;
        locals.var_guard127_rv = 0.0;

        let (assign9940_e10968, assign9940_e10968_d_n2, assign9940_e10968_d_n3, assign9940_e10968_d_n4, assign9940_e10968_d_n7, assign9940_e10968_d_n14, assign9940_e10968_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard126 == 0.0)) && (locals.var_guard127 != 0.0)) {
        let assign9940_e10966: f64 = (locals.var_fn97_calc_iq__etab).exp();
        (assign9940_e10966, 0.0, (assign9940_e10966 * locals.var_fn97_calc_iq__etab_dn3), (assign9940_e10966 * locals.var_fn97_calc_iq__etab_dn4), 0.0, (assign9940_e10966 * locals.var_fn97_calc_iq__etab_dn14), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9940_e10968;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9940_e10968_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9940_e10968_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9940_e10968_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9940_e10968_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9940_e10968_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9940_e10968_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign9950_e10984, assign9950_e10984_d_n2, assign9950_e10984_d_n3, assign9950_e10984_d_n4, assign9950_e10984_d_n7, assign9950_e10984_d_n14, assign9950_e10984_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) && (locals.var_guard126 == 0.0)) && (locals.var_guard127 == 0.0)) {
        let assign9950_e10980: f64 = (locals.var_fn97_calc_iq__etab).exp();
        let assign9950_e10981: f64 = (1.0 + assign9950_e10980);
        let assign9950_e10982: f64 = (assign9950_e10981).ln();
        (assign9950_e10982, 0.0, ((assign9950_e10980 * locals.var_fn97_calc_iq__etab_dn3) / assign9950_e10981), ((assign9950_e10980 * locals.var_fn97_calc_iq__etab_dn4) / assign9950_e10981), 0.0, ((assign9950_e10980 * locals.var_fn97_calc_iq__etab_dn14) / assign9950_e10981), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign9950_e10984;
        locals.var_fn97_calc_iq__exparg_dn2 = assign9950_e10984_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign9950_e10984_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign9950_e10984_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign9950_e10984_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign9950_e10984_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign9950_e10984_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign9960_e11002, assign9960_e11002_d_n2, assign9960_e11002_d_n3, assign9960_e11002_d_n4, assign9960_e11002_d_n7, assign9960_e11002_d_n14, assign9960_e11002_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign9960_e10990: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign9960_e10992: f64 = (assign9960_e10990 * locals.var_fn97_calc_iq__type);
        let assign9960_e10994: f64 = (assign9960_e10992 * locals.var_fn97_calc_iq__cb);
        let assign9960_e10996: f64 = (assign9960_e10994 * locals.var_fn97_calc_iq__two_n_phit0);
        let assign9960_e10998: f64 = (assign9960_e10996 * locals.var_fn97_calc_iq__exparg);
        let assign9960_e11000: f64 = (assign9960_e10998 * locals.var_fn97_calc_iq__trapfracdl);
        (assign9960_e11000, ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn3) * locals.var_fn97_calc_iq__trapfracdl), ((((((assign9960_e10992 * locals.var_fn97_calc_iq__cb_dn4) * locals.var_fn97_calc_iq__two_n_phit0) + (assign9960_e10994 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) * locals.var_fn97_calc_iq__exparg) + (assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn4)) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign9960_e10996 * locals.var_fn97_calc_iq__exparg_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qbout = assign9960_e11002;
        locals.var_fn97_calc_iq__qbout_dn2 = assign9960_e11002_d_n2;
        locals.var_fn97_calc_iq__qbout_dn3 = assign9960_e11002_d_n3;
        locals.var_fn97_calc_iq__qbout_dn4 = assign9960_e11002_d_n4;
        locals.var_fn97_calc_iq__qbout_dn7 = assign9960_e11002_d_n7;
        locals.var_fn97_calc_iq__qbout_dn14 = assign9960_e11002_d_n14;
        locals.var_fn97_calc_iq__qbout_dn15 = assign9960_e11002_d_n15;
        locals.var_fn97_calc_iq__qbout_rv = 0.0;

        let (assign9970_e11009, assign9970_e11009_d_n2, assign9970_e11009_d_n3, assign9970_e11009_d_n4, assign9970_e11009_d_n7, assign9970_e11009_d_n14, assign9970_e11009_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qcout = assign9970_e11009;
        locals.var_fn97_calc_iq__qcout_dn2 = assign9970_e11009_d_n2;
        locals.var_fn97_calc_iq__qcout_dn3 = assign9970_e11009_d_n3;
        locals.var_fn97_calc_iq__qcout_dn4 = assign9970_e11009_d_n4;
        locals.var_fn97_calc_iq__qcout_dn7 = assign9970_e11009_d_n7;
        locals.var_fn97_calc_iq__qcout_dn14 = assign9970_e11009_d_n14;
        locals.var_fn97_calc_iq__qcout_dn15 = assign9970_e11009_d_n15;
        locals.var_fn97_calc_iq__qcout_rv = 0.0;

        let (assign9980_e11016, assign9980_e11016_d_n2, assign9980_e11016_d_n3, assign9980_e11016_d_n4, assign9980_e11016_d_n7, assign9980_e11016_d_n14, assign9980_e11016_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard123 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qbout = assign9980_e11016;
        locals.var_fn97_calc_iq__qbout_dn2 = assign9980_e11016_d_n2;
        locals.var_fn97_calc_iq__qbout_dn3 = assign9980_e11016_d_n3;
        locals.var_fn97_calc_iq__qbout_dn4 = assign9980_e11016_d_n4;
        locals.var_fn97_calc_iq__qbout_dn7 = assign9980_e11016_d_n7;
        locals.var_fn97_calc_iq__qbout_dn14 = assign9980_e11016_d_n14;
        locals.var_fn97_calc_iq__qbout_dn15 = assign9980_e11016_d_n15;
        locals.var_fn97_calc_iq__qbout_rv = 0.0;

        let assign9990_e11019: f64 = if locals.var_fn97_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign9990_e11019;
        locals.var_guard128_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10000_e11035, assign10000_e11035_d_n2, assign10000_e11035_d_n4, assign10000_e11035_d_n7, assign10000_e11035_d_n14,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign10000_e11027: f64 = (p.p51 * 0.5);
        let assign10000_e11029: f64 = (assign10000_e11027 * locals.var_fn97_calc_iq__alpha_phit);
        let assign10000_e11030: f64 = (locals.var_fn97_calc_iq__vtof - assign10000_e11029);
        let assign10000_e11031: f64 = (locals.var_fn97_calc_iq__vgsin - assign10000_e11030);
        let assign10000_e11033: f64 = (assign10000_e11031 / locals.var_fn97_calc_iq__two_n_phit0);
        (assign10000_e11033, (locals.var_fn97_calc_iq__vgsin_dn2 / locals.var_fn97_calc_iq__two_n_phit0), ((((-(locals.var_fn97_calc_iq__vtof_dn4 - (assign10000_e11027 * locals.var_fn97_calc_iq__alpha_phit_dn4))) * locals.var_fn97_calc_iq__two_n_phit0) - (assign10000_e11031 * locals.var_fn97_calc_iq__two_n_phit0_dn4)) / (locals.var_fn97_calc_iq__two_n_phit0 * locals.var_fn97_calc_iq__two_n_phit0)), (locals.var_fn97_calc_iq__vgsin_dn7 / locals.var_fn97_calc_iq__two_n_phit0), (locals.var_fn97_calc_iq__vgsin_dn14 / locals.var_fn97_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn97_calc_iq__etags, locals.var_fn97_calc_iq__etags_dn2, locals.var_fn97_calc_iq__etags_dn4, locals.var_fn97_calc_iq__etags_dn7, locals.var_fn97_calc_iq__etags_dn14,)
    }
};
        locals.var_fn97_calc_iq__etags = assign10000_e11035;
        locals.var_fn97_calc_iq__etags_dn2 = assign10000_e11035_d_n2;
        locals.var_fn97_calc_iq__etags_dn4 = assign10000_e11035_d_n4;
        locals.var_fn97_calc_iq__etags_dn7 = assign10000_e11035_d_n7;
        locals.var_fn97_calc_iq__etags_dn14 = assign10000_e11035_d_n14;
        locals.var_fn97_calc_iq__etags_rv = 0.0;

        let assign10010_e11038: f64 = if locals.var_fn97_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign10010_e11038;
        locals.var_guard129_rv = 0.0;

        let (assign10020_e11046, assign10020_e11046_d_n2, assign10020_e11046_d_n3, assign10020_e11046_d_n4, assign10020_e11046_d_n7, assign10020_e11046_d_n14, assign10020_e11046_d_n15,) = {
    if (((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 != 0.0)) {
        (locals.var_fn97_calc_iq__etags, locals.var_fn97_calc_iq__etags_dn2, 0.0, locals.var_fn97_calc_iq__etags_dn4, locals.var_fn97_calc_iq__etags_dn7, locals.var_fn97_calc_iq__etags_dn14, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign10020_e11046;
        locals.var_fn97_calc_iq__exparg_dn2 = assign10020_e11046_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign10020_e11046_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign10020_e11046_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign10020_e11046_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign10020_e11046_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign10020_e11046_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let assign10030_e11049: f64 = (-50.0);
        let assign10030_e11050: f64 = if locals.var_fn97_calc_iq__etags < assign10030_e11049 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign10030_e11050;
        locals.var_guard130_rv = 0.0;

        let (assign10040_e11062, assign10040_e11062_d_n2, assign10040_e11062_d_n3, assign10040_e11062_d_n4, assign10040_e11062_d_n7, assign10040_e11062_d_n14, assign10040_e11062_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) && (locals.var_guard130 != 0.0)) {
        let assign10040_e11060: f64 = (locals.var_fn97_calc_iq__etags).exp();
        (assign10040_e11060, (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn2), 0.0, (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn4), (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn7), (assign10040_e11060 * locals.var_fn97_calc_iq__etags_dn14), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign10040_e11062;
        locals.var_fn97_calc_iq__exparg_dn2 = assign10040_e11062_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign10040_e11062_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign10040_e11062_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign10040_e11062_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign10040_e11062_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign10040_e11062_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign10050_e11078, assign10050_e11078_d_n2, assign10050_e11078_d_n3, assign10050_e11078_d_n4, assign10050_e11078_d_n7, assign10050_e11078_d_n14, assign10050_e11078_d_n15,) = {
    if ((((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) && (locals.var_guard129 == 0.0)) && (locals.var_guard130 == 0.0)) {
        let assign10050_e11074: f64 = (locals.var_fn97_calc_iq__etags).exp();
        let assign10050_e11075: f64 = (1.0 + assign10050_e11074);
        let assign10050_e11076: f64 = (assign10050_e11075).ln();
        (assign10050_e11076, ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn2) / assign10050_e11075), 0.0, ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn4) / assign10050_e11075), ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn7) / assign10050_e11075), ((assign10050_e11074 * locals.var_fn97_calc_iq__etags_dn14) / assign10050_e11075), 0.0,)
    } else {
        (locals.var_fn97_calc_iq__exparg, locals.var_fn97_calc_iq__exparg_dn2, locals.var_fn97_calc_iq__exparg_dn3, locals.var_fn97_calc_iq__exparg_dn4, locals.var_fn97_calc_iq__exparg_dn7, locals.var_fn97_calc_iq__exparg_dn14, locals.var_fn97_calc_iq__exparg_dn15,)
    }
};
        locals.var_fn97_calc_iq__exparg = assign10050_e11078;
        locals.var_fn97_calc_iq__exparg_dn2 = assign10050_e11078_d_n2;
        locals.var_fn97_calc_iq__exparg_dn3 = assign10050_e11078_d_n3;
        locals.var_fn97_calc_iq__exparg_dn4 = assign10050_e11078_d_n4;
        locals.var_fn97_calc_iq__exparg_dn7 = assign10050_e11078_d_n7;
        locals.var_fn97_calc_iq__exparg_dn14 = assign10050_e11078_d_n14;
        locals.var_fn97_calc_iq__exparg_dn15 = assign10050_e11078_d_n15;
        locals.var_fn97_calc_iq__exparg_rv = 0.0;

        let (assign10060_e11096, assign10060_e11096_d_n2, assign10060_e11096_d_n3, assign10060_e11096_d_n4, assign10060_e11096_d_n7, assign10060_e11096_d_n14, assign10060_e11096_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign10060_e11084: f64 = (locals.var_fn97_calc_iq__w * locals.var_fn97_calc_iq__ngf);
        let assign10060_e11086: f64 = (assign10060_e11084 * locals.var_fn97_calc_iq__type);
        let assign10060_e11088: f64 = (assign10060_e11086 * locals.var_fn97_calc_iq__cs);
        let assign10060_e11090: f64 = (assign10060_e11088 * locals.var_fn97_calc_iq__two_n_phit0);
        let assign10060_e11092: f64 = (assign10060_e11090 * locals.var_fn97_calc_iq__exparg);
        let assign10060_e11094: f64 = (assign10060_e11092 * locals.var_fn97_calc_iq__trapfracdl);
        (assign10060_e11094, ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn2) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn3) * locals.var_fn97_calc_iq__trapfracdl), ((((assign10060_e11088 * locals.var_fn97_calc_iq__two_n_phit0_dn4) * locals.var_fn97_calc_iq__exparg) + (assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn4)) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn7) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn14) * locals.var_fn97_calc_iq__trapfracdl), ((assign10060_e11090 * locals.var_fn97_calc_iq__exparg_dn15) * locals.var_fn97_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsout = assign10060_e11096;
        locals.var_fn97_calc_iq__qsout_dn2 = assign10060_e11096_d_n2;
        locals.var_fn97_calc_iq__qsout_dn3 = assign10060_e11096_d_n3;
        locals.var_fn97_calc_iq__qsout_dn4 = assign10060_e11096_d_n4;
        locals.var_fn97_calc_iq__qsout_dn7 = assign10060_e11096_d_n7;
        locals.var_fn97_calc_iq__qsout_dn14 = assign10060_e11096_d_n14;
        locals.var_fn97_calc_iq__qsout_dn15 = assign10060_e11096_d_n15;
        locals.var_fn97_calc_iq__qsout_rv = 0.0;

        let (assign10070_e11103, assign10070_e11103_d_n2, assign10070_e11103_d_n3, assign10070_e11103_d_n4, assign10070_e11103_d_n7, assign10070_e11103_d_n14, assign10070_e11103_d_n15,) = {
    if ((locals.var_guard96 != 0.0) && (locals.var_guard128 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    }
};
        locals.var_fn97_calc_iq__qsout = assign10070_e11103;
        locals.var_fn97_calc_iq__qsout_dn2 = assign10070_e11103_d_n2;
        locals.var_fn97_calc_iq__qsout_dn3 = assign10070_e11103_d_n3;
        locals.var_fn97_calc_iq__qsout_dn4 = assign10070_e11103_d_n4;
        locals.var_fn97_calc_iq__qsout_dn7 = assign10070_e11103_d_n7;
        locals.var_fn97_calc_iq__qsout_dn14 = assign10070_e11103_d_n14;
        locals.var_fn97_calc_iq__qsout_dn15 = assign10070_e11103_d_n15;
        locals.var_fn97_calc_iq__qsout_rv = 0.0;

        let (assign10100_e11115, assign10100_e11115_d_n2, assign10100_e11115_d_n4, assign10100_e11115_d_n7, assign10100_e11115_d_n14, assign10100_e11115_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qgsout, locals.var_fn97_calc_iq__qgsout_dn2, locals.var_fn97_calc_iq__qgsout_dn4, locals.var_fn97_calc_iq__qgsout_dn7, locals.var_fn97_calc_iq__qgsout_dn14, locals.var_fn97_calc_iq__qgsout_dn15,)
    } else {
        (locals.var_qgsfp2, locals.var_qgsfp2_dn2, locals.var_qgsfp2_dn4, locals.var_qgsfp2_dn7, locals.var_qgsfp2_dn14, locals.var_qgsfp2_dn15,)
    }
};
        locals.var_qgsfp2 = assign10100_e11115;
        locals.var_qgsfp2_dn2 = assign10100_e11115_d_n2;
        locals.var_qgsfp2_dn4 = assign10100_e11115_d_n4;
        locals.var_qgsfp2_dn7 = assign10100_e11115_d_n7;
        locals.var_qgsfp2_dn14 = assign10100_e11115_d_n14;
        locals.var_qgsfp2_dn15 = assign10100_e11115_d_n15;
        locals.var_qgsfp2_rv = 0.0;

        let (assign10110_e11119, assign10110_e11119_d_n2, assign10110_e11119_d_n4, assign10110_e11119_d_n7, assign10110_e11119_d_n14, assign10110_e11119_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qgdout, locals.var_fn97_calc_iq__qgdout_dn2, locals.var_fn97_calc_iq__qgdout_dn4, locals.var_fn97_calc_iq__qgdout_dn7, locals.var_fn97_calc_iq__qgdout_dn14, locals.var_fn97_calc_iq__qgdout_dn15,)
    } else {
        (locals.var_qgdfp2, locals.var_qgdfp2_dn2, locals.var_qgdfp2_dn4, locals.var_qgdfp2_dn7, locals.var_qgdfp2_dn14, locals.var_qgdfp2_dn15,)
    }
};
        locals.var_qgdfp2 = assign10110_e11119;
        locals.var_qgdfp2_dn2 = assign10110_e11119_d_n2;
        locals.var_qgdfp2_dn4 = assign10110_e11119_d_n4;
        locals.var_qgdfp2_dn7 = assign10110_e11119_d_n7;
        locals.var_qgdfp2_dn14 = assign10110_e11119_d_n14;
        locals.var_qgdfp2_dn15 = assign10110_e11119_d_n15;
        locals.var_qgdfp2_rv = 0.0;

        let (assign10120_e11123, assign10120_e11123_d_n2, assign10120_e11123_d_n3, assign10120_e11123_d_n4, assign10120_e11123_d_n7, assign10120_e11123_d_n14, assign10120_e11123_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qcout, locals.var_fn97_calc_iq__qcout_dn2, locals.var_fn97_calc_iq__qcout_dn3, locals.var_fn97_calc_iq__qcout_dn4, locals.var_fn97_calc_iq__qcout_dn7, locals.var_fn97_calc_iq__qcout_dn14, locals.var_fn97_calc_iq__qcout_dn15,)
    } else {
        (locals.var_qcfp2, locals.var_qcfp2_dn2, locals.var_qcfp2_dn3, locals.var_qcfp2_dn4, locals.var_qcfp2_dn7, locals.var_qcfp2_dn14, locals.var_qcfp2_dn15,)
    }
};
        locals.var_qcfp2 = assign10120_e11123;
        locals.var_qcfp2_dn2 = assign10120_e11123_d_n2;
        locals.var_qcfp2_dn3 = assign10120_e11123_d_n3;
        locals.var_qcfp2_dn4 = assign10120_e11123_d_n4;
        locals.var_qcfp2_dn7 = assign10120_e11123_d_n7;
        locals.var_qcfp2_dn14 = assign10120_e11123_d_n14;
        locals.var_qcfp2_dn15 = assign10120_e11123_d_n15;
        locals.var_qcfp2_rv = 0.0;

        let (assign10130_e11127, assign10130_e11127_d_n2, assign10130_e11127_d_n3, assign10130_e11127_d_n4, assign10130_e11127_d_n7, assign10130_e11127_d_n14, assign10130_e11127_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qbout, locals.var_fn97_calc_iq__qbout_dn2, locals.var_fn97_calc_iq__qbout_dn3, locals.var_fn97_calc_iq__qbout_dn4, locals.var_fn97_calc_iq__qbout_dn7, locals.var_fn97_calc_iq__qbout_dn14, locals.var_fn97_calc_iq__qbout_dn15,)
    } else {
        (locals.var_qbfp2, locals.var_qbfp2_dn2, locals.var_qbfp2_dn3, locals.var_qbfp2_dn4, locals.var_qbfp2_dn7, locals.var_qbfp2_dn14, locals.var_qbfp2_dn15,)
    }
};
        locals.var_qbfp2 = assign10130_e11127;
        locals.var_qbfp2_dn2 = assign10130_e11127_d_n2;
        locals.var_qbfp2_dn3 = assign10130_e11127_d_n3;
        locals.var_qbfp2_dn4 = assign10130_e11127_d_n4;
        locals.var_qbfp2_dn7 = assign10130_e11127_d_n7;
        locals.var_qbfp2_dn14 = assign10130_e11127_d_n14;
        locals.var_qbfp2_dn15 = assign10130_e11127_d_n15;
        locals.var_qbfp2_rv = 0.0;

        let (assign10140_e11131, assign10140_e11131_d_n2, assign10140_e11131_d_n3, assign10140_e11131_d_n4, assign10140_e11131_d_n7, assign10140_e11131_d_n14, assign10140_e11131_d_n15,) = {
    if (locals.var_guard96 != 0.0) {
        (locals.var_fn97_calc_iq__qsout, locals.var_fn97_calc_iq__qsout_dn2, locals.var_fn97_calc_iq__qsout_dn3, locals.var_fn97_calc_iq__qsout_dn4, locals.var_fn97_calc_iq__qsout_dn7, locals.var_fn97_calc_iq__qsout_dn14, locals.var_fn97_calc_iq__qsout_dn15,)
    } else {
        (locals.var_qsfp2, locals.var_qsfp2_dn2, locals.var_qsfp2_dn3, locals.var_qsfp2_dn4, locals.var_qsfp2_dn7, locals.var_qsfp2_dn14, locals.var_qsfp2_dn15,)
    }
};
        locals.var_qsfp2 = assign10140_e11131;
        locals.var_qsfp2_dn2 = assign10140_e11131_d_n2;
        locals.var_qsfp2_dn3 = assign10140_e11131_d_n3;
        locals.var_qsfp2_dn4 = assign10140_e11131_d_n4;
        locals.var_qsfp2_dn7 = assign10140_e11131_d_n7;
        locals.var_qsfp2_dn14 = assign10140_e11131_d_n14;
        locals.var_qsfp2_dn15 = assign10140_e11131_d_n15;
        locals.var_qsfp2_rv = 0.0;

        let assign10180_e11146: f64 = if p.p188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign10180_e11146;
        locals.var_guard131_rv = 0.0;

        locals.var_qgsfp1 = 0.0;
        locals.var_qgsfp1_dn2 = 0.0;
        locals.var_qgsfp1_dn4 = 0.0;
        locals.var_qgsfp1_dn5 = 0.0;
        locals.var_qgsfp1_dn7 = 0.0;
        locals.var_qgsfp1_dn14 = 0.0;
        locals.var_qgsfp1_rv = 0.0;

        locals.var_qgdfp1 = 0.0;
        locals.var_qgdfp1_dn2 = 0.0;
        locals.var_qgdfp1_dn4 = 0.0;
        locals.var_qgdfp1_dn5 = 0.0;
        locals.var_qgdfp1_dn7 = 0.0;
        locals.var_qgdfp1_dn14 = 0.0;
        locals.var_qgdfp1_rv = 0.0;

        locals.var_qcfp1 = 0.0;
        locals.var_qcfp1_dn2 = 0.0;
        locals.var_qcfp1_dn3 = 0.0;
        locals.var_qcfp1_dn4 = 0.0;
        locals.var_qcfp1_dn5 = 0.0;
        locals.var_qcfp1_dn7 = 0.0;
        locals.var_qcfp1_dn14 = 0.0;
        locals.var_qcfp1_rv = 0.0;

        locals.var_qbfp1 = 0.0;
        locals.var_qbfp1_dn2 = 0.0;
        locals.var_qbfp1_dn3 = 0.0;
        locals.var_qbfp1_dn4 = 0.0;
        locals.var_qbfp1_dn5 = 0.0;
        locals.var_qbfp1_dn7 = 0.0;
        locals.var_qbfp1_dn14 = 0.0;
        locals.var_qbfp1_rv = 0.0;

        locals.var_qsfp1 = 0.0;
        locals.var_qsfp1_dn2 = 0.0;
        locals.var_qsfp1_dn3 = 0.0;
        locals.var_qsfp1_dn4 = 0.0;
        locals.var_qsfp1_dn5 = 0.0;
        locals.var_qsfp1_dn7 = 0.0;
        locals.var_qsfp1_dn14 = 0.0;
        locals.var_qsfp1_rv = 0.0;

        let assign10270_e11157: f64 = if p.p167 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign10270_e11157;
        locals.var_guard132_rv = 0.0;

        let (assign10300_e11169, assign10300_e11169_d_n2, assign10300_e11169_d_n4, assign10300_e11169_d_n5, assign10300_e11169_d_n7, assign10300_e11169_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qgsout, locals.var_fn133_calc_iq__qgsout_dn2, locals.var_fn133_calc_iq__qgsout_dn4, locals.var_fn133_calc_iq__qgsout_dn5, locals.var_fn133_calc_iq__qgsout_dn7, locals.var_fn133_calc_iq__qgsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgsout = assign10300_e11169;
        locals.var_fn133_calc_iq__qgsout_dn2 = assign10300_e11169_d_n2;
        locals.var_fn133_calc_iq__qgsout_dn4 = assign10300_e11169_d_n4;
        locals.var_fn133_calc_iq__qgsout_dn5 = assign10300_e11169_d_n5;
        locals.var_fn133_calc_iq__qgsout_dn7 = assign10300_e11169_d_n7;
        locals.var_fn133_calc_iq__qgsout_dn14 = assign10300_e11169_d_n14;
        locals.var_fn133_calc_iq__qgsout_rv = 0.0;

        let (assign10310_e11173, assign10310_e11173_d_n2, assign10310_e11173_d_n4, assign10310_e11173_d_n5, assign10310_e11173_d_n7, assign10310_e11173_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qgdout, locals.var_fn133_calc_iq__qgdout_dn2, locals.var_fn133_calc_iq__qgdout_dn4, locals.var_fn133_calc_iq__qgdout_dn5, locals.var_fn133_calc_iq__qgdout_dn7, locals.var_fn133_calc_iq__qgdout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qgdout = assign10310_e11173;
        locals.var_fn133_calc_iq__qgdout_dn2 = assign10310_e11173_d_n2;
        locals.var_fn133_calc_iq__qgdout_dn4 = assign10310_e11173_d_n4;
        locals.var_fn133_calc_iq__qgdout_dn5 = assign10310_e11173_d_n5;
        locals.var_fn133_calc_iq__qgdout_dn7 = assign10310_e11173_d_n7;
        locals.var_fn133_calc_iq__qgdout_dn14 = assign10310_e11173_d_n14;
        locals.var_fn133_calc_iq__qgdout_rv = 0.0;

        let (assign10320_e11177, assign10320_e11177_d_n2, assign10320_e11177_d_n3, assign10320_e11177_d_n4, assign10320_e11177_d_n5, assign10320_e11177_d_n7, assign10320_e11177_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qcout, locals.var_fn133_calc_iq__qcout_dn2, locals.var_fn133_calc_iq__qcout_dn3, locals.var_fn133_calc_iq__qcout_dn4, locals.var_fn133_calc_iq__qcout_dn5, locals.var_fn133_calc_iq__qcout_dn7, locals.var_fn133_calc_iq__qcout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qcout = assign10320_e11177;
        locals.var_fn133_calc_iq__qcout_dn2 = assign10320_e11177_d_n2;
        locals.var_fn133_calc_iq__qcout_dn3 = assign10320_e11177_d_n3;
        locals.var_fn133_calc_iq__qcout_dn4 = assign10320_e11177_d_n4;
        locals.var_fn133_calc_iq__qcout_dn5 = assign10320_e11177_d_n5;
        locals.var_fn133_calc_iq__qcout_dn7 = assign10320_e11177_d_n7;
        locals.var_fn133_calc_iq__qcout_dn14 = assign10320_e11177_d_n14;
        locals.var_fn133_calc_iq__qcout_rv = 0.0;

        let (assign10330_e11181, assign10330_e11181_d_n2, assign10330_e11181_d_n3, assign10330_e11181_d_n4, assign10330_e11181_d_n5, assign10330_e11181_d_n7, assign10330_e11181_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qbout, locals.var_fn133_calc_iq__qbout_dn2, locals.var_fn133_calc_iq__qbout_dn3, locals.var_fn133_calc_iq__qbout_dn4, locals.var_fn133_calc_iq__qbout_dn5, locals.var_fn133_calc_iq__qbout_dn7, locals.var_fn133_calc_iq__qbout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qbout = assign10330_e11181;
        locals.var_fn133_calc_iq__qbout_dn2 = assign10330_e11181_d_n2;
        locals.var_fn133_calc_iq__qbout_dn3 = assign10330_e11181_d_n3;
        locals.var_fn133_calc_iq__qbout_dn4 = assign10330_e11181_d_n4;
        locals.var_fn133_calc_iq__qbout_dn5 = assign10330_e11181_d_n5;
        locals.var_fn133_calc_iq__qbout_dn7 = assign10330_e11181_d_n7;
        locals.var_fn133_calc_iq__qbout_dn14 = assign10330_e11181_d_n14;
        locals.var_fn133_calc_iq__qbout_rv = 0.0;

        let (assign10340_e11185, assign10340_e11185_d_n2, assign10340_e11185_d_n3, assign10340_e11185_d_n4, assign10340_e11185_d_n5, assign10340_e11185_d_n7, assign10340_e11185_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qsout, locals.var_fn133_calc_iq__qsout_dn2, locals.var_fn133_calc_iq__qsout_dn3, locals.var_fn133_calc_iq__qsout_dn4, locals.var_fn133_calc_iq__qsout_dn5, locals.var_fn133_calc_iq__qsout_dn7, locals.var_fn133_calc_iq__qsout_dn14,)
    }
};
        locals.var_fn133_calc_iq__qsout = assign10340_e11185;
        locals.var_fn133_calc_iq__qsout_dn2 = assign10340_e11185_d_n2;
        locals.var_fn133_calc_iq__qsout_dn3 = assign10340_e11185_d_n3;
        locals.var_fn133_calc_iq__qsout_dn4 = assign10340_e11185_d_n4;
        locals.var_fn133_calc_iq__qsout_dn5 = assign10340_e11185_d_n5;
        locals.var_fn133_calc_iq__qsout_dn7 = assign10340_e11185_d_n7;
        locals.var_fn133_calc_iq__qsout_dn14 = assign10340_e11185_d_n14;
        locals.var_fn133_calc_iq__qsout_rv = 0.0;

        let (assign10350_e11189, assign10350_e11189_d_n4, assign10350_e11189_d_n5, assign10350_e11189_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vtdibl, locals.var_fn133_calc_iq__vtdibl_dn4, locals.var_fn133_calc_iq__vtdibl_dn5, locals.var_fn133_calc_iq__vtdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vtdibl = assign10350_e11189;
        locals.var_fn133_calc_iq__vtdibl_dn4 = assign10350_e11189_d_n4;
        locals.var_fn133_calc_iq__vtdibl_dn5 = assign10350_e11189_d_n5;
        locals.var_fn133_calc_iq__vtdibl_dn14 = assign10350_e11189_d_n14;
        locals.var_fn133_calc_iq__vtdibl_rv = 0.0;

        let (assign10360_e11193, assign10360_e11193_d_n2, assign10360_e11193_d_n3, assign10360_e11193_d_n4, assign10360_e11193_d_n5, assign10360_e11193_d_n7, assign10360_e11193_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vdsat1, locals.var_fn133_calc_iq__vdsat1_dn2, locals.var_fn133_calc_iq__vdsat1_dn3, locals.var_fn133_calc_iq__vdsat1_dn4, locals.var_fn133_calc_iq__vdsat1_dn5, locals.var_fn133_calc_iq__vdsat1_dn7, locals.var_fn133_calc_iq__vdsat1_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsat1 = assign10360_e11193;
        locals.var_fn133_calc_iq__vdsat1_dn2 = assign10360_e11193_d_n2;
        locals.var_fn133_calc_iq__vdsat1_dn3 = assign10360_e11193_d_n3;
        locals.var_fn133_calc_iq__vdsat1_dn4 = assign10360_e11193_d_n4;
        locals.var_fn133_calc_iq__vdsat1_dn5 = assign10360_e11193_d_n5;
        locals.var_fn133_calc_iq__vdsat1_dn7 = assign10360_e11193_d_n7;
        locals.var_fn133_calc_iq__vdsat1_dn14 = assign10360_e11193_d_n14;
        locals.var_fn133_calc_iq__vdsat1_rv = 0.0;

        let (assign10370_e11197, assign10370_e11197_d_n2, assign10370_e11197_d_n5, assign10370_e11197_d_n7,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vgsfp1, locals.var_vgsfp1_dn2, locals.var_vgsfp1_dn5, locals.var_vgsfp1_dn7,)
    } else {
        (locals.var_fn133_calc_iq__vgsin, locals.var_fn133_calc_iq__vgsin_dn2, locals.var_fn133_calc_iq__vgsin_dn5, locals.var_fn133_calc_iq__vgsin_dn7,)
    }
};
        locals.var_fn133_calc_iq__vgsin = assign10370_e11197;
        locals.var_fn133_calc_iq__vgsin_dn2 = assign10370_e11197_d_n2;
        locals.var_fn133_calc_iq__vgsin_dn5 = assign10370_e11197_d_n5;
        locals.var_fn133_calc_iq__vgsin_dn7 = assign10370_e11197_d_n7;
        locals.var_fn133_calc_iq__vgsin_rv = 0.0;

        let (assign10380_e11201, assign10380_e11201_d_n5, assign10380_e11201_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vdsfp1, locals.var_vdsfp1_dn5, locals.var_vdsfp1_dn14,)
    } else {
        (locals.var_fn133_calc_iq__vdsin, locals.var_fn133_calc_iq__vdsin_dn5, locals.var_fn133_calc_iq__vdsin_dn14,)
    }
};
        locals.var_fn133_calc_iq__vdsin = assign10380_e11201;
        locals.var_fn133_calc_iq__vdsin_dn5 = assign10380_e11201_d_n5;
        locals.var_fn133_calc_iq__vdsin_dn14 = assign10380_e11201_d_n14;
        locals.var_fn133_calc_iq__vdsin_rv = 0.0;

        let (assign10390_e11205,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p173,)
    } else {
        (locals.var_fn133_calc_iq__qcbflag,)
    }
};
        locals.var_fn133_calc_iq__qcbflag = assign10390_e11205;
        locals.var_fn133_calc_iq__qcbflag_rv = 0.0;

        let (assign10400_e11209, assign10400_e11209_d_n2, assign10400_e11209_d_n5, assign10400_e11209_d_n7,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vcfp1, locals.var_vcfp1_dn2, locals.var_vcfp1_dn5, locals.var_vcfp1_dn7,)
    } else {
        (locals.var_fn133_calc_iq__vcin, locals.var_fn133_calc_iq__vcin_dn2, locals.var_fn133_calc_iq__vcin_dn5, locals.var_fn133_calc_iq__vcin_dn7,)
    }
};
        locals.var_fn133_calc_iq__vcin = assign10400_e11209;
        locals.var_fn133_calc_iq__vcin_dn2 = assign10400_e11209_d_n2;
        locals.var_fn133_calc_iq__vcin_dn5 = assign10400_e11209_d_n5;
        locals.var_fn133_calc_iq__vcin_dn7 = assign10400_e11209_d_n7;
        locals.var_fn133_calc_iq__vcin_rv = 0.0;

        let (assign10410_e11213, assign10410_e11213_d_n3, assign10410_e11213_d_n5,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_vbfp1, locals.var_vbfp1_dn3, locals.var_vbfp1_dn5,)
    } else {
        (locals.var_fn133_calc_iq__vbin, locals.var_fn133_calc_iq__vbin_dn3, locals.var_fn133_calc_iq__vbin_dn5,)
    }
};
        locals.var_fn133_calc_iq__vbin = assign10410_e11213;
        locals.var_fn133_calc_iq__vbin_dn3 = assign10410_e11213_d_n3;
        locals.var_fn133_calc_iq__vbin_dn5 = assign10410_e11213_d_n5;
        locals.var_fn133_calc_iq__vbin_rv = 0.0;

        let (assign10420_e11217,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p171,)
    } else {
        (locals.var_fn133_calc_iq__qgsflag,)
    }
};
        locals.var_fn133_calc_iq__qgsflag = assign10420_e11217;
        locals.var_fn133_calc_iq__qgsflag_rv = 0.0;

        let (assign10430_e11221, assign10430_e11221_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn133_calc_iq__tambin, locals.var_fn133_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn133_calc_iq__tambin = assign10430_e11221;
        locals.var_fn133_calc_iq__tambin_dn4 = assign10430_e11221_d_n4;
        locals.var_fn133_calc_iq__tambin_rv = 0.0;

        let (assign10440_e11225,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn133_calc_iq__tnomin,)
    }
};
        locals.var_fn133_calc_iq__tnomin = assign10440_e11225;
        locals.var_fn133_calc_iq__tnomin_rv = 0.0;

        let (assign10450_e11229, assign10450_e11229_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn133_calc_iq__phitin, locals.var_fn133_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn133_calc_iq__phitin = assign10450_e11229;
        locals.var_fn133_calc_iq__phitin_dn4 = assign10450_e11229_d_n4;
        locals.var_fn133_calc_iq__phitin_rv = 0.0;

        let (assign10460_e11233,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn133_calc_iq__w,)
    }
};
        locals.var_fn133_calc_iq__w = assign10460_e11233;
        locals.var_fn133_calc_iq__w_rv = 0.0;

        let (assign10470_e11237,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p167,)
    } else {
        (locals.var_fn133_calc_iq__lin,)
    }
};
        locals.var_fn133_calc_iq__lin = assign10470_e11237;
        locals.var_fn133_calc_iq__lin_rv = 0.0;

        let (assign10480_e11241, assign10480_e11241_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_cgfp1t, locals.var_cgfp1t_dn4,)
    } else {
        (locals.var_fn133_calc_iq__cgin, locals.var_fn133_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn133_calc_iq__cgin = assign10480_e11241;
        locals.var_fn133_calc_iq__cgin_dn4 = assign10480_e11241_d_n4;
        locals.var_fn133_calc_iq__cgin_rv = 0.0;

        let (assign10490_e11245,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p172,)
    } else {
        (locals.var_fn133_calc_iq__cs,)
    }
};
        locals.var_fn133_calc_iq__cs = assign10490_e11245;
        locals.var_fn133_calc_iq__cs_rv = 0.0;

        let (assign10500_e11249, assign10500_e11249_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_ccfp1t, locals.var_ccfp1t_dn4,)
    } else {
        (locals.var_fn133_calc_iq__cc, locals.var_fn133_calc_iq__cc_dn4,)
    }
};
        locals.var_fn133_calc_iq__cc = assign10500_e11249;
        locals.var_fn133_calc_iq__cc_dn4 = assign10500_e11249_d_n4;
        locals.var_fn133_calc_iq__cc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10510_e11253, assign10510_e11253_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (locals.var_cbfp1t, locals.var_cbfp1t_dn4,)
    } else {
        (locals.var_fn133_calc_iq__cb, locals.var_fn133_calc_iq__cb_dn4,)
    }
};
        locals.var_fn133_calc_iq__cb = assign10510_e11253;
        locals.var_fn133_calc_iq__cb_dn4 = assign10510_e11253_d_n4;
        locals.var_fn133_calc_iq__cb_rv = 0.0;

        let (assign10520_e11257,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p168,)
    } else {
        (locals.var_fn133_calc_iq__vto,)
    }
};
        locals.var_fn133_calc_iq__vto = assign10520_e11257;
        locals.var_fn133_calc_iq__vto_rv = 0.0;

        let (assign10530_e11261,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p182,)
    } else {
        (locals.var_fn133_calc_iq__ss,)
    }
};
        locals.var_fn133_calc_iq__ss = assign10530_e11261;
        locals.var_fn133_calc_iq__ss_rv = 0.0;

        let (assign10540_e11265,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p181,)
    } else {
        (locals.var_fn133_calc_iq__delta1,)
    }
};
        locals.var_fn133_calc_iq__delta1 = assign10540_e11265;
        locals.var_fn133_calc_iq__delta1_rv = 0.0;

        let (assign10550_e11269,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn133_calc_iq__delta2,)
    }
};
        locals.var_fn133_calc_iq__delta2 = assign10550_e11269;
        locals.var_fn133_calc_iq__delta2_rv = 0.0;

        let (assign10560_e11273,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p183,)
    } else {
        (locals.var_fn133_calc_iq__nd,)
    }
};
        locals.var_fn133_calc_iq__nd = assign10560_e11273;
        locals.var_fn133_calc_iq__nd_rv = 0.0;

        let (assign10570_e11277,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p187,)
    } else {
        (locals.var_fn133_calc_iq__alpha,)
    }
};
        locals.var_fn133_calc_iq__alpha = assign10570_e11277;
        locals.var_fn133_calc_iq__alpha_rv = 0.0;

        let (assign10580_e11281,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p178,)
    } else {
        (locals.var_fn133_calc_iq__vel0,)
    }
};
        locals.var_fn133_calc_iq__vel0 = assign10580_e11281;
        locals.var_fn133_calc_iq__vel0_rv = 0.0;

        let (assign10590_e11285,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p179,)
    } else {
        (locals.var_fn133_calc_iq__mu0,)
    }
};
        locals.var_fn133_calc_iq__mu0 = assign10590_e11285;
        locals.var_fn133_calc_iq__mu0_rv = 0.0;

        let (assign10600_e11289,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p180,)
    } else {
        (locals.var_fn133_calc_iq__beta,)
    }
};
        locals.var_fn133_calc_iq__beta = assign10600_e11289;
        locals.var_fn133_calc_iq__beta_rv = 0.0;

        let (assign10610_e11293,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p186,)
    } else {
        (locals.var_fn133_calc_iq__mtheta,)
    }
};
        locals.var_fn133_calc_iq__mtheta = assign10610_e11293;
        locals.var_fn133_calc_iq__mtheta_rv = 0.0;

        let (assign10620_e11297,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p185,)
    } else {
        (locals.var_fn133_calc_iq__vtheta,)
    }
};
        locals.var_fn133_calc_iq__vtheta = assign10620_e11297;
        locals.var_fn133_calc_iq__vtheta_rv = 0.0;

        let (assign10630_e11301,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p184,)
    } else {
        (locals.var_fn133_calc_iq__vtzeta,)
    }
};
        locals.var_fn133_calc_iq__vtzeta = assign10630_e11301;
        locals.var_fn133_calc_iq__vtzeta_rv = 0.0;

        let (assign10640_e11305,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn133_calc_iq__dibsat,)
    }
};
        locals.var_fn133_calc_iq__dibsat = assign10640_e11305;
        locals.var_fn133_calc_iq__dibsat_rv = 0.0;

        let (assign10650_e11309,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn133_calc_iq__epsilon,)
    }
};
        locals.var_fn133_calc_iq__epsilon = assign10650_e11309;
        locals.var_fn133_calc_iq__epsilon_rv = 0.0;

        let (assign10660_e11313,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn133_calc_iq__vzeta,)
    }
};
        locals.var_fn133_calc_iq__vzeta = assign10660_e11313;
        locals.var_fn133_calc_iq__vzeta_rv = 0.0;

        let (assign10670_e11317,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn133_calc_iq__lambda,)
    }
};
        locals.var_fn133_calc_iq__lambda = assign10670_e11317;
        locals.var_fn133_calc_iq__lambda_rv = 0.0;

        let (assign10680_e11321,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn133_calc_iq__ngf,)
    }
};
        locals.var_fn133_calc_iq__ngf = assign10680_e11321;
        locals.var_fn133_calc_iq__ngf_rv = 0.0;

        let (assign10690_e11325,) = {
    if (locals.var_guard132 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn133_calc_iq__type,)
    }
};
        locals.var_fn133_calc_iq__type = assign10690_e11325;
        locals.var_fn133_calc_iq__type_rv = 0.0;

        let (assign10700_e11329,) = {
    if (locals.var_guard132 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn133_calc_iq__trapfracdl,)
    }
};
        locals.var_fn133_calc_iq__trapfracdl = assign10700_e11329;
        locals.var_fn133_calc_iq__trapfracdl_rv = 0.0;

        let (assign10710_e11333, assign10710_e11333_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__alpha_phit, locals.var_fn133_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn133_calc_iq__alpha_phit = assign10710_e11333;
        locals.var_fn133_calc_iq__alpha_phit_dn4 = assign10710_e11333_d_n4;
        locals.var_fn133_calc_iq__alpha_phit_rv = 0.0;

        let (assign10720_e11337, assign10720_e11337_d_n5, assign10720_e11337_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__delta, locals.var_fn133_calc_iq__delta_dn5, locals.var_fn133_calc_iq__delta_dn14,)
    }
};
        locals.var_fn133_calc_iq__delta = assign10720_e11337;
        locals.var_fn133_calc_iq__delta_dn5 = assign10720_e11337_d_n5;
        locals.var_fn133_calc_iq__delta_dn14 = assign10720_e11337_d_n14;
        locals.var_fn133_calc_iq__delta_rv = 0.0;

        let (assign10730_e11341, assign10730_e11341_d_n4, assign10730_e11341_d_n5, assign10730_e11341_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__n, locals.var_fn133_calc_iq__n_dn4, locals.var_fn133_calc_iq__n_dn5, locals.var_fn133_calc_iq__n_dn14,)
    }
};
        locals.var_fn133_calc_iq__n = assign10730_e11341;
        locals.var_fn133_calc_iq__n_dn4 = assign10730_e11341_d_n4;
        locals.var_fn133_calc_iq__n_dn5 = assign10730_e11341_d_n5;
        locals.var_fn133_calc_iq__n_dn14 = assign10730_e11341_d_n14;
        locals.var_fn133_calc_iq__n_rv = 0.0;

        let (assign10740_e11345, assign10740_e11345_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vtof, locals.var_fn133_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn133_calc_iq__vtof = assign10740_e11345;
        locals.var_fn133_calc_iq__vtof_dn4 = assign10740_e11345_d_n4;
        locals.var_fn133_calc_iq__vtof_rv = 0.0;

        let (assign10750_e11349, assign10750_e11349_d_n5, assign10750_e11349_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vsatdibl, locals.var_fn133_calc_iq__vsatdibl_dn5, locals.var_fn133_calc_iq__vsatdibl_dn14,)
    }
};
        locals.var_fn133_calc_iq__vsatdibl = assign10750_e11349;
        locals.var_fn133_calc_iq__vsatdibl_dn5 = assign10750_e11349_d_n5;
        locals.var_fn133_calc_iq__vsatdibl_dn14 = assign10750_e11349_d_n14;
        locals.var_fn133_calc_iq__vsatdibl_rv = 0.0;

        let (assign10760_e11353, assign10760_e11353_d_n2, assign10760_e11353_d_n3, assign10760_e11353_d_n4, assign10760_e11353_d_n5, assign10760_e11353_d_n7, assign10760_e11353_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs, locals.var_fn133_calc_iq__ffs_dn2, locals.var_fn133_calc_iq__ffs_dn3, locals.var_fn133_calc_iq__ffs_dn4, locals.var_fn133_calc_iq__ffs_dn5, locals.var_fn133_calc_iq__ffs_dn7, locals.var_fn133_calc_iq__ffs_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs = assign10760_e11353;
        locals.var_fn133_calc_iq__ffs_dn2 = assign10760_e11353_d_n2;
        locals.var_fn133_calc_iq__ffs_dn3 = assign10760_e11353_d_n3;
        locals.var_fn133_calc_iq__ffs_dn4 = assign10760_e11353_d_n4;
        locals.var_fn133_calc_iq__ffs_dn5 = assign10760_e11353_d_n5;
        locals.var_fn133_calc_iq__ffs_dn7 = assign10760_e11353_d_n7;
        locals.var_fn133_calc_iq__ffs_dn14 = assign10760_e11353_d_n14;
        locals.var_fn133_calc_iq__ffs_rv = 0.0;

        let (assign10770_e11357, assign10770_e11357_d_n4, assign10770_e11357_d_n5, assign10770_e11357_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit, locals.var_fn133_calc_iq__two_n_phit_dn4, locals.var_fn133_calc_iq__two_n_phit_dn5, locals.var_fn133_calc_iq__two_n_phit_dn14,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit = assign10770_e11357;
        locals.var_fn133_calc_iq__two_n_phit_dn4 = assign10770_e11357_d_n4;
        locals.var_fn133_calc_iq__two_n_phit_dn5 = assign10770_e11357_d_n5;
        locals.var_fn133_calc_iq__two_n_phit_dn14 = assign10770_e11357_d_n14;
        locals.var_fn133_calc_iq__two_n_phit_rv = 0.0;

        let (assign10780_e11361, assign10780_e11361_d_n4, assign10780_e11361_d_n5, assign10780_e11361_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qref, locals.var_fn133_calc_iq__qref_dn4, locals.var_fn133_calc_iq__qref_dn5, locals.var_fn133_calc_iq__qref_dn14,)
    }
};
        locals.var_fn133_calc_iq__qref = assign10780_e11361;
        locals.var_fn133_calc_iq__qref_dn4 = assign10780_e11361_d_n4;
        locals.var_fn133_calc_iq__qref_dn5 = assign10780_e11361_d_n5;
        locals.var_fn133_calc_iq__qref_dn14 = assign10780_e11361_d_n14;
        locals.var_fn133_calc_iq__qref_rv = 0.0;

        let (assign10790_e11365, assign10790_e11365_d_n2, assign10790_e11365_d_n3, assign10790_e11365_d_n4, assign10790_e11365_d_n5, assign10790_e11365_d_n7, assign10790_e11365_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etas, locals.var_fn133_calc_iq__etas_dn2, locals.var_fn133_calc_iq__etas_dn3, locals.var_fn133_calc_iq__etas_dn4, locals.var_fn133_calc_iq__etas_dn5, locals.var_fn133_calc_iq__etas_dn7, locals.var_fn133_calc_iq__etas_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas = assign10790_e11365;
        locals.var_fn133_calc_iq__etas_dn2 = assign10790_e11365_d_n2;
        locals.var_fn133_calc_iq__etas_dn3 = assign10790_e11365_d_n3;
        locals.var_fn133_calc_iq__etas_dn4 = assign10790_e11365_d_n4;
        locals.var_fn133_calc_iq__etas_dn5 = assign10790_e11365_d_n5;
        locals.var_fn133_calc_iq__etas_dn7 = assign10790_e11365_d_n7;
        locals.var_fn133_calc_iq__etas_dn14 = assign10790_e11365_d_n14;
        locals.var_fn133_calc_iq__etas_rv = 0.0;

        let (assign10800_e11369, assign10800_e11369_d_n2, assign10800_e11369_d_n3, assign10800_e11369_d_n4, assign10800_e11369_d_n5, assign10800_e11369_d_n7, assign10800_e11369_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvs, locals.var_fn133_calc_iq__qinvs_dn2, locals.var_fn133_calc_iq__qinvs_dn3, locals.var_fn133_calc_iq__qinvs_dn4, locals.var_fn133_calc_iq__qinvs_dn5, locals.var_fn133_calc_iq__qinvs_dn7, locals.var_fn133_calc_iq__qinvs_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs = assign10800_e11369;
        locals.var_fn133_calc_iq__qinvs_dn2 = assign10800_e11369_d_n2;
        locals.var_fn133_calc_iq__qinvs_dn3 = assign10800_e11369_d_n3;
        locals.var_fn133_calc_iq__qinvs_dn4 = assign10800_e11369_d_n4;
        locals.var_fn133_calc_iq__qinvs_dn5 = assign10800_e11369_d_n5;
        locals.var_fn133_calc_iq__qinvs_dn7 = assign10800_e11369_d_n7;
        locals.var_fn133_calc_iq__qinvs_dn14 = assign10800_e11369_d_n14;
        locals.var_fn133_calc_iq__qinvs_rv = 0.0;

        let (assign10810_e11373, assign10810_e11373_d_n2, assign10810_e11373_d_n3, assign10810_e11373_d_n4, assign10810_e11373_d_n5, assign10810_e11373_d_n7, assign10810_e11373_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__muf, locals.var_fn133_calc_iq__muf_dn2, locals.var_fn133_calc_iq__muf_dn3, locals.var_fn133_calc_iq__muf_dn4, locals.var_fn133_calc_iq__muf_dn5, locals.var_fn133_calc_iq__muf_dn7, locals.var_fn133_calc_iq__muf_dn14,)
    }
};
        locals.var_fn133_calc_iq__muf = assign10810_e11373;
        locals.var_fn133_calc_iq__muf_dn2 = assign10810_e11373_d_n2;
        locals.var_fn133_calc_iq__muf_dn3 = assign10810_e11373_d_n3;
        locals.var_fn133_calc_iq__muf_dn4 = assign10810_e11373_d_n4;
        locals.var_fn133_calc_iq__muf_dn5 = assign10810_e11373_d_n5;
        locals.var_fn133_calc_iq__muf_dn7 = assign10810_e11373_d_n7;
        locals.var_fn133_calc_iq__muf_dn14 = assign10810_e11373_d_n14;
        locals.var_fn133_calc_iq__muf_rv = 0.0;

        let (assign10820_e11377, assign10820_e11377_d_n2, assign10820_e11377_d_n3, assign10820_e11377_d_n4, assign10820_e11377_d_n5, assign10820_e11377_d_n7, assign10820_e11377_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vx, locals.var_fn133_calc_iq__vx_dn2, locals.var_fn133_calc_iq__vx_dn3, locals.var_fn133_calc_iq__vx_dn4, locals.var_fn133_calc_iq__vx_dn5, locals.var_fn133_calc_iq__vx_dn7, locals.var_fn133_calc_iq__vx_dn14,)
    }
};
        locals.var_fn133_calc_iq__vx = assign10820_e11377;
        locals.var_fn133_calc_iq__vx_dn2 = assign10820_e11377_d_n2;
        locals.var_fn133_calc_iq__vx_dn3 = assign10820_e11377_d_n3;
        locals.var_fn133_calc_iq__vx_dn4 = assign10820_e11377_d_n4;
        locals.var_fn133_calc_iq__vx_dn5 = assign10820_e11377_d_n5;
        locals.var_fn133_calc_iq__vx_dn7 = assign10820_e11377_d_n7;
        locals.var_fn133_calc_iq__vx_dn14 = assign10820_e11377_d_n14;
        locals.var_fn133_calc_iq__vx_rv = 0.0;

        let (assign10840_e11385, assign10840_e11385_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__n0, locals.var_fn133_calc_iq__n0_dn4,)
    }
};
        locals.var_fn133_calc_iq__n0 = assign10840_e11385;
        locals.var_fn133_calc_iq__n0_dn4 = assign10840_e11385_d_n4;
        locals.var_fn133_calc_iq__n0_rv = 0.0;

        let (assign10850_e11389, assign10850_e11389_d_n2, assign10850_e11389_d_n4, assign10850_e11389_d_n5, assign10850_e11389_d_n7, assign10850_e11389_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ffs0, locals.var_fn133_calc_iq__ffs0_dn2, locals.var_fn133_calc_iq__ffs0_dn4, locals.var_fn133_calc_iq__ffs0_dn5, locals.var_fn133_calc_iq__ffs0_dn7, locals.var_fn133_calc_iq__ffs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__ffs0 = assign10850_e11389;
        locals.var_fn133_calc_iq__ffs0_dn2 = assign10850_e11389_d_n2;
        locals.var_fn133_calc_iq__ffs0_dn4 = assign10850_e11389_d_n4;
        locals.var_fn133_calc_iq__ffs0_dn5 = assign10850_e11389_d_n5;
        locals.var_fn133_calc_iq__ffs0_dn7 = assign10850_e11389_d_n7;
        locals.var_fn133_calc_iq__ffs0_dn14 = assign10850_e11389_d_n14;
        locals.var_fn133_calc_iq__ffs0_rv = 0.0;

        let (assign10860_e11393, assign10860_e11393_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__two_n_phit0, locals.var_fn133_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn133_calc_iq__two_n_phit0 = assign10860_e11393;
        locals.var_fn133_calc_iq__two_n_phit0_dn4 = assign10860_e11393_d_n4;
        locals.var_fn133_calc_iq__two_n_phit0_rv = 0.0;

        let (assign10870_e11397, assign10870_e11397_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qref0, locals.var_fn133_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn133_calc_iq__qref0 = assign10870_e11397;
        locals.var_fn133_calc_iq__qref0_dn4 = assign10870_e11397_d_n4;
        locals.var_fn133_calc_iq__qref0_rv = 0.0;

        let (assign10880_e11401, assign10880_e11401_d_n2, assign10880_e11401_d_n4, assign10880_e11401_d_n5, assign10880_e11401_d_n7, assign10880_e11401_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__etas0, locals.var_fn133_calc_iq__etas0_dn2, locals.var_fn133_calc_iq__etas0_dn4, locals.var_fn133_calc_iq__etas0_dn5, locals.var_fn133_calc_iq__etas0_dn7, locals.var_fn133_calc_iq__etas0_dn14,)
    }
};
        locals.var_fn133_calc_iq__etas0 = assign10880_e11401;
        locals.var_fn133_calc_iq__etas0_dn2 = assign10880_e11401_d_n2;
        locals.var_fn133_calc_iq__etas0_dn4 = assign10880_e11401_d_n4;
        locals.var_fn133_calc_iq__etas0_dn5 = assign10880_e11401_d_n5;
        locals.var_fn133_calc_iq__etas0_dn7 = assign10880_e11401_d_n7;
        locals.var_fn133_calc_iq__etas0_dn14 = assign10880_e11401_d_n14;
        locals.var_fn133_calc_iq__etas0_rv = 0.0;

        let (assign10890_e11405, assign10890_e11405_d_n2, assign10890_e11405_d_n4, assign10890_e11405_d_n5, assign10890_e11405_d_n7, assign10890_e11405_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__qinvs0, locals.var_fn133_calc_iq__qinvs0_dn2, locals.var_fn133_calc_iq__qinvs0_dn4, locals.var_fn133_calc_iq__qinvs0_dn5, locals.var_fn133_calc_iq__qinvs0_dn7, locals.var_fn133_calc_iq__qinvs0_dn14,)
    }
};
        locals.var_fn133_calc_iq__qinvs0 = assign10890_e11405;
        locals.var_fn133_calc_iq__qinvs0_dn2 = assign10890_e11405_d_n2;
        locals.var_fn133_calc_iq__qinvs0_dn4 = assign10890_e11405_d_n4;
        locals.var_fn133_calc_iq__qinvs0_dn5 = assign10890_e11405_d_n5;
        locals.var_fn133_calc_iq__qinvs0_dn7 = assign10890_e11405_d_n7;
        locals.var_fn133_calc_iq__qinvs0_dn14 = assign10890_e11405_d_n14;
        locals.var_fn133_calc_iq__qinvs0_rv = 0.0;

        let (assign10900_e11409, assign10900_e11409_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__muf0, locals.var_fn133_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn133_calc_iq__muf0 = assign10900_e11409;
        locals.var_fn133_calc_iq__muf0_dn4 = assign10900_e11409_d_n4;
        locals.var_fn133_calc_iq__muf0_rv = 0.0;

        let (assign10910_e11413, assign10910_e11413_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__vx0, locals.var_fn133_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn133_calc_iq__vx0 = assign10910_e11413;
        locals.var_fn133_calc_iq__vx0_dn4 = assign10910_e11413_d_n4;
        locals.var_fn133_calc_iq__vx0_rv = 0.0;

        let (assign10920_e11417, assign10920_e11417_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__tfacmobin, locals.var_fn133_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn133_calc_iq__tfacmobin = assign10920_e11417;
        locals.var_fn133_calc_iq__tfacmobin_dn4 = assign10920_e11417_d_n4;
        locals.var_fn133_calc_iq__tfacmobin_rv = 0.0;

        let (assign10930_e11421, assign10930_e11421_d_n2, assign10930_e11421_d_n3, assign10930_e11421_d_n4, assign10930_e11421_d_n5, assign10930_e11421_d_n7, assign10930_e11421_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__ff, locals.var_fn133_calc_iq__ff_dn2, locals.var_fn133_calc_iq__ff_dn3, locals.var_fn133_calc_iq__ff_dn4, locals.var_fn133_calc_iq__ff_dn5, locals.var_fn133_calc_iq__ff_dn7, locals.var_fn133_calc_iq__ff_dn14,)
    }
};
        locals.var_fn133_calc_iq__ff = assign10930_e11421;
        locals.var_fn133_calc_iq__ff_dn2 = assign10930_e11421_d_n2;
        locals.var_fn133_calc_iq__ff_dn3 = assign10930_e11421_d_n3;
        locals.var_fn133_calc_iq__ff_dn4 = assign10930_e11421_d_n4;
        locals.var_fn133_calc_iq__ff_dn5 = assign10930_e11421_d_n5;
        locals.var_fn133_calc_iq__ff_dn7 = assign10930_e11421_d_n7;
        locals.var_fn133_calc_iq__ff_dn14 = assign10930_e11421_d_n14;
        locals.var_fn133_calc_iq__ff_rv = 0.0;

        let (assign10940_e11425, assign10940_e11425_d_n2, assign10940_e11425_d_n3, assign10940_e11425_d_n4, assign10940_e11425_d_n5, assign10940_e11425_d_n7, assign10940_e11425_d_n14,) = {
    if (locals.var_guard132 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn133_calc_iq__eta, locals.var_fn133_calc_iq__eta_dn2, locals.var_fn133_calc_iq__eta_dn3, locals.var_fn133_calc_iq__eta_dn4, locals.var_fn133_calc_iq__eta_dn5, locals.var_fn133_calc_iq__eta_dn7, locals.var_fn133_calc_iq__eta_dn14,)
    }
};
        locals.var_fn133_calc_iq__eta = assign10940_e11425;
        locals.var_fn133_calc_iq__eta_dn2 = assign10940_e11425_d_n2;
        locals.var_fn133_calc_iq__eta_dn3 = assign10940_e11425_d_n3;
        locals.var_fn133_calc_iq__eta_dn4 = assign10940_e11425_d_n4;
        locals.var_fn133_calc_iq__eta_dn5 = assign10940_e11425_d_n5;
        locals.var_fn133_calc_iq__eta_dn7 = assign10940_e11425_d_n7;
        locals.var_fn133_calc_iq__eta_dn14 = assign10940_e11425_d_n14;
        locals.var_fn133_calc_iq__eta_rv = 0.0;

    }
}
