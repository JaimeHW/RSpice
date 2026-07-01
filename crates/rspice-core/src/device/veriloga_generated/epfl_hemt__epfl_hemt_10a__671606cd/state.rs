#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

pub struct Parameters {
    pub p0: f64,
    pub p1: f64,
    pub p2: f64,
    pub p3: f64,
    pub p4: f64,
    pub p5: f64,
    pub p6: f64,
    pub p7: f64,
    pub p8: f64,
    pub p9: f64,
    pub p10: f64,
    pub p11: f64,
    pub p12: f64,
    pub p13: f64,
    pub p14: f64,
    pub p15: f64,
    pub p16: f64,
    pub p17: f64,
    pub p18: f64,
    pub p19: f64,
    pub p20: f64,
    pub p21: f64,
    pub p22: f64,
    pub p23: f64,
    pub p24: f64,
    pub p25: f64,
    pub p26: f64,
    pub p27: f64,
    pub p28: f64,
    pub p29: f64,
    pub p30: f64,
    pub p31: f64,
    pub p32: f64,
    pub p33: f64,
    pub p34: f64,
    pub p35: f64,
    pub p36: f64,
    pub p37: f64,
    pub p38: f64,
    pub p39: f64,
    pub p40: f64,
    pub p41: f64,
    pub p42: f64,
    pub p43: f64,
    pub p44: f64,
    pub p45: f64,
    pub p46: f64,
    pub p47: f64,
}

impl Copy for Parameters {}

impl Clone for Parameters {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            let params = &mut *ptr;
            params.p0 = 1e-6;
            params.p1 = 2e-6;
            params.p2 = 7e-6;
            params.p3 = 0.003;
            params.p4 = 1.0;
            params.p5 = 6e-8;
            params.p6 = 300.0;
            params.p7 = -5.43;
            params.p8 = 0.02;
            params.p9 = -0.01;
            params.p10 = 0.045;
            params.p11 = 1e19;
            params.p12 = 0.3;
            params.p13 = 1.6e-12;
            params.p14 = 200000000.0;
            params.p15 = 8000000.0;
            params.p16 = 200.0;
            params.p17 = 1.0;
            params.p18 = 0.0;
            params.p19 = 0.0;
            params.p20 = 1e-9;
            params.p21 = 0.0;
            params.p22 = 0.165;
            params.p23 = 1.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 1.0;
            params.p27 = -1.7;
            params.p28 = -2.2;
            params.p29 = 0.5;
            params.p30 = 1.0;
            params.p31 = 70.0;
            params.p32 = 1e-8;
            params.p33 = 0.0;
            params.p34 = 1.0;
            params.p35 = 20.0;
            params.p36 = 1e-9;
            params.p37 = 5e17;
            params.p38 = 0.155;
            params.p39 = 30000.0;
            params.p40 = 0.0022;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 0.0022;
            params.p44 = 0.0;
            params.p45 = 2.0;
            params.p46 = 0.0;
            params.p47 = 1.2;
            boxed.assume_init()
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        *Self::new_box()
    }
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
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
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

