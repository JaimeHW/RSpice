//! Built-in XSPICE Code Models
//!
//! Provides a comprehensive library of analog and digital code models.
//!
//! # Analog Models
//!
//! - **gain** - Voltage gain block
//! - **summer** - Analog summer
//! - **mult** - Analog multiplier  
//! - **divider** / **divide** - Analog divider
//! - **pwl** - Piecewise-linear controlled source
//! - **pwlts** - Piecewise-linear time-series source
//! - **filesource** / **file_source** - Analog vector stimulus from file
//! - **table2d** / **table3d** - File-backed multidimensional lookup tables
//! - **multi_input_pwl** - Multi-input piecewise-linear analog gate
//! - **spice2poly** / **icm_spice2poly** - SPICE2-compatible polynomial controlled source
//! - **xfer** - AC table transfer function
//! - **s_xfer** - s-domain transfer function
//! - **sine** - Controlled sine-wave oscillator
//! - **square** - Controlled square-wave oscillator
//! - **triangle** - Controlled triangle-wave oscillator
//! - **limit** - Limiter with clipping
//! - **climit** - Controlled limiter
//! - **hyst** - Hysteresis block
//! - **delay** - Analog delay line
//! - **astate** - Analog state return
//! - **oneshot** - Analog one-shot pulse generator
//! - **integrator** / **int** - Continuous-time integrator
//! - **differentiator** / **d_dt** - Continuous-time differentiator
//! - **slew** - Slew-rate limited follower
//! - **aswitch** - Analog switch
//! - **pswitch** - PSPICE-compatible analog switch
//! - **sidiode** - Simple XSPICE diode
//! - **zener** - XSPICE Zener diode
//! - **memristor** - Threshold memristive device
//! - **core** - Magnetic core
//! - **cmeter** / **lmeter** - Topology-measuring C/L meters
//! - **lcouple** - Inductive winding/core coupling
//! - **ilimit** - Current-limited analog output driver
//! - **seegen** - Single-event-effect current generator
//! - **potentiometer** - Three-terminal analog potentiometer
//! - **tline** - Generic transmission line
//! - **cpline** - Coupled transmission line
//! - **mlin** - Microstrip transmission line
//! - **cpmlin** - Coupled microstrip transmission line
//! - **msopen** - Microstrip open-end admittance
//!
//! # A/D and D/A Bridges
//!
//! - **adc_bridge** - Analog to digital converter
//! - **dac_bridge** - Digital to analog converter
//! - **bidi_bridge** - Bidirectional analog/digital node bridge
//!
//! # Digital Models
//!
//! - **d_source** - Digital stimulus from file
//! - **d_inverter** - Inverter gate
//! - **d_buffer** - Buffer gate
//! - **d_and** / **d_nand** - AND/NAND gates
//! - **d_or** / **d_nor** - OR/NOR gates
//! - **d_xor** / **d_xnor** - XOR/XNOR gates
//! - **d_tristate** - Tri-state buffer
//! - **d_pullup** / **d_pulldown** - Pull resistors
//! - **d_lut** / **d_genlut** - Digital lookup tables
//! - **d_fdiv** - Digital frequency divider
//! - **d_osc** / **d_pwm** - Controlled digital oscillators
//! - **nco** - MIDI numerically controlled oscillator
//! - **d_process** - External digital process co-simulation
//! - **d_cosim** - External irreversible digital co-simulation
//! - **d_dff** - D flip-flop
//! - **d_jkff** - JK flip-flop
//! - **d_tff** - T flip-flop
//! - **d_srff** - SR flip-flop
//! - **d_dlatch** - D latch
//! - **d_srlatch** - SR latch
//! - **d_state** - State machine
//! - **d_ram** - Random access memory
//!
//! # Real Event Models
//!
//! - **d_to_real** - Digital to real-valued event converter
//! - **real_gain** - Real-valued event gain
//! - **real_delay** - Clocked real-valued event delay
//! - **real_to_v** - Real-valued event to analog converter
//!
//! # Debug/Example Models
//!
//! - **print_param_types** - ngspice example parameter-channel model

// Analog behavioral blocks
mod analog;

// A/D and D/A bridges
mod bridges;

// Lookup-table analog sources
mod lookup;

// File-backed analog vector source
mod file_source;

// File-backed multidimensional lookup tables
mod table;

// Multi-input analog sources
mod multi_input;

// SPICE2-compatible polynomial source
mod spice2poly;

// Frequency-domain transfer functions
mod transfer;

// Controlled waveform oscillators
mod waveform;

// XSPICE xtradev devices
mod xtradev;

// XSPICE xtraevt event models
mod xtraevt;

// XSPICE transmission-line models
mod tlines;

// XSPICE debug/example models
mod debug;

// Digital primitives
mod digital;

// Re-export all models
pub use analog::{
    AnalogDelayLine, AnalogOneShot, AnalogStateReturn, ControlledLimiter, Differentiator,
    DifferentiatorAlias, DivideAlias, Divider, Gain, HysteresisBlock, Integrator, IntegratorAlias,
    Limiter, Multiplier, SampleHold, SlewRateFollower, Summer,
};

pub use bridges::{AdcBridge, BidiBridge, DacBridge};

pub use digital::{
    DFlipFlop, DLatch, DigitalAnd, DigitalBuffer, DigitalCosim, DigitalFrequencyDivider,
    DigitalGenericLookupTable, DigitalInverter, DigitalLookupTable, DigitalNand, DigitalNor,
    DigitalOpenCollector, DigitalOpenEmitter, DigitalOr, DigitalOscillator, DigitalProcess,
    DigitalPulldown, DigitalPullup, DigitalPwmOscillator, DigitalRam, DigitalSource,
    DigitalStateMachine, DigitalTristate, DigitalXnor, DigitalXor, JkFlipFlop,
    NumericallyControlledOscillator, SrFlipFlop, SrLatch, TFlipFlop,
};

pub use lookup::{PiecewiseLinear, PiecewiseLinearTimeSeries};

pub use file_source::{FileSource, FileSourceAlias};

pub use table::{Table2D, Table3D};

pub use multi_input::MultiInputPwl;

pub use spice2poly::{IcmSpice2Poly, Spice2Poly};

pub use transfer::{SXfer, Xfer};

pub use waveform::{SineOscillator, SquareOscillator, TriangleOscillator};

pub use tlines::{
    CoupledMicrostripLine, CoupledTransmissionLine, GenericTransmissionLine, MicrostripLine,
    MicrostripOpenEnd,
};

pub(crate) use xtradev::XTRADEV_METER_MEASURED_VALUE_PARAM;
pub use xtradev::{
    AnalogSwitch, CapacitanceMeter, CapacitorIc, Core, Ilimit, InductanceMeter, InductorIc,
    LcCouple, Memristor, Potentiometer, Pswitch, SeeGenerator, Sidiode, Zener,
};

pub use xtraevt::{DigitalToReal, RealDelay, RealGain, RealToVoltage};

pub use debug::PrintParamTypes;
