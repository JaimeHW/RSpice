#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 332],
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
            const DEFAULTS_0: [f64; 42] = [
                5e-6, 5e-6, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 27.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 7000000.0, 9.025e-5, 1e-7, 1.1785, 0.0, 0.0,
                1e19, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 42);
            {
                let params = &mut *ptr;
                params[42] = params[41];
                validate_finite_parameter("XWDC", params[42]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 71] = [
                1e-6, 1e-6, 0.0, 0.0, 2.0, 0.0, -1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.1, 1e-8, 1e-8, 0.0,
                1e17, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.23,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0,
                0.0, 0.0, 1.0, 300.0, 30.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.3, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 2000000000000000.0, 2.0,
                0.0, 0.0, 1.0, 1.0, 1.5, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(43), 71);
            {
                let params = &mut *ptr;
                params[114] = if (params[33] > 0.0) { 2.0 } else { 1.0 };
                validate_parameter("BB", params[114], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 113] = [
                10.0, 50.0, 0.0, 10.0, 20.0, 0.0025, 1.0, 2e-6,
                0.8, 3e-8, 0.5, 0.0, 1.0, 0.8, 0.0, 1.0,
                0.0, 1.0, -1.0, 0.0, 1.0, 0.002, 1e-8, 1e-20,
                1.5, 0.55, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                5e17, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 3e-8, 0.7, 2.0, 1.0,
                1.0, 0.0, 0.01, 0.1, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 5e18, 0.0, 0.0,
                0.0, 5e-6, 1000000.0, 0.3, 0.0, 0.2, 1e-6, 0.0,
                10000.0, 20000000.0, 0.3, 0.0, 7500.0, 0.25, 1e-6, 1e-15,
                5000000.0, -5000000.0, 5e-16, 1.0, 0.0, 0.01, 0.005, 10000000000.0,
                1e-16, 0.0, 1.0, 27.0, 1e-10, 0.7, 8e-7, 3.5e-9,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(115), 113);
            {
                let params = &mut *ptr;
                params[228] = params[226];
                validate_parameter("TFOXGIDL", params[228], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 46] = [
                1e-8, 1e17, 1e18, 0.0, 1.0, 0.0, 1.0, 100000000000000.0,
                0.1, 1e-7, 0.0, 3.5, 0.0, 1.0, 0.0, 1.0,
                100.0, 0.0, 0.0, 0.0, 25000.0, 0.0, 2e-8, 1e-8,
                0.0, 3.0, 3.5, 1.0, 0.5, 0.0, 0.0, 1.0,
                1.0, 1.0, 1.0, 1e-11, 1.5e-11, 5e-16, 1.0, 0.0,
                1.0, 1.0, 1.0, 1e-11, 1.5e-11, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(229), 46);
            {
                let params = &mut *ptr;
                params[275] = params[94];
                validate_finite_parameter("MUEPH0B", params[275]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[276] = params[249];
                validate_finite_parameter("MUEPH1B", params[276]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[277] = params[95];
                validate_finite_parameter("MUEPHWB", params[277]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[278] = params[96];
                validate_finite_parameter("MUEPWPB", params[278]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[279] = params[99];
                validate_finite_parameter("MUEPHSB", params[279]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[280] = params[100];
                validate_finite_parameter("MUEPSPB", params[280]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[281] = params[97];
                validate_finite_parameter("MUEPHLB", params[281]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[282] = params[98];
                validate_finite_parameter("MUEPLPB", params[282]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[283] = params[106];
                validate_finite_parameter("MUESR0B", params[283]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[284] = params[105];
                validate_parameter("MUESR1B", params[284], false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[285] = params[107];
                validate_finite_parameter("MUESRLB", params[285]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[286] = params[110];
                validate_finite_parameter("MUESLPB", params[286]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[287] = params[108];
                validate_finite_parameter("MUESRWB", params[287]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[288] = params[109];
                validate_finite_parameter("MUESWPB", params[288]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[289] = params[86];
                validate_finite_parameter("MUECB0B", params[289]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[290] = params[87];
                validate_finite_parameter("MUECB1B", params[290]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[291] = params[88];
                validate_finite_parameter("MUECB0LPB", params[291]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[292] = params[89];
                validate_finite_parameter("MUECB1LPB", params[292]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[293] = params[90];
                validate_finite_parameter("MUECB0L2B", params[293]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[294] = params[91];
                validate_finite_parameter("MUECB0L2PB", params[294]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[295] = params[92];
                validate_finite_parameter("MUECB1L2B", params[295]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params[296] = params[93];
                validate_finite_parameter("MUECB1L2PB", params[296]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (*ptr).values.as_mut_ptr().add(297), 5);
            {
                let params = &mut *ptr;
                params[302] = params[299];
                validate_finite_parameter("MUEQBB", params[302]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 29] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1e-6,
                1e-6, 0.0, 0.0, 0.0, 1e19, 1000.0, 1000.0, 30000000.0,
                30000000.0, 0.0, 0.0, 1e-6, 1.0, 1.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (*ptr).values.as_mut_ptr().add(303), 29);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 335] = [
    ("l", 0), ("w", 1), ("ngcon", 2), ("xgw", 3), ("xgl", 4), ("nf", 5), ("sa", 6), ("sb", 7), ("sd", 8), ("temp", 9), ("dtemp", 10), ("sca", 11), ("scb", 12), ("scc", 13), ("coadov", 14), ("coisub", 15),
    ("cofbe", 16), ("coiigs", 17), ("cogidl", 18), ("coovlp", 19), ("coign", 20), ("coflick", 21), ("coisti", 22), ("cothrml", 23), ("conqs", 24), ("corg", 25), ("coievb", 26), ("cohist", 27), ("coselfheat", 28), ("covbsbiz", 29), ("coqovsm", 30), ("info", 31),
    ("conewmub", 32), ("type", 33), ("vmax", 34), ("bgtmp1", 35), ("bgtmp2", 36), ("eg0", 37), ("xld", 38), ("vfbover", 39), ("nover", 40), ("xwd", 41), ("xwdc", 42), ("saref", 43), ("sbref", 44), ("xqy", 45), ("xqy1", 46), ("xqy2", 47),
    ("rshg", 48), ("vfbc", 49), ("vfbcl1", 50), ("vfbcl1p", 51), ("vfbcl2", 52), ("vfbcl2p", 53), ("vfbhamp", 54), ("vbi", 55), ("parl1", 56), ("parl2", 57), ("lp", 58), ("nsubp", 59), ("nsubp0", 60), ("nsubwp", 61), ("wl1", 62), ("wl1p", 63),
    ("wl2", 64), ("wl2p", 65), ("scp1", 66), ("scp2", 67), ("scp3", 68), ("sc1", 69), ("sc2", 70), ("sc3", 71), ("scr1", 72), ("scr2", 73), ("scr3", 74), ("pgd1", 75), ("pgd2", 76), ("pgd4", 77), ("ndep", 78), ("ndepl", 79),
    ("ndeplp", 80), ("ninv", 81), ("ninvl", 82), ("ninvlp", 83), ("ninvd", 84), ("ninvdp", 85), ("muecb0", 86), ("muecb1", 87), ("muecb0lp", 88), ("muecb1lp", 89), ("muecb0l2", 90), ("muecb0l2p", 91), ("muecb1l2", 92), ("muecb1l2p", 93), ("mueph0", 94), ("muephw", 95),
    ("muepwp", 96), ("muephl", 97), ("mueplp", 98), ("muephs", 99), ("muepsp", 100), ("vtmp", 101), ("vtmpl", 102), ("vtmplp", 103), ("wvth0", 104), ("muesr1", 105), ("muesr0", 106), ("muesrl", 107), ("muesrw", 108), ("mueswp", 109), ("mueslp", 110), ("muetmp", 111),
    ("muetmpl", 112), ("muetmplp", 113), ("bb", 114), ("ddltmax", 115), ("ddltslp", 116), ("ddltict", 117), ("sub1", 118), ("sub2", 119), ("sub1l", 120), ("sub1lp", 121), ("sub2l", 122), ("svds", 123), ("slg", 124), ("svbs", 125), ("svbsl", 126), ("svbslp", 127),
    ("svgs", 128), ("svgsl", 129), ("svgslp", 130), ("svgsw", 131), ("svgswp", 132), ("vfbsub", 133), ("vfbsubl", 134), ("vfbsublp", 135), ("subdlt", 136), ("hist1", 137), ("hist2", 138), ("qhe1", 139), ("qhe2", 140), ("evb1", 141), ("evb2", 142), ("evb3", 143),
    ("fvbs", 144), ("ibpc1", 145), ("ibpc2", 146), ("nsti", 147), ("nstil", 148), ("nstilp", 149), ("nstiw", 150), ("nstiwp", 151), ("wsti", 152), ("ratwsti", 153), ("wstil", 154), ("wstilp", 155), ("wstiw", 156), ("wstiwp", 157), ("scsti1", 158), ("scsti2", 159),
    ("vthsti", 160), ("vdsti", 161), ("muesti1", 162), ("muesti2", 163), ("muesti3", 164), ("nsubpsti1", 165), ("nsubpsti2", 166), ("nsubpsti3", 167), ("nsubssti1", 168), ("nsubssti2", 169), ("nsubssti3", 170), ("tpoly", 171), ("cgbo", 172), ("cgdo", 173), ("cgso", 174), ("lover", 175),
    ("clm1", 176), ("clm2", 177), ("clm3", 178), ("clm5", 179), ("clm6", 180), ("vover", 181), ("voverp", 182), ("vovers", 183), ("voversp", 184), ("voverl", 185), ("voverlp", 186), ("voverw", 187), ("voverwp", 188), ("wfc", 189), ("nsubsw", 190), ("nsubcw", 190),
    ("nsubswp", 191), ("nsubcwp", 191), ("nsubsmax", 192), ("nsubcmax", 192), ("qme1", 193), ("qme2", 194), ("qme3", 195), ("gidl1", 196), ("gidl2", 197), ("gidl3", 198), ("gidl4", 199), ("gidl5", 200), ("gidlbpl1", 201), ("gidlbplt", 202), ("gleak1", 203), ("gleak2", 204),
    ("gleak3", 205), ("gleak4", 206), ("gleak5", 207), ("gleak6", 208), ("gleak7", 209), ("glksd1", 210), ("glksd2", 211), ("glksd3", 212), ("glkb1", 213), ("glkb2", 214), ("glkb3", 215), ("vzadd0", 216), ("pzadd0", 217), ("nftrp", 218), ("nfalp", 219), ("cit", 220),
    ("falph", 221), ("tnom", 222), ("dly1", 223), ("dly2", 224), ("dly3", 225), ("tfox", 226), ("tsoi", 227), ("tfoxgidl", 228), ("tbox", 229), ("nsubs", 230), ("nsubb", 231), ("nsubbl", 232), ("nsubblp", 233), ("nsubbw", 234), ("nsubbwp", 235), ("nsubbmin", 236),
    ("rth0", 237), ("cth0", 238), ("ptl", 239), ("ptp", 240), ("pt2", 241), ("ptlp", 242), ("pt4", 243), ("pt4p", 244), ("ptdlt", 245), ("gdl", 246), ("gdlp", 247), ("gdld", 248), ("mueph1", 249), ("sc5", 250), ("xldl", 251), ("xldlmin", 252),
    ("muetmp1", 253), ("vbsbnd", 254), ("vbsmax", 255), ("gleak8", 256), ("gleak9", 257), ("gleak10", 258), ("glksd4", 259), ("glksd5", 260), ("glkb4", 261), ("glkb5", 262), ("glkb6", 263), ("glkb7", 264), ("glkb8", 265), ("glkb21", 266), ("glkb22", 267), ("glkb23", 268),
    ("glkb24", 269), ("glkb25", 270), ("glkb26", 271), ("glkb27", 272), ("glkb28", 273), ("ptmueph", 274), ("mueph0b", 275), ("mueph1b", 276), ("muephwb", 277), ("muepwpb", 278), ("muephsb", 279), ("muepspb", 280), ("muephlb", 281), ("mueplpb", 282), ("muesr0b", 283), ("muesr1b", 284),
    ("muesrlb", 285), ("mueslpb", 286), ("muesrwb", 287), ("mueswpb", 288), ("muecb0b", 289), ("muecb1b", 290), ("muecb0lpb", 291), ("muecb1lpb", 292), ("muecb0l2b", 293), ("muecb0l2pb", 294), ("muecb1l2b", 295), ("muecb1l2pb", 296), ("pthrou", 297), ("vfbshift", 298), ("mueqb", 299), ("mueqbl", 300),
    ("mueqblp", 301), ("mueqbb", 302), ("cocinv", 303), ("web", 304), ("wec", 305), ("nsubswpe", 306), ("nsubpwpe", 307), ("nrs", 308), ("nrd", 309), ("ldrift", 310), ("ldrifts", 311), ("cors", 312), ("cord", 313), ("rsh", 314), ("novers", 315), ("rdrmued", 316),
    ("rdrmues", 317), ("rdrvmaxd", 318), ("rdrvmaxs", 319), ("rdrmuetmp", 320), ("rdrvtmp", 321), ("rdrdjunc", 322), ("rdrbbd", 323), ("rdrbbs", 324), ("rdrbbtmp", 325), ("rdrvmaxw", 326), ("rdrvmaxwp", 327), ("rdrvmaxl", 328), ("rdrvmaxlp", 329), ("rdrmuel", 330), ("rdrmuelp", 331),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 332] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 332] = [
    None, None, None, None, Some(0), None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, Some(0), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 332] = [
    "L", "W", "NGCON", "XGW", "XGL", "NF", "SA", "SB", "SD", "TEMP", "DTEMP", "SCA", "SCB", "SCC", "COADOV", "COISUB",
    "COFBE", "COIIGS", "COGIDL", "COOVLP", "COIGN", "COFLICK", "COISTI", "COTHRML", "CONQS", "CORG", "COIEVB", "COHIST", "COSELFHEAT", "COVBSBIZ", "COQOVSM", "INFO",
    "CONEWMUB", "TYPE", "VMAX", "BGTMP1", "BGTMP2", "EG0", "XLD", "VFBOVER", "NOVER", "XWD", "XWDC", "SAREF", "SBREF", "XQY", "XQY1", "XQY2",
    "RSHG", "VFBC", "VFBCL1", "VFBCL1P", "VFBCL2", "VFBCL2P", "VFBHAMP", "VBI", "PARL1", "PARL2", "LP", "NSUBP", "NSUBP0", "NSUBWP", "WL1", "WL1P",
    "WL2", "WL2P", "SCP1", "SCP2", "SCP3", "SC1", "SC2", "SC3", "SCR1", "SCR2", "SCR3", "PGD1", "PGD2", "PGD4", "NDEP", "NDEPL",
    "NDEPLP", "NINV", "NINVL", "NINVLP", "NINVD", "NINVDP", "MUECB0", "MUECB1", "MUECB0LP", "MUECB1LP", "MUECB0L2", "MUECB0L2P", "MUECB1L2", "MUECB1L2P", "MUEPH0", "MUEPHW",
    "MUEPWP", "MUEPHL", "MUEPLP", "MUEPHS", "MUEPSP", "VTMP", "VTMPL", "VTMPLP", "WVTH0", "MUESR1", "MUESR0", "MUESRL", "MUESRW", "MUESWP", "MUESLP", "MUETMP",
    "MUETMPL", "MUETMPLP", "BB", "DDLTMAX", "DDLTSLP", "DDLTICT", "SUB1", "SUB2", "SUB1L", "SUB1LP", "SUB2L", "SVDS", "SLG", "SVBS", "SVBSL", "SVBSLP",
    "SVGS", "SVGSL", "SVGSLP", "SVGSW", "SVGSWP", "VFBSUB", "VFBSUBL", "VFBSUBLP", "SUBDLT", "HIST1", "HIST2", "QHE1", "QHE2", "EVB1", "EVB2", "EVB3",
    "FVBS", "IBPC1", "IBPC2", "NSTI", "NSTIL", "NSTILP", "NSTIW", "NSTIWP", "WSTI", "RATWSTI", "WSTIL", "WSTILP", "WSTIW", "WSTIWP", "SCSTI1", "SCSTI2",
    "VTHSTI", "VDSTI", "MUESTI1", "MUESTI2", "MUESTI3", "NSUBPSTI1", "NSUBPSTI2", "NSUBPSTI3", "NSUBSSTI1", "NSUBSSTI2", "NSUBSSTI3", "TPOLY", "CGBO", "CGDO", "CGSO", "LOVER",
    "CLM1", "CLM2", "CLM3", "CLM5", "CLM6", "VOVER", "VOVERP", "VOVERS", "VOVERSP", "VOVERL", "VOVERLP", "VOVERW", "VOVERWP", "WFC", "NSUBSW", "NSUBSWP",
    "NSUBSMAX", "QME1", "QME2", "QME3", "GIDL1", "GIDL2", "GIDL3", "GIDL4", "GIDL5", "GIDLBPL1", "GIDLBPLT", "GLEAK1", "GLEAK2", "GLEAK3", "GLEAK4", "GLEAK5",
    "GLEAK6", "GLEAK7", "GLKSD1", "GLKSD2", "GLKSD3", "GLKB1", "GLKB2", "GLKB3", "VZADD0", "PZADD0", "NFTRP", "NFALP", "CIT", "FALPH", "TNOM", "DLY1",
    "DLY2", "DLY3", "TFOX", "TSOI", "TFOXGIDL", "TBOX", "NSUBS", "NSUBB", "NSUBBL", "NSUBBLP", "NSUBBW", "NSUBBWP", "NSUBBMIN", "RTH0", "CTH0", "PTL",
    "PTP", "PT2", "PTLP", "PT4", "PT4P", "PTDLT", "GDL", "GDLP", "GDLD", "MUEPH1", "SC5", "XLDL", "XLDLMIN", "MUETMP1", "VBSBND", "VBSMAX",
    "GLEAK8", "GLEAK9", "GLEAK10", "GLKSD4", "GLKSD5", "GLKB4", "GLKB5", "GLKB6", "GLKB7", "GLKB8", "GLKB21", "GLKB22", "GLKB23", "GLKB24", "GLKB25", "GLKB26",
    "GLKB27", "GLKB28", "PTMUEPH", "MUEPH0B", "MUEPH1B", "MUEPHWB", "MUEPWPB", "MUEPHSB", "MUEPSPB", "MUEPHLB", "MUEPLPB", "MUESR0B", "MUESR1B", "MUESRLB", "MUESLPB", "MUESRWB",
    "MUESWPB", "MUECB0B", "MUECB1B", "MUECB0LPB", "MUECB1LPB", "MUECB0L2B", "MUECB0L2PB", "MUECB1L2B", "MUECB1L2PB", "PTHROU", "VFBSHIFT", "MUEQB", "MUEQBL", "MUEQBLP", "MUEQBB", "COCINV",
    "WEB", "WEC", "NSUBSWPE", "NSUBPWPE", "NRS", "NRD", "LDRIFT", "LDRIFTS", "CORS", "CORD", "RSH", "NOVERS", "RDRMUED", "RDRMUES", "RDRVMAXD", "RDRVMAXS",
    "RDRMUETMP", "RDRVTMP", "RDRDJUNC", "RDRBBD", "RDRBBS", "RDRBBTMP", "RDRVMAXW", "RDRVMAXWP", "RDRVMAXL", "RDRVMAXLP", "RDRMUEL", "RDRMUELP",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 332] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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

const PARAMETER_INTEGER_FLAGS: [bool; 332] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 332] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 22.0, label: "22.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None,
    None, None, None, None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 332] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 32.0, label: "32.0" }), None,
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
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 332] = [
    3, 3, 3, 0, 2, 3, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 3, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 3, 0, 2, 3, 0, 2, 3, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 2, 0, 3, 3, 3, 3,
    0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 332] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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
        4 => Some(ParameterBound { value: (-params[0]), label: "computed lower-bound expression" }),
        56 => Some(ParameterBound { value: ((-params[0]) / 2.0), label: "computed lower-bound expression" }),
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
    pub branches: [usize; 8],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 332]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<9, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) canonical_reactive: Box<[f64; 50]>,
    pub(crate) canonical_staged: Box<[f64; 203]>,
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
    pub const INTERNAL_NODE_NAMES: [&str; 9] = ["t", "gp", "bp", "n", "nqs_qi", "nqs_qb", "nqs_qhs", "dp", "sp"];

    pub const BRANCH_COUNT: usize = 8;
    pub const PARAMETER_COUNT: usize = 332;
    pub const VARIABLE_COUNT: usize = 1096;
    pub const DDT_STATE_COUNT: usize = 9;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "a3de17f51d0244404c6dc8e7193f2190278027fb5ed5e8a649c8257e528d5f16";
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
        let mut values = Vec::with_capacity(45);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(9);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 45);
        debug_assert_eq!(state.flags.len(), 9);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimsotb_va'", name));
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
