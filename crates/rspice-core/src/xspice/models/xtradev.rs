//! XSPICE xtradev code models.

use super::analog::{climit_transfer, smooth_corner, smooth_discontinuity};
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec,
    PortType,
};
use crate::{Complex64, Value};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

const CORE_TABLE_RESOURCE: &str = "xspice.xtradev.core.table";
const ILIMIT_EVAL_RESOURCE: &str = "xspice.xtradev.ilimit.eval";
const CORE_TABLE_UNSET_MIDPOINT_INDEX: usize = usize::MAX;
const CORE_TABLE_CURSOR_LINEAR_STEPS: usize = 8;

#[derive(Debug, Default)]
pub struct Potentiometer;

#[derive(Debug, Default)]
pub struct AnalogSwitch;

#[derive(Debug, Default)]
pub struct Pswitch;

#[derive(Debug, Default)]
pub struct Sidiode;

#[derive(Debug, Default)]
pub struct Zener;

#[derive(Debug, Default)]
pub struct Memristor;

#[derive(Debug, Default)]
pub struct Core;

#[derive(Debug, Default)]
pub struct CapacitorIc;

#[derive(Debug, Default)]
pub struct InductorIc;

#[derive(Debug, Default)]
pub struct CapacitanceMeter;

#[derive(Debug, Default)]
pub struct InductanceMeter;

#[derive(Debug, Default)]
pub struct LcCouple;

#[derive(Debug, Default)]
pub struct Ilimit;

#[derive(Debug, Default)]
pub struct SeeGenerator;

#[derive(Debug, Clone, Copy)]
struct PotentiometerSplit {
    r_lower: Value,
    r_upper: Value,
    g_lower: Value,
    g_upper: Value,
}

#[derive(Debug, Clone, Copy)]
struct AswitchEval {
    output_conductance: Value,
    control_partial: Value,
    output_current: Value,
}

#[derive(Debug, Clone, Copy)]
struct PswitchEval {
    resistance: Value,
    output_conductance: Value,
    control_partial: Value,
    control_conductance: Value,
    output_current: Value,
    control_current: Value,
}

#[derive(Debug, Clone, Copy)]
struct SidiodeEval {
    current: Value,
    derivative: Value,
}

#[derive(Debug, Clone, Copy)]
struct ZenerEval {
    current: Value,
    derivative: Value,
}

#[derive(Debug, Clone, Copy)]
struct MemristorParams {
    rmin: Value,
    rmax: Value,
    rinit: Value,
    alpha: Value,
    beta: Value,
    vt: Value,
}

#[derive(Debug, Clone, Copy)]
struct MemristorEval {
    current: Value,
    derivative: Value,
}

#[derive(Debug, Clone, Copy)]
struct CorePoint {
    h: Value,
    b: Value,
}

#[derive(Debug)]
struct CoreTable {
    points: Vec<CorePoint>,
    midpoints: Vec<Value>,
    strictly_increasing_h: bool,
    last_midpoint_index: AtomicUsize,
}

impl Clone for CoreTable {
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
            midpoints: self.midpoints.clone(),
            strictly_increasing_h: self.strictly_increasing_h,
            last_midpoint_index: AtomicUsize::new(CORE_TABLE_UNSET_MIDPOINT_INDEX),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoreTableSignature {
    h_revision: Option<u64>,
    b_revision: Option<u64>,
}

#[derive(Debug, Clone)]
struct CoreTableResource {
    signature: CoreTableSignature,
    data: CmResult<Arc<CoreTable>>,
}

#[derive(Debug, Clone, Copy)]
struct CoreEval {
    current: Value,
    derivative: Value,
}

#[derive(Debug, Clone, Copy)]
struct CoreHysteresisParams {
    in_low: Value,
    in_high: Value,
    hyst: Value,
    out_lower_limit: Value,
    out_upper_limit: Value,
    input_domain: Value,
}

pub(crate) const XTRADEV_METER_MEASURED_VALUE_PARAM: &str = "__rspice_measured_value";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XtradevPortKey {
    R0,
    R1,
    Wiper,
    Out,
    CntlIn,
    MmfOut,
    L,
    PosPwr,
    NegPwr,
    Other,
}

#[inline]
fn xtradev_port_key(output_port: &str) -> XtradevPortKey {
    if output_port.eq_ignore_ascii_case("r0") {
        XtradevPortKey::R0
    } else if output_port.eq_ignore_ascii_case("r1") {
        XtradevPortKey::R1
    } else if output_port.eq_ignore_ascii_case("wiper") {
        XtradevPortKey::Wiper
    } else if output_port.eq_ignore_ascii_case("out") {
        XtradevPortKey::Out
    } else if output_port.eq_ignore_ascii_case("cntl_in") {
        XtradevPortKey::CntlIn
    } else if output_port.eq_ignore_ascii_case("mmf_out") {
        XtradevPortKey::MmfOut
    } else if output_port.eq_ignore_ascii_case("l") {
        XtradevPortKey::L
    } else if output_port.eq_ignore_ascii_case("pos_pwr") {
        XtradevPortKey::PosPwr
    } else if output_port.eq_ignore_ascii_case("neg_pwr") {
        XtradevPortKey::NegPwr
    } else {
        XtradevPortKey::Other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IlimitEval {
    out_current: Value,
    out_in_partial: Value,
    out_out_partial: Value,
    out_pos_partial: Value,
    out_neg_partial: Value,
    pos_current: Value,
    pos_in_partial: Value,
    pos_out_partial: Value,
    pos_pos_partial: Value,
    pos_neg_partial: Value,
    neg_current: Value,
    neg_in_partial: Value,
    neg_out_partial: Value,
    neg_pos_partial: Value,
    neg_neg_partial: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct IlimitEvalSignature {
    in_offset: Value,
    gain: Value,
    r_out_source: Value,
    r_out_sink: Value,
    i_limit_source: Value,
    i_limit_sink: Value,
    v_pwr_range: Value,
    i_source_range: Value,
    i_sink_range: Value,
    r_out_domain: Value,
    input: Value,
    output: Value,
    pos_pwr: Value,
    neg_pwr: Value,
    pos_pwr_connected: bool,
    neg_pwr_connected: bool,
    init: bool,
}

#[derive(Debug, Clone, Copy)]
struct IlimitEvalResource {
    signature: IlimitEvalSignature,
    eval: IlimitEval,
}

#[derive(Debug, Clone, Copy)]
struct SeeGeneratorParams {
    tfall: Value,
    trise: Value,
    tdelay: Value,
    tperiod: Value,
    inull: Value,
    let_value: Value,
    cdepth: Value,
    angle: Value,
    ctrlthres: Value,
    perlim: bool,
}

fn invalid_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn inout_conductance_port(name: &str, description: &str) -> PortSpec {
    PortSpec {
        name: name.to_string(),
        direction: PortDirection::InOut,
        default_type: PortType::Conductance,
        allowed_types: vec![PortType::Conductance],
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: description.to_string(),
    }
}

fn gd_port(
    name: &str,
    description: &str,
    allowed_types: Vec<PortType>,
    default_type: PortType,
) -> PortSpec {
    PortSpec {
        name: name.to_string(),
        direction: PortDirection::InOut,
        default_type,
        allowed_types,
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: description.to_string(),
    }
}

fn capacitoric_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![PortSpec {
            name: "cap".to_string(),
            direction: PortDirection::InOut,
            default_type: PortType::DifferentialHybrid,
            allowed_types: vec![PortType::DifferentialHybrid],
            is_vector: false,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: "Capacitor terminals".to_string(),
        }]
    })
}

fn inductoric_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![PortSpec {
            name: "ind".to_string(),
            direction: PortDirection::InOut,
            default_type: PortType::DifferentialConductance,
            allowed_types: vec![PortType::DifferentialConductance],
            is_vector: false,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: "Inductor terminals".to_string(),
        }]
    })
}

fn capacitoric_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("c", 0.0)
                .required()
                .with_description("Capacitance"),
            ParamSpec::real("ic", 0.0).with_description("Voltage initial condition"),
        ]
    })
}

fn inductoric_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("l", 0.0)
                .required()
                .with_description("Inductance"),
            ParamSpec::real("ic", 0.0).with_description("Current initial condition"),
        ]
    })
}

fn potentiometer_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            inout_conductance_port("r0", "Potentiometer connection 0"),
            inout_conductance_port("wiper", "Wiper contact"),
            inout_conductance_port("r1", "Potentiometer connection 1"),
        ]
    })
}

fn aswitch_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "cntl_in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![
                    PortType::Voltage,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                    PortType::VoltageName,
                ],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Control input".to_string(),
            },
            gd_port(
                "out",
                "Resistive output conductance port",
                vec![PortType::DifferentialConductance],
                PortType::DifferentialConductance,
            ),
        ]
    })
}

fn pswitch_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            gd_port(
                "cntl_in",
                "Control input conductance port",
                vec![PortType::Conductance, PortType::DifferentialConductance],
                PortType::DifferentialConductance,
            ),
            gd_port(
                "out",
                "Resistive output conductance port",
                vec![PortType::DifferentialConductance],
                PortType::DifferentialConductance,
            ),
        ]
    })
}

fn sidiode_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![gd_port(
            "ds",
            "Diode conductance port",
            vec![PortType::DifferentialConductance],
            PortType::DifferentialConductance,
        )]
    })
}

fn zener_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![gd_port(
            "z",
            "Zener conductance port",
            vec![PortType::DifferentialConductance],
            PortType::DifferentialConductance,
        )]
    })
}

fn memristor_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![gd_port(
            "memris",
            "Memristor conductance port",
            vec![PortType::DifferentialConductance],
            PortType::DifferentialConductance,
        )]
    })
}

fn core_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![gd_port(
            "mc",
            "Magnetic core conductance port",
            vec![PortType::Conductance, PortType::DifferentialConductance],
            PortType::DifferentialConductance,
        )]
    })
}

fn meter_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![PortType::Voltage, PortType::DifferentialVoltage],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Meter input node".to_string(),
            },
            PortSpec {
                name: "out".to_string(),
                direction: PortDirection::Out,
                default_type: PortType::Voltage,
                allowed_types: vec![
                    PortType::Voltage,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                ],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Meter output".to_string(),
            },
        ]
    })
}

fn lcouple_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "l".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::Hybrid, PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Inductor winding hybrid port".to_string(),
            },
            PortSpec {
                name: "mmf_out".to_string(),
                direction: PortDirection::InOut,
                default_type: PortType::DifferentialHybrid,
                allowed_types: vec![PortType::DifferentialHybrid],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Magnetic core MMF hybrid port".to_string(),
            },
        ]
    })
}

fn ilimit_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![
                    PortType::Voltage,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                    PortType::VoltageName,
                ],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Input".to_string(),
            },
            gd_port(
                "pos_pwr",
                "Positive power supply",
                vec![PortType::Conductance, PortType::DifferentialConductance],
                PortType::Conductance,
            )
            .nullable(),
            gd_port(
                "neg_pwr",
                "Negative power supply",
                vec![PortType::Conductance, PortType::DifferentialConductance],
                PortType::Conductance,
            )
            .nullable(),
            gd_port(
                "out",
                "Current-limited output",
                vec![PortType::Conductance, PortType::DifferentialConductance],
                PortType::Conductance,
            ),
        ]
    })
}

fn seegen_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "ctrl".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![
                    PortType::Voltage,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                ],
                is_vector: false,
                null_allowed: true,
                vector_min_len: None,
                vector_max_len: None,
                description: "Control input".to_string(),
            },
            PortSpec {
                name: "mon".to_string(),
                direction: PortDirection::Out,
                default_type: PortType::Voltage,
                allowed_types: vec![PortType::Voltage],
                is_vector: false,
                null_allowed: true,
                vector_min_len: None,
                vector_max_len: None,
                description: "Monitor output".to_string(),
            },
            PortSpec {
                name: "out".to_string(),
                direction: PortDirection::Out,
                default_type: PortType::Current,
                allowed_types: vec![PortType::Current, PortType::DifferentialCurrent],
                is_vector: true,
                null_allowed: false,
                vector_min_len: Some(1),
                vector_max_len: None,
                description: "Vector current pulse outputs".to_string(),
            },
        ]
    })
}

fn potentiometer_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("position", 0.5)
                .with_description("Wiper position, clamped to the official 0.0 to 1.0 guard range"),
            ParamSpec::boolean("log", false).with_description("Use logarithmic resistance split"),
            ParamSpec::real("r", 1.0e5).with_description("Total resistance"),
            ParamSpec::real("log_multiplier", 1.0)
                .with_description("Logarithmic resistance multiplier"),
        ]
    })
}

fn aswitch_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("cntl_off", 0.0).with_description("Control off threshold"),
            ParamSpec::real("cntl_on", 1.0).with_description("Control on threshold"),
            ParamSpec::boolean("log", true).with_description("Use logarithmic resistance law"),
            ParamSpec::real("r_off", 1.0e12).with_description("Off resistance"),
            ParamSpec::real("r_on", 1.0).with_description("On resistance"),
            ParamSpec::boolean("limit", false).with_description("Clamp resistance to r_on/r_off"),
        ]
    })
}

fn pswitch_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("cntl_off", 0.0).with_description("Control off threshold"),
            ParamSpec::real("cntl_on", 1.0).with_description("Control on threshold"),
            ParamSpec::boolean("log", true).with_description("Use logarithmic resistance law"),
            ParamSpec::real("r_off", 1.0e12).with_description("Off resistance"),
            ParamSpec::real("r_on", 1.0).with_description("On resistance"),
            ParamSpec::real("r_cntl_in", 1.0e12)
                .with_description("Input resistance for the control terminal"),
        ]
    })
}

fn sidiode_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("ron", 1.0)
                .with_description("On-state resistance, clamped to official lower limit"),
            ParamSpec::real("roff", 1.0)
                .with_description("Off-state resistance, clamped to official lower limit"),
            ParamSpec::real("vfwd", 0.0)
                .with_description("Forward voltage threshold, clamped to official lower limit"),
            ParamSpec::real("vrev", 1.0e30).with_description(
                "Reverse breakdown voltage magnitude, clamped to official lower limit",
            ),
            ParamSpec::real("ilimit", 1.0e30)
                .with_description("Forward current limit, clamped to official lower limit"),
            ParamSpec::real("revilimit", 1.0e30).with_description(
                "Reverse breakdown current limit magnitude, clamped to official lower limit",
            ),
            ParamSpec::real("epsilon", 0.0).with_description(
                "Forward quadratic transition width, clamped to official lower limit",
            ),
            ParamSpec::real("revepsilon", 0.0).with_description(
                "Reverse quadratic transition width, clamped to official lower limit",
            ),
            ParamSpec::real("rrev", 0.0).with_description("Reverse breakdown resistance"),
        ]
    })
}

fn zener_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("v_breakdown", 0.0)
                .required()
                .with_description("Breakdown voltage magnitude, clamped to official limits"),
            ParamSpec::real("i_breakdown", 2.0e-2)
                .with_description("Breakdown current, clamped to official lower limit"),
            ParamSpec::real("r_breakdown", 1.0)
                .with_description("Breakdown resistance, clamped to official lower limit"),
            ParamSpec::real("i_rev", 1.0e-6)
                .with_description("Reverse leakage current, clamped to official lower limit"),
            ParamSpec::real("i_sat", 1.0e-12)
                .with_description("Forward saturation current, clamped to official lower limit"),
            ParamSpec::real("n_forward", 1.0)
                .with_description("Forward emission coefficient, clamped to official limits"),
            ParamSpec::boolean("limit_switch", false).with_description("Enable voltage limiting"),
        ]
    })
}

