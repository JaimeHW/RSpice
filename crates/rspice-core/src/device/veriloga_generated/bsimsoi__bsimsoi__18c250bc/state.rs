#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 1044],
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
            const DEFAULTS_0: [f64; 28] = [
                0.0, 5e-6, 5e-6, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1e-5,
                1.0, 1.0, 50.0, 50.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 28);
            {
                let params = &mut *ptr;
                params[28] = params[26];
                validate_parameter("AGBCPD", params[28], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                0.0, 1.0, 1.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(29), 5);
            {
                let params = &mut *ptr;
                params[34] = params[32];
                validate_parameter("MULT_FN", params[34], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 7] = [
                0.0, 0.0, 1.0, 4.7, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(35), 7);
            {
                let params = &mut *ptr;
                params[42] = if (params[38] >= 4.2) { 1.0 } else { 0.0 };
                validate_parameter("VGSTCVMOD", params[42], true, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 13] = [
                0.0, 0.0, 1e-8, 3.9, 11.7, 14500000000.0, 1.16, 0.000702,
                1108.0, 4.05, 4.05, 1.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(43), 13);
            {
                let params = &mut *ptr;
                params[56] = if (params[37] == 1.0) { 1.5 } else { (-1.5) };
                validate_finite_parameter("VDDEOT", params[56]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 10] = [
                300.15, 1.0, 1.0, 11.7, 2.0, 1.0, 0.0, 1.0,
                1.0, 1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(57), 10);
            {
                let params = &mut *ptr;
                params[67] = params[66];
                validate_parameter("TOXM", params[67], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 38] = [
                0.0, 0.00024, 0.0, 0.0, 0.0, 1.0, 80000.0, 33000.0,
                1.0, 0.0, 0.0, 1.0, -0.6, 6e16, 1.7e17, 0.0,
                1e20, 0.0, 0.0, 0.0, -3.0, 1.55e-7, 0.53, -0.11,
                0.0, 0.022, -0.0186, 0.0, 0.0, 2.5e-6, 0.0, 2.2,
                0.53, -0.032, 0.0, 5300000.0, -0.032, 0.56,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(68), 38);
            {
                let params = &mut *ptr;
                params[106] = params[105];
                validate_finite_parameter("DSUB", params[106]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[107] = if (params[37] == 1.0) { 0.7 } else { (-0.7) };
                validate_finite_parameter("VTHO", params[107]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[108] = params[107];
                validate_finite_parameter("VTH0", params[108]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 5] = [
                -1.0, 2.25e-9, 4.31e-9, 5.87e-19, -7.61e-18,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(109), 5);
            {
                let params = &mut *ptr;
                params[114] = if (params[62] == 3.0) { (-0.0465) } else { (-4.65e-11) };
                validate_finite_parameter("UC", params[114]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[115] = if (params[62] == 3.0) { (-0.056) } else { (-5.6e-11) };
                validate_finite_parameter("UC1", params[115]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[116] = if (params[37] == 1.0) { 0.067 } else { 0.025 };
                validate_finite_parameter("U0", params[116]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[117] = if (params[37] == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("EU", params[117]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 1] = [
                -1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(118), 1);
            {
                let params = &mut *ptr;
                params[119] = if (params[37] == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("UCS", params[119]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 27] = [
                -0.004775, 0.0, 0.0, 0.0, 0.0, -0.08, 27.0, 0.0,
                0.0, 0.0, 0.01, 0.0, 100.0, 50.0, 50.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.08, -0.07,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(120), 27);
            {
                let params = &mut *ptr;
                params[147] = params[145];
                validate_finite_parameter("ETA0CV", params[147]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[148] = params[146];
                validate_finite_parameter("ETABCV", params[148]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 8] = [
                1.3, 0.39, 0.0086, 0.0, 0.0, 3e-7, 1e-7, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(149), 8);
            {
                let params = &mut *ptr;
                params[157] = params[155];
                validate_parameter("XJ", params[157], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 7] = [
                0.0, 2300000000.0, 0.0, 0.5, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(158), 7);
            {
                let params = &mut *ptr;
                params[165] = params[158];
                validate_finite_parameter("AGISL", params[165]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[166] = params[159];
                validate_finite_parameter("BGISL", params[166]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[167] = params[160];
                validate_finite_parameter("BGISL1", params[167]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[168] = params[161];
                validate_finite_parameter("CGISL", params[168]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[169] = params[162];
                validate_finite_parameter("RGISL", params[169]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[170] = params[163];
                validate_finite_parameter("KGISL", params[170]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[171] = params[164];
                validate_finite_parameter("FGISL", params[171]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(172), 1);
            {
                let params = &mut *ptr;
                params[173] = params[172];
                validate_parameter("NDIODED", params[173], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(174), 1);
            {
                let params = &mut *ptr;
                params[175] = params[174];
                validate_finite_parameter("XDIF", params[175]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 2] = [
                1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(176), 2);
            {
                let params = &mut *ptr;
                params[178] = params[175];
                validate_finite_parameter("XDIFD", params[178]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[179] = params[176];
                validate_finite_parameter("XRECD", params[179]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[180] = params[177];
                validate_finite_parameter("XTUND", params[180]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 1] = [
                0.7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(181), 1);
            {
                let params = &mut *ptr;
                params[182] = params[181];
                validate_parameter("PBSWGD", params[182], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(183), 1);
            {
                let params = &mut *ptr;
                params[184] = params[183];
                validate_finite_parameter("MJSWGD", params[184]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 1] = [
                1e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(185), 1);
            {
                let params = &mut *ptr;
                params[186] = params[185];
                validate_parameter("CJSWGD", params[186], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 29] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.6, 0.0, 1e-8, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(187), 29);
            {
                let params = &mut *ptr;
                params[216] = params[197];
                validate_finite_parameter("DWC", params[216]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[217] = params[187];
                validate_finite_parameter("DLC", params[217]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(218), 1);
            {
                let params = &mut *ptr;
                params[219] = if (params[37] == 1.0) { 6.25e41 } else { 6.188e40 };
                validate_finite_parameter("NOIA", params[219]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[220] = if (params[37] == 1.0) { 3.125e26 } else { 1.5e25 };
                validate_finite_parameter("NOIB", params[220]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 32] = [
                8750000000.0, 1.0, 0.0, 3.5, 0.395, 100000.0, 1.5, 3.5,
                0.577, 0.37, 1.0, 1e-6, 1e-6, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(221), 32);
            {
                let params = &mut *ptr;
                params[253] = params[251];
                validate_finite_parameter("STETA0CV", params[253]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[254] = params[252];
                validate_finite_parameter("LODETA0CV", params[254]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 68] = [
                1e-12, 2.0, 1e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1e-20, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                41000000.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.1, 0.9, 0.0, 0.0, 0.5,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.4, 0.0, 10000000.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(255), 68);
            {
                let params = &mut *ptr;
                params[323] = params[322];
                validate_parameter("NTUND", params[323], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 1] = [
                2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(324), 1);
            {
                let params = &mut *ptr;
                params[325] = params[324];
                validate_parameter("NRECF0D", params[325], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(326), 1);
            {
                let params = &mut *ptr;
                params[327] = params[326];
                validate_parameter("NRECR0D", params[327], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 1] = [
                1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(328), 1);
            {
                let params = &mut *ptr;
                params[329] = params[328];
                validate_parameter("IDBJT", params[329], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(330), 1);
            {
                let params = &mut *ptr;
                params[331] = params[330];
                validate_parameter("IDDIF", params[331], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                1e-5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(332), 1);
            {
                let params = &mut *ptr;
                params[333] = params[332];
                validate_parameter("IDREC", params[333], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(334), 1);
            {
                let params = &mut *ptr;
                params[335] = params[334];
                validate_parameter("IDTUN", params[335], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 2] = [
                2e-6, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(336), 2);
            {
                let params = &mut *ptr;
                params[338] = params[337];
                validate_finite_parameter("VREC0D", params[338]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (*ptr).values.as_mut_ptr().add(339), 1);
            {
                let params = &mut *ptr;
                params[340] = params[339];
                validate_finite_parameter("VTUN0D", params[340]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 6] = [
                1.0, 2e-7, 1.0, 10.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (*ptr).values.as_mut_ptr().add(341), 6);
            {
                let params = &mut *ptr;
                params[347] = params[346];
                validate_finite_parameter("AHLID", params[347]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 16] = [
                0.0, 0.0, 0.0, 1e-12, -1.0, 0.0, 0.0, 0.0,
                0.3, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (*ptr).values.as_mut_ptr().add(348), 16);
            {
                let params = &mut *ptr;
                params[364] = params[362];
                validate_finite_parameter("TCJSWGD", params[364]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[365] = params[363];
                validate_finite_parameter("TPBSWGD", params[365]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 3] = [
                1.0, 15.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (*ptr).values.as_mut_ptr().add(366), 3);
            {
                let params = &mut *ptr;
                params[369] = params[368];
                validate_parameter("NOFF2", params[369], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 6] = [
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (*ptr).values.as_mut_ptr().add(370), 6);
            {
                let params = &mut *ptr;
                params[376] = params[66];
                validate_parameter("TOXQM", params[376], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 21] = [
                0.0, 1000000000000000.0, 1.0, 2.5e-9, 1.2, 0.075, 0.35, 0.0,
                0.03, 300.0, 0.026, 0.43, 0.0, 0.05, 17.0, 0.043,
                0.0, 0.0054, 0.0075, 5.0, 0.005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (*ptr).values.as_mut_ptr().add(377), 21);
            {
                let params = &mut *ptr;
                params[398] = if (params[37] == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGC", params[398]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (*ptr).values.as_mut_ptr().add(399), 1);
            {
                let params = &mut *ptr;
                params[400] = if (params[37] == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGC", params[400]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[401] = if (params[37] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params[401]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[402] = if (params[37] == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGSD", params[402]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (*ptr).values.as_mut_ptr().add(403), 1);
            {
                let params = &mut *ptr;
                params[404] = if (params[37] == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGSD", params[404]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[405] = if (params[37] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGSD", params[405]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 4] = [
                1.0, 1.0, 1.0, 2.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (*ptr).values.as_mut_ptr().add(406), 4);
            {
                let params = &mut *ptr;
                params[410] = params[187];
                validate_finite_parameter("DLCIG", params[410]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 20] = [
                0.0, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1000.0, 12.0, 1.0, 0.1, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (*ptr).values.as_mut_ptr().add(411), 20);
            {
                let params = &mut *ptr;
                params[431] = 0.001;
                validate_parameter("MINR", params[431], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 41] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (*ptr).values.as_mut_ptr().add(432), 41);
            {
                let params = &mut *ptr;
                params[473] = params[470];
                validate_finite_parameter("LXDIFD", params[473]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[474] = params[471];
                validate_finite_parameter("LXRECD", params[474]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[475] = params[472];
                validate_finite_parameter("LXTUND", params[475]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 64] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (*ptr).values.as_mut_ptr().add(476), 64);
            {
                let params = &mut *ptr;
                params[540] = params[538];
                validate_finite_parameter("LETA0CV", params[540]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[541] = params[539];
                validate_finite_parameter("LETABCV", params[541]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 36] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (*ptr).values.as_mut_ptr().add(542), 36);
            {
                let params = &mut *ptr;
                params[578] = params[571];
                validate_finite_parameter("LAGISL", params[578]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[579] = params[572];
                validate_finite_parameter("LBGISL", params[579]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[580] = params[573];
                validate_finite_parameter("LBGISL1", params[580]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[581] = params[574];
                validate_finite_parameter("LCGISL", params[581]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[582] = params[575];
                validate_finite_parameter("LRGISL", params[582]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[583] = params[576];
                validate_finite_parameter("LKGISL", params[583]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[584] = params[577];
                validate_finite_parameter("LFGISL", params[584]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (*ptr).values.as_mut_ptr().add(585), 1);
            {
                let params = &mut *ptr;
                params[586] = params[585];
                validate_finite_parameter("LNTUND", params[586]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (*ptr).values.as_mut_ptr().add(587), 1);
            {
                let params = &mut *ptr;
                params[588] = params[587];
                validate_finite_parameter("LNDIODED", params[588]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (*ptr).values.as_mut_ptr().add(589), 1);
            {
                let params = &mut *ptr;
                params[590] = params[589];
                validate_finite_parameter("LNRECF0D", params[590]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (*ptr).values.as_mut_ptr().add(591), 1);
            {
                let params = &mut *ptr;
                params[592] = params[591];
                validate_finite_parameter("LNRECR0D", params[592]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (*ptr).values.as_mut_ptr().add(593), 1);
            {
                let params = &mut *ptr;
                params[594] = params[593];
                validate_finite_parameter("LIDBJT", params[594]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (*ptr).values.as_mut_ptr().add(595), 1);
            {
                let params = &mut *ptr;
                params[596] = params[595];
                validate_finite_parameter("LIDDIF", params[596]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (*ptr).values.as_mut_ptr().add(597), 1);
            {
                let params = &mut *ptr;
                params[598] = params[597];
                validate_finite_parameter("LIDREC", params[598]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (*ptr).values.as_mut_ptr().add(599), 1);
            {
                let params = &mut *ptr;
                params[600] = params[599];
                validate_finite_parameter("LIDTUN", params[600]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (*ptr).values.as_mut_ptr().add(601), 1);
            {
                let params = &mut *ptr;
                params[602] = params[601];
                validate_finite_parameter("LVREC0D", params[602]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (*ptr).values.as_mut_ptr().add(603), 1);
            {
                let params = &mut *ptr;
                params[604] = params[603];
                validate_finite_parameter("LVTUN0D", params[604]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (*ptr).values.as_mut_ptr().add(605), 5);
            {
                let params = &mut *ptr;
                params[610] = params[609];
                validate_finite_parameter("LAHLID", params[610]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (*ptr).values.as_mut_ptr().add(611), 6);
            {
                let params = &mut *ptr;
                params[617] = params[616];
                validate_finite_parameter("LNOFF2", params[617]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 45] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (*ptr).values.as_mut_ptr().add(618), 45);
            {
                let params = &mut *ptr;
                params[663] = params[660];
                validate_finite_parameter("WXDIFD", params[663]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[664] = params[661];
                validate_finite_parameter("WXRECD", params[664]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[665] = params[662];
                validate_finite_parameter("WXTUND", params[665]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 64] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (*ptr).values.as_mut_ptr().add(666), 64);
            {
                let params = &mut *ptr;
                params[730] = params[728];
                validate_finite_parameter("WETA0CV", params[730]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[731] = params[729];
                validate_finite_parameter("WETABCV", params[731]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 36] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (*ptr).values.as_mut_ptr().add(732), 36);
            {
                let params = &mut *ptr;
                params[768] = params[761];
                validate_finite_parameter("WAGISL", params[768]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[769] = params[762];
                validate_finite_parameter("WBGISL", params[769]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[770] = params[763];
                validate_finite_parameter("WBGISL1", params[770]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[771] = params[764];
                validate_finite_parameter("WCGISL", params[771]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[772] = params[765];
                validate_finite_parameter("WRGISL", params[772]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[773] = params[766];
                validate_finite_parameter("WKGISL", params[773]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[774] = params[767];
                validate_finite_parameter("WFGISL", params[774]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (*ptr).values.as_mut_ptr().add(775), 1);
            {
                let params = &mut *ptr;
                params[776] = params[775];
                validate_finite_parameter("WNTUND", params[776]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (*ptr).values.as_mut_ptr().add(777), 1);
            {
                let params = &mut *ptr;
                params[778] = params[777];
                validate_finite_parameter("WNDIODED", params[778]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (*ptr).values.as_mut_ptr().add(779), 1);
            {
                let params = &mut *ptr;
                params[780] = params[779];
                validate_finite_parameter("WNRECF0D", params[780]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (*ptr).values.as_mut_ptr().add(781), 1);
            {
                let params = &mut *ptr;
                params[782] = params[781];
                validate_finite_parameter("WNRECR0D", params[782]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (*ptr).values.as_mut_ptr().add(783), 1);
            {
                let params = &mut *ptr;
                params[784] = params[783];
                validate_finite_parameter("WIDBJT", params[784]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (*ptr).values.as_mut_ptr().add(785), 1);
            {
                let params = &mut *ptr;
                params[786] = params[785];
                validate_finite_parameter("WIDDIF", params[786]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (*ptr).values.as_mut_ptr().add(787), 1);
            {
                let params = &mut *ptr;
                params[788] = params[787];
                validate_finite_parameter("WIDREC", params[788]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (*ptr).values.as_mut_ptr().add(789), 1);
            {
                let params = &mut *ptr;
                params[790] = params[789];
                validate_finite_parameter("WIDTUN", params[790]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (*ptr).values.as_mut_ptr().add(791), 1);
            {
                let params = &mut *ptr;
                params[792] = params[791];
                validate_finite_parameter("WVREC0D", params[792]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (*ptr).values.as_mut_ptr().add(793), 1);
            {
                let params = &mut *ptr;
                params[794] = params[793];
                validate_finite_parameter("WVTUN0D", params[794]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (*ptr).values.as_mut_ptr().add(795), 5);
            {
                let params = &mut *ptr;
                params[800] = params[799];
                validate_finite_parameter("WAHLID", params[800]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (*ptr).values.as_mut_ptr().add(801), 6);
            {
                let params = &mut *ptr;
                params[807] = params[806];
                validate_finite_parameter("WNOFF2", params[807]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 45] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (*ptr).values.as_mut_ptr().add(808), 45);
            {
                let params = &mut *ptr;
                params[853] = params[850];
                validate_finite_parameter("PXDIFD", params[853]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[854] = params[851];
                validate_finite_parameter("PXRECD", params[854]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[855] = params[852];
                validate_finite_parameter("PXTUND", params[855]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 64] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (*ptr).values.as_mut_ptr().add(856), 64);
            {
                let params = &mut *ptr;
                params[920] = params[918];
                validate_finite_parameter("PETA0CV", params[920]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[921] = params[919];
                validate_finite_parameter("PETABCV", params[921]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 36] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (*ptr).values.as_mut_ptr().add(922), 36);
            {
                let params = &mut *ptr;
                params[958] = params[951];
                validate_finite_parameter("PAGISL", params[958]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[959] = params[952];
                validate_finite_parameter("PBGISL", params[959]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[960] = params[953];
                validate_finite_parameter("PBGISL1", params[960]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[961] = params[954];
                validate_finite_parameter("PCGISL", params[961]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[962] = params[955];
                validate_finite_parameter("PRGISL", params[962]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[963] = params[956];
                validate_finite_parameter("PKGISL", params[963]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[964] = params[957];
                validate_finite_parameter("PFGISL", params[964]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_71: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_71.as_ptr(), (*ptr).values.as_mut_ptr().add(965), 1);
            {
                let params = &mut *ptr;
                params[966] = params[965];
                validate_finite_parameter("PNTUND", params[966]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_72: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_72.as_ptr(), (*ptr).values.as_mut_ptr().add(967), 1);
            {
                let params = &mut *ptr;
                params[968] = params[967];
                validate_finite_parameter("PNDIODED", params[968]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_73: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_73.as_ptr(), (*ptr).values.as_mut_ptr().add(969), 1);
            {
                let params = &mut *ptr;
                params[970] = params[969];
                validate_finite_parameter("PNRECF0D", params[970]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_74: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_74.as_ptr(), (*ptr).values.as_mut_ptr().add(971), 1);
            {
                let params = &mut *ptr;
                params[972] = params[971];
                validate_finite_parameter("PNRECR0D", params[972]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_75: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_75.as_ptr(), (*ptr).values.as_mut_ptr().add(973), 1);
            {
                let params = &mut *ptr;
                params[974] = params[973];
                validate_finite_parameter("PIDBJT", params[974]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_76: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_76.as_ptr(), (*ptr).values.as_mut_ptr().add(975), 1);
            {
                let params = &mut *ptr;
                params[976] = params[975];
                validate_finite_parameter("PIDDIF", params[976]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_77: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_77.as_ptr(), (*ptr).values.as_mut_ptr().add(977), 1);
            {
                let params = &mut *ptr;
                params[978] = params[977];
                validate_finite_parameter("PIDREC", params[978]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_78: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_78.as_ptr(), (*ptr).values.as_mut_ptr().add(979), 1);
            {
                let params = &mut *ptr;
                params[980] = params[979];
                validate_finite_parameter("PIDTUN", params[980]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_79: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_79.as_ptr(), (*ptr).values.as_mut_ptr().add(981), 1);
            {
                let params = &mut *ptr;
                params[982] = params[981];
                validate_finite_parameter("PVREC0D", params[982]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_80: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_80.as_ptr(), (*ptr).values.as_mut_ptr().add(983), 1);
            {
                let params = &mut *ptr;
                params[984] = params[983];
                validate_finite_parameter("PVTUN0D", params[984]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_81: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_81.as_ptr(), (*ptr).values.as_mut_ptr().add(985), 5);
            {
                let params = &mut *ptr;
                params[990] = params[989];
                validate_finite_parameter("PAHLID", params[990]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_82: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_82.as_ptr(), (*ptr).values.as_mut_ptr().add(991), 6);
            {
                let params = &mut *ptr;
                params[997] = params[996];
                validate_finite_parameter("PNOFF2", params[997]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_83: [f64; 23] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.74e-7,
                0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_83.as_ptr(), (*ptr).values.as_mut_ptr().add(998), 23);
            {
                let params = &mut *ptr;
                params[1021] = params[1013];
                validate_finite_parameter("LPE0", params[1021]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1022] = params[1017];
                validate_finite_parameter("EGIDL", params[1022]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1023] = params[1022];
                validate_finite_parameter("EGISL", params[1023]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1024] = params[1014];
                validate_finite_parameter("LLPE0", params[1024]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1025] = params[1018];
                validate_finite_parameter("LEGIDL", params[1025]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1026] = params[1025];
                validate_finite_parameter("LEGISL", params[1026]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1027] = params[1015];
                validate_finite_parameter("WLPE0", params[1027]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1028] = params[1019];
                validate_finite_parameter("WEGIDL", params[1028]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1029] = params[1028];
                validate_finite_parameter("WEGISL", params[1029]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1030] = params[1016];
                validate_finite_parameter("PLPE0", params[1030]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1031] = params[1020];
                validate_finite_parameter("PEGIDL", params[1031]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1032] = params[1031];
                validate_finite_parameter("PEGISL", params[1032]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_84: [f64; 11] = [
                1.12, 1.12, 3.7622e-7, -31051000000.0, 4.9758e-7, -23570000000.0, 3.42537e-7, 4.97232e-7,
                1166450000000.0, 745669000000.0, 0.026,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_84.as_ptr(), (*ptr).values.as_mut_ptr().add(1033), 11);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1044] = [
    ("dtemp", 0), ("l", 1), ("w", 2), ("nf", 3), ("sa", 4), ("sb", 5), ("sd", 6), ("ad", 7), ("as", 8), ("pd", 9), ("ps", 10), ("nrd", 11), ("nrs", 12), ("bjtoff", 13), ("rth0", 14), ("cth0", 15),
    ("nrb", 16), ("frbody", 17), ("rbdb", 18), ("rbsb", 19), ("delvto", 20), ("soimod", 21), ("nbc", 22), ("nseg", 23), ("pdbcp", 24), ("psbcp", 25), ("agbcp", 26), ("agbcp2", 27), ("agbcpd", 28), ("aebcp", 29), ("ids0mult", 30), ("u0mult", 31),
    ("mult_i", 32), ("mult_q", 33), ("mult_fn", 34), ("tnodeout", 35), ("shmod", 36), ("type", 37), ("version", 38), ("rgatemod", 39), ("rbodymod", 40), ("mtrlmod", 41), ("vgstcvmod", 42), ("gidlmod", 43), ("iiimod", 44), ("eot", 45), ("epsrox", 46), ("epsrsub", 47),
    ("ni0sub", 48), ("bg0sub", 49), ("tbgasub", 50), ("tbgbsub", 51), ("phig", 52), ("easub", 53), ("leffeot", 54), ("weffeot", 55), ("vddeot", 56), ("tempeot", 57), ("ados", 58), ("bdos", 59), ("epsrgate", 60), ("capmod", 61), ("mobmod", 62), ("paramchk", 63),
    ("nodechk", 64), ("binunit", 65), ("tox", 66), ("toxm", 67), ("dtoxcv", 68), ("cdsc", 69), ("cdscb", 70), ("cdscd", 71), ("cit", 72), ("nfactor", 73), ("vsat", 74), ("at", 75), ("a0", 76), ("ags", 77), ("a1", 78), ("a2", 79),
    ("keta", 80), ("nsub", 81), ("nch", 82), ("ngate", 83), ("nsd", 84), ("gamma1", 85), ("gamma2", 86), ("vbx", 87), ("vbm", 88), ("xt", 89), ("k1", 90), ("kt1", 91), ("kt1l", 92), ("kt2", 93), ("k2", 94), ("k3", 95),
    ("k3b", 96), ("w0", 97), ("lpeb", 98), ("dvt0", 99), ("dvt1", 100), ("dvt2", 101), ("dvt0w", 102), ("dvt1w", 103), ("dvt2w", 104), ("drout", 105), ("dsub", 106), ("vtho", 107), ("vth0", 108), ("vfb", 109), ("ua", 110), ("ua1", 111),
    ("ub", 112), ("ub1", 113), ("uc", 114), ("uc1", 115), ("u0", 116), ("eu", 117), ("ute", 118), ("ucs", 119), ("ucste", 120), ("ud", 121), ("ud1", 122), ("ubg1", 123), ("ubg2", 124), ("voff", 125), ("tnom", 126), ("cgso", 127),
    ("cgdo", 128), ("xpart", 129), ("delta", 130), ("rsh", 131), ("rdsw", 132), ("rsw", 133), ("rdw", 134), ("rsc", 135), ("rdc", 136), ("trs", 137), ("trd", 138), ("rswmin", 139), ("rdwmin", 140), ("prwg", 141), ("prwb", 142), ("prwe", 143),
    ("prt", 144), ("eta0", 145), ("etab", 146), ("eta0cv", 147), ("etabcv", 148), ("pclm", 149), ("pdiblc1", 150), ("pdiblc2", 151), ("pdiblcb", 152), ("pvag", 153), ("tbox", 154), ("tsi", 155), ("etsi", 156), ("xj", 157), ("agidl", 158), ("bgidl", 159),
    ("bgidl1", 160), ("cgidl", 161), ("rgidl", 162), ("kgidl", 163), ("fgidl", 164), ("agisl", 165), ("bgisl", 166), ("bgisl1", 167), ("cgisl", 168), ("rgisl", 169), ("kgisl", 170), ("fgisl", 171), ("ndiode", 172), ("ndioded", 173), ("xbjt", 174), ("xdif", 175),
    ("xrec", 176), ("xtun", 177), ("xdifd", 178), ("xrecd", 179), ("xtund", 180), ("pbswg", 181), ("pbswgd", 182), ("mjswg", 183), ("mjswgd", 184), ("cjswg", 185), ("cjswgd", 186), ("lint", 187), ("ll", 188), ("llc", 189), ("lln", 190), ("lw", 191),
    ("lwc", 192), ("lwn", 193), ("lwl", 194), ("lwlc", 195), ("wr", 196), ("wint", 197), ("dwg", 198), ("dwb", 199), ("wl", 200), ("wlc", 201), ("wln", 202), ("ww", 203), ("wwc", 204), ("wwn", 205), ("wwl", 206), ("wwlc", 207),
    ("b0", 208), ("b1", 209), ("cgsl", 210), ("cgdl", 211), ("ckappa", 212), ("cf", 213), ("clc", 214), ("cle", 215), ("dwc", 216), ("dlc", 217), ("alpha0", 218), ("noia", 219), ("noib", 220), ("noic", 221), ("fnoimod", 222), ("tnoimod", 223),
    ("tnoic", 224), ("rnoic", 225), ("scalen", 226), ("tnoia", 227), ("tnoib", 228), ("rnoia", 229), ("rnoib", 230), ("ntnoi", 231), ("saref", 232), ("sbref", 233), ("wlod", 234), ("ku0", 235), ("kvsat", 236), ("kvth0", 237), ("tku0", 238), ("llodku0", 239),
    ("wlodku0", 240), ("llodvth", 241), ("wlodvth", 242), ("lku0", 243), ("wku0", 244), ("pku0", 245), ("lkvth0", 246), ("wkvth0", 247), ("pkvth0", 248), ("stk2", 249), ("lodk2", 250), ("steta0", 251), ("lodeta0", 252), ("steta0cv", 253), ("lodeta0cv", 254), ("gbmin", 255),
    ("bf", 256), ("w0flk", 257), ("dvtp0", 258), ("ldvtp0", 259), ("wdvtp0", 260), ("pdvtp0", 261), ("dvtp1", 262), ("ldvtp1", 263), ("wdvtp1", 264), ("pdvtp1", 265), ("dvtp2", 266), ("ldvtp2", 267), ("wdvtp2", 268), ("pdvtp2", 269), ("dvtp3", 270), ("ldvtp3", 271),
    ("wdvtp3", 272), ("pdvtp3", 273), ("dvtp4", 274), ("ldvtp4", 275), ("wdvtp4", 276), ("pdvtp4", 277), ("minv", 278), ("lminv", 279), ("wminv", 280), ("pminv", 281), ("pdits", 282), ("pditsl", 283), ("pditsd", 284), ("fprout", 285), ("lfprout", 286), ("lpdits", 287),
    ("lpditsd", 288), ("wfprout", 289), ("wpdits", 290), ("wpditsd", 291), ("pfprout", 292), ("ppdits", 293), ("ppditsd", 294), ("em", 295), ("ef", 296), ("af", 297), ("kf", 298), ("noif", 299), ("k1w1", 300), ("k1w2", 301), ("ketas", 302), ("dwbc", 303),
    ("beta0", 304), ("beta1", 305), ("beta2", 306), ("vdsatii0", 307), ("tii", 308), ("lii", 309), ("sii0", 310), ("sii1", 311), ("sii2", 312), ("siid", 313), ("fbjtii", 314), ("ebjtii", 315), ("cbjtii", 316), ("vbci", 317), ("abjtii", 318), ("mbjtii", 319),
    ("tvbci", 320), ("esatii", 321), ("ntun", 322), ("ntund", 323), ("nrecf0", 324), ("nrecf0d", 325), ("nrecr0", 326), ("nrecr0d", 327), ("isbjt", 328), ("idbjt", 329), ("isdif", 330), ("iddif", 331), ("isrec", 332), ("idrec", 333), ("istun", 334), ("idtun", 335),
    ("ln", 336), ("vrec0", 337), ("vrec0d", 338), ("vtun0", 339), ("vtun0d", 340), ("nbjt", 341), ("lbjt0", 342), ("ldif0", 343), ("vabjt", 344), ("aely", 345), ("ahli", 346), ("ahlid", 347), ("rbody", 348), ("rbsh", 349), ("cgeo", 350), ("tt", 351),
    ("ndif", 352), ("vsdfb", 353), ("vsdth", 354), ("csdmin", 355), ("asd", 356), ("csdesw", 357), ("ntrecf", 358), ("ntrecr", 359), ("dlcb", 360), ("fbody", 361), ("tcjswg", 362), ("tpbswg", 363), ("tcjswgd", 364), ("tpbswgd", 365), ("acde", 366), ("moin", 367),
    ("noff", 368), ("noff2", 369), ("delvt", 370), ("kb1", 371), ("dlbg", 372), ("cfrcoeff", 373), ("igbmod", 374), ("igcmod", 375), ("toxqm", 376), ("wth0", 377), ("rhalo", 378), ("ntox", 379), ("toxref", 380), ("ebg", 381), ("vevb", 382), ("alphagb1", 383),
    ("alphagb1_t", 384), ("betagb1", 385), ("vgb1", 386), ("vecb", 387), ("alphagb2", 388), ("alphagb2_t", 389), ("betagb2", 390), ("vgb2", 391), ("aigbcp2", 392), ("aigbcp2_t", 393), ("bigbcp2", 394), ("cigbcp2", 395), ("voxh", 396), ("deltavox", 397), ("aigc", 398), ("aigc1", 399),
    ("bigc", 400), ("cigc", 401), ("aigsd", 402), ("aigsd1", 403), ("bigsd", 404), ("cigsd", 405), ("nigc", 406), ("pigcd", 407), ("poxedge", 408), ("igt", 409), ("dlcig", 410), ("vbs0pd", 411), ("vbs0fd", 412), ("vbsa", 413), ("nofffd", 414), ("vofffd", 415),
    ("k1b", 416), ("k2b", 417), ("dk2b", 418), ("dvbd0", 419), ("dvbd1", 420), ("moinfd", 421), ("xrcrg1", 422), ("xrcrg2", 423), ("rshg", 424), ("ngcon", 425), ("rver", 426), ("xgw", 427), ("xgl", 428), ("rdsmod", 429), ("ids0multmod", 430), ("minr", 431),
    ("fdmod", 432), ("vsce", 433), ("cdsbs", 434), ("minvcv", 435), ("lminvcv", 436), ("wminvcv", 437), ("pminvcv", 438), ("voffcv", 439), ("lvoffcv", 440), ("wvoffcv", 441), ("pvoffcv", 442), ("lxj", 443), ("lalphagb1", 444), ("lalphagb1_t", 445), ("lbetagb1", 446), ("lalphagb2", 447),
    ("lalphagb2_t", 448), ("lbetagb2", 449), ("laigbcp2", 450), ("laigbcp2_t", 451), ("lbigbcp2", 452), ("lcigbcp2", 453), ("lcgsl", 454), ("lcgdl", 455), ("lckappa", 456), ("lndif", 457), ("lute", 458), ("lkt1", 459), ("lkt1l", 460), ("lkt2", 461), ("lua1", 462), ("lub1", 463),
    ("luc1", 464), ("lat", 465), ("lprt", 466), ("lntrecf", 467), ("lntrecr", 468), ("lxbjt", 469), ("lxdif", 470), ("lxrec", 471), ("lxtun", 472), ("lxdifd", 473), ("lxrecd", 474), ("lxtund", 475), ("laigc", 476), ("laigc1", 477), ("lbigc", 478), ("lcigc", 479),
    ("laigsd", 480), ("laigsd1", 481), ("lbigsd", 482), ("lcigsd", 483), ("lnigc", 484), ("lpigcd", 485), ("lpoxedge", 486), ("ligt", 487), ("lnch", 488), ("lnsub", 489), ("lngate", 490), ("lnsd", 491), ("lvth0", 492), ("lvfb", 493), ("lk1", 494), ("lk1w1", 495),
    ("lk1w2", 496), ("lk2", 497), ("lk3", 498), ("lk3b", 499), ("lkb1", 500), ("lw0", 501), ("llpeb", 502), ("ldvt0", 503), ("ldvt1", 504), ("ldvt2", 505), ("ldvt0w", 506), ("ldvt1w", 507), ("ldvt2w", 508), ("lu0", 509), ("leu", 510), ("lua", 511),
    ("lub", 512), ("luc", 513), ("lud", 514), ("lud1", 515), ("lucste", 516), ("lucs", 517), ("lvsat", 518), ("la0", 519), ("lags", 520), ("lb0", 521), ("lb1", 522), ("lketa", 523), ("lketas", 524), ("la1", 525), ("la2", 526), ("lrdsw", 527),
    ("lrsw", 528), ("lrdw", 529), ("lprwb", 530), ("lprwe", 531), ("lprwg", 532), ("lwr", 533), ("lnfactor", 534), ("ldwg", 535), ("ldwb", 536), ("lvoff", 537), ("leta0", 538), ("letab", 539), ("leta0cv", 540), ("letabcv", 541), ("ldsub", 542), ("lcit", 543),
    ("lcdsc", 544), ("lcdscb", 545), ("lcdscd", 546), ("lpclm", 547), ("lpdiblc1", 548), ("lpdiblc2", 549), ("lpdiblcb", 550), ("ldrout", 551), ("lpvag", 552), ("ldelta", 553), ("lalpha0", 554), ("lfbjtii", 555), ("labjtii", 556), ("lcbjtii", 557), ("lebjtii", 558), ("lmbjtii", 559),
    ("lvbci", 560), ("lbeta0", 561), ("lbeta1", 562), ("lbeta2", 563), ("lvdsatii0", 564), ("llii", 565), ("lesatii", 566), ("lsii0", 567), ("lsii1", 568), ("lsii2", 569), ("lsiid", 570), ("lagidl", 571), ("lbgidl", 572), ("lbgidl1", 573), ("lcgidl", 574), ("lrgidl", 575),
    ("lkgidl", 576), ("lfgidl", 577), ("lagisl", 578), ("lbgisl", 579), ("lbgisl1", 580), ("lcgisl", 581), ("lrgisl", 582), ("lkgisl", 583), ("lfgisl", 584), ("lntun", 585), ("lntund", 586), ("lndiode", 587), ("lndioded", 588), ("lnrecf0", 589), ("lnrecf0d", 590), ("lnrecr0", 591),
    ("lnrecr0d", 592), ("lisbjt", 593), ("lidbjt", 594), ("lisdif", 595), ("liddif", 596), ("lisrec", 597), ("lidrec", 598), ("listun", 599), ("lidtun", 600), ("lvrec0", 601), ("lvrec0d", 602), ("lvtun0", 603), ("lvtun0d", 604), ("lnbjt", 605), ("llbjt0", 606), ("lvabjt", 607),
    ("laely", 608), ("lahli", 609), ("lahlid", 610), ("lvsdfb", 611), ("lvsdth", 612), ("ldelvt", 613), ("lacde", 614), ("lmoin", 615), ("lnoff", 616), ("lnoff2", 617), ("lxrcrg1", 618), ("lxrcrg2", 619), ("lvbsa", 620), ("lvsce", 621), ("lcdsbs", 622), ("lnofffd", 623),
    ("lvofffd", 624), ("lk1b", 625), ("lk2b", 626), ("ldk2b", 627), ("ldvbd0", 628), ("ldvbd1", 629), ("lmoinfd", 630), ("lvbs0pd", 631), ("lvbs0fd", 632), ("wxj", 633), ("walphagb1", 634), ("walphagb1_t", 635), ("wbetagb1", 636), ("walphagb2", 637), ("walphagb2_t", 638), ("wbetagb2", 639),
    ("waigbcp2", 640), ("waigbcp2_t", 641), ("wbigbcp2", 642), ("wcigbcp2", 643), ("wcgsl", 644), ("wcgdl", 645), ("wckappa", 646), ("wndif", 647), ("wute", 648), ("wkt1", 649), ("wkt1l", 650), ("wkt2", 651), ("wua1", 652), ("wub1", 653), ("wuc1", 654), ("wat", 655),
    ("wprt", 656), ("wntrecf", 657), ("wntrecr", 658), ("wxbjt", 659), ("wxdif", 660), ("wxrec", 661), ("wxtun", 662), ("wxdifd", 663), ("wxrecd", 664), ("wxtund", 665), ("waigc", 666), ("waigc1", 667), ("wbigc", 668), ("wcigc", 669), ("waigsd", 670), ("waigsd1", 671),
    ("wbigsd", 672), ("wcigsd", 673), ("wnigc", 674), ("wpigcd", 675), ("wpoxedge", 676), ("wigt", 677), ("wnch", 678), ("wnsub", 679), ("wngate", 680), ("wnsd", 681), ("wvth0", 682), ("wvfb", 683), ("wk1", 684), ("wk1w1", 685), ("wk1w2", 686), ("wk2", 687),
    ("wk3", 688), ("wk3b", 689), ("wkb1", 690), ("ww0", 691), ("wlpeb", 692), ("wdvt0", 693), ("wdvt1", 694), ("wdvt2", 695), ("wdvt0w", 696), ("wdvt1w", 697), ("wdvt2w", 698), ("wu0", 699), ("weu", 700), ("wua", 701), ("wub", 702), ("wuc", 703),
    ("wud", 704), ("wud1", 705), ("wucste", 706), ("wucs", 707), ("wvsat", 708), ("wa0", 709), ("wags", 710), ("wb0", 711), ("wb1", 712), ("wketa", 713), ("wketas", 714), ("wa1", 715), ("wa2", 716), ("wrdsw", 717), ("wrsw", 718), ("wrdw", 719),
    ("wprwb", 720), ("wprwe", 721), ("wprwg", 722), ("wwr", 723), ("wnfactor", 724), ("wdwg", 725), ("wdwb", 726), ("wvoff", 727), ("weta0", 728), ("wetab", 729), ("weta0cv", 730), ("wetabcv", 731), ("wdsub", 732), ("wcit", 733), ("wcdsc", 734), ("wcdscb", 735),
    ("wcdscd", 736), ("wpclm", 737), ("wpdiblc1", 738), ("wpdiblc2", 739), ("wpdiblcb", 740), ("wdrout", 741), ("wpvag", 742), ("wdelta", 743), ("walpha0", 744), ("wfbjtii", 745), ("wabjtii", 746), ("wcbjtii", 747), ("webjtii", 748), ("wmbjtii", 749), ("wvbci", 750), ("wbeta0", 751),
    ("wbeta1", 752), ("wbeta2", 753), ("wvdsatii0", 754), ("wlii", 755), ("wesatii", 756), ("wsii0", 757), ("wsii1", 758), ("wsii2", 759), ("wsiid", 760), ("wagidl", 761), ("wbgidl", 762), ("wbgidl1", 763), ("wcgidl", 764), ("wrgidl", 765), ("wkgidl", 766), ("wfgidl", 767),
    ("wagisl", 768), ("wbgisl", 769), ("wbgisl1", 770), ("wcgisl", 771), ("wrgisl", 772), ("wkgisl", 773), ("wfgisl", 774), ("wntun", 775), ("wntund", 776), ("wndiode", 777), ("wndioded", 778), ("wnrecf0", 779), ("wnrecf0d", 780), ("wnrecr0", 781), ("wnrecr0d", 782), ("wisbjt", 783),
    ("widbjt", 784), ("wisdif", 785), ("widdif", 786), ("wisrec", 787), ("widrec", 788), ("wistun", 789), ("widtun", 790), ("wvrec0", 791), ("wvrec0d", 792), ("wvtun0", 793), ("wvtun0d", 794), ("wnbjt", 795), ("wlbjt0", 796), ("wvabjt", 797), ("waely", 798), ("wahli", 799),
    ("wahlid", 800), ("wvsdfb", 801), ("wvsdth", 802), ("wdelvt", 803), ("wacde", 804), ("wmoin", 805), ("wnoff", 806), ("wnoff2", 807), ("wxrcrg1", 808), ("wxrcrg2", 809), ("wvbsa", 810), ("wvsce", 811), ("wcdsbs", 812), ("wnofffd", 813), ("wvofffd", 814), ("wk1b", 815),
    ("wk2b", 816), ("wdk2b", 817), ("wdvbd0", 818), ("wdvbd1", 819), ("wmoinfd", 820), ("wvbs0pd", 821), ("wvbs0fd", 822), ("pxj", 823), ("palphagb1", 824), ("palphagb1_t", 825), ("pbetagb1", 826), ("palphagb2", 827), ("palphagb2_t", 828), ("pbetagb2", 829), ("paigbcp2", 830), ("paigbcp2_t", 831),
    ("pbigbcp2", 832), ("pcigbcp2", 833), ("pcgsl", 834), ("pcgdl", 835), ("pckappa", 836), ("pndif", 837), ("pute", 838), ("pkt1", 839), ("pkt1l", 840), ("pkt2", 841), ("pua1", 842), ("pub1", 843), ("puc1", 844), ("pat", 845), ("pprt", 846), ("pntrecf", 847),
    ("pntrecr", 848), ("pxbjt", 849), ("pxdif", 850), ("pxrec", 851), ("pxtun", 852), ("pxdifd", 853), ("pxrecd", 854), ("pxtund", 855), ("paigc", 856), ("paigc1", 857), ("pbigc", 858), ("pcigc", 859), ("paigsd", 860), ("paigsd1", 861), ("pbigsd", 862), ("pcigsd", 863),
    ("pnigc", 864), ("ppigcd", 865), ("ppoxedge", 866), ("pigt", 867), ("pnch", 868), ("pnsub", 869), ("pnsd", 870), ("pngate", 871), ("pvth0", 872), ("pvfb", 873), ("pk1", 874), ("pk1w1", 875), ("pk1w2", 876), ("pk2", 877), ("pk3", 878), ("pk3b", 879),
    ("pkb1", 880), ("pw0", 881), ("plpeb", 882), ("pdvt0", 883), ("pdvt1", 884), ("pdvt2", 885), ("pdvt0w", 886), ("pdvt1w", 887), ("pdvt2w", 888), ("pu0", 889), ("peu", 890), ("pua", 891), ("pub", 892), ("puc", 893), ("pud", 894), ("pud1", 895),
    ("pucste", 896), ("pucs", 897), ("pvsat", 898), ("pa0", 899), ("pags", 900), ("pb0", 901), ("pb1", 902), ("pketa", 903), ("pketas", 904), ("pa1", 905), ("pa2", 906), ("prdsw", 907), ("prsw", 908), ("prdw", 909), ("pprwb", 910), ("pprwe", 911),
    ("pprwg", 912), ("pwr", 913), ("pnfactor", 914), ("pdwg", 915), ("pdwb", 916), ("pvoff", 917), ("peta0", 918), ("petab", 919), ("peta0cv", 920), ("petabcv", 921), ("pdsub", 922), ("pcit", 923), ("pcdsc", 924), ("pcdscb", 925), ("pcdscd", 926), ("ppclm", 927),
    ("ppdiblc1", 928), ("ppdiblc2", 929), ("ppdiblcb", 930), ("pdrout", 931), ("ppvag", 932), ("pdelta", 933), ("palpha0", 934), ("pfbjtii", 935), ("pabjtii", 936), ("pcbjtii", 937), ("pebjtii", 938), ("pmbjtii", 939), ("pvbci", 940), ("pbeta0", 941), ("pbeta1", 942), ("pbeta2", 943),
    ("pvdsatii0", 944), ("plii", 945), ("pesatii", 946), ("psii0", 947), ("psii1", 948), ("psii2", 949), ("psiid", 950), ("pagidl", 951), ("pbgidl", 952), ("pbgidl1", 953), ("pcgidl", 954), ("prgidl", 955), ("pkgidl", 956), ("pfgidl", 957), ("pagisl", 958), ("pbgisl", 959),
    ("pbgisl1", 960), ("pcgisl", 961), ("prgisl", 962), ("pkgisl", 963), ("pfgisl", 964), ("pntun", 965), ("pntund", 966), ("pndiode", 967), ("pndioded", 968), ("pnrecf0", 969), ("pnrecf0d", 970), ("pnrecr0", 971), ("pnrecr0d", 972), ("pisbjt", 973), ("pidbjt", 974), ("pisdif", 975),
    ("piddif", 976), ("pisrec", 977), ("pidrec", 978), ("pistun", 979), ("pidtun", 980), ("pvrec0", 981), ("pvrec0d", 982), ("pvtun0", 983), ("pvtun0d", 984), ("pnbjt", 985), ("plbjt0", 986), ("pvabjt", 987), ("paely", 988), ("pahli", 989), ("pahlid", 990), ("pvsdfb", 991),
    ("pvsdth", 992), ("pdelvt", 993), ("pacde", 994), ("pmoin", 995), ("pnoff", 996), ("pnoff2", 997), ("pxrcrg1", 998), ("pxrcrg2", 999), ("pvbsa", 1000), ("pvsce", 1001), ("pcdsbs", 1002), ("pnofffd", 1003), ("pvofffd", 1004), ("pk1b", 1005), ("pk2b", 1006), ("pdk2b", 1007),
    ("pdvbd0", 1008), ("pdvbd1", 1009), ("pmoinfd", 1010), ("pvbs0pd", 1011), ("pvbs0fd", 1012), ("nlx", 1013), ("lnlx", 1014), ("wnlx", 1015), ("pnlx", 1016), ("ngidl", 1017), ("lngidl", 1018), ("wngidl", 1019), ("pngidl", 1020), ("lpe0", 1021), ("egidl", 1022), ("egisl", 1023),
    ("llpe0", 1024), ("legidl", 1025), ("legisl", 1026), ("wlpe0", 1027), ("wegidl", 1028), ("wegisl", 1029), ("plpe0", 1030), ("pegidl", 1031), ("pegisl", 1032), ("eggbcp2", 1033), ("eggdep", 1034), ("agb1", 1035), ("bgb1", 1036), ("agb2", 1037), ("bgb2", 1038), ("agbc2n", 1039),
    ("agbc2p", 1040), ("bgbc2n", 1041), ("bgbc2p", 1042), ("vtm00", 1043),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1044] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1044] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1044] = [
    "DTEMP", "L", "W", "NF", "SA", "SB", "SD", "AD", "AS", "PD", "PS", "NRD", "NRS", "BJTOFF", "RTH0", "CTH0",
    "NRB", "FRBODY", "RBDB", "RBSB", "DELVTO", "SOIMOD", "NBC", "NSEG", "PDBCP", "PSBCP", "AGBCP", "AGBCP2", "AGBCPD", "AEBCP", "IDS0MULT", "U0MULT",
    "MULT_I", "MULT_Q", "MULT_FN", "TNODEOUT", "SHMOD", "TYPE", "VERSION", "RGATEMOD", "RBODYMOD", "MTRLMOD", "VGSTCVMOD", "GIDLMOD", "IIIMOD", "EOT", "EPSROX", "EPSRSUB",
    "NI0SUB", "BG0SUB", "TBGASUB", "TBGBSUB", "PHIG", "EASUB", "LEFFEOT", "WEFFEOT", "VDDEOT", "TEMPEOT", "ADOS", "BDOS", "EPSRGATE", "CAPMOD", "MOBMOD", "PARAMCHK",
    "NODECHK", "BINUNIT", "TOX", "TOXM", "DTOXCV", "CDSC", "CDSCB", "CDSCD", "CIT", "NFACTOR", "VSAT", "AT", "A0", "AGS", "A1", "A2",
    "KETA", "NSUB", "NCH", "NGATE", "NSD", "GAMMA1", "GAMMA2", "VBX", "VBM", "XT", "K1", "KT1", "KT1L", "KT2", "K2", "K3",
    "K3B", "W0", "LPEB", "DVT0", "DVT1", "DVT2", "DVT0W", "DVT1W", "DVT2W", "DROUT", "DSUB", "VTHO", "VTH0", "VFB", "UA", "UA1",
    "UB", "UB1", "UC", "UC1", "U0", "EU", "UTE", "UCS", "UCSTE", "UD", "UD1", "UBG1", "UBG2", "VOFF", "TNOM", "CGSO",
    "CGDO", "XPART", "DELTA", "RSH", "RDSW", "RSW", "RDW", "RSC", "RDC", "TRS", "TRD", "RSWMIN", "RDWMIN", "PRWG", "PRWB", "PRWE",
    "PRT", "ETA0", "ETAB", "ETA0CV", "ETABCV", "PCLM", "PDIBLC1", "PDIBLC2", "PDIBLCB", "PVAG", "TBOX", "TSI", "ETSI", "XJ", "AGIDL", "BGIDL",
    "BGIDL1", "CGIDL", "RGIDL", "KGIDL", "FGIDL", "AGISL", "BGISL", "BGISL1", "CGISL", "RGISL", "KGISL", "FGISL", "NDIODE", "NDIODED", "XBJT", "XDIF",
    "XREC", "XTUN", "XDIFD", "XRECD", "XTUND", "PBSWG", "PBSWGD", "MJSWG", "MJSWGD", "CJSWG", "CJSWGD", "LINT", "LL", "LLC", "LLN", "LW",
    "LWC", "LWN", "LWL", "LWLC", "WR", "WINT", "DWG", "DWB", "WL", "WLC", "WLN", "WW", "WWC", "WWN", "WWL", "WWLC",
    "B0", "B1", "CGSL", "CGDL", "CKAPPA", "CF", "CLC", "CLE", "DWC", "DLC", "ALPHA0", "NOIA", "NOIB", "NOIC", "FNOIMOD", "TNOIMOD",
    "TNOIC", "RNOIC", "SCALEN", "TNOIA", "TNOIB", "RNOIA", "RNOIB", "NTNOI", "SAREF", "SBREF", "WLOD", "KU0", "KVSAT", "KVTH0", "TKU0", "LLODKU0",
    "WLODKU0", "LLODVTH", "WLODVTH", "LKU0", "WKU0", "PKU0", "LKVTH0", "WKVTH0", "PKVTH0", "STK2", "LODK2", "STETA0", "LODETA0", "STETA0CV", "LODETA0CV", "GBMIN",
    "BF", "W0FLK", "DVTP0", "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2", "PDVTP2", "DVTP3", "LDVTP3",
    "WDVTP3", "PDVTP3", "DVTP4", "LDVTP4", "WDVTP4", "PDVTP4", "MINV", "LMINV", "WMINV", "PMINV", "PDITS", "PDITSL", "PDITSD", "FPROUT", "LFPROUT", "LPDITS",
    "LPDITSD", "WFPROUT", "WPDITS", "WPDITSD", "PFPROUT", "PPDITS", "PPDITSD", "EM", "EF", "AF", "KF", "NOIF", "K1W1", "K1W2", "KETAS", "DWBC",
    "BETA0", "BETA1", "BETA2", "VDSATII0", "TII", "LII", "SII0", "SII1", "SII2", "SIID", "FBJTII", "EBJTII", "CBJTII", "VBCI", "ABJTII", "MBJTII",
    "TVBCI", "ESATII", "NTUN", "NTUND", "NRECF0", "NRECF0D", "NRECR0", "NRECR0D", "ISBJT", "IDBJT", "ISDIF", "IDDIF", "ISREC", "IDREC", "ISTUN", "IDTUN",
    "LN", "VREC0", "VREC0D", "VTUN0", "VTUN0D", "NBJT", "LBJT0", "LDIF0", "VABJT", "AELY", "AHLI", "AHLID", "RBODY", "RBSH", "CGEO", "TT",
    "NDIF", "VSDFB", "VSDTH", "CSDMIN", "ASD", "CSDESW", "NTRECF", "NTRECR", "DLCB", "FBODY", "TCJSWG", "TPBSWG", "TCJSWGD", "TPBSWGD", "ACDE", "MOIN",
    "NOFF", "NOFF2", "DELVT", "KB1", "DLBG", "CFRCOEFF", "IGBMOD", "IGCMOD", "TOXQM", "WTH0", "RHALO", "NTOX", "TOXREF", "EBG", "VEVB", "ALPHAGB1",
    "ALPHAGB1_T", "BETAGB1", "VGB1", "VECB", "ALPHAGB2", "ALPHAGB2_T", "BETAGB2", "VGB2", "AIGBCP2", "AIGBCP2_T", "BIGBCP2", "CIGBCP2", "VOXH", "DELTAVOX", "AIGC", "AIGC1",
    "BIGC", "CIGC", "AIGSD", "AIGSD1", "BIGSD", "CIGSD", "NIGC", "PIGCD", "POXEDGE", "IGT", "DLCIG", "VBS0PD", "VBS0FD", "VBSA", "NOFFFD", "VOFFFD",
    "K1B", "K2B", "DK2B", "DVBD0", "DVBD1", "MOINFD", "XRCRG1", "XRCRG2", "RSHG", "NGCON", "RVER", "XGW", "XGL", "RDSMOD", "IDS0MULTMOD", "MINR",
    "FDMOD", "VSCE", "CDSBS", "MINVCV", "LMINVCV", "WMINVCV", "PMINVCV", "VOFFCV", "LVOFFCV", "WVOFFCV", "PVOFFCV", "LXJ", "LALPHAGB1", "LALPHAGB1_T", "LBETAGB1", "LALPHAGB2",
    "LALPHAGB2_T", "LBETAGB2", "LAIGBCP2", "LAIGBCP2_T", "LBIGBCP2", "LCIGBCP2", "LCGSL", "LCGDL", "LCKAPPA", "LNDIF", "LUTE", "LKT1", "LKT1L", "LKT2", "LUA1", "LUB1",
    "LUC1", "LAT", "LPRT", "LNTRECF", "LNTRECR", "LXBJT", "LXDIF", "LXREC", "LXTUN", "LXDIFD", "LXRECD", "LXTUND", "LAIGC", "LAIGC1", "LBIGC", "LCIGC",
    "LAIGSD", "LAIGSD1", "LBIGSD", "LCIGSD", "LNIGC", "LPIGCD", "LPOXEDGE", "LIGT", "LNCH", "LNSUB", "LNGATE", "LNSD", "LVTH0", "LVFB", "LK1", "LK1W1",
    "LK1W2", "LK2", "LK3", "LK3B", "LKB1", "LW0", "LLPEB", "LDVT0", "LDVT1", "LDVT2", "LDVT0W", "LDVT1W", "LDVT2W", "LU0", "LEU", "LUA",
    "LUB", "LUC", "LUD", "LUD1", "LUCSTE", "LUCS", "LVSAT", "LA0", "LAGS", "LB0", "LB1", "LKETA", "LKETAS", "LA1", "LA2", "LRDSW",
    "LRSW", "LRDW", "LPRWB", "LPRWE", "LPRWG", "LWR", "LNFACTOR", "LDWG", "LDWB", "LVOFF", "LETA0", "LETAB", "LETA0CV", "LETABCV", "LDSUB", "LCIT",
    "LCDSC", "LCDSCB", "LCDSCD", "LPCLM", "LPDIBLC1", "LPDIBLC2", "LPDIBLCB", "LDROUT", "LPVAG", "LDELTA", "LALPHA0", "LFBJTII", "LABJTII", "LCBJTII", "LEBJTII", "LMBJTII",
    "LVBCI", "LBETA0", "LBETA1", "LBETA2", "LVDSATII0", "LLII", "LESATII", "LSII0", "LSII1", "LSII2", "LSIID", "LAGIDL", "LBGIDL", "LBGIDL1", "LCGIDL", "LRGIDL",
    "LKGIDL", "LFGIDL", "LAGISL", "LBGISL", "LBGISL1", "LCGISL", "LRGISL", "LKGISL", "LFGISL", "LNTUN", "LNTUND", "LNDIODE", "LNDIODED", "LNRECF0", "LNRECF0D", "LNRECR0",
    "LNRECR0D", "LISBJT", "LIDBJT", "LISDIF", "LIDDIF", "LISREC", "LIDREC", "LISTUN", "LIDTUN", "LVREC0", "LVREC0D", "LVTUN0", "LVTUN0D", "LNBJT", "LLBJT0", "LVABJT",
    "LAELY", "LAHLI", "LAHLID", "LVSDFB", "LVSDTH", "LDELVT", "LACDE", "LMOIN", "LNOFF", "LNOFF2", "LXRCRG1", "LXRCRG2", "LVBSA", "LVSCE", "LCDSBS", "LNOFFFD",
    "LVOFFFD", "LK1B", "LK2B", "LDK2B", "LDVBD0", "LDVBD1", "LMOINFD", "LVBS0PD", "LVBS0FD", "WXJ", "WALPHAGB1", "WALPHAGB1_T", "WBETAGB1", "WALPHAGB2", "WALPHAGB2_T", "WBETAGB2",
    "WAIGBCP2", "WAIGBCP2_T", "WBIGBCP2", "WCIGBCP2", "WCGSL", "WCGDL", "WCKAPPA", "WNDIF", "WUTE", "WKT1", "WKT1L", "WKT2", "WUA1", "WUB1", "WUC1", "WAT",
    "WPRT", "WNTRECF", "WNTRECR", "WXBJT", "WXDIF", "WXREC", "WXTUN", "WXDIFD", "WXRECD", "WXTUND", "WAIGC", "WAIGC1", "WBIGC", "WCIGC", "WAIGSD", "WAIGSD1",
    "WBIGSD", "WCIGSD", "WNIGC", "WPIGCD", "WPOXEDGE", "WIGT", "WNCH", "WNSUB", "WNGATE", "WNSD", "WVTH0", "WVFB", "WK1", "WK1W1", "WK1W2", "WK2",
    "WK3", "WK3B", "WKB1", "WW0", "WLPEB", "WDVT0", "WDVT1", "WDVT2", "WDVT0W", "WDVT1W", "WDVT2W", "WU0", "WEU", "WUA", "WUB", "WUC",
    "WUD", "WUD1", "WUCSTE", "WUCS", "WVSAT", "WA0", "WAGS", "WB0", "WB1", "WKETA", "WKETAS", "WA1", "WA2", "WRDSW", "WRSW", "WRDW",
    "WPRWB", "WPRWE", "WPRWG", "WWR", "WNFACTOR", "WDWG", "WDWB", "WVOFF", "WETA0", "WETAB", "WETA0CV", "WETABCV", "WDSUB", "WCIT", "WCDSC", "WCDSCB",
    "WCDSCD", "WPCLM", "WPDIBLC1", "WPDIBLC2", "WPDIBLCB", "WDROUT", "WPVAG", "WDELTA", "WALPHA0", "WFBJTII", "WABJTII", "WCBJTII", "WEBJTII", "WMBJTII", "WVBCI", "WBETA0",
    "WBETA1", "WBETA2", "WVDSATII0", "WLII", "WESATII", "WSII0", "WSII1", "WSII2", "WSIID", "WAGIDL", "WBGIDL", "WBGIDL1", "WCGIDL", "WRGIDL", "WKGIDL", "WFGIDL",
    "WAGISL", "WBGISL", "WBGISL1", "WCGISL", "WRGISL", "WKGISL", "WFGISL", "WNTUN", "WNTUND", "WNDIODE", "WNDIODED", "WNRECF0", "WNRECF0D", "WNRECR0", "WNRECR0D", "WISBJT",
    "WIDBJT", "WISDIF", "WIDDIF", "WISREC", "WIDREC", "WISTUN", "WIDTUN", "WVREC0", "WVREC0D", "WVTUN0", "WVTUN0D", "WNBJT", "WLBJT0", "WVABJT", "WAELY", "WAHLI",
    "WAHLID", "WVSDFB", "WVSDTH", "WDELVT", "WACDE", "WMOIN", "WNOFF", "WNOFF2", "WXRCRG1", "WXRCRG2", "WVBSA", "WVSCE", "WCDSBS", "WNOFFFD", "WVOFFFD", "WK1B",
    "WK2B", "WDK2B", "WDVBD0", "WDVBD1", "WMOINFD", "WVBS0PD", "WVBS0FD", "PXJ", "PALPHAGB1", "PALPHAGB1_T", "PBETAGB1", "PALPHAGB2", "PALPHAGB2_T", "PBETAGB2", "PAIGBCP2", "PAIGBCP2_T",
    "PBIGBCP2", "PCIGBCP2", "PCGSL", "PCGDL", "PCKAPPA", "PNDIF", "PUTE", "PKT1", "PKT1L", "PKT2", "PUA1", "PUB1", "PUC1", "PAT", "PPRT", "PNTRECF",
    "PNTRECR", "PXBJT", "PXDIF", "PXREC", "PXTUN", "PXDIFD", "PXRECD", "PXTUND", "PAIGC", "PAIGC1", "PBIGC", "PCIGC", "PAIGSD", "PAIGSD1", "PBIGSD", "PCIGSD",
    "PNIGC", "PPIGCD", "PPOXEDGE", "PIGT", "PNCH", "PNSUB", "PNSD", "PNGATE", "PVTH0", "PVFB", "PK1", "PK1W1", "PK1W2", "PK2", "PK3", "PK3B",
    "PKB1", "PW0", "PLPEB", "PDVT0", "PDVT1", "PDVT2", "PDVT0W", "PDVT1W", "PDVT2W", "PU0", "PEU", "PUA", "PUB", "PUC", "PUD", "PUD1",
    "PUCSTE", "PUCS", "PVSAT", "PA0", "PAGS", "PB0", "PB1", "PKETA", "PKETAS", "PA1", "PA2", "PRDSW", "PRSW", "PRDW", "PPRWB", "PPRWE",
    "PPRWG", "PWR", "PNFACTOR", "PDWG", "PDWB", "PVOFF", "PETA0", "PETAB", "PETA0CV", "PETABCV", "PDSUB", "PCIT", "PCDSC", "PCDSCB", "PCDSCD", "PPCLM",
    "PPDIBLC1", "PPDIBLC2", "PPDIBLCB", "PDROUT", "PPVAG", "PDELTA", "PALPHA0", "PFBJTII", "PABJTII", "PCBJTII", "PEBJTII", "PMBJTII", "PVBCI", "PBETA0", "PBETA1", "PBETA2",
    "PVDSATII0", "PLII", "PESATII", "PSII0", "PSII1", "PSII2", "PSIID", "PAGIDL", "PBGIDL", "PBGIDL1", "PCGIDL", "PRGIDL", "PKGIDL", "PFGIDL", "PAGISL", "PBGISL",
    "PBGISL1", "PCGISL", "PRGISL", "PKGISL", "PFGISL", "PNTUN", "PNTUND", "PNDIODE", "PNDIODED", "PNRECF0", "PNRECF0D", "PNRECR0", "PNRECR0D", "PISBJT", "PIDBJT", "PISDIF",
    "PIDDIF", "PISREC", "PIDREC", "PISTUN", "PIDTUN", "PVREC0", "PVREC0D", "PVTUN0", "PVTUN0D", "PNBJT", "PLBJT0", "PVABJT", "PAELY", "PAHLI", "PAHLID", "PVSDFB",
    "PVSDTH", "PDELVT", "PACDE", "PMOIN", "PNOFF", "PNOFF2", "PXRCRG1", "PXRCRG2", "PVBSA", "PVSCE", "PCDSBS", "PNOFFFD", "PVOFFFD", "PK1B", "PK2B", "PDK2B",
    "PDVBD0", "PDVBD1", "PMOINFD", "PVBS0PD", "PVBS0FD", "NLX", "LNLX", "WNLX", "PNLX", "NGIDL", "LNGIDL", "WNGIDL", "PNGIDL", "LPE0", "EGIDL", "EGISL",
    "LLPE0", "LEGIDL", "LEGISL", "WLPE0", "WEGIDL", "WEGISL", "PLPE0", "PEGIDL", "PEGISL", "EGGBCP2", "EGGDEP", "AGB1", "BGB1", "AGB2", "BGB2", "AGBC2N",
    "AGBC2P", "BGBC2N", "BGBC2P", "VTM00",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1044] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1044] = [
    false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1044] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
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
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1044] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
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
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
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
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
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
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1044] = [
    0, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
    2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 2, 3, 2, 2, 2, 2, 2, 0, 3, 0, 0, 3, 2, 2, 3, 0, 0, 0,
    0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 2, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    2, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 3, 3, 0, 0, 2, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2,
    0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 2, 2, 0, 3, 0, 3, 0,
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1044] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
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

fn canonical_boxed_zero_f64<const N: usize>() -> Box<[f64; N]> {
    // SAFETY: every slot is an f64, and all-zero bytes are 0.0.
    let mut boxed = Box::<[f64; N]>::new_uninit();
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

pub struct Instance {
    pub nodes: [usize; 14],
    pub branches: [usize; 19],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1044]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<22, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) canonical_reactive: Box<[f64; 162]>,
    pub(crate) canonical_staged: Box<[f64; 893]>,
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
            canonical_staged: self.canonical_staged.clone(),
            canonical_instance_valid: self.canonical_instance_valid,
            canonical_temperature_valid: self.canonical_temperature_valid,
            canonical_temperature: self.canonical_temperature,
            canonical_thermal_voltage: self.canonical_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 7;
    pub const INTERNAL_NODE_COUNT: usize = 7;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 7] = ["di", "si", "gi", "gm", "sb", "db", "N"];

    pub const BRANCH_COUNT: usize = 19;
    pub const PARAMETER_COUNT: usize = 1044;
    pub const VARIABLE_COUNT: usize = 1569;
    pub const DDT_STATE_COUNT: usize = 22;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "f236f46ab49a993b0ab967be5b9cb3bb564e966a5a22b159cea4a375332faa55";
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
            canonical_reactive: canonical_boxed_zero_f64(),
            canonical_staged: canonical_boxed_zero_f64(),
            canonical_instance_valid: false,
            canonical_temperature_valid: false,
            canonical_temperature: 0.0,
            canonical_thermal_voltage: 0.0,
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(110);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(22);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 110);
        debug_assert_eq!(state.flags.len(), 22);
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
