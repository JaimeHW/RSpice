#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 1918],
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
            const DEFAULTS_0: [f64; 20] = [
                3e-8, 3e-8, 4e-8, 1.5e-8, 8e-8, 1.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 20);
            {
                let params = &mut *ptr;
                params[20] = params[0];
                validate_parameter("lrsd", params[20], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 13] = [
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(21), 13);
            {
                let params = &mut *ptr;
                params[34] = params[28];
                validate_finite_parameter("covd", params[34]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[35] = params[29];
                validate_finite_parameter("lcovd", params[35]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[36] = params[30];
                validate_finite_parameter("ncovd", params[36]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[37] = params[31];
                validate_finite_parameter("pcovd", params[37]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[38] = params[32];
                validate_finite_parameter("wcovd", params[38]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[39] = params[33];
                validate_finite_parameter("p2covd", params[39]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 6] = [
                5e-9, 2e-9, 5e-9, 6e-9, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(40), 6);
            {
                let params = &mut *ptr;
                params[46] = params[44];
                validate_parameter("dws2", params[46], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[47] = params[45];
                validate_parameter("dach2", params[47], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[48] = params[44];
                validate_parameter("dws3", params[48], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[49] = params[45];
                validate_parameter("dach3", params[49], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[50] = params[44];
                validate_parameter("dws4", params[50], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[51] = params[45];
                validate_parameter("dach4", params[51], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[52] = params[44];
                validate_parameter("dws5", params[52], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[53] = params[45];
                validate_parameter("dach5", params[53], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[54] = params[44];
                validate_parameter("dws6", params[54], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[55] = params[45];
                validate_parameter("dach6", params[55], false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 95] = [
                1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 1e-9, 1.2e-9, 1.4e-7, 3e-8, 0.0, 0.0, 0.0,
                100000.0, 2e26, 0.0, 0.0, 0.0, 100000.0, 3.9, 11.9,
                4.05, 1.1e16, 1.12, 2.86e25, 1e-15, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e22, 0.0, 0.0,
                0.0, 4.61, 0.0, 0.0, 0.0, 0.0, 0.0, -0.2,
                -0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(56), 95);
            {
                let params = &mut *ptr;
                params[151] = 0.001;
                validate_parameter("minr", params[151], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 4] = [
                0.0, 100000.0, 0.0, 100000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(152), 4);
            {
                let params = &mut *ptr;
                params[156] = params[154];
                validate_finite_parameter("cdscdrn1", params[156]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[157] = params[155];
                validate_finite_parameter("cdscdrn2", params[157]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 3] = [
                0.0, 100000.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(158), 3);
            {
                let params = &mut *ptr;
                params[161] = params[158];
                validate_finite_parameter("eta0n1cv", params[161]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[162] = params[159];
                validate_parameter("eta0n2cv", params[162], false, Some((1e-5, "1e-5")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[163] = params[160];
                validate_finite_parameter("eta0ltcv", params[163]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (*ptr).values.as_mut_ptr().add(164), 1);
            {
                let params = &mut *ptr;
                params[165] = params[164];
                validate_finite_parameter("teta0cv", params[165]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[166] = params[164];
                validate_finite_parameter("teta0r", params[166]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 27] = [
                0.0, 1e-7, 0.0, 1e-7, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (*ptr).values.as_mut_ptr().add(167), 27);
            {
                let params = &mut *ptr;
                params[194] = params[188];
                validate_finite_parameter("citr", params[194]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[195] = params[189];
                validate_finite_parameter("lcitr", params[195]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[196] = params[190];
                validate_finite_parameter("ncitr", params[196]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[197] = params[191];
                validate_finite_parameter("pcitr", params[197]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[198] = params[192];
                validate_finite_parameter("wcitr", params[198]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[199] = params[193];
                validate_finite_parameter("p2citr", params[199]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 12] = [
                0.007, 0.0, 0.0, 0.0, 0.0, 0.0, 0.007, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (*ptr).values.as_mut_ptr().add(200), 12);
            {
                let params = &mut *ptr;
                params[212] = params[206];
                validate_finite_parameter("cdscdr", params[212]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[213] = params[207];
                validate_finite_parameter("lcdscdr", params[213]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[214] = params[208];
                validate_finite_parameter("ncdscdr", params[214]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[215] = params[209];
                validate_finite_parameter("pcdscdr", params[215]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[216] = params[210];
                validate_finite_parameter("wcdscdr", params[216]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[217] = params[211];
                validate_finite_parameter("p2cdscdr", params[217]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (*ptr).values.as_mut_ptr().add(218), 12);
            {
                let params = &mut *ptr;
                params[230] = params[224];
                validate_finite_parameter("dvt1ss", params[230]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[231] = params[225];
                validate_finite_parameter("ldvt1ss", params[231]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[232] = params[226];
                validate_finite_parameter("ndvt1ss", params[232]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[233] = params[227];
                validate_finite_parameter("pdvt1ss", params[233]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[234] = params[228];
                validate_finite_parameter("wdvt1ss", params[234]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[235] = params[229];
                validate_finite_parameter("p2dvt1ss", params[235]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 18] = [
                0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (*ptr).values.as_mut_ptr().add(236), 18);
            {
                let params = &mut *ptr;
                params[254] = params[242];
                validate_finite_parameter("eta0r", params[254]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[255] = params[243];
                validate_finite_parameter("leta0r", params[255]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[256] = params[244];
                validate_finite_parameter("neta0r", params[256]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[257] = params[245];
                validate_finite_parameter("peta0r", params[257]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[258] = params[246];
                validate_finite_parameter("weta0r", params[258]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[259] = params[247];
                validate_finite_parameter("p2eta0r", params[259]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[260] = params[242];
                validate_finite_parameter("eta0cv", params[260]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[261] = params[243];
                validate_finite_parameter("leta0cv", params[261]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[262] = params[244];
                validate_finite_parameter("neta0cv", params[262]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[263] = params[245];
                validate_finite_parameter("peta0cv", params[263]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[264] = params[246];
                validate_finite_parameter("weta0cv", params[264]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[265] = params[247];
                validate_finite_parameter("p2eta0cv", params[265]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 24] = [
                1.06, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 5e-9, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (*ptr).values.as_mut_ptr().add(266), 24);
            {
                let params = &mut *ptr;
                params[290] = params[284];
                validate_finite_parameter("dvtshiftr", params[290]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[291] = params[285];
                validate_finite_parameter("ldvtshiftr", params[291]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[292] = params[286];
                validate_finite_parameter("ndvtshiftr", params[292]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[293] = params[287];
                validate_finite_parameter("pdvtshiftr", params[293]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[294] = params[288];
                validate_finite_parameter("wdvtshiftr", params[294]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[295] = params[289];
                validate_finite_parameter("p2dvtshiftr", params[295]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (*ptr).values.as_mut_ptr().add(296), 24);
            {
                let params = &mut *ptr;
                params[320] = params[308];
                validate_finite_parameter("k2si", params[320]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[321] = params[309];
                validate_finite_parameter("lk2si", params[321]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[322] = params[310];
                validate_finite_parameter("nk2si", params[322]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[323] = params[311];
                validate_finite_parameter("pk2si", params[323]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[324] = params[312];
                validate_finite_parameter("wk2si", params[324]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[325] = params[313];
                validate_finite_parameter("p2k2si", params[325]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[326] = params[314];
                validate_finite_parameter("k2si1", params[326]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[327] = params[315];
                validate_finite_parameter("lk2si1", params[327]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[328] = params[316];
                validate_finite_parameter("nk2si1", params[328]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[329] = params[317];
                validate_finite_parameter("pk2si1", params[329]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[330] = params[318];
                validate_finite_parameter("wk2si1", params[330]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[331] = params[319];
                validate_finite_parameter("p2k2si1", params[331]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (*ptr).values.as_mut_ptr().add(332), 12);
            {
                let params = &mut *ptr;
                params[344] = params[332];
                validate_finite_parameter("k2sisat", params[344]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[345] = params[333];
                validate_finite_parameter("lk2sisat", params[345]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[346] = params[334];
                validate_finite_parameter("nk2sisat", params[346]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[347] = params[335];
                validate_finite_parameter("pk2sisat", params[347]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[348] = params[336];
                validate_finite_parameter("wk2sisat", params[348]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[349] = params[337];
                validate_finite_parameter("p2k2sisat", params[349]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[350] = params[338];
                validate_finite_parameter("k2sisat1", params[350]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[351] = params[339];
                validate_finite_parameter("lk2sisat1", params[351]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[352] = params[340];
                validate_finite_parameter("nk2sisat1", params[352]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[353] = params[341];
                validate_finite_parameter("pk2sisat1", params[353]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[354] = params[342];
                validate_finite_parameter("wk2sisat1", params[354]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[355] = params[343];
                validate_finite_parameter("p2k2sisat1", params[355]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 76] = [
                0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-6, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.2e-8, 0.001, 0.001, 0.66, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 100000.0, 0.0, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (*ptr).values.as_mut_ptr().add(356), 76);
            {
                let params = &mut *ptr;
                params[432] = params[428];
                validate_finite_parameter("vsat1n1", params[432]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[433] = params[429];
                validate_finite_parameter("vsat1n2", params[433]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[434] = params[432];
                validate_finite_parameter("vsat1rn1", params[434]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[435] = params[433];
                validate_finite_parameter("vsat1rn2", params[435]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[436] = params[430];
                validate_finite_parameter("avsat1", params[436]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[437] = params[431];
                validate_finite_parameter("bvsat1", params[437]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (*ptr).values.as_mut_ptr().add(438), 2);
            {
                let params = &mut *ptr;
                params[440] = params[430];
                validate_finite_parameter("avsatcv", params[440]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[441] = params[431];
                validate_finite_parameter("bvsatcv", params[441]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[442] = params[438];
                validate_finite_parameter("apsatcv", params[442]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[443] = params[439];
                validate_finite_parameter("bpsatcv", params[443]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (*ptr).values.as_mut_ptr().add(444), 2);
            {
                let params = &mut *ptr;
                params[446] = params[444];
                validate_finite_parameter("amexpr", params[446]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[447] = params[445];
                validate_finite_parameter("bmexpr", params[447]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 4] = [
                0.0, 1e-7, 0.0, -4e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (*ptr).values.as_mut_ptr().add(448), 4);
            {
                let params = &mut *ptr;
                params[452] = params[450];
                validate_finite_parameter("tmexpr", params[452]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 8] = [
                0.01, 85000.0, 85000.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (*ptr).values.as_mut_ptr().add(453), 8);
            {
                let params = &mut *ptr;
                params[461] = params[455];
                validate_finite_parameter("vsatr", params[461]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[462] = params[456];
                validate_finite_parameter("lvsatr", params[462]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[463] = params[457];
                validate_finite_parameter("nvsatr", params[463]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[464] = params[458];
                validate_finite_parameter("pvsatr", params[464]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[465] = params[459];
                validate_finite_parameter("wvsatr", params[465]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[466] = params[460];
                validate_finite_parameter("p2vsatr", params[466]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[467] = params[455];
                validate_finite_parameter("vsat1", params[467]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[468] = params[456];
                validate_finite_parameter("lvsat1", params[468]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[469] = params[457];
                validate_finite_parameter("nvsat1", params[469]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[470] = params[458];
                validate_finite_parameter("pvsat1", params[470]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[471] = params[459];
                validate_finite_parameter("wvsat1", params[471]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[472] = params[460];
                validate_finite_parameter("p2vsat1", params[472]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[473] = params[467];
                validate_finite_parameter("vsat1r", params[473]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[474] = params[468];
                validate_finite_parameter("lvsat1r", params[474]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[475] = params[469];
                validate_finite_parameter("nvsat1r", params[475]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[476] = params[470];
                validate_finite_parameter("pvsat1r", params[476]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[477] = params[471];
                validate_finite_parameter("wvsat1r", params[477]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[478] = params[472];
                validate_finite_parameter("p2vsat1r", params[478]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 21] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -0.0002, -2e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (*ptr).values.as_mut_ptr().add(479), 21);
            {
                let params = &mut *ptr;
                params[500] = params[492];
                validate_finite_parameter("ksativr", params[500]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[501] = params[493];
                validate_finite_parameter("lksativr", params[501]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[502] = params[494];
                validate_finite_parameter("nksativr", params[502]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[503] = params[495];
                validate_finite_parameter("pksativr", params[503]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[504] = params[496];
                validate_finite_parameter("wksativr", params[504]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[505] = params[497];
                validate_finite_parameter("p2ksativr", params[505]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[506] = params[455];
                validate_finite_parameter("vsatcv", params[506]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[507] = params[456];
                validate_finite_parameter("lvsatcv", params[507]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[508] = params[457];
                validate_finite_parameter("nvsatcv", params[508]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[509] = params[458];
                validate_finite_parameter("pvsatcv", params[509]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[510] = params[459];
                validate_finite_parameter("wvsatcv", params[510]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[511] = params[460];
                validate_finite_parameter("p2vsatcv", params[511]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 6] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (*ptr).values.as_mut_ptr().add(512), 6);
            {
                let params = &mut *ptr;
                params[518] = params[479];
                validate_finite_parameter("deltavsatcv", params[518]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[519] = params[480];
                validate_finite_parameter("ldeltavsatcv", params[519]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[520] = params[481];
                validate_finite_parameter("ndeltavsatcv", params[520]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[521] = params[482];
                validate_finite_parameter("pdeltavsatcv", params[521]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[522] = params[483];
                validate_finite_parameter("wdeltavsatcv", params[522]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[523] = params[484];
                validate_finite_parameter("p2deltavsatcv", params[523]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[524] = params[485];
                validate_finite_parameter("psatcv", params[524]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[525] = params[486];
                validate_finite_parameter("lpsatcv", params[525]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[526] = params[487];
                validate_finite_parameter("npsatcv", params[526]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[527] = params[488];
                validate_finite_parameter("ppsatcv", params[527]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[528] = params[489];
                validate_finite_parameter("wpsatcv", params[528]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[529] = params[490];
                validate_finite_parameter("p2psatcv", params[529]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 7] = [
                4.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (*ptr).values.as_mut_ptr().add(530), 7);
            {
                let params = &mut *ptr;
                params[537] = params[531];
                validate_finite_parameter("mexpr", params[537]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[538] = params[532];
                validate_finite_parameter("lmexpr", params[538]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[539] = params[533];
                validate_finite_parameter("nmexpr", params[539]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[540] = params[534];
                validate_finite_parameter("pmexpr", params[540]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[541] = params[535];
                validate_finite_parameter("wmexpr", params[541]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[542] = params[536];
                validate_finite_parameter("p2mexpr", params[542]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (*ptr).values.as_mut_ptr().add(543), 6);
            {
                let params = &mut *ptr;
                params[549] = params[543];
                validate_finite_parameter("ptwgr", params[549]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[550] = params[544];
                validate_finite_parameter("lptwgr", params[550]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[551] = params[545];
                validate_finite_parameter("nptwgr", params[551]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[552] = params[546];
                validate_finite_parameter("pptwgr", params[552]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[553] = params[547];
                validate_finite_parameter("wptwgr", params[553]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[554] = params[548];
                validate_finite_parameter("p2ptwgr", params[554]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 7] = [
                -0.00156, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (*ptr).values.as_mut_ptr().add(555), 7);
            {
                let params = &mut *ptr;
                params[562] = params[555];
                validate_finite_parameter("atr", params[562]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[563] = params[556];
                validate_finite_parameter("latr", params[563]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[564] = params[557];
                validate_finite_parameter("natr", params[564]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[565] = params[558];
                validate_finite_parameter("patr", params[565]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[566] = params[559];
                validate_finite_parameter("watr", params[566]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[567] = params[560];
                validate_finite_parameter("p2atr", params[567]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[568] = params[555];
                validate_finite_parameter("atcv", params[568]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[569] = params[556];
                validate_finite_parameter("latcv", params[569]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[570] = params[557];
                validate_finite_parameter("natcv", params[570]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[571] = params[558];
                validate_finite_parameter("patcv", params[571]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[572] = params[559];
                validate_finite_parameter("watcv", params[572]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[573] = params[560];
                validate_finite_parameter("p2atcv", params[573]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[574] = params[561];
                validate_finite_parameter("at2cv", params[574]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 7] = [
                0.004, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (*ptr).values.as_mut_ptr().add(575), 7);
            {
                let params = &mut *ptr;
                params[582] = params[581];
                validate_finite_parameter("u0n1cv", params[582]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[583] = params[581];
                validate_finite_parameter("u0n1r", params[583]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                100000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (*ptr).values.as_mut_ptr().add(584), 1);
            {
                let params = &mut *ptr;
                params[585] = params[584];
                validate_finite_parameter("u0n2cv", params[585]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[586] = params[584];
                validate_finite_parameter("u0n2r", params[586]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 3] = [
                0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (*ptr).values.as_mut_ptr().add(587), 3);
            {
                let params = &mut *ptr;
                params[590] = params[589];
                validate_finite_parameter("lpar", params[590]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (*ptr).values.as_mut_ptr().add(591), 1);
            {
                let params = &mut *ptr;
                params[592] = params[591];
                validate_finite_parameter("auar", params[592]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 1] = [
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (*ptr).values.as_mut_ptr().add(593), 1);
            {
                let params = &mut *ptr;
                params[594] = params[593];
                validate_finite_parameter("buar", params[594]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (*ptr).values.as_mut_ptr().add(595), 1);
            {
                let params = &mut *ptr;
                params[596] = params[595];
                validate_finite_parameter("aeur", params[596]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 1] = [
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (*ptr).values.as_mut_ptr().add(597), 1);
            {
                let params = &mut *ptr;
                params[598] = params[597];
                validate_finite_parameter("beur", params[598]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (*ptr).values.as_mut_ptr().add(599), 1);
            {
                let params = &mut *ptr;
                params[600] = params[599];
                validate_finite_parameter("audr", params[600]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                5e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (*ptr).values.as_mut_ptr().add(601), 1);
            {
                let params = &mut *ptr;
                params[602] = params[601];
                validate_finite_parameter("budr", params[602]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 8] = [
                0.0, 0.01, 0.03, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (*ptr).values.as_mut_ptr().add(603), 8);
            {
                let params = &mut *ptr;
                params[611] = params[605];
                validate_finite_parameter("u0r", params[611]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[612] = params[606];
                validate_finite_parameter("lu0r", params[612]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[613] = params[607];
                validate_finite_parameter("nu0r", params[613]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[614] = params[608];
                validate_finite_parameter("pu0r", params[614]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[615] = params[609];
                validate_finite_parameter("wu0r", params[615]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[616] = params[610];
                validate_finite_parameter("p2u0r", params[616]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[617] = params[605];
                validate_finite_parameter("u0cv", params[617]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[618] = params[606];
                validate_finite_parameter("lu0cv", params[618]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[619] = params[607];
                validate_finite_parameter("nu0cv", params[619]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[620] = params[608];
                validate_finite_parameter("pu0cv", params[620]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[621] = params[609];
                validate_finite_parameter("wu0cv", params[621]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[622] = params[610];
                validate_finite_parameter("p2u0cv", params[622]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 12] = [
                2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (*ptr).values.as_mut_ptr().add(623), 12);
            {
                let params = &mut *ptr;
                params[635] = params[629];
                validate_finite_parameter("upr", params[635]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[636] = params[630];
                validate_finite_parameter("lupr", params[636]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[637] = params[631];
                validate_finite_parameter("nupr", params[637]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[638] = params[632];
                validate_finite_parameter("pupr", params[638]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[639] = params[633];
                validate_finite_parameter("wupr", params[639]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[640] = params[634];
                validate_finite_parameter("p2upr", params[640]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 6] = [
                0.3, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (*ptr).values.as_mut_ptr().add(641), 6);
            {
                let params = &mut *ptr;
                params[647] = params[641];
                validate_finite_parameter("uar", params[647]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[648] = params[642];
                validate_finite_parameter("luar", params[648]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[649] = params[643];
                validate_finite_parameter("nuar", params[649]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[650] = params[644];
                validate_finite_parameter("puar", params[650]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[651] = params[645];
                validate_finite_parameter("wuar", params[651]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[652] = params[646];
                validate_finite_parameter("p2uar", params[652]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[653] = params[641];
                validate_finite_parameter("uacv", params[653]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[654] = params[642];
                validate_finite_parameter("luacv", params[654]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[655] = params[643];
                validate_finite_parameter("nuacv", params[655]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[656] = params[644];
                validate_finite_parameter("puacv", params[656]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[657] = params[645];
                validate_finite_parameter("wuacv", params[657]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[658] = params[646];
                validate_finite_parameter("p2uacv", params[658]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (*ptr).values.as_mut_ptr().add(659), 6);
            {
                let params = &mut *ptr;
                params[665] = params[659];
                validate_finite_parameter("ucr", params[665]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[666] = params[660];
                validate_finite_parameter("lucr", params[666]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[667] = params[661];
                validate_finite_parameter("nucr", params[667]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[668] = params[662];
                validate_finite_parameter("pucr", params[668]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[669] = params[663];
                validate_finite_parameter("wucr", params[669]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[670] = params[664];
                validate_finite_parameter("p2ucr", params[670]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[671] = params[659];
                validate_finite_parameter("uccv", params[671]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[672] = params[660];
                validate_finite_parameter("luccv", params[672]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[673] = params[661];
                validate_finite_parameter("nuccv", params[673]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[674] = params[662];
                validate_finite_parameter("puccv", params[674]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[675] = params[663];
                validate_finite_parameter("wuccv", params[675]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[676] = params[664];
                validate_finite_parameter("p2uccv", params[676]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 6] = [
                2.5, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (*ptr).values.as_mut_ptr().add(677), 6);
            {
                let params = &mut *ptr;
                params[683] = params[677];
                validate_finite_parameter("eur", params[683]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[684] = params[678];
                validate_finite_parameter("leur", params[684]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[685] = params[679];
                validate_finite_parameter("neur", params[685]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[686] = params[680];
                validate_finite_parameter("peur", params[686]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[687] = params[681];
                validate_finite_parameter("weur", params[687]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[688] = params[682];
                validate_finite_parameter("p2eur", params[688]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (*ptr).values.as_mut_ptr().add(689), 6);
            {
                let params = &mut *ptr;
                params[695] = params[689];
                validate_finite_parameter("udr", params[695]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[696] = params[690];
                validate_finite_parameter("ludr", params[696]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[697] = params[691];
                validate_finite_parameter("nudr", params[697]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[698] = params[692];
                validate_finite_parameter("pudr", params[698]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[699] = params[693];
                validate_finite_parameter("wudr", params[699]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[700] = params[694];
                validate_finite_parameter("p2udr", params[700]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[701] = params[689];
                validate_finite_parameter("udcv", params[701]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[702] = params[690];
                validate_finite_parameter("ludcv", params[702]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[703] = params[691];
                validate_finite_parameter("nudcv", params[703]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[704] = params[692];
                validate_finite_parameter("pudcv", params[704]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[705] = params[693];
                validate_finite_parameter("wudcv", params[705]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[706] = params[694];
                validate_finite_parameter("p2udcv", params[706]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 36] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-5, 0.0,
                0.0, 0.0, 0.0, 0.0, -10.0, 0.0, 0.0, 0.0,
                0.0, 0.0, -2e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
                -10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (*ptr).values.as_mut_ptr().add(707), 36);
            {
                let params = &mut *ptr;
                params[743] = params[737];
                validate_finite_parameter("uter", params[743]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[744] = params[738];
                validate_finite_parameter("luter", params[744]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[745] = params[739];
                validate_finite_parameter("nuter", params[745]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[746] = params[740];
                validate_finite_parameter("puter", params[746]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[747] = params[741];
                validate_finite_parameter("wuter", params[747]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[748] = params[742];
                validate_finite_parameter("p2uter", params[748]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[749] = params[737];
                validate_finite_parameter("utecv", params[749]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[750] = params[738];
                validate_finite_parameter("lutecv", params[750]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[751] = params[739];
                validate_finite_parameter("nutecv", params[751]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[752] = params[740];
                validate_finite_parameter("putecv", params[752]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[753] = params[741];
                validate_finite_parameter("wutecv", params[753]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[754] = params[742];
                validate_finite_parameter("p2utecv", params[754]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 6] = [
                -0.4, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (*ptr).values.as_mut_ptr().add(755), 6);
            {
                let params = &mut *ptr;
                params[761] = params[755];
                validate_finite_parameter("ute1cv", params[761]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[762] = params[756];
                validate_finite_parameter("lute1cv", params[762]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[763] = params[757];
                validate_finite_parameter("nute1cv", params[763]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[764] = params[758];
                validate_finite_parameter("pute1cv", params[764]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[765] = params[759];
                validate_finite_parameter("wute1cv", params[765]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[766] = params[760];
                validate_finite_parameter("p2ute1cv", params[766]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 6] = [
                -0.0015, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (*ptr).values.as_mut_ptr().add(767), 6);
            {
                let params = &mut *ptr;
                params[773] = params[767];
                validate_finite_parameter("utlr", params[773]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[774] = params[768];
                validate_finite_parameter("lutlr", params[774]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[775] = params[769];
                validate_finite_parameter("nutlr", params[775]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[776] = params[770];
                validate_finite_parameter("putlr", params[776]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[777] = params[771];
                validate_finite_parameter("wutlr", params[777]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[778] = params[772];
                validate_finite_parameter("p2utlr", params[778]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[779] = params[767];
                validate_finite_parameter("utlcv", params[779]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[780] = params[768];
                validate_finite_parameter("lutlcv", params[780]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[781] = params[769];
                validate_finite_parameter("nutlcv", params[781]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[782] = params[770];
                validate_finite_parameter("putlcv", params[782]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[783] = params[771];
                validate_finite_parameter("wutlcv", params[783]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[784] = params[772];
                validate_finite_parameter("p2utlcv", params[784]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.001032, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (*ptr).values.as_mut_ptr().add(785), 12);
            {
                let params = &mut *ptr;
                params[797] = params[791];
                validate_finite_parameter("ua1r", params[797]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[798] = params[792];
                validate_finite_parameter("lua1r", params[798]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[799] = params[793];
                validate_finite_parameter("nua1r", params[799]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[800] = params[794];
                validate_finite_parameter("pua1r", params[800]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[801] = params[795];
                validate_finite_parameter("wua1r", params[801]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[802] = params[796];
                validate_finite_parameter("p2ua1r", params[802]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[803] = params[791];
                validate_finite_parameter("ua1cv", params[803]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[804] = params[792];
                validate_finite_parameter("lua1cv", params[804]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[805] = params[793];
                validate_finite_parameter("nua1cv", params[805]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[806] = params[794];
                validate_finite_parameter("pua1cv", params[806]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[807] = params[795];
                validate_finite_parameter("wua1cv", params[807]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[808] = params[796];
                validate_finite_parameter("p2ua1cv", params[808]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 6] = [
                -0.04, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (*ptr).values.as_mut_ptr().add(809), 6);
            {
                let params = &mut *ptr;
                params[815] = params[809];
                validate_finite_parameter("ua2cv", params[815]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[816] = params[810];
                validate_finite_parameter("lua2cv", params[816]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[817] = params[811];
                validate_finite_parameter("nua2cv", params[817]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[818] = params[812];
                validate_finite_parameter("pua2cv", params[818]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[819] = params[813];
                validate_finite_parameter("wua2cv", params[819]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[820] = params[814];
                validate_finite_parameter("p2ua2cv", params[820]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 12] = [
                -0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 5.6e-11, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (*ptr).values.as_mut_ptr().add(821), 12);
            {
                let params = &mut *ptr;
                params[833] = params[827];
                validate_finite_parameter("uc1r", params[833]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[834] = params[828];
                validate_finite_parameter("luc1r", params[834]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[835] = params[829];
                validate_finite_parameter("nuc1r", params[835]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[836] = params[830];
                validate_finite_parameter("puc1r", params[836]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[837] = params[831];
                validate_finite_parameter("wuc1r", params[837]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[838] = params[832];
                validate_finite_parameter("p2uc1r", params[838]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[839] = params[827];
                validate_finite_parameter("uc1cv", params[839]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[840] = params[828];
                validate_finite_parameter("luc1cv", params[840]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[841] = params[829];
                validate_finite_parameter("nuc1cv", params[841]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[842] = params[830];
                validate_finite_parameter("puc1cv", params[842]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[843] = params[831];
                validate_finite_parameter("wuc1cv", params[843]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[844] = params[832];
                validate_finite_parameter("p2uc1cv", params[844]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (*ptr).values.as_mut_ptr().add(845), 6);
            {
                let params = &mut *ptr;
                params[851] = params[845];
                validate_finite_parameter("ud1r", params[851]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[852] = params[846];
                validate_finite_parameter("lud1r", params[852]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[853] = params[847];
                validate_finite_parameter("nud1r", params[853]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[854] = params[848];
                validate_finite_parameter("pud1r", params[854]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[855] = params[849];
                validate_finite_parameter("wud1r", params[855]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[856] = params[850];
                validate_finite_parameter("p2ud1r", params[856]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[857] = params[845];
                validate_finite_parameter("ud1cv", params[857]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[858] = params[846];
                validate_finite_parameter("lud1cv", params[858]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[859] = params[847];
                validate_finite_parameter("nud1cv", params[859]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[860] = params[848];
                validate_finite_parameter("pud1cv", params[860]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[861] = params[849];
                validate_finite_parameter("wud1cv", params[861]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[862] = params[850];
                validate_finite_parameter("p2ud1cv", params[862]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 6] = [
                -0.04, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (*ptr).values.as_mut_ptr().add(863), 6);
            {
                let params = &mut *ptr;
                params[869] = params[863];
                validate_finite_parameter("ud2cv", params[869]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[870] = params[864];
                validate_finite_parameter("lud2cv", params[870]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[871] = params[865];
                validate_finite_parameter("nud2cv", params[871]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[872] = params[866];
                validate_finite_parameter("pud2cv", params[872]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[873] = params[867];
                validate_finite_parameter("wud2cv", params[873]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[874] = params[868];
                validate_finite_parameter("p2ud2cv", params[874]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 14] = [
                -0.004775, 0.0, 0.0, 0.0, 0.0, 0.0, -0.04, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (*ptr).values.as_mut_ptr().add(875), 14);
            {
                let params = &mut *ptr;
                params[889] = params[623];
                validate_finite_parameter("etamobthin", params[889]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 2] = [
                7.5e-9, 0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (*ptr).values.as_mut_ptr().add(890), 2);
            {
                let params = &mut *ptr;
                params[892] = params[641];
                validate_finite_parameter("uathin", params[892]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 4] = [
                9e-9, 0.09, 6.4e-9, 0.2,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (*ptr).values.as_mut_ptr().add(893), 4);
            {
                let params = &mut *ptr;
                params[897] = params[677];
                validate_finite_parameter("euthin", params[897]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 3] = [
                3.5, 6e-9, 0.2,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (*ptr).values.as_mut_ptr().add(898), 3);
            {
                let params = &mut *ptr;
                params[901] = params[689];
                validate_finite_parameter("udthin", params[901]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 16] = [
                8.1e-9, 1.3, 1.5, 1.1, 26.6, 4.0, 0.0, 0.0,
                1e-7, 0.0, 0.0, 1e-7, 0.0, 0.0, 1e-7, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (*ptr).values.as_mut_ptr().add(902), 16);
            {
                let params = &mut *ptr;
                params[918] = params[917];
                validate_parameter("rsdrr", params[918], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[919] = params[917];
                validate_parameter("rddr", params[919], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[920] = params[919];
                validate_parameter("rddrr", params[920], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (*ptr).values.as_mut_ptr().add(921), 1);
            {
                let params = &mut *ptr;
                params[922] = params[921];
                validate_finite_parameter("prddr", params[922]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (*ptr).values.as_mut_ptr().add(923), 1);
            {
                let params = &mut *ptr;
                params[924] = params[923];
                validate_finite_parameter("trddr", params[924]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 24] = [
                100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 50.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 50.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (*ptr).values.as_mut_ptr().add(925), 24);
            {
                let params = &mut *ptr;
                params[949] = params[943];
                validate_finite_parameter("prwgd", params[949]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 47] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.001, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0004, 0.0, 0.0, 0.0, 0.0, 0.0, 170.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.01, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.3, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0002, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (*ptr).values.as_mut_ptr().add(950), 47);
            {
                let params = &mut *ptr;
                params[997] = params[985];
                validate_finite_parameter("pdibl1r", params[997]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[998] = params[986];
                validate_finite_parameter("lpdibl1r", params[998]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[999] = params[987];
                validate_finite_parameter("npdibl1r", params[999]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1000] = params[988];
                validate_finite_parameter("ppdibl1r", params[1000]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1001] = params[989];
                validate_finite_parameter("wpdibl1r", params[1001]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1002] = params[990];
                validate_finite_parameter("p2pdibl1r", params[1002]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1003] = params[991];
                validate_finite_parameter("pdibl2r", params[1003]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1004] = params[992];
                validate_finite_parameter("lpdibl2r", params[1004]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1005] = params[993];
                validate_finite_parameter("npdibl2r", params[1005]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1006] = params[994];
                validate_finite_parameter("ppdibl2r", params[1006]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1007] = params[995];
                validate_finite_parameter("wpdibl2r", params[1007]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1008] = params[996];
                validate_finite_parameter("p2pdibl2r", params[1008]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 13] = [
                1.06, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (*ptr).values.as_mut_ptr().add(1009), 13);
            {
                let params = &mut *ptr;
                params[1022] = params[1021];
                validate_finite_parameter("apclmr", params[1022]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (*ptr).values.as_mut_ptr().add(1023), 1);
            {
                let params = &mut *ptr;
                params[1024] = params[1023];
                validate_finite_parameter("bpclmr", params[1024]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 7] = [
                0.013, -2e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (*ptr).values.as_mut_ptr().add(1025), 7);
            {
                let params = &mut *ptr;
                params[1032] = params[1025];
                validate_finite_parameter("pclmr", params[1032]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1033] = params[1027];
                validate_finite_parameter("lpclmr", params[1033]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1034] = params[1028];
                validate_finite_parameter("npclmr", params[1034]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1035] = params[1029];
                validate_finite_parameter("ppclmr", params[1035]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1036] = params[1030];
                validate_finite_parameter("wpclmr", params[1036]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1037] = params[1031];
                validate_finite_parameter("p2pclmr", params[1037]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (*ptr).values.as_mut_ptr().add(1038), 6);
            {
                let params = &mut *ptr;
                params[1044] = params[1025];
                validate_finite_parameter("pclmcv", params[1044]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1045] = params[1027];
                validate_finite_parameter("lpclmcv", params[1045]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1046] = params[1028];
                validate_finite_parameter("npclmcv", params[1046]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1047] = params[1029];
                validate_finite_parameter("ppclmcv", params[1047]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1048] = params[1030];
                validate_finite_parameter("wpclmcv", params[1048]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1049] = params[1031];
                validate_finite_parameter("p2pclmcv", params[1049]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 29] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.001, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (*ptr).values.as_mut_ptr().add(1050), 29);
            {
                let params = &mut *ptr;
                params[1079] = params[1078];
                validate_finite_parameter("rshd", params[1079]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 25] = [
                1e-8, 1e-8, 1e-12, 1.0, 0.5, 0.0, 0.0, 6e-9,
                3.9, 3e-8, 3e-8, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (*ptr).values.as_mut_ptr().add(1080), 25);
            {
                let params = &mut *ptr;
                params[1105] = params[1104];
                validate_finite_parameter("dlcigd", params[1105]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (*ptr).values.as_mut_ptr().add(1106), 1);
            {
                let params = &mut *ptr;
                params[1107] = params[1106];
                validate_finite_parameter("vfbsdcv", params[1107]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                1.2e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (*ptr).values.as_mut_ptr().add(1108), 1);
            {
                let params = &mut *ptr;
                params[1109] = params[90];
                validate_parameter("toxg", params[1109], false, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 129] = [
                0.001, 0.001, 0.0005, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0111, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.000949, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.006, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.1, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0136, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.00171, 0.0, 0.0, 0.0, 0.0, 0.0, 0.075,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0136, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00171,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.075, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0136, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.00171, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.075, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (*ptr).values.as_mut_ptr().add(1110), 129);
            {
                let params = &mut *ptr;
                params[1239] = params[1215];
                validate_finite_parameter("aigd", params[1239]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1240] = params[1216];
                validate_finite_parameter("laigd", params[1240]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1241] = params[1217];
                validate_finite_parameter("naigd", params[1241]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1242] = params[1218];
                validate_finite_parameter("paigd", params[1242]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1243] = params[1219];
                validate_finite_parameter("waigd", params[1243]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1244] = params[1220];
                validate_finite_parameter("p2aigd", params[1244]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1245] = params[1221];
                validate_finite_parameter("aigd1", params[1245]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1246] = params[1222];
                validate_finite_parameter("laigd1", params[1246]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1247] = params[1223];
                validate_finite_parameter("naigd1", params[1247]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1248] = params[1224];
                validate_finite_parameter("paigd1", params[1248]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1249] = params[1225];
                validate_finite_parameter("waigd1", params[1249]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1250] = params[1226];
                validate_finite_parameter("p2aigd1", params[1250]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1251] = params[1227];
                validate_finite_parameter("bigd", params[1251]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1252] = params[1228];
                validate_finite_parameter("lbigd", params[1252]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1253] = params[1229];
                validate_finite_parameter("nbigd", params[1253]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1254] = params[1230];
                validate_finite_parameter("pbigd", params[1254]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1255] = params[1231];
                validate_finite_parameter("wbigd", params[1255]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1256] = params[1232];
                validate_finite_parameter("p2bigd", params[1256]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1257] = params[1233];
                validate_finite_parameter("cigd", params[1257]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1258] = params[1234];
                validate_finite_parameter("lcigd", params[1258]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1259] = params[1235];
                validate_finite_parameter("ncigd", params[1259]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1260] = params[1236];
                validate_finite_parameter("pcigd", params[1260]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1261] = params[1237];
                validate_finite_parameter("wcigd", params[1261]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1262] = params[1238];
                validate_finite_parameter("p2cigd", params[1262]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 36] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 6.055e-12, 0.0,
                0.0, 0.0, 0.0, 0.0, 300000000.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (*ptr).values.as_mut_ptr().add(1263), 36);
            {
                let params = &mut *ptr;
                params[1299] = params[1269];
                validate_finite_parameter("agisl", params[1299]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1300] = params[1270];
                validate_finite_parameter("lagisl", params[1300]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1301] = params[1271];
                validate_finite_parameter("nagisl", params[1301]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1302] = params[1272];
                validate_finite_parameter("pagisl", params[1302]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1303] = params[1273];
                validate_finite_parameter("wagisl", params[1303]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1304] = params[1274];
                validate_finite_parameter("p2agisl", params[1304]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1305] = params[1275];
                validate_finite_parameter("bgisl", params[1305]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1306] = params[1276];
                validate_finite_parameter("lbgisl", params[1306]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1307] = params[1277];
                validate_finite_parameter("nbgisl", params[1307]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1308] = params[1278];
                validate_finite_parameter("pbgisl", params[1308]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1309] = params[1279];
                validate_finite_parameter("wbgisl", params[1309]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1310] = params[1280];
                validate_finite_parameter("p2bgisl", params[1310]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1311] = params[1281];
                validate_finite_parameter("cgisl", params[1311]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1312] = params[1282];
                validate_finite_parameter("lcgisl", params[1312]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1313] = params[1283];
                validate_finite_parameter("ncgisl", params[1313]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1314] = params[1284];
                validate_finite_parameter("pcgisl", params[1314]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1315] = params[1285];
                validate_finite_parameter("wcgisl", params[1315]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1316] = params[1286];
                validate_finite_parameter("p2cgisl", params[1316]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1317] = params[1287];
                validate_finite_parameter("egisl", params[1317]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1318] = params[1288];
                validate_finite_parameter("legisl", params[1318]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1319] = params[1289];
                validate_finite_parameter("negisl", params[1319]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1320] = params[1290];
                validate_finite_parameter("pegisl", params[1320]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1321] = params[1291];
                validate_finite_parameter("wegisl", params[1321]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1322] = params[1292];
                validate_finite_parameter("p2egisl", params[1322]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1323] = params[1293];
                validate_finite_parameter("pgisl", params[1323]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1324] = params[1294];
                validate_finite_parameter("lpgisl", params[1324]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1325] = params[1295];
                validate_finite_parameter("npgisl", params[1325]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1326] = params[1296];
                validate_finite_parameter("ppgisl", params[1326]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1327] = params[1297];
                validate_finite_parameter("wpgisl", params[1327]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1328] = params[1298];
                validate_finite_parameter("p2pgisl", params[1328]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 24] = [
                1e-27, 0.0, 0.0, 0.0, 0.0, 0.0, 6.3e-5, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.215, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.382, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (*ptr).values.as_mut_ptr().add(1329), 24);
            {
                let params = &mut *ptr;
                params[1353] = params[1329];
                validate_finite_parameter("atats", params[1353]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (*ptr).values.as_mut_ptr().add(1354), 5);
            {
                let params = &mut *ptr;
                params[1359] = params[1335];
                validate_finite_parameter("btats", params[1359]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (*ptr).values.as_mut_ptr().add(1360), 5);
            {
                let params = &mut *ptr;
                params[1365] = params[1341];
                validate_finite_parameter("ctats", params[1365]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (*ptr).values.as_mut_ptr().add(1366), 5);
            {
                let params = &mut *ptr;
                params[1371] = params[1347];
                validate_finite_parameter("dtats", params[1371]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 6.055e-12, 0.0, 0.0,
                0.0, 0.0, 0.0, 300000000.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (*ptr).values.as_mut_ptr().add(1372), 35);
            {
                let params = &mut *ptr;
                params[1407] = params[1377];
                validate_finite_parameter("agislb", params[1407]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1408] = params[1378];
                validate_finite_parameter("lagislb", params[1408]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1409] = params[1379];
                validate_finite_parameter("nagislb", params[1409]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1410] = params[1380];
                validate_finite_parameter("pagislb", params[1410]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1411] = params[1381];
                validate_finite_parameter("wagislb", params[1411]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1412] = params[1382];
                validate_finite_parameter("p2agislb", params[1412]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1413] = params[1383];
                validate_finite_parameter("bgislb", params[1413]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1414] = params[1384];
                validate_finite_parameter("lbgislb", params[1414]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1415] = params[1385];
                validate_finite_parameter("nbgislb", params[1415]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1416] = params[1386];
                validate_finite_parameter("pbgislb", params[1416]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1417] = params[1387];
                validate_finite_parameter("wbgislb", params[1417]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1418] = params[1388];
                validate_finite_parameter("p2bgislb", params[1418]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1419] = params[1389];
                validate_finite_parameter("cgislb", params[1419]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1420] = params[1390];
                validate_finite_parameter("lcgislb", params[1420]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1421] = params[1391];
                validate_finite_parameter("ncgislb", params[1421]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1422] = params[1392];
                validate_finite_parameter("pcgislb", params[1422]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1423] = params[1393];
                validate_finite_parameter("wcgislb", params[1423]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1424] = params[1394];
                validate_finite_parameter("p2cgislb", params[1424]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1425] = params[1395];
                validate_finite_parameter("egislb", params[1425]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1426] = params[1396];
                validate_finite_parameter("legislb", params[1426]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1427] = params[1397];
                validate_finite_parameter("negislb", params[1427]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1428] = params[1398];
                validate_finite_parameter("pegislb", params[1428]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1429] = params[1399];
                validate_finite_parameter("wegislb", params[1429]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1430] = params[1400];
                validate_finite_parameter("p2egislb", params[1430]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1431] = params[1401];
                validate_finite_parameter("pgislb", params[1431]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1432] = params[1402];
                validate_finite_parameter("lpgislb", params[1432]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1433] = params[1403];
                validate_finite_parameter("npgislb", params[1433]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1434] = params[1404];
                validate_finite_parameter("ppgislb", params[1434]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1435] = params[1405];
                validate_finite_parameter("wpgislb", params[1435]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1436] = params[1406];
                validate_finite_parameter("p2pgislb", params[1436]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_71: [f64; 91] = [
                0.0, 0.0, 0.0, 0.0, 0.1, 0.1, 0.1, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 10000000.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 5e-10, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_71.as_ptr(), (*ptr).values.as_mut_ptr().add(1437), 91);
            {
                let params = &mut *ptr;
                params[1528] = params[89];
                validate_parameter("eotacc", params[1528], false, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_72: [f64; 7] = [
                0.0, 2.5e-11, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_72.as_ptr(), (*ptr).values.as_mut_ptr().add(1529), 7);
            {
                let params = &mut *ptr;
                params[1536] = params[1530];
                validate_finite_parameter("cfd", params[1536]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1537] = params[1531];
                validate_finite_parameter("lcfd", params[1537]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1538] = params[1532];
                validate_finite_parameter("ncfd", params[1538]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1539] = params[1533];
                validate_finite_parameter("pcfd", params[1539]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1540] = params[1534];
                validate_finite_parameter("wcfd", params[1540]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1541] = params[1535];
                validate_finite_parameter("p2cfd", params[1541]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_73: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_73.as_ptr(), (*ptr).values.as_mut_ptr().add(1542), 1);
            {
                let params = &mut *ptr;
                params[1543] = params[1542];
                validate_parameter("cgdo", params[1543], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_74: [f64; 9] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_74.as_ptr(), (*ptr).values.as_mut_ptr().add(1544), 9);
            {
                let params = &mut *ptr;
                params[1553] = params[1547];
                validate_finite_parameter("cgdl", params[1553]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1554] = params[1548];
                validate_finite_parameter("lcgdl", params[1554]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1555] = params[1549];
                validate_finite_parameter("ncgdl", params[1555]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1556] = params[1550];
                validate_finite_parameter("pcgdl", params[1556]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1557] = params[1551];
                validate_finite_parameter("wcgdl", params[1557]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1558] = params[1552];
                validate_finite_parameter("p2cgdl", params[1558]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_75: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_75.as_ptr(), (*ptr).values.as_mut_ptr().add(1559), 12);
            {
                let params = &mut *ptr;
                params[1571] = params[1565];
                validate_finite_parameter("ckappad", params[1571]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1572] = params[1566];
                validate_finite_parameter("lckappad", params[1572]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1573] = params[1567];
                validate_finite_parameter("nckappad", params[1573]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1574] = params[1568];
                validate_finite_parameter("pckappad", params[1574]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1575] = params[1569];
                validate_finite_parameter("wckappad", params[1575]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1576] = params[1570];
                validate_finite_parameter("p2ckappad", params[1576]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_76: [f64; 8] = [
                0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_76.as_ptr(), (*ptr).values.as_mut_ptr().add(1577), 8);
            {
                let params = &mut *ptr;
                params[1585] = params[1584];
                validate_parameter("cjd", params[1585], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_77: [f64; 1] = [
                5e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_77.as_ptr(), (*ptr).values.as_mut_ptr().add(1586), 1);
            {
                let params = &mut *ptr;
                params[1587] = params[1586];
                validate_parameter("cjswd", params[1587], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_78: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_78.as_ptr(), (*ptr).values.as_mut_ptr().add(1588), 1);
            {
                let params = &mut *ptr;
                params[1589] = params[1588];
                validate_parameter("cjswgd", params[1589], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_79: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_79.as_ptr(), (*ptr).values.as_mut_ptr().add(1590), 1);
            {
                let params = &mut *ptr;
                params[1591] = params[1590];
                validate_finite_parameter("pbd", params[1591]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_80: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_80.as_ptr(), (*ptr).values.as_mut_ptr().add(1592), 1);
            {
                let params = &mut *ptr;
                params[1593] = params[1592];
                validate_finite_parameter("pbswd", params[1593]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1594] = params[1592];
                validate_finite_parameter("pbswgs", params[1594]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1595] = params[1594];
                validate_finite_parameter("pbswgd", params[1595]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_81: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_81.as_ptr(), (*ptr).values.as_mut_ptr().add(1596), 1);
            {
                let params = &mut *ptr;
                params[1597] = params[1596];
                validate_parameter("mjd", params[1597], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_82: [f64; 1] = [
                0.33,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_82.as_ptr(), (*ptr).values.as_mut_ptr().add(1598), 1);
            {
                let params = &mut *ptr;
                params[1599] = params[1598];
                validate_parameter("mjswd", params[1599], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1600] = params[1598];
                validate_parameter("mjswgs", params[1600], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1601] = params[1600];
                validate_parameter("mjswgd", params[1601], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_83: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_83.as_ptr(), (*ptr).values.as_mut_ptr().add(1602), 1);
            {
                let params = &mut *ptr;
                params[1603] = params[1602];
                validate_parameter("sjd", params[1603], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_84: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_84.as_ptr(), (*ptr).values.as_mut_ptr().add(1604), 1);
            {
                let params = &mut *ptr;
                params[1605] = params[1604];
                validate_parameter("sjswd", params[1605], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_85: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_85.as_ptr(), (*ptr).values.as_mut_ptr().add(1606), 1);
            {
                let params = &mut *ptr;
                params[1607] = params[1606];
                validate_parameter("sjswgd", params[1607], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_86: [f64; 1] = [
                0.125,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_86.as_ptr(), (*ptr).values.as_mut_ptr().add(1608), 1);
            {
                let params = &mut *ptr;
                params[1609] = params[1608];
                validate_finite_parameter("mjd2", params[1609]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_87: [f64; 1] = [
                0.083,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_87.as_ptr(), (*ptr).values.as_mut_ptr().add(1610), 1);
            {
                let params = &mut *ptr;
                params[1611] = params[1610];
                validate_finite_parameter("mjswd2", params[1611]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1612] = params[1610];
                validate_finite_parameter("mjswgs2", params[1612]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1613] = params[1612];
                validate_finite_parameter("mjswgd2", params[1613]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_88: [f64; 1] = [
                0.0001,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_88.as_ptr(), (*ptr).values.as_mut_ptr().add(1614), 1);
            {
                let params = &mut *ptr;
                params[1615] = params[1614];
                validate_parameter("jsd", params[1615], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_89: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_89.as_ptr(), (*ptr).values.as_mut_ptr().add(1616), 1);
            {
                let params = &mut *ptr;
                params[1617] = params[1616];
                validate_parameter("jswd", params[1617], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_90: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_90.as_ptr(), (*ptr).values.as_mut_ptr().add(1618), 1);
            {
                let params = &mut *ptr;
                params[1619] = params[1618];
                validate_parameter("jswgd", params[1619], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_91: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_91.as_ptr(), (*ptr).values.as_mut_ptr().add(1620), 1);
            {
                let params = &mut *ptr;
                params[1621] = params[1620];
                validate_parameter("njd", params[1621], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_92: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_92.as_ptr(), (*ptr).values.as_mut_ptr().add(1622), 1);
            {
                let params = &mut *ptr;
                params[1623] = params[1622];
                validate_finite_parameter("ijthdfwd", params[1623]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_93: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_93.as_ptr(), (*ptr).values.as_mut_ptr().add(1624), 1);
            {
                let params = &mut *ptr;
                params[1625] = params[1624];
                validate_finite_parameter("ijthdrev", params[1625]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_94: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_94.as_ptr(), (*ptr).values.as_mut_ptr().add(1626), 1);
            {
                let params = &mut *ptr;
                params[1627] = params[1626];
                validate_finite_parameter("bvd", params[1627]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_95: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_95.as_ptr(), (*ptr).values.as_mut_ptr().add(1628), 1);
            {
                let params = &mut *ptr;
                params[1629] = params[1628];
                validate_finite_parameter("xjbvd", params[1629]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_96: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_96.as_ptr(), (*ptr).values.as_mut_ptr().add(1630), 1);
            {
                let params = &mut *ptr;
                params[1631] = params[1630];
                validate_finite_parameter("jtsd", params[1631]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_97: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_97.as_ptr(), (*ptr).values.as_mut_ptr().add(1632), 1);
            {
                let params = &mut *ptr;
                params[1633] = params[1632];
                validate_finite_parameter("jtsswd", params[1633]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_98: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_98.as_ptr(), (*ptr).values.as_mut_ptr().add(1634), 1);
            {
                let params = &mut *ptr;
                params[1635] = params[1634];
                validate_finite_parameter("jtsswgd", params[1635]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_99: [f64; 2] = [
                0.0, 20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_99.as_ptr(), (*ptr).values.as_mut_ptr().add(1636), 2);
            {
                let params = &mut *ptr;
                params[1638] = params[1637];
                validate_finite_parameter("njtsd", params[1638]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_100: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_100.as_ptr(), (*ptr).values.as_mut_ptr().add(1639), 1);
            {
                let params = &mut *ptr;
                params[1640] = params[1639];
                validate_finite_parameter("njtsswd", params[1640]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_101: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_101.as_ptr(), (*ptr).values.as_mut_ptr().add(1641), 1);
            {
                let params = &mut *ptr;
                params[1642] = params[1641];
                validate_finite_parameter("njtsswgd", params[1642]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_102: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_102.as_ptr(), (*ptr).values.as_mut_ptr().add(1643), 1);
            {
                let params = &mut *ptr;
                params[1644] = params[1643];
                validate_finite_parameter("vtsd", params[1644]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_103: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_103.as_ptr(), (*ptr).values.as_mut_ptr().add(1645), 1);
            {
                let params = &mut *ptr;
                params[1646] = params[1645];
                validate_finite_parameter("vtsswd", params[1646]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_104: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_104.as_ptr(), (*ptr).values.as_mut_ptr().add(1647), 1);
            {
                let params = &mut *ptr;
                params[1648] = params[1647];
                validate_finite_parameter("vtsswgd", params[1648]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_105: [f64; 40] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 12.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                41000000.0, 6.25e39, 3.125e24, 87500000.0, 1.0, 1.0, 0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_105.as_ptr(), (*ptr).values.as_mut_ptr().add(1649), 40);
            {
                let params = &mut *ptr;
                params[1689] = params[1682];
                validate_finite_parameter("noia2", params[1689]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_106: [f64; 38] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 1.2, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.5774, 0.0, 0.3652, 0.0, 0.3953, 0.0,
                0.0, 0.0, 0.1, 27.0, 0.000702, 1108.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_106.as_ptr(), (*ptr).values.as_mut_ptr().add(1690), 38);
            {
                let params = &mut *ptr;
                params[1728] = params[1727];
                validate_finite_parameter("xtid", params[1728]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_107: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_107.as_ptr(), (*ptr).values.as_mut_ptr().add(1729), 1);
            {
                let params = &mut *ptr;
                params[1730] = params[1729];
                validate_finite_parameter("xtsd", params[1730]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_108: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_108.as_ptr(), (*ptr).values.as_mut_ptr().add(1731), 1);
            {
                let params = &mut *ptr;
                params[1732] = params[1731];
                validate_finite_parameter("xtsswd", params[1732]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_109: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_109.as_ptr(), (*ptr).values.as_mut_ptr().add(1733), 1);
            {
                let params = &mut *ptr;
                params[1734] = params[1733];
                validate_finite_parameter("xtsswgd", params[1734]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_110: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_110.as_ptr(), (*ptr).values.as_mut_ptr().add(1735), 1);
            {
                let params = &mut *ptr;
                params[1736] = params[1735];
                validate_finite_parameter("tnjtsd", params[1736]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_111: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_111.as_ptr(), (*ptr).values.as_mut_ptr().add(1737), 1);
            {
                let params = &mut *ptr;
                params[1738] = params[1737];
                validate_finite_parameter("tnjtsswd", params[1738]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_112: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_112.as_ptr(), (*ptr).values.as_mut_ptr().add(1739), 1);
            {
                let params = &mut *ptr;
                params[1740] = params[1739];
                validate_finite_parameter("tnjtsswgd", params[1740]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_113: [f64; 109] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.01, 0.1,
                40.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -0.003, 0.0, 0.0, 0.0, 0.0,
                0.0, -1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5,
                0.0, 0.0, 0.0, 0.0, 0.0, 50.0, 0.0, 1.0,
                0.001, 0.0, 0.01, 1e-5, 0.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.5556, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                3.0, 2.6, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
                2.6, 0.0, 0.0, 0.0, 0.0, 0.0, 9.5e-9, 0.1,
                14.0, 0.0, 0.0, 0.0, 0.0, 0.0, 24.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 24.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 2.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_113.as_ptr(), (*ptr).values.as_mut_ptr().add(1741), 109);
            {
                let params = &mut *ptr;
                params[1850] = params[1827];
                validate_parameter("wssp0", params[1850], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[1851] = params[1828];
                validate_parameter("wsspr", params[1851], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_114: [f64; 57] = [
                8e-9, 0.139, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
                1.0, 11.2, 0.0, 0.0, 0.0, 0.0, 0.0, 8.02,
                0.0, 0.0, 0.0, 0.0, 0.0, 6.18, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.8, 1.0, 0.67, 0.23, 1.1, 2.4, 2.0,
                2.0, 6.0, 2.4, 5e16, 100000.0, 0.0, 0.0, 60.0,
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_114.as_ptr(), (*ptr).values.as_mut_ptr().add(1852), 57);
            {
                let params = &mut *ptr;
                params[1909] = params[1903];
                validate_parameter("nvsrs", params[1909], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_115: [f64; 8] = [
                0.0, 0.0, 0.0, 0.0, 0.001, 0.001, 8.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_115.as_ptr(), (*ptr).values.as_mut_ptr().add(1910), 8);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1918] = [
    ("l", 0), ("lover", 1), ("dia", 2), ("tfin", 3), ("fpitch", 4), ("nfin", 5), ("ngcon", 6), ("aseo", 7), ("adeo", 8), ("pseo", 9), ("pdeo", 10), ("asej", 11), ("adej", 12), ("psej", 13), ("pdej", 14), ("cgsp", 15),
    ("cgdp", 16), ("cdsp", 17), ("nrs", 18), ("nrd", 19), ("lrsd", 20), ("nfinnom", 21), ("dtemp", 22), ("delvtrand", 23), ("u0mult", 24), ("ids0mult", 25), ("igc0mult", 26), ("igb0mult", 27), ("covs", 28), ("lcovs", 29), ("ncovs", 30), ("pcovs", 31),
    ("wcovs", 32), ("p2covs", 33), ("covd", 34), ("lcovd", 35), ("ncovd", 36), ("pcovd", 37), ("wcovd", 38), ("p2covd", 39), ("tgaa", 40), ("tsus", 41), ("hpff", 42), ("wgaa", 43), ("dws1", 44), ("dach1", 45), ("dws2", 46), ("dach2", 47),
    ("dws3", 48), ("dach3", 49), ("dws4", 50), ("dach4", 51), ("dws5", 52), ("dach5", 53), ("dws6", 54), ("dach6", 55), ("ngaa", 56), ("subbandmod", 57), ("mobscmod", 58), ("nf", 59), ("type", 60), ("bulkmod", 61), ("geomod", 62), ("cgeo1sw", 63),
    ("rdsmod", 64), ("hvmod", 65), ("asymmod", 66), ("cvmod", 67), ("igcmod", 68), ("igbmod", 69), ("gidlmod", 70), ("iimod", 71), ("tnoimod", 72), ("nqsmod", 73), ("shmod", 74), ("tempmod", 75), ("rgatemod", 76), ("rgeomod", 77), ("cgeomod", 78), ("fnmod", 79),
    ("cryomod", 80), ("sh_warn", 81), ("igclamp", 82), ("ll", 83), ("lln", 84), ("dlc", 85), ("dlcacc", 86), ("dwcacc", 87), ("llc", 88), ("eot", 89), ("toxp", 90), ("eotbox", 91), ("hfin", 92), ("deltaw", 93), ("deltawcv", 94), ("nbodyn1", 95),
    ("nbodyn2", 96), ("nsd", 97), ("phigl", 98), ("phiglt", 99), ("phign1", 100), ("phign2", 101), ("epsrox", 102), ("epsrsub", 103), ("easub", 104), ("ni0sub", 105), ("bg0sub", 106), ("nc0sub", 107), ("imin", 108), ("xl", 109), ("lxl", 110), ("nxl", 111),
    ("pxl", 112), ("lint", 113), ("llint", 114), ("nlint", 115), ("plint", 116), ("dlbin", 117), ("ldlbin", 118), ("ndlbin", 119), ("pdlbin", 120), ("xw", 121), ("lxw", 122), ("nxw", 123), ("pxw", 124), ("wxw", 125), ("p2xw", 126), ("dwbin", 127),
    ("ldwbin", 128), ("ndwbin", 129), ("pdwbin", 130), ("wdwbin", 131), ("p2dwbin", 132), ("nbody", 133), ("lnbody", 134), ("nnbody", 135), ("pnbody", 136), ("phig", 137), ("lphig", 138), ("nphig", 139), ("pphig", 140), ("wphig", 141), ("p2phig", 142), ("vfbdriftd", 143),
    ("vfbdrifts", 144), ("ngate", 145), ("lngate", 146), ("nngate", 147), ("pngate", 148), ("wngate", 149), ("p2ngate", 150), ("minr", 151), ("cdscn1", 152), ("cdscn2", 153), ("cdscdn1", 154), ("cdscdn2", 155), ("cdscdrn1", 156), ("cdscdrn2", 157), ("eta0n1", 158), ("eta0n2", 159),
    ("eta0lt", 160), ("eta0n1cv", 161), ("eta0n2cv", 162), ("eta0ltcv", 163), ("teta0", 164), ("teta0cv", 165), ("teta0r", 166), ("advtp0", 167), ("bdvtp0", 168), ("advtp1", 169), ("bdvtp1", 170), ("dvtp2", 171), ("thetasce", 172), ("thetadibl", 173), ("thetasw", 174), ("nvtm", 175),
    ("dvtp0", 176), ("ldvtp0", 177), ("ndvtp0", 178), ("pdvtp0", 179), ("wdvtp0", 180), ("p2dvtp0", 181), ("dvtp1", 182), ("ldvtp1", 183), ("ndvtp1", 184), ("pdvtp1", 185), ("wdvtp1", 186), ("p2dvtp1", 187), ("cit", 188), ("lcit", 189), ("ncit", 190), ("pcit", 191),
    ("wcit", 192), ("p2cit", 193), ("citr", 194), ("lcitr", 195), ("ncitr", 196), ("pcitr", 197), ("wcitr", 198), ("p2citr", 199), ("cdsc", 200), ("lcdsc", 201), ("ncdsc", 202), ("pcdsc", 203), ("wcdsc", 204), ("p2cdsc", 205), ("cdscd", 206), ("lcdscd", 207),
    ("ncdscd", 208), ("pcdscd", 209), ("wcdscd", 210), ("p2cdscd", 211), ("cdscdr", 212), ("lcdscdr", 213), ("ncdscdr", 214), ("pcdscdr", 215), ("wcdscdr", 216), ("p2cdscdr", 217), ("dvt0", 218), ("ldvt0", 219), ("ndvt0", 220), ("pdvt0", 221), ("wdvt0", 222), ("p2dvt0", 223),
    ("dvt1", 224), ("ldvt1", 225), ("ndvt1", 226), ("pdvt1", 227), ("wdvt1", 228), ("p2dvt1", 229), ("dvt1ss", 230), ("ldvt1ss", 231), ("ndvt1ss", 232), ("pdvt1ss", 233), ("wdvt1ss", 234), ("p2dvt1ss", 235), ("phin", 236), ("lphin", 237), ("nphin", 238), ("pphin", 239),
    ("wphin", 240), ("p2phin", 241), ("eta0", 242), ("leta0", 243), ("neta0", 244), ("peta0", 245), ("weta0", 246), ("p2eta0", 247), ("eta1", 248), ("leta1", 249), ("neta1", 250), ("peta1", 251), ("weta1", 252), ("p2eta1", 253), ("eta0r", 254), ("leta0r", 255),
    ("neta0r", 256), ("peta0r", 257), ("weta0r", 258), ("p2eta0r", 259), ("eta0cv", 260), ("leta0cv", 261), ("neta0cv", 262), ("peta0cv", 263), ("weta0cv", 264), ("p2eta0cv", 265), ("dsub", 266), ("ldsub", 267), ("ndsub", 268), ("pdsub", 269), ("wdsub", 270), ("p2dsub", 271),
    ("k1rsce", 272), ("lk1rsce", 273), ("nk1rsce", 274), ("pk1rsce", 275), ("wk1rsce", 276), ("p2k1rsce", 277), ("lpe0", 278), ("llpe0", 279), ("nlpe0", 280), ("plpe0", 281), ("wlpe0", 282), ("p2lpe0", 283), ("dvtshift", 284), ("ldvtshift", 285), ("ndvtshift", 286), ("pdvtshift", 287),
    ("wdvtshift", 288), ("p2dvtshift", 289), ("dvtshiftr", 290), ("ldvtshiftr", 291), ("ndvtshiftr", 292), ("pdvtshiftr", 293), ("wdvtshiftr", 294), ("p2dvtshiftr", 295), ("k0", 296), ("lk0", 297), ("nk0", 298), ("pk0", 299), ("wk0", 300), ("p2k0", 301), ("k01", 302), ("lk01", 303),
    ("nk01", 304), ("pk01", 305), ("wk01", 306), ("p2k01", 307), ("k0si", 308), ("lk0si", 309), ("nk0si", 310), ("pk0si", 311), ("wk0si", 312), ("p2k0si", 313), ("k0si1", 314), ("lk0si1", 315), ("nk0si1", 316), ("pk0si1", 317), ("wk0si1", 318), ("p2k0si1", 319),
    ("k2si", 320), ("lk2si", 321), ("nk2si", 322), ("pk2si", 323), ("wk2si", 324), ("p2k2si", 325), ("k2si1", 326), ("lk2si1", 327), ("nk2si1", 328), ("pk2si1", 329), ("wk2si1", 330), ("p2k2si1", 331), ("k0sisat", 332), ("lk0sisat", 333), ("nk0sisat", 334), ("pk0sisat", 335),
    ("wk0sisat", 336), ("p2k0sisat", 337), ("k0sisat1", 338), ("lk0sisat1", 339), ("nk0sisat1", 340), ("pk0sisat1", 341), ("wk0sisat1", 342), ("p2k0sisat1", 343), ("k2sisat", 344), ("lk2sisat", 345), ("nk2sisat", 346), ("pk2sisat", 347), ("wk2sisat", 348), ("p2k2sisat", 349), ("k2sisat1", 350), ("lk2sisat1", 351),
    ("nk2sisat1", 352), ("pk2sisat1", 353), ("wk2sisat1", 354), ("p2k2sisat1", 355), ("phibe", 356), ("lphibe", 357), ("nphibe", 358), ("pphibe", 359), ("wphibe", 360), ("p2phibe", 361), ("k1", 362), ("lk1", 363), ("nk1", 364), ("pk1", 365), ("wk1", 366), ("p2k1", 367),
    ("k11", 368), ("lk11", 369), ("nk11", 370), ("pk11", 371), ("wk11", 372), ("p2k11", 373), ("k2sat", 374), ("lk2sat", 375), ("nk2sat", 376), ("pk2sat", 377), ("wk2sat", 378), ("p2k2sat", 379), ("k2sat1", 380), ("lk2sat1", 381), ("nk2sat1", 382), ("pk2sat1", 383),
    ("wk2sat1", 384), ("p2k2sat1", 385), ("k2", 386), ("lk2", 387), ("nk2", 388), ("pk2", 389), ("wk2", 390), ("p2k2", 391), ("k21", 392), ("lk21", 393), ("nk21", 394), ("pk21", 395), ("wk21", 396), ("p2k21", 397), ("aqmtcen", 398), ("bqmtcen", 399),
    ("qm0", 400), ("qm0acc", 401), ("pqmacc", 402), ("qmfactor", 403), ("lqmfactor", 404), ("nqmfactor", 405), ("pqmfactor", 406), ("wqmfactor", 407), ("p2qmfactor", 408), ("qmtcencv", 409), ("lqmtcencv", 410), ("nqmtcencv", 411), ("pqmtcencv", 412), ("wqmtcencv", 413), ("p2qmtcencv", 414), ("qmtcencva", 415),
    ("lqmtcencva", 416), ("nqmtcencva", 417), ("pqmtcencva", 418), ("wqmtcencva", 419), ("p2qmtcencva", 420), ("pqm", 421), ("lpqm", 422), ("npqm", 423), ("ppqm", 424), ("wpqm", 425), ("p2pqm", 426), ("pqml", 427), ("vsatn1", 428), ("vsatn2", 429), ("avsat", 430), ("bvsat", 431),
    ("vsat1n1", 432), ("vsat1n2", 433), ("vsat1rn1", 434), ("vsat1rn2", 435), ("avsat1", 436), ("bvsat1", 437), ("apsat", 438), ("bpsat", 439), ("avsatcv", 440), ("bvsatcv", 441), ("apsatcv", 442), ("bpsatcv", 443), ("amexp", 444), ("bmexp", 445), ("amexpr", 446), ("bmexpr", 447),
    ("aptwg", 448), ("bptwg", 449), ("tmexp", 450), ("tmexp2", 451), ("tmexpr", 452), ("dvsatclamp", 453), ("vsatdr", 454), ("vsat", 455), ("lvsat", 456), ("nvsat", 457), ("pvsat", 458), ("wvsat", 459), ("p2vsat", 460), ("vsatr", 461), ("lvsatr", 462), ("nvsatr", 463),
    ("pvsatr", 464), ("wvsatr", 465), ("p2vsatr", 466), ("vsat1", 467), ("lvsat1", 468), ("nvsat1", 469), ("pvsat1", 470), ("wvsat1", 471), ("p2vsat1", 472), ("vsat1r", 473), ("lvsat1r", 474), ("nvsat1r", 475), ("pvsat1r", 476), ("wvsat1r", 477), ("p2vsat1r", 478), ("deltavsat", 479),
    ("ldeltavsat", 480), ("ndeltavsat", 481), ("pdeltavsat", 482), ("wdeltavsat", 483), ("p2deltavsat", 484), ("psat", 485), ("lpsat", 486), ("npsat", 487), ("ppsat", 488), ("wpsat", 489), ("p2psat", 490), ("ksativdr", 491), ("ksativ", 492), ("lksativ", 493), ("nksativ", 494), ("pksativ", 495),
    ("wksativ", 496), ("p2ksativ", 497), ("ksativt1", 498), ("ksativt2", 499), ("ksativr", 500), ("lksativr", 501), ("nksativr", 502), ("pksativr", 503), ("wksativr", 504), ("p2ksativr", 505), ("vsatcv", 506), ("lvsatcv", 507), ("nvsatcv", 508), ("pvsatcv", 509), ("wvsatcv", 510), ("p2vsatcv", 511),
    ("asat", 512), ("lasat", 513), ("nasat", 514), ("pasat", 515), ("wasat", 516), ("p2asat", 517), ("deltavsatcv", 518), ("ldeltavsatcv", 519), ("ndeltavsatcv", 520), ("pdeltavsatcv", 521), ("wdeltavsatcv", 522), ("p2deltavsatcv", 523), ("psatcv", 524), ("lpsatcv", 525), ("npsatcv", 526), ("ppsatcv", 527),
    ("wpsatcv", 528), ("p2psatcv", 529), ("mexpdr", 530), ("mexp", 531), ("lmexp", 532), ("nmexp", 533), ("pmexp", 534), ("wmexp", 535), ("p2mexp", 536), ("mexpr", 537), ("lmexpr", 538), ("nmexpr", 539), ("pmexpr", 540), ("wmexpr", 541), ("p2mexpr", 542), ("ptwg", 543),
    ("lptwg", 544), ("nptwg", 545), ("pptwg", 546), ("wptwg", 547), ("p2ptwg", 548), ("ptwgr", 549), ("lptwgr", 550), ("nptwgr", 551), ("pptwgr", 552), ("wptwgr", 553), ("p2ptwgr", 554), ("at", 555), ("lat", 556), ("nat", 557), ("pat", 558), ("wat", 559),
    ("p2at", 560), ("at2", 561), ("atr", 562), ("latr", 563), ("natr", 564), ("patr", 565), ("watr", 566), ("p2atr", 567), ("atcv", 568), ("latcv", 569), ("natcv", 570), ("patcv", 571), ("watcv", 572), ("p2atcv", 573), ("at2cv", 574), ("ptwgt", 575),
    ("lptwgt", 576), ("nptwgt", 577), ("pptwgt", 578), ("wptwgt", 579), ("p2ptwgt", 580), ("u0n1", 581), ("u0n1cv", 582), ("u0n1r", 583), ("u0n2", 584), ("u0n2cv", 585), ("u0n2r", 586), ("u0lt", 587), ("u0ltcv", 588), ("lpa", 589), ("lpar", 590), ("aua", 591),
    ("auar", 592), ("bua", 593), ("buar", 594), ("aeu", 595), ("aeur", 596), ("beu", 597), ("beur", 598), ("aud", 599), ("audr", 600), ("bud", 601), ("budr", 602), ("chargewf", 603), ("dmobclamp", 604), ("u0", 605), ("lu0", 606), ("nu0", 607),
    ("pu0", 608), ("wu0", 609), ("p2u0", 610), ("u0r", 611), ("lu0r", 612), ("nu0r", 613), ("pu0r", 614), ("wu0r", 615), ("p2u0r", 616), ("u0cv", 617), ("lu0cv", 618), ("nu0cv", 619), ("pu0cv", 620), ("wu0cv", 621), ("p2u0cv", 622), ("etamob", 623),
    ("letamob", 624), ("netamob", 625), ("petamob", 626), ("wetamob", 627), ("p2etamob", 628), ("up", 629), ("lup", 630), ("nup", 631), ("pup", 632), ("wup", 633), ("p2up", 634), ("upr", 635), ("lupr", 636), ("nupr", 637), ("pupr", 638), ("wupr", 639),
    ("p2upr", 640), ("ua", 641), ("lua", 642), ("nua", 643), ("pua", 644), ("wua", 645), ("p2ua", 646), ("uar", 647), ("luar", 648), ("nuar", 649), ("puar", 650), ("wuar", 651), ("p2uar", 652), ("uacv", 653), ("luacv", 654), ("nuacv", 655),
    ("puacv", 656), ("wuacv", 657), ("p2uacv", 658), ("uc", 659), ("luc", 660), ("nuc", 661), ("puc", 662), ("wuc", 663), ("p2uc", 664), ("ucr", 665), ("lucr", 666), ("nucr", 667), ("pucr", 668), ("wucr", 669), ("p2ucr", 670), ("uccv", 671),
    ("luccv", 672), ("nuccv", 673), ("puccv", 674), ("wuccv", 675), ("p2uccv", 676), ("eu", 677), ("leu", 678), ("neu", 679), ("peu", 680), ("weu", 681), ("p2eu", 682), ("eur", 683), ("leur", 684), ("neur", 685), ("peur", 686), ("weur", 687),
    ("p2eur", 688), ("ud", 689), ("lud", 690), ("nud", 691), ("pud", 692), ("wud", 693), ("p2ud", 694), ("udr", 695), ("ludr", 696), ("nudr", 697), ("pudr", 698), ("wudr", 699), ("p2udr", 700), ("udcv", 701), ("ludcv", 702), ("nudcv", 703),
    ("pudcv", 704), ("wudcv", 705), ("p2udcv", 706), ("ucs", 707), ("lucs", 708), ("nucs", 709), ("pucs", 710), ("wucs", 711), ("p2ucs", 712), ("uds", 713), ("luds", 714), ("nuds", 715), ("puds", 716), ("wuds", 717), ("p2uds", 718), ("uds1", 719),
    ("luds1", 720), ("nuds1", 721), ("puds1", 722), ("wuds1", 723), ("p2uds1", 724), ("udd", 725), ("ludd", 726), ("nudd", 727), ("pudd", 728), ("wudd", 729), ("p2udd", 730), ("udd1", 731), ("ludd1", 732), ("nudd1", 733), ("pudd1", 734), ("wudd1", 735),
    ("p2udd1", 736), ("ute", 737), ("lute", 738), ("nute", 739), ("pute", 740), ("wute", 741), ("p2ute", 742), ("uter", 743), ("luter", 744), ("nuter", 745), ("puter", 746), ("wuter", 747), ("p2uter", 748), ("utecv", 749), ("lutecv", 750), ("nutecv", 751),
    ("putecv", 752), ("wutecv", 753), ("p2utecv", 754), ("ute1", 755), ("lute1", 756), ("nute1", 757), ("pute1", 758), ("wute1", 759), ("p2ute1", 760), ("ute1cv", 761), ("lute1cv", 762), ("nute1cv", 763), ("pute1cv", 764), ("wute1cv", 765), ("p2ute1cv", 766), ("utl", 767),
    ("lutl", 768), ("nutl", 769), ("putl", 770), ("wutl", 771), ("p2utl", 772), ("utlr", 773), ("lutlr", 774), ("nutlr", 775), ("putlr", 776), ("wutlr", 777), ("p2utlr", 778), ("utlcv", 779), ("lutlcv", 780), ("nutlcv", 781), ("putlcv", 782), ("wutlcv", 783),
    ("p2utlcv", 784), ("emobt", 785), ("lemobt", 786), ("nemobt", 787), ("pemobt", 788), ("wemobt", 789), ("p2emobt", 790), ("ua1", 791), ("lua1", 792), ("nua1", 793), ("pua1", 794), ("wua1", 795), ("p2ua1", 796), ("ua1r", 797), ("lua1r", 798), ("nua1r", 799),
    ("pua1r", 800), ("wua1r", 801), ("p2ua1r", 802), ("ua1cv", 803), ("lua1cv", 804), ("nua1cv", 805), ("pua1cv", 806), ("wua1cv", 807), ("p2ua1cv", 808), ("ua2", 809), ("lua2", 810), ("nua2", 811), ("pua2", 812), ("wua2", 813), ("p2ua2", 814), ("ua2cv", 815),
    ("lua2cv", 816), ("nua2cv", 817), ("pua2cv", 818), ("wua2cv", 819), ("p2ua2cv", 820), ("eu1", 821), ("leu1", 822), ("neu1", 823), ("peu1", 824), ("weu1", 825), ("p2eu1", 826), ("uc1", 827), ("luc1", 828), ("nuc1", 829), ("puc1", 830), ("wuc1", 831),
    ("p2uc1", 832), ("uc1r", 833), ("luc1r", 834), ("nuc1r", 835), ("puc1r", 836), ("wuc1r", 837), ("p2uc1r", 838), ("uc1cv", 839), ("luc1cv", 840), ("nuc1cv", 841), ("puc1cv", 842), ("wuc1cv", 843), ("p2uc1cv", 844), ("ud1", 845), ("lud1", 846), ("nud1", 847),
    ("pud1", 848), ("wud1", 849), ("p2ud1", 850), ("ud1r", 851), ("lud1r", 852), ("nud1r", 853), ("pud1r", 854), ("wud1r", 855), ("p2ud1r", 856), ("ud1cv", 857), ("lud1cv", 858), ("nud1cv", 859), ("pud1cv", 860), ("wud1cv", 861), ("p2ud1cv", 862), ("ud2", 863),
    ("lud2", 864), ("nud2", 865), ("pud2", 866), ("wud2", 867), ("p2ud2", 868), ("ud2cv", 869), ("lud2cv", 870), ("nud2cv", 871), ("pud2cv", 872), ("wud2cv", 873), ("p2ud2cv", 874), ("ucste", 875), ("lucste", 876), ("nucste", 877), ("pucste", 878), ("wucste", 879),
    ("p2ucste", 880), ("ucste1", 881), ("lucste1", 882), ("nucste1", 883), ("pucste1", 884), ("wucste1", 885), ("p2ucste1", 886), ("muhc0", 887), ("muhc1", 888), ("etamobthin", 889), ("etamobtni", 890), ("etamobir", 891), ("uathin", 892), ("uatsat", 893), ("uartsc", 894), ("uatni", 895),
    ("uair", 896), ("euthin", 897), ("euptsc", 898), ("eutni", 899), ("euir", 900), ("udthin", 901), ("udtsat", 902), ("udptsc", 903), ("u0etawsc", 904), ("egbulk", 905), ("u0emsm1", 906), ("u0emsm2", 907), ("rdswmin", 908), ("ardsw", 909), ("brdsw", 910), ("rswmin", 911),
    ("arsw", 912), ("brsw", 913), ("rdwmin", 914), ("ardw", 915), ("brdw", 916), ("rsdr", 917), ("rsdrr", 918), ("rddr", 919), ("rddrr", 920), ("prsdr", 921), ("prddr", 922), ("trsdr", 923), ("trddr", 924), ("rdsw", 925), ("lrdsw", 926), ("nrdsw", 927),
    ("prdsw", 928), ("wrdsw", 929), ("p2rdsw", 930), ("rsw", 931), ("lrsw", 932), ("nrsw", 933), ("prsw", 934), ("wrsw", 935), ("p2rsw", 936), ("rdw", 937), ("lrdw", 938), ("nrdw", 939), ("prdw", 940), ("wrdw", 941), ("p2rdw", 942), ("prwgs", 943),
    ("lprwgs", 944), ("nprwgs", 945), ("pprwgs", 946), ("wprwgs", 947), ("p2prwgs", 948), ("prwgd", 949), ("lprwgd", 950), ("nprwgd", 951), ("pprwgd", 952), ("wprwgd", 953), ("p2prwgd", 954), ("wr", 955), ("lwr", 956), ("nwr", 957), ("pwr", 958), ("wwr", 959),
    ("p2wr", 960), ("prt", 961), ("lprt", 962), ("nprt", 963), ("pprt", 964), ("wprt", 965), ("p2prt", 966), ("prt1", 967), ("lprt1", 968), ("nprt1", 969), ("pprt1", 970), ("wprt1", 971), ("p2prt1", 972), ("tr0", 973), ("ltr0", 974), ("ntr0", 975),
    ("ptr0", 976), ("wtr0", 977), ("p2tr0", 978), ("sprt", 979), ("lsprt", 980), ("nsprt", 981), ("psprt", 982), ("wsprt", 983), ("p2sprt", 984), ("pdibl1", 985), ("lpdibl1", 986), ("npdibl1", 987), ("ppdibl1", 988), ("wpdibl1", 989), ("p2pdibl1", 990), ("pdibl2", 991),
    ("lpdibl2", 992), ("npdibl2", 993), ("ppdibl2", 994), ("wpdibl2", 995), ("p2pdibl2", 996), ("pdibl1r", 997), ("lpdibl1r", 998), ("npdibl1r", 999), ("ppdibl1r", 1000), ("wpdibl1r", 1001), ("p2pdibl1r", 1002), ("pdibl2r", 1003), ("lpdibl2r", 1004), ("npdibl2r", 1005), ("ppdibl2r", 1006), ("wpdibl2r", 1007),
    ("p2pdibl2r", 1008), ("drout", 1009), ("ldrout", 1010), ("ndrout", 1011), ("pdrout", 1012), ("wdrout", 1013), ("p2drout", 1014), ("pvag", 1015), ("lpvag", 1016), ("npvag", 1017), ("ppvag", 1018), ("wpvag", 1019), ("p2pvag", 1020), ("apclm", 1021), ("apclmr", 1022), ("bpclm", 1023),
    ("bpclmr", 1024), ("pclm", 1025), ("pclmt", 1026), ("lpclm", 1027), ("npclm", 1028), ("ppclm", 1029), ("wpclm", 1030), ("p2pclm", 1031), ("pclmr", 1032), ("lpclmr", 1033), ("npclmr", 1034), ("ppclmr", 1035), ("wpclmr", 1036), ("p2pclmr", 1037), ("pclmg", 1038), ("lpclmg", 1039),
    ("npclmg", 1040), ("ppclmg", 1041), ("wpclmg", 1042), ("p2pclmg", 1043), ("pclmcv", 1044), ("lpclmcv", 1045), ("npclmcv", 1046), ("ppclmcv", 1047), ("wpclmcv", 1048), ("p2pclmcv", 1049), ("a1", 1050), ("la1", 1051), ("na1", 1052), ("pa1", 1053), ("wa1", 1054), ("p2a1", 1055),
    ("a11", 1056), ("la11", 1057), ("na11", 1058), ("pa11", 1059), ("wa11", 1060), ("p2a11", 1061), ("a2", 1062), ("la2", 1063), ("na2", 1064), ("pa2", 1065), ("wa2", 1066), ("p2a2", 1067), ("a21", 1068), ("la21", 1069), ("na21", 1070), ("pa21", 1071),
    ("wa21", 1072), ("p2a21", 1073), ("rgext", 1074), ("rgfin", 1075), ("rgint", 1076), ("rgp", 1077), ("rshs", 1078), ("rshd", 1079), ("hepi", 1080), ("tsili", 1081), ("rhoc", 1082), ("rhorsd", 1083), ("cratio", 1084), ("deltaprsd", 1085), ("sdterm", 1086), ("lsp", 1087),
    ("epsrsp", 1088), ("tgate", 1089), ("tmask", 1090), ("asiliend", 1091), ("arsdend", 1092), ("prsdend", 1093), ("rgeoa", 1094), ("rgeob", 1095), ("rgeoc", 1096), ("rgeod", 1097), ("rgeoe", 1098), ("cgeoa", 1099), ("cgeob", 1100), ("cgeoc", 1101), ("cgeod", 1102), ("cgeoe", 1103),
    ("dlcigs", 1104), ("dlcigd", 1105), ("vfbsd", 1106), ("vfbsdcv", 1107), ("toxref", 1108), ("toxg", 1109), ("igbinvclamp", 1110), ("igbaccclamp", 1111), ("igcinvclamp", 1112), ("ntox", 1113), ("lntox", 1114), ("nntox", 1115), ("pntox", 1116), ("wntox", 1117), ("p2ntox", 1118), ("aigbinv", 1119),
    ("laigbinv", 1120), ("naigbinv", 1121), ("paigbinv", 1122), ("waigbinv", 1123), ("p2aigbinv", 1124), ("aigbinv1", 1125), ("laigbinv1", 1126), ("naigbinv1", 1127), ("paigbinv1", 1128), ("waigbinv1", 1129), ("p2aigbinv1", 1130), ("bigbinv", 1131), ("lbigbinv", 1132), ("nbigbinv", 1133), ("pbigbinv", 1134), ("wbigbinv", 1135),
    ("p2bigbinv", 1136), ("cigbinv", 1137), ("lcigbinv", 1138), ("ncigbinv", 1139), ("pcigbinv", 1140), ("wcigbinv", 1141), ("p2cigbinv", 1142), ("eigbinv", 1143), ("leigbinv", 1144), ("neigbinv", 1145), ("peigbinv", 1146), ("weigbinv", 1147), ("p2eigbinv", 1148), ("nigbinv", 1149), ("lnigbinv", 1150), ("nnigbinv", 1151),
    ("pnigbinv", 1152), ("wnigbinv", 1153), ("p2nigbinv", 1154), ("aigbacc", 1155), ("laigbacc", 1156), ("naigbacc", 1157), ("paigbacc", 1158), ("waigbacc", 1159), ("p2aigbacc", 1160), ("aigbacc1", 1161), ("laigbacc1", 1162), ("naigbacc1", 1163), ("paigbacc1", 1164), ("waigbacc1", 1165), ("p2aigbacc1", 1166), ("bigbacc", 1167),
    ("lbigbacc", 1168), ("nbigbacc", 1169), ("pbigbacc", 1170), ("wbigbacc", 1171), ("p2bigbacc", 1172), ("cigbacc", 1173), ("lcigbacc", 1174), ("ncigbacc", 1175), ("pcigbacc", 1176), ("wcigbacc", 1177), ("p2cigbacc", 1178), ("nigbacc", 1179), ("lnigbacc", 1180), ("nnigbacc", 1181), ("pnigbacc", 1182), ("wnigbacc", 1183),
    ("p2nigbacc", 1184), ("aigc", 1185), ("laigc", 1186), ("naigc", 1187), ("paigc", 1188), ("waigc", 1189), ("p2aigc", 1190), ("aigc1", 1191), ("laigc1", 1192), ("naigc1", 1193), ("paigc1", 1194), ("waigc1", 1195), ("p2aigc1", 1196), ("bigc", 1197), ("lbigc", 1198), ("nbigc", 1199),
    ("pbigc", 1200), ("wbigc", 1201), ("p2bigc", 1202), ("cigc", 1203), ("lcigc", 1204), ("ncigc", 1205), ("pcigc", 1206), ("wcigc", 1207), ("p2cigc", 1208), ("pigcd", 1209), ("lpigcd", 1210), ("npigcd", 1211), ("ppigcd", 1212), ("wpigcd", 1213), ("p2pigcd", 1214), ("aigs", 1215),
    ("laigs", 1216), ("naigs", 1217), ("paigs", 1218), ("waigs", 1219), ("p2aigs", 1220), ("aigs1", 1221), ("laigs1", 1222), ("naigs1", 1223), ("paigs1", 1224), ("waigs1", 1225), ("p2aigs1", 1226), ("bigs", 1227), ("lbigs", 1228), ("nbigs", 1229), ("pbigs", 1230), ("wbigs", 1231),
    ("p2bigs", 1232), ("cigs", 1233), ("lcigs", 1234), ("ncigs", 1235), ("pcigs", 1236), ("wcigs", 1237), ("p2cigs", 1238), ("aigd", 1239), ("laigd", 1240), ("naigd", 1241), ("paigd", 1242), ("waigd", 1243), ("p2aigd", 1244), ("aigd1", 1245), ("laigd1", 1246), ("naigd1", 1247),
    ("paigd1", 1248), ("waigd1", 1249), ("p2aigd1", 1250), ("bigd", 1251), ("lbigd", 1252), ("nbigd", 1253), ("pbigd", 1254), ("wbigd", 1255), ("p2bigd", 1256), ("cigd", 1257), ("lcigd", 1258), ("ncigd", 1259), ("pcigd", 1260), ("wcigd", 1261), ("p2cigd", 1262), ("poxedge", 1263),
    ("lpoxedge", 1264), ("npoxedge", 1265), ("ppoxedge", 1266), ("wpoxedge", 1267), ("p2poxedge", 1268), ("agidl", 1269), ("lagidl", 1270), ("nagidl", 1271), ("pagidl", 1272), ("wagidl", 1273), ("p2agidl", 1274), ("bgidl", 1275), ("lbgidl", 1276), ("nbgidl", 1277), ("pbgidl", 1278), ("wbgidl", 1279),
    ("p2bgidl", 1280), ("cgidl", 1281), ("lcgidl", 1282), ("ncgidl", 1283), ("pcgidl", 1284), ("wcgidl", 1285), ("p2cgidl", 1286), ("egidl", 1287), ("legidl", 1288), ("negidl", 1289), ("pegidl", 1290), ("wegidl", 1291), ("p2egidl", 1292), ("pgidl", 1293), ("lpgidl", 1294), ("npgidl", 1295),
    ("ppgidl", 1296), ("wpgidl", 1297), ("p2pgidl", 1298), ("agisl", 1299), ("lagisl", 1300), ("nagisl", 1301), ("pagisl", 1302), ("wagisl", 1303), ("p2agisl", 1304), ("bgisl", 1305), ("lbgisl", 1306), ("nbgisl", 1307), ("pbgisl", 1308), ("wbgisl", 1309), ("p2bgisl", 1310), ("cgisl", 1311),
    ("lcgisl", 1312), ("ncgisl", 1313), ("pcgisl", 1314), ("wcgisl", 1315), ("p2cgisl", 1316), ("egisl", 1317), ("legisl", 1318), ("negisl", 1319), ("pegisl", 1320), ("wegisl", 1321), ("p2egisl", 1322), ("pgisl", 1323), ("lpgisl", 1324), ("npgisl", 1325), ("ppgisl", 1326), ("wpgisl", 1327),
    ("p2pgisl", 1328), ("atatd", 1329), ("latatd", 1330), ("natatd", 1331), ("patatd", 1332), ("watatd", 1333), ("p2atatd", 1334), ("btatd", 1335), ("lbtatd", 1336), ("nbtatd", 1337), ("pbtatd", 1338), ("wbtatd", 1339), ("p2btatd", 1340), ("ctatd", 1341), ("lctatd", 1342), ("nctatd", 1343),
    ("pctatd", 1344), ("wctatd", 1345), ("p2ctatd", 1346), ("dtatd", 1347), ("ldtatd", 1348), ("ndtatd", 1349), ("pdtatd", 1350), ("wdtatd", 1351), ("p2dtatd", 1352), ("atats", 1353), ("latats", 1354), ("natats", 1355), ("patats", 1356), ("watats", 1357), ("p2atats", 1358), ("btats", 1359),
    ("lbtats", 1360), ("nbtats", 1361), ("pbtats", 1362), ("wbtats", 1363), ("p2btats", 1364), ("ctats", 1365), ("lctats", 1366), ("nctats", 1367), ("pctats", 1368), ("wctats", 1369), ("p2ctats", 1370), ("dtats", 1371), ("ldtats", 1372), ("ndtats", 1373), ("pdtats", 1374), ("wdtats", 1375),
    ("p2dtats", 1376), ("agidlb", 1377), ("lagidlb", 1378), ("nagidlb", 1379), ("pagidlb", 1380), ("wagidlb", 1381), ("p2agidlb", 1382), ("bgidlb", 1383), ("lbgidlb", 1384), ("nbgidlb", 1385), ("pbgidlb", 1386), ("wbgidlb", 1387), ("p2bgidlb", 1388), ("cgidlb", 1389), ("lcgidlb", 1390), ("ncgidlb", 1391),
    ("pcgidlb", 1392), ("wcgidlb", 1393), ("p2cgidlb", 1394), ("egidlb", 1395), ("legidlb", 1396), ("negidlb", 1397), ("pegidlb", 1398), ("wegidlb", 1399), ("p2egidlb", 1400), ("pgidlb", 1401), ("lpgidlb", 1402), ("npgidlb", 1403), ("ppgidlb", 1404), ("wpgidlb", 1405), ("p2pgidlb", 1406), ("agislb", 1407),
    ("lagislb", 1408), ("nagislb", 1409), ("pagislb", 1410), ("wagislb", 1411), ("p2agislb", 1412), ("bgislb", 1413), ("lbgislb", 1414), ("nbgislb", 1415), ("pbgislb", 1416), ("wbgislb", 1417), ("p2bgislb", 1418), ("cgislb", 1419), ("lcgislb", 1420), ("ncgislb", 1421), ("pcgislb", 1422), ("wcgislb", 1423),
    ("p2cgislb", 1424), ("egislb", 1425), ("legislb", 1426), ("negislb", 1427), ("pegislb", 1428), ("wegislb", 1429), ("p2egislb", 1430), ("pgislb", 1431), ("lpgislb", 1432), ("npgislb", 1433), ("ppgislb", 1434), ("wpgislb", 1435), ("p2pgislb", 1436), ("alpha01", 1437), ("alpha11", 1438), ("alphaii01", 1439),
    ("alphaii11", 1440), ("iimod2clamp1", 1441), ("iimod2clamp2", 1442), ("iimod2clamp3", 1443), ("alpha0", 1444), ("lalpha0", 1445), ("nalpha0", 1446), ("palpha0", 1447), ("walpha0", 1448), ("p2alpha0", 1449), ("alpha1", 1450), ("lalpha1", 1451), ("nalpha1", 1452), ("palpha1", 1453), ("walpha1", 1454), ("p2alpha1", 1455),
    ("beta0", 1456), ("lbeta0", 1457), ("nbeta0", 1458), ("pbeta0", 1459), ("wbeta0", 1460), ("p2beta0", 1461), ("alphaii0", 1462), ("lalphaii0", 1463), ("nalphaii0", 1464), ("palphaii0", 1465), ("walphaii0", 1466), ("p2alphaii0", 1467), ("alphaii1", 1468), ("lalphaii1", 1469), ("nalphaii1", 1470), ("palphaii1", 1471),
    ("walphaii1", 1472), ("p2alphaii1", 1473), ("betaii0", 1474), ("lbetaii0", 1475), ("nbetaii0", 1476), ("pbetaii0", 1477), ("wbetaii0", 1478), ("p2betaii0", 1479), ("betaii1", 1480), ("lbetaii1", 1481), ("nbetaii1", 1482), ("pbetaii1", 1483), ("wbetaii1", 1484), ("p2betaii1", 1485), ("betaii2", 1486), ("lbetaii2", 1487),
    ("nbetaii2", 1488), ("pbetaii2", 1489), ("wbetaii2", 1490), ("p2betaii2", 1491), ("esatii", 1492), ("lesatii", 1493), ("nesatii", 1494), ("pesatii", 1495), ("wesatii", 1496), ("p2esatii", 1497), ("lii", 1498), ("llii", 1499), ("nlii", 1500), ("plii", 1501), ("wlii", 1502), ("p2lii", 1503),
    ("sii0", 1504), ("lsii0", 1505), ("nsii0", 1506), ("psii0", 1507), ("wsii0", 1508), ("p2sii0", 1509), ("sii1", 1510), ("lsii1", 1511), ("nsii1", 1512), ("psii1", 1513), ("wsii1", 1514), ("p2sii1", 1515), ("sii2", 1516), ("lsii2", 1517), ("nsii2", 1518), ("psii2", 1519),
    ("wsii2", 1520), ("p2sii2", 1521), ("siid", 1522), ("lsiid", 1523), ("nsiid", 1524), ("psiid", 1525), ("wsiid", 1526), ("p2siid", 1527), ("eotacc", 1528), ("delvfbacc", 1529), ("cfs", 1530), ("lcfs", 1531), ("ncfs", 1532), ("pcfs", 1533), ("wcfs", 1534), ("p2cfs", 1535),
    ("cfd", 1536), ("lcfd", 1537), ("ncfd", 1538), ("pcfd", 1539), ("wcfd", 1540), ("p2cfd", 1541), ("cgso", 1542), ("cgdo", 1543), ("cgbo", 1544), ("cgbn", 1545), ("cgbw", 1546), ("cgsl", 1547), ("lcgsl", 1548), ("ncgsl", 1549), ("pcgsl", 1550), ("wcgsl", 1551),
    ("p2cgsl", 1552), ("cgdl", 1553), ("lcgdl", 1554), ("ncgdl", 1555), ("pcgdl", 1556), ("wcgdl", 1557), ("p2cgdl", 1558), ("cgbl", 1559), ("lcgbl", 1560), ("ncgbl", 1561), ("pcgbl", 1562), ("wcgbl", 1563), ("p2cgbl", 1564), ("ckappas", 1565), ("lckappas", 1566), ("nckappas", 1567),
    ("pckappas", 1568), ("wckappas", 1569), ("p2ckappas", 1570), ("ckappad", 1571), ("lckappad", 1572), ("nckappad", 1573), ("pckappad", 1574), ("wckappad", 1575), ("p2ckappad", 1576), ("ckappab", 1577), ("lckappab", 1578), ("nckappab", 1579), ("pckappab", 1580), ("wckappab", 1581), ("p2ckappab", 1582), ("csdesw", 1583),
    ("cjs", 1584), ("cjd", 1585), ("cjsws", 1586), ("cjswd", 1587), ("cjswgs", 1588), ("cjswgd", 1589), ("pbs", 1590), ("pbd", 1591), ("pbsws", 1592), ("pbswd", 1593), ("pbswgs", 1594), ("pbswgd", 1595), ("mjs", 1596), ("mjd", 1597), ("mjsws", 1598), ("mjswd", 1599),
    ("mjswgs", 1600), ("mjswgd", 1601), ("sjs", 1602), ("sjd", 1603), ("sjsws", 1604), ("sjswd", 1605), ("sjswgs", 1606), ("sjswgd", 1607), ("mjs2", 1608), ("mjd2", 1609), ("mjsws2", 1610), ("mjswd2", 1611), ("mjswgs2", 1612), ("mjswgd2", 1613), ("jss", 1614), ("jsd", 1615),
    ("jsws", 1616), ("jswd", 1617), ("jswgs", 1618), ("jswgd", 1619), ("njs", 1620), ("njd", 1621), ("ijthsfwd", 1622), ("ijthdfwd", 1623), ("ijthsrev", 1624), ("ijthdrev", 1625), ("bvs", 1626), ("bvd", 1627), ("xjbvs", 1628), ("xjbvd", 1629), ("jtss", 1630), ("jtsd", 1631),
    ("jtssws", 1632), ("jtsswd", 1633), ("jtsswgs", 1634), ("jtsswgd", 1635), ("jtweff", 1636), ("njts", 1637), ("njtsd", 1638), ("njtssw", 1639), ("njtsswd", 1640), ("njtsswg", 1641), ("njtsswgd", 1642), ("vtss", 1643), ("vtsd", 1644), ("vtssws", 1645), ("vtsswd", 1646), ("vtsswgs", 1647),
    ("vtsswgd", 1648), ("lintigen", 1649), ("ntgen", 1650), ("lntgen", 1651), ("nntgen", 1652), ("pntgen", 1653), ("wntgen", 1654), ("p2ntgen", 1655), ("aigen", 1656), ("laigen", 1657), ("naigen", 1658), ("paigen", 1659), ("waigen", 1660), ("p2aigen", 1661), ("bigen", 1662), ("lbigen", 1663),
    ("nbigen", 1664), ("pbigen", 1665), ("wbigen", 1666), ("p2bigen", 1667), ("xrcrg1", 1668), ("lxrcrg1", 1669), ("nxrcrg1", 1670), ("pxrcrg1", 1671), ("wxrcrg1", 1672), ("p2xrcrg1", 1673), ("xrcrg2", 1674), ("lxrcrg2", 1675), ("nxrcrg2", 1676), ("pxrcrg2", 1677), ("wxrcrg2", 1678), ("p2xrcrg2", 1679),
    ("ef", 1680), ("em", 1681), ("noia", 1682), ("noib", 1683), ("noic", 1684), ("k0noi", 1685), ("k1noi", 1686), ("lintnoi", 1687), ("smooth", 1688), ("noia2", 1689), ("lnoia2", 1690), ("nnoia2", 1691), ("pnoia2", 1692), ("wnoia2", 1693), ("p2noia2", 1694), ("mpower", 1695),
    ("lmpower", 1696), ("nmpower", 1697), ("pmpower", 1698), ("wmpower", 1699), ("p2mpower", 1700), ("qsref", 1701), ("lqsref", 1702), ("nqsref", 1703), ("pqsref", 1704), ("wqsref", 1705), ("p2qsref", 1706), ("ntnoi", 1707), ("rnoia", 1708), ("tnoia", 1709), ("rnoib", 1710), ("tnoib", 1711),
    ("rnoic", 1712), ("tnoic", 1713), ("rnoik", 1714), ("tnoik", 1715), ("tnoik2", 1716), ("tnom", 1717), ("tbgasub", 1718), ("tbgbsub", 1719), ("kt1l", 1720), ("tcj", 1721), ("tcjsw", 1722), ("tcjswg", 1723), ("tpb", 1724), ("tpbsw", 1725), ("tpbswg", 1726), ("xtis", 1727),
    ("xtid", 1728), ("xtss", 1729), ("xtsd", 1730), ("xtssws", 1731), ("xtsswd", 1732), ("xtsswgs", 1733), ("xtsswgd", 1734), ("tnjts", 1735), ("tnjtsd", 1736), ("tnjtssw", 1737), ("tnjtsswd", 1738), ("tnjtsswg", 1739), ("tnjtsswgd", 1740), ("kt1", 1741), ("lkt1", 1742), ("nkt1", 1743),
    ("pkt1", 1744), ("wkt1", 1745), ("p2kt1", 1746), ("kt11", 1747), ("kt12", 1748), ("tvth", 1749), ("tss", 1750), ("ltss", 1751), ("ntss", 1752), ("ptss", 1753), ("wtss", 1754), ("p2tss", 1755), ("iit", 1756), ("liit", 1757), ("niit", 1758), ("piit", 1759),
    ("wiit", 1760), ("p2iit", 1761), ("tii", 1762), ("ltii", 1763), ("ntii", 1764), ("ptii", 1765), ("wtii", 1766), ("p2tii", 1767), ("tgidl", 1768), ("ltgidl", 1769), ("ntgidl", 1770), ("ptgidl", 1771), ("wtgidl", 1772), ("p2tgidl", 1773), ("ttat", 1774), ("lttat", 1775),
    ("nttat", 1776), ("pttat", 1777), ("wttat", 1778), ("p2ttat", 1779), ("igt", 1780), ("ligt", 1781), ("nigt", 1782), ("pigt", 1783), ("wigt", 1784), ("p2igt", 1785), ("tlow", 1786), ("tlow1", 1787), ("dtlow", 1788), ("dtlow1", 1789), ("klow1", 1790), ("rth0", 1791),
    ("cth0", 1792), ("wth0", 1793), ("ashexp", 1794), ("bshexp", 1795), ("cshexp", 1796), ("ash", 1797), ("csh", 1798), ("ach_ufcm", 1799), ("cins_ufcm", 1800), ("w_ufcm", 1801), ("tfin_top", 1802), ("tfin_base", 1803), ("qmfactorcv", 1804), ("alpha_ufcm", 1805), ("dim1h", 1806), ("dimension1", 1807),
    ("ldimension1", 1808), ("ndimension1", 1809), ("pdimension1", 1810), ("wdimension1", 1811), ("p2dimension1", 1812), ("dim2h", 1813), ("dimension2", 1814), ("ldimension2", 1815), ("ndimension2", 1816), ("pdimension2", 1817), ("wdimension2", 1818), ("p2dimension2", 1819), ("dim3h", 1820), ("dimension3", 1821), ("ldimension3", 1822), ("ndimension3", 1823),
    ("pdimension3", 1824), ("wdimension3", 1825), ("p2dimension3", 1826), ("wdim0", 1827), ("wdimr", 1828), ("ssp1", 1829), ("lssp1", 1830), ("nssp1", 1831), ("pssp1", 1832), ("wssp1", 1833), ("p2ssp1", 1834), ("ssp2", 1835), ("lssp2", 1836), ("nssp2", 1837), ("pssp2", 1838), ("wssp2", 1839),
    ("p2ssp2", 1840), ("ssp3", 1841), ("lssp3", 1842), ("nssp3", 1843), ("pssp3", 1844), ("wssp3", 1845), ("p2ssp3", 1846), ("dssp1", 1847), ("dssp2", 1848), ("dssp3", 1849), ("wssp0", 1850), ("wsspr", 1851), ("wgaanom", 1852), ("e2nom", 1853), ("le2nom", 1854), ("ne2nom", 1855),
    ("pe2nom", 1856), ("we2nom", 1857), ("p2e2nom", 1858), ("e3nom", 1859), ("le3nom", 1860), ("ne3nom", 1861), ("pe3nom", 1862), ("we3nom", 1863), ("p2e3nom", 1864), ("mfe2", 1865), ("mfe3", 1866), ("wsfe2", 1867), ("wsfe3", 1868), ("mfq1nom", 1869), ("lmfq1nom", 1870), ("nmfq1nom", 1871),
    ("pmfq1nom", 1872), ("wmfq1nom", 1873), ("p2mfq1nom", 1874), ("mfq2nom", 1875), ("lmfq2nom", 1876), ("nmfq2nom", 1877), ("pmfq2nom", 1878), ("wmfq2nom", 1879), ("p2mfq2nom", 1880), ("mfq3nom", 1881), ("lmfq3nom", 1882), ("nmfq3nom", 1883), ("pmfq3nom", 1884), ("wmfq3nom", 1885), ("p2mfq3nom", 1886), ("mfq1", 1887),
    ("mfq2", 1888), ("mfq3", 1889), ("wsfq1", 1890), ("wsfq2", 1891), ("wsfq3", 1892), ("tsre2", 1893), ("tdwse2", 1894), ("tsre3", 1895), ("tdwse3", 1896), ("tsrq1", 1897), ("tdwsq1", 1898), ("tsrq2", 1899), ("tdwsq2", 1900), ("tsrq3", 1901), ("tdwsq3", 1902), ("nvsrd", 1903),
    ("vsatrsd", 1904), ("ptwgvsrsd", 1905), ("ptwg1vsrsd", 1906), ("psatxvsrsd", 1907), ("mvsrsd", 1908), ("nvsrs", 1909), ("rdlcw", 1910), ("rslcw", 1911), ("prtvsrsd", 1912), ("atvsrsd", 1913), ("vsrdfactor", 1914), ("vsrsfactor", 1915), ("rdvds", 1916), ("gavsrd", 1917),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1918] = [
    None, None, None, None, Some(3), None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1918] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1918] = [
    "l", "lover", "dia", "tfin", "fpitch", "nfin", "ngcon", "aseo", "adeo", "pseo", "pdeo", "asej", "adej", "psej", "pdej", "cgsp",
    "cgdp", "cdsp", "nrs", "nrd", "lrsd", "nfinnom", "dtemp", "delvtrand", "u0mult", "ids0mult", "igc0mult", "igb0mult", "covs", "lcovs", "ncovs", "pcovs",
    "wcovs", "p2covs", "covd", "lcovd", "ncovd", "pcovd", "wcovd", "p2covd", "tgaa", "tsus", "hpff", "wgaa", "dws1", "dach1", "dws2", "dach2",
    "dws3", "dach3", "dws4", "dach4", "dws5", "dach5", "dws6", "dach6", "ngaa", "subbandmod", "mobscmod", "nf", "type", "bulkmod", "geomod", "cgeo1sw",
    "rdsmod", "hvmod", "asymmod", "cvmod", "igcmod", "igbmod", "gidlmod", "iimod", "tnoimod", "nqsmod", "shmod", "tempmod", "rgatemod", "rgeomod", "cgeomod", "fnmod",
    "cryomod", "sh_warn", "igclamp", "ll", "lln", "dlc", "dlcacc", "dwcacc", "llc", "eot", "toxp", "eotbox", "hfin", "deltaw", "deltawcv", "nbodyn1",
    "nbodyn2", "nsd", "phigl", "phiglt", "phign1", "phign2", "epsrox", "epsrsub", "easub", "ni0sub", "bg0sub", "nc0sub", "imin", "xl", "lxl", "nxl",
    "pxl", "lint", "llint", "nlint", "plint", "dlbin", "ldlbin", "ndlbin", "pdlbin", "xw", "lxw", "nxw", "pxw", "wxw", "p2xw", "dwbin",
    "ldwbin", "ndwbin", "pdwbin", "wdwbin", "p2dwbin", "nbody", "lnbody", "nnbody", "pnbody", "phig", "lphig", "nphig", "pphig", "wphig", "p2phig", "vfbdriftd",
    "vfbdrifts", "ngate", "lngate", "nngate", "pngate", "wngate", "p2ngate", "minr", "cdscn1", "cdscn2", "cdscdn1", "cdscdn2", "cdscdrn1", "cdscdrn2", "eta0n1", "eta0n2",
    "eta0lt", "eta0n1cv", "eta0n2cv", "eta0ltcv", "teta0", "teta0cv", "teta0r", "advtp0", "bdvtp0", "advtp1", "bdvtp1", "dvtp2", "thetasce", "thetadibl", "thetasw", "nvtm",
    "dvtp0", "ldvtp0", "ndvtp0", "pdvtp0", "wdvtp0", "p2dvtp0", "dvtp1", "ldvtp1", "ndvtp1", "pdvtp1", "wdvtp1", "p2dvtp1", "cit", "lcit", "ncit", "pcit",
    "wcit", "p2cit", "citr", "lcitr", "ncitr", "pcitr", "wcitr", "p2citr", "cdsc", "lcdsc", "ncdsc", "pcdsc", "wcdsc", "p2cdsc", "cdscd", "lcdscd",
    "ncdscd", "pcdscd", "wcdscd", "p2cdscd", "cdscdr", "lcdscdr", "ncdscdr", "pcdscdr", "wcdscdr", "p2cdscdr", "dvt0", "ldvt0", "ndvt0", "pdvt0", "wdvt0", "p2dvt0",
    "dvt1", "ldvt1", "ndvt1", "pdvt1", "wdvt1", "p2dvt1", "dvt1ss", "ldvt1ss", "ndvt1ss", "pdvt1ss", "wdvt1ss", "p2dvt1ss", "phin", "lphin", "nphin", "pphin",
    "wphin", "p2phin", "eta0", "leta0", "neta0", "peta0", "weta0", "p2eta0", "eta1", "leta1", "neta1", "peta1", "weta1", "p2eta1", "eta0r", "leta0r",
    "neta0r", "peta0r", "weta0r", "p2eta0r", "eta0cv", "leta0cv", "neta0cv", "peta0cv", "weta0cv", "p2eta0cv", "dsub", "ldsub", "ndsub", "pdsub", "wdsub", "p2dsub",
    "k1rsce", "lk1rsce", "nk1rsce", "pk1rsce", "wk1rsce", "p2k1rsce", "lpe0", "llpe0", "nlpe0", "plpe0", "wlpe0", "p2lpe0", "dvtshift", "ldvtshift", "ndvtshift", "pdvtshift",
    "wdvtshift", "p2dvtshift", "dvtshiftr", "ldvtshiftr", "ndvtshiftr", "pdvtshiftr", "wdvtshiftr", "p2dvtshiftr", "k0", "lk0", "nk0", "pk0", "wk0", "p2k0", "k01", "lk01",
    "nk01", "pk01", "wk01", "p2k01", "k0si", "lk0si", "nk0si", "pk0si", "wk0si", "p2k0si", "k0si1", "lk0si1", "nk0si1", "pk0si1", "wk0si1", "p2k0si1",
    "k2si", "lk2si", "nk2si", "pk2si", "wk2si", "p2k2si", "k2si1", "lk2si1", "nk2si1", "pk2si1", "wk2si1", "p2k2si1", "k0sisat", "lk0sisat", "nk0sisat", "pk0sisat",
    "wk0sisat", "p2k0sisat", "k0sisat1", "lk0sisat1", "nk0sisat1", "pk0sisat1", "wk0sisat1", "p2k0sisat1", "k2sisat", "lk2sisat", "nk2sisat", "pk2sisat", "wk2sisat", "p2k2sisat", "k2sisat1", "lk2sisat1",
    "nk2sisat1", "pk2sisat1", "wk2sisat1", "p2k2sisat1", "phibe", "lphibe", "nphibe", "pphibe", "wphibe", "p2phibe", "k1", "lk1", "nk1", "pk1", "wk1", "p2k1",
    "k11", "lk11", "nk11", "pk11", "wk11", "p2k11", "k2sat", "lk2sat", "nk2sat", "pk2sat", "wk2sat", "p2k2sat", "k2sat1", "lk2sat1", "nk2sat1", "pk2sat1",
    "wk2sat1", "p2k2sat1", "k2", "lk2", "nk2", "pk2", "wk2", "p2k2", "k21", "lk21", "nk21", "pk21", "wk21", "p2k21", "aqmtcen", "bqmtcen",
    "qm0", "qm0acc", "pqmacc", "qmfactor", "lqmfactor", "nqmfactor", "pqmfactor", "wqmfactor", "p2qmfactor", "qmtcencv", "lqmtcencv", "nqmtcencv", "pqmtcencv", "wqmtcencv", "p2qmtcencv", "qmtcencva",
    "lqmtcencva", "nqmtcencva", "pqmtcencva", "wqmtcencva", "p2qmtcencva", "pqm", "lpqm", "npqm", "ppqm", "wpqm", "p2pqm", "pqml", "vsatn1", "vsatn2", "avsat", "bvsat",
    "vsat1n1", "vsat1n2", "vsat1rn1", "vsat1rn2", "avsat1", "bvsat1", "apsat", "bpsat", "avsatcv", "bvsatcv", "apsatcv", "bpsatcv", "amexp", "bmexp", "amexpr", "bmexpr",
    "aptwg", "bptwg", "tmexp", "tmexp2", "tmexpr", "dvsatclamp", "vsatdr", "vsat", "lvsat", "nvsat", "pvsat", "wvsat", "p2vsat", "vsatr", "lvsatr", "nvsatr",
    "pvsatr", "wvsatr", "p2vsatr", "vsat1", "lvsat1", "nvsat1", "pvsat1", "wvsat1", "p2vsat1", "vsat1r", "lvsat1r", "nvsat1r", "pvsat1r", "wvsat1r", "p2vsat1r", "deltavsat",
    "ldeltavsat", "ndeltavsat", "pdeltavsat", "wdeltavsat", "p2deltavsat", "psat", "lpsat", "npsat", "ppsat", "wpsat", "p2psat", "ksativdr", "ksativ", "lksativ", "nksativ", "pksativ",
    "wksativ", "p2ksativ", "ksativt1", "ksativt2", "ksativr", "lksativr", "nksativr", "pksativr", "wksativr", "p2ksativr", "vsatcv", "lvsatcv", "nvsatcv", "pvsatcv", "wvsatcv", "p2vsatcv",
    "asat", "lasat", "nasat", "pasat", "wasat", "p2asat", "deltavsatcv", "ldeltavsatcv", "ndeltavsatcv", "pdeltavsatcv", "wdeltavsatcv", "p2deltavsatcv", "psatcv", "lpsatcv", "npsatcv", "ppsatcv",
    "wpsatcv", "p2psatcv", "mexpdr", "mexp", "lmexp", "nmexp", "pmexp", "wmexp", "p2mexp", "mexpr", "lmexpr", "nmexpr", "pmexpr", "wmexpr", "p2mexpr", "ptwg",
    "lptwg", "nptwg", "pptwg", "wptwg", "p2ptwg", "ptwgr", "lptwgr", "nptwgr", "pptwgr", "wptwgr", "p2ptwgr", "at", "lat", "nat", "pat", "wat",
    "p2at", "at2", "atr", "latr", "natr", "patr", "watr", "p2atr", "atcv", "latcv", "natcv", "patcv", "watcv", "p2atcv", "at2cv", "ptwgt",
    "lptwgt", "nptwgt", "pptwgt", "wptwgt", "p2ptwgt", "u0n1", "u0n1cv", "u0n1r", "u0n2", "u0n2cv", "u0n2r", "u0lt", "u0ltcv", "lpa", "lpar", "aua",
    "auar", "bua", "buar", "aeu", "aeur", "beu", "beur", "aud", "audr", "bud", "budr", "chargewf", "dmobclamp", "u0", "lu0", "nu0",
    "pu0", "wu0", "p2u0", "u0r", "lu0r", "nu0r", "pu0r", "wu0r", "p2u0r", "u0cv", "lu0cv", "nu0cv", "pu0cv", "wu0cv", "p2u0cv", "etamob",
    "letamob", "netamob", "petamob", "wetamob", "p2etamob", "up", "lup", "nup", "pup", "wup", "p2up", "upr", "lupr", "nupr", "pupr", "wupr",
    "p2upr", "ua", "lua", "nua", "pua", "wua", "p2ua", "uar", "luar", "nuar", "puar", "wuar", "p2uar", "uacv", "luacv", "nuacv",
    "puacv", "wuacv", "p2uacv", "uc", "luc", "nuc", "puc", "wuc", "p2uc", "ucr", "lucr", "nucr", "pucr", "wucr", "p2ucr", "uccv",
    "luccv", "nuccv", "puccv", "wuccv", "p2uccv", "eu", "leu", "neu", "peu", "weu", "p2eu", "eur", "leur", "neur", "peur", "weur",
    "p2eur", "ud", "lud", "nud", "pud", "wud", "p2ud", "udr", "ludr", "nudr", "pudr", "wudr", "p2udr", "udcv", "ludcv", "nudcv",
    "pudcv", "wudcv", "p2udcv", "ucs", "lucs", "nucs", "pucs", "wucs", "p2ucs", "uds", "luds", "nuds", "puds", "wuds", "p2uds", "uds1",
    "luds1", "nuds1", "puds1", "wuds1", "p2uds1", "udd", "ludd", "nudd", "pudd", "wudd", "p2udd", "udd1", "ludd1", "nudd1", "pudd1", "wudd1",
    "p2udd1", "ute", "lute", "nute", "pute", "wute", "p2ute", "uter", "luter", "nuter", "puter", "wuter", "p2uter", "utecv", "lutecv", "nutecv",
    "putecv", "wutecv", "p2utecv", "ute1", "lute1", "nute1", "pute1", "wute1", "p2ute1", "ute1cv", "lute1cv", "nute1cv", "pute1cv", "wute1cv", "p2ute1cv", "utl",
    "lutl", "nutl", "putl", "wutl", "p2utl", "utlr", "lutlr", "nutlr", "putlr", "wutlr", "p2utlr", "utlcv", "lutlcv", "nutlcv", "putlcv", "wutlcv",
    "p2utlcv", "emobt", "lemobt", "nemobt", "pemobt", "wemobt", "p2emobt", "ua1", "lua1", "nua1", "pua1", "wua1", "p2ua1", "ua1r", "lua1r", "nua1r",
    "pua1r", "wua1r", "p2ua1r", "ua1cv", "lua1cv", "nua1cv", "pua1cv", "wua1cv", "p2ua1cv", "ua2", "lua2", "nua2", "pua2", "wua2", "p2ua2", "ua2cv",
    "lua2cv", "nua2cv", "pua2cv", "wua2cv", "p2ua2cv", "eu1", "leu1", "neu1", "peu1", "weu1", "p2eu1", "uc1", "luc1", "nuc1", "puc1", "wuc1",
    "p2uc1", "uc1r", "luc1r", "nuc1r", "puc1r", "wuc1r", "p2uc1r", "uc1cv", "luc1cv", "nuc1cv", "puc1cv", "wuc1cv", "p2uc1cv", "ud1", "lud1", "nud1",
    "pud1", "wud1", "p2ud1", "ud1r", "lud1r", "nud1r", "pud1r", "wud1r", "p2ud1r", "ud1cv", "lud1cv", "nud1cv", "pud1cv", "wud1cv", "p2ud1cv", "ud2",
    "lud2", "nud2", "pud2", "wud2", "p2ud2", "ud2cv", "lud2cv", "nud2cv", "pud2cv", "wud2cv", "p2ud2cv", "ucste", "lucste", "nucste", "pucste", "wucste",
    "p2ucste", "ucste1", "lucste1", "nucste1", "pucste1", "wucste1", "p2ucste1", "muhc0", "muhc1", "etamobthin", "etamobtni", "etamobir", "uathin", "uatsat", "uartsc", "uatni",
    "uair", "euthin", "euptsc", "eutni", "euir", "udthin", "udtsat", "udptsc", "u0etawsc", "egbulk", "u0emsm1", "u0emsm2", "rdswmin", "ardsw", "brdsw", "rswmin",
    "arsw", "brsw", "rdwmin", "ardw", "brdw", "rsdr", "rsdrr", "rddr", "rddrr", "prsdr", "prddr", "trsdr", "trddr", "rdsw", "lrdsw", "nrdsw",
    "prdsw", "wrdsw", "p2rdsw", "rsw", "lrsw", "nrsw", "prsw", "wrsw", "p2rsw", "rdw", "lrdw", "nrdw", "prdw", "wrdw", "p2rdw", "prwgs",
    "lprwgs", "nprwgs", "pprwgs", "wprwgs", "p2prwgs", "prwgd", "lprwgd", "nprwgd", "pprwgd", "wprwgd", "p2prwgd", "wr", "lwr", "nwr", "pwr", "wwr",
    "p2wr", "prt", "lprt", "nprt", "pprt", "wprt", "p2prt", "prt1", "lprt1", "nprt1", "pprt1", "wprt1", "p2prt1", "tr0", "ltr0", "ntr0",
    "ptr0", "wtr0", "p2tr0", "sprt", "lsprt", "nsprt", "psprt", "wsprt", "p2sprt", "pdibl1", "lpdibl1", "npdibl1", "ppdibl1", "wpdibl1", "p2pdibl1", "pdibl2",
    "lpdibl2", "npdibl2", "ppdibl2", "wpdibl2", "p2pdibl2", "pdibl1r", "lpdibl1r", "npdibl1r", "ppdibl1r", "wpdibl1r", "p2pdibl1r", "pdibl2r", "lpdibl2r", "npdibl2r", "ppdibl2r", "wpdibl2r",
    "p2pdibl2r", "drout", "ldrout", "ndrout", "pdrout", "wdrout", "p2drout", "pvag", "lpvag", "npvag", "ppvag", "wpvag", "p2pvag", "apclm", "apclmr", "bpclm",
    "bpclmr", "pclm", "pclmt", "lpclm", "npclm", "ppclm", "wpclm", "p2pclm", "pclmr", "lpclmr", "npclmr", "ppclmr", "wpclmr", "p2pclmr", "pclmg", "lpclmg",
    "npclmg", "ppclmg", "wpclmg", "p2pclmg", "pclmcv", "lpclmcv", "npclmcv", "ppclmcv", "wpclmcv", "p2pclmcv", "a1", "la1", "na1", "pa1", "wa1", "p2a1",
    "a11", "la11", "na11", "pa11", "wa11", "p2a11", "a2", "la2", "na2", "pa2", "wa2", "p2a2", "a21", "la21", "na21", "pa21",
    "wa21", "p2a21", "rgext", "rgfin", "rgint", "rgp", "rshs", "rshd", "hepi", "tsili", "rhoc", "rhorsd", "cratio", "deltaprsd", "sdterm", "lsp",
    "epsrsp", "tgate", "tmask", "asiliend", "arsdend", "prsdend", "rgeoa", "rgeob", "rgeoc", "rgeod", "rgeoe", "cgeoa", "cgeob", "cgeoc", "cgeod", "cgeoe",
    "dlcigs", "dlcigd", "vfbsd", "vfbsdcv", "toxref", "toxg", "igbinvclamp", "igbaccclamp", "igcinvclamp", "ntox", "lntox", "nntox", "pntox", "wntox", "p2ntox", "aigbinv",
    "laigbinv", "naigbinv", "paigbinv", "waigbinv", "p2aigbinv", "aigbinv1", "laigbinv1", "naigbinv1", "paigbinv1", "waigbinv1", "p2aigbinv1", "bigbinv", "lbigbinv", "nbigbinv", "pbigbinv", "wbigbinv",
    "p2bigbinv", "cigbinv", "lcigbinv", "ncigbinv", "pcigbinv", "wcigbinv", "p2cigbinv", "eigbinv", "leigbinv", "neigbinv", "peigbinv", "weigbinv", "p2eigbinv", "nigbinv", "lnigbinv", "nnigbinv",
    "pnigbinv", "wnigbinv", "p2nigbinv", "aigbacc", "laigbacc", "naigbacc", "paigbacc", "waigbacc", "p2aigbacc", "aigbacc1", "laigbacc1", "naigbacc1", "paigbacc1", "waigbacc1", "p2aigbacc1", "bigbacc",
    "lbigbacc", "nbigbacc", "pbigbacc", "wbigbacc", "p2bigbacc", "cigbacc", "lcigbacc", "ncigbacc", "pcigbacc", "wcigbacc", "p2cigbacc", "nigbacc", "lnigbacc", "nnigbacc", "pnigbacc", "wnigbacc",
    "p2nigbacc", "aigc", "laigc", "naigc", "paigc", "waigc", "p2aigc", "aigc1", "laigc1", "naigc1", "paigc1", "waigc1", "p2aigc1", "bigc", "lbigc", "nbigc",
    "pbigc", "wbigc", "p2bigc", "cigc", "lcigc", "ncigc", "pcigc", "wcigc", "p2cigc", "pigcd", "lpigcd", "npigcd", "ppigcd", "wpigcd", "p2pigcd", "aigs",
    "laigs", "naigs", "paigs", "waigs", "p2aigs", "aigs1", "laigs1", "naigs1", "paigs1", "waigs1", "p2aigs1", "bigs", "lbigs", "nbigs", "pbigs", "wbigs",
    "p2bigs", "cigs", "lcigs", "ncigs", "pcigs", "wcigs", "p2cigs", "aigd", "laigd", "naigd", "paigd", "waigd", "p2aigd", "aigd1", "laigd1", "naigd1",
    "paigd1", "waigd1", "p2aigd1", "bigd", "lbigd", "nbigd", "pbigd", "wbigd", "p2bigd", "cigd", "lcigd", "ncigd", "pcigd", "wcigd", "p2cigd", "poxedge",
    "lpoxedge", "npoxedge", "ppoxedge", "wpoxedge", "p2poxedge", "agidl", "lagidl", "nagidl", "pagidl", "wagidl", "p2agidl", "bgidl", "lbgidl", "nbgidl", "pbgidl", "wbgidl",
    "p2bgidl", "cgidl", "lcgidl", "ncgidl", "pcgidl", "wcgidl", "p2cgidl", "egidl", "legidl", "negidl", "pegidl", "wegidl", "p2egidl", "pgidl", "lpgidl", "npgidl",
    "ppgidl", "wpgidl", "p2pgidl", "agisl", "lagisl", "nagisl", "pagisl", "wagisl", "p2agisl", "bgisl", "lbgisl", "nbgisl", "pbgisl", "wbgisl", "p2bgisl", "cgisl",
    "lcgisl", "ncgisl", "pcgisl", "wcgisl", "p2cgisl", "egisl", "legisl", "negisl", "pegisl", "wegisl", "p2egisl", "pgisl", "lpgisl", "npgisl", "ppgisl", "wpgisl",
    "p2pgisl", "atatd", "latatd", "natatd", "patatd", "watatd", "p2atatd", "btatd", "lbtatd", "nbtatd", "pbtatd", "wbtatd", "p2btatd", "ctatd", "lctatd", "nctatd",
    "pctatd", "wctatd", "p2ctatd", "dtatd", "ldtatd", "ndtatd", "pdtatd", "wdtatd", "p2dtatd", "atats", "latats", "natats", "patats", "watats", "p2atats", "btats",
    "lbtats", "nbtats", "pbtats", "wbtats", "p2btats", "ctats", "lctats", "nctats", "pctats", "wctats", "p2ctats", "dtats", "ldtats", "ndtats", "pdtats", "wdtats",
    "p2dtats", "agidlb", "lagidlb", "nagidlb", "pagidlb", "wagidlb", "p2agidlb", "bgidlb", "lbgidlb", "nbgidlb", "pbgidlb", "wbgidlb", "p2bgidlb", "cgidlb", "lcgidlb", "ncgidlb",
    "pcgidlb", "wcgidlb", "p2cgidlb", "egidlb", "legidlb", "negidlb", "pegidlb", "wegidlb", "p2egidlb", "pgidlb", "lpgidlb", "npgidlb", "ppgidlb", "wpgidlb", "p2pgidlb", "agislb",
    "lagislb", "nagislb", "pagislb", "wagislb", "p2agislb", "bgislb", "lbgislb", "nbgislb", "pbgislb", "wbgislb", "p2bgislb", "cgislb", "lcgislb", "ncgislb", "pcgislb", "wcgislb",
    "p2cgislb", "egislb", "legislb", "negislb", "pegislb", "wegislb", "p2egislb", "pgislb", "lpgislb", "npgislb", "ppgislb", "wpgislb", "p2pgislb", "alpha01", "alpha11", "alphaii01",
    "alphaii11", "iimod2clamp1", "iimod2clamp2", "iimod2clamp3", "alpha0", "lalpha0", "nalpha0", "palpha0", "walpha0", "p2alpha0", "alpha1", "lalpha1", "nalpha1", "palpha1", "walpha1", "p2alpha1",
    "beta0", "lbeta0", "nbeta0", "pbeta0", "wbeta0", "p2beta0", "alphaii0", "lalphaii0", "nalphaii0", "palphaii0", "walphaii0", "p2alphaii0", "alphaii1", "lalphaii1", "nalphaii1", "palphaii1",
    "walphaii1", "p2alphaii1", "betaii0", "lbetaii0", "nbetaii0", "pbetaii0", "wbetaii0", "p2betaii0", "betaii1", "lbetaii1", "nbetaii1", "pbetaii1", "wbetaii1", "p2betaii1", "betaii2", "lbetaii2",
    "nbetaii2", "pbetaii2", "wbetaii2", "p2betaii2", "esatii", "lesatii", "nesatii", "pesatii", "wesatii", "p2esatii", "lii", "llii", "nlii", "plii", "wlii", "p2lii",
    "sii0", "lsii0", "nsii0", "psii0", "wsii0", "p2sii0", "sii1", "lsii1", "nsii1", "psii1", "wsii1", "p2sii1", "sii2", "lsii2", "nsii2", "psii2",
    "wsii2", "p2sii2", "siid", "lsiid", "nsiid", "psiid", "wsiid", "p2siid", "eotacc", "delvfbacc", "cfs", "lcfs", "ncfs", "pcfs", "wcfs", "p2cfs",
    "cfd", "lcfd", "ncfd", "pcfd", "wcfd", "p2cfd", "cgso", "cgdo", "cgbo", "cgbn", "cgbw", "cgsl", "lcgsl", "ncgsl", "pcgsl", "wcgsl",
    "p2cgsl", "cgdl", "lcgdl", "ncgdl", "pcgdl", "wcgdl", "p2cgdl", "cgbl", "lcgbl", "ncgbl", "pcgbl", "wcgbl", "p2cgbl", "ckappas", "lckappas", "nckappas",
    "pckappas", "wckappas", "p2ckappas", "ckappad", "lckappad", "nckappad", "pckappad", "wckappad", "p2ckappad", "ckappab", "lckappab", "nckappab", "pckappab", "wckappab", "p2ckappab", "csdesw",
    "cjs", "cjd", "cjsws", "cjswd", "cjswgs", "cjswgd", "pbs", "pbd", "pbsws", "pbswd", "pbswgs", "pbswgd", "mjs", "mjd", "mjsws", "mjswd",
    "mjswgs", "mjswgd", "sjs", "sjd", "sjsws", "sjswd", "sjswgs", "sjswgd", "mjs2", "mjd2", "mjsws2", "mjswd2", "mjswgs2", "mjswgd2", "jss", "jsd",
    "jsws", "jswd", "jswgs", "jswgd", "njs", "njd", "ijthsfwd", "ijthdfwd", "ijthsrev", "ijthdrev", "bvs", "bvd", "xjbvs", "xjbvd", "jtss", "jtsd",
    "jtssws", "jtsswd", "jtsswgs", "jtsswgd", "jtweff", "njts", "njtsd", "njtssw", "njtsswd", "njtsswg", "njtsswgd", "vtss", "vtsd", "vtssws", "vtsswd", "vtsswgs",
    "vtsswgd", "lintigen", "ntgen", "lntgen", "nntgen", "pntgen", "wntgen", "p2ntgen", "aigen", "laigen", "naigen", "paigen", "waigen", "p2aigen", "bigen", "lbigen",
    "nbigen", "pbigen", "wbigen", "p2bigen", "xrcrg1", "lxrcrg1", "nxrcrg1", "pxrcrg1", "wxrcrg1", "p2xrcrg1", "xrcrg2", "lxrcrg2", "nxrcrg2", "pxrcrg2", "wxrcrg2", "p2xrcrg2",
    "ef", "em", "noia", "noib", "noic", "k0noi", "k1noi", "lintnoi", "smooth", "noia2", "lnoia2", "nnoia2", "pnoia2", "wnoia2", "p2noia2", "mpower",
    "lmpower", "nmpower", "pmpower", "wmpower", "p2mpower", "qsref", "lqsref", "nqsref", "pqsref", "wqsref", "p2qsref", "ntnoi", "rnoia", "tnoia", "rnoib", "tnoib",
    "rnoic", "tnoic", "rnoik", "tnoik", "tnoik2", "tnom", "tbgasub", "tbgbsub", "kt1l", "tcj", "tcjsw", "tcjswg", "tpb", "tpbsw", "tpbswg", "xtis",
    "xtid", "xtss", "xtsd", "xtssws", "xtsswd", "xtsswgs", "xtsswgd", "tnjts", "tnjtsd", "tnjtssw", "tnjtsswd", "tnjtsswg", "tnjtsswgd", "kt1", "lkt1", "nkt1",
    "pkt1", "wkt1", "p2kt1", "kt11", "kt12", "tvth", "tss", "ltss", "ntss", "ptss", "wtss", "p2tss", "iit", "liit", "niit", "piit",
    "wiit", "p2iit", "tii", "ltii", "ntii", "ptii", "wtii", "p2tii", "tgidl", "ltgidl", "ntgidl", "ptgidl", "wtgidl", "p2tgidl", "ttat", "lttat",
    "nttat", "pttat", "wttat", "p2ttat", "igt", "ligt", "nigt", "pigt", "wigt", "p2igt", "tlow", "tlow1", "dtlow", "dtlow1", "klow1", "rth0",
    "cth0", "wth0", "ashexp", "bshexp", "cshexp", "ash", "csh", "ach_ufcm", "cins_ufcm", "w_ufcm", "tfin_top", "tfin_base", "qmfactorcv", "alpha_ufcm", "dim1h", "dimension1",
    "ldimension1", "ndimension1", "pdimension1", "wdimension1", "p2dimension1", "dim2h", "dimension2", "ldimension2", "ndimension2", "pdimension2", "wdimension2", "p2dimension2", "dim3h", "dimension3", "ldimension3", "ndimension3",
    "pdimension3", "wdimension3", "p2dimension3", "wdim0", "wdimr", "ssp1", "lssp1", "nssp1", "pssp1", "wssp1", "p2ssp1", "ssp2", "lssp2", "nssp2", "pssp2", "wssp2",
    "p2ssp2", "ssp3", "lssp3", "nssp3", "pssp3", "wssp3", "p2ssp3", "dssp1", "dssp2", "dssp3", "wssp0", "wsspr", "wgaanom", "e2nom", "le2nom", "ne2nom",
    "pe2nom", "we2nom", "p2e2nom", "e3nom", "le3nom", "ne3nom", "pe3nom", "we3nom", "p2e3nom", "mfe2", "mfe3", "wsfe2", "wsfe3", "mfq1nom", "lmfq1nom", "nmfq1nom",
    "pmfq1nom", "wmfq1nom", "p2mfq1nom", "mfq2nom", "lmfq2nom", "nmfq2nom", "pmfq2nom", "wmfq2nom", "p2mfq2nom", "mfq3nom", "lmfq3nom", "nmfq3nom", "pmfq3nom", "wmfq3nom", "p2mfq3nom", "mfq1",
    "mfq2", "mfq3", "wsfq1", "wsfq2", "wsfq3", "tsre2", "tdwse2", "tsre3", "tdwse3", "tsrq1", "tdwsq1", "tsrq2", "tdwsq2", "tsrq3", "tdwsq3", "nvsrd",
    "vsatrsd", "ptwgvsrsd", "ptwg1vsrsd", "psatxvsrsd", "mvsrsd", "nvsrs", "rdlcw", "rslcw", "prtvsrsd", "atvsrsd", "vsrdfactor", "vsrsfactor", "rdvds", "gavsrd",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1918] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1918] = [
    false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1918] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None,
    None, Some(ParameterBound { value: 2e25, label: "2e25" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e-5, label: "1e-5" }),
    None, None, Some(ParameterBound { value: 1e-5, label: "1e-5" }), None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.01, label: "0.01" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -273.15, label: "-273.15" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0001, label: "0.0001" }), Some(ParameterBound { value: 0.0001, label: "0.0001" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1918] = [
    None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 6.0, label: "6.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e27, label: "1e27" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1918] = [
    2, 2, 2, 2, 2, 3, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 0, 0, 3, 2, 2, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2,
    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 2, 3, 0, 2, 2, 2,
    3, 0, 2, 2, 3, 0, 2, 2, 2, 2, 2, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3,
    2, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 2, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0,
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
    0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3,
    3, 3, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 2, 2, 2, 3, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0, 0, 3, 3, 2, 0, 3, 3, 3, 2, 2, 0, 0, 0, 0, 0, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1918] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[],
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

#[derive(Clone)]
pub(crate) struct StructuredStaticState<const INSTANCE_VALUES: usize, const TEMPERATURE_VALUES: usize> {
    pub(crate) instance_values: [f64; INSTANCE_VALUES],
    pub(crate) temperature_values: [f64; TEMPERATURE_VALUES],
    pub(crate) instance_valid: bool,
    pub(crate) temperature_valid: bool,
    pub(crate) temperature: f64,
    pub(crate) thermal_voltage: f64,
}

impl<const INSTANCE_VALUES: usize, const TEMPERATURE_VALUES: usize> StructuredStaticState<INSTANCE_VALUES, TEMPERATURE_VALUES> {
    fn new_shared() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            instance_values: [0.0; INSTANCE_VALUES],
            temperature_values: [0.0; TEMPERATURE_VALUES],
            instance_valid: false,
            temperature_valid: false,
            temperature: 0.0,
            thermal_voltage: 0.0,
        })
    }
}

pub struct Instance {
    pub nodes: [usize; 17],
    pub branches: [usize; 18],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1918]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<28, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) structured_static: std::sync::Arc<StructuredStaticState<923, 0>>,
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
            structured_static: self.structured_static.clone(),
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 12;
    pub const NODE_COUNT: usize = 17;
    pub const INTERNAL_NODE_NAMES: [&str; 12] = ["di", "si", "di1", "si1", "di2", "ge", "gi", "gint", "gints", "gintd", "q", "n"];

    pub const BRANCH_COUNT: usize = 18;
    pub const PARAMETER_COUNT: usize = 1918;
    pub const VARIABLE_COUNT: usize = 1763;
    pub const DDT_STATE_COUNT: usize = 28;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "075f94558786b6c737f1f6aa62ec71f7e160917f679ed575eece7139f8982f24";
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
            structured_static: StructuredStaticState::new_shared(),
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(140);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(28);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 140);
        debug_assert_eq!(state.flags.len(), 28);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimcmg_va'", name));
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
            let cache = std::sync::Arc::make_mut(&mut self.structured_static);
            cache.instance_valid = false;
            cache.temperature_valid = false;
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