pub struct Instance {
    pub nodes: [usize; 12],
    pub branches: [usize; 3],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 48]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 1]>,
    pub(crate) ddt_state_previous: Box<[f64; 1]>,
    pub(crate) ddt_state_older: Box<[f64; 1]>,
    pub(crate) ddt_state_initialized: Box<[bool; 1]>,
    pub(crate) ddt_derivative_current: Box<[f64; 1]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 1]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v32: bool,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v388: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v692: f64,
    pub(crate) scalar_v693: f64,
    pub(crate) scalar_v704: f64,
    pub(crate) scalar_v705: f64,
    pub(crate) scalar_v706: f64,
    pub(crate) scalar_v710: f64,
    pub(crate) scalar_v713: f64,
    pub(crate) scalar_v714: f64,
    pub(crate) scalar_v717: f64,
    pub(crate) scalar_v718: f64,
    pub(crate) scalar_v719: f64,
    pub(crate) scalar_v723: f64,
    pub(crate) scalar_v731: f64,
    pub(crate) scalar_v734: f64,
    pub(crate) scalar_v736: f64,
    pub(crate) scalar_v737: f64,
    pub(crate) scalar_v738: f64,
    pub(crate) scalar_v739: f64,
    pub(crate) scalar_v742: f64,
    pub(crate) scalar_v748: f64,
    pub(crate) scalar_v764: f64,
    pub(crate) scalar_v765: bool,
    pub(crate) scalar_v777: bool,
    pub(crate) scalar_v781: f64,
    pub(crate) scalar_v791: f64,
    pub(crate) scalar_v796: f64,
    pub(crate) scalar_v797: f64,
    pub(crate) scalar_v800: f64,
    pub(crate) scalar_v814: f64,
    pub(crate) scalar_v1103: f64,
    pub(crate) scalar_v1104: f64,
    pub(crate) scalar_v1106: f64,
    pub(crate) scalar_v1114: f64,
    pub(crate) scalar_v1125: f64,
    pub(crate) scalar_v1969: f64,
    pub(crate) scalar_v1972: f64,
    pub(crate) scalar_v1973: f64,
    pub(crate) scalar_v3786: f64,
    pub(crate) scalar_v3787: f64,
    pub(crate) scalar_v3788: f64,
    pub(crate) scalar_v3793: f64,
    pub(crate) scalar_v3825: f64,
    pub(crate) scalar_v3836: f64,
    pub(crate) scalar_v4042: f64,
    pub(crate) scalar_v4043: f64,
    pub(crate) scalar_v4044: f64,
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
            ddt_state_current: self.ddt_state_current.clone(),
            ddt_state_previous: self.ddt_state_previous.clone(),
            ddt_state_older: self.ddt_state_older.clone(),
            ddt_state_initialized: self.ddt_state_initialized.clone(),
            ddt_derivative_current: self.ddt_derivative_current.clone(),
            ddt_derivative_previous: self.ddt_derivative_previous.clone(),
            idt_state_current: self.idt_state_current.clone(),
            idt_state_previous: self.idt_state_previous.clone(),
            idt_state_initialized: self.idt_state_initialized.clone(),
            time: self.time,
            timestep: self.timestep,
            ddt_coefficients: self.ddt_coefficients,
            scalar_v8: self.scalar_v8,
            scalar_v14: self.scalar_v14,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v52: self.scalar_v52,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v67: self.scalar_v67,
            scalar_v77: self.scalar_v77,
            scalar_v81: self.scalar_v81,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v95: self.scalar_v95,
            scalar_v113: self.scalar_v113,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v125: self.scalar_v125,
            scalar_v185: self.scalar_v185,
            scalar_v191: self.scalar_v191,
            scalar_v192: self.scalar_v192,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v363: self.scalar_v363,
            scalar_v387: self.scalar_v387,
            scalar_v388: self.scalar_v388,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v444: self.scalar_v444,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v455: self.scalar_v455,
            scalar_v461: self.scalar_v461,
            scalar_v692: self.scalar_v692,
            scalar_v693: self.scalar_v693,
            scalar_v704: self.scalar_v704,
            scalar_v705: self.scalar_v705,
            scalar_v706: self.scalar_v706,
            scalar_v710: self.scalar_v710,
            scalar_v713: self.scalar_v713,
            scalar_v714: self.scalar_v714,
            scalar_v717: self.scalar_v717,
            scalar_v718: self.scalar_v718,
            scalar_v719: self.scalar_v719,
            scalar_v723: self.scalar_v723,
            scalar_v731: self.scalar_v731,
            scalar_v734: self.scalar_v734,
            scalar_v736: self.scalar_v736,
            scalar_v737: self.scalar_v737,
            scalar_v738: self.scalar_v738,
            scalar_v739: self.scalar_v739,
            scalar_v742: self.scalar_v742,
            scalar_v748: self.scalar_v748,
            scalar_v764: self.scalar_v764,
            scalar_v765: self.scalar_v765,
            scalar_v777: self.scalar_v777,
            scalar_v781: self.scalar_v781,
            scalar_v791: self.scalar_v791,
            scalar_v796: self.scalar_v796,
            scalar_v797: self.scalar_v797,
            scalar_v800: self.scalar_v800,
            scalar_v814: self.scalar_v814,
            scalar_v1103: self.scalar_v1103,
            scalar_v1104: self.scalar_v1104,
            scalar_v1106: self.scalar_v1106,
            scalar_v1114: self.scalar_v1114,
            scalar_v1125: self.scalar_v1125,
            scalar_v1969: self.scalar_v1969,
            scalar_v1972: self.scalar_v1972,
            scalar_v1973: self.scalar_v1973,
            scalar_v3786: self.scalar_v3786,
            scalar_v3787: self.scalar_v3787,
            scalar_v3788: self.scalar_v3788,
            scalar_v3793: self.scalar_v3793,
            scalar_v3825: self.scalar_v3825,
            scalar_v3836: self.scalar_v3836,
            scalar_v4042: self.scalar_v4042,
            scalar_v4043: self.scalar_v4043,
            scalar_v4044: self.scalar_v4044,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 7;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 7] = ["di", "si", "gi", "gm", "bi", "sbulk", "dbulk"];

    pub const BRANCH_COUNT: usize = 3;
    pub const PARAMETER_COUNT: usize = 48;
    pub const VARIABLE_COUNT: usize = 149;
    pub const DDT_STATE_COUNT: usize = 1;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        let mut instance = Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::new_box(),
            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),
            multiplicity: 1.0,
            ddt_state_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_older: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_derivative_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_derivative_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            ddt_coefficients: GeneratedDdtCoefficients::inactive(),
            scalar_v8: 0.0,
            scalar_v14: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v32: false,
            scalar_v35: 0.0,
            scalar_v36: 0.0,
            scalar_v37: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v52: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v67: 0.0,
            scalar_v77: 0.0,
            scalar_v81: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v95: 0.0,
            scalar_v113: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v125: 0.0,
            scalar_v185: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v348: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v353: 0.0,
            scalar_v354: 0.0,
            scalar_v355: 0.0,
            scalar_v358: 0.0,
            scalar_v359: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v363: 0.0,
            scalar_v387: 0.0,
            scalar_v388: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v444: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v455: 0.0,
            scalar_v461: 0.0,
            scalar_v692: 0.0,
            scalar_v693: 0.0,
            scalar_v704: 0.0,
            scalar_v705: 0.0,
            scalar_v706: 0.0,
            scalar_v710: 0.0,
            scalar_v713: 0.0,
            scalar_v714: 0.0,
            scalar_v717: 0.0,
            scalar_v718: 0.0,
            scalar_v719: 0.0,
            scalar_v723: 0.0,
            scalar_v731: 0.0,
            scalar_v734: 0.0,
            scalar_v736: 0.0,
            scalar_v737: 0.0,
            scalar_v738: 0.0,
            scalar_v739: 0.0,
            scalar_v742: 0.0,
            scalar_v748: 0.0,
            scalar_v764: 0.0,
            scalar_v765: false,
            scalar_v777: false,
            scalar_v781: 0.0,
            scalar_v791: 0.0,
            scalar_v796: 0.0,
            scalar_v797: 0.0,
            scalar_v800: 0.0,
            scalar_v814: 0.0,
            scalar_v1103: 0.0,
            scalar_v1104: 0.0,
            scalar_v1106: 0.0,
            scalar_v1114: 0.0,
            scalar_v1125: 0.0,
            scalar_v1969: 0.0,
            scalar_v1972: 0.0,
            scalar_v1973: 0.0,
            scalar_v3786: 0.0,
            scalar_v3787: 0.0,
            scalar_v3788: 0.0,
            scalar_v3793: 0.0,
            scalar_v3825: 0.0,
            scalar_v3836: 0.0,
            scalar_v4042: 0.0,
            scalar_v4043: 0.0,
            scalar_v4044: 0.0,
        };
        instance.recompute_instance_static();
        instance
    }

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        let Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_older,
            ddt_state_initialized,
            ddt_derivative_current,
            ddt_derivative_previous,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            ddt_coefficients,
            scalar_v8,
            scalar_v14,
            scalar_v25,
            scalar_v26,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v42,
            scalar_v43,
            scalar_v52,
            scalar_v58,
            scalar_v59,
            scalar_v67,
            scalar_v77,
            scalar_v81,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v95,
            scalar_v113,
            scalar_v115,
            scalar_v116,
            scalar_v125,
            scalar_v185,
            scalar_v191,
            scalar_v192,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v358,
            scalar_v359,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v387,
            scalar_v388,
            scalar_v417,
            scalar_v418,
            scalar_v444,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v455,
            scalar_v461,
            scalar_v692,
            scalar_v693,
            scalar_v704,
            scalar_v705,
            scalar_v706,
            scalar_v710,
            scalar_v713,
            scalar_v714,
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v723,
            scalar_v731,
            scalar_v734,
            scalar_v736,
            scalar_v737,
            scalar_v738,
            scalar_v739,
            scalar_v742,
            scalar_v748,
            scalar_v764,
            scalar_v765,
            scalar_v777,
            scalar_v781,
            scalar_v791,
            scalar_v796,
            scalar_v797,
            scalar_v800,
            scalar_v814,
            scalar_v1103,
            scalar_v1104,
            scalar_v1106,
            scalar_v1114,
            scalar_v1125,
            scalar_v1969,
            scalar_v1972,
            scalar_v1973,
            scalar_v3786,
            scalar_v3787,
            scalar_v3788,
            scalar_v3793,
            scalar_v3825,
            scalar_v3836,
            scalar_v4042,
            scalar_v4043,
            scalar_v4044,
        } = snapshot;
        *self = Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_older,
            ddt_state_initialized,
            ddt_derivative_current,
            ddt_derivative_previous,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            ddt_coefficients,
            scalar_v8,
            scalar_v14,
            scalar_v25,
            scalar_v26,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v42,
            scalar_v43,
            scalar_v52,
            scalar_v58,
            scalar_v59,
            scalar_v67,
            scalar_v77,
            scalar_v81,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v95,
            scalar_v113,
            scalar_v115,
            scalar_v116,
            scalar_v125,
            scalar_v185,
            scalar_v191,
            scalar_v192,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v358,
            scalar_v359,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v387,
            scalar_v388,
            scalar_v417,
            scalar_v418,
            scalar_v444,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v455,
            scalar_v461,
            scalar_v692,
            scalar_v693,
            scalar_v704,
            scalar_v705,
            scalar_v706,
            scalar_v710,
            scalar_v713,
            scalar_v714,
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v723,
            scalar_v731,
            scalar_v734,
            scalar_v736,
            scalar_v737,
            scalar_v738,
            scalar_v739,
            scalar_v742,
            scalar_v748,
            scalar_v764,
            scalar_v765,
            scalar_v777,
            scalar_v781,
            scalar_v791,
            scalar_v796,
            scalar_v797,
            scalar_v800,
            scalar_v814,
            scalar_v1103,
            scalar_v1104,
            scalar_v1106,
            scalar_v1114,
            scalar_v1125,
            scalar_v1969,
            scalar_v1972,
            scalar_v1973,
            scalar_v3786,
            scalar_v3787,
            scalar_v3788,
            scalar_v3793,
            scalar_v3825,
            scalar_v3836,
            scalar_v4042,
            scalar_v4043,
            scalar_v4044,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "lard" => { validate_parameter("Lard", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "lars" => { validate_parameter("Lars", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "x1" => { validate_parameter("x1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "va" => { validate_finite_parameter("VA", value)?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "eta0" => { validate_finite_parameter("ETA0", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "etab" => { validate_finite_parameter("ETAB", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "phin" => { validate_finite_parameter("PHIN", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "ndep" => { validate_parameter("NDEP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "xx" => { validate_parameter("xx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "gg" => { validate_parameter("gg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "e0" => { validate_parameter("E0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ucrit" => { validate_finite_parameter("UCRIT", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "aclm" => { validate_parameter("ACLM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "delta" => { validate_parameter("DELTA", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "cit" => { validate_parameter("CIT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "nfactor" => { validate_parameter("NFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "cdscd" => { validate_parameter("CDSCD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "cdscb" => { validate_parameter("CDSCB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "u0" => { validate_parameter("U0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "ua" => { validate_parameter("UA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "uc" => { validate_parameter("UC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "ud" => { validate_parameter("UD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "eu" => { validate_parameter("EU", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "bex" => { validate_finite_parameter("BEX", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "ucex" => { validate_finite_parameter("UCEX", value)?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "etavsat" => { validate_finite_parameter("ETAVSAT", value)?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "usc" => { validate_parameter("USC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "avdsx" => { validate_parameter("AVDSX", value, Some((5.0, "5.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "lc" => { validate_parameter("LC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "lambda" => { validate_parameter("LAMBDA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "ns0" => { validate_parameter("ns0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "mu0acc" => { validate_parameter("mu0acc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "vsat0acc" => { validate_parameter("vsat0acc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "rcs" => { validate_parameter("Rcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "ktrs" => { validate_parameter("ktrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "ktrd" => { validate_parameter("ktrd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "rcd" => { validate_parameter("Rcd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "kth1" => { validate_parameter("kth1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "kth2" => { validate_parameter("kth2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "kth3" => { validate_finite_parameter("kth3", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "gsar" => { validate_parameter("gsar", value, None, true, None, true, &[(0.0, "0.0")])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'EPFL_HEMT_10a'", name)),
        }
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            self.multiplicity = multiplicity;
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
            self.ddt_state_older[index] = self.ddt_state_previous[index];
            self.ddt_state_previous[index] = self.ddt_state_current[index];
            self.ddt_derivative_previous[index] = self.ddt_derivative_current[index];
            self.ddt_state_initialized[index] = true;
            index += 1;
        }
        let mut index = 0usize;
        while index < Self::IDT_STATE_COUNT {
            self.idt_state_previous[index] = self.idt_state_current[index];
            self.idt_state_initialized[index] = true;
            index += 1;
        }
    }

    #[inline]
    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {
        debug_assert!(slot < Self::DDT_STATE_COUNT, "generated ddt state slot out of range");
        let previous = if self.ddt_state_initialized[slot] {
            self.ddt_state_previous[slot]
        } else {
            value
        };
        let older = if self.ddt_state_initialized[slot] {
            self.ddt_state_older[slot]
        } else {
            value
        };
        self.ddt_state_current[slot] = value;
        if self.ddt_coefficients.active {
            let result = value * self.ddt_coefficients.derivative_scale
                - previous * self.ddt_coefficients.previous_value_scale
                - older * self.ddt_coefficients.older_value_scale
                - self.ddt_derivative_previous[slot] * self.ddt_coefficients.previous_derivative_scale;
            self.ddt_derivative_current[slot] = result;
            result
        } else {
            self.ddt_state_current[slot] = value;
            self.ddt_state_previous[slot] = value;
            self.ddt_state_older[slot] = value;
            self.ddt_derivative_current[slot] = 0.0;
            self.ddt_derivative_previous[slot] = 0.0;
            self.ddt_state_initialized[slot] = true;
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
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        let v8: f64 = p.p34;
        self.scalar_v8 = v8;
        let v14: f64 = p.p6;
        self.scalar_v14 = v14;
        let v25: f64 = p.p12;
        self.scalar_v25 = v25;
        let v26: f64 = (1.0 - p.p12);
        self.scalar_v26 = v26;
        let v28: f64 = (p.p12 * 8.5);
        self.scalar_v28 = v28;
        let v29: f64 = (8.9 * v26);
        self.scalar_v29 = v29;
        let v30: f64 = (v28 + v29);
        self.scalar_v30 = v30;
        let v31: f64 = (8.85418e-12 * v30);
        self.scalar_v31 = v31;
        let v32: bool = (1.0 != p.p34);
        self.scalar_v32 = v32;
        let v35: f64 = (if v32 { 0.3333333333333333 } else { 0.5 });
        self.scalar_v35 = v35;
        let v36: f64 = p.p22;
        self.scalar_v36 = v36;
        let v37: f64 = p.p27;
        self.scalar_v37 = v37;
        let v42: f64 = p.p3;
        self.scalar_v42 = v42;
        let v43: f64 = p.p0;
        self.scalar_v43 = v43;
        let v52: f64 = p.p11;
        self.scalar_v52 = v52;
        let v58: f64 = p.p5;
        self.scalar_v58 = v58;
        let v59: f64 = (v31 / p.p5);
        self.scalar_v59 = v59;
        let v67: f64 = p.p31;
        self.scalar_v67 = v67;
        let v77: f64 = (2.0 / p.p31);
        self.scalar_v77 = v77;
        let v81: f64 = (v77 * 0.6931471805599453);
        self.scalar_v81 = v81;
        let v89: f64 = p.p18;
        self.scalar_v89 = v89;
        let v90: f64 = p.p19;
        self.scalar_v90 = v90;
        let v91: f64 = (p.p18 + p.p19);
        self.scalar_v91 = v91;
        let v92: f64 = p.p20;
        self.scalar_v92 = v92;
        let v95: f64 = p.p21;
        self.scalar_v95 = v95;
        let v113: f64 = p.p7;
        self.scalar_v113 = v113;
        let v115: f64 = p.p8;
        self.scalar_v115 = v115;
        let v116: f64 = p.p9;
        self.scalar_v116 = v116;
        let v125: f64 = (p.p11 * 2.52482255208e-29);
        self.scalar_v125 = v125;
        let v185: f64 = p.p13;
        self.scalar_v185 = v185;
        let v191: f64 = (p.p11 / 1.8e25);
        self.scalar_v191 = v191;
        let v192: f64 = ((v191) as f64).ln();
        self.scalar_v192 = v192;
        let v348: f64 = p.p14;
        self.scalar_v348 = v348;
        let v349: f64 = (7.8802202e-11 * p.p14);
        self.scalar_v349 = v349;
        let v350: f64 = (v59 / v349);
        self.scalar_v350 = v350;
        let v353: f64 = p.p30;
        self.scalar_v353 = v353;
        let v354: f64 = p.p23;
        self.scalar_v354 = v354;
        let v355: f64 = p.p24;
        self.scalar_v355 = v355;
        let v358: f64 = p.p26;
        self.scalar_v358 = v358;
        let v359: f64 = p.p25;
        self.scalar_v359 = v359;
        let v361: f64 = p.p15;
        self.scalar_v361 = v361;
        let v362: f64 = p.p28;
        self.scalar_v362 = v362;
        let v363: f64 = (-p.p28);
        self.scalar_v363 = v363;
        let v387: f64 = p.p17;
        self.scalar_v387 = v387;
        let v388: f64 = (2.0 - p.p17);
        self.scalar_v388 = v388;
        let v417: f64 = p.p16;
        self.scalar_v417 = v417;
        let v418: f64 = (p.p16 / p.p17);
        self.scalar_v418 = v418;
        let v444: f64 = p.p32;
        self.scalar_v444 = v444;
        let v448: f64 = (2.0 * p.p32);
        self.scalar_v448 = v448;
        let v449: f64 = (p.p0 - v448);
        self.scalar_v449 = v449;
        let v450: f64 = (p.p32 / v449);
        self.scalar_v450 = v450;
        let v451: f64 = p.p33;
        self.scalar_v451 = v451;
        let v452: f64 = (p.p32 * p.p33);
        self.scalar_v452 = v452;
        let v455: f64 = (2.0 * v450);
        self.scalar_v455 = v455;
        let v461: f64 = (1.0 + v450);
        self.scalar_v461 = v461;
        let v692: f64 = p.p4;
        self.scalar_v692 = v692;
        let v693: f64 = (2.0 * p.p4);
        self.scalar_v693 = v693;
        let v704: f64 = p.p37;
        self.scalar_v704 = v704;
        let v705: f64 = p.p39;
        self.scalar_v705 = v705;
        let v706: f64 = p.p44;
        self.scalar_v706 = v706;
        let v710: f64 = p.p45;
        self.scalar_v710 = v710;
        let v713: f64 = p.p38;
        self.scalar_v713 = v713;
        let v714: f64 = p.p46;
        self.scalar_v714 = v714;
        let v717: f64 = (1.602e-19 * p.p37);
        self.scalar_v717 = v717;
        let v718: f64 = (p.p3 * v717);
        self.scalar_v718 = v718;
        let v719: f64 = p.p1;
        self.scalar_v719 = v719;
        let v723: f64 = p.p2;
        self.scalar_v723 = v723;
        let v731: f64 = p.p47;
        self.scalar_v731 = v731;
        let v734: f64 = (1.0 / p.p47);
        self.scalar_v734 = v734;
        let v736: f64 = p.p40;
        self.scalar_v736 = v736;
        let v737: f64 = (p.p40 / p.p3);
        self.scalar_v737 = v737;
        let v738: f64 = p.p43;
        self.scalar_v738 = v738;
        let v739: f64 = (p.p43 / p.p3);
        self.scalar_v739 = v739;
        let v742: f64 = p.p42;
        self.scalar_v742 = v742;
        let v748: f64 = p.p41;
        self.scalar_v748 = v748;
        let v764: f64 = p.p35;
        self.scalar_v764 = v764;
        let v765: bool = (0.0 != p.p35);
        self.scalar_v765 = v765;
        let v777: bool = (!v765);
        self.scalar_v777 = v777;
        let v781: f64 = (1.0 / p.p6);
        self.scalar_v781 = v781;
        let v791: f64 = (p.p27 - 1.0);
        self.scalar_v791 = v791;
        let v796: f64 = (0.0259 * v781);
        self.scalar_v796 = v796;
        let v797: f64 = (-v796);
        self.scalar_v797 = v797;
        let v800: f64 = (3.204e-19 * v796);
        self.scalar_v800 = v800;
        let v814: f64 = (-p.p31);
        self.scalar_v814 = v814;
        let v1103: f64 = (p.p13 * v796);
        self.scalar_v1103 = v1103;
        let v1104: f64 = (-v1103);
        self.scalar_v1104 = v1104;
        let v1106: f64 = (8.353992494899963e17 * v796);
        self.scalar_v1106 = v1106;
        let v1114: f64 = (1.602e-19 * v796);
        self.scalar_v1114 = v1114;
        let v1125: f64 = (1.602e-19 * v1106);
        self.scalar_v1125 = v1125;
        let v1969: f64 = (p.p30 - 1.0);
        self.scalar_v1969 = v1969;
        let v1972: f64 = (p.p26 - 1.0);
        self.scalar_v1972 = v1972;
        let v1973: f64 = (v363 - 1.0);
        self.scalar_v1973 = v1973;
        let v3786: f64 = (p.p44 * v781);
        self.scalar_v3786 = v3786;
        let v3787: f64 = (-v3786);
        self.scalar_v3787 = v3787;
        let v3788: f64 = (p.p45 - 1.0);
        self.scalar_v3788 = v3788;
        let v3793: f64 = (p.p46 - 1.0);
        self.scalar_v3793 = v3793;
        let v3825: f64 = (p.p47 - 1.0);
        self.scalar_v3825 = v3825;
        let v3836: f64 = (v734 - 1.0);
        self.scalar_v3836 = v3836;
        let v4042: f64 = (1.0 / p.p35);
        self.scalar_v4042 = v4042;
        let v4043: f64 = (if v765 { v4042 } else { 0.0 });
        self.scalar_v4043 = v4043;
        let v4044: f64 = (if v777 { 1000000000.0 } else { 0.0 });
        self.scalar_v4044 = v4044;
    }
}
