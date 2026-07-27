#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 864],
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
            const DEFAULTS_0: [f64; 36] = [
                2e-6, 5e-6, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 3e17, 0.0, 0.0, 0.0,
                0.0, 3.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 36);
            {
                let params = &mut *ptr;
                params[36] = if (params[34] != 0.0) { params[35] } else { 0.0 };
                validate_parameter("COOVLPS", params[36], true, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 28] = [
                1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 2.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 50.0, 50.0, 1.0, 0.0,
                0.0, 1.0, 1e-6, 1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(37), 28);
            {
                let params = &mut *ptr;
                params[65] = if (params[34] != 0.0) { params[63] } else { 3e-8 };
                validate_parameter("LOVER", params[65], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[66] = if (params[34] != 0.0) { params[63] } else { params[65] };
                validate_parameter("LOVERS", params[66], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 2] = [
                1e-6, 1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(67), 2);
            {
                let params = &mut *ptr;
                params[69] = if (params[34] != 0.0) { params[67] } else { 0.0 };
                validate_parameter("LDRIFT1S", params[69], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[70] = if (params[34] != 0.0) { params[68] } else { 1e-6 };
                validate_parameter("LDRIFT2S", params[70], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[71] = (params[69] + params[70]);
                validate_parameter("LDRIFTS", params[71], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(72), 3);
            {
                let params = &mut *ptr;
                params[75] = if (params[34] != 0.0) { params[74] } else { 0.0 };
                validate_parameter("RS", params[75], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 20] = [
                5e17, 0.3, 1.0, 0.0, 0.1, 1.0, 0.07, 0.005,
                0.0, 0.0, 0.0, 1.0, 2.51, 10000000.0, 0.0, 0.0,
                9.025e-5, 1e-7, 1.1785, 7e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(76), 20);
            {
                let params = &mut *ptr;
                params[96] = params[95];
                validate_finite_parameter("TOXB", params[96]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 11] = [
                0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(97), 11);
            {
                let params = &mut *ptr;
                params[108] = if (((params[88] * 10.0) % 10.0) < 3.0) { 0.0 } else { 10.0 };
                validate_parameter("DDLTSLP", params[108], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[109] = if (((params[88] * 10.0) % 10.0) < 3.0) { 10.0 } else { 0.0 };
                validate_finite_parameter("DDLTICT", params[109]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 2] = [
                -0.5, 3e16,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(110), 2);
            {
                let params = &mut *ptr;
                params[112] = if (params[34] != 0.0) { params[111] } else { 1e17 };
                validate_parameter("NOVERS", params[112], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 2] = [
                5.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(113), 2);
            {
                let params = &mut *ptr;
                params[115] = params[114];
                validate_finite_parameter("XWDC", params[115]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 15] = [
                0.0, 0.0, 1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(116), 15);
            {
                let params = &mut *ptr;
                params[131] = params[130];
                validate_parameter("RSHS", params[131], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 4] = [
                0.0, 0.0, 0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(132), 4);
            {
                let params = &mut *ptr;
                params[136] = if (params[42] != 0.0) { (-0.2) } else { (-1.0) };
                validate_finite_parameter("VFBC", params[136]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 1] = [
                1.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(137), 1);
            {
                let params = &mut *ptr;
                params[138] = if (params[42] != 0.0) { 5e16 } else { 3e17 };
                validate_parameter("NSUBC", params[138], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(139), 1);
            {
                let params = &mut *ptr;
                params[140] = if (params[42] != 0.0) { 0.0 } else { 1.5e-8 };
                validate_parameter("LP", params[140], false, Some((0.0, "0.0")), false, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[141] = if (params[42] != 0.0) { 1e17 } else { 1e18 };
                validate_parameter("NSUBP", params[141], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 19] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.5,
                1000.0, 100.0, 0.3,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(142), 19);
            {
                let params = &mut *ptr;
                params[161] = if (params[87] > 0.0) { 20000.0 } else { 9000.0 };
                validate_parameter("MUEPH1", params[161], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 10] = [
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(162), 10);
            {
                let params = &mut *ptr;
                params[172] = if (params[42] != 0.0) { 5000000000000000.0 } else { 600000000000000.0 };
                validate_parameter("MUESR1", params[172], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 5] = [
                0.0, 0.0, 1.0, 1.0, 1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(173), 5);
            {
                let params = &mut *ptr;
                params[178] = if (params[87] > 0.0) { 2.0 } else { 1.0 };
                validate_parameter("BB", params[178], false, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 8] = [
                10.0, 25.0, 0.8, 0.5, 0.0, 1.0, 0.8, 3e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(179), 8);
            {
                let params = &mut *ptr;
                params[187] = params[179];
                validate_finite_parameter("SUB1SNP", params[187]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[188] = (0.6 * params[180]);
                validate_finite_parameter("SUB2SNP", params[188]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[189] = params[185];
                validate_finite_parameter("SVDSSNP", params[189]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 79] = [
                0.0025, 1.0, 2e-6, 0.0, 50.0, 0.00017, 0.0, 0.012,
                0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 5e17, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1e-50, 0.0,
                0.0, 0.0, 0.9, 2e-7, 0.05, 2.0, 1.0, 1.0,
                0.0, 0.3, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 2.0, 30000000.0, 0.9, 0.0, 0.2, 50.0,
                10000000.0, 0.06, 4.0, 7500.0, 0.25, 1e-6, 0.5, 1e-15,
                1000.0, -1000.0, 5e-16, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.01, 0.005, 10000000000.0, 1e-19, 0.0, 3.9, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(190), 79);
            {
                let params = &mut *ptr;
                params[269] = if (params[34] != 0.0) { params[268] } else { 0.0 };
                validate_parameter("CGSO", params[269], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 25] = [
                1e-10, 0.7, 8e-7, 8e-5, 27.0, 2.1e-7, 0.6, 1e-12,
                0.0, 0.0, -1.0, 0.0, -0.3, 0.0, 3.5, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 100.0, 1e-7,
                1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(270), 25);
            {
                let params = &mut *ptr;
                params[295] = params[114];
                validate_finite_parameter("XWDLD", params[295]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 44] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, -10.5, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.3, 1e-6, 0.7,
                1000000000000000.0, 0.1, 0.8, 0.4,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(296), 44);
            {
                let params = &mut *ptr;
                params[340] = if (params[42] < 3.0) { 1e17 } else { 4e16 };
                validate_parameter("NDEPM", params[340], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(341), 2);
            {
                let params = &mut *ptr;
                params[343] = if (params[42] < 3.0) { 2.0000000000000002e-7 } else { 3.0000000000000004e-7 };
                validate_parameter("TNDEP", params[343], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[344] = (params[343] * 1e-6);
                validate_parameter("TNDEPMIN", params[344], false, None, true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(345), 1);
            {
                let params = &mut *ptr;
                params[346] = if (params[42] < 3.0) { 1000.0 } else { 100000000.0 };
                validate_finite_parameter("DEPMUE0", params[346]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(347), 2);
            {
                let params = &mut *ptr;
                params[349] = if (params[42] < 3.0) { 0.0 } else { 100.0 };
                validate_finite_parameter("DEPMUE1", params[349]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 10] = [
                0.0, 1.0, 1000.0, 0.0, 100.0, 0.0, 0.0, 1.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(350), 10);
            {
                let params = &mut *ptr;
                params[360] = if (params[42] < 3.0) { 0.5 } else { 0.1 };
                validate_finite_parameter("DEPLEAK", params[360]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 6] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(361), 6);
            {
                let params = &mut *ptr;
                params[367] = if (params[42] < 3.0) { 30000000.0 } else { 10000000.0 };
                validate_parameter("DEPVMAX", params[367], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 8] = [
                0.0, 1.0, 2.0, 0.5, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(368), 8);
            {
                let params = &mut *ptr;
                params[376] = if (params[42] < 3.0) { 0.3 } else { 0.0 };
                validate_finite_parameter("DEPMUEPH0", params[376]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[377] = if (params[42] < 3.0) { 5000.0 } else { 400.0 };
                validate_parameter("DEPMUEPH1", params[377], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[378] = if (params[42] < 3.0) { 1.0 } else { 2.0 };
                validate_parameter("DEPBB", params[378], false, Some((0.01, "0.01")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 4] = [
                0.0, 1.5, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(379), 4);
            {
                let params = &mut *ptr;
                params[383] = if (params[42] < 3.0) { 3.0 } else { 1.0 };
                validate_parameter("DEPDDLT", params[383], false, Some((0.1, "0.1")), false, Some((20.0, "20.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 8] = [
                100.0, 10.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(384), 8);
            {
                let params = &mut *ptr;
                params[392] = params[136];
                validate_finite_parameter("DEPVFBC", params[392]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 2] = [
                0.1, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(393), 2);
            {
                let params = &mut *ptr;
                params[395] = params[394];
                validate_parameter("DEPSUBSL0", params[395], false, Some((1e-8, "1e-8")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 11] = [
                0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.01,
                0.01, 0.05, 0.2,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (*ptr).values.as_mut_ptr().add(396), 11);
            {
                let params = &mut *ptr;
                params[407] = if (params[42] < 3.0) { 0.0 } else { 0.2 };
                validate_parameter("DEPVGPSL", params[407], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 5] = [
                0.5, 1000.0, 0.0, 0.0, 30000000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (*ptr).values.as_mut_ptr().add(408), 5);
            {
                let params = &mut *ptr;
                params[413] = params[409];
                validate_parameter("RDRMUES", params[413], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[414] = params[412];
                validate_parameter("RDRVMAXS", params[414], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (*ptr).values.as_mut_ptr().add(415), 1);
            {
                let params = &mut *ptr;
                params[416] = params[415];
                validate_finite_parameter("RDRMUETMPS", params[416]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (*ptr).values.as_mut_ptr().add(417), 1);
            {
                let params = &mut *ptr;
                params[418] = params[417];
                validate_finite_parameter("RDRVTMPS", params[418]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 15] = [
                1e-6, 0.0, 1e-8, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 100000.0, 0.0, 0.0, 500.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (*ptr).values.as_mut_ptr().add(419), 15);
            {
                let params = &mut *ptr;
                params[434] = ((-100.0) * params[87]);
                validate_finite_parameter("VGSMIN", params[434]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (*ptr).values.as_mut_ptr().add(435), 2);
            {
                let params = &mut *ptr;
                params[437] = params[436];
                validate_parameter("RDRBBS", params[437], false, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (*ptr).values.as_mut_ptr().add(438), 1);
            {
                let params = &mut *ptr;
                params[439] = params[438];
                validate_finite_parameter("RDRBBTMPS", params[439]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 53] = [
                1.0, 1.0, 1.0, 0.0, 0.0001, 0.0, 0.0, 100.0,
                0.0, 1.0, 0.0, 0.0, 3e-8, 1e20, 0.0, 0.0,
                0.0, 0.0, 5e-7, 0.0, 0.0, 1.0, 1.0, 1.0,
                2.0, 0.0005, 5e-10, 5e-10, 0.5, 0.33, 0.33, 1.0,
                1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (*ptr).values.as_mut_ptr().add(440), 53);
            {
                let params = &mut *ptr;
                params[493] = params[458];
                validate_finite_parameter("JS0D", params[493]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[494] = params[459];
                validate_finite_parameter("JS0SWD", params[494]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[495] = params[460];
                validate_finite_parameter("JS0SWGD", params[495]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[496] = params[461];
                validate_parameter("NJD", params[496], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[497] = params[462];
                validate_parameter("NJSWD", params[497], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[498] = params[463];
                validate_parameter("NJSWGD", params[498], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[499] = params[464];
                validate_finite_parameter("XTID", params[499]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[500] = params[465];
                validate_finite_parameter("CJD", params[500]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[501] = params[466];
                validate_finite_parameter("CJSWD", params[501]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[502] = params[467];
                validate_finite_parameter("CJSWGD", params[502]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[503] = params[468];
                validate_parameter("MJD", params[503], false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[504] = params[469];
                validate_parameter("MJSWD", params[504], false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[505] = params[470];
                validate_parameter("MJSWGD", params[505], false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[506] = params[471];
                validate_parameter("PBD", params[506], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[507] = params[472];
                validate_parameter("PBSWD", params[507], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[508] = params[473];
                validate_parameter("PBSWGD", params[508], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[509] = params[474];
                validate_finite_parameter("XTI2D", params[509]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[510] = params[475];
                validate_finite_parameter("CISBD", params[510]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[511] = params[476];
                validate_finite_parameter("CVBD", params[511]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[512] = params[477];
                validate_finite_parameter("CTEMPD", params[512]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[513] = params[478];
                validate_finite_parameter("CISBKD", params[513]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[514] = params[479];
                validate_finite_parameter("DIVXD", params[514]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[515] = params[480];
                validate_finite_parameter("VDIFFJD", params[515]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[516] = params[493];
                validate_finite_parameter("JS0S", params[516]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[517] = params[494];
                validate_finite_parameter("JS0SWS", params[517]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[518] = params[495];
                validate_finite_parameter("JS0SWGS", params[518]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[519] = params[496];
                validate_parameter("NJS", params[519], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[520] = params[497];
                validate_parameter("NJSWS", params[520], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[521] = params[498];
                validate_parameter("NJSWGS", params[521], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[522] = params[499];
                validate_finite_parameter("XTIS", params[522]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[523] = params[500];
                validate_finite_parameter("CJS", params[523]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[524] = params[501];
                validate_finite_parameter("CJSWS", params[524]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[525] = params[502];
                validate_finite_parameter("CJSWGS", params[525]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[526] = params[503];
                validate_parameter("MJS", params[526], false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[527] = params[504];
                validate_parameter("MJSWS", params[527], false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[528] = params[505];
                validate_parameter("MJSWGS", params[528], false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[529] = params[506];
                validate_parameter("PBS", params[529], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[530] = params[507];
                validate_parameter("PBSWS", params[530], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[531] = params[508];
                validate_parameter("PBSWGS", params[531], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[532] = params[509];
                validate_finite_parameter("XTI2S", params[532]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[533] = params[510];
                validate_finite_parameter("CISBS", params[533]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[534] = params[511];
                validate_finite_parameter("CVBS", params[534]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[535] = params[512];
                validate_finite_parameter("CTEMPS", params[535]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[536] = params[513];
                validate_finite_parameter("CISBKS", params[536]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[537] = params[514];
                validate_finite_parameter("DIVXS", params[537]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[538] = params[515];
                validate_finite_parameter("VDIFFJS", params[538]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 48] = [
                0.0, 1e16, 1.0, 10.0, 5e-9, 2e-7, 5e-6, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (*ptr).values.as_mut_ptr().add(539), 48);
            {
                let params = &mut *ptr;
                params[587] = params[582];
                validate_finite_parameter("LSUB1SNP", params[587]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[588] = params[583];
                validate_finite_parameter("LSUB2SNP", params[588]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[589] = params[584];
                validate_finite_parameter("LSVDSSNP", params[589]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 85] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (*ptr).values.as_mut_ptr().add(590), 85);
            {
                let params = &mut *ptr;
                params[675] = params[670];
                validate_finite_parameter("WSUB1SNP", params[675]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[676] = params[671];
                validate_finite_parameter("WSUB2SNP", params[676]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[677] = params[672];
                validate_finite_parameter("WSVDSSNP", params[677]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 85] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (*ptr).values.as_mut_ptr().add(678), 85);
            {
                let params = &mut *ptr;
                params[763] = params[758];
                validate_finite_parameter("PSUB1SNP", params[763]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[764] = params[759];
                validate_finite_parameter("PSUB2SNP", params[764]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[765] = params[760];
                validate_finite_parameter("PSVDSSNP", params[765]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 58] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (*ptr).values.as_mut_ptr().add(766), 58);
            {
                let params = &mut *ptr;
                params[824] = params[819];
                validate_finite_parameter("LJS0D", params[824]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[825] = params[820];
                validate_finite_parameter("LJS0SWD", params[825]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[826] = params[821];
                validate_finite_parameter("LNJD", params[826]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[827] = params[822];
                validate_finite_parameter("LCISBKD", params[827]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[828] = params[823];
                validate_finite_parameter("LVDIFFJD", params[828]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[829] = params[824];
                validate_finite_parameter("LJS0S", params[829]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[830] = params[825];
                validate_finite_parameter("LJS0SWS", params[830]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[831] = params[826];
                validate_finite_parameter("LNJS", params[831]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[832] = params[827];
                validate_finite_parameter("LCISBKS", params[832]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[833] = params[828];
                validate_finite_parameter("LVDIFFJS", params[833]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (*ptr).values.as_mut_ptr().add(834), 5);
            {
                let params = &mut *ptr;
                params[839] = params[834];
                validate_finite_parameter("WJS0D", params[839]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[840] = params[835];
                validate_finite_parameter("WJS0SWD", params[840]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[841] = params[836];
                validate_finite_parameter("WNJD", params[841]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[842] = params[837];
                validate_finite_parameter("WCISBKD", params[842]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[843] = params[838];
                validate_finite_parameter("WVDIFFJD", params[843]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[844] = params[839];
                validate_finite_parameter("WJS0S", params[844]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[845] = params[840];
                validate_finite_parameter("WJS0SWS", params[845]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[846] = params[841];
                validate_finite_parameter("WNJS", params[846]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[847] = params[842];
                validate_finite_parameter("WCISBKS", params[847]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[848] = params[843];
                validate_finite_parameter("WVDIFFJS", params[848]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (*ptr).values.as_mut_ptr().add(849), 5);
            {
                let params = &mut *ptr;
                params[854] = params[849];
                validate_finite_parameter("PJS0D", params[854]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[855] = params[850];
                validate_finite_parameter("PJS0SWD", params[855]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[856] = params[851];
                validate_finite_parameter("PNJD", params[856]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[857] = params[852];
                validate_finite_parameter("PCISBKD", params[857]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[858] = params[853];
                validate_finite_parameter("PVDIFFJD", params[858]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[859] = params[854];
                validate_finite_parameter("PJS0S", params[859]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[860] = params[855];
                validate_finite_parameter("PJS0SWS", params[860]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[861] = params[856];
                validate_finite_parameter("PNJS", params[861]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[862] = params[857];
                validate_finite_parameter("PCISBKS", params[862]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[863] = params[858];
                validate_finite_parameter("PVDIFFJS", params[863]).expect("generated Verilog-A parameter default must satisfy declared range");
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 864] = [
    ("l", 0), ("w", 1), ("nrd", 2), ("nrs", 3), ("ngcon", 4), ("xgw", 5), ("xgl", 6), ("nf", 7), ("sa", 8), ("sb", 9), ("sd", 10), ("dtemp", 11), ("nsubcdfm", 12), ("ad", 13), ("as", 14), ("pd", 15),
    ("ps", 16), ("corsrd", 17), ("cors", 18), ("cord", 19), ("coiprv", 20), ("copprv", 21), ("coadov", 22), ("coisub", 23), ("coiigs", 24), ("cogidl", 25), ("coflick", 26), ("coisti", 27), ("conqs", 28), ("conqsov", 29), ("cothrml", 30), ("coign", 31),
    ("codfm", 32), ("coqovsm", 33), ("cosym", 34), ("coovlp", 35), ("coovlps", 36), ("covbscl", 37), ("coqovcl", 38), ("cotemp", 39), ("cordrift", 40), ("coerrrep", 41), ("codep", 42), ("covdsres", 43), ("coddlt", 44), ("cohbd", 45), ("cosnp", 46), ("info", 47),
    ("codio", 48), ("cofixcss", 49), ("coovjunc", 50), ("corg", 51), ("corbnet", 52), ("coselfheat", 53), ("cosubnode", 54), ("cotrench", 55), ("rbpb", 56), ("rbpd", 57), ("rbps", 58), ("rdov13", 59), ("rdslp1", 60), ("rdvg11", 61), ("rdict1", 62), ("loverld", 63),
    ("wtrench", 64), ("lover", 65), ("lovers", 66), ("ldrift1", 67), ("ldrift2", 68), ("ldrift1s", 69), ("ldrift2s", 70), ("ldrifts", 71), ("subld1", 72), ("subld2", 73), ("rd", 74), ("rs", 75), ("npext", 76), ("vover", 77), ("falph", 78), ("cgbo", 79),
    ("rth0", 80), ("powrat", 81), ("rdvd", 82), ("rd23", 83), ("rd24", 84), ("rdvb", 85), ("cvdsover", 86), ("type", 87), ("version", 88), ("vmax", 89), ("vmaxt1", 90), ("vmaxt2", 91), ("bgtmp1", 92), ("bgtmp2", 93), ("eg0", 94), ("tox", 95),
    ("toxb", 96), ("xld", 97), ("rdov11", 98), ("rdov12", 99), ("rdslp2", 100), ("rdict2", 101), ("subld1l", 102), ("subld1lp", 103), ("xpdv", 104), ("xpvdth", 105), ("xpvdthg", 106), ("ddltmax", 107), ("ddltslp", 108), ("ddltict", 109), ("vfbover", 110), ("nover", 111),
    ("novers", 112), ("olmdlt", 113), ("xwd", 114), ("xwdc", 115), ("xl", 116), ("xw", 117), ("saref", 118), ("sbref", 119), ("ll", 120), ("lld", 121), ("lln", 122), ("wl", 123), ("wl1", 124), ("wl1p", 125), ("wl2", 126), ("wl2p", 127),
    ("wld", 128), ("wln", 129), ("rsh", 130), ("rshs", 131), ("rshg", 132), ("xqy", 133), ("xqy1", 134), ("xqy2", 135), ("vfbc", 136), ("vbi", 137), ("nsubc", 138), ("parl2", 139), ("lp", 140), ("nsubp", 141), ("nsubp0", 142), ("nsubwp", 143),
    ("scp1", 144), ("scp2", 145), ("scp3", 146), ("sc1", 147), ("sc2", 148), ("sc3", 149), ("sc4", 150), ("pgd1", 151), ("pgd2", 152), ("pgd4", 153), ("ndep", 154), ("ndepl", 155), ("ndeplp", 156), ("ninv", 157), ("muecb0", 158), ("muecb1", 159),
    ("mueph0", 160), ("mueph1", 161), ("muephw", 162), ("muepwp", 163), ("muephl", 164), ("mueplp", 165), ("mueefb", 166), ("muephs", 167), ("muepsp", 168), ("vtmp", 169), ("wvth0", 170), ("muesr0", 171), ("muesr1", 172), ("muesrl", 173), ("muesrw", 174), ("mueswp", 175),
    ("mueslp", 176), ("muetmp", 177), ("bb", 178), ("sub1", 179), ("sub2", 180), ("svgs", 181), ("svbs", 182), ("svbsl", 183), ("svbslp", 184), ("svds", 185), ("slg", 186), ("sub1snp", 187), ("sub2snp", 188), ("svdssnp", 189), ("sub1l", 190), ("sub1lp", 191),
    ("sub2l", 192), ("subtmp", 193), ("fn1", 194), ("fn2", 195), ("fn3", 196), ("fvbs", 197), ("svgsl", 198), ("svgslp", 199), ("svgsw", 200), ("svgswp", 201), ("slgl", 202), ("slglp", 203), ("nsti", 204), ("wsti", 205), ("wstil", 206), ("wstilp", 207),
    ("wstiw", 208), ("wstiwp", 209), ("scsti1", 210), ("scsti2", 211), ("vthsti", 212), ("vdsti", 213), ("muesti1", 214), ("muesti2", 215), ("muesti3", 216), ("nsubpsti1", 217), ("nsubpsti2", 218), ("nsubpsti3", 219), ("lpext", 220), ("scp21", 221), ("scp22", 222), ("bs1", 223),
    ("bs2", 224), ("tpoly", 225), ("clm1", 226), ("clm2", 227), ("clm3", 228), ("clm5", 229), ("clm6", 230), ("voverp", 231), ("wfc", 232), ("nsubcw", 233), ("nsubcwp", 234), ("qme1", 235), ("qme2", 236), ("qme3", 237), ("vovers", 238), ("voversp", 239),
    ("gidl1", 240), ("gidl2", 241), ("gidl3", 242), ("gidl4", 243), ("gidl5", 244), ("gleak1", 245), ("gleak2", 246), ("gleak3", 247), ("gleak4", 248), ("gleak5", 249), ("gleak6", 250), ("gleak7", 251), ("glpart1", 252), ("glksd1", 253), ("glksd2", 254), ("glksd3", 255),
    ("glkb1", 256), ("glkb2", 257), ("glkb3", 258), ("egig", 259), ("igtemp2", 260), ("igtemp3", 261), ("vzadd0", 262), ("pzadd0", 263), ("nftrp", 264), ("nfalp", 265), ("cit", 266), ("kappa", 267), ("cgdo", 268), ("cgso", 269), ("dly1", 270), ("dly2", 271),
    ("dly3", 272), ("dlyov", 273), ("tnom", 274), ("ovslp", 275), ("ovmag", 276), ("gbmin", 277), ("ibpc1", 278), ("ibpc1l", 279), ("ibpc1lp", 280), ("ibpc2", 281), ("mphdfm", 282), ("ptl", 283), ("ptp", 284), ("pt2", 285), ("ptlp", 286), ("gdl", 287),
    ("gdlp", 288), ("gdld", 289), ("pt4", 290), ("pt4p", 291), ("rdvg12", 292), ("cth0", 293), ("xldld", 294), ("xwdld", 295), ("rd20", 296), ("rd21", 297), ("rd22", 298), ("rd22d", 299), ("rd25", 300), ("rdvdl", 301), ("rdvdlp", 302), ("rdvds", 303),
    ("rdvdsp", 304), ("rd23l", 305), ("rd23lp", 306), ("rd23s", 307), ("rd23sp", 308), ("rds", 309), ("rdsp", 310), ("rdtemp1", 311), ("rdtemp2", 312), ("rdvdtemp1", 313), ("rdvdtemp2", 314), ("rth0w", 315), ("rth0wp", 316), ("rth0l", 317), ("rth0lp", 318), ("ninvd", 319),
    ("ninvdl", 320), ("ninvdlp", 321), ("ninvdw", 322), ("ninvdwp", 323), ("ninvdt1", 324), ("ninvdt2", 325), ("vbsmin", 326), ("rth0nf", 327), ("rthtemp1", 328), ("rthtemp2", 329), ("prattemp1", 330), ("prattemp2", 331), ("rdvsub", 332), ("rdvdsub", 333), ("ddrift", 334), ("vbisub", 335),
    ("nsubsub", 336), ("shemaxdlt", 337), ("vbfwdmx", 338), ("vbfwdbnd", 339), ("ndepm", 340), ("ndepml", 341), ("ndepmlp", 342), ("tndep", 343), ("tndepmin", 344), ("tndepv", 345), ("depmue0", 346), ("depmue0l", 347), ("depmue0lp", 348), ("depmue1", 349), ("depmue1l", 350), ("depmue1lp", 351),
    ("depmue2", 352), ("depmuea1", 353), ("depmueback0", 354), ("depmueback1", 355), ("depmueback0l", 356), ("depmueback0lp", 357), ("depmueback1l", 358), ("depmueback1lp", 359), ("depleak", 360), ("depleakl", 361), ("depleaklp", 362), ("depjleak", 363), ("depwlp", 364), ("depwlpt", 365), ("depeta", 366), ("depvmax", 367),
    ("depvmaxl", 368), ("depvmaxlp", 369), ("depvdsef1", 370), ("depvdsef2", 371), ("depvdsef1l", 372), ("depvdsef1lp", 373), ("depvdsef2l", 374), ("depvdsef2lp", 375), ("depmueph0", 376), ("depmueph1", 377), ("depbb", 378), ("depvtmp", 379), ("depmuetmp", 380), ("depmue0tmp", 381), ("depmue2tmp", 382), ("depddlt", 383),
    ("depninvdc", 384), ("depninvdh", 385), ("depninvdl", 386), ("depninvdlp", 387), ("depninvdw", 388), ("depninvdwp", 389), ("depninvdt1", 390), ("depninvdt2", 391), ("depvfbc", 392), ("depdvfbc", 393), ("depsubsl", 394), ("depsubsl0", 395), ("depvsatr", 396), ("depvsata", 397), ("deprbr", 398), ("depvleak", 399),
    ("depcar", 400), ("deprdrdl1", 401), ("deprdrdl2", 402), ("depps", 403), ("depqf", 404), ("depqfres", 405), ("depfdpd", 406), ("depvgpsl", 407), ("deppb0", 408), ("rdrmue", 409), ("rdrmuebs1", 410), ("rdrmuebs2", 411), ("rdrvmax", 412), ("rdrmues", 413), ("rdrvmaxs", 414), ("rdrmuetmp", 415),
    ("rdrmuetmps", 416), ("rdrvtmp", 417), ("rdrvtmps", 418), ("rdrdjunc", 419), ("rdrcx", 420), ("rdrcar", 421), ("rdrdl1", 422), ("rdrdl2", 423), ("rdrvmaxw", 424), ("rdrvmaxwp", 425), ("rdrvmaxl", 426), ("rdrvmaxlp", 427), ("rdrmuel", 428), ("rdrmuelp", 429), ("rdrqover", 430), ("qovadd", 431),
    ("qovjunc", 432), ("shemax", 433), ("vgsmin", 434), ("gdsleak", 435), ("rdrbb", 436), ("rdrbbs", 437), ("rdrbbtmp", 438), ("rdrbbtmps", 439), ("ndrilim", 440), ("ndridlt", 441), ("ndripw", 442), ("gmin", 443), ("rmin", 444), ("hbda", 445), ("hbdb", 446), ("hbdc", 447),
    ("hbdctmp", 448), ("hbdf", 449), ("copt", 450), ("copspt", 451), ("xjpt", 452), ("njunc", 453), ("mupt", 454), ("vfbpt", 455), ("pslimpt", 456), ("ps0pt", 457), ("js0", 458), ("js0sw", 459), ("js0swg", 460), ("nj", 461), ("njsw", 462), ("njswg", 463),
    ("xti", 464), ("cj", 465), ("cjsw", 466), ("cjswg", 467), ("mj", 468), ("mjsw", 469), ("mjswg", 470), ("pb", 471), ("pbsw", 472), ("pbswg", 473), ("xti2", 474), ("cisb", 475), ("cvb", 476), ("ctemp", 477), ("cisbk", 478), ("divx", 479),
    ("vdiffj", 480), ("tcjbd", 481), ("tcjbs", 482), ("tcjbdsw", 483), ("tcjbssw", 484), ("tcjbdswg", 485), ("tcjbsswg", 486), ("tpbbd", 487), ("tpbbs", 488), ("tpbbdsw", 489), ("tpbbssw", 490), ("tpbbdswg", 491), ("tpbbsswg", 492), ("js0d", 493), ("js0swd", 494), ("js0swgd", 495),
    ("njd", 496), ("njswd", 497), ("njswgd", 498), ("xtid", 499), ("cjd", 500), ("cjswd", 501), ("cjswgd", 502), ("mjd", 503), ("mjswd", 504), ("mjswgd", 505), ("pbd", 506), ("pbswd", 507), ("pbswgd", 508), ("xti2d", 509), ("cisbd", 510), ("cvbd", 511),
    ("ctempd", 512), ("cisbkd", 513), ("divxd", 514), ("vdiffjd", 515), ("js0s", 516), ("js0sws", 517), ("js0swgs", 518), ("njs", 519), ("njsws", 520), ("njswgs", 521), ("xtis", 522), ("cjs", 523), ("cjsws", 524), ("cjswgs", 525), ("mjs", 526), ("mjsws", 527),
    ("mjswgs", 528), ("pbs", 529), ("pbsws", 530), ("pbswgs", 531), ("xti2s", 532), ("cisbs", 533), ("cvbs", 534), ("ctemps", 535), ("cisbks", 536), ("divxs", 537), ("vdiffjs", 538), ("corecovery", 539), ("ndibot", 540), ("inj1", 541), ("inj2", 542), ("nqs", 543),
    ("tau", 544), ("wi", 545), ("depnqs", 546), ("taut", 547), ("injt", 548), ("lmin", 549), ("lmax", 550), ("wmin", 551), ("wmax", 552), ("lbinn", 553), ("wbinn", 554), ("lvmax", 555), ("lbgtmp1", 556), ("lbgtmp2", 557), ("leg0", 558), ("lvfbover", 559),
    ("lnover", 560), ("lnovers", 561), ("lwl2", 562), ("lvfbc", 563), ("lnsubc", 564), ("lnsubp", 565), ("lscp1", 566), ("lscp2", 567), ("lscp3", 568), ("lsc1", 569), ("lsc2", 570), ("lsc3", 571), ("lpgd1", 572), ("lndep", 573), ("lninv", 574), ("lmuecb0", 575),
    ("lmuecb1", 576), ("lmueph1", 577), ("lvtmp", 578), ("lwvth0", 579), ("lmuesr1", 580), ("lmuetmp", 581), ("lsub1", 582), ("lsub2", 583), ("lsvds", 584), ("lsvbs", 585), ("lsvgs", 586), ("lsub1snp", 587), ("lsub2snp", 588), ("lsvdssnp", 589), ("lfn1", 590), ("lfn2", 591),
    ("lfn3", 592), ("lfvbs", 593), ("lnsti", 594), ("lwsti", 595), ("lscsti1", 596), ("lscsti2", 597), ("lvthsti", 598), ("lmuesti1", 599), ("lmuesti2", 600), ("lmuesti3", 601), ("lnsubpsti1", 602), ("lnsubpsti2", 603), ("lnsubpsti3", 604), ("lcgso", 605), ("lcgdo", 606), ("lclm1", 607),
    ("lclm2", 608), ("lclm3", 609), ("lwfc", 610), ("lgidl1", 611), ("lgidl2", 612), ("lgleak1", 613), ("lgleak2", 614), ("lgleak3", 615), ("lgleak6", 616), ("lglksd1", 617), ("lglksd2", 618), ("lglkb1", 619), ("lglkb2", 620), ("lnftrp", 621), ("lnfalp", 622), ("libpc1", 623),
    ("libpc2", 624), ("lcgbo", 625), ("lcvdsover", 626), ("lfalph", 627), ("lnpext", 628), ("lpowrat", 629), ("lrd", 630), ("lrd22", 631), ("lrd23", 632), ("lrd24", 633), ("lrdict1", 634), ("lrdov13", 635), ("lrdslp1", 636), ("lrdvb", 637), ("lrdvd", 638), ("lrdvg11", 639),
    ("lrs", 640), ("lrth0", 641), ("lvover", 642), ("wvmax", 643), ("wbgtmp1", 644), ("wbgtmp2", 645), ("weg0", 646), ("wvfbover", 647), ("wnover", 648), ("wnovers", 649), ("wwl2", 650), ("wvfbc", 651), ("wnsubc", 652), ("wnsubp", 653), ("wscp1", 654), ("wscp2", 655),
    ("wscp3", 656), ("wsc1", 657), ("wsc2", 658), ("wsc3", 659), ("wpgd1", 660), ("wndep", 661), ("wninv", 662), ("wmuecb0", 663), ("wmuecb1", 664), ("wmueph1", 665), ("wvtmp", 666), ("wwvth0", 667), ("wmuesr1", 668), ("wmuetmp", 669), ("wsub1", 670), ("wsub2", 671),
    ("wsvds", 672), ("wsvbs", 673), ("wsvgs", 674), ("wsub1snp", 675), ("wsub2snp", 676), ("wsvdssnp", 677), ("wfn1", 678), ("wfn2", 679), ("wfn3", 680), ("wfvbs", 681), ("wnsti", 682), ("wwsti", 683), ("wscsti1", 684), ("wscsti2", 685), ("wvthsti", 686), ("wmuesti1", 687),
    ("wmuesti2", 688), ("wmuesti3", 689), ("wnsubpsti1", 690), ("wnsubpsti2", 691), ("wnsubpsti3", 692), ("wcgso", 693), ("wcgdo", 694), ("wclm1", 695), ("wclm2", 696), ("wclm3", 697), ("wwfc", 698), ("wgidl1", 699), ("wgidl2", 700), ("wgleak1", 701), ("wgleak2", 702), ("wgleak3", 703),
    ("wgleak6", 704), ("wglksd1", 705), ("wglksd2", 706), ("wglkb1", 707), ("wglkb2", 708), ("wnftrp", 709), ("wnfalp", 710), ("wibpc1", 711), ("wibpc2", 712), ("wcgbo", 713), ("wcvdsover", 714), ("wfalph", 715), ("wnpext", 716), ("wpowrat", 717), ("wrd", 718), ("wrd22", 719),
    ("wrd23", 720), ("wrd24", 721), ("wrdict1", 722), ("wrdov13", 723), ("wrdslp1", 724), ("wrdvb", 725), ("wrdvd", 726), ("wrdvg11", 727), ("wrs", 728), ("wrth0", 729), ("wvover", 730), ("pvmax", 731), ("pbgtmp1", 732), ("pbgtmp2", 733), ("peg0", 734), ("pvfbover", 735),
    ("pnover", 736), ("pnovers", 737), ("pwl2", 738), ("pvfbc", 739), ("pnsubc", 740), ("pnsubp", 741), ("pscp1", 742), ("pscp2", 743), ("pscp3", 744), ("psc1", 745), ("psc2", 746), ("psc3", 747), ("ppgd1", 748), ("pndep", 749), ("pninv", 750), ("pmuecb0", 751),
    ("pmuecb1", 752), ("pmueph1", 753), ("pvtmp", 754), ("pwvth0", 755), ("pmuesr1", 756), ("pmuetmp", 757), ("psub1", 758), ("psub2", 759), ("psvds", 760), ("psvbs", 761), ("psvgs", 762), ("psub1snp", 763), ("psub2snp", 764), ("psvdssnp", 765), ("pfn1", 766), ("pfn2", 767),
    ("pfn3", 768), ("pfvbs", 769), ("pnsti", 770), ("pwsti", 771), ("pscsti1", 772), ("pscsti2", 773), ("pvthsti", 774), ("pmuesti1", 775), ("pmuesti2", 776), ("pmuesti3", 777), ("pnsubpsti1", 778), ("pnsubpsti2", 779), ("pnsubpsti3", 780), ("pcgso", 781), ("pcgdo", 782), ("pclm1", 783),
    ("pclm2", 784), ("pclm3", 785), ("pwfc", 786), ("pgidl1", 787), ("pgidl2", 788), ("pgleak1", 789), ("pgleak2", 790), ("pgleak3", 791), ("pgleak6", 792), ("pglksd1", 793), ("pglksd2", 794), ("pglkb1", 795), ("pglkb2", 796), ("pnftrp", 797), ("pnfalp", 798), ("pibpc1", 799),
    ("pibpc2", 800), ("pcgbo", 801), ("pcvdsover", 802), ("pfalph", 803), ("pnpext", 804), ("ppowrat", 805), ("prd", 806), ("prd22", 807), ("prd23", 808), ("prd24", 809), ("prdict1", 810), ("prdov13", 811), ("prdslp1", 812), ("prdvb", 813), ("prdvd", 814), ("prdvg11", 815),
    ("prs", 816), ("prth0", 817), ("pvover", 818), ("ljs0", 819), ("ljs0sw", 820), ("lnj", 821), ("lcisbk", 822), ("lvdiffj", 823), ("ljs0d", 824), ("ljs0swd", 825), ("lnjd", 826), ("lcisbkd", 827), ("lvdiffjd", 828), ("ljs0s", 829), ("ljs0sws", 830), ("lnjs", 831),
    ("lcisbks", 832), ("lvdiffjs", 833), ("wjs0", 834), ("wjs0sw", 835), ("wnj", 836), ("wcisbk", 837), ("wvdiffj", 838), ("wjs0d", 839), ("wjs0swd", 840), ("wnjd", 841), ("wcisbkd", 842), ("wvdiffjd", 843), ("wjs0s", 844), ("wjs0sws", 845), ("wnjs", 846), ("wcisbks", 847),
    ("wvdiffjs", 848), ("pjs0", 849), ("pjs0sw", 850), ("pnj", 851), ("pcisbk", 852), ("pvdiffj", 853), ("pjs0d", 854), ("pjs0swd", 855), ("pnjd", 856), ("pcisbkd", 857), ("pvdiffjd", 858), ("pjs0s", 859), ("pjs0sws", 860), ("pnjs", 861), ("pcisbks", 862), ("pvdiffjs", 863),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 864] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 864] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, Some(0), Some(0), None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, Some(343), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 864] = [
    "L", "W", "NRD", "NRS", "NGCON", "XGW", "XGL", "NF", "SA", "SB", "SD", "DTEMP", "NSUBCDFM", "AD", "AS", "PD",
    "PS", "CORSRD", "CORS", "CORD", "COIPRV", "COPPRV", "COADOV", "COISUB", "COIIGS", "COGIDL", "COFLICK", "COISTI", "CONQS", "CONQSOV", "COTHRML", "COIGN",
    "CODFM", "COQOVSM", "COSYM", "COOVLP", "COOVLPS", "COVBSCL", "COQOVCL", "COTEMP", "CORDRIFT", "COERRREP", "CODEP", "COVDSRES", "CODDLT", "COHBD", "COSNP", "INFO",
    "CODIO", "COFIXCSS", "COOVJUNC", "CORG", "CORBNET", "COSELFHEAT", "COSUBNODE", "COTRENCH", "RBPB", "RBPD", "RBPS", "RDOV13", "RDSLP1", "RDVG11", "RDICT1", "LOVERLD",
    "WTRENCH", "LOVER", "LOVERS", "LDRIFT1", "LDRIFT2", "LDRIFT1S", "LDRIFT2S", "LDRIFTS", "SUBLD1", "SUBLD2", "RD", "RS", "NPEXT", "VOVER", "FALPH", "CGBO",
    "RTH0", "POWRAT", "RDVD", "RD23", "RD24", "RDVB", "CVDSOVER", "TYPE", "VERSION", "VMAX", "VMAXT1", "VMAXT2", "BGTMP1", "BGTMP2", "EG0", "TOX",
    "TOXB", "XLD", "RDOV11", "RDOV12", "RDSLP2", "RDICT2", "SUBLD1L", "SUBLD1LP", "XPDV", "XPVDTH", "XPVDTHG", "DDLTMAX", "DDLTSLP", "DDLTICT", "VFBOVER", "NOVER",
    "NOVERS", "OLMDLT", "XWD", "XWDC", "XL", "XW", "SAREF", "SBREF", "LL", "LLD", "LLN", "WL", "WL1", "WL1P", "WL2", "WL2P",
    "WLD", "WLN", "RSH", "RSHS", "RSHG", "XQY", "XQY1", "XQY2", "VFBC", "VBI", "NSUBC", "PARL2", "LP", "NSUBP", "NSUBP0", "NSUBWP",
    "SCP1", "SCP2", "SCP3", "SC1", "SC2", "SC3", "SC4", "PGD1", "PGD2", "PGD4", "NDEP", "NDEPL", "NDEPLP", "NINV", "MUECB0", "MUECB1",
    "MUEPH0", "MUEPH1", "MUEPHW", "MUEPWP", "MUEPHL", "MUEPLP", "MUEEFB", "MUEPHS", "MUEPSP", "VTMP", "WVTH0", "MUESR0", "MUESR1", "MUESRL", "MUESRW", "MUESWP",
    "MUESLP", "MUETMP", "BB", "SUB1", "SUB2", "SVGS", "SVBS", "SVBSL", "SVBSLP", "SVDS", "SLG", "SUB1SNP", "SUB2SNP", "SVDSSNP", "SUB1L", "SUB1LP",
    "SUB2L", "SUBTMP", "FN1", "FN2", "FN3", "FVBS", "SVGSL", "SVGSLP", "SVGSW", "SVGSWP", "SLGL", "SLGLP", "NSTI", "WSTI", "WSTIL", "WSTILP",
    "WSTIW", "WSTIWP", "SCSTI1", "SCSTI2", "VTHSTI", "VDSTI", "MUESTI1", "MUESTI2", "MUESTI3", "NSUBPSTI1", "NSUBPSTI2", "NSUBPSTI3", "LPEXT", "SCP21", "SCP22", "BS1",
    "BS2", "TPOLY", "CLM1", "CLM2", "CLM3", "CLM5", "CLM6", "VOVERP", "WFC", "NSUBCW", "NSUBCWP", "QME1", "QME2", "QME3", "VOVERS", "VOVERSP",
    "GIDL1", "GIDL2", "GIDL3", "GIDL4", "GIDL5", "GLEAK1", "GLEAK2", "GLEAK3", "GLEAK4", "GLEAK5", "GLEAK6", "GLEAK7", "GLPART1", "GLKSD1", "GLKSD2", "GLKSD3",
    "GLKB1", "GLKB2", "GLKB3", "EGIG", "IGTEMP2", "IGTEMP3", "VZADD0", "PZADD0", "NFTRP", "NFALP", "CIT", "KAPPA", "CGDO", "CGSO", "DLY1", "DLY2",
    "DLY3", "DLYOV", "TNOM", "OVSLP", "OVMAG", "GBMIN", "IBPC1", "IBPC1L", "IBPC1LP", "IBPC2", "MPHDFM", "PTL", "PTP", "PT2", "PTLP", "GDL",
    "GDLP", "GDLD", "PT4", "PT4P", "RDVG12", "CTH0", "XLDLD", "XWDLD", "RD20", "RD21", "RD22", "RD22D", "RD25", "RDVDL", "RDVDLP", "RDVDS",
    "RDVDSP", "RD23L", "RD23LP", "RD23S", "RD23SP", "RDS", "RDSP", "RDTEMP1", "RDTEMP2", "RDVDTEMP1", "RDVDTEMP2", "RTH0W", "RTH0WP", "RTH0L", "RTH0LP", "NINVD",
    "NINVDL", "NINVDLP", "NINVDW", "NINVDWP", "NINVDT1", "NINVDT2", "VBSMIN", "RTH0NF", "RTHTEMP1", "RTHTEMP2", "PRATTEMP1", "PRATTEMP2", "RDVSUB", "RDVDSUB", "DDRIFT", "VBISUB",
    "NSUBSUB", "SHEMAXDLT", "VBFWDMX", "VBFWDBND", "NDEPM", "NDEPML", "NDEPMLP", "TNDEP", "TNDEPMIN", "TNDEPV", "DEPMUE0", "DEPMUE0L", "DEPMUE0LP", "DEPMUE1", "DEPMUE1L", "DEPMUE1LP",
    "DEPMUE2", "DEPMUEA1", "DEPMUEBACK0", "DEPMUEBACK1", "DEPMUEBACK0L", "DEPMUEBACK0LP", "DEPMUEBACK1L", "DEPMUEBACK1LP", "DEPLEAK", "DEPLEAKL", "DEPLEAKLP", "DEPJLEAK", "DEPWLP", "DEPWLPT", "DEPETA", "DEPVMAX",
    "DEPVMAXL", "DEPVMAXLP", "DEPVDSEF1", "DEPVDSEF2", "DEPVDSEF1L", "DEPVDSEF1LP", "DEPVDSEF2L", "DEPVDSEF2LP", "DEPMUEPH0", "DEPMUEPH1", "DEPBB", "DEPVTMP", "DEPMUETMP", "DEPMUE0TMP", "DEPMUE2TMP", "DEPDDLT",
    "DEPNINVDC", "DEPNINVDH", "DEPNINVDL", "DEPNINVDLP", "DEPNINVDW", "DEPNINVDWP", "DEPNINVDT1", "DEPNINVDT2", "DEPVFBC", "DEPDVFBC", "DEPSUBSL", "DEPSUBSL0", "DEPVSATR", "DEPVSATA", "DEPRBR", "DEPVLEAK",
    "DEPCAR", "DEPRDRDL1", "DEPRDRDL2", "DEPPS", "DEPQF", "DEPQFRES", "DEPFDPD", "DEPVGPSL", "DEPPB0", "RDRMUE", "RDRMUEBS1", "RDRMUEBS2", "RDRVMAX", "RDRMUES", "RDRVMAXS", "RDRMUETMP",
    "RDRMUETMPS", "RDRVTMP", "RDRVTMPS", "RDRDJUNC", "RDRCX", "RDRCAR", "RDRDL1", "RDRDL2", "RDRVMAXW", "RDRVMAXWP", "RDRVMAXL", "RDRVMAXLP", "RDRMUEL", "RDRMUELP", "RDRQOVER", "QOVADD",
    "QOVJUNC", "SHEMAX", "VGSMIN", "GDSLEAK", "RDRBB", "RDRBBS", "RDRBBTMP", "RDRBBTMPS", "NDRILIM", "NDRIDLT", "NDRIPW", "GMIN", "RMIN", "HBDA", "HBDB", "HBDC",
    "HBDCTMP", "HBDF", "COPT", "COPSPT", "XJPT", "NJUNC", "MUPT", "VFBPT", "PSLIMPT", "PS0PT", "JS0", "JS0SW", "JS0SWG", "NJ", "NJSW", "NJSWG",
    "XTI", "CJ", "CJSW", "CJSWG", "MJ", "MJSW", "MJSWG", "PB", "PBSW", "PBSWG", "XTI2", "CISB", "CVB", "CTEMP", "CISBK", "DIVX",
    "VDIFFJ", "TCJBD", "TCJBS", "TCJBDSW", "TCJBSSW", "TCJBDSWG", "TCJBSSWG", "TPBBD", "TPBBS", "TPBBDSW", "TPBBSSW", "TPBBDSWG", "TPBBSSWG", "JS0D", "JS0SWD", "JS0SWGD",
    "NJD", "NJSWD", "NJSWGD", "XTID", "CJD", "CJSWD", "CJSWGD", "MJD", "MJSWD", "MJSWGD", "PBD", "PBSWD", "PBSWGD", "XTI2D", "CISBD", "CVBD",
    "CTEMPD", "CISBKD", "DIVXD", "VDIFFJD", "JS0S", "JS0SWS", "JS0SWGS", "NJS", "NJSWS", "NJSWGS", "XTIS", "CJS", "CJSWS", "CJSWGS", "MJS", "MJSWS",
    "MJSWGS", "PBS", "PBSWS", "PBSWGS", "XTI2S", "CISBS", "CVBS", "CTEMPS", "CISBKS", "DIVXS", "VDIFFJS", "CORECOVERY", "NDIBOT", "INJ1", "INJ2", "NQS",
    "TAU", "WI", "DEPNQS", "TAUT", "INJT", "LMIN", "LMAX", "WMIN", "WMAX", "LBINN", "WBINN", "LVMAX", "LBGTMP1", "LBGTMP2", "LEG0", "LVFBOVER",
    "LNOVER", "LNOVERS", "LWL2", "LVFBC", "LNSUBC", "LNSUBP", "LSCP1", "LSCP2", "LSCP3", "LSC1", "LSC2", "LSC3", "LPGD1", "LNDEP", "LNINV", "LMUECB0",
    "LMUECB1", "LMUEPH1", "LVTMP", "LWVTH0", "LMUESR1", "LMUETMP", "LSUB1", "LSUB2", "LSVDS", "LSVBS", "LSVGS", "LSUB1SNP", "LSUB2SNP", "LSVDSSNP", "LFN1", "LFN2",
    "LFN3", "LFVBS", "LNSTI", "LWSTI", "LSCSTI1", "LSCSTI2", "LVTHSTI", "LMUESTI1", "LMUESTI2", "LMUESTI3", "LNSUBPSTI1", "LNSUBPSTI2", "LNSUBPSTI3", "LCGSO", "LCGDO", "LCLM1",
    "LCLM2", "LCLM3", "LWFC", "LGIDL1", "LGIDL2", "LGLEAK1", "LGLEAK2", "LGLEAK3", "LGLEAK6", "LGLKSD1", "LGLKSD2", "LGLKB1", "LGLKB2", "LNFTRP", "LNFALP", "LIBPC1",
    "LIBPC2", "LCGBO", "LCVDSOVER", "LFALPH", "LNPEXT", "LPOWRAT", "LRD", "LRD22", "LRD23", "LRD24", "LRDICT1", "LRDOV13", "LRDSLP1", "LRDVB", "LRDVD", "LRDVG11",
    "LRS", "LRTH0", "LVOVER", "WVMAX", "WBGTMP1", "WBGTMP2", "WEG0", "WVFBOVER", "WNOVER", "WNOVERS", "WWL2", "WVFBC", "WNSUBC", "WNSUBP", "WSCP1", "WSCP2",
    "WSCP3", "WSC1", "WSC2", "WSC3", "WPGD1", "WNDEP", "WNINV", "WMUECB0", "WMUECB1", "WMUEPH1", "WVTMP", "WWVTH0", "WMUESR1", "WMUETMP", "WSUB1", "WSUB2",
    "WSVDS", "WSVBS", "WSVGS", "WSUB1SNP", "WSUB2SNP", "WSVDSSNP", "WFN1", "WFN2", "WFN3", "WFVBS", "WNSTI", "WWSTI", "WSCSTI1", "WSCSTI2", "WVTHSTI", "WMUESTI1",
    "WMUESTI2", "WMUESTI3", "WNSUBPSTI1", "WNSUBPSTI2", "WNSUBPSTI3", "WCGSO", "WCGDO", "WCLM1", "WCLM2", "WCLM3", "WWFC", "WGIDL1", "WGIDL2", "WGLEAK1", "WGLEAK2", "WGLEAK3",
    "WGLEAK6", "WGLKSD1", "WGLKSD2", "WGLKB1", "WGLKB2", "WNFTRP", "WNFALP", "WIBPC1", "WIBPC2", "WCGBO", "WCVDSOVER", "WFALPH", "WNPEXT", "WPOWRAT", "WRD", "WRD22",
    "WRD23", "WRD24", "WRDICT1", "WRDOV13", "WRDSLP1", "WRDVB", "WRDVD", "WRDVG11", "WRS", "WRTH0", "WVOVER", "PVMAX", "PBGTMP1", "PBGTMP2", "PEG0", "PVFBOVER",
    "PNOVER", "PNOVERS", "PWL2", "PVFBC", "PNSUBC", "PNSUBP", "PSCP1", "PSCP2", "PSCP3", "PSC1", "PSC2", "PSC3", "PPGD1", "PNDEP", "PNINV", "PMUECB0",
    "PMUECB1", "PMUEPH1", "PVTMP", "PWVTH0", "PMUESR1", "PMUETMP", "PSUB1", "PSUB2", "PSVDS", "PSVBS", "PSVGS", "PSUB1SNP", "PSUB2SNP", "PSVDSSNP", "PFN1", "PFN2",
    "PFN3", "PFVBS", "PNSTI", "PWSTI", "PSCSTI1", "PSCSTI2", "PVTHSTI", "PMUESTI1", "PMUESTI2", "PMUESTI3", "PNSUBPSTI1", "PNSUBPSTI2", "PNSUBPSTI3", "PCGSO", "PCGDO", "PCLM1",
    "PCLM2", "PCLM3", "PWFC", "PGIDL1", "PGIDL2", "PGLEAK1", "PGLEAK2", "PGLEAK3", "PGLEAK6", "PGLKSD1", "PGLKSD2", "PGLKB1", "PGLKB2", "PNFTRP", "PNFALP", "PIBPC1",
    "PIBPC2", "PCGBO", "PCVDSOVER", "PFALPH", "PNPEXT", "PPOWRAT", "PRD", "PRD22", "PRD23", "PRD24", "PRDICT1", "PRDOV13", "PRDSLP1", "PRDVB", "PRDVD", "PRDVG11",
    "PRS", "PRTH0", "PVOVER", "LJS0", "LJS0SW", "LNJ", "LCISBK", "LVDIFFJ", "LJS0D", "LJS0SWD", "LNJD", "LCISBKD", "LVDIFFJD", "LJS0S", "LJS0SWS", "LNJS",
    "LCISBKS", "LVDIFFJS", "WJS0", "WJS0SW", "WNJ", "WCISBK", "WVDIFFJ", "WJS0D", "WJS0SWD", "WNJD", "WCISBKD", "WVDIFFJD", "WJS0S", "WJS0SWS", "WNJS", "WCISBKS",
    "WVDIFFJS", "PJS0", "PJS0SW", "PNJ", "PCISBK", "PVDIFFJ", "PJS0D", "PJS0SWD", "PNJD", "PCISBKD", "PVDIFFJD", "PJS0S", "PJS0SWS", "PNJS", "PCISBKS", "PVDIFFJS",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 864] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 864] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
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
    false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 864] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -273.15, label: "-273.15" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -0.5, label: "-0.5" }),
    None, None, None, None, Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 864] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-5, label: "1e-5" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 20.0, label: "20.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, Some(ParameterBound { value: 8.0, label: "8.0" }), Some(ParameterBound { value: 8.0, label: "8.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), None,
    Some(ParameterBound { value: 0.5, label: "0.5" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
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
];

const PARAMETER_RANGE_FLAGS: [u8; 864] = [
    2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 3, 0, 3, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 3,
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 2, 3, 0, 3, 0, 0, 0,
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 3, 2, 2, 3, 0, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 3, 3, 3, 0,
    0, 0, 0, 3, 0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 0, 0, 2, 2, 3, 0, 3, 0, 0, 0,
    0, 0, 0, 0, 2, 3, 2, 0, 0, 2, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 864] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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
];

fn parameter_computed_min_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        344 => Some(ParameterBound { value: (params[343] * 1e-9), label: "computed lower-bound expression" }),
        422 => Some(ParameterBound { value: (-(params[67] + params[68])), label: "computed lower-bound expression" }),
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
        423 => Some(ParameterBound { value: (params[67] + params[68]), label: "computed upper-bound expression" }),
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
    pub nodes: [usize; 18],
    pub branches: [usize; 12],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 864]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<21, 0>>,
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
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 14;
    pub const NODE_COUNT: usize = 18;
    pub const INTERNAL_NODE_NAMES: [&str; 14] = ["temp", "dp", "gp", "sp", "bp", "db", "sb", "qi", "qb", "qbd", "n", "charge_A", "charge_K", "depl_A"];

    pub const BRANCH_COUNT: usize = 12;
    pub const PARAMETER_COUNT: usize = 864;
    pub const VARIABLE_COUNT: usize = 3410;
    pub const DDT_STATE_COUNT: usize = 21;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "5795f33919b49219b7bb31283d65a2d8dd339433dfbf3d6d20bf8d23c220d9d3";
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
        let mut values = Vec::with_capacity(105);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(21);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 105);
        debug_assert_eq!(state.flags.len(), 21);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimhv_n4_va'", name));
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
