#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 996],
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
                0.0, 5e-6, 5e-6, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 1e-5, 1.0, 1.0, 50.0, 50.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 30);
            {
                let params = &mut *ptr;
                params[30] = params[28];
                validate_parameter("AGBCPD", params[30], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 9] = [
                0.0, 0.0, 0.0, 1.0, 4.6, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(31), 9);
            {
                let params = &mut *ptr;
                params[40] = if (params[35] >= 4.2) { 1.0 } else { 0.0 };
                validate_parameter("VGSTCVMOD", params[40], true, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 13] = [
                0.0, 0.0, 1e-8, 3.9, 11.7, 14500000000.0, 1.16, 0.000702,
                1108.0, 4.05, 4.05, 1.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(41), 13);
            {
                let params = &mut *ptr;
                params[54] = if (params[34] == 1.0) { 1.5 } else { (-1.5) };
                validate_finite_parameter("VDDEOT", params[54]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 10] = [
                300.15, 1.0, 1.0, 11.7, 2.0, 1.0, 0.0, 1.0,
                1.0, 1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(55), 10);
            {
                let params = &mut *ptr;
                params[65] = params[64];
                validate_parameter("TOXP", params[65], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[66] = params[64];
                validate_parameter("TOXM", params[66], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 38] = [
                0.0, 0.00024, 0.0, 0.0, 0.0, 1.0, 80000.0, 33000.0,
                1.0, 0.0, 0.0, 1.0, -0.6, 6e16, 1.7e17, 0.0,
                1e20, 0.0, 0.0, 0.0, -3.0, 1.55e-7, 0.53, -0.11,
                0.0, 0.022, -0.0186, 0.0, 0.0, 2.5e-6, 0.0, 2.2,
                0.53, -0.032, 0.0, 5300000.0, -0.032, 0.56,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(67), 38);
            {
                let params = &mut *ptr;
                params[105] = params[104];
                validate_finite_parameter("DSUB", params[105]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[106] = if (params[34] == 1.0) { 0.7 } else { (-0.7) };
                validate_finite_parameter("VTHO", params[106]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[107] = params[106];
                validate_finite_parameter("VTH0", params[107]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 5] = [
                -1.0, 2.25e-9, 4.31e-9, 5.87e-19, -7.61e-18,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(108), 5);
            {
                let params = &mut *ptr;
                params[113] = if (params[60] == 3.0) { (-0.0465) } else { (-4.65e-11) };
                validate_finite_parameter("UC", params[113]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[114] = if (params[60] == 3.0) { (-0.056) } else { (-5.6e-11) };
                validate_finite_parameter("UC1", params[114]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[115] = if (params[34] == 1.0) { 0.067 } else { 0.025 };
                validate_finite_parameter("U0", params[115]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[116] = if (params[34] == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("EU", params[116]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 1] = [
                -1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(117), 1);
            {
                let params = &mut *ptr;
                params[118] = if (params[34] == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("UCS", params[118]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 20] = [
                -0.004775, 0.0, 0.0, -0.08, 27.0, 0.0, 0.0, 0.0,
                0.01, 0.0, 100.0, 50.0, 50.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.08, -0.07,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(119), 20);
            {
                let params = &mut *ptr;
                params[139] = params[137];
                validate_finite_parameter("ETA0CV", params[139]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[140] = params[138];
                validate_finite_parameter("ETABCV", params[140]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 8] = [
                1.3, 0.39, 0.0086, 0.0, 0.0, 3e-7, 1e-7, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(141), 8);
            {
                let params = &mut *ptr;
                params[149] = params[147];
                validate_parameter("XJ", params[149], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 6] = [
                0.0, 2300000000.0, 0.5, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(150), 6);
            {
                let params = &mut *ptr;
                params[156] = params[150];
                validate_finite_parameter("AGISL", params[156]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[157] = params[151];
                validate_finite_parameter("BGISL", params[157]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[158] = params[152];
                validate_finite_parameter("CGISL", params[158]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[159] = params[153];
                validate_finite_parameter("RGISL", params[159]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[160] = params[154];
                validate_finite_parameter("KGISL", params[160]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[161] = params[155];
                validate_finite_parameter("FGISL", params[161]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(162), 1);
            {
                let params = &mut *ptr;
                params[163] = params[162];
                validate_finite_parameter("NDIODED", params[163]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(164), 1);
            {
                let params = &mut *ptr;
                params[165] = params[164];
                validate_finite_parameter("XDIF", params[165]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 2] = [
                1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(166), 2);
            {
                let params = &mut *ptr;
                params[168] = params[165];
                validate_finite_parameter("XDIFD", params[168]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[169] = params[166];
                validate_finite_parameter("XRECD", params[169]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[170] = params[167];
                validate_finite_parameter("XTUND", params[170]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 1] = [
                0.7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(171), 1);
            {
                let params = &mut *ptr;
                params[172] = params[171];
                validate_finite_parameter("PBSWGD", params[172]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(173), 1);
            {
                let params = &mut *ptr;
                params[174] = params[173];
                validate_finite_parameter("MJSWGD", params[174]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 1] = [
                1e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(175), 1);
            {
                let params = &mut *ptr;
                params[176] = params[175];
                validate_parameter("CJSWGD", params[176], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 29] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.6, 0.0, 1e-8, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(177), 29);
            {
                let params = &mut *ptr;
                params[206] = params[187];
                validate_finite_parameter("DWC", params[206]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[207] = params[177];
                validate_finite_parameter("DLC", params[207]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(208), 1);
            {
                let params = &mut *ptr;
                params[209] = if (params[34] == 1.0) { 6.25e41 } else { 6.188e40 };
                validate_finite_parameter("NOIA", params[209]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[210] = if (params[34] == 1.0) { 3.125e26 } else { 1.5e25 };
                validate_finite_parameter("NOIB", params[210]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 29] = [
                8750000000.0, 1.0, 0.0, 1.5, 3.5, 0.577, 0.37, 1.0,
                1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(211), 29);
            {
                let params = &mut *ptr;
                params[240] = params[238];
                validate_finite_parameter("STETA0CV", params[240]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[241] = params[239];
                validate_finite_parameter("LODETA0CV", params[241]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 68] = [
                1e-12, 2.0, 1e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1e-20, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                41000000.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.1, 0.9, 0.0, 0.0, 0.5,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.4, 0.0, 10000000.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(242), 68);
            {
                let params = &mut *ptr;
                params[310] = params[309];
                validate_parameter("NTUND", params[310], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 1] = [
                2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(311), 1);
            {
                let params = &mut *ptr;
                params[312] = params[311];
                validate_parameter("NRECF0D", params[312], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(313), 1);
            {
                let params = &mut *ptr;
                params[314] = params[313];
                validate_parameter("NRECR0D", params[314], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 1] = [
                1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(315), 1);
            {
                let params = &mut *ptr;
                params[316] = params[315];
                validate_parameter("IDBJT", params[316], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(317), 1);
            {
                let params = &mut *ptr;
                params[318] = params[317];
                validate_parameter("IDDIF", params[318], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 1] = [
                1e-5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(319), 1);
            {
                let params = &mut *ptr;
                params[320] = params[319];
                validate_parameter("IDREC", params[320], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(321), 1);
            {
                let params = &mut *ptr;
                params[322] = params[321];
                validate_parameter("IDTUN", params[322], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 2] = [
                2e-6, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(323), 2);
            {
                let params = &mut *ptr;
                params[325] = params[324];
                validate_finite_parameter("VREC0D", params[325]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(326), 1);
            {
                let params = &mut *ptr;
                params[327] = params[326];
                validate_finite_parameter("VTUN0D", params[327]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 6] = [
                1.0, 2e-7, 1.0, 10.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (*ptr).values.as_mut_ptr().add(328), 6);
            {
                let params = &mut *ptr;
                params[334] = params[333];
                validate_finite_parameter("AHLID", params[334]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 16] = [
                0.0, 0.0, 0.0, 1e-12, -1.0, 0.0, 0.0, 0.0,
                0.3, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (*ptr).values.as_mut_ptr().add(335), 16);
            {
                let params = &mut *ptr;
                params[351] = params[349];
                validate_finite_parameter("TCJSWGD", params[351]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[352] = params[350];
                validate_finite_parameter("TPBSWGD", params[352]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 3] = [
                1.0, 15.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (*ptr).values.as_mut_ptr().add(353), 3);
            {
                let params = &mut *ptr;
                params[356] = params[355];
                validate_parameter("NOFF2", params[356], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 5] = [
                0.0, 1.0, 0.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (*ptr).values.as_mut_ptr().add(357), 5);
            {
                let params = &mut *ptr;
                params[362] = params[361];
                validate_parameter("IGMOD", params[362], true, None, false, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (*ptr).values.as_mut_ptr().add(363), 1);
            {
                let params = &mut *ptr;
                params[364] = params[64];
                validate_finite_parameter("TOXQM", params[364]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 18] = [
                0.0, 1000000000000000.0, 1.0, 2.5e-9, 1.2, 0.075, 0.35, 0.03,
                300.0, 0.026, 0.43, 0.05, 17.0, 0.043, 0.0054, 0.0075,
                5.0, 0.005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (*ptr).values.as_mut_ptr().add(365), 18);
            {
                let params = &mut *ptr;
                params[383] = if (params[34] == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGC", params[383]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[384] = if (params[34] == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGC", params[384]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[385] = if (params[34] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params[385]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[386] = if (params[34] == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGSD", params[386]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[387] = if (params[34] == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGSD", params[387]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[388] = if (params[34] == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGSD", params[388]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 3] = [
                1.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (*ptr).values.as_mut_ptr().add(389), 3);
            {
                let params = &mut *ptr;
                params[392] = params[177];
                validate_finite_parameter("DLCIG", params[392]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 56] = [
                0.0, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1000.0, 12.0, 1.0, 0.1, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (*ptr).values.as_mut_ptr().add(393), 56);
            {
                let params = &mut *ptr;
                params[449] = params[446];
                validate_finite_parameter("LXDIFD", params[449]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[450] = params[447];
                validate_finite_parameter("LXRECD", params[450]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[451] = params[448];
                validate_finite_parameter("LXTUND", params[451]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 60] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (*ptr).values.as_mut_ptr().add(452), 60);
            {
                let params = &mut *ptr;
                params[512] = params[510];
                validate_finite_parameter("LETA0CV", params[512]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[513] = params[511];
                validate_finite_parameter("LETABCV", params[513]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (*ptr).values.as_mut_ptr().add(514), 35);
            {
                let params = &mut *ptr;
                params[549] = params[543];
                validate_finite_parameter("LAGISL", params[549]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[550] = params[544];
                validate_finite_parameter("LBGISL", params[550]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[551] = params[545];
                validate_finite_parameter("LCGISL", params[551]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[552] = params[546];
                validate_finite_parameter("LRGISL", params[552]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[553] = params[547];
                validate_finite_parameter("LKGISL", params[553]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[554] = params[548];
                validate_finite_parameter("LFGISL", params[554]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (*ptr).values.as_mut_ptr().add(555), 1);
            {
                let params = &mut *ptr;
                params[556] = params[555];
                validate_finite_parameter("LNTUND", params[556]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (*ptr).values.as_mut_ptr().add(557), 1);
            {
                let params = &mut *ptr;
                params[558] = params[557];
                validate_finite_parameter("LNDIODED", params[558]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (*ptr).values.as_mut_ptr().add(559), 1);
            {
                let params = &mut *ptr;
                params[560] = params[559];
                validate_finite_parameter("LNRECF0D", params[560]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (*ptr).values.as_mut_ptr().add(561), 1);
            {
                let params = &mut *ptr;
                params[562] = params[561];
                validate_finite_parameter("LNRECR0D", params[562]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (*ptr).values.as_mut_ptr().add(563), 1);
            {
                let params = &mut *ptr;
                params[564] = params[563];
                validate_finite_parameter("LIDBJT", params[564]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (*ptr).values.as_mut_ptr().add(565), 1);
            {
                let params = &mut *ptr;
                params[566] = params[565];
                validate_finite_parameter("LIDDIF", params[566]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (*ptr).values.as_mut_ptr().add(567), 1);
            {
                let params = &mut *ptr;
                params[568] = params[567];
                validate_finite_parameter("LIDREC", params[568]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (*ptr).values.as_mut_ptr().add(569), 1);
            {
                let params = &mut *ptr;
                params[570] = params[569];
                validate_finite_parameter("LIDTUN", params[570]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (*ptr).values.as_mut_ptr().add(571), 1);
            {
                let params = &mut *ptr;
                params[572] = params[571];
                validate_finite_parameter("LVREC0D", params[572]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (*ptr).values.as_mut_ptr().add(573), 1);
            {
                let params = &mut *ptr;
                params[574] = params[573];
                validate_finite_parameter("LVTUN0D", params[574]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (*ptr).values.as_mut_ptr().add(575), 5);
            {
                let params = &mut *ptr;
                params[580] = params[579];
                validate_finite_parameter("LAHLID", params[580]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (*ptr).values.as_mut_ptr().add(581), 6);
            {
                let params = &mut *ptr;
                params[587] = params[586];
                validate_finite_parameter("LNOFF2", params[587]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 42] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (*ptr).values.as_mut_ptr().add(588), 42);
            {
                let params = &mut *ptr;
                params[630] = params[627];
                validate_finite_parameter("WXDIFD", params[630]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[631] = params[628];
                validate_finite_parameter("WXRECD", params[631]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[632] = params[629];
                validate_finite_parameter("WXTUND", params[632]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 60] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (*ptr).values.as_mut_ptr().add(633), 60);
            {
                let params = &mut *ptr;
                params[693] = params[691];
                validate_finite_parameter("WETA0CV", params[693]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[694] = params[692];
                validate_finite_parameter("WETABCV", params[694]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (*ptr).values.as_mut_ptr().add(695), 35);
            {
                let params = &mut *ptr;
                params[730] = params[724];
                validate_finite_parameter("WAGISL", params[730]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[731] = params[725];
                validate_finite_parameter("WBGISL", params[731]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[732] = params[726];
                validate_finite_parameter("WCGISL", params[732]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[733] = params[727];
                validate_finite_parameter("WRGISL", params[733]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[734] = params[728];
                validate_finite_parameter("WKGISL", params[734]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[735] = params[729];
                validate_finite_parameter("WFGISL", params[735]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (*ptr).values.as_mut_ptr().add(736), 1);
            {
                let params = &mut *ptr;
                params[737] = params[736];
                validate_finite_parameter("WNTUND", params[737]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (*ptr).values.as_mut_ptr().add(738), 1);
            {
                let params = &mut *ptr;
                params[739] = params[738];
                validate_finite_parameter("WNDIODED", params[739]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (*ptr).values.as_mut_ptr().add(740), 1);
            {
                let params = &mut *ptr;
                params[741] = params[740];
                validate_finite_parameter("WNRECF0D", params[741]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (*ptr).values.as_mut_ptr().add(742), 1);
            {
                let params = &mut *ptr;
                params[743] = params[742];
                validate_finite_parameter("WNRECR0D", params[743]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (*ptr).values.as_mut_ptr().add(744), 1);
            {
                let params = &mut *ptr;
                params[745] = params[744];
                validate_finite_parameter("WIDBJT", params[745]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (*ptr).values.as_mut_ptr().add(746), 1);
            {
                let params = &mut *ptr;
                params[747] = params[746];
                validate_finite_parameter("WIDDIF", params[747]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (*ptr).values.as_mut_ptr().add(748), 1);
            {
                let params = &mut *ptr;
                params[749] = params[748];
                validate_finite_parameter("WIDREC", params[749]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (*ptr).values.as_mut_ptr().add(750), 1);
            {
                let params = &mut *ptr;
                params[751] = params[750];
                validate_finite_parameter("WIDTUN", params[751]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (*ptr).values.as_mut_ptr().add(752), 1);
            {
                let params = &mut *ptr;
                params[753] = params[752];
                validate_finite_parameter("WVREC0D", params[753]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (*ptr).values.as_mut_ptr().add(754), 1);
            {
                let params = &mut *ptr;
                params[755] = params[754];
                validate_finite_parameter("WVTUN0D", params[755]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (*ptr).values.as_mut_ptr().add(756), 5);
            {
                let params = &mut *ptr;
                params[761] = params[760];
                validate_finite_parameter("WAHLID", params[761]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (*ptr).values.as_mut_ptr().add(762), 6);
            {
                let params = &mut *ptr;
                params[768] = params[767];
                validate_finite_parameter("WNOFF2", params[768]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 42] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (*ptr).values.as_mut_ptr().add(769), 42);
            {
                let params = &mut *ptr;
                params[811] = params[808];
                validate_finite_parameter("PXDIFD", params[811]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[812] = params[809];
                validate_finite_parameter("PXRECD", params[812]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[813] = params[810];
                validate_finite_parameter("PXTUND", params[813]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 60] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (*ptr).values.as_mut_ptr().add(814), 60);
            {
                let params = &mut *ptr;
                params[874] = params[872];
                validate_finite_parameter("PETA0CV", params[874]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[875] = params[873];
                validate_finite_parameter("PETABCV", params[875]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (*ptr).values.as_mut_ptr().add(876), 35);
            {
                let params = &mut *ptr;
                params[911] = params[905];
                validate_finite_parameter("PAGISL", params[911]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[912] = params[906];
                validate_finite_parameter("PBGISL", params[912]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[913] = params[907];
                validate_finite_parameter("PCGISL", params[913]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[914] = params[908];
                validate_finite_parameter("PRGISL", params[914]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[915] = params[909];
                validate_finite_parameter("PKGISL", params[915]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[916] = params[910];
                validate_finite_parameter("PFGISL", params[916]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (*ptr).values.as_mut_ptr().add(917), 1);
            {
                let params = &mut *ptr;
                params[918] = params[917];
                validate_finite_parameter("PNTUND", params[918]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (*ptr).values.as_mut_ptr().add(919), 1);
            {
                let params = &mut *ptr;
                params[920] = params[919];
                validate_finite_parameter("PNDIODED", params[920]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (*ptr).values.as_mut_ptr().add(921), 1);
            {
                let params = &mut *ptr;
                params[922] = params[921];
                validate_finite_parameter("PNRECF0D", params[922]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_71: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_71.as_ptr(), (*ptr).values.as_mut_ptr().add(923), 1);
            {
                let params = &mut *ptr;
                params[924] = params[923];
                validate_finite_parameter("PNRECR0D", params[924]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_72: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_72.as_ptr(), (*ptr).values.as_mut_ptr().add(925), 1);
            {
                let params = &mut *ptr;
                params[926] = params[925];
                validate_finite_parameter("PIDBJT", params[926]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_73: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_73.as_ptr(), (*ptr).values.as_mut_ptr().add(927), 1);
            {
                let params = &mut *ptr;
                params[928] = params[927];
                validate_finite_parameter("PIDDIF", params[928]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_74: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_74.as_ptr(), (*ptr).values.as_mut_ptr().add(929), 1);
            {
                let params = &mut *ptr;
                params[930] = params[929];
                validate_finite_parameter("PIDREC", params[930]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_75: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_75.as_ptr(), (*ptr).values.as_mut_ptr().add(931), 1);
            {
                let params = &mut *ptr;
                params[932] = params[931];
                validate_finite_parameter("PIDTUN", params[932]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_76: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_76.as_ptr(), (*ptr).values.as_mut_ptr().add(933), 1);
            {
                let params = &mut *ptr;
                params[934] = params[933];
                validate_finite_parameter("PVREC0D", params[934]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_77: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_77.as_ptr(), (*ptr).values.as_mut_ptr().add(935), 1);
            {
                let params = &mut *ptr;
                params[936] = params[935];
                validate_finite_parameter("PVTUN0D", params[936]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_78: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_78.as_ptr(), (*ptr).values.as_mut_ptr().add(937), 5);
            {
                let params = &mut *ptr;
                params[942] = params[941];
                validate_finite_parameter("PAHLID", params[942]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_79: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_79.as_ptr(), (*ptr).values.as_mut_ptr().add(943), 6);
            {
                let params = &mut *ptr;
                params[949] = params[948];
                validate_finite_parameter("PNOFF2", params[949]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_80: [f64; 23] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.74e-7,
                0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_80.as_ptr(), (*ptr).values.as_mut_ptr().add(950), 23);
            {
                let params = &mut *ptr;
                params[973] = params[965];
                validate_finite_parameter("LPE0", params[973]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[974] = params[969];
                validate_finite_parameter("EGIDL", params[974]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[975] = params[974];
                validate_finite_parameter("EGISL", params[975]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[976] = params[966];
                validate_finite_parameter("LLPE0", params[976]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[977] = params[970];
                validate_finite_parameter("LEGIDL", params[977]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[978] = params[977];
                validate_finite_parameter("LEGISL", params[978]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[979] = params[967];
                validate_finite_parameter("WLPE0", params[979]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[980] = params[971];
                validate_finite_parameter("WEGIDL", params[980]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[981] = params[980];
                validate_finite_parameter("WEGISL", params[981]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[982] = params[968];
                validate_finite_parameter("PLPE0", params[982]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[983] = params[972];
                validate_finite_parameter("PEGIDL", params[983]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[984] = params[983];
                validate_finite_parameter("PEGISL", params[984]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_81: [f64; 11] = [
                1.12, 1.12, 3.7622e-7, -31051000000.0, 4.9758e-7, -23570000000.0, 3.4254e-7, 4.9723e-7,
                1166500000000.0, 745670000000.0, 0.026,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_81.as_ptr(), (*ptr).values.as_mut_ptr().add(985), 11);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 996] = [
    ("dtemp", 0), ("l", 1), ("w", 2), ("nf", 3), ("sa", 4), ("sb", 5), ("sd", 6), ("ad", 7), ("as", 8), ("pd", 9), ("ps", 10), ("nrd", 11), ("nrs", 12), ("off", 13), ("bjtoff", 14), ("debug", 15),
    ("rth0", 16), ("cth0", 17), ("nrb", 18), ("frbody", 19), ("rbdb", 20), ("rbsb", 21), ("delvto", 22), ("soimod", 23), ("nbc", 24), ("nseg", 25), ("pdbcp", 26), ("psbcp", 27), ("agbcp", 28), ("agbcp2", 29), ("agbcpd", 30), ("aebcp", 31),
    ("tnodeout", 32), ("shmod", 33), ("type", 34), ("version", 35), ("vbsusr", 36), ("rgatemod", 37), ("rbodymod", 38), ("mtrlmod", 39), ("vgstcvmod", 40), ("gidlmod", 41), ("iiimod", 42), ("eot", 43), ("epsrox", 44), ("epsrsub", 45), ("ni0sub", 46), ("bg0sub", 47),
    ("tbgasub", 48), ("tbgbsub", 49), ("phig", 50), ("easub", 51), ("leffeot", 52), ("weffeot", 53), ("vddeot", 54), ("tempeot", 55), ("ados", 56), ("bdos", 57), ("epsrgate", 58), ("capmod", 59), ("mobmod", 60), ("paramchk", 61), ("nodechk", 62), ("binunit", 63),
    ("tox", 64), ("toxp", 65), ("toxm", 66), ("dtoxcv", 67), ("cdsc", 68), ("cdscb", 69), ("cdscd", 70), ("cit", 71), ("nfactor", 72), ("vsat", 73), ("at", 74), ("a0", 75), ("ags", 76), ("a1", 77), ("a2", 78), ("keta", 79),
    ("nsub", 80), ("nch", 81), ("ngate", 82), ("nsd", 83), ("gamma1", 84), ("gamma2", 85), ("vbx", 86), ("vbm", 87), ("xt", 88), ("k1", 89), ("kt1", 90), ("kt1l", 91), ("kt2", 92), ("k2", 93), ("k3", 94), ("k3b", 95),
    ("w0", 96), ("lpeb", 97), ("dvt0", 98), ("dvt1", 99), ("dvt2", 100), ("dvt0w", 101), ("dvt1w", 102), ("dvt2w", 103), ("drout", 104), ("dsub", 105), ("vtho", 106), ("vth0", 107), ("vfb", 108), ("ua", 109), ("ua1", 110), ("ub", 111),
    ("ub1", 112), ("uc", 113), ("uc1", 114), ("u0", 115), ("eu", 116), ("ute", 117), ("ucs", 118), ("ucste", 119), ("ud", 120), ("ud1", 121), ("voff", 122), ("tnom", 123), ("cgso", 124), ("cgdo", 125), ("xpart", 126), ("delta", 127),
    ("rsh", 128), ("rdsw", 129), ("rsw", 130), ("rdw", 131), ("rswmin", 132), ("rdwmin", 133), ("prwg", 134), ("prwb", 135), ("prt", 136), ("eta0", 137), ("etab", 138), ("eta0cv", 139), ("etabcv", 140), ("pclm", 141), ("pdiblc1", 142), ("pdiblc2", 143),
    ("pdiblcb", 144), ("pvag", 145), ("tbox", 146), ("tsi", 147), ("etsi", 148), ("xj", 149), ("agidl", 150), ("bgidl", 151), ("cgidl", 152), ("rgidl", 153), ("kgidl", 154), ("fgidl", 155), ("agisl", 156), ("bgisl", 157), ("cgisl", 158), ("rgisl", 159),
    ("kgisl", 160), ("fgisl", 161), ("ndiode", 162), ("ndioded", 163), ("xbjt", 164), ("xdif", 165), ("xrec", 166), ("xtun", 167), ("xdifd", 168), ("xrecd", 169), ("xtund", 170), ("pbswg", 171), ("pbswgd", 172), ("mjswg", 173), ("mjswgd", 174), ("cjswg", 175),
    ("cjswgd", 176), ("lint", 177), ("ll", 178), ("llc", 179), ("lln", 180), ("lw", 181), ("lwc", 182), ("lwn", 183), ("lwl", 184), ("lwlc", 185), ("wr", 186), ("wint", 187), ("dwg", 188), ("dwb", 189), ("wl", 190), ("wlc", 191),
    ("wln", 192), ("ww", 193), ("wwc", 194), ("wwn", 195), ("wwl", 196), ("wwlc", 197), ("b0", 198), ("b1", 199), ("cgsl", 200), ("cgdl", 201), ("ckappa", 202), ("cf", 203), ("clc", 204), ("cle", 205), ("dwc", 206), ("dlc", 207),
    ("alpha0", 208), ("noia", 209), ("noib", 210), ("noic", 211), ("fnoimod", 212), ("tnoimod", 213), ("tnoia", 214), ("tnoib", 215), ("rnoia", 216), ("rnoib", 217), ("ntnoi", 218), ("saref", 219), ("sbref", 220), ("wlod", 221), ("ku0", 222), ("kvsat", 223),
    ("kvth0", 224), ("tku0", 225), ("llodku0", 226), ("wlodku0", 227), ("llodvth", 228), ("wlodvth", 229), ("lku0", 230), ("wku0", 231), ("pku0", 232), ("lkvth0", 233), ("wkvth0", 234), ("pkvth0", 235), ("stk2", 236), ("lodk2", 237), ("steta0", 238), ("lodeta0", 239),
    ("steta0cv", 240), ("lodeta0cv", 241), ("gbmin", 242), ("bf", 243), ("w0flk", 244), ("dvtp0", 245), ("ldvtp0", 246), ("wdvtp0", 247), ("pdvtp0", 248), ("dvtp1", 249), ("ldvtp1", 250), ("wdvtp1", 251), ("pdvtp1", 252), ("dvtp2", 253), ("ldvtp2", 254), ("wdvtp2", 255),
    ("pdvtp2", 256), ("dvtp3", 257), ("ldvtp3", 258), ("wdvtp3", 259), ("pdvtp3", 260), ("dvtp4", 261), ("ldvtp4", 262), ("wdvtp4", 263), ("pdvtp4", 264), ("minv", 265), ("lminv", 266), ("wminv", 267), ("pminv", 268), ("pdits", 269), ("pditsl", 270), ("pditsd", 271),
    ("fprout", 272), ("lfprout", 273), ("lpdits", 274), ("lpditsd", 275), ("wfprout", 276), ("wpdits", 277), ("wpditsd", 278), ("pfprout", 279), ("ppdits", 280), ("ppditsd", 281), ("em", 282), ("ef", 283), ("af", 284), ("kf", 285), ("noif", 286), ("k1w1", 287),
    ("k1w2", 288), ("ketas", 289), ("dwbc", 290), ("beta0", 291), ("beta1", 292), ("beta2", 293), ("vdsatii0", 294), ("tii", 295), ("lii", 296), ("sii0", 297), ("sii1", 298), ("sii2", 299), ("siid", 300), ("fbjtii", 301), ("ebjtii", 302), ("cbjtii", 303),
    ("vbci", 304), ("abjtii", 305), ("mbjtii", 306), ("tvbci", 307), ("esatii", 308), ("ntun", 309), ("ntund", 310), ("nrecf0", 311), ("nrecf0d", 312), ("nrecr0", 313), ("nrecr0d", 314), ("isbjt", 315), ("idbjt", 316), ("isdif", 317), ("iddif", 318), ("isrec", 319),
    ("idrec", 320), ("istun", 321), ("idtun", 322), ("ln", 323), ("vrec0", 324), ("vrec0d", 325), ("vtun0", 326), ("vtun0d", 327), ("nbjt", 328), ("lbjt0", 329), ("ldif0", 330), ("vabjt", 331), ("aely", 332), ("ahli", 333), ("ahlid", 334), ("rbody", 335),
    ("rbsh", 336), ("cgeo", 337), ("tt", 338), ("ndif", 339), ("vsdfb", 340), ("vsdth", 341), ("csdmin", 342), ("asd", 343), ("csdesw", 344), ("ntrecf", 345), ("ntrecr", 346), ("dlcb", 347), ("fbody", 348), ("tcjswg", 349), ("tpbswg", 350), ("tcjswgd", 351),
    ("tpbswgd", 352), ("acde", 353), ("moin", 354), ("noff", 355), ("noff2", 356), ("delvt", 357), ("kb1", 358), ("dlbg", 359), ("cfrcoeff", 360), ("igbmod", 361), ("igmod", 362), ("igcmod", 363), ("toxqm", 364), ("wth0", 365), ("rhalo", 366), ("ntox", 367),
    ("toxref", 368), ("ebg", 369), ("vevb", 370), ("alphagb1", 371), ("betagb1", 372), ("vgb1", 373), ("vecb", 374), ("alphagb2", 375), ("betagb2", 376), ("vgb2", 377), ("aigbcp2", 378), ("bigbcp2", 379), ("cigbcp2", 380), ("voxh", 381), ("deltavox", 382), ("aigc", 383),
    ("bigc", 384), ("cigc", 385), ("aigsd", 386), ("bigsd", 387), ("cigsd", 388), ("nigc", 389), ("pigcd", 390), ("poxedge", 391), ("dlcig", 392), ("vbs0pd", 393), ("vbs0fd", 394), ("vbsa", 395), ("nofffd", 396), ("vofffd", 397), ("k1b", 398), ("k2b", 399),
    ("dk2b", 400), ("dvbd0", 401), ("dvbd1", 402), ("moinfd", 403), ("xrcrg1", 404), ("xrcrg2", 405), ("rshg", 406), ("ngcon", 407), ("xgw", 408), ("xgl", 409), ("rdsmod", 410), ("fdmod", 411), ("vsce", 412), ("cdsbs", 413), ("minvcv", 414), ("lminvcv", 415),
    ("wminvcv", 416), ("pminvcv", 417), ("voffcv", 418), ("lvoffcv", 419), ("wvoffcv", 420), ("pvoffcv", 421), ("lxj", 422), ("lalphagb1", 423), ("lbetagb1", 424), ("lalphagb2", 425), ("lbetagb2", 426), ("laigbcp2", 427), ("lbigbcp2", 428), ("lcigbcp2", 429), ("lcgsl", 430), ("lcgdl", 431),
    ("lckappa", 432), ("lndif", 433), ("lute", 434), ("lkt1", 435), ("lkt1l", 436), ("lkt2", 437), ("lua1", 438), ("lub1", 439), ("luc1", 440), ("lat", 441), ("lprt", 442), ("lntrecf", 443), ("lntrecr", 444), ("lxbjt", 445), ("lxdif", 446), ("lxrec", 447),
    ("lxtun", 448), ("lxdifd", 449), ("lxrecd", 450), ("lxtund", 451), ("laigc", 452), ("lbigc", 453), ("lcigc", 454), ("laigsd", 455), ("lbigsd", 456), ("lcigsd", 457), ("lnigc", 458), ("lpigcd", 459), ("lpoxedge", 460), ("lnch", 461), ("lnsub", 462), ("lngate", 463),
    ("lnsd", 464), ("lvth0", 465), ("lvfb", 466), ("lk1", 467), ("lk1w1", 468), ("lk1w2", 469), ("lk2", 470), ("lk3", 471), ("lk3b", 472), ("lkb1", 473), ("lw0", 474), ("llpeb", 475), ("ldvt0", 476), ("ldvt1", 477), ("ldvt2", 478), ("ldvt0w", 479),
    ("ldvt1w", 480), ("ldvt2w", 481), ("lu0", 482), ("leu", 483), ("lua", 484), ("lub", 485), ("luc", 486), ("lud", 487), ("lud1", 488), ("lucste", 489), ("lucs", 490), ("lvsat", 491), ("la0", 492), ("lags", 493), ("lb0", 494), ("lb1", 495),
    ("lketa", 496), ("lketas", 497), ("la1", 498), ("la2", 499), ("lrdsw", 500), ("lrsw", 501), ("lrdw", 502), ("lprwb", 503), ("lprwg", 504), ("lwr", 505), ("lnfactor", 506), ("ldwg", 507), ("ldwb", 508), ("lvoff", 509), ("leta0", 510), ("letab", 511),
    ("leta0cv", 512), ("letabcv", 513), ("ldsub", 514), ("lcit", 515), ("lcdsc", 516), ("lcdscb", 517), ("lcdscd", 518), ("lpclm", 519), ("lpdiblc1", 520), ("lpdiblc2", 521), ("lpdiblcb", 522), ("ldrout", 523), ("lpvag", 524), ("ldelta", 525), ("lalpha0", 526), ("lfbjtii", 527),
    ("labjtii", 528), ("lcbjtii", 529), ("lebjtii", 530), ("lmbjtii", 531), ("lvbci", 532), ("lbeta0", 533), ("lbeta1", 534), ("lbeta2", 535), ("lvdsatii0", 536), ("llii", 537), ("lesatii", 538), ("lsii0", 539), ("lsii1", 540), ("lsii2", 541), ("lsiid", 542), ("lagidl", 543),
    ("lbgidl", 544), ("lcgidl", 545), ("lrgidl", 546), ("lkgidl", 547), ("lfgidl", 548), ("lagisl", 549), ("lbgisl", 550), ("lcgisl", 551), ("lrgisl", 552), ("lkgisl", 553), ("lfgisl", 554), ("lntun", 555), ("lntund", 556), ("lndiode", 557), ("lndioded", 558), ("lnrecf0", 559),
    ("lnrecf0d", 560), ("lnrecr0", 561), ("lnrecr0d", 562), ("lisbjt", 563), ("lidbjt", 564), ("lisdif", 565), ("liddif", 566), ("lisrec", 567), ("lidrec", 568), ("listun", 569), ("lidtun", 570), ("lvrec0", 571), ("lvrec0d", 572), ("lvtun0", 573), ("lvtun0d", 574), ("lnbjt", 575),
    ("llbjt0", 576), ("lvabjt", 577), ("laely", 578), ("lahli", 579), ("lahlid", 580), ("lvsdfb", 581), ("lvsdth", 582), ("ldelvt", 583), ("lacde", 584), ("lmoin", 585), ("lnoff", 586), ("lnoff2", 587), ("lxrcrg1", 588), ("lxrcrg2", 589), ("lvbsa", 590), ("lvsce", 591),
    ("lcdsbs", 592), ("lnofffd", 593), ("lvofffd", 594), ("lk1b", 595), ("lk2b", 596), ("ldk2b", 597), ("ldvbd0", 598), ("ldvbd1", 599), ("lmoinfd", 600), ("lvbs0pd", 601), ("lvbs0fd", 602), ("wxj", 603), ("walphagb1", 604), ("wbetagb1", 605), ("walphagb2", 606), ("wbetagb2", 607),
    ("waigbcp2", 608), ("wbigbcp2", 609), ("wcigbcp2", 610), ("wcgsl", 611), ("wcgdl", 612), ("wckappa", 613), ("wndif", 614), ("wute", 615), ("wkt1", 616), ("wkt1l", 617), ("wkt2", 618), ("wua1", 619), ("wub1", 620), ("wuc1", 621), ("wat", 622), ("wprt", 623),
    ("wntrecf", 624), ("wntrecr", 625), ("wxbjt", 626), ("wxdif", 627), ("wxrec", 628), ("wxtun", 629), ("wxdifd", 630), ("wxrecd", 631), ("wxtund", 632), ("waigc", 633), ("wbigc", 634), ("wcigc", 635), ("waigsd", 636), ("wbigsd", 637), ("wcigsd", 638), ("wnigc", 639),
    ("wpigcd", 640), ("wpoxedge", 641), ("wnch", 642), ("wnsub", 643), ("wngate", 644), ("wnsd", 645), ("wvth0", 646), ("wvfb", 647), ("wk1", 648), ("wk1w1", 649), ("wk1w2", 650), ("wk2", 651), ("wk3", 652), ("wk3b", 653), ("wkb1", 654), ("ww0", 655),
    ("wlpeb", 656), ("wdvt0", 657), ("wdvt1", 658), ("wdvt2", 659), ("wdvt0w", 660), ("wdvt1w", 661), ("wdvt2w", 662), ("wu0", 663), ("weu", 664), ("wua", 665), ("wub", 666), ("wuc", 667), ("wud", 668), ("wud1", 669), ("wucste", 670), ("wucs", 671),
    ("wvsat", 672), ("wa0", 673), ("wags", 674), ("wb0", 675), ("wb1", 676), ("wketa", 677), ("wketas", 678), ("wa1", 679), ("wa2", 680), ("wrdsw", 681), ("wrsw", 682), ("wrdw", 683), ("wprwb", 684), ("wprwg", 685), ("wwr", 686), ("wnfactor", 687),
    ("wdwg", 688), ("wdwb", 689), ("wvoff", 690), ("weta0", 691), ("wetab", 692), ("weta0cv", 693), ("wetabcv", 694), ("wdsub", 695), ("wcit", 696), ("wcdsc", 697), ("wcdscb", 698), ("wcdscd", 699), ("wpclm", 700), ("wpdiblc1", 701), ("wpdiblc2", 702), ("wpdiblcb", 703),
    ("wdrout", 704), ("wpvag", 705), ("wdelta", 706), ("walpha0", 707), ("wfbjtii", 708), ("wabjtii", 709), ("wcbjtii", 710), ("webjtii", 711), ("wmbjtii", 712), ("wvbci", 713), ("wbeta0", 714), ("wbeta1", 715), ("wbeta2", 716), ("wvdsatii0", 717), ("wlii", 718), ("wesatii", 719),
    ("wsii0", 720), ("wsii1", 721), ("wsii2", 722), ("wsiid", 723), ("wagidl", 724), ("wbgidl", 725), ("wcgidl", 726), ("wrgidl", 727), ("wkgidl", 728), ("wfgidl", 729), ("wagisl", 730), ("wbgisl", 731), ("wcgisl", 732), ("wrgisl", 733), ("wkgisl", 734), ("wfgisl", 735),
    ("wntun", 736), ("wntund", 737), ("wndiode", 738), ("wndioded", 739), ("wnrecf0", 740), ("wnrecf0d", 741), ("wnrecr0", 742), ("wnrecr0d", 743), ("wisbjt", 744), ("widbjt", 745), ("wisdif", 746), ("widdif", 747), ("wisrec", 748), ("widrec", 749), ("wistun", 750), ("widtun", 751),
    ("wvrec0", 752), ("wvrec0d", 753), ("wvtun0", 754), ("wvtun0d", 755), ("wnbjt", 756), ("wlbjt0", 757), ("wvabjt", 758), ("waely", 759), ("wahli", 760), ("wahlid", 761), ("wvsdfb", 762), ("wvsdth", 763), ("wdelvt", 764), ("wacde", 765), ("wmoin", 766), ("wnoff", 767),
    ("wnoff2", 768), ("wxrcrg1", 769), ("wxrcrg2", 770), ("wvbsa", 771), ("wvsce", 772), ("wcdsbs", 773), ("wnofffd", 774), ("wvofffd", 775), ("wk1b", 776), ("wk2b", 777), ("wdk2b", 778), ("wdvbd0", 779), ("wdvbd1", 780), ("wmoinfd", 781), ("wvbs0pd", 782), ("wvbs0fd", 783),
    ("pxj", 784), ("palphagb1", 785), ("pbetagb1", 786), ("palphagb2", 787), ("pbetagb2", 788), ("paigbcp2", 789), ("pbigbcp2", 790), ("pcigbcp2", 791), ("pcgsl", 792), ("pcgdl", 793), ("pckappa", 794), ("pndif", 795), ("pute", 796), ("pkt1", 797), ("pkt1l", 798), ("pkt2", 799),
    ("pua1", 800), ("pub1", 801), ("puc1", 802), ("pat", 803), ("pprt", 804), ("pntrecf", 805), ("pntrecr", 806), ("pxbjt", 807), ("pxdif", 808), ("pxrec", 809), ("pxtun", 810), ("pxdifd", 811), ("pxrecd", 812), ("pxtund", 813), ("paigc", 814), ("pbigc", 815),
    ("pcigc", 816), ("paigsd", 817), ("pbigsd", 818), ("pcigsd", 819), ("pnigc", 820), ("ppigcd", 821), ("ppoxedge", 822), ("pnch", 823), ("pnsub", 824), ("pnsd", 825), ("pngate", 826), ("pvth0", 827), ("pvfb", 828), ("pk1", 829), ("pk1w1", 830), ("pk1w2", 831),
    ("pk2", 832), ("pk3", 833), ("pk3b", 834), ("pkb1", 835), ("pw0", 836), ("plpeb", 837), ("pdvt0", 838), ("pdvt1", 839), ("pdvt2", 840), ("pdvt0w", 841), ("pdvt1w", 842), ("pdvt2w", 843), ("pu0", 844), ("peu", 845), ("pua", 846), ("pub", 847),
    ("puc", 848), ("pud", 849), ("pud1", 850), ("pucste", 851), ("pucs", 852), ("pvsat", 853), ("pa0", 854), ("pags", 855), ("pb0", 856), ("pb1", 857), ("pketa", 858), ("pketas", 859), ("pa1", 860), ("pa2", 861), ("prdsw", 862), ("prsw", 863),
    ("prdw", 864), ("pprwb", 865), ("pprwg", 866), ("pwr", 867), ("pnfactor", 868), ("pdwg", 869), ("pdwb", 870), ("pvoff", 871), ("peta0", 872), ("petab", 873), ("peta0cv", 874), ("petabcv", 875), ("pdsub", 876), ("pcit", 877), ("pcdsc", 878), ("pcdscb", 879),
    ("pcdscd", 880), ("ppclm", 881), ("ppdiblc1", 882), ("ppdiblc2", 883), ("ppdiblcb", 884), ("pdrout", 885), ("ppvag", 886), ("pdelta", 887), ("palpha0", 888), ("pfbjtii", 889), ("pabjtii", 890), ("pcbjtii", 891), ("pebjtii", 892), ("pmbjtii", 893), ("pvbci", 894), ("pbeta0", 895),
    ("pbeta1", 896), ("pbeta2", 897), ("pvdsatii0", 898), ("plii", 899), ("pesatii", 900), ("psii0", 901), ("psii1", 902), ("psii2", 903), ("psiid", 904), ("pagidl", 905), ("pbgidl", 906), ("pcgidl", 907), ("prgidl", 908), ("pkgidl", 909), ("pfgidl", 910), ("pagisl", 911),
    ("pbgisl", 912), ("pcgisl", 913), ("prgisl", 914), ("pkgisl", 915), ("pfgisl", 916), ("pntun", 917), ("pntund", 918), ("pndiode", 919), ("pndioded", 920), ("pnrecf0", 921), ("pnrecf0d", 922), ("pnrecr0", 923), ("pnrecr0d", 924), ("pisbjt", 925), ("pidbjt", 926), ("pisdif", 927),
    ("piddif", 928), ("pisrec", 929), ("pidrec", 930), ("pistun", 931), ("pidtun", 932), ("pvrec0", 933), ("pvrec0d", 934), ("pvtun0", 935), ("pvtun0d", 936), ("pnbjt", 937), ("plbjt0", 938), ("pvabjt", 939), ("paely", 940), ("pahli", 941), ("pahlid", 942), ("pvsdfb", 943),
    ("pvsdth", 944), ("pdelvt", 945), ("pacde", 946), ("pmoin", 947), ("pnoff", 948), ("pnoff2", 949), ("pxrcrg1", 950), ("pxrcrg2", 951), ("pvbsa", 952), ("pvsce", 953), ("pcdsbs", 954), ("pnofffd", 955), ("pvofffd", 956), ("pk1b", 957), ("pk2b", 958), ("pdk2b", 959),
    ("pdvbd0", 960), ("pdvbd1", 961), ("pmoinfd", 962), ("pvbs0pd", 963), ("pvbs0fd", 964), ("nlx", 965), ("lnlx", 966), ("wnlx", 967), ("pnlx", 968), ("ngidl", 969), ("lngidl", 970), ("wngidl", 971), ("pngidl", 972), ("lpe0", 973), ("egidl", 974), ("egisl", 975),
    ("llpe0", 976), ("legidl", 977), ("legisl", 978), ("wlpe0", 979), ("wegidl", 980), ("wegisl", 981), ("plpe0", 982), ("pegidl", 983), ("pegisl", 984), ("eggbcp2", 985), ("eggdep", 986), ("agb1", 987), ("bgb1", 988), ("agb2", 989), ("bgb2", 990), ("agbc2n", 991),
    ("agbc2p", 992), ("bgbc2n", 993), ("bgbc2p", 994), ("vtm00", 995),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 996] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
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

const PARAMETER_MAX_REFERENCES: [Option<usize>; 996] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
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

const PARAMETER_DISPLAY_NAMES: [&str; 996] = [
    "DTEMP", "L", "W", "NF", "SA", "SB", "SD", "AD", "AS", "PD", "PS", "NRD", "NRS", "OFF", "BJTOFF", "DEBUG",
    "RTH0", "CTH0", "NRB", "FRBODY", "RBDB", "RBSB", "DELVTO", "SOIMOD", "NBC", "NSEG", "PDBCP", "PSBCP", "AGBCP", "AGBCP2", "AGBCPD", "AEBCP",
    "TNODEOUT", "SHMOD", "TYPE", "VERSION", "VBSUSR", "RGATEMOD", "RBODYMOD", "MTRLMOD", "VGSTCVMOD", "GIDLMOD", "IIIMOD", "EOT", "EPSROX", "EPSRSUB", "NI0SUB", "BG0SUB",
    "TBGASUB", "TBGBSUB", "PHIG", "EASUB", "LEFFEOT", "WEFFEOT", "VDDEOT", "TEMPEOT", "ADOS", "BDOS", "EPSRGATE", "CAPMOD", "MOBMOD", "PARAMCHK", "NODECHK", "BINUNIT",
    "TOX", "TOXP", "TOXM", "DTOXCV", "CDSC", "CDSCB", "CDSCD", "CIT", "NFACTOR", "VSAT", "AT", "A0", "AGS", "A1", "A2", "KETA",
    "NSUB", "NCH", "NGATE", "NSD", "GAMMA1", "GAMMA2", "VBX", "VBM", "XT", "K1", "KT1", "KT1L", "KT2", "K2", "K3", "K3B",
    "W0", "LPEB", "DVT0", "DVT1", "DVT2", "DVT0W", "DVT1W", "DVT2W", "DROUT", "DSUB", "VTHO", "VTH0", "VFB", "UA", "UA1", "UB",
    "UB1", "UC", "UC1", "U0", "EU", "UTE", "UCS", "UCSTE", "UD", "UD1", "VOFF", "TNOM", "CGSO", "CGDO", "XPART", "DELTA",
    "RSH", "RDSW", "RSW", "RDW", "RSWMIN", "RDWMIN", "PRWG", "PRWB", "PRT", "ETA0", "ETAB", "ETA0CV", "ETABCV", "PCLM", "PDIBLC1", "PDIBLC2",
    "PDIBLCB", "PVAG", "TBOX", "TSI", "ETSI", "XJ", "AGIDL", "BGIDL", "CGIDL", "RGIDL", "KGIDL", "FGIDL", "AGISL", "BGISL", "CGISL", "RGISL",
    "KGISL", "FGISL", "NDIODE", "NDIODED", "XBJT", "XDIF", "XREC", "XTUN", "XDIFD", "XRECD", "XTUND", "PBSWG", "PBSWGD", "MJSWG", "MJSWGD", "CJSWG",
    "CJSWGD", "LINT", "LL", "LLC", "LLN", "LW", "LWC", "LWN", "LWL", "LWLC", "WR", "WINT", "DWG", "DWB", "WL", "WLC",
    "WLN", "WW", "WWC", "WWN", "WWL", "WWLC", "B0", "B1", "CGSL", "CGDL", "CKAPPA", "CF", "CLC", "CLE", "DWC", "DLC",
    "ALPHA0", "NOIA", "NOIB", "NOIC", "FNOIMOD", "TNOIMOD", "TNOIA", "TNOIB", "RNOIA", "RNOIB", "NTNOI", "SAREF", "SBREF", "WLOD", "KU0", "KVSAT",
    "KVTH0", "TKU0", "LLODKU0", "WLODKU0", "LLODVTH", "WLODVTH", "LKU0", "WKU0", "PKU0", "LKVTH0", "WKVTH0", "PKVTH0", "STK2", "LODK2", "STETA0", "LODETA0",
    "STETA0CV", "LODETA0CV", "GBMIN", "BF", "W0FLK", "DVTP0", "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2",
    "PDVTP2", "DVTP3", "LDVTP3", "WDVTP3", "PDVTP3", "DVTP4", "LDVTP4", "WDVTP4", "PDVTP4", "MINV", "LMINV", "WMINV", "PMINV", "PDITS", "PDITSL", "PDITSD",
    "FPROUT", "LFPROUT", "LPDITS", "LPDITSD", "WFPROUT", "WPDITS", "WPDITSD", "PFPROUT", "PPDITS", "PPDITSD", "EM", "EF", "AF", "KF", "NOIF", "K1W1",
    "K1W2", "KETAS", "DWBC", "BETA0", "BETA1", "BETA2", "VDSATII0", "TII", "LII", "SII0", "SII1", "SII2", "SIID", "FBJTII", "EBJTII", "CBJTII",
    "VBCI", "ABJTII", "MBJTII", "TVBCI", "ESATII", "NTUN", "NTUND", "NRECF0", "NRECF0D", "NRECR0", "NRECR0D", "ISBJT", "IDBJT", "ISDIF", "IDDIF", "ISREC",
    "IDREC", "ISTUN", "IDTUN", "LN", "VREC0", "VREC0D", "VTUN0", "VTUN0D", "NBJT", "LBJT0", "LDIF0", "VABJT", "AELY", "AHLI", "AHLID", "RBODY",
    "RBSH", "CGEO", "TT", "NDIF", "VSDFB", "VSDTH", "CSDMIN", "ASD", "CSDESW", "NTRECF", "NTRECR", "DLCB", "FBODY", "TCJSWG", "TPBSWG", "TCJSWGD",
    "TPBSWGD", "ACDE", "MOIN", "NOFF", "NOFF2", "DELVT", "KB1", "DLBG", "CFRCOEFF", "IGBMOD", "IGMOD", "IGCMOD", "TOXQM", "WTH0", "RHALO", "NTOX",
    "TOXREF", "EBG", "VEVB", "ALPHAGB1", "BETAGB1", "VGB1", "VECB", "ALPHAGB2", "BETAGB2", "VGB2", "AIGBCP2", "BIGBCP2", "CIGBCP2", "VOXH", "DELTAVOX", "AIGC",
    "BIGC", "CIGC", "AIGSD", "BIGSD", "CIGSD", "NIGC", "PIGCD", "POXEDGE", "DLCIG", "VBS0PD", "VBS0FD", "VBSA", "NOFFFD", "VOFFFD", "K1B", "K2B",
    "DK2B", "DVBD0", "DVBD1", "MOINFD", "XRCRG1", "XRCRG2", "RSHG", "NGCON", "XGW", "XGL", "RDSMOD", "FDMOD", "VSCE", "CDSBS", "MINVCV", "LMINVCV",
    "WMINVCV", "PMINVCV", "VOFFCV", "LVOFFCV", "WVOFFCV", "PVOFFCV", "LXJ", "LALPHAGB1", "LBETAGB1", "LALPHAGB2", "LBETAGB2", "LAIGBCP2", "LBIGBCP2", "LCIGBCP2", "LCGSL", "LCGDL",
    "LCKAPPA", "LNDIF", "LUTE", "LKT1", "LKT1L", "LKT2", "LUA1", "LUB1", "LUC1", "LAT", "LPRT", "LNTRECF", "LNTRECR", "LXBJT", "LXDIF", "LXREC",
    "LXTUN", "LXDIFD", "LXRECD", "LXTUND", "LAIGC", "LBIGC", "LCIGC", "LAIGSD", "LBIGSD", "LCIGSD", "LNIGC", "LPIGCD", "LPOXEDGE", "LNCH", "LNSUB", "LNGATE",
    "LNSD", "LVTH0", "LVFB", "LK1", "LK1W1", "LK1W2", "LK2", "LK3", "LK3B", "LKB1", "LW0", "LLPEB", "LDVT0", "LDVT1", "LDVT2", "LDVT0W",
    "LDVT1W", "LDVT2W", "LU0", "LEU", "LUA", "LUB", "LUC", "LUD", "LUD1", "LUCSTE", "LUCS", "LVSAT", "LA0", "LAGS", "LB0", "LB1",
    "LKETA", "LKETAS", "LA1", "LA2", "LRDSW", "LRSW", "LRDW", "LPRWB", "LPRWG", "LWR", "LNFACTOR", "LDWG", "LDWB", "LVOFF", "LETA0", "LETAB",
    "LETA0CV", "LETABCV", "LDSUB", "LCIT", "LCDSC", "LCDSCB", "LCDSCD", "LPCLM", "LPDIBLC1", "LPDIBLC2", "LPDIBLCB", "LDROUT", "LPVAG", "LDELTA", "LALPHA0", "LFBJTII",
    "LABJTII", "LCBJTII", "LEBJTII", "LMBJTII", "LVBCI", "LBETA0", "LBETA1", "LBETA2", "LVDSATII0", "LLII", "LESATII", "LSII0", "LSII1", "LSII2", "LSIID", "LAGIDL",
    "LBGIDL", "LCGIDL", "LRGIDL", "LKGIDL", "LFGIDL", "LAGISL", "LBGISL", "LCGISL", "LRGISL", "LKGISL", "LFGISL", "LNTUN", "LNTUND", "LNDIODE", "LNDIODED", "LNRECF0",
    "LNRECF0D", "LNRECR0", "LNRECR0D", "LISBJT", "LIDBJT", "LISDIF", "LIDDIF", "LISREC", "LIDREC", "LISTUN", "LIDTUN", "LVREC0", "LVREC0D", "LVTUN0", "LVTUN0D", "LNBJT",
    "LLBJT0", "LVABJT", "LAELY", "LAHLI", "LAHLID", "LVSDFB", "LVSDTH", "LDELVT", "LACDE", "LMOIN", "LNOFF", "LNOFF2", "LXRCRG1", "LXRCRG2", "LVBSA", "LVSCE",
    "LCDSBS", "LNOFFFD", "LVOFFFD", "LK1B", "LK2B", "LDK2B", "LDVBD0", "LDVBD1", "LMOINFD", "LVBS0PD", "LVBS0FD", "WXJ", "WALPHAGB1", "WBETAGB1", "WALPHAGB2", "WBETAGB2",
    "WAIGBCP2", "WBIGBCP2", "WCIGBCP2", "WCGSL", "WCGDL", "WCKAPPA", "WNDIF", "WUTE", "WKT1", "WKT1L", "WKT2", "WUA1", "WUB1", "WUC1", "WAT", "WPRT",
    "WNTRECF", "WNTRECR", "WXBJT", "WXDIF", "WXREC", "WXTUN", "WXDIFD", "WXRECD", "WXTUND", "WAIGC", "WBIGC", "WCIGC", "WAIGSD", "WBIGSD", "WCIGSD", "WNIGC",
    "WPIGCD", "WPOXEDGE", "WNCH", "WNSUB", "WNGATE", "WNSD", "WVTH0", "WVFB", "WK1", "WK1W1", "WK1W2", "WK2", "WK3", "WK3B", "WKB1", "WW0",
    "WLPEB", "WDVT0", "WDVT1", "WDVT2", "WDVT0W", "WDVT1W", "WDVT2W", "WU0", "WEU", "WUA", "WUB", "WUC", "WUD", "WUD1", "WUCSTE", "WUCS",
    "WVSAT", "WA0", "WAGS", "WB0", "WB1", "WKETA", "WKETAS", "WA1", "WA2", "WRDSW", "WRSW", "WRDW", "WPRWB", "WPRWG", "WWR", "WNFACTOR",
    "WDWG", "WDWB", "WVOFF", "WETA0", "WETAB", "WETA0CV", "WETABCV", "WDSUB", "WCIT", "WCDSC", "WCDSCB", "WCDSCD", "WPCLM", "WPDIBLC1", "WPDIBLC2", "WPDIBLCB",
    "WDROUT", "WPVAG", "WDELTA", "WALPHA0", "WFBJTII", "WABJTII", "WCBJTII", "WEBJTII", "WMBJTII", "WVBCI", "WBETA0", "WBETA1", "WBETA2", "WVDSATII0", "WLII", "WESATII",
    "WSII0", "WSII1", "WSII2", "WSIID", "WAGIDL", "WBGIDL", "WCGIDL", "WRGIDL", "WKGIDL", "WFGIDL", "WAGISL", "WBGISL", "WCGISL", "WRGISL", "WKGISL", "WFGISL",
    "WNTUN", "WNTUND", "WNDIODE", "WNDIODED", "WNRECF0", "WNRECF0D", "WNRECR0", "WNRECR0D", "WISBJT", "WIDBJT", "WISDIF", "WIDDIF", "WISREC", "WIDREC", "WISTUN", "WIDTUN",
    "WVREC0", "WVREC0D", "WVTUN0", "WVTUN0D", "WNBJT", "WLBJT0", "WVABJT", "WAELY", "WAHLI", "WAHLID", "WVSDFB", "WVSDTH", "WDELVT", "WACDE", "WMOIN", "WNOFF",
    "WNOFF2", "WXRCRG1", "WXRCRG2", "WVBSA", "WVSCE", "WCDSBS", "WNOFFFD", "WVOFFFD", "WK1B", "WK2B", "WDK2B", "WDVBD0", "WDVBD1", "WMOINFD", "WVBS0PD", "WVBS0FD",
    "PXJ", "PALPHAGB1", "PBETAGB1", "PALPHAGB2", "PBETAGB2", "PAIGBCP2", "PBIGBCP2", "PCIGBCP2", "PCGSL", "PCGDL", "PCKAPPA", "PNDIF", "PUTE", "PKT1", "PKT1L", "PKT2",
    "PUA1", "PUB1", "PUC1", "PAT", "PPRT", "PNTRECF", "PNTRECR", "PXBJT", "PXDIF", "PXREC", "PXTUN", "PXDIFD", "PXRECD", "PXTUND", "PAIGC", "PBIGC",
    "PCIGC", "PAIGSD", "PBIGSD", "PCIGSD", "PNIGC", "PPIGCD", "PPOXEDGE", "PNCH", "PNSUB", "PNSD", "PNGATE", "PVTH0", "PVFB", "PK1", "PK1W1", "PK1W2",
    "PK2", "PK3", "PK3B", "PKB1", "PW0", "PLPEB", "PDVT0", "PDVT1", "PDVT2", "PDVT0W", "PDVT1W", "PDVT2W", "PU0", "PEU", "PUA", "PUB",
    "PUC", "PUD", "PUD1", "PUCSTE", "PUCS", "PVSAT", "PA0", "PAGS", "PB0", "PB1", "PKETA", "PKETAS", "PA1", "PA2", "PRDSW", "PRSW",
    "PRDW", "PPRWB", "PPRWG", "PWR", "PNFACTOR", "PDWG", "PDWB", "PVOFF", "PETA0", "PETAB", "PETA0CV", "PETABCV", "PDSUB", "PCIT", "PCDSC", "PCDSCB",
    "PCDSCD", "PPCLM", "PPDIBLC1", "PPDIBLC2", "PPDIBLCB", "PDROUT", "PPVAG", "PDELTA", "PALPHA0", "PFBJTII", "PABJTII", "PCBJTII", "PEBJTII", "PMBJTII", "PVBCI", "PBETA0",
    "PBETA1", "PBETA2", "PVDSATII0", "PLII", "PESATII", "PSII0", "PSII1", "PSII2", "PSIID", "PAGIDL", "PBGIDL", "PCGIDL", "PRGIDL", "PKGIDL", "PFGIDL", "PAGISL",
    "PBGISL", "PCGISL", "PRGISL", "PKGISL", "PFGISL", "PNTUN", "PNTUND", "PNDIODE", "PNDIODED", "PNRECF0", "PNRECF0D", "PNRECR0", "PNRECR0D", "PISBJT", "PIDBJT", "PISDIF",
    "PIDDIF", "PISREC", "PIDREC", "PISTUN", "PIDTUN", "PVREC0", "PVREC0D", "PVTUN0", "PVTUN0D", "PNBJT", "PLBJT0", "PVABJT", "PAELY", "PAHLI", "PAHLID", "PVSDFB",
    "PVSDTH", "PDELVT", "PACDE", "PMOIN", "PNOFF", "PNOFF2", "PXRCRG1", "PXRCRG2", "PVBSA", "PVSCE", "PCDSBS", "PNOFFFD", "PVOFFFD", "PK1B", "PK2B", "PDK2B",
    "PDVBD0", "PDVBD1", "PMOINFD", "PVBS0PD", "PVBS0FD", "NLX", "LNLX", "WNLX", "PNLX", "NGIDL", "LNGIDL", "WNGIDL", "PNGIDL", "LPE0", "EGIDL", "EGISL",
    "LLPE0", "LEGIDL", "LEGISL", "WLPE0", "WEGIDL", "WEGISL", "PLPE0", "PEGIDL", "PEGISL", "EGGBCP2", "EGGDEP", "AGB1", "BGB1", "AGB2", "BGB2", "AGBC2N",
    "AGBC2P", "BGBC2N", "BGBC2P", "VTM00",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 996] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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

const PARAMETER_INTEGER_FLAGS: [bool; 996] = [
    false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
    true, true, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false,
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
    false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 996] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 996] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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

const PARAMETER_RANGE_FLAGS: [u8; 996] = [
    0, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
    0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 2, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
    3, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0,
    2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
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
    0, 0, 0, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 996] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 13],
    pub branches: [usize; 9],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 996]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<15, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) canonical_reactive: Box<[f64; 77]>,
    pub(crate) canonical_staged: Box<[f64; 856]>,
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
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 9;
    pub const NODE_COUNT: usize = 13;
    pub const INTERNAL_NODE_NAMES: [&str; 9] = ["p", "b", "t", "di", "si", "gi", "gm", "sb", "db"];

    pub const BRANCH_COUNT: usize = 9;
    pub const PARAMETER_COUNT: usize = 996;
    pub const VARIABLE_COUNT: usize = 1871;
    pub const DDT_STATE_COUNT: usize = 15;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "b1955c92ba0fe519d6801fabf0a252f0fe0bd4e8594bb9dad449721b30bd5a1b";
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
        let mut values = Vec::with_capacity(75);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(15);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 75);
        debug_assert_eq!(state.flags.len(), 15);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimsoi_va'", name));
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
