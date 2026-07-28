#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 503],
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
            const DEFAULTS_0: [f64; 33] = [
                0.0, 102.8, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 21.0,
                150.0, 1.0, 0.0, 0.001, 1e-6, 1e-6, 1e-12, 1e-12,
                1e-6, 1e-6, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 33);
            {
                let params = &mut *ptr;
                params[33] = params[31];
                validate_parameter("MULT_FN", params[33], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 469] = [
                0.0, 1.0, 0.0, 1.0, 1e-7, 0.0, 0.0, 2e-9,
                1e-8, 0.0, 1e-7, 0.0, 3e18, 0.0, 2e-9, 1e20,
                1e20, 0.0, 0.0, 0.0, 1e21, 1.0, 1.0, 0.0,
                1.0, 1e22, 0.0, 0.0, 0.0, 1.0, 0.0, 0.2,
                0.0, 0.0, 0.05, 1.0, 1.5, 0.0, 0.0, 0.0,
                0.0, 1.5, 0.0, 2.0, 1.0, 0.0, 0.0, 1.5,
                0.0, 0.0, 1.0, 0.0, 1.0, 30.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, -0.1, 0.0, 0.0, 8.0,
                0.0, 0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.375, 0.063, 0.375, 0.063,
                0.375, 0.063, 0.0, 1.0, 3.1, 0.0, 0.0, 0.0,
                0.2, 0.0, 0.0, 0.0, 41.0, 41.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.2, 0.05,
                1.5, 1.0, 10.0, 0.0, 1.0, 1e-12, 0.0, 1e22,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.04e-18, 0.0, 10000.0, 0.0, 1e-11, 1.0, 0.0, 8e22,
                30000000.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0,
                1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-9,
                1e-8, 0.0, 1e-7, 0.0, 3e18, 0.0, 2e-9, 1e20,
                1e20, 0.0, 0.0, 2.0, 0.0, 2.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e21, 0.0,
                1.0, 1.0, 0.0, 2.0, 0.0, 1.0, 1e22, 0.0,
                0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 0.2, 0.0,
                0.0, 0.0, 0.05, 0.0, 0.0, 1e-8, 0.0, 0.0,
                1e-8, 0.0, 0.0, 1e-8, 1.0, 1.5, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 2.0, 1.0,
                0.0, 0.0, 1.5, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 30.0, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                -0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0,
                1.0, 0.0, 1.5, 0.0, 1.0, 0.0, 2.0, 0.0,
                0.0, 0.5, 0.0, 1.5, 0.0, 0.0, 0.05, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.375,
                0.063, 0.375, 0.063, 0.375, 0.063, 0.0, 1.0, 3.1,
                0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0,
                0.0, 41.0, 41.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1e-8, 0.0, 0.0, 0.0, 0.0,
                2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 2.0, 0.0, 1.0, 0.0, 2.0,
                0.0, 1.0, 0.2, 0.0, 1e-8, 0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 10.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 1e22, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 8.0, 0.0, 1.0, 0.0, 1.5,
                0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 100000.0, 1.5, 3.0, 4.5, 0.0, 1e-12, 1e-7,
                0.0, 1.0, 0.0, 2.0, 8e22, 0.0, 30000000.0, 0.0,
                0.0, 0.0, 1.0, 1.0, 1e-6, 1e-6, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1e-7,
                3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0, 1e-15, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(34), 469);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 504] = [
    ("swscale", 0), ("version", 1), ("swsubdep", 2), ("swigate", 3), ("swgidl", 4), ("swshe", 5), ("swign", 6), ("swjunasym", 7), ("swimpact", 8), ("swpdep", 9), ("swcryo", 10), ("swqmod", 11), ("swedge", 12), ("qmc", 13), ("type", 14), ("tr", 15),
    ("tref", 15), ("tmax", 16), ("tmin", 17), ("atmin", 18), ("btmin", 19), ("l", 20), ("w", 21), ("asource", 22), ("adrain", 23), ("psource", 24), ("pdrain", 25), ("sa", 26), ("sb", 27), ("sd", 28), ("nf", 29), ("mult", 30),
    ("mult_i", 31), ("mult_q", 32), ("mult_fn", 33), ("delvto", 34), ("factuo", 35), ("dtemp", 36), ("ngcon", 37), ("xgw", 38), ("nrs", 39), ("nrd", 40), ("toxe", 41), ("tsi", 42), ("xge", 43), ("tbox", 44), ("nch", 45), ("nsub", 46),
    ("ct", 47), ("toxp", 48), ("nov", 49), ("novd", 50), ("vfb", 51), ("vfbb", 52), ("stvfb", 53), ("np", 54), ("cicf", 55), ("cic", 56), ("psce", 57), ("psceb", 58), ("nsddc", 59), ("pscedlb", 60), ("pnce", 61), ("cf", 62),
    ("cfb", 63), ("stcf", 64), ("cfd", 65), ("cfdl", 66), ("cfdlb", 67), ("betn", 68), ("betnb", 69), ("stbet", 70), ("cs", 71), ("csfi", 72), ("csbi", 73), ("stcs", 74), ("thecs", 75), ("stthecs", 76), ("csthr", 77), ("csthrb", 78),
    ("mue", 79), ("stmue", 80), ("themu", 81), ("stthemu", 82), ("xcor", 83), ("xcorb", 84), ("stxcor", 85), ("feta", 86), ("rs", 87), ("rsig", 88), ("strs", 89), ("rsg", 90), ("thersg", 91), ("rsb", 92), ("thesat", 93), ("stthesat", 94),
    ("thesatg", 95), ("thesatb", 96), ("ax", 97), ("alp", 98), ("alp1", 99), ("alpb", 100), ("vp", 101), ("vpg", 102), ("gco", 103), ("iginv", 104), ("igovinv", 105), ("igovinvd", 106), ("igovacc", 107), ("igovaccd", 108), ("stig", 109), ("gc2ch", 110),
    ("gc3ch", 111), ("gc2ovinv", 112), ("gc3ovinv", 113), ("gc2ovacc", 114), ("gc3ovacc", 115), ("gcdov", 116), ("gcvdov", 117), ("chib", 118), ("niginv", 119), ("fnovinv", 120), ("fnovinvd", 121), ("gcovinvfn", 122), ("stigfn", 123), ("agidl", 124), ("agidld", 125), ("bgidl", 126),
    ("bgidld", 127), ("stbgidl", 128), ("stbgidld", 129), ("cgidl", 130), ("cgidld", 131), ("dgidl", 132), ("dgidld", 133), ("ctedge", 134), ("vfbedge", 135), ("vfbbedge", 136), ("stvfbedge", 137), ("cicfedge", 138), ("cicedge", 139), ("psceedge", 140), ("pscebedge", 141), ("cfedge", 142),
    ("cfbedge", 143), ("cfdedge", 144), ("betnedge", 145), ("stbetedge", 146), ("a1", 147), ("a2", 148), ("sta2", 149), ("a3", 150), ("areaq", 151), ("cgbov", 152), ("nsdac", 153), ("fif", 154), ("fsceac", 155), ("vfbac", 156), ("vfbbac", 157), ("psceac", 158),
    ("cfac", 159), ("thesatac", 160), ("axac", 161), ("alpac", 162), ("cov", 163), ("covd", 164), ("covdl", 165), ("covdlb", 166), ("dvfbov", 167), ("cfr", 168), ("cfrd", 169), ("csd", 170), ("csdbp", 171), ("rth", 172), ("strth", 173), ("cth", 174),
    ("fnt", 175), ("fntexc", 176), ("nfa", 177), ("nfb", 178), ("nfc", 179), ("nfe", 180), ("nfeb", 181), ("ef", 182), ("kdrift", 183), ("kdiff", 184), ("fracinv", 185), ("kfracinv", 186), ("rg", 187), ("rse", 188), ("rde", 189), ("rwell", 190),
    ("lvaro", 191), ("lvarl", 192), ("lvarw", 193), ("lap", 194), ("wvaro", 195), ("wvarl", 196), ("wvarw", 197), ("wot", 198), ("dlq", 199), ("dwq", 200), ("toxeo", 201), ("tsio", 202), ("xgeo", 203), ("tboxo", 204), ("ncho", 205), ("nsubo", 206),
    ("cto", 207), ("toxpo", 208), ("novo", 209), ("novdo", 210), ("vfbo", 211), ("vfbl", 212), ("vfblexp", 213), ("vfbl2", 214), ("vfblexp2", 215), ("vfbw", 216), ("vfblw", 217), ("vfbbo", 218), ("vfblbo", 219), ("stvfbo", 220), ("stvfbl", 221), ("stvfbw", 222),
    ("stvfblw", 223), ("npo", 224), ("npl", 225), ("cicfo", 226), ("cico", 227), ("pscel", 228), ("pscelexp", 229), ("pscew", 230), ("pscebo", 231), ("nsddco", 232), ("pscedlbo", 233), ("pncew", 234), ("cfl", 235), ("cflexp", 236), ("cfw", 237), ("cfbo", 238),
    ("stcfl", 239), ("cfdo", 240), ("cfdll", 241), ("cfdlw", 242), ("cfdlbo", 243), ("uo", 244), ("fbet1", 245), ("fbet1w", 246), ("lp1", 247), ("lp1w", 248), ("fbet2", 249), ("lp2", 250), ("betw1", 251), ("betw2", 252), ("wbet", 253), ("betnbo", 254),
    ("stbeto", 255), ("stbetl", 256), ("stbetw", 257), ("stbetlw", 258), ("cso", 259), ("csl", 260), ("cslexp", 261), ("csw", 262), ("cslw", 263), ("csfio", 264), ("csbio", 265), ("stcso", 266), ("stcsl", 267), ("stcsw", 268), ("stcslw", 269), ("thecso", 270),
    ("stthecso", 271), ("csthro", 272), ("csthrbo", 273), ("mueo", 274), ("stmueo", 275), ("themuo", 276), ("stthemuo", 277), ("xcoro", 278), ("xcorl", 279), ("xcorlexp", 280), ("xcorw", 281), ("xcorlw", 282), ("xcorbo", 283), ("stxcoro", 284), ("fetao", 285), ("rsw1", 286),
    ("rsw2", 287), ("rsigo", 288), ("strso", 289), ("rsgo", 290), ("thersgo", 291), ("rsbo", 292), ("thesato", 293), ("thesatl", 294), ("thesatlexp", 295), ("thesatw", 296), ("thesatlw", 297), ("stthesato", 298), ("stthesatl", 299), ("stthesatw", 300), ("stthesatlw", 301), ("thesatgo", 302),
    ("thesatbo", 303), ("axo", 304), ("axl", 305), ("axlexp", 306), ("axl2", 307), ("axlexp2", 308), ("alpl1", 309), ("alplexp", 310), ("alpl2", 311), ("alplexp2", 312), ("alpw", 313), ("alp1l1", 314), ("alp1lexp", 315), ("alp1l2", 316), ("alp1lexp2", 317), ("alp1w", 318),
    ("alpbo", 319), ("vpo", 320), ("vpgo", 321), ("gcoo", 322), ("iginvlw", 323), ("igovinvw", 324), ("igovinvdw", 325), ("igovaccw", 326), ("igovaccdw", 327), ("stigo", 328), ("gc2cho", 329), ("gc3cho", 330), ("gc2ovinvo", 331), ("gc3ovinvo", 332), ("gc2ovacco", 333), ("gc3ovacco", 334),
    ("gcdovl", 335), ("gcvdovo", 336), ("chibo", 337), ("niginvo", 338), ("fnovinvw", 339), ("fnovinvdw", 340), ("gcovinvfno", 341), ("stigfno", 342), ("agidlo", 343), ("agidldo", 344), ("agidlw", 345), ("agidldw", 346), ("bgidlo", 347), ("bgidldo", 348), ("stbgidlo", 349), ("stbgidldo", 350),
    ("cgidlo", 351), ("cgidldo", 352), ("dgidlo", 353), ("dgidldo", 354), ("dgidll", 355), ("dgidldl", 356), ("wedge", 357), ("wedgew", 358), ("ctedgeo", 359), ("vfbedgeo", 360), ("vfbedgel", 361), ("vfbedgelexp", 362), ("vfbedgew", 363), ("vfbedgelw", 364), ("vfbbedgeo", 365), ("stvfbedgeo", 366),
    ("stvfbedgel", 367), ("stvfbedgew", 368), ("stvfbedgelw", 369), ("cicfedgeo", 370), ("cicedgeo", 371), ("psceedgel", 372), ("psceedgelexp", 373), ("psceedgew", 374), ("pscebedgeo", 375), ("cfedgel", 376), ("cfedgelexp", 377), ("cfedgew", 378), ("cfbedgeo", 379), ("cfdedgeo", 380), ("fbetedge", 381), ("lpedge", 382),
    ("betedgew", 383), ("stbetedgeo", 384), ("stbetedgel", 385), ("stbetedgew", 386), ("stbetedgelw", 387), ("a1o", 388), ("a1l", 389), ("a1w", 390), ("a2o", 391), ("sta2o", 392), ("a3o", 393), ("a3l", 394), ("a3w", 395), ("cgbovo", 396), ("cgbovl", 397), ("nsdaco", 398),
    ("fifw", 399), ("fsceaco", 400), ("vfbaco", 401), ("vfbacl", 402), ("vfbaclexp", 403), ("vfbacl2", 404), ("vfbaclexp2", 405), ("vfbacw", 406), ("vfbaclw", 407), ("vfbbaco", 408), ("vfblbaco", 409), ("psceacl", 410), ("psceaclexp", 411), ("psceacw", 412), ("cfacl", 413), ("cfaclexp", 414),
    ("cfacw", 415), ("thesataco", 416), ("thesatacl", 417), ("thesataclexp", 418), ("thesatacw", 419), ("thesataclw", 420), ("axaco", 421), ("axacl", 422), ("axaclexp", 423), ("axacl2", 424), ("axaclexp2", 425), ("alpacl1", 426), ("alpaclexp", 427), ("alpacl2", 428), ("alpaclexp2", 429), ("alpacw", 430),
    ("lovo", 431), ("lovdo", 432), ("covdlo", 433), ("covdlw", 434), ("covdlbo", 435), ("dvfbovo", 436), ("cfro", 437), ("cfrdo", 438), ("cfrw", 439), ("cfrdw", 440), ("csdo", 441), ("csdbpo", 442), ("rtho", 443), ("rthl", 444), ("rthw", 445), ("rthlw", 446),
    ("strtho", 447), ("ctho", 448), ("lambtho", 449), ("ftho", 450), ("fnto", 451), ("fntexcl", 452), ("fntexclexp", 453), ("nfalw", 454), ("nfaw", 455), ("nfblw", 456), ("nfclw", 457), ("nfeo", 458), ("nfebo", 459), ("efo", 460), ("swstress", 461), ("saref", 462),
    ("sbref", 463), ("wlod", 464), ("kuo", 465), ("kvsat", 466), ("tkuo", 467), ("lkuo", 468), ("wkuo", 469), ("pkuo", 470), ("llodkuo", 471), ("wlodkuo", 472), ("kvtho", 473), ("lkvtho", 474), ("wkvtho", 475), ("pkvtho", 476), ("llodvth", 477), ("wlodvth", 478),
    ("stetao", 479), ("lodetao", 480), ("strlambda", 481), ("stralpha", 482), ("strdvfbo", 483), ("strwdvfbo", 484), ("strdcfl", 485), ("strruo", 486), ("strtruo", 487), ("strrvsat", 488), ("kdrifto", 489), ("kdriftl", 490), ("kdiffo", 491), ("kdiffl", 492), ("fracinvo", 493), ("kfracinvo", 494),
    ("rgo", 495), ("rint", 496), ("rvpoly", 497), ("rshg", 498), ("dlsil", 499), ("rsh", 500), ("rshd", 501), ("rwello", 502),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 503] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 503] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 503] = [
    "SWSCALE", "VERSION", "SWSUBDEP", "SWIGATE", "SWGIDL", "SWSHE", "SWIGN", "SWJUNASYM", "SWIMPACT", "SWPDEP", "SWCRYO", "SWQMOD", "SWEDGE", "QMC", "TYPE", "TR",
    "TMAX", "TMIN", "ATMIN", "BTMIN", "L", "W", "ASOURCE", "ADRAIN", "PSOURCE", "PDRAIN", "SA", "SB", "SD", "NF", "MULT", "MULT_I",
    "MULT_Q", "MULT_FN", "DELVTO", "FACTUO", "DTEMP", "NGCON", "XGW", "NRS", "NRD", "TOXE", "TSI", "XGE", "TBOX", "NCH", "NSUB", "CT",
    "TOXP", "NOV", "NOVD", "VFB", "VFBB", "STVFB", "NP", "CICF", "CIC", "PSCE", "PSCEB", "NSDDC", "PSCEDLB", "PNCE", "CF", "CFB",
    "STCF", "CFD", "CFDL", "CFDLB", "BETN", "BETNB", "STBET", "CS", "CSFI", "CSBI", "STCS", "THECS", "STTHECS", "CSTHR", "CSTHRB", "MUE",
    "STMUE", "THEMU", "STTHEMU", "XCOR", "XCORB", "STXCOR", "FETA", "RS", "RSIG", "STRS", "RSG", "THERSG", "RSB", "THESAT", "STTHESAT", "THESATG",
    "THESATB", "AX", "ALP", "ALP1", "ALPB", "VP", "VPG", "GCO", "IGINV", "IGOVINV", "IGOVINVD", "IGOVACC", "IGOVACCD", "STIG", "GC2CH", "GC3CH",
    "GC2OVINV", "GC3OVINV", "GC2OVACC", "GC3OVACC", "GCDOV", "GCVDOV", "CHIB", "NIGINV", "FNOVINV", "FNOVINVD", "GCOVINVFN", "STIGFN", "AGIDL", "AGIDLD", "BGIDL", "BGIDLD",
    "STBGIDL", "STBGIDLD", "CGIDL", "CGIDLD", "DGIDL", "DGIDLD", "CTEDGE", "VFBEDGE", "VFBBEDGE", "STVFBEDGE", "CICFEDGE", "CICEDGE", "PSCEEDGE", "PSCEBEDGE", "CFEDGE", "CFBEDGE",
    "CFDEDGE", "BETNEDGE", "STBETEDGE", "A1", "A2", "STA2", "A3", "AREAQ", "CGBOV", "NSDAC", "FIF", "FSCEAC", "VFBAC", "VFBBAC", "PSCEAC", "CFAC",
    "THESATAC", "AXAC", "ALPAC", "COV", "COVD", "COVDL", "COVDLB", "DVFBOV", "CFR", "CFRD", "CSD", "CSDBP", "RTH", "STRTH", "CTH", "FNT",
    "FNTEXC", "NFA", "NFB", "NFC", "NFE", "NFEB", "EF", "KDRIFT", "KDIFF", "FRACINV", "KFRACINV", "RG", "RSE", "RDE", "RWELL", "LVARO",
    "LVARL", "LVARW", "LAP", "WVARO", "WVARL", "WVARW", "WOT", "DLQ", "DWQ", "TOXEO", "TSIO", "XGEO", "TBOXO", "NCHO", "NSUBO", "CTO",
    "TOXPO", "NOVO", "NOVDO", "VFBO", "VFBL", "VFBLEXP", "VFBL2", "VFBLEXP2", "VFBW", "VFBLW", "VFBBO", "VFBLBO", "STVFBO", "STVFBL", "STVFBW", "STVFBLW",
    "NPO", "NPL", "CICFO", "CICO", "PSCEL", "PSCELEXP", "PSCEW", "PSCEBO", "NSDDCO", "PSCEDLBO", "PNCEW", "CFL", "CFLEXP", "CFW", "CFBO", "STCFL",
    "CFDO", "CFDLL", "CFDLW", "CFDLBO", "UO", "FBET1", "FBET1W", "LP1", "LP1W", "FBET2", "LP2", "BETW1", "BETW2", "WBET", "BETNBO", "STBETO",
    "STBETL", "STBETW", "STBETLW", "CSO", "CSL", "CSLEXP", "CSW", "CSLW", "CSFIO", "CSBIO", "STCSO", "STCSL", "STCSW", "STCSLW", "THECSO", "STTHECSO",
    "CSTHRO", "CSTHRBO", "MUEO", "STMUEO", "THEMUO", "STTHEMUO", "XCORO", "XCORL", "XCORLEXP", "XCORW", "XCORLW", "XCORBO", "STXCORO", "FETAO", "RSW1", "RSW2",
    "RSIGO", "STRSO", "RSGO", "THERSGO", "RSBO", "THESATO", "THESATL", "THESATLEXP", "THESATW", "THESATLW", "STTHESATO", "STTHESATL", "STTHESATW", "STTHESATLW", "THESATGO", "THESATBO",
    "AXO", "AXL", "AXLEXP", "AXL2", "AXLEXP2", "ALPL1", "ALPLEXP", "ALPL2", "ALPLEXP2", "ALPW", "ALP1L1", "ALP1LEXP", "ALP1L2", "ALP1LEXP2", "ALP1W", "ALPBO",
    "VPO", "VPGO", "GCOO", "IGINVLW", "IGOVINVW", "IGOVINVDW", "IGOVACCW", "IGOVACCDW", "STIGO", "GC2CHO", "GC3CHO", "GC2OVINVO", "GC3OVINVO", "GC2OVACCO", "GC3OVACCO", "GCDOVL",
    "GCVDOVO", "CHIBO", "NIGINVO", "FNOVINVW", "FNOVINVDW", "GCOVINVFNO", "STIGFNO", "AGIDLO", "AGIDLDO", "AGIDLW", "AGIDLDW", "BGIDLO", "BGIDLDO", "STBGIDLO", "STBGIDLDO", "CGIDLO",
    "CGIDLDO", "DGIDLO", "DGIDLDO", "DGIDLL", "DGIDLDL", "WEDGE", "WEDGEW", "CTEDGEO", "VFBEDGEO", "VFBEDGEL", "VFBEDGELEXP", "VFBEDGEW", "VFBEDGELW", "VFBBEDGEO", "STVFBEDGEO", "STVFBEDGEL",
    "STVFBEDGEW", "STVFBEDGELW", "CICFEDGEO", "CICEDGEO", "PSCEEDGEL", "PSCEEDGELEXP", "PSCEEDGEW", "PSCEBEDGEO", "CFEDGEL", "CFEDGELEXP", "CFEDGEW", "CFBEDGEO", "CFDEDGEO", "FBETEDGE", "LPEDGE", "BETEDGEW",
    "STBETEDGEO", "STBETEDGEL", "STBETEDGEW", "STBETEDGELW", "A1O", "A1L", "A1W", "A2O", "STA2O", "A3O", "A3L", "A3W", "CGBOVO", "CGBOVL", "NSDACO", "FIFW",
    "FSCEACO", "VFBACO", "VFBACL", "VFBACLEXP", "VFBACL2", "VFBACLEXP2", "VFBACW", "VFBACLW", "VFBBACO", "VFBLBACO", "PSCEACL", "PSCEACLEXP", "PSCEACW", "CFACL", "CFACLEXP", "CFACW",
    "THESATACO", "THESATACL", "THESATACLEXP", "THESATACW", "THESATACLW", "AXACO", "AXACL", "AXACLEXP", "AXACL2", "AXACLEXP2", "ALPACL1", "ALPACLEXP", "ALPACL2", "ALPACLEXP2", "ALPACW", "LOVO",
    "LOVDO", "COVDLO", "COVDLW", "COVDLBO", "DVFBOVO", "CFRO", "CFRDO", "CFRW", "CFRDW", "CSDO", "CSDBPO", "RTHO", "RTHL", "RTHW", "RTHLW", "STRTHO",
    "CTHO", "LAMBTHO", "FTHO", "FNTO", "FNTEXCL", "FNTEXCLEXP", "NFALW", "NFAW", "NFBLW", "NFCLW", "NFEO", "NFEBO", "EFO", "SWSTRESS", "SAREF", "SBREF",
    "WLOD", "KUO", "KVSAT", "TKUO", "LKUO", "WKUO", "PKUO", "LLODKUO", "WLODKUO", "KVTHO", "LKVTHO", "WKVTHO", "PKVTHO", "LLODVTH", "WLODVTH", "STETAO",
    "LODETAO", "STRLAMBDA", "STRALPHA", "STRDVFBO", "STRWDVFBO", "STRDCFL", "STRRUO", "STRTRUO", "STRRVSAT", "KDRIFTO", "KDRIFTL", "KDIFFO", "KDIFFL", "FRACINVO", "KFRACINVO", "RGO",
    "RINT", "RVPOLY", "RSHG", "DLSIL", "RSH", "RSHD", "RWELLO",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 503] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 503] = [
    true, false, true, true, true, true, true, true, true, true, true, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false,
    false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 503] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 3e-9, label: "3e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 3e-10, label: "3e-10" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), None, None, None, Some(ParameterBound { value: 1e19, label: "1e19" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.05, label: "0.05" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }),
    Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-6, label: "1e-6" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-15, label: "1e-15" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 3e-9, label: "3e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 3e-10, label: "3e-10" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.05, label: "0.05" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }),
    None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }),
    None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.5, label: "0.5" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-15, label: "1e-15" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 503] = [
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None,
    None, Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e-6, label: "1e-6" }), None, None, None,
    Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 1e21, label: "1e21" }), Some(ParameterBound { value: 1e21, label: "1e21" }), None, None, None, Some(ParameterBound { value: 1e22, label: "1e22" }), Some(ParameterBound { value: 10.0, label: "10.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), None, Some(ParameterBound { value: 1e22, label: "1e22" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 16.0, label: "16.0" }), None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e22, label: "1e22" }), None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), None,
    None, Some(ParameterBound { value: 16.0, label: "16.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e-6, label: "1e-6" }), None, None, None,
    Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 1e21, label: "1e21" }), Some(ParameterBound { value: 1e21, label: "1e21" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None,
    Some(ParameterBound { value: 1e22, label: "1e22" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1e22, label: "1e22" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e-5, label: "1e-5" }), None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 503] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 3, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 2, 2,
    0, 2, 0, 2, 2, 0, 0, 2, 2, 2, 0, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 2, 0, 2,
    2, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2,
    0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 2, 0, 0, 0, 2,
    2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0,
    2, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 2, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0,
    2, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0,
    2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0,
    2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0,
    2, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 2, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 503] = [
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
    &[], &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 14],
    pub branches: [usize; 4],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 503]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<24, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) structured_static: std::sync::Arc<StructuredStaticState<443, 60>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 9;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 9] = ["NSIG", "si", "di", "bp", "gp", "Gnqs", "Gnqs2", "Dnqs", "gndnqs"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 503;
    pub const VARIABLE_COUNT: usize = 1911;
    pub const DDT_STATE_COUNT: usize = 24;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "62e1eaf944a380ceccef3d303842a4010bc21d7e6f28e3e5daed6b8adf1bb272";
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
        let mut values = Vec::with_capacity(120);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(24);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 120);
        debug_assert_eq!(state.flags.len(), 24);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'l_utsoi'", name));
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
