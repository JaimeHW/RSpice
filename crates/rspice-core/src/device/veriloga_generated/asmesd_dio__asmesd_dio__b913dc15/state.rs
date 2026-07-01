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
    pub p48: f64,
    pub p49: f64,
    pub p50: f64,
    pub p51: f64,
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
            params.p0 = 1e-17;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 5.0;
            params.p4 = 10.0;
            params.p5 = 10.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.01;
            params.p9 = 1.11;
            params.p10 = 0.0;
            params.p11 = 10.0;
            params.p12 = 1e-5;
            params.p13 = 0.0;
            params.p14 = 1e-6;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 0.75;
            params.p18 = 0.33;
            params.p19 = 0.0;
            params.p20 = 0.001;
            params.p21 = 1.11;
            params.p22 = 3.0;
            params.p23 = 0.5;
            params.p24 = 0.5;
            params.p25 = 25.0;
            params.p26 = 1000.0;
            params.p27 = 0.0;
            params.p28 = 1.0;
            params.p29 = 1.0;
            params.p30 = 2.0;
            params.p31 = 0.0;
            params.p32 = 1.0;
            params.p33 = 0.0005;
            params.p34 = 0.0005;
            params.p35 = 5e-6;
            params.p36 = 1e-7;
            params.p37 = 0.0;
            params.p38 = 0.0;
            params.p39 = 2.0;
            params.p40 = 100.0;
            params.p41 = 0.0;
            params.p42 = 1e-5;
            params.p43 = 1.0;
            params.p44 = 1.0;
            params.p45 = 0.0;
            params.p46 = 0.001;
            validate_parameter("minr", params.p46, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p47 = 5.0;
            params.p48 = 100.0;
            params.p49 = 2.0;
            params.p50 = 100.0;
            params.p51 = 2.0;
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
    pub nodes: [usize; 7],
    pub branches: [usize; 7],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 52]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 6]>,
    pub(crate) ddt_state_previous: Box<[f64; 6]>,
    pub(crate) ddt_state_older: Box<[f64; 6]>,
    pub(crate) ddt_state_initialized: Box<[bool; 6]>,
    pub(crate) ddt_derivative_current: Box<[f64; 6]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 6]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v244: bool,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: bool,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v278: bool,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: bool,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: bool,
    pub(crate) scalar_v323: bool,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v326: bool,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: bool,
    pub(crate) scalar_v329: bool,
    pub(crate) scalar_v330: bool,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: bool,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: bool,
    pub(crate) scalar_v340: bool,
    pub(crate) scalar_v342: bool,
    pub(crate) scalar_v343: bool,
    pub(crate) scalar_v344: bool,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: bool,
    pub(crate) scalar_v366: bool,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v380: bool,
    pub(crate) scalar_v381: bool,
    pub(crate) scalar_v382: bool,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v385: bool,
    pub(crate) scalar_v386: bool,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: bool,
    pub(crate) scalar_v395: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v401: bool,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v598: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v679: f64,
    pub(crate) scalar_v755: f64,
    pub(crate) scalar_v759: f64,
    pub(crate) scalar_v760: f64,
    pub(crate) scalar_v763: f64,
    pub(crate) scalar_v764: f64,
    pub(crate) scalar_v765: f64,
    pub(crate) scalar_v767: f64,
    pub(crate) scalar_v768: f64,
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
            scalar_v3: self.scalar_v3,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v23: self.scalar_v23,
            scalar_v25: self.scalar_v25,
            scalar_v30: self.scalar_v30,
            scalar_v32: self.scalar_v32,
            scalar_v35: self.scalar_v35,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v48: self.scalar_v48,
            scalar_v49: self.scalar_v49,
            scalar_v53: self.scalar_v53,
            scalar_v55: self.scalar_v55,
            scalar_v81: self.scalar_v81,
            scalar_v86: self.scalar_v86,
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v104: self.scalar_v104,
            scalar_v116: self.scalar_v116,
            scalar_v122: self.scalar_v122,
            scalar_v173: self.scalar_v173,
            scalar_v184: self.scalar_v184,
            scalar_v192: self.scalar_v192,
            scalar_v215: self.scalar_v215,
            scalar_v218: self.scalar_v218,
            scalar_v221: self.scalar_v221,
            scalar_v224: self.scalar_v224,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v232: self.scalar_v232,
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v240: self.scalar_v240,
            scalar_v243: self.scalar_v243,
            scalar_v244: self.scalar_v244,
            scalar_v245: self.scalar_v245,
            scalar_v248: self.scalar_v248,
            scalar_v252: self.scalar_v252,
            scalar_v255: self.scalar_v255,
            scalar_v258: self.scalar_v258,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v271: self.scalar_v271,
            scalar_v273: self.scalar_v273,
            scalar_v278: self.scalar_v278,
            scalar_v281: self.scalar_v281,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v295: self.scalar_v295,
            scalar_v299: self.scalar_v299,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v353: self.scalar_v353,
            scalar_v360: self.scalar_v360,
            scalar_v364: self.scalar_v364,
            scalar_v365: self.scalar_v365,
            scalar_v366: self.scalar_v366,
            scalar_v376: self.scalar_v376,
            scalar_v380: self.scalar_v380,
            scalar_v381: self.scalar_v381,
            scalar_v382: self.scalar_v382,
            scalar_v384: self.scalar_v384,
            scalar_v385: self.scalar_v385,
            scalar_v386: self.scalar_v386,
            scalar_v387: self.scalar_v387,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v395: self.scalar_v395,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v488: self.scalar_v488,
            scalar_v597: self.scalar_v597,
            scalar_v598: self.scalar_v598,
            scalar_v678: self.scalar_v678,
            scalar_v679: self.scalar_v679,
            scalar_v755: self.scalar_v755,
            scalar_v759: self.scalar_v759,
            scalar_v760: self.scalar_v760,
            scalar_v763: self.scalar_v763,
            scalar_v764: self.scalar_v764,
            scalar_v765: self.scalar_v765,
            scalar_v767: self.scalar_v767,
            scalar_v768: self.scalar_v768,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 7;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["bi", "ei", "dt1", "tt"];

    pub const BRANCH_COUNT: usize = 7;
    pub const PARAMETER_COUNT: usize = 52;
    pub const VARIABLE_COUNT: usize = 75;
    pub const DDT_STATE_COUNT: usize = 6;
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
            scalar_v3: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v23: 0.0,
            scalar_v25: 0.0,
            scalar_v30: 0.0,
            scalar_v32: 0.0,
            scalar_v35: 0.0,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
            scalar_v48: 0.0,
            scalar_v49: 0.0,
            scalar_v53: 0.0,
            scalar_v55: 0.0,
            scalar_v81: 0.0,
            scalar_v86: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v104: 0.0,
            scalar_v116: 0.0,
            scalar_v122: 0.0,
            scalar_v173: 0.0,
            scalar_v184: 0.0,
            scalar_v192: 0.0,
            scalar_v215: 0.0,
            scalar_v218: 0.0,
            scalar_v221: 0.0,
            scalar_v224: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v232: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v240: 0.0,
            scalar_v243: 0.0,
            scalar_v244: false,
            scalar_v245: 0.0,
            scalar_v248: 0.0,
            scalar_v252: 0.0,
            scalar_v255: 0.0,
            scalar_v258: 0.0,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v267: 0.0,
            scalar_v268: false,
            scalar_v271: 0.0,
            scalar_v273: 0.0,
            scalar_v278: false,
            scalar_v281: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v295: 0.0,
            scalar_v299: 0.0,
            scalar_v319: 0.0,
            scalar_v320: false,
            scalar_v321: 0.0,
            scalar_v322: false,
            scalar_v323: false,
            scalar_v325: false,
            scalar_v326: false,
            scalar_v327: 0.0,
            scalar_v328: false,
            scalar_v329: false,
            scalar_v330: false,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: false,
            scalar_v338: 0.0,
            scalar_v339: false,
            scalar_v340: false,
            scalar_v342: false,
            scalar_v343: false,
            scalar_v344: false,
            scalar_v353: 0.0,
            scalar_v360: 0.0,
            scalar_v364: 0.0,
            scalar_v365: false,
            scalar_v366: false,
            scalar_v376: 0.0,
            scalar_v380: false,
            scalar_v381: false,
            scalar_v382: false,
            scalar_v384: 0.0,
            scalar_v385: false,
            scalar_v386: false,
            scalar_v387: 0.0,
            scalar_v393: 0.0,
            scalar_v394: false,
            scalar_v395: 0.0,
            scalar_v400: 0.0,
            scalar_v401: false,
            scalar_v402: 0.0,
            scalar_v488: 0.0,
            scalar_v597: 0.0,
            scalar_v598: 0.0,
            scalar_v678: 0.0,
            scalar_v679: 0.0,
            scalar_v755: 0.0,
            scalar_v759: 0.0,
            scalar_v760: 0.0,
            scalar_v763: 0.0,
            scalar_v764: 0.0,
            scalar_v765: 0.0,
            scalar_v767: 0.0,
            scalar_v768: 0.0,
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
            scalar_v3,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v23,
            scalar_v25,
            scalar_v30,
            scalar_v32,
            scalar_v35,
            scalar_v38,
            scalar_v39,
            scalar_v43,
            scalar_v44,
            scalar_v48,
            scalar_v49,
            scalar_v53,
            scalar_v55,
            scalar_v81,
            scalar_v86,
            scalar_v88,
            scalar_v89,
            scalar_v104,
            scalar_v116,
            scalar_v122,
            scalar_v173,
            scalar_v184,
            scalar_v192,
            scalar_v215,
            scalar_v218,
            scalar_v221,
            scalar_v224,
            scalar_v227,
            scalar_v228,
            scalar_v232,
            scalar_v235,
            scalar_v236,
            scalar_v240,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v248,
            scalar_v252,
            scalar_v255,
            scalar_v258,
            scalar_v261,
            scalar_v262,
            scalar_v267,
            scalar_v268,
            scalar_v271,
            scalar_v273,
            scalar_v278,
            scalar_v281,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v295,
            scalar_v299,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v353,
            scalar_v360,
            scalar_v364,
            scalar_v365,
            scalar_v366,
            scalar_v376,
            scalar_v380,
            scalar_v381,
            scalar_v382,
            scalar_v384,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v488,
            scalar_v597,
            scalar_v598,
            scalar_v678,
            scalar_v679,
            scalar_v755,
            scalar_v759,
            scalar_v760,
            scalar_v763,
            scalar_v764,
            scalar_v765,
            scalar_v767,
            scalar_v768,
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
            scalar_v3,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v23,
            scalar_v25,
            scalar_v30,
            scalar_v32,
            scalar_v35,
            scalar_v38,
            scalar_v39,
            scalar_v43,
            scalar_v44,
            scalar_v48,
            scalar_v49,
            scalar_v53,
            scalar_v55,
            scalar_v81,
            scalar_v86,
            scalar_v88,
            scalar_v89,
            scalar_v104,
            scalar_v116,
            scalar_v122,
            scalar_v173,
            scalar_v184,
            scalar_v192,
            scalar_v215,
            scalar_v218,
            scalar_v221,
            scalar_v224,
            scalar_v227,
            scalar_v228,
            scalar_v232,
            scalar_v235,
            scalar_v236,
            scalar_v240,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v248,
            scalar_v252,
            scalar_v255,
            scalar_v258,
            scalar_v261,
            scalar_v262,
            scalar_v267,
            scalar_v268,
            scalar_v271,
            scalar_v273,
            scalar_v278,
            scalar_v281,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v295,
            scalar_v299,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v353,
            scalar_v360,
            scalar_v364,
            scalar_v365,
            scalar_v366,
            scalar_v376,
            scalar_v380,
            scalar_v381,
            scalar_v382,
            scalar_v384,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v488,
            scalar_v597,
            scalar_v598,
            scalar_v678,
            scalar_v679,
            scalar_v755,
            scalar_v759,
            scalar_v760,
            scalar_v763,
            scalar_v764,
            scalar_v765,
            scalar_v767,
            scalar_v768,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "isr" => { validate_parameter("isr", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "ntr" => { validate_parameter("ntr", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "vtr" => { validate_parameter("vtr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "bvr" => { validate_parameter("bvr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "xbvr" => { validate_finite_parameter("xbvr", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "xjbv" => { validate_parameter("xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "ther" => { validate_parameter("ther", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "theexp" => { validate_parameter("theexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "xtheexp" => { validate_finite_parameter("xtheexp", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "nbv" => { validate_parameter("nbv", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "rb" => { validate_parameter("rb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "rbe" => { validate_parameter("rbe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ree" => { validate_parameter("ree", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "vje" => { validate_parameter("vje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "mje" => { validate_parameter("mje", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "tf" => { validate_parameter("tf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "qtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "vtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "eg" => { validate_parameter("eg", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "xti" => { validate_parameter("xti", value, Some((0.0, "0.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "xtir" => { validate_parameter("xtir", value, Some((-20.0, "-20.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-40.0, "-40.0")), false, Some((125.0, "125.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "tfail" => { validate_parameter("tfail", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "extmod" => { validate_parameter("extmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "rbmod" => { validate_parameter("rbmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "rth1" => { validate_parameter("rth1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "cth1" => { validate_parameter("cth1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "arb" => { validate_parameter("arb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "are" => { validate_parameter("are", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "texp" => { validate_parameter("texp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "vtf0" => { validate_parameter("vtf0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "atff" => { validate_parameter("atff", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "n" => { validate_parameter("n", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "qexp" => { validate_parameter("qexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "ijbv" => { validate_parameter("ijbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "vsatb" => { validate_parameter("vsatb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "mexp" => { validate_parameter("mexp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "vsate" => { validate_parameter("vsate", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "mexpe" => { validate_parameter("mexpe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmesd_dio'", name)),
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
        let v3: f64 = p.p45;
        self.scalar_v3 = v3;
        let v14: f64 = p.p43;
        self.scalar_v14 = v14;
        let v15: f64 = p.p42;
        self.scalar_v15 = v15;
        let v16: f64 = (p.p43 * p.p42);
        self.scalar_v16 = v16;
        let v17: f64 = p.p25;
        self.scalar_v17 = v17;
        let v18: f64 = (273.15 + p.p25);
        self.scalar_v18 = v18;
        let v23: f64 = p.p22;
        self.scalar_v23 = v23;
        let v25: f64 = p.p21;
        self.scalar_v25 = v25;
        let v30: f64 = p.p23;
        self.scalar_v30 = v30;
        let v32: f64 = p.p0;
        self.scalar_v32 = v32;
        let v35: f64 = p.p2;
        self.scalar_v35 = v35;
        let v38: f64 = p.p47;
        self.scalar_v38 = v38;
        let v39: f64 = p.p7;
        self.scalar_v39 = v39;
        let v43: f64 = p.p5;
        self.scalar_v43 = v43;
        let v44: f64 = p.p6;
        self.scalar_v44 = v44;
        let v48: f64 = p.p9;
        self.scalar_v48 = v48;
        let v49: f64 = p.p10;
        self.scalar_v49 = v49;
        let v53: f64 = p.p16;
        self.scalar_v53 = v53;
        let v55: f64 = (v18 / 300.15);
        self.scalar_v55 = v55;
        let v81: f64 = p.p17;
        self.scalar_v81 = v81;
        let v86: f64 = p.p18;
        self.scalar_v86 = v86;
        let v88: f64 = (v18 - 300.15);
        self.scalar_v88 = v88;
        let v89: f64 = (0.0004 * v88);
        self.scalar_v89 = v89;
        let v104: f64 = p.p29;
        self.scalar_v104 = v104;
        let v116: f64 = p.p1;
        self.scalar_v116 = v116;
        let v122: f64 = p.p11;
        self.scalar_v122 = v122;
        let v173: f64 = p.p8;
        self.scalar_v173 = v173;
        let v184: f64 = p.p4;
        self.scalar_v184 = v184;
        let v192: f64 = p.p3;
        self.scalar_v192 = v192;
        let v215: f64 = p.p48;
        self.scalar_v215 = v215;
        let v218: f64 = p.p49;
        self.scalar_v218 = v218;
        let v221: f64 = p.p50;
        self.scalar_v221 = v221;
        let v224: f64 = p.p51;
        self.scalar_v224 = v224;
        let v227: f64 = p.p12;
        self.scalar_v227 = v227;
        let v228: f64 = p.p37;
        self.scalar_v228 = v228;
        let v232: f64 = (1.0 / p.p49);
        self.scalar_v232 = v232;
        let v235: f64 = p.p14;
        self.scalar_v235 = v235;
        let v236: f64 = p.p38;
        self.scalar_v236 = v236;
        let v240: f64 = (1.0 / p.p51);
        self.scalar_v240 = v240;
        let v243: f64 = p.p31;
        self.scalar_v243 = v243;
        let v244: bool = (1.0 == p.p31);
        self.scalar_v244 = v244;
        let v245: f64 = p.p13;
        self.scalar_v245 = v245;
        let v248: f64 = p.p15;
        self.scalar_v248 = v248;
        let v252: f64 = p.p40;
        self.scalar_v252 = v252;
        let v255: f64 = p.p39;
        self.scalar_v255 = v255;
        let v258: f64 = (1.0 / p.p39);
        self.scalar_v258 = v258;
        let v261: f64 = p.p19;
        self.scalar_v261 = v261;
        let v262: f64 = p.p41;
        self.scalar_v262 = v262;
        let v267: f64 = p.p32;
        self.scalar_v267 = v267;
        let v268: bool = (1.0 == p.p32);
        self.scalar_v268 = v268;
        let v271: f64 = p.p20;
        self.scalar_v271 = v271;
        let v273: f64 = p.p44;
        self.scalar_v273 = v273;
        let v278: bool = (!v268);
        self.scalar_v278 = v278;
        let v281: f64 = p.p24;
        self.scalar_v281 = v281;
        let v285: f64 = (-1.0 - p.p18);
        self.scalar_v285 = v285;
        let v286: f64 = (1.0 - p.p24);
        self.scalar_v286 = v286;
        let v287: f64 = ((v286) as f64).ln();
        self.scalar_v287 = v287;
        let v288: f64 = (v285 * v287);
        self.scalar_v288 = v288;
        let v289: f64 = ((v288) as f64).exp();
        self.scalar_v289 = v289;
        let v295: f64 = (1.0 - p.p18);
        self.scalar_v295 = v295;
        let v299: f64 = (p.p18 * 0.5);
        self.scalar_v299 = v299;
        let v319: f64 = p.p30;
        self.scalar_v319 = v319;
        let v320: bool = (1.0 == p.p30);
        self.scalar_v320 = v320;
        let v321: f64 = p.p33;
        self.scalar_v321 = v321;
        let v322: bool = (p.p33 > 0.0);
        self.scalar_v322 = v322;
        let v323: bool = (v320 && v322);
        self.scalar_v323 = v323;
        let v325: bool = (p.p30 == 2.0);
        self.scalar_v325 = v325;
        let v326: bool = (v322 && v325);
        self.scalar_v326 = v326;
        let v327: f64 = p.p35;
        self.scalar_v327 = v327;
        let v328: bool = (p.p35 > 0.0);
        self.scalar_v328 = v328;
        let v329: bool = (v326 && v328);
        self.scalar_v329 = v329;
        let v330: bool = (-1.0 == p.p30);
        self.scalar_v330 = v330;
        let v331: f64 = (p.p31 * p.p13);
        self.scalar_v331 = v331;
        let v332: f64 = (p.p12 + v331);
        self.scalar_v332 = v332;
        let v333: f64 = (v332 / v16);
        self.scalar_v333 = v333;
        let v334: f64 = (p.p31 * p.p15);
        self.scalar_v334 = v334;
        let v335: f64 = (p.p14 + v334);
        self.scalar_v335 = v335;
        let v336: f64 = (v335 / v16);
        self.scalar_v336 = v336;
        let v337: bool = (v333 > 0.0);
        self.scalar_v337 = v337;
        let v338: f64 = p.p46;
        self.scalar_v338 = v338;
        let v339: bool = (v333 >= p.p46);
        self.scalar_v339 = v339;
        let v340: bool = (v337 && v339);
        self.scalar_v340 = v340;
        let v342: bool = (v336 > 0.0);
        self.scalar_v342 = v342;
        let v343: bool = (v336 >= p.p46);
        self.scalar_v343 = v343;
        let v344: bool = (v342 && v343);
        self.scalar_v344 = v344;
        let v353: f64 = (if v278 { 0.0 } else { 0.0 });
        self.scalar_v353 = v353;
        let v360: f64 = p.p34;
        self.scalar_v360 = v360;
        let v364: f64 = (if v323 { 0.0 } else { 0.0 });
        self.scalar_v364 = v364;
        let v365: bool = (!v323);
        self.scalar_v365 = v365;
        let v366: bool = (v329 && v365);
        self.scalar_v366 = v366;
        let v376: f64 = p.p36;
        self.scalar_v376 = v376;
        let v380: bool = (!v329);
        self.scalar_v380 = v380;
        let v381: bool = (v365 && v380);
        self.scalar_v381 = v381;
        let v382: bool = (v330 && v381);
        self.scalar_v382 = v382;
        let v384: f64 = (if v382 { 0.0 } else { 0.0 });
        self.scalar_v384 = v384;
        let v385: bool = (!v330);
        self.scalar_v385 = v385;
        let v386: bool = (v381 && v385);
        self.scalar_v386 = v386;
        let v387: f64 = (if v386 { 0.0 } else { 0.0 });
        self.scalar_v387 = v387;
        let v393: f64 = (if v340 { 0.0 } else { 0.0 });
        self.scalar_v393 = v393;
        let v394: bool = (!v340);
        self.scalar_v394 = v394;
        let v395: f64 = (if v394 { 0.0 } else { 0.0 });
        self.scalar_v395 = v395;
        let v400: f64 = (if v344 { 0.0 } else { 0.0 });
        self.scalar_v400 = v400;
        let v401: bool = (!v344);
        self.scalar_v401 = v401;
        let v402: f64 = (if v401 { 0.0 } else { 0.0 });
        self.scalar_v402 = v402;
        let v488: f64 = (-p.p29);
        self.scalar_v488 = v488;
        let v597: f64 = (p.p4 * v488);
        self.scalar_v597 = v597;
        let v598: f64 = (p.p29 * p.p4);
        self.scalar_v598 = v598;
        let v678: f64 = (p.p29 * v299);
        self.scalar_v678 = v678;
        let v679: f64 = (v299 * v488);
        self.scalar_v679 = v679;
        let v755: f64 = (if v268 { 1.0 } else { 0.0 });
        self.scalar_v755 = v755;
        let v759: f64 = (1.0 / p.p33);
        self.scalar_v759 = v759;
        let v760: f64 = (if v323 { v759 } else { 0.0 });
        self.scalar_v760 = v760;
        let v763: f64 = (-1.0 / p.p33);
        self.scalar_v763 = v763;
        let v764: f64 = (if v366 { v759 } else { 0.0 });
        self.scalar_v764 = v764;
        let v765: f64 = (if v366 { v763 } else { 0.0 });
        self.scalar_v765 = v765;
        let v767: f64 = (1.0 / p.p35);
        self.scalar_v767 = v767;
        let v768: f64 = (if v366 { v767 } else { 0.0 });
        self.scalar_v768 = v768;
    }
}
