#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4550_e4676, assign4550_e4676_d_n1, assign4550_e4676_d_n3, assign4550_e4676_d_n4, assign4550_e4676_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign4550_e4667: f64 = (1.0 - p.p68);
        let assign4550_e4668: f64 = (locals.var_pwq__blk301 * assign4550_e4667);
        let assign4550_e4669: f64 = (1.0 - assign4550_e4668);
        let assign4550_e4670: f64 = (locals.var_pp_t * assign4550_e4669);
        let assign4550_e4673: f64 = (1.0 - p.p81);
        let assign4550_e4674: f64 = (assign4550_e4670 / assign4550_e4673);
        (assign4550_e4674, 0.0, ((locals.var_pp_t_dn3 * assign4550_e4669) / assign4550_e4673), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk302, locals.var_qlo__blk302_dn1, locals.var_qlo__blk302_dn3, locals.var_qlo__blk302_dn4, locals.var_qlo__blk302_dn5,)
    }
};
        locals.var_qlo__blk302 = assign4550_e4676;
        locals.var_qlo__blk302_dn1 = assign4550_e4676_d_n1;
        locals.var_qlo__blk302_dn3 = assign4550_e4676_d_n3;
        locals.var_qlo__blk302_dn4 = assign4550_e4676_d_n4;
        locals.var_qlo__blk302_dn5 = assign4550_e4676_d_n5;
        locals.var_qlo__blk302_rv = 0.0;

        let (assign4560_e4702, assign4560_e4702_d_n1, assign4560_e4702_d_n3, assign4560_e4702_d_n4, assign4560_e4702_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign4560_e4688: f64 = (0.5 * p.p81);
        let assign4560_e4690: f64 = (assign4560_e4688 * locals.var_dvh__blk300);
        let assign4560_e4694: f64 = (1.0 - p.p68);
        let assign4560_e4695: f64 = (locals.var_pp_t * assign4560_e4694);
        let assign4560_e4696: f64 = (assign4560_e4690 / assign4560_e4695);
        let assign4560_e4697: f64 = (1.0 + assign4560_e4696);
        let assign4560_e4698: f64 = (locals.var_dvh__blk300 * assign4560_e4697);
        let assign4560_e4700: f64 = (assign4560_e4698 * locals.var_pwq__blk301);
        (assign4560_e4700, (((locals.var_dvh__blk300_dn1 * assign4560_e4697) + (locals.var_dvh__blk300 * ((assign4560_e4688 * locals.var_dvh__blk300_dn1) / assign4560_e4695))) * locals.var_pwq__blk301), (((locals.var_dvh__blk300_dn3 * assign4560_e4697) + (locals.var_dvh__blk300 * ((((assign4560_e4688 * locals.var_dvh__blk300_dn3) * assign4560_e4695) - (assign4560_e4690 * (locals.var_pp_t_dn3 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * locals.var_pwq__blk301), (((locals.var_dvh__blk300_dn4 * assign4560_e4697) + (locals.var_dvh__blk300 * ((assign4560_e4688 * locals.var_dvh__blk300_dn4) / assign4560_e4695))) * locals.var_pwq__blk301), (((locals.var_dvh__blk300_dn5 * assign4560_e4697) + (locals.var_dvh__blk300 * ((assign4560_e4688 * locals.var_dvh__blk300_dn5) / assign4560_e4695))) * locals.var_pwq__blk301),)
    } else {
        (locals.var_qhi__blk303, locals.var_qhi__blk303_dn1, locals.var_qhi__blk303_dn3, locals.var_qhi__blk303_dn4, locals.var_qhi__blk303_dn5,)
    }
};
        locals.var_qhi__blk303 = assign4560_e4702;
        locals.var_qhi__blk303_dn1 = assign4560_e4702_d_n1;
        locals.var_qhi__blk303_dn3 = assign4560_e4702_d_n3;
        locals.var_qhi__blk303_dn4 = assign4560_e4702_d_n4;
        locals.var_qhi__blk303_dn5 = assign4560_e4702_d_n5;
        locals.var_qhi__blk303_rv = 0.0;

        let (assign4570_e4729, assign4570_e4729_d_n1, assign4570_e4729_d_n3, assign4570_e4729_d_n4, assign4570_e4729_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 == 0.0)) {
        let assign4570_e4716: f64 = (locals.var_vcl / locals.var_pp_t);
        let assign4570_e4717: f64 = (1.0 - assign4570_e4716);
        let assign4570_e4720: f64 = (1.0 - p.p81);
        let assign4570_e4721: f64 = (assign4570_e4717).powf(assign4570_e4720);
        let assign4570_e4722: f64 = (1.0 - assign4570_e4721);
        let assign4570_e4723: f64 = (locals.var_pp_t * assign4570_e4722);
        let assign4570_e4726: f64 = (1.0 - p.p81);
        let assign4570_e4727: f64 = (assign4570_e4723 / assign4570_e4726);
        (assign4570_e4727, ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pp_t)))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(locals.var_vcl_dn1 / locals.var_pp_t)) / assign4570_e4717))) })) / assign4570_e4726), (((locals.var_pp_t_dn3 * assign4570_e4722) + (locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pp_t)))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(locals.var_vcl_dn4 / locals.var_pp_t)) / assign4570_e4717))) })) / assign4570_e4726), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pp_t)))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(locals.var_vcl_dn5 / locals.var_pp_t)) / assign4570_e4717))) })) / assign4570_e4726),)
    } else {
        (locals.var_qlo__blk302, locals.var_qlo__blk302_dn1, locals.var_qlo__blk302_dn3, locals.var_qlo__blk302_dn4, locals.var_qlo__blk302_dn5,)
    }
};
        locals.var_qlo__blk302 = assign4570_e4729;
        locals.var_qlo__blk302_dn1 = assign4570_e4729_d_n1;
        locals.var_qlo__blk302_dn3 = assign4570_e4729_d_n3;
        locals.var_qlo__blk302_dn4 = assign4570_e4729_d_n4;
        locals.var_qlo__blk302_dn5 = assign4570_e4729_d_n5;
        locals.var_qlo__blk302_rv = 0.0;

        let (assign4580_e4740, assign4580_e4740_d_n1, assign4580_e4740_d_n3, assign4580_e4740_d_n4, assign4580_e4740_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk303, locals.var_qhi__blk303_dn1, locals.var_qhi__blk303_dn3, locals.var_qhi__blk303_dn4, locals.var_qhi__blk303_dn5,)
    }
};
        locals.var_qhi__blk303 = assign4580_e4740;
        locals.var_qhi__blk303_dn1 = assign4580_e4740_d_n1;
        locals.var_qhi__blk303_dn3 = assign4580_e4740_d_n3;
        locals.var_qhi__blk303_dn4 = assign4580_e4740_d_n4;
        locals.var_qhi__blk303_dn5 = assign4580_e4740_d_n5;
        locals.var_qhi__blk303_rv = 0.0;

        let (assign4590_e4750, assign4590_e4750_d_n1, assign4590_e4750_d_n3, assign4590_e4750_d_n4, assign4590_e4750_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign4590_e4748: f64 = (locals.var_qlo__blk302 + locals.var_qhi__blk303);
        (assign4590_e4748, (locals.var_qlo__blk302_dn1 + locals.var_qhi__blk303_dn1), (locals.var_qlo__blk302_dn3 + locals.var_qhi__blk303_dn3), (locals.var_qlo__blk302_dn4 + locals.var_qhi__blk303_dn4), (locals.var_qlo__blk302_dn5 + locals.var_qhi__blk303_dn5),)
    } else {
        (locals.var_argp__blk284, locals.var_argp__blk284_dn1, locals.var_argp__blk284_dn3, locals.var_argp__blk284_dn4, locals.var_argp__blk284_dn5,)
    }
};
        locals.var_argp__blk284 = assign4590_e4750;
        locals.var_argp__blk284_dn1 = assign4590_e4750_d_n1;
        locals.var_argp__blk284_dn3 = assign4590_e4750_d_n3;
        locals.var_argp__blk284_dn4 = assign4590_e4750_d_n4;
        locals.var_argp__blk284_dn5 = assign4590_e4750_d_n5;
        locals.var_argp__blk284_rv = 0.0;

        let (assign4600_e4768, assign4600_e4768_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4600_e4759: f64 = (locals.var_dv0__blk299 * locals.var_dv0__blk299);
        let assign4600_e4762: f64 = (4.0 * p.p82);
        let assign4600_e4764: f64 = (assign4600_e4762 * p.p82);
        let assign4600_e4765: f64 = (assign4600_e4759 + assign4600_e4764);
        let assign4600_e4766: f64 = (assign4600_e4765).sqrt();
        (assign4600_e4766, (((locals.var_dv0__blk299_dn3 * locals.var_dv0__blk299) + (locals.var_dv0__blk299 * locals.var_dv0__blk299_dn3)) / (2.0 * assign4600_e4766)),)
    } else {
        (locals.var_mv0__blk304, locals.var_mv0__blk304_dn3,)
    }
};
        locals.var_mv0__blk304 = assign4600_e4768;
        locals.var_mv0__blk304_dn3 = assign4600_e4768_d_n3;
        locals.var_mv0__blk304_rv = 0.0;

        let (assign4610_e4782, assign4610_e4782_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4610_e4776: f64 = (-0.5);
        let assign4610_e4779: f64 = (locals.var_dv0__blk299 + locals.var_mv0__blk304);
        let assign4610_e4780: f64 = (assign4610_e4776 * assign4610_e4779);
        (assign4610_e4780, (assign4610_e4776 * (locals.var_dv0__blk299_dn3 + locals.var_mv0__blk304_dn3)),)
    } else {
        (locals.var_vl0__blk305, locals.var_vl0__blk305_dn3,)
    }
};
        locals.var_vl0__blk305 = assign4610_e4782;
        locals.var_vl0__blk305_dn3 = assign4610_e4782_d_n3;
        locals.var_vl0__blk305_rv = 0.0;

        let (assign4620_e4793, assign4620_e4793_d_n1, assign4620_e4793_d_n3, assign4620_e4793_d_n4, assign4620_e4793_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4620_e4791: f64 = (locals.var_vcl + locals.var_dv0__blk299);
        (assign4620_e4791, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk299_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv__blk306, locals.var_dv__blk306_dn1, locals.var_dv__blk306_dn3, locals.var_dv__blk306_dn4, locals.var_dv__blk306_dn5,)
    }
};
        locals.var_dv__blk306 = assign4620_e4793;
        locals.var_dv__blk306_dn1 = assign4620_e4793_d_n1;
        locals.var_dv__blk306_dn3 = assign4620_e4793_d_n3;
        locals.var_dv__blk306_dn4 = assign4620_e4793_d_n4;
        locals.var_dv__blk306_dn5 = assign4620_e4793_d_n5;
        locals.var_dv__blk306_rv = 0.0;

        let (assign4630_e4811, assign4630_e4811_d_n1, assign4630_e4811_d_n3, assign4630_e4811_d_n4, assign4630_e4811_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4630_e4802: f64 = (locals.var_dv__blk306 * locals.var_dv__blk306);
        let assign4630_e4805: f64 = (4.0 * p.p82);
        let assign4630_e4807: f64 = (assign4630_e4805 * p.p82);
        let assign4630_e4808: f64 = (assign4630_e4802 + assign4630_e4807);
        let assign4630_e4809: f64 = (assign4630_e4808).sqrt();
        (assign4630_e4809, (((locals.var_dv__blk306_dn1 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn1)) / (2.0 * assign4630_e4809)), (((locals.var_dv__blk306_dn3 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn3)) / (2.0 * assign4630_e4809)), (((locals.var_dv__blk306_dn4 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn4)) / (2.0 * assign4630_e4809)), (((locals.var_dv__blk306_dn5 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn5)) / (2.0 * assign4630_e4809)),)
    } else {
        (locals.var_mv__blk307, locals.var_mv__blk307_dn1, locals.var_mv__blk307_dn3, locals.var_mv__blk307_dn4, locals.var_mv__blk307_dn5,)
    }
};
        locals.var_mv__blk307 = assign4630_e4811;
        locals.var_mv__blk307_dn1 = assign4630_e4811_d_n1;
        locals.var_mv__blk307_dn3 = assign4630_e4811_d_n3;
        locals.var_mv__blk307_dn4 = assign4630_e4811_d_n4;
        locals.var_mv__blk307_dn5 = assign4630_e4811_d_n5;
        locals.var_mv__blk307_rv = 0.0;

        let (assign4640_e4826, assign4640_e4826_d_n1, assign4640_e4826_d_n3, assign4640_e4826_d_n4, assign4640_e4826_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4640_e4821: f64 = (locals.var_dv__blk306 - locals.var_mv__blk307);
        let assign4640_e4822: f64 = (0.5 * assign4640_e4821);
        let assign4640_e4824: f64 = (assign4640_e4822 - locals.var_dv0__blk299);
        (assign4640_e4824, (0.5 * (locals.var_dv__blk306_dn1 - locals.var_mv__blk307_dn1)), ((0.5 * (locals.var_dv__blk306_dn3 - locals.var_mv__blk307_dn3)) - locals.var_dv0__blk299_dn3), (0.5 * (locals.var_dv__blk306_dn4 - locals.var_mv__blk307_dn4)), (0.5 * (locals.var_dv__blk306_dn5 - locals.var_mv__blk307_dn5)),)
    } else {
        (locals.var_vl__blk308, locals.var_vl__blk308_dn1, locals.var_vl__blk308_dn3, locals.var_vl__blk308_dn4, locals.var_vl__blk308_dn5,)
    }
};
        locals.var_vl__blk308 = assign4640_e4826;
        locals.var_vl__blk308_dn1 = assign4640_e4826_d_n1;
        locals.var_vl__blk308_dn3 = assign4640_e4826_d_n3;
        locals.var_vl__blk308_dn4 = assign4640_e4826_d_n4;
        locals.var_vl__blk308_dn5 = assign4640_e4826_d_n5;
        locals.var_vl__blk308_rv = 0.0;

        let (assign4650_e4850, assign4650_e4850_d_n1, assign4650_e4850_d_n3, assign4650_e4850_d_n4, assign4650_e4850_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4650_e4834: f64 = (-locals.var_pp_t);
        let assign4650_e4838: f64 = (locals.var_vl__blk308 / locals.var_pp_t);
        let assign4650_e4839: f64 = (1.0 - assign4650_e4838);
        let assign4650_e4842: f64 = (1.0 - p.p81);
        let assign4650_e4843: f64 = (assign4650_e4839).powf(assign4650_e4842);
        let assign4650_e4844: f64 = (assign4650_e4834 * assign4650_e4843);
        let assign4650_e4847: f64 = (1.0 - p.p81);
        let assign4650_e4848: f64 = (assign4650_e4844 / assign4650_e4847);
        (assign4650_e4848, ((assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(locals.var_vl__blk308_dn1 / locals.var_pp_t)))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(locals.var_vl__blk308_dn1 / locals.var_pp_t)) / assign4650_e4839))) }) / assign4650_e4847), ((((-locals.var_pp_t_dn3) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((locals.var_vl__blk308_dn3 * locals.var_pp_t) - (locals.var_vl__blk308 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((locals.var_vl__blk308_dn3 * locals.var_pp_t) - (locals.var_vl__blk308 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(locals.var_vl__blk308_dn4 / locals.var_pp_t)))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(locals.var_vl__blk308_dn4 / locals.var_pp_t)) / assign4650_e4839))) }) / assign4650_e4847), ((assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(locals.var_vl__blk308_dn5 / locals.var_pp_t)))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(locals.var_vl__blk308_dn5 / locals.var_pp_t)) / assign4650_e4839))) }) / assign4650_e4847),)
    } else {
        (locals.var_qlo__blk302, locals.var_qlo__blk302_dn1, locals.var_qlo__blk302_dn3, locals.var_qlo__blk302_dn4, locals.var_qlo__blk302_dn5,)
    }
};
        locals.var_qlo__blk302 = assign4650_e4850;
        locals.var_qlo__blk302_dn1 = assign4650_e4850_d_n1;
        locals.var_qlo__blk302_dn3 = assign4650_e4850_d_n3;
        locals.var_qlo__blk302_dn4 = assign4650_e4850_d_n4;
        locals.var_qlo__blk302_dn5 = assign4650_e4850_d_n5;
        locals.var_qlo__blk302_rv = 0.0;

        let (assign4660_e4890, assign4660_e4890_d_n1, assign4660_e4890_d_n3, assign4660_e4890_d_n4, assign4660_e4890_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4660_e4860: f64 = (1.0 - p.p68);
        let assign4660_e4862: f64 = (-p.p81);
        let assign4660_e4863: f64 = (assign4660_e4860).powf(assign4660_e4862);
        let assign4660_e4866: f64 = (locals.var_vcl - locals.var_vl__blk308);
        let assign4660_e4868: f64 = (assign4660_e4866 + locals.var_vl0__blk305);
        let assign4660_e4869: f64 = (assign4660_e4863 * assign4660_e4868);
        let assign4660_e4873: f64 = (0.5 * p.p81);
        let assign4660_e4876: f64 = (locals.var_vcl - locals.var_vl__blk308);
        let assign4660_e4878: f64 = (assign4660_e4876 + locals.var_vl0__blk305);
        let assign4660_e4879: f64 = (assign4660_e4873 * assign4660_e4878);
        let assign4660_e4883: f64 = (1.0 - p.p68);
        let assign4660_e4884: f64 = (locals.var_pp_t * assign4660_e4883);
        let assign4660_e4885: f64 = (assign4660_e4879 / assign4660_e4884);
        let assign4660_e4886: f64 = (1.0 + assign4660_e4885);
        let assign4660_e4887: f64 = (assign4660_e4869 * assign4660_e4886);
        let assign4660_e4888: f64 = (locals.var_qlo__blk302 + assign4660_e4887);
        (assign4660_e4888, (locals.var_qlo__blk302_dn1 + (((assign4660_e4863 * (locals.var_vcl_dn1 - locals.var_vl__blk308_dn1)) * assign4660_e4886) + (assign4660_e4869 * ((assign4660_e4873 * (locals.var_vcl_dn1 - locals.var_vl__blk308_dn1)) / assign4660_e4884)))), (locals.var_qlo__blk302_dn3 + (((assign4660_e4863 * ((locals.var_vcl_dn3 - locals.var_vl__blk308_dn3) + locals.var_vl0__blk305_dn3)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((locals.var_vcl_dn3 - locals.var_vl__blk308_dn3) + locals.var_vl0__blk305_dn3)) * assign4660_e4884) - (assign4660_e4879 * (locals.var_pp_t_dn3 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (locals.var_qlo__blk302_dn4 + (((assign4660_e4863 * (locals.var_vcl_dn4 - locals.var_vl__blk308_dn4)) * assign4660_e4886) + (assign4660_e4869 * ((assign4660_e4873 * (locals.var_vcl_dn4 - locals.var_vl__blk308_dn4)) / assign4660_e4884)))), (locals.var_qlo__blk302_dn5 + (((assign4660_e4863 * (locals.var_vcl_dn5 - locals.var_vl__blk308_dn5)) * assign4660_e4886) + (assign4660_e4869 * ((assign4660_e4873 * (locals.var_vcl_dn5 - locals.var_vl__blk308_dn5)) / assign4660_e4884)))),)
    } else {
        (locals.var_argp__blk284, locals.var_argp__blk284_dn1, locals.var_argp__blk284_dn3, locals.var_argp__blk284_dn4, locals.var_argp__blk284_dn5,)
    }
};
        locals.var_argp__blk284 = assign4660_e4890;
        locals.var_argp__blk284_dn1 = assign4660_e4890_d_n1;
        locals.var_argp__blk284_dn3 = assign4660_e4890_d_n3;
        locals.var_argp__blk284_dn4 = assign4660_e4890_d_n4;
        locals.var_argp__blk284_dn5 = assign4660_e4890_d_n5;
        locals.var_argp__blk284_rv = 0.0;

        let (assign4670_e4897, assign4670_e4897_d_n1, assign4670_e4897_d_n3, assign4670_e4897_d_n4, assign4670_e4897_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard298 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_argp__blk284, locals.var_argp__blk284_dn1, locals.var_argp__blk284_dn3, locals.var_argp__blk284_dn4, locals.var_argp__blk284_dn5,)
    }
};
        locals.var_argp__blk284 = assign4670_e4897;
        locals.var_argp__blk284_dn1 = assign4670_e4897_d_n1;
        locals.var_argp__blk284_dn3 = assign4670_e4897_d_n3;
        locals.var_argp__blk284_dn4 = assign4670_e4897_d_n4;
        locals.var_argp__blk284_dn5 = assign4670_e4897_d_n5;
        locals.var_argp__blk284_rv = 0.0;

        let (assign4680_e4907, assign4680_e4907_d_n1, assign4680_e4907_d_n3, assign4680_e4907_d_n4, assign4680_e4907_d_n5,) = {
    if (locals.var_guard280 != 0.0) {
        let assign4680_e4901: f64 = (locals.var_acja__blk281 * locals.var_arga__blk283);
        let assign4680_e4904: f64 = (locals.var_pcjp__blk282 * locals.var_argp__blk284);
        let assign4680_e4905: f64 = (assign4680_e4901 + assign4680_e4904);
        (assign4680_e4905, (((locals.var_acja__blk281_dn1 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn1)) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn1)), (((locals.var_acja__blk281_dn3 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn3)) + ((locals.var_pcjp__blk282_dn3 * locals.var_argp__blk284) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn3))), (((locals.var_acja__blk281_dn4 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn4)) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn4)), (((locals.var_acja__blk281_dn5 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn5)) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn5)),)
    } else {
        (locals.var_qcp2, locals.var_qcp2_dn1, locals.var_qcp2_dn3, locals.var_qcp2_dn4, locals.var_qcp2_dn5,)
    }
};
        locals.var_qcp2 = assign4680_e4907;
        locals.var_qcp2_dn1 = assign4680_e4907_d_n1;
        locals.var_qcp2_dn3 = assign4680_e4907_d_n3;
        locals.var_qcp2_dn4 = assign4680_e4907_d_n4;
        locals.var_qcp2_dn5 = assign4680_e4907_d_n5;
        locals.var_qcp2_rv = 0.0;

        let (assign4690_e4912, assign4690_e4912_d_n1, assign4690_e4912_d_n3, assign4690_e4912_d_n4, assign4690_e4912_d_n5,) = {
    if (locals.var_guard280 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qcp2, locals.var_qcp2_dn1, locals.var_qcp2_dn3, locals.var_qcp2_dn4, locals.var_qcp2_dn5,)
    }
};
        locals.var_qcp2 = assign4690_e4912;
        locals.var_qcp2_dn1 = assign4690_e4912_d_n1;
        locals.var_qcp2_dn3 = assign4690_e4912_d_n3;
        locals.var_qcp2_dn4 = assign4690_e4912_d_n4;
        locals.var_qcp2_dn5 = assign4690_e4912_d_n5;
        locals.var_qcp2_rv = 0.0;

        let assign4700_e4916: f64 = (locals.var_cf1 * locals.var_vc1);
        let assign4700_e4917: f64 = (locals.var_qcp1 + assign4700_e4916);
        locals.var_qcp1 = assign4700_e4917;
        locals.var_qcp1_dn1 = (locals.var_qcp1_dn1 + ((locals.var_cf1_dn1 * locals.var_vc1) + (locals.var_cf1 * locals.var_vc1_dn1)));
        locals.var_qcp1_dn3 = (locals.var_qcp1_dn3 + (locals.var_cf1_dn3 * locals.var_vc1));
        locals.var_qcp1_dn4 = (locals.var_qcp1_dn4 + ((locals.var_cf1_dn4 * locals.var_vc1) + (locals.var_cf1 * locals.var_vc1_dn4)));
        locals.var_qcp1_dn5 = (locals.var_qcp1_dn5 + (locals.var_cf1_dn5 * locals.var_vc1));
        locals.var_qcp1_rv = 0.0;

        let assign4710_e4921: f64 = (locals.var_cf2 * locals.var_vc2);
        let assign4710_e4922: f64 = (locals.var_qcp2 + assign4710_e4921);
        locals.var_qcp2 = assign4710_e4922;
        locals.var_qcp2_dn1 = (locals.var_qcp2_dn1 + ((locals.var_cf2_dn1 * locals.var_vc2) + (locals.var_cf2 * locals.var_vc2_dn1)));
        locals.var_qcp2_dn3 = (locals.var_qcp2_dn3 + (locals.var_cf2_dn3 * locals.var_vc2));
        locals.var_qcp2_dn4 = (locals.var_qcp2_dn4 + (locals.var_cf2_dn4 * locals.var_vc2));
        locals.var_qcp2_dn5 = (locals.var_qcp2_dn5 + ((locals.var_cf2_dn5 * locals.var_vc2) + (locals.var_cf2 * locals.var_vc2_dn5)));
        locals.var_qcp2_rv = 0.0;

        let assign4720_e4924: f64 = (-p.p21);
        let assign4720_e4926: f64 = (assign4720_e4924 * locals.var_qcp1);
        locals.var_qcp1 = assign4720_e4926;
        locals.var_qcp1_dn1 = (assign4720_e4924 * locals.var_qcp1_dn1);
        locals.var_qcp1_dn3 = (assign4720_e4924 * locals.var_qcp1_dn3);
        locals.var_qcp1_dn4 = (assign4720_e4924 * locals.var_qcp1_dn4);
        locals.var_qcp1_dn5 = (assign4720_e4924 * locals.var_qcp1_dn5);
        locals.var_qcp1_rv = 0.0;

        let assign4730_e4928: f64 = (-p.p21);
        let assign4730_e4930: f64 = (assign4730_e4928 * locals.var_qcp2);
        locals.var_qcp2 = assign4730_e4930;
        locals.var_qcp2_dn1 = (assign4730_e4928 * locals.var_qcp2_dn1);
        locals.var_qcp2_dn3 = (assign4730_e4928 * locals.var_qcp2_dn3);
        locals.var_qcp2_dn4 = (assign4730_e4928 * locals.var_qcp2_dn4);
        locals.var_qcp2_dn5 = (assign4730_e4928 * locals.var_qcp2_dn5);
        locals.var_qcp2_rv = 0.0;

        let assign4740_e4933: f64 = (locals.var_dt_et * locals.var_cth);
        locals.var_qcth = assign4740_e4933;
        locals.var_qcth_dn3 = (locals.var_dt_et_dn3 * locals.var_cth);
        locals.var_qcth_rv = 0.0;

        let (assign4770_e4949,) = {
    if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
        (locals.var_leff_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign4770_e4949;
        locals.var_len_rv = 0.0;

        let (assign4780_e4955,) = {
    if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
        (locals.var_weff_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign4780_e4955;
        locals.var_wid_rv = 0.0;

        let (assign4790_e4962,) = {
    if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
        (locals.var_l_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign4790_e4962;
        locals.var_len_rv = 0.0;

        let (assign4800_e4969,) = {
    if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
        (locals.var_w_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign4800_e4969;
        locals.var_wid_rv = 0.0;

        locals.var_cth = locals.var_cth;
        locals.var_cth_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq0_value: f64 = locals.var_irb;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq0_value),
            [1, 3, 4, 5],
            [multiplicity * (locals.var_irb_dn1), multiplicity * (locals.var_irb_dn3), multiplicity * (locals.var_irb_dn4), multiplicity * (locals.var_irb_dn5)],
            [],
            [],
            1.0,
        );
        let eq1_value: f64 = locals.var_ip1;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(4),
            multiplicity * (eq1_value),
            [1, 3, 4, 5],
            [multiplicity * (locals.var_ip1_dn1), multiplicity * (locals.var_ip1_dn3), multiplicity * (locals.var_ip1_dn4), multiplicity * (locals.var_ip1_dn5)],
            [],
            [],
            1.0,
        );
        let eq2_value: f64 = locals.var_ip2;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq2_value),
            [1, 3, 4, 5],
            [multiplicity * (locals.var_ip2_dn1), multiplicity * (locals.var_ip2_dn3), multiplicity * (locals.var_ip2_dn4), multiplicity * (locals.var_ip2_dn5)],
            [],
            [],
            1.0,
        );
        let eq3_value: f64 = locals.var_irth;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq3_value),
            3,
            multiplicity * (locals.var_irth_dn3),
        );
        let eq4_value: f64 = locals.var_ith;
        let eq4_node_derivatives: [f64; 6] = [locals.var_ith_dn0, locals.var_ith_dn1, locals.var_ith_dn2, locals.var_ith_dn3, locals.var_ith_dn4, locals.var_ith_dn5];
        let eq4_branch_derivatives: [f64; 2] = [locals.var_ith_db0, locals.var_ith_db1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq6_e162, eq6_e162_d_n0, eq6_e162_d_n3, eq6_e162_d_n4,) = {
    if (locals.var_guard311 == 0.0) {
        let eq6_e159: f64 = (locals.var_rc1_tnom * locals.var_tcrc);
        let eq6_e159_d_n3: f64 = (locals.var_rc1_tnom * locals.var_tcrc_dn3);
        let __rspice_inv_cse_0: f64 = 1.0 / eq6_e159;
        let eq6_e160: f64 = ((nv0 - nv4) * __rspice_inv_cse_0);
        let eq6_e160_d_n0: f64 = (1.0 * __rspice_inv_cse_0);
        let eq6_e160_d_n3: f64 = (-(((nv0 - nv4) * eq6_e159_d_n3) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n4: f64 = (-1.0 / eq6_e159);
        (eq6_e160, eq6_e160_d_n0, eq6_e160_d_n3, eq6_e160_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e162;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(4),
            multiplicity * (eq6_value),
            0,
            multiplicity * (eq6_e162_d_n0),
            3,
            multiplicity * (eq6_e162_d_n3),
            4,
            multiplicity * (eq6_e162_d_n4),
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq8_e179, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n5,) = {
    if (locals.var_guard312 == 0.0) {
        let eq8_e176: f64 = (locals.var_rc2_tnom * locals.var_tcrc);
        let eq8_e176_d_n3: f64 = (locals.var_rc2_tnom * locals.var_tcrc_dn3);
        let __rspice_inv_cse_0: f64 = 1.0 / eq8_e176;
        let eq8_e177: f64 = ((nv2 - nv5) * __rspice_inv_cse_0);
        let eq8_e177_d_n2: f64 = (1.0 * __rspice_inv_cse_0);
        let eq8_e177_d_n3: f64 = (-(((nv2 - nv5) * eq8_e176_d_n3) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n5: f64 = (-1.0 / eq8_e176);
        (eq8_e177, eq8_e177_d_n2, eq8_e177_d_n3, eq8_e177_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e179;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * (eq8_value),
            2,
            multiplicity * (eq8_e179_d_n2),
            3,
            multiplicity * (eq8_e179_d_n3),
            5,
            multiplicity * (eq8_e179_d_n5),
        );
        let eq9_e181: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qcp1);
        let eq9_value: f64 = eq9_e181;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(4),
            multiplicity * (eq9_value),
            [1, 3, 4, 5],
            [multiplicity * ((locals.var_qcp1_dn1 * ddt_scale)), multiplicity * ((locals.var_qcp1_dn3 * ddt_scale)), multiplicity * ((locals.var_qcp1_dn4 * ddt_scale)), multiplicity * ((locals.var_qcp1_dn5 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq10_e183: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qcp2);
        let eq10_value: f64 = eq10_e183;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq10_value),
            [1, 3, 4, 5],
            [multiplicity * ((locals.var_qcp2_dn1 * ddt_scale)), multiplicity * ((locals.var_qcp2_dn3 * ddt_scale)), multiplicity * ((locals.var_qcp2_dn4 * ddt_scale)), multiplicity * ((locals.var_qcp2_dn5 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq11_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qcth);
        let eq11_value: f64 = eq11_e185;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            3,
            multiplicity * ((locals.var_qcth_dn3 * ddt_scale)),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq9_e181_q: f64 = locals.var_qcp1;
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[4]),
            &[
                GeneratedDerivative::node(nodes[1], multiplicity * (locals.var_qcp1_dn1)),
                GeneratedDerivative::node(nodes[3], multiplicity * (locals.var_qcp1_dn3)),
                GeneratedDerivative::node(nodes[4], multiplicity * (locals.var_qcp1_dn4)),
                GeneratedDerivative::node(nodes[5], multiplicity * (locals.var_qcp1_dn5)),
            ],
        );
        let eq10_e183_q: f64 = locals.var_qcp2;
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[5]),
            &[
                GeneratedDerivative::node(nodes[1], multiplicity * (locals.var_qcp2_dn1)),
                GeneratedDerivative::node(nodes[3], multiplicity * (locals.var_qcp2_dn3)),
                GeneratedDerivative::node(nodes[4], multiplicity * (locals.var_qcp2_dn4)),
                GeneratedDerivative::node(nodes[5], multiplicity * (locals.var_qcp2_dn5)),
            ],
        );
        let eq11_e185_q: f64 = locals.var_qcth;
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (locals.var_qcth_dn3),
        );
    }
}
