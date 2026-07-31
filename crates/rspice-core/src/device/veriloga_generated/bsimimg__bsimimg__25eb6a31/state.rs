#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 760],
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
            const DEFAULTS_0: [f64; 13] = [
                3e-8, 1e-6, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 13);
            {
                let params = &mut *ptr;
                params[13] = (-params[12]);
                validate_parameter("WELLTYPE", params[13], true, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 33] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-9,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(14), 33);
            {
                let params = &mut *ptr;
                params[47] = params[45];
                validate_parameter("EOT1P", params[47], false, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 10] = [
                0.0, 8e-9, 1e22, 2e26, 5e23, 4.05, 1.1e16, 1.12,
                2.86e25, 4.61,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(48), 10);
            {
                let params = &mut *ptr;
                params[58] = if (params[13] == (-1.0)) { (params[53] + params[55]) } else { params[53] };
                validate_parameter("PHIG2", params[58], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 40] = [
                11.9, 3.9, 0.0, 0.0, 0.0, 0.14, 0.14, 0.0,
                0.0, 0.0, 0.0, 0.0, 19.2, 0.45, 0.045, 2.0,
                0.0, 0.375, 0.0, 0.0, 0.0, 1e-7, 0.0, 1e-7,
                0.0, 0.0, -0.32, 8.2e-9, 0.0, 1e-9, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.54, 0.001, 0.66,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(59), 40);
            {
                let params = &mut *ptr;
                params[99] = params[45];
                validate_parameter("TOXP", params[99], false, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 3] = [
                85000.0, 0.0, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(100), 3);
            {
                let params = &mut *ptr;
                params[103] = params[100];
                validate_finite_parameter("VSAT1", params[103]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[104] = params[101];
                validate_finite_parameter("AVSAT1", params[104]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[105] = params[102];
                validate_finite_parameter("BVSAT1", params[105]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[106] = params[100];
                validate_finite_parameter("VSATCV", params[106]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[107] = params[101];
                validate_finite_parameter("AVSATCV", params[107]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[108] = params[102];
                validate_finite_parameter("BVSATCV", params[108]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 89] = [
                1.0, 1.0, 1.0, 0.0, 4.0, 0.0, 1.0, 0.0,
                0.0, 1e-7, -0.00156, 0.0, 0.0, 0.004, 0.0, 0.0,
                0.0, 1e-7, 0.0, 1e-7, 0.0, 0.0, 0.0, 0.0,
                1e-7, 0.01, 0.03, 2.0, 0.0, 1.0, 0.3, 0.0,
                1e-7, 2.5, 0.0, 1e-7, 0.0, 0.0, 1e-7, 0.0,
                0.0, 5e-8, 0.0, 0.0, 5e-8, 0.01, 1.0, 0.0,
                -0.0015, 0.001032, 0.0, 0.0, -0.004775, 0.0, 0.0, 0.0,
                1e-7, 0.03, 0.3, 0.0, 1e-7, 2.5, 0.0, 1e-7,
                0.0, 0.0, 1e-7, 0.0, 0.0, 5e-8, 0.0, 0.0,
                5e-8, 1.0, 0.0, 0.0, 1e-7, 2.0, 0.0, 1.0,
                0.0, 0.0, 100.0, 0.0, 1e-7, 0.0, 50.0, 0.0,
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(109), 89);
            {
                let params = &mut *ptr;
                params[198] = params[194];
                validate_parameter("RDWMIN", params[198], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[199] = params[195];
                validate_parameter("RDW", params[199], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[200] = params[196];
                validate_finite_parameter("ARDW", params[200]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[201] = params[197];
                validate_finite_parameter("BRDW", params[201]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 14] = [
                0.0, 0.0, 1.0, 0.001, 1.3, 0.0002, 1.06, 1.0,
                0.013, 0.0, 1e-7, 0.0, 0.013, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(202), 14);
            {
                let params = &mut *ptr;
                params[216] = params[215];
                validate_parameter("RSHD", params[216], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 18] = [
                0.0111, 0.000949, 0.006, 1.1, 3.0, 0.0136, 0.00171, 0.075,
                1.0, 0.0136, 0.00171, 0.075, 1.0, 1.0, 0.0136, 0.00171,
                0.075, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(217), 18);
            {
                let params = &mut *ptr;
                params[235] = params[234];
                validate_parameter("DLCIGD", params[235], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[236] = params[231];
                validate_finite_parameter("AIGD", params[236]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[237] = params[232];
                validate_finite_parameter("BIGD", params[237]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[238] = params[233];
                validate_finite_parameter("CIGD", params[238]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 4] = [
                1.2e-9, 1.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(239), 4);
            {
                let params = &mut *ptr;
                params[243] = params[242];
                validate_finite_parameter("DIGD", params[243]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 6] = [
                6.055e-12, 300000000.0, 0.2, 1.0, 1.0, 0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(244), 6);
            {
                let params = &mut *ptr;
                params[250] = params[244];
                validate_finite_parameter("AGISL", params[250]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[251] = params[245];
                validate_finite_parameter("BGISL", params[251]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[252] = params[246];
                validate_finite_parameter("EGISL", params[252]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[253] = params[247];
                validate_finite_parameter("PGISL", params[253]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[254] = params[248];
                validate_finite_parameter("VBGISL", params[254]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[255] = params[249];
                validate_finite_parameter("VBEGISL", params[255]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 4] = [
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(256), 4);
            {
                let params = &mut *ptr;
                params[260] = params[259];
                validate_finite_parameter("LOVD", params[260]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(261), 1);
            {
                let params = &mut *ptr;
                params[262] = params[261];
                validate_parameter("CFD", params[262], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(263), 1);
            {
                let params = &mut *ptr;
                params[264] = params[263];
                validate_parameter("CGDL", params[264], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 1] = [
                0.6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(265), 1);
            {
                let params = &mut *ptr;
                params[266] = params[265];
                validate_parameter("CKAPPAD", params[266], false, Some((0.02, "0.02")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(267), 3);
            {
                let params = &mut *ptr;
                params[270] = params[268];
                validate_finite_parameter("PCOVBD0", params[270]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[271] = params[269];
                validate_finite_parameter("PCOVBD1", params[271]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 7] = [
                1.0, 0.0, -1.0, 0.12, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(272), 7);
            {
                let params = &mut *ptr;
                params[279] = params[272];
                validate_finite_parameter("KBG0NW", params[279]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[280] = params[273];
                validate_finite_parameter("KBG1NW", params[280]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[281] = params[274];
                validate_finite_parameter("KBG2NW", params[281]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[282] = params[275];
                validate_finite_parameter("DBGNW", params[282]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[283] = params[276];
                validate_finite_parameter("BPFACTORNW", params[283]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[284] = params[277];
                validate_finite_parameter("VKNEE1NW", params[284]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[285] = params[278];
                validate_parameter("VKNEE2NW", params[285], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 5] = [
                1.0, 41000000.0, 6.25e39, 3.125e24, 87500000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(286), 5);
            {
                let params = &mut *ptr;
                params[291] = params[288];
                validate_parameter("NOIA2", params[291], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 15] = [
                2.0, 1.2, 0.05, 1.0, 0.0, 27.0, 400.0, 0.000702,
                1108.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.003,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(292), 15);
            {
                let params = &mut *ptr;
                params[307] = params[306];
                validate_finite_parameter("TGISL", params[307]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 17] = [
                2.5, 0.0, 0.01, 1e-5, 0.0, 0.0, 0.0, 1.0,
                0.1, 12.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(308), 17);
            {
                let params = &mut *ptr;
                params[325] = params[322];
                validate_finite_parameter("LRDW", params[325]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[326] = params[323];
                validate_finite_parameter("WRDW", params[326]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[327] = params[324];
                validate_finite_parameter("PRDW", params[327]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 225] = [
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
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(328), 225);
            {
                let params = &mut *ptr;
                params[553] = params[550];
                validate_finite_parameter("LTGISL", params[553]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[554] = params[551];
                validate_finite_parameter("WTGISL", params[554]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[555] = params[552];
                validate_finite_parameter("PTGISL", params[555]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 63] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(556), 63);
            {
                let params = &mut *ptr;
                params[619] = params[601];
                validate_finite_parameter("LAGISL", params[619]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[620] = params[602];
                validate_finite_parameter("WAGISL", params[620]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[621] = params[603];
                validate_finite_parameter("PAGISL", params[621]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[622] = params[604];
                validate_finite_parameter("LBGISL", params[622]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[623] = params[605];
                validate_finite_parameter("WBGISL", params[623]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[624] = params[606];
                validate_finite_parameter("PBGISL", params[624]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[625] = params[607];
                validate_finite_parameter("LEGISL", params[625]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[626] = params[608];
                validate_finite_parameter("WEGISL", params[626]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[627] = params[609];
                validate_finite_parameter("PEGISL", params[627]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[628] = params[610];
                validate_finite_parameter("LPGISL", params[628]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[629] = params[611];
                validate_finite_parameter("WPGISL", params[629]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[630] = params[612];
                validate_finite_parameter("PPGISL", params[630]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[631] = params[613];
                validate_finite_parameter("LVBGISL", params[631]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[632] = params[614];
                validate_finite_parameter("WVBGISL", params[632]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[633] = params[615];
                validate_finite_parameter("PVBGISL", params[633]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[634] = params[616];
                validate_finite_parameter("LVBEGISL", params[634]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[635] = params[617];
                validate_finite_parameter("WVBEGISL", params[635]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[636] = params[618];
                validate_finite_parameter("PVBEGISL", params[636]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(637), 3);
            {
                let params = &mut *ptr;
                params[640] = params[637];
                validate_finite_parameter("LAIGD", params[640]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[641] = params[638];
                validate_finite_parameter("WAIGD", params[641]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[642] = params[639];
                validate_finite_parameter("PAIGD", params[642]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(643), 3);
            {
                let params = &mut *ptr;
                params[646] = params[643];
                validate_finite_parameter("LBIGD", params[646]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[647] = params[644];
                validate_finite_parameter("WBIGD", params[647]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[648] = params[645];
                validate_finite_parameter("PBIGD", params[648]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(649), 3);
            {
                let params = &mut *ptr;
                params[652] = params[649];
                validate_finite_parameter("LCIGD", params[652]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[653] = params[650];
                validate_finite_parameter("WCIGD", params[653]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[654] = params[651];
                validate_finite_parameter("PCIGD", params[654]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(655), 3);
            {
                let params = &mut *ptr;
                params[658] = params[655];
                validate_finite_parameter("LDIGD", params[658]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[659] = params[656];
                validate_finite_parameter("WDIGD", params[659]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[660] = params[657];
                validate_finite_parameter("PDIGD", params[660]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 9] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(661), 9);
            {
                let params = &mut *ptr;
                params[670] = params[667];
                validate_finite_parameter("LLOVD", params[670]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[671] = params[668];
                validate_finite_parameter("WLOVD", params[671]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[672] = params[669];
                validate_finite_parameter("PLOVD", params[672]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(673), 3);
            {
                let params = &mut *ptr;
                params[676] = params[673];
                validate_finite_parameter("LCFD", params[676]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[677] = params[674];
                validate_finite_parameter("WCFD", params[677]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[678] = params[675];
                validate_finite_parameter("PCFD", params[678]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 81] = [
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
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(679), 81);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 760] = [
    ("l", 0), ("w", 1), ("nf", 2), ("as", 3), ("ad", 4), ("ps", 5), ("pd", 6), ("nrs", 7), ("nrd", 8), ("dtemp", 9), ("delvtrand", 10), ("u0mult", 11), ("type", 12), ("welltype", 13), ("rdsmod", 14), ("gidlmod", 15),
    ("igcmod", 16), ("igbmod", 17), ("shmod", 18), ("rgatemod", 19), ("nqsmod", 20), ("nfmod", 21), ("fnmod", 22), ("xl", 23), ("xw", 24), ("lint", 25), ("ll", 26), ("lw", 27), ("lwl", 28), ("lln", 29), ("lwn", 30), ("wint", 31),
    ("wl", 32), ("ww", 33), ("wwl", 34), ("wln", 35), ("wwn", 36), ("dlc", 37), ("llc", 38), ("lwc", 39), ("lwlc", 40), ("dwc", 41), ("wlc", 42), ("wwc", 43), ("wwlc", 44), ("eot1", 45), ("eot2", 46), ("eot1p", 47),
    ("dtox1", 48), ("tsi", 49), ("nbody", 50), ("nsd", 51), ("nbg", 52), ("easub", 53), ("ni0sub", 54), ("bg0sub", 55), ("nc0sub", 56), ("phig1", 57), ("phig2", 58), ("epsrsub", 59), ("epsrox1", 60), ("ascl", 61), ("bscl", 62), ("cit", 63),
    ("cdsc", 64), ("cdscd", 65), ("cbgcbg0", 66), ("cbgcbg0p", 67), ("cbgcbg", 68), ("cbgcbgp", 69), ("cbgcbgd", 70), ("dvt0", 71), ("dvt1", 72), ("phin", 73), ("eta0", 74), ("eta1", 75), ("dsub", 76), ("dvtp0", 77), ("dvtp1", 78), ("advtp0", 79),
    ("bdvtp0", 80), ("advtp1", 81), ("bdvtp1", 82), ("dvtp2", 83), ("etab", 84), ("k1rsce", 85), ("lpe0", 86), ("dsc0", 87), ("dsc1", 88), ("k0", 89), ("k01", 90), ("k0si", 91), ("k0si1", 92), ("k0sisat", 93), ("k0sisat1", 94), ("qmtcencv", 95),
    ("etaqm", 96), ("qm0", 97), ("pqm", 98), ("toxp", 99), ("vsat", 100), ("avsat", 101), ("bvsat", 102), ("vsat1", 103), ("avsat1", 104), ("bvsat1", 105), ("vsatcv", 106), ("avsatcv", 107), ("bvsatcv", 108), ("deltavsat", 109), ("ksativ", 110), ("ksubiv", 111),
    ("ksativb", 112), ("mexp", 113), ("amexp", 114), ("bmexp", 115), ("ptwg", 116), ("aptwg", 117), ("bptwg", 118), ("at", 119), ("atl", 120), ("tmexp", 121), ("ptwgt", 122), ("ptwgb", 123), ("ptwgb2", 124), ("aptwgb", 125), ("bptwgb", 126), ("aptwgb2", 127),
    ("bptwgb2", 128), ("vsatb", 129), ("atb", 130), ("atbl", 131), ("avsatb", 132), ("bvsatb", 133), ("dvsatclamp", 134), ("u0", 135), ("etamob", 136), ("up", 137), ("lpa", 138), ("ua", 139), ("aua", 140), ("bua", 141), ("eu", 142), ("aeu", 143),
    ("beu", 144), ("uc", 145), ("auc", 146), ("buc", 147), ("ud", 148), ("aud", 149), ("bud", 150), ("udb", 151), ("audb", 152), ("budb", 153), ("dmobclamp", 154), ("ucs", 155), ("ute", 156), ("utl", 157), ("ua1", 158), ("uc1", 159),
    ("ud1", 160), ("ucste", 161), ("chargewf", 162), ("eub", 163), ("aeub", 164), ("beub", 165), ("u02", 166), ("ua2", 167), ("aua2", 168), ("bua2", 169), ("eu2", 170), ("aeu2", 171), ("beu2", 172), ("uc2", 173), ("auc2", 174), ("buc2", 175),
    ("ud2", 176), ("aud2", 177), ("bud2", 178), ("udb2", 179), ("audb2", 180), ("budb2", 181), ("ucs2", 182), ("eub2", 183), ("aeub2", 184), ("beub2", 185), ("etamob2", 186), ("up2", 187), ("lpa2", 188), ("chargewf2", 189), ("rdswmin", 190), ("rdsw", 191),
    ("ardsw", 192), ("brdsw", 193), ("rswmin", 194), ("rsw", 195), ("arsw", 196), ("brsw", 197), ("rdwmin", 198), ("rdw", 199), ("ardw", 200), ("brdw", 201), ("prwg", 202), ("prwb", 203), ("wr", 204), ("prt", 205), ("pdibl1", 206), ("pdibl2", 207),
    ("drout", 208), ("pvag", 209), ("pclm", 210), ("apclm", 211), ("bpclm", 212), ("pclmg", 213), ("pclmcv", 214), ("rshs", 215), ("rshd", 216), ("aigbinv", 217), ("bigbinv", 218), ("cigbinv", 219), ("eigbinv", 220), ("nigbinv", 221), ("aigbacc", 222), ("bigbacc", 223),
    ("cigbacc", 224), ("nigbacc", 225), ("aigc", 226), ("bigc", 227), ("cigc", 228), ("pigcd", 229), ("digc", 230), ("aigs", 231), ("bigs", 232), ("cigs", 233), ("dlcigs", 234), ("dlcigd", 235), ("aigd", 236), ("bigd", 237), ("cigd", 238), ("toxref", 239),
    ("ntox", 240), ("poxedge", 241), ("digs", 242), ("digd", 243), ("agidl", 244), ("bgidl", 245), ("egidl", 246), ("pgidl", 247), ("vbgidl", 248), ("vbegidl", 249), ("agisl", 250), ("bgisl", 251), ("egisl", 252), ("pgisl", 253), ("vbgisl", 254), ("vbegisl", 255),
    ("alpha0", 256), ("alpha1", 257), ("beta0", 258), ("lovs", 259), ("lovd", 260), ("cfs", 261), ("cfd", 262), ("cgsl", 263), ("cgdl", 264), ("ckappas", 265), ("ckappad", 266), ("csdbgsw", 267), ("pcovbs0", 268), ("pcovbs1", 269), ("pcovbd0", 270), ("pcovbd1", 271),
    ("kbg0pw", 272), ("kbg1pw", 273), ("kbg2pw", 274), ("dbgpw", 275), ("bpfactorpw", 276), ("vknee1pw", 277), ("vknee2pw", 278), ("kbg0nw", 279), ("kbg1nw", 280), ("kbg2nw", 281), ("dbgnw", 282), ("bpfactornw", 283), ("vknee1nw", 284), ("vknee2nw", 285), ("ef", 286), ("em", 287),
    ("noia", 288), ("noib", 289), ("noic", 290), ("noia2", 291), ("smooth", 292), ("mpower", 293), ("qsref", 294), ("ntnoi", 295), ("lintnoi", 296), ("tnom", 297), ("tmaxc", 298), ("tbgasub", 299), ("tbgbsub", 300), ("kt1", 301), ("kt1l", 302), ("kt2", 303),
    ("kt2l", 304), ("iit", 305), ("tgidl", 306), ("tgisl", 307), ("igt", 308), ("teta0", 309), ("rth0", 310), ("cth0", 311), ("wth0", 312), ("xgw", 313), ("xgl", 314), ("ngcon", 315), ("rshg", 316), ("xrcrg1", 317), ("xrcrg2", 318), ("lrdsw", 319),
    ("wrdsw", 320), ("prdsw", 321), ("lrsw", 322), ("wrsw", 323), ("prsw", 324), ("lrdw", 325), ("wrdw", 326), ("prdw", 327), ("lprwg", 328), ("wprwg", 329), ("pprwg", 330), ("lprwb", 331), ("wprwb", 332), ("pprwb", 333), ("lwr", 334), ("wwr", 335),
    ("pwr", 336), ("lphig1", 337), ("wphig1", 338), ("pphig1", 339), ("lphig2", 340), ("wphig2", 341), ("pphig2", 342), ("lnsd", 343), ("wnsd", 344), ("pnsd", 345), ("lnbody", 346), ("wnbody", 347), ("pnbody", 348), ("lcit", 349), ("wcit", 350), ("pcit", 351),
    ("lcdsc", 352), ("wcdsc", 353), ("pcdsc", 354), ("lcdscd", 355), ("wcdscd", 356), ("pcdscd", 357), ("lcbgcbg", 358), ("wcbgcbg", 359), ("pcbgcbg", 360), ("lbpfactorpw", 361), ("wbpfactorpw", 362), ("pbpfactorpw", 363), ("lvknee1pw", 364), ("wvknee1pw", 365), ("pvknee1pw", 366), ("lvknee2pw", 367),
    ("wvknee2pw", 368), ("pvknee2pw", 369), ("ldbgpw", 370), ("wdbgpw", 371), ("pdbgpw", 372), ("lkbg0pw", 373), ("wkbg0pw", 374), ("pkbg0pw", 375), ("lkbg1pw", 376), ("wkbg1pw", 377), ("pkbg1pw", 378), ("lkbg2pw", 379), ("wkbg2pw", 380), ("pkbg2pw", 381), ("lbpfactornw", 382), ("wbpfactornw", 383),
    ("pbpfactornw", 384), ("lvknee1nw", 385), ("wvknee1nw", 386), ("pvknee1nw", 387), ("lvknee2nw", 388), ("wvknee2nw", 389), ("pvknee2nw", 390), ("ldbgnw", 391), ("wdbgnw", 392), ("pdbgnw", 393), ("lkbg0nw", 394), ("wkbg0nw", 395), ("pkbg0nw", 396), ("lkbg1nw", 397), ("wkbg1nw", 398), ("pkbg1nw", 399),
    ("lkbg2nw", 400), ("wkbg2nw", 401), ("pkbg2nw", 402), ("ldvt0", 403), ("wdvt0", 404), ("pdvt0", 405), ("ldvt1", 406), ("wdvt1", 407), ("pdvt1", 408), ("lphin", 409), ("wphin", 410), ("pphin", 411), ("leta0", 412), ("weta0", 413), ("peta0", 414), ("leta1", 415),
    ("weta1", 416), ("peta1", 417), ("letab", 418), ("wetab", 419), ("petab", 420), ("ldsub", 421), ("wdsub", 422), ("pdsub", 423), ("lk1rsce", 424), ("wk1rsce", 425), ("pk1rsce", 426), ("llpe0", 427), ("wlpe0", 428), ("plpe0", 429), ("ldsc0", 430), ("wdsc0", 431),
    ("pdsc0", 432), ("ldsc1", 433), ("wdsc1", 434), ("pdsc1", 435), ("lascl", 436), ("wascl", 437), ("pascl", 438), ("lbscl", 439), ("wbscl", 440), ("pbscl", 441), ("lk0", 442), ("wk0", 443), ("pk0", 444), ("lk01", 445), ("wk01", 446), ("pk01", 447),
    ("lk0si", 448), ("wk0si", 449), ("pk0si", 450), ("lk0si1", 451), ("wk0si1", 452), ("pk0si1", 453), ("lk0sisat", 454), ("nk0sisat", 455), ("pk0sisat", 456), ("lk0sisat1", 457), ("nk0sisat1", 458), ("pk0sisat1", 459), ("lmexp", 460), ("wmexp", 461), ("pmexp", 462), ("lptwg", 463),
    ("wptwg", 464), ("pptwg", 465), ("lptwgb", 466), ("wptwgb", 467), ("pptwgb", 468), ("lptwgb2", 469), ("wptwgb2", 470), ("pptwgb2", 471), ("lptwgt", 472), ("wptwgt", 473), ("pptwgt", 474), ("lu0", 475), ("wu0", 476), ("pu0", 477), ("lua", 478), ("wua", 479),
    ("pua", 480), ("luc", 481), ("wuc", 482), ("puc", 483), ("lud", 484), ("wud", 485), ("pud", 486), ("lucs", 487), ("wucs", 488), ("pucs", 489), ("leu", 490), ("weu", 491), ("peu", 492), ("leub", 493), ("weub", 494), ("peub", 495),
    ("lutl", 496), ("wutl", 497), ("putl", 498), ("lute", 499), ("wute", 500), ("pute", 501), ("lua1", 502), ("wua1", 503), ("pua1", 504), ("lud1", 505), ("wud1", 506), ("pud1", 507), ("lucste", 508), ("wucste", 509), ("pucste", 510), ("letamob", 511),
    ("wetamob", 512), ("petamob", 513), ("lu02", 514), ("wu02", 515), ("pu02", 516), ("lua2", 517), ("wua2", 518), ("pua2", 519), ("luc2", 520), ("wuc2", 521), ("puc2", 522), ("lud2", 523), ("wud2", 524), ("pud2", 525), ("lucs2", 526), ("wucs2", 527),
    ("pucs2", 528), ("leu2", 529), ("weu2", 530), ("peu2", 531), ("leub2", 532), ("weub2", 533), ("peub2", 534), ("letamob2", 535), ("wetamob2", 536), ("petamob2", 537), ("lat", 538), ("wat", 539), ("pat", 540), ("latb", 541), ("watb", 542), ("patb", 543),
    ("lprt", 544), ("wprt", 545), ("pprt", 546), ("liit", 547), ("wiit", 548), ("piit", 549), ("ltgidl", 550), ("wtgidl", 551), ("ptgidl", 552), ("ltgisl", 553), ("wtgisl", 554), ("ptgisl", 555), ("ligt", 556), ("wigt", 557), ("pigt", 558), ("lpclm", 559),
    ("wpclm", 560), ("ppclm", 561), ("lpclmcv", 562), ("wpclmcv", 563), ("ppclmcv", 564), ("ldrout", 565), ("wdrout", 566), ("pdrout", 567), ("lpdibl1", 568), ("wpdibl1", 569), ("ppdibl1", 570), ("lpdibl2", 571), ("wpdibl2", 572), ("ppdibl2", 573), ("lpvag", 574), ("wpvag", 575),
    ("ppvag", 576), ("lalpha0", 577), ("walpha0", 578), ("palpha0", 579), ("lalpha1", 580), ("walpha1", 581), ("palpha1", 582), ("lbeta0", 583), ("wbeta0", 584), ("pbeta0", 585), ("laigc", 586), ("waigc", 587), ("paigc", 588), ("lbigc", 589), ("wbigc", 590), ("pbigc", 591),
    ("lcigc", 592), ("wcigc", 593), ("pcigc", 594), ("ldigc", 595), ("wdigc", 596), ("pdigc", 597), ("lpigcd", 598), ("wpigcd", 599), ("ppigcd", 600), ("lagidl", 601), ("wagidl", 602), ("pagidl", 603), ("lbgidl", 604), ("wbgidl", 605), ("pbgidl", 606), ("legidl", 607),
    ("wegidl", 608), ("pegidl", 609), ("lpgidl", 610), ("wpgidl", 611), ("ppgidl", 612), ("lvbgidl", 613), ("wvbgidl", 614), ("pvbgidl", 615), ("lvbegidl", 616), ("wvbegidl", 617), ("pvbegidl", 618), ("lagisl", 619), ("wagisl", 620), ("pagisl", 621), ("lbgisl", 622), ("wbgisl", 623),
    ("pbgisl", 624), ("legisl", 625), ("wegisl", 626), ("pegisl", 627), ("lpgisl", 628), ("wpgisl", 629), ("ppgisl", 630), ("lvbgisl", 631), ("wvbgisl", 632), ("pvbgisl", 633), ("lvbegisl", 634), ("wvbegisl", 635), ("pvbegisl", 636), ("laigs", 637), ("waigs", 638), ("paigs", 639),
    ("laigd", 640), ("waigd", 641), ("paigd", 642), ("lbigs", 643), ("wbigs", 644), ("pbigs", 645), ("lbigd", 646), ("wbigd", 647), ("pbigd", 648), ("lcigs", 649), ("wcigs", 650), ("pcigs", 651), ("lcigd", 652), ("wcigd", 653), ("pcigd", 654), ("ldigs", 655),
    ("wdigs", 656), ("pdigs", 657), ("ldigd", 658), ("wdigd", 659), ("pdigd", 660), ("lntox", 661), ("wntox", 662), ("pntox", 663), ("lpoxedge", 664), ("wpoxedge", 665), ("ppoxedge", 666), ("llovs", 667), ("wlovs", 668), ("plovs", 669), ("llovd", 670), ("wlovd", 671),
    ("plovd", 672), ("lcfs", 673), ("wcfs", 674), ("pcfs", 675), ("lcfd", 676), ("wcfd", 677), ("pcfd", 678), ("lvsat", 679), ("wvsat", 680), ("pvsat", 681), ("lvsatb", 682), ("wvsatb", 683), ("pvsatb", 684), ("lvsat1", 685), ("wvsat1", 686), ("pvsat1", 687),
    ("lvsatcv", 688), ("wvsatcv", 689), ("pvsatcv", 690), ("lksativ", 691), ("wksativ", 692), ("pksativ", 693), ("lksubiv", 694), ("wksubiv", 695), ("pksubiv", 696), ("lksativb", 697), ("wksativb", 698), ("pksativb", 699), ("lup", 700), ("wup", 701), ("pup", 702), ("lup2", 703),
    ("wup2", 704), ("pup2", 705), ("laigbinv", 706), ("waigbinv", 707), ("paigbinv", 708), ("lbigbinv", 709), ("wbigbinv", 710), ("pbigbinv", 711), ("lcigbinv", 712), ("wcigbinv", 713), ("pcigbinv", 714), ("leigbinv", 715), ("weigbinv", 716), ("peigbinv", 717), ("lnigbinv", 718), ("wnigbinv", 719),
    ("pnigbinv", 720), ("laigbacc", 721), ("waigbacc", 722), ("paigbacc", 723), ("lbigbacc", 724), ("wbigbacc", 725), ("pbigbacc", 726), ("lcigbacc", 727), ("wcigbacc", 728), ("pcigbacc", 729), ("lnigbacc", 730), ("wnigbacc", 731), ("pnigbacc", 732), ("lxrcrg1", 733), ("wxrcrg1", 734), ("pxrcrg1", 735),
    ("lxrcrg2", 736), ("wxrcrg2", 737), ("pxrcrg2", 738), ("lqmtcencv", 739), ("wqmtcencv", 740), ("pqmtcencv", 741), ("letaqm", 742), ("wetaqm", 743), ("petaqm", 744), ("lqm0", 745), ("wqm0", 746), ("pqm0", 747), ("lpqm", 748), ("wpqm", 749), ("ppqm", 750), ("lnoia2", 751),
    ("wnoia2", 752), ("pnoia2", 753), ("lmpower", 754), ("wmpower", 755), ("pmpower", 756), ("lqsref", 757), ("wqsref", 758), ("pqsref", 759),
];

pub(crate) const PARAMETER_MODEL_FLAGS: [bool; 760] = [
    false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
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
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 760] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 760] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 760] = [
    "L", "W", "NF", "AS", "AD", "PS", "PD", "NRS", "NRD", "DTEMP", "DELVTRAND", "U0MULT", "TYPE", "WELLTYPE", "RDSMOD", "GIDLMOD",
    "IGCMOD", "IGBMOD", "SHMOD", "RGATEMOD", "NQSMOD", "NFMOD", "FNMOD", "XL", "XW", "LINT", "LL", "LW", "LWL", "LLN", "LWN", "WINT",
    "WL", "WW", "WWL", "WLN", "WWN", "DLC", "LLC", "LWC", "LWLC", "DWC", "WLC", "WWC", "WWLC", "EOT1", "EOT2", "EOT1P",
    "DTOX1", "TSI", "NBODY", "NSD", "NBG", "EASUB", "NI0SUB", "BG0SUB", "NC0SUB", "PHIG1", "PHIG2", "EPSRSUB", "EPSROX1", "ASCL", "BSCL", "CIT",
    "CDSC", "CDSCD", "CBGCBG0", "CBGCBG0P", "CBGCBG", "CBGCBGP", "CBGCBGD", "DVT0", "DVT1", "PHIN", "ETA0", "ETA1", "DSUB", "DVTP0", "DVTP1", "ADVTP0",
    "BDVTP0", "ADVTP1", "BDVTP1", "DVTP2", "ETAB", "K1RSCE", "LPE0", "DSC0", "DSC1", "K0", "K01", "K0SI", "K0SI1", "K0SISAT", "K0SISAT1", "QMTCENCV",
    "ETAQM", "QM0", "PQM", "TOXP", "VSAT", "AVSAT", "BVSAT", "VSAT1", "AVSAT1", "BVSAT1", "VSATCV", "AVSATCV", "BVSATCV", "DELTAVSAT", "KSATIV", "KSUBIV",
    "KSATIVB", "MEXP", "AMEXP", "BMEXP", "PTWG", "APTWG", "BPTWG", "AT", "ATL", "TMEXP", "PTWGT", "PTWGB", "PTWGB2", "APTWGB", "BPTWGB", "APTWGB2",
    "BPTWGB2", "VSATB", "ATB", "ATBL", "AVSATB", "BVSATB", "DVSATCLAMP", "U0", "ETAMOB", "UP", "LPA", "UA", "AUA", "BUA", "EU", "AEU",
    "BEU", "UC", "AUC", "BUC", "UD", "AUD", "BUD", "UDB", "AUDB", "BUDB", "DMOBCLAMP", "UCS", "UTE", "UTL", "UA1", "UC1",
    "UD1", "UCSTE", "CHARGEWF", "EUB", "AEUB", "BEUB", "U02", "UA2", "AUA2", "BUA2", "EU2", "AEU2", "BEU2", "UC2", "AUC2", "BUC2",
    "UD2", "AUD2", "BUD2", "UDB2", "AUDB2", "BUDB2", "UCS2", "EUB2", "AEUB2", "BEUB2", "ETAMOB2", "UP2", "LPA2", "CHARGEWF2", "RDSWMIN", "RDSW",
    "ARDSW", "BRDSW", "RSWMIN", "RSW", "ARSW", "BRSW", "RDWMIN", "RDW", "ARDW", "BRDW", "PRWG", "PRWB", "WR", "PRT", "PDIBL1", "PDIBL2",
    "DROUT", "PVAG", "PCLM", "APCLM", "BPCLM", "PCLMG", "PCLMCV", "RSHS", "RSHD", "AIGBINV", "BIGBINV", "CIGBINV", "EIGBINV", "NIGBINV", "AIGBACC", "BIGBACC",
    "CIGBACC", "NIGBACC", "AIGC", "BIGC", "CIGC", "PIGCD", "DIGC", "AIGS", "BIGS", "CIGS", "DLCIGS", "DLCIGD", "AIGD", "BIGD", "CIGD", "TOXREF",
    "NTOX", "POXEDGE", "DIGS", "DIGD", "AGIDL", "BGIDL", "EGIDL", "PGIDL", "VBGIDL", "VBEGIDL", "AGISL", "BGISL", "EGISL", "PGISL", "VBGISL", "VBEGISL",
    "ALPHA0", "ALPHA1", "BETA0", "LOVS", "LOVD", "CFS", "CFD", "CGSL", "CGDL", "CKAPPAS", "CKAPPAD", "CSDBGSW", "PCOVBS0", "PCOVBS1", "PCOVBD0", "PCOVBD1",
    "KBG0PW", "KBG1PW", "KBG2PW", "DBGPW", "BPFACTORPW", "VKNEE1PW", "VKNEE2PW", "KBG0NW", "KBG1NW", "KBG2NW", "DBGNW", "BPFACTORNW", "VKNEE1NW", "VKNEE2NW", "EF", "EM",
    "NOIA", "NOIB", "NOIC", "NOIA2", "SMOOTH", "MPOWER", "QSREF", "NTNOI", "LINTNOI", "TNOM", "TMAXC", "TBGASUB", "TBGBSUB", "KT1", "KT1L", "KT2",
    "KT2L", "IIT", "TGIDL", "TGISL", "IGT", "TETA0", "RTH0", "CTH0", "WTH0", "XGW", "XGL", "NGCON", "RSHG", "XRCRG1", "XRCRG2", "LRDSW",
    "WRDSW", "PRDSW", "LRSW", "WRSW", "PRSW", "LRDW", "WRDW", "PRDW", "LPRWG", "WPRWG", "PPRWG", "LPRWB", "WPRWB", "PPRWB", "LWR", "WWR",
    "PWR", "LPHIG1", "WPHIG1", "PPHIG1", "LPHIG2", "WPHIG2", "PPHIG2", "LNSD", "WNSD", "PNSD", "LNBODY", "WNBODY", "PNBODY", "LCIT", "WCIT", "PCIT",
    "LCDSC", "WCDSC", "PCDSC", "LCDSCD", "WCDSCD", "PCDSCD", "LCBGCBG", "WCBGCBG", "PCBGCBG", "LBPFACTORPW", "WBPFACTORPW", "PBPFACTORPW", "LVKNEE1PW", "WVKNEE1PW", "PVKNEE1PW", "LVKNEE2PW",
    "WVKNEE2PW", "PVKNEE2PW", "LDBGPW", "WDBGPW", "PDBGPW", "LKBG0PW", "WKBG0PW", "PKBG0PW", "LKBG1PW", "WKBG1PW", "PKBG1PW", "LKBG2PW", "WKBG2PW", "PKBG2PW", "LBPFACTORNW", "WBPFACTORNW",
    "PBPFACTORNW", "LVKNEE1NW", "WVKNEE1NW", "PVKNEE1NW", "LVKNEE2NW", "WVKNEE2NW", "PVKNEE2NW", "LDBGNW", "WDBGNW", "PDBGNW", "LKBG0NW", "WKBG0NW", "PKBG0NW", "LKBG1NW", "WKBG1NW", "PKBG1NW",
    "LKBG2NW", "WKBG2NW", "PKBG2NW", "LDVT0", "WDVT0", "PDVT0", "LDVT1", "WDVT1", "PDVT1", "LPHIN", "WPHIN", "PPHIN", "LETA0", "WETA0", "PETA0", "LETA1",
    "WETA1", "PETA1", "LETAB", "WETAB", "PETAB", "LDSUB", "WDSUB", "PDSUB", "LK1RSCE", "WK1RSCE", "PK1RSCE", "LLPE0", "WLPE0", "PLPE0", "LDSC0", "WDSC0",
    "PDSC0", "LDSC1", "WDSC1", "PDSC1", "LASCL", "WASCL", "PASCL", "LBSCL", "WBSCL", "PBSCL", "LK0", "WK0", "PK0", "LK01", "WK01", "PK01",
    "LK0SI", "WK0SI", "PK0SI", "LK0SI1", "WK0SI1", "PK0SI1", "LK0SISAT", "NK0SISAT", "PK0SISAT", "LK0SISAT1", "NK0SISAT1", "PK0SISAT1", "LMEXP", "WMEXP", "PMEXP", "LPTWG",
    "WPTWG", "PPTWG", "LPTWGB", "WPTWGB", "PPTWGB", "LPTWGB2", "WPTWGB2", "PPTWGB2", "LPTWGT", "WPTWGT", "PPTWGT", "LU0", "WU0", "PU0", "LUA", "WUA",
    "PUA", "LUC", "WUC", "PUC", "LUD", "WUD", "PUD", "LUCS", "WUCS", "PUCS", "LEU", "WEU", "PEU", "LEUB", "WEUB", "PEUB",
    "LUTL", "WUTL", "PUTL", "LUTE", "WUTE", "PUTE", "LUA1", "WUA1", "PUA1", "LUD1", "WUD1", "PUD1", "LUCSTE", "WUCSTE", "PUCSTE", "LETAMOB",
    "WETAMOB", "PETAMOB", "LU02", "WU02", "PU02", "LUA2", "WUA2", "PUA2", "LUC2", "WUC2", "PUC2", "LUD2", "WUD2", "PUD2", "LUCS2", "WUCS2",
    "PUCS2", "LEU2", "WEU2", "PEU2", "LEUB2", "WEUB2", "PEUB2", "LETAMOB2", "WETAMOB2", "PETAMOB2", "LAT", "WAT", "PAT", "LATB", "WATB", "PATB",
    "LPRT", "WPRT", "PPRT", "LIIT", "WIIT", "PIIT", "LTGIDL", "WTGIDL", "PTGIDL", "LTGISL", "WTGISL", "PTGISL", "LIGT", "WIGT", "PIGT", "LPCLM",
    "WPCLM", "PPCLM", "LPCLMCV", "WPCLMCV", "PPCLMCV", "LDROUT", "WDROUT", "PDROUT", "LPDIBL1", "WPDIBL1", "PPDIBL1", "LPDIBL2", "WPDIBL2", "PPDIBL2", "LPVAG", "WPVAG",
    "PPVAG", "LALPHA0", "WALPHA0", "PALPHA0", "LALPHA1", "WALPHA1", "PALPHA1", "LBETA0", "WBETA0", "PBETA0", "LAIGC", "WAIGC", "PAIGC", "LBIGC", "WBIGC", "PBIGC",
    "LCIGC", "WCIGC", "PCIGC", "LDIGC", "WDIGC", "PDIGC", "LPIGCD", "WPIGCD", "PPIGCD", "LAGIDL", "WAGIDL", "PAGIDL", "LBGIDL", "WBGIDL", "PBGIDL", "LEGIDL",
    "WEGIDL", "PEGIDL", "LPGIDL", "WPGIDL", "PPGIDL", "LVBGIDL", "WVBGIDL", "PVBGIDL", "LVBEGIDL", "WVBEGIDL", "PVBEGIDL", "LAGISL", "WAGISL", "PAGISL", "LBGISL", "WBGISL",
    "PBGISL", "LEGISL", "WEGISL", "PEGISL", "LPGISL", "WPGISL", "PPGISL", "LVBGISL", "WVBGISL", "PVBGISL", "LVBEGISL", "WVBEGISL", "PVBEGISL", "LAIGS", "WAIGS", "PAIGS",
    "LAIGD", "WAIGD", "PAIGD", "LBIGS", "WBIGS", "PBIGS", "LBIGD", "WBIGD", "PBIGD", "LCIGS", "WCIGS", "PCIGS", "LCIGD", "WCIGD", "PCIGD", "LDIGS",
    "WDIGS", "PDIGS", "LDIGD", "WDIGD", "PDIGD", "LNTOX", "WNTOX", "PNTOX", "LPOXEDGE", "WPOXEDGE", "PPOXEDGE", "LLOVS", "WLOVS", "PLOVS", "LLOVD", "WLOVD",
    "PLOVD", "LCFS", "WCFS", "PCFS", "LCFD", "WCFD", "PCFD", "LVSAT", "WVSAT", "PVSAT", "LVSATB", "WVSATB", "PVSATB", "LVSAT1", "WVSAT1", "PVSAT1",
    "LVSATCV", "WVSATCV", "PVSATCV", "LKSATIV", "WKSATIV", "PKSATIV", "LKSUBIV", "WKSUBIV", "PKSUBIV", "LKSATIVB", "WKSATIVB", "PKSATIVB", "LUP", "WUP", "PUP", "LUP2",
    "WUP2", "PUP2", "LAIGBINV", "WAIGBINV", "PAIGBINV", "LBIGBINV", "WBIGBINV", "PBIGBINV", "LCIGBINV", "WCIGBINV", "PCIGBINV", "LEIGBINV", "WEIGBINV", "PEIGBINV", "LNIGBINV", "WNIGBINV",
    "PNIGBINV", "LAIGBACC", "WAIGBACC", "PAIGBACC", "LBIGBACC", "WBIGBACC", "PBIGBACC", "LCIGBACC", "WCIGBACC", "PCIGBACC", "LNIGBACC", "WNIGBACC", "PNIGBACC", "LXRCRG1", "WXRCRG1", "PXRCRG1",
    "LXRCRG2", "WXRCRG2", "PXRCRG2", "LQMTCENCV", "WQMTCENCV", "PQMTCENCV", "LETAQM", "WETAQM", "PETAQM", "LQM0", "WQM0", "PQM0", "LPQM", "WPQM", "PPQM", "LNOIA2",
    "WNOIA2", "PNOIA2", "LMPOWER", "WMPOWER", "PMPOWER", "LQSREF", "WQSREF", "PQSREF",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 760] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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

const PARAMETER_INTEGER_FLAGS: [bool; 760] = [
    false, false, true, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 760] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }),
    None, Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 2e25, label: "2e25" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.01, label: "0.01" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.01, label: "0.01" }), None, None, None, None, None,
    None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.02, label: "0.02" }), Some(ParameterBound { value: 0.02, label: "0.02" }), None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: -273.15, label: "-273.15" }), None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 760] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 5e24, label: "5e24" }), Some(ParameterBound { value: 1e27, label: "1e27" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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

const PARAMETER_RANGE_FLAGS: [u8; 760] = [
    2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 2, 2, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2,
    0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 1, 3,
    3, 3, 3, 3, 3, 3, 3, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 3, 0, 2, 0, 0, 0,
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
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 760] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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
        314 => Some(ParameterBound { value: (params[0] + params[23]), label: "computed upper-bound expression" }),
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

pub(crate) type CanonicalModelValues = [f64; 123];
pub struct Instance {
    pub nodes: [usize; 9],
    pub branches: [usize; 5],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 760]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<8, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) canonical_reactive: Box<[f64; 71]>,
    pub(crate) canonical_model_values: Option<std::sync::Arc<CanonicalModelValues>>,
    pub(crate) canonical_staged: Box<[f64; 359]>,
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
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 9;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["di", "si", "ge", "gi"];

    pub const BRANCH_COUNT: usize = 5;
    pub const PARAMETER_COUNT: usize = 760;
    pub const VARIABLE_COUNT: usize = 676;
    pub const DDT_STATE_COUNT: usize = 8;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "84ea2750b3d868fe33ee9868b06e7aca5584d41b9b025bd42350a19b9e7a9ba8";
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
            canonical_model_values: None,
            canonical_staged: canonical_boxed_zero_f64(),
            canonical_instance_valid: false,
            canonical_temperature_valid: false,
            canonical_temperature: 0.0,
            canonical_thermal_voltage: 0.0,
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(40);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(8);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 40);
        debug_assert_eq!(state.flags.len(), 8);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimimg'", name));
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