fn memristor_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("rmin", 10.0).with_description("Minimum resistance"),
            ParamSpec::real("rmax", 10_000.0).with_description("Maximum resistance"),
            ParamSpec::real("rinit", 7_000.0).with_description("Initial resistance"),
            ParamSpec::real("alpha", 0.0).with_description("Below-threshold resistance rate"),
            ParamSpec::real("beta", 1.0).with_description("Above-threshold resistance rate"),
            ParamSpec::real("vt", 0.0).with_description("Voltage threshold"),
        ]
    })
}

fn core_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("h_array", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("Magnetic field lookup points"),
            ParamSpec::real_vector("b_array", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("Flux density lookup points"),
            ParamSpec::real("area", 0.0)
                .required()
                .with_description("Core cross-sectional area"),
            ParamSpec::real("length", 0.0)
                .required()
                .with_description("Core magnetic path length"),
            ParamSpec::real("input_domain", 0.01)
                .with_description("Smoothing range, clamped to the official [1e-12, 0.5] limits"),
            ParamSpec::boolean("fraction", true)
                .with_description("Treat input_domain as a fraction of adjacent spacing"),
            ParamSpec::integer("mode", 1)
                .with_description("1 = PWL, 2 = hysteresis; out-of-range values clamp"),
            ParamSpec::real("in_low", 0.0).with_description("Hysteresis lower input value"),
            ParamSpec::real("in_high", 1.0).with_description("Hysteresis upper input value"),
            ParamSpec::real("hyst", 0.1)
                .with_description("Hysteresis width, clamped to the official nonnegative limit"),
            ParamSpec::real("out_lower_limit", 0.0)
                .with_description("Hysteresis lower output limit"),
            ParamSpec::real("out_upper_limit", 1.0)
                .with_description("Hysteresis upper output limit"),
        ]
    })
}

fn meter_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("gain", 1.0).with_description("Meter conversion gain"),
            ParamSpec::real(XTRADEV_METER_MEASURED_VALUE_PARAM, 0.0)
                .with_description("Builder-injected static topology measurement"),
        ]
    })
}

fn lcouple_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![ParamSpec::real("num_turns", 1.0).with_description("Number of winding turns")]
    })
}

fn ilimit_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("in_offset", 0.0).with_description("Input offset"),
            ParamSpec::real("gain", 1.0).with_description("Input gain"),
            ParamSpec::real("r_out_source", 1.0)
                .with_description("Sourcing output resistance, clamped to official limits"),
            ParamSpec::real("r_out_sink", 1.0)
                .with_description("Sinking output resistance, clamped to official limits"),
            ParamSpec::real("i_limit_source", 10.0e-3)
                .with_description("Sourcing current limit, clamped to official lower limit"),
            ParamSpec::real("i_limit_sink", 10.0e-3)
                .with_description("Sinking current limit, clamped to official lower limit"),
            ParamSpec::real("v_pwr_range", 1.0e-6)
                .with_description("Power-rail smoothing range, clamped to official lower limit"),
            ParamSpec::real("i_source_range", 1.0e-9).with_description(
                "Sourcing current smoothing range, clamped to official lower limit",
            ),
            ParamSpec::real("i_sink_range", 1.0e-9).with_description(
                "Sinking current smoothing range, clamped to official lower limit",
            ),
            ParamSpec::real("r_out_domain", 1.0e-9).with_description(
                "Output resistance transition domain, clamped to official lower limit",
            ),
        ]
    })
}

fn seegen_parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("tfall", 500.0e-12).with_description("Pulse fall time"),
            ParamSpec::real("trise", 20.0e-12).with_description("Pulse rise time"),
            ParamSpec::real("tdelay", 0.0).with_description("Pulse delay"),
            ParamSpec::real("inull", 0.0).with_description("Maximum pulse current"),
            ParamSpec::real("tperiod", 0.0).with_description("Pulse repetition period"),
            ParamSpec::real("ctrlthres", 0.5).with_description("Control voltage threshold"),
            ParamSpec::real("let", 10.0).with_description("Linear energy transfer"),
            ParamSpec::real("cdepth", 1.0).with_description("Charge collection depth"),
            ParamSpec::real("angle", 0.0)
                .with_description("Particle entrance angle, clamped to official limits"),
            ParamSpec::boolean("perlim", true)
                .with_description("Stop after the last vector output"),
        ]
    })
}

