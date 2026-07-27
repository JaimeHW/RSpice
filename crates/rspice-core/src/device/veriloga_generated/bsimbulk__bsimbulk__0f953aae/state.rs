#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 1138],
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
            const DEFAULTS_0: [f64; 30] = [
                1e-5, 1e-5, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 50.0, 50.0, 50.0, 50.0,
                50.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 30);
            {
                let params = &mut *ptr;
                params[30] = params[28];
                validate_parameter("MULT_FN", params[30], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 47] = [
                0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-5, 1.0, 1.0, 0.0,
                1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(31), 47);
            {
                let params = &mut *ptr;
                params[78] = params[77];
                validate_parameter("TOXP", params[78], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 13] = [
                0.0, 1e24, 0.0, 1.0, 0.0, 2.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(79), 13);
            {
                let params = &mut *ptr;
                params[92] = params[80];
                validate_finite_parameter("NDEPCV", params[92]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[93] = params[81];
                validate_finite_parameter("NDEPCVL1", params[93]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[94] = params[82];
                validate_parameter("NDEPCVLEXP1", params[94], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[95] = params[83];
                validate_finite_parameter("NDEPCVL2", params[95]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[96] = params[84];
                validate_parameter("NDEPCVLEXP2", params[96], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[97] = params[85];
                validate_finite_parameter("NDEPCVW", params[97]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[98] = params[86];
                validate_parameter("NDEPCVWEXP", params[98], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[99] = params[87];
                validate_finite_parameter("NDEPCVWL", params[99]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[100] = params[88];
                validate_parameter("NDEPCVWLEXP", params[100], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[101] = params[89];
                validate_finite_parameter("LNDEPCV", params[101]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[102] = params[90];
                validate_finite_parameter("WNDEPCV", params[102]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[103] = params[91];
                validate_finite_parameter("PNDEPCV", params[103]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 22] = [
                5e25, 0.0, 0.0, 0.0, 1.1e16, 1.17, 11.9, 3.9,
                1.5e-7, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(104), 22);
            {
                let params = &mut *ptr;
                params[126] = params[116];
                validate_finite_parameter("VFBCV", params[126]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[127] = params[117];
                validate_finite_parameter("LVFBCV", params[127]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[128] = params[118];
                validate_finite_parameter("WVFBCV", params[128]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[129] = params[119];
                validate_finite_parameter("PVFBCV", params[129]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[130] = params[120];
                validate_finite_parameter("VFBCVL", params[130]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[131] = params[121];
                validate_parameter("VFBCVLEXP", params[131], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[132] = params[122];
                validate_finite_parameter("VFBCVW", params[132]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[133] = params[123];
                validate_parameter("VFBCVWEXP", params[133], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[134] = params[124];
                validate_finite_parameter("VFBCVWL", params[134]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[135] = params[125];
                validate_parameter("VFBCVWLEXP", params[135], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(136), 2);
            {
                let params = &mut *ptr;
                params[138] = params[73];
                validate_finite_parameter("DWJ", params[138]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 36] = [
                1e26, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.045, 0.0, 0.0, 0.0,
                0.08, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(139), 36);
            {
                let params = &mut *ptr;
                params[175] = params[171];
                validate_finite_parameter("ETA0R", params[175]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[176] = params[172];
                validate_finite_parameter("LETA0R", params[176]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[177] = params[173];
                validate_finite_parameter("WETA0R", params[177]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[178] = params[174];
                validate_finite_parameter("PETA0R", params[178]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 50] = [
                1.0, -0.07, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.001, 0.54, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 1e-9, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(179), 50);
            {
                let params = &mut *ptr;
                params[229] = params[223];
                validate_finite_parameter("CDSCDR", params[229]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[230] = params[226];
                validate_finite_parameter("LCDSCDR", params[230]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[231] = params[227];
                validate_finite_parameter("WCDSCDR", params[231]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[232] = params[228];
                validate_finite_parameter("PCDSCDR", params[232]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 16] = [
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 100000.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(233), 16);
            {
                let params = &mut *ptr;
                params[249] = params[239];
                validate_finite_parameter("VSATR", params[249]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[250] = params[240];
                validate_finite_parameter("LVSATR", params[250]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[251] = params[241];
                validate_finite_parameter("WVSATR", params[251]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[252] = params[242];
                validate_finite_parameter("PVSATR", params[252]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 6] = [
                0.125, 0.0, 0.0, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(253), 6);
            {
                let params = &mut *ptr;
                params[259] = params[239];
                validate_finite_parameter("VSATCV", params[259]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[260] = params[240];
                validate_finite_parameter("LVSATCV", params[260]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[261] = params[241];
                validate_finite_parameter("WVSATCV", params[261]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[262] = params[242];
                validate_finite_parameter("PVSATCV", params[262]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[263] = params[243];
                validate_finite_parameter("VSATCVL", params[263]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[264] = params[244];
                validate_parameter("VSATCVLEXP", params[264], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[265] = params[245];
                validate_finite_parameter("VSATCVW", params[265]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[266] = params[246];
                validate_parameter("VSATCVWEXP", params[266], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[267] = params[247];
                validate_finite_parameter("VSATCVWL", params[267]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[268] = params[248];
                validate_parameter("VSATCVWLEXP", params[268], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 10] = [
                0.0, 1e-8, 0.0, 1e-8, 0.067, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(269), 10);
            {
                let params = &mut *ptr;
                params[279] = params[273];
                validate_finite_parameter("U0R", params[279]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[280] = params[276];
                validate_finite_parameter("LU0R", params[280]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[281] = params[277];
                validate_finite_parameter("WU0R", params[281]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[282] = params[278];
                validate_finite_parameter("PU0R", params[282]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 11] = [
                1.0, 0.001, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(283), 11);
            {
                let params = &mut *ptr;
                params[294] = params[284];
                validate_finite_parameter("UAR", params[294]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[295] = params[291];
                validate_finite_parameter("LUAR", params[295]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[296] = params[292];
                validate_finite_parameter("WUAR", params[296]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[297] = params[293];
                validate_finite_parameter("PUAR", params[297]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 16] = [
                1.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.001, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(298), 16);
            {
                let params = &mut *ptr;
                params[314] = params[308];
                validate_finite_parameter("UDR", params[314]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[315] = params[311];
                validate_finite_parameter("LUDR", params[315]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[316] = params[312];
                validate_finite_parameter("WUDR", params[316]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[317] = params[313];
                validate_finite_parameter("PUDR", params[317]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 4] = [
                2.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(318), 4);
            {
                let params = &mut *ptr;
                params[322] = params[318];
                validate_finite_parameter("UCSR", params[322]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[323] = params[319];
                validate_finite_parameter("LUCSR", params[323]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[324] = params[320];
                validate_finite_parameter("WUCSR", params[324]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[325] = params[321];
                validate_finite_parameter("PUCSR", params[325]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 10] = [
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(326), 10);
            {
                let params = &mut *ptr;
                params[336] = params[326];
                validate_finite_parameter("UCR", params[336]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[337] = params[333];
                validate_finite_parameter("LUCR", params[337]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[338] = params[334];
                validate_finite_parameter("WUCR", params[338]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[339] = params[335];
                validate_finite_parameter("PUCR", params[339]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 6] = [
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(340), 6);
            {
                let params = &mut *ptr;
                params[346] = params[340];
                validate_finite_parameter("PCLMR", params[346]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[347] = params[343];
                validate_finite_parameter("LPCLMR", params[347]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[348] = params[344];
                validate_finite_parameter("WPCLMR", params[348]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[349] = params[345];
                validate_finite_parameter("PPCLMR", params[349]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(350), 1);
            {
                let params = &mut *ptr;
                params[351] = params[340];
                validate_finite_parameter("PCLMCV", params[351]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[352] = params[341];
                validate_finite_parameter("PCLMCVL", params[352]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[353] = params[342];
                validate_parameter("PCLMCVLEXP", params[353], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[354] = params[343];
                validate_finite_parameter("LPCLMCV", params[354]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[355] = params[344];
                validate_finite_parameter("WPCLMCV", params[355]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[356] = params[345];
                validate_finite_parameter("PPCLMCV", params[356]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 42] = [
                424000000.0, 0.0, 0.0, 0.0, 1e-8, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(357), 42);
            {
                let params = &mut *ptr;
                params[399] = params[389];
                validate_finite_parameter("RDWMIN", params[399]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[400] = params[390];
                validate_finite_parameter("LRDWMIN", params[400]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[401] = params[391];
                validate_finite_parameter("WRDWMIN", params[401]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[402] = params[392];
                validate_finite_parameter("PRDWMIN", params[402]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[403] = params[393];
                validate_finite_parameter("RDW", params[403]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[404] = params[394];
                validate_finite_parameter("LRDW", params[404]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[405] = params[395];
                validate_finite_parameter("WRDW", params[405]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[406] = params[396];
                validate_finite_parameter("PRDW", params[406]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[407] = params[397];
                validate_finite_parameter("RDWL", params[407]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[408] = params[398];
                validate_parameter("RDWLEXP", params[408], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 17] = [
                0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(409), 17);
            {
                let params = &mut *ptr;
                params[426] = params[419];
                validate_finite_parameter("PSATR", params[426]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[427] = params[420];
                validate_finite_parameter("LPSATR", params[427]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[428] = params[421];
                validate_finite_parameter("WPSATR", params[428]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[429] = params[422];
                validate_finite_parameter("PPSATR", params[429]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 10] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(430), 10);
            {
                let params = &mut *ptr;
                params[440] = params[434];
                validate_finite_parameter("PTWGR", params[440]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[441] = params[435];
                validate_finite_parameter("LPTWGR", params[441]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[442] = params[436];
                validate_finite_parameter("WPTWGR", params[442]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[443] = params[437];
                validate_finite_parameter("PPTWGR", params[443]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 22] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(444), 22);
            {
                let params = &mut *ptr;
                params[466] = params[460];
                validate_finite_parameter("PDIBLCR", params[466]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[467] = params[463];
                validate_finite_parameter("LPDIBLCR", params[467]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[468] = params[464];
                validate_finite_parameter("WPDIBLCR", params[468]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[469] = params[465];
                validate_finite_parameter("PPDIBLCR", params[469]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 30] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(470), 30);
            {
                let params = &mut *ptr;
                params[500] = params[484];
                validate_finite_parameter("ALPHADR", params[500]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[501] = params[494];
                validate_finite_parameter("BETADR", params[501]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 23] = [
                1.0, 5.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(502), 23);
            {
                let params = &mut *ptr;
                params[525] = params[484];
                validate_finite_parameter("ALPHA0R", params[525]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[526] = params[489];
                validate_finite_parameter("LALPHA0R", params[526]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[527] = params[490];
                validate_finite_parameter("WALPHA0R", params[527]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[528] = params[491];
                validate_finite_parameter("PALPHA0R", params[528]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[529] = params[494];
                validate_finite_parameter("BETA0R", params[529]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[530] = params[497];
                validate_finite_parameter("LBETA0R", params[530]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[531] = params[498];
                validate_finite_parameter("WBETA0R", params[531]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[532] = params[499];
                validate_finite_parameter("PBETA0R", params[532]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 9] = [
                0.0136, 0.00171, 0.075, 1.0, 0.0111, 0.000949, 0.006, 1.1,
                3.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(533), 9);
            {
                let params = &mut *ptr;
                params[542] = if (params[39] == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGC", params[542]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[543] = if (params[39] == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGC", params[543]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[544] = if (params[39] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params[544]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[545] = if (params[39] == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGS", params[545]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[546] = if (params[39] == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGS", params[546]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[547] = if (params[39] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGS", params[547]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[548] = if (params[39] == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGD", params[548]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[549] = if (params[39] == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGD", params[549]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[550] = if (params[39] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGD", params[550]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[551] = params[57];
                validate_finite_parameter("DLCIG", params[551]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[552] = params[551];
                validate_finite_parameter("DLCIGD", params[552]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 95] = [
                1.0, 1.0, 3e-9, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 2300000000.0, 0.0, 0.0, 0.0, 0.5,
                0.0, 0.0, 0.0, 0.8, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(553), 95);
            {
                let params = &mut *ptr;
                params[648] = params[630];
                validate_finite_parameter("AGISL", params[648]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[649] = params[631];
                validate_finite_parameter("AGISLL", params[649]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[650] = params[632];
                validate_finite_parameter("AGISLW", params[650]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[651] = params[633];
                validate_finite_parameter("LAGISL", params[651]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[652] = params[634];
                validate_finite_parameter("WAGISL", params[652]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[653] = params[635];
                validate_finite_parameter("PAGISL", params[653]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[654] = params[636];
                validate_finite_parameter("BGISL", params[654]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[655] = params[637];
                validate_finite_parameter("LBGISL", params[655]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[656] = params[638];
                validate_finite_parameter("WBGISL", params[656]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[657] = params[639];
                validate_finite_parameter("PBGISL", params[657]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[658] = params[640];
                validate_finite_parameter("CGISL", params[658]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[659] = params[641];
                validate_finite_parameter("LCGISL", params[659]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[660] = params[642];
                validate_finite_parameter("WCGISL", params[660]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[661] = params[643];
                validate_finite_parameter("PCGISL", params[661]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[662] = params[644];
                validate_finite_parameter("EGISL", params[662]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[663] = params[645];
                validate_finite_parameter("LEGISL", params[663]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[664] = params[646];
                validate_finite_parameter("WEGISL", params[664]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[665] = params[647];
                validate_finite_parameter("PEGISL", params[665]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 30] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.6, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0, 0.0,
                1000000.0, 1.0, 1000000.0, 1.0, 0.1, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(666), 30);
            {
                let params = &mut *ptr;
                params[696] = params[695];
                validate_parameter("DMCI", params[696], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 5] = [
                0.0, 0.0, 0.0, 0.1, 0.0005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(697), 5);
            {
                let params = &mut *ptr;
                params[702] = params[701];
                validate_finite_parameter("CJD", params[702]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 1] = [
                5e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(703), 1);
            {
                let params = &mut *ptr;
                params[704] = params[703];
                validate_finite_parameter("CJSWD", params[704]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(705), 1);
            {
                let params = &mut *ptr;
                params[706] = params[705];
                validate_finite_parameter("CJSWGD", params[706]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (*ptr).values.as_mut_ptr().add(707), 1);
            {
                let params = &mut *ptr;
                params[708] = params[707];
                validate_finite_parameter("PBD", params[708]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (*ptr).values.as_mut_ptr().add(709), 1);
            {
                let params = &mut *ptr;
                params[710] = params[709];
                validate_finite_parameter("PBSWD", params[710]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[711] = params[709];
                validate_finite_parameter("PBSWGS", params[711]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[712] = params[711];
                validate_finite_parameter("PBSWGD", params[712]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (*ptr).values.as_mut_ptr().add(713), 1);
            {
                let params = &mut *ptr;
                params[714] = params[713];
                validate_finite_parameter("MJD", params[714]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                0.33,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (*ptr).values.as_mut_ptr().add(715), 1);
            {
                let params = &mut *ptr;
                params[716] = params[715];
                validate_finite_parameter("MJSWD", params[716]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[717] = params[715];
                validate_finite_parameter("MJSWGS", params[717]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[718] = params[717];
                validate_finite_parameter("MJSWGD", params[718]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                0.0001,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (*ptr).values.as_mut_ptr().add(719), 1);
            {
                let params = &mut *ptr;
                params[720] = params[719];
                validate_finite_parameter("JSD", params[720]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (*ptr).values.as_mut_ptr().add(721), 1);
            {
                let params = &mut *ptr;
                params[722] = params[721];
                validate_finite_parameter("JSWD", params[722]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (*ptr).values.as_mut_ptr().add(723), 1);
            {
                let params = &mut *ptr;
                params[724] = params[723];
                validate_finite_parameter("JSWGD", params[724]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (*ptr).values.as_mut_ptr().add(725), 1);
            {
                let params = &mut *ptr;
                params[726] = params[725];
                validate_parameter("NJD", params[726], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (*ptr).values.as_mut_ptr().add(727), 1);
            {
                let params = &mut *ptr;
                params[728] = params[727];
                validate_finite_parameter("IJTHDFWD", params[728]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (*ptr).values.as_mut_ptr().add(729), 1);
            {
                let params = &mut *ptr;
                params[730] = params[729];
                validate_finite_parameter("IJTHDREV", params[730]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (*ptr).values.as_mut_ptr().add(731), 1);
            {
                let params = &mut *ptr;
                params[732] = params[731];
                validate_finite_parameter("BVD", params[732]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (*ptr).values.as_mut_ptr().add(733), 1);
            {
                let params = &mut *ptr;
                params[734] = params[733];
                validate_parameter("XJBVD", params[734], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (*ptr).values.as_mut_ptr().add(735), 1);
            {
                let params = &mut *ptr;
                params[736] = params[735];
                validate_finite_parameter("JTSD", params[736]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (*ptr).values.as_mut_ptr().add(737), 1);
            {
                let params = &mut *ptr;
                params[738] = params[737];
                validate_finite_parameter("JTSSWD", params[738]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (*ptr).values.as_mut_ptr().add(739), 1);
            {
                let params = &mut *ptr;
                params[740] = params[739];
                validate_finite_parameter("JTSSWGD", params[740]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 2] = [
                0.0, 20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (*ptr).values.as_mut_ptr().add(741), 2);
            {
                let params = &mut *ptr;
                params[743] = params[742];
                validate_finite_parameter("NJTSD", params[743]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (*ptr).values.as_mut_ptr().add(744), 1);
            {
                let params = &mut *ptr;
                params[745] = params[744];
                validate_finite_parameter("NJTSSWD", params[745]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (*ptr).values.as_mut_ptr().add(746), 1);
            {
                let params = &mut *ptr;
                params[747] = params[746];
                validate_finite_parameter("NJTSSWGD", params[747]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (*ptr).values.as_mut_ptr().add(748), 1);
            {
                let params = &mut *ptr;
                params[749] = params[748];
                validate_finite_parameter("VTSD", params[749]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (*ptr).values.as_mut_ptr().add(750), 1);
            {
                let params = &mut *ptr;
                params[751] = params[750];
                validate_finite_parameter("VTSSWD", params[751]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (*ptr).values.as_mut_ptr().add(752), 1);
            {
                let params = &mut *ptr;
                params[753] = params[752];
                validate_finite_parameter("VTSSWGD", params[753]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 142] = [
                12.0, 1.0, 1e-12, 50.0, 0.0, 0.0, 0.0, 50.0,
                0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 100.0,
                0.0, 0.0, 0.0, 100.0, 100.0, 100.0, 100.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 41000000.0, 6.25e40,
                0.0, 0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0,
                0.05, 0.0, 0.0, 0.0, 2.0, 3.125e25, 875000000.0, 0.0,
                0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 2.0, 2.0,
                1.0, 0.577, 0.5164, 0.395, 1.5, 3.5, 0.0, 1.0,
                0.0, 0.0, 27.0, 0.000473, 636.0, 0.0, -1.5, 0.0,
                0.0, 0.0, 0.0, 0.001, 0.0, 0.0, 0.0, 0.0,
                5.6e-11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, -0.004775, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.00156, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, -0.11, 1.0, 0.0, 0.0, 0.0, 0.0, 0.022,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (*ptr).values.as_mut_ptr().add(754), 142);
            {
                let params = &mut *ptr;
                params[896] = params[895];
                validate_finite_parameter("XTID", params[896]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (*ptr).values.as_mut_ptr().add(897), 1);
            {
                let params = &mut *ptr;
                params[898] = params[897];
                validate_finite_parameter("XTSD", params[898]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (*ptr).values.as_mut_ptr().add(899), 1);
            {
                let params = &mut *ptr;
                params[900] = params[899];
                validate_finite_parameter("XTSSWD", params[900]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (*ptr).values.as_mut_ptr().add(901), 1);
            {
                let params = &mut *ptr;
                params[902] = params[901];
                validate_finite_parameter("XTSSWGD", params[902]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (*ptr).values.as_mut_ptr().add(903), 1);
            {
                let params = &mut *ptr;
                params[904] = params[903];
                validate_finite_parameter("TNJTSD", params[904]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (*ptr).values.as_mut_ptr().add(905), 1);
            {
                let params = &mut *ptr;
                params[906] = params[905];
                validate_finite_parameter("TNJTSSWD", params[906]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (*ptr).values.as_mut_ptr().add(907), 1);
            {
                let params = &mut *ptr;
                params[908] = params[907];
                validate_finite_parameter("TNJTSSWGD", params[908]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 158] = [
                0.0, 1e-5, 0.0, 1e-6, 1e-6, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-6, 400.0,
                336000000.0, 0.185, 0.3, 1.4, 0.0, 0.49, 1.42, 20.0,
                1e-8, 0.0, 0.0, 1.0, 0.0, 1e24, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.08, 0.0, 0.0, 0.0, -0.07, 0.0, 0.0,
                0.0, -0.11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.022, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.2, 0.53, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1e-5, 0.0, 0.0,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (*ptr).values.as_mut_ptr().add(909), 158);
            {
                let params = &mut *ptr;
                params[1067] = params[785];
                validate_finite_parameter("NOIA2", params[1067]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1068] = params[80];
                validate_parameter("HNDEP", params[1068], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (*ptr).values.as_mut_ptr().add(1069), 24);
            {
                let params = &mut *ptr;
                params[1093] = 0.001;
                validate_parameter("minr", params[1093], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 10] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 5e16, 100000.0, 0.0,
                0.0, 60.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (*ptr).values.as_mut_ptr().add(1094), 10);
            {
                let params = &mut *ptr;
                params[1104] = params[1101];
                validate_parameter("PTWGHVII", params[1104], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1105] = params[1102];
                validate_finite_parameter("PTWGHV1II", params[1105]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1106] = params[1103];
                validate_parameter("PSATXHVII", params[1106], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 2] = [
                1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (*ptr).values.as_mut_ptr().add(1107), 2);
            {
                let params = &mut *ptr;
                params[1109] = params[1099];
                validate_parameter("NDRIFTS", params[1109], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                100.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (*ptr).values.as_mut_ptr().add(1110), 1);
            {
                let params = &mut *ptr;
                params[1111] = params[1110];
                validate_parameter("RDLCWCV", params[1111], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 4] = [
                0.0, 0.0, -1.0, 5.000000000000001e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (*ptr).values.as_mut_ptr().add(1112), 4);
            {
                let params = &mut *ptr;
                params[1116] = params[1115];
                validate_finite_parameter("LOVERACC", params[1116]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1117] = params[80];
                validate_parameter("NDR", params[1117], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 16] = [
                0.0, 1.0, 0.0, 0.0, 0.001, 0.6, 0.0, 0.0,
                8.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (*ptr).values.as_mut_ptr().add(1118), 16);
            {
                let params = &mut *ptr;
                params[1134] = params[1130];
                validate_finite_parameter("A0CV", params[1134]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1135] = params[1131];
                validate_finite_parameter("AGSCV", params[1135]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1136] = params[1133];
                validate_parameter("KETACV", params[1136], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (*ptr).values.as_mut_ptr().add(1137), 1);
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

#[derive(Copy, Clone)]
struct ParameterBound {
    value: f64,
    label: &'static str,
}

const PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
const PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

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

fn validate_parameter_bounds(
    name: &str,
    value: f64,
    flags: u8,
    min: Option<ParameterBound>,
    max: Option<ParameterBound>,
    excluded: &[ParameterBound],
) -> Result<(), String> {
    if let Some(min) = min {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = max {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in excluded {
        if value == excluded.value {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
        }
    }
    Ok(())
}

fn validate_finite_parameter(name: &str, value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter '{}' must be finite, got {}", name, value));
    }
    Ok(())
}

fn validate_parameter(
    name: &str,
    value: f64,
    integer: bool,
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
    if integer && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    if let Some((min, label)) = min {
        if min_exclusive {
            if value <= min {
                return Err(format!("parameter '{}' must be > {}, got {}", name, label, value));
            }
        } else if value < min {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, label, value));
        }
    }
    if let Some((max, label)) = max {
        if max_exclusive {
            if value >= max {
                return Err(format!("parameter '{}' must be < {}, got {}", name, label, value));
            }
        } else if value > max {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, label, value));
        }
    }
    for (excluded, label) in excluded {
        if value == *excluded {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, label, value));
        }
    }
    Ok(())
}

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1138] = [
    ("l", 0), ("w", 1), ("nf", 2), ("nrs", 3), ("nrd", 4), ("vfbsdoff", 5), ("minz", 6), ("rgatemod", 7), ("rbodymod", 8), ("geomod", 9), ("rgeomod", 10), ("rbpb", 11), ("rbpd", 12), ("rbps", 13), ("rbdb", 14), ("rbsb", 15),
    ("rdb", 16), ("sa", 17), ("sb", 18), ("sd", 19), ("sca", 20), ("scb", 21), ("scc", 22), ("sc", 23), ("as", 24), ("ad", 25), ("ps", 26), ("pd", 27), ("mult_i", 28), ("mult_q", 29), ("mult_fn", 30), ("xgw", 31),
    ("ngcon", 32), ("dtemp", 33), ("mulu0", 34), ("delvto", 35), ("ids0mult", 36), ("edgefet", 37), ("sslmod", 38), ("type", 39), ("cvmod", 40), ("covmod", 41), ("rdsmod", 42), ("wpemod", 43), ("asymmod", 44), ("gidlmod", 45), ("igcmod", 46), ("igbmod", 47),
    ("tnoimod", 48), ("shmod", 49), ("mobscale", 50), ("llong", 51), ("lmlt", 52), ("wmlt", 53), ("xl", 54), ("wwide", 55), ("xw", 56), ("lint", 57), ("ll", 58), ("lw", 59), ("lwl", 60), ("lln", 61), ("lwn", 62), ("wint", 63),
    ("wl", 64), ("ww", 65), ("wwl", 66), ("wln", 67), ("wwn", 68), ("dlc", 69), ("llc", 70), ("lwc", 71), ("lwlc", 72), ("dwc", 73), ("wlc", 74), ("wwc", 75), ("wwlc", 76), ("toxe", 77), ("toxp", 78), ("dtox", 79),
    ("ndep", 80), ("ndepl1", 81), ("ndeplexp1", 82), ("ndepl2", 83), ("ndeplexp2", 84), ("ndepw", 85), ("ndepwexp", 86), ("ndepwl", 87), ("ndepwlexp", 88), ("lndep", 89), ("wndep", 90), ("pndep", 91), ("ndepcv", 92), ("ndepcvl1", 93), ("ndepcvlexp1", 94), ("ndepcvl2", 95),
    ("ndepcvlexp2", 96), ("ndepcvw", 97), ("ndepcvwexp", 98), ("ndepcvwl", 99), ("ndepcvwlexp", 100), ("lndepcv", 101), ("wndepcv", 102), ("pndepcv", 103), ("ngate", 104), ("lngate", 105), ("wngate", 106), ("pngate", 107), ("ni0sub", 108), ("bg0sub", 109), ("epsrsub", 110), ("epsrox", 111),
    ("xj", 112), ("lxj", 113), ("wxj", 114), ("pxj", 115), ("vfb", 116), ("lvfb", 117), ("wvfb", 118), ("pvfb", 119), ("vfbl", 120), ("vfblexp", 121), ("vfbw", 122), ("vfbwexp", 123), ("vfbwl", 124), ("vfbwlexp", 125), ("vfbcv", 126), ("lvfbcv", 127),
    ("wvfbcv", 128), ("pvfbcv", 129), ("vfbcvl", 130), ("vfbcvlexp", 131), ("vfbcvw", 132), ("vfbcvwexp", 133), ("vfbcvwl", 134), ("vfbcvwlexp", 135), ("delvfbacc", 136), ("permod", 137), ("dwj", 138), ("nsd", 139), ("lnsd", 140), ("wnsd", 141), ("pnsd", 142), ("dvtp0", 143),
    ("ldvtp0", 144), ("wdvtp0", 145), ("pdvtp0", 146), ("dvtp1", 147), ("ldvtp1", 148), ("wdvtp1", 149), ("pdvtp1", 150), ("dvtp2", 151), ("ldvtp2", 152), ("wdvtp2", 153), ("pdvtp2", 154), ("dvtp3", 155), ("ldvtp3", 156), ("wdvtp3", 157), ("pdvtp3", 158), ("dvtp4", 159),
    ("ldvtp4", 160), ("wdvtp4", 161), ("pdvtp4", 162), ("dvtp5", 163), ("ldvtp5", 164), ("wdvtp5", 165), ("pdvtp5", 166), ("phin", 167), ("lphin", 168), ("wphin", 169), ("pphin", 170), ("eta0", 171), ("leta0", 172), ("weta0", 173), ("peta0", 174), ("eta0r", 175),
    ("leta0r", 176), ("weta0r", 177), ("peta0r", 178), ("dsub", 179), ("etab", 180), ("etabexp", 181), ("letab", 182), ("wetab", 183), ("petab", 184), ("k1", 185), ("k1l", 186), ("k1lexp", 187), ("k1w", 188), ("k1wexp", 189), ("k1wl", 190), ("k1wlexp", 191),
    ("lk1", 192), ("wk1", 193), ("pk1", 194), ("k2", 195), ("k2l", 196), ("k2lexp", 197), ("k2w", 198), ("k2wexp", 199), ("k2wl", 200), ("k2wlexp", 201), ("lk2", 202), ("wk2", 203), ("pk2", 204), ("ados", 205), ("bdos", 206), ("qm0", 207),
    ("etaqm", 208), ("cit", 209), ("lcit", 210), ("wcit", 211), ("pcit", 212), ("nfactor", 213), ("nfactorl", 214), ("nfactorlexp", 215), ("nfactorw", 216), ("nfactorwexp", 217), ("nfactorwl", 218), ("nfactorwlexp", 219), ("lnfactor", 220), ("wnfactor", 221), ("pnfactor", 222), ("cdscd", 223),
    ("cdscdl", 224), ("cdscdlexp", 225), ("lcdscd", 226), ("wcdscd", 227), ("pcdscd", 228), ("cdscdr", 229), ("lcdscdr", 230), ("wcdscdr", 231), ("pcdscdr", 232), ("cdscb", 233), ("cdscbl", 234), ("cdscblexp", 235), ("lcdscb", 236), ("wcdscb", 237), ("pcdscb", 238), ("vsat", 239),
    ("lvsat", 240), ("wvsat", 241), ("pvsat", 242), ("vsatl", 243), ("vsatlexp", 244), ("vsatw", 245), ("vsatwexp", 246), ("vsatwl", 247), ("vsatwlexp", 248), ("vsatr", 249), ("lvsatr", 250), ("wvsatr", 251), ("pvsatr", 252), ("delta", 253), ("ldelta", 254), ("wdelta", 255),
    ("pdelta", 256), ("deltal", 257), ("deltalexp", 258), ("vsatcv", 259), ("lvsatcv", 260), ("wvsatcv", 261), ("pvsatcv", 262), ("vsatcvl", 263), ("vsatcvlexp", 264), ("vsatcvw", 265), ("vsatcvwexp", 266), ("vsatcvwl", 267), ("vsatcvwlexp", 268), ("up1", 269), ("lp1", 270), ("up2", 271),
    ("lp2", 272), ("u0", 273), ("u0l", 274), ("u0lexp", 275), ("lu0", 276), ("wu0", 277), ("pu0", 278), ("u0r", 279), ("lu0r", 280), ("wu0r", 281), ("pu0r", 282), ("etamob", 283), ("ua", 284), ("ual", 285), ("ualexp", 286), ("uaw", 287),
    ("uawexp", 288), ("uawl", 289), ("uawlexp", 290), ("lua", 291), ("wua", 292), ("pua", 293), ("uar", 294), ("luar", 295), ("wuar", 296), ("puar", 297), ("eu", 298), ("leu", 299), ("weu", 300), ("peu", 301), ("eul", 302), ("eulexp", 303),
    ("euw", 304), ("euwexp", 305), ("euwl", 306), ("euwlexp", 307), ("ud", 308), ("udl", 309), ("udlexp", 310), ("lud", 311), ("wud", 312), ("pud", 313), ("udr", 314), ("ludr", 315), ("wudr", 316), ("pudr", 317), ("ucs", 318), ("lucs", 319),
    ("wucs", 320), ("pucs", 321), ("ucsr", 322), ("lucsr", 323), ("wucsr", 324), ("pucsr", 325), ("uc", 326), ("ucl", 327), ("uclexp", 328), ("ucw", 329), ("ucwexp", 330), ("ucwl", 331), ("ucwlexp", 332), ("luc", 333), ("wuc", 334), ("puc", 335),
    ("ucr", 336), ("lucr", 337), ("wucr", 338), ("pucr", 339), ("pclm", 340), ("pclml", 341), ("pclmlexp", 342), ("lpclm", 343), ("wpclm", 344), ("ppclm", 345), ("pclmr", 346), ("lpclmr", 347), ("wpclmr", 348), ("ppclmr", 349), ("pclmg", 350), ("pclmcv", 351),
    ("pclmcvl", 352), ("pclmcvlexp", 353), ("lpclmcv", 354), ("wpclmcv", 355), ("ppclmcv", 356), ("pscbe1", 357), ("lpscbe1", 358), ("wpscbe1", 359), ("ppscbe1", 360), ("pscbe2", 361), ("lpscbe2", 362), ("wpscbe2", 363), ("ppscbe2", 364), ("pdits", 365), ("lpdits", 366), ("wpdits", 367),
    ("ppdits", 368), ("pditsl", 369), ("pditsd", 370), ("lpditsd", 371), ("wpditsd", 372), ("ppditsd", 373), ("rsh", 374), ("prwg", 375), ("lprwg", 376), ("wprwg", 377), ("pprwg", 378), ("prwb", 379), ("lprwb", 380), ("wprwb", 381), ("pprwb", 382), ("prwbl", 383),
    ("prwblexp", 384), ("wr", 385), ("lwr", 386), ("wwr", 387), ("pwr", 388), ("rswmin", 389), ("lrswmin", 390), ("wrswmin", 391), ("prswmin", 392), ("rsw", 393), ("lrsw", 394), ("wrsw", 395), ("prsw", 396), ("rswl", 397), ("rswlexp", 398), ("rdwmin", 399),
    ("lrdwmin", 400), ("wrdwmin", 401), ("prdwmin", 402), ("rdw", 403), ("lrdw", 404), ("wrdw", 405), ("prdw", 406), ("rdwl", 407), ("rdwlexp", 408), ("rdswmin", 409), ("lrdswmin", 410), ("wrdswmin", 411), ("prdswmin", 412), ("rdsw", 413), ("rdswl", 414), ("rdswlexp", 415),
    ("lrdsw", 416), ("wrdsw", 417), ("prdsw", 418), ("psat", 419), ("lpsat", 420), ("wpsat", 421), ("ppsat", 422), ("psatl", 423), ("psatlexp", 424), ("psatb", 425), ("psatr", 426), ("lpsatr", 427), ("wpsatr", 428), ("ppsatr", 429), ("lpsatb", 430), ("wpsatb", 431),
    ("ppsatb", 432), ("psatx", 433), ("ptwg", 434), ("lptwg", 435), ("wptwg", 436), ("pptwg", 437), ("ptwgl", 438), ("ptwglexp", 439), ("ptwgr", 440), ("lptwgr", 441), ("wptwgr", 442), ("pptwgr", 443), ("a1", 444), ("la1", 445), ("wa1", 446), ("pa1", 447),
    ("a11", 448), ("la11", 449), ("wa11", 450), ("pa11", 451), ("a2", 452), ("la2", 453), ("wa2", 454), ("pa2", 455), ("a21", 456), ("la21", 457), ("wa21", 458), ("pa21", 459), ("pdiblc", 460), ("pdiblcl", 461), ("pdiblclexp", 462), ("lpdiblc", 463),
    ("wpdiblc", 464), ("ppdiblc", 465), ("pdiblcr", 466), ("lpdiblcr", 467), ("wpdiblcr", 468), ("ppdiblcr", 469), ("pdiblcb", 470), ("lpdiblcb", 471), ("wpdiblcb", 472), ("ppdiblcb", 473), ("pvag", 474), ("lpvag", 475), ("wpvag", 476), ("ppvag", 477), ("fprout", 478), ("fproutl", 479),
    ("fproutlexp", 480), ("lfprout", 481), ("wfprout", 482), ("pfprout", 483), ("alpha0", 484), ("alpha0l", 485), ("alpha0lexp", 486), ("alpha0w", 487), ("alpha0wexp", 488), ("lalpha0", 489), ("walpha0", 490), ("palpha0", 491), ("alpha3", 492), ("alpha4", 493), ("beta0", 494), ("beta0w", 495),
    ("beta0wexp", 496), ("lbeta0", 497), ("wbeta0", 498), ("pbeta0", 499), ("alphadr", 500), ("betadr", 501), ("drii1", 502), ("drii2", 503), ("deltaii", 504), ("alpha1", 505), ("alpha2", 506), ("alphadr1", 507), ("alphadr2", 508), ("alphadr3", 509), ("alphadr4", 510), ("drexp", 511),
    ("drii3", 512), ("drii4", 513), ("cmd1", 514), ("cmd2", 515), ("cms1", 516), ("cms2", 517), ("beta1", 518), ("beta1w", 519), ("beta1wexp", 520), ("beta2", 521), ("beta2w", 522), ("beta2wexp", 523), ("beta3", 524), ("alpha0r", 525), ("lalpha0r", 526), ("walpha0r", 527),
    ("palpha0r", 528), ("beta0r", 529), ("lbeta0r", 530), ("wbeta0r", 531), ("pbeta0r", 532), ("aigbacc", 533), ("bigbacc", 534), ("cigbacc", 535), ("nigbacc", 536), ("aigbinv", 537), ("bigbinv", 538), ("cigbinv", 539), ("eigbinv", 540), ("nigbinv", 541), ("aigc", 542), ("bigc", 543),
    ("cigc", 544), ("aigs", 545), ("bigs", 546), ("cigs", 547), ("aigd", 548), ("bigd", 549), ("cigd", 550), ("dlcig", 551), ("dlcigd", 552), ("poxedge", 553), ("ntox", 554), ("toxref", 555), ("pigcd", 556), ("aigcl", 557), ("aigcw", 558), ("aigsl", 559),
    ("aigsw", 560), ("aigdl", 561), ("aigdw", 562), ("pigcdl", 563), ("laigbinv", 564), ("waigbinv", 565), ("paigbinv", 566), ("lbigbinv", 567), ("wbigbinv", 568), ("pbigbinv", 569), ("lcigbinv", 570), ("wcigbinv", 571), ("pcigbinv", 572), ("leigbinv", 573), ("weigbinv", 574), ("peigbinv", 575),
    ("lnigbinv", 576), ("wnigbinv", 577), ("pnigbinv", 578), ("laigbacc", 579), ("waigbacc", 580), ("paigbacc", 581), ("lbigbacc", 582), ("wbigbacc", 583), ("pbigbacc", 584), ("lcigbacc", 585), ("wcigbacc", 586), ("pcigbacc", 587), ("lnigbacc", 588), ("wnigbacc", 589), ("pnigbacc", 590), ("laigc", 591),
    ("waigc", 592), ("paigc", 593), ("lbigc", 594), ("wbigc", 595), ("pbigc", 596), ("lcigc", 597), ("wcigc", 598), ("pcigc", 599), ("laigs", 600), ("waigs", 601), ("paigs", 602), ("lbigs", 603), ("wbigs", 604), ("pbigs", 605), ("lcigs", 606), ("wcigs", 607),
    ("pcigs", 608), ("laigd", 609), ("waigd", 610), ("paigd", 611), ("lbigd", 612), ("wbigd", 613), ("pbigd", 614), ("lcigd", 615), ("wcigd", 616), ("pcigd", 617), ("lpoxedge", 618), ("wpoxedge", 619), ("ppoxedge", 620), ("ldlcig", 621), ("wdlcig", 622), ("pdlcig", 623),
    ("ldlcigd", 624), ("wdlcigd", 625), ("pdlcigd", 626), ("lntox", 627), ("wntox", 628), ("pntox", 629), ("agidl", 630), ("agidll", 631), ("agidlw", 632), ("lagidl", 633), ("wagidl", 634), ("pagidl", 635), ("bgidl", 636), ("lbgidl", 637), ("wbgidl", 638), ("pbgidl", 639),
    ("cgidl", 640), ("lcgidl", 641), ("wcgidl", 642), ("pcgidl", 643), ("egidl", 644), ("legidl", 645), ("wegidl", 646), ("pegidl", 647), ("agisl", 648), ("agisll", 649), ("agislw", 650), ("lagisl", 651), ("wagisl", 652), ("pagisl", 653), ("bgisl", 654), ("lbgisl", 655),
    ("wbgisl", 656), ("pbgisl", 657), ("cgisl", 658), ("lcgisl", 659), ("wcgisl", 660), ("pcgisl", 661), ("egisl", 662), ("legisl", 663), ("wegisl", 664), ("pegisl", 665), ("cf", 666), ("lcf", 667), ("wcf", 668), ("pcf", 669), ("cfrcoeff", 670), ("cgso", 671),
    ("cgdo", 672), ("cgbo", 673), ("cgsl", 674), ("lcgsl", 675), ("wcgsl", 676), ("pcgsl", 677), ("cgdl", 678), ("lcgdl", 679), ("wcgdl", 680), ("pcgdl", 681), ("ckappas", 682), ("lckappas", 683), ("wckappas", 684), ("pckappas", 685), ("ckappad", 686), ("lckappad", 687),
    ("wckappad", 688), ("pckappad", 689), ("ckappad1", 690), ("ckappad2", 691), ("ckappas1", 692), ("ckappas2", 693), ("spqbacv", 694), ("dmcg", 695), ("dmci", 696), ("dmdg", 697), ("dmcgt", 698), ("xgl", 699), ("rshg", 700), ("cjs", 701), ("cjd", 702), ("cjsws", 703),
    ("cjswd", 704), ("cjswgs", 705), ("cjswgd", 706), ("pbs", 707), ("pbd", 708), ("pbsws", 709), ("pbswd", 710), ("pbswgs", 711), ("pbswgd", 712), ("mjs", 713), ("mjd", 714), ("mjsws", 715), ("mjswd", 716), ("mjswgs", 717), ("mjswgd", 718), ("jss", 719),
    ("jsd", 720), ("jsws", 721), ("jswd", 722), ("jswgs", 723), ("jswgd", 724), ("njs", 725), ("njd", 726), ("ijthsfwd", 727), ("ijthdfwd", 728), ("ijthsrev", 729), ("ijthdrev", 730), ("bvs", 731), ("bvd", 732), ("xjbvs", 733), ("xjbvd", 734), ("jtss", 735),
    ("jtsd", 736), ("jtssws", 737), ("jtsswd", 738), ("jtsswgs", 739), ("jtsswgd", 740), ("jtweff", 741), ("njts", 742), ("njtsd", 743), ("njtssw", 744), ("njtsswd", 745), ("njtsswg", 746), ("njtsswgd", 747), ("vtss", 748), ("vtsd", 749), ("vtssws", 750), ("vtsswd", 751),
    ("vtsswgs", 752), ("vtsswgd", 753), ("xrcrg1", 754), ("xrcrg2", 755), ("gbmin", 756), ("rbps0", 757), ("rbpsl", 758), ("rbpsw", 759), ("rbpsnf", 760), ("rbpd0", 761), ("rbpdl", 762), ("rbpdw", 763), ("rbpdnf", 764), ("rbpbx0", 765), ("rbpbxl", 766), ("rbpbxw", 767),
    ("rbpbxnf", 768), ("rbpby0", 769), ("rbpbyl", 770), ("rbpbyw", 771), ("rbpbynf", 772), ("rbsbx0", 773), ("rbsby0", 774), ("rbdbx0", 775), ("rbdby0", 776), ("rbsdbxl", 777), ("rbsdbxw", 778), ("rbsdbxnf", 779), ("rbsdbyl", 780), ("rbsdbyw", 781), ("rbsdbynf", 782), ("ef", 783),
    ("em", 784), ("noia", 785), ("noia3", 786), ("lnoia3", 787), ("wnoia3", 788), ("pnoia3", 789), ("mpower", 790), ("lmpower", 791), ("wmpower", 792), ("pmpower", 793), ("qsref", 794), ("lqsref", 795), ("wqsref", 796), ("pqsref", 797), ("spfn", 798), ("noib", 799),
    ("noic", 800), ("lintnoi", 801), ("noia1", 802), ("noiax", 803), ("bfns", 804), ("bfnd", 805), ("kfns", 806), ("kfnd", 807), ("afns", 808), ("afnd", 809), ("ntnoi", 810), ("rnoia", 811), ("rnoib", 812), ("rnoic", 813), ("tnoia", 814), ("tnoib", 815),
    ("tnoic", 816), ("binunit", 817), ("dlbin", 818), ("dwbin", 819), ("tnom", 820), ("tbgasub", 821), ("tbgbsub", 822), ("tnfactor", 823), ("ute", 824), ("lute", 825), ("wute", 826), ("pute", 827), ("utel", 828), ("ua1", 829), ("lua1", 830), ("wua1", 831),
    ("pua1", 832), ("ua1l", 833), ("uc1", 834), ("luc1", 835), ("wuc1", 836), ("puc1", 837), ("ud1", 838), ("lud1", 839), ("wud1", 840), ("pud1", 841), ("ud1l", 842), ("eu1", 843), ("leu1", 844), ("weu1", 845), ("peu1", 846), ("ucste", 847),
    ("lucste", 848), ("wucste", 849), ("pucste", 850), ("teta0", 851), ("prt", 852), ("lprt", 853), ("wprt", 854), ("pprt", 855), ("at", 856), ("lat", 857), ("wat", 858), ("pat", 859), ("atl", 860), ("tdelta", 861), ("ptwgt", 862), ("lptwgt", 863),
    ("wptwgt", 864), ("pptwgt", 865), ("ptwgtl", 866), ("kt1", 867), ("kt1exp", 868), ("kt1l", 869), ("lkt1", 870), ("wkt1", 871), ("pkt1", 872), ("kt2", 873), ("lkt2", 874), ("wkt2", 875), ("pkt2", 876), ("iit", 877), ("liit", 878), ("wiit", 879),
    ("piit", 880), ("igt", 881), ("ligt", 882), ("wigt", 883), ("pigt", 884), ("tgidl", 885), ("ltgidl", 886), ("wtgidl", 887), ("ptgidl", 888), ("tcj", 889), ("tcjsw", 890), ("tcjswg", 891), ("tpb", 892), ("tpbsw", 893), ("tpbswg", 894), ("xtis", 895),
    ("xtid", 896), ("xtss", 897), ("xtsd", 898), ("xtssws", 899), ("xtsswd", 900), ("xtsswgs", 901), ("xtsswgd", 902), ("tnjts", 903), ("tnjtsd", 904), ("tnjtssw", 905), ("tnjtsswd", 906), ("tnjtsswg", 907), ("tnjtsswgd", 908), ("rth0", 909), ("cth0", 910), ("wth0", 911),
    ("saref", 912), ("sbref", 913), ("wlod", 914), ("ku0", 915), ("kvsat", 916), ("tku0", 917), ("lku0", 918), ("wku0", 919), ("pku0", 920), ("llodku0", 921), ("wlodku0", 922), ("kvth0", 923), ("lkvth0", 924), ("wkvth0", 925), ("pkvth0", 926), ("llodvth", 927),
    ("wlodvth", 928), ("stk2", 929), ("lodk2", 930), ("steta0", 931), ("lodeta0", 932), ("web", 933), ("wec", 934), ("kvth0we", 935), ("lkvth0we", 936), ("wkvth0we", 937), ("pkvth0we", 938), ("k2we", 939), ("lk2we", 940), ("wk2we", 941), ("pk2we", 942), ("ku0we", 943),
    ("lku0we", 944), ("wku0we", 945), ("pku0we", 946), ("scref", 947), ("ssl0", 948), ("ssl1", 949), ("ssl2", 950), ("ssl3", 951), ("ssl4", 952), ("ssl5", 953), ("sslexp1", 954), ("sslexp2", 955), ("avdsx", 956), ("wedge", 957), ("dgammaedge", 958), ("dgammaedgel", 959),
    ("dgammaedgelexp", 960), ("dvtedge", 961), ("ndepedge", 962), ("lndepedge", 963), ("wndepedge", 964), ("pndepedge", 965), ("nfactoredge", 966), ("lnfactoredge", 967), ("wnfactoredge", 968), ("pnfactoredge", 969), ("citedge", 970), ("lcitedge", 971), ("wcitedge", 972), ("pcitedge", 973), ("cdscdedge", 974), ("lcdscdedge", 975),
    ("wcdscdedge", 976), ("pcdscdedge", 977), ("cdscbedge", 978), ("lcdscbedge", 979), ("wcdscbedge", 980), ("pcdscbedge", 981), ("eta0edge", 982), ("leta0edge", 983), ("weta0edge", 984), ("peta0edge", 985), ("etabedge", 986), ("letabedge", 987), ("wetabedge", 988), ("petabedge", 989), ("kt1edge", 990), ("lkt1edge", 991),
    ("wkt1edge", 992), ("pkt1edge", 993), ("kt1ledge", 994), ("lkt1ledge", 995), ("wkt1ledge", 996), ("pkt1ledge", 997), ("kt2edge", 998), ("lkt2edge", 999), ("wkt2edge", 1000), ("pkt2edge", 1001), ("kt1expedge", 1002), ("lkt1expedge", 1003), ("wkt1expedge", 1004), ("pkt1expedge", 1005), ("tnfactoredge", 1006), ("ltnfactoredge", 1007),
    ("wtnfactoredge", 1008), ("ptnfactoredge", 1009), ("teta0edge", 1010), ("lteta0edge", 1011), ("wteta0edge", 1012), ("pteta0edge", 1013), ("dvt0edge", 1014), ("dvt1edge", 1015), ("dvt2edge", 1016), ("k2edge", 1017), ("lk2edge", 1018), ("wk2edge", 1019), ("pk2edge", 1020), ("kvth0edge", 1021), ("lkvth0edge", 1022), ("wkvth0edge", 1023),
    ("pkvth0edge", 1024), ("kvth0edgewe", 1025), ("lkvth0edgewe", 1026), ("wkvth0edgewe", 1027), ("pkvth0edgewe", 1028), ("k2edgewe", 1029), ("lk2edgewe", 1030), ("wk2edgewe", 1031), ("pk2edgewe", 1032), ("stk2edge", 1033), ("lstk2edge", 1034), ("wstk2edge", 1035), ("pstk2edge", 1036), ("steta0edge", 1037), ("lsteta0edge", 1038), ("wsteta0edge", 1039),
    ("psteta0edge", 1040), ("igclamp", 1041), ("lp", 1042), ("rnoik", 1043), ("tnoik", 1044), ("tnoik2", 1045), ("k0", 1046), ("lk0", 1047), ("wk0", 1048), ("pk0", 1049), ("k01", 1050), ("lk01", 1051), ("wk01", 1052), ("pk01", 1053), ("m0", 1054), ("lm0", 1055),
    ("wm0", 1056), ("pm0", 1057), ("m01", 1058), ("lm01", 1059), ("wm01", 1060), ("pm01", 1061), ("nedge", 1062), ("noia1_edge", 1063), ("noiax_edge", 1064), ("fnoimod", 1065), ("lh", 1066), ("noia2", 1067), ("hndep", 1068), ("c0", 1069), ("lc0", 1070), ("wc0", 1071),
    ("pc0", 1072), ("c01", 1073), ("lc01", 1074), ("wc01", 1075), ("pc01", 1076), ("c0si", 1077), ("lc0si", 1078), ("wc0si", 1079), ("pc0si", 1080), ("c0si1", 1081), ("lc0si1", 1082), ("wc0si1", 1083), ("pc0si1", 1084), ("c0sisat", 1085), ("lc0sisat", 1086), ("wc0sisat", 1087),
    ("pc0sisat", 1088), ("c0sisat1", 1089), ("lc0sisat1", 1090), ("wc0sisat1", 1091), ("pc0sisat1", 1092), ("minr", 1093), ("hvmod", 1094), ("hvcap", 1095), ("hvcaps", 1096), ("rbodyhvmod", 1097), ("iimod", 1098), ("ndriftd", 1099), ("vdrift", 1100), ("ptwghv", 1101), ("ptwghv1", 1102), ("psatxhv", 1103),
    ("ptwghvii", 1104), ("ptwghv1ii", 1105), ("psatxhvii", 1106), ("mdrift", 1107), ("dsmooth", 1108), ("ndrifts", 1109), ("rdlcw", 1110), ("rdlcwcv", 1111), ("rslcw", 1112), ("pdrwb", 1113), ("vfbov", 1114), ("lover", 1115), ("loveracc", 1116), ("ndr", 1117), ("slhv", 1118), ("slhv1", 1119),
    ("prthv", 1120), ("athv", 1121), ("hvfactor", 1122), ("asymp", 1123), ("drb1", 1124), ("drb2", 1125), ("rdvds", 1126), ("gadrift", 1127), ("xpart", 1128), ("abulk", 1129), ("a0", 1130), ("ags", 1131), ("ags1", 1132), ("keta", 1133), ("a0cv", 1134), ("agscv", 1135),
    ("ketacv", 1136), ("cvslope", 1137),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1138] = [
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
    None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1138] = [
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
    None, None, None, None, None, None, None, None, None, None, Some(0), None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1138] = [
    "L", "W", "NF", "NRS", "NRD", "VFBSDOFF", "MINZ", "RGATEMOD", "RBODYMOD", "GEOMOD", "RGEOMOD", "RBPB", "RBPD", "RBPS", "RBDB", "RBSB",
    "RDB", "SA", "SB", "SD", "SCA", "SCB", "SCC", "SC", "AS", "AD", "PS", "PD", "MULT_I", "MULT_Q", "MULT_FN", "XGW",
    "NGCON", "DTEMP", "MULU0", "DELVTO", "IDS0MULT", "EDGEFET", "SSLMOD", "TYPE", "CVMOD", "COVMOD", "RDSMOD", "WPEMOD", "ASYMMOD", "GIDLMOD", "IGCMOD", "IGBMOD",
    "TNOIMOD", "SHMOD", "MOBSCALE", "LLONG", "LMLT", "WMLT", "XL", "WWIDE", "XW", "LINT", "LL", "LW", "LWL", "LLN", "LWN", "WINT",
    "WL", "WW", "WWL", "WLN", "WWN", "DLC", "LLC", "LWC", "LWLC", "DWC", "WLC", "WWC", "WWLC", "TOXE", "TOXP", "DTOX",
    "NDEP", "NDEPL1", "NDEPLEXP1", "NDEPL2", "NDEPLEXP2", "NDEPW", "NDEPWEXP", "NDEPWL", "NDEPWLEXP", "LNDEP", "WNDEP", "PNDEP", "NDEPCV", "NDEPCVL1", "NDEPCVLEXP1", "NDEPCVL2",
    "NDEPCVLEXP2", "NDEPCVW", "NDEPCVWEXP", "NDEPCVWL", "NDEPCVWLEXP", "LNDEPCV", "WNDEPCV", "PNDEPCV", "NGATE", "LNGATE", "WNGATE", "PNGATE", "NI0SUB", "BG0SUB", "EPSRSUB", "EPSROX",
    "XJ", "LXJ", "WXJ", "PXJ", "VFB", "LVFB", "WVFB", "PVFB", "VFBL", "VFBLEXP", "VFBW", "VFBWEXP", "VFBWL", "VFBWLEXP", "VFBCV", "LVFBCV",
    "WVFBCV", "PVFBCV", "VFBCVL", "VFBCVLEXP", "VFBCVW", "VFBCVWEXP", "VFBCVWL", "VFBCVWLEXP", "DELVFBACC", "PERMOD", "DWJ", "NSD", "LNSD", "WNSD", "PNSD", "DVTP0",
    "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2", "PDVTP2", "DVTP3", "LDVTP3", "WDVTP3", "PDVTP3", "DVTP4",
    "LDVTP4", "WDVTP4", "PDVTP4", "DVTP5", "LDVTP5", "WDVTP5", "PDVTP5", "PHIN", "LPHIN", "WPHIN", "PPHIN", "ETA0", "LETA0", "WETA0", "PETA0", "ETA0R",
    "LETA0R", "WETA0R", "PETA0R", "DSUB", "ETAB", "ETABEXP", "LETAB", "WETAB", "PETAB", "K1", "K1L", "K1LEXP", "K1W", "K1WEXP", "K1WL", "K1WLEXP",
    "LK1", "WK1", "PK1", "K2", "K2L", "K2LEXP", "K2W", "K2WEXP", "K2WL", "K2WLEXP", "LK2", "WK2", "PK2", "ADOS", "BDOS", "QM0",
    "ETAQM", "CIT", "LCIT", "WCIT", "PCIT", "NFACTOR", "NFACTORL", "NFACTORLEXP", "NFACTORW", "NFACTORWEXP", "NFACTORWL", "NFACTORWLEXP", "LNFACTOR", "WNFACTOR", "PNFACTOR", "CDSCD",
    "CDSCDL", "CDSCDLEXP", "LCDSCD", "WCDSCD", "PCDSCD", "CDSCDR", "LCDSCDR", "WCDSCDR", "PCDSCDR", "CDSCB", "CDSCBL", "CDSCBLEXP", "LCDSCB", "WCDSCB", "PCDSCB", "VSAT",
    "LVSAT", "WVSAT", "PVSAT", "VSATL", "VSATLEXP", "VSATW", "VSATWEXP", "VSATWL", "VSATWLEXP", "VSATR", "LVSATR", "WVSATR", "PVSATR", "DELTA", "LDELTA", "WDELTA",
    "PDELTA", "DELTAL", "DELTALEXP", "VSATCV", "LVSATCV", "WVSATCV", "PVSATCV", "VSATCVL", "VSATCVLEXP", "VSATCVW", "VSATCVWEXP", "VSATCVWL", "VSATCVWLEXP", "UP1", "LP1", "UP2",
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
    "PPSATB", "PSATX", "PTWG", "LPTWG", "WPTWG", "PPTWG", "PTWGL", "PTWGLEXP", "PTWGR", "LPTWGR", "WPTWGR", "PPTWGR", "A1", "LA1", "WA1", "PA1",
    "A11", "LA11", "WA11", "PA11", "A2", "LA2", "WA2", "PA2", "A21", "LA21", "WA21", "PA21", "PDIBLC", "PDIBLCL", "PDIBLCLEXP", "LPDIBLC",
    "WPDIBLC", "PPDIBLC", "PDIBLCR", "LPDIBLCR", "WPDIBLCR", "PPDIBLCR", "PDIBLCB", "LPDIBLCB", "WPDIBLCB", "PPDIBLCB", "PVAG", "LPVAG", "WPVAG", "PPVAG", "FPROUT", "FPROUTL",
    "FPROUTLEXP", "LFPROUT", "WFPROUT", "PFPROUT", "ALPHA0", "ALPHA0L", "ALPHA0LEXP", "ALPHA0W", "ALPHA0WEXP", "LALPHA0", "WALPHA0", "PALPHA0", "ALPHA3", "ALPHA4", "BETA0", "BETA0W",
    "BETA0WEXP", "LBETA0", "WBETA0", "PBETA0", "ALPHADR", "BETADR", "DRII1", "DRII2", "DELTAII", "ALPHA1", "ALPHA2", "ALPHADR1", "ALPHADR2", "ALPHADR3", "ALPHADR4", "DREXP",
    "DRII3", "DRII4", "CMD1", "CMD2", "CMS1", "CMS2", "BETA1", "BETA1W", "BETA1WEXP", "BETA2", "BETA2W", "BETA2WEXP", "BETA3", "ALPHA0R", "LALPHA0R", "WALPHA0R",
    "PALPHA0R", "BETA0R", "LBETA0R", "WBETA0R", "PBETA0R", "AIGBACC", "BIGBACC", "CIGBACC", "NIGBACC", "AIGBINV", "BIGBINV", "CIGBINV", "EIGBINV", "NIGBINV", "AIGC", "BIGC",
    "CIGC", "AIGS", "BIGS", "CIGS", "AIGD", "BIGD", "CIGD", "DLCIG", "DLCIGD", "POXEDGE", "NTOX", "TOXREF", "PIGCD", "AIGCL", "AIGCW", "AIGSL",
    "AIGSW", "AIGDL", "AIGDW", "PIGCDL", "LAIGBINV", "WAIGBINV", "PAIGBINV", "LBIGBINV", "WBIGBINV", "PBIGBINV", "LCIGBINV", "WCIGBINV", "PCIGBINV", "LEIGBINV", "WEIGBINV", "PEIGBINV",
    "LNIGBINV", "WNIGBINV", "PNIGBINV", "LAIGBACC", "WAIGBACC", "PAIGBACC", "LBIGBACC", "WBIGBACC", "PBIGBACC", "LCIGBACC", "WCIGBACC", "PCIGBACC", "LNIGBACC", "WNIGBACC", "PNIGBACC", "LAIGC",
    "WAIGC", "PAIGC", "LBIGC", "WBIGC", "PBIGC", "LCIGC", "WCIGC", "PCIGC", "LAIGS", "WAIGS", "PAIGS", "LBIGS", "WBIGS", "PBIGS", "LCIGS", "WCIGS",
    "PCIGS", "LAIGD", "WAIGD", "PAIGD", "LBIGD", "WBIGD", "PBIGD", "LCIGD", "WCIGD", "PCIGD", "LPOXEDGE", "WPOXEDGE", "PPOXEDGE", "LDLCIG", "WDLCIG", "PDLCIG",
    "LDLCIGD", "WDLCIGD", "PDLCIGD", "LNTOX", "WNTOX", "PNTOX", "AGIDL", "AGIDLL", "AGIDLW", "LAGIDL", "WAGIDL", "PAGIDL", "BGIDL", "LBGIDL", "WBGIDL", "PBGIDL",
    "CGIDL", "LCGIDL", "WCGIDL", "PCGIDL", "EGIDL", "LEGIDL", "WEGIDL", "PEGIDL", "AGISL", "AGISLL", "AGISLW", "LAGISL", "WAGISL", "PAGISL", "BGISL", "LBGISL",
    "WBGISL", "PBGISL", "CGISL", "LCGISL", "WCGISL", "PCGISL", "EGISL", "LEGISL", "WEGISL", "PEGISL", "CF", "LCF", "WCF", "PCF", "CFRCOEFF", "CGSO",
    "CGDO", "CGBO", "CGSL", "LCGSL", "WCGSL", "PCGSL", "CGDL", "LCGDL", "WCGDL", "PCGDL", "CKAPPAS", "LCKAPPAS", "WCKAPPAS", "PCKAPPAS", "CKAPPAD", "LCKAPPAD",
    "WCKAPPAD", "PCKAPPAD", "CKAPPAD1", "CKAPPAD2", "CKAPPAS1", "CKAPPAS2", "SPQBACV", "DMCG", "DMCI", "DMDG", "DMCGT", "XGL", "RSHG", "CJS", "CJD", "CJSWS",
    "CJSWD", "CJSWGS", "CJSWGD", "PBS", "PBD", "PBSWS", "PBSWD", "PBSWGS", "PBSWGD", "MJS", "MJD", "MJSWS", "MJSWD", "MJSWGS", "MJSWGD", "JSS",
    "JSD", "JSWS", "JSWD", "JSWGS", "JSWGD", "NJS", "NJD", "IJTHSFWD", "IJTHDFWD", "IJTHSREV", "IJTHDREV", "BVS", "BVD", "XJBVS", "XJBVD", "JTSS",
    "JTSD", "JTSSWS", "JTSSWD", "JTSSWGS", "JTSSWGD", "JTWEFF", "NJTS", "NJTSD", "NJTSSW", "NJTSSWD", "NJTSSWG", "NJTSSWGD", "VTSS", "VTSD", "VTSSWS", "VTSSWD",
    "VTSSWGS", "VTSSWGD", "XRCRG1", "XRCRG2", "GBMIN", "RBPS0", "RBPSL", "RBPSW", "RBPSNF", "RBPD0", "RBPDL", "RBPDW", "RBPDNF", "RBPBX0", "RBPBXL", "RBPBXW",
    "RBPBXNF", "RBPBY0", "RBPBYL", "RBPBYW", "RBPBYNF", "RBSBX0", "RBSBY0", "RBDBX0", "RBDBY0", "RBSDBXL", "RBSDBXW", "RBSDBXNF", "RBSDBYL", "RBSDBYW", "RBSDBYNF", "EF",
    "EM", "NOIA", "NOIA3", "LNOIA3", "WNOIA3", "PNOIA3", "MPOWER", "LMPOWER", "WMPOWER", "PMPOWER", "QSREF", "LQSREF", "WQSREF", "PQSREF", "SPFN", "NOIB",
    "NOIC", "LINTNOI", "NOIA1", "NOIAX", "BFNS", "BFND", "KFNS", "KFND", "AFNS", "AFND", "NTNOI", "RNOIA", "RNOIB", "RNOIC", "TNOIA", "TNOIB",
    "TNOIC", "BINUNIT", "DLBIN", "DWBIN", "TNOM", "TBGASUB", "TBGBSUB", "TNFACTOR", "UTE", "LUTE", "WUTE", "PUTE", "UTEL", "UA1", "LUA1", "WUA1",
    "PUA1", "UA1L", "UC1", "LUC1", "WUC1", "PUC1", "UD1", "LUD1", "WUD1", "PUD1", "UD1L", "EU1", "LEU1", "WEU1", "PEU1", "UCSTE",
    "LUCSTE", "WUCSTE", "PUCSTE", "TETA0", "PRT", "LPRT", "WPRT", "PPRT", "AT", "LAT", "WAT", "PAT", "ATL", "TDELTA", "PTWGT", "LPTWGT",
    "WPTWGT", "PPTWGT", "PTWGTL", "KT1", "KT1EXP", "KT1L", "LKT1", "WKT1", "PKT1", "KT2", "LKT2", "WKT2", "PKT2", "IIT", "LIIT", "WIIT",
    "PIIT", "IGT", "LIGT", "WIGT", "PIGT", "TGIDL", "LTGIDL", "WTGIDL", "PTGIDL", "TCJ", "TCJSW", "TCJSWG", "TPB", "TPBSW", "TPBSWG", "XTIS",
    "XTID", "XTSS", "XTSD", "XTSSWS", "XTSSWD", "XTSSWGS", "XTSSWGD", "TNJTS", "TNJTSD", "TNJTSSW", "TNJTSSWD", "TNJTSSWG", "TNJTSSWGD", "RTH0", "CTH0", "WTH0",
    "SAREF", "SBREF", "WLOD", "KU0", "KVSAT", "TKU0", "LKU0", "WKU0", "PKU0", "LLODKU0", "WLODKU0", "KVTH0", "LKVTH0", "WKVTH0", "PKVTH0", "LLODVTH",
    "WLODVTH", "STK2", "LODK2", "STETA0", "LODETA0", "WEB", "WEC", "KVTH0WE", "LKVTH0WE", "WKVTH0WE", "PKVTH0WE", "K2WE", "LK2WE", "WK2WE", "PK2WE", "KU0WE",
    "LKU0WE", "WKU0WE", "PKU0WE", "SCREF", "SSL0", "SSL1", "SSL2", "SSL3", "SSL4", "SSL5", "SSLEXP1", "SSLEXP2", "AVDSX", "WEDGE", "DGAMMAEDGE", "DGAMMAEDGEL",
    "DGAMMAEDGELEXP", "DVTEDGE", "NDEPEDGE", "LNDEPEDGE", "WNDEPEDGE", "PNDEPEDGE", "NFACTOREDGE", "LNFACTOREDGE", "WNFACTOREDGE", "PNFACTOREDGE", "CITEDGE", "LCITEDGE", "WCITEDGE", "PCITEDGE", "CDSCDEDGE", "LCDSCDEDGE",
    "WCDSCDEDGE", "PCDSCDEDGE", "CDSCBEDGE", "LCDSCBEDGE", "WCDSCBEDGE", "PCDSCBEDGE", "ETA0EDGE", "LETA0EDGE", "WETA0EDGE", "PETA0EDGE", "ETABEDGE", "LETABEDGE", "WETABEDGE", "PETABEDGE", "KT1EDGE", "LKT1EDGE",
    "WKT1EDGE", "PKT1EDGE", "KT1LEDGE", "LKT1LEDGE", "WKT1LEDGE", "PKT1LEDGE", "KT2EDGE", "LKT2EDGE", "WKT2EDGE", "PKT2EDGE", "KT1EXPEDGE", "LKT1EXPEDGE", "WKT1EXPEDGE", "PKT1EXPEDGE", "TNFACTOREDGE", "LTNFACTOREDGE",
    "WTNFACTOREDGE", "PTNFACTOREDGE", "TETA0EDGE", "LTETA0EDGE", "WTETA0EDGE", "PTETA0EDGE", "DVT0EDGE", "DVT1EDGE", "DVT2EDGE", "K2EDGE", "LK2EDGE", "WK2EDGE", "PK2EDGE", "KVTH0EDGE", "LKVTH0EDGE", "WKVTH0EDGE",
    "PKVTH0EDGE", "KVTH0EDGEWE", "LKVTH0EDGEWE", "WKVTH0EDGEWE", "PKVTH0EDGEWE", "K2EDGEWE", "LK2EDGEWE", "WK2EDGEWE", "PK2EDGEWE", "STK2EDGE", "LSTK2EDGE", "WSTK2EDGE", "PSTK2EDGE", "STETA0EDGE", "LSTETA0EDGE", "WSTETA0EDGE",
    "PSTETA0EDGE", "IGCLAMP", "LP", "RNOIK", "TNOIK", "TNOIK2", "K0", "LK0", "WK0", "PK0", "K01", "LK01", "WK01", "PK01", "M0", "LM0",
    "WM0", "PM0", "M01", "LM01", "WM01", "PM01", "NEDGE", "NOIA1_EDGE", "NOIAX_EDGE", "FNOIMOD", "LH", "NOIA2", "HNDEP", "C0", "LC0", "WC0",
    "PC0", "C01", "LC01", "WC01", "PC01", "C0SI", "LC0SI", "WC0SI", "PC0SI", "C0SI1", "LC0SI1", "WC0SI1", "PC0SI1", "C0SISAT", "LC0SISAT", "WC0SISAT",
    "PC0SISAT", "C0SISAT1", "LC0SISAT1", "WC0SISAT1", "PC0SISAT1", "minr", "HVMOD", "HVCAP", "HVCAPS", "RBODYHVMOD", "IIMOD", "NDRIFTD", "VDRIFT", "PTWGHV", "PTWGHV1", "PSATXHV",
    "PTWGHVII", "PTWGHV1II", "PSATXHVII", "MDRIFT", "DSMOOTH", "NDRIFTS", "RDLCW", "RDLCWCV", "RSLCW", "PDRWB", "VFBOV", "LOVER", "LOVERACC", "NDR", "SLHV", "SLHV1",
    "PRTHV", "ATHV", "HVFACTOR", "ASYMP", "DRB1", "DRB2", "RDVDS", "GADRIFT", "XPART", "ABULK", "A0", "AGS", "AGS1", "KETA", "A0CV", "AGSCV",
    "KETACV", "CVSLOPE",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1138] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1138] = [
    false, false, true, false, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1138] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
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
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -50.0, label: "-50.0" }), None, None, None,
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
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None,
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
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0001, label: "0.0001" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1138] = [
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 8.0, label: "8.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
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
    None, None, None, None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }),
    None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, None,
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
    None, None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None,
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
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, None,
    None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1138] = [
    3, 3, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 0,
    0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0,
    3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0,
    0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3,
    0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 2, 2, 3, 2, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
    3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 1,
    2, 2, 2, 0, 2, 0, 2, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 2, 2, 2, 2, 2, 3, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 3, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 2, 2, 2, 3, 2, 2, 2, 3, 2, 2,
    2, 3, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 2, 0,
    0, 0, 2, 3, 3, 3, 2, 2, 3, 3, 2, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3,
    3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 2, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 3, 2, 0, 3, 2, 0, 3, 3, 2, 3, 2, 2, 2, 0, 0, 0, 0, 3, 2, 3,
    0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 3, 2, 0, 0, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1138] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[],
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
        699 => Some(ParameterBound { value: ((params[0] * params[52]) + params[54]), label: "computed upper-bound expression" }),
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

fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {
    let mut boxed = Box::<[f64; N]>::new_uninit();
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {
    let mut boxed = Box::<[bool; N]>::new_uninit();
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
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

pub struct Instance {
    pub nodes: [usize; 17],
    pub branches: [usize; 14],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1138]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<16, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 12;
    pub const NODE_COUNT: usize = 17;
    pub const INTERNAL_NODE_NAMES: [&str; 12] = ["di", "di1", "si", "si1", "gi", "gm", "bi", "sbulk", "dbulk", "ddbulk", "N1", "N2"];

    pub const BRANCH_COUNT: usize = 14;
    pub const PARAMETER_COUNT: usize = 1138;
    pub const VARIABLE_COUNT: usize = 1631;
    pub const DDT_STATE_COUNT: usize = 16;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "d947bd2badcd76763a6e5082f8f7fdf785365a82515c42002af3206c3f78a9a5";
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
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(80);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(16);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 80);
        debug_assert_eq!(state.flags.len(), 16);
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

    pub(crate) fn capture_persistent_state(&self) -> GeneratedVerilogAPersistentState {
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

    pub(crate) fn validate_persistent_state_shape(&self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
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

    pub(crate) fn restore_persistent_state(&mut self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimbulk'", name));
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
        let _ = invalidates_caches;
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) -> Result<(), String> {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            self.multiplicity = multiplicity;
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
