#![allow(dead_code, non_snake_case, unused_imports, unused_parens, unused_variables)]

use rspice_veriloga_runtime::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState, boxed_zero_bool_array, boxed_zero_f64_array};
use rspice_veriloga_runtime::{GeneratedParameterBound as ParameterBound, GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG as PARAMETER_MAX_EXCLUSIVE_FLAG, GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG as PARAMETER_MIN_EXCLUSIVE_FLAG, validate_generated_finite_parameter as validate_finite_parameter, validate_generated_parameter as validate_parameter, validate_generated_parameter_bounds as validate_parameter_bounds};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 1401],
}

impl std::ops::Index<usize> for Parameters {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output { &self.values[index] }
}

impl std::ops::IndexMut<usize> for Parameters {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.values[index] }
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: every parameter slot is f64, so zero bytes are valid 0.0 values; numeric default chunks are copied into the values array.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 77] = [
                1e-5, 1e-5, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e-5, 1.0, 1.0, 0.0, 1e-5, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 4e-8, 2e-7, 3e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 77);
            {
                let params = &mut *ptr;
                params[77] = params[76];
                validate_parameter("TOXP", params[77], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 13] = [
                0.0, 1e24, 0.0, 1.0, 0.0, 2.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(78), 13);
            {
                let params = &mut *ptr;
                params[91] = params[79];
                validate_finite_parameter("NDEPCV", params[91]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[92] = params[80];
                validate_finite_parameter("NDEPCVL1", params[92]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[93] = params[81];
                validate_parameter("NDEPCVLEXP1", params[93], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[94] = params[82];
                validate_finite_parameter("NDEPCVL2", params[94]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[95] = params[83];
                validate_parameter("NDEPCVLEXP2", params[95], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[96] = params[84];
                validate_finite_parameter("NDEPCVW", params[96]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[97] = params[85];
                validate_parameter("NDEPCVWEXP", params[97], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[98] = params[86];
                validate_finite_parameter("NDEPCVWL", params[98]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[99] = params[87];
                validate_parameter("NDEPCVWLEXP", params[99], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[100] = params[88];
                validate_finite_parameter("LNDEPCV", params[100]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[101] = params[89];
                validate_finite_parameter("WNDEPCV", params[101]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[102] = params[90];
                validate_finite_parameter("PNDEPCV", params[102]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 26] = [
                5e25, 0.0, 0.0, 0.0, 1.1e16, 1.17, 11.9, 3.9,
                1.5e-7, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(103), 26);
            {
                let params = &mut *ptr;
                params[129] = params[115];
                validate_finite_parameter("VFBCV", params[129]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[130] = params[116];
                validate_finite_parameter("LVFBCV", params[130]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[131] = params[117];
                validate_finite_parameter("WVFBCV", params[131]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[132] = params[118];
                validate_finite_parameter("PVFBCV", params[132]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[133] = params[123];
                validate_finite_parameter("VFBCVL", params[133]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[134] = params[124];
                validate_parameter("VFBCVLEXP", params[134], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[135] = params[125];
                validate_finite_parameter("VFBCVW", params[135]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[136] = params[126];
                validate_parameter("VFBCVWEXP", params[136], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[137] = params[127];
                validate_finite_parameter("VFBCVWL", params[137]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[138] = params[128];
                validate_parameter("VFBCVWLEXP", params[138], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(139), 1);
            {
                let params = &mut *ptr;
                params[140] = params[115];
                validate_finite_parameter("VFBAGBCP2", params[140]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[141] = params[79];
                validate_parameter("NDEPAGBCP2", params[141], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 56] = [
                1e26, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.045, 0.0, 0.0, 0.0, 0.08, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(142), 56);
            {
                let params = &mut *ptr;
                params[198] = params[194];
                validate_finite_parameter("ETA0R", params[198]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[199] = params[195];
                validate_finite_parameter("LETA0R", params[199]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[200] = params[196];
                validate_finite_parameter("WETA0R", params[200]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[201] = params[197];
                validate_finite_parameter("PETA0R", params[201]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 82] = [
                1.0, -0.07, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.001, 0.54, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-9, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(202), 82);
            {
                let params = &mut *ptr;
                params[284] = params[258];
                validate_finite_parameter("CDSCDR", params[284]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[285] = params[259];
                validate_finite_parameter("LCDSCDR", params[285]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[286] = params[260];
                validate_finite_parameter("WCDSCDR", params[286]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[287] = params[261];
                validate_finite_parameter("PCDSCDR", params[287]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 17] = [
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 100000.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(288), 17);
            {
                let params = &mut *ptr;
                params[305] = params[295];
                validate_finite_parameter("VSATR", params[305]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[306] = params[296];
                validate_finite_parameter("LVSATR", params[306]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[307] = params[297];
                validate_finite_parameter("WVSATR", params[307]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[308] = params[298];
                validate_finite_parameter("PVSATR", params[308]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 6] = [
                0.125, 0.0, 0.0, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(309), 6);
            {
                let params = &mut *ptr;
                params[315] = params[295];
                validate_finite_parameter("VSATCV", params[315]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[316] = params[296];
                validate_finite_parameter("LVSATCV", params[316]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[317] = params[297];
                validate_finite_parameter("WVSATCV", params[317]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[318] = params[298];
                validate_finite_parameter("PVSATCV", params[318]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[319] = params[299];
                validate_finite_parameter("VSATCVL", params[319]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[320] = params[300];
                validate_parameter("VSATCVLEXP", params[320], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[321] = params[301];
                validate_finite_parameter("VSATCVW", params[321]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[322] = params[302];
                validate_parameter("VSATCVWEXP", params[322], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[323] = params[303];
                validate_finite_parameter("VSATCVWL", params[323]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[324] = params[304];
                validate_parameter("VSATCVWLEXP", params[324], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 18] = [
                0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-8, 0.0, 1e-8, 0.067, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(325), 18);
            {
                let params = &mut *ptr;
                params[343] = params[337];
                validate_finite_parameter("U0R", params[343]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[344] = params[340];
                validate_finite_parameter("LU0R", params[344]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[345] = params[341];
                validate_finite_parameter("WU0R", params[345]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[346] = params[342];
                validate_finite_parameter("PU0R", params[346]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 11] = [
                1.0, 0.001, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(347), 11);
            {
                let params = &mut *ptr;
                params[358] = params[348];
                validate_finite_parameter("UAR", params[358]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[359] = params[355];
                validate_finite_parameter("LUAR", params[359]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[360] = params[356];
                validate_finite_parameter("WUAR", params[360]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[361] = params[357];
                validate_finite_parameter("PUAR", params[361]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 16] = [
                1.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.001, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(362), 16);
            {
                let params = &mut *ptr;
                params[378] = params[372];
                validate_finite_parameter("UDR", params[378]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[379] = params[375];
                validate_finite_parameter("LUDR", params[379]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[380] = params[376];
                validate_finite_parameter("WUDR", params[380]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[381] = params[377];
                validate_finite_parameter("PUDR", params[381]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 4] = [
                2.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(382), 4);
            {
                let params = &mut *ptr;
                params[386] = params[382];
                validate_finite_parameter("UCSR", params[386]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[387] = params[383];
                validate_finite_parameter("LUCSR", params[387]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[388] = params[384];
                validate_finite_parameter("WUCSR", params[388]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[389] = params[385];
                validate_finite_parameter("PUCSR", params[389]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 10] = [
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(390), 10);
            {
                let params = &mut *ptr;
                params[400] = params[390];
                validate_finite_parameter("UCR", params[400]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[401] = params[397];
                validate_finite_parameter("LUCR", params[401]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[402] = params[398];
                validate_finite_parameter("WUCR", params[402]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[403] = params[399];
                validate_finite_parameter("PUCR", params[403]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 6] = [
                0.003, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(404), 6);
            {
                let params = &mut *ptr;
                params[410] = params[404];
                validate_finite_parameter("PCLMR", params[410]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[411] = params[407];
                validate_finite_parameter("LPCLMR", params[411]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[412] = params[408];
                validate_finite_parameter("WPCLMR", params[412]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[413] = params[409];
                validate_finite_parameter("PPCLMR", params[413]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(414), 1);
            {
                let params = &mut *ptr;
                params[415] = params[404];
                validate_finite_parameter("PCLMCV", params[415]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[416] = params[405];
                validate_finite_parameter("PCLMCVL", params[416]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[417] = params[406];
                validate_parameter("PCLMCVLEXP", params[417], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[418] = params[407];
                validate_finite_parameter("LPCLMCV", params[418]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[419] = params[408];
                validate_finite_parameter("WPCLMCV", params[419]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[420] = params[409];
                validate_finite_parameter("PPCLMCV", params[420]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 42] = [
                424000000.0, 0.0, 0.0, 0.0, 1e-8, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(421), 42);
            {
                let params = &mut *ptr;
                params[463] = params[453];
                validate_finite_parameter("RDWMIN", params[463]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[464] = params[454];
                validate_finite_parameter("LRDWMIN", params[464]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[465] = params[455];
                validate_finite_parameter("WRDWMIN", params[465]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[466] = params[456];
                validate_finite_parameter("PRDWMIN", params[466]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[467] = params[457];
                validate_finite_parameter("RDW", params[467]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[468] = params[458];
                validate_finite_parameter("LRDW", params[468]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[469] = params[459];
                validate_finite_parameter("WRDW", params[469]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[470] = params[460];
                validate_finite_parameter("PRDW", params[470]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[471] = params[461];
                validate_finite_parameter("RDWL", params[471]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[472] = params[462];
                validate_parameter("RDWLEXP", params[472], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 17] = [
                0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(473), 17);
            {
                let params = &mut *ptr;
                params[490] = params[483];
                validate_finite_parameter("PSATR", params[490]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[491] = params[484];
                validate_finite_parameter("LPSATR", params[491]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[492] = params[485];
                validate_finite_parameter("WPSATR", params[492]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[493] = params[486];
                validate_finite_parameter("PPSATR", params[493]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 12] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.05, 0.01, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(494), 12);
            {
                let params = &mut *ptr;
                params[506] = params[498];
                validate_finite_parameter("PTWGR", params[506]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[507] = params[499];
                validate_finite_parameter("LPTWGR", params[507]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[508] = params[500];
                validate_finite_parameter("WPTWGR", params[508]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[509] = params[501];
                validate_finite_parameter("PPTWGR", params[509]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 26] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(510), 26);
            {
                let params = &mut *ptr;
                params[536] = params[530];
                validate_finite_parameter("PDIBLCR", params[536]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[537] = params[533];
                validate_finite_parameter("LPDIBLCR", params[537]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[538] = params[534];
                validate_finite_parameter("WPDIBLCR", params[538]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[539] = params[535];
                validate_finite_parameter("PPDIBLCR", params[539]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 10.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(540), 24);
            {
                let params = &mut *ptr;
                params[564] = params[563];
                validate_finite_parameter("AHLID", params[564]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(565), 1);
            {
                let params = &mut *ptr;
                params[566] = params[565];
                validate_finite_parameter("LAHLID", params[566]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(567), 1);
            {
                let params = &mut *ptr;
                params[568] = params[567];
                validate_finite_parameter("WAHLID", params[568]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(569), 1);
            {
                let params = &mut *ptr;
                params[570] = params[569];
                validate_finite_parameter("PAHLID", params[570]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 12] = [
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(571), 12);
            {
                let params = &mut *ptr;
                params[583] = params[579];
                validate_finite_parameter("IDBJT", params[583]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[584] = params[582];
                validate_finite_parameter("LIDBJT", params[584]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[585] = params[581];
                validate_finite_parameter("WIDBJT", params[585]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[586] = params[580];
                validate_finite_parameter("PIDBJT", params[586]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 120] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-7,
                2e-6, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 10000000.0, 0.0, 0.0, 0.0,
                0.1, 0.1, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.4, 0.0, 0.0, 0.0, 0.026, 0.35, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0,
                0.0, 0.43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.05, 0.0, 0.0, 0.0, 17.0, 300.0, 3.7622e-7,
                -31051000000.0, 4.9758e-7, -23570000000.0, 3.4254e-7, 4.9723e-7, 1166500000000.0, 745670000000.0, 1.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(587), 120);
            {
                let params = &mut *ptr;
                params[707] = if (params[30] == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGC", params[707]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[708] = if (params[30] == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGC", params[708]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[709] = if (params[30] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params[709]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[710] = if (params[30] == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGS", params[710]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(711), 1);
            {
                let params = &mut *ptr;
                params[712] = if (params[30] == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGS", params[712]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[713] = if (params[30] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGS", params[713]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[714] = if (params[30] == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGD", params[714]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(715), 1);
            {
                let params = &mut *ptr;
                params[716] = if (params[30] == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGD", params[716]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[717] = if (params[30] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGD", params[717]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[718] = params[54];
                validate_finite_parameter("DLCIG", params[718]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[719] = params[718];
                validate_finite_parameter("DLCIGD", params[719]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 101] = [
                1.0, 1.0, 3e-9, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.043,
                0.0, 0.0054, 0.0075, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 2300000000.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0,
                0.0, 0.8, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(720), 101);
            {
                let params = &mut *ptr;
                params[821] = params[799];
                validate_finite_parameter("AGISL", params[821]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[822] = params[800];
                validate_finite_parameter("AGISLL", params[822]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[823] = params[801];
                validate_finite_parameter("AGISLW", params[823]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[824] = params[802];
                validate_finite_parameter("LAGISL", params[824]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[825] = params[803];
                validate_finite_parameter("WAGISL", params[825]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[826] = params[804];
                validate_finite_parameter("PAGISL", params[826]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[827] = params[805];
                validate_finite_parameter("BGISL", params[827]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[828] = params[806];
                validate_finite_parameter("BGISL1", params[828]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[829] = params[807];
                validate_finite_parameter("LBGISL", params[829]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[830] = params[808];
                validate_finite_parameter("WBGISL", params[830]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[831] = params[809];
                validate_finite_parameter("PBGISL", params[831]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[832] = params[810];
                validate_finite_parameter("LBGISL1", params[832]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[833] = params[811];
                validate_finite_parameter("WBGISL1", params[833]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[834] = params[812];
                validate_finite_parameter("PBGISL1", params[834]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[835] = params[813];
                validate_finite_parameter("CGISL", params[835]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[836] = params[814];
                validate_finite_parameter("LCGISL", params[836]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[837] = params[815];
                validate_finite_parameter("WCGISL", params[837]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[838] = params[816];
                validate_finite_parameter("PCGISL", params[838]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[839] = params[817];
                validate_finite_parameter("EGISL", params[839]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[840] = params[818];
                validate_finite_parameter("LEGISL", params[840]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[841] = params[819];
                validate_finite_parameter("WEGISL", params[841]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[842] = params[820];
                validate_finite_parameter("PEGISL", params[842]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 12] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (*ptr).values.as_mut_ptr().add(843), 12);
            {
                let params = &mut *ptr;
                params[855] = params[843];
                validate_finite_parameter("RGISL", params[855]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[856] = params[844];
                validate_finite_parameter("LRGISL", params[856]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[857] = params[845];
                validate_finite_parameter("WRGISL", params[857]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[858] = params[846];
                validate_finite_parameter("PRGISL", params[858]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[859] = params[847];
                validate_finite_parameter("KGISL", params[859]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[860] = params[848];
                validate_finite_parameter("LKGISL", params[860]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[861] = params[849];
                validate_finite_parameter("WKGISL", params[861]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[862] = params[850];
                validate_finite_parameter("PKGISL", params[862]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[863] = params[851];
                validate_finite_parameter("FGISL", params[863]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[864] = params[852];
                validate_finite_parameter("LFGISL", params[864]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[865] = params[853];
                validate_finite_parameter("WFGISL", params[865]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[866] = params[854];
                validate_finite_parameter("PFGISL", params[866]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 29] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.6, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0, 0.0,
                1000000.0, 1.0, 1000000.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (*ptr).values.as_mut_ptr().add(867), 29);
            {
                let params = &mut *ptr;
                params[896] = params[895];
                validate_parameter("DMCI", params[896], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 5] = [
                0.0, 0.0, 0.0, 0.1, 0.0005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (*ptr).values.as_mut_ptr().add(897), 5);
            {
                let params = &mut *ptr;
                params[902] = params[901];
                validate_finite_parameter("CJD", params[902]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                5e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (*ptr).values.as_mut_ptr().add(903), 1);
            {
                let params = &mut *ptr;
                params[904] = params[903];
                validate_finite_parameter("CJSWD", params[904]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (*ptr).values.as_mut_ptr().add(905), 1);
            {
                let params = &mut *ptr;
                params[906] = params[905];
                validate_finite_parameter("CJSWGD", params[906]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (*ptr).values.as_mut_ptr().add(907), 1);
            {
                let params = &mut *ptr;
                params[908] = params[907];
                validate_finite_parameter("PBD", params[908]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (*ptr).values.as_mut_ptr().add(909), 1);
            {
                let params = &mut *ptr;
                params[910] = params[909];
                validate_finite_parameter("PBSWD", params[910]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[911] = params[909];
                validate_finite_parameter("PBSWGS", params[911]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[912] = params[911];
                validate_finite_parameter("PBSWGD", params[912]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (*ptr).values.as_mut_ptr().add(913), 1);
            {
                let params = &mut *ptr;
                params[914] = params[913];
                validate_finite_parameter("MJD", params[914]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 1] = [
                0.33,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (*ptr).values.as_mut_ptr().add(915), 1);
            {
                let params = &mut *ptr;
                params[916] = params[915];
                validate_finite_parameter("MJSWD", params[916]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[917] = params[915];
                validate_finite_parameter("MJSWGS", params[917]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[918] = params[917];
                validate_finite_parameter("MJSWGD", params[918]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 8] = [
                1e-12, 1.0, -1.0, 0.0, 0.0, 0.0, 0.026, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (*ptr).values.as_mut_ptr().add(919), 8);
            {
                let params = &mut *ptr;
                params[927] = params[70];
                validate_finite_parameter("DWJ", params[927]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[928] = params[571];
                validate_finite_parameter("XDIF", params[928]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 4] = [
                0.0, 0.0, 0.0, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (*ptr).values.as_mut_ptr().add(929), 4);
            {
                let params = &mut *ptr;
                params[933] = params[932];
                validate_finite_parameter("IDDIF", params[933]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (*ptr).values.as_mut_ptr().add(934), 1);
            {
                let params = &mut *ptr;
                params[935] = params[934];
                validate_finite_parameter("LIDDIF", params[935]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (*ptr).values.as_mut_ptr().add(936), 1);
            {
                let params = &mut *ptr;
                params[937] = params[936];
                validate_finite_parameter("WIDDIF", params[937]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (*ptr).values.as_mut_ptr().add(938), 1);
            {
                let params = &mut *ptr;
                params[939] = params[938];
                validate_finite_parameter("PIDDIF", params[939]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 13] = [
                2.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 1e-5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (*ptr).values.as_mut_ptr().add(940), 13);
            {
                let params = &mut *ptr;
                params[953] = params[952];
                validate_finite_parameter("IDREC", params[953]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (*ptr).values.as_mut_ptr().add(954), 1);
            {
                let params = &mut *ptr;
                params[955] = params[954];
                validate_finite_parameter("LIDREC", params[955]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (*ptr).values.as_mut_ptr().add(956), 1);
            {
                let params = &mut *ptr;
                params[957] = params[956];
                validate_finite_parameter("WIDREC", params[957]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (*ptr).values.as_mut_ptr().add(958), 1);
            {
                let params = &mut *ptr;
                params[959] = params[958];
                validate_finite_parameter("PIDREC", params[959]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 9] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (*ptr).values.as_mut_ptr().add(960), 9);
            {
                let params = &mut *ptr;
                params[969] = params[968];
                validate_finite_parameter("IDTUN", params[969]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (*ptr).values.as_mut_ptr().add(970), 1);
            {
                let params = &mut *ptr;
                params[971] = params[970];
                validate_finite_parameter("LIDTUN", params[971]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (*ptr).values.as_mut_ptr().add(972), 1);
            {
                let params = &mut *ptr;
                params[973] = params[972];
                validate_finite_parameter("WIDTUN", params[973]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (*ptr).values.as_mut_ptr().add(974), 1);
            {
                let params = &mut *ptr;
                params[975] = params[974];
                validate_finite_parameter("PIDTUN", params[975]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (*ptr).values.as_mut_ptr().add(976), 1);
            {
                let params = &mut *ptr;
                params[977] = params[976];
                validate_finite_parameter("XTUND", params[977]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (*ptr).values.as_mut_ptr().add(978), 1);
            {
                let params = &mut *ptr;
                params[979] = params[978];
                validate_finite_parameter("LXTUND", params[979]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (*ptr).values.as_mut_ptr().add(980), 1);
            {
                let params = &mut *ptr;
                params[981] = params[980];
                validate_finite_parameter("WXTUND", params[981]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (*ptr).values.as_mut_ptr().add(982), 1);
            {
                let params = &mut *ptr;
                params[983] = params[982];
                validate_finite_parameter("PXTUND", params[983]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (*ptr).values.as_mut_ptr().add(984), 1);
            {
                let params = &mut *ptr;
                params[985] = params[984];
                validate_finite_parameter("NTUND", params[985]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (*ptr).values.as_mut_ptr().add(986), 1);
            {
                let params = &mut *ptr;
                params[987] = params[986];
                validate_finite_parameter("LNTUND", params[987]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (*ptr).values.as_mut_ptr().add(988), 1);
            {
                let params = &mut *ptr;
                params[989] = params[988];
                validate_finite_parameter("WNTUND", params[989]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (*ptr).values.as_mut_ptr().add(990), 1);
            {
                let params = &mut *ptr;
                params[991] = params[990];
                validate_finite_parameter("PNTUND", params[991]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (*ptr).values.as_mut_ptr().add(992), 1);
            {
                let params = &mut *ptr;
                params[993] = params[992];
                validate_finite_parameter("VTUN0D", params[993]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (*ptr).values.as_mut_ptr().add(994), 1);
            {
                let params = &mut *ptr;
                params[995] = params[994];
                validate_finite_parameter("LVTUN0D", params[995]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (*ptr).values.as_mut_ptr().add(996), 1);
            {
                let params = &mut *ptr;
                params[997] = params[996];
                validate_finite_parameter("WVTUN0D", params[997]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (*ptr).values.as_mut_ptr().add(998), 1);
            {
                let params = &mut *ptr;
                params[999] = params[998];
                validate_finite_parameter("PVTUN0D", params[999]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (*ptr).values.as_mut_ptr().add(1000), 1);
            {
                let params = &mut *ptr;
                params[1001] = params[1000];
                validate_finite_parameter("VREC0D", params[1001]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (*ptr).values.as_mut_ptr().add(1002), 1);
            {
                let params = &mut *ptr;
                params[1003] = params[1002];
                validate_finite_parameter("LVREC0D", params[1003]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (*ptr).values.as_mut_ptr().add(1004), 1);
            {
                let params = &mut *ptr;
                params[1005] = params[1004];
                validate_finite_parameter("WVREC0D", params[1005]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (*ptr).values.as_mut_ptr().add(1006), 1);
            {
                let params = &mut *ptr;
                params[1007] = params[1006];
                validate_finite_parameter("PVREC0D", params[1007]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 313] = [
                12.0, 1.0, 1.0, 41000000.0, 6.25e40, 3.125e25, 875000000.0, 0.0,
                0.0, 1.0, 1.0, 0.577, 0.5164, 0.395, 1.5, 3.5,
                0.0, 1.0, 0.0, 0.0, 27.0, 0.000473, 636.0, 0.0,
                -1.5, 0.0, 0.0, 0.0, 0.0, 0.001, 0.0, 0.0,
                0.0, 0.0, 5.6e-11, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.004775,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                -0.00156, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -0.11, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.022, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-5, 0.0, 1e-6, 1e-6,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-6, 400.0, 336000000.0, 0.185, 0.3, 1.4, 0.0,
                0.49, 1.42, 20.0, 1e-8, 0.0, 0.0, 1.0, 0.0,
                1e24, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-9, 0.0, 0.0, 0.0,
                1e-9, 0.0, 0.0, 0.0, 1e-9, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.08, 0.0,
                0.0, 0.0, -0.07, 0.0, 0.0, 0.0, -0.11, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.022, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.2, 0.53,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                1e-5, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (*ptr).values.as_mut_ptr().add(1008), 313);
            {
                let params = &mut *ptr;
                params[1321] = params[1012];
                validate_finite_parameter("NOIA2", params[1321]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1322] = params[79];
                validate_parameter("HNDEP", params[1322], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (*ptr).values.as_mut_ptr().add(1323), 24);
            {
                let params = &mut *ptr;
                params[1347] = 0.001;
                validate_parameter("minr", params[1347], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 5] = [
                1.0, 0.0, 0.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (*ptr).values.as_mut_ptr().add(1348), 5);
            {
                let params = &mut *ptr;
                params[1353] = params[1349];
                validate_finite_parameter("A0CV", params[1353]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1354] = params[1350];
                validate_finite_parameter("AGSCV", params[1354]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1355] = params[1352];
                validate_parameter("KETACV", params[1355], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 25] = [
                0.0, 1.0, 0.0, 1.0, 1000000000000000.0, 0.067, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 5e24, 0.0, 0.0,
                0.0, 1.0, 0.001, 0.0, 0.0, 0.0, 0.0, 0.0,
                2e-12,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (*ptr).values.as_mut_ptr().add(1356), 25);
            {
                let params = &mut *ptr;
                params[1381] = params[1379];
                validate_finite_parameter("AGBCPD", params[1381]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 17] = [
                0.0, 1.12, 6e22, 0.0, 0.0, 0.0, 1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (*ptr).values.as_mut_ptr().add(1382), 17);
            {
                let params = &mut *ptr;
                params[1399] = params[1397];
                validate_parameter("ACEDB", params[1399], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1400] = params[1398];
                validate_finite_parameter("BCEDB", params[1400]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            let params = &*ptr;
            for index in 0..PARAMETER_DISPLAY_NAMES.len() {
                let value = read_parameter_slot(params, index);
                validate_parameter_metadata(params, index, value).expect("generated Verilog-A parameter defaults must satisfy declared ranges");
            }
            boxed.assume_init()
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        *Self::new_box()
    }
}

#[inline]
fn read_parameter_slot(parameters: &Parameters, index: usize) -> f64 {
    debug_assert!(index < PARAMETER_DISPLAY_NAMES.len(), "generated parameter index out of range");
    parameters.values[index]
}

fn validate_parameter_scalar_metadata(index: usize, value: f64) -> Result<(), String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter index {} is out of range", index));
    };
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    validate_parameter_bounds(
        name,
        value,
        flags,
        PARAMETER_MIN_BOUNDS[index],
        PARAMETER_MAX_BOUNDS[index],
        PARAMETER_EXCLUDED_BOUNDS[index],
    )
}

fn validate_parameter_metadata(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    validate_parameter_scalar_metadata(index, value)?;
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    let computed_min = parameter_computed_min_bound(parameters, index)?;
    let lower_source_count = usize::from(PARAMETER_MIN_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MIN_REFERENCES[index].is_some())
        + usize::from(computed_min.is_some());
    if lower_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting lower-bound sources", name));
    }
    let min = match PARAMETER_MIN_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_min.or(PARAMETER_MIN_BOUNDS[index]),
    };
    let computed_max = parameter_computed_max_bound(parameters, index)?;
    let upper_source_count = usize::from(PARAMETER_MAX_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MAX_REFERENCES[index].is_some())
        + usize::from(computed_max.is_some());
    if upper_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting upper-bound sources", name));
    }
    let max = match PARAMETER_MAX_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_max.or(PARAMETER_MAX_BOUNDS[index]),
    };
    if let (Some(min), Some(max)) = (min, max) {
        let empty = min.value > max.value
            || (min.value == max.value
                && flags & (PARAMETER_MIN_EXCLUSIVE_FLAG | PARAMETER_MAX_EXCLUSIVE_FLAG) != 0);
        if empty {
            return Err(format!(
                "parameter '{}' has an empty range: lower bound {}={} exceeds upper bound {}={}",
                name, min.label, min.value, max.label, max.value
            ));
        }
    }
    validate_parameter_bounds(name, value, flags, min, max, PARAMETER_EXCLUDED_BOUNDS[index])?;
    for &reference in PARAMETER_EXCLUDED_REFERENCES[index] {
        let excluded = parameter_bound_from_reference(parameters, reference)?;
        if value == excluded.value {
            return Err(format!(
                "parameter '{}' must not equal {}={}, got {}",
                name, excluded.label, excluded.value, value
            ));
        }
    }
    validate_parameter_computed_exclusions(parameters, index, value)?;
    Ok(())
}

fn parameter_bound_from_reference(
    parameters: &Parameters,
    index: usize,
) -> Result<ParameterBound, String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter range reference {} is out of range", index));
    };
    let value = read_parameter_slot(parameters, index);
    validate_finite_parameter(name, value)?;
    Ok(ParameterBound { value, label: name })
}


const PARAMETER_NAME_LOOKUP: [(&str, usize); 1401] = [
    ("l", 0), ("w", 1), ("nf", 2), ("nrs", 3), ("nrd", 4), ("vfbsdoff", 5), ("minz", 6), ("rgatemod", 7), ("geomod", 8), ("rgeomod", 9), ("sa", 10), ("sb", 11), ("sd", 12), ("sca", 13), ("scb", 14), ("scc", 15),
    ("sc", 16), ("as", 17), ("ad", 18), ("ps", 19), ("pd", 20), ("xgw", 21), ("ngcon", 22), ("dtemp", 23), ("mulu0", 24), ("delvto", 25), ("ids0mult", 26), ("edgefet", 27), ("sslmod", 28), ("soimod", 29), ("type", 30), ("cvmod", 31),
    ("covmod", 32), ("rdsmod", 33), ("wpemod", 34), ("asymmod", 35), ("gidlmod", 36), ("igcmod", 37), ("igbmod", 38), ("tnoimod", 39), ("tnodeout", 40), ("shmod", 41), ("mobscale", 42), ("bodymod", 43), ("iiimod", 44), ("modagbcp2", 45), ("pdemod", 46), ("fbody1", 47),
    ("llong", 48), ("lmlt", 49), ("wmlt", 50), ("xl", 51), ("wwide", 52), ("xw", 53), ("lint", 54), ("ll", 55), ("lw", 56), ("lwl", 57), ("lln", 58), ("lwn", 59), ("wint", 60), ("wl", 61), ("ww", 62), ("wwl", 63),
    ("wln", 64), ("wwn", 65), ("dlc", 66), ("llc", 67), ("lwc", 68), ("lwlc", 69), ("dwc", 70), ("wlc", 71), ("wwc", 72), ("wwlc", 73), ("tsi", 74), ("tbox", 75), ("toxe", 76), ("toxp", 77), ("dtox", 78), ("ndep", 79),
    ("ndepl1", 80), ("ndeplexp1", 81), ("ndepl2", 82), ("ndeplexp2", 83), ("ndepw", 84), ("ndepwexp", 85), ("ndepwl", 86), ("ndepwlexp", 87), ("lndep", 88), ("wndep", 89), ("pndep", 90), ("ndepcv", 91), ("ndepcvl1", 92), ("ndepcvlexp1", 93), ("ndepcvl2", 94), ("ndepcvlexp2", 95),
    ("ndepcvw", 96), ("ndepcvwexp", 97), ("ndepcvwl", 98), ("ndepcvwlexp", 99), ("lndepcv", 100), ("wndepcv", 101), ("pndepcv", 102), ("ngate", 103), ("lngate", 104), ("wngate", 105), ("pngate", 106), ("ni0sub", 107), ("bg0sub", 108), ("epsrsub", 109), ("epsrox", 110), ("xj", 111),
    ("lxj", 112), ("wxj", 113), ("pxj", 114), ("vfb", 115), ("lvfb", 116), ("wvfb", 117), ("pvfb", 118), ("vfbb", 119), ("lvfbb", 120), ("wvfbb", 121), ("pvfbb", 122), ("vfbl", 123), ("vfblexp", 124), ("vfbw", 125), ("vfbwexp", 126), ("vfbwl", 127),
    ("vfbwlexp", 128), ("vfbcv", 129), ("lvfbcv", 130), ("wvfbcv", 131), ("pvfbcv", 132), ("vfbcvl", 133), ("vfbcvlexp", 134), ("vfbcvw", 135), ("vfbcvwexp", 136), ("vfbcvwl", 137), ("vfbcvwlexp", 138), ("delvfbacc", 139), ("vfbagbcp2", 140), ("ndepagbcp2", 141), ("nsd", 142), ("lnsd", 143),
    ("wnsd", 144), ("pnsd", 145), ("dvtp0", 146), ("ldvtp0", 147), ("wdvtp0", 148), ("pdvtp0", 149), ("dvtp1", 150), ("ldvtp1", 151), ("wdvtp1", 152), ("pdvtp1", 153), ("dvtp2", 154), ("ldvtp2", 155), ("wdvtp2", 156), ("pdvtp2", 157), ("dvtp3", 158), ("ldvtp3", 159),
    ("wdvtp3", 160), ("pdvtp3", 161), ("dvtp4", 162), ("ldvtp4", 163), ("wdvtp4", 164), ("pdvtp4", 165), ("dvtp5", 166), ("ldvtp5", 167), ("wdvtp5", 168), ("pdvtp5", 169), ("dvbd0", 170), ("ldvbd0", 171), ("wdvbd0", 172), ("pdvbd0", 173), ("dvbd1", 174), ("ldvbd1", 175),
    ("wdvbd1", 176), ("pdvbd1", 177), ("vsce", 178), ("lvsce", 179), ("wvsce", 180), ("pvsce", 181), ("cdsbs1", 182), ("lcdsbs1", 183), ("wcdsbs1", 184), ("pcdsbs1", 185), ("cdsbs", 186), ("lcdsbs", 187), ("wcdsbs", 188), ("pcdsbs", 189), ("phin", 190), ("lphin", 191),
    ("wphin", 192), ("pphin", 193), ("eta0", 194), ("leta0", 195), ("weta0", 196), ("peta0", 197), ("eta0r", 198), ("leta0r", 199), ("weta0r", 200), ("peta0r", 201), ("dsub", 202), ("etab", 203), ("etabexp", 204), ("letab", 205), ("wetab", 206), ("petab", 207),
    ("k1", 208), ("k1l", 209), ("k1lexp", 210), ("k1w", 211), ("k1wexp", 212), ("k1wl", 213), ("k1wlexp", 214), ("lk1", 215), ("wk1", 216), ("pk1", 217), ("k2", 218), ("k2l", 219), ("k2lexp", 220), ("k2w", 221), ("k2wexp", 222), ("k2wl", 223),
    ("k2wlexp", 224), ("lk2", 225), ("wk2", 226), ("pk2", 227), ("ados", 228), ("bdos", 229), ("qm0", 230), ("etaqm", 231), ("cit", 232), ("lcit", 233), ("wcit", 234), ("pcit", 235), ("nfactor", 236), ("nfactorl", 237), ("nfactorlexp", 238), ("nfactorw", 239),
    ("nfactorwexp", 240), ("nfactorwl", 241), ("nfactorwlexp", 242), ("lnfactor", 243), ("wnfactor", 244), ("pnfactor", 245), ("ascl", 246), ("lascl", 247), ("wascl", 248), ("pascl", 249), ("bscl", 250), ("lbscl", 251), ("wbscl", 252), ("pbscl", 253), ("dvt1", 254), ("ldvt1", 255),
    ("wdvt1", 256), ("pdvt1", 257), ("cdscd", 258), ("lcdscd", 259), ("wcdscd", 260), ("pcdscd", 261), ("cdsc", 262), ("lcdsc", 263), ("wcdsc", 264), ("pcdsc", 265), ("csecsed", 266), ("cbcbd", 267), ("csecse0", 268), ("csecse0p", 269), ("csecse", 270), ("lcsecse", 271),
    ("wcsecse", 272), ("pcsecse", 273), ("csecsep", 274), ("cbcb", 275), ("lcbcb", 276), ("wcbcb", 277), ("pcbcb", 278), ("cbcbp", 279), ("cbcb0", 280), ("cbcb0p", 281), ("cdscdl", 282), ("cdscdlexp", 283), ("cdscdr", 284), ("lcdscdr", 285), ("wcdscdr", 286), ("pcdscdr", 287),
    ("cdscb", 288), ("cdscbl", 289), ("cdscblexp", 290), ("lcdscb", 291), ("wcdscb", 292), ("pcdscb", 293), ("vbsa", 294), ("vsat", 295), ("lvsat", 296), ("wvsat", 297), ("pvsat", 298), ("vsatl", 299), ("vsatlexp", 300), ("vsatw", 301), ("vsatwexp", 302), ("vsatwl", 303),
    ("vsatwlexp", 304), ("vsatr", 305), ("lvsatr", 306), ("wvsatr", 307), ("pvsatr", 308), ("delta", 309), ("ldelta", 310), ("wdelta", 311), ("pdelta", 312), ("deltal", 313), ("deltalexp", 314), ("vsatcv", 315), ("lvsatcv", 316), ("wvsatcv", 317), ("pvsatcv", 318), ("vsatcvl", 319),
    ("vsatcvlexp", 320), ("vsatcvw", 321), ("vsatcvwexp", 322), ("vsatcvwl", 323), ("vsatcvwlexp", 324), ("thesat", 325), ("lthesat", 326), ("wthesat", 327), ("pthesat", 328), ("lpe1", 329), ("llpe1", 330), ("wlpe1", 331), ("plpe1", 332), ("up1", 333), ("lp1", 334), ("up2", 335),
    ("lp2", 336), ("u0", 337), ("u0l", 338), ("u0lexp", 339), ("lu0", 340), ("wu0", 341), ("pu0", 342), ("u0r", 343), ("lu0r", 344), ("wu0r", 345), ("pu0r", 346), ("etamob", 347), ("ua", 348), ("ual", 349), ("ualexp", 350), ("uaw", 351),
    ("uawexp", 352), ("uawl", 353), ("uawlexp", 354), ("lua", 355), ("wua", 356), ("pua", 357), ("uar", 358), ("luar", 359), ("wuar", 360), ("puar", 361), ("eu", 362), ("leu", 363), ("weu", 364), ("peu", 365), ("eul", 366), ("eulexp", 367),
    ("euw", 368), ("euwexp", 369), ("euwl", 370), ("euwlexp", 371), ("ud", 372), ("udl", 373), ("udlexp", 374), ("lud", 375), ("wud", 376), ("pud", 377), ("udr", 378), ("ludr", 379), ("wudr", 380), ("pudr", 381), ("ucs", 382), ("lucs", 383),
    ("wucs", 384), ("pucs", 385), ("ucsr", 386), ("lucsr", 387), ("wucsr", 388), ("pucsr", 389), ("uc", 390), ("ucl", 391), ("uclexp", 392), ("ucw", 393), ("ucwexp", 394), ("ucwl", 395), ("ucwlexp", 396), ("luc", 397), ("wuc", 398), ("puc", 399),
    ("ucr", 400), ("lucr", 401), ("wucr", 402), ("pucr", 403), ("pclm", 404), ("pclml", 405), ("pclmlexp", 406), ("lpclm", 407), ("wpclm", 408), ("ppclm", 409), ("pclmr", 410), ("lpclmr", 411), ("wpclmr", 412), ("ppclmr", 413), ("pclmg", 414), ("pclmcv", 415),
    ("pclmcvl", 416), ("pclmcvlexp", 417), ("lpclmcv", 418), ("wpclmcv", 419), ("ppclmcv", 420), ("pscbe1", 421), ("lpscbe1", 422), ("wpscbe1", 423), ("ppscbe1", 424), ("pscbe2", 425), ("lpscbe2", 426), ("wpscbe2", 427), ("ppscbe2", 428), ("pdits", 429), ("lpdits", 430), ("wpdits", 431),
    ("ppdits", 432), ("pditsl", 433), ("pditsd", 434), ("lpditsd", 435), ("wpditsd", 436), ("ppditsd", 437), ("rsh", 438), ("prwg", 439), ("lprwg", 440), ("wprwg", 441), ("pprwg", 442), ("prwb", 443), ("lprwb", 444), ("wprwb", 445), ("pprwb", 446), ("prwbl", 447),
    ("prwblexp", 448), ("wr", 449), ("lwr", 450), ("wwr", 451), ("pwr", 452), ("rswmin", 453), ("lrswmin", 454), ("wrswmin", 455), ("prswmin", 456), ("rsw", 457), ("lrsw", 458), ("wrsw", 459), ("prsw", 460), ("rswl", 461), ("rswlexp", 462), ("rdwmin", 463),
    ("lrdwmin", 464), ("wrdwmin", 465), ("prdwmin", 466), ("rdw", 467), ("lrdw", 468), ("wrdw", 469), ("prdw", 470), ("rdwl", 471), ("rdwlexp", 472), ("rdswmin", 473), ("lrdswmin", 474), ("wrdswmin", 475), ("prdswmin", 476), ("rdsw", 477), ("rdswl", 478), ("rdswlexp", 479),
    ("lrdsw", 480), ("wrdsw", 481), ("prdsw", 482), ("psat", 483), ("lpsat", 484), ("wpsat", 485), ("ppsat", 486), ("psatl", 487), ("psatlexp", 488), ("psatb", 489), ("psatr", 490), ("lpsatr", 491), ("wpsatr", 492), ("ppsatr", 493), ("lpsatb", 494), ("wpsatb", 495),
    ("ppsatb", 496), ("psatx", 497), ("ptwg", 498), ("lptwg", 499), ("wptwg", 500), ("pptwg", 501), ("ptwgl", 502), ("vp", 503), ("alp", 504), ("ptwglexp", 505), ("ptwgr", 506), ("lptwgr", 507), ("wptwgr", 508), ("pptwgr", 509), ("ksativ", 510), ("lksativ", 511),
    ("wksativ", 512), ("pksativ", 513), ("a1", 514), ("la1", 515), ("wa1", 516), ("pa1", 517), ("a11", 518), ("la11", 519), ("wa11", 520), ("pa11", 521), ("a2", 522), ("la2", 523), ("wa2", 524), ("pa2", 525), ("a21", 526), ("la21", 527),
    ("wa21", 528), ("pa21", 529), ("pdiblc", 530), ("pdiblcl", 531), ("pdiblclexp", 532), ("lpdiblc", 533), ("wpdiblc", 534), ("ppdiblc", 535), ("pdiblcr", 536), ("lpdiblcr", 537), ("wpdiblcr", 538), ("ppdiblcr", 539), ("pdiblcb", 540), ("lpdiblcb", 541), ("wpdiblcb", 542), ("ppdiblcb", 543),
    ("pvag", 544), ("lpvag", 545), ("wpvag", 546), ("ppvag", 547), ("fprout", 548), ("fproutl", 549), ("fproutlexp", 550), ("lfprout", 551), ("wfprout", 552), ("pfprout", 553), ("bjtoff", 554), ("vabjt", 555), ("lvabjt", 556), ("wvabjt", 557), ("pvabjt", 558), ("aely", 559),
    ("laely", 560), ("waely", 561), ("paely", 562), ("ahli", 563), ("ahlid", 564), ("lahli", 565), ("lahlid", 566), ("wahli", 567), ("wahlid", 568), ("pahli", 569), ("pahlid", 570), ("xbjt", 571), ("lxbjt", 572), ("wxbjt", 573), ("pxbjt", 574), ("ndiode", 575),
    ("lndiode", 576), ("wndiode", 577), ("pndiode", 578), ("isbjt", 579), ("pisbjt", 580), ("wisbjt", 581), ("lisbjt", 582), ("idbjt", 583), ("lidbjt", 584), ("widbjt", 585), ("pidbjt", 586), ("nbjt", 587), ("lnbjt", 588), ("llbjt0", 589), ("wnbjt", 590), ("wlbjt0", 591),
    ("pnbjt", 592), ("plbjt0", 593), ("lbjt0", 594), ("ln", 595), ("vdsatii0", 596), ("lvdsatii0", 597), ("wvdsatii0", 598), ("pvdsatii0", 599), ("tii", 600), ("alpha0", 601), ("alpha0l", 602), ("alpha0lexp", 603), ("lalpha0", 604), ("walpha0", 605), ("palpha0", 606), ("beta0", 607),
    ("lbeta0", 608), ("wbeta0", 609), ("pbeta0", 610), ("beta1", 611), ("lbeta1", 612), ("wbeta1", 613), ("pbeta1", 614), ("beta2", 615), ("lbeta2", 616), ("wbeta2", 617), ("pbeta2", 618), ("lii", 619), ("llii", 620), ("wlii", 621), ("plii", 622), ("sii0", 623),
    ("lsii0", 624), ("wsii0", 625), ("psii0", 626), ("sii1", 627), ("lsii1", 628), ("wsii1", 629), ("psii1", 630), ("sii2", 631), ("lsii2", 632), ("wsii2", 633), ("psii2", 634), ("siid", 635), ("lsiid", 636), ("wsiid", 637), ("psiid", 638), ("esatii", 639),
    ("lesatii", 640), ("wesatii", 641), ("pesatii", 642), ("iimod2clamp1", 643), ("iimod2clamp2", 644), ("iimod2clamp3", 645), ("fbjtii", 646), ("lfbjtii", 647), ("wfbjtii", 648), ("pfbjtii", 649), ("ebjtii", 650), ("cbjtii", 651), ("abjtii", 652), ("labjtii", 653), ("lcbjtii", 654), ("lebjtii", 655),
    ("wabjtii", 656), ("wcbjtii", 657), ("webjtii", 658), ("pabjtii", 659), ("pcbjtii", 660), ("pebjtii", 661), ("vbci", 662), ("lvbci", 663), ("wvbci", 664), ("pvbci", 665), ("tvbci", 666), ("mbjtii", 667), ("lmbjtii", 668), ("wmbjtii", 669), ("pmbjtii", 670), ("vecb", 671),
    ("alphagb1", 672), ("lalphagb1", 673), ("walphagb1", 674), ("palphagb1", 675), ("alphagb1_t", 676), ("lalphagb1_t", 677), ("walphagb1_t", 678), ("palphagb1_t", 679), ("betagb1", 680), ("lbetagb1", 681), ("wbetagb1", 682), ("pbetagb1", 683), ("alphagb2", 684), ("lalphagb2", 685), ("walphagb2", 686), ("palphagb2", 687),
    ("alphagb2_t", 688), ("lalphagb2_t", 689), ("walphagb2_t", 690), ("palphagb2_t", 691), ("betagb2", 692), ("lbetagb2", 693), ("wbetagb2", 694), ("pbetagb2", 695), ("vgb2", 696), ("vgb1", 697), ("agb1", 698), ("bgb1", 699), ("agb2", 700), ("bgb2", 701), ("agbc2n", 702), ("agbc2p", 703),
    ("bgbc2n", 704), ("bgbc2p", 705), ("eigbinv", 706), ("aigc", 707), ("bigc", 708), ("cigc", 709), ("aigs", 710), ("aigs1", 711), ("bigs", 712), ("cigs", 713), ("aigd", 714), ("aigd1", 715), ("bigd", 716), ("cigd", 717), ("dlcig", 718), ("dlcigd", 719),
    ("poxedge", 720), ("ntox", 721), ("toxref", 722), ("pigcd", 723), ("aigcl", 724), ("aigcw", 725), ("aigc1", 726), ("aigsl", 727), ("aigsw", 728), ("aigdl", 729), ("aigdw", 730), ("pigcdl", 731), ("leigbinv", 732), ("weigbinv", 733), ("peigbinv", 734), ("laigc", 735),
    ("laigc1", 736), ("waigc", 737), ("waigc1", 738), ("paigc", 739), ("paigc1", 740), ("lbigc", 741), ("wbigc", 742), ("pbigc", 743), ("lcigc", 744), ("wcigc", 745), ("pcigc", 746), ("laigs", 747), ("laigs1", 748), ("waigs", 749), ("waigs1", 750), ("paigs", 751),
    ("paigs1", 752), ("lbigs", 753), ("wbigs", 754), ("pbigs", 755), ("lcigs", 756), ("wcigs", 757), ("pcigs", 758), ("laigd", 759), ("laigd1", 760), ("waigd", 761), ("waigd1", 762), ("paigd", 763), ("paigd1", 764), ("lbigd", 765), ("wbigd", 766), ("pbigd", 767),
    ("lcigd", 768), ("wcigd", 769), ("pcigd", 770), ("lpoxedge", 771), ("wpoxedge", 772), ("ppoxedge", 773), ("ldlcig", 774), ("wdlcig", 775), ("pdlcig", 776), ("ldlcigd", 777), ("wdlcigd", 778), ("pdlcigd", 779), ("lntox", 780), ("wntox", 781), ("pntox", 782), ("aigbcp2", 783),
    ("aigbcp2_t", 784), ("bigbcp2", 785), ("cigbcp2", 786), ("laigbcp2", 787), ("laigbcp2_t", 788), ("lbigbcp2", 789), ("lcigbcp2", 790), ("waigbcp2", 791), ("waigbcp2_t", 792), ("wbigbcp2", 793), ("wcigbcp2", 794), ("paigbcp2", 795), ("paigbcp2_t", 796), ("pbigbcp2", 797), ("pcigbcp2", 798), ("agidl", 799),
    ("agidll", 800), ("agidlw", 801), ("lagidl", 802), ("wagidl", 803), ("pagidl", 804), ("bgidl", 805), ("bgidl1", 806), ("lbgidl", 807), ("wbgidl", 808), ("pbgidl", 809), ("lbgidl1", 810), ("wbgidl1", 811), ("pbgidl1", 812), ("cgidl", 813), ("lcgidl", 814), ("wcgidl", 815),
    ("pcgidl", 816), ("egidl", 817), ("legidl", 818), ("wegidl", 819), ("pegidl", 820), ("agisl", 821), ("agisll", 822), ("agislw", 823), ("lagisl", 824), ("wagisl", 825), ("pagisl", 826), ("bgisl", 827), ("bgisl1", 828), ("lbgisl", 829), ("wbgisl", 830), ("pbgisl", 831),
    ("lbgisl1", 832), ("wbgisl1", 833), ("pbgisl1", 834), ("cgisl", 835), ("lcgisl", 836), ("wcgisl", 837), ("pcgisl", 838), ("egisl", 839), ("legisl", 840), ("wegisl", 841), ("pegisl", 842), ("rgidl", 843), ("lrgidl", 844), ("wrgidl", 845), ("prgidl", 846), ("kgidl", 847),
    ("lkgidl", 848), ("wkgidl", 849), ("pkgidl", 850), ("fgidl", 851), ("lfgidl", 852), ("wfgidl", 853), ("pfgidl", 854), ("rgisl", 855), ("lrgisl", 856), ("wrgisl", 857), ("prgisl", 858), ("kgisl", 859), ("lkgisl", 860), ("wkgisl", 861), ("pkgisl", 862), ("fgisl", 863),
    ("lfgisl", 864), ("wfgisl", 865), ("pfgisl", 866), ("cf", 867), ("lcf", 868), ("wcf", 869), ("pcf", 870), ("cfrcoeff", 871), ("cgso", 872), ("cgdo", 873), ("cgbo", 874), ("cgsl", 875), ("lcgsl", 876), ("wcgsl", 877), ("pcgsl", 878), ("cgdl", 879),
    ("lcgdl", 880), ("wcgdl", 881), ("pcgdl", 882), ("ckappas", 883), ("lckappas", 884), ("wckappas", 885), ("pckappas", 886), ("ckappad", 887), ("lckappad", 888), ("wckappad", 889), ("pckappad", 890), ("ckappad1", 891), ("ckappad2", 892), ("ckappas1", 893), ("ckappas2", 894), ("dmcg", 895),
    ("dmci", 896), ("dmdg", 897), ("dmcgt", 898), ("xgl", 899), ("rshg", 900), ("cjs", 901), ("cjd", 902), ("cjsws", 903), ("cjswd", 904), ("cjswgs", 905), ("cjswgd", 906), ("pbs", 907), ("pbd", 908), ("pbsws", 909), ("pbswd", 910), ("pbswgs", 911),
    ("pbswgd", 912), ("mjs", 913), ("mjd", 914), ("mjsws", 915), ("mjswd", 916), ("mjswgs", 917), ("mjswgd", 918), ("tt", 919), ("ldif0", 920), ("ndif", 921), ("lndif", 922), ("wndif", 923), ("pndif", 924), ("vtm00", 925), ("permod", 926), ("dwj", 927),
    ("xdif", 928), ("lxdif", 929), ("wxdif", 930), ("pxdif", 931), ("isdif", 932), ("iddif", 933), ("lisdif", 934), ("liddif", 935), ("wisdif", 936), ("widdif", 937), ("pisdif", 938), ("piddif", 939), ("nrecf0", 940), ("lnrecf0", 941), ("wnrecf0", 942), ("pnrecf0", 943),
    ("nrecr0", 944), ("lnrecr0", 945), ("wnrecr0", 946), ("pnrecr0", 947), ("xrec", 948), ("lxrec", 949), ("wxrec", 950), ("pxrec", 951), ("isrec", 952), ("idrec", 953), ("lisrec", 954), ("lidrec", 955), ("wisrec", 956), ("widrec", 957), ("pisrec", 958), ("pidrec", 959),
    ("ntrecf", 960), ("ntrecr", 961), ("lntrecf", 962), ("lntrecr", 963), ("wntrecf", 964), ("wntrecr", 965), ("pntrecf", 966), ("pntrecr", 967), ("istun", 968), ("idtun", 969), ("listun", 970), ("lidtun", 971), ("wistun", 972), ("widtun", 973), ("pistun", 974), ("pidtun", 975),
    ("xtun", 976), ("xtund", 977), ("lxtun", 978), ("lxtund", 979), ("wxtun", 980), ("wxtund", 981), ("pxtun", 982), ("pxtund", 983), ("ntun", 984), ("ntund", 985), ("lntun", 986), ("lntund", 987), ("wntun", 988), ("wntund", 989), ("pntun", 990), ("pntund", 991),
    ("vtun0", 992), ("vtun0d", 993), ("lvtun0", 994), ("lvtun0d", 995), ("wvtun0", 996), ("wvtun0d", 997), ("pvtun0", 998), ("pvtun0d", 999), ("vrec0", 1000), ("vrec0d", 1001), ("lvrec0", 1002), ("lvrec0d", 1003), ("wvrec0", 1004), ("wvrec0d", 1005), ("pvrec0", 1006), ("pvrec0d", 1007),
    ("xrcrg1", 1008), ("xrcrg2", 1009), ("ef", 1010), ("em", 1011), ("noia", 1012), ("noib", 1013), ("noic", 1014), ("lintnoi", 1015), ("noia1", 1016), ("noiax", 1017), ("ntnoi", 1018), ("rnoia", 1019), ("rnoib", 1020), ("rnoic", 1021), ("tnoia", 1022), ("tnoib", 1023),
    ("tnoic", 1024), ("binunit", 1025), ("dlbin", 1026), ("dwbin", 1027), ("tnom", 1028), ("tbgasub", 1029), ("tbgbsub", 1030), ("tnfactor", 1031), ("ute", 1032), ("lute", 1033), ("wute", 1034), ("pute", 1035), ("utel", 1036), ("ua1", 1037), ("lua1", 1038), ("wua1", 1039),
    ("pua1", 1040), ("ua1l", 1041), ("uc1", 1042), ("luc1", 1043), ("wuc1", 1044), ("puc1", 1045), ("ud1", 1046), ("lud1", 1047), ("wud1", 1048), ("pud1", 1049), ("ud1l", 1050), ("eu1", 1051), ("leu1", 1052), ("weu1", 1053), ("peu1", 1054), ("ucste", 1055),
    ("lucste", 1056), ("wucste", 1057), ("pucste", 1058), ("teta0", 1059), ("prt", 1060), ("lprt", 1061), ("wprt", 1062), ("pprt", 1063), ("at", 1064), ("lat", 1065), ("wat", 1066), ("pat", 1067), ("atl", 1068), ("tdelta", 1069), ("ptwgt", 1070), ("lptwgt", 1071),
    ("wptwgt", 1072), ("pptwgt", 1073), ("ptwgtl", 1074), ("kt1", 1075), ("kt1exp", 1076), ("kt1l", 1077), ("lkt1", 1078), ("wkt1", 1079), ("pkt1", 1080), ("kt2", 1081), ("lkt2", 1082), ("wkt2", 1083), ("pkt2", 1084), ("iit", 1085), ("liit", 1086), ("wiit", 1087),
    ("piit", 1088), ("igt", 1089), ("ligt", 1090), ("wigt", 1091), ("pigt", 1092), ("tcj", 1093), ("tcjsw", 1094), ("tcjswg", 1095), ("tpb", 1096), ("tpbsw", 1097), ("tpbswg", 1098), ("rth0", 1099), ("cth0", 1100), ("wth0", 1101), ("saref", 1102), ("sbref", 1103),
    ("wlod", 1104), ("ku0", 1105), ("kvsat", 1106), ("tku0", 1107), ("lku0", 1108), ("wku0", 1109), ("pku0", 1110), ("llodku0", 1111), ("wlodku0", 1112), ("kvth0", 1113), ("lkvth0", 1114), ("wkvth0", 1115), ("pkvth0", 1116), ("llodvth", 1117), ("wlodvth", 1118), ("stk2", 1119),
    ("lodk2", 1120), ("steta0", 1121), ("lodeta0", 1122), ("web", 1123), ("wec", 1124), ("kvth0we", 1125), ("lkvth0we", 1126), ("wkvth0we", 1127), ("pkvth0we", 1128), ("k2we", 1129), ("lk2we", 1130), ("wk2we", 1131), ("pk2we", 1132), ("ku0we", 1133), ("lku0we", 1134), ("wku0we", 1135),
    ("pku0we", 1136), ("scref", 1137), ("ssl0", 1138), ("ssl1", 1139), ("ssl2", 1140), ("ssl3", 1141), ("ssl4", 1142), ("ssl5", 1143), ("sslexp1", 1144), ("sslexp2", 1145), ("avdsx", 1146), ("wedge", 1147), ("dgammaedge", 1148), ("dgammaedgel", 1149), ("dgammaedgelexp", 1150), ("dvtedge", 1151),
    ("ndepedge", 1152), ("lndepedge", 1153), ("wndepedge", 1154), ("pndepedge", 1155), ("nfactoredge", 1156), ("lnfactoredge", 1157), ("wnfactoredge", 1158), ("pnfactoredge", 1159), ("citedge", 1160), ("lcitedge", 1161), ("wcitedge", 1162), ("pcitedge", 1163), ("cdscedge", 1164), ("lcdscedge", 1165), ("wcdscedge", 1166), ("pcdscedge", 1167),
    ("cdscdedge", 1168), ("lcdscdedge", 1169), ("wcdscdedge", 1170), ("pcdscdedge", 1171), ("cdscdedger", 1172), ("lcdscdedger", 1173), ("wcdscdedger", 1174), ("pcdscdedger", 1175), ("csecseedge", 1176), ("lcsecseedge", 1177), ("wcsecseedge", 1178), ("pcsecseedge", 1179), ("csecsepedge", 1180), ("csecse0edge", 1181), ("csecse0pedge", 1182), ("csecsededge", 1183),
    ("cbcb0edge", 1184), ("cbcb0pedge", 1185), ("cdscbedge", 1186), ("lcdscbedge", 1187), ("wcdscbedge", 1188), ("pcdscbedge", 1189), ("cbcbpedge", 1190), ("cbcbedge", 1191), ("lcbcbedge", 1192), ("wcbcbedge", 1193), ("pcbcbedge", 1194), ("cbcbdedge", 1195), ("k1edge", 1196), ("k1ledge", 1197), ("k1lexpedge", 1198), ("k1wedge", 1199),
    ("k1wexpedge", 1200), ("k1wledge", 1201), ("k1wlexpedge", 1202), ("lk1edge", 1203), ("wk1edge", 1204), ("pk1edge", 1205), ("eta0edge", 1206), ("leta0edge", 1207), ("weta0edge", 1208), ("peta0edge", 1209), ("etabedge", 1210), ("letabedge", 1211), ("wetabedge", 1212), ("petabedge", 1213), ("kt1edge", 1214), ("lkt1edge", 1215),
    ("wkt1edge", 1216), ("pkt1edge", 1217), ("kt1ledge", 1218), ("lkt1ledge", 1219), ("wkt1ledge", 1220), ("pkt1ledge", 1221), ("kt2edge", 1222), ("lkt2edge", 1223), ("wkt2edge", 1224), ("pkt2edge", 1225), ("kt1expedge", 1226), ("lkt1expedge", 1227), ("wkt1expedge", 1228), ("pkt1expedge", 1229), ("tnfactoredge", 1230), ("ltnfactoredge", 1231),
    ("wtnfactoredge", 1232), ("ptnfactoredge", 1233), ("teta0edge", 1234), ("lteta0edge", 1235), ("wteta0edge", 1236), ("pteta0edge", 1237), ("dvtp0edge", 1238), ("ldvtp0edge", 1239), ("wdvtp0edge", 1240), ("pdvtp0edge", 1241), ("dvtp1edge", 1242), ("ldvtp1edge", 1243), ("wdvtp1edge", 1244), ("pdvtp1edge", 1245), ("dvtp2edge", 1246), ("ldvtp2edge", 1247),
    ("wdvtp2edge", 1248), ("pdvtp2edge", 1249), ("dvtp3edge", 1250), ("ldvtp3edge", 1251), ("wdvtp3edge", 1252), ("pdvtp3edge", 1253), ("dvtp4edge", 1254), ("ldvtp4edge", 1255), ("wdvtp4edge", 1256), ("pdvtp4edge", 1257), ("dvtp5edge", 1258), ("ldvtp5edge", 1259), ("wdvtp5edge", 1260), ("pdvtp5edge", 1261), ("dvt0edge", 1262), ("dvt1edge", 1263),
    ("dvt2edge", 1264), ("k2edge", 1265), ("k2ledge", 1266), ("k2lexpedge", 1267), ("k2wedge", 1268), ("k2wexpedge", 1269), ("k2wledge", 1270), ("k2wlexpedge", 1271), ("lk2edge", 1272), ("wk2edge", 1273), ("pk2edge", 1274), ("kvth0edge", 1275), ("lkvth0edge", 1276), ("wkvth0edge", 1277), ("pkvth0edge", 1278), ("kvth0edgewe", 1279),
    ("lkvth0edgewe", 1280), ("wkvth0edgewe", 1281), ("pkvth0edgewe", 1282), ("k2edgewe", 1283), ("lk2edgewe", 1284), ("wk2edgewe", 1285), ("pk2edgewe", 1286), ("stk2edge", 1287), ("lstk2edge", 1288), ("wstk2edge", 1289), ("pstk2edge", 1290), ("steta0edge", 1291), ("lsteta0edge", 1292), ("wsteta0edge", 1293), ("psteta0edge", 1294), ("igclamp", 1295),
    ("lp", 1296), ("rnoik", 1297), ("tnoik", 1298), ("tnoik2", 1299), ("k0", 1300), ("lk0", 1301), ("wk0", 1302), ("pk0", 1303), ("k01", 1304), ("lk01", 1305), ("wk01", 1306), ("pk01", 1307), ("m0", 1308), ("lm0", 1309), ("wm0", 1310), ("pm0", 1311),
    ("m01", 1312), ("lm01", 1313), ("wm01", 1314), ("pm01", 1315), ("nedge", 1316), ("noia1_edge", 1317), ("noiax_edge", 1318), ("fnoimod", 1319), ("lh", 1320), ("noia2", 1321), ("hndep", 1322), ("c0", 1323), ("lc0", 1324), ("wc0", 1325), ("pc0", 1326), ("c01", 1327),
    ("lc01", 1328), ("wc01", 1329), ("pc01", 1330), ("c0si", 1331), ("lc0si", 1332), ("wc0si", 1333), ("pc0si", 1334), ("c0si1", 1335), ("lc0si1", 1336), ("wc0si1", 1337), ("pc0si1", 1338), ("c0sisat", 1339), ("lc0sisat", 1340), ("wc0sisat", 1341), ("pc0sisat", 1342), ("c0sisat1", 1343),
    ("lc0sisat1", 1344), ("wc0sisat1", 1345), ("pc0sisat1", 1346), ("minr", 1347), ("abulk", 1348), ("a0", 1349), ("ags", 1350), ("ags1", 1351), ("keta", 1352), ("a0cv", 1353), ("agscv", 1354), ("ketacv", 1355), ("rbody", 1356), ("frbody", 1357), ("rbsh", 1358), ("nrb", 1359),
    ("rhalo", 1360), ("ub", 1361), ("lub", 1362), ("wub", 1363), ("pub", 1364), ("ubte", 1365), ("lubte", 1366), ("wubte", 1367), ("pubte", 1368), ("neff", 1369), ("lneff", 1370), ("wneff", 1371), ("pneff", 1372), ("nseg", 1373), ("rbodyagbcp2", 1374), ("nbc", 1375),
    ("dwbc", 1376), ("pdbcp", 1377), ("psbcp", 1378), ("agbcp", 1379), ("agbcp2", 1380), ("agbcpd", 1381), ("aebcp", 1382), ("eggbcp2", 1383), ("nsub", 1384), ("lnsub", 1385), ("wnsub", 1386), ("pnsub", 1387), ("fbody", 1388), ("kb1", 1389), ("lkb1", 1390), ("wkb1", 1391),
    ("pkb1", 1392), ("dlbg", 1393), ("dlcb", 1394), ("csdesw", 1395), ("csdmin", 1396), ("acesb", 1397), ("bcesb", 1398), ("acedb", 1399), ("bcedb", 1400),
];

pub(crate) const PARAMETER_MODEL_FLAGS: [bool; 1401] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true,
    true, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, false,
    true, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1401] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1401] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, Some(0), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1401] = [
    "L", "W", "NF", "NRS", "NRD", "VFBSDOFF", "MINZ", "RGATEMOD", "GEOMOD", "RGEOMOD", "SA", "SB", "SD", "SCA", "SCB", "SCC",
    "SC", "AS", "AD", "PS", "PD", "XGW", "NGCON", "DTEMP", "MULU0", "DELVTO", "IDS0MULT", "EDGEFET", "SSLMOD", "SOIMOD", "TYPE", "CVMOD",
    "COVMOD", "RDSMOD", "WPEMOD", "ASYMMOD", "GIDLMOD", "IGCMOD", "IGBMOD", "TNOIMOD", "TNODEOUT", "SHMOD", "MOBSCALE", "BODYMOD", "IIIMOD", "MODAGBCP2", "PDEMOD", "FBODY1",
    "LLONG", "LMLT", "WMLT", "XL", "WWIDE", "XW", "LINT", "LL", "LW", "LWL", "LLN", "LWN", "WINT", "WL", "WW", "WWL",
    "WLN", "WWN", "DLC", "LLC", "LWC", "LWLC", "DWC", "WLC", "WWC", "WWLC", "TSI", "TBOX", "TOXE", "TOXP", "DTOX", "NDEP",
    "NDEPL1", "NDEPLEXP1", "NDEPL2", "NDEPLEXP2", "NDEPW", "NDEPWEXP", "NDEPWL", "NDEPWLEXP", "LNDEP", "WNDEP", "PNDEP", "NDEPCV", "NDEPCVL1", "NDEPCVLEXP1", "NDEPCVL2", "NDEPCVLEXP2",
    "NDEPCVW", "NDEPCVWEXP", "NDEPCVWL", "NDEPCVWLEXP", "LNDEPCV", "WNDEPCV", "PNDEPCV", "NGATE", "LNGATE", "WNGATE", "PNGATE", "NI0SUB", "BG0SUB", "EPSRSUB", "EPSROX", "XJ",
    "LXJ", "WXJ", "PXJ", "VFB", "LVFB", "WVFB", "PVFB", "VFBB", "LVFBB", "WVFBB", "PVFBB", "VFBL", "VFBLEXP", "VFBW", "VFBWEXP", "VFBWL",
    "VFBWLEXP", "VFBCV", "LVFBCV", "WVFBCV", "PVFBCV", "VFBCVL", "VFBCVLEXP", "VFBCVW", "VFBCVWEXP", "VFBCVWL", "VFBCVWLEXP", "DELVFBACC", "VFBAGBCP2", "NDEPAGBCP2", "NSD", "LNSD",
    "WNSD", "PNSD", "DVTP0", "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2", "PDVTP2", "DVTP3", "LDVTP3",
    "WDVTP3", "PDVTP3", "DVTP4", "LDVTP4", "WDVTP4", "PDVTP4", "DVTP5", "LDVTP5", "WDVTP5", "PDVTP5", "DVBD0", "LDVBD0", "WDVBD0", "PDVBD0", "DVBD1", "LDVBD1",
    "WDVBD1", "PDVBD1", "VSCE", "LVSCE", "WVSCE", "PVSCE", "CDSBS1", "LCDSBS1", "WCDSBS1", "PCDSBS1", "CDSBS", "LCDSBS", "WCDSBS", "PCDSBS", "PHIN", "LPHIN",
    "WPHIN", "PPHIN", "ETA0", "LETA0", "WETA0", "PETA0", "ETA0R", "LETA0R", "WETA0R", "PETA0R", "DSUB", "ETAB", "ETABEXP", "LETAB", "WETAB", "PETAB",
    "K1", "K1L", "K1LEXP", "K1W", "K1WEXP", "K1WL", "K1WLEXP", "LK1", "WK1", "PK1", "K2", "K2L", "K2LEXP", "K2W", "K2WEXP", "K2WL",
    "K2WLEXP", "LK2", "WK2", "PK2", "ADOS", "BDOS", "QM0", "ETAQM", "CIT", "LCIT", "WCIT", "PCIT", "NFACTOR", "NFACTORL", "NFACTORLEXP", "NFACTORW",
    "NFACTORWEXP", "NFACTORWL", "NFACTORWLEXP", "LNFACTOR", "WNFACTOR", "PNFACTOR", "ASCL", "LASCL", "WASCL", "PASCL", "BSCL", "LBSCL", "WBSCL", "PBSCL", "DVT1", "LDVT1",
    "WDVT1", "PDVT1", "CDSCD", "LCDSCD", "WCDSCD", "PCDSCD", "CDSC", "LCDSC", "WCDSC", "PCDSC", "CSECSED", "CBCBD", "CSECSE0", "CSECSE0P", "CSECSE", "LCSECSE",
    "WCSECSE", "PCSECSE", "CSECSEP", "CBCB", "LCBCB", "WCBCB", "PCBCB", "CBCBP", "CBCB0", "CBCB0P", "CDSCDL", "CDSCDLEXP", "CDSCDR", "LCDSCDR", "WCDSCDR", "PCDSCDR",
    "CDSCB", "CDSCBL", "CDSCBLEXP", "LCDSCB", "WCDSCB", "PCDSCB", "VBSA", "VSAT", "LVSAT", "WVSAT", "PVSAT", "VSATL", "VSATLEXP", "VSATW", "VSATWEXP", "VSATWL",
    "VSATWLEXP", "VSATR", "LVSATR", "WVSATR", "PVSATR", "DELTA", "LDELTA", "WDELTA", "PDELTA", "DELTAL", "DELTALEXP", "VSATCV", "LVSATCV", "WVSATCV", "PVSATCV", "VSATCVL",
    "VSATCVLEXP", "VSATCVW", "VSATCVWEXP", "VSATCVWL", "VSATCVWLEXP", "THESAT", "LTHESAT", "WTHESAT", "PTHESAT", "LPE1", "LLPE1", "WLPE1", "PLPE1", "UP1", "LP1", "UP2",
    "LP2", "U0", "U0L", "U0LEXP", "LU0", "WU0", "PU0", "U0R", "LU0R", "WU0R", "PU0R", "ETAMOB", "UA", "UAL", "UALEXP", "UAW",
    "UAWEXP", "UAWL", "UAWLEXP", "LUA", "WUA", "PUA", "UAR", "LUAR", "WUAR", "PUAR", "EU", "LEU", "WEU", "PEU", "EUL", "EULEXP",
    "EUW", "EUWEXP", "EUWL", "EUWLEXP", "UD", "UDL", "UDLEXP", "LUD", "WUD", "PUD", "UDR", "LUDR", "WUDR", "PUDR", "UCS", "LUCS",
    "WUCS", "PUCS", "UCSR", "LUCSR", "WUCSR", "PUCSR", "UC", "UCL", "UCLEXP", "UCW", "UCWEXP", "UCWL", "UCWLEXP", "LUC", "WUC", "PUC",
    "UCR", "LUCR", "WUCR", "PUCR", "PCLM", "PCLML", "PCLMLEXP", "LPCLM", "WPCLM", "PPCLM", "PCLMR", "LPCLMR", "WPCLMR", "PPCLMR", "PCLMG", "PCLMCV",
    "PCLMCVL", "PCLMCVLEXP", "LPCLMCV", "WPCLMCV", "PPCLMCV", "PSCBE1", "LPSCBE1", "WPSCBE1", "PPSCBE1", "PSCBE2", "LPSCBE2", "WPSCBE2", "PPSCBE2", "PDITS", "LPDITS", "WPDITS",
    "PPDITS", "PDITSL", "PDITSD", "LPDITSD", "WPDITSD", "PPDITSD", "RSH", "PRWG", "LPRWG", "WPRWG", "PPRWG", "PRWB", "LPRWB", "WPRWB", "PPRWB", "PRWBL",
    "PRWBLEXP", "WR", "LWR", "WWR", "PWR", "RSWMIN", "LRSWMIN", "WRSWMIN", "PRSWMIN", "RSW", "LRSW", "WRSW", "PRSW", "RSWL", "RSWLEXP", "RDWMIN",
    "LRDWMIN", "WRDWMIN", "PRDWMIN", "RDW", "LRDW", "WRDW", "PRDW", "RDWL", "RDWLEXP", "RDSWMIN", "LRDSWMIN", "WRDSWMIN", "PRDSWMIN", "RDSW", "RDSWL", "RDSWLEXP",
    "LRDSW", "WRDSW", "PRDSW", "PSAT", "LPSAT", "WPSAT", "PPSAT", "PSATL", "PSATLEXP", "PSATB", "PSATR", "LPSATR", "WPSATR", "PPSATR", "LPSATB", "WPSATB",
    "PPSATB", "PSATX", "PTWG", "LPTWG", "WPTWG", "PPTWG", "PTWGL", "VP", "ALP", "PTWGLEXP", "PTWGR", "LPTWGR", "WPTWGR", "PPTWGR", "KSATIV", "LKSATIV",
    "WKSATIV", "PKSATIV", "A1", "LA1", "WA1", "PA1", "A11", "LA11", "WA11", "PA11", "A2", "LA2", "WA2", "PA2", "A21", "LA21",
    "WA21", "PA21", "PDIBLC", "PDIBLCL", "PDIBLCLEXP", "LPDIBLC", "WPDIBLC", "PPDIBLC", "PDIBLCR", "LPDIBLCR", "WPDIBLCR", "PPDIBLCR", "PDIBLCB", "LPDIBLCB", "WPDIBLCB", "PPDIBLCB",
    "PVAG", "LPVAG", "WPVAG", "PPVAG", "FPROUT", "FPROUTL", "FPROUTLEXP", "LFPROUT", "WFPROUT", "PFPROUT", "BJTOFF", "VABJT", "LVABJT", "WVABJT", "PVABJT", "AELY",
    "LAELY", "WAELY", "PAELY", "AHLI", "AHLID", "LAHLI", "LAHLID", "WAHLI", "WAHLID", "PAHLI", "PAHLID", "XBJT", "LXBJT", "WXBJT", "PXBJT", "NDIODE",
    "LNDIODE", "WNDIODE", "PNDIODE", "ISBJT", "PISBJT", "WISBJT", "LISBJT", "IDBJT", "LIDBJT", "WIDBJT", "PIDBJT", "NBJT", "LNBJT", "LLBJT0", "WNBJT", "WLBJT0",
    "PNBJT", "PLBJT0", "LBJT0", "LN", "VDSATII0", "LVDSATII0", "WVDSATII0", "PVDSATII0", "TII", "ALPHA0", "ALPHA0L", "ALPHA0LEXP", "LALPHA0", "WALPHA0", "PALPHA0", "BETA0",
    "LBETA0", "WBETA0", "PBETA0", "BETA1", "LBETA1", "WBETA1", "PBETA1", "BETA2", "LBETA2", "WBETA2", "PBETA2", "LII", "LLII", "WLII", "PLII", "SII0",
    "LSII0", "WSII0", "PSII0", "SII1", "LSII1", "WSII1", "PSII1", "SII2", "LSII2", "WSII2", "PSII2", "SIID", "LSIID", "WSIID", "PSIID", "ESATII",
    "LESATII", "WESATII", "PESATII", "IIMOD2CLAMP1", "IIMOD2CLAMP2", "IIMOD2CLAMP3", "FBJTII", "LFBJTII", "WFBJTII", "PFBJTII", "EBJTII", "CBJTII", "ABJTII", "LABJTII", "LCBJTII", "LEBJTII",
    "WABJTII", "WCBJTII", "WEBJTII", "PABJTII", "PCBJTII", "PEBJTII", "VBCI", "LVBCI", "WVBCI", "PVBCI", "TVBCI", "MBJTII", "LMBJTII", "WMBJTII", "PMBJTII", "VECB",
    "ALPHAGB1", "LALPHAGB1", "WALPHAGB1", "PALPHAGB1", "ALPHAGB1_T", "LALPHAGB1_T", "WALPHAGB1_T", "PALPHAGB1_T", "BETAGB1", "LBETAGB1", "WBETAGB1", "PBETAGB1", "ALPHAGB2", "LALPHAGB2", "WALPHAGB2", "PALPHAGB2",
    "ALPHAGB2_T", "LALPHAGB2_T", "WALPHAGB2_T", "PALPHAGB2_T", "BETAGB2", "LBETAGB2", "WBETAGB2", "PBETAGB2", "VGB2", "VGB1", "AGB1", "BGB1", "AGB2", "BGB2", "AGBC2N", "AGBC2P",
    "BGBC2N", "BGBC2P", "EIGBINV", "AIGC", "BIGC", "CIGC", "AIGS", "AIGS1", "BIGS", "CIGS", "AIGD", "AIGD1", "BIGD", "CIGD", "DLCIG", "DLCIGD",
    "POXEDGE", "NTOX", "TOXREF", "PIGCD", "AIGCL", "AIGCW", "AIGC1", "AIGSL", "AIGSW", "AIGDL", "AIGDW", "PIGCDL", "LEIGBINV", "WEIGBINV", "PEIGBINV", "LAIGC",
    "LAIGC1", "WAIGC", "WAIGC1", "PAIGC", "PAIGC1", "LBIGC", "WBIGC", "PBIGC", "LCIGC", "WCIGC", "PCIGC", "LAIGS", "LAIGS1", "WAIGS", "WAIGS1", "PAIGS",
    "PAIGS1", "LBIGS", "WBIGS", "PBIGS", "LCIGS", "WCIGS", "PCIGS", "LAIGD", "LAIGD1", "WAIGD", "WAIGD1", "PAIGD", "PAIGD1", "LBIGD", "WBIGD", "PBIGD",
    "LCIGD", "WCIGD", "PCIGD", "LPOXEDGE", "WPOXEDGE", "PPOXEDGE", "LDLCIG", "WDLCIG", "PDLCIG", "LDLCIGD", "WDLCIGD", "PDLCIGD", "LNTOX", "WNTOX", "PNTOX", "AIGBCP2",
    "AIGBCP2_T", "BIGBCP2", "CIGBCP2", "LAIGBCP2", "LAIGBCP2_T", "LBIGBCP2", "LCIGBCP2", "WAIGBCP2", "WAIGBCP2_T", "WBIGBCP2", "WCIGBCP2", "PAIGBCP2", "PAIGBCP2_T", "PBIGBCP2", "PCIGBCP2", "AGIDL",
    "AGIDLL", "AGIDLW", "LAGIDL", "WAGIDL", "PAGIDL", "BGIDL", "BGIDL1", "LBGIDL", "WBGIDL", "PBGIDL", "LBGIDL1", "WBGIDL1", "PBGIDL1", "CGIDL", "LCGIDL", "WCGIDL",
    "PCGIDL", "EGIDL", "LEGIDL", "WEGIDL", "PEGIDL", "AGISL", "AGISLL", "AGISLW", "LAGISL", "WAGISL", "PAGISL", "BGISL", "BGISL1", "LBGISL", "WBGISL", "PBGISL",
    "LBGISL1", "WBGISL1", "PBGISL1", "CGISL", "LCGISL", "WCGISL", "PCGISL", "EGISL", "LEGISL", "WEGISL", "PEGISL", "RGIDL", "LRGIDL", "WRGIDL", "PRGIDL", "KGIDL",
    "LKGIDL", "WKGIDL", "PKGIDL", "FGIDL", "LFGIDL", "WFGIDL", "PFGIDL", "RGISL", "LRGISL", "WRGISL", "PRGISL", "KGISL", "LKGISL", "WKGISL", "PKGISL", "FGISL",
    "LFGISL", "WFGISL", "PFGISL", "CF", "LCF", "WCF", "PCF", "CFRCOEFF", "CGSO", "CGDO", "CGBO", "CGSL", "LCGSL", "WCGSL", "PCGSL", "CGDL",
    "LCGDL", "WCGDL", "PCGDL", "CKAPPAS", "LCKAPPAS", "WCKAPPAS", "PCKAPPAS", "CKAPPAD", "LCKAPPAD", "WCKAPPAD", "PCKAPPAD", "CKAPPAD1", "CKAPPAD2", "CKAPPAS1", "CKAPPAS2", "DMCG",
    "DMCI", "DMDG", "DMCGT", "XGL", "RSHG", "CJS", "CJD", "CJSWS", "CJSWD", "CJSWGS", "CJSWGD", "PBS", "PBD", "PBSWS", "PBSWD", "PBSWGS",
    "PBSWGD", "MJS", "MJD", "MJSWS", "MJSWD", "MJSWGS", "MJSWGD", "TT", "LDIF0", "NDIF", "LNDIF", "WNDIF", "PNDIF", "VTM00", "PERMOD", "DWJ",
    "XDIF", "LXDIF", "WXDIF", "PXDIF", "ISDIF", "IDDIF", "LISDIF", "LIDDIF", "WISDIF", "WIDDIF", "PISDIF", "PIDDIF", "NRECF0", "LNRECF0", "WNRECF0", "PNRECF0",
    "NRECR0", "LNRECR0", "WNRECR0", "PNRECR0", "XREC", "LXREC", "WXREC", "PXREC", "ISREC", "IDREC", "LISREC", "LIDREC", "WISREC", "WIDREC", "PISREC", "PIDREC",
    "NTRECF", "NTRECR", "LNTRECF", "LNTRECR", "WNTRECF", "WNTRECR", "PNTRECF", "PNTRECR", "ISTUN", "IDTUN", "LISTUN", "LIDTUN", "WISTUN", "WIDTUN", "PISTUN", "PIDTUN",
    "XTUN", "XTUND", "LXTUN", "LXTUND", "WXTUN", "WXTUND", "PXTUN", "PXTUND", "NTUN", "NTUND", "LNTUN", "LNTUND", "WNTUN", "WNTUND", "PNTUN", "PNTUND",
    "VTUN0", "VTUN0D", "LVTUN0", "LVTUN0D", "WVTUN0", "WVTUN0D", "PVTUN0", "PVTUN0D", "VREC0", "VREC0D", "LVREC0", "LVREC0D", "WVREC0", "WVREC0D", "PVREC0", "PVREC0D",
    "XRCRG1", "XRCRG2", "EF", "EM", "NOIA", "NOIB", "NOIC", "LINTNOI", "NOIA1", "NOIAX", "NTNOI", "RNOIA", "RNOIB", "RNOIC", "TNOIA", "TNOIB",
    "TNOIC", "BINUNIT", "DLBIN", "DWBIN", "TNOM", "TBGASUB", "TBGBSUB", "TNFACTOR", "UTE", "LUTE", "WUTE", "PUTE", "UTEL", "UA1", "LUA1", "WUA1",
    "PUA1", "UA1L", "UC1", "LUC1", "WUC1", "PUC1", "UD1", "LUD1", "WUD1", "PUD1", "UD1L", "EU1", "LEU1", "WEU1", "PEU1", "UCSTE",
    "LUCSTE", "WUCSTE", "PUCSTE", "TETA0", "PRT", "LPRT", "WPRT", "PPRT", "AT", "LAT", "WAT", "PAT", "ATL", "TDELTA", "PTWGT", "LPTWGT",
    "WPTWGT", "PPTWGT", "PTWGTL", "KT1", "KT1EXP", "KT1L", "LKT1", "WKT1", "PKT1", "KT2", "LKT2", "WKT2", "PKT2", "IIT", "LIIT", "WIIT",
    "PIIT", "IGT", "LIGT", "WIGT", "PIGT", "TCJ", "TCJSW", "TCJSWG", "TPB", "TPBSW", "TPBSWG", "RTH0", "CTH0", "WTH0", "SAREF", "SBREF",
    "WLOD", "KU0", "KVSAT", "TKU0", "LKU0", "WKU0", "PKU0", "LLODKU0", "WLODKU0", "KVTH0", "LKVTH0", "WKVTH0", "PKVTH0", "LLODVTH", "WLODVTH", "STK2",
    "LODK2", "STETA0", "LODETA0", "WEB", "WEC", "KVTH0WE", "LKVTH0WE", "WKVTH0WE", "PKVTH0WE", "K2WE", "LK2WE", "WK2WE", "PK2WE", "KU0WE", "LKU0WE", "WKU0WE",
    "PKU0WE", "SCREF", "SSL0", "SSL1", "SSL2", "SSL3", "SSL4", "SSL5", "SSLEXP1", "SSLEXP2", "AVDSX", "WEDGE", "DGAMMAEDGE", "DGAMMAEDGEL", "DGAMMAEDGELEXP", "DVTEDGE",
    "NDEPEDGE", "LNDEPEDGE", "WNDEPEDGE", "PNDEPEDGE", "NFACTOREDGE", "LNFACTOREDGE", "WNFACTOREDGE", "PNFACTOREDGE", "CITEDGE", "LCITEDGE", "WCITEDGE", "PCITEDGE", "CDSCEDGE", "LCDSCEDGE", "WCDSCEDGE", "PCDSCEDGE",
    "CDSCDEDGE", "LCDSCDEDGE", "WCDSCDEDGE", "PCDSCDEDGE", "CDSCDEDGER", "LCDSCDEDGER", "WCDSCDEDGER", "PCDSCDEDGER", "CSECSEEDGE", "LCSECSEEDGE", "WCSECSEEDGE", "PCSECSEEDGE", "CSECSEPEDGE", "CSECSE0EDGE", "CSECSE0PEDGE", "CSECSEDEDGE",
    "CBCB0EDGE", "CBCB0PEDGE", "CDSCBEDGE", "LCDSCBEDGE", "WCDSCBEDGE", "PCDSCBEDGE", "CBCBPEDGE", "CBCBEDGE", "LCBCBEDGE", "WCBCBEDGE", "PCBCBEDGE", "CBCBDEDGE", "K1EDGE", "K1LEDGE", "K1LEXPEDGE", "K1WEDGE",
    "K1WEXPEDGE", "K1WLEDGE", "K1WLEXPEDGE", "LK1EDGE", "WK1EDGE", "PK1EDGE", "ETA0EDGE", "LETA0EDGE", "WETA0EDGE", "PETA0EDGE", "ETABEDGE", "LETABEDGE", "WETABEDGE", "PETABEDGE", "KT1EDGE", "LKT1EDGE",
    "WKT1EDGE", "PKT1EDGE", "KT1LEDGE", "LKT1LEDGE", "WKT1LEDGE", "PKT1LEDGE", "KT2EDGE", "LKT2EDGE", "WKT2EDGE", "PKT2EDGE", "KT1EXPEDGE", "LKT1EXPEDGE", "WKT1EXPEDGE", "PKT1EXPEDGE", "TNFACTOREDGE", "LTNFACTOREDGE",
    "WTNFACTOREDGE", "PTNFACTOREDGE", "TETA0EDGE", "LTETA0EDGE", "WTETA0EDGE", "PTETA0EDGE", "DVTP0EDGE", "LDVTP0EDGE", "WDVTP0EDGE", "PDVTP0EDGE", "DVTP1EDGE", "LDVTP1EDGE", "WDVTP1EDGE", "PDVTP1EDGE", "DVTP2EDGE", "LDVTP2EDGE",
    "WDVTP2EDGE", "PDVTP2EDGE", "DVTP3EDGE", "LDVTP3EDGE", "WDVTP3EDGE", "PDVTP3EDGE", "DVTP4EDGE", "LDVTP4EDGE", "WDVTP4EDGE", "PDVTP4EDGE", "DVTP5EDGE", "LDVTP5EDGE", "WDVTP5EDGE", "PDVTP5EDGE", "DVT0EDGE", "DVT1EDGE",
    "DVT2EDGE", "K2EDGE", "K2LEDGE", "K2LEXPEDGE", "K2WEDGE", "K2WEXPEDGE", "K2WLEDGE", "K2WLEXPEDGE", "LK2EDGE", "WK2EDGE", "PK2EDGE", "KVTH0EDGE", "LKVTH0EDGE", "WKVTH0EDGE", "PKVTH0EDGE", "KVTH0EDGEWE",
    "LKVTH0EDGEWE", "WKVTH0EDGEWE", "PKVTH0EDGEWE", "K2EDGEWE", "LK2EDGEWE", "WK2EDGEWE", "PK2EDGEWE", "STK2EDGE", "LSTK2EDGE", "WSTK2EDGE", "PSTK2EDGE", "STETA0EDGE", "LSTETA0EDGE", "WSTETA0EDGE", "PSTETA0EDGE", "IGCLAMP",
    "LP", "RNOIK", "TNOIK", "TNOIK2", "K0", "LK0", "WK0", "PK0", "K01", "LK01", "WK01", "PK01", "M0", "LM0", "WM0", "PM0",
    "M01", "LM01", "WM01", "PM01", "NEDGE", "NOIA1_EDGE", "NOIAX_EDGE", "FNOIMOD", "LH", "NOIA2", "HNDEP", "C0", "LC0", "WC0", "PC0", "C01",
    "LC01", "WC01", "PC01", "C0SI", "LC0SI", "WC0SI", "PC0SI", "C0SI1", "LC0SI1", "WC0SI1", "PC0SI1", "C0SISAT", "LC0SISAT", "WC0SISAT", "PC0SISAT", "C0SISAT1",
    "LC0SISAT1", "WC0SISAT1", "PC0SISAT1", "minr", "ABULK", "A0", "AGS", "AGS1", "KETA", "A0CV", "AGSCV", "KETACV", "RBODY", "FRBODY", "RBSH", "NRB",
    "RHALO", "UB", "LUB", "WUB", "PUB", "UBTE", "LUBTE", "WUBTE", "PUBTE", "NEFF", "LNEFF", "WNEFF", "PNEFF", "NSEG", "RBODYAGBCP2", "NBC",
    "DWBC", "PDBCP", "PSBCP", "AGBCP", "AGBCP2", "AGBCPD", "AEBCP", "EGGBCP2", "NSUB", "LNSUB", "WNSUB", "PNSUB", "FBODY", "KB1", "LKB1", "WKB1",
    "PKB1", "DLBG", "DLCB", "CSDESW", "CSDMIN", "ACESB", "BCESB", "ACEDB", "BCEDB",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1401] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1401] = [
    false, false, true, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1401] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -50.0, label: "-50.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1401] = [
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 8.0, label: "8.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1401] = [
    3, 3, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3,
    0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0,
    3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0,
    3, 0, 0, 0, 2, 2, 3, 2, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
    3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
    3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 2,
    2, 2, 2, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 1, 0, 0, 0, 0, 0, 2, 3, 2, 0, 0, 0, 3, 3,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3, 3, 3,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 3, 2, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1401] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[],
];

fn parameter_computed_min_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        _ => None,
    };
    if let Some(bound) = bound {
        validate_finite_parameter(bound.label, bound.value)?;
    }
    Ok(bound)
}

fn parameter_computed_max_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        899 => Some(ParameterBound { value: ((params[0] * params[49]) + params[51]), label: "computed upper-bound expression" }),
        _ => None,
    };
    if let Some(bound) = bound {
        validate_finite_parameter(bound.label, bound.value)?;
    }
    Ok(bound)
}

fn validate_parameter_computed_exclusions(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    let params = parameters;
    match index {
        _ => {}
    }
    Ok(())
}

fn parameter_index_for_name(name: &str) -> Option<usize> {
    PARAMETER_NAME_LOOKUP
        .iter()
        .find_map(|(candidate, index)| (*candidate == name).then_some(*index))
}

#[derive(Clone)]
pub(crate) struct StampState<const DDT: usize, const IDT: usize> {
    pub(crate) ddt_current: [f64; DDT],
    pub(crate) ddt_previous: [f64; DDT],
    pub(crate) ddt_older: [f64; DDT],
    pub(crate) ddt_derivative_current: [f64; DDT],
    pub(crate) ddt_derivative_previous: [f64; DDT],
    pub(crate) idt_current: [f64; IDT],
    pub(crate) idt_previous: [f64; IDT],
    pub(crate) ddt_initialized: [bool; DDT],
    pub(crate) idt_initialized: [bool; IDT],
}

impl<const DDT: usize, const IDT: usize> StampState<DDT, IDT> {
    fn new_box() -> Box<Self> {
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            // SAFETY: every field is an array of f64 or bool; all-zero bytes are valid values for both.
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }
}

pub(crate) type CanonicalModelValues = [f64; 280];
pub struct Instance {
    pub nodes: [usize; 14],
    pub branches: [usize; 12],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1401]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<23, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) canonical_reactive: Box<[f64; 216]>,
    pub(crate) canonical_model_values: Option<std::sync::Arc<CanonicalModelValues>>,
    pub(crate) canonical_staged: Box<[f64; 1192]>,
    pub(crate) canonical_instance_valid: bool,
    pub(crate) canonical_temperature_valid: bool,
    pub(crate) canonical_temperature: f64,
    pub(crate) canonical_thermal_voltage: f64,
}

impl Clone for Instance {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes,
            branches: self.branches,
            params: self.params.clone(),
            param_given: self.param_given.clone(),
            multiplicity: self.multiplicity,
            stamp_state: self.stamp_state.clone(),
            time: self.time,
            timestep: self.timestep,
            ddt_coefficients: self.ddt_coefficients,
            canonical_reactive: self.canonical_reactive.clone(),
            canonical_model_values: self.canonical_model_values.clone(),
            canonical_staged: self.canonical_staged.clone(),
            canonical_instance_valid: self.canonical_instance_valid,
            canonical_temperature_valid: self.canonical_temperature_valid,
            canonical_temperature: self.canonical_temperature,
            canonical_thermal_voltage: self.canonical_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 6;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["di", "si", "gi", "gm", "bi", "bi2", "N1", "N2"];

    pub const BRANCH_COUNT: usize = 12;
    pub const PARAMETER_COUNT: usize = 1401;
    pub const VARIABLE_COUNT: usize = 2224;
    pub const DDT_STATE_COUNT: usize = 23;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "c12f61706bdd43180981229b92887455097b72d7e7401674c0574b861f7d60f4";
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::new_box(),
            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),
            multiplicity: 1.0,
            stamp_state: StampState::new_box(),
            time: 0.0,
            timestep: 0.0,
            ddt_coefficients: GeneratedDdtCoefficients::inactive(),
            canonical_reactive: boxed_zero_f64_array(),
            canonical_model_values: None,
            canonical_staged: boxed_zero_f64_array(),
            canonical_instance_valid: false,
            canonical_temperature_valid: false,
            canonical_temperature: 0.0,
            canonical_thermal_voltage: 0.0,
        }
    }

    #[doc(hidden)]
    pub fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(115);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(23);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    #[doc(hidden)]
    pub fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 115);
        debug_assert_eq!(state.flags.len(), 23);
        let mut rollback_values = state.values.as_slice();
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_current.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_previous.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_older.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_derivative_current.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_derivative_previous.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::IDT_STATE_COUNT);
        self.stamp_state.idt_current.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::IDT_STATE_COUNT);
        self.stamp_state.idt_previous.copy_from_slice(field);
        rollback_values = remaining;
        let mut rollback_flags = state.flags.as_slice();
        let (field, remaining) = rollback_flags.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_initialized.copy_from_slice(field);
        rollback_flags = remaining;
        let (field, remaining) = rollback_flags.split_at(Self::IDT_STATE_COUNT);
        self.stamp_state.idt_initialized.copy_from_slice(field);
        rollback_flags = remaining;
        debug_assert!(rollback_values.is_empty());
        debug_assert!(rollback_flags.is_empty());
    }

    #[doc(hidden)]
    pub fn capture_persistent_state(&self) -> GeneratedVerilogAPersistentState {
        GeneratedVerilogAPersistentState {
            ddt_previous: self.stamp_state.ddt_previous.to_vec(),
            ddt_older: self.stamp_state.ddt_older.to_vec(),
            ddt_derivative_previous: self.stamp_state.ddt_derivative_previous.to_vec(),
            ddt_initialized: self.stamp_state.ddt_initialized.to_vec(),
            idt_previous: self.stamp_state.idt_previous.to_vec(),
            idt_initialized: self.stamp_state.idt_initialized.to_vec(),
            limiter_anchor: Vec::new(),
            limiter_initialized: Vec::new(),
        }
    }

    #[doc(hidden)]
    pub fn validate_persistent_state_shape(&self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
        if state.ddt_previous.len() != Self::DDT_STATE_COUNT || state.ddt_older.len() != Self::DDT_STATE_COUNT || state.ddt_derivative_previous.len() != Self::DDT_STATE_COUNT || state.ddt_initialized.len() != Self::DDT_STATE_COUNT {
            return Err(format!("generated ddt checkpoint shape mismatch: expected {}, found {} / {} / {} / {}", Self::DDT_STATE_COUNT, state.ddt_previous.len(), state.ddt_older.len(), state.ddt_derivative_previous.len(), state.ddt_initialized.len()));
        }
        if state.idt_previous.len() != Self::IDT_STATE_COUNT || state.idt_initialized.len() != Self::IDT_STATE_COUNT {
            return Err(format!("generated idt checkpoint shape mismatch: expected {}, found {} / {}", Self::IDT_STATE_COUNT, state.idt_previous.len(), state.idt_initialized.len()));
        }
        if state.ddt_previous.iter().chain(&state.ddt_older).chain(&state.ddt_derivative_previous).chain(&state.idt_previous).chain(&state.limiter_anchor).any(|value| !value.is_finite()) {
            return Err("generated Verilog-A checkpoint contains non-finite persistent state".to_string());
        }
        Ok(())
    }

    #[doc(hidden)]
    pub fn restore_persistent_state(&mut self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
        self.validate_persistent_state_shape(state)?;
        self.stamp_state.ddt_previous.copy_from_slice(&state.ddt_previous);
        self.stamp_state.ddt_current.copy_from_slice(&state.ddt_previous);
        self.stamp_state.ddt_older.copy_from_slice(&state.ddt_older);
        self.stamp_state.ddt_derivative_previous.copy_from_slice(&state.ddt_derivative_previous);
        self.stamp_state.ddt_derivative_current.copy_from_slice(&state.ddt_derivative_previous);
        self.stamp_state.ddt_initialized.copy_from_slice(&state.ddt_initialized);
        self.stamp_state.idt_previous.copy_from_slice(&state.idt_previous);
        self.stamp_state.idt_current.copy_from_slice(&state.idt_previous);
        self.stamp_state.idt_initialized.copy_from_slice(&state.idt_initialized);
        Ok(())
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        let lower = name.to_ascii_lowercase();
        let Some(index) = parameter_index_for_name(lower.as_str()) else {
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimsoi'", name));
        };
        validate_parameter_scalar_metadata(index, value)?;
        let was_given = self.param_given[index];
        let value_changed = self.write_parameter_slot(index, value);
        self.finish_set_parameter(index, value_changed || !was_given);
        Ok(())
    }

    /// Validate the complete parameter vector after applying all instance overrides.
    pub fn validate_parameters(&self) -> Result<(), String> {
        for index in 0..Self::PARAMETER_COUNT {
            let value = read_parameter_slot(self.params.as_ref(), index);
            validate_parameter_metadata(self.params.as_ref(), index, value)?;
        }
        Ok(())
    }

    #[inline]
    fn write_parameter_slot(&mut self, index: usize, value: f64) -> bool {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        let slot = &mut self.params.values[index];
        let changed = slot.to_bits() != value.to_bits();
        *slot = value;
        changed
    }

    #[inline]
    fn finish_set_parameter(&mut self, index: usize, invalidates_caches: bool) {
        self.mark_param_given(index);
        if invalidates_caches {
            if PARAMETER_MODEL_FLAGS[index] {
                self.canonical_model_values = None;
            }
            self.canonical_instance_valid = false;
            self.canonical_temperature_valid = false;
        }
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) -> Result<(), String> {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            let changed = self.multiplicity.to_bits() != multiplicity.to_bits();
            self.multiplicity = multiplicity;
            if changed {
                self.canonical_instance_valid = false;
                self.canonical_temperature_valid = false;
            }
            Ok(())
        } else {
            Err(format!("instance multiplicity 'm' must be finite and > 0.0, got {}", multiplicity))
        }
    }

    #[inline]
    pub fn set_timepoint(&mut self, time: f64, timestep: f64, ddt_coefficients: GeneratedDdtCoefficients) {
        self.time = time;
        self.timestep = timestep;
        self.ddt_coefficients = ddt_coefficients;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.stamp_state.ddt_older[index] = self.stamp_state.ddt_previous[index];
            self.stamp_state.ddt_previous[index] = self.stamp_state.ddt_current[index];
            self.stamp_state.ddt_derivative_previous[index] = self.stamp_state.ddt_derivative_current[index];
            self.stamp_state.ddt_initialized[index] = true;
            index += 1;
        }
        let mut index = 0usize;
        while index < Self::IDT_STATE_COUNT {
            self.stamp_state.idt_previous[index] = self.stamp_state.idt_current[index];
            self.stamp_state.idt_initialized[index] = true;
            index += 1;
        }
    }

    #[inline]
    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {
        debug_assert!(slot < Self::DDT_STATE_COUNT, "generated ddt state slot out of range");
        let previous = if self.stamp_state.ddt_initialized[slot] {
            self.stamp_state.ddt_previous[slot]
        } else {
            value
        };
        let older = if self.stamp_state.ddt_initialized[slot] {
            self.stamp_state.ddt_older[slot]
        } else {
            value
        };
        self.stamp_state.ddt_current[slot] = value;
        if self.ddt_coefficients.active {
            let result = value * self.ddt_coefficients.derivative_scale
                - previous * self.ddt_coefficients.previous_value_scale
                - older * self.ddt_coefficients.older_value_scale
                - self.stamp_state.ddt_derivative_previous[slot] * self.ddt_coefficients.previous_derivative_scale;
            self.stamp_state.ddt_derivative_current[slot] = result;
            result
        } else {
            self.stamp_state.ddt_current[slot] = value;
            self.stamp_state.ddt_previous[slot] = value;
            self.stamp_state.ddt_older[slot] = value;
            self.stamp_state.ddt_derivative_current[slot] = 0.0;
            self.stamp_state.ddt_derivative_previous[slot] = 0.0;
            self.stamp_state.ddt_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.ddt_coefficients.active {
            derivative * self.ddt_coefficients.derivative_scale
        } else {
            0.0
        }
    }
    #[inline]
    pub fn limiter_converged(&self) -> bool {
        true
    }
}
