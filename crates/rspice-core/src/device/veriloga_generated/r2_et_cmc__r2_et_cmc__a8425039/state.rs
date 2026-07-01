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
            params.p1 = 1e-6;
            params.p2 = 100.0;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 0.0;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 2.0;
            params.p10 = 1.0;
            params.p11 = 0.0;
            params.p12 = -100.0;
            params.p13 = 500.0;
            params.p14 = 0.001;
            params.p15 = 1002.0;
            params.p16 = 27.0;
            params.p17 = 100.0;
            params.p18 = 0.0;
            params.p19 = 9900000000.0;
            params.p20 = 0.0;
            params.p21 = 9900000000.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 2.0;
            params.p32 = 1.0;
            params.p33 = 0.0;
            params.p34 = 100.0;
            params.p35 = -100.0;
            params.p36 = 500.0;
            params.p37 = 0.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 0.0;
            params.p44 = 1000000.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.0;
            params.p48 = 0.0;
            params.p49 = 0.0;
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
    pub nodes: [usize; 3],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 50]>,
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
    pub(crate) scalar_v0: f64,
    pub(crate) scalar_v1: f64,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v4: bool,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: bool,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: bool,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: bool,
    pub(crate) scalar_v31: bool,
    pub(crate) scalar_v32: bool,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: bool,
    pub(crate) scalar_v37: bool,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: bool,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: bool,
    pub(crate) scalar_v44: bool,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: bool,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: bool,
    pub(crate) scalar_v49: bool,
    pub(crate) scalar_v50: bool,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v59: bool,
    pub(crate) scalar_v60: bool,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: bool,
    pub(crate) scalar_v66: bool,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: bool,
    pub(crate) scalar_v75: bool,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: bool,
    pub(crate) scalar_v81: bool,
    pub(crate) scalar_v82: bool,
    pub(crate) scalar_v83: bool,
    pub(crate) scalar_v84: bool,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: bool,
    pub(crate) scalar_v92: bool,
    pub(crate) scalar_v93: bool,
    pub(crate) scalar_v94: bool,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: bool,
    pub(crate) scalar_v102: bool,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: bool,
    pub(crate) scalar_v107: bool,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: bool,
    pub(crate) scalar_v115: bool,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: bool,
    pub(crate) scalar_v121: bool,
    pub(crate) scalar_v122: bool,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: bool,
    pub(crate) scalar_v130: bool,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: bool,
    pub(crate) scalar_v138: bool,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: bool,
    pub(crate) scalar_v146: bool,
    pub(crate) scalar_v147: bool,
    pub(crate) scalar_v148: bool,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: bool,
    pub(crate) scalar_v153: bool,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: bool,
    pub(crate) scalar_v156: bool,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: bool,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: bool,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: bool,
    pub(crate) scalar_v170: bool,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: bool,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: bool,
    pub(crate) scalar_v183: bool,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v271: bool,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v298: bool,
    pub(crate) scalar_v308: bool,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
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
            scalar_v0: self.scalar_v0,
            scalar_v1: self.scalar_v1,
            scalar_v3: self.scalar_v3,
            scalar_v4: self.scalar_v4,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v9: self.scalar_v9,
            scalar_v10: self.scalar_v10,
            scalar_v11: self.scalar_v11,
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v17: self.scalar_v17,
            scalar_v19: self.scalar_v19,
            scalar_v20: self.scalar_v20,
            scalar_v22: self.scalar_v22,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v45: self.scalar_v45,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v72: self.scalar_v72,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v77: self.scalar_v77,
            scalar_v78: self.scalar_v78,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v83: self.scalar_v83,
            scalar_v84: self.scalar_v84,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v87: self.scalar_v87,
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v146: self.scalar_v146,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v181: self.scalar_v181,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v184: self.scalar_v184,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v189: self.scalar_v189,
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v192: self.scalar_v192,
            scalar_v193: self.scalar_v193,
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v198: self.scalar_v198,
            scalar_v199: self.scalar_v199,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v204: self.scalar_v204,
            scalar_v205: self.scalar_v205,
            scalar_v206: self.scalar_v206,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v209: self.scalar_v209,
            scalar_v210: self.scalar_v210,
            scalar_v211: self.scalar_v211,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v215: self.scalar_v215,
            scalar_v216: self.scalar_v216,
            scalar_v217: self.scalar_v217,
            scalar_v218: self.scalar_v218,
            scalar_v219: self.scalar_v219,
            scalar_v220: self.scalar_v220,
            scalar_v221: self.scalar_v221,
            scalar_v222: self.scalar_v222,
            scalar_v224: self.scalar_v224,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v271: self.scalar_v271,
            scalar_v274: self.scalar_v274,
            scalar_v281: self.scalar_v281,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v298: self.scalar_v298,
            scalar_v308: self.scalar_v308,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v374: self.scalar_v374,
            scalar_v378: self.scalar_v378,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 3;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 50;
    pub const VARIABLE_COUNT: usize = 102;
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
            scalar_v0: 0.0,
            scalar_v1: 0.0,
            scalar_v3: 0.0,
            scalar_v4: false,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v9: 0.0,
            scalar_v10: 0.0,
            scalar_v11: 0.0,
            scalar_v12: 0.0,
            scalar_v13: false,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v17: 0.0,
            scalar_v19: 0.0,
            scalar_v20: 0.0,
            scalar_v22: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v27: false,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: false,
            scalar_v31: false,
            scalar_v32: false,
            scalar_v34: 0.0,
            scalar_v35: 0.0,
            scalar_v36: false,
            scalar_v37: false,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v40: 0.0,
            scalar_v41: false,
            scalar_v42: 0.0,
            scalar_v43: false,
            scalar_v44: false,
            scalar_v45: 0.0,
            scalar_v46: false,
            scalar_v47: 0.0,
            scalar_v48: false,
            scalar_v49: false,
            scalar_v50: false,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v59: false,
            scalar_v60: false,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
            scalar_v65: false,
            scalar_v66: false,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: 0.0,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v73: 0.0,
            scalar_v74: false,
            scalar_v75: false,
            scalar_v76: 0.0,
            scalar_v77: 0.0,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: false,
            scalar_v81: false,
            scalar_v82: false,
            scalar_v83: false,
            scalar_v84: false,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: false,
            scalar_v92: false,
            scalar_v93: false,
            scalar_v94: false,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v101: false,
            scalar_v102: false,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: false,
            scalar_v107: false,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: false,
            scalar_v115: false,
            scalar_v116: 0.0,
            scalar_v117: 0.0,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: false,
            scalar_v121: false,
            scalar_v122: false,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v128: 0.0,
            scalar_v129: false,
            scalar_v130: false,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: false,
            scalar_v138: false,
            scalar_v139: 0.0,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v144: 0.0,
            scalar_v145: false,
            scalar_v146: false,
            scalar_v147: false,
            scalar_v148: false,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v152: false,
            scalar_v153: false,
            scalar_v154: 0.0,
            scalar_v155: false,
            scalar_v156: false,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v160: 0.0,
            scalar_v161: 0.0,
            scalar_v162: false,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: false,
            scalar_v166: 0.0,
            scalar_v167: false,
            scalar_v168: 0.0,
            scalar_v169: false,
            scalar_v170: false,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: false,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: false,
            scalar_v183: false,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: 0.0,
            scalar_v195: 0.0,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
            scalar_v198: 0.0,
            scalar_v199: 0.0,
            scalar_v201: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v204: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v210: 0.0,
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
            scalar_v219: 0.0,
            scalar_v220: 0.0,
            scalar_v221: 0.0,
            scalar_v222: 0.0,
            scalar_v224: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v271: false,
            scalar_v274: 0.0,
            scalar_v281: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v298: false,
            scalar_v308: false,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v338: 0.0,
            scalar_v374: 0.0,
            scalar_v378: 0.0,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
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
            scalar_v0,
            scalar_v1,
            scalar_v3,
            scalar_v4,
            scalar_v6,
            scalar_v7,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v73,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
            scalar_v84,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v210,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v219,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v224,
            scalar_v227,
            scalar_v228,
            scalar_v235,
            scalar_v236,
            scalar_v271,
            scalar_v274,
            scalar_v281,
            scalar_v291,
            scalar_v292,
            scalar_v298,
            scalar_v308,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v374,
            scalar_v378,
            scalar_v23,
            scalar_v24,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
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
            scalar_v0,
            scalar_v1,
            scalar_v3,
            scalar_v4,
            scalar_v6,
            scalar_v7,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v73,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
            scalar_v84,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v210,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v219,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v224,
            scalar_v227,
            scalar_v228,
            scalar_v235,
            scalar_v236,
            scalar_v271,
            scalar_v274,
            scalar_v281,
            scalar_v291,
            scalar_v292,
            scalar_v298,
            scalar_v308,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v374,
            scalar_v378,
            scalar_v23,
            scalar_v24,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "r" => { validate_parameter("r", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c1" => { validate_parameter("c1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c2" => { validate_parameter("c2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dra" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isnoisy" => { validate_parameter("isnoisy", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_et" => { validate_parameter("sw_et", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthresh" => { validate_parameter("rthresh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "level" => { validate_finite_parameter("level", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmin" => { validate_parameter("lmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmax" => { validate_parameter("lmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmin" => { validate_parameter("wmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmax" => { validate_parameter("wmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xw" => { validate_finite_parameter("xw", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xl" => { validate_finite_parameter("xl", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dxle" => { validate_finite_parameter("dxle", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_efgeo" => { validate_parameter("sw_efgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "q3" => { validate_parameter("q3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p3" => { validate_parameter("p3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "q2" => { validate_parameter("q2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p2" => { validate_parameter("p2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_fngeo" => { validate_parameter("sw_fngeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "jmax" => { validate_parameter("jmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1" => { validate_finite_parameter("tc1", value)?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2" => { validate_finite_parameter("tc2", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1l" => { validate_finite_parameter("tc1l", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2l" => { validate_finite_parameter("tc2l", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1w" => { validate_finite_parameter("tc1w", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2w" => { validate_finite_parameter("tc2w", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1kfn" => { validate_finite_parameter("tc1kfn", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gth0" => { validate_parameter("gth0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gthp" => { validate_parameter("gthp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gtha" => { validate_parameter("gtha", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cthp" => { validate_parameter("cthp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctha" => { validate_parameter("ctha", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'r2_et_cmc'", name)),
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
        let param_given = self.param_given.as_ref();
        let v0: f64 = if param_given[10] { 1.0 } else { 0.0 };
        self.scalar_v0 = v0;
        let v1: f64 = p.p10;
        self.scalar_v1 = v1;
        let v3: f64 = (if (if param_given[10] { 1.0 } else { 0.0 } != 0.0) { p.p10 } else { 0.0 });
        self.scalar_v3 = v3;
        let v4: bool = (!(if param_given[10] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v4 = v4;
        let v6: f64 = (if v4 { 1.0 } else { v3 });
        self.scalar_v6 = v6;
        let v7: f64 = if param_given[11] { 1.0 } else { 0.0 };
        self.scalar_v7 = v7;
        let v9: f64 = p.p11;
        self.scalar_v9 = v9;
        let v10: f64 = (0.01 * p.p11);
        self.scalar_v10 = v10;
        let v11: f64 = (1.0 - v10);
        self.scalar_v11 = v11;
        let v12: f64 = (if (if param_given[11] { 1.0 } else { 0.0 } != 0.0) { v11 } else { 0.0 });
        self.scalar_v12 = v12;
        let v13: bool = (!(if param_given[11] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v13 = v13;
        let v14: f64 = (if v13 { 1.0 } else { v12 });
        self.scalar_v14 = v14;
        let v15: f64 = (v6 * v14);
        self.scalar_v15 = v15;
        let v17: f64 = (v15 * 1000000.0);
        self.scalar_v17 = v17;
        let v19: f64 = p.p16;
        self.scalar_v19 = v19;
        let v20: f64 = (273.15 + p.p16);
        self.scalar_v20 = v20;
        let v22: f64 = p.p5;
        self.scalar_v22 = v22;
        let v25: f64 = p.p3;
        self.scalar_v25 = v25;
        let v26: f64 = p.p4;
        self.scalar_v26 = v26;
        let v27: bool = ((p.p3 != 0.0) && (p.p4 != 0.0));
        self.scalar_v27 = v27;
        let v28: f64 = p.p23;
        self.scalar_v28 = v28;
        let v29: f64 = (if v27 { p.p23 } else { 0.0 });
        self.scalar_v29 = v29;
        let v30: bool = ((p.p3 != 0.0) || (p.p4 != 0.0));
        self.scalar_v30 = v30;
        let v31: bool = (!v27);
        self.scalar_v31 = v31;
        let v32: bool = (v30 && v31);
        self.scalar_v32 = v32;
        let v34: f64 = (p.p23 * 0.5);
        self.scalar_v34 = v34;
        let v35: f64 = (if v32 { v34 } else { v29 });
        self.scalar_v35 = v35;
        let v36: bool = (!v30);
        self.scalar_v36 = v36;
        let v37: bool = (v31 && v36);
        self.scalar_v37 = v37;
        let v38: f64 = (if v37 { 0.0 } else { v35 });
        self.scalar_v38 = v38;
        let v39: f64 = if param_given[1] { 1.0 } else { 0.0 };
        self.scalar_v39 = v39;
        let v40: f64 = if param_given[2] { 1.0 } else { 0.0 };
        self.scalar_v40 = v40;
        let v41: bool = ((if param_given[1] { 1.0 } else { 0.0 } != 0.0) && (if param_given[2] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v41 = v41;
        let v42: f64 = if param_given[0] { 1.0 } else { 0.0 };
        self.scalar_v42 = v42;
        let v43: bool = (!(if param_given[0] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v43 = v43;
        let v44: bool = (v41 && v43);
        self.scalar_v44 = v44;
        let v45: f64 = p.p2;
        self.scalar_v45 = v45;
        let v46: bool = (0.0 == p.p2);
        self.scalar_v46 = v46;
        let v47: f64 = p.p1;
        self.scalar_v47 = v47;
        let v48: bool = (0.0 == p.p1);
        self.scalar_v48 = v48;
        let v49: bool = (v46 || v48);
        self.scalar_v49 = v49;
        let v50: bool = (v44 && v49);
        self.scalar_v50 = v50;
        let v51: f64 = (if v50 { 0.0 } else { 0.0 });
        self.scalar_v51 = v51;
        let v52: f64 = p.p0;
        self.scalar_v52 = v52;
        let v53: f64 = (v17 * p.p0);
        self.scalar_v53 = v53;
        let v54: f64 = (if v50 { v53 } else { 0.0 });
        self.scalar_v54 = v54;
        let v55: f64 = p.p22;
        self.scalar_v55 = v55;
        let v56: f64 = (v54 + p.p22);
        self.scalar_v56 = v56;
        let v57: f64 = (if v50 { v56 } else { 0.0 });
        self.scalar_v57 = v57;
        let v59: bool = (!v49);
        self.scalar_v59 = v59;
        let v60: bool = (v44 && v59);
        self.scalar_v60 = v60;
        let v61: f64 = (v17 * p.p1);
        self.scalar_v61 = v61;
        let v62: f64 = (if v60 { v61 } else { v51 });
        self.scalar_v62 = v62;
        let v63: f64 = (v38 + v62);
        self.scalar_v63 = v63;
        let v64: f64 = (if v60 { v63 } else { v51 });
        self.scalar_v64 = v64;
        let v65: bool = (v64 > 0.0);
        self.scalar_v65 = v65;
        let v66: bool = (v60 && v65);
        self.scalar_v66 = v66;
        let v67: f64 = p.p17;
        self.scalar_v67 = v67;
        let v68: f64 = (p.p17 / p.p2);
        self.scalar_v68 = v68;
        let v69: f64 = (v64 * v68);
        self.scalar_v69 = v69;
        let v70: f64 = (if v66 { v69 } else { v57 });
        self.scalar_v70 = v70;
        let v71: f64 = (v70 - p.p22);
        self.scalar_v71 = v71;
        let v72: f64 = (if v66 { v71 } else { v54 });
        self.scalar_v72 = v72;
        let v73: f64 = (if v66 { p.p2 } else { v51 });
        self.scalar_v73 = v73;
        let v74: bool = (!v65);
        self.scalar_v74 = v74;
        let v75: bool = (v60 && v74);
        self.scalar_v75 = v75;
        let v76: f64 = (if v75 { v53 } else { v72 });
        self.scalar_v76 = v76;
        let v77: f64 = (p.p22 + v76);
        self.scalar_v77 = v77;
        let v78: f64 = (if v75 { v77 } else { v70 });
        self.scalar_v78 = v78;
        let v79: f64 = (if v75 { 0.0 } else { v73 });
        self.scalar_v79 = v79;
        let v80: bool = (!(if param_given[1] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v80 = v80;
        let v81: bool = ((if param_given[2] { 1.0 } else { 0.0 } != 0.0) && v80);
        self.scalar_v81 = v81;
        let v82: bool = (!v44);
        self.scalar_v82 = v82;
        let v83: bool = (v81 && v82);
        self.scalar_v83 = v83;
        let v84: bool = (v46 && v83);
        self.scalar_v84 = v84;
        let v85: f64 = (if v84 { 0.0 } else { v62 });
        self.scalar_v85 = v85;
        let v86: f64 = (if v84 { 0.0 } else { v64 });
        self.scalar_v86 = v86;
        let v87: f64 = (if v84 { v53 } else { v76 });
        self.scalar_v87 = v87;
        let v88: f64 = (p.p22 + v87);
        self.scalar_v88 = v88;
        let v89: f64 = (if v84 { v88 } else { v78 });
        self.scalar_v89 = v89;
        let v90: f64 = (if v84 { 0.0 } else { v79 });
        self.scalar_v90 = v90;
        let v91: bool = (0.0 == p.p0);
        self.scalar_v91 = v91;
        let v92: bool = (!v46);
        self.scalar_v92 = v92;
        let v93: bool = (v83 && v92);
        self.scalar_v93 = v93;
        let v94: bool = (v91 && v93);
        self.scalar_v94 = v94;
        let v95: f64 = (if v94 { 0.0 } else { v87 });
        self.scalar_v95 = v95;
        let v96: f64 = (if v94 { 0.0 } else { v89 });
        self.scalar_v96 = v96;
        let v97: f64 = (if v94 { v61 } else { v85 });
        self.scalar_v97 = v97;
        let v98: f64 = (v38 + v97);
        self.scalar_v98 = v98;
        let v99: f64 = (if v94 { v98 } else { v86 });
        self.scalar_v99 = v99;
        let v100: f64 = (if v94 { 1e99 } else { v90 });
        self.scalar_v100 = v100;
        let v101: bool = (!v91);
        self.scalar_v101 = v101;
        let v102: bool = (v93 && v101);
        self.scalar_v102 = v102;
        let v103: f64 = (if v102 { v53 } else { v95 });
        self.scalar_v103 = v103;
        let v104: f64 = (p.p22 + v103);
        self.scalar_v104 = v104;
        let v105: f64 = (if v102 { v104 } else { v96 });
        self.scalar_v105 = v105;
        let v106: bool = (v105 > 0.0);
        self.scalar_v106 = v106;
        let v107: bool = (v102 && v106);
        self.scalar_v107 = v107;
        let v108: f64 = (p.p2 / p.p17);
        self.scalar_v108 = v108;
        let v109: f64 = (v105 * v108);
        self.scalar_v109 = v109;
        let v110: f64 = (if v107 { v109 } else { v99 });
        self.scalar_v110 = v110;
        let v111: f64 = (v110 - v38);
        self.scalar_v111 = v111;
        let v112: f64 = (if v107 { v111 } else { v97 });
        self.scalar_v112 = v112;
        let v113: f64 = (if v107 { p.p2 } else { v100 });
        self.scalar_v113 = v113;
        let v114: bool = (!v106);
        self.scalar_v114 = v114;
        let v115: bool = (v102 && v114);
        self.scalar_v115 = v115;
        let v116: f64 = (if v115 { v61 } else { v112 });
        self.scalar_v116 = v116;
        let v117: f64 = (v38 + v116);
        self.scalar_v117 = v117;
        let v118: f64 = (if v115 { v117 } else { v110 });
        self.scalar_v118 = v118;
        let v119: f64 = (if v115 { 1e99 } else { v113 });
        self.scalar_v119 = v119;
        let v120: bool = (!v81);
        self.scalar_v120 = v120;
        let v121: bool = (v82 && v120);
        self.scalar_v121 = v121;
        let v122: bool = (v91 && v121);
        self.scalar_v122 = v122;
        let v123: f64 = (if v122 { 0.0 } else { v103 });
        self.scalar_v123 = v123;
        let v124: f64 = (if v122 { 0.0 } else { v105 });
        self.scalar_v124 = v124;
        let v125: f64 = (if v122 { v61 } else { v116 });
        self.scalar_v125 = v125;
        let v126: f64 = (v38 + v125);
        self.scalar_v126 = v126;
        let v127: f64 = (if v122 { v126 } else { v118 });
        self.scalar_v127 = v127;
        let v128: f64 = (if v122 { 1e99 } else { v119 });
        self.scalar_v128 = v128;
        let v129: bool = (v101 && v121);
        self.scalar_v129 = v129;
        let v130: bool = (v48 && v129);
        self.scalar_v130 = v130;
        let v131: f64 = (if v130 { 0.0 } else { v125 });
        self.scalar_v131 = v131;
        let v132: f64 = (if v130 { 0.0 } else { v127 });
        self.scalar_v132 = v132;
        let v133: f64 = (if v130 { v53 } else { v123 });
        self.scalar_v133 = v133;
        let v134: f64 = (p.p22 + v133);
        self.scalar_v134 = v134;
        let v135: f64 = (if v130 { v134 } else { v124 });
        self.scalar_v135 = v135;
        let v136: f64 = (if v130 { 0.0 } else { v128 });
        self.scalar_v136 = v136;
        let v137: bool = (!v48);
        self.scalar_v137 = v137;
        let v138: bool = (v129 && v137);
        self.scalar_v138 = v138;
        let v139: f64 = (if v138 { v53 } else { v133 });
        self.scalar_v139 = v139;
        let v140: f64 = (p.p22 + v139);
        self.scalar_v140 = v140;
        let v141: f64 = (if v138 { v140 } else { v135 });
        self.scalar_v141 = v141;
        let v142: f64 = (if v138 { v61 } else { v131 });
        self.scalar_v142 = v142;
        let v143: f64 = (v38 + v142);
        self.scalar_v143 = v143;
        let v144: f64 = (if v138 { v143 } else { v132 });
        self.scalar_v144 = v144;
        let v145: bool = (v141 > 0.0);
        self.scalar_v145 = v145;
        let v146: bool = (v144 > 0.0);
        self.scalar_v146 = v146;
        let v147: bool = (v138 && v145);
        self.scalar_v147 = v147;
        let v148: bool = (v146 && v147);
        self.scalar_v148 = v148;
        let v149: f64 = (v144 / v141);
        self.scalar_v149 = v149;
        let v150: f64 = (p.p17 * v149);
        self.scalar_v150 = v150;
        let v151: f64 = (if v148 { v150 } else { v136 });
        self.scalar_v151 = v151;
        let v152: bool = (!v146);
        self.scalar_v152 = v152;
        let v153: bool = (v147 && v152);
        self.scalar_v153 = v153;
        let v154: f64 = (if v153 { 0.0 } else { v151 });
        self.scalar_v154 = v154;
        let v155: bool = (!v145);
        self.scalar_v155 = v155;
        let v156: bool = (v138 && v155);
        self.scalar_v156 = v156;
        let v157: f64 = (if v156 { 1e99 } else { v154 });
        self.scalar_v157 = v157;
        let v158: f64 = p.p25;
        self.scalar_v158 = v158;
        let v159: f64 = p.p24;
        self.scalar_v159 = v159;
        let v160: f64 = (v144 + p.p24);
        self.scalar_v160 = v160;
        let v161: f64 = (if (p.p25 != 0.0) { v160 } else { 0.0 });
        self.scalar_v161 = v161;
        let v162: bool = (!(p.p25 != 0.0));
        self.scalar_v162 = v162;
        let v163: f64 = (v142 + p.p24);
        self.scalar_v163 = v163;
        let v164: f64 = (if v162 { v163 } else { v161 });
        self.scalar_v164 = v164;
        let v165: bool = (v157 > 0.0);
        self.scalar_v165 = v165;
        let v166: f64 = p.p29;
        self.scalar_v166 = v166;
        let v167: bool = (p.p29 > 0.0);
        self.scalar_v167 = v167;
        let v168: f64 = p.p27;
        self.scalar_v168 = v168;
        let v169: bool = (p.p27 > 0.0);
        self.scalar_v169 = v169;
        let v170: bool = (v167 || v169);
        self.scalar_v170 = v170;
        let v171: f64 = p.p37;
        self.scalar_v171 = v171;
        let v172: f64 = p.p38;
        self.scalar_v172 = v172;
        let v173: bool = (v27 && v146);
        self.scalar_v173 = v173;
        let v174: f64 = p.p39;
        self.scalar_v174 = v174;
        let v175: f64 = (p.p39 / v144);
        self.scalar_v175 = v175;
        let v176: f64 = (p.p37 + v175);
        self.scalar_v176 = v176;
        let v177: f64 = (if v173 { v176 } else { p.p37 });
        self.scalar_v177 = v177;
        let v178: f64 = p.p40;
        self.scalar_v178 = v178;
        let v179: f64 = (p.p40 / v144);
        self.scalar_v179 = v179;
        let v180: f64 = (p.p38 + v179);
        self.scalar_v180 = v180;
        let v181: f64 = (if v173 { v180 } else { p.p38 });
        self.scalar_v181 = v181;
        let v182: bool = (v31 && v146);
        self.scalar_v182 = v182;
        let v183: bool = (v30 && v182);
        self.scalar_v183 = v183;
        let v184: f64 = (0.5 * p.p39);
        self.scalar_v184 = v184;
        let v185: f64 = (v184 / v144);
        self.scalar_v185 = v185;
        let v186: f64 = (v177 + v185);
        self.scalar_v186 = v186;
        let v187: f64 = (if v183 { v186 } else { v177 });
        self.scalar_v187 = v187;
        let v188: f64 = (0.5 * p.p40);
        self.scalar_v188 = v188;
        let v189: f64 = (v188 / v144);
        self.scalar_v189 = v189;
        let v190: f64 = (v181 + v189);
        self.scalar_v190 = v190;
        let v191: f64 = (if v183 { v190 } else { v181 });
        self.scalar_v191 = v191;
        let v192: f64 = p.p41;
        self.scalar_v192 = v192;
        let v193: f64 = (p.p41 / v141);
        self.scalar_v193 = v193;
        let v194: f64 = (v187 + v193);
        self.scalar_v194 = v194;
        let v195: f64 = (if v145 { v194 } else { v187 });
        self.scalar_v195 = v195;
        let v196: f64 = p.p42;
        self.scalar_v196 = v196;
        let v197: f64 = (p.p42 / v141);
        self.scalar_v197 = v197;
        let v198: f64 = (v191 + v197);
        self.scalar_v198 = v198;
        let v199: f64 = (if v145 { v198 } else { v191 });
        self.scalar_v199 = v199;
        let v201: f64 = (v139 + v142);
        self.scalar_v201 = v201;
        let v202: f64 = (2.0 * v201);
        self.scalar_v202 = v202;
        let v203: f64 = (if v27 { v202 } else { 0.0 });
        self.scalar_v203 = v203;
        let v204: f64 = (v142 * 2.0);
        self.scalar_v204 = v204;
        let v205: f64 = (v139 + v204);
        self.scalar_v205 = v205;
        let v206: f64 = (if v32 { v205 } else { v203 });
        self.scalar_v206 = v206;
        let v207: f64 = (if v37 { v204 } else { v206 });
        self.scalar_v207 = v207;
        let v208: f64 = (v139 * v142);
        self.scalar_v208 = v208;
        let v209: f64 = p.p44;
        self.scalar_v209 = v209;
        let v210: f64 = p.p45;
        self.scalar_v210 = v210;
        let v211: f64 = (v207 * p.p45);
        self.scalar_v211 = v211;
        let v212: f64 = (p.p44 + v211);
        self.scalar_v212 = v212;
        let v213: f64 = p.p46;
        self.scalar_v213 = v213;
        let v214: f64 = (v208 * p.p46);
        self.scalar_v214 = v214;
        let v215: f64 = (v212 + v214);
        self.scalar_v215 = v215;
        let v216: f64 = p.p47;
        self.scalar_v216 = v216;
        let v217: f64 = p.p48;
        self.scalar_v217 = v217;
        let v218: f64 = (v207 * p.p48);
        self.scalar_v218 = v218;
        let v219: f64 = (p.p47 + v218);
        self.scalar_v219 = v219;
        let v220: f64 = p.p49;
        self.scalar_v220 = v220;
        let v221: f64 = (v208 * p.p49);
        self.scalar_v221 = v221;
        let v222: f64 = (v219 + v221);
        self.scalar_v222 = v222;
        let v224: f64 = p.p7;
        self.scalar_v224 = v224;
        let v227: f64 = p.p35;
        self.scalar_v227 = v227;
        let v228: f64 = (1.0 + p.p35);
        self.scalar_v228 = v228;
        let v235: f64 = p.p36;
        self.scalar_v235 = v235;
        let v236: f64 = (p.p36 - 1.0);
        self.scalar_v236 = v236;
        let v271: bool = (v165 && v170);
        self.scalar_v271 = v271;
        let v274: f64 = p.p28;
        self.scalar_v274 = v274;
        let v281: f64 = p.p26;
        self.scalar_v281 = v281;
        let v291: f64 = (1.0 - p.p29);
        self.scalar_v291 = v291;
        let v292: f64 = (v291 - p.p27);
        self.scalar_v292 = v292;
        let v298: bool = (!v271);
        self.scalar_v298 = v298;
        let v308: bool = (!(p.p7 != 0.0));
        self.scalar_v308 = v308;
        let v331: f64 = (1.0 / v164);
        self.scalar_v331 = v331;
        let v332: f64 = (-1.0 / v164);
        self.scalar_v332 = v332;
        let v333: f64 = (if v271 { v331 } else { 0.0 });
        self.scalar_v333 = v333;
        let v334: f64 = (if v271 { v332 } else { 0.0 });
        self.scalar_v334 = v334;
        let v335: f64 = (p.p28 * v333);
        self.scalar_v335 = v335;
        let v336: f64 = (p.p28 * v334);
        self.scalar_v336 = v336;
        let v337: f64 = (if v271 { v335 } else { 0.0 });
        self.scalar_v337 = v337;
        let v338: f64 = (if v271 { v336 } else { 0.0 });
        self.scalar_v338 = v338;
        let v374: f64 = (if (p.p7 != 0.0) { v215 } else { 0.0 });
        self.scalar_v374 = v374;
        let v378: f64 = (if v308 { 1000000.0 } else { 0.0 });
        self.scalar_v378 = v378;
    }

    #[inline]
    fn invalidate_temperature_static(&mut self) {
        self.scalar_temperature_static_valid = false;
    }

    #[inline]
    pub(super) fn ensure_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {
        if !self.scalar_temperature_static_valid
            || self.scalar_temperature_static_temperature.to_bits() != temperature.to_bits()
            || self.scalar_temperature_static_thermal_voltage.to_bits() != thermal_voltage.to_bits()
        {
            self.recompute_temperature_static(temperature, thermal_voltage);
        }
    }

    #[inline]
    fn recompute_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {
        let p = &(*self.params);
        let v23: f64 = (temperature + self.scalar_v22);
        self.scalar_v23 = v23;
        let v24: f64 = (self.scalar_v23 - 273.15);
        self.scalar_v24 = v24;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