fn finite_param(ctx: &CmContext, name: &str, default: Value) -> CmResult<Value> {
    let value = ctx.param_or(name, default);
    if !value.is_finite() {
        return Err(invalid_param(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value)
}

fn positive_param(ctx: &CmContext, name: &str, default: Value) -> CmResult<Value> {
    let value = finite_param(ctx, name, default)?;
    if value <= 0.0 {
        return Err(invalid_param(
            name,
            format!("value must be positive, got {value}"),
        ));
    }
    Ok(value)
}

fn guarded_position(ctx: &CmContext) -> CmResult<Value> {
    let mut position = finite_param(ctx, "position", 0.5)?;
    if position <= 0.0 {
        position = 1.0e-9;
    } else if position >= 1.0 {
        position = 0.999_999_999;
    }
    Ok(position)
}

fn potentiometer_split(ctx: &CmContext) -> CmResult<PotentiometerSplit> {
    let position = guarded_position(ctx)?;
    let resistance = finite_param(ctx, "r", 1.0e5)?;

    let log = ctx.param_or("log", 0.0) > 0.5;
    let r_lower = if log {
        let log_multiplier = finite_param(ctx, "log_multiplier", 1.0)?;
        let divisor = 10.0_f64.powf(position * log_multiplier);
        if !divisor.is_finite() || divisor <= 0.0 {
            return Err(invalid_param(
                "log_multiplier",
                format!(
                    "logarithmic resistance divisor must be positive and finite, got {divisor}"
                ),
            ));
        }
        resistance / divisor
    } else {
        position * resistance
    };
    let r_upper = resistance - r_lower;

    if !r_lower.is_finite() || !r_upper.is_finite() || r_lower == 0.0 || r_upper == 0.0 {
        return Err(invalid_param(
            "position",
            format!(
                "resistance split must be nonzero and finite, got r_lower={r_lower}, r_upper={r_upper}"
            ),
        ));
    }

    Ok(PotentiometerSplit {
        r_lower,
        r_upper,
        g_lower: 1.0 / r_lower,
        g_upper: 1.0 / r_upper,
    })
}

fn aswitch_eval(ctx: &CmContext) -> CmResult<AswitchEval> {
    let cntl_on = finite_param(ctx, "cntl_on", 1.0)?;
    let cntl_off = finite_param(ctx, "cntl_off", 0.0)?;
    let limit = ctx.param_or("limit", 0.0) > 0.5;
    let log_mode = ctx.param_or("log", 1.0) > 0.5;
    let r_off = if log_mode || limit {
        positive_param(ctx, "r_off", 1.0e12)?
    } else {
        finite_param(ctx, "r_off", 1.0e12)?
    };
    let mut r_on = finite_param(ctx, "r_on", 1.0)?;
    r_on = r_on.max(1.0e-3);

    let cntl_delta = cntl_on - cntl_off;
    if cntl_delta.abs() < 1.0e-12 {
        return Ok(AswitchEval {
            output_conductance: 0.0,
            control_partial: 0.0,
            output_current: 0.0,
        });
    }

    let control = ctx.input("cntl_in");
    let output_voltage = ctx.input("out");
    let (mut resistance, control_partial) = if log_mode {
        let intermediate = (r_off / r_on).ln() / cntl_delta;
        let mut resistance = r_on * (intermediate * (cntl_on - control)).exp();
        if limit {
            if resistance < r_on {
                resistance = r_on;
            }
            if resistance > r_off {
                resistance = r_off;
            }
        } else if resistance <= 1.0e-9 {
            resistance = 1.0e-9;
        }
        (resistance, intermediate * output_voltage / resistance)
    } else {
        let intermediate = (r_on - r_off) / cntl_delta;
        let mut resistance =
            control * intermediate + ((r_off * cntl_on - r_on * cntl_off) / cntl_delta);
        if limit {
            if resistance < r_on {
                resistance = r_on;
            }
            if resistance > r_off {
                resistance = r_off;
            }
        } else if resistance <= 1.0e-9 {
            resistance = 1.0e-9;
        }
        (
            resistance,
            -intermediate * output_voltage / (resistance * resistance),
        )
    };

    if !resistance.is_finite() || resistance <= 0.0 {
        resistance = 1.0e-9;
    }

    let output_conductance = 1.0 / resistance;
    Ok(AswitchEval {
        output_conductance,
        control_partial,
        output_current: output_voltage * output_conductance,
    })
}

fn pswitch_eval(ctx: &CmContext) -> CmResult<PswitchEval> {
    let mut r_on = finite_param(ctx, "r_on", 1.0)?;
    let mut r_off = finite_param(ctx, "r_off", 1.0e12)?;
    let r_cntl_in = finite_param(ctx, "r_cntl_in", 1.0e12)?;
    let log_mode = ctx.param_or("log", 1.0) > 0.5;
    if log_mode && r_off <= 0.0 {
        return Err(invalid_param(
            "r_off",
            format!("logarithmic pswitch off resistance must be positive, got {r_off}"),
        ));
    }
    if r_cntl_in == 0.0 {
        return Err(invalid_param(
            "r_cntl_in",
            "control input resistance must be nonzero",
        ));
    }

    r_on = r_on.max(1.0e-3);
    r_off = r_off.min(1.0e12);

    let mut cntl_on = finite_param(ctx, "cntl_on", 1.0)?;
    let mut cntl_off = finite_param(ctx, "cntl_off", 0.0)?;
    if (cntl_on - cntl_off).abs() < 1.0e-12 {
        cntl_on += 0.001;
        cntl_off -= 0.001;
    }

    let control = ctx.input("cntl_in");
    let output_voltage = ctx.input("out");

    let (mut resistance, control_partial) = if log_mode {
        let normalized_diff: Value = if cntl_on > cntl_off { 1.0 } else { -1.0 };
        let logmean = (r_on * r_off).sqrt().ln();
        let logratio = (r_on / r_off).ln();
        let c1 = 1.5 * logratio / normalized_diff;
        let c3 = 2.0 * logratio / normalized_diff.powi(3);
        let c2 = 3.0 * c3;

        let (inmean, out_of_limit, r) = if cntl_on > cntl_off {
            let inmean = (control - cntl_off) / (cntl_on - cntl_off) - 0.5;
            if control > cntl_on {
                (inmean, true, r_on)
            } else if control < cntl_off {
                (inmean, true, r_off)
            } else {
                (
                    inmean,
                    false,
                    (logmean + c1 * inmean - c3 * inmean.powi(3))
                        .exp()
                        .max(r_on),
                )
            }
        } else {
            let inmean = (cntl_on - control) / (cntl_on - cntl_off) - 0.5;
            if control < cntl_on {
                (inmean, true, r_on)
            } else if control > cntl_off {
                (inmean, true, r_off)
            } else {
                (
                    inmean,
                    false,
                    (logmean + c1 * inmean - c3 * inmean.powi(3))
                        .exp()
                        .max(r_on),
                )
            }
        };

        let partial = if out_of_limit {
            0.0
        } else {
            output_voltage / r * (c2 * inmean * inmean - c1)
        };
        (r, partial)
    } else {
        let cntl_diff = cntl_on - cntl_off;
        let intermediate = (r_on - r_off) / cntl_diff;
        let r = if cntl_diff >= 0.0 {
            if control < cntl_off {
                r_off
            } else if control > cntl_on {
                r_on
            } else {
                control * intermediate + ((r_off * cntl_on - r_on * cntl_off) / cntl_diff)
            }
        } else if control > cntl_off {
            r_off
        } else if control < cntl_on {
            r_on
        } else {
            control * intermediate + ((r_off * cntl_on - r_on * cntl_off) / cntl_diff)
        };

        let limited = r <= 1.0e-9;
        let r = r.max(1.0e-9);
        let partial = if limited
            || (cntl_diff >= 0.0 && (control < cntl_off || control > cntl_on))
            || (cntl_diff < 0.0 && (control > cntl_off || control < cntl_on))
        {
            0.0
        } else {
            -intermediate * output_voltage / (r * r)
        };
        (r, partial)
    };

    if !resistance.is_finite() || resistance <= 0.0 {
        resistance = 1.0e-9;
    }

    let output_conductance = 1.0 / resistance;
    let control_conductance = 1.0 / r_cntl_in;
    Ok(PswitchEval {
        resistance,
        output_conductance,
        control_partial,
        control_conductance,
        output_current: output_voltage * output_conductance,
        control_current: control * control_conductance,
    })
}

fn sidiode_eval(ctx: &CmContext) -> CmResult<SidiodeEval> {
    let ron = finite_param(ctx, "ron", 1.0)?.max(1.0e-6);
    let roff = finite_param(ctx, "roff", 1.0)?.max(1.0e-12);
    let vfwd = finite_param(ctx, "vfwd", 0.0)?.max(0.0);
    let vrev = -finite_param(ctx, "vrev", 1.0e30)?.max(0.0);
    let ilimit = finite_param(ctx, "ilimit", 1.0e30)?.max(1.0e-15);
    let revilimit = -finite_param(ctx, "revilimit", 1.0e30)?.max(1.0e-15);
    let epsilon = finite_param(ctx, "epsilon", 0.0)?.max(0.0);
    let revepsilon = finite_param(ctx, "revepsilon", 0.0)?.max(0.0);
    let rrev = finite_param(ctx, "rrev", 0.0)?;

    let goff = 1.0 / roff;
    let gon = 1.0 / ron;
    let grev = if rrev == 0.0 { gon } else { 1.0 / rrev };

    let va = vrev - revepsilon;
    let vb = vrev;
    let vc = vfwd;
    let vd = vfwd + epsilon;
    let a2 = grev / revilimit;
    let a1 = if revepsilon > 0.0 {
        Some((goff - grev) / revepsilon)
    } else {
        None
    };
    let b2 = gon / ilimit;
    let b1 = if epsilon > 0.0 {
        Some((gon - goff) / epsilon)
    } else {
        None
    };
    let has_forward_limit = ilimit < 1.0e29;
    let has_reverse_limit = revilimit > -1.0e29;
    let vin = ctx.input("ds");

    let (current, derivative) = if vin < va {
        if has_reverse_limit {
            let a1 = a1.unwrap_or(0.0);
            let tmp = (a2 * (vin - va)).tanh();
            let ia = goff * va + 0.5 * (va - vb) * (va - vb) * a1;
            ((revilimit - ia) * tmp + ia, grev * (1.0 - tmp * tmp))
        } else {
            (vrev * goff + (vin - vrev) * grev, grev)
        }
    } else if let Some(a1) = a1
        && vin >= va
        && vin < vb
    {
        (
            0.5 * (vin - vb) * (vin - vb) * a1 + vin * goff,
            (vin - vb) * a1 + goff,
        )
    } else if vin >= vb && vin < vc {
        (vin * goff, goff)
    } else if let Some(b1) = b1
        && vin >= vc
        && vin < vd
    {
        (
            0.5 * (vin - vc) * (vin - vc) * b1 + vin * goff,
            (vin - vc) * b1 + goff,
        )
    } else if has_forward_limit {
        let b1 = b1.unwrap_or(0.0);
        let tmp = (b2 * (vin - vd)).tanh();
        let id = goff * vd + 0.5 * (vd - vc) * (vd - vc) * b1;
        ((ilimit - id) * tmp + id, gon * (1.0 - tmp * tmp))
    } else {
        (vfwd * goff + (vin - vfwd) * gon, gon)
    };

    Ok(SidiodeEval {
        current,
        derivative,
    })
}

fn zener_limited_voltage(ctx: &mut CmContext) -> Value {
    let mut voltage = ctx.input("z");
    if ctx.param_or("limit_switch", 0.0) <= 0.5 || ctx.is_ac() {
        return voltage;
    }

    let previous = ctx.state(0);
    let increment = if previous.abs() >= 1.0 {
        0.1 * previous
    } else if voltage < 0.0 {
        -0.1
    } else {
        0.1
    };
    let limited = previous + increment;
    if voltage.abs() > limited.abs() {
        voltage = limited;
    }
    xtradev_set_state(ctx, 0, voltage);
    voltage
}

fn zener_eval_for_voltage(ctx: &CmContext, voltage: Value) -> CmResult<ZenerEval> {
    let v_breakdown = finite_param(ctx, "v_breakdown", 0.0)?.clamp(1.0e-6, 1.0e6);
    let i_breakdown = finite_param(ctx, "i_breakdown", 2.0e-2)?.max(1.0e-9);
    let r_breakdown = finite_param(ctx, "r_breakdown", 1.0)?.max(1.0e-12);
    let i_rev = finite_param(ctx, "i_rev", 1.0e-6)?.max(1.0e-9);
    let i_sat = finite_param(ctx, "i_sat", 1.0e-12)?.max(1.0e-15);
    let n_forward = finite_param(ctx, "n_forward", 1.0)?.clamp(0.1, 10.0);

    const VT: Value = 0.026;
    let v_1_2 = n_forward * VT * (n_forward * VT / 10.0).ln();
    let k = 1.0 / i_breakdown / r_breakdown;
    let v_2_3 = -v_breakdown + (10.0 / i_sat / r_breakdown).ln() / k;

    let (mut current, mut derivative) = if v_2_3 < v_1_2 {
        let i0 = 1.0e-6;
        let v0 = -v_breakdown + (i_breakdown / i0).ln() / k;
        let ord_1_2 = i_sat * ((v_1_2 / n_forward / VT).exp() - 1.0);
        let a = i_sat / 10.0;
        let b = ord_1_2 - a * v_1_2;
        let c = a * v_2_3 + b + i0 * (-k * (v_2_3 - v0)).exp();

        if voltage >= v_1_2 {
            let temp = (voltage / n_forward / VT).exp();
            (i_sat * (temp - 1.0), i_sat / n_forward / VT * temp)
        } else if voltage >= v_2_3 {
            (a * voltage + b, i_sat / 10.0)
        } else {
            let temp = (-k * (voltage - v0)).exp();
            (-i0 * temp + c, k * i0 * temp)
        }
    } else {
        let i0 = i_breakdown / ((k * v_breakdown).exp() - 1.0);
        let slope1 = i_sat / n_forward / VT;
        let slope2 = i0 * k;

        if voltage >= 0.0 {
            let temp = (voltage / n_forward / VT).exp();
            let mut current = i_sat * (temp - 1.0);
            let mut derivative = i_sat / n_forward / VT * temp;
            let diff = slope2 - slope1;
            if diff > 0.0 {
                current += diff * voltage;
                derivative += diff;
            }
            (current, derivative)
        } else {
            let temp = (-k * voltage).exp();
            let mut current = -i0 * (temp - 1.0);
            let mut derivative = k * i0 * temp;
            let diff = slope1 - slope2;
            if diff > 0.0 {
                current += diff * voltage;
                derivative += diff;
            }
            (current, derivative)
        }
    };

    let leakage = i_rev / v_breakdown;
    current += leakage * voltage;
    derivative += leakage;

    Ok(ZenerEval {
        current,
        derivative,
    })
}

fn memristor_params(ctx: &CmContext) -> CmResult<MemristorParams> {
    Ok(MemristorParams {
        rmin: finite_param(ctx, "rmin", 10.0)?,
        rmax: finite_param(ctx, "rmax", 10_000.0)?,
        rinit: finite_param(ctx, "rinit", 7_000.0)?,
        alpha: finite_param(ctx, "alpha", 0.0)?,
        beta: finite_param(ctx, "beta", 1.0)?,
        vt: finite_param(ctx, "vt", 0.0)?,
    })
}

fn memristor_window(voltage: Value, params: MemristorParams) -> Value {
    params.beta * voltage
        + 0.5
            * (params.alpha - params.beta)
            * ((voltage + params.vt).abs() - (voltage - params.vt).abs())
}

fn memristor_abs_derivative(value: Value) -> Value {
    if value > 0.0 {
        1.0
    } else if value < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn memristor_window_derivative(voltage: Value, params: MemristorParams) -> Value {
    params.beta
        + 0.5
            * (params.alpha - params.beta)
            * (memristor_abs_derivative(voltage + params.vt)
                - memristor_abs_derivative(voltage - params.vt))
}

fn memristor_eval_for_context(ctx: &CmContext) -> CmResult<(MemristorEval, Value)> {
    let params = memristor_params(ctx)?;
    let voltage = ctx.input("memris");
    let accepted_resistance = {
        let value = ctx.state_prev(0);
        if value.is_finite() {
            value
        } else {
            params.rinit
        }
    };

    let (resistance, resistance_derivative) = if ctx.is_transient() {
        let rate = memristor_window(voltage, params);
        let active = (rate > 0.0 && accepted_resistance < params.rmax)
            || (rate < 0.0 && accepted_resistance > params.rmin);
        if active {
            let integration_partial = transient_integrator_partial(ctx);
            let resistance = accepted_resistance + integration_partial * rate;
            let resistance_derivative =
                integration_partial * memristor_window_derivative(voltage, params);
            (resistance, resistance_derivative)
        } else {
            (accepted_resistance, 0.0)
        }
    } else {
        (accepted_resistance, 0.0)
    };

    let conductance = 1.0 / resistance;
    let current = voltage * conductance;
    let derivative = conductance - voltage * resistance_derivative * conductance * conductance;
    Ok((
        MemristorEval {
            current,
            derivative,
        },
        resistance,
    ))
}

fn memristor_eval(ctx: &mut CmContext) -> CmResult<MemristorEval> {
    let (eval, resistance) = memristor_eval_for_context(ctx)?;
    xtradev_set_state(ctx, 0, resistance);
    Ok(eval)
}

const CORE_MODE_PWL: i64 = 1;
const CORE_MODE_HYSTERESIS: i64 = 2;
const CORE_HYSTERESIS_UNINITIALIZED: i64 = -1;
const CORE_HYSTERESIS_FALLING: i64 = 0;
const CORE_HYSTERESIS_RISING: i64 = 1;

fn core_mode(ctx: &CmContext) -> CmResult<i64> {
    let raw = finite_param(ctx, "mode", CORE_MODE_PWL as Value)?;
    let mode = raw.round() as i64;
    if (raw - mode as Value).abs() > f64::EPSILON {
        return Err(invalid_param(
            "mode",
            format!("mode must be an integer, got {raw}"),
        ));
    }
    Ok(mode.clamp(CORE_MODE_PWL, CORE_MODE_HYSTERESIS))
}

fn core_input_domain(ctx: &CmContext) -> CmResult<Value> {
    let input_domain = finite_param(ctx, "input_domain", 0.01)?;
    Ok(input_domain.clamp(1.0e-12, 0.5))
}

fn core_area(ctx: &CmContext) -> CmResult<Value> {
    finite_param(ctx, "area", 0.0)
}

fn core_length(ctx: &CmContext) -> CmResult<Value> {
    let length = finite_param(ctx, "length", 0.0)?;
    if length == 0.0 {
        return Err(invalid_param("length", "length must be nonzero"));
    }
    Ok(length)
}

fn core_table_signature(ctx: &CmContext) -> CoreTableSignature {
    CoreTableSignature {
        h_revision: ctx.real_vector_param_revision("h_array"),
        b_revision: ctx.real_vector_param_revision("b_array"),
    }
}

fn core_table_signature_matches(ctx: &CmContext, signature: &CoreTableSignature) -> bool {
    core_table_signature(ctx) == *signature
}

fn core_table_from_points(points: Vec<CorePoint>) -> CoreTable {
    let midpoints = points
        .windows(2)
        .map(|window| (window[0].h + window[1].h) * 0.5)
        .collect();
    let strictly_increasing_h = points.windows(2).all(|window| window[0].h < window[1].h);
    CoreTable {
        points,
        midpoints,
        strictly_increasing_h,
        last_midpoint_index: AtomicUsize::new(CORE_TABLE_UNSET_MIDPOINT_INDEX),
    }
}

fn core_table_uncached(ctx: &CmContext) -> CmResult<CoreTable> {
    let h_values = ctx
        .real_vector_param("h_array")
        .ok_or_else(|| CmError::MissingParameter("h_array".to_string()))?;
    let b_values = ctx
        .real_vector_param("b_array")
        .ok_or_else(|| CmError::MissingParameter("b_array".to_string()))?;

    if h_values.len() != b_values.len() {
        return Err(invalid_param(
            "h_array/b_array",
            format!(
                "h_array length {} must match b_array length {}",
                h_values.len(),
                b_values.len()
            ),
        ));
    }
    if h_values.len() < 2 {
        return Err(invalid_param(
            "h_array/b_array",
            format!(
                "h_array and b_array require at least 2 points, got {}",
                h_values.len()
            ),
        ));
    }

    let mut points: Vec<CorePoint> = Vec::with_capacity(h_values.len());
    for (index, (&h, &b)) in h_values.iter().zip(b_values).enumerate() {
        if !h.is_finite() {
            return Err(invalid_param(
                "h_array",
                format!("point {index} must be finite, got {h}"),
            ));
        }
        if !b.is_finite() {
            return Err(invalid_param(
                "b_array",
                format!("point {index} must be finite, got {b}"),
            ));
        }
        points.push(CorePoint { h, b });
    }

    Ok(core_table_from_points(points))
}

fn cache_core_table(ctx: &mut CmContext) -> CmResult<Arc<CoreTable>> {
    if let Some(resource) = ctx.resource::<CoreTableResource>(CORE_TABLE_RESOURCE)
        && core_table_signature_matches(ctx, &resource.signature)
    {
        return resource.data.clone();
    }

    let signature = core_table_signature(ctx);
    let data = core_table_uncached(ctx).map(Arc::new);
    ctx.set_resource(
        CORE_TABLE_RESOURCE,
        Arc::new(CoreTableResource {
            signature,
            data: data.clone(),
        }),
    );
    data
}

fn core_validate_common(ctx: &mut CmContext) -> CmResult<()> {
    core_mode(ctx)?;
    cache_core_table(ctx)?;
    core_area(ctx)?;
    core_length(ctx)?;
    core_input_domain(ctx)?;
    Ok(())
}

fn core_pwl_smoothing_allowed(table: &CoreTable, input_domain: Value, fraction: bool) -> bool {
    if fraction {
        return true;
    }

    for window in table.points.windows(2) {
        let spacing = window[1].h - window[0].h;
        if spacing < 2.0 * input_domain {
            return false;
        }
    }
    true
}

fn core_segment_slope(left: CorePoint, right: CorePoint) -> Value {
    (right.b - left.b) / (right.h - left.h)
}

fn core_linear_value(point: CorePoint, slope: Value, h_input: Value) -> (Value, Value) {
    (point.b + (h_input - point.h) * slope, slope)
}

fn core_midpoint_index_contains(table: &CoreTable, index: usize, h_input: Value) -> bool {
    let last_index = table.points.len() - 1;
    debug_assert!(index > 0);
    debug_assert!(index < last_index);
    table.midpoints[index - 1] <= h_input && h_input < table.midpoints[index]
}

fn core_midpoint_index_binary(table: &CoreTable, h_input: Value) -> usize {
    let last_index = table.points.len() - 1;
    table.midpoints[1..last_index].partition_point(|midpoint| h_input >= *midpoint) + 1
}

fn core_midpoint_index_with_cursor(table: &CoreTable, h_input: Value) -> usize {
    let last_index = table.points.len() - 1;
    let mut index = table.last_midpoint_index.load(Ordering::Relaxed);

    if index == CORE_TABLE_UNSET_MIDPOINT_INDEX || index == 0 || index >= last_index {
        index = core_midpoint_index_binary(table, h_input);
        table.last_midpoint_index.store(index, Ordering::Relaxed);
        return index;
    }

    if core_midpoint_index_contains(table, index, h_input) {
        return index;
    }

    let mut steps = 0;
    if h_input >= table.midpoints[index] {
        while index + 1 < last_index
            && h_input >= table.midpoints[index]
            && steps < CORE_TABLE_CURSOR_LINEAR_STEPS
        {
            index += 1;
            steps += 1;
        }
    } else {
        while index > 1
            && h_input < table.midpoints[index - 1]
            && steps < CORE_TABLE_CURSOR_LINEAR_STEPS
        {
            index -= 1;
            steps += 1;
        }
    }

    if !core_midpoint_index_contains(table, index, h_input) {
        index = core_midpoint_index_binary(table, h_input);
    }
    table.last_midpoint_index.store(index, Ordering::Relaxed);
    index
}

fn core_pwl_flux_density(
    table: &CoreTable,
    h_input: Value,
    input_domain: Value,
    fraction: bool,
) -> (Value, Value) {
    let points = table.points.as_slice();
    let last_index = points.len() - 1;
    let lower_midpoint = table.midpoints[0];
    if h_input <= lower_midpoint {
        let slope = core_segment_slope(points[0], points[1]);
        return core_linear_value(points[0], slope, h_input);
    }

    let upper_midpoint = table.midpoints[last_index - 1];
    if h_input >= upper_midpoint {
        let slope = core_segment_slope(points[last_index - 1], points[last_index]);
        return core_linear_value(points[last_index], slope, h_input);
    }

    let index = if table.strictly_increasing_h {
        core_midpoint_index_with_cursor(table, h_input)
    } else {
        let mut index = 1;
        while index < last_index && h_input >= table.midpoints[index] {
            index += 1;
        }
        index
    };

    if index < last_index {
        let lower_seg = points[index].h - points[index - 1].h;
        let upper_seg = points[index + 1].h - points[index].h;
        let domain = if fraction {
            input_domain * lower_seg.min(upper_seg)
        } else {
            input_domain
        };
        let threshold_lower = points[index].h - domain;
        let threshold_upper = points[index].h + domain;

        if h_input < threshold_lower {
            let slope = core_segment_slope(points[index - 1], points[index]);
            return core_linear_value(points[index], slope, h_input);
        }
        if h_input < threshold_upper {
            return smooth_corner(
                h_input,
                points[index].h,
                points[index].b,
                domain,
                core_segment_slope(points[index - 1], points[index]),
                core_segment_slope(points[index], points[index + 1]),
            );
        }

        let slope = core_segment_slope(points[index], points[index + 1]);
        return core_linear_value(points[index], slope, h_input);
    }

    let slope = core_segment_slope(points[last_index - 1], points[last_index]);
    core_linear_value(points[last_index], slope, h_input)
}

fn core_pwl_eval_from_table(ctx: &CmContext, table: &CoreTable) -> CmResult<CoreEval> {
    let input_domain = core_input_domain(ctx)?;
    let fraction = ctx.param_or("fraction", 1.0) > 0.5;
    if !core_pwl_smoothing_allowed(table, input_domain, fraction) {
        return Ok(CoreEval {
            current: 0.0,
            derivative: 0.0,
        });
    }

    let area = core_area(ctx)?;
    let length = core_length(ctx)?;
    let h_input = ctx.input("mc") / length;
    let (flux_density, dflux_density_dh) =
        core_pwl_flux_density(table, h_input, input_domain, fraction);

    Ok(CoreEval {
        current: flux_density * area,
        derivative: dflux_density_dh * area / length,
    })
}

fn core_pwl_eval(ctx: &mut CmContext) -> CmResult<CoreEval> {
    let table = cache_core_table(ctx)?;
    core_pwl_eval_from_table(ctx, &table)
}

fn core_pwl_eval_for_context(ctx: &CmContext) -> CmResult<CoreEval> {
    if let Some(resource) = ctx.resource::<CoreTableResource>(CORE_TABLE_RESOURCE)
        && core_table_signature_matches(ctx, &resource.signature)
    {
        let table = resource.data.clone()?;
        return core_pwl_eval_from_table(ctx, &table);
    }

    let table = core_table_uncached(ctx)?;
    core_pwl_eval_from_table(ctx, &table)
}

fn core_hysteresis_params(ctx: &CmContext) -> CmResult<CoreHysteresisParams> {
    let in_low = finite_param(ctx, "in_low", 0.0)?;
    let in_high = finite_param(ctx, "in_high", 1.0)?;
    if in_high == in_low {
        return Err(invalid_param(
            "in_high",
            format!("in_high must differ from in_low, got in_low={in_low}, in_high={in_high}"),
        ));
    }

    let input_domain = core_input_domain(ctx)?;
    let fraction = ctx.param_or("fraction", 1.0) > 0.5;
    Ok(CoreHysteresisParams {
        in_low,
        in_high,
        hyst: finite_param(ctx, "hyst", 0.1)?.max(0.0),
        out_lower_limit: finite_param(ctx, "out_lower_limit", 0.0)?,
        out_upper_limit: finite_param(ctx, "out_upper_limit", 1.0)?,
        input_domain: if fraction {
            input_domain * (in_high - in_low)
        } else {
            input_domain
        },
    })
}

fn core_hysteresis_eval_for_context(ctx: &CmContext) -> CmResult<(CoreEval, i64)> {
    let params = core_hysteresis_params(ctx)?;
    let slope =
        (params.out_upper_limit - params.out_lower_limit) / (params.in_high - params.in_low);
    let x_rise_linear = params.in_low + params.hyst;
    let x_rise_zero = params.in_high + params.hyst;
    let x_fall_linear = params.in_high - params.hyst;
    let x_fall_zero = params.in_low - params.hyst;
    let input = ctx.input("mc");
    let old_state = ctx.int_state(0);
    let old_rising = if old_state == CORE_HYSTERESIS_UNINITIALIZED {
        input < x_rise_zero + params.input_domain
    } else {
        old_state == CORE_HYSTERESIS_RISING
    };

    let mut rising = old_rising;
    let (current, derivative) = if old_rising {
        if input <= x_rise_linear - params.input_domain {
            (params.out_lower_limit, 0.0)
        } else if input <= x_rise_linear + params.input_domain {
            smooth_corner(
                input,
                x_rise_linear,
                params.out_lower_limit,
                params.input_domain,
                0.0,
                slope,
            )
        } else if input <= x_rise_zero - params.input_domain {
            (
                (input - x_rise_linear) * slope + params.out_lower_limit,
                slope,
            )
        } else if input <= x_rise_zero + params.input_domain {
            smooth_corner(
                input,
                x_rise_zero,
                params.out_upper_limit,
                params.input_domain,
                slope,
                0.0,
            )
        } else {
            rising = false;
            (params.out_upper_limit, 0.0)
        }
    } else if input >= x_fall_linear + params.input_domain {
        (params.out_upper_limit, 0.0)
    } else if input >= x_fall_linear - params.input_domain {
        smooth_corner(
            input,
            x_fall_linear,
            params.out_upper_limit,
            params.input_domain,
            slope,
            0.0,
        )
    } else if input >= x_fall_zero + params.input_domain {
        (
            (input - x_fall_zero) * slope + params.out_lower_limit,
            slope,
        )
    } else if input >= x_fall_zero - params.input_domain {
        smooth_corner(
            input,
            x_fall_zero,
            params.out_lower_limit,
            params.input_domain,
            0.0,
            slope,
        )
    } else {
        rising = true;
        (params.out_lower_limit, 0.0)
    };

    let next_state = if rising {
        CORE_HYSTERESIS_RISING
    } else {
        CORE_HYSTERESIS_FALLING
    };

    Ok((
        CoreEval {
            current,
            derivative,
        },
        next_state,
    ))
}

fn core_hysteresis_eval(ctx: &mut CmContext) -> CmResult<CoreEval> {
    let (eval, next_state) = core_hysteresis_eval_for_context(ctx)?;
    xtradev_set_int_state(ctx, 0, next_state);
    Ok(eval)
}

fn core_eval(ctx: &mut CmContext) -> CmResult<CoreEval> {
    match core_mode(ctx)? {
        CORE_MODE_PWL => core_pwl_eval(ctx),
        CORE_MODE_HYSTERESIS => core_hysteresis_eval(ctx),
        _ => unreachable!("core_mode validates the mode range"),
    }
}

fn core_eval_for_context(ctx: &CmContext) -> CmResult<CoreEval> {
    match core_mode(ctx)? {
        CORE_MODE_PWL => core_pwl_eval_for_context(ctx),
        CORE_MODE_HYSTERESIS => core_hysteresis_eval_for_context(ctx).map(|(eval, _)| eval),
        _ => unreachable!("core_mode validates the mode range"),
    }
}

fn ilimit_params(ctx: &CmContext) -> CmResult<()> {
    finite_param(ctx, "in_offset", 0.0)?;
    finite_param(ctx, "gain", 1.0)?;
    finite_param(ctx, "r_out_source", 1.0)?;
    finite_param(ctx, "r_out_sink", 1.0)?;
    finite_param(ctx, "i_limit_source", 10.0e-3)?;
    finite_param(ctx, "i_limit_sink", 10.0e-3)?;
    finite_param(ctx, "v_pwr_range", 1.0e-6)?;
    finite_param(ctx, "i_source_range", 1.0e-9)?;
    finite_param(ctx, "i_sink_range", 1.0e-9)?;
    finite_param(ctx, "r_out_domain", 1.0e-9)?;
    Ok(())
}

fn ilimit_port_connected(ctx: &CmContext, name: &str) -> bool {
    ctx.port_width(name) > 0
}

fn ilimit_eval_signature(ctx: &CmContext) -> CmResult<IlimitEvalSignature> {
    let pos_pwr_connected = ilimit_port_connected(ctx, "pos_pwr");
    let neg_pwr_connected = ilimit_port_connected(ctx, "neg_pwr");
    Ok(IlimitEvalSignature {
        in_offset: finite_param(ctx, "in_offset", 0.0)?,
        gain: finite_param(ctx, "gain", 1.0)?,
        r_out_source: finite_param(ctx, "r_out_source", 1.0)?,
        r_out_sink: finite_param(ctx, "r_out_sink", 1.0)?,
        i_limit_source: finite_param(ctx, "i_limit_source", 10.0e-3)?,
        i_limit_sink: finite_param(ctx, "i_limit_sink", 10.0e-3)?,
        v_pwr_range: finite_param(ctx, "v_pwr_range", 1.0e-6)?,
        i_source_range: finite_param(ctx, "i_source_range", 1.0e-9)?,
        i_sink_range: finite_param(ctx, "i_sink_range", 1.0e-9)?,
        r_out_domain: finite_param(ctx, "r_out_domain", 1.0e-9)?,
        input: ctx.input("in"),
        output: ctx.input("out"),
        pos_pwr: if pos_pwr_connected {
            ctx.input("pos_pwr")
        } else {
            1.0e6
        },
        neg_pwr: if neg_pwr_connected {
            ctx.input("neg_pwr")
        } else {
            -1.0e6
        },
        pos_pwr_connected,
        neg_pwr_connected,
        init: ctx.is_init(),
    })
}

fn ilimit_eval_for_context(ctx: &CmContext) -> CmResult<IlimitEval> {
    let signature = ilimit_eval_signature(ctx)?;
    if let Some(resource) = ctx.resource::<IlimitEvalResource>(ILIMIT_EVAL_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.eval);
    }

    Ok(ilimit_eval_from_signature(&signature))
}

fn cache_ilimit_eval(ctx: &mut CmContext) -> CmResult<IlimitEval> {
    let signature = ilimit_eval_signature(ctx)?;
    if let Some(resource) = ctx.resource::<IlimitEvalResource>(ILIMIT_EVAL_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.eval);
    }

    let eval = ilimit_eval_from_signature(&signature);
    ctx.set_resource(
        ILIMIT_EVAL_RESOURCE,
        Arc::new(IlimitEvalResource { signature, eval }),
    );
    Ok(eval)
}

fn ilimit_eval(ctx: &CmContext) -> CmResult<IlimitEval> {
    let signature = ilimit_eval_signature(ctx)?;
    Ok(ilimit_eval_from_signature(&signature))
}

fn ilimit_eval_from_signature(signature: &IlimitEvalSignature) -> IlimitEval {
    let in_offset = signature.in_offset;
    let gain = signature.gain;
    let r_out_source = signature.r_out_source.clamp(1.0e-9, 1.0e9);
    let r_out_sink = signature.r_out_sink.clamp(1.0e-9, 1.0e9);
    let i_limit_source = signature.i_limit_source.max(1.0e-12);
    let i_limit_sink = signature.i_limit_sink.max(1.0e-12);
    let v_pwr_range = signature.v_pwr_range.max(1.0e-15);
    let i_source_range = signature.i_source_range.max(1.0e-15);
    let i_sink_range = signature.i_sink_range.max(1.0e-15);
    let r_out_domain = signature.r_out_domain.max(1.0e-15);
    let vout = signature.output;
    let pos_pwr_in = signature.pos_pwr;
    let neg_pwr_in = signature.neg_pwr;
    let pos_pwr_connected = signature.pos_pwr_connected;
    let neg_pwr_connected = signature.neg_pwr_connected;

    let (veq, pveq_pvin, pveq_pvneg, pveq_pvpos) = if signature.init {
        ((pos_pwr_in - neg_pwr_in) * 0.5, 0.0, 0.0, 0.0)
    } else {
        climit_transfer(
            signature.input,
            in_offset,
            pos_pwr_in,
            neg_pwr_in,
            0.0,
            0.0,
            v_pwr_range,
            gain,
            false,
        )
    };

    let (r_out, pr_out_px) = if (r_out_source - r_out_sink).abs() <= 0.0 {
        (r_out_source, 0.0)
    } else {
        smooth_discontinuity(
            veq - vout,
            -r_out_domain,
            r_out_sink,
            r_out_domain,
            r_out_source,
        )
    };

    let mut i_out = (veq - vout) / r_out;
    let mut pi_out_pvin = pveq_pvin / r_out - veq * pr_out_px * pveq_pvin / (r_out * r_out);
    let mut pi_out_pvout = -1.0 / r_out - vout * pr_out_px / (r_out * r_out);
    let mut pi_out_ppos_pwr = pveq_pvpos / r_out - veq * pr_out_px * pveq_pvpos / (r_out * r_out);
    let mut pi_out_pneg_pwr = pveq_pvneg / r_out - veq * pr_out_px * pveq_pvneg / (r_out * r_out);

    let mut i_pos_pwr = 0.0;
    let mut pi_pos_pvin = 0.0;
    let mut pi_pos_pvneg = 0.0;
    let mut pi_pos_pvpos = 0.0;
    let mut pi_pos_pvout = 0.0;

    let mut i_neg_pwr = 0.0;
    let mut pi_neg_pvin = 0.0;
    let mut pi_neg_pvneg = 0.0;
    let mut pi_neg_pvpos = 0.0;
    let mut pi_neg_pvout = 0.0;

    let i_threshold_lower = -i_limit_sink + i_sink_range;
    let i_threshold_upper = i_limit_source - i_source_range;

    if i_out < 0.0 {
        if i_out < i_threshold_lower {
            if i_out < -i_limit_sink - i_sink_range {
                i_out = -i_limit_sink;
                i_neg_pwr = -i_out;
                pi_out_pvin = 0.0;
                pi_out_pvout = 0.0;
                pi_out_ppos_pwr = 0.0;
                pi_out_pneg_pwr = 0.0;
            } else {
                let (limited, pi_out_plimit) =
                    smooth_corner(i_out, -i_limit_sink, -i_limit_sink, i_sink_range, 0.0, 1.0);
                i_out = limited;
                pi_out_pvin *= pi_out_plimit;
                pi_out_pvout *= pi_out_plimit;
                pi_out_ppos_pwr *= pi_out_plimit;
                pi_out_pneg_pwr *= pi_out_plimit;

                i_neg_pwr = -i_out;
                pi_neg_pvin = -pi_out_pvin;
                pi_neg_pvneg = -pi_out_pneg_pwr;
                pi_neg_pvpos = -pi_out_ppos_pwr;
                pi_neg_pvout = -pi_out_pvout;
            }
        } else if i_out > -2.0 * i_sink_range {
            let (mut limited, pi_neg_plimit) =
                smooth_corner(i_out, -i_sink_range, 0.0, i_sink_range, 1.0, 0.0);
            limited = -limited;
            i_neg_pwr = limited;
            pi_neg_pvin = -pi_out_pvin * pi_neg_plimit;
            pi_neg_pvneg = -pi_out_pneg_pwr * pi_neg_plimit;
            pi_neg_pvpos = -pi_out_ppos_pwr * pi_neg_plimit;
            pi_neg_pvout = -pi_out_pvout * pi_neg_plimit;
        } else {
            i_neg_pwr = -i_out;
            pi_neg_pvin = -pi_out_pvin;
            pi_neg_pvneg = -pi_out_pneg_pwr;
            pi_neg_pvpos = -pi_out_ppos_pwr;
            pi_neg_pvout = -pi_out_pvout;
        }
    } else if i_out > i_threshold_upper {
        if i_out > i_limit_source + i_source_range {
            i_out = i_limit_source;
            i_pos_pwr = -i_out;
            pi_out_pvin = 0.0;
            pi_out_pvout = 0.0;
            pi_out_ppos_pwr = 0.0;
            pi_out_pneg_pwr = 0.0;
        } else {
            let (limited, pi_out_plimit) = smooth_corner(
                i_out,
                i_limit_source,
                i_limit_source,
                i_sink_range,
                1.0,
                0.0,
            );
            i_out = limited;
            pi_out_pvin *= pi_out_plimit;
            pi_out_pvout *= pi_out_plimit;
            pi_out_ppos_pwr *= pi_out_plimit;
            pi_out_pneg_pwr *= pi_out_plimit;

            i_pos_pwr = -i_out;
            pi_pos_pvin = -pi_out_pvin;
            pi_pos_pvneg = -pi_out_pneg_pwr;
            pi_pos_pvpos = -pi_out_ppos_pwr;
            pi_pos_pvout = -pi_out_pvout;
        }
    } else if i_out < 2.0 * i_source_range {
        let (mut limited, pi_pos_plimit) =
            smooth_corner(i_out, i_source_range, 0.0, i_source_range, 0.0, 1.0);
        limited = -limited;
        i_pos_pwr = limited;
        pi_pos_pvin = -pi_out_pvin * pi_pos_plimit;
        pi_pos_pvneg = -pi_out_pneg_pwr * pi_pos_plimit;
        pi_pos_pvpos = -pi_out_ppos_pwr * pi_pos_plimit;
        pi_pos_pvout = -pi_out_pvout * pi_pos_plimit;
    } else {
        i_pos_pwr = -i_out;
        pi_pos_pvin = -pi_out_pvin;
        pi_pos_pvneg = -pi_out_pneg_pwr;
        pi_pos_pvpos = -pi_out_ppos_pwr;
        pi_pos_pvout = -pi_out_pvout;
    }

    IlimitEval {
        out_current: -i_out,
        out_in_partial: -pi_out_pvin,
        out_out_partial: -pi_out_pvout,
        out_pos_partial: if pos_pwr_connected {
            -pi_out_ppos_pwr
        } else {
            0.0
        },
        out_neg_partial: if neg_pwr_connected {
            -pi_out_pneg_pwr
        } else {
            0.0
        },
        pos_current: -i_pos_pwr,
        pos_in_partial: -pi_pos_pvin,
        pos_out_partial: -pi_pos_pvout,
        pos_pos_partial: -pi_pos_pvpos,
        pos_neg_partial: if neg_pwr_connected {
            -pi_pos_pvneg
        } else {
            0.0
        },
        neg_current: -i_neg_pwr,
        neg_in_partial: -pi_neg_pvin,
        neg_out_partial: -pi_neg_pvout,
        neg_pos_partial: if pos_pwr_connected {
            -pi_neg_pvpos
        } else {
            0.0
        },
        neg_neg_partial: -pi_neg_pvneg,
    }
}

fn seegen_params(ctx: &CmContext) -> CmResult<SeeGeneratorParams> {
    let tfall = finite_param(ctx, "tfall", 500.0e-12)?;
    let trise = finite_param(ctx, "trise", 20.0e-12)?;

    let tdelay = finite_param(ctx, "tdelay", 0.0)?;
    let tperiod = finite_param(ctx, "tperiod", 0.0)?;
    let inull = finite_param(ctx, "inull", 0.0)?;
    let let_value = finite_param(ctx, "let", 10.0)?;
    let cdepth = finite_param(ctx, "cdepth", 1.0)?;
    let angle = finite_param(ctx, "angle", 0.0)?.clamp(0.0, 1.57079);
    let ctrlthres = finite_param(ctx, "ctrlthres", 0.5)?;
    let perlim = ctx.param_or("perlim", 1.0) > 0.5;

    Ok(SeeGeneratorParams {
        tfall,
        trise,
        tdelay,
        tperiod,
        inull,
        let_value,
        cdepth,
        angle,
        ctrlthres,
        perlim,
    })
}

fn seegen_peak_time(start: Value, params: SeeGeneratorParams) -> Value {
    start
        + params.tfall * params.trise * (params.trise / params.tfall).ln()
            / (params.trise - params.tfall)
}

fn seegen_channel_advance_time(start: Value, params: SeeGeneratorParams) -> Option<Value> {
    (params.tperiod > 0.0).then_some(start + params.tperiod * 0.9)
}

fn seegen_request_pulse_breakpoints(ctx: &mut CmContext, start: Value, params: SeeGeneratorParams) {
    if !xtradev_commits_state(ctx) {
        return;
    }
    if start.is_finite() && start >= ctx.time {
        ctx.request_breakpoint(start);
    }
    let peak = seegen_peak_time(start, params);
    if peak.is_finite() && peak >= ctx.time {
        ctx.request_breakpoint(peak);
    }
    if let Some(advance) = seegen_channel_advance_time(start, params)
        && advance.is_finite()
        && advance >= ctx.time
    {
        ctx.request_breakpoint(advance);
    }
}

fn seegen_push_pulse_breakpoints(
    breakpoints: &mut Vec<Value>,
    start: Value,
    params: SeeGeneratorParams,
) {
    if start.is_finite() && start >= 0.0 {
        breakpoints.push(start);
    }
    let peak = seegen_peak_time(start, params);
    if peak.is_finite() && peak >= 0.0 {
        breakpoints.push(peak);
    }
    if let Some(advance) = seegen_channel_advance_time(start, params)
        && advance.is_finite()
        && advance >= 0.0
    {
        breakpoints.push(advance);
    }
}

fn seegen_effective_inull(params: SeeGeneratorParams) -> Value {
    if params.inull != 0.0 {
        return params.inull;
    }

    let let_eff = params.let_value / params.angle.cos();
    let collected_charge = 1.035e-14 * let_eff * params.cdepth;
    collected_charge / (params.tfall - params.trise)
}

fn seegen_current(time: Value, pulse_start: Value, params: SeeGeneratorParams) -> Value {
    if time < pulse_start {
        return 0.0;
    }

    let elapsed = time - pulse_start;
    let inull = seegen_effective_inull(params);
    inull * (-(elapsed / params.tfall)).exp() - inull * (-(elapsed / params.trise)).exp()
}

fn seegen_ctrl_connected(ctx: &CmContext) -> bool {
    ctx.port_width("ctrl") > 0
}

fn seegen_mon_connected(ctx: &CmContext) -> bool {
    ctx.port_width("mon") > 0
}

const SEEGEN_STATE_LAST_START: usize = 0;
const SEEGEN_STATE_LAST_CTRL: usize = 1;
const SEEGEN_INT_INITIALIZED: usize = 0;
const SEEGEN_INT_PULSE_NUMBER: usize = 1;

fn seegen_output_window_elapsed(
    time: Value,
    pulse_start: Value,
    params: SeeGeneratorParams,
) -> bool {
    time > pulse_start + params.tperiod * 0.9
}

fn seegen_initial_runtime(ctx: &CmContext, params: SeeGeneratorParams) -> (Value, Value, i64) {
    let ctrl_connected = seegen_ctrl_connected(ctx);
    let ctrl = if ctrl_connected {
        ctx.input("ctrl")
    } else {
        1.0
    };
    let first_start = if ctrl_connected {
        1.0e12
    } else {
        params.tdelay
    };

    (first_start, ctrl, 1)
}

fn node_row(node: usize) -> Option<usize> {
    node.checked_sub(1)
}

fn stamp_two_terminal_conductance(
    ctx: &mut CmContext,
    port_a: &str,
    port_b: &str,
    conductance: Value,
) {
    let a = ctx.port_node(port_a).unwrap_or(0).checked_sub(1);
    let b = ctx.port_node(port_b).unwrap_or(0).checked_sub(1);

    if let Some(row) = a {
        ctx.stamp_conductance(row, row, conductance);
    }
    if let Some(row) = b {
        ctx.stamp_conductance(row, row, conductance);
    }
    if let (Some(row_a), Some(row_b)) = (a, b) {
        ctx.stamp_conductance(row_a, row_b, -conductance);
        ctx.stamp_conductance(row_b, row_a, -conductance);
    }
}

fn stamp_pair_conductance(ctx: &mut CmContext, pair: (usize, usize), conductance: Value) {
    let (pos, neg) = pair;
    let pos = node_row(pos);
    let neg = node_row(neg);

    if let Some(row) = pos {
        ctx.stamp_conductance(row, row, conductance);
    }
    if let Some(row) = neg {
        ctx.stamp_conductance(row, row, conductance);
    }
    if let (Some(row_pos), Some(row_neg)) = (pos, neg) {
        ctx.stamp_conductance(row_pos, row_neg, -conductance);
        ctx.stamp_conductance(row_neg, row_pos, -conductance);
    }
}

fn stamp_pair_control_partial(
    ctx: &mut CmContext,
    output_pair: (usize, usize),
    control_pair: (usize, usize),
    partial: Value,
) {
    let (out_pos, out_neg) = (node_row(output_pair.0), node_row(output_pair.1));
    let (ctrl_pos, ctrl_neg) = (node_row(control_pair.0), node_row(control_pair.1));

    for (row, row_sign) in [(out_pos, 1.0), (out_neg, -1.0)] {
        let Some(row) = row else {
            continue;
        };
        if let Some(col) = ctrl_pos {
            ctx.stamp_conductance(row, col, row_sign * partial);
        }
        if let Some(col) = ctrl_neg {
            ctx.stamp_conductance(row, col, -row_sign * partial);
        }
    }
}

fn stamp_pair_control_column(
    ctx: &mut CmContext,
    output_pair: (usize, usize),
    control_column: usize,
    partial: Value,
) {
    let (out_pos, out_neg) = (node_row(output_pair.0), node_row(output_pair.1));
    if let Some(row) = out_pos {
        ctx.stamp_conductance(row, control_column, partial);
    }
    if let Some(row) = out_neg {
        ctx.stamp_conductance(row, control_column, -partial);
    }
}

fn stamp_pair_port_partial(
    ctx: &mut CmContext,
    output_pair: (usize, usize),
    control_port: &str,
    partial: Value,
) {
    if partial == 0.0 || !partial.is_finite() {
        return;
    }
    if let Some(column) = ctx.port_control_column(control_port) {
        stamp_pair_control_column(ctx, output_pair, column, partial);
    } else if let Some(control_pair) = ctx.port_node_pair(control_port) {
        stamp_pair_control_partial(ctx, output_pair, control_pair, partial);
    }
}

fn stamp_pair_current_rhs(ctx: &mut CmContext, pair: (usize, usize), equivalent: Value) {
    if let Some(row) = node_row(pair.0) {
        ctx.stamp_rhs(row, -equivalent);
    }
    if let Some(row) = node_row(pair.1) {
        ctx.stamp_rhs(row, equivalent);
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct PairPartials {
    input: Value,
    output: Value,
    pos_pwr: Value,
    neg_pwr: Value,
}

fn stamp_ilimit_current(
    ctx: &mut CmContext,
    pair: (usize, usize),
    port_name: &str,
    current: Value,
    partials: PairPartials,
) {
    stamp_pair_port_partial(ctx, pair, "in", partials.input);
    stamp_pair_conductance(ctx, pair, partials.output);
    stamp_pair_port_partial(ctx, pair, "pos_pwr", partials.pos_pwr);
    stamp_pair_port_partial(ctx, pair, "neg_pwr", partials.neg_pwr);

    if !ctx.is_ac() {
        let mut equivalent = current;
        equivalent -= partials.input * ctx.input("in");
        equivalent -= partials.output * ctx.input(port_name);
        if ilimit_port_connected(ctx, "pos_pwr") {
            equivalent -= partials.pos_pwr * ctx.input("pos_pwr");
        }
        if ilimit_port_connected(ctx, "neg_pwr") {
            equivalent -= partials.neg_pwr * ctx.input("neg_pwr");
        }
        stamp_pair_current_rhs(ctx, pair, equivalent);
    }

    ctx.set_output(port_name, current);
}

fn finite_reactive_param(ctx: &CmContext, name: &str, model: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(invalid_param(
            name,
            format!("{model}: {name} must be finite, got {value}"),
        ))
    }
}

fn transient_integrator_partial(ctx: &CmContext) -> Value {
    if ctx.is_transient() && ctx.timestep.is_finite() && ctx.timestep > 0.0 {
        ctx.timestep
    } else {
        0.0
    }
}

fn xtradev_commits_state(ctx: &CmContext) -> bool {
    ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
}

fn xtradev_set_state(ctx: &mut CmContext, index: usize, value: Value) {
    if xtradev_commits_state(ctx) {
        ctx.set_state(index, value);
    }
}

fn xtradev_set_int_state(ctx: &mut CmContext, index: usize, value: i64) {
    if xtradev_commits_state(ctx) {
        ctx.set_int_state(index, value);
    }
}

fn transient_partial(ctx: &CmContext, scale: Value) -> Value {
    if ctx.is_transient()
        && ctx.analog_ramp_factor() >= 1.0
        && ctx.timestep.is_finite()
        && ctx.timestep > 0.0
        && scale.is_finite()
        && scale != 0.0
    {
        transient_integrator_partial(ctx) / scale
    } else {
        0.0
    }
}

impl CodeModel for CapacitorIc {
    fn name(&self) -> &str {
        "capacitoric"
    }

    fn description(&self) -> &str {
        "XSPICE capacitor with voltage initial condition"
    }

    fn ports(&self) -> &[PortSpec] {
        capacitoric_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        capacitoric_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        finite_reactive_param(ctx, "c", "capacitoric")?;
        ctx.allocate_states(1);
        ctx.set_initial_state(0, ctx.param_or("ic", 0.0));
        Ok(())
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let ramptime = ctx.ramptime();
        if ramptime.is_finite() && ramptime > 0.0 {
            Ok(vec![ramptime])
        } else {
            Ok(Vec::new())
        }
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let capacitance = finite_reactive_param(ctx, "c", "capacitoric")?;
        if ctx.is_dc() {
            ctx.set_output_with_partial("cap", ctx.param_or("ic", 0.0), 0.0);
            return Ok(());
        }

        let ramp_factor = ctx.analog_ramp_factor();
        if ctx.is_transient() && ramp_factor < 1.0 {
            let output = ctx.param_or("ic", 0.0) * ramp_factor;
            xtradev_set_state(ctx, 0, output);
            ctx.set_output_with_partial("cap", output, 0.0);
            return Ok(());
        }

        let partial = transient_partial(ctx, capacitance);
        let output = if partial != 0.0 {
            ctx.state_prev(0) + partial * ctx.input("cap")
        } else {
            ctx.state_prev(0)
        };
        xtradev_set_state(ctx, 0, output);
        ctx.set_output_with_partial("cap", output, partial);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("cap") {
            return Vec::new();
        }
        let Ok(capacitance) = finite_reactive_param(ctx, "c", "capacitoric") else {
            return Vec::new();
        };
        let partial = transient_partial(ctx, capacitance);
        if partial != 0.0 {
            vec![("cap".to_string(), partial)]
        } else {
            Vec::new()
        }
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("cap") {
            return Vec::new();
        }
        let Ok(capacitance) = finite_reactive_param(ctx, "c", "capacitoric") else {
            return Vec::new();
        };
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() || omega == 0.0 {
            return Vec::new();
        }
        vec![(
            "cap".to_string(),
            Complex64::new(0.0, -1.0 / (omega * capacitance)),
        )]
    }
}

impl CodeModel for InductorIc {
    fn name(&self) -> &str {
        "inductoric"
    }

    fn description(&self) -> &str {
        "XSPICE inductor with current initial condition"
    }

    fn ports(&self) -> &[PortSpec] {
        inductoric_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        inductoric_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        finite_reactive_param(ctx, "l", "inductoric")?;
        ctx.allocate_states(1);
        ctx.set_initial_state(0, ctx.param_or("ic", 0.0));
        Ok(())
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let ramptime = ctx.ramptime();
        if ramptime.is_finite() && ramptime > 0.0 {
            Ok(vec![ramptime])
        } else {
            Ok(Vec::new())
        }
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let inductance = finite_reactive_param(ctx, "l", "inductoric")?;
        let pair = ctx.port_node_pair("ind").unwrap_or((0, 0));

        let (current, partial) = if ctx.is_dc() {
            (ctx.param_or("ic", 0.0), 0.0)
        } else if ctx.is_transient() && ctx.analog_ramp_factor() < 1.0 {
            let current = ctx.param_or("ic", 0.0) * ctx.analog_ramp_factor();
            xtradev_set_state(ctx, 0, current);
            (current, 0.0)
        } else {
            let partial = transient_partial(ctx, inductance);
            let current = if partial != 0.0 {
                ctx.state_prev(0) + partial * ctx.input("ind")
            } else {
                ctx.state_prev(0)
            };
            xtradev_set_state(ctx, 0, current);
            (current, partial)
        };

        if partial != 0.0 {
            stamp_pair_conductance(ctx, pair, partial);
        }
        if !ctx.is_ac() {
            stamp_pair_current_rhs(ctx, pair, current - partial * ctx.input("ind"));
        }
        ctx.set_output_with_partial("ind", current, partial);
        Ok(())
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("ind") {
            return Vec::new();
        }
        let Ok(inductance) = finite_reactive_param(ctx, "l", "inductoric") else {
            return Vec::new();
        };
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() || omega == 0.0 {
            return Vec::new();
        }
        vec![("ind".to_string(), Complex64::new(0.0, omega * inductance))]
    }
}

impl CodeModel for Potentiometer {
    fn name(&self) -> &str {
        "potentiometer"
    }

    fn description(&self) -> &str {
        "Three-terminal analog potentiometer"
    }

    fn ports(&self) -> &[PortSpec] {
        potentiometer_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        potentiometer_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        potentiometer_split(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let split = potentiometer_split(ctx)?;

        stamp_two_terminal_conductance(ctx, "r0", "wiper", split.g_lower);
        stamp_two_terminal_conductance(ctx, "wiper", "r1", split.g_upper);

        let vr0 = ctx.input("r0");
        let vwiper = ctx.input("wiper");
        let vr1 = ctx.input("r1");
        ctx.set_output("r0", (vr0 - vwiper) / split.r_lower);
        ctx.set_output("r1", (vr1 - vwiper) / split.r_upper);
        ctx.set_output(
            "wiper",
            ((vwiper - vr0) / split.r_lower) + ((vwiper - vr1) / split.r_upper),
        );

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Ok(split) = potentiometer_split(ctx) else {
            return Vec::new();
        };
        match xtradev_port_key(output_port) {
            XtradevPortKey::R0 => vec![
                ("r0".to_string(), split.g_lower),
                ("wiper".to_string(), -split.g_lower),
            ],
            XtradevPortKey::Wiper => vec![
                ("r0".to_string(), -split.g_lower),
                ("wiper".to_string(), split.g_lower + split.g_upper),
                ("r1".to_string(), -split.g_upper),
            ],
            XtradevPortKey::R1 => vec![
                ("wiper".to_string(), -split.g_upper),
                ("r1".to_string(), split.g_upper),
            ],
            _ => Vec::new(),
        }
    }
}

impl CodeModel for AnalogSwitch {
    fn name(&self) -> &str {
        "aswitch"
    }

    fn description(&self) -> &str {
        "Official XSPICE analog switch"
    }

    fn ports(&self) -> &[PortSpec] {
        aswitch_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        aswitch_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        aswitch_eval(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = aswitch_eval(ctx)?;
        let output_pair = ctx.port_node_pair("out").unwrap_or((0, 0));

        stamp_pair_conductance(ctx, output_pair, eval.output_conductance);
        stamp_pair_port_partial(ctx, output_pair, "cntl_in", eval.control_partial);

        let output_equivalent = eval.output_current
            - eval.output_conductance * ctx.input("out")
            - eval.control_partial * ctx.input("cntl_in");
        stamp_pair_current_rhs(ctx, output_pair, output_equivalent);

        ctx.set_output("out", eval.output_current);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let Ok(eval) = aswitch_eval(ctx) else {
            return Vec::new();
        };
        vec![
            ("out".to_string(), eval.output_conductance),
            ("cntl_in".to_string(), eval.control_partial),
        ]
    }
}

impl CodeModel for Pswitch {
    fn name(&self) -> &str {
        "pswitch"
    }

    fn description(&self) -> &str {
        "PSPICE-compatible analog switch"
    }

    fn ports(&self) -> &[PortSpec] {
        pswitch_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        pswitch_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        pswitch_eval(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = pswitch_eval(ctx)?;
        let control_pair = ctx.port_node_pair("cntl_in").unwrap_or((0, 0));
        let output_pair = ctx.port_node_pair("out").unwrap_or((0, 0));

        stamp_pair_conductance(ctx, output_pair, eval.output_conductance);
        stamp_pair_control_partial(ctx, output_pair, control_pair, eval.control_partial);
        let output_equivalent = eval.output_current
            - eval.output_conductance * ctx.input("out")
            - eval.control_partial * ctx.input("cntl_in");
        stamp_pair_current_rhs(ctx, output_pair, output_equivalent);

        stamp_pair_conductance(ctx, control_pair, eval.control_conductance);

        ctx.set_output("out", eval.output_current);
        ctx.set_output("cntl_in", eval.control_current);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Ok(eval) = pswitch_eval(ctx) else {
            return Vec::new();
        };
        match xtradev_port_key(output_port) {
            XtradevPortKey::Out => vec![
                ("out".to_string(), eval.output_conductance),
                ("cntl_in".to_string(), eval.control_partial),
            ],
            XtradevPortKey::CntlIn => vec![("cntl_in".to_string(), eval.control_conductance)],
            _ => Vec::new(),
        }
    }
}

impl CodeModel for Sidiode {
    fn name(&self) -> &str {
        "sidiode"
    }

    fn description(&self) -> &str {
        "Simple diode"
    }

    fn ports(&self) -> &[PortSpec] {
        sidiode_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        sidiode_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        sidiode_eval(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = sidiode_eval(ctx)?;
        let pair = ctx.port_node_pair("ds").unwrap_or((0, 0));

        stamp_pair_conductance(ctx, pair, eval.derivative);
        if !ctx.is_ac() {
            let equivalent = eval.current - eval.derivative * ctx.input("ds");
            stamp_pair_current_rhs(ctx, pair, equivalent);
        }
        ctx.set_output("ds", eval.current);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("ds") {
            return Vec::new();
        }
        let Ok(eval) = sidiode_eval(ctx) else {
            return Vec::new();
        };
        vec![("ds".to_string(), eval.derivative)]
    }
}

impl CodeModel for Zener {
    fn name(&self) -> &str {
        "zener"
    }

    fn description(&self) -> &str {
        "Zener diode"
    }

    fn ports(&self) -> &[PortSpec] {
        zener_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        zener_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(1);
        ctx.set_initial_state(0, 0.0);
        zener_eval_for_voltage(ctx, ctx.input("z"))?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let voltage = zener_limited_voltage(ctx);
        let eval = zener_eval_for_voltage(ctx, voltage)?;
        let pair = ctx.port_node_pair("z").unwrap_or((0, 0));

        stamp_pair_conductance(ctx, pair, eval.derivative);
        if !ctx.is_ac() {
            let equivalent = eval.current - eval.derivative * voltage;
            stamp_pair_current_rhs(ctx, pair, equivalent);
        }
        ctx.set_output("z", eval.current);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("z") {
            return Vec::new();
        }
        let Ok(eval) = zener_eval_for_voltage(ctx, ctx.input("z")) else {
            return Vec::new();
        };
        vec![("z".to_string(), eval.derivative)]
    }
}

impl CodeModel for Memristor {
    fn name(&self) -> &str {
        "memristor"
    }

    fn description(&self) -> &str {
        "Threshold memristive device"
    }

    fn ports(&self) -> &[PortSpec] {
        memristor_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        memristor_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = memristor_params(ctx)?;
        ctx.allocate_states(1);
        ctx.set_initial_state(0, params.rinit);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = memristor_eval(ctx)?;
        let pair = ctx.port_node_pair("memris").unwrap_or((0, 0));

        stamp_pair_conductance(ctx, pair, eval.derivative);
        if !ctx.is_ac() {
            let equivalent = eval.current - eval.derivative * ctx.input("memris");
            stamp_pair_current_rhs(ctx, pair, equivalent);
        }
        ctx.set_output("memris", eval.current);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("memris") {
            return Vec::new();
        }
        let Ok((eval, _)) = memristor_eval_for_context(ctx) else {
            return Vec::new();
        };
        vec![("memris".to_string(), eval.derivative)]
    }
}

impl CodeModel for Core {
    fn name(&self) -> &str {
        "core"
    }

    fn description(&self) -> &str {
        "Magnetic core"
    }

    fn ports(&self) -> &[PortSpec] {
        core_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        core_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        core_validate_common(ctx)?;
        if core_mode(ctx)? == CORE_MODE_HYSTERESIS {
            core_hysteresis_params(ctx)?;
        }
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, CORE_HYSTERESIS_UNINITIALIZED);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = core_eval(ctx)?;
        let pair = ctx.port_node_pair("mc").unwrap_or((0, 0));

        stamp_pair_conductance(ctx, pair, eval.derivative);
        if !ctx.is_ac() {
            let equivalent = eval.current - eval.derivative * ctx.input("mc");
            stamp_pair_current_rhs(ctx, pair, equivalent);
        }
        ctx.set_output("mc", eval.current);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("mc") {
            return Vec::new();
        }
        let Ok(eval) = core_eval_for_context(ctx) else {
            return Vec::new();
        };
        vec![("mc".to_string(), eval.derivative)]
    }
}

impl CodeModel for CapacitanceMeter {
    fn name(&self) -> &str {
        "cmeter"
    }

    fn description(&self) -> &str {
        "ATESSE-compatible capacitance meter"
    }

    fn ports(&self) -> &[PortSpec] {
        meter_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        meter_parameters()
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let output =
            ctx.param_or("gain", 1.0) * ctx.param_or(XTRADEV_METER_MEASURED_VALUE_PARAM, 0.0);
        ctx.set_output("out", output);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }
}

impl CodeModel for InductanceMeter {
    fn name(&self) -> &str {
        "lmeter"
    }

    fn description(&self) -> &str {
        "ATESSE-compatible inductance meter"
    }

    fn ports(&self) -> &[PortSpec] {
        meter_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        meter_parameters()
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let output =
            ctx.param_or("gain", 1.0) * ctx.param_or(XTRADEV_METER_MEASURED_VALUE_PARAM, 0.0);
        ctx.set_output("out", output);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }
}

impl CodeModel for LcCouple {
    fn name(&self) -> &str {
        "lcouple"
    }

    fn description(&self) -> &str {
        "Inductive coupling for magnetic core models"
    }

    fn ports(&self) -> &[PortSpec] {
        lcouple_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        lcouple_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let num_turns = ctx.param_or("num_turns", 1.0);
        let input_current = ctx.input("l");
        let in_flux = -ctx.input("mmf_out");

        ctx.set_output("mmf_out", num_turns * input_current);

        if ctx.time == 0.0 || !ctx.is_transient() {
            ctx.set_output("l", 0.0);
        } else {
            let delta = ctx.time - ctx.time_prev;
            if !(delta > 0.0 && delta.is_finite()) {
                return Err(CmError::InvalidParameter {
                    name: "time".to_string(),
                    message: format!("lcouple requires a positive finite timestep, got {delta}"),
                });
            }
            ctx.set_output("l", num_turns * (in_flux - ctx.state_prev(0)) / delta);
        }

        xtradev_set_state(ctx, 0, in_flux);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let num_turns = ctx.param_or("num_turns", 1.0);
        match xtradev_port_key(output_port) {
            XtradevPortKey::MmfOut => vec![("l".to_string(), num_turns)],
            XtradevPortKey::L if ctx.time != 0.0 && ctx.is_transient() => {
                let delta = ctx.time - ctx.time_prev;
                if delta > 0.0 && delta.is_finite() {
                    vec![("mmf_out".to_string(), -num_turns / delta)]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, crate::Complex64)> {
        let num_turns = ctx.param_or("num_turns", 1.0);
        match xtradev_port_key(output_port) {
            XtradevPortKey::L => vec![(
                "mmf_out".to_string(),
                crate::Complex64::new(0.0, num_turns * std::f64::consts::TAU * frequency),
            )],
            XtradevPortKey::MmfOut => {
                vec![("l".to_string(), crate::Complex64::new(num_turns, 0.0))]
            }
            _ => Vec::new(),
        }
    }
}

impl CodeModel for Ilimit {
    fn name(&self) -> &str {
        "ilimit"
    }

    fn description(&self) -> &str {
        "Current-limited analog output driver"
    }

    fn ports(&self) -> &[PortSpec] {
        ilimit_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        ilimit_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ilimit_params(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = cache_ilimit_eval(ctx)?;
        let out_pair = ctx.port_node_pair("out").unwrap_or((0, 0));
        stamp_ilimit_current(
            ctx,
            out_pair,
            "out",
            eval.out_current,
            PairPartials {
                input: eval.out_in_partial,
                output: eval.out_out_partial,
                pos_pwr: eval.out_pos_partial,
                neg_pwr: eval.out_neg_partial,
            },
        );

        if ilimit_port_connected(ctx, "pos_pwr") {
            let pos_pair = ctx.port_node_pair("pos_pwr").unwrap_or((0, 0));
            stamp_ilimit_current(
                ctx,
                pos_pair,
                "pos_pwr",
                eval.pos_current,
                PairPartials {
                    input: eval.pos_in_partial,
                    output: eval.pos_out_partial,
                    pos_pwr: eval.pos_pos_partial,
                    neg_pwr: eval.pos_neg_partial,
                },
            );
        }

        if ilimit_port_connected(ctx, "neg_pwr") {
            let neg_pair = ctx.port_node_pair("neg_pwr").unwrap_or((0, 0));
            stamp_ilimit_current(
                ctx,
                neg_pair,
                "neg_pwr",
                eval.neg_current,
                PairPartials {
                    input: eval.neg_in_partial,
                    output: eval.neg_out_partial,
                    pos_pwr: eval.neg_pos_partial,
                    neg_pwr: eval.neg_neg_partial,
                },
            );
        }

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        fn partials(items: [(&str, Value); 4]) -> Vec<(String, Value)> {
            items
                .into_iter()
                .filter_map(|(port, partial)| {
                    (partial != 0.0 && partial.is_finite()).then(|| (port.to_string(), partial))
                })
                .collect()
        }

        let Ok(eval) = ilimit_eval_for_context(ctx) else {
            return Vec::new();
        };

        match xtradev_port_key(output_port) {
            XtradevPortKey::Out => partials([
                ("in", eval.out_in_partial),
                ("out", eval.out_out_partial),
                ("pos_pwr", eval.out_pos_partial),
                ("neg_pwr", eval.out_neg_partial),
            ]),
            XtradevPortKey::PosPwr if ilimit_port_connected(ctx, "pos_pwr") => partials([
                ("in", eval.pos_in_partial),
                ("out", eval.pos_out_partial),
                ("pos_pwr", eval.pos_pos_partial),
                ("neg_pwr", eval.pos_neg_partial),
            ]),
            XtradevPortKey::NegPwr if ilimit_port_connected(ctx, "neg_pwr") => partials([
                ("in", eval.neg_in_partial),
                ("out", eval.neg_out_partial),
                ("pos_pwr", eval.neg_pos_partial),
                ("neg_pwr", eval.neg_neg_partial),
            ]),
            _ => Vec::new(),
        }
    }
}

impl CodeModel for SeeGenerator {
    fn name(&self) -> &str {
        "seegen"
    }

    fn description(&self) -> &str {
        "Single-event-effect current generator"
    }

    fn ports(&self) -> &[PortSpec] {
        seegen_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        seegen_parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(2);
        ctx.allocate_int_states(2);
        ctx.set_initial_state(SEEGEN_STATE_LAST_START, 1.0e12);
        ctx.set_initial_state(SEEGEN_STATE_LAST_CTRL, 0.0);
        ctx.set_int_state(SEEGEN_INT_INITIALIZED, 0);
        ctx.set_int_state(SEEGEN_INT_PULSE_NUMBER, 1);
        seegen_params(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = seegen_params(ctx)?;
        let width = ctx.port_width("out").max(1);

        if ctx.is_ac() {
            return Ok(());
        }
        if !ctx.is_transient() {
            ctx.set_output_vector_from_fn("out", width, |_| 0.0);
            ctx.set_output("mon", 0.0);
            return Ok(());
        }

        let ctrl_connected = seegen_ctrl_connected(ctx);
        let ctrl = if ctrl_connected {
            ctx.input("ctrl")
        } else {
            1.0
        };

        let initial_runtime = (ctx.int_state(SEEGEN_INT_INITIALIZED) == 0)
            .then(|| seegen_initial_runtime(ctx, params));
        let mut pulse_start = initial_runtime
            .map(|(start, _, _)| start)
            .unwrap_or_else(|| ctx.state(SEEGEN_STATE_LAST_START));
        let mut pulse_number = initial_runtime
            .map(|(_, _, number)| number)
            .unwrap_or_else(|| ctx.int_state(SEEGEN_INT_PULSE_NUMBER).max(1));
        let last_ctrl = initial_runtime
            .map(|(_, last_ctrl, _)| last_ctrl)
            .unwrap_or_else(|| ctx.state(SEEGEN_STATE_LAST_CTRL));
        let mut assign_outputs = true;

        if ctrl_connected && last_ctrl < params.ctrlthres && ctrl >= params.ctrlthres {
            pulse_start = ctx.time + params.tdelay;
            pulse_number = 1;
            assign_outputs = true;
            seegen_request_pulse_breakpoints(ctx, pulse_start, params);
        }

        let current = if (1..=width as i64).contains(&pulse_number) {
            seegen_current(ctx.time, pulse_start, params)
        } else {
            0.0
        };

        if seegen_output_window_elapsed(ctx.time, pulse_start, params) {
            let next_pulse_number = pulse_number + 1;
            if next_pulse_number > width as i64 && params.perlim {
                pulse_start = 1.0e12;
                pulse_number = next_pulse_number;
                assign_outputs = false;
            } else {
                pulse_start += params.tperiod;
                pulse_number = if next_pulse_number > width as i64 {
                    1
                } else {
                    next_pulse_number
                };
                seegen_request_pulse_breakpoints(ctx, pulse_start, params);
            }
        }

        if assign_outputs && (1..=width as i64).contains(&pulse_number) {
            let active_index = pulse_number as usize - 1;
            ctx.set_output_vector_from_fn("out", width, |index| {
                if index == active_index { current } else { 0.0 }
            });
            if seegen_mon_connected(ctx) {
                ctx.set_output("mon", current);
            } else {
                ctx.set_output("mon", 0.0);
            }
        }
        xtradev_set_state(ctx, SEEGEN_STATE_LAST_START, pulse_start);
        xtradev_set_state(ctx, SEEGEN_STATE_LAST_CTRL, ctrl);
        xtradev_set_int_state(ctx, SEEGEN_INT_INITIALIZED, 1);
        xtradev_set_int_state(ctx, SEEGEN_INT_PULSE_NUMBER, pulse_number);

        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        Vec::new()
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let params = seegen_params(ctx)?;
        if seegen_ctrl_connected(ctx) {
            return Ok(Vec::new());
        }

        let width = ctx.port_width("out").max(1);
        let pulse_count = if params.tperiod > 0.0 && params.perlim {
            width
        } else {
            1
        };

        let mut breakpoints = Vec::with_capacity(pulse_count * 3);
        for index in 0..pulse_count {
            let start = params.tdelay + params.tperiod.max(0.0) * index as Value;
            seegen_push_pulse_breakpoints(&mut breakpoints, start, params);
        }
        Ok(breakpoints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ilimit_cache_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("gain", 1.0);
        ctx.set_param("r_out_source", 1000.0);
        ctx.set_param("r_out_sink", 1000.0);
        ctx.set_param("i_limit_source", 1.0);
        ctx.set_param("i_limit_sink", 1.0);
        ctx.set_param("v_pwr_range", 1.0e-6);
        ctx.set_param("i_source_range", 1.0e-9);
        ctx.set_param("i_sink_range", 1.0e-9);
        ctx.set_param("r_out_domain", 1.0e-9);
        ctx.set_port_width("pos_pwr", 0);
        ctx.set_port_width("neg_pwr", 0);
        ctx.set_input_analog("in", 1.0);
        ctx.set_input_analog("out", 0.25);
        ctx
    }

    fn sentinel_ilimit_eval() -> IlimitEval {
        IlimitEval {
            out_current: 123.0,
            out_in_partial: 3.0,
            out_out_partial: 4.0,
            out_pos_partial: 0.0,
            out_neg_partial: 0.0,
            pos_current: 0.0,
            pos_in_partial: 0.0,
            pos_out_partial: 0.0,
            pos_pos_partial: 0.0,
            pos_neg_partial: 0.0,
            neg_current: 0.0,
            neg_in_partial: 0.0,
            neg_out_partial: 0.0,
            neg_pos_partial: 0.0,
            neg_neg_partial: 0.0,
        }
    }

    #[test]
    fn logarithmic_split_matches_official_formula() {
        let mut ctx = CmContext::new();
        ctx.set_param("position", 0.5);
        ctx.set_param("r", 1.0e5);
        ctx.set_param("log", 1.0);
        ctx.set_param("log_multiplier", 2.0);

        let split = potentiometer_split(&ctx).expect("valid split");

        assert!((split.r_lower - 1.0e4).abs() < 1.0e-9);
        assert!((split.r_upper - 9.0e4).abs() < 1.0e-9);
    }

    #[test]
    fn end_positions_are_guarded_like_official_model() {
        let mut ctx = CmContext::new();
        ctx.set_param("position", 0.0);
        ctx.set_param("r", 1.0e5);

        let low = potentiometer_split(&ctx).expect("guarded low split");
        assert!(low.r_lower > 0.0);
        assert!(low.r_upper > 0.0);

        ctx.set_param("position", 1.0);
        let high = potentiometer_split(&ctx).expect("guarded high split");
        assert!(high.r_lower > 0.0);
        assert!(high.r_upper > 0.0);
    }

    #[test]
    fn ilimit_eval_cache_reuses_current_result_until_inputs_change() {
        let mut ctx = ilimit_cache_context();
        let direct = ilimit_eval(&ctx).expect("direct ilimit eval");
        assert_eq!(
            cache_ilimit_eval(&mut ctx).expect("cache ilimit eval"),
            direct
        );

        let signature = ilimit_eval_signature(&ctx).expect("current ilimit signature");
        let sentinel = sentinel_ilimit_eval();
        ctx.set_resource(
            ILIMIT_EVAL_RESOURCE,
            Arc::new(IlimitEvalResource {
                signature,
                eval: sentinel,
            }),
        );

        assert_eq!(
            cache_ilimit_eval(&mut ctx).expect("matching mutable eval reuses cache"),
            sentinel
        );
        assert_eq!(
            ilimit_eval_for_context(&ctx).expect("matching read-only eval reuses cache"),
            sentinel
        );
        assert_eq!(
            Ilimit.output_input_partials(&ctx, "out"),
            vec![("in".to_string(), 3.0), ("out".to_string(), 4.0)]
        );

        ctx.set_input_analog("out", 0.5);
        let updated = ilimit_eval_for_context(&ctx).expect("changed output invalidates cache");
        assert_eq!(
            updated,
            ilimit_eval(&ctx).expect("direct updated ilimit eval")
        );
        assert_ne!(updated, sentinel);
    }

    #[test]
    fn ilimit_eval_cache_invalidates_when_optional_power_port_connects() {
        let mut ctx = ilimit_cache_context();
        let signature = ilimit_eval_signature(&ctx).expect("current ilimit signature");
        let sentinel = sentinel_ilimit_eval();
        ctx.set_resource(
            ILIMIT_EVAL_RESOURCE,
            Arc::new(IlimitEvalResource {
                signature,
                eval: sentinel,
            }),
        );

        assert_eq!(
            ilimit_eval_for_context(&ctx).expect("matching null-power eval reuses cache"),
            sentinel
        );

        ctx.set_port_width("pos_pwr", 1);
        ctx.set_input_analog("pos_pwr", 5.0);
        let updated =
            ilimit_eval_for_context(&ctx).expect("connected power input invalidates cache");
        assert_eq!(
            updated,
            ilimit_eval(&ctx).expect("direct power-connected eval")
        );
        assert_ne!(updated, sentinel);
    }

    #[test]
    fn pswitch_linear_region_matches_official_formula() {
        let mut ctx = CmContext::new();
        ctx.set_param("log", 0.0);
        ctx.set_param("cntl_off", 0.0);
        ctx.set_param("cntl_on", 1.0);
        ctx.set_param("r_on", 1000.0);
        ctx.set_param("r_off", 9000.0);
        ctx.set_param("r_cntl_in", 1.0e12);
        ctx.set_input_analog("cntl_in", 0.5);
        ctx.set_input_analog("out", 0.5);

        let eval = pswitch_eval(&ctx).expect("linear pswitch eval");

        assert!((eval.resistance - 5000.0).abs() < 1.0e-9);
        assert!((eval.output_conductance - 2.0e-4).abs() < 1.0e-15);
        assert!((eval.control_partial - 1.6e-4).abs() < 1.0e-15);
    }

    #[test]
    fn pswitch_log_region_matches_official_formula_at_midpoint() {
        let mut ctx = CmContext::new();
        ctx.set_param("log", 1.0);
        ctx.set_param("cntl_off", 0.0);
        ctx.set_param("cntl_on", 1.0);
        ctx.set_param("r_on", 1000.0);
        ctx.set_param("r_off", 9000.0);
        ctx.set_param("r_cntl_in", 1.0e12);
        ctx.set_input_analog("cntl_in", 0.5);
        ctx.set_input_analog("out", 0.5);

        let eval = pswitch_eval(&ctx).expect("log pswitch eval");

        assert!((eval.resistance - 3000.0).abs() < 1.0e-9);
    }

    #[test]
    fn sidiode_forward_no_limit_region_matches_official_formula() {
        let mut ctx = CmContext::new();
        ctx.set_param("ron", 1000.0);
        ctx.set_param("roff", 1.0e12);
        ctx.set_param("vfwd", 0.0);
        ctx.set_param("vrev", 1.0e30);
        ctx.set_param("ilimit", 1.0e30);
        ctx.set_param("revilimit", 1.0e30);
        ctx.set_param("epsilon", 0.0);
        ctx.set_param("revepsilon", 0.0);
        ctx.set_param("rrev", 0.0);
        ctx.set_input_analog("ds", 5.0);

        let eval = sidiode_eval(&ctx).expect("sidiode eval");

        assert!((eval.current - 5.0e-3).abs() < 1.0e-15);
        assert!((eval.derivative - 1.0e-3).abs() < 1.0e-15);
    }

    #[test]
    fn zener_default_three_segment_forward_formula_matches_official_model() {
        let mut ctx = CmContext::new();
        ctx.set_param("v_breakdown", 5.0);
        ctx.set_input_analog("z", 0.1);

        let eval = zener_eval_for_voltage(&ctx, 0.1).expect("zener eval");
        let expected_current = 1.0e-12 * ((0.1 / 0.026_f64).exp() - 1.0) + 1.0e-6 / 5.0 * 0.1;
        let expected_derivative = 1.0e-12 / 0.026_f64 * (0.1 / 0.026_f64).exp() + 1.0e-6 / 5.0;

        assert!((eval.current - expected_current).abs() < 1.0e-18);
        assert!((eval.derivative - expected_derivative).abs() < 1.0e-18);
    }

    #[test]
    fn zener_limit_switch_probe_does_not_commit_previous_voltage() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("v_breakdown", 1.0);
        ctx.set_param("i_breakdown", 2.0e-2);
        ctx.set_param("r_breakdown", 1.0);
        ctx.set_param("i_rev", 1.0e-6);
        ctx.set_param("i_sat", 1.0e-12);
        ctx.set_param("n_forward", 1.0);
        ctx.set_param("limit_switch", 1.0);

        Zener.init(&mut ctx).expect("zener init");

        ctx.set_input_analog("z", 5.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        Zener.evaluate(&mut ctx).expect("probe zener");
        assert_eq!(
            ctx.state(0),
            0.0,
            "rollbackable zener limiter probe must not commit previous voltage"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Zener.evaluate(&mut ctx).expect("commit zener");
        assert!(
            (ctx.state(0) - 0.1).abs() < 1.0e-15,
            "accepted zener limiter step should commit the limited voltage, got {:e}",
            ctx.state(0)
        );
    }

    #[test]
    fn memristor_rollbackable_probe_does_not_commit_resistance_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("rmin", 10.0);
        ctx.set_param("rmax", 1000.0);
        ctx.set_param("rinit", 100.0);
        ctx.set_param("alpha", 0.0);
        ctx.set_param("beta", 1.0e9);
        ctx.set_param("vt", 0.0);
        ctx.set_input_analog("memris", 1.0);
        ctx.time = 1.0e-9;
        ctx.timestep = 1.0e-9;

        Memristor.init(&mut ctx).expect("memristor init");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        Memristor.evaluate(&mut ctx).expect("probes memristor");
        assert_eq!(
            ctx.state(0),
            100.0,
            "rollbackable memristor probe must not commit resistance state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Memristor.evaluate(&mut ctx).expect("commits memristor");
        assert!(
            ctx.state(0) > 100.0,
            "accepted memristor evaluation should commit resistance state"
        );
    }

    #[test]
    fn memristor_transient_partial_linearizes_current() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("rmin", 10.0);
        ctx.set_param("rmax", 1000.0);
        ctx.set_param("rinit", 100.0);
        ctx.set_param("alpha", 0.0);
        ctx.set_param("beta", 2.0);
        ctx.set_param("vt", 0.0);
        ctx.set_input_analog("memris", 4.0);
        ctx.time = 0.5;
        ctx.timestep = 0.5;

        Memristor.init(&mut ctx).expect("memristor init");
        let (eval, resistance) =
            memristor_eval_for_context(&ctx).expect("memristor transient eval");
        let expected_resistance = 104.0;
        let expected_current = 4.0 / expected_resistance;
        let expected_derivative =
            1.0 / expected_resistance - 4.0 / (expected_resistance * expected_resistance);

        assert!((resistance - expected_resistance).abs() < 1.0e-12);
        assert!((eval.current - expected_current).abs() < 1.0e-15);
        assert!((eval.derivative - expected_derivative).abs() < 1.0e-15);

        let partials = Memristor.output_input_partials(&ctx, "memris");
        assert_eq!(partials.len(), 1);
        assert_eq!(partials[0].0, "memris");
        assert!((partials[0].1 - expected_derivative).abs() < 1.0e-15);
        assert_eq!(
            ctx.state(0),
            100.0,
            "querying memristor partials must not commit resistance state"
        );
    }

    #[test]
    fn capacitoric_rollbackable_probe_does_not_commit_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("c", 2.0);
        ctx.set_param("ic", 1.0);
        ctx.set_input_analog("cap", 4.0);
        ctx.time = 0.5;
        ctx.timestep = 0.5;

        CapacitorIc.init(&mut ctx).expect("capacitoric init");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        CapacitorIc.evaluate(&mut ctx).expect("probes capacitoric");
        assert_eq!(
            ctx.state(0),
            1.0,
            "rollbackable capacitoric probe must not commit stored voltage"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        CapacitorIc.evaluate(&mut ctx).expect("commits capacitoric");
        assert_eq!(ctx.state(0), 2.0);
    }

    #[test]
    fn capacitoric_ramptime_ramps_initial_condition_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("c", 2.0);
        ctx.set_param("ic", 4.0);
        ctx.set_ramptime(2.0);
        ctx.set_input_analog("cap", 100.0);
        ctx.time = 0.5;
        ctx.timestep = 0.5;

        CapacitorIc.init(&mut ctx).expect("capacitoric init");
        CapacitorIc.evaluate(&mut ctx).expect("ramps capacitoric");

        assert_eq!(ctx.output("cap"), 1.0);
        assert_eq!(ctx.partial("cap"), 0.0);
        assert_eq!(ctx.state(0), 1.0);
        assert_eq!(
            CapacitorIc
                .transient_breakpoints(&ctx)
                .expect("breakpoints"),
            vec![2.0]
        );
    }

    #[test]
    fn inductoric_rollbackable_probe_does_not_commit_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("l", 2.0);
        ctx.set_param("ic", 1.0);
        ctx.set_input_analog("ind", 4.0);
        ctx.time = 0.5;
        ctx.timestep = 0.5;

        InductorIc.init(&mut ctx).expect("inductoric init");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        InductorIc.evaluate(&mut ctx).expect("probes inductoric");
        assert_eq!(
            ctx.state(0),
            1.0,
            "rollbackable inductoric probe must not commit stored current"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        InductorIc.evaluate(&mut ctx).expect("commits inductoric");
        assert_eq!(ctx.state(0), 2.0);
    }

    #[test]
    fn inductoric_ramptime_ramps_initial_condition_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("l", 2.0);
        ctx.set_param("ic", 4.0);
        ctx.set_ramptime(2.0);
        ctx.set_input_analog("ind", 100.0);
        ctx.time = 0.5;
        ctx.timestep = 0.5;

        InductorIc.init(&mut ctx).expect("inductoric init");
        InductorIc.evaluate(&mut ctx).expect("ramps inductoric");

        assert_eq!(ctx.output("ind"), 1.0);
        assert_eq!(ctx.partial("ind"), 0.0);
        assert_eq!(ctx.state(0), 1.0);
        assert_eq!(
            InductorIc.transient_breakpoints(&ctx).expect("breakpoints"),
            vec![2.0]
        );
    }

    #[test]
    fn lcouple_rollbackable_probe_does_not_commit_flux_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("num_turns", 3.0);
        ctx.set_input_analog("l", 0.0);
        ctx.set_input_analog("mmf_out", -2.0);
        ctx.time_prev = 0.0;
        ctx.time = 1.0e-9;
        ctx.timestep = 1.0e-9;

        LcCouple.init(&mut ctx).expect("lcouple init");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        LcCouple.evaluate(&mut ctx).expect("probes lcouple");
        assert_eq!(
            ctx.state(0),
            0.0,
            "rollbackable lcouple probe must not commit flux state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        LcCouple.evaluate(&mut ctx).expect("commits lcouple");
        assert_eq!(ctx.state(0), 2.0);
    }

    #[test]
    fn seegen_rollbackable_probe_does_not_commit_runtime_or_breakpoints() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_port_width("ctrl", 0);
        ctx.set_port_width("mon", 0);
        ctx.set_port_width("out", 2);
        ctx.set_param("tfall", 2.0e-9);
        ctx.set_param("trise", 0.2e-9);
        ctx.set_param("tdelay", 0.0);
        ctx.set_param("tperiod", 1.0e-9);
        ctx.set_param("inull", 1.0);
        ctx.time = 0.9e-9 + 5.0e-19;

        SeeGenerator.init(&mut ctx).expect("seegen init");
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        SeeGenerator.evaluate(&mut ctx).expect("probes seegen");
        assert_eq!(
            ctx.int_state(SEEGEN_INT_INITIALIZED),
            0,
            "rollbackable seegen probe must not mark runtime initialized"
        );
        assert_eq!(
            ctx.int_state(SEEGEN_INT_PULSE_NUMBER),
            1,
            "rollbackable seegen probe must not commit pulse channel"
        );
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable seegen probe must not queue dynamic breakpoints"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SeeGenerator.evaluate(&mut ctx).expect("commits seegen");
        assert_eq!(ctx.int_state(SEEGEN_INT_INITIALIZED), 1);
        assert_eq!(ctx.int_state(SEEGEN_INT_PULSE_NUMBER), 2);
    }

    #[test]
    fn seegen_advances_output_channel_at_official_window_boundary() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_port_width("ctrl", 0);
        ctx.set_port_width("mon", 0);
        ctx.set_port_width("out", 2);
        ctx.set_param("tfall", 2.0e-9);
        ctx.set_param("trise", 0.2e-9);
        ctx.set_param("tdelay", 0.0);
        ctx.set_param("tperiod", 1.0e-9);
        ctx.set_param("inull", 1.0);

        SeeGenerator.init(&mut ctx).expect("seegen init");
        SeeGenerator
            .evaluate(&mut ctx)
            .expect("initial seegen evaluation");

        ctx.time = 0.9e-9 + 5.0e-19;
        SeeGenerator
            .evaluate(&mut ctx)
            .expect("boundary seegen evaluation");

        let out = ctx.output_vector("out");
        assert_eq!(ctx.int_state(SEEGEN_INT_PULSE_NUMBER), 2);
        assert!(
            out[0].abs() <= 1.0e-15 && out[1] > 0.0,
            "ngspice advances seegen output channel for TIME > start + 0.9*tperiod; got {out:?}"
        );
    }

    #[test]
    fn seegen_large_timestep_advances_only_one_channel_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        ctx.set_port_width("ctrl", 0);
        ctx.set_port_width("mon", 0);
        ctx.set_port_width("out", 3);
        ctx.set_param("tfall", 1.0e-9);
        ctx.set_param("trise", 0.2e-9);
        ctx.set_param("tdelay", 0.5e-9);
        ctx.set_param("tperiod", 1.0e-9);
        ctx.set_param("inull", 1.0e-3);
        ctx.set_param("perlim", 0.0);

        SeeGenerator.init(&mut ctx).expect("seegen init");
        SeeGenerator
            .evaluate(&mut ctx)
            .expect("initial seegen evaluation");

        ctx.time = 5.0e-9;
        SeeGenerator
            .evaluate(&mut ctx)
            .expect("large-step seegen evaluation");
        assert_eq!(ctx.int_state(SEEGEN_INT_PULSE_NUMBER), 2);
        assert!(
            (ctx.state(SEEGEN_STATE_LAST_START) - 1.5e-9).abs() < 1.0e-21,
            "ngspice advances seegen by one period per call, got start={:e}",
            ctx.state(SEEGEN_STATE_LAST_START)
        );

        ctx.time = 5.1e-9;
        SeeGenerator
            .evaluate(&mut ctx)
            .expect("follow-up seegen evaluation");
        let out = ctx.output_vector("out");
        let expected = 1.0e-3 * (-3.6_f64).exp() - 1.0e-3 * (-18.0_f64).exp();
        assert_eq!(ctx.int_state(SEEGEN_INT_PULSE_NUMBER), 3);
        assert!(
            out[0].abs() <= 1.0e-15
                && out[1].abs() <= 1.0e-15
                && (out[2] - expected).abs() <= 1.0e-12,
            "ngspice computes one delayed pulse tail and assigns it after one channel advance; expected out[2]={expected:e}, got {out:?}"
        );
    }

    #[test]
    fn core_table_cache_reloads_when_h_or_b_vectors_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("h_array", vec![0.0, 1.0, 2.0]);
        ctx.set_real_vector_param("b_array", vec![0.0, 1.0, 4.0]);

        let first = cache_core_table(&mut ctx).expect("core table caches");
        let second = cache_core_table(&mut ctx).expect("core table reuses cache");
        assert!(Arc::ptr_eq(&first, &second));
        assert!(first.strictly_increasing_h);
        assert_eq!(first.midpoints, vec![0.5, 1.5]);
        assert_eq!(
            first.last_midpoint_index.load(Ordering::Relaxed),
            CORE_TABLE_UNSET_MIDPOINT_INDEX
        );

        ctx.set_real_vector_param("unrelated", vec![9.0, 10.0]);
        let after_unrelated =
            cache_core_table(&mut ctx).expect("unrelated vector preserves core table");
        assert!(Arc::ptr_eq(&first, &after_unrelated));

        ctx.set_real_vector_param("b_array", vec![0.0, 2.0, 8.0]);
        let updated = cache_core_table(&mut ctx).expect("updated core table caches");
        assert!(!Arc::ptr_eq(&first, &updated));
        assert_eq!(updated.points[1].b, 2.0);
    }

    #[test]
    fn core_pwl_binary_midpoint_lookup_matches_legacy_scan_path() {
        let table = core_table_from_points(vec![
            CorePoint { h: 0.0, b: 0.0 },
            CorePoint { h: 1.0, b: 1.0 },
            CorePoint { h: 2.0, b: 4.0 },
            CorePoint { h: 3.0, b: 9.0 },
            CorePoint { h: 4.0, b: 16.0 },
        ]);
        assert!(table.strictly_increasing_h);

        let mut legacy = table.clone();
        legacy.strictly_increasing_h = false;

        for h_input in [0.25, 0.5, 1.25, 1.5, 1.9, 2.5, 3.6] {
            let fast = core_pwl_flux_density(&table, h_input, 0.2, false);
            let scanned = core_pwl_flux_density(&legacy, h_input, 0.2, false);
            assert_eq!(fast, scanned, "h_input={h_input}");
        }
    }

    #[test]
    fn core_pwl_midpoint_cursor_falls_back_for_large_h_jumps() {
        let table = core_table_from_points(
            (0..=24)
                .map(|h| CorePoint {
                    h: h as Value,
                    b: (h * h) as Value,
                })
                .collect(),
        );
        assert!(table.strictly_increasing_h);

        let mut legacy = table.clone();
        legacy.strictly_increasing_h = false;

        let low = core_pwl_flux_density(&table, 2.25, 0.2, false);
        assert_eq!(low, core_pwl_flux_density(&legacy, 2.25, 0.2, false));
        assert_eq!(table.last_midpoint_index.load(Ordering::Relaxed), 2);

        let high = core_pwl_flux_density(&table, 22.25, 0.2, false);
        assert_eq!(high, core_pwl_flux_density(&legacy, 22.25, 0.2, false));
        assert_eq!(
            table.last_midpoint_index.load(Ordering::Relaxed),
            22,
            "large non-local h jumps should land on the binary-search midpoint index"
        );
    }

    #[test]
    fn core_pwl_partials_match_flux_derivative() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("h_array", vec![0.0, 1.0, 2.0]);
        ctx.set_real_vector_param("b_array", vec![0.0, 2.0, 5.0]);
        ctx.set_param("area", 2.0);
        ctx.set_param("length", 4.0);
        ctx.set_param("mode", CORE_MODE_PWL as Value);
        ctx.set_param("input_domain", 0.1);
        ctx.set_param("fraction", 0.0);
        ctx.set_input_analog("mc", 6.4);

        Core.init(&mut ctx).expect("core init");
        let eval = core_eval_for_context(&ctx).expect("read-only core eval");
        let partials = Core.output_input_partials(&ctx, "mc");

        assert_eq!(partials.len(), 1);
        assert_eq!(partials[0].0, "mc");
        assert!((eval.derivative - 1.5).abs() < 1.0e-15);
        assert!((partials[0].1 - eval.derivative).abs() < 1.0e-15);
    }

    #[test]
    fn core_hysteresis_does_not_commit_rollbackable_probe_branch_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_real_vector_param("h_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("b_array", vec![0.0, 1.0]);
        ctx.set_param("area", 1.0);
        ctx.set_param("length", 1.0);
        ctx.set_param("mode", CORE_MODE_HYSTERESIS as Value);
        ctx.set_param("in_low", 0.0);
        ctx.set_param("in_high", 1.0);
        ctx.set_param("hyst", 0.2);
        ctx.set_param("out_lower_limit", 0.0);
        ctx.set_param("out_upper_limit", 1.0);
        ctx.set_param("input_domain", 0.0);
        ctx.set_param("fraction", 0.0);

        Core.init(&mut ctx).expect("core init");

        ctx.set_input_analog("mc", 0.0);
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Core.evaluate(&mut ctx)
            .expect("records initial rising branch");
        assert_eq!(ctx.int_state(0), CORE_HYSTERESIS_RISING);

        ctx.set_input_analog("mc", 1.3);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        Core.evaluate(&mut ctx).expect("probes falling branch");
        assert_eq!(
            ctx.int_state(0),
            CORE_HYSTERESIS_RISING,
            "rollbackable core probe must not advance remembered hysteresis branch"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Core.evaluate(&mut ctx)
            .expect("commits accepted falling branch");
        assert_eq!(ctx.int_state(0), CORE_HYSTERESIS_FALLING);
    }

    #[test]
    fn core_hysteresis_partials_do_not_commit_branch_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_real_vector_param("h_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("b_array", vec![0.0, 1.0]);
        ctx.set_param("area", 1.0);
        ctx.set_param("length", 1.0);
        ctx.set_param("mode", CORE_MODE_HYSTERESIS as Value);
        ctx.set_param("in_low", 0.0);
        ctx.set_param("in_high", 1.0);
        ctx.set_param("hyst", 0.2);
        ctx.set_param("out_lower_limit", 0.0);
        ctx.set_param("out_upper_limit", 1.0);
        ctx.set_param("input_domain", 0.0);
        ctx.set_param("fraction", 0.0);

        Core.init(&mut ctx).expect("core init");

        ctx.set_input_analog("mc", 0.0);
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Core.evaluate(&mut ctx)
            .expect("records initial rising branch");
        assert_eq!(ctx.int_state(0), CORE_HYSTERESIS_RISING);

        ctx.set_input_analog("mc", 1.3);
        let partials = Core.output_input_partials(&ctx, "mc");

        assert_eq!(partials.len(), 1);
        assert_eq!(
            ctx.int_state(0),
            CORE_HYSTERESIS_RISING,
            "querying core partials must not advance remembered hysteresis branch"
        );
    }
}
